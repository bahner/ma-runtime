//! Minimal Kubo HTTP API wrappers for DAG operations not re-exported by
//! `ma_core`.  Only `dag_put` and `dag_get` are needed here; other Kubo
//! operations (`ipfs_add`, `cat_bytes`) are used directly from `ma_core`.

use anyhow::{anyhow, Result};
use reqwest::multipart;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::OnceLock;

/// HTTP client with hard timeouts.  `dag/get` on a CID that is not in the
/// local store makes Kubo search the network — without a client-side bound
/// that request (and whatever task awaits it) would hang indefinitely.
pub(crate) fn client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .connect_timeout(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    })
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
struct NameResolveResponse {
    #[serde(default, rename = "Path")]
    path_upper: String,
    #[serde(default, rename = "path")]
    path_lower: String,
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
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/pin/add");
    client()
        .post(&url)
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
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/pin/update");
    let resp = client()
        .post(&url)
        .query(&[("arg", old_cid), ("arg", new_cid), ("unpin", "true")])
        .send()
        .await?;
    if resp.status().is_success() {
        return Ok(());
    }
    let body = resp.text().await.unwrap_or_default();
    // Kubo returns "not recursively pinned already" (or similar) when the old
    // CID was never pinned.  Fall back to a plain recursive pin of the new CID.
    if body.contains("not recursively pinned") || body.contains("not pinned") {
        return pin_add(kubo_url, new_cid).await;
    }
    Err(anyhow!("pin/update {old_cid} → {new_cid} failed: {body}"))
}

/// Add raw bytes to IPFS via Kubo and return the resulting CID.
pub async fn ipfs_add_bytes(kubo_url: &str, data: Vec<u8>) -> Result<String> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/add");

    let part = multipart::Part::bytes(data).file_name("data");
    let form = multipart::Form::new().part("file", part);

    let body = client()
        .post(url)
        .query(&[("pin", "true")])
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

/// Fetch raw bytes from IPFS through Kubo's `/cat` endpoint.
pub async fn cat_bytes(kubo_url: &str, cid: &str) -> Result<Vec<u8>> {
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

/// Fetch raw IPLD block bytes from local Kubo using `/api/v0/block/get`.
pub async fn block_get_bytes(kubo_url: &str, cid: &str) -> Result<Vec<u8>> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/block/get");

    let body = client()
        .post(&url)
        .query(&[("arg", cid)])
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    Ok(body.to_vec())
}

/// Resolve an IPNS key through local Kubo's name resolver and return the target CID.
pub async fn name_resolve(kubo_url: &str, ipns_id: &str) -> Result<String> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/name/resolve");
    let arg = format!("/ipns/{ipns_id}");

    let body = client()
        .post(&url)
        .query(&[("arg", arg.as_str()), ("recursive", "true")])
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let parsed: NameResolveResponse = serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed parsing name/resolve response for {arg}: {e} body={body}"))?;
    let path = if parsed.path_upper.is_empty() {
        parsed.path_lower
    } else {
        parsed.path_upper
    };
    let cid = path
        .trim()
        .strip_prefix("/ipfs/")
        .map_or_else(|| path.trim().to_string(), ToString::to_string);
    if cid.is_empty() {
        Err(anyhow!(
            "name/resolve returned empty path for {arg}: {body}"
        ))
    } else {
        Ok(cid)
    }
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
