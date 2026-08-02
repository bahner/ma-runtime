//! `/ma/rpc/0.0.1` handler: entity plugin dispatch and `:ping`.

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use ciborium::Value as CborValue;
use ma_core::{
    Did, DidDocumentResolver, Ipld, SigningKey, CONTENT_TYPE_TERM, MESSAGE_TYPE_RPC,
    MESSAGE_TYPE_RPC_REPLY,
};
use tracing::{debug, error, info, warn};

use crate::acl::{check_full, AclCache, AclMap, GroupCache, CAP_RPC};
use crate::entity::{
    CastInput, IpldLink, Lifecycle, LocalMessage, PluginMsg, SendEnvelope, SetBehaviourRequest,
};
use crate::plugin::EntityRegistry;
use crate::routing::local_target_fragment;
use crate::status::SharedStats;

pub const RPC_PROTOCOL_ID: &str = "/ma/rpc/0.0.1";

// ── Handler context ────────────────────────────────────────────────────────────

pub struct RpcHandlerCtx {
    pub our_did: Arc<str>,
    pub signing_key: Arc<SigningKey>,
    pub endpoint: Arc<dyn ma_core::MaEndpoint>,
    pub kubo_rpc_url: Arc<str>,
    pub resolver: Arc<dyn DidDocumentResolver>,
    pub doc_cache: Option<crate::ipfs::DocCache>,
    pub did_resolve: crate::ipfs::DidResolveSettings,
    pub entity_registry: EntityRegistry,
    pub kind_registry: crate::entity::KindRegistry,
    pub envelope_tx: tokio::sync::mpsc::Sender<(String, SendEnvelope)>,
    pub stats: SharedStats,
    pub acl_cache: AclCache,
    pub group_cache: GroupCache,
    pub manifest_writer: crate::manifest::ManifestWriter,
    pub shared_config: Arc<tokio::sync::RwLock<ma_core::Config>>,
    pub runtime_slug: Arc<str>,
    pub runtime_ipns_key: [u8; 32],
    pub ipns_publish: crate::ipfs::IpnsPublishSettings,
    pub did_publish_timeout_secs: u64,
}

// ── Entity creation helper ─────────────────────────────────────────────────────

async fn persist_new_entity(
    manifest_writer: &crate::manifest::ManifestWriter,
    kubo_url: &str,
    fragment: &str,
    entity_node: &crate::entity::EntityNode,
) -> Result<()> {
    let entity_cid = crate::kubo::dag_put(kubo_url, entity_node).await?;
    let fragment = fragment.to_string();
    manifest_writer
        .mutate(move |m| {
            m.entities.insert(fragment, IpldLink::new(&entity_cid));
            Ok(())
        })
        .await?;
    Ok(())
}

async fn public_plugin_config_for_rpc(
    ctx: &RpcHandlerCtx,
) -> Result<std::collections::BTreeMap<String, String>> {
    let root_cid = ctx
        .stats
        .read()
        .await
        .root_cid
        .clone()
        .ok_or_else(|| anyhow!("no manifest root CID available"))?;
    let manifest: crate::entity::RuntimeManifest =
        crate::kubo::dag_get(&ctx.kubo_rpc_url, &root_cid).await?;
    let cfg = ctx.shared_config.read().await;
    Ok(crate::crud::config::public_plugin_config(&manifest, &cfg))
}

fn normalize_behaviour_cid(value: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("behaviour reference is empty"));
    }
    if let Some(cid) = trimmed.strip_prefix("/ipfs/") {
        if cid.is_empty() {
            return Err(anyhow!("/ipfs/ behaviour reference is missing a CID"));
        }
        return Ok(cid.to_string());
    }
    if trimmed.starts_with("/ipns/") {
        return Err(anyhow!("/ipns/ behaviour references are not supported here; publish the code to /ipfs/<cid> first"));
    }
    Ok(trimmed.to_string())
}

async fn load_entity_node_for_update(
    ctx: &RpcHandlerCtx,
    fragment: &str,
    behaviour_cid: Option<&str>,
) -> Result<crate::entity::EntityNode> {
    let root_cid = ctx
        .stats
        .read()
        .await
        .root_cid
        .clone()
        .ok_or_else(|| anyhow!("no manifest root CID available"))?;
    let manifest: crate::entity::RuntimeManifest =
        crate::kubo::dag_get(&ctx.kubo_rpc_url, &root_cid).await?;
    let link = manifest
        .entities
        .get(fragment)
        .ok_or_else(|| anyhow!("entity '{fragment}' is not in the manifest"))?;
    let mut node: crate::entity::EntityNode =
        crate::kubo::dag_get(&ctx.kubo_rpc_url, &link.cid).await?;
    node.behaviour = behaviour_cid.map(IpldLink::new);
    Ok(node)
}

