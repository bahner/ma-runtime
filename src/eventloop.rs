//! The daemon's main event loop and graceful shutdown.
//!
//! Drains the RPC, IPFS-publish, and CRUD service inboxes each tick, delivers
//! plugin envelopes, and on Ctrl-C persists entity state and closes the iroh
//! endpoint.  Split out of `main.rs` so the entry point covers only startup.

use std::sync::Arc;
use std::time::Duration;
use std::{collections::hash_map::Entry, collections::HashMap, hash::Hash};

use anyhow::Result;
use ciborium::Value as CborValue;
use ma_core::config::Config;
use ma_core::{
    Did, DidDocumentResolver, Inbox, MaEndpoint, Message, SigningKey, CONTENT_TYPE_TERM,
    INBOX_PROTOCOL_ID, IPFS_PROTOCOL_ID, MESSAGE_TYPE_CRUD, MESSAGE_TYPE_CRUD_REPLY,
    MESSAGE_TYPE_IDENTITY_PUBLISH_REQUEST, MESSAGE_TYPE_IPFS_REQUEST, MESSAGE_TYPE_RPC,
    MESSAGE_TYPE_RPC_REPLY,
};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, RwLock, Semaphore};
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use zeroize::Zeroize;

use crate::acl::{AclCache, GroupCache, SharedAcl};
use crate::entity::{
    CastInput, EntityNode, IpldLink, KindRegistry, Lifecycle, LocalMessage, PluginMsg, SendEnvelope,
};
use crate::ipfs::IpfsServiceState;
use crate::manifest::ManifestWriter;
use crate::plugin::EntityRegistry;
use crate::routing::{local_actor_url, local_target_fragment};
use crate::status::SharedStats;
use crate::{bootstrap, crud, i18n, inbox, ipfs, rpc, status};

const PLUGIN_OUTBOX_DRAIN_BUDGET: usize = 64;
const LOCAL_PLUGIN_DISPATCH_LIMIT: usize = 16;
const REMOTE_PLUGIN_DELIVERY_LIMIT: usize = 16;

#[derive(Clone)]
struct LocalSideEffectCtx {
    kind_registry: KindRegistry,
    envelope_tx: Sender<(String, SendEnvelope)>,
    stats: SharedStats,
    shared_config: Arc<RwLock<Config>>,
    entity_creation_gate: Arc<Mutex<()>>,
}

async fn register_created_entity(
    entity_registry: &EntityRegistry,
    fragment: String,
    entity: Arc<crate::plugin::EntityPlugin>,
) -> bool {
    let mut registry = entity_registry.write().await;
    insert_if_absent(&mut registry, fragment, entity)
}

