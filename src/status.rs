use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::{Path, State};
use axum::http::{header, Method, StatusCode, Uri};
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::{json, Value};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

use crate::acl::SharedAcl;
use crate::crud::config::DEFAULT_ZION_SOURCE;
use crate::entity::RuntimeManifest;

const ZION_CONFIG_KEY: &str = "zion";

#[derive(Default)]
pub struct Stats {
    pub our_did: String,
    pub endpoint_id: String,
    pub ipfs_requests: u64,
    pub rpc_requests: u64,
    pub started_at: u64,
    pub ipfs_publisher_enabled: bool,
    pub entity_names: Vec<String>,
    pub root_cid: Option<String>,
    pub kubo_rpc_url: String,
    /// DIDs that own this runtime; set via --owner / config or POST /claim.
    pub owners: Vec<String>,
    /// Path to config.yaml; used by /claim to persist owners.
    pub config_path: Option<PathBuf>,
}

pub type SharedStats = Arc<RwLock<Stats>>;

#[derive(Default)]
struct ProcessMetrics {
    pid: u32,
    vm_peak_kib: Option<u64>,
    vm_size_kib: Option<u64>,
    vm_rss_kib: Option<u64>,
    vm_data_kib: Option<u64>,
    threads: Option<u64>,
    open_fds: Option<u64>,
}

/// Combined axum router state for the status server.
#[derive(Clone)]
pub struct StatusState {
    pub stats: SharedStats,
    pub acl: SharedAcl,
}

pub fn spawn_status_server(stats: SharedStats, acl: SharedAcl, status_bind: SocketAddr) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    let status_router = Router::new()
        .route("/", get(handle_index))
        .route("/status.json", get(handle_status_json))
        .route("/bootstrap.yaml", get(handle_bootstrap_yaml))
        .route("/ipfs/*path", get(handle_ipfs_path))
        .route("/ipns/*path", get(handle_ipns_path))
        .route("/ipld/*path", get(handle_ipld_path))
        .route("/zion", get(handle_zion_redirect))
        .route("/zion/", get(handle_zion_index))
        .route("/zion/*path", get(handle_zion_path))
        .route("/claim", post(handle_claim))
        .fallback(handle_zion_root_asset)
        .layer(cors)
        .with_state(StatusState { stats, acl });

    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(status_bind)
            .await
            .expect("status server bind failed");
        info!(bind = %status_bind, "{}", crate::i18n::t("status-listening"));
        axum::serve(listener, status_router)
            .await
            .expect("status server failed");
    });
}

pub fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

