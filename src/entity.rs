//! Intra-runtime message types, plugin I/O types, and IPLD schema types for
//! the entity dispatch system.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Nested IPLD kinds tree — enables IPLD path traversal like:
///   `ipfs dag get .../kinds/ma/stateless/python/0.0.1`
///
/// Protocol IDs such as `/ma/stateless/python/0.0.1` are stored by stripping
/// the leading `/` and splitting on `/`, forming a tree:
///   `kinds.ma.stateless.python["0.0.1"]` = `{"/": "bafy..."}`
///
/// Use [`KindTree::insert_protocol`], [`KindTree::get_protocol`], etc.
/// for protocol-ID-based access. Serialises as a plain nested JSON/CBOR object.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KindTree(BTreeMap<String, KindTreeNode>);

/// A node in the nested [`KindTree`].
///
/// Either a leaf (IPLD link to a published [`KindNode`]) or a branch
/// (inner map continuing the path traversal).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KindTreeNode {
    /// Leaf: IPLD link to a published [`KindNode`].
    Leaf(IpldLink),
    /// Branch: inner map continuing the path traversal.
    Branch(BTreeMap<String, Self>),
}

impl KindTree {
    /// Insert a kind by full protocol ID (e.g. `/ma/stateless/python/0.0.1`).
    pub fn insert_protocol(&mut self, protocol: &str, link: IpldLink) {
        let segments: Vec<&str> = protocol.trim_start_matches('/').split('/').collect();
        kind_tree_insert(&mut self.0, &segments, link);
    }

    /// Look up an IPLD link by full protocol ID.
    pub fn get_protocol(&self, protocol: &str) -> Option<&IpldLink> {
        let segments: Vec<&str> = protocol.trim_start_matches('/').split('/').collect();
        kind_tree_get(&self.0, &segments)
    }

    /// Remove a kind by full protocol ID. Returns `true` when something was removed.
    pub fn remove_protocol(&mut self, protocol: &str) -> bool {
        let segments: Vec<&str> = protocol.trim_start_matches('/').split('/').collect();
        kind_tree_remove(&mut self.0, &segments)
    }

    /// Collect all protocol IDs stored in this tree (each prefixed with `/`).
    pub fn protocol_ids(&self) -> Vec<String> {
        let mut ids = Vec::new();
        kind_tree_collect_ids(&self.0, "", &mut ids);
        ids
    }

    /// Iterate over all `(protocol_id, link)` pairs in the tree.
    pub fn iter_protocols(&self) -> impl Iterator<Item = (String, &IpldLink)> {
        self.protocol_ids()
            .into_iter()
            .filter_map(|id| self.get_protocol(&id).map(|link| (id, link)))
    }
}

fn kind_tree_insert(map: &mut BTreeMap<String, KindTreeNode>, segments: &[&str], link: IpldLink) {
    match segments {
        [] => {}
        [key] => {
            map.insert((*key).to_string(), KindTreeNode::Leaf(link));
        }
        [key, rest @ ..] => {
            let entry = map
                .entry((*key).to_string())
                .or_insert_with(|| KindTreeNode::Branch(BTreeMap::new()));
            if let KindTreeNode::Branch(inner) = entry {
                kind_tree_insert(inner, rest, link);
            }
        }
    }
}

fn kind_tree_get<'a>(
    map: &'a BTreeMap<String, KindTreeNode>,
    segments: &[&str],
) -> Option<&'a IpldLink> {
    match segments {
        [] => None,
        [key] => match map.get(*key)? {
            KindTreeNode::Leaf(link) => Some(link),
            KindTreeNode::Branch(_) => None,
        },
        [key, rest @ ..] => match map.get(*key)? {
            KindTreeNode::Branch(inner) => kind_tree_get(inner, rest),
            KindTreeNode::Leaf(_) => None,
        },
    }
}

fn kind_tree_remove(map: &mut BTreeMap<String, KindTreeNode>, segments: &[&str]) -> bool {
    match segments {
        [] => false,
        [key] => map.remove(*key).is_some(),
        [key, rest @ ..] => {
            if let Some(KindTreeNode::Branch(inner)) = map.get_mut(*key) {
                let removed = kind_tree_remove(inner, rest);
                if inner.is_empty() {
                    map.remove(*key);
                }
                removed
            } else {
                false
            }
        }
    }
}

fn kind_tree_collect_ids(
    map: &BTreeMap<String, KindTreeNode>,
    prefix: &str,
    ids: &mut Vec<String>,
) {
    for (key, node) in map {
        let path = format!("{prefix}/{key}");
        match node {
            KindTreeNode::Leaf(_) => ids.push(path),
            KindTreeNode::Branch(inner) => kind_tree_collect_ids(inner, &path, ids),
        }
    }
}

// ── IPLD link ─────────────────────────────────────────────────────────────────

