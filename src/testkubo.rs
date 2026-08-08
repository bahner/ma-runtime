//! In-process mock of the Kubo HTTP DAG API for integration tests.
//!
//! Stores DAG nodes content-addressed (blake3 of the JSON body) and serves the
//! `dag/put`, `dag/get`, `dag/resolve`, `cat`, and `pin/*` endpoints that
//! [`crate::kubo`] and `ma_core` talk to — enough to drive the manifest writer
//! and other IPFS-backed flows without a real Kubo daemon.

use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::{RawQuery, State},
    routing::post,
    Router,
};
use tokio::sync::Mutex;

pub async fn test_endpoint(secret: [u8; 32]) -> Box<dyn ma_core::MaEndpoint> {
    let bundle = ma_core::SecretBundle::generate();
    let resolver = Arc::new(ma_core::IpfsGatewayResolver::new("http://127.0.0.1:9"));
    ma_core::new_ma_endpoint(
        secret,
        bundle.encryption_key().expect("test encryption key"),
        resolver,
        false,
    )
    .await
    .expect("test endpoint")
}

#[derive(Clone, Default)]
struct Store(Arc<Mutex<HashMap<String, Vec<u8>>>>);

/// A running in-process mock Kubo.  Pass [`MockKubo::url`] wherever a
/// `kubo_rpc_url` is expected.
pub struct MockKubo {
    url: String,
    store: Store,
}

impl MockKubo {
    /// Bind an ephemeral port and start serving the mock DAG API.
    pub async fn start() -> Self {
        let store = Store::default();
        let app = Router::new()
            .route("/api/v0/dag/put", post(dag_put))
            .route("/api/v0/dag/get", post(dag_get))
            .route("/api/v0/dag/resolve", post(dag_resolve))
            .route("/api/v0/name/resolve", post(name_resolve))
            .route("/api/v0/add", post(add))
            .route("/api/v0/block/get", post(block_get))
            .route("/api/v0/cat", post(cat))
            .route("/api/v0/pin/update", post(pin_ok))
            .route("/api/v0/pin/add", post(pin_ok))
            .with_state(store.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        Self {
            url: format!("http://{addr}"),
            store,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    /// Store raw bytes (e.g. a Wasm module) and return their fake CID, so
    /// tests can serve plugin behaviour blobs through `/api/v0/cat`.
    pub async fn add_bytes(&self, bytes: Vec<u8>) -> String {
        let cid = fake_cid(&bytes);
        self.store.0.lock().await.insert(cid.clone(), bytes);
        cid
    }
}

/// Content-address a node body.  Deterministic and CID-shaped, but not a real
/// multihash — the mock only ever uses it as an opaque key.
fn fake_cid(bytes: &[u8]) -> String {
    format!("bafyrei{}", blake3::hash(bytes).to_hex())
}

/// Extract the JSON object from a multipart body: the span from the first `{`
/// to the last `}`.  Multipart boundaries and part headers contain no braces,
/// so this reliably isolates the serialised node for our controlled payloads.
fn extract_json(body: &[u8]) -> Vec<u8> {
    match (
        body.iter().position(|&b| b == b'{'),
        body.iter().rposition(|&b| b == b'}'),
    ) {
        (Some(s), Some(e)) if e >= s => body[s..=e].to_vec(),
        _ => Vec::new(),
    }
}

fn extract_multipart_file(body: &[u8]) -> Vec<u8> {
    let Some(start) = body.windows(4).position(|w| w == b"\r\n\r\n") else {
        return body.to_vec();
    };
    let content_start = start + 4;
    let content = &body[content_start..];
    let end = content
        .windows(4)
        .position(|w| w == b"\r\n--")
        .unwrap_or(content.len());
    content[..end].to_vec()
}

fn query_arg(raw: Option<String>, key: &str) -> Option<String> {
    raw?.split('&').find_map(|kv| {
        let (k, v) = kv.split_once('=')?;
        (k == key).then(|| percent_decode(v))
    })
}

/// Minimal percent-decoder for query values (e.g. `%2F` -> `/`). Real HTTP
/// clients (reqwest) percent-encode query arguments containing reserved
/// characters like `/`; axum's `RawQuery` extractor hands back the raw,
/// still-encoded string, so callers must decode it themselves.
fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(hex) = std::str::from_utf8(&bytes[i + 1..i + 3]) {
                if let Ok(byte) = u8::from_str_radix(hex, 16) {
                    out.push(byte);
                    i += 3;
                    continue;
                }
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

async fn dag_put(State(store): State<Store>, body: Bytes) -> String {
    let json = extract_json(&body);
    let cid = fake_cid(&json);
    store.0.lock().await.insert(cid.clone(), json);
    format!("{{\"Cid\":{{\"/\":\"{cid}\"}}}}")
}

async fn dag_get(State(store): State<Store>, RawQuery(q): RawQuery) -> Vec<u8> {
    let cid = query_arg(q, "arg").unwrap_or_default();
    store.0.lock().await.get(&cid).cloned().unwrap_or_default()
}

async fn cat(State(store): State<Store>, RawQuery(q): RawQuery) -> Vec<u8> {
    let arg = query_arg(q, "arg").unwrap_or_default();
    let cid = arg.strip_prefix("/ipfs/").unwrap_or(&arg);
    store.0.lock().await.get(cid).cloned().unwrap_or_default()
}

async fn add(State(store): State<Store>, body: Bytes) -> String {
    let bytes = extract_multipart_file(&body);
    let cid = fake_cid(&bytes);
    store.0.lock().await.insert(cid.clone(), bytes);
    format!("{{\"Hash\":\"{cid}\"}}")
}

async fn block_get(State(store): State<Store>, RawQuery(q): RawQuery) -> Vec<u8> {
    let arg = query_arg(q, "arg").unwrap_or_default();
    let cid = arg.strip_prefix("/ipfs/").unwrap_or(&arg);
    store.0.lock().await.get(cid).cloned().unwrap_or_default()
}

async fn dag_resolve(RawQuery(q): RawQuery) -> String {
    let arg = query_arg(q, "arg").unwrap_or_default();
    // Real Kubo resolves `/ipfs/<cid>[/sub/path]` down to the bare CID, and
    // `/ipns/<key>` similarly after name resolution. The mock has no real
    // IPNS name resolution to perform, so it treats an `/ipns/<key>` path
    // the same as `/ipfs/<cid>`: strip the prefix and pass the rest
    // through as-is (tests that need a genuinely mutable pointer publish
    // fresh content under the same "key" string instead). Anything else
    // (e.g. an already-bare CID) passes through unchanged.
    let cid = arg
        .strip_prefix("/ipfs/")
        .or_else(|| arg.strip_prefix("/ipns/"))
        .unwrap_or(&arg);
    format!("{{\"Cid\":{{\"/\":\"{cid}\"}}}}")
}

async fn name_resolve(RawQuery(q): RawQuery) -> String {
    let arg = query_arg(q, "arg").unwrap_or_default();
    let cid = arg.strip_prefix("/ipns/").unwrap_or(&arg);
    format!("{{\"Path\":\"/ipfs/{cid}\"}}")
}

async fn pin_ok() -> &'static str {
    ""
}