async fn handle_index(State(state): State<StatusState>) -> impl IntoResponse {
    let shared_stats = &state.stats;
    let (
        our_did,
        endpoint_id,
        ipfs_requests,
        rpc_requests,
        uptime,
        ipfs_enabled,
        entity_names,
        root_cid,
        owners,
        kubo_rpc_url,
    ) = {
        let s = shared_stats.read().await;
        (
            s.our_did.clone(),
            s.endpoint_id.clone(),
            s.ipfs_requests,
            s.rpc_requests,
            now_unix_secs().saturating_sub(s.started_at),
            s.ipfs_publisher_enabled,
            s.entity_names.clone(),
            s.root_cid.clone(),
            s.owners.clone(),
            s.kubo_rpc_url.clone(),
        )
    };
    let zion_path =
        manifest_config_string(&kubo_rpc_url, root_cid.as_deref(), ZION_CONFIG_KEY).await;
    let zion_trust_html = zion_source_trust_notice(zion_path.as_deref());
    let ipfs_status = if ipfs_enabled { "enabled" } else { "disabled" };
    let entities_html = if entity_names.is_empty() {
        "<em>none</em>".to_string()
    } else {
        entity_names
            .iter()
            .map(|n| format!("<code>#{n}</code>"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let ipns_html = did_to_ipns_path(&our_did).unwrap_or_else(|| "-".to_string());
    let root_cid_html = root_cid.as_deref().unwrap_or("-").to_string();
    let owner_html = if owners.is_empty() {
        "<em>unclaimed</em>".to_string()
    } else {
        owners.join("<br>")
    };
    let zion_html = zion_path.map_or_else(
        || "<em>not configured</em>".to_string(),
        |path| format!(r#"<a href="/zion/">/zion/</a> <code>{path}</code>"#),
    );
    let process = process_metrics();
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>間 Runtime</title>
<style>body{{font-family:monospace;max-width:700px;margin:2em auto;background:#111;color:#eee}}
h1{{color:#7cf}}table{{border-collapse:collapse;width:100%}}
td,th{{padding:6px 12px;border:1px solid #333;text-align:left}}
th{{background:#222}}a{{color:#7cf}}.trust-warning{{border:1px solid #d99b32;padding:12px;color:#ffd27a}}</style></head>
<body>
<h1>間 Runtime</h1>
<table>
<tr><th>Field</th><th>Value</th></tr>
<tr><td>DID</td><td>{our_did}</td></tr>
<tr><td>IPNS</td><td>{ipns_html}</td></tr>
<tr><td>Endpoint ID (iroh)</td><td>{endpoint_id}</td></tr>
<tr><td>Uptime (seconds)</td><td>{uptime}</td></tr>
<tr><td>IPFS publisher</td><td>{ipfs_status}</td></tr>
<tr><td>IPFS publish requests</td><td>{ipfs_requests}</td></tr>
<tr><td>RPC requests</td><td>{rpc_requests}</td></tr>
<tr><td>Entities</td><td>{entities_html}</td></tr>
<tr><td>Runtime</td><td>{root_cid_html}</td></tr>
<tr><td>Zion</td><td>{zion_html}</td></tr>
<tr><td>Owner</td><td>{owner_html}</td></tr>
<tr><td>Process PID</td><td>{pid}</td></tr>
<tr><td>Process threads</td><td>{threads}</td></tr>
<tr><td>Open file descriptors</td><td>{open_fds}</td></tr>
<tr><td>VmRSS</td><td>{vm_rss}</td></tr>
<tr><td>VmSize</td><td>{vm_size}</td></tr>
<tr><td>VmData</td><td>{vm_data}</td></tr>
<tr><td>VmPeak</td><td>{vm_peak}</td></tr>
</table>
{zion_trust_html}
<p><a href="/status.json">status.json</a> &bull; <a href="/bootstrap.yaml">bootstrap.yaml</a></p>
</body></html>"#,
        pid = process.pid,
        threads = format_opt_count(process.threads),
        open_fds = format_opt_count(process.open_fds),
        vm_rss = format_opt_kib(process.vm_rss_kib),
        vm_size = format_opt_kib(process.vm_size_kib),
        vm_data = format_opt_kib(process.vm_data_kib),
        vm_peak = format_opt_kib(process.vm_peak_kib),
    );
    Html(html)
}

async fn handle_status_json(State(state): State<StatusState>) -> impl IntoResponse {
    let shared_stats = &state.stats;
    let (
        our_did,
        endpoint_id,
        ipfs_requests,
        rpc_requests,
        started_at,
        uptime,
        ipfs_enabled,
        entity_names,
        root_cid,
        kubo_rpc_url,
    ) = {
        let s = shared_stats.read().await;
        (
            s.our_did.clone(),
            s.endpoint_id.clone(),
            s.ipfs_requests,
            s.rpc_requests,
            s.started_at,
            now_unix_secs().saturating_sub(s.started_at),
            s.ipfs_publisher_enabled,
            s.entity_names.clone(),
            s.root_cid.clone(),
            s.kubo_rpc_url.clone(),
        )
    };
    let zion_source =
        manifest_config_string(&kubo_rpc_url, root_cid.as_deref(), ZION_CONFIG_KEY).await;
    let runtime: Value = root_cid.map_or(Value::Null, |cid| json!({ "/": cid }));
    let ipns = did_to_ipns_path(&our_did);
    let process = process_metrics();
    let body = json!({
        "did": our_did,
        "ipns": ipns,
        "endpoint_id": endpoint_id,
        "uptime_secs": uptime,
        "ipfs_publisher": ipfs_enabled,
        "ipfs_requests": ipfs_requests,
        "rpc_requests": rpc_requests,
        "started_at": started_at,
        "entity_names": entity_names,
        "runtime": runtime,
        "zion": {
            "source": zion_source,
            "path": "/zion/",
        },
        "process": {
            "pid": process.pid,
            "threads": process.threads,
            "open_fds": process.open_fds,
            "memory": {
                "vm_peak_kib": process.vm_peak_kib,
                "vm_size_kib": process.vm_size_kib,
                "vm_rss_kib": process.vm_rss_kib,
                "vm_data_kib": process.vm_data_kib,
            }
        },
    });
    (
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        body.to_string(),
    )
}

fn did_to_ipns_path(did: &str) -> Option<String> {
    let identity = did.strip_prefix("did:ma:")?;
    Some(format!("/ipns/{identity}"))
}

async fn manifest_config_string(
    kubo_url: &str,
    root_cid: Option<&str>,
    key: &str,
) -> Option<String> {
    let Some(root_cid) = root_cid else {
        return default_config_string(key);
    };
    let Ok(manifest) = crate::kubo::dag_get::<RuntimeManifest>(kubo_url, root_cid).await else {
        return default_config_string(key);
    };
    let value = manifest
        .config
        .get(key)
        .and_then(serde_yaml::Value::as_str)
        .map(ToString::to_string)
        .or_else(|| default_config_string(key))?;
    normalize_zion_source(&value)
}

fn default_config_string(key: &str) -> Option<String> {
    (key == ZION_CONFIG_KEY).then(|| DEFAULT_ZION_SOURCE.to_string())
}

fn zion_source_trust_notice(source: Option<&str>) -> &'static str {
    if source == Some(DEFAULT_ZION_SOURCE) {
        r#"<p class="trust-warning"><strong>Bootstrap trust:</strong> This default Zion copy is controlled by the ma maintainer's IPNS key. Do not treat that key as a trust root. Publish Zion under an IPNS key you control, then replace <code>/config/zion</code> with your own <code>/ipns/...</code> source.</p>"#
    } else {
        ""
    }
}

fn normalize_zion_source(value: &str) -> Option<String> {
    let value = value.trim().trim_end_matches('/');
    if value.is_empty() {
        return None;
    }
    if value.starts_with("/ipfs/") || value.starts_with("/ipns/") {
        Some(value.to_string())
    } else {
        Some(format!("/ipfs/{value}"))
    }
}

fn zion_asset_path(source: &str, path: &str) -> String {
    format!("{}/{path}", source.trim_end_matches('/'))
}

async fn handle_zion_redirect() -> impl IntoResponse {
    Redirect::permanent("/zion/")
}

async fn handle_zion_index(State(state): State<StatusState>) -> impl IntoResponse {
    serve_zion_path(state, "index.html").await
}

async fn handle_zion_path(
    State(state): State<StatusState>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };
    serve_zion_path(state, path).await
}

async fn handle_zion_root_asset(State(state): State<StatusState>, uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    if path.is_empty() {
        return text_response(StatusCode::NOT_FOUND, "not found");
    }
    serve_zion_path(state, path).await
}

async fn handle_ipfs_path(
    State(state): State<StatusState>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    handle_gateway_path(state, "ipfs", path).await
}

async fn handle_ipns_path(
    State(state): State<StatusState>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    handle_gateway_path(state, "ipns", path).await
}

async fn handle_ipld_path(
    State(state): State<StatusState>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    if path.is_empty() || path.split('/').any(|part| part == "..") {
        return text_response(StatusCode::BAD_REQUEST, "invalid IPLD path");
    }

    let ipfs_path = if path.starts_with("ipfs/") || path.starts_with("ipns/") {
        format!("/{path}")
    } else {
        format!("/ipfs/{path}")
    };
    let kubo_rpc_url = state.stats.read().await.kubo_rpc_url.clone();
    match kubo_block_get_ipld_path(&kubo_rpc_url, &ipfs_path).await {
        Ok(Some(bytes)) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "application/octet-stream"),
                (header::CACHE_CONTROL, "no-store"),
            ],
            bytes,
        )
            .into_response(),
        Ok(None) => text_response(StatusCode::NOT_FOUND, "IPLD path not found"),
        Err(e) => {
            warn!(path = %ipfs_path, error = %e, "failed to relay IPLD block from Kubo");
            text_response(StatusCode::BAD_GATEWAY, "failed to fetch IPLD block")
        }
    }
}

async fn handle_gateway_path(
    state: StatusState,
    namespace: &'static str,
    path: String,
) -> axum::response::Response {
    let path = path.trim_start_matches('/');
    if path.is_empty() || path.split('/').any(|part| part == "..") {
        return text_response(StatusCode::BAD_REQUEST, "invalid gateway path");
    }

    let kubo_rpc_url = state.stats.read().await.kubo_rpc_url.clone();
    match kubo_cat_ipfs_path(&kubo_rpc_url, &format!("/{namespace}/{path}")).await {
        Ok(Some(bytes)) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, content_type_for_path(path)),
                (header::CACHE_CONTROL, "no-store"),
            ],
            bytes,
        )
            .into_response(),
        Ok(None) => text_response(StatusCode::NOT_FOUND, "gateway path not found"),
        Err(e) => {
            warn!(namespace, path = %path, error = %e, "failed to relay gateway asset from Kubo");
            text_response(
                StatusCode::BAD_GATEWAY,
                "failed to fetch gateway asset from Kubo",
            )
        }
    }
}

async fn serve_zion_path(state: StatusState, path: &str) -> axum::response::Response {
    if path.split('/').any(|part| part == "..") {
        return text_response(StatusCode::BAD_REQUEST, "invalid zion path");
    }

    let (root_cid, kubo_rpc_url) = {
        let s = state.stats.read().await;
        (s.root_cid.clone(), s.kubo_rpc_url.clone())
    };
    let Some(zion_source) =
        manifest_config_string(&kubo_rpc_url, root_cid.as_deref(), ZION_CONFIG_KEY).await
    else {
        return text_response(
            StatusCode::NOT_FOUND,
            "runtime /config/zion is not configured",
        );
    };

    let ipfs_path = zion_asset_path(&zion_source, path);
    match kubo_cat_ipfs_path(&kubo_rpc_url, &ipfs_path).await {
        Ok(Some(bytes)) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, content_type_for_path(path)),
                (header::CACHE_CONTROL, "no-store"),
            ],
            bytes,
        )
            .into_response(),
        Ok(None) => text_response(StatusCode::NOT_FOUND, "zion asset not found"),
        Err(e) => {
            warn!(path = %path, source = %zion_source, error = %e, "failed to proxy zion asset from IPFS");
            text_response(
                StatusCode::BAD_GATEWAY,
                "failed to fetch zion asset from IPFS",
            )
        }
    }
}