/// An IPLD DAG link serialised as `{"/": "bafy..."}`.
///
/// Kubo's `dag/put` endpoint with `input-codec=dag-json` recognises this as a
/// proper IPLD link, enabling `ipfs dag get` traversal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpldLink {
    #[serde(rename = "/")]
    pub cid: String,
}

impl IpldLink {
    pub fn new(cid: impl Into<String>) -> Self {
        Self { cid: cid.into() }
    }
}

// ── Local (intra-runtime) message ─────────────────────────────────────────────

/// An intra-runtime message.  Follows the same schema as `ma_core::Message`
/// but no signature is required — the runtime is the trusted authority for
/// local dispatch. Plugin-facing `from` and `to` values should still be full
/// DID-URLs; the runtime may use fragments internally only for registry lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalMessage {
    pub id: String,
    pub from: String,
    pub to: String,
    /// Unix epoch seconds.
    pub created_at: u64,
    /// Expiry as Unix epoch seconds (0 = never expires). Field name matches
    /// the canonical wire format (ma-messaging-format-v1.md §2, `exp`) and
    /// `ma_core::Message.exp` exactly — not spelled out as `expires`.
    pub exp: u64,
    pub reply_to: Option<String>,
    pub message_type: String,
    pub content_type: String,
    /// CBOR-encoded payload (verb atom or `[":verb", args…]` array).
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
}

// ── Plugin context and I/O ────────────────────────────────────────────────────

/// Which ABI a plugin kind implements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginKind {
    /// Stateless — no state is passed in or out. Replies are sent via the
    /// `ma_send` host function. Exports `on_message`, same as `Stateful`.
    Stateless,
    /// Stateful — runtime passes current state in around the call; plugin
    /// may queue new state bytes via `ma_set_state`. Replies via `ma_send`.
    /// Exports `on_message`, same as `Stateless`.
    Stateful,
}

/// Plugin-facing message — the subset of `LocalMessage` that plugins
/// actually use.
///
/// `created_at`/`exp` were historically excluded here (epoch-second
/// integers in the uint32 range triggered a broken
/// `struct.unpack_from('>I',…)` code path in **extism-py** WASM builds,
/// crashing every `handle_call`). That was a Python-guest-specific bug —
/// the reference Rust guest (`lambda-ma/scheme-actor`, via `ciborium`) has no
/// such issue, and ma-scheme-v1.md §4 requires `msg-created-at`/`msg-exp`
/// accessors, so both fields are reinstated here. A Python plugin built
/// against the old extism-py would need its own fix on that side before
/// relying on these two fields again; unknown fields are otherwise ignored
/// by serde on deserialise either way.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMsg {
    pub id: String,
    pub from: String,
    pub to: String,
    /// Unix epoch seconds.
    pub created_at: u64,
    /// Expiry as Unix epoch seconds (0 = never expires). Matches the wire
    /// format's `exp` field name exactly.
    pub exp: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    pub message_type: String,
    pub content_type: String,
    /// CBOR-encoded payload (verb atom or `[":verb", args…]` array).
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
}

impl From<&LocalMessage> for PluginMsg {
    fn from(m: &LocalMessage) -> Self {
        Self {
            id: m.id.clone(),
            from: m.from.clone(),
            to: m.to.clone(),
            created_at: m.created_at,
            exp: m.exp,
            reply_to: m.reply_to.clone(),
            message_type: m.message_type.clone(),
            content_type: m.content_type.clone(),
            content: m.content.clone(),
        }
    }
}

/// Input passed (CBOR-encoded) to both `handle_cast` (stateless) and
/// `handle_call` (stateful) exports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastInput {
    pub msg: PluginMsg,
}

/// Minimal message reference carried by `ma_reply` — the runtime only needs
/// the sender DID and the original message ID for routing; full `LocalMessage`
/// fields (u64 timestamps, content bytes) are not required and must NOT be
/// encoded by plugins to avoid triggering CBOR encoding of large u64 values
/// via `struct.pack` in Python WASM builds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgRef {
    pub id: String,
    pub from: String,
}

impl From<&LocalMessage> for MsgRef {
    fn from(m: &LocalMessage) -> Self {
        Self {
            id: m.id.clone(),
            from: m.from.clone(),
        }
    }
}

/// Input for the `ma_reply` host function.
///
/// Plugin passes back only `id` and `from` from the original message;
/// the runtime fills in `to` and `reply_to` automatically.
/// `content_type` is the MIME type of the reply body.
/// Serde ignores any extra fields — existing WASMs that send the full
/// `LocalMessage` map continue to work without rebuilding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyRequest {
    /// Minimal reference to the original message — only `id` and `from` used.
    pub msg: MsgRef,
    /// MIME type of the reply body.
    pub content_type: String,
    /// Serialised reply body bytes.
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
}

