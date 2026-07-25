use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    sync::Arc,
};

use anyhow::{anyhow, Context, Result};
use ciborium::Value as CborValue;
use ma_core::{
    Did, DidDocumentResolver, IpfsGatewayResolver, Ipld, CONTENT_TYPE_TERM, CONTENT_TYPE_TERM_CBOR,
    CONTENT_TYPE_TERM_YAML,
};
use tracing::{info, warn};

use crate::entity::{EntityNode, KindNode, RuntimeManifest};

use super::CrudHandlerCtx;

// ── Path helpers ───────────────────────────────────────────────────────────────

/// Parse `/ns/seg1/seg2` → `("ns", ["seg1", "seg2"])`.
pub(super) fn parse_path(path: &str) -> Result<(&str, Vec<String>)> {
    let body = path
        .strip_prefix('/')
        .ok_or_else(|| anyhow!("CRUD path must start with '/' — got: {path}"))?;
    let (ns, rest_str) = body.split_once('/').unwrap_or((body, ""));
    if ns.is_empty() {
        return Err(anyhow!("CRUD path has no handler segment: {path}"));
    }
    let segs: Vec<String> = rest_str
        .split('/')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    Ok((ns, segs))
}

/// Decoded CRUD operation from a single incoming message payload.
pub(super) enum CrudOp {
    /// `[".path"]`
    Get(String),
    /// `[".path", value]` — value is a CBOR scalar or `/ipfs/…`, `/ipns/…` reference
    Set(String, CborValue),
    /// `[".path", ""]` — empty string value means delete
    Delete(String),
}

/// Decode a `application/vnd.ma.crud.request` payload.
///
/// - `[".path"]`          → GET
/// - `[".path", ""]`      → DELETE (empty string = delete)
/// - `[".path", value]`   → SET (value is a CBOR scalar or `/ipfs/…`, `/ipns/…` reference)
pub(super) fn decode_crud_payload(content: &[u8]) -> Result<CrudOp> {
    let val: CborValue =
        ciborium::de::from_reader(content).context("invalid CBOR in CRUD payload")?;
    let CborValue::Array(items) = val else {
        return Err(anyhow!("CRUD payload must be a CBOR array"));
    };
    match items.len() {
        1 => {
            let CborValue::Text(path) = items.into_iter().next().unwrap() else {
                return Err(anyhow!("CRUD get: path must be a text string"));
            };
            Ok(CrudOp::Get(path))
        }
        2 => {
            let mut it = items.into_iter();
            let first = it.next().unwrap();
            let second = it.next().unwrap();
            let CborValue::Text(path) = first else {
                return Err(anyhow!("CRUD payload: path must be a text string"));
            };
            match second {
                CborValue::Text(s) if s.is_empty() => Ok(CrudOp::Delete(path)),
                value => Ok(CrudOp::Set(path, value)),
            }
        }
        n => Err(anyhow!(
            "CRUD payload must be a 1 or 2-element CBOR array, got {n}"
        )),
    }
}

/// True if `s` is an explicit `/ipfs/<cid>` or `/ipns/<key>` reference.
///
/// Explicit prefixes are required wherever a CID/IPNS reference is
/// expected — bare CID strings are treated as plain text and never
/// auto-detected.
pub(super) fn is_ipfs_ref(s: &str) -> bool {
    s.starts_with("/ipfs/") || s.starts_with("/ipns/")
}

/// Resolve an explicit `/ipfs/<cid>` or `/ipns/<key>` reference to a bare
/// CID via Kubo's `dag/resolve` API. Returns `None` if `s` is not an
/// `/ipfs/` or `/ipns/` reference (see [`is_ipfs_ref`]).
pub(super) async fn resolve_ipfs_ref(kubo_url: &str, s: &str) -> Result<Option<String>> {
    if !is_ipfs_ref(s) {
        return Ok(None);
    }
    Ok(Some(crate::kubo::dag_resolve(kubo_url, s).await?))
}

// ── Manifest helpers ───────────────────────────────────────────────────────────

pub(super) async fn with_manifest_crud<F>(ctx: &CrudHandlerCtx, f: F) -> Result<String>
where
    F: FnOnce(&mut RuntimeManifest) -> Result<()>,
{
    // All manifest mutations are serialised through the writer, which owns the
    // authoritative root CID — no read-modify-write race on a stale base.
    let new_cid = ctx.manifest_writer.mutate(f).await?;
    update_stats_entities(ctx).await;
    Ok(new_cid)
}