async fn kubo_cat_ipfs_path(kubo_url: &str, ipfs_path: &str) -> anyhow::Result<Option<Vec<u8>>> {
    let base = kubo_url.trim_end_matches('/');
    let response = crate::kubo::client()
        .post(format!("{base}/api/v0/cat"))
        .query(&[("arg", ipfs_path)])
        .send()
        .await?;
    if !response.status().is_success() {
        return Ok(None);
    }
    let bytes = response.bytes().await?;
    Ok(Some(bytes.to_vec()))
}

async fn kubo_block_get_ipld_path(
    kubo_url: &str,
    ipfs_path: &str,
) -> anyhow::Result<Option<Vec<u8>>> {
    let Ok(cid) = crate::kubo::dag_resolve(kubo_url, ipfs_path).await else {
        return Ok(None);
    };
    let base = kubo_url.trim_end_matches('/');
    let response = crate::kubo::client()
        .post(format!("{base}/api/v0/block/get"))
        .query(&[("arg", cid.as_str())])
        .send()
        .await?;
    if !response.status().is_success() {
        return Ok(None);
    }
    let bytes = response.bytes().await?;
    Ok(Some(bytes.to_vec()))
}

fn text_response(status: StatusCode, body: &str) -> axum::response::Response {
    (
        status,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        body.to_string(),
    )
        .into_response()
}

