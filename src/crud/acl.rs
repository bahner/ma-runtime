use anyhow::{anyhow, Context, Result};
use ciborium::Value as CborValue;
use tracing::info;

use crate::entity::IpldLink;

use super::helpers::{
    load_manifest, resolve_ipfs_ref, send_crud_error, send_crud_i18n_error, send_crud_ok_cid,
    send_crud_reply_cbor, with_manifest_crud,
};
use super::CrudHandlerCtx;

/// Handle `:acls` — the root-level named ACL library stored in `manifest.acls`.
///
/// - `:acls`              → list all named ACL names
/// - `:acls.<name>`       → get CID for named ACL
/// - `:acls.<name>: <cid>`→ set named ACL by CID
/// - `:acls.<name>:`      → delete named ACL
pub(super) async fn handle_root_acls(
    message: &ma_core::Message,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    match (rest, tail, args.as_slice()) {
        // List all named ACLs.
        ([], None, []) => {
            let manifest = load_manifest(ctx).await?;
            let names: Vec<CborValue> = manifest
                .acls
                .keys()
                .map(|k| CborValue::Text(k.clone()))
                .collect();
            send_crud_reply_cbor(
                message,
                reply_type,
                ctx,
                &CborValue::Array(vec![
                    CborValue::Text(":ok".to_string()),
                    CborValue::Array(names),
                ]),
            )
            .await
        }
        // Get a named ACL's CID.
        ([acl_name], None, []) => {
            let manifest = load_manifest(ctx).await?;
            let Some(link) = manifest.acls.get(acl_name.as_str()) else {
                return send_crud_error(message, reply_type, ctx, "acl-not-found").await;
            };
            let ipfs_path = format!("/ipfs/{}", link.cid);
            send_crud_reply_cbor(message, reply_type, ctx, &CborValue::Text(ipfs_path)).await
        }
        // Set a named ACL by CID.
        ([acl_name], Some(""), [CborValue::Text(raw)]) => {
            let Some(cid) = resolve_ipfs_ref(&ctx.kubo_rpc_url, raw).await? else {
                return send_crud_i18n_error(message, reply_type, ctx, "cidv1-required").await;
            };
            // Validate ACL document shape before accepting it as a named ACL.
            // This prevents silent cache misses later during entity dispatch.
            let acl_map = crate::acl::load_acl_from_cid(&ctx.kubo_rpc_url, &cid)
                .await
                .with_context(|| format!("loading named ACL '{acl_name}' from /ipfs/{cid}"))?;
            let acl_name = acl_name.clone();
            with_manifest_crud(ctx, |m| {
                m.acls.insert(acl_name.clone(), IpldLink::new(&cid));
                Ok(())
            })
            .await?;
            let cache_key = format!("acls.{acl_name}");
            ctx.acl_cache.write().await.insert(cache_key, acl_map);
            send_crud_ok_cid(message, reply_type, ctx, &cid).await
        }
        // Delete a named ACL.
        ([acl_name], Some(""), []) => {
            let acl_name = acl_name.clone();
            let manifest = load_manifest(ctx).await?;
            if !manifest.acls.contains_key(&acl_name) {
                return send_crud_error(message, reply_type, ctx, "acl-not-found").await;
            }
            let new_root = with_manifest_crud(ctx, |m| {
                m.acls.remove(&acl_name);
                Ok(())
            })
            .await?;
            let cache_key = format!("acls.{acl_name}");
            ctx.acl_cache.write().await.remove(&cache_key);
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        _ => Err(anyhow!("unknown :acls operation")),
    }
}

pub(super) async fn handle_root_acl(
    message: &ma_core::Message,
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    match (tail, args.as_slice()) {
        (None, []) => {
            let manifest = load_manifest(ctx).await?;
            // No `/ipfs/` prefix when no root ACL has been set yet — there
            // is genuinely no reference to point at.
            let value = manifest
                .acl
                .as_ref()
                .map(|link| format!("/ipfs/{}", link.cid))
                .unwrap_or_default();
            send_crud_reply_cbor(message, reply_type, ctx, &CborValue::Text(value)).await
        }
        (Some(""), [CborValue::Text(raw)]) => {
            let Some(cid) = resolve_ipfs_ref(&ctx.kubo_rpc_url, raw).await? else {
                return send_crud_i18n_error(message, reply_type, ctx, "cidv1-required").await;
            };
            let acl_map = crate::acl::load_acl_from_cid(&ctx.kubo_rpc_url, &cid)
                .await
                .with_context(|| format!("loading root ACL from {cid}"))?;
            with_manifest_crud(ctx, |m| {
                m.acl = Some(IpldLink::new(&cid));
                Ok(())
            })
            .await?;
            *ctx.root_acl.write().await = acl_map;
            info!(from = %message.from, cid = %cid, "{}", crate::i18n::t("crud-acl-updated"));
            send_crud_ok_cid(message, reply_type, ctx, &cid).await
        }
        // DELETE is refused — the root transport ACL must never be cleared.
        (Some(""), []) => {
            send_crud_i18n_error(message, reply_type, ctx, "refuse-delete-root").await
        }
        _ => Err(anyhow!("unknown :acl operation")),
    }
}
