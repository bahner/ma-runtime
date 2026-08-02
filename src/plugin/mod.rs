//! Extism-based Wasm plugin wrapper and native entity dispatch.
//!
//! [`EntityPlugin`] supports two backends:
//!
//! - **Extism** — Wasm loaded from IPFS, used for all user-defined entities.
//! - **Native** — a compiled-in Rust closure, used for built-in system entities
//!   such as `#scheduler`.  Native entities are registered via
//!   [`EntityPlugin::new_native`]; [`EntityPlugin::load`] **must not** be
//!   called for [`Evaluator::Native`] kinds.
//!
//! Both backends implement the same dispatch surface: [`EntityPlugin::on_message`].
//! The [`PluginKind`] field determines whether state is threaded in/out (for
//! Extism) or which path the closure takes (for Native) — the Wasm export
//! called is always `on_message`, regardless of statefulness.

use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Context, Result};
use tokio::sync::{mpsc::UnboundedSender, oneshot, RwLock};
use tracing::{debug, info, warn};

use crate::kubo::cat_bytes;

use crate::entity::{
    CastInput, CreateEntityRequest, EntityNode, Evaluator, KindNode, Lifecycle, PluginKind,
    SendEnvelope, SetBehaviourRequest,
};

mod backend;
use backend::{run_native_thread, run_wasm_thread, WasmThreadCfg};

// ── Actor thread model ────────────────────────────────────────────────────────
//
// Every entity runs on its own dedicated OS thread that exclusively owns its
// Wasm `Plugin` (or native closure).  Dispatch is message-passing: callers send
// an [`EntityMsg`] on the entity's channel and receive the result on a oneshot.
//
// This eliminates `block_in_place` entirely: no Wasm call ever parks a Tokio
// worker thread.  Entities communicate exclusively via `ma_send` (fire-and-
// forget).  There is no synchronous inter-entity call primitive.

/// Messages sent to an entity's dedicated worker thread.
enum EntityMsg {
    /// Dispatch a message to the `on_message` export.
    Dispatch {
        /// `true` → stateful (state threaded in/out around the call);
        /// `false` → stateless (no state threading). The export called is
        /// always `on_message` either way.
        stateful: bool,
        input: CastInput,
        reply: oneshot::Sender<Result<DispatchResult>>,
    },
    /// Take the pending (unsaved) state bytes out of the entity's `StateCtx`.
    /// Used by [`EntityPlugin::trigger_save`].
    TakePending {
        reply: oneshot::Sender<Option<Vec<u8>>>,
    },
    /// Record that `bytes` were successfully persisted to IPFS: update the
    /// `persisted` snapshot and clear the dirty flag.
    MarkSaved(Vec<u8>),
    /// Deliver `:shutdown` and return any pending state bytes.
    Shutdown {
        reply: oneshot::Sender<Result<Option<Vec<u8>>>>,
    },
    /// Stop the worker thread and drop the owned plugin/native actor.
    Terminate,
}

// ── Native dispatch type ─────────────────────────────────────────────────────

type NativeDispatchFn = dyn Fn(&CastInput) -> anyhow::Result<DispatchResult> + Send + Sync;
type NativeSignalFn = dyn Fn(NativeSignal) -> anyhow::Result<()> + Send + Sync;
type NativeTakePendingFn = dyn Fn() -> Option<Vec<u8>> + Send + Sync;
type NativeMarkSavedFn = dyn Fn(Vec<u8>) + Send + Sync;
type NativeFactoryFn = dyn Fn() -> NativeActor + Send + Sync;

/// Lifecycle signal delivered to a native entity.
pub enum NativeSignal {
    SetState(Vec<u8>),
    Init(Vec<u8>),
    Start,
    Shutdown,
}

/// Compiled-in Rust backend for a native entity.
pub struct NativeActor {
    dispatch: Arc<NativeDispatchFn>,
    signal: Arc<NativeSignalFn>,
    take_pending: Arc<NativeTakePendingFn>,
    mark_saved: Arc<NativeMarkSavedFn>,
}

impl NativeActor {
    pub fn new(
        dispatch: impl Fn(&CastInput) -> anyhow::Result<DispatchResult> + Send + Sync + 'static,
    ) -> Self {
        Self {
            dispatch: Arc::new(dispatch),
            signal: Arc::new(|_| Ok(())),
            take_pending: Arc::new(|| None),
            mark_saved: Arc::new(|_| {}),
        }
    }

    pub fn with_signal(
        mut self,
        signal: impl Fn(NativeSignal) -> anyhow::Result<()> + Send + Sync + 'static,
    ) -> Self {
        self.signal = Arc::new(signal);
        self
    }

    pub fn with_state_hooks(
        mut self,
        take_pending: impl Fn() -> Option<Vec<u8>> + Send + Sync + 'static,
        mark_saved: impl Fn(Vec<u8>) + Send + Sync + 'static,
    ) -> Self {
        self.take_pending = Arc::new(take_pending);
        self.mark_saved = Arc::new(mark_saved);
        self
    }
}

pub type NativeFactory = Arc<NativeFactoryFn>;
pub type NativeFactories = HashMap<String, NativeFactory>;

// ── Dispatch result ─────────────────────────────────────────────────────────