pub(super) async fn current_root_cid(ctx: &CrudHandlerCtx) -> Result<String> {
    ctx.stats
        .read()
        .await
        .root_cid
        .clone()
        .ok_or_else(|| anyhow!("no manifest root CID available"))
}

/// Fetch and deserialise the current `RuntimeManifest` from IPFS.
pub(super) async fn load_manifest(ctx: &CrudHandlerCtx) -> Result<RuntimeManifest> {
    let root_cid = current_root_cid(ctx).await?;
    crate::kubo::dag_get(&ctx.kubo_rpc_url, &root_cid).await
}

pub(super) async fn runtime_config_snapshot(
    ctx: &CrudHandlerCtx,
) -> Result<BTreeMap<String, String>> {
    let manifest = load_manifest(ctx).await?;
    let cfg = ctx.shared_config.read().await;
    Ok(super::config::public_plugin_config(&manifest, &cfg))
}

/// Fetch a group's flat `Vec<String>` DID-list document by `cid`, insert it
/// into `group_cache` under `name`. Logs success or failure; non-fatal either
/// way — a group that fails to load simply resolves to no members.
pub(super) async fn group_cache_update(ctx: &CrudHandlerCtx, name: &str, cid: &str) {
    match crate::kubo::dag_get::<Vec<String>>(&ctx.kubo_rpc_url, cid).await {
        Ok(members) => {
            ctx.group_cache
                .write()
                .await
                .insert(name.to_string(), members);
            info!(name = %name, %cid, "group loaded into cache");
        }
        Err(e) => {
            warn!(name = %name, %cid, error = %e, "failed to load group into cache");
        }
    }
}

async fn load_raw_kind_nodes(
    ctx: &CrudHandlerCtx,
    manifest: &RuntimeManifest,
) -> HashMap<String, KindNode> {
    let mut raw_kinds = HashMap::new();
    for (protocol, link) in manifest.kinds.iter_protocols() {
        match crate::kubo::dag_get::<KindNode>(&ctx.kubo_rpc_url, &link.cid).await {
            Ok(kind) => {
                raw_kinds.insert(protocol, kind);
            }
            Err(e) => {
                warn!(protocol = %protocol, cid = %link.cid, error = %e, "failed to fetch kind node for dependency reload");
            }
        }
    }
    raw_kinds
}

fn kind_depends_on(
    protocol: &str,
    updated_protocol: &str,
    raw_kinds: &HashMap<String, KindNode>,
) -> bool {
    const MAX_DEPTH: usize = 8;
    if protocol == updated_protocol {
        return true;
    }

    let mut seen = BTreeSet::new();
    let mut current = raw_kinds
        .get(protocol)
        .and_then(|kind| kind.extends.as_deref());
    for _ in 0..MAX_DEPTH {
        let Some(base_protocol) = current else {
            return false;
        };
        if base_protocol == updated_protocol {
            return true;
        }
        if !seen.insert(base_protocol.to_string()) {
            warn!(protocol = %protocol, base = %base_protocol, "kind extends cycle while computing dependency reload set");
            return false;
        }
        current = raw_kinds
            .get(base_protocol)
            .and_then(|kind| kind.extends.as_deref());
    }

    warn!(protocol = %protocol, updated = %updated_protocol, "kind extends chain exceeded reload dependency depth");
    false
}

pub(super) fn affected_kind_protocols(
    updated_protocol: &str,
    raw_kinds: &HashMap<String, KindNode>,
) -> BTreeSet<String> {
    let mut affected = BTreeSet::from([updated_protocol.to_string()]);
    affected.extend(
        raw_kinds
            .keys()
            .filter(|protocol| kind_depends_on(protocol, updated_protocol, raw_kinds))
            .cloned(),
    );
    affected
}

