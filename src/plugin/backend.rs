//! Extism plugin backend: host functions, the private context types they
//! capture, and the per-entity worker threads.
//!
//! All Wasm interaction lives here.  Each entity's `Plugin` is built and driven
//! on its own dedicated worker thread (`run_wasm_thread`), so no Wasm call ever
//! parks a Tokio worker.  The public handle and message types live in the
//! parent module.

use anyhow::{anyhow, Result};
use extism::{host_fn, Function, Manifest, Plugin, PluginBuilder, UserData, Wasm, PTR};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    oneshot,
};
use tracing::{info, warn};

use crate::entity::{
    CastInput, CreateEntityRequest, Lifecycle, ReplyRequest, SendEnvelope, SetBehaviourRequest,
};

use super::{DispatchResult, EntityMsg, EntityRegistry, NativeActor, NativeSignal};

// ── Fragment generation ───────────────────────────────────────────────────────

/// Generate an 8-character URL-safe alphanumeric fragment (nanoid-style).
fn generate_fragment() -> String {
    use rand::Rng;
    const ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::thread_rng();
    (0..8)
        .map(|_| ALPHABET[rng.gen_range(0..ALPHABET.len())] as char)
        .collect()
}

// ── Host functions ────────────────────────────────────────────────────────────

// Context captured by `ma_send` and `ma_reply` host functions.
//
// Sending is fire-and-forget: the envelope is forwarded to the main event
// loop via an unbounded channel.  The scheduler (and any other dispatch
// path) has zero envelope-handling responsibility.
struct OutboxCtx {
    tx: Sender<(String, SendEnvelope)>,
    fragment: String,
}

// `ma_send` host function exposed to plugins (namespace `extism:host/user`).
//
// The plugin passes a CBOR-encoded `SendEnvelope`.  The host forwards it
// directly to the main event loop via the outbox channel.
host_fn!(ma_send_fn(user_data: OutboxCtx; input: Vec<u8>) -> Vec<u8> {
    let envelope: SendEnvelope = from_cbor_bytes(&input)?;
    let arc = user_data.get()?;
    let ctx = arc.lock().unwrap();
    ctx.tx
        .try_send((ctx.fragment.clone(), envelope))
        .map_err(|e| anyhow!("plugin outbox is full: {e}"))?;
    drop(ctx);
    Ok(Vec::new())
});

// `ma_reply` host function: convenience wrapper around `ma_send`.
//
// Plugin passes a CBOR-encoded `ReplyRequest { msg, content }`.  The runtime
// fills in `to` (= msg.from), `reply_to` (= msg.id), and `content_type`
// automatically — plugin only provides the reply body.
host_fn!(ma_reply_fn(user_data: OutboxCtx; input: Vec<u8>) -> Vec<u8> {
    let req: ReplyRequest = from_cbor_bytes(&input)?;
    // If inside a synchronous ma_call (capture mode), capture the first reply
    // for this dispatch instead of enqueuing to the outbox.
    let envelope = SendEnvelope {
        to: req.msg.from,
        content_type: req.content_type,
        message_type: None,
        content: req.content,
        reply_to: Some(req.msg.id),
    };
    let arc = user_data.get()?;
    let ctx = arc.lock().unwrap();
    ctx.tx
        .try_send((ctx.fragment.clone(), envelope))
        .map_err(|e| anyhow!("plugin outbox is full: {e}"))?;
    drop(ctx);
    Ok(Vec::new())
});

// Internal state context shared between the `ma_set_state` host function and
// `EntityPlugin`.  Lives inside a `UserData<StateCtx>`.
struct StateCtx {
    /// New state bytes queued for IPFS persistence by the current dispatch.
    pending: Option<Vec<u8>>,
    /// Last successfully persisted snapshot (loaded from IPFS at startup, then
    /// updated by `mark_saved`).  Used for change detection.
    persisted: Option<Vec<u8>>,
    /// `true` when `pending` differs from `persisted` and has not yet been
    /// written to IPFS.
    dirty: bool,
    /// Monotonic counter bumped when a dispatch queues a distinct pending
    /// state. Used to report state persistence work once per change, not once
    /// per later message while the same bytes remain unsaved.
    save_generation: u64,
}

impl StateCtx {
    const fn new(persisted: Vec<u8>) -> Self {
        Self {
            pending: None,
            persisted: Some(persisted),
            dirty: false,
            save_generation: 0,
        }
    }

    fn mark_saved(&mut self, bytes: &[u8]) {
        self.persisted = Some(bytes.to_owned());
        if self.pending.as_deref() == Some(bytes) {
            self.pending = None;
            self.dirty = false;
        } else {
            self.dirty = self
                .pending
                .as_deref()
                .is_some_and(|pending| Some(pending) != self.persisted.as_deref());
        }
    }

    fn queue_state(&mut self, bytes: Vec<u8>) {
        if self.persisted.as_deref() != Some(bytes.as_slice())
            && self.pending.as_deref() != Some(bytes.as_slice())
        {
            self.pending = Some(bytes);
            self.dirty = true;
            self.save_generation = self.save_generation.wrapping_add(1);
        }
    }
}