fn content_type_for_path(path: &str) -> &'static str {
    match path.rsplit_once('.').map(|(_, ext)| ext) {
        Some("css") => "text/css; charset=utf-8",
        Some("html") => "text/html; charset=utf-8",
        Some("js" | "mjs") => "text/javascript; charset=utf-8",
        Some("json") => "application/json",
        Some("wasm") => "application/wasm",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("ico") => "image/x-icon",
        Some("ftl" | "txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn process_metrics() -> ProcessMetrics {
    let mut metrics = ProcessMetrics {
        pid: std::process::id(),
        ..Default::default()
    };
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if let Some(value) = line.strip_prefix("VmPeak:") {
                metrics.vm_peak_kib = parse_status_number(value);
            } else if let Some(value) = line.strip_prefix("VmSize:") {
                metrics.vm_size_kib = parse_status_number(value);
            } else if let Some(value) = line.strip_prefix("VmRSS:") {
                metrics.vm_rss_kib = parse_status_number(value);
            } else if let Some(value) = line.strip_prefix("VmData:") {
                metrics.vm_data_kib = parse_status_number(value);
            } else if let Some(value) = line.strip_prefix("Threads:") {
                metrics.threads = parse_status_number(value);
            }
        }
    }
    metrics.open_fds = std::fs::read_dir("/proc/self/fd")
        .ok()
        .map(|entries| entries.filter_map(Result::ok).count() as u64);
    metrics
}