// ── IPLD schema types ─────────────────────────────────────────────────────────

/// IPLD node representing a kind (Wasm ABI contract).
///
/// Every kind's Wasm module exports exactly two functions, always:
/// `on_message` (incoming messages) and `on_signal` (every
/// runtime-originated lifecycle event — `:set-state`/`:set-behaviour`/
/// `:init`/`:start`/`:shutdown`, ma-scheme-v1.md §3). There is
/// deliberately no field here declaring which exports a kind provides or
/// which lifecycle stages it uses (an earlier draft had `api`/`lifecycle`
/// fields for exactly that) — whether a given signal does anything for a
/// given entity is determined purely by data availability (does state
/// exist? does a behaviour reference exist? is this genesis?), never by
/// anything a `KindNode` declares.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KindNode {
    pub protocol: String,
    /// IPLD link to the compiled Wasm module bytes shared by every entity of
    /// this kind. **Optional** — absent for kinds where each entity supplies
    /// its *own*, distinct Wasm binary instead (e.g. a generic
    /// "bring-your-own-compiled-actor" kind like `/ma/python/actor/0.0.1`):
    /// for those, `EntityNode.behaviour` holds that entity's own Wasm bytes
    /// directly (instantiated as-is, never interpreted as text) and this
    /// field is omitted entirely. When present, `KindNode.behaviour` and
    /// `EntityNode.behaviour` are interpreted source text layers fed to the
    /// shared binary (e.g. the ma-scheme case) — never raw Wasm bytes in
    /// that case.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<IpldLink>,
    /// How the runtime executes Wasm bytes for this kind. Serialised as
    /// `type` (was `evaluator` in an earlier draft of this spec — same
    /// field, renamed).
    #[serde(rename = "type", default)]
    pub kind_type: Evaluator,
    /// Optional kind-level behaviour source. Only meaningful when `cid`
    /// (above) is `Some`: the linked text is loaded before any per-entity
    /// `EntityNode.behaviour` text and handed to the shared binary via
    /// `:set-behaviour`. Derived kinds append their behaviour after base
    /// kinds, so a chain can layer stdlib → room defaults → entity source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behaviour: Option<IpldLink>,
    /// Resolved base-first behaviour chain. This is runtime-only state built
    /// by `resolve_kind_extends`; raw `KindNode`s on IPFS still carry only their
    /// own optional `behaviour` link.
    #[serde(default, skip_serializing, skip_deserializing)]
    pub behaviour_chain: Vec<IpldLink>,
    /// Host functions the runtime makes available to plugins of this kind.
    /// Principle of least privilege: only register what the kind actually needs.
    pub host_functions: Vec<String>,
    /// Kind attributes. Required keys: `stateful` (bool), `wasi` (bool).
    /// `stateful: true` means the runtime must load/persist state around
    /// dispatch. Never inferred from the `api` list — the explicit
    /// attribute is the source of truth.
    #[serde(default)]
    pub attributes: BTreeMap<String, serde_json::Value>,
    /// Optional base kind's protocol ID to inherit from (single hop per
    /// document; chains are followed by `resolve_kind_extends`). Lets a
    /// variant kind (e.g. `/ma/genesis/0.0.1`) reuse a generic kind's
    /// `cid`/`host_functions` without repeating them — see
    /// `resolve_kind_extends` for exact merge semantics. Never present on
    /// an already-resolved `KindNode` (cleared once resolved).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extends: Option<String>,
}

impl KindNode {
    /// Whether plugins of this kind require WASI system-call support.
    pub fn wasi(&self) -> bool {
        self.attributes
            .get("wasi")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    }

    /// Whether plugins of this kind are stateful.
    /// Read from the explicit `stateful` attribute — never derived from the `api` list.
    /// Stateful plugins have state loaded before `init()` / `handle_call` and
    /// persisted afterwards; stateless plugins have no such lifecycle.
    pub fn plugin_kind(&self) -> PluginKind {
        if self
            .attributes
            .get("stateful")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        {
            PluginKind::Stateful
        } else {
            PluginKind::Stateless
        }
    }
}

