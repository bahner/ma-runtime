use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use ciborium::Value as CborValue;

use crate::entity::{IpldLink, KindNode, KindTree};

use super::helpers::{
    load_manifest, runtime_config_snapshot, send_crud_error, send_crud_i18n_error, send_crud_ok,
    send_crud_ok_cid, send_crud_ok_yaml, send_crud_reply_cbor, spawn_kind_dependency_reloads,
    spawn_kind_dependency_reloads_for, with_manifest_crud, with_manifest_crud_async,
};
use super::CrudHandlerCtx;

// ── Kinds handler ────────────────────────────────────────────────────────────

/// Handle `/kinds` CRUD operations.
///
/// | Operation | Path | Body |
/// |-----------|------|------|
/// | List all  | `GET /kinds` | — |
/// | Apply tree | `SET /kinds` | `/ipfs/<cid>` |
/// | Get kind  | `GET /kinds/ma/avatar/0.0.1` | — |
/// | Upsert    | `SET /kinds/ma/avatar/0.0.1` | `/ipfs/<cid>` |
/// | Delete    | `DEL /kinds/ma/avatar/0.0.1` | — |
pub(super) async fn handle_kinds_ns(
    message: &ma_core::Message,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    if rest.is_empty() {
        // Operations on /kinds root
        return match (tail, args.as_slice()) {
            // GET /kinds → list all protocol IDs
            (None, []) => {
                let manifest = load_manifest(ctx).await?;
                let ids: Vec<String> = manifest.kinds.protocol_ids();
                send_crud_reply_cbor(message, reply_type, ctx, &ids).await
            }
            // DELETE /kinds → refuse
            (Some(""), []) => {
                send_crud_i18n_error(message, reply_type, ctx, "refuse-delete-root").await
            }
            // SET /kinds <cid> → apply a KindTree overlay; absent protocols are preserved.
            (Some(""), [CborValue::Text(raw)]) => {
                apply_kinds_tree(message, reply_type, ctx, raw).await
            }
            _ => Err(anyhow!("unknown kinds root operation")),
        };
    }

    // /kinds/ma/avatar/0.0.1 → protocol_id = "/ma/avatar/0.0.1"
    let protocol_id = format!("/{}", rest.join("/"));

    match (tail, args.as_slice()) {
        // GET /kinds/<protocol> → fetch and return KindNode as YAML
        (None, []) => {
            let manifest = load_manifest(ctx).await?;
            match manifest.kinds.get_protocol(&protocol_id) {
                Some(link) => {
                    let kind: KindNode = crate::kubo::dag_get(&ctx.kubo_rpc_url, &link.cid).await?;
                    send_crud_ok_yaml(message, reply_type, ctx, &kind).await
                }
                None => send_crud_error(message, reply_type, ctx, "kind-not-found").await,
            }
        }
        // SET /kinds/<protocol> <cid> → upsert kind
        (Some(""), [CborValue::Text(raw)]) => {
            let cid = crate::kubo::dag_resolve(&ctx.kubo_rpc_url, raw).await?;
            let raw_kind: KindNode = crate::kubo::dag_get(&ctx.kubo_rpc_url, &cid)
                .await
                .with_context(|| format!("fetching kind node for '{protocol_id}'"))?;
            let manifest = load_manifest(ctx).await?;
            let kind_node = if raw_kind.extends.is_some() {
                crate::entity::resolve_kind_extends(&ctx.kubo_rpc_url, &manifest, raw_kind).await?
            } else {
                raw_kind
            };
            with_manifest_crud(ctx, |m| {
                m.kinds.insert_protocol(&protocol_id, IpldLink::new(&cid));
                Ok(())
            })
            .await?;
            ctx.kind_registry
                .write()
                .await
                .insert(protocol_id.clone(), Arc::new(kind_node));
            match runtime_config_snapshot(ctx).await {
                Ok(runtime_config) => {
                    match spawn_kind_dependency_reloads(&protocol_id, ctx, runtime_config).await {
                        Ok(reload_count) => {
                            tracing::info!(protocol = %protocol_id, reload_count, "kind dependents scheduled for reload");
                        }
                        Err(e) => {
                            tracing::warn!(protocol = %protocol_id, error = %e, "failed to schedule kind dependent reloads");
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!(protocol = %protocol_id, error = %e, "failed to build runtime config for kind dependent reloads");
                }
            }
            send_crud_ok_cid(message, reply_type, ctx, &cid).await
        }
        // DELETE /kinds/<protocol> → remove kind
        (Some(""), []) => {
            let manifest = load_manifest(ctx).await?;
            if manifest.kinds.get_protocol(&protocol_id).is_none() {
                return send_crud_error(message, reply_type, ctx, "kind-not-found").await;
            }
            with_manifest_crud(ctx, |m| {
                m.kinds.remove_protocol(&protocol_id);
                Ok(())
            })
            .await?;
            ctx.kind_registry.write().await.remove(&protocol_id);
            send_crud_ok(message, reply_type, ctx).await
        }
        _ => Err(anyhow!("unknown kinds operation")),
    }
}

async fn apply_kinds_tree(
    message: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
    raw: &str,
) -> Result<()> {
    tracing::info!(from = %message.from, reference = %raw, "kinds tree overlay requested");
    let cid = crate::kubo::dag_resolve(&ctx.kubo_rpc_url, raw).await?;
    tracing::info!(from = %message.from, cid = %cid, "kinds tree overlay resolved");
    let overlay: KindTree = crate::kubo::dag_get(&ctx.kubo_rpc_url, &cid)
        .await
        .with_context(|| format!("fetching kinds tree '{cid}'"))?;
    tracing::info!(from = %message.from, cid = %cid, "kinds tree overlay loaded");
    let kubo_rpc_url = ctx.kubo_rpc_url.clone();
    tracing::info!(from = %message.from, cid = %cid, "applying kinds tree overlay to manifest");
    let (_, changed_protocols) = with_manifest_crud_async(ctx, |manifest| {
        Box::pin(async move {
            crate::bootstrap::apply_kinds_tree_overlay(manifest, &overlay, &kubo_rpc_url).await
        })
    })
    .await?;
    tracing::info!(from = %message.from, cid = %cid, changed = changed_protocols.len(), "kinds tree overlay applied");
    schedule_overlay_reloads(ctx, &changed_protocols).await;
    send_crud_ok_cid(message, reply_type, ctx, &cid).await
}

async fn schedule_overlay_reloads(ctx: &CrudHandlerCtx, changed_protocols: &[String]) {
    if changed_protocols.is_empty() {
        return;
    }
    let runtime_config = match runtime_config_snapshot(ctx).await {
        Ok(runtime_config) => runtime_config,
        Err(e) => {
            tracing::warn!(error = %e, "failed to build runtime config for kind overlay reloads");
            return;
        }
    };
    match spawn_kind_dependency_reloads_for(changed_protocols, ctx, runtime_config).await {
        Ok(reload_count) => {
            tracing::info!(
                changed = changed_protocols.len(),
                reload_count,
                "kind overlay dependents scheduled for reload"
            );
        }
        Err(e) => {
            tracing::warn!(changed = changed_protocols.len(), error = %e, "failed to schedule kind overlay dependent reloads");
        }
    }
}
