use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    sync::Arc,
};

use anyhow::{anyhow, Context, Result};
use ciborium::Value as CborValue;
use cid::{Cid, Version};
use ma_core::{
    Did, DidDocumentResolver, Ipld, CONTENT_TYPE_TERM, CONTENT_TYPE_TERM_CBOR,
    CONTENT_TYPE_TERM_YAML,
};
use tokio::sync::Semaphore;
use tracing::{debug, error, info, warn};

use crate::entity::{EntityNode, KindNode, RuntimeManifest};

use super::CrudHandlerCtx;

const MAX_CONCURRENT_KIND_RELOADS: usize = 4;

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

/// True if `s` is a bare base32 `CIDv1`.
///
/// Structured CRUD fields store deterministic IPLD links, so IPNS and gateway
/// path forms are intentionally rejected here.
pub(super) fn is_cidv1_ref(s: &str) -> bool {
    s.starts_with('b') && Cid::try_from(s).is_ok_and(|cid| cid.version() == Version::V1)
}

/// Validate and return a bare `CIDv1` reference. Returns `None` if `s` is not a
/// bare `CIDv1` (see [`is_cidv1_ref`]).
pub(super) fn cidv1_ref(s: &str) -> Option<String> {
    if !is_cidv1_ref(s) {
        return None;
    }
    Some(s.to_string())
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

pub(super) async fn with_manifest_crud_async<F, T>(
    ctx: &CrudHandlerCtx,
    f: F,
) -> Result<(String, T)>
where
    F: for<'a> FnOnce(&'a mut RuntimeManifest) -> crate::manifest::ManifestMutationFuture<'a, T>,
{
    let result = ctx.manifest_writer.mutate_async(f).await?;
    update_stats_entities(ctx).await;
    Ok(result)
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

#[cfg(test)]
pub(super) fn affected_kind_protocols(
    updated_protocol: &str,
    raw_kinds: &HashMap<String, KindNode>,
) -> BTreeSet<String> {
    affected_kind_protocols_for(&[updated_protocol.to_string()], raw_kinds)
}

fn affected_kind_protocols_for(
    updated_protocols: &[String],
    raw_kinds: &HashMap<String, KindNode>,
) -> BTreeSet<String> {
    let mut affected = BTreeSet::new();
    for updated_protocol in updated_protocols {
        affected.insert(updated_protocol.clone());
        affected.extend(
            raw_kinds
                .keys()
                .filter(|protocol| kind_depends_on(protocol, updated_protocol, raw_kinds))
                .cloned(),
        );
    }
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

/// Bundled dependencies for spawning one or more entity reload tasks.
///
/// Grouping these lets [`spawn_entity_reload`] take a single argument instead
/// of a long, error-prone parameter list. All fields are cheaply `Clone`able
/// (`Arc`-backed), so callers build one `EntityReloadCtx` and `.clone()` it
/// per entity.
#[derive(Clone)]
pub(super) struct EntityReloadCtx {
    pub(super) kind_registry: crate::entity::KindRegistry,
    pub(super) stats: crate::status::SharedStats,
    pub(super) kubo_rpc_url: Arc<str>,
    pub(super) our_did: Arc<str>,
    pub(super) envelope_tx: tokio::sync::mpsc::Sender<(String, crate::entity::SendEnvelope)>,
    pub(super) entity_registry: crate::plugin::EntityRegistry,
    pub(super) manifest_writer: crate::manifest::ManifestWriter,
    pub(super) runtime_config: BTreeMap<String, String>,
    pub(super) reload_shutdown_timeout: std::time::Duration,
    pub(super) reload_gate: Arc<Semaphore>,
}

pub(super) async fn spawn_kind_dependency_reloads(
    updated_protocol: &str,
    ctx: &CrudHandlerCtx,
    runtime_config: BTreeMap<String, String>,
) -> Result<usize> {
    spawn_kind_dependency_reloads_for(&[updated_protocol.to_string()], ctx, runtime_config).await
}

pub(super) async fn spawn_kind_dependency_reloads_for(
    updated_protocols: &[String],
    ctx: &CrudHandlerCtx,
    runtime_config: BTreeMap<String, String>,
) -> Result<usize> {
    let manifest = load_manifest(ctx).await?;
    let raw_kinds = load_raw_kind_nodes(ctx, &manifest).await;
    let affected = affected_kind_protocols_for(updated_protocols, &raw_kinds);
    hydrate_affected_kind_registry(ctx, &manifest, &raw_kinds, &affected).await;
    let reload_shutdown_timeout = {
        let cfg = ctx.shared_config.read().await;
        super::config::wasm_reload_shutdown_timeout(&cfg)
    };
    let reload_ctx = EntityReloadCtx {
        kind_registry: ctx.kind_registry.clone(),
        stats: ctx.stats.clone(),
        kubo_rpc_url: Arc::clone(&ctx.kubo_rpc_url),
        our_did: Arc::clone(&ctx.our_did),
        envelope_tx: ctx.envelope_tx.clone(),
        entity_registry: ctx.entity_registry.clone(),
        manifest_writer: ctx.manifest_writer.clone(),
        runtime_config,
        reload_shutdown_timeout,
        reload_gate: Arc::new(Semaphore::new(MAX_CONCURRENT_KIND_RELOADS)),
    };

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
        spawn_entity_reload(name, entity_node, reload_ctx.clone());
        reload_count += 1;
    }

    Ok(reload_count)
}

/// Spawn an independent task that loads a plugin from `entity_node` and inserts
/// it into the entity registry (replacing any existing version).
///
/// Returns immediately — the reload happens asynchronously so the CRUD event
/// loop is never blocked by WASM fetching, instantiation, or `init()`. `ctx`
/// supplies a shared semaphore so a broad kind overlay queues reloads instead
/// of starting unbounded concurrent Kubo/Wasm work.
///
/// Mirrors `bootstrap::load_entities`'s lifecycle-persistence step: if the
/// load transitions `lifecycle` (typically `new` → `running` on first
/// genesis), the updated `EntityNode` is republished to IPFS and the
/// manifest is updated to point at the new CID — otherwise a later daemon
/// restart would re-read the stale `lifecycle: new` node and incorrectly
/// re-fire the `:init` signal a second time.
pub(super) fn spawn_entity_reload(name: String, entity_node: EntityNode, ctx: EntityReloadCtx) {
    tokio::spawn(async move {
        let Ok(_permit) = Arc::clone(&ctx.reload_gate).acquire_owned().await else {
            warn!(name = %name, kind = %entity_node.kind, "entity reload skipped because reload gate closed");
            return;
        };
        info!(
            name = %name,
            kind = %entity_node.kind,
            state_cid = ?entity_node.state.as_ref().map(|link| link.cid.as_str()),
            behaviour_cid = ?entity_node.behaviour.as_ref().map(|link| link.cid.as_str()),
            "entity reload started"
        );

        let Ok(kind_node) = resolve_kind_node_for_reload(&ctx, &name, &entity_node.kind).await
        else {
            return;
        };

        let (iroh_node_id, started_at) = {
            let s = ctx.stats.read().await;
            (s.endpoint_id.clone(), s.started_at)
        };

        let current_entity = ctx.entity_registry.read().await.get(&name).cloned();
        let mut entity_node = entity_node;
        if let Some(current) = current_entity.as_ref() {
            match refresh_entity_before_reload(&ctx, &name, current).await {
                Ok(refreshed) => entity_node = refreshed,
                Err(()) => return,
            }
        }

        let init_payload = entity_node.init.as_ref().map(|s| s.as_bytes().to_vec());
        let load_result =
            crate::plugin::EntityPlugin::load_with_fibonacci_backoff(crate::plugin::LoadArgs {
                fragment: name.clone(),
                node: &entity_node,
                kind_node: &kind_node,
                our_did: &ctx.our_did,
                kubo_url: &ctx.kubo_rpc_url,
                envelope_tx: ctx.envelope_tx.clone(),
                entity_registry: ctx.entity_registry.clone(),
                iroh_node_id: &iroh_node_id,
                started_at,
                runtime_config: ctx.runtime_config.clone(),
                init_payload, // EntityNode.init (§ genesis-via-CRUD), only fires if genesis
            })
            .await;

        finish_reload(&ctx, &name, current_entity, load_result).await;
    });
}

/// Resolve the [`KindNode`] needed to reload `entity_kind`: prefer the
/// hydrated in-memory kind registry, with a manifest/IPFS fallback for stale
/// or externally-mutated roots. On any failure the reload error is persisted
/// to the manifest and `Err(())` is returned so the caller can bail out.
async fn resolve_kind_node_for_reload(
    ctx: &EntityReloadCtx,
    name: &str,
    entity_kind: &str,
) -> Result<Arc<KindNode>, ()> {
    let cached = ctx.kind_registry.read().await.get(entity_kind).cloned();
    if let Some(kind_node) = cached {
        return Ok(kind_node);
    }

    let root_cid = ctx.stats.read().await.root_cid.clone();
    let Some(root_cid) = root_cid else {
        let reason = "no root CID available; cannot reload entity";
        warn!(name = %name, kind = %entity_kind, reason);
        mark_entity_reload_failed(&ctx.manifest_writer, name, reason).await;
        return Err(());
    };

    let manifest: RuntimeManifest = match crate::kubo::dag_get(&ctx.kubo_rpc_url, &root_cid).await {
        Ok(m) => m,
        Err(e) => {
            let reason = format!("failed to load manifest for kind lookup: {e}");
            warn!(name = %name, kind = %entity_kind, error = %e, "failed to load manifest for kind lookup");
            mark_entity_reload_failed(&ctx.manifest_writer, name, &reason).await;
            return Err(());
        }
    };

    let Some(kind_link) = manifest.kinds.get_protocol(entity_kind).cloned() else {
        let reason = format!("kind '{entity_kind}' is not in manifest; cannot reload entity");
        warn!(name = %name, kind = %entity_kind, "kind not in manifest; cannot reload entity");
        mark_entity_reload_failed(&ctx.manifest_writer, name, &reason).await;
        return Err(());
    };

    let raw_kind: KindNode = match crate::kubo::dag_get(&ctx.kubo_rpc_url, &kind_link.cid).await {
        Ok(k) => k,
        Err(e) => {
            let reason = format!("failed to fetch kind node: {e}");
            warn!(name = %name, kind = %entity_kind, error = %e, "failed to fetch kind node; cannot reload entity");
            mark_entity_reload_failed(&ctx.manifest_writer, name, &reason).await;
            return Err(());
        }
    };

    let resolved = if raw_kind.extends.is_some() {
        match crate::entity::resolve_kind_extends(&ctx.kubo_rpc_url, &manifest, raw_kind).await {
            Ok(k) => k,
            Err(e) => {
                let reason = format!("failed to resolve kind extends chain: {e}");
                warn!(name = %name, kind = %entity_kind, error = %e, "failed to resolve kind extends chain; cannot reload entity");
                mark_entity_reload_failed(&ctx.manifest_writer, name, &reason).await;
                return Err(());
            }
        }
    } else {
        raw_kind
    };

    Ok(Arc::new(resolved))
}

/// Persist the current entity's pending state (if any) before it is
/// replaced, then re-read its `EntityNode` so the reload sees that state
/// CID. On any failure the reload error is persisted and `Err(())` is
/// returned so the caller can bail out and keep the current plugin running.
async fn refresh_entity_before_reload(
    ctx: &EntityReloadCtx,
    name: &str,
    current: &crate::plugin::EntityPlugin,
) -> Result<EntityNode, ()> {
    match current
        .prepare_reload_save(&ctx.kubo_rpc_url, ctx.reload_shutdown_timeout)
        .await
    {
        Ok(Some(cid)) => {
            if let Err(e) = ctx.manifest_writer.set_entity_state(name, &cid).await {
                let reason = format!("failed to publish current state before reload: {e}");
                warn!(name = %name, cid = %cid, error = %e, "failed to update manifest with current state before reload; keeping current plugin");
                mark_entity_reload_failed(&ctx.manifest_writer, name, &reason).await;
                return Err(());
            }
        }
        Ok(None) => {}
        Err(e) => {
            let reason = format!("failed to persist current state before reload: {e}");
            warn!(name = %name, timeout_ms = ctx.reload_shutdown_timeout.as_millis(), error = %e, "failed to persist current state before reload; keeping current plugin");
            mark_entity_reload_failed(&ctx.manifest_writer, name, &reason).await;
            return Err(());
        }
    }

    match ctx.manifest_writer.entity_node(name).await {
        Ok(node) => Ok(node),
        Err(e) => {
            let reason = format!("failed to reload current entity node after saving state: {e}");
            warn!(name = %name, error = %e, "failed to load current entity node before reload; keeping current plugin");
            mark_entity_reload_failed(&ctx.manifest_writer, name, &reason).await;
            Err(())
        }
    }
}

/// Apply the outcome of `EntityPlugin::load_with_fibonacci_backoff` to the
/// entity registry and manifest: install the new plugin (or roll back on
/// failure), terminate the superseded worker, and record success/failure.
async fn finish_reload(
    ctx: &EntityReloadCtx,
    name: &str,
    current_entity: Option<Arc<crate::plugin::EntityPlugin>>,
    result: Result<(crate::plugin::EntityPlugin, crate::entity::Lifecycle)>,
) {
    match result {
        Ok((ep, lifecycle)) => {
            if lifecycle == crate::entity::Lifecycle::Error && current_entity.is_some() {
                let reason = "plugin lifecycle returned error during reload";
                error!(name = %name, reason, "entity failed to reload; unloading until next reload");
                ctx.entity_registry.write().await.remove(name);
                if let Some(current) = current_entity {
                    current.terminate_worker();
                }
                mark_entity_reload_failed(&ctx.manifest_writer, name, reason).await;
                return;
            }
            let replacement_state_cid = match ep.trigger_save(&ctx.kubo_rpc_url).await {
                Ok(cid) => cid,
                Err(e) => {
                    warn!(name = %name, error = %e, "failed to persist state produced during reload");
                    None
                }
            };
            ctx.entity_registry
                .write()
                .await
                .insert(name.to_string(), Arc::new(ep));
            if let Some(current) = current_entity {
                current.terminate_worker();
            }
            info!(name = %name, lifecycle = %lifecycle, "{}", crate::i18n::t("entity-reloaded"));
            if lifecycle == crate::entity::Lifecycle::Running {
                match ctx
                    .manifest_writer
                    .complete_entity_reload(name, replacement_state_cid.as_deref())
                    .await
                {
                    Ok(root_cid) => {
                        info!(name = %name, root_cid = %root_cid, "updated reloaded entity in manifest");
                    }
                    Err(e) => {
                        warn!(name = %name, error = %e, "failed to update reloaded entity in manifest");
                    }
                }
            }
        }
        Err(e) => {
            let reason = format!("failed to load entity plugin: {e}");
            error!(
                name = %name,
                error = %e,
                "entity failed to reload; unloading until next reload"
            );
            ctx.entity_registry.write().await.remove(name);
            if let Some(current) = current_entity {
                current.terminate_worker();
            }
            mark_entity_reload_failed(&ctx.manifest_writer, name, &reason).await;
        }
    }
}

#[cfg(test)]
mod reload_tests {
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use tokio::sync::{RwLock, Semaphore};

    use super::{spawn_entity_reload, EntityReloadCtx};
    use crate::entity::{EntityNode, Evaluator, IpldLink, KindNode, RuntimeManifest, SendEnvelope};
    use crate::manifest::ManifestWriter;
    use crate::plugin::{new_entity_registry, DispatchResult, EntityPlugin, NativeActor};
    use crate::status::Stats;
    use crate::testkubo::MockKubo;

    const GOOD_WAT: &str = r#"
        (module
          (func $ok (result i32) (i32.const 0))
          (export "on_signal" (func $ok))
          (export "on_message" (func $ok)))
    "#;

    fn stateful_kind(protocol: &str, evaluator: Evaluator, wasm_cid: Option<&str>) -> KindNode {
        let mut attributes = BTreeMap::new();
        attributes.insert("stateful".to_string(), serde_json::Value::Bool(true));
        attributes.insert("wasi".to_string(), serde_json::Value::Bool(false));
        KindNode {
            protocol: protocol.to_string(),
            cid: wasm_cid.map(IpldLink::new),
            kind_type: evaluator,
            behaviour: None,
            behaviour_chain: Vec::new(),
            host_functions: Vec::new(),
            attributes,
            extends: None,
        }
    }

    fn entity_node(kind: &str, state_cid: &str) -> EntityNode {
        EntityNode {
            kind: kind.to_string(),
            behaviour: None,
            acl: String::new(),
            state: Some(IpldLink::new(state_cid)),
            parent: None,
            label: None,
            attributes: BTreeMap::new(),
            init: None,
            initialised: true,
            reload_error: None,
        }
    }

    /// Everything needed to drive `spawn_entity_reload` once and observe its
    /// outcome, bundled to keep the test itself short.
    struct ReloadFixture {
        kubo: MockKubo,
        node: EntityNode,
        stats: Arc<RwLock<Stats>>,
        entity_registry: crate::plugin::EntityRegistry,
        current: Arc<EntityPlugin>,
        reload_ctx: EntityReloadCtx,
    }

    /// Build a stateful "room" entity backed by a native actor, register a
    /// replacement Wasm kind for its protocol, and assemble the
    /// `EntityReloadCtx` needed to reload it.
    async fn build_reload_fixture(protocol: &str) -> ReloadFixture {
        let kubo = MockKubo::start().await;
        let wasm_cid = kubo.add_bytes(wat::parse_str(GOOD_WAT).unwrap()).await;
        let replacement_kind = stateful_kind(protocol, Evaluator::Extism, Some(&wasm_cid));
        let old_state_cid = kubo.add_bytes(b"old room state".to_vec()).await;
        let node = entity_node(protocol, &old_state_cid);
        let entity_cid = crate::kubo::dag_put(kubo.url(), &node).await.unwrap();
        let mut manifest = RuntimeManifest::default();
        manifest
            .entities
            .insert("room".to_string(), IpldLink::new(entity_cid));
        let root_cid = crate::kubo::dag_put(kubo.url(), &manifest).await.unwrap();
        let stats = Arc::new(RwLock::new(Stats {
            root_cid: Some(root_cid.clone()),
            ..Default::default()
        }));
        let manifest_writer = ManifestWriter::new(root_cid, kubo.url().to_string(), stats.clone());

        let pending_state = Arc::new(Mutex::new(Some(
            br#"{"children":{"did:ma:test#lamp":{"kind":"thing"}}}"#.to_vec(),
        )));
        let actor = NativeActor::new(|_| {
            Ok(DispatchResult {
                output: Vec::new(),
                pending_state: None,
                create_requests: Vec::new(),
                delete_requests: Vec::new(),
                behaviour_requests: Vec::new(),
            })
        })
        .with_state_hooks(
            {
                let pending_state = Arc::clone(&pending_state);
                move || pending_state.lock().unwrap().clone()
            },
            {
                let pending_state = Arc::clone(&pending_state);
                move |_| *pending_state.lock().unwrap() = None
            },
        );
        let native_kind = stateful_kind(protocol, Evaluator::Native, None);
        let (current, _) =
            EntityPlugin::new_native("room", &node, &native_kind, actor, Vec::new(), None).unwrap();
        let current = Arc::new(current);
        let entity_registry = new_entity_registry();
        entity_registry
            .write()
            .await
            .insert("room".to_string(), current.clone());
        let kind_registry = crate::entity::new_kind_registry();
        kind_registry
            .write()
            .await
            .insert(protocol.to_string(), Arc::new(replacement_kind));
        let (envelope_tx, _envelope_rx) = tokio::sync::mpsc::channel::<(String, SendEnvelope)>(16);

        let reload_ctx = EntityReloadCtx {
            kind_registry,
            stats: stats.clone(),
            kubo_rpc_url: Arc::from(kubo.url().to_string()),
            our_did: Arc::from("did:ma:test"),
            envelope_tx,
            entity_registry: entity_registry.clone(),
            manifest_writer,
            runtime_config: BTreeMap::new(),
            reload_shutdown_timeout: Duration::from_secs(1),
            reload_gate: Arc::new(Semaphore::new(1)),
        };

        ReloadFixture {
            kubo,
            node,
            stats,
            entity_registry,
            current,
            reload_ctx,
        }
    }

    /// Wait until the registry entry for "room" no longer points at
    /// `current` (i.e. the reload has installed its replacement).
    async fn wait_for_reload_replacement(
        entity_registry: &crate::plugin::EntityRegistry,
        current: &Arc<EntityPlugin>,
    ) {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        loop {
            let replacement = entity_registry.read().await.get("room").cloned();
            if replacement.is_some_and(|replacement| !Arc::ptr_eq(&replacement, current)) {
                break;
            }
            assert!(
                tokio::time::Instant::now() < deadline,
                "timed out waiting for replacement entity"
            );
            tokio::task::yield_now().await;
        }
    }

    /// Assert the pre-reload pending state was published to the manifest's
    /// "room" entity node.
    async fn assert_reloaded_state_persisted(kubo: &MockKubo, stats: &Arc<RwLock<Stats>>) {
        let latest_root = stats.read().await.root_cid.clone().unwrap();
        let latest_manifest: RuntimeManifest = crate::kubo::dag_get(kubo.url(), &latest_root)
            .await
            .unwrap();
        let latest_entity: EntityNode = crate::kubo::dag_get(
            kubo.url(),
            &latest_manifest.entities.get("room").unwrap().cid,
        )
        .await
        .unwrap();
        let saved_state = crate::kubo::cat_bytes(
            kubo.url(),
            &latest_entity.state.expect("state CID after reload").cid,
        )
        .await
        .unwrap();
        assert_eq!(
            saved_state,
            br#"{"children":{"did:ma:test#lamp":{"kind":"thing"}}}"#
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn kind_reload_publishes_pre_reload_state_to_manifest() {
        let protocol = "/ma/test/reload-state/0.0.1";
        let fixture = build_reload_fixture(protocol).await;

        spawn_entity_reload("room".to_string(), fixture.node, fixture.reload_ctx);

        wait_for_reload_replacement(&fixture.entity_registry, &fixture.current).await;
        assert_reloaded_state_persisted(&fixture.kubo, &fixture.stats).await;
    }
}

async fn update_stats_entities(ctx: &CrudHandlerCtx) {
    let names: Vec<String> = ctx.entity_registry.read().await.keys().cloned().collect();
    ctx.stats.write().await.entity_names = names;
}

async fn mark_entity_reload_failed(
    manifest_writer: &crate::manifest::ManifestWriter,
    name: &str,
    reason: &str,
) {
    if let Err(e) = manifest_writer
        .set_entity_reload_error(name, Some(reason))
        .await
    {
        warn!(name = %name, reason = %reason, error = %e, "failed to persist entity reload error");
    }
}

// ── i18n helpers ───────────────────────────────────────────────────────────────

/// Resolve caller's DID document and extract their preferred language.
/// Falls back to the runtime's own language on any error.
pub(super) async fn caller_lang(from: &str, resolver: &dyn DidDocumentResolver) -> String {
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

    let reply = ma_core::Message::new_reply(
        &incoming.to,
        &incoming.from,
        reply_type,
        content_type,
        content,
        &incoming.id,
        ctx.signing_key.as_ref(),
    )
    .context("failed to build CRUD reply")?;

    let outbox_result = if let Some(doc_cache) = &ctx.doc_cache {
        crate::ipfs::open_outbox_for_did(
            &ctx.endpoint,
            &ctx.resolver,
            doc_cache,
            &sender,
            ma_core::CRUD_PROTOCOL_ID,
            ctx.did_resolve,
        )
        .await
    } else {
        ctx.endpoint
            .outbox(
                ctx.resolver.as_ref(),
                &sender.base_id(),
                ma_core::CRUD_PROTOCOL_ID,
            )
            .await
            .map_err(anyhow::Error::from)
    };

    match outbox_result {
        Ok(mut outbox) => {
            outbox
                .send(&reply)
                .await
                .context("CRUD reply send failed")?;
            info!(to = %incoming.from, reply_to = %incoming.id, "CRUD reply sent");
        }
        Err(err) => {
            debug!(error = %err, to = %incoming.from, "CRUD reply delivery failed");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{affected_kind_protocols, decode_crud_payload, is_cidv1_ref, parse_path, CrudOp};
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
    fn is_cidv1_ref_accepts_bare_cidv1() {
        assert!(is_cidv1_ref(
            "bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetojuzjevtenxquvyku"
        ));
    }

    #[test]
    fn is_cidv1_ref_rejects_path_ipns_cidv0_and_plain_text() {
        assert!(!is_cidv1_ref(
            "/ipfs/bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetojuzjevtenxquvyku"
        ));
        assert!(!is_cidv1_ref("/ipns/k51qzi5uqu5dl"));
        assert!(!is_cidv1_ref(
            "QmYwAPJzv5CZsnAzt8auVTLBhdgcq7M4Z6b5q8v8z6C6xF"
        ));
        assert!(!is_cidv1_ref("plain text"));
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