/// Resolve `kind`'s `extends` chain (if any) against `manifest.kinds`,
/// fetching each base kind from IPFS in turn and merging it underneath
/// the derived kind's own fields. A kind with no `extends` is returned
/// unchanged (no fetch performed).
///
/// Merge rules (the derived/more-specific kind's own value always wins
/// where it has one):
/// - `cid`: derived `Some` overrides; derived `None` inherits the base's.
/// - `behaviour`: all links are composed base-first. The resolved kind keeps
///   a runtime-only `behaviour_chain`; the public `behaviour` field remains
///   the most-derived own behaviour link for round-tripping/debug display.
/// - `kind_type`: **never** inherited — always the derived kind's own
///   value (defaults to `Extism` like any other `KindNode`).
/// - `attributes`: key-level merge — a key present on the derived kind
///   overrides that same key on the base; keys the derived kind doesn't
///   mention are inherited from the base. Mirrors the entity-over-kind
///   attribute merge (`effective_attribute`) one layer up.
/// - `host_functions`: additive union (base's entries first, then any of
///   the derived kind's own not already present), deduplicated.
///   There is no "subtract" syntax — a kind that must drop a base host
///   function should not `extends` it.
/// - `protocol`: always the derived kind's own identity, never inherited.
///
/// Follows at most 8 hops before erroring, as a guard against a
/// misconfigured extends cycle; errors if a named base protocol isn't in
/// `manifest.kinds` at all.
pub async fn resolve_kind_extends(
    kubo_url: &str,
    manifest: &RuntimeManifest,
    mut kind: KindNode,
) -> anyhow::Result<KindNode> {
    const MAX_DEPTH: usize = 8;
    let mut depth = 0;
    while let Some(base_protocol) = kind.extends.clone() {
        depth += 1;
        if depth > MAX_DEPTH {
            anyhow::bail!(
                "kind '{}' extends chain exceeds {MAX_DEPTH} hops (possible cycle) at '{base_protocol}'",
                kind.protocol
            );
        }
        let base_link = manifest.kinds.get_protocol(&base_protocol).ok_or_else(|| {
            anyhow::anyhow!(
                "kind '{}' extends unknown base kind '{base_protocol}'",
                kind.protocol
            )
        })?;
        let base: KindNode = crate::kubo::dag_get(kubo_url, &base_link.cid).await?;
        let base_extends = base.extends.clone();
        kind = merge_kind_over_base(base, kind);
        kind.extends = base_extends;
    }
    Ok(kind)
}

fn merge_kind_over_base(base: KindNode, derived: KindNode) -> KindNode {
    let mut host_functions = base.host_functions.clone();
    for f in &derived.host_functions {
        if !host_functions.contains(f) {
            host_functions.push(f.clone());
        }
    }
    let mut attributes = base.attributes.clone();
    attributes.extend(derived.attributes.clone());
    let mut behaviour_chain = if base.behaviour_chain.is_empty() {
        base.behaviour.iter().cloned().collect::<Vec<_>>()
    } else {
        base.behaviour_chain.clone()
    };
    if derived.behaviour_chain.is_empty() {
        if let Some(link) = &derived.behaviour {
            behaviour_chain.push(link.clone());
        }
    } else {
        behaviour_chain.extend(derived.behaviour_chain.clone());
    }
    KindNode {
        protocol: derived.protocol,
        cid: derived.cid.or(base.cid),
        kind_type: derived.kind_type,
        behaviour: derived.behaviour.or(base.behaviour),
        behaviour_chain,
        host_functions,
        attributes,
        extends: None,
    }
}

/// In-memory registry of loaded [`KindNode`]s.  Single source of truth for all
/// kind attributes at runtime.  Populated at bootstrap and updated on kind upsert.
pub type KindRegistry = Arc<RwLock<HashMap<String, Arc<KindNode>>>>;

/// Create a new, empty [`KindRegistry`].
pub fn new_kind_registry() -> KindRegistry {
    Arc::new(RwLock::new(HashMap::new()))
}

/// Entity fragment names reserved by the runtime system.
/// These names cannot be used as entity names.
/// `genesis` is reserved for entities created directly under that exact
/// fragment name — a runtime convention, not a hardcoded requirement;
/// other entities of a genesis-attribute kind may still be created under
/// other names via CRUD/`ma_create_entity`.
pub const RESERVED_ENTITY_NAMES: &[&str] = &["root", "acl", "scheduler", "runtime", "genesis"];

/// Outcome of a plugin load — **not** persisted anywhere. Purely the
/// transient result `EntityPlugin::load` hands back to its caller so it
/// knows whether to keep the loaded entity or discard/report it (e.g.
/// `ma_create_entity` discards a brand-new entity whose `:init` signal
/// returned `[:error, …]`). Genesis-ness itself is tracked in the
/// background via `EntityNode.initialised` (below) — never via this type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Lifecycle {
    #[default]
    Running,
    Error,
}

impl std::fmt::Display for Lifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Running => write!(f, "running"),
            Self::Error => write!(f, "error"),
        }
    }
}

/// How the runtime executes the plugin bytes for a [`KindNode`].
///
/// Stored in `KindNode.kind_type` (serialised as `type`); defaults to
/// [`Extism`](Evaluator::Extism).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Evaluator {
    #[default]
    Extism,
    Native,
    Bash,
    Lua,
}