fn parse_status_number(value: &str) -> Option<u64> {
    value.split_whitespace().next()?.parse().ok()
}

fn format_opt_count(value: Option<u64>) -> String {
    value.map_or_else(|| "-".to_string(), |n| n.to_string())
}

fn format_opt_kib(value: Option<u64>) -> String {
    value.map_or_else(|| "-".to_string(), |n| format!("{n} KiB"))
}

async fn handle_bootstrap_yaml(State(state): State<StatusState>) -> impl IntoResponse {
    let shared_stats = &state.stats;
    let (root_cid, kubo_rpc_url) = {
        let s = shared_stats.read().await;
        (s.root_cid.clone(), s.kubo_rpc_url.clone())
    };

    let Some(cid) = root_cid else {
        return (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            [(axum::http::header::CONTENT_TYPE, "text/plain")],
            "Runtime not yet initialised (no root CID)".to_string(),
        )
            .into_response();
    };

    match crate::bootstrap::export_bootstrap_yaml(&cid, &kubo_rpc_url).await {
        Ok(yaml) => (
            axum::http::StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "text/yaml; charset=utf-8")],
            yaml,
        )
            .into_response(),
        Err(e) => {
            warn!(error = %e, "failed to export bootstrap.yaml");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                [(axum::http::header::CONTENT_TYPE, "text/plain")],
                format!("export failed: {e}"),
            )
                .into_response()
        }
    }
}

#[derive(serde::Deserialize)]
struct ClaimBody {
    owner: String,
}

