//! Handler for the `/ma/inbox/0.0.1` service.
//!
//! Inbox messages are fire-and-forget: fragment-addressed messages are routed
//! to the target entity's `handle_cast` (stateless) or `handle_call` (stateful,
//! to allow state persistence), and the result is discarded.  No reply is sent.
//! Unfragmented (broadcast / runtime-level) messages are logged and dropped.

use std::sync::Arc;

use anyhow::Result;
use ma_core::ipfs_add;
use tracing::{error, info, warn};

use crate::entity::{CastInput, LocalMessage, PluginMsg};
use crate::plugin::EntityRegistry;
use crate::routing::local_target_fragment;

pub struct InboxHandlerCtx {
    pub our_did: Arc<str>,
    pub entity_registry: EntityRegistry,
    pub kubo_rpc_url: Arc<str>,
    pub manifest_writer: crate::manifest::ManifestWriter,
}

pub async fn handle_inbox_message(message: &ma_core::Message, ctx: &InboxHandlerCtx) -> Result<()> {
    if message.payload().is_empty() {
        error!(
            from = %message.from,
            to = %message.to,
            id = %message.id,
            message_type = %message.message_type,
            "inbox: empty payload dropped"
        );
        return Ok(());
    }

    let fragment = local_target_fragment(&message.to, &ctx.our_did);

    let Some(fragment) = fragment.as_deref() else {
        info!(
            from = %message.from,
            to = %message.to,
            message_type = %message.message_type,
            "inbox: unfragmented message dropped"
        );
        return Ok(());
    };

    let entity = ctx.entity_registry.read().await.get(fragment).cloned();

    let Some(entity) = entity else {
        warn!(fragment = %fragment, "inbox: unknown entity fragment; dropped");
        return Ok(());
    };

    info!(
        fragment = %fragment,
        from = %message.from,
        message_type = %message.message_type,
        "inbox: dispatching to entity"
    );

    let local_msg = LocalMessage {
        id: message.id.clone(),
        from: message.from.clone(),
        to: message.to.clone(),
        created_at: message.created_at,
        exp: message.exp,
        reply_to: message.reply_to.clone(),
        message_type: message.message_type.clone(),
        content_type: message.content_type.clone(),
        content: message.content.clone(),
    };
    let cast_input = CastInput {
        msg: PluginMsg::from(&local_msg),
    };

    let result = entity.on_message(&cast_input).await?;

    // Persist state if the entity called ma_set_state during this dispatch.
    if let Some(state_bytes) = result.pending_state {
        let kubo_url = Arc::clone(&ctx.kubo_rpc_url);
        let fragment_str = entity.fragment.clone();
        let entity_arc = Arc::clone(&entity);
        let writer = ctx.manifest_writer.clone();
        tokio::spawn(async move {
            match ipfs_add(&kubo_url, state_bytes.clone()).await {
                Ok(cid) => match writer.set_entity_state(&fragment_str, &cid).await {
                    Ok(root_cid) => {
                        entity_arc.mark_saved(state_bytes);
                        info!(fragment = %fragment_str, cid = %cid, %root_cid, "inbox: entity state persisted");
                    }
                    Err(e) => {
                        warn!(fragment = %fragment_str, cid = %cid, error = %e, "inbox: failed to update manifest with entity state");
                    }
                },
                Err(e) => {
                    warn!(fragment = %fragment_str, error = %e, "inbox: failed to persist entity state");
                }
            }
        });
    }

    if !result.behaviour_requests.is_empty() {
        warn!(fragment = %entity.fragment, "inbox: ma_set_behaviour requests are ignored on fire-and-forget inbox dispatch");
    }

    Ok(())
}