/// IPLD node representing a single entity.
///
/// Access is controlled by the entity-level ACL.  WASI support is derived
/// from the `kind` protocol string at plugin-load time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityNode {
    pub kind: String,
    /// IPLD link to this entity's own behaviour source. For shared-binary
    /// scriptable kinds this text is appended after all kind-level behaviour
    /// layers. For kinds with no shared `KindNode.cid`, this is the entity's
    /// own Wasm binary instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behaviour: Option<IpldLink>,
    /// Entity verb-ACL — name string resolved via `acls.<name>` in the root
    /// manifest (e.g. `"fortune"`). Cached under `"acls.<name>"` at startup.
    /// Empty string means deny-all (fail-closed).
    #[serde(default)]
    pub acl: String,
    /// IPLD link to persisted state (optional).
    /// Omitted when absent, which is the expected shape for stateless entities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<IpldLink>,
    /// DID-URL of this entity's parent in the entity tree.
    /// Absent for `#root` (tree root). Used to derive ACL and cascade delete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Human-readable label for display purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Entity-level attribute overrides — merged over `KindNode.attributes`
    /// at read time (entity wins on key collision), never persisted
    /// merged. Lets a single generic kind (e.g. `/ma/scheme/actor/0.0.1`)
    /// serve special-purpose entities without a dedicated kind
    /// registration — e.g. `{"genesis": true}` marks this entity as a
    /// tree root: creating it requires an owner and forces `parent` to
    /// `None`, regardless of what the kind itself declares. See
    /// [`effective_attribute`].
    #[serde(default)]
    pub attributes: BTreeMap<String, serde_json::Value>,
    /// Opaque, persisted creation payload — raw ma-scheme source text,
    /// evaluated verbatim via the `:init` signal on this entity's very
    /// first load only (gated by `initialised`, not by this field's
    /// presence). Unlike `CreateEntityRequest.init_payload` (the
    /// `ma_create_entity`/`ma-create-actor` transient argument), this is
    /// part of the entity's own published document, so entities created
    /// directly via CRUD/bootstrap (which have no calling actor to supply
    /// a transient payload) can still seed instance-specific data. Left
    /// in place after use — harmless, since `:init` only ever fires once
    /// regardless of whether this field is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub init: Option<String>,
    /// Whether this entity has ever completed a successful genesis load
    /// (the `:set-state`/`:set-behaviour`/`:init`/`:start` signal
    /// sequence, all delivered through `on_signal`). Defaults to `false`
    /// — never something a caller needs to declare when authoring an
    /// `EntityNode`; omitting it entirely means "brand new", which is
    /// exactly the common case. The runtime flips it to `true` and
    /// republishes the node itself, in the background, immediately after
    /// a successful genesis load — never something a caller manages by
    /// hand. Not a status/history field: once `true` it never reverts
    /// (there is no `Stopped`/`Error` tracked here) — that information is
    /// purely transient, see [`Lifecycle`].
    #[serde(default)]
    pub initialised: bool,
}

/// Read attribute `key`, checking `entity.attributes` first and falling
/// back to `kind.attributes` — the entity-over-kind override mechanism
/// described on [`EntityNode::attributes`]. Neither map is mutated; the
/// merge happens only at read time.
pub fn effective_attribute<'a>(
    kind: &'a KindNode,
    entity: &'a EntityNode,
    key: &str,
) -> Option<&'a serde_json::Value> {
    entity
        .attributes
        .get(key)
        .or_else(|| kind.attributes.get(key))
}

/// Whether `entity` (of `kind`) is marked as a genesis/tree-root entity —
/// `true` if either the entity or its kind declares `"genesis": true`
/// (entity wins on conflict). Genesis entities require an owner to create
/// and always get `parent: None`, enforced at CRUD upsert time
/// (`crud/entities.rs`) regardless of what the caller supplied.
pub fn is_genesis_entity(kind: &KindNode, entity: &EntityNode) -> bool {
    effective_attribute(kind, entity, "genesis")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
}

/// Root IPLD node for this runtime.
/// Stored as CID in `config.yaml` and published into the DID document under
/// `ma.runtime`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuntimeManifest {
    /// Transport-level ACL — IPLD link to an ACL document.
    /// Loaded once at startup. When absent, falls back to the YAML-based
    /// ACL supplied via `--acl-file`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acl: Option<IpldLink>,
    /// Root-level verb-ACL library — name → IPLD link to `AclMap`.
    /// Used by root entities (`entity.acl` resolves here).
    /// Each entry cached at startup under `"acls.<name>"`.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub acls: HashMap<String, IpldLink>,
    /// Runtime protocol identifier (e.g. `"/ma/runtime/0.1.0"`).
    #[serde(default)]
    pub protocol: String,
    /// Global kinds registry. Shared across all namespaces.
    #[serde(default)]
    pub kinds: KindTree,
    /// Global entity registry — bare name → IPLD link to [`EntityNode`].
    ///
    /// Keys are globally unique entity names (e.g. `"fortune"`, `"rms"`).
    /// No `#` prefix. The corresponding DID fragment equals the key:
    /// `did:ma:<ipns>#fortune` ↔ `entities["fortune"]`.
    pub entities: HashMap<String, IpldLink>,
    #[serde(default)]
    pub i18n: HashMap<String, IpldLink>,
    /// Named group registry — name → IPLD link to a flat `Vec<String>` of
    /// member DIDs. Referenced from any `AclMap` as principal `+<name>`,
    /// CRUD-addressed as `/grp/<name>`.
    ///
    /// The `"owners"` entry is the runtime's authoritative owner list — same
    /// storage as any other group, no special resolution logic — but it is
    /// protected against deletion (see `crud/grp.rs`): the entry may be set
    /// to point at an empty list, but the key itself may never be removed.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub grp: HashMap<String, IpldLink>,
    #[serde(default)]
    pub config: BTreeMap<String, serde_yaml::Value>,
}