/// Return value from `on_message`.
pub struct DispatchResult {
    /// Raw CBOR bytes returned by the plugin export. No longer consumed by
    /// production dispatch (entities reply via the `ma_reply` host function
    /// instead, not via this return value) since the removal of the
    /// `+#<fragment>`/`ma-set` actor-probe ACL group mechanism, its only
    /// production consumer. Kept for test diagnostics and potential future
    /// synchronous-reply use cases.
    #[allow(dead_code)]
    pub output: Vec<u8>,
    /// State bytes queued by the plugin via `ma_set_state` host function.
    /// `None` if the plugin did not call `ma_set_state` during this invocation.
    pub pending_state: Option<Vec<u8>>,
    /// Entity creation requests enqueued via `ma_create_entity` host function.
    pub create_requests: Vec<CreateEntityRequest>,
    /// Entity deletion requests enqueued via `ma_delete_entity` host function.
    /// Each entry is the fragment of the entity to delete.
    pub delete_requests: Vec<String>,
    /// Self-behaviour update requests enqueued via `ma_set_behaviour`.
    pub behaviour_requests: Vec<SetBehaviourRequest>,
}

impl DispatchResult {
    fn pending_state_len(&self) -> usize {
        self.pending_state.as_ref().map_or(0, Vec::len)
    }
}

// ── Registry type alias ───────────────────────────────────────────────────────

/// Thread-safe map from fragment name (e.g. `"fortune"`) to its entity handle.
pub type EntityRegistry = Arc<RwLock<HashMap<String, Arc<EntityPlugin>>>>;

pub fn new_entity_registry() -> EntityRegistry {
    Arc::new(RwLock::new(HashMap::new()))
}

pub fn graceful_shutdown_timeout() -> std::time::Duration {
    backend::wasm_call_timeout() * 2
}

// ── EntityPlugin (handle) ─────────────────────────────────────────────────────

/// Handle to an entity running on its own dedicated worker thread.
///
/// The actual Wasm `Plugin` (or native closure) lives on that thread and is
/// never shared; all interaction goes through the [`EntityMsg`] channel.  This
/// struct is a cheap, cloneable handle carrying the entity's metadata plus the
/// channel sender.
pub struct EntityPlugin {
    /// Globally unique entity name (bare, no `#` prefix, no dots).
    /// Matches the key in `RuntimeManifest.entities` and equals the DID fragment:
    /// `did:ma:<ipns>#<fragment>` → lookup `entities[fragment]`.
    pub fragment: String,
    pub kind: PluginKind,
    /// ACL name string — resolved via `acls.<acl>` in the root manifest.
    /// Empty string means deny-all (fail-closed).
    /// For native entities the ACL is applied normally; the runtime does not
    /// bypass entity-level ACLs for compiled-in entities.
    pub acl: String,
    /// Parent fragment — the entity that owns/created this one, if any.
    /// Immutable after creation. Only the parent may delete this entity.
    pub parent: Option<String>,
    /// `true` for compiled-in native entities (e.g. `#scheduler`).
    native: bool,
    /// Channel to the entity's dedicated worker thread.
    tx: UnboundedSender<EntityMsg>,
}

impl EntityPlugin {
    /// Create a native entity plugin backed by a compiled-in Rust closure.
    ///
    /// Use this for built-in system entities (e.g. `#scheduler`) whose
    /// implementation is compiled into the runtime binary.  The closure
    /// receives a [`CastInput`] and returns a [`DispatchResult`].
    ///
    /// The closure runs on a dedicated worker thread that has entered the
    /// current Tokio runtime context, so `tokio::spawn` inside the closure
    /// works.  Must be called from within a Tokio runtime.
    ///
    /// Returns `(plugin, Lifecycle::Running)` — native entities are always
    /// immediately running; they never go through `init()`.
    pub fn new_native(
        fragment: impl Into<String>,
        node: &EntityNode,
        kind_node: &KindNode,
        actor: NativeActor,
        init_state: Vec<u8>,
        init_payload: Option<Vec<u8>>,
    ) -> Result<(Self, Lifecycle)> {
        let fragment = fragment.into();
        let kind = kind_node.plugin_kind();
        let is_genesis = !node.initialised;
        run_native_lifecycle(&fragment, &actor, is_genesis, init_state, init_payload)?;

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<EntityMsg>();
        let handle = tokio::runtime::Handle::current();
        let thread_name = format!("entity-{fragment}");
        std::thread::Builder::new()
            .name(thread_name)
            .spawn(move || run_native_thread(actor, handle, rx))
            .with_context(|| format!("spawning native worker thread for '{fragment}'"))?;

        let ep = Self {
            fragment,
            kind,
            acl: node.acl.clone(),
            parent: node.parent.clone(),
            native: true,
            tx,
        };
        Ok((ep, Lifecycle::Running))
    }

    /// Returns `true` if this plugin is backed by a compiled-in Rust closure.
    pub const fn is_native(&self) -> bool {
        self.native
    }