// `ma_set_state` host function: plugin calls this to queue a new state.
// Sets `dirty` **only** when the bytes actually differ from the last
// persisted snapshot — no-op saves do not pollute the dirty flag.
host_fn!(ma_set_state_fn(user_data: StateCtx; input: Vec<u8>) -> Vec<u8> {
    let arc = user_data.get()?;
    let mut ctx = arc.lock().unwrap();
    ctx.queue_state(input);
    drop(ctx);
    Ok(Vec::new())
});

// ── ma_create_entity host function ────────────────────────────────────────────

// Context captured by `ma_create_entity` host function.
struct CreateEntityCtx {
    pending: Vec<CreateEntityRequest>,
    /// Fragment of the calling (parent) entity.
    caller_fragment: String,
}

// `ma_create_entity` host function: plugin requests creation of a new entity.
//
// Input is CBOR-encoded `{ "kind": "/ma/…/0.0.1", "behaviour": "bafyCID",
// "init": <payload>, "fragment": "<string>" }`. For shared-binary
// scriptable kinds, `behaviour` is appended after kind-level behaviour layers;
// `init` is the opaque `:init` signal creation payload (§14.2.1). Both are
// optional. When `fragment` is present the runtime validates and uses it
// directly; otherwise a random nanoid fragment is generated. Actual plugin
// loading and manifest persistence happen after dispatch returns.
#[derive(serde::Deserialize)]
struct CreateEntityInput {
    kind: String,
    #[serde(default)]
    behaviour: Option<String>,
    #[serde(default, with = "serde_bytes")]
    init: Option<Vec<u8>>,
    /// Optional explicit fragment chosen by the actor.
    #[serde(default)]
    fragment: Option<String>,
}

fn validate_entity_fragment(fragment: &str) -> Result<()> {
    if fragment.is_empty() || fragment.chars().any(char::is_control) {
        return Err(anyhow!("entity fragment is invalid"));
    }
    if fragment.contains('#') {
        return Err(anyhow!("entity fragment must not contain '#'"));
    }
    if crate::entity::RESERVED_ENTITY_NAMES.contains(&fragment) {
        return Err(anyhow!("entity fragment '{fragment}' is reserved"));
    }
    Ok(())
}

host_fn!(ma_create_entity_fn(user_data: CreateEntityCtx; input: Vec<u8>) -> Vec<u8> {
    let req: CreateEntityInput = from_cbor_bytes(&input)?;
    let arc = user_data.get()?;
    let mut ctx = arc.lock().unwrap();
    let fragment = if let Some(fragment) = req.fragment {
        validate_entity_fragment(&fragment)?;
        fragment
    } else {
        generate_fragment()
    };
    let parent = ctx.caller_fragment.clone();
    ctx.pending.push(CreateEntityRequest {
        fragment: fragment.clone(),
        kind_protocol: req.kind,
        behaviour_cid: req.behaviour,
        init_payload: req.init,
        parent,
    });
    drop(ctx);
    let mut out = Vec::new();
    ciborium::ser::into_writer(&fragment, &mut out)
        .map_err(|e| extism::Error::msg(format!("ma_create_entity: CBOR encode: {e}")))?;
    Ok(out)
});

// ── ma_set_behaviour host function ───────────────────────────────────────────

struct SetBehaviourCtx {
    pending: Vec<SetBehaviourRequest>,
    self_fragment: String,
}

host_fn!(ma_set_behaviour_fn(user_data: SetBehaviourCtx; input: Vec<u8>) -> Vec<u8> {
    let behaviour = String::from_utf8(input)
        .map_err(|e| extism::Error::msg(format!("ma_set_behaviour: invalid UTF-8: {e}")))?;
    let behaviour = normalize_behaviour_ref(&behaviour)
        .map_err(|e| extism::Error::msg(format!("ma_set_behaviour: {e}")))?;
    let arc = user_data.get()?;
    {
        let mut ctx = arc.lock().unwrap();
        let fragment = ctx.self_fragment.clone();
        ctx.pending.push(SetBehaviourRequest {
            fragment,
            behaviour_cid: behaviour,
        });
    }
    Ok(Vec::new())
});

fn normalize_behaviour_ref(value: &str) -> Result<Option<String>> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == "#f" {
        return Ok(None);
    }
    if let Some(cid) = trimmed.strip_prefix("/ipfs/") {
        if cid.is_empty() {
            return Err(anyhow!("/ipfs/ behaviour reference is missing a CID"));
        }
        return Ok(Some(cid.to_string()));
    }
    if trimmed.starts_with("/ipns/") {
        return Err(anyhow!("/ipns/ behaviour references are not supported here; publish the code to /ipfs/<cid> first"));
    }
    Ok(Some(trimmed.to_string()))
}

// ── ma_entity_exists host function ───────────────────────────────────────────

// Context captured by `ma_entity_exists` host function.
struct EntityExistsCtx {
    registry: EntityRegistry,
    our_did: String,
}

// `ma_entity_exists` host function: test whether a local entity fragment is live.
//
// Input is raw UTF-8, either `fragment`, `#fragment`, or this runtime's full
// `did:ma:...#fragment` DID-URL. Foreign DID-URLs always return false.
// Output is raw UTF-8: `true` or `false`.
host_fn!(ma_entity_exists_fn(user_data: EntityExistsCtx; input: Vec<u8>) -> Vec<u8> {
    let target = String::from_utf8(input)
        .map_err(|e| extism::Error::msg(format!("ma_entity_exists: invalid UTF-8: {e}")))?;
    let arc = user_data.get()?;
    let ctx = arc.lock().unwrap();
    let fragment = entity_fragment(&target, &ctx.our_did);
    let registry = ctx.registry.clone();
    drop(ctx);
    let exists = fragment
        .as_deref()
        .is_some_and(|fragment| registry.blocking_read().contains_key(fragment));
    Ok(if exists { b"true".to_vec() } else { b"false".to_vec() })
});

