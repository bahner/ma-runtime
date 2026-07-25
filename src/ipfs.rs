use anyhow::{anyhow, Context, Result};
use ma_core::ipfs::ipns_key_name_for_parts;
use ma_core::ipfs::IpfsDidPublisher;
use ma_core::ipfs_add;
use ma_core::{
    ipns_from_secret, resolve_endpoint_for_protocol, validate_identity_publish_message,
    validate_ipfs_request, Did, Document, Inbox, IpfsGatewayResolver, ReplayGuard, SigningKey,
    MESSAGE_TYPE_IDENTITY_PUBLISH_REQUEST, MESSAGE_TYPE_IPFS_REQUEST, MESSAGE_TYPE_RPC_REPLY,
};
use reqwest::multipart;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{info, warn};
use zeroize::Zeroizing;

use crate::acl::{check_full, AclMap, GroupCache, CAP_IDENTITY_PUBLISH, CAP_IPFS};
use crate::i18n;
use crate::rpc::RPC_PROTOCOL_ID;

/// Cache of sender DID base-id → their most-recently-seen Document.
/// Populated when a `DidDocumentPublish` request arrives; used to avoid
/// IPNS re-resolution when delivering the ipfs-store CID reply.
pub type DocCache = Arc<Mutex<HashMap<String, Document>>>;

/// All state owned by the optional IPFS publisher service.
pub struct IpfsServiceState {
    pub messages: Inbox<ma_core::Message>,
    pub publisher: IpfsDidPublisher,
    pub replay_guard: ReplayGuard,
    /// Recently-seen sender documents — avoids IPNS lookups for reply delivery.
    pub doc_cache: DocCache,
}