async fn apply_behaviour_request(req: SetBehaviourRequest, ctx: &RpcHandlerCtx) -> Result<()> {
    let behaviour_cid = req
        .behaviour_cid
        .as_deref()
        .map(normalize_behaviour_cid)
        .transpose()?;
    let reload_shutdown_timeout = {
        let cfg = ctx.shared_config.read().await;
        crate::crud::config::wasm_reload_shutdown_timeout(&cfg)
    };
    let current_entity = ctx.entity_registry.read().await.get(&req.fragment).cloned();
    if let Some(ref current) = current_entity {
        match current
            .prepare_reload_save(&ctx.kubo_rpc_url, reload_shutdown_timeout)
            .await
        {
            Ok(Some(state_cid)) => {
                ctx.manifest_writer
                    .set_entity_state(&req.fragment, &state_cid)
                    .await?;
            }
            Ok(None) => {}
            Err(e) => {
                warn!(fragment = %req.fragment, timeout_ms = reload_shutdown_timeout.as_millis(), error = %e, "ma_set_behaviour: failed to persist current state before reload; proceeding with replacement");
            }
        }
    }
    let mut updated_node =
        load_entity_node_for_update(ctx, &req.fragment, behaviour_cid.as_deref()).await?;
    if current_entity.is_some() {
        updated_node.initialised = true;
    }
    let kind_node = ctx
        .kind_registry
        .read()
        .await
        .get(&updated_node.kind)
        .cloned()
        .ok_or_else(|| anyhow!("kind '{}' is not in registry", updated_node.kind))?;
    let (iroh_node_id, started_at) = {
        let s = ctx.stats.read().await;
        (s.endpoint_id.clone(), s.started_at)
    };
    let runtime_config = public_plugin_config_for_rpc(ctx).await.unwrap_or_else(|e| {
        warn!(error = %e, "ma_set_behaviour: failed to build public plugin config; continuing with entity-local config only");
        std::collections::BTreeMap::new()
    });
    let (plugin, lifecycle) = crate::plugin::EntityPlugin::load_with_fibonacci_backoff(
        req.fragment.clone(),
        &updated_node,
        &kind_node,
        &ctx.our_did,
        &ctx.kubo_rpc_url,
        ctx.envelope_tx.clone(),
        ctx.entity_registry.clone(),
        &iroh_node_id,
        started_at,
        runtime_config,
        None,
    )
    .await?;
    let plugin = Arc::new(plugin);
    let state_bytes = plugin.trigger_save(&ctx.kubo_rpc_url).await.ok().flatten();

    let root_cid = ctx
        .manifest_writer
        .set_entity_behaviour(&req.fragment, behaviour_cid.as_deref())
        .await?;
    if let Some(state_cid) = state_bytes {
        if let Err(e) = ctx
            .manifest_writer
            .set_entity_state(&req.fragment, &state_cid)
            .await
        {
            warn!(fragment = %req.fragment, cid = %state_cid, error = %e, "ma_set_behaviour: failed to persist reloaded entity state");
        }
    }
    ctx.entity_registry
        .write()
        .await
        .insert(req.fragment.clone(), plugin);
    if let Some(current) = current_entity {
        current.terminate_worker();
    }
    info!(fragment = %req.fragment, ?lifecycle, %root_cid, "entity behaviour updated via ma_set_behaviour");
    Ok(())
}

// ── Entry point ────────────────────────────────────────────────────────────────