    /// Load a Wasm plugin from IPFS, spawn its worker thread, and drive it
    /// through the applicable lifecycle signals (`:set-state`/
    /// `:set-behaviour`/`:init`/`:start`) via the single `on_signal` export,
    /// in order.
    ///
    /// The Wasm `Plugin` is created and driven entirely on the dedicated worker
    /// thread — it never crosses a thread boundary — so no `unsafe impl Send`
    /// is required and no Tokio worker is ever blocked by a Wasm call.
    ///
    /// `init_payload` is the opaque creation payload for the `:init` signal
    /// (§14.2.1); pass `None` for an ordinary reload (bootstrap/restart) of
    /// an already-existing entity. It is only meaningful when
    /// `node.initialised` is `false` (this entity's very first load) —
    /// `:init` fires exactly once, ever, for a given entity.
    ///
    /// Returns `(handle, Lifecycle::Running)` on success, or
    /// `(handle, Lifecycle::Error)` if `:init` returned `[:error, …]` (the
    /// entity is still dispatchable for debugging).
    /// Returns `Err` for fatal errors (Wasm fetch / plugin instantiation), or
    /// when `kind_node.kind_type == Evaluator::Native` (use
    /// [`EntityPlugin::new_native`] instead).
    #[allow(clippy::too_many_lines, clippy::too_many_arguments)]
    pub async fn load(
        fragment: impl Into<String>,
        node: &EntityNode,
        kind_node: &KindNode,
        our_did: &str,
        kubo_url: &str,
        envelope_tx: UnboundedSender<(String, SendEnvelope)>,
        entity_registry: EntityRegistry,
        iroh_node_id: &str,
        started_at: u64,
        runtime_config: std::collections::BTreeMap<String, String>,
        init_payload: Option<Vec<u8>>,
    ) -> Result<(Self, Lifecycle)> {
        let fragment = fragment.into();
        let kind = kind_node.plugin_kind();
        let wasi = kind_node.wasi();

        // Native entities must be registered via new_native(), not load().
        if kind_node.kind_type == Evaluator::Native {
            return Err(anyhow!(
                "entity '{fragment}' has kind type 'native': use EntityPlugin::new_native() instead of load()"
            ));
        }

        // Only Extism is supported beyond this point.
        if kind_node.kind_type != Evaluator::Extism {
            return Err(anyhow!(
                "unsupported kind type {:?} for '{fragment}'",
                kind_node.kind_type
            ));
        }

        // Wasm bytes source depends on whether this kind shares one binary
        // across all its entities (`cid` present) or lets each entity supply
        // its own (`cid` absent — the entity's own Wasm bytes live on
        // `EntityNode.behaviour` instead, instantiated as-is, never resolved
        // as interpreted text).
        let (wasm_cid, wasm_bytes, entity_behaviour_cid, behaviour_text) = if let Some(shared_cid) =
            &kind_node.cid
        {
            let wasm_cid = shared_cid.cid.clone();
            info!(fragment = %fragment, cid = %wasm_cid, kind = ?kind, wasi = wasi, "loading entity plugin (shared binary)");
            let wasm_bytes = cat_bytes(kubo_url, &wasm_cid)
                .await
                .with_context(|| format!("fetching wasm for '{fragment}' from {wasm_cid}"))?;

            // Compose kind-level behaviour (base-first, when `extends` was
            // resolved) followed by the entity's own source. The runtime only
            // concatenates bytes; parsing/evaluation still happens inside the
            // actor host via `:set-behaviour`.
            let entity_behaviour_cid = node.behaviour.as_ref().map(|l| l.cid.clone());
            let kind_behaviours = if kind_node.behaviour_chain.is_empty() {
                kind_node.behaviour.iter().cloned().collect::<Vec<_>>()
            } else {
                kind_node.behaviour_chain.clone()
            };
            let kind_behaviour_cids = kind_behaviours
                .iter()
                .map(|link| link.cid.as_str())
                .collect::<Vec<_>>()
                .join(",");
            info!(
                fragment = %fragment,
                kind = %kind_node.protocol,
                kind_behaviour_cids = %kind_behaviour_cids,
                entity_behaviour_cid = ?entity_behaviour_cid,
                "resolved entity behaviour chain"
            );
            let behaviour_text: Option<Vec<u8>> =
                if kind_behaviours.is_empty() && entity_behaviour_cid.is_none() {
                    None
                } else {
                    let mut parts = Vec::new();
                    for link in &kind_behaviours {
                        parts.push(
                            crate::behaviour::fetch_behaviour(kubo_url, &link.cid)
                                .await
                                .with_context(|| {
                                    format!(
                                        "fetching kind behaviour for '{fragment}' from {}",
                                        link.cid
                                    )
                                })?,
                        );
                    }
                    if let Some(cid) = &entity_behaviour_cid {
                        parts.push(
                            crate::behaviour::fetch_behaviour(kubo_url, cid)
                                .await
                                .with_context(|| {
                                    format!("fetching entity behaviour for '{fragment}' from {cid}")
                                })?,
                        );
                    }
                    let mut combined = Vec::new();
                    for part in parts {
                        if !combined.is_empty() && !combined.ends_with(b"\n") {
                            combined.push(b'\n');
                        }
                        combined.extend_from_slice(&part);
                        if !combined.ends_with(b"\n") {
                            combined.push(b'\n');
                        }
                    }
                    Some(combined)
                };
            (wasm_cid, wasm_bytes, entity_behaviour_cid, behaviour_text)
        } else {
            // No shared binary for this kind — the entity must supply its
            // own Wasm bytes via `behaviour`.
            let entity_cid = node.behaviour.as_ref().map(|l| l.cid.clone()).ok_or_else(|| {
                    anyhow!(
                        "entity '{fragment}' has kind '{}' with no shared cid: entity must supply its own Wasm via behaviour",
                        kind_node.protocol
                    )
                })?;
            info!(fragment = %fragment, cid = %entity_cid, kind = ?kind, wasi = wasi, "loading entity plugin (own binary via behaviour)");
            let wasm_bytes = cat_bytes(kubo_url, &entity_cid)
                .await
                .with_context(|| format!("fetching wasm for '{fragment}' from {entity_cid}"))?;
            (entity_cid, wasm_bytes, None, None)
        };

        // For stateful plugins: fetch persisted state so StateCtx has the
        // correct baseline and set_state can restore it.  Stateless plugins
        // have no state; module-level code handles any one-time setup.
        let init_state: Vec<u8> = if kind == PluginKind::Stateful {
            match &node.state {
                Some(link) => cat_bytes(kubo_url, &link.cid).await.unwrap_or_default(),
                None => Vec::new(),
            }
        } else {
            Vec::new()
        };

        let is_genesis = !node.initialised;

        // Assemble all Send-able data the worker thread needs.  The Wasm
        // Plugin and its host Functions are built *on* the thread.
        let cfg = WasmThreadCfg {
            fragment: fragment.clone(),
            our_did: our_did.to_string(),
            wasm_bytes,
            init_state,
            wasi,
            host_functions: kind_node.host_functions.clone(),
            is_genesis,
            init_payload,
            behaviour_text,
            node_kind: node.kind.clone(),
            envelope_tx,
            wasm_cid,
            entity_behaviour_cid,
            kubo_url: kubo_url.to_string(),
            tokio_handle: tokio::runtime::Handle::current(),
            iroh_node_id: iroh_node_id.to_string(),
            started_at,
            parent: node.parent.clone(),
            runtime_config,
            entity_registry,
        };

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<EntityMsg>();
        let (life_tx, life_rx) = oneshot::channel::<Result<Lifecycle>>();
        let thread_name = format!("entity-{fragment}");
        std::thread::Builder::new()
            .name(thread_name)
            .spawn(move || run_wasm_thread(cfg, rx, life_tx))
            .with_context(|| format!("spawning worker thread for '{fragment}'"))?;

        // Wait for the thread to build the plugin and run the genesis/start
        // lifecycle stages.  Bounded: Wasm *execution* is capped by the
        // extism epoch timeout, but plugin *instantiation* (compilation) is
        // not — a pathological module could otherwise hang bootstrap or a
        // reload task forever.
        let load_timeout = backend::wasm_load_timeout();
        let lifecycle = match tokio::time::timeout(load_timeout, life_rx).await {
            Ok(Ok(Ok(lc))) => lc,
            Ok(Ok(Err(e))) => return Err(e),
            Ok(Err(_)) => {
                return Err(anyhow!(
                    "entity '{fragment}' worker thread exited before genesis/start completed"
                ))
            }
            Err(_) => {
                // The worker thread is left to its fate: if it ever finishes
                // building, it exits on its own because `life_tx.send` fails.
                return Err(anyhow!(
                    "entity '{fragment}' plugin build/genesis timed out after {}s",
                    load_timeout.as_secs()
                ));
            }
        };

        let ep = Self {
            fragment,
            kind,
            acl: node.acl.clone(),
            parent: node.parent.clone(),
            native: false,
            tx,
        };
        Ok((ep, lifecycle))
    }