fn insert_if_absent<K: Eq + Hash, V>(registry: &mut HashMap<K, V>, key: K, value: V) -> bool {
    match registry.entry(key) {
        Entry::Vacant(entry) => {
            entry.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

async fn public_plugin_config_for_local(
    kubo_url: &str,
    side_effects: &LocalSideEffectCtx,
) -> Result<std::collections::BTreeMap<String, String>> {
    crate::crud::config::fetch_public_plugin_config(
        &side_effects.stats,
        kubo_url,
        &side_effects.shared_config,
    )
    .await
}

/// Map a `message_type` string to the iroh delivery protocol.
///
/// Only RPC and its reply go to `/ma/rpc/0.0.1`; IPFS requests go to
/// `/ma/ipfs/0.0.1`; CRUD goes to `/ma/crud/0.0.1`.  Everything else
/// (message, broadcast, chat, emote, unknown) falls back to `/ma/inbox/0.0.1`.
fn protocol_for(msg_type: &str) -> &'static str {
    match msg_type {
        MESSAGE_TYPE_RPC | MESSAGE_TYPE_RPC_REPLY => rpc::RPC_PROTOCOL_ID,
        MESSAGE_TYPE_IPFS_REQUEST | MESSAGE_TYPE_IDENTITY_PUBLISH_REQUEST => IPFS_PROTOCOL_ID,
        MESSAGE_TYPE_CRUD | MESSAGE_TYPE_CRUD_REPLY => crud::CRUD_PROTOCOL_ID,
        _ => INBOX_PROTOCOL_ID,
    }
}

struct LocalDispatchArgs {
    sender_fragment: String,
    target_fragment: String,
    env: SendEnvelope,
    msg_type: String,
    entity_registry: EntityRegistry,
    manifest_writer: ManifestWriter,
    kubo_url: String,
    our_did: String,
    side_effects: Option<LocalSideEffectCtx>,
}

async fn dispatch_local_plugin_envelope(args: LocalDispatchArgs) {
    let LocalDispatchArgs {
        sender_fragment,
        target_fragment,
        env,
        msg_type,
        entity_registry,
        manifest_writer,
        kubo_url,
        our_did,
        side_effects,
    } = args;

    let mut entity = None;
    for attempt in 0..40 {
        entity = entity_registry.read().await.get(&target_fragment).cloned();
        if entity.is_some() {
            break;
        }
        if attempt < 39 {
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    }
    let Some(entity) = entity else {
        warn!(fragment = %sender_fragment, to = %env.to, target = %target_fragment, "plugin envelope: unknown local recipient; skipped");
        return;
    };

    let local_msg = LocalMessage {
        id: Uuid::new_v4().to_string(),
        from: local_actor_url(&our_did, &sender_fragment),
        to: local_actor_url(&our_did, &target_fragment),
        created_at: status::now_unix_secs(),
        exp: 0,
        reply_to: env.reply_to.clone(),
        message_type: msg_type,
        content_type: env.content_type.clone(),
        content: env.content.clone(),
    };
    let cast_input = CastInput {
        msg: PluginMsg::from(&local_msg),
    };

    debug!(
        fragment = %sender_fragment,
        target = %target_fragment,
        from = %local_msg.from,
        to = %local_msg.to,
        id = %local_msg.id,
        reply_to = ?local_msg.reply_to,
        msg_type = %local_msg.message_type,
        "plugin envelope: local dispatch start"
    );
    let result = match entity.on_message(&cast_input).await {
        Ok(result) => {
            debug!(
                fragment = %sender_fragment,
                target = %target_fragment,
                from = %local_msg.from,
                to = %local_msg.to,
                id = %local_msg.id,
                "plugin envelope: local dispatch finish"
            );
            result
        }
        Err(err) => {
            warn!(fragment = %target_fragment, from = %local_msg.from, error = %err, "plugin envelope: local dispatch failed");
            return;
        }
    };

    if let Some(state_bytes) = result.pending_state {
        entity.spawn_state_persist(
            kubo_url.clone(),
            manifest_writer.clone(),
            state_bytes,
            "plugin envelope",
        );
    }

    if let Some(side_effects) = side_effects {
        let ctx = CreateCtx {
            entity_registry: &entity_registry,
            manifest_writer: &manifest_writer,
            kubo_url: &kubo_url,
            our_did: &our_did,
            side_effects: &side_effects,
        };
        for req in result.create_requests {
            handle_create_request(req, &entity, &ctx).await;
        }
    } else if !result.create_requests.is_empty() {
        warn!(
            count = result.create_requests.len(),
            "plugin envelope: create requests ignored without side-effect context"
        );
    }
}

/// Bundle of shared references needed to service one or more
/// `ma_create_entity` requests, kept together to stay under clippy's
/// argument-count limit across `handle_create_request`/`finish_create_request`.
#[derive(Clone, Copy)]
struct CreateCtx<'a> {
    entity_registry: &'a EntityRegistry,
    manifest_writer: &'a ManifestWriter,
    kubo_url: &'a str,
    our_did: &'a str,
    side_effects: &'a LocalSideEffectCtx,
}

/// Handle one `ma_create_entity` request enqueued by `entity`'s dispatch:
/// load the new entity's kind, spawn it, and either register it (persisting
/// to the manifest) or report the failure back to the requesting parent.
async fn handle_create_request(
    req: crate::entity::CreateEntityRequest,
    entity: &crate::plugin::EntityPlugin,
    ctx: &CreateCtx<'_>,
) {
    let side_effects = ctx.side_effects;
    let entity_registry = ctx.entity_registry;
    let _creation_guard = side_effects.entity_creation_gate.lock().await;
    if entity_registry.read().await.contains_key(&req.fragment) {
        debug!(fragment = %req.fragment, kind = %req.kind_protocol,
            "ma_create_entity: entity already exists; keeping current entity");
        return;
    }
    let maybe_kind = side_effects
        .kind_registry
        .read()
        .await
        .get(&req.kind_protocol)
        .cloned();
    let Some(kind_node) = maybe_kind else {
        warn!(caller = %entity.fragment, kind = %req.kind_protocol,
            "ma_create_entity: kind not in registry; skipped");
        return;
    };

    let entity_node = EntityNode {
        kind: req.kind_protocol.clone(),
        behaviour: match req
            .behaviour_cid
            .as_deref()
            .map(crate::entity::normalize_behaviour_cid)
            .transpose()
        {
            Ok(value) => value.as_deref().map(IpldLink::new),
            Err(e) => {
                warn!(fragment = %req.fragment, kind = %req.kind_protocol, error = %e,
                    "ma_create_entity: invalid behaviour reference; skipped");
                return;
            }
        },
        acl: entity.acl.clone(),
        state: None,
        parent: Some(entity.fragment.clone()),
        label: None,
        attributes: std::collections::BTreeMap::new(),
        init: None,
        initialised: false,
        reload_error: None,
    };

    let (iroh_node_id, started_at) = {
        let stats = side_effects.stats.read().await;
        (stats.endpoint_id.clone(), stats.started_at)
    };
    let runtime_config = public_plugin_config_for_local(ctx.kubo_url, side_effects)
        .await
        .unwrap_or_else(|e| {
            warn!(error = %e, "ma_create_entity: failed to build public plugin config; continuing with entity-local config only");
            std::collections::BTreeMap::new()
        });

    let load_result =
        crate::plugin::EntityPlugin::load_with_fibonacci_backoff(crate::plugin::LoadArgs {
            fragment: req.fragment.clone(),
            node: &entity_node,
            kind_node: &kind_node,
            our_did: ctx.our_did,
            kubo_url: ctx.kubo_url,
            envelope_tx: side_effects.envelope_tx.clone(),
            entity_registry: entity_registry.clone(),
            iroh_node_id: &iroh_node_id,
            started_at,
            runtime_config,
            init_payload: req.init_payload.clone(),
        })
        .await;

    finish_create_request(load_result, &req, entity_node, ctx).await;
}

/// Handle the outcome of `EntityPlugin::load_with_fibonacci_backoff` for one
/// `ma_create_entity` request: register a successfully started entity (or
/// report failure back to the requesting parent).
async fn finish_create_request(
    load_result: anyhow::Result<(crate::plugin::EntityPlugin, Lifecycle)>,
    req: &crate::entity::CreateEntityRequest,
    entity_node: EntityNode,
    ctx: &CreateCtx<'_>,
) {
    let CreateCtx {
        entity_registry,
        manifest_writer,
        kubo_url,
        our_did,
        side_effects,
    } = *ctx;
    match load_result {
        Ok((ep, Lifecycle::Running)) => {
            let mut running_node = entity_node.clone();
            running_node.initialised = true;
            if let Ok(Some(cid)) = ep.trigger_save(kubo_url).await {
                running_node.state = Some(IpldLink::new(cid));
            }
            let registered =
                register_created_entity(entity_registry, req.fragment.clone(), Arc::new(ep)).await;
            if !registered {
                debug!(fragment = %req.fragment, kind = %req.kind_protocol,
                    "ma_create_entity: concurrent entity won registration; discarding duplicate");
                return;
            }
            info!(fragment = %req.fragment, kind = %req.kind_protocol,
                parent = %req.parent, "entity created via ma_create_entity");
            let fragment = req.fragment.clone();
            let writer = manifest_writer.clone();
            tokio::spawn(async move {
                if let Err(e) = writer.insert_entity(&fragment, &running_node).await {
                    warn!(fragment = %fragment, error = %e, "failed to persist new entity to manifest");
                }
            });
        }
        Ok((_, Lifecycle::Error)) => {
            warn!(fragment = %req.fragment, kind = %req.kind_protocol,
                "ma_create_entity: init() returned :error; entity discarded");
            let err_content = {
                let mut buf = Vec::new();
                let _ = ciborium::ser::into_writer(
                    &CborValue::Array(vec![
                        CborValue::Text(":error".into()),
                        CborValue::Text(format!("init() failed for #{}", req.fragment)),
                        CborValue::Text(req.fragment.clone()),
                    ]),
                    &mut buf,
                );
                buf
            };
            let _ = crate::plugin::enqueue_envelope(
                &side_effects.envelope_tx,
                &req.parent,
                SendEnvelope {
                    to: format!("{our_did}#{}", req.parent),
                    content_type: CONTENT_TYPE_TERM.to_string(),
                    message_type: None,
                    content: err_content,
                    reply_to: None,
                },
            );
        }
        Err(e) => {
            warn!(fragment = %req.fragment, kind = %req.kind_protocol,
                error = %e, "ma_create_entity: EntityPlugin::load failed");
        }
    }
}

/// Arguments for [`run`], bundled to keep the call site (and this module's
/// public surface) manageable — see [`EventLoopState`] for the equivalent
/// bundle used internally once the loop is running.
pub struct RunArgs {
    pub endpoint: Arc<dyn MaEndpoint>,
    pub rpc_messages: Inbox<Message>,
    pub inbox_messages: Inbox<Message>,
    pub crud_messages: Option<Inbox<Message>>,
    pub ipfs_state: Option<IpfsServiceState>,
    pub envelope_tx: Sender<(String, SendEnvelope)>,
    pub envelope_rx: Receiver<(String, SendEnvelope)>,
    pub shared_config: Arc<RwLock<Config>>,
    pub shared_resolver: Arc<dyn DidDocumentResolver>,
    pub stats: SharedStats,
    pub acl: SharedAcl,
    pub acl_cache: AclCache,
    pub group_cache: GroupCache,
    pub entity_registry: EntityRegistry,
    pub kind_registry: KindRegistry,
    pub manifest_writer: ManifestWriter,
    pub our_did: String,
    pub signing_key: SigningKey,
    pub runtime_ipns_key: [u8; 32],
    pub runtime_slug: String,
    pub remote_pin: Option<ma_core::config::RemotePinConfig>,
    pub did_publish_timeout_secs: u64,
    pub ipns_publish: ipfs::IpnsPublishSettings,
    pub did_resolve: ipfs::DidResolveSettings,
    pub poll_ms: u64,
}

/// All state the running event loop needs, one field per former `run`
/// parameter plus the dispatch/creation gates. Each protocol's drain logic
/// is a separate method so no single function grows unreadably long.
struct EventLoopState {
    endpoint: Arc<dyn MaEndpoint>,
    rpc_messages: Inbox<Message>,
    inbox_messages: Inbox<Message>,
    crud_messages: Option<Inbox<Message>>,
    ipfs_state: Option<IpfsServiceState>,
    envelope_tx: Sender<(String, SendEnvelope)>,
    envelope_rx: Receiver<(String, SendEnvelope)>,
    shared_config: Arc<RwLock<Config>>,
    shared_resolver: Arc<dyn DidDocumentResolver>,
    stats: SharedStats,
    acl: SharedAcl,
    acl_cache: AclCache,
    group_cache: GroupCache,
    entity_registry: EntityRegistry,
    kind_registry: KindRegistry,
    manifest_writer: ManifestWriter,
    our_did: String,
    signing_key: SigningKey,
    runtime_ipns_key: [u8; 32],
    runtime_slug: String,
    remote_pin: Option<ma_core::config::RemotePinConfig>,
    did_publish_timeout_secs: u64,
    ipns_publish: ipfs::IpnsPublishSettings,
    did_resolve: ipfs::DidResolveSettings,
    local_plugin_dispatch_gate: Arc<Semaphore>,
    remote_plugin_delivery_gate: Arc<Semaphore>,
    entity_creation_gate: Arc<Mutex<()>>,
}

impl EventLoopState {
    fn new(args: RunArgs) -> Self {
        let RunArgs {
            endpoint,
            rpc_messages,
            inbox_messages,
            crud_messages,
            ipfs_state,
            envelope_tx,
            envelope_rx,
            shared_config,
            shared_resolver,
            stats,
            acl,
            acl_cache,
            group_cache,
            entity_registry,
            kind_registry,
            manifest_writer,
            our_did,
            signing_key,
            runtime_ipns_key,
            runtime_slug,
            remote_pin,
            did_publish_timeout_secs,
            ipns_publish,
            did_resolve,
            poll_ms: _,
        } = args;

        Self {
            endpoint,
            rpc_messages,
            inbox_messages,
            crud_messages,
            ipfs_state,
            envelope_tx,
            envelope_rx,
            shared_config,
            shared_resolver,
            stats,
            acl,
            acl_cache,
            group_cache,
            entity_registry,
            kind_registry,
            manifest_writer,
            our_did,
            signing_key,
            runtime_ipns_key,
            runtime_slug,
            remote_pin,
            did_publish_timeout_secs,
            ipns_publish,
            did_resolve,
            local_plugin_dispatch_gate: Arc::new(Semaphore::new(LOCAL_PLUGIN_DISPATCH_LIMIT)),
            remote_plugin_delivery_gate: Arc::new(Semaphore::new(REMOTE_PLUGIN_DELIVERY_LIMIT)),
            entity_creation_gate: Arc::new(Mutex::new(())),
        }
    }

    /// One tick: drain every inbound protocol inbox, then the plugin outbox.
    async fn drain_tick(&mut self) {
        let now = status::now_unix_secs();
        let kubo_url = self.shared_config.read().await.kubo_rpc_url.clone();
        let shared_doc_cache = self
            .ipfs_state
            .as_ref()
            .map(|ipfs| Arc::clone(&ipfs.doc_cache));

        self.drain_rpc_messages(now, shared_doc_cache.as_ref(), &kubo_url)
            .await;
        self.drain_ipfs_messages(now, &kubo_url).await;
        self.drain_crud_messages(now, shared_doc_cache.as_ref(), &kubo_url)
            .await;
        self.drain_inbox_messages(now, shared_doc_cache.as_ref(), &kubo_url)
            .await;
        self.drain_plugin_envelopes(&kubo_url).await;
    }

    // Drain /ma/rpc/0.0.1
    async fn drain_rpc_messages(
        &self,
        now: u64,
        shared_doc_cache: Option<&ipfs::DocCache>,
        kubo_url: &str,
    ) {
        while let Some(mut message) = self.rpc_messages.pop(now) {
            if let Some(doc_cache) = shared_doc_cache {
                ipfs::record_inbound_contact(doc_cache, &message.from).await;
            }
            debug!(
                node = %message.from,
                protocol = rpc::RPC_PROTOCOL_ID,
                "{}", i18n::t("node-connected")
            );
            debug!(
                from = %message.from,
                to = %message.to,
                id = %message.id,
                message_type = %message.message_type,
                "{}", i18n::t("rpc-message-received")
            );
            {
                let mut s = self.stats.write().await;
                s.rpc_requests += 1;
            }
            let acl_snapshot = self.acl.read().await.clone();
            let ctx = rpc::RpcHandlerCtx {
                our_did: Arc::from(self.our_did.as_str()),
                signing_key: Arc::new(self.signing_key.clone()),
                endpoint: Arc::clone(&self.endpoint),
                kubo_rpc_url: Arc::from(kubo_url),
                resolver: Arc::clone(&self.shared_resolver),
                doc_cache: shared_doc_cache.map(Arc::clone),
                did_resolve: self.did_resolve,
                entity_registry: self.entity_registry.clone(),
                kind_registry: self.kind_registry.clone(),
                envelope_tx: self.envelope_tx.clone(),
                stats: self.stats.clone(),
                acl_cache: self.acl_cache.clone(),
                group_cache: self.group_cache.clone(),
                manifest_writer: self.manifest_writer.clone(),
                shared_config: Arc::clone(&self.shared_config),
                runtime_slug: Arc::from(self.runtime_slug.as_str()),
                runtime_ipns_key: self.runtime_ipns_key,
                ipns_publish: self.ipns_publish,
                did_publish_timeout_secs: self.did_publish_timeout_secs,
            };
            tokio::spawn(async move {
                if let Err(err) = tokio::time::timeout(
                    Duration::from_secs(30),
                    rpc::handle_rpc_message(&message, &acl_snapshot, &ctx),
                )
                .await
                .unwrap_or_else(|_| Err(anyhow::anyhow!("rpc handler timed out")))
                {
                    warn!(error = %err, from = %message.from, "{}", i18n::t("rpc-message-rejected"));
                }
                message.content.zeroize();
                message.signature.zeroize();
            });
        }
    }

    // Drain /ma/ipfs/0.0.1
    async fn drain_ipfs_messages(&mut self, now: u64, kubo_url: &str) {
        let Some(ref mut ipfs) = self.ipfs_state else {
            return;
        };
        while let Some(mut message) = ipfs.messages.pop(now) {
            ipfs::record_inbound_contact(&ipfs.doc_cache, &message.from).await;
            debug!(
                node = %message.from,
                protocol = IPFS_PROTOCOL_ID,
                "{}", i18n::t("node-connected")
            );
            debug!(
                from = %message.from,
                to = %message.to,
                id = %message.id,
                message_type = %message.message_type,
                content_len = message.content.len(),
                "{}", i18n::t("received-encrypted-ma-msg")
            );
            {
                let mut s = self.stats.write().await;
                s.ipfs_requests += 1;
            }
            let acl_snapshot = self.acl.read().await.clone();
            if let Err(err) = tokio::time::timeout(
                Duration::from_mins(1),
                ipfs::handle_ipfs_message(
                    &message,
                    &acl_snapshot,
                    &ipfs::IpfsHandlerCtx {
                        our_did: &self.our_did,
                        signing_key: &self.signing_key,
                        endpoint: Arc::clone(&self.endpoint),
                        kubo_rpc_url: kubo_url,
                        remote_pin: self.remote_pin.as_ref(),
                        ipns_publish: self.ipns_publish,
                        did_resolve: self.did_resolve,
                        resolver: Arc::clone(&self.shared_resolver),
                        doc_cache: Arc::clone(&ipfs.doc_cache),
                        group_cache: self.group_cache.clone(),
                    },
                    &mut ipfs.replay_guard,
                ),
            )
            .await
            .unwrap_or_else(|_| Err(anyhow::anyhow!("ipfs handler timed out")))
            {
                warn!(error = %err, from = %message.from, "{}", i18n::t("ipfs-message-rejected"));
            }
            message.content.zeroize();
            message.signature.zeroize();
        }
    }

    // Drain /ma/crud/0.0.1
    async fn drain_crud_messages(
        &mut self,
        now: u64,
        shared_doc_cache: Option<&ipfs::DocCache>,
        kubo_url: &str,
    ) {
        let Some(ref mut crud_inbox) = self.crud_messages else {
            return;
        };
        while let Some(mut message) = crud_inbox.pop(now) {
            if let Some(doc_cache) = shared_doc_cache {
                ipfs::record_inbound_contact(doc_cache, &message.from).await;
            }
            info!(
                from = %message.from,
                to = %message.to,
                id = %message.id,
                message_type = %message.message_type,
                "{}", i18n::t("crud-message-received")
            );
            // Snapshot the ACL and drop the read guard *before* the
            // await. handle_crud_message may acquire a write lock on
            // the same SharedAcl (e.g. :acl: edit-save), and holding
            // a read guard across that await would deadlock.
            let acl_snapshot = self.acl.read().await.clone();
            let ctx = crud::CrudHandlerCtx {
                our_did: Arc::from(self.our_did.as_str()),
                signing_key: Arc::new(self.signing_key.clone()),
                endpoint: Arc::clone(&self.endpoint),
                kubo_rpc_url: Arc::from(kubo_url),
                resolver: Arc::clone(&self.shared_resolver),
                doc_cache: shared_doc_cache.map(Arc::clone),
                did_resolve: self.did_resolve,
                stats: self.stats.clone(),
                entity_registry: self.entity_registry.clone(),
                kind_registry: self.kind_registry.clone(),
                shared_config: Arc::clone(&self.shared_config),
                acl_cache: self.acl_cache.clone(),
                group_cache: self.group_cache.clone(),
                root_acl: self.acl.clone(),
                envelope_tx: self.envelope_tx.clone(),
                manifest_writer: self.manifest_writer.clone(),
            };
            if let Err(err) = tokio::time::timeout(
                Duration::from_secs(30),
                crud::handle_crud_message(&message, &acl_snapshot, &ctx),
            )
            .await
            .unwrap_or_else(|_| Err(anyhow::anyhow!("crud handler timed out")))
            {
                warn!(error = %err, from = %message.from, "CRUD message rejected");
            }
            message.content.zeroize();
            message.signature.zeroize();
        }
    }

    // Drain /ma/inbox/0.0.1
    async fn drain_inbox_messages(
        &self,
        now: u64,
        shared_doc_cache: Option<&ipfs::DocCache>,
        kubo_url: &str,
    ) {
        while let Some(mut message) = self.inbox_messages.pop(now) {
            if let Some(doc_cache) = shared_doc_cache {
                ipfs::record_inbound_contact(doc_cache, &message.from).await;
            }
            debug!(
                from = %message.from,
                to = %message.to,
                message_type = %message.message_type,
                "{}", i18n::t("inbox-message-received")
            );
            let ctx = inbox::InboxHandlerCtx {
                our_did: Arc::from(self.our_did.as_str()),
                entity_registry: self.entity_registry.clone(),
                kubo_rpc_url: Arc::from(kubo_url),
                manifest_writer: self.manifest_writer.clone(),
            };
            tokio::spawn(async move {
                if let Err(err) = tokio::time::timeout(
                    Duration::from_secs(30),
                    inbox::handle_inbox_message(&message, &ctx),
                )
                .await
                .unwrap_or_else(|_| Err(anyhow::anyhow!("inbox handler timed out")))
                {
                    warn!(error = %err, from = %message.from, "inbox message rejected");
                }
                message.content.zeroize();
                message.signature.zeroize();
            });
        }
    }

    // Drain plugin outbox — envelopes sent fire-and-forget by ma_send/ma_reply.
    // Keep this bounded so a plugin message loop cannot monopolise
    // the event loop and starve CRUD/kinds/status processing.
    async fn drain_plugin_envelopes(&mut self, kubo_url: &str) {
        let mut drained_plugin_envelopes = 0usize;
        for _ in 0..PLUGIN_OUTBOX_DRAIN_BUDGET {
            let Ok((fragment, env)) = self.envelope_rx.try_recv() else {
                break;
            };
            drained_plugin_envelopes += 1;
            let msg_type = if env.reply_to.is_some() {
                MESSAGE_TYPE_RPC_REPLY.to_string()
            } else {
                env.message_type
                    .clone()
                    .unwrap_or_else(|| MESSAGE_TYPE_RPC.to_string())
            };
            if let Some(target_fragment) = local_target_fragment(&env.to, &self.our_did) {
                self.dispatch_envelope_locally(fragment, target_fragment, env, msg_type, kubo_url)
                    .await;
                continue;
            }
            self.dispatch_envelope_remotely(fragment, env, &msg_type);
        }
        if drained_plugin_envelopes == PLUGIN_OUTBOX_DRAIN_BUDGET {
            warn!(
                budget = PLUGIN_OUTBOX_DRAIN_BUDGET,
                "plugin outbox drain budget exhausted; deferring remaining envelopes"
            );
        }
    }

    /// Hand a plugin envelope addressed to one of our own entities off to
    /// `dispatch_local_plugin_envelope`, bounded by the local dispatch gate.
    async fn dispatch_envelope_locally(
        &self,
        fragment: String,
        target_fragment: String,
        env: SendEnvelope,
        msg_type: String,
        kubo_url: &str,
    ) {
        if env.reply_to.is_some() {
            debug!(
                fragment = %fragment,
                target = %target_fragment,
                reply_to = ?env.reply_to,
                "plugin envelope: local RPC reply dropped (no local reply waiter)"
            );
            return;
        }
        let side_effects = LocalSideEffectCtx {
            kind_registry: self.kind_registry.clone(),
            envelope_tx: self.envelope_tx.clone(),
            stats: self.stats.clone(),
            shared_config: Arc::clone(&self.shared_config),
            entity_creation_gate: Arc::clone(&self.entity_creation_gate),
        };
        let Ok(permit) = self
            .local_plugin_dispatch_gate
            .clone()
            .acquire_owned()
            .await
        else {
            warn!(fragment = %fragment, target = %target_fragment, "plugin envelope: local dispatch gate closed");
            return;
        };
        let args = LocalDispatchArgs {
            sender_fragment: fragment.clone(),
            target_fragment: target_fragment.clone(),
            env,
            msg_type,
            entity_registry: self.entity_registry.clone(),
            manifest_writer: self.manifest_writer.clone(),
            kubo_url: kubo_url.to_string(),
            our_did: self.our_did.clone(),
            side_effects: Some(side_effects),
        };
        tokio::spawn(async move {
            let _permit = permit;
            if tokio::time::timeout(
                Duration::from_secs(30),
                dispatch_local_plugin_envelope(args),
            )
            .await
            .is_err()
            {
                warn!(fragment = %fragment, target = %target_fragment, "plugin envelope: local dispatch timed out");
            }
        });
    }

    /// Sign a plugin envelope addressed to a remote peer and hand delivery
    /// off to a spawned task, bounded by the remote delivery gate.
    fn dispatch_envelope_remotely(&self, fragment: String, env: SendEnvelope, msg_type: &str) {
        let our_did = &self.our_did;
        let sender_did_url = format!("{our_did}#{fragment}");
        let recipient = match Did::try_from(env.to.as_str()) {
            Ok(d) => d,
            Err(e) => {
                warn!(fragment = %fragment, to = %env.to, error = %e, "plugin envelope: invalid recipient DID; skipped");
                return;
            }
        };
        let message = match env.reply_to.as_deref() {
            Some(reply_to) => ma_core::Message::new_reply(
                &sender_did_url,
                &env.to,
                msg_type,
                &env.content_type,
                &env.content,
                reply_to,
                &self.signing_key,
            ),
            None => ma_core::Message::new(
                &sender_did_url,
                &env.to,
                msg_type,
                &env.content_type,
                &env.content,
                &self.signing_key,
            ),
        };
        let msg = match message {
            Ok(m) => m,
            Err(e) => {
                warn!(fragment = %fragment, error = %e, "plugin envelope: failed to build message; skipped");
                return;
            }
        };
        let protocol = protocol_for(msg_type);
        // Spawn each delivery independently so one unreachable peer
        // cannot block others. Outbox opening applies its own
        // bounded retries for transient DID/IPNS resolution delays.
        let ep = Arc::clone(&self.endpoint);
        let res = Arc::clone(&self.shared_resolver);
        let doc_cache = self
            .ipfs_state
            .as_ref()
            .map(|ipfs| Arc::clone(&ipfs.doc_cache));
        let base = recipient.base_id();
        let did_resolve = self.did_resolve;
        let Ok(permit) = self.remote_plugin_delivery_gate.clone().try_acquire_owned() else {
            debug!(fragment = %fragment, to = %env.to, limit = REMOTE_PLUGIN_DELIVERY_LIMIT, "plugin envelope: remote delivery limit reached; envelope dropped");
            return;
        };
        tokio::spawn(async move {
            let _permit = permit;
            let outbox_result = if let Some(doc_cache) = doc_cache {
                ipfs::open_outbox_for_did(&ep, &res, &doc_cache, &recipient, protocol, did_resolve)
                    .await
            } else {
                match tokio::time::timeout(
                    Duration::from_secs(5),
                    ep.outbox(res.as_ref(), &base, protocol),
                )
                .await
                {
                    Ok(Ok(outbox)) => Ok(outbox),
                    Ok(Err(err)) => Err(anyhow::Error::from(err)),
                    Err(_) => Err(anyhow::anyhow!("outbox connect timed out (5 s)")),
                }
            };

            match outbox_result {
                Ok(mut outbox) => {
                    if let Err(e) = outbox.send(&msg).await {
                        warn!(fragment = %fragment, to = %env.to, error = %e, "plugin envelope delivery failed; dropping envelope");
                    }
                }
                Err(e) => {
                    debug!(fragment = %fragment, to = %env.to, error = %e, "plugin envelope: outbox open failed; dropping envelope");
                }
            }
        });
    }

    /// Handle a Ctrl-C signal: persist entity state and publish the final
    /// root CID. Returns `true` if the event loop should now break out of
    /// its `select!` loop, or `false` if shutdown was aborted (e.g. entity
    /// state failed to save) and the loop should keep running.
    async fn handle_shutdown(&self, signal: std::io::Result<()>) -> bool {
        if let Err(err) = signal {
            error!(error = %err, "{}", i18n::t("ctrlc-handler-failed"));
        }
        eprintln!();
        eprintln!("{}", i18n::t("shutdown-requested"));
        info!("{}", i18n::t("shutdown-requested"));
        let kubo_url = self.shared_config.read().await.kubo_rpc_url.clone();

        // ── Persist entity states before exit ─────────────────────────
        let Some(rc) = self.stats.read().await.root_cid.clone() else {
            return true;
        };
        let count = self.entity_registry.read().await.len();
        if count > 0 {
            info!(count = %count, "{}", i18n::t("entity-states-saving"));
            match bootstrap::save_all_entity_states(
                &self.manifest_writer,
                &kubo_url,
                &self.entity_registry,
            )
            .await
            {
                Ok(new_cid) => {
                    self.stats.write().await.root_cid = Some(new_cid.clone());
                    info!(cid = %new_cid, "{}", i18n::t("entity-states-saved"));
                }
                Err(e) => {
                    error!(error = %e, "Failed to save entity states");
                    error!("shutdown aborted; runtime remains active so state can be saved on a later shutdown attempt");
                    return false;
                }
            }
        }

        let latest_root_cid = self.stats.read().await.root_cid.clone().unwrap_or(rc);
        {
            let mut config = self.shared_config.write().await;
            if let Err(err) = crate::startup::persist_root_cid(&mut config, &latest_root_cid) {
                error!(root_cid = %latest_root_cid, error = %err, "failed to persist root_cid during shutdown");
            }
        }
        match tokio::time::timeout(
            Duration::from_secs(self.did_publish_timeout_secs),
            ipfs::publish_runtime_root_cid(
                &kubo_url,
                &self.runtime_slug,
                &self.runtime_ipns_key,
                &latest_root_cid,
                self.ipns_publish,
            ),
        )
        .await
        {
            Ok(Ok(_)) => {
                info!(runtime_cid = %latest_root_cid, "shutdown runtime_ipns publish succeeded");
            }
            Ok(Err(err)) => {
                error!(runtime_cid = %latest_root_cid, error = %format!("{err:#}"), "shutdown runtime_ipns publish failed");
            }
            Err(_) => {
                error!(runtime_cid = %latest_root_cid, "shutdown runtime_ipns publish timed out");
            }
        }

        true
    }

    /// Wait for in-flight delivery tasks to release their `Arc` clones, then
    /// close the endpoint gracefully (or give up after 10 s).
    async fn close_endpoint(&mut self) {
        info!("{}", i18n::t("closing-endpoint"));
        let close_deadline = tokio::time::Instant::now() + Duration::from_secs(10);
        while Arc::strong_count(&self.endpoint) > 1 && tokio::time::Instant::now() < close_deadline
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        match Arc::get_mut(&mut self.endpoint) {
            Some(ep) => {
                if tokio::time::timeout(Duration::from_secs(5), ep.close())
                    .await
                    .is_err()
                {
                    warn!("endpoint close timed out after 5 s; forcing exit");
                }
            }
            None => {
                warn!(
                    "endpoint still held by in-flight tasks after 10 s; dropping without graceful close"
                );
            }
        }
        info!("{}", i18n::t("shutdown-complete"));
    }
}

/// The daemon's main event loop and graceful shutdown.
pub async fn run(args: RunArgs) -> Result<()> {
    let poll_ms = args.poll_ms;
    let mut state = EventLoopState::new(args);
    let mut ticker = tokio::time::interval(Duration::from_millis(poll_ms));
    let mut ctrl_c = Box::pin(tokio::signal::ctrl_c());

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                state.drain_tick().await;
            }
            signal = &mut ctrl_c => {
                if state.handle_shutdown(signal).await {
                    break;
                }
                ctrl_c = Box::pin(tokio::signal::ctrl_c());
            }
        }
    }

    state.close_endpoint().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{insert_if_absent, local_actor_url, local_target_fragment};

    fn test_did(seed: u8) -> String {
        format!(
            "did:ma:{}",
            ma_core::ipns_from_secret([seed; 32]).expect("test IPNS identifier")
        )
    }

    #[test]
    fn concurrent_entity_creation_keeps_first_registered_entity() {
        let mut registry = HashMap::new();

        assert!(insert_if_absent(&mut registry, "avatar", "with-inventory"));
        assert!(!insert_if_absent(
            &mut registry,
            "avatar",
            "without-inventory"
        ));
        assert_eq!(registry.get("avatar"), Some(&"with-inventory"));
    }

    #[test]
    fn local_target_fragment_accepts_local_forms_only() {
        let our_did = test_did(1);
        let remote_did = test_did(2);

        assert_eq!(local_target_fragment("#room", &our_did), None);
        assert_eq!(local_target_fragment("room", &our_did), None);
        assert_eq!(
            local_target_fragment(&format!("{our_did}#room"), &our_did),
            Some("room".to_string())
        );
        assert_eq!(
            local_target_fragment(&format!("{our_did}#room"), &format!("{our_did}#root")),
            Some("room".to_string())
        );

        assert_eq!(local_target_fragment(&our_did, &our_did), None);
        assert_eq!(
            local_target_fragment(&format!("{remote_did}#room"), &our_did),
            None
        );
        assert_eq!(local_target_fragment("/entities/room", &our_did), None);
    }

    #[test]
    fn local_actor_url_always_uses_own_base_did() {
        let our_did = test_did(1);
        assert_eq!(local_actor_url(&our_did, "rms"), format!("{our_did}#rms"));
        assert_eq!(
            local_actor_url(&format!("{our_did}#root"), "rms"),
            format!("{our_did}#rms")
        );
    }
}