async fn hydrate_affected_kind_registry(
    ctx: &CrudHandlerCtx,
    manifest: &RuntimeManifest,
    raw_kinds: &HashMap<String, KindNode>,
    affected: &BTreeSet<String>,
) {
    for protocol in affected {
        let Some(raw_kind) = raw_kinds.get(protocol).cloned() else {
            warn!(protocol = %protocol, "affected kind missing from raw registry; cannot hydrate reload dependency");
            continue;
        };
        let resolved = if raw_kind.extends.is_some() {
            match crate::entity::resolve_kind_extends(&ctx.kubo_rpc_url, manifest, raw_kind).await {
                Ok(kind) => kind,
                Err(e) => {
                    warn!(protocol = %protocol, error = %e, "failed to resolve kind dependency for reload");
                    continue;
                }
            }
        } else {
            raw_kind
        };
        ctx.kind_registry
            .write()
            .await
            .insert(protocol.clone(), Arc::new(resolved));
    }
}

pub(super) async fn spawn_kind_dependency_reloads(
    updated_protocol: &str,
    ctx: &CrudHandlerCtx,
    runtime_config: BTreeMap<String, String>,
) -> Result<usize> {
    let manifest = load_manifest(ctx).await?;
    let raw_kinds = load_raw_kind_nodes(ctx, &manifest).await;
    let affected = affected_kind_protocols(updated_protocol, &raw_kinds);
    hydrate_affected_kind_registry(ctx, &manifest, &raw_kinds, &affected).await;
    let mut reload_count = 0usize;

    for (name, link) in manifest.entities {
        let entity_node: EntityNode = match crate::kubo::dag_get(&ctx.kubo_rpc_url, &link.cid).await
        {
            Ok(entity) => entity,
            Err(e) => {
                warn!(name = %name, cid = %link.cid, error = %e, "failed to fetch entity node for dependency reload");
                continue;
            }
        };
        if !affected.contains(&entity_node.kind) {
            continue;
        }
        spawn_entity_reload(
            name,
            entity_node,
            ctx.kind_registry.clone(),
            ctx.stats.clone(),
            Arc::clone(&ctx.kubo_rpc_url),
            Arc::clone(&ctx.our_did),
            ctx.envelope_tx.clone(),
            ctx.entity_registry.clone(),
            ctx.manifest_writer.clone(),
            runtime_config.clone(),
        );
        reload_count += 1;
    }

    Ok(reload_count)
}