    /// Dispatch to the `on_message` export. Threads state in/out around the
    /// call automatically based on `self.kind` (stateful vs stateless) —
    /// callers never need to branch on kind themselves.
    pub async fn on_message(&self, input: &CastInput) -> Result<DispatchResult> {
        let before_rss = current_rss_kib();
        let message_verb = plugin_message_verb(&input.msg.content);
        debug!(
            fragment = %self.fragment,
            from = %input.msg.from,
            to = %input.msg.to,
            id = %input.msg.id,
            content_len = input.msg.content.len(),
            verb = ?message_verb,
            "plugin dispatch start"
        );
        let result = self
            .dispatch(self.kind == PluginKind::Stateful, input)
            .await;
        match &result {
            Ok(result) => debug!(
                fragment = %self.fragment,
                from = %input.msg.from,
                to = %input.msg.to,
                id = %input.msg.id,
                content_len = input.msg.content.len(),
                verb = ?message_verb,
                pending_state_bytes = result.pending_state_len(),
                create_requests = result.create_requests.len(),
                delete_requests = result.delete_requests.len(),
                behaviour_requests = result.behaviour_requests.len(),
                "plugin dispatch finish"
            ),
            Err(error) => warn!(
                fragment = %self.fragment,
                from = %input.msg.from,
                to = %input.msg.to,
                id = %input.msg.id,
                content_len = input.msg.content.len(),
                verb = ?message_verb,
                error = %error,
                "plugin dispatch failed"
            ),
        }
        if let (Some(before), Some(after)) = (before_rss, current_rss_kib()) {
            let delta = after.saturating_sub(before);
            let threshold = memory_growth_log_threshold_kib();
            if delta >= threshold {
                warn!(
                    fragment = %self.fragment,
                    from = %input.msg.from,
                    to = %input.msg.to,
                    id = %input.msg.id,
                    content_len = input.msg.content.len(),
                    verb = ?message_verb,
                    rss_before_kib = before,
                    rss_after_kib = after,
                    rss_delta_kib = delta,
                    threshold_kib = threshold,
                    "plugin dispatch increased process RSS"
                );
            }
        }
        result
    }