// ── Plugin host-function I/O ──────────────────────────────────────────────────

/// Outbound message queued by a plugin via the `ma_send` host function.
///
/// `to`           — recipient DID (or DID-URL).
/// `content_type` — MIME type of the payload (e.g. `application/cbor`).
/// `message_type` — envelope routing type (e.g. `application/vnd.ma.chat`).  If
///                  `None` the runtime defaults to `MESSAGE_TYPE_RPC`.  The
///                  protocol used for delivery is derived from this field; see
///                  `eventloop::protocol_for`.
/// `reply_to`     — if set, marks this as a reply; overrides `message_type` to
///                  `MESSAGE_TYPE_RPC_REPLY` and routes via `/ma/rpc/0.0.1`.
/// `content`      — raw payload bytes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEnvelope {
    pub to: String,
    pub content_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
    pub reply_to: Option<String>,
}

/// A request to create a new entity, queued by the `ma_create_entity` host function.
/// Processed by the runtime after the plugin dispatch returns.
#[derive(Debug, Clone)]
pub struct CreateEntityRequest {
    /// Pre-generated nanoid-style fragment for the new entity (8 chars, URL-safe).
    pub fragment: String,
    /// Protocol ID of the kind to instantiate (e.g. `/ma/stateless/ping/0.0.1`).
    pub kind_protocol: String,
    /// Optional per-entity behaviour source CID — becomes `EntityNode.behaviour`.
    /// For shared-binary scriptable kinds, this is appended after kind-level
    /// behaviour layers.
    pub behaviour_cid: Option<String>,
    /// Optional, opaque creation payload passed verbatim via the `:init`
    /// signal on this entity's very first load. Not persisted — discarded
    /// after that call.
    pub init_payload: Option<Vec<u8>>,
    /// Fragment of the creating (parent) entity.
    pub parent: String,
}

#[derive(Debug, Clone)]
pub struct SetBehaviourRequest {
    pub fragment: String,
    pub behaviour_cid: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{
        effective_attribute, is_genesis_entity, resolve_kind_extends, EntityNode, Evaluator,
        IpldLink, KindNode, KindTree, RuntimeManifest,
    };
    use std::collections::BTreeMap;

    #[test]
    fn kind_tree_nested_serialisation() {
        let mut tree = KindTree::default();
        tree.insert_protocol("/ma/stateless/python/0.0.1", IpldLink::new("bafyAAA"));
        tree.insert_protocol("/ma/stateful/python/0.0.1", IpldLink::new("bafyBBB"));

        let val = serde_json::to_value(&tree).expect("serialise kind tree");
        // Verify nested structure accessible via path segments.
        assert_eq!(
            val["ma"]["stateless"]["python"]["0.0.1"]["/"], "bafyAAA",
            "stateless python leaf"
        );
        assert_eq!(
            val["ma"]["stateful"]["python"]["0.0.1"]["/"], "bafyBBB",
            "stateful python leaf"
        );
    }

    #[test]
    fn kind_tree_protocol_ids_roundtrip() {
        let mut tree = KindTree::default();
        tree.insert_protocol("/ma/stateless/python/0.0.1", IpldLink::new("bafyAAA"));
        tree.insert_protocol("/ma/stateful/python/0.0.1", IpldLink::new("bafyBBB"));

        let mut ids = tree.protocol_ids();
        ids.sort();
        assert_eq!(
            ids,
            vec!["/ma/stateful/python/0.0.1", "/ma/stateless/python/0.0.1"]
        );
    }

    #[test]
    fn kind_tree_remove_prunes_empty_branches() {
        let mut tree = KindTree::default();
        tree.insert_protocol("/ma/stateless/python/0.0.1", IpldLink::new("bafyAAA"));
        assert!(tree.remove_protocol("/ma/stateless/python/0.0.1"));
        assert!(
            tree.protocol_ids().is_empty(),
            "tree should be empty after remove"
        );
    }

