//! Minimal Kubo HTTP API wrappers for DAG operations not re-exported by
//! `ma_core`.  Only `dag_put` and `dag_get` are needed here; other Kubo
//! operations (`ipfs_add`, `cat_bytes`) are used directly from `ma_core`.

use anyhow::{anyhow, Result};
use reqwest::multipart;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::Duration;
use tokio::sync::{Semaphore, SemaphorePermit};

const MAX_CONCURRENT_KUBO_REQUESTS: usize = 16;
const KUBO_PERMIT_TIMEOUT: Duration = Duration::from_secs(5);

/// HTTP client with hard timeouts.  `dag/get` on a CID that is not in the
/// local store makes Kubo search the network — without a client-side bound
/// that request (and whatever task awaits it) would hang indefinitely.
pub fn client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .connect_timeout(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    })
}

fn request_gate() -> &'static Semaphore {
    static GATE: OnceLock<Semaphore> = OnceLock::new();
    GATE.get_or_init(|| Semaphore::new(MAX_CONCURRENT_KUBO_REQUESTS))
}

async fn acquire_request_permit(op: &str) -> Result<SemaphorePermit<'static>> {
    tokio::time::timeout(KUBO_PERMIT_TIMEOUT, request_gate().acquire())
        .await
        .map_err(|_| anyhow!("kubo request queue timed out for {op}"))?
        .map_err(|_| anyhow!("kubo request gate closed for {op}"))
}

// ── Response types ────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct DagPutCid {
    #[serde(rename = "/")]
    slash: String,
}

#[derive(Deserialize)]
struct DagPutResponse {
    #[serde(default, rename = "Cid")]
    cid_upper: Option<DagPutCid>,
    #[serde(default)]
    cid: Option<DagPutCid>,
}

#[derive(Deserialize)]
struct DagResolveResponse {
    #[serde(rename = "Cid")]
    cid: DagPutCid,
}