/// Spawn an independent task that loads a plugin from `entity_node` and inserts
/// it into the entity registry (replacing any existing version).
///
/// Returns immediately — the reload happens asynchronously so the CRUD event
/// loop is never blocked by WASM fetching, instantiation, or `init()`.
///
/// Mirrors `bootstrap::load_entities`'s lifecycle-persistence step: if the
/// load transitions `lifecycle` (typically `new` → `running` on first
/// genesis), the updated `EntityNode` is republished to IPFS and the
/// manifest is updated to point at the new CID — otherwise a later daemon
/// restart would re-read the stale `lifecycle: new` node and incorrectly
/// re-fire the `:init` signal a second time.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(super) fn spawn_entity_reload(
    name: String,
    entity_node: EntityNode,
    kind_registry: crate::entity::KindRegistry,
    stats: crate::status::SharedStats,
    kubo_rpc_url: Arc<str>,
    our_did: Arc<str>,
    envelope_tx: tokio::sync::mpsc::UnboundedSender<(String, crate::entity::SendEnvelope)>,
    entity_registry: crate::plugin::EntityRegistry,
    manifest_writer: crate::manifest::ManifestWriter,
    runtime_config: std::collections::BTreeMap<String, String>,
) {
    tokio::spawn(async move {
        // Prefer the hydrated in-memory kind registry, with a manifest/IPFS
        // fallback for stale or externally-mutated roots.
        let kind_node: Arc<KindNode> = {
            let registry = kind_registry.read().await;
            if let Some(k) = registry.get(&entity_node.kind).cloned() {
                k
            } else {
                drop(registry);
                let root_cid = stats.read().await.root_cid.clone();
                let Some(root_cid) = root_cid else {
                    warn!(name = %name, kind = %entity_node.kind, "no root CID available; cannot reload entity");
                    return;
                };
                let manifest: crate::entity::RuntimeManifest = match crate::kubo::dag_get(
                    &kubo_rpc_url,
                    &root_cid,
                )
                .await
                {
                    Ok(m) => m,
                    Err(e) => {
                        warn!(name = %name, kind = %entity_node.kind, error = %e, "failed to load manifest for kind lookup");
                        return;
                    }
                };
                let kind_link = if let Some(l) = manifest.kinds.get_protocol(&entity_node.kind) {
                    l.clone()
                } else {
                    warn!(name = %name, kind = %entity_node.kind, "kind not in manifest; cannot reload entity");
                    return;
                };
                let raw_kind: KindNode = match crate::kubo::dag_get(&kubo_rpc_url, &kind_link.cid)
                    .await
                {
                    Ok(k) => k,
                    Err(e) => {
                        warn!(name = %name, kind = %entity_node.kind, error = %e, "failed to fetch kind node; cannot reload entity");
                        return;
                    }
                };
                let resolved = if raw_kind.extends.is_some() {
                    match crate::entity::resolve_kind_extends(&kubo_rpc_url, &manifest, raw_kind)
                        .await
                    {
                        Ok(k) => k,
                        Err(e) => {
                            warn!(name = %name, kind = %entity_node.kind, error = %e, "failed to resolve kind extends chain; cannot reload entity");
                            return;
                        }
                    }
                } else {
                    raw_kind
                };
                Arc::new(resolved)
            }
        };

        let (iroh_node_id, started_at) = {
            let s = stats.read().await;
            (s.endpoint_id.clone(), s.started_at)
        };

        let init_payload = entity_node.init.as_ref().map(|s| s.as_bytes().to_vec());
        match crate::plugin::EntityPlugin::load(
            name.clone(),
            &entity_node,
            &kind_node,
            &our_did,
            &kubo_rpc_url,
            envelope_tx,
            entity_registry.clone(),
            &iroh_node_id,
            started_at,
            runtime_config,
            init_payload, // EntityNode.init (§ genesis-via-CRUD), only fires if genesis
        )
        .await
        {
            Ok((ep, lifecycle)) => {
                let mut updated_node = entity_node.clone();
                if let Ok(Some(cid)) = ep.trigger_save(&kubo_rpc_url).await {
                    updated_node.state = Some(crate::entity::IpldLink::new(cid));
                }
                entity_registry
                    .write()
                    .await
                    .insert(name.clone(), Arc::new(ep));
                info!(name = %name, lifecycle = %lifecycle, "{}", crate::i18n::t("entity-reloaded"));
                // Persist lifecycle/state changes to IPFS, exactly like
                // bootstrap::load_entities does at startup. Otherwise a later
                // daemon restart re-reads stale state and/or `initialised`.
                let state_changed = entity_node.state.as_ref().map(|l| l.cid.as_str())
                    != updated_node.state.as_ref().map(|l| l.cid.as_str());
                if lifecycle == crate::entity::Lifecycle::Running
                    && (!entity_node.initialised || state_changed)
                {
                    let mut updated = updated_node;
                    if !entity_node.initialised {
                        updated.initialised = true;
                    }
                    match crate::kubo::dag_put(&kubo_rpc_url, &updated).await {
                        Ok(new_cid) => {
                            let name_for_mutation = name.clone();
                            let new_cid_for_mutation = new_cid.clone();
                            match manifest_writer
                                .mutate(move |m| {
                                    m.entities.insert(
                                        name_for_mutation,
                                        crate::entity::IpldLink::new(new_cid_for_mutation),
                                    );
                                    Ok(())
                                })
                                .await
                            {
                                Ok(root_cid) => {
                                    info!(name = %name, cid = %new_cid, root_cid = %root_cid, "updated entity lifecycle in manifest");
                                }
                                Err(e) => {
                                    warn!(name = %name, error = %e, "failed to update manifest with new entity lifecycle CID");
                                }
                            }
                        }
                        Err(e) => {
                            warn!(name = %name, error = %e, "failed to persist updated entity lifecycle to IPFS");
                        }
                    }
                }
            }
            Err(e) => {
                warn!(
                    name = %name,
                    error = %e,
                    "{}",
                    crate::i18n::t("entity-load-failed")
                );
            }
        }
    });
}

async fn update_stats_entities(ctx: &CrudHandlerCtx) {
    let names: Vec<String> = ctx.entity_registry.read().await.keys().cloned().collect();
    ctx.stats.write().await.entity_names = names;
}

// ── i18n helpers ───────────────────────────────────────────────────────────────