    #[test]
    fn serializing_entity_without_state_omits_state_field() {
        let node = EntityNode {
            kind: "/ma/stateless/python/0.0.1".to_string(),
            behaviour: Some(IpldLink {
                cid: "bafybehaviour".to_string(),
            }),
            acl: String::new(),
            state: None,
            parent: None,
            label: None,
            attributes: BTreeMap::new(),
            init: None,
            initialised: false,
        };

        let value = serde_json::to_value(&node).expect("serialise entity node");
        assert!(
            value.get("state").is_none(),
            "state must be omitted when None"
        );
    }

    #[test]
    fn serializing_entity_always_includes_acl_field() {
        let node = EntityNode {
            kind: "/ma/stateless/python/0.0.1".to_string(),
            behaviour: Some(IpldLink {
                cid: "bafybehaviour".to_string(),
            }),
            acl: String::new(),
            state: None,
            parent: None,
            label: None,
            attributes: BTreeMap::new(),
            init: None,
            initialised: false,
        };
        let value = serde_json::to_value(&node).expect("serialise entity node");
        assert!(
            value.get("acl").is_some(),
            "acl must always be present in serialised form"
        );
    }

    fn plain_kind_node() -> KindNode {
        KindNode {
            protocol: "/ma/test/0.0.1".to_string(),
            cid: None,
            kind_type: Evaluator::Extism,
            behaviour: None,
            behaviour_chain: Vec::new(),
            host_functions: vec![],
            attributes: BTreeMap::new(),
            extends: None,
        }
    }

    fn plain_entity_node() -> EntityNode {
        EntityNode {
            kind: "/ma/test/0.0.1".to_string(),
            behaviour: None,
            acl: String::new(),
            state: None,
            parent: None,
            label: None,
            attributes: BTreeMap::new(),
            init: None,
            initialised: false,
        }
    }

    #[test]
    fn effective_attribute_falls_back_to_kind_when_entity_absent() {
        let mut kind = plain_kind_node();
        kind.attributes
            .insert("genesis".to_string(), serde_json::Value::Bool(true));
        let entity = plain_entity_node();
        assert_eq!(
            effective_attribute(&kind, &entity, "genesis"),
            Some(&serde_json::Value::Bool(true))
        );
    }

    #[test]
    fn effective_attribute_entity_overrides_kind() {
        let mut kind = plain_kind_node();
        kind.attributes
            .insert("genesis".to_string(), serde_json::Value::Bool(false));
        let mut entity = plain_entity_node();
        entity
            .attributes
            .insert("genesis".to_string(), serde_json::Value::Bool(true));
        assert_eq!(
            effective_attribute(&kind, &entity, "genesis"),
            Some(&serde_json::Value::Bool(true))
        );
    }

    #[test]
    fn effective_attribute_absent_on_both_is_none() {
        let kind = plain_kind_node();
        let entity = plain_entity_node();
        assert_eq!(effective_attribute(&kind, &entity, "genesis"), None);
    }

    #[test]
    fn is_genesis_entity_true_via_kind_attribute() {
        let mut kind = plain_kind_node();
        kind.attributes
            .insert("genesis".to_string(), serde_json::Value::Bool(true));
        let entity = plain_entity_node();
        assert!(is_genesis_entity(&kind, &entity));
    }

    #[test]
    fn is_genesis_entity_true_via_entity_attribute_overriding_generic_kind() {
        let kind = plain_kind_node();
        let mut entity = plain_entity_node();
        entity
            .attributes
            .insert("genesis".to_string(), serde_json::Value::Bool(true));
        assert!(is_genesis_entity(&kind, &entity));
    }

    #[test]
    fn is_genesis_entity_false_by_default() {
        let kind = plain_kind_node();
        let entity = plain_entity_node();
        assert!(!is_genesis_entity(&kind, &entity));
    }

    #[test]
    fn is_genesis_entity_entity_can_override_kind_to_false() {
        let mut kind = plain_kind_node();
        kind.attributes
            .insert("genesis".to_string(), serde_json::Value::Bool(true));
        let mut entity = plain_entity_node();
        entity
            .attributes
            .insert("genesis".to_string(), serde_json::Value::Bool(false));
        assert!(!is_genesis_entity(&kind, &entity));
    }