pub async fn handle_rpc_message(
    message: &ma_core::Message,
    acl: &AclMap,
    ctx: &RpcHandlerCtx,
) -> Result<()> {
    if rpc_message_kind(&message.message_type) == RpcMessageKind::Reply {
        debug!(
            from = %message.from,
            to = %message.to,
            reply_to = ?message.reply_to,
            "RPC reply ignored: no runtime reply waiter"
        );
        return Ok(());
    }

    let owners = ctx.stats.read().await.owners.clone();
    if !crate::acl::is_owner(&owners, &message.from) {
        let group_cache = ctx.group_cache.clone();
        check_full(acl, &message.from, &[CAP_RPC], |key| {
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
    }

    if rpc_message_kind(&message.message_type) != RpcMessageKind::Request {
        return Err(anyhow!(
            "unsupported RPC message type '{}' on {}",
            message.message_type,
            RPC_PROTOCOL_ID,
        ));
    }

    let payload = message.payload();
    if payload.is_empty() {
        let reason = "empty RPC payload";
        error!(
            from = %message.from,
            to = %message.to,
            id = %message.id,
            message_type = %message.message_type,
            "RPC message rejected: empty payload"
        );
        return send_rpc_error_reply(message, ctx, reason);
    }

    let term: CborValue =
        ciborium::de::from_reader(payload.as_slice()).context("invalid CBOR in RPC message")?;

    // Fragment routing: entity plugin dispatch.
    if let Some(fragment) = local_target_fragment(&message.to, &ctx.our_did) {
        if fragment == "root" && rpc_verb(&term) == Some(":publish") {
            return handle_root_publish_rpc(message, ctx, &owners).await;
        }
        let ep = ctx.entity_registry.read().await.get(&fragment).cloned();
        return if let Some(entity) = ep {
            let fragment_for_log = entity.fragment.clone();
            match handle_entity_plugin_message(message, term, entity, ctx).await {
                Ok(reply) => {
                    if let Some(content) = reply {
                        send_rpc_reply(message, ctx, &content)?;
                    }
                    Ok(())
                }
                Err(err) => {
                    let reason = err.to_string();
                    warn!(
                        fragment = %fragment_for_log,
                        from = %message.from,
                        error = %reason,
                        "plugin dispatch rejected"
                    );
                    send_rpc_error_reply(message, ctx, &reason)
                }
            }
        } else {
            let reason = format!("unknown entity fragment: {fragment}");
            debug!(fragment = %fragment, "{}", crate::i18n::t("entity-not-found"));
            send_rpc_error_reply(message, ctx, &reason)
        };
    }

    handle_root_runtime_rpc(message, ctx, &term).await
}

async fn handle_root_publish_rpc(
    message: &ma_core::Message,
    ctx: &RpcHandlerCtx,
    owners: &[String],
) -> Result<()> {
    if !crate::acl::is_owner(owners, &message.from) {
        return send_rpc_error_reply(message, ctx, "owner required");
    }

    let Some(root_cid) = ctx.stats.read().await.root_cid.clone() else {
        return send_rpc_error_reply(message, ctx, "no manifest root CID available");
    };
    let kubo_url = ctx.kubo_rpc_url.to_string();
    match tokio::time::timeout(
        std::time::Duration::from_secs(ctx.did_publish_timeout_secs),
        crate::ipfs::publish_runtime_root_cid(
            &kubo_url,
            &ctx.runtime_slug,
            &ctx.runtime_ipns_key,
            &root_cid,
            ctx.ipns_publish,
        ),
    )
    .await
    {
        Ok(Ok(_)) => {}
        Ok(Err(err)) => return send_rpc_error_reply(message, ctx, &format!("{err:#}")),
        Err(_) => return send_rpc_error_reply(message, ctx, "root publish timed out"),
    }

    let mut payload = Vec::new();
    ciborium::ser::into_writer(
        &CborValue::Array(vec![
            CborValue::Text(":ok".to_string()),
            CborValue::Text(root_cid),
        ]),
        &mut payload,
    )
    .context("encode :publish reply")?;
    send_rpc_reply(message, ctx, &payload)
}

fn rpc_verb(term: &CborValue) -> Option<&str> {
    match term {
        CborValue::Text(verb) => Some(verb.as_str()),
        CborValue::Array(items) => match items.first() {
            Some(CborValue::Text(verb)) => Some(verb.as_str()),
            _ => None,
        },
        _ => None,
    }
}

async fn handle_root_runtime_rpc(
    message: &ma_core::Message,
    ctx: &RpcHandlerCtx,
    term: &CborValue,
) -> Result<()> {
    let ping_text = match term {
        CborValue::Text(s) => s.as_str(),
        _ => return send_rpc_i18n_error(message, ctx, "rpc-not-text-atom").await,
    };

    match ping_text {
        ":ping" => {
            debug!("{}", crate::i18n::t("ping-received"));
            let mut pong = Vec::new();
            ciborium::ser::into_writer(&CborValue::Text(":pong".to_string()), &mut pong)
                .context("encode :pong")?;
            send_rpc_reply(message, ctx, &pong)
        }
        ":name" => send_text_atom_reply(
            message,
            ctx,
            &runtime_config_text_value(ctx, "name").await?,
            "encode :name reply",
        ),
        ":description" => send_text_atom_reply(
            message,
            ctx,
            &runtime_config_text_value(ctx, "description").await?,
            "encode :description reply",
        ),
        _ => send_rpc_i18n_error(message, ctx, "rpc-unknown-verb").await,
    }
}

fn send_text_atom_reply(
    message: &ma_core::Message,
    ctx: &RpcHandlerCtx,
    value: &str,
    encode_context: &str,
) -> Result<()> {
    let mut payload = Vec::new();
    ciborium::ser::into_writer(&CborValue::Text(value.to_string()), &mut payload)
        .with_context(|| encode_context.to_string())?;
    send_rpc_reply(message, ctx, &payload)
}

async fn runtime_config_text_value(ctx: &RpcHandlerCtx, key: &str) -> Result<String> {
    let root_cid = ctx
        .stats
        .read()
        .await
        .root_cid
        .clone()
        .ok_or_else(|| anyhow!("no manifest root CID available"))?;
    let manifest: crate::entity::RuntimeManifest =
        crate::kubo::dag_get(&ctx.kubo_rpc_url, &root_cid).await?;

    manifest
        .config
        .get(key)
        .and_then(serde_yaml::Value::as_str)
        .map_or_else(
            || {
                crate::crud::config::default_manifest_config_value(key)
                    .and_then(|value| value.as_str().map(str::to_string))
                    .ok_or_else(|| anyhow!("config key not found: {key}"))
            },
            |value| Ok(value.to_string()),
        )
}

// ── Fragment extraction ────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RpcMessageKind {
    Request,
    Reply,
    Unsupported,
}

fn rpc_message_kind(message_type: &str) -> RpcMessageKind {
    match message_type {
        MESSAGE_TYPE_RPC => RpcMessageKind::Request,
        MESSAGE_TYPE_RPC_REPLY => RpcMessageKind::Reply,
        _ => RpcMessageKind::Unsupported,
    }
}

// ── Entity plugin dispatch ────────────────────────────────────────────────────

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::too_many_lines
)]
async fn handle_entity_plugin_message(
    message: &ma_core::Message,
    term: CborValue,
    entity: Arc<crate::plugin::EntityPlugin>,
    ctx: &RpcHandlerCtx,
) -> Result<Option<Vec<u8>>> {
    debug!(fragment = %entity.fragment, from = %message.from, "{}", crate::i18n::t("entity-dispatched"));

    // Entity verb-ACL enforcement.
    // Extract the verb from the CBOR term (text atom or first array element).
    let verb_str: Option<String> = match &term {
        CborValue::Text(s) => Some(s.clone()),
        CborValue::Array(items) => {
            if let Some(CborValue::Text(s)) = items.first() {
                Some(s.clone())
            } else {
                None
            }
        }
        _ => None,
    };
    debug!(
        fragment = %entity.fragment,
        from = %message.from,
        to = %message.to,
        id = %message.id,
        reply_to = ?message.reply_to,
        verb = ?verb_str,
        term = ?term,
        "entity RPC dispatch"
    );

    // Empty acl field → deny-all (fail-closed). Matches EntityNode.acl doc contract.
    if entity.acl.is_empty() {
        return Err(anyhow!(
            "entity '{}' has no ACL configured: access denied (fail-closed)",
            entity.fragment
        ));
    }
    let acl_name = &entity.acl;
    let acl_key = format!("acls.{acl_name}");
    let maybe_acl = ctx.acl_cache.read().await.get(&acl_key).cloned();
    if let Some(ref acl_map) = maybe_acl {
        let verb_ref = verb_str.as_deref().unwrap_or("*");
        let group_cache = ctx.group_cache.clone();
        // Pre-normalize caller: `did:ma:<our_did>#fragment` → `#fragment` so
        // that the `"#"` local-entity wildcard in ACL maps matches intra-runtime
        // callers.  This is safe because message signatures are cryptographically
        // verified — a remote peer cannot forge our DID as the sender.
        let caller_did = message
            .from
            .strip_prefix(&format!("{}#", ctx.our_did))
            .map_or_else(|| message.from.clone(), |frag| format!("#{frag}"));
        crate::acl::check_full(acl_map, &caller_did, &[verb_ref, "*"], |key| {
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
        .await
        .with_context(|| {
            format!(
                "entity '{}' ACL denied {} calling {:?}",
                entity.fragment, message.from, verb_str
            )
        })?;
    } else {
        // ACL name set but not in cache → deny (fail-closed).
        return Err(anyhow!(
            "entity '{}' ACL '{}' not found in cache: access denied",
            entity.fragment,
            acl_name
        ));
    }

    let mut content_bytes = Vec::new();
    ciborium::ser::into_writer(&term, &mut content_bytes)
        .context("re-encoding RPC content for plugin dispatch")?;

    let local_msg = LocalMessage {
        id: message.id.clone(),
        from: message.from.clone(),
        to: message.to.clone(),
        created_at: message.created_at,
        exp: message.exp,
        reply_to: message.reply_to.clone(),
        message_type: message.message_type.clone(),
        content_type: message.content_type.clone(),
        content: content_bytes,
    };

    let cast_input = CastInput {
        msg: PluginMsg::from(&local_msg),
    };
    // The underlying plugin error (`e`) can contain arbitrary internal detail —
    // for Wasm/Python plugins this includes a full interpreter traceback with
    // absolute build-machine file paths (venv layout, username, etc). That is
    // never safe to hand to a remote RPC caller. Log the full detail locally
    // and propagate only a sanitized, fragment-scoped reason on the wire.
    let result = entity.on_message(&cast_input).await.map_err(|e| {
        warn!(
            fragment = %entity.fragment,
            from = %message.from,
            error = %e,
            "plugin execution failed"
        );
        anyhow!("entity '{}' plugin execution failed", entity.fragment)
    })?;
    debug!(
        fragment = %entity.fragment,
        from = %message.from,
        to = %message.to,
        id = %message.id,
        verb = ?verb_str,
        create_requests = result.create_requests.len(),
        pending_state = result.pending_state.is_some(),
        delete_requests = result.delete_requests.len(),
        behaviour_requests = result.behaviour_requests.len(),
        "entity RPC dispatch side effects"
    );

    // If the plugin called `ma_set_state` during this dispatch, persist to IPFS.
    // Spawned so the main event loop is not blocked by the IPFS round-trip.
    if let Some(state_bytes) = result.pending_state {
        let kubo_url = ctx.kubo_rpc_url.to_string();
        let writer = ctx.manifest_writer.clone();
        entity.spawn_state_persist(kubo_url, writer, state_bytes, "rpc");
    }

    // Process entity creation requests queued by `ma_create_entity` host function.
    for req in result.create_requests {
        let maybe_kind = ctx
            .kind_registry
            .read()
            .await
            .get(&req.kind_protocol)
            .cloned();
        let Some(kind_node) = maybe_kind else {
            warn!(caller = %entity.fragment, kind = %req.kind_protocol,
                "ma_create_entity: kind not in registry; skipped");
            continue;
        };

        let entity_node = crate::entity::EntityNode {
            kind: req.kind_protocol.clone(),
            behaviour: req
                .behaviour_cid
                .as_deref()
                .map(normalize_behaviour_cid)
                .transpose()?
                .as_deref()
                .map(IpldLink::new),
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
            let s = ctx.stats.read().await;
            (s.endpoint_id.clone(), s.started_at)
        };
        let runtime_config = public_plugin_config_for_rpc(ctx).await.unwrap_or_else(|e| {
            warn!(error = %e, "ma_create_entity: failed to build public plugin config; continuing with entity-local config only");
            std::collections::BTreeMap::new()
        });

        match crate::plugin::EntityPlugin::load(
            req.fragment.clone(),
            &entity_node,
            &kind_node,
            &ctx.our_did,
            &ctx.kubo_rpc_url,
            ctx.envelope_tx.clone(),
            ctx.entity_registry.clone(),
            &iroh_node_id,
            started_at,
            runtime_config,
            req.init_payload.clone(),
        )
        .await
        {
            Ok((ep, Lifecycle::Running)) => {
                let mut running_node = entity_node.clone();
                running_node.initialised = true;
                if let Ok(Some(cid)) = ep.trigger_save(&ctx.kubo_rpc_url).await {
                    running_node.state = Some(IpldLink::new(cid));
                }
                ctx.entity_registry
                    .write()
                    .await
                    .insert(req.fragment.clone(), Arc::new(ep));
                debug!(fragment = %req.fragment, kind = %req.kind_protocol,
                    parent = %req.parent, "entity created via ma_create_entity");
                // Persist the updated manifest in the background so the main
                // event loop is not blocked by the IPFS dag_put.  The entity is
                // already live in the in-memory registry above.  The manifest
                // writer serialises this against all other manifest mutations,
                // so concurrent creates can no longer clobber each other.
                let kubo_url = ctx.kubo_rpc_url.to_string();
                let fragment = req.fragment.clone();
                let writer = ctx.manifest_writer.clone();
                tokio::spawn(async move {
                    if let Err(e) =
                        persist_new_entity(&writer, &kubo_url, &fragment, &running_node).await
                    {
                        warn!(fragment = %fragment, error = %e, "failed to persist new entity to manifest");
                    }
                });
            }
            Ok((_, Lifecycle::Error)) => {
                // New entity — init() failed: send error back to parent plugin,
                // do NOT persist to manifest.
                warn!(fragment = %req.fragment, kind = %req.kind_protocol,
                    "ma_create_entity: init() returned :error; entity discarded");
                // Queue error reply into parent's outbox via envelope_tx.
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
                    &ctx.envelope_tx,
                    &req.parent,
                    crate::entity::SendEnvelope {
                        to: format!("{}#{}", ctx.our_did, req.parent),
                        content_type: ma_core::CONTENT_TYPE_TERM.to_string(),
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

    // Process entity deletion requests queued by `ma_delete_entity` host function.
    for target_fragment in result.delete_requests {
        let reg_read = ctx.entity_registry.read().await;
        let target = if let Some(e) = reg_read.get(&target_fragment) {
            e.clone()
        } else {
            warn!(caller = %entity.fragment, target = %target_fragment,
                "ma_delete_entity: target not found; skipped");
            continue;
        };

        // Authorisation: caller must be the target's parent (or self-delete).
        let authorised = target.parent.as_deref() == Some(entity.fragment.as_str())
            || target_fragment == entity.fragment;
        if !authorised {
            warn!(caller = %entity.fragment, target = %target_fragment,
                "ma_delete_entity: caller is not parent; denied");
            continue;
        }

        // Refuse to delete if any entity in the registry has this as its parent.
        let has_children = reg_read
            .values()
            .any(|e| e.parent.as_deref() == Some(target_fragment.as_str()));
        if has_children {
            warn!(caller = %entity.fragment, target = %target_fragment,
                "ma_delete_entity: target has children; denied");
            continue;
        }
        drop(reg_read);

        ctx.entity_registry.write().await.remove(&target_fragment);
        debug!(caller = %entity.fragment, target = %target_fragment, "entity deleted via ma_delete_entity");
    }

    for req in result.behaviour_requests {
        apply_behaviour_request(req, ctx).await?;
    }

    Ok((entity.is_native() && !result.output.is_empty()).then_some(result.output))
}

// ── Generic reply helper ───────────────────────────────────────────────────────

fn send_rpc_reply(incoming: &ma_core::Message, ctx: &RpcHandlerCtx, content: &[u8]) -> Result<()> {
    send_rpc_reply_typed(incoming, ctx, CONTENT_TYPE_TERM, content)
}

fn send_rpc_reply_typed(
    incoming: &ma_core::Message,
    ctx: &RpcHandlerCtx,
    content_type: &str,
    content: &[u8],
) -> Result<()> {
    let sender = Did::try_from(incoming.from.as_str())
        .with_context(|| format!("invalid sender DID: {}", incoming.from))?;

    let mut reply = ma_core::Message::new(
        ctx.our_did.as_ref(),
        &incoming.from,
        MESSAGE_TYPE_RPC_REPLY,
        content_type,
        content,
        &ctx.signing_key,
    )
    .context("failed to build RPC reply")?;
    reply.reply_to = Some(incoming.id.clone());

    // Spawn the delivery so the event loop is never blocked by DID resolution
    // or QUIC connection setup. The reply is fire-and-forget from the handler's
    // perspective; failures are logged but do not affect the caller.
    let endpoint = Arc::clone(&ctx.endpoint);
    let resolver = Arc::clone(&ctx.resolver);
    let doc_cache = ctx.doc_cache.as_ref().map(Arc::clone);
    let did_resolve = ctx.did_resolve;
    let from = incoming.from.clone();
    let msg_id = incoming.id.clone();
    tokio::spawn(async move {
        let outbox_result = if let Some(doc_cache) = doc_cache {
            crate::ipfs::open_outbox_for_did(
                &endpoint,
                &resolver,
                &doc_cache,
                &sender,
                RPC_PROTOCOL_ID,
                did_resolve,
            )
            .await
        } else {
            endpoint
                .outbox(resolver.as_ref(), &sender.base_id(), RPC_PROTOCOL_ID)
                .await
                .map_err(anyhow::Error::from)
        };

        match outbox_result {
            Ok(mut outbox) => {
                if let Err(err) = outbox.send(&reply).await {
                    warn!(error = %err, to = %from, "RPC reply send failed");
                } else {
                    debug!(to = %from, reply_to = %msg_id, "{}", crate::i18n::t("rpc-reply-sent"));
                }
            }
            Err(err) => {
                debug!(error = %err, to = %from, "RPC reply delivery failed");
            }
        }
    });
    Ok(())
}

/// Resolve the caller's preferred language from their DID document.
/// Falls back to the runtime's own language on any error.
async fn rpc_caller_lang(from: &str, ctx: &RpcHandlerCtx) -> String {
    if let Ok(doc) = ctx.resolver.resolve(from).await {
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

/// Send an RPC error reply with the message localised to the caller's language.
async fn send_rpc_i18n_error(
    incoming: &ma_core::Message,
    ctx: &RpcHandlerCtx,
    key: &str,
) -> Result<()> {
    let lang = rpc_caller_lang(&incoming.from, ctx).await;
    send_rpc_error_reply(incoming, ctx, &crate::i18n::t_lang(&lang, key))
}

fn send_rpc_error_reply(
    incoming: &ma_core::Message,
    ctx: &RpcHandlerCtx,
    reason: &str,
) -> Result<()> {
    let mut payload = Vec::new();
    ciborium::ser::into_writer(
        &CborValue::Array(vec![
            CborValue::Text(":error".to_string()),
            CborValue::Text(reason.to_string()),
        ]),
        &mut payload,
    )
    .context("failed to encode RPC error reply")?;
    send_rpc_reply(incoming, ctx, &payload)
}

#[cfg(test)]
mod tests {
    use ciborium::Value as CborValue;
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    use crate::acl::{new_acl_cache, new_group_cache, AclMap, CapabilityEntry};
    use crate::entity::{new_kind_registry, EntityNode, Evaluator, KindNode, RuntimeManifest};
    use crate::manifest::ManifestWriter;
    use crate::plugin::new_entity_registry;
    use crate::schedule::SchedulerCtx;
    use crate::scheduler_actor;
    use crate::status::Stats;
    use crate::testkubo::MockKubo;

    use ma_core::{MESSAGE_TYPE_RPC, MESSAGE_TYPE_RPC_REPLY};

    use super::{
        handle_entity_plugin_message, rpc_message_kind, rpc_verb, RpcMessageKind, RPC_PROTOCOL_ID,
    };

    #[test]
    fn rpc_reply_is_classified_separately_from_requests() {
        assert_eq!(rpc_message_kind(MESSAGE_TYPE_RPC), RpcMessageKind::Request);
        assert_eq!(
            rpc_message_kind(MESSAGE_TYPE_RPC_REPLY),
            RpcMessageKind::Reply
        );
        assert_eq!(rpc_message_kind("text/plain"), RpcMessageKind::Unsupported);
    }

    #[test]
    fn rpc_verb_extracts_text_atom_and_array_head() {
        assert_eq!(
            rpc_verb(&CborValue::Text(":publish".to_string())),
            Some(":publish")
        );
        assert_eq!(
            rpc_verb(&CborValue::Array(vec![
                CborValue::Text(":publish".to_string()),
                CborValue::Text("now".to_string()),
            ])),
            Some(":publish")
        );
        assert_eq!(rpc_verb(&CborValue::Array(vec![])), None);
    }

    #[allow(clippy::too_many_lines)]
    #[tokio::test(flavor = "multi_thread")]
    async fn scheduler_help_native_output_is_exposed_to_reply_layer() {
        let kubo = MockKubo::start().await;
        let initial_root = crate::kubo::dag_put(kubo.url(), &RuntimeManifest::default())
            .await
            .unwrap();
        let stats = Arc::new(RwLock::new(Stats {
            root_cid: Some(initial_root.clone()),
            ..Default::default()
        }));
        let manifest_writer = ManifestWriter::new(
            initial_root,
            kubo.url().to_string(),
            stats.clone(),
            None,
            None,
        );

        let mut scheduler_acl = AclMap::new();
        scheduler_acl.insert("*".to_string(), CapabilityEntry::from_caps([":help"]));

        let kind_registry = new_kind_registry();
        let entity_registry = new_entity_registry();
        let (envelope_tx, _envelope_rx) =
            tokio::sync::mpsc::channel::<(String, crate::entity::SendEnvelope)>(16);

        let runtime_did = ma_core::Did::new_url("k51qzi5uqu5runtime", None::<String>).unwrap();
        let runtime_signing = ma_core::SigningKey::generate(
            ma_core::Did::new_url("k51qzi5uqu5runtime", Some("sign")).unwrap(),
        )
        .unwrap();
        let sender_did = ma_core::Did::new_url("k51qzi5uqu5sender", None::<String>).unwrap();
        let sender_signing = ma_core::SigningKey::generate(
            ma_core::Did::new_url("k51qzi5uqu5sender", Some("sign")).unwrap(),
        )
        .unwrap();

        let mut runtime_endpoint_box = ma_core::new_ma_endpoint([11u8; 32], false).await.unwrap();
        let _runtime_rpc_inbox = runtime_endpoint_box.service(RPC_PROTOCOL_ID);
        let runtime_endpoint: Arc<dyn ma_core::MaEndpoint> = Arc::from(runtime_endpoint_box);

        let scheduler_kind = KindNode {
            protocol: scheduler_actor::SCHEDULER_KIND.to_string(),
            cid: None,
            kind_type: Evaluator::Native,
            behaviour: None,
            behaviour_chain: Vec::new(),
            host_functions: vec![],
            attributes: BTreeMap::new(),
            extends: None,
        };
        kind_registry.write().await.insert(
            scheduler_kind.protocol.clone(),
            Arc::new(scheduler_kind.clone()),
        );

        let scheduler_node = EntityNode {
            kind: scheduler_actor::SCHEDULER_KIND.to_string(),
            behaviour: None,
            acl: "scheduler".to_string(),
            state: None,
            parent: None,
            label: None,
            attributes: BTreeMap::new(),
            init: None,
            initialised: true,
            reload_error: None,
        };
        let job_scheduler = Arc::new(tokio_cron_scheduler::JobScheduler::new().await.unwrap());
        let native_actor = scheduler_actor::native_actor(
            job_scheduler,
            SchedulerCtx {
                entity_registry: entity_registry.clone(),
                manifest_writer: Arc::new(tokio::sync::RwLock::new(None)),
                kubo_rpc_url: kubo.url().to_string(),
                our_did: runtime_did.base_id(),
            },
        );
        let (scheduler_plugin, _) = crate::plugin::EntityPlugin::new_native(
            "scheduler",
            &scheduler_node,
            &scheduler_kind,
            native_actor,
            Vec::new(),
            None,
        )
        .unwrap();
        entity_registry
            .write()
            .await
            .insert("scheduler".to_string(), Arc::new(scheduler_plugin));

        let acl_cache = new_acl_cache();
        acl_cache
            .write()
            .await
            .insert("acls.scheduler".to_string(), scheduler_acl);

        let ctx = super::RpcHandlerCtx {
            our_did: Arc::from(runtime_did.base_id()),
            signing_key: Arc::new(runtime_signing),
            endpoint: runtime_endpoint,
            kubo_rpc_url: Arc::from(kubo.url().to_string()),
            resolver: Arc::new(ma_core::IpfsGatewayResolver::new("http://127.0.0.1:9")),
            doc_cache: None,
            did_resolve: crate::ipfs::DidResolveSettings::default(),
            entity_registry: entity_registry.clone(),
            kind_registry,
            envelope_tx,
            stats,
            acl_cache,
            group_cache: new_group_cache(),
            manifest_writer,
            shared_config: Arc::new(RwLock::new(ma_core::Config {
                slug: "ma".to_string(),
                log_level: "info".to_string(),
                log_level_stdout: "info".to_string(),
                did_resolver_positive_ttl_secs: 0,
                did_resolver_negative_ttl_secs: 0,
                log_file: None,
                kubo_rpc_url: kubo.url().to_string(),
                kubo_key_alias: "ma".to_string(),
                pin_remote: false,
                pin_remote_service: None,
                pin_remote_name: None,
                secret_bundle: None,
                secret_bundle_passphrase: None,
                config_path: None,
                extra: serde_yaml::Mapping::new(),
            })),
            runtime_slug: Arc::from("ma"),
            runtime_ipns_key: [7u8; 32],
            ipns_publish: crate::ipfs::IpnsPublishSettings::default(),
            did_publish_timeout_secs: 120,
        };

        let mut payload = Vec::new();
        ciborium::ser::into_writer(&ciborium::Value::Text(":help".to_string()), &mut payload)
            .unwrap();
        let incoming = ma_core::Message::new(
            sender_did.base_id(),
            format!("{}#scheduler", runtime_did.base_id()),
            MESSAGE_TYPE_RPC,
            ma_core::CONTENT_TYPE_TERM,
            &payload,
            &sender_signing,
        )
        .unwrap();

        let reply = handle_entity_plugin_message(
            &incoming,
            ciborium::Value::Text(":help".to_string()),
            entity_registry
                .read()
                .await
                .get("scheduler")
                .unwrap()
                .clone(),
            &ctx,
        )
        .await
        .unwrap();

        let payload = reply.expect("native scheduler help should produce a reply payload");
        let term: ciborium::Value = ciborium::de::from_reader(payload.as_slice()).unwrap();
        match term {
            ciborium::Value::Text(text) => {
                assert!(text.contains("scheduler help"));
                assert!(text.contains(":cron"));
            }
            other => panic!("expected text help reply, got {other:?}"),
        }
    }
}