fn entity_fragment(target: &str, our_did: &str) -> Option<String> {
    let target = target.trim();
    if target.is_empty() {
        return None;
    }
    if let Some(fragment) = target.strip_prefix('#') {
        return (!fragment.is_empty()).then(|| fragment.to_string());
    }
    if target.starts_with("did:ma:") {
        let (did, fragment) = target.split_once('#')?;
        if did == our_did && !fragment.is_empty() {
            return Some(fragment.to_string());
        }
        return None;
    }
    (!target.contains('#') && !target.contains('/')).then(|| target.to_string())
}

// ── Wasm execution timeouts ───────────────────────────────────────────────────

/// Parse a duration in whole seconds from `var`, falling back to `default`.
fn env_secs(var: &str, default: u64) -> std::time::Duration {
    std::time::Duration::from_secs(
        std::env::var(var)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(default),
    )
}

/// Hard cap on any single Wasm export invocation (`init` / `on_message`).
/// Enforced by extism via wasmtime epoch interruption — a plugin stuck in an
/// infinite loop gets aborted and the worker thread survives.
///
/// Override with `MA_WASM_CALL_TIMEOUT_SECS` (used by tests; also an
/// operational escape hatch).
pub(super) fn wasm_call_timeout() -> std::time::Duration {
    env_secs("MA_WASM_CALL_TIMEOUT_SECS", 30)
}

/// Hard cap on building a Wasm plugin and completing its startup lifecycle.
/// Kept separate from the per-call cap so tests can shorten execution timeout
/// without making unrelated plugin instantiation flaky on slower targets.
pub(super) fn wasm_load_timeout() -> std::time::Duration {
    env_secs("MA_WASM_LOAD_TIMEOUT_SECS", 60)
}