    #[tokio::test]
    async fn resolve_kind_extends_merges_derived_over_base() {
        let kubo = crate::testkubo::MockKubo::start().await;

        let base = KindNode {
            protocol: "/ma/scheme/actor/0.0.1".to_string(),
            cid: Some(IpldLink::new("bafyactorwasm")),
            kind_type: Evaluator::Extism,
            behaviour: Some(IpldLink::new("bafystdlib")),
            behaviour_chain: Vec::new(),
            host_functions: vec!["ma_reply".to_string(), "ma_set_state".to_string()],
            attributes: {
                let mut m = BTreeMap::new();
                m.insert("stateful".to_string(), serde_json::Value::Bool(true));
                m.insert("wasi".to_string(), serde_json::Value::Bool(false));
                m
            },
            extends: None,
        };
        let base_cid = crate::kubo::dag_put(kubo.url(), &base).await.unwrap();

        let mut manifest = RuntimeManifest::default();
        manifest
            .kinds
            .insert_protocol("/ma/scheme/actor/0.0.1", IpldLink::new(&base_cid));

        let derived = KindNode {
            protocol: "/ma/genesis/0.0.1".to_string(),
            cid: None, // inherit base's
            kind_type: Evaluator::Extism,
            behaviour: Some(IpldLink::new("bafyroom")),
            behaviour_chain: Vec::new(),
            host_functions: vec!["ma_create_entity".to_string()], // additive
            attributes: {
                let mut m = BTreeMap::new();
                m.insert("genesis".to_string(), serde_json::Value::Bool(true));
                m
            },
            extends: Some("/ma/scheme/actor/0.0.1".to_string()),
        };

        let resolved = resolve_kind_extends(kubo.url(), &manifest, derived)
            .await
            .expect("resolve extends");

        assert_eq!(
            resolved.protocol, "/ma/genesis/0.0.1",
            "own identity preserved"
        );
        assert_eq!(
            resolved.cid.map(|l| l.cid),
            Some("bafyactorwasm".to_string()),
            "cid inherited from base since derived didn't set one"
        );
        assert_eq!(
            resolved.behaviour.map(|l| l.cid),
            Some("bafyroom".to_string()),
            "most-derived own behaviour remains visible"
        );
        assert_eq!(
            resolved
                .behaviour_chain
                .iter()
                .map(|l| l.cid.as_str())
                .collect::<Vec<_>>(),
            vec!["bafystdlib", "bafyroom"],
            "behaviour_chain composes base first"
        );
        assert_eq!(
            resolved.host_functions,
            vec![
                "ma_reply".to_string(),
                "ma_set_state".to_string(),
                "ma_create_entity".to_string()
            ],
            "host_functions is an additive union, base first"
        );
        assert_eq!(
            resolved.attributes.get("stateful"),
            Some(&serde_json::Value::Bool(true)),
            "stateful inherited from base"
        );
        assert_eq!(
            resolved.attributes.get("genesis"),
            Some(&serde_json::Value::Bool(true)),
            "genesis is the derived kind's own attribute"
        );
        assert!(resolved.extends.is_none(), "extends cleared once resolved");
    }

    #[tokio::test]
    async fn resolve_kind_extends_is_a_noop_when_absent() {
        let kubo = crate::testkubo::MockKubo::start().await;
        let manifest = RuntimeManifest::default();
        let kind = plain_kind_node();
        let resolved = resolve_kind_extends(kubo.url(), &manifest, kind.clone())
            .await
            .unwrap();
        assert_eq!(resolved.protocol, kind.protocol);
        assert_eq!(resolved.host_functions, kind.host_functions);
    }

    #[tokio::test]
    async fn resolve_kind_extends_errors_on_unknown_base() {
        let kubo = crate::testkubo::MockKubo::start().await;
        let manifest = RuntimeManifest::default();
        let mut kind = plain_kind_node();
        kind.extends = Some("/ma/does/not/exist/0.0.1".to_string());
        let err = resolve_kind_extends(kubo.url(), &manifest, kind)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("unknown base kind"), "{err}");
    }

    #[test]
    fn deserializing_entity_accepts_missing_state_field() {
        let raw = r#"{
            "kind": "/ma/stateless/python/0.0.1",
            "behaviour": {"/": "bafybehaviour"},
            "acl": ""
        }"#;

        let node: EntityNode = serde_json::from_str(raw).expect("deserialise entity node");
        assert!(
            node.state.is_none(),
            "missing state should deserialise as None"
        );
    }

    #[test]
    fn deserializing_entity_accepts_null_state_field() {
        let raw = r#"{
            "kind": "/ma/stateless/python/0.0.1",
            "behaviour": {"/": "bafybehaviour"},
            "acl": "",
            "state": null
        }"#;

        let node: EntityNode = serde_json::from_str(raw).expect("deserialise entity node");
        assert!(
            node.state.is_none(),
            "null state should deserialise as None"
        );
    }

    #[test]
    fn deserializing_entity_accepts_missing_behaviour_field() {
        let raw = r#"{
            "kind": "/ma/scheduler/0.0.1",
            "acl": "open"
        }"#;

        let node: EntityNode = serde_json::from_str(raw).expect("deserialise entity node");
        assert!(
            node.behaviour.is_none(),
            "missing behaviour should deserialise as None (native entity)"
        );
    }
}