/// Resolve caller's DID document and extract their preferred language.
/// Falls back to the runtime's own language on any error.
pub(super) async fn caller_lang(from: &str, resolver: &IpfsGatewayResolver) -> String {
    if let Ok(doc) = resolver.resolve(from).await {
        if let Some(Ipld::Map(ma)) = &doc.ma {
            if let Some(Ipld::String(lang)) = ma.get("lang") {
                if crate::i18n::has_lang(lang) {
                    return lang.clone();
                }
            }
        }
    }
    crate::i18n::runtime_lang()
}

/// Send a CRUD error reply with a message localised to the caller's language.
pub(super) async fn send_crud_i18n_error(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
    key: &str,
) -> Result<()> {
    let lang = caller_lang(&incoming.from, ctx.resolver.as_ref()).await;
    send_crud_error(incoming, reply_type, ctx, &crate::i18n::t_lang(&lang, key)).await
}

/// Like [`send_crud_i18n_error`] but substitutes `%name%` placeholders in the
/// translated message before sending.
pub(super) async fn send_crud_i18n_errorf(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
    key: &str,
    args: &[(&str, &str)],
) -> Result<()> {
    let lang = caller_lang(&incoming.from, ctx.resolver.as_ref()).await;
    send_crud_error(
        incoming,
        reply_type,
        ctx,
        &crate::i18n::tf_lang(&lang, key, args),
    )
    .await
}

// ── Reply helpers ──────────────────────────────────────────────────────────────

pub(super) async fn send_crud_ok(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(&CborValue::Text(":ok".to_string()), &mut out)
        .context("encoding :ok")?;
    send_crud_reply(incoming, reply_type, ctx, &out).await
}

pub(super) async fn send_crud_ok_path(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
    path: &str,
) -> Result<()> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(
        &CborValue::Array(vec![
            CborValue::Text(":ok".to_string()),
            CborValue::Text(path.to_string()),
        ]),
        &mut out,
    )
    .context("encoding [:ok, path]")?;
    send_crud_reply(incoming, reply_type, ctx, &out).await
}

pub(super) async fn send_crud_ok_cid(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
    cid: &str,
) -> Result<()> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(
        &CborValue::Array(vec![
            CborValue::Text(":ok".to_string()),
            CborValue::Text(cid.to_string()),
        ]),
        &mut out,
    )
    .context("encoding [:ok, cid]")?;
    send_crud_reply(incoming, reply_type, ctx, &out).await
}

pub(super) async fn send_crud_error(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
    reason: &str,
) -> Result<()> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(
        &CborValue::Array(vec![
            CborValue::Text(":error".to_string()),
            CborValue::Text(reason.to_string()),
        ]),
        &mut out,
    )
    .context("encoding error reply")?;
    send_crud_reply(incoming, reply_type, ctx, &out).await
}

pub(super) async fn send_crud_reply_cbor<T: serde::Serialize + Sync>(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
    value: &T,
) -> Result<()> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(value, &mut out).context("encoding CBOR reply")?;
    send_crud_reply(incoming, reply_type, ctx, &out).await
}

/// Send a GET data reply whose payload is a raw CBOR-serialised struct
/// (e.g. `EntityNode`).  Uses `CONTENT_TYPE_TERM_CBOR` so the receiver
/// knows it must decode CBOR to display the value.
pub(super) async fn send_crud_data_cbor<T: serde::Serialize + Sync>(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
    value: &T,
) -> Result<()> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(value, &mut out).context("encoding CBOR data reply")?;
    send_crud_reply_raw(incoming, reply_type, ctx, CONTENT_TYPE_TERM_CBOR, &out).await
}

/// Send a GET data reply whose payload is an inline YAML string.
///
/// Wraps in `[":ok", yaml_str]` CBOR term with `content_type = "text/yaml"`.
/// The receiver unwraps the `:ok` tuple and uses `content_type` to know
/// the payload is YAML — message-type and content-type are kept separate.
pub(super) async fn send_crud_ok_yaml<T: serde::Serialize + Sync>(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
    value: &T,
) -> Result<()> {
    let yaml_str = serde_yaml::to_string(value).context("encoding YAML reply")?;
    let mut out = Vec::new();
    ciborium::ser::into_writer(
        &CborValue::Array(vec![
            CborValue::Text(":ok".into()),
            CborValue::Text(yaml_str),
        ]),
        &mut out,
    )
    .context("encoding [:ok, yaml] CBOR array")?;
    send_crud_reply_raw(incoming, reply_type, ctx, "text/yaml", &out).await
}