    /// Send a dispatch to the worker thread and await the result.
    ///
    /// Bounded by a backstop timeout slightly above the Wasm execution cap:
    /// no caller can ever wait forever on a wedged worker, even if the epoch
    /// interrupt itself were to fail.
    async fn dispatch(&self, stateful: bool, input: &CastInput) -> Result<DispatchResult> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.tx
            .send(EntityMsg::Dispatch {
                stateful,
                input: input.clone(),
                reply: reply_tx,
            })
            .map_err(|_| anyhow!("entity '{}' worker thread is gone", self.fragment))?;
        // 2× the Wasm cap: the dispatch may sit behind one already-running
        // call (≤ 1× cap) before its own execution starts (≤ 1× cap).
        let backstop = backend::wasm_call_timeout() * 2;
        tokio::time::timeout(backstop, reply_rx).await.map_or_else(
            |_| {
                Err(anyhow!(
                    "entity '{}' dispatch timed out after {}s (worker wedged?)",
                    self.fragment,
                    backstop.as_secs()
                ))
            },
            |reply| {
                reply.unwrap_or_else(|_| {
                    Err(anyhow!("entity '{}' dropped dispatch reply", self.fragment))
                })
            },
        )
    }

    /// Record a successful IPFS persist: update the persisted snapshot and
    /// clear the dirty flag.
    pub fn mark_saved(&self, saved_bytes: Vec<u8>) {
        let _ = self.tx.send(EntityMsg::MarkSaved(saved_bytes));
    }

    /// Persist any state queued by `ma_set_state` during the last dispatch.
    ///
    /// Plugins call `ma_set_state` reactively inside `on_message`; this method
    /// flushes whatever is still queued to IPFS and returns the resulting CID.
    /// Returns `Ok(None)` when there is no pending state (nothing to save).
    pub async fn trigger_save(&self, kubo_url: &str) -> Result<Option<String>> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.tx
            .send(EntityMsg::TakePending { reply: reply_tx })
            .map_err(|_| anyhow!("entity '{}' worker thread is gone", self.fragment))?;
        // Bounded: the TakePending may sit behind one wedged dispatch
        // (≤ 1× Wasm cap) — never wait forever (shutdown path uses this).
        let backstop = backend::wasm_call_timeout() * 2;
        let pending = match tokio::time::timeout(backstop, reply_rx).await {
            Ok(reply) => reply
                .map_err(|_| anyhow!("entity '{}' dropped TakePending reply", self.fragment))?,
            Err(_) => {
                return Err(anyhow!(
                    "entity '{}' TakePending timed out after {}s (worker wedged?)",
                    self.fragment,
                    backstop.as_secs()
                ))
            }
        };

        if let Some(bytes) = pending {
            let cid = crate::kubo::ipfs_add_bytes_unpinned(kubo_url, bytes.clone())
                .await
                .map_err(|e| anyhow!("ipfs_add for '{}' state: {e}", self.fragment))?;
            self.mark_saved(bytes);
            Ok(Some(cid))
        } else {
            Ok(None)
        }
    }

    async fn shutdown_signal_save(
        &self,
        kubo_url: &str,
        backstop: std::time::Duration,
    ) -> Result<Option<String>> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.tx
            .send(EntityMsg::Shutdown { reply: reply_tx })
            .map_err(|_| anyhow!("entity '{}' worker thread is gone", self.fragment))?;
        let pending = match tokio::time::timeout(backstop, reply_rx).await {
            Ok(reply) => reply
                .map_err(|_| anyhow!("entity '{}' dropped Shutdown reply", self.fragment))??,
            Err(_) => {
                return Err(anyhow!(
                    "entity '{}' Shutdown timed out after {}s (worker wedged?)",
                    self.fragment,
                    backstop.as_secs()
                ))
            }
        };

        if let Some(bytes) = pending {
            let cid = crate::kubo::ipfs_add_bytes_unpinned(kubo_url, bytes.clone())
                .await
                .map_err(|e| anyhow!("ipfs_add for '{}' shutdown state: {e}", self.fragment))?;
            self.mark_saved(bytes);
            Ok(Some(cid))
        } else {
            Ok(None)
        }
    }

    /// Deliver the entity's shutdown signal and persist any state it queues,
    /// but keep the worker alive until the replacement entity is ready.
    pub async fn prepare_reload_save(
        &self,
        kubo_url: &str,
        backstop: std::time::Duration,
    ) -> Result<Option<String>> {
        self.shutdown_signal_save(kubo_url, backstop).await
    }

    /// Stop this entity's worker thread after it has been removed from the
    /// live registry or superseded by a replacement plugin.
    pub fn terminate_worker(&self) {
        let _ = self.tx.send(EntityMsg::Terminate);
    }
}