fn wasm_max_pages() -> u32 {
    std::env::var("MA_WASM_MAX_PAGES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4096)
}

fn env_bytes(var: &str, default: u64) -> u64 {
    std::env::var(var)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

fn wasmtime_config() -> wasmtime::Config {
    let mut config = wasmtime::Config::new();
    config.memory_reservation(env_bytes(
        "MA_WASM_MEMORY_RESERVATION_BYTES",
        64 * 1024 * 1024,
    ));
    config.memory_reservation_for_growth(env_bytes(
        "MA_WASM_MEMORY_RESERVATION_FOR_GROWTH_BYTES",
        1024 * 1024,
    ));
    config
}

// ── ma_delete_entity host function ───────────────────────────────────────────

/// Context captured by `ma_delete_entity` and `ma_end` host functions.
struct DeleteEntityCtx {
    pending: Vec<String>,
    /// Set by `ma_end` — entity requests its own removal after this dispatch.
    self_terminate: bool,
    /// Fragment of the owning entity; used when `self_terminate` is true.
    self_fragment: String,
}

// `ma_delete_entity` host function: plugin requests removal of another entity.
//
// Input is a CBOR-encoded fragment string.
// The request is queued; the runtime validates and removes after dispatch.
host_fn!(ma_delete_entity_fn(user_data: DeleteEntityCtx; input: Vec<u8>) -> Vec<u8> {
    let target: String = from_cbor_bytes(&input)?;
    let arc = user_data.get()?;
    arc.lock().unwrap().pending.push(target);
    let mut out = Vec::new();
    ciborium::ser::into_writer(&":queued", &mut out)
        .map_err(|e| extism::Error::msg(format!("ma_delete_entity: CBOR encode: {e}")))?;
    Ok(out)
});

// `ma_end` host function: entity requests its own removal (self-termination).
//
// Takes no meaningful input.  After the current dispatch completes the runtime
// removes this entity from the registry and manifest.
host_fn!(ma_end_fn(user_data: DeleteEntityCtx; _input: Vec<u8>) -> Vec<u8> {
    let arc = user_data.get()?;
    arc.lock().unwrap().self_terminate = true;
    let mut out = Vec::new();
    ciborium::ser::into_writer(&ciborium::Value::Text(":ok".to_string()), &mut out)
        .map_err(|e| extism::Error::msg(format!("ma_end: CBOR encode: {e}")))?;
    Ok(out)
});

// ── ma-include-ipfs host function ─────────────────────────────────────────────
//
// Available only to scriptable kinds whose language supports library
// composition (ma-scheme does, via `ma-include-ipfs`, ma-scheme-v1.md §11.1).
//
// There is deliberately no `ma_get_behaviour`/`ma_get_behaviour_cid`/
// `ma_set_behaviour_cid` here — an earlier draft had all three, plus a
// queued-mutation-and-republish mechanism mirroring `ma_create_entity`/
// `ma_delete_entity`. Removed entirely: an entity's behaviour reference is
// immutable from within ma-scheme (ma-scheme-v1.md §11) — a script that
// needs its own reference reads it from config instead (`"behaviour"` key,
// see `build_plugin_config` below).

// Context captured by `ma_ipfs_include`.
struct BehaviourCtx {
    kubo_url: String,
    /// Handle into the Tokio runtime, used to block on the (async) IPFS
    /// fetch from this synchronous host-function callback.
    handle: tokio::runtime::Handle,
}

// `ma_ipfs_include` host function: resolves a single `ma-include-ipfs`
// reference (ma-scheme-v1.md §11.1) -- a literal `#!/ipfs/<cid>` or
// `#!/ipns/<key>` token -- to its raw content bytes. A single, flat fetch;
// all recursion/depth/cycle tracking is the guest's own responsibility
// (`lambda-ma/scheme-actor`), not this host function's.
//
// Input is raw UTF-8 bytes, NOT CBOR: extism-pdk's `#[host_fn]` macro
// sends a `String` argument via `ToBytes for String` (identity — the raw
// bytes of the string), not through CBOR encoding. Unlike the
// CBOR-encoded inputs elsewhere in this file (written to match Python
// actors manually constructing CBOR payloads), this host function is
// called only by the Rust ma-scheme actor guest via that macro, so it
// must match what the macro actually sends.
host_fn!(ma_ipfs_include_fn(user_data: BehaviourCtx; input: Vec<u8>) -> Vec<u8> {
    let reference = String::from_utf8(input)
        .map_err(|e| extism::Error::msg(format!("ma_ipfs_include: reference is not valid UTF-8: {e}")))?;
    let (kubo_url, handle) = {
        let arc = user_data.get()?;
        let ctx = arc.lock().unwrap();
        (ctx.kubo_url.clone(), ctx.handle.clone())
    };
    let bytes = handle
        .block_on(crate::behaviour::resolve_ipfs_include(&kubo_url, &reference))
        .map_err(|e| extism::Error::msg(format!("ma_ipfs_include: {e}")))?;
    Ok(bytes)
});

// ── Worker threads ────────────────────────────────────────────────────────────

/// Everything a Wasm entity's worker thread needs to build and run its plugin.
/// All fields are `Send`; the non-`Send` `Plugin` is constructed on the thread.
pub(super) struct WasmThreadCfg {
    pub(super) fragment: String,
    pub(super) our_did: String,
    pub(super) wasm_bytes: Vec<u8>,
    pub(super) init_state: Vec<u8>,
    pub(super) wasi: bool,
    pub(super) host_functions: Vec<String>,
    /// `true` only on this entity's very first ever load — gates whether
    /// the `:init` signal fires at all.
    pub(super) is_genesis: bool,
    /// Opaque creation payload for the `:init` signal, only `Some` when
    /// `is_genesis`.
    pub(super) init_payload: Option<Vec<u8>>,
    /// Pre-resolved behaviour source text for the `:set-behaviour` signal,
    /// assembled from kind-level and entity-level behaviour links.
    pub(super) behaviour_text: Option<Vec<u8>>,
    pub(super) node_kind: String,
    pub(super) envelope_tx: Sender<(String, SendEnvelope)>,
    /// IPFS CID of the kind's shared Wasm binary (`KindNode.cid`).
    pub(super) wasm_cid: String,
    /// This entity's own behaviour source reference, if any (`EntityNode.behaviour`).
    pub(super) entity_behaviour_cid: Option<String>,
    /// Kubo RPC URL, needed by `ma_ipfs_include` to resolve a reference on
    /// demand.
    pub(super) kubo_url: String,
    /// Handle into the Tokio runtime, used to block on IPFS fetches from the
    /// synchronous `ma_ipfs_include` host-function callback.
    pub(super) tokio_handle: tokio::runtime::Handle,
    /// iroh QUIC node ID of this runtime.
    pub(super) iroh_node_id: String,
    /// Unix epoch seconds when the runtime process started.
    pub(super) started_at: u64,
    /// DID-URL of the parent entity, if any.
    pub(super) parent: Option<String>,
    /// Public runtime/manifest config exposed to the entity as read-only config.
    pub(super) runtime_config: std::collections::BTreeMap<String, String>,
    /// Live entity registry, used by local introspection host functions.
    pub(super) entity_registry: EntityRegistry,
}

/// Handles retained by the worker thread to drain plugin side-effects after
/// each dispatch and to service state messages.
struct WasmThreadState {
    plugin: Plugin,
    state: UserData<StateCtx>,
    create_queue: UserData<CreateEntityCtx>,
    delete_queue: UserData<DeleteEntityCtx>,
    behaviour_queue: UserData<SetBehaviourCtx>,
}

/// Build the flat config map injected into the extism `Manifest` for this
/// entity.  Available to the plugin at any time via `extism_config_get`.
///
/// Keys:
///   `self`         full DID-URL of this entity (`did:ma:<runtime>#<id>`)  [always]
///   `id`           bare fragment without `#`                               [always]
///   `kind`         kind protocol ID e.g. `/ma/root/0.0.1`                 [always]
///   `cid`          IPFS CID of the kind's shared Wasm binary              [always]
///   `behaviour`    this entity's own `EntityNode.behaviour` reference     [if set]
///   `runtime`      runtime's own DID                                       [always]
///   `iroh_node_id` iroh QUIC node ID of this runtime                      [always]
///   `started_at`   Unix epoch seconds when the runtime started            [always]
///   `parent`       DID-URL of the parent entity                           [if set]
fn build_plugin_config(cfg: &WasmThreadCfg) -> std::collections::BTreeMap<String, String> {
    let mut config = cfg.runtime_config.clone();
    config.insert(
        "self".to_string(),
        format!("{}#{}", cfg.our_did, cfg.fragment),
    );
    config.insert("id".to_string(), cfg.fragment.clone());
    config.insert("kind".to_string(), cfg.node_kind.clone());
    config.insert("cid".to_string(), cfg.wasm_cid.clone());
    if let Some(behaviour_cid) = &cfg.entity_behaviour_cid {
        config.insert("behaviour".to_string(), behaviour_cid.clone());
    }
    config.insert("runtime".to_string(), cfg.our_did.clone());
    config.insert("iroh_node_id".to_string(), cfg.iroh_node_id.clone());
    config.insert("started_at".to_string(), cfg.started_at.to_string());
    if let Some(parent) = &cfg.parent {
        config.insert("parent".to_string(), parent.clone());
    }
    config
}

/// Build the Wasm plugin and its filtered host-function set on the worker thread.
fn build_wasm_plugin(cfg: &WasmThreadCfg) -> Result<WasmThreadState> {
    let outbox_ctx_send = UserData::new(OutboxCtx {
        tx: cfg.envelope_tx.clone(),
        fragment: cfg.fragment.clone(),
    });
    let outbox_ctx_reply = UserData::new(OutboxCtx {
        tx: cfg.envelope_tx.clone(),
        fragment: cfg.fragment.clone(),
    });
    let state: UserData<StateCtx> = UserData::new(StateCtx::new(cfg.init_state.clone()));
    let create_queue: UserData<CreateEntityCtx> = UserData::new(CreateEntityCtx {
        pending: Vec::new(),
        caller_fragment: cfg.fragment.clone(),
    });
    let delete_queue: UserData<DeleteEntityCtx> = UserData::new(DeleteEntityCtx {
        pending: Vec::new(),
        self_terminate: false,
        self_fragment: cfg.fragment.clone(),
    });
    let behaviour_queue: UserData<SetBehaviourCtx> = UserData::new(SetBehaviourCtx {
        pending: Vec::new(),
        self_fragment: cfg.fragment.clone(),
    });
    let entity_exists_ctx: UserData<EntityExistsCtx> = UserData::new(EntityExistsCtx {
        registry: cfg.entity_registry.clone(),
        our_did: cfg.our_did.clone(),
    });
    let behaviour: UserData<BehaviourCtx> = UserData::new(BehaviourCtx {
        kubo_url: cfg.kubo_url.clone(),
        handle: cfg.tokio_handle.clone(),
    });
    let host_fns = build_host_functions(
        cfg,
        outbox_ctx_reply,
        HostFunctionCtx {
            outbox_ctx_send,
            state: state.clone(),
            create_queue: create_queue.clone(),
            delete_queue: delete_queue.clone(),
            behaviour_queue: behaviour_queue.clone(),
            entity_exists_ctx,
            behaviour,
        },
    );

    let manifest = Manifest::new([Wasm::data(cfg.wasm_bytes.clone())])
        .with_memory_max(wasm_max_pages())
        .with_timeout(wasm_call_timeout())
        .with_config(build_plugin_config(cfg).into_iter());
    let plugin = PluginBuilder::new(manifest)
        .with_functions(host_fns)
        .with_wasi(cfg.wasi)
        .with_cache_disabled()
        .with_wasmtime_config(wasmtime_config())
        .build()
        .map_err(|e| anyhow!("failed to create extism plugin for '{}': {e}", cfg.fragment))?;

    Ok(WasmThreadState {
        plugin,
        state,
        create_queue,
        delete_queue,
        behaviour_queue,
    })
}

struct HostFunctionCtx {
    outbox_ctx_send: UserData<OutboxCtx>,
    state: UserData<StateCtx>,
    create_queue: UserData<CreateEntityCtx>,
    delete_queue: UserData<DeleteEntityCtx>,
    behaviour_queue: UserData<SetBehaviourCtx>,
    entity_exists_ctx: UserData<EntityExistsCtx>,
    behaviour: UserData<BehaviourCtx>,
}

fn build_host_functions(
    cfg: &WasmThreadCfg,
    outbox_ctx_reply: UserData<OutboxCtx>,
    ctx: HostFunctionCtx,
) -> Vec<Function> {
    // The filter preserves this list order for hosts that request only a subset
    // of functions.
    let all_fns: Vec<(&str, Function)> = vec![
        (
            "ma_reply",
            Function::new("ma_reply", [PTR], [PTR], outbox_ctx_reply, ma_reply_fn),
        ),
        (
            "ma_set_state",
            Function::new("ma_set_state", [PTR], [PTR], ctx.state, ma_set_state_fn),
        ),
        (
            "ma_send",
            Function::new("ma_send", [PTR], [PTR], ctx.outbox_ctx_send, ma_send_fn),
        ),
        (
            "ma_end",
            Function::new("ma_end", [PTR], [PTR], ctx.delete_queue.clone(), ma_end_fn),
        ),
        (
            "ma_create_entity",
            Function::new(
                "ma_create_entity",
                [PTR],
                [PTR],
                ctx.create_queue,
                ma_create_entity_fn,
            ),
        ),
        (
            "ma_delete_entity",
            Function::new(
                "ma_delete_entity",
                [PTR],
                [PTR],
                ctx.delete_queue,
                ma_delete_entity_fn,
            ),
        ),
        (
            "ma_set_behaviour",
            Function::new(
                "ma_set_behaviour",
                [PTR],
                [PTR],
                ctx.behaviour_queue,
                ma_set_behaviour_fn,
            ),
        ),
        (
            "ma_ipfs_include",
            Function::new(
                "ma_ipfs_include",
                [PTR],
                [PTR],
                ctx.behaviour,
                ma_ipfs_include_fn,
            ),
        ),
        (
            "ma_entity_exists",
            Function::new(
                "ma_entity_exists",
                [PTR],
                [PTR],
                ctx.entity_exists_ctx,
                ma_entity_exists_fn,
            ),
        ),
    ];
    let allowed: std::collections::HashSet<&str> =
        cfg.host_functions.iter().map(String::as_str).collect();
    all_fns
        .into_iter()
        .filter(|(name, _)| allowed.contains(*name))
        .map(|(_, f)| f)
        .collect()
}

/// Parse a plugin export's CBOR-encoded return value as `:ok`/`[:ok, …]` vs
/// `[:error, reason]`. Returns `Ok(None)` for anything that isn't a
/// recognised error tuple (treated as success), `Ok(Some(reason))` for an
/// explicit `[:error, reason]`.
fn parse_error_reason(bytes: &[u8]) -> Option<String> {
    match ciborium::de::from_reader::<ciborium::Value, _>(bytes) {
        Ok(ciborium::Value::Array(ref v))
            if v.first() == Some(&ciborium::Value::Text(":error".into())) =>
        {
            let reason = v
                .get(1)
                .and_then(|r| {
                    if let ciborium::Value::Text(s) = r {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| "unknown".to_string());
            Some(reason)
        }
        _ => None,
    }
}

/// Encode a bare-atom signal term (no associated data), e.g. `:start`,
/// matching the wire shape `on_signal` expects (ma-runtime-v1.md §14.2).
fn signal_atom(name: &str) -> Vec<u8> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(&ciborium::Value::Text(name.to_string()), &mut out)
        .expect("encoding a signal atom cannot fail");
    out
}

/// Encode a `[atom, bytes]` signal term (e.g. `:set-state`/`:set-behaviour`/
/// `:init`), matching the wire shape `on_signal` expects.
fn signal_with_data(name: &str, data: &[u8]) -> Vec<u8> {
    let term = ciborium::Value::Array(vec![
        ciborium::Value::Text(name.to_string()),
        ciborium::Value::Bytes(data.to_vec()),
    ]);
    let mut out = Vec::new();
    ciborium::ser::into_writer(&term, &mut out).expect("encoding a signal term cannot fail");
    out
}

/// Drive the freshly built plugin through the applicable lifecycle signals,
/// in order: `:set-state` (only if state exists), `:set-behaviour` (only if
/// a behaviour reference resolves), `:init` (only at genesis), `:start`
/// (always) — all delivered through the single `on_signal` export
/// (ma-runtime-v1.md §14.2). There is no per-kind declaration of which of
/// these apply; firing is purely data-driven. Returns the resulting
/// [`Lifecycle`] — `Error` only if `:init` (the one genesis-time signal a
/// script may use to reject creation) returns `[:error, reason]`.
///
/// Entity context is available via `extism_config_get` throughout; each
/// signal carries only the specific data documented in ma-runtime-v1.md
/// §14.3.
fn run_genesis_and_start(ts: &mut WasmThreadState, cfg: &WasmThreadCfg) -> Result<Lifecycle> {
    if !cfg.init_state.is_empty() {
        info!(fragment = %cfg.fragment, state_bytes = cfg.init_state.len(), "entity lifecycle signal start: set-state");
        ts.plugin
            .call::<&[u8], Vec<u8>>(
                "on_signal",
                signal_with_data(":set-state", &cfg.init_state).as_slice(),
            )
            .map_err(|e| anyhow!("on_signal(:set-state) failed for '{}': {e}", cfg.fragment))?;
        info!(fragment = %cfg.fragment, "entity lifecycle signal finish: set-state");
    }

    if let Some(text) = &cfg.behaviour_text {
        info!(fragment = %cfg.fragment, behaviour_bytes = text.len(), "entity lifecycle signal start: set-behaviour");
        ts.plugin
            .call::<&[u8], Vec<u8>>(
                "on_signal",
                signal_with_data(":set-behaviour", text).as_slice(),
            )
            .map_err(|e| {
                anyhow!(
                    "on_signal(:set-behaviour) failed for '{}': {e}",
                    cfg.fragment
                )
            })?;
        info!(fragment = %cfg.fragment, "entity lifecycle signal finish: set-behaviour");
    }

    let mut lifecycle = Lifecycle::Running;
    if cfg.is_genesis {
        let payload = cfg.init_payload.as_deref().unwrap_or(&[]);
        info!(fragment = %cfg.fragment, init_payload_bytes = payload.len(), "entity lifecycle signal start: init");
        let result_bytes = ts
            .plugin
            .call::<&[u8], Vec<u8>>("on_signal", signal_with_data(":init", payload).as_slice())
            .map_err(|e| anyhow!("on_signal(:init) failed for '{}': {e}", cfg.fragment))?;
        if let Some(reason) = parse_error_reason(&result_bytes) {
            warn!(fragment = %cfg.fragment, reason = %reason, "on_signal(:init) returned :error");
            lifecycle = Lifecycle::Error;
        }
        info!(fragment = %cfg.fragment, "entity lifecycle signal finish: init");
    }

    info!(fragment = %cfg.fragment, "entity lifecycle signal start: start");
    let result_bytes = ts
        .plugin
        .call::<&[u8], Vec<u8>>("on_signal", signal_atom(":start").as_slice())
        .map_err(|e| anyhow!("on_signal(:start) failed for '{}': {e}", cfg.fragment))?;
    if let Some(reason) = parse_error_reason(&result_bytes) {
        warn!(fragment = %cfg.fragment, reason = %reason, "on_signal(:start) returned :error");
    }
    info!(fragment = %cfg.fragment, "entity lifecycle signal finish: start");

    Ok(lifecycle)
}

/// Execute one dispatch to the plugin, draining side-effects into a
/// [`DispatchResult`].  Runs on the entity's own worker thread.
fn execute_dispatch(
    ts: &mut WasmThreadState,
    fragment: &str,
    stateful: bool,
    input: &CastInput,
) -> Result<DispatchResult> {
    let export = "on_message";
    let _ = stateful; // still tracked for PluginKind but export name is unified
    let mut input_bytes = Vec::new();
    ciborium::ser::into_writer(input, &mut input_bytes)
        .map_err(|e| anyhow!("failed to CBOR-encode CastInput: {e}"))?;

    let state_generation_before = ts
        .state
        .get()
        .map_err(|e| anyhow!("state error: {e}"))?
        .lock()
        .map_err(|e| anyhow!("state poisoned: {e}"))?
        .save_generation;

    let output = ts
        .plugin
        .call::<&[u8], Vec<u8>>(export, input_bytes.as_slice())
        .map_err(|e| anyhow!("{export}() failed for '{fragment}': {e}"));

    let output = output?;

    let pending_state = {
        let state = ts.state.get().map_err(|e| anyhow!("state error: {e}"))?;
        let state = state.lock().map_err(|e| anyhow!("state poisoned: {e}"))?;
        if state.dirty && state.save_generation != state_generation_before {
            state.pending.clone()
        } else {
            None
        }
    };

    let create_requests = ts
        .create_queue
        .get()
        .map_err(|e| anyhow!("create_queue error: {e}"))?
        .lock()
        .map_err(|e| anyhow!("create_queue poisoned: {e}"))?
        .pending
        .drain(..)
        .collect();

    let delete_requests = {
        let arc = ts
            .delete_queue
            .get()
            .map_err(|e| anyhow!("delete_queue error: {e}"))?;
        let mut dq = arc
            .lock()
            .map_err(|e| anyhow!("delete_queue poisoned: {e}"))?;
        let mut reqs: Vec<String> = dq.pending.drain(..).collect();
        if dq.self_terminate {
            reqs.push(dq.self_fragment.clone());
            dq.self_terminate = false;
        }
        reqs
    };

    let behaviour_requests = ts
        .behaviour_queue
        .get()
        .map_err(|e| anyhow!("behaviour_queue error: {e}"))?
        .lock()
        .map_err(|e| anyhow!("behaviour_queue poisoned: {e}"))?
        .pending
        .drain(..)
        .collect();

    Ok(DispatchResult {
        output,
        pending_state,
        create_requests,
        delete_requests,
        behaviour_requests,
    })
}

/// Entry point for a Wasm entity's dedicated worker thread.
///
/// Builds the plugin, drives it through the applicable genesis/start
/// lifecycle stages, reports the resulting lifecycle back to
/// [`EntityPlugin::load`], then serves dispatch / state messages until the
/// channel closes.  The thread is a plain OS thread (never a Tokio worker),
/// so blocking here is safe.
#[allow(clippy::needless_pass_by_value)] // cfg is moved into and owned by the thread
pub(super) fn run_wasm_thread(
    cfg: WasmThreadCfg,
    mut rx: Receiver<EntityMsg>,
    life_tx: oneshot::Sender<Result<Lifecycle>>,
) {
    info!(
        fragment = %cfg.fragment,
        kind = %cfg.node_kind,
        wasm_bytes = cfg.wasm_bytes.len(),
        state_bytes = cfg.init_state.len(),
        behaviour_bytes = cfg.behaviour_text.as_ref().map_or(0, Vec::len),
        init_payload_bytes = cfg.init_payload.as_ref().map_or(0, Vec::len),
        "entity wasm worker build start"
    );
    let mut ts = match build_wasm_plugin(&cfg) {
        Ok(ts) => ts,
        Err(e) => {
            let _ = life_tx.send(Err(e));
            return;
        }
    };
    info!(fragment = %cfg.fragment, "entity wasm worker build finish");
    let lifecycle = match run_genesis_and_start(&mut ts, &cfg) {
        Ok(lc) => lc,
        Err(e) => {
            let _ = life_tx.send(Err(e));
            return;
        }
    };
    if life_tx.send(Ok(lifecycle)).is_err() {
        // Loader gave up (dropped the receiver); nothing to serve.
        return;
    }

    while let Some(msg) = rx.blocking_recv() {
        match msg {
            EntityMsg::Dispatch {
                stateful,
                input,
                reply,
            } => {
                let res = execute_dispatch(&mut ts, &cfg.fragment, stateful, &input);
                let _ = reply.send(res);
            }
            EntityMsg::TakePending { reply } => {
                let pending = ts
                    .state
                    .get()
                    .ok()
                    .and_then(|arc| arc.lock().ok().and_then(|c| c.pending.clone()));
                let _ = reply.send(pending);
            }
            EntityMsg::MarkSaved(bytes) => {
                if let Ok(arc) = ts.state.get() {
                    if let Ok(mut c) = arc.lock() {
                        c.mark_saved(&bytes);
                    }
                }
            }
            EntityMsg::Shutdown { reply } => {
                let signal_result = ts
                    .plugin
                    .call::<&[u8], Vec<u8>>("on_signal", signal_atom(":shutdown").as_slice());
                let pending = ts
                    .state
                    .get()
                    .ok()
                    .and_then(|arc| arc.lock().ok().and_then(|c| c.pending.clone()));
                let _ = match signal_result {
                    Ok(_) => reply.send(Ok(pending)),
                    Err(e) => reply.send(Err(anyhow!("on_signal(:shutdown) failed: {e}"))),
                };
            }
            EntityMsg::Terminate => break,
        }
    }
}

/// Entry point for a native (compiled-in) entity's worker thread.
///
/// The closure may call `tokio::spawn` (e.g. `#scheduler`), so the runtime
/// context is entered *only* around the closure — never around `blocking_recv`,
/// which would panic inside an async context.
#[allow(clippy::needless_pass_by_value)] // handler + handle are owned by the thread
pub(super) fn run_native_thread(
    actor: NativeActor,
    handle: tokio::runtime::Handle,
    mut rx: Receiver<EntityMsg>,
) {
    while let Some(msg) = rx.blocking_recv() {
        match msg {
            EntityMsg::Dispatch { input, reply, .. } => {
                let res = {
                    let _guard = handle.enter();
                    (actor.dispatch)(&input)
                };
                let _ = reply.send(res);
            }
            EntityMsg::TakePending { reply } => {
                let _ = reply.send((actor.take_pending)());
            }
            EntityMsg::MarkSaved(bytes) => (actor.mark_saved)(bytes),
            EntityMsg::Shutdown { reply } => {
                let signal_result = (actor.signal)(NativeSignal::Shutdown);
                let pending = (actor.take_pending)();
                let _ = match signal_result {
                    Ok(()) => reply.send(Ok(pending)),
                    Err(e) => reply.send(Err(anyhow!("native on_signal(:shutdown) failed: {e}"))),
                };
            }
            EntityMsg::Terminate => break,
        }
    }
}
// ── CBOR helpers ──────────────────────────────────────────────────────────────

fn from_cbor_bytes<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    ciborium::de::from_reader(bytes).map_err(|e| anyhow!("CBOR decode error: {e}"))
}

#[cfg(test)]
mod tests {
    use super::{generate_fragment, validate_entity_fragment, StateCtx};

    #[test]
    fn generate_fragment_is_8_alphanumeric() {
        let f = generate_fragment();
        assert_eq!(f.len(), 8);
        assert!(f.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn generate_fragment_varies() {
        let set: std::collections::HashSet<_> = (0..5).map(|_| generate_fragment()).collect();
        assert!(set.len() > 1, "fragments should not all be identical");
    }

    #[test]
    fn explicit_entity_fragment_validation_rejects_invalid_names() {
        assert!(validate_entity_fragment("6437b3ac38465133").is_ok());
        assert!(validate_entity_fragment("").is_err());
        assert!(validate_entity_fragment("bad\nname").is_err());
        assert!(validate_entity_fragment("bad#name").is_err());
        assert!(validate_entity_fragment("root").is_err());
    }

    #[test]
    fn mark_saved_keeps_newer_pending_state() {
        let mut state = StateCtx::new(b"initial".to_vec());
        state.pending = Some(b"old".to_vec());
        state.dirty = true;

        state.pending = Some(b"new".to_vec());
        state.mark_saved(b"old");

        assert_eq!(state.persisted.as_deref(), Some(b"old".as_slice()));
        assert_eq!(state.pending.as_deref(), Some(b"new".as_slice()));
        assert!(state.dirty);

        state.mark_saved(b"new");

        assert_eq!(state.persisted.as_deref(), Some(b"new".as_slice()));
        assert!(state.pending.is_none());
        assert!(!state.dirty);
    }

    #[test]
    fn queue_state_bumps_generation_only_for_distinct_unsaved_state() {
        let mut state = StateCtx::new(b"initial".to_vec());

        state.queue_state(b"next".to_vec());
        assert_eq!(state.save_generation, 1);
        assert_eq!(state.pending.as_deref(), Some(b"next".as_slice()));
        assert!(state.dirty);

        state.queue_state(b"next".to_vec());
        assert_eq!(state.save_generation, 1);

        state.queue_state(b"newer".to_vec());
        assert_eq!(state.save_generation, 2);
        assert_eq!(state.pending.as_deref(), Some(b"newer".as_slice()));
    }
}