impl IpfsServiceState {
    pub fn new(messages: Inbox<ma_core::Message>, publisher: IpfsDidPublisher) -> Self {
        Self {
            messages,
            publisher,
            replay_guard: ReplayGuard::default(),
            doc_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub struct IpfsHandlerCtx<'a> {
    pub our_did: &'a str,
    pub signing_key: &'a SigningKey,
    /// Arc so reply-delivery tasks can be spawned without blocking the event loop.
    pub endpoint: Arc<dyn ma_core::MaEndpoint>,
    pub kubo_rpc_url: &'a str,
    pub publisher: &'a IpfsDidPublisher,
    pub resolver: Arc<IpfsGatewayResolver>,
    /// Shared document cache — populated on `DidDocumentPublish`, read on Store.
    pub doc_cache: DocCache,
    /// Named group cache — backs the `+<name>` principal syntax in the root ACL.
    pub group_cache: GroupCache,
}

pub async fn do_publish_own_document(
    kubo_url: String,
    runtime_slug: String,
    doc_cbor: Vec<u8>,
    ipns_secret_key: Vec<u8>,
    publish_lifetime_hours: u64,
) -> Result<()> {
    // Wrap in Zeroizing so the key bytes are cleared on return *and* on
    // async cancellation (e.g. if the 2-minute timeout fires and drops
    // the future before the explicit zeroize at the end could run).
    let ipns_secret_key = Zeroizing::new(ipns_secret_key);
    let publisher = IpfsDidPublisher::new(&kubo_url)?;
    publisher.wait_until_ready(10).await?;

    // Decode once so we can derive deterministic Kubo key alias from DID IPNS.
    let document = Document::decode(&doc_cbor)
        .map_err(|e| anyhow!("invalid own DID document dag-cbor: {e}"))?;
    let document_did = Did::try_from(document.id.as_str())
        .map_err(|e| anyhow!("invalid own DID '{}': {e}", document.id))?;
    let key_name = ipns_key_name_for_parts(&["runtime", &runtime_slug], &document_did.ipns);

    ensure_kubo_ipns_key(&kubo_url, &key_name, &document_did.ipns, &ipns_secret_key).await?;
    let cid = dag_put_cbor(&kubo_url, &doc_cbor).await?;
    name_publish(&kubo_url, &key_name, &cid, publish_lifetime_hours).await?;
    Ok(())
}

pub async fn resolve_runtime_root_cid_by_ipns_id(
    kubo_url: &str,
    ipns_id: &str,
) -> Result<Option<String>> {
    let key_id = list_keys(kubo_url).await?.into_iter().find_map(|(_, id)| {
        if id == ipns_id {
            Some(id)
        } else {
            None
        }
    });

    let Some(key_id) = key_id else {
        return Ok(None);
    };

    resolve_ipns_path(kubo_url, &key_id).await
}

/// Publish a runtime IPLD root CID to the runtime's dedicated IPNS.
///
/// Uses the extra `"runtime_ipns"` key from the `SecretBundle` — distinct from
/// `ipns_secret_key` which is reserved for the DID document.  Imports the key
/// into Kubo on first use (idempotent).
pub async fn publish_runtime_root_cid(
    kubo_url: &str,
    runtime_slug: &str,
    runtime_ipns_key: &[u8; 32],
    root_cid: &str,
    publish_lifetime_hours: u64,
) -> Result<String> {
    let runtime_ipns_id =
        ipns_from_secret(*runtime_ipns_key).context("failed to derive runtime IPNS id")?;
    let key_name = ipns_key_name_for_parts(&["runtime", runtime_slug, "runtime"], &runtime_ipns_id);
    ensure_kubo_ipns_key(kubo_url, &key_name, &runtime_ipns_id, runtime_ipns_key).await?;
    name_publish(kubo_url, &key_name, root_cid, publish_lifetime_hours).await
}

#[derive(Debug, Deserialize)]
struct DagPutCid {
    #[serde(rename = "/")]
    slash: String,
}

#[derive(Debug, Deserialize)]
struct DagPutResponse {
    #[serde(default, rename = "Cid")]
    cid_upper: Option<DagPutCid>,
    #[serde(default)]
    cid: Option<DagPutCid>,
}

#[derive(Debug, Deserialize)]
struct NamePublishResponse {
    #[serde(default, rename = "Value")]
    value_upper: String,
    #[serde(default, rename = "value")]
    value_lower: String,
}

#[derive(Debug, Deserialize)]
struct NameResolveResponse {
    #[serde(default, rename = "Path")]
    path_upper: String,
    #[serde(default, rename = "path")]
    path_lower: String,
}

#[derive(Debug, Deserialize)]
struct KeyListEntry {
    #[serde(default, rename = "Name")]
    name: String,
    #[serde(default, rename = "name")]
    name_lower: String,
    #[serde(default, rename = "Id")]
    id: String,
    #[serde(default, rename = "id")]
    id_lower: String,
}

#[derive(Debug, Deserialize)]
struct KeyListResponse {
    #[serde(default, rename = "Keys")]
    keys: Vec<KeyListEntry>,
}

#[derive(Debug, Deserialize)]
struct KeyImportResponse {
    #[serde(default, rename = "Id")]
    id_upper: String,
    #[serde(default, rename = "id")]
    id_lower: String,
}

async fn dag_put_cbor(kubo_url: &str, data: &[u8]) -> Result<String> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/dag/put");

    let part = multipart::Part::bytes(data.to_vec())
        .file_name("document.cbor")
        .mime_str("application/octet-stream")?;
    let form = multipart::Form::new().part("file", part);

    let body = reqwest::Client::new()
        .post(url)
        .query(&[
            ("store-codec", "dag-cbor"),
            ("input-codec", "dag-cbor"),
            ("pin", "true"),
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

async fn name_publish(
    kubo_url: &str,
    key_name: &str,
    cid: &str,
    publish_lifetime_hours: u64,
) -> Result<String> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/name/publish");
    let arg = format!(
        "/ipfs/{}",
        cid.trim_start_matches('/').trim_start_matches("ipfs/")
    );

    let lifetime = format!("{publish_lifetime_hours}h");
    let body = reqwest::Client::new()
        .post(url)
        .query(&[
            ("arg", arg.as_str()),
            ("key", key_name),
            ("allow-offline", "true"),
            ("lifetime", lifetime.as_str()),
            ("resolve", "false"),
            ("quieter", "true"),
        ])
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let parsed: NamePublishResponse = serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed parsing name/publish response: {e} body={body}"))?;
    let value = if parsed.value_upper.is_empty() {
        parsed.value_lower
    } else {
        parsed.value_upper
    };
    if value.is_empty() {
        return Err(anyhow!("missing value in name/publish response: {body}"));
    }
    Ok(value)
}

async fn resolve_ipns_path(kubo_url: &str, key_id: &str) -> Result<Option<String>> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/name/resolve");
    let arg = format!("/ipns/{key_id}");

    let body = reqwest::Client::new()
        .post(url)
        .query(&[("arg", arg.as_str()), ("recursive", "true")])
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let parsed: NameResolveResponse = serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed parsing name/resolve response: {e} body={body}"))?;
    let path = if parsed.path_upper.is_empty() {
        parsed.path_lower
    } else {
        parsed.path_upper
    };
    if path.is_empty() {
        return Ok(None);
    }

    let cid = path
        .trim()
        .strip_prefix("/ipfs/")
        .map_or_else(|| path.trim().to_string(), ToString::to_string);
    if cid.is_empty() {
        Ok(None)
    } else {
        Ok(Some(cid))
    }
}

async fn ensure_kubo_ipns_key(
    kubo_url: &str,
    key_name: &str,
    expected_ipns_id: &str,
    ipns_secret_key: &[u8],
) -> Result<()> {
    let existing = list_keys(kubo_url)
        .await?
        .into_iter()
        .find(|(name, _)| name == key_name);

    if let Some((_, id)) = existing {
        if id.trim() != expected_ipns_id {
            return Err(anyhow!(
                "existing key '{key_name}' has IPNS id '{id}' but expected '{expected_ipns_id}'"
            ));
        }
        return Ok(());
    }

    let raw_key: [u8; 32] = ipns_secret_key
        .try_into()
        .map_err(|_| anyhow!("ipns_secret_key must be 32 bytes"))?;
    let keypair = libp2p_identity::Keypair::ed25519_from_bytes(raw_key)
        .map_err(|e| anyhow!("invalid ipns key: {e}"))?;
    let protobuf_key = keypair
        .to_protobuf_encoding()
        .map_err(|e| anyhow!("failed to encode ipns key: {e}"))?;

    let imported_id = import_key(kubo_url, key_name, protobuf_key).await?;
    if imported_id.trim() != expected_ipns_id {
        return Err(anyhow!(
            "imported key IPNS id '{imported_id}' does not match expected '{expected_ipns_id}'"
        ));
    }
    Ok(())
}

async fn list_keys(kubo_url: &str) -> Result<Vec<(String, String)>> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/key/list");

    let body = reqwest::Client::new()
        .post(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let parsed: KeyListResponse = serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed parsing key/list response: {e} body={body}"))?;

    Ok(parsed
        .keys
        .into_iter()
        .filter_map(|k| {
            let name = if k.name.trim().is_empty() {
                k.name_lower.trim().to_string()
            } else {
                k.name.trim().to_string()
            };
            let id = if k.id.trim().is_empty() {
                k.id_lower.trim().to_string()
            } else {
                k.id.trim().to_string()
            };
            if name.is_empty() || id.is_empty() {
                None
            } else {
                Some((name, id))
            }
        })
        .collect())
}

async fn import_key(kubo_url: &str, key_name: &str, key_bytes: Vec<u8>) -> Result<String> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/key/import");

    let part = multipart::Part::bytes(key_bytes)
        .file_name("ipns.key")
        .mime_str("application/octet-stream")?;
    let form = multipart::Form::new().part("file", part);

    let body = reqwest::Client::new()
        .post(url)
        .query(&[
            ("arg", key_name),
            ("ipns-base", "base36"),
            ("allow-any-key-type", "true"),
        ])
        .multipart(form)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let parsed: KeyImportResponse = serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed parsing key/import response: {e} body={body}"))?;
    let id = if parsed.id_upper.trim().is_empty() {
        parsed.id_lower
    } else {
        parsed.id_upper
    };
    if id.trim().is_empty() {
        return Err(anyhow!("missing id in key/import response: {body}"));
    }
    Ok(id.trim().to_string())
}

/// Encode `[:ok, cid]` as CBOR bytes for an IPFS service reply.
fn encode_ok_cid_reply(cid: &str) -> Result<Vec<u8>> {
    let reply_atom: Vec<ciborium::Value> = vec![
        ciborium::Value::Text(":ok".to_string()),
        ciborium::Value::Text(cid.to_string()),
    ];
    let mut reply_bytes = Vec::new();
    ciborium::ser::into_writer(&ciborium::Value::Array(reply_atom), &mut reply_bytes)
        .context("failed to encode CBOR ok-cid reply")?;
    Ok(reply_bytes)
}

/// Build an RPC reply `Message` addressed to the sender's `#rpc` fragment.
///
/// Returns `(message, sender_did, rpc_did_url)`.
fn build_rpc_reply_message(
    ctx: &IpfsHandlerCtx<'_>,
    from: &str,
    in_reply_to: &str,
    payload: &[u8],
) -> Result<(ma_core::Message, Did, String)> {
    let sender = Did::try_from(from).with_context(|| format!("invalid sender DID: {from}"))?;
    let rpc_did_url = format!("did:ma:{}#rpc", sender.ipns);
    let mut reply = ma_core::Message::new(
        ctx.our_did,
        &rpc_did_url,
        MESSAGE_TYPE_RPC_REPLY,
        "application/cbor",
        payload,
        ctx.signing_key,
    )
    .context("failed to build reply message")?;
    reply.reply_to = Some(in_reply_to.to_string());
    Ok((reply, sender, rpc_did_url))
}

/// Extract the iroh endpoint ID for `protocol` from a document's `ma.services`.
fn endpoint_for_protocol_from_doc(doc: &Document, protocol: &str) -> Option<String> {
    let services = doc
        .ma
        .as_ref()
        .and_then(|ma| ma.get("services").ok().flatten())
        .and_then(|s| serde_json::to_value(s).ok());
    resolve_endpoint_for_protocol(services.as_ref(), protocol)
}

/// Open an outbox for `target`.  Prefers a cached document (avoids IPNS
/// re-resolution); falls back to the resolver if the cache misses or if the
/// cached endpoint is stale (connect times out or errors).
///
/// Takes individual Arc values so it can be called from spawned tasks without
/// needing a reference to the short-lived `IpfsHandlerCtx`.
pub(crate) async fn open_outbox_for_did(
    endpoint: &Arc<dyn ma_core::MaEndpoint>,
    resolver: &Arc<IpfsGatewayResolver>,
    doc_cache: &DocCache,
    target: &Did,
    protocol: &str,
) -> Result<ma_core::Outbox> {
    let target_base = target.base_id();
    let cached_doc = doc_cache.lock().await.get(&target_base).cloned();

    if let Some(ref doc) = cached_doc {
        if let Some(eid) = endpoint_for_protocol_from_doc(doc, protocol) {
            // Use a short deadline so a stale cached endpoint ID does not block
            // the handler indefinitely.
            match tokio::time::timeout(
                Duration::from_secs(5),
                endpoint.connect_outbox(doc, &eid, &target_base, protocol),
            )
            .await
            {
                Ok(Ok(outbox)) => return Ok(outbox),
                Ok(Err(e)) => {
                    warn!(error = %e, to = %target_base, protocol = %protocol, "cached outbox connect failed, falling back to IPNS");
                }
                Err(_elapsed) => {
                    warn!(to = %target_base, protocol = %protocol, "cached outbox connect timed out, falling back to IPNS");
                }
            }
        }
    }

    // IPNS fallback — also guarded by a timeout so a slow or unreachable
    // resolver does not block the handler indefinitely.
    tokio::time::timeout(
        Duration::from_secs(10),
        endpoint.outbox(resolver.as_ref(), &target_base, protocol),
    )
    .await
    .map_err(|_| anyhow::anyhow!("IPNS outbox resolve timed out for {target_base}"))?
    .map_err(anyhow::Error::from)
}

pub async fn handle_ipfs_message(
    message: &ma_core::Message,
    acl: &AclMap,
    ctx: &IpfsHandlerCtx<'_>,
    replay_guard: &mut ReplayGuard,
) -> Result<()> {
    let cap = match message.message_type.as_str() {
        MESSAGE_TYPE_IDENTITY_PUBLISH_REQUEST => CAP_IDENTITY_PUBLISH,
        MESSAGE_TYPE_IPFS_REQUEST => CAP_IPFS,
        other => {
            return Err(anyhow!(
                "unsupported message type on /ma/ipfs/0.0.1: {other}"
            ))
        }
    };

    let group_cache = ctx.group_cache.clone();
    check_full(acl, &message.from, &[cap], |key| {
        let group_cache = group_cache.clone();
        let name = key.strip_prefix('+').unwrap_or(key).to_string();
        async move {
            Ok(group_cache
                .read()
                .await
                .get(&name)
                .cloned()
                .unwrap_or_default())
        }
    })
    .await?;

    let headers = message.headers();
    replay_guard
        .check_and_insert(&headers)
        .context("replay or invalid headers")?;

    match message.message_type.as_str() {
        MESSAGE_TYPE_IDENTITY_PUBLISH_REQUEST => {
            let v = validate_identity_publish_message(message)
                .context("invalid identity-publish request")?;
            handle_did_document_publish(message, v, ctx).await
        }
        MESSAGE_TYPE_IPFS_REQUEST => {
            let v = validate_ipfs_request(message).context("invalid ipfs-store request")?;
            handle_ipfs_store(message, &v, ctx).await
        }
        other => Err(anyhow!(
            "unsupported message type on /ma/ipfs/0.0.1: {other}"
        )),
    }
}

async fn handle_did_document_publish(
    message: &ma_core::Message,
    v: ma_core::ValidatedIdentityPublish,
    ctx: &IpfsHandlerCtx<'_>,
) -> Result<()> {
    info!(from = %message.from, id = %message.id, "{}", i18n::t("did-publish-request-received"));

    let ma_core::ValidatedIdentityPublish {
        document_bytes,
        ipns_secret_key,
        document,
        document_did,
    } = v;

    // Cache the document NOW — before the slow Kubo publish — so any concurrent
    // IPFS-store reply tasks can resolve this sender's endpoint immediately.
    let sender_for_cache = ma_core::Did::try_from(message.from.as_str())
        .with_context(|| format!("invalid sender DID: {}", message.from))?;
    ctx.doc_cache
        .lock()
        .await
        .insert(sender_for_cache.base_id(), document.clone());

    let key = Zeroizing::new(ipns_secret_key);
    let cid = ctx
        .publisher
        .publish_document(&document_bytes, &key)
        .await
        .context("kubo DID publish failed")?
        .ok_or_else(|| anyhow!("publisher returned no CID"))?;
    info!(did = %document_did.id(), cid = %cid, "{}", i18n::t("document-published"));

    let reply_bytes = encode_ok_cid_reply(&cid)?;
    let (reply, sender, rpc_did_url) =
        build_rpc_reply_message(ctx, &message.from, &message.id, &reply_bytes)?;

    // Spawn reply delivery so a slow or stale iroh connection never blocks
    // the main event loop (and therefore never prevents Ctrl-C from firing).
    match endpoint_for_protocol_from_doc(&document, RPC_PROTOCOL_ID) {
        Some(eid) => {
            let endpoint = Arc::clone(&ctx.endpoint);
            let document = document.clone();
            let sender_base = sender.base_id();
            tokio::spawn(async move {
                match tokio::time::timeout(
                    Duration::from_secs(15),
                    endpoint.connect_outbox(&document, &eid, &sender_base, RPC_PROTOCOL_ID),
                )
                .await
                {
                    Ok(Ok(mut outbox)) => {
                        match tokio::time::timeout(Duration::from_secs(15), outbox.send(&reply))
                            .await
                        {
                            Ok(Ok(())) => {
                                info!(to = %rpc_did_url, cid = %cid, "{}", i18n::t("did-publish-cid-reply-sent"));
                            }
                            Ok(Err(e)) => {
                                warn!(error = %e, to = %rpc_did_url, "ipfs-publish reply send failed");
                            }
                            Err(_) => warn!(to = %rpc_did_url, "ipfs-publish reply send timed out"),
                        }
                    }
                    Ok(Err(err)) => {
                        warn!(error = %err, to = %rpc_did_url, "{}", i18n::t("did-publish-resolve-failed"));
                    }
                    Err(_) => {
                        warn!(to = %rpc_did_url, "ipfs-publish connect timed out");
                    }
                }
            });
        }
        None => {
            warn!(to = %rpc_did_url, "{}", i18n::t("did-publish-resolve-failed"));
        }
    }

    Ok(())
}

async fn handle_ipfs_store(
    orig_message: &ma_core::Message,
    v: &ma_core::ValidatedIpfsStore,
    ctx: &IpfsHandlerCtx<'_>,
) -> Result<()> {
    info!(from = %orig_message.from, id = %orig_message.id, "{}", i18n::t("ipfs-store-request-received"));

    // DAG-CBOR content: decode back to a value and use the dag-json input path
    // in Kubo (store-codec=dag-cbor).  Sending raw CBOR bytes with
    // input-codec=dag-cbor produces a raw block (bafkrei…) instead of a
    // proper dag-cbor node (bafy…), so we go through dag-json which is the
    // reliable path.
    let cid = if v.content_type == "application/vnd.ipld.dag-cbor" {
        let val: serde_json::Value = ciborium::de::from_reader(v.content.as_slice())
            .context("failed to decode incoming DAG-CBOR")?;
        crate::kubo::dag_put(ctx.kubo_rpc_url, &val)
            .await
            .context("dag put failed")?
    } else {
        ipfs_add(ctx.kubo_rpc_url, v.content.clone())
            .await
            .context("ipfs add failed")?
    };

    info!(cid = %cid, from = %orig_message.from, "{}", i18n::t("ipfs-stored"));

    let reply_bytes = encode_ok_cid_reply(&cid)?;
    let (reply, sender, rpc_did_url) =
        build_rpc_reply_message(ctx, &orig_message.from, &orig_message.id, &reply_bytes)?;

    // Spawn reply delivery so a slow or unreachable iroh connection never
    // blocks the main event loop (and therefore never prevents Ctrl-C).
    let endpoint = Arc::clone(&ctx.endpoint);
    let resolver = Arc::clone(&ctx.resolver);
    let doc_cache = Arc::clone(&ctx.doc_cache);
    tokio::spawn(async move {
        match open_outbox_for_did(&endpoint, &resolver, &doc_cache, &sender, RPC_PROTOCOL_ID).await
        {
            Ok(mut outbox) => {
                match tokio::time::timeout(Duration::from_secs(15), outbox.send(&reply)).await {
                    Ok(Ok(())) => {
                        info!(to = %rpc_did_url, cid = %cid, "{}", i18n::t("ipfs-store-cid-reply-sent"));
                    }
                    Ok(Err(e)) => {
                        warn!(error = %e, to = %rpc_did_url, "ipfs-store reply send failed");
                    }
                    Err(_) => warn!(to = %rpc_did_url, "ipfs-store reply send timed out"),
                }
            }
            Err(err) => {
                warn!(error = %err, to = %rpc_did_url, "{}", i18n::t("ipfs-store-resolve-failed"));
            }
        }
    });

    Ok(())
}