fn plugin_message_verb(content: &[u8]) -> Option<String> {
    match ciborium::de::from_reader::<ciborium::Value, _>(content) {
        Ok(ciborium::Value::Text(verb)) => Some(verb),
        Ok(ciborium::Value::Array(items)) => match items.first() {
            Some(ciborium::Value::Text(verb)) => Some(verb.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn memory_growth_log_threshold_kib() -> u64 {
    std::env::var("MA_MEMORY_GROWTH_LOG_THRESHOLD_KIB")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(16 * 1024)
}

fn current_rss_kib() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    status.lines().find_map(|line| {
        let value = line.strip_prefix("VmRSS:")?;
        value.split_whitespace().next()?.parse().ok()
    })
}

fn run_native_lifecycle(
    fragment: &str,
    actor: &NativeActor,
    is_genesis: bool,
    init_state: Vec<u8>,
    init_payload: Option<Vec<u8>>,
) -> Result<()> {
    if !init_state.is_empty() {
        (actor.signal)(NativeSignal::SetState(init_state))
            .with_context(|| format!("native on_signal(:set-state) failed for '{fragment}'"))?;
    }
    if is_genesis {
        (actor.signal)(NativeSignal::Init(init_payload.unwrap_or_default()))
            .with_context(|| format!("native on_signal(:init) failed for '{fragment}'"))?;
    }
    (actor.signal)(NativeSignal::Start)
        .with_context(|| format!("native on_signal(:start) failed for '{fragment}'"))?;
    Ok(())
}

#[cfg(test)]
mod hostile {
    //! Hostile-plugin isolation tests: no matter how spectacularly a Wasm
    //! module misbehaves at load or runtime, it must never block anything
    //! beyond itself, and every failure must surface as a bounded `Err`.
    //!
    //! Uses [`crate::testkubo::MockKubo`] to serve the Wasm bytes — no real
    //! Kubo or network needed.

    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};
    use std::time::{Duration, Instant};

    use anyhow::{bail, Context as _};

    use crate::entity::{
        CastInput, EntityNode, Evaluator, IpldLink, KindNode, Lifecycle, PluginKind, PluginMsg,
        SendEnvelope,
    };
    use crate::testkubo::MockKubo;

    use super::{new_entity_registry, DispatchResult, EntityPlugin, NativeActor, NativeSignal};

    /// A module whose every export spins forever — used to prove that an
    /// infinite loop in `on_signal` (fired unconditionally at load time for
    /// the `:start` signal) fails within the timeout bound instead of
    /// hanging the load.
    const EVIL_WAT: &str = r#"
        (module
          (func $spin (result i32) (loop $l (br $l)) (i32.const 0))
          (export "on_signal" (func $spin))
          (export "on_message" (func $spin)))
    "#;

    /// A module whose `on_signal` returns immediately but whose
    /// `on_message` spins forever — used to prove that a load succeeds
    /// (since `on_signal` is fast) while dispatches to that entity hang in
    /// isolation, without affecting any other entity.
    const EVIL_ON_MESSAGE_ONLY_WAT: &str = r#"
        (module
          (func $ok (result i32) (i32.const 0))
          (func $spin (result i32) (loop $l (br $l)) (i32.const 0))
          (export "on_signal" (func $ok))
          (export "on_message" (func $spin)))
    "#;

    /// A module whose exports return immediately.
    const GOOD_WAT: &str = r#"
        (module
          (func $ok (result i32) (i32.const 0))
          (export "on_signal" (func $ok))
          (export "on_message" (func $ok)))
    "#;

    fn kind_node(wasm_cid: &str) -> KindNode {
        let mut attributes = BTreeMap::new();
        attributes.insert("stateful".to_string(), serde_json::Value::Bool(true));
        attributes.insert("wasi".to_string(), serde_json::Value::Bool(false));
        KindNode {
            protocol: "/ma/test/0.0.1".to_string(),
            cid: Some(IpldLink::new(wasm_cid)),
            kind_type: Evaluator::Extism,
            behaviour: None,
            behaviour_chain: Vec::new(),
            host_functions: vec![],
            attributes,
            extends: None,
        }
    }

    fn native_kind_node() -> KindNode {
        let mut attributes = BTreeMap::new();
        attributes.insert("stateful".to_string(), serde_json::Value::Bool(true));
        KindNode {
            protocol: "/ma/test/native/0.0.1".to_string(),
            cid: None,
            kind_type: Evaluator::Native,
            behaviour: None,
            behaviour_chain: Vec::new(),
            host_functions: vec![],
            attributes,
            extends: None,
        }
    }

    fn entity_node() -> EntityNode {
        EntityNode {
            kind: "/ma/test/0.0.1".to_string(),
            behaviour: None,
            acl: "open".to_string(),
            state: None,
            parent: None,
            label: None,
            attributes: BTreeMap::new(),
            init: None,
            initialised: false,
            reload_error: None,
        }
    }

    fn cast_input(id: &str) -> CastInput {
        let mut content = Vec::new();
        ciborium::ser::into_writer(&ciborium::Value::Text(":poke".into()), &mut content).unwrap();
        CastInput {
            msg: PluginMsg {
                id: id.to_string(),
                from: "did:ma:tester".to_string(),
                to: "did:ma:testrunner#x".to_string(),
                created_at: 0,
                exp: 0,
                reply_to: None,
                message_type: ma_core::MESSAGE_TYPE_RPC.to_string(),
                content_type: ma_core::CONTENT_TYPE_TERM.to_string(),
                content,
            },
        }
    }

    fn cast_input_with_content(id: &str, content_value: &ciborium::Value) -> CastInput {
        let mut content = Vec::new();
        ciborium::ser::into_writer(content_value, &mut content).unwrap();
        CastInput {
            msg: PluginMsg {
                id: id.to_string(),
                from: "did:ma:testrunner#construct".to_string(),
                to: "did:ma:testrunner#duckie".to_string(),
                created_at: 0,
                exp: 0,
                reply_to: None,
                message_type: ma_core::MESSAGE_TYPE_RPC.to_string(),
                content_type: ma_core::CONTENT_TYPE_TERM.to_string(),
                content,
            },
        }
    }

    fn lambda_ma_fixture_dir() -> anyhow::Result<Option<PathBuf>> {
        if let Some(dir) = std::env::var_os("LAMBDA_MA_DIR") {
            let dir = PathBuf::from(dir);
            let wasm = dir.join("scheme-actor/actor.wasm");
            if !wasm.is_file() {
                bail!(
                    "LAMBDA_MA_DIR points at '{}', but '{}' does not exist; build lambda-ma/scheme-actor first",
                    dir.display(),
                    wasm.display()
                );
            }
            return Ok(Some(dir));
        }

        let dir = PathBuf::from("../lambda-ma");
        if dir.join("scheme-actor/actor.wasm").is_file() {
            Ok(Some(dir))
        } else {
            Ok(None)
        }
    }

    fn read_lambda_ma_fixture(lambda_ma_dir: &Path, relative: &str) -> anyhow::Result<Vec<u8>> {
        let path = lambda_ma_dir.join(relative);
        std::fs::read(&path).with_context(|| format!("read lambda-ma fixture '{}'", path.display()))
    }

    fn read_lambda_ma_text(lambda_ma_dir: &Path, relative: &str) -> anyhow::Result<String> {
        let path = lambda_ma_dir.join(relative);
        std::fs::read_to_string(&path)
            .with_context(|| format!("read lambda-ma fixture '{}'", path.display()))
    }

    async fn load(
        kubo_url: &str,
        fragment: &str,
        cid: &str,
        envelope_tx: tokio::sync::mpsc::UnboundedSender<(String, SendEnvelope)>,
        registry: super::EntityRegistry,
    ) -> anyhow::Result<(EntityPlugin, Lifecycle)> {
        EntityPlugin::load(
            fragment,
            &entity_node(),
            &kind_node(cid),
            "did:ma:testrunner",
            kubo_url,
            envelope_tx,
            registry,
            "",
            0,
            std::collections::BTreeMap::new(),
            None,
        )
        .await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn scheme_actor_wasm_dispatches_report_parent_with_args_first() -> anyhow::Result<()> {
        let Some(lambda_ma_dir) = lambda_ma_fixture_dir()? else {
            eprintln!(
                "skipping lambda-ma scheme actor fixture test; set LAMBDA_MA_DIR to a checkout with scheme-actor/actor.wasm"
            );
            return Ok(());
        };

        let kubo = MockKubo::start().await;
        let (envelope_tx, mut envelope_rx) =
            tokio::sync::mpsc::unbounded_channel::<(String, SendEnvelope)>();
        let registry = new_entity_registry();
        let wasm = read_lambda_ma_fixture(&lambda_ma_dir, "scheme-actor/actor.wasm")?;
        let wasm_cid = kubo.add_bytes(wasm).await;

        let behaviour = [
            read_lambda_ma_text(&lambda_ma_dir, "scheme-actor/stdlib.ma")?,
            read_lambda_ma_text(&lambda_ma_dir, "scheme-actor/actor.ma")?,
            read_lambda_ma_text(&lambda_ma_dir, "scheme-actor/state.ma")?,
            read_lambda_ma_text(&lambda_ma_dir, "actors/agent.ma")?,
            read_lambda_ma_text(&lambda_ma_dir, "actors/duck.ma")?,
        ]
        .join("\n");
        let behaviour_cid = kubo.add_bytes(behaviour.into_bytes()).await;

        let mut node = entity_node();
        node.kind = "/ma/scheme/agent/0.0.1".to_string();
        node.behaviour = None;
        node.initialised = true;
        let mut kind = kind_node(&wasm_cid);
        kind.protocol = "/ma/scheme/agent/0.0.1".to_string();
        kind.behaviour_chain = vec![IpldLink::new(&behaviour_cid)];
        kind.host_functions = vec![
            "ma_reply".to_string(),
            "ma_send".to_string(),
            "ma_set_state".to_string(),
            "ma_ipfs_include".to_string(),
            "ma_create_entity".to_string(),
            "ma_entity_exists".to_string(),
            "ma_end".to_string(),
            "ma_set_behaviour".to_string(),
        ];

        let (plugin, _) = EntityPlugin::load(
            "duckie",
            &node,
            &kind,
            "did:ma:testrunner",
            kubo.url(),
            envelope_tx,
            registry,
            "",
            1,
            std::collections::BTreeMap::new(),
            None,
        )
        .await
        .expect("scheme actor wasm must load");

        let input = cast_input_with_content(
            "report-parent-1",
            &ciborium::Value::Array(vec![
                ciborium::Value::Text(":report-parent".into()),
                ciborium::Value::Text("did:ma:testrunner#construct".into()),
                ciborium::Value::Text("tick-1".into()),
                ciborium::Value::Text("nonce-1".into()),
            ]),
        );
        plugin
            .on_message(&input)
            .await
            .expect("report-parent dispatch");

        let mut saw_parent_report = false;
        while let Ok((_fragment, envelope)) = envelope_rx.try_recv() {
            if envelope.to == "did:ma:testrunner#construct" {
                saw_parent_report = true;
            }
        }
        assert!(
            saw_parent_report,
            "agent should answer room with parent report"
        );
        Ok(())
    }

    /// One combined test (not parallel-safe pieces): the Wasm timeout env var
    /// is process-global, so all hostile scenarios run under one setting.
    #[tokio::test(flavor = "multi_thread")]
    #[allow(clippy::too_many_lines)]
    async fn hostile_wasm_never_blocks_anything_else() {
        std::env::set_var("MA_WASM_CALL_TIMEOUT_SECS", "2");
        let kubo = MockKubo::start().await;
        let (envelope_tx, _envelope_rx) =
            tokio::sync::mpsc::unbounded_channel::<(String, SendEnvelope)>();
        let registry = new_entity_registry();

        let evil_cid = kubo.add_bytes(wat::parse_str(EVIL_WAT).unwrap()).await;
        let evil_message_cid = kubo
            .add_bytes(wat::parse_str(EVIL_ON_MESSAGE_ONLY_WAT).unwrap())
            .await;
        let good_cid = kubo.add_bytes(wat::parse_str(GOOD_WAT).unwrap()).await;
        let garbage_cid = kubo.add_bytes(b"this is not wasm at all".to_vec()).await;

        // ── 1. Garbage bytes: load fails cleanly and quickly. ────────────────
        let t = Instant::now();
        let res = load(
            kubo.url(),
            "garbage",
            &garbage_cid,
            envelope_tx.clone(),
            registry.clone(),
        )
        .await;
        assert!(res.is_err(), "garbage wasm must fail to load");
        assert!(
            t.elapsed() < Duration::from_secs(5),
            "garbage load not bounded: {:?}",
            t.elapsed()
        );

        // ── 2. Infinite loop in on_signal(:start): load fails within the
        //       bound. `:start` fires unconditionally on every load. ────────
        let t = Instant::now();
        let res = load(
            kubo.url(),
            "evil-init",
            &evil_cid,
            envelope_tx.clone(),
            registry.clone(),
        )
        .await;
        assert!(
            res.is_err(),
            "infinite on_signal(:start) must fail, not hang"
        );
        assert!(
            t.elapsed() < Duration::from_secs(10),
            "infinite on_signal(:start) not bounded: {:?}",
            t.elapsed()
        );

        // ── 3. Infinite loop in on_message only: load succeeds quickly
        //       (on_signal is fast), dispatch errors within the bound, and a
        //       healthy entity is fully responsive meanwhile. ────────────────
        let (evil, _) = load(
            kubo.url(),
            "evil",
            &evil_message_cid,
            envelope_tx.clone(),
            registry.clone(),
        )
        .await
        .expect("evil load (fast on_signal, spinning on_message) should succeed");
        let (good, _) = load(
            kubo.url(),
            "good",
            &good_cid,
            envelope_tx.clone(),
            registry.clone(),
        )
        .await
        .expect("good load should succeed");
        let evil = std::sync::Arc::new(evil);
        registry
            .write()
            .await
            .insert("evil".to_string(), evil.clone());

        // Kick off the wedging dispatch.
        let evil_clone = evil.clone();
        let wedged =
            tokio::spawn(async move { evil_clone.on_message(&cast_input("wedge-1")).await });

        // While evil spins, the good entity must answer immediately.
        tokio::time::sleep(Duration::from_millis(200)).await;
        let t = Instant::now();
        good.on_message(&cast_input("good-1"))
            .await
            .expect("good entity must not be affected by evil one");
        assert!(
            t.elapsed() < Duration::from_secs(1),
            "good entity was starved: {:?}",
            t.elapsed()
        );

        // The wedged dispatch must come back as a bounded error.
        let t = Instant::now();
        let res = wedged.await.expect("join");
        assert!(res.is_err(), "infinite on_message must error, got Ok");
        assert!(
            t.elapsed() < Duration::from_secs(10),
            "wedged dispatch not bounded: {:?}",
            t.elapsed()
        );

        // ── 4. The evil worker survives the epoch abort and stays bounded. ───
        let t = Instant::now();
        let res2 = evil.on_message(&cast_input("wedge-2")).await;
        assert!(res2.is_err(), "second dispatch must also error");
        assert!(
            t.elapsed() < Duration::from_secs(10),
            "second wedged dispatch not bounded: {:?}",
            t.elapsed()
        );

        // ── 5. Reload over a wedged entity works: replace registry entry. ───
        let (evil2, _) = load(
            kubo.url(),
            "evil",
            &good_cid, // "fixed" version
            envelope_tx.clone(),
            registry.clone(),
        )
        .await
        .expect("reload over wedged entity should succeed");
        registry
            .write()
            .await
            .insert("evil".to_string(), std::sync::Arc::new(evil2));
        let reloaded = registry.read().await.get("evil").cloned().unwrap();
        reloaded
            .on_message(&cast_input("post-reload"))
            .await
            .expect("reloaded (fixed) entity must dispatch fine");

        std::env::remove_var("MA_WASM_CALL_TIMEOUT_SECS");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn prepare_reload_save_respects_short_timeout() {
        let actor = NativeActor::new(|_| {
            Ok(DispatchResult {
                output: Vec::new(),
                pending_state: None,
                create_requests: Vec::new(),
                delete_requests: Vec::new(),
                behaviour_requests: Vec::new(),
            })
        })
        .with_signal(|signal| {
            if matches!(signal, NativeSignal::Shutdown) {
                std::thread::sleep(Duration::from_secs(2));
            }
            Ok(())
        })
        .with_state_hooks(|| None, |_| {});

        let (plugin, lifecycle) = EntityPlugin::new_native(
            "slow-shutdown",
            &entity_node(),
            &native_kind_node(),
            actor,
            Vec::new(),
            None,
        )
        .expect("native plugin should load");
        assert_eq!(lifecycle, Lifecycle::Running);
        assert_eq!(plugin.kind, PluginKind::Stateful);

        let started = Instant::now();
        let err = plugin
            .prepare_reload_save("http://127.0.0.1:9", Duration::from_millis(20))
            .await
            .expect_err("slow shutdown signal must hit the short reload timeout");
        assert!(
            started.elapsed() < Duration::from_millis(500),
            "prepare_reload_save ignored short timeout: {:?}",
            started.elapsed()
        );
        assert!(err.to_string().contains("Shutdown timed out"));
    }
}