async fn handle_claim(
    State(state): State<StatusState>,
    Json(body): Json<ClaimBody>,
) -> impl IntoResponse {
    let (owners, config_path) = {
        let s = state.stats.read().await;
        (s.owners.clone(), s.config_path.clone())
    };

    if owners.iter().any(|owner| owner == &body.owner) {
        return (
            axum::http::StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            serde_json::json!({"owners": owners, "status": "already-claimed"}).to_string(),
        )
            .into_response();
    }

    if !owners.is_empty() {
        return (
            axum::http::StatusCode::CONFLICT,
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            serde_json::json!({"error": "already claimed", "owners": owners}).to_string(),
        )
            .into_response();
    }

    let new_owners = vec![body.owner.clone()];
    {
        let mut s = state.stats.write().await;
        s.owners.clone_from(&new_owners);
    }
    info!(owner = %body.owner, "{}", crate::i18n::t("runtime-claimed"));

    // Grant owner all capabilities in the live root ACL.
    grant_owners_in_acl(&state.acl, &new_owners).await;

    // Persist to config.yaml so the claim survives a restart.
    if let Some(ref path) = config_path {
        match persist_owners_to_config(path, &new_owners) {
            Ok(()) => info!("{}", crate::i18n::t("runtime-claim-persisted")),
            Err(e) => warn!(error = %e, "failed to persist owners to config.yaml"),
        }
    }

    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        serde_json::json!({"owners": new_owners, "status": "claimed"}).to_string(),
    )
        .into_response()
}

/// Write the `owner` key as a YAML sequence into the config file at `path`.
///
/// The file is read as a raw YAML mapping, the key is inserted or updated,
/// and the result is written back with mode 0600.
pub fn persist_owners_to_config(path: &std::path::Path, owners: &[String]) -> anyhow::Result<()> {
    let yaml_text = if path.exists() {
        std::fs::read_to_string(path)?
    } else {
        String::new()
    };

    let yaml_to_parse = if yaml_text.trim().is_empty() {
        "{}".to_string()
    } else {
        yaml_text
    };
    let mut config = ma_core::config::Config::from_yaml_str(&yaml_to_parse)?;

    config.config_path = Some(path.to_path_buf());

    let owners_val = serde_yaml::to_value(owners)?;
    config
        .extra
        .insert(serde_yaml::Value::String("owners".to_string()), owners_val);

    config.save()?;
    Ok(())
}

/// Grant `owner` all capabilities (`"*"`) in the shared root ACL.
///
/// Called both at startup (when owner is already configured) and when
/// `POST /claim` fires, so the owner can immediately use RPC even before
/// any manifest ACL has been published.
/// Grant every DID in `owners` all capabilities (`["*"]`) in the shared root ACL.
///
/// Called at startup (from known owners list) and when `POST /claim` fires.
pub async fn grant_owners_in_acl(acl: &SharedAcl, owners: &[String]) {
    use ma_core::CapabilityEntry;
    let wildcard: std::collections::BTreeSet<String> = std::iter::once("*".to_string()).collect();
    let mut map = acl.write().await;
    for owner in owners {
        map.insert(owner.clone(), CapabilityEntry::Allow(wildcard.clone()));
    }
}

/// Publish a minimal `RuntimeManifest` to Kubo with an owner-only root ACL
/// and `/grp/owners` set. Returns the resulting root CID.
///
/// Called once during `POST /claim` when no manifest exists yet.
pub async fn bootstrap_minimal_manifest(
    kubo_rpc_url: &str,
    owners: &[String],
) -> anyhow::Result<String> {
    use crate::entity::{IpldLink, RuntimeManifest};
    use anyhow::Context as _;
    use ma_core::{AclMap, CapabilityEntry};

    // Unclaimed system (no owners): open ACL so DID publishing works out of the box.
    // Once an owner claims the runtime, the manifest is rebuilt with an owner-only ACL.
    let wildcard: std::collections::BTreeSet<String> = std::iter::once("*".to_string()).collect();
    let mut acl_map = AclMap::new();
    if owners.is_empty() {
        acl_map.insert("*".to_string(), CapabilityEntry::Allow(wildcard.clone()));
    } else {
        for owner in owners {
            acl_map.insert(owner.clone(), CapabilityEntry::Allow(wildcard.clone()));
        }
    }

    let acl_cid = crate::kubo::dag_put(kubo_rpc_url, &acl_map)
        .await
        .context("dag_put owner-only ACL")?;

    let owners_cid = crate::kubo::dag_put(kubo_rpc_url, &owners.to_vec())
        .await
        .context("dag_put owners group")?;

    let mut grp = std::collections::HashMap::new();
    grp.insert("owners".to_string(), IpldLink::new(&owners_cid));

    let mut config = std::collections::BTreeMap::new();
    config.insert(
        "name".to_string(),
        serde_yaml::Value::String(crate::crud::config::DEFAULT_RUNTIME_NAME.to_string()),
    );
    config.insert(
        "description".to_string(),
        serde_yaml::Value::String(crate::crud::config::DEFAULT_RUNTIME_DESCRIPTION.to_string()),
    );

    let manifest = RuntimeManifest {
        acl: Some(IpldLink::new(&acl_cid)),
        grp,
        config,
        ..Default::default()
    };

    let root_cid = crate::kubo::dag_put(kubo_rpc_url, &manifest)
        .await
        .context("dag_put minimal manifest")?;

    crate::kubo::pin_add(kubo_rpc_url, &root_cid)
        .await
        .context("pinning minimal manifest")?;

    Ok(root_cid)
}