/// Send a GET data reply whose payload is an inline YAML string (encoded
/// as a CBOR text value).  Uses `CONTENT_TYPE_TERM_YAML` so the receiver
/// can use it directly as editor content without further decoding.
pub(super) async fn send_crud_data_yaml<T: serde::Serialize + Sync>(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
    value: &T,
) -> Result<()> {
    let yaml_str = serde_yaml::to_string(value).context("encoding YAML reply")?;
    let mut out = Vec::new();
    ciborium::ser::into_writer(&CborValue::Text(yaml_str), &mut out)
        .context("encoding YAML string as CBOR text")?;
    send_crud_reply_raw(incoming, reply_type, ctx, CONTENT_TYPE_TERM_YAML, &out).await
}

pub(super) async fn send_crud_reply(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
    content: &[u8],
) -> Result<()> {
    send_crud_reply_raw(incoming, reply_type, ctx, CONTENT_TYPE_TERM, content).await
}

async fn send_crud_reply_raw(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
    content_type: &str,
    content: &[u8],
) -> Result<()> {
    let sender = Did::try_from(incoming.from.as_str())
        .with_context(|| format!("invalid sender DID: {}", incoming.from))?;

    let mut reply = ma_core::Message::new(
        ctx.our_did.as_ref(),
        &incoming.from,
        reply_type,
        content_type,
        content,
        ctx.signing_key.as_ref(),
    )
    .context("failed to build CRUD reply")?;
    reply.reply_to = Some(incoming.id.clone());

    match ctx
        .endpoint
        .outbox(
            ctx.resolver.as_ref(),
            &sender.base_id(),
            ma_core::CRUD_PROTOCOL_ID,
        )
        .await
    {
        Ok(mut outbox) => {
            outbox
                .send(&reply)
                .await
                .context("CRUD reply send failed")?;
            info!(to = %incoming.from, reply_to = %incoming.id, "CRUD reply sent");
        }
        Err(err) => {
            warn!(error = %err, to = %incoming.from, "CRUD reply delivery failed");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{affected_kind_protocols, decode_crud_payload, is_ipfs_ref, parse_path, CrudOp};
    use ciborium::Value as CborValue;
    use std::collections::{BTreeMap, HashMap};

    use crate::entity::{Evaluator, KindNode};

    fn cbor(v: &CborValue) -> Vec<u8> {
        let mut buf = Vec::new();
        ciborium::ser::into_writer(v, &mut buf).unwrap();
        buf
    }

    fn kind(protocol: &str, extends: Option<&str>) -> KindNode {
        KindNode {
            protocol: protocol.to_string(),
            cid: None,
            kind_type: Evaluator::Extism,
            behaviour: None,
            behaviour_chain: Vec::new(),
            host_functions: vec![],
            attributes: BTreeMap::new(),
            extends: extends.map(str::to_string),
        }
    }

    fn raw_kinds(kinds: Vec<KindNode>) -> HashMap<String, KindNode> {
        kinds
            .into_iter()
            .map(|kind| (kind.protocol.clone(), kind))
            .collect()
    }

    #[test]
    fn parse_path_splits_namespace_and_segments() {
        let (ns, segs) = parse_path("/entities/rms/acl").unwrap();
        assert_eq!(ns, "entities");
        assert_eq!(segs, vec!["rms", "acl"]);
    }

    #[test]
    fn parse_path_bare_namespace_has_no_segments() {
        let (ns, segs) = parse_path("/entities").unwrap();
        assert_eq!(ns, "entities");
        assert!(segs.is_empty());
    }

    #[test]
    fn parse_path_requires_leading_slash() {
        assert!(parse_path("entities/rms").is_err());
    }

    #[test]
    fn parse_path_rejects_empty_namespace() {
        assert!(parse_path("//rms").is_err());
    }

    #[test]
    fn parse_path_ignores_double_and_trailing_slashes() {
        let (ns, segs) = parse_path("/entities//rms/").unwrap();
        assert_eq!(ns, "entities");
        assert_eq!(segs, vec!["rms"]);
    }

    #[test]
    fn decode_crud_get_on_single_element() {
        let payload = cbor(&CborValue::Array(vec![CborValue::Text("/entities".into())]));
        assert!(
            matches!(decode_crud_payload(&payload).unwrap(), CrudOp::Get(p) if p == "/entities")
        );
    }

    #[test]
    fn decode_crud_delete_on_empty_string() {
        let payload = cbor(&CborValue::Array(vec![
            CborValue::Text("/entities/rms".into()),
            CborValue::Text(String::new()),
        ]));
        assert!(
            matches!(decode_crud_payload(&payload).unwrap(), CrudOp::Delete(p) if p == "/entities/rms")
        );
    }

    #[test]
    fn decode_crud_set_on_value() {
        let payload = cbor(&CborValue::Array(vec![
            CborValue::Text("/config/k".into()),
            CborValue::Text("v".into()),
        ]));
        assert!(
            matches!(decode_crud_payload(&payload).unwrap(), CrudOp::Set(p, _) if p == "/config/k")
        );
    }

    #[test]
    fn decode_crud_rejects_non_array() {
        let payload = cbor(&CborValue::Text("nope".into()));
        assert!(decode_crud_payload(&payload).is_err());
    }

    #[test]
    fn decode_crud_rejects_wrong_arity() {
        let payload = cbor(&CborValue::Array(vec![
            CborValue::Text("a".into()),
            CborValue::Text("b".into()),
            CborValue::Text("c".into()),
        ]));
        assert!(decode_crud_payload(&payload).is_err());
    }

    #[test]
    fn is_ipfs_ref_accepts_ipfs_and_ipns() {
        assert!(is_ipfs_ref("/ipfs/bafy123"));
        assert!(is_ipfs_ref("/ipns/k51q123"));
    }

    #[test]
    fn is_ipfs_ref_rejects_bare_cid_and_other_paths() {
        assert!(!is_ipfs_ref("bafy123"));
        assert!(!is_ipfs_ref("/ipld/bafy123"));
        assert!(!is_ipfs_ref("plain text"));
    }

    #[test]
    fn affected_kind_protocols_includes_descendants_only() {
        let raw = raw_kinds(vec![
            kind("/ma/root/0.0.1", None),
            kind("/ma/room/0.0.1", Some("/ma/root/0.0.1")),
            kind("/ma/fancy-room/0.0.1", Some("/ma/room/0.0.1")),
            kind("/ma/avatar/0.0.1", None),
        ]);

        let affected = affected_kind_protocols("/ma/room/0.0.1", &raw);

        assert!(affected.contains("/ma/room/0.0.1"));
        assert!(affected.contains("/ma/fancy-room/0.0.1"));
        assert!(!affected.contains("/ma/root/0.0.1"));
        assert!(!affected.contains("/ma/avatar/0.0.1"));
    }

    #[test]
    fn affected_kind_protocols_root_update_includes_whole_subtree() {
        let raw = raw_kinds(vec![
            kind("/ma/root/0.0.1", None),
            kind("/ma/room/0.0.1", Some("/ma/root/0.0.1")),
            kind("/ma/fancy-room/0.0.1", Some("/ma/room/0.0.1")),
            kind("/ma/avatar/0.0.1", None),
        ]);

        let affected = affected_kind_protocols("/ma/root/0.0.1", &raw);

        assert!(affected.contains("/ma/root/0.0.1"));
        assert!(affected.contains("/ma/room/0.0.1"));
        assert!(affected.contains("/ma/fancy-room/0.0.1"));
        assert!(!affected.contains("/ma/avatar/0.0.1"));
    }

    #[test]
    fn affected_kind_protocols_skips_malformed_branches() {
        let raw = raw_kinds(vec![
            kind("/ma/root/0.0.1", None),
            kind("/ma/room/0.0.1", Some("/ma/root/0.0.1")),
            kind("/ma/missing-base/0.0.1", Some("/ma/nope/0.0.1")),
            kind("/ma/cycle-a/0.0.1", Some("/ma/cycle-b/0.0.1")),
            kind("/ma/cycle-b/0.0.1", Some("/ma/cycle-a/0.0.1")),
        ]);

        let affected = affected_kind_protocols("/ma/root/0.0.1", &raw);

        assert!(affected.contains("/ma/root/0.0.1"));
        assert!(affected.contains("/ma/room/0.0.1"));
        assert!(!affected.contains("/ma/missing-base/0.0.1"));
        assert!(!affected.contains("/ma/cycle-a/0.0.1"));
        assert!(!affected.contains("/ma/cycle-b/0.0.1"));
    }
}