#[derive(Deserialize)]
struct AddResponse {
    #[serde(rename = "Hash")]
    hash: String,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Publish a serialisable value as a `dag-cbor` IPLD node via Kubo.
/// Input is serialised as `dag-json`; Kubo converts and stores as `dag-cbor`.
/// Returns the resulting CID string.
pub async fn dag_put<T: Serialize + Sync>(kubo_url: &str, value: &T) -> Result<String> {
    let _permit = acquire_request_permit("dag/put").await?;
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/dag/put");
    let payload = serde_json::to_vec(value)?;

    let part = multipart::Part::bytes(payload)
        .file_name("node.json")
        .mime_str("application/json")?;
    let form = multipart::Form::new().part("file", part);

    let body = client()
        .post(url)
        .query(&[
            ("store-codec", "dag-cbor"),
            ("input-codec", "dag-json"),
            ("pin", "false"),
        ])
        .multipart(form)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let parsed: DagPutResponse = serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed parsing dag/put response: {e} body={body}"))?;
    parsed
        .cid_upper
        .or(parsed.cid)
        .map(|c| c.slash)
        .ok_or_else(|| anyhow!("missing CID in dag/put response: {body}"))
}

/// Recursively pin a CID. Used for first-time bootstrap when there is no
/// prior root to update from.
pub async fn pin_add(kubo_url: &str, cid: &str) -> Result<()> {
    let permit = acquire_request_permit("pin/add").await?;
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/pin/add");
    client()
        .post(&url)
        .query(&[("arg", cid), ("recursive", "true")])
        .send()
        .await?
        .error_for_status()?;
    drop(permit);
    routing_provide(kubo_url, cid).await?;
    Ok(())
}

/// Announce a CID to IPFS routing so independent Kubo nodes can discover the
/// provider and fetch the recursively pinned DAG.
pub async fn routing_provide(kubo_url: &str, cid: &str) -> Result<()> {
    let _permit = acquire_request_permit("routing/provide").await?;
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/routing/provide");
    client()
        .post(url)
        .query(&[("arg", cid), ("recursive", "true")])
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

/// Atomically move the recursive pin from `old_cid` to `new_cid` via
/// Kubo's `pin/update` endpoint (`unpin=true`).  If `old_cid` was not
/// pinned, falls back to `pin_add(new_cid)` so first-time callers work too.
pub async fn pin_update(kubo_url: &str, old_cid: &str, new_cid: &str) -> Result<()> {
    let permit = acquire_request_permit("pin/update").await?;
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/pin/update");
    let resp = client()
        .post(&url)
        .query(&[("arg", old_cid), ("arg", new_cid), ("unpin", "true")])
        .send()
        .await?;
    if resp.status().is_success() {
        drop(permit);
        routing_provide(kubo_url, new_cid).await?;
        return Ok(());
    }
    let body = resp.text().await.unwrap_or_default();
    // Kubo returns "not recursively pinned already" (or similar) when the old
    // CID was never pinned.  Fall back to a plain recursive pin of the new CID.
    if body.contains("not recursively pinned") || body.contains("not pinned") {
        drop(permit);
        return pin_add(kubo_url, new_cid).await;
    }
    Err(anyhow!("pin/update {old_cid} → {new_cid} failed: {body}"))
}

async fn ipfs_add_bytes_with_pin(kubo_url: &str, data: Vec<u8>, pin: bool) -> Result<String> {
    let _permit = acquire_request_permit("add").await?;
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/add");

    let part = multipart::Part::bytes(data).file_name("data");
    let form = multipart::Form::new().part("file", part);

    let body = client()
        .post(url)
        .query(&[("pin", if pin { "true" } else { "false" })])
        .multipart(form)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let parsed: AddResponse = serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed parsing add response: {e} body={body}"))?;
    Ok(parsed.hash)
}

/// Add raw bytes to IPFS via Kubo without creating a direct local pin.
///
/// Use this for runtime-owned state that is linked from the root manifest and
/// therefore kept alive by the root recursive pin lifecycle.
pub async fn ipfs_add_bytes_unpinned(kubo_url: &str, data: Vec<u8>) -> Result<String> {
    ipfs_add_bytes_with_pin(kubo_url, data, false).await
}

/// Fetch raw bytes from IPFS through Kubo's `/cat` endpoint.
pub async fn cat_bytes(kubo_url: &str, cid: &str) -> Result<Vec<u8>> {
    let _permit = acquire_request_permit("cat").await?;
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/cat");

    let bytes = client()
        .post(url)
        .query(&[("arg", cid)])
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    Ok(bytes.to_vec())
}

/// Fetch an IPLD node from Kubo and deserialise it from `dag-json`.
pub async fn dag_get<T: DeserializeOwned>(kubo_url: &str, cid: &str) -> Result<T> {
    let _permit = acquire_request_permit("dag/get").await?;
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/dag/get");

    let body = client()
        .post(&url)
        .query(&[("arg", cid), ("output-codec", "dag-json")])
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed to deserialise dag/get response for {cid}: {e} body={body}"))
}

/// Resolve an IPFS/IPNS path to a bare CID string.
///
/// Accepts bare CIDs (`bafy…`), `/ipfs/<cid>`, and `/ipns/<key>` paths.
/// `/ipns/` paths are resolved through Kubo's name resolution.
pub async fn dag_resolve(kubo_url: &str, path: &str) -> Result<String> {
    // Bare CID — nothing to resolve.
    if !path.starts_with('/') {
        return Ok(path.to_string());
    }

    let _permit = acquire_request_permit("dag/resolve").await?;

    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/dag/resolve");

    let body = client()
        .post(&url)
        .query(&[("arg", path)])
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let parsed: DagResolveResponse = serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed parsing dag/resolve response for {path}: {e} body={body}"))?;
    Ok(parsed.cid.slash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Bytes,
        extract::{RawQuery, State},
        routing::post,
        Router,
    };
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[derive(Clone, Default)]
    struct AddState(Arc<Mutex<Vec<String>>>);

    async fn add_handler(State(state): State<AddState>, RawQuery(q): RawQuery, _: Bytes) -> String {
        let pin = q
            .unwrap_or_default()
            .split('&')
            .find_map(|kv| kv.strip_prefix("pin="))
            .unwrap_or("")
            .to_string();
        state.0.lock().await.push(pin);
        "{\"Hash\":\"bafyreibkhaddtestcid\"}".to_string()
    }

    async fn start_add_server(state: AddState) -> String {
        let app = Router::new()
            .route("/api/v0/add", post(add_handler))
            .with_state(state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        format!("http://{addr}")
    }

    #[tokio::test]
    async fn state_add_uses_unpinned_kubo_add() {
        let state = AddState::default();
        let url = start_add_server(state.clone()).await;

        let cid = ipfs_add_bytes_unpinned(&url, b"state".to_vec())
            .await
            .unwrap();

        assert_eq!(cid, "bafyreibkhaddtestcid");
        assert_eq!(state.0.lock().await.as_slice(), &["false"]);
    }
}