#[cfg(test)]
mod tests {
    use super::{
        default_config_string, kubo_block_get_ipld_path, normalize_zion_source, zion_asset_path,
        zion_source_trust_notice, ZION_CONFIG_KEY,
    };
    use crate::crud::config::DEFAULT_ZION_SOURCE;

    #[test]
    fn zion_default_source_is_hardcoded_ipns_path() {
        assert_eq!(
            default_config_string(ZION_CONFIG_KEY).as_deref(),
            Some(DEFAULT_ZION_SOURCE)
        );
        assert_eq!(
            normalize_zion_source(DEFAULT_ZION_SOURCE).as_deref(),
            Some(DEFAULT_ZION_SOURCE)
        );
        assert!(zion_source_trust_notice(Some(DEFAULT_ZION_SOURCE)).contains("trust root"));
        assert!(zion_source_trust_notice(Some("/ipns/user-controlled")).is_empty());
    }

    #[test]
    fn normalizes_zion_source_to_ipfs_or_ipns_path() {
        assert_eq!(
            normalize_zion_source("bafybeigdyrzt").as_deref(),
            Some("/ipfs/bafybeigdyrzt")
        );
        assert_eq!(
            normalize_zion_source("/ipfs/bafybeigdyrzt/").as_deref(),
            Some("/ipfs/bafybeigdyrzt")
        );
        assert_eq!(
            normalize_zion_source(
                "/ipns/k51qzi5uqu5dkw4n2093va9ydfnvex2rwfhy0jb0bb1p9giaoam465pdk1y726"
            )
            .as_deref(),
            Some("/ipns/k51qzi5uqu5dkw4n2093va9ydfnvex2rwfhy0jb0bb1p9giaoam465pdk1y726")
        );
        assert_eq!(normalize_zion_source("   "), None);
    }

    #[test]
    fn appends_zion_asset_path_to_source() {
        assert_eq!(
            zion_asset_path(
                "/ipns/k51qzi5uqu5dkw4n2093va9ydfnvex2rwfhy0jb0bb1p9giaoam465pdk1y726",
                "index.html"
            ),
            "/ipns/k51qzi5uqu5dkw4n2093va9ydfnvex2rwfhy0jb0bb1p9giaoam465pdk1y726/index.html"
        );
        assert_eq!(
            zion_asset_path("/ipfs/bafybeigdyrzt/", "style/zion.css"),
            "/ipfs/bafybeigdyrzt/style/zion.css"
        );
    }

    #[tokio::test]
    async fn block_get_fetches_ipfs_path_bytes() {
        let kubo = crate::testkubo::MockKubo::start().await;
        let bytes = vec![0xa1, 0x61, b'*', 0x81, 0x63, b'r', b'p', b'c'];
        let cid = kubo.add_bytes(bytes.clone()).await;
        let path = format!("/ipfs/{cid}");

        let fetched = kubo_block_get_ipld_path(kubo.url(), &path)
            .await
            .expect("block get should succeed")
            .expect("block should exist");

        assert_eq!(fetched, bytes);
    }
}
