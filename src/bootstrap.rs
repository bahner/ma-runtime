//! Bootstrap: YAML → IPLD manifest + FTL locales → Kubo.
//!
//! Run once before starting the runtime:
//! ```sh
//! ma --gen-root-cid bootstrap.yaml
//! ```
//! CID for the runtime root manifest is written back to `config.yaml`.
//! Subsequent daemon starts load entities and locales from IPFS.

use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use crate::acl::AclMap;
use crate::entity::Evaluator;
use crate::entity::{
    EntityNode, IpldLink, KindNode, KindRegistry, KindTree, PluginKind, RuntimeManifest,
};
use crate::kubo;
use crate::plugin;

// ── YAML bootstrap schema ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapYaml {
    pub runtime: BootstrapRuntime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapRuntime {
    #[serde(default)]
    pub kinds: BootstrapKindsDict,
    /// Root transport-gate ACL — inline `AclMap` published to IPFS at bootstrap.
    /// Controls who may use the RPC, inbox, and IPFS services.
    /// If absent, the daemon falls back to `--acl-file` (or open access).
    #[serde(default)]
    pub acl: Option<AclMap>,
    /// Entities: bare name → inline entity descriptor.
    /// Bootstrap publishes each as a DAG-CBOR [`EntityNode`] and stores the CID
    /// in the manifest. Keys are bare names (e.g. `"fortune"`), not `#`-prefixed.
    #[serde(default)]
    pub entities: HashMap<String, BootstrapEntity>,
    /// Named ACL library: name → inline `AclMap` published to IPFS at bootstrap.
    /// Reference an ACL by name in an `EntityNode`'s `acl` field.
    #[serde(default)]
    pub acls: HashMap<String, AclMap>,
    /// Named group registry: name → inline flat DID list, published to IPFS
    /// at bootstrap. Referenced from any `AclMap` as principal `+<name>`.
    /// The `"owners"` entry (if present) is the runtime's authoritative
    /// owner list — same storage as any other group, no special field.
    #[serde(default)]
    pub grp: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapKind {
    /// IPLD link to the compiled Wasm module bytes shared by every entity of
    /// this kind. Absent for kinds where each entity supplies its own Wasm
    /// via `EntityNode.behaviour` instead (see `KindNode.cid`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<IpldLink>,
    /// How the runtime executes Wasm bytes for this kind. YAML key is `type`
    /// (was `evaluator` in an earlier draft — same field, renamed).
    #[serde(rename = "type", default)]
    pub kind_type: crate::entity::Evaluator,
    /// Optional kind-level behaviour source text CID. For scriptable shared
    /// binary kinds, this source is loaded before per-entity behaviour.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behaviour: Option<IpldLink>,
    #[serde(default)]
    pub host_functions: Vec<String>,
    #[serde(default)]
    pub attributes: BTreeMap<String, serde_json::Value>,
    /// Optional base kind's protocol ID to inherit from — see
    /// `crate::entity::resolve_kind_extends` for merge semantics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extends: Option<String>,
}

/// Flat map: protocol ID → kind descriptor.
/// Keys are full protocol ID strings, e.g. `/ma/stateless/python/0.0.1`.
pub type BootstrapKindsDict = BTreeMap<String, BootstrapKind>;

/// Entity entry in the bootstrap YAML — either a bare CID or an inline descriptor.
///
/// ```yaml
/// entities:
///   # pre-published EntityNode — just the CID:
///   rms: bafyreid...
///
///   # inline — bootstrap builds and publishes the EntityNode:
///   fortune:
///     kind: /ma/stateless/python/0.0.1
///     behaviour:
///       /: QmaBC...   # IPLD link to Wasm bytes
///     acl: open       # optional; empty = deny-all
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BootstrapEntity {
    /// Pre-published [`EntityNode`] — the CID is stored directly in the manifest.
    Cid(String),
    /// Inline descriptor — bootstrap publishes the [`EntityNode`] to IPFS.
    Inline {
        /// Protocol ID of this entity's kind (e.g. `/ma/stateless/python/0.0.1`).
        kind: String,
        /// IPLD link to this entity's own behaviour source. For shared-binary
        /// scriptable kinds this text is appended after kind-level behaviour
        /// layers; for kinds with no shared `cid`, it is the entity's own
        /// Wasm binary.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        behaviour: Option<IpldLink>,
        /// Named ACL reference resolved via `acls.<name>` in the manifest.
        /// Empty string = deny-all (fail-closed).
        #[serde(default)]
        acl: String,
        /// IPLD link to persisted initial state (stateful entities only).
        #[serde(default)]
        state: Option<IpldLink>,
        /// Entity-level attribute overrides, merged over the kind's own
        /// attributes (entity wins) — see `EntityNode::attributes`. E.g.
        /// `{"genesis": true}` marks this as a tree-root entity.
        #[serde(default)]
        attributes: std::collections::BTreeMap<String, serde_json::Value>,
        /// Opaque, persisted creation payload — see `EntityNode::init`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        init: Option<String>,
    },
}

// ── Result type ───────────────────────────────────────────────────────────────

/// Result of a successful bootstrap run.
#[derive(Debug)]
pub struct BootstrapResult {
    pub root_cid: String,
}

// ── Core bootstrap logic ──────────────────────────────────────────────────────

/// Parse `yaml_path`, publish all IPLD nodes and FTL files to Kubo,
/// and return the resulting CIDs.
pub async fn run_bootstrap(
    yaml_path: &std::path::Path,
    kubo_url: &str,
    runtime_config: BTreeMap<String, serde_yaml::Value>,
    active_lang: &str,
    old_root_cid: Option<&str>,
) -> Result<BootstrapResult> {
    let raw = std::fs::read_to_string(yaml_path)
        .with_context(|| format!("reading bootstrap file: {}", yaml_path.display()))?;
    let yaml: BootstrapYaml = serde_yaml::from_str(&raw).context("parsing bootstrap YAML")?;

    build_manifest(
        &yaml.runtime,
        kubo_url,
        runtime_config,
        active_lang,
        old_root_cid,
    )
    .await
}

/// Build the full IPLD manifest and publish to Kubo. Returns root CID.
pub async fn build_manifest(
    cfg: &BootstrapRuntime,
    kubo_url: &str,
    mut runtime_config: BTreeMap<String, serde_yaml::Value>,
    active_lang: &str,
    old_root_cid: Option<&str>,
) -> Result<BootstrapResult> {
    let kinds = publish_kinds(cfg, kubo_url).await?;
    let entities_map = publish_entities(cfg, kubo_url).await?;
    let acls_map = publish_named_acls(cfg, kubo_url).await?;
    let grp_map = publish_groups(cfg, kubo_url).await?;
    let root_acl_link = publish_root_acl(cfg, kubo_url).await?;

    let i18n = crate::i18n::bundled_lang_map();
    if i18n.contains_key(active_lang) {
        runtime_config.insert(
            "i18n".to_string(),
            serde_yaml::Value::String(active_lang.to_string()),
        );
    }
    let root = RuntimeManifest {
        acl: root_acl_link,
        acls: acls_map,
        protocol: "/ma/runtime/0.1.0".to_string(),
        kinds,
        entities: entities_map,
        i18n,
        grp: grp_map,
        config: runtime_config,
    };
    let root_cid = kubo::dag_put(kubo_url, &root)
        .await
        .context("dag_put root manifest")?;
    tracing::info!(root_cid = %root_cid, "Published runtime root manifest");

    if let Some(old) = old_root_cid {
        if let Err(e) = kubo::pin_update(kubo_url, old, &root_cid).await {
            tracing::warn!(old = %old, new = %root_cid, error = %e, "pin/update failed after bootstrap");
        }
    } else {
        kubo::pin_add(kubo_url, &root_cid)
            .await
            .context("pinning new root manifest")?;
    }

    Ok(BootstrapResult { root_cid })
}

async fn publish_kinds(cfg: &BootstrapRuntime, kubo_url: &str) -> Result<KindTree> {
    let mut kinds = KindTree::default();
    for (protocol, bk) in &cfg.kinds {
        let node = KindNode {
            protocol: protocol.clone(),
            cid: bk.cid.clone(),
            kind_type: bk.kind_type.clone(),
            behaviour: bk.behaviour.clone(),
            behaviour_chain: Vec::new(),
            host_functions: bk.host_functions.clone(),
            attributes: bk.attributes.clone(),
            extends: bk.extends.clone(),
        };
        let cid = kubo::dag_put(kubo_url, &node)
            .await
            .with_context(|| format!("dag_put kind {protocol}"))?;
        tracing::info!(protocol = %protocol, cid = %cid, "Published kind node");
        kinds.insert_protocol(protocol, IpldLink::new(cid));
    }
    Ok(kinds)
}

async fn publish_entities(
    cfg: &BootstrapRuntime,
    kubo_url: &str,
) -> Result<HashMap<String, IpldLink>> {
    let mut entities_map: HashMap<String, IpldLink> = HashMap::new();
    for (name, be) in &cfg.entities {
        let link = match be {
            BootstrapEntity::Cid(cid) => {
                tracing::info!(name = %name, cid = %cid, "Registering pre-published entity");
                IpldLink::new(cid)
            }
            BootstrapEntity::Inline {
                kind,
                behaviour,
                acl,
                state,
                attributes,
                init,
            } => {
                // Validate that the entity's ACL name exists in the manifest
                // (if the entity specifies a non-empty ACL).
                if !acl.is_empty() && !cfg.acls.contains_key(acl.as_str()) {
                    let available: Vec<&String> = cfg.acls.keys().collect();
                    return Err(anyhow!(
                        "entity '{}' references unknown ACL '{}' (available: {})",
                        name,
                        acl,
                        available
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
                let node = EntityNode {
                    kind: kind.clone(),
                    behaviour: behaviour.clone(),
                    acl: acl.clone(),
                    state: state.clone(),
                    parent: None,
                    label: None,
                    attributes: attributes.clone(),
                    init: init.clone(),
                    initialised: false,
                };
                let cid = kubo::dag_put(kubo_url, &node)
                    .await
                    .with_context(|| format!("dag_put entity {name}"))?;
                tracing::info!(name = %name, cid = %cid, "Published entity node");
                IpldLink::new(cid)
            }
        };
        entities_map.insert(name.clone(), link);
    }
    Ok(entities_map)
}

async fn publish_named_acls(
    cfg: &BootstrapRuntime,
    kubo_url: &str,
) -> Result<HashMap<String, IpldLink>> {
    let mut acls_map: HashMap<String, IpldLink> = HashMap::new();
    for (name, acl) in &cfg.acls {
        let cid = kubo::dag_put(kubo_url, acl)
            .await
            .with_context(|| format!("dag_put acl {name}"))?;
        tracing::info!(name = %name, cid = %cid, "Published ACL node");
        acls_map.insert(name.clone(), IpldLink::new(cid));
    }
    Ok(acls_map)
}

async fn publish_groups(
    cfg: &BootstrapRuntime,
    kubo_url: &str,
) -> Result<HashMap<String, IpldLink>> {
    let mut grp_map: HashMap<String, IpldLink> = HashMap::new();
    for (name, members) in &cfg.grp {
        let cid = kubo::dag_put(kubo_url, members)
            .await
            .with_context(|| format!("dag_put group {name}"))?;
        tracing::info!(name = %name, cid = %cid, "Published group node");
        grp_map.insert(name.clone(), IpldLink::new(cid));
    }
    Ok(grp_map)
}

/// Publish the root transport-gate ACL with owner DIDs injected as `["*"]`.
async fn publish_root_acl(cfg: &BootstrapRuntime, kubo_url: &str) -> Result<Option<IpldLink>> {
    let mut root_acl: AclMap = cfg.acl.clone().unwrap_or_default();
    for owner in cfg.grp.get("owners").into_iter().flatten() {
        root_acl.insert(owner.clone(), crate::acl::CapabilityEntry::from_caps(["*"]));
    }
    let cid = kubo::dag_put(kubo_url, &root_acl)
        .await
        .context("dag_put root acl")?;
    tracing::info!(cid = %cid, "Published root transport-gate ACL");
    Ok(Some(IpldLink::new(cid)))
}

/// Export the current runtime manifest as a `BootstrapYaml` YAML string.
///
/// Fetches every linked IPLD node (kinds, entities, named ACLs, root ACL)
/// from Kubo and reconstructs the full bootstrap descriptor so it can be
/// edited and re-bootstrapped with `ma --gen-root-cid`.
pub async fn export_bootstrap_yaml(root_cid: &str, kubo_url: &str) -> Result<String> {
    let manifest: RuntimeManifest = kubo::dag_get(kubo_url, root_cid)
        .await
        .context("fetching root manifest")?;

    // Kinds: fetch each KindNode by CID.
    let mut kinds = BootstrapKindsDict::new();
    for (protocol, kind_link) in manifest.kinds.iter_protocols() {
        let node: KindNode = kubo::dag_get(kubo_url, &kind_link.cid)
            .await
            .with_context(|| format!("fetching kind {protocol}"))?;
        kinds.insert(
            protocol,
            BootstrapKind {
                cid: node.cid,
                kind_type: node.kind_type,
                behaviour: node.behaviour,
                host_functions: node.host_functions,
                attributes: node.attributes,
                extends: node.extends,
            },
        );
    }

    // Entities: fetch each EntityNode and reconstruct inline descriptor.
    let mut entities: HashMap<String, BootstrapEntity> = HashMap::new();
    for (name, link) in &manifest.entities {
        let node: EntityNode = kubo::dag_get(kubo_url, &link.cid)
            .await
            .with_context(|| format!("fetching entity {name}"))?;
        entities.insert(
            name.clone(),
            BootstrapEntity::Inline {
                kind: node.kind,
                behaviour: node.behaviour,
                acl: node.acl,
                state: node.state,
                attributes: node.attributes,
                init: node.init,
            },
        );
    }

    // Named ACLs: fetch each AclMap by CID.
    let mut acls: HashMap<String, AclMap> = HashMap::new();
    for (name, link) in &manifest.acls {
        let acl_map: AclMap = kubo::dag_get(kubo_url, &link.cid)
            .await
            .with_context(|| format!("fetching acl {name}"))?;
        acls.insert(name.clone(), acl_map);
    }

    // Named groups: fetch each flat DID list by CID.
    let mut grp: HashMap<String, Vec<String>> = HashMap::new();
    for (name, link) in &manifest.grp {
        let members: Vec<String> = kubo::dag_get(kubo_url, &link.cid)
            .await
            .with_context(|| format!("fetching group {name}"))?;
        grp.insert(name.clone(), members);
    }

    // Root transport-gate ACL.
    let acl: Option<AclMap> = if let Some(ref link) = manifest.acl {
        Some(
            kubo::dag_get(kubo_url, &link.cid)
                .await
                .context("fetching root acl")?,
        )
    } else {
        None
    };

    let yaml = BootstrapYaml {
        runtime: BootstrapRuntime {
            kinds,
            acl,
            entities,
            acls,
            grp,
        },
    };
    serde_yaml::to_string(&yaml).context("serializing bootstrap YAML")
}

// ── Startup entity loading ────────────────────────────────────────────────────

/// Fetch the `RuntimeManifest` at `root_cid`, load each entity plugin, and
/// insert them into `registry`.  Persists `lifecycle: running` back to IPFS
/// for every successfully loaded entity whose stored lifecycle differs.
/// Returns `(count, Some(new_root_cid))` when any entity nodes were updated,
/// or `(count, None)` when nothing changed.
#[allow(clippy::too_many_arguments)]
pub async fn load_entities(
    root_cid: &str,
    kubo_url: &str,
    daemon_config: &ma_core::Config,
    our_did: &str,
    registry: &plugin::EntityRegistry,
    kind_registry: &KindRegistry,
    native_factories: &plugin::NativeFactories,
    envelope_tx: tokio::sync::mpsc::UnboundedSender<(String, crate::entity::SendEnvelope)>,
    iroh_node_id: &str,
    started_at: u64,
) -> (usize, Option<String>) {
    let mut manifest = match kubo::dag_get::<RuntimeManifest>(kubo_url, root_cid).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to fetch runtime manifest {root_cid}: {e}");
            return (0, None);
        }
    };

    let kind_count = hydrate_kind_registry(&manifest, kubo_url, kind_registry).await;
    tracing::info!(count = %kind_count, "Kind registry hydrated from manifest");
    let runtime_config = crate::crud::config::public_plugin_config(&manifest, daemon_config);

    let mut loaded = 0usize;
    let mut manifest_updated = false;
    for (name, link) in manifest.entities.clone() {
        let Some((node, kind_node)) = load_entity_and_kind(&manifest, kubo_url, &name, &link).await
        else {
            continue;
        };

        let load = LoadEntityArgs {
            name: &name,
            node: &node,
            kind_node: &kind_node,
            our_did,
            kubo_url,
            envelope_tx: envelope_tx.clone(),
            registry: registry.clone(),
            iroh_node_id,
            started_at,
            runtime_config: &runtime_config,
        };

        let load_result = if is_native_kind(&kind_node) {
            load_native_entity(load, native_factories).await
        } else {
            load_wasm_entity(load).await
        };

        if let Some(result) = load_result {
            if let Some(new_link) = result.initialised_link {
                manifest.entities.insert(name.clone(), new_link);
                manifest_updated = true;
            }
            loaded += 1;
        }
    }

    if !manifest_updated {
        return (loaded, None);
    }

    // Publish updated manifest and swap pin.
    match kubo::dag_put(kubo_url, &manifest).await {
        Ok(new_root) => {
            if let Err(e) = kubo::pin_update(kubo_url, root_cid, &new_root).await {
                tracing::warn!(old = %root_cid, new = %new_root, error = %e, "pin/update failed after lifecycle persist");
            }
            tracing::info!(root_cid = %new_root, "Published updated manifest after lifecycle transitions");
            (loaded, Some(new_root))
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to publish manifest after lifecycle transitions");
            (loaded, None)
        }
    }
}

async fn hydrate_kind_registry(
    manifest: &RuntimeManifest,
    kubo_url: &str,
    registry: &KindRegistry,
) -> usize {
    let mut loaded = 0usize;
    for (protocol, link) in manifest.kinds.iter_protocols() {
        let raw_kind: KindNode = match kubo::dag_get(kubo_url, &link.cid).await {
            Ok(k) => k,
            Err(e) => {
                tracing::warn!(protocol = %protocol, cid = %link.cid, "Failed to fetch kind node for registry: {e}");
                continue;
            }
        };
        let kind_node = if raw_kind.extends.is_some() {
            match crate::entity::resolve_kind_extends(kubo_url, manifest, raw_kind).await {
                Ok(k) => k,
                Err(e) => {
                    tracing::warn!(protocol = %protocol, "Failed to resolve kind extends chain for registry: {e}");
                    continue;
                }
            }
        } else {
            raw_kind
        };
        registry.write().await.insert(protocol, Arc::new(kind_node));
        loaded += 1;
    }
    loaded
}

async fn load_entity_and_kind(
    manifest: &RuntimeManifest,
    kubo_url: &str,
    name: &str,
    link: &IpldLink,
) -> Option<(EntityNode, KindNode)> {
    let node: EntityNode = match kubo::dag_get(kubo_url, &link.cid).await {
        Ok(n) => n,
        Err(e) => {
            tracing::warn!(name = %name, cid = %link.cid, "Failed to fetch entity node: {e}");
            return None;
        }
    };

    let Some(kind_link) = manifest.kinds.get_protocol(&node.kind).cloned() else {
        tracing::warn!(name = %name, kind = %node.kind, "Kind not found in manifest; skipping entity");
        return None;
    };

    let raw_kind: KindNode = match kubo::dag_get(kubo_url, &kind_link.cid).await {
        Ok(k) => k,
        Err(e) => {
            tracing::warn!(name = %name, kind = %node.kind, cid = %kind_link.cid, "Failed to fetch kind node: {e}");
            return None;
        }
    };

    let kind_node = if raw_kind.extends.is_some() {
        match crate::entity::resolve_kind_extends(kubo_url, manifest, raw_kind).await {
            Ok(k) => k,
            Err(e) => {
                tracing::warn!(name = %name, kind = %node.kind, "Failed to resolve kind extends chain: {e}");
                return None;
            }
        }
    } else {
        raw_kind
    };

    Some((node, kind_node))
}

fn is_native_kind(kind_node: &KindNode) -> bool {
    kind_node.kind_type == Evaluator::Native
}

struct LoadEntityArgs<'a> {
    name: &'a str,
    node: &'a EntityNode,
    kind_node: &'a KindNode,
    our_did: &'a str,
    kubo_url: &'a str,
    envelope_tx: tokio::sync::mpsc::UnboundedSender<(String, crate::entity::SendEnvelope)>,
    registry: plugin::EntityRegistry,
    iroh_node_id: &'a str,
    started_at: u64,
    runtime_config: &'a std::collections::BTreeMap<String, String>,
}

struct LoadedEntity {
    initialised_link: Option<IpldLink>,
}

async fn load_wasm_entity(args: LoadEntityArgs<'_>) -> Option<LoadedEntity> {
    let init_payload = args.node.init.as_ref().map(|s| s.as_bytes().to_vec());
    match plugin::EntityPlugin::load(
        args.name.to_string(),
        args.node,
        args.kind_node,
        args.our_did,
        args.kubo_url,
        args.envelope_tx.clone(),
        args.registry.clone(),
        args.iroh_node_id,
        args.started_at,
        args.runtime_config.clone(),
        init_payload,
    )
    .await
    {
        Ok((ep, lifecycle)) => {
            tracing::info!(name = %args.name, lifecycle = %lifecycle, "{}", crate::i18n::t("entity-loaded"));
            let mut node = args.node.clone();
            if let Ok(Some(cid)) = ep.trigger_save(args.kubo_url).await {
                node.state = Some(IpldLink::new(cid));
            }
            let updated_link = persist_initialised_transition(&args, &node, &lifecycle).await;
            args.registry
                .write()
                .await
                .insert(args.name.to_string(), Arc::new(ep));
            Some(LoadedEntity {
                initialised_link: updated_link,
            })
        }
        Err(e) => {
            tracing::warn!(name = %args.name, error = %e, "{}", crate::i18n::t("entity-load-failed"));
            None
        }
    }
}

async fn load_native_entity(
    args: LoadEntityArgs<'_>,
    native_factories: &plugin::NativeFactories,
) -> Option<LoadedEntity> {
    let Some(factory) = native_factories.get(&args.kind_node.protocol) else {
        tracing::warn!(name = %args.name, kind = %args.kind_node.protocol, "Native kind has no registered runtime implementation");
        return None;
    };

    let init_state = load_initial_state(args.node, args.kind_node, args.kubo_url).await;
    let init_payload = args.node.init.as_ref().map(|s| s.as_bytes().to_vec());
    match plugin::EntityPlugin::new_native(
        args.name.to_string(),
        args.node,
        args.kind_node,
        factory(),
        init_state,
        init_payload,
    ) {
        Ok((ep, lifecycle)) => {
            tracing::info!(name = %args.name, lifecycle = %lifecycle, "{}", crate::i18n::t("entity-loaded"));
            let mut node = args.node.clone();
            if let Ok(Some(cid)) = ep.trigger_save(args.kubo_url).await {
                node.state = Some(IpldLink::new(cid));
            }
            let updated_link = persist_initialised_transition(&args, &node, &lifecycle).await;
            args.registry
                .write()
                .await
                .insert(args.name.to_string(), Arc::new(ep));
            Some(LoadedEntity {
                initialised_link: updated_link,
            })
        }
        Err(e) => {
            tracing::warn!(name = %args.name, error = %e, "{}", crate::i18n::t("entity-load-failed"));
            None
        }
    }
}

async fn load_initial_state(node: &EntityNode, kind_node: &KindNode, kubo_url: &str) -> Vec<u8> {
    if kind_node.plugin_kind() != PluginKind::Stateful {
        return Vec::new();
    }
    match &node.state {
        Some(link) => ma_core::cat_bytes(kubo_url, &link.cid)
            .await
            .unwrap_or_default(),
        None => Vec::new(),
    }
}

async fn persist_initialised_transition(
    args: &LoadEntityArgs<'_>,
    node: &EntityNode,
    lifecycle: &crate::entity::Lifecycle,
) -> Option<IpldLink> {
    if lifecycle != &crate::entity::Lifecycle::Running {
        return None;
    }

    let mut updated = node.clone();
    let state_changed = args.node.state.as_ref().map(|l| l.cid.as_str())
        != updated.state.as_ref().map(|l| l.cid.as_str());
    let initialised_changed = !args.node.initialised;
    if initialised_changed {
        updated.initialised = true;
    }
    if !initialised_changed && !state_changed {
        return None;
    }
    match kubo::dag_put(args.kubo_url, &updated).await {
        Ok(new_cid) => {
            tracing::debug!(name = %args.name, cid = %new_cid, "Updated entity lifecycle in IPFS");
            Some(IpldLink::new(new_cid))
        }
        Err(e) => {
            tracing::warn!(name = %args.name, error = %e, "Failed to write updated entity lifecycle to IPFS");
            None
        }
    }
}

// ── Graceful shutdown: persist entity states ──────────────────────────────────
/// nodes and root manifest.
/// Logs progress at `info` level with per-entity phases.  Returns the new
/// root CID on success.
pub async fn save_all_entity_states(
    root_cid: &str,
    kubo_url: &str,
    registry: &plugin::EntityRegistry,
) -> Result<String> {
    // Phase 1: fetch current manifest.
    tracing::info!(root_cid = %root_cid, "Fetching current runtime manifest");
    let mut manifest: RuntimeManifest = kubo::dag_get(kubo_url, root_cid)
        .await
        .context("fetching current runtime manifest")?;

    // Snapshot the registry so we don't hold the lock during async IPFS calls.
    let snapshot: Vec<(String, Arc<plugin::EntityPlugin>)> = registry
        .read()
        .await
        .iter()
        .map(|(k, v)| (k.clone(), Arc::clone(v)))
        .collect();

    // Phase 2: persist each entity's state and lifecycle.
    // Stateless entities skip state saving but still get lifecycle: stopped.
    for (name, entity) in &snapshot {
        // Stateless native entities are manifest markers/compiled-in runtime
        // hooks. Stateful native entities still pass through prepare_reload_save(),
        // whose native backend may currently be a no-op.
        if entity.is_native() && entity.kind == PluginKind::Stateless {
            continue;
        }
        let Some(entity_link) = manifest.entities.get(name).cloned() else {
            tracing::warn!(name = %name, "Entity in registry but not in manifest, skipping");
            continue;
        };
        let mut entity_node: EntityNode = match kubo::dag_get(kubo_url, &entity_link.cid).await {
            Ok(n) => n,
            Err(e) => {
                tracing::warn!(name = %name, error = %e, "Failed to fetch entity node for state update");
                continue;
            }
        };

        if entity.kind != PluginKind::Stateless {
            tracing::info!(name = %name, "{}", crate::i18n::t("entity-state-saving"));
            match entity
                .prepare_reload_save(kubo_url, crate::plugin::graceful_shutdown_timeout())
                .await
            {
                Ok(Some(cid)) => {
                    tracing::info!(name = %name, cid = %cid, "{}", crate::i18n::t("entity-state-saved"));
                    entity_node.state = Some(IpldLink::new(cid));
                }
                Ok(None) => {
                    tracing::info!(name = %name, "{}", crate::i18n::t("entity-state-empty"));
                }
                Err(e) => {
                    return Err(e).with_context(|| format!("saving state for entity '{name}'"));
                }
            }
        }

        match kubo::dag_put(kubo_url, &entity_node).await {
            Ok(new_cid) => {
                tracing::info!(name = %name, cid = %new_cid, "Updated entity node on shutdown");
                manifest
                    .entities
                    .insert(name.clone(), IpldLink::new(new_cid));
            }
            Err(e) => {
                return Err(e)
                    .with_context(|| format!("publishing updated entity node for '{name}'"));
            }
        }
    }

    // Phase 3: publish updated manifest.
    tracing::info!("Publishing updated runtime manifest");
    let new_root_cid = kubo::dag_put(kubo_url, &manifest)
        .await
        .context("dag_put updated manifest")?;

    // Swap pins atomically via pin/update.
    if let Err(e) = kubo::pin_update(kubo_url, root_cid, &new_root_cid).await {
        tracing::warn!(old = %root_cid, new = %new_root_cid, error = %e, "pin/update failed after state save");
    }

    Ok(new_root_cid)
}

#[cfg(test)]
mod tests {
    use super::BootstrapYaml;
    use ma_core::{check_cap, CAP_IDENTITY_PUBLISH, CAP_IPFS, CAP_RPC};

    #[test]
    fn example_yaml_parses() {
        let raw = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bootstrap.example.yaml"
        ))
        .unwrap();
        let yaml: BootstrapYaml =
            serde_yaml::from_str(&raw).expect("bootstrap.example.yaml must parse");
        assert!(
            yaml.runtime.kinds.is_empty(),
            "runtime bootstrap example must not carry world-owned kind CIDs"
        );
        assert!(
            yaml.runtime.entities.is_empty(),
            "runtime bootstrap example must not carry world-owned entities"
        );
        assert!(
            yaml.runtime.grp.contains_key("owners"),
            "owners group must be present"
        );
        assert!(yaml.runtime.acls.contains_key("open"));
        assert!(yaml.runtime.acls.contains_key("owners"));
        assert!(yaml.runtime.acls.contains_key("scheduler"));

        let root_acl = yaml.runtime.acl.expect("root ACL must be present");
        assert!(check_cap(&root_acl, "did:ma:alice", CAP_IPFS).is_ok());
        assert!(check_cap(&root_acl, "did:ma:alice", CAP_IDENTITY_PUBLISH).is_ok());
        assert!(check_cap(&root_acl, "did:ma:alice", CAP_RPC).is_ok());
    }
}
