use anyhow::{anyhow, Result};
use ciborium::Value as CborValue;

use crate::entity::IpldLink;

use super::helpers::{
    group_cache_update, load_manifest, resolve_ipfs_ref, send_crud_error, send_crud_i18n_error,
    send_crud_ok_cid, send_crud_reply_cbor, with_manifest_crud,
};
use super::CrudHandlerCtx;

/// The one group name that may never be deleted (may still be set to an
/// empty list) — see `RuntimeManifest.grp`'s doc comment.
const OWNERS_GROUP: &str = "owners";

/// Handle `/grp` — the flat named-group registry stored in `manifest.grp`.
///
/// Each entry is an IPLD link to a plain `Vec<String>` of member DIDs,
/// referenced from any `AclMap` as principal `+<name>`.
///
/// - `/grp`              → list all group names
/// - `/grp/<name>`        → get CID for a group
/// - `/grp/<name>: <cid>` → set a group by CID
/// - `/grp/<name>:`       → delete a group (refused for `"owners"`)
pub(super) async fn handle_root_grp(
    message: &ma_core::Message,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    match (rest, tail, args.as_slice()) {
        // List all group names.
        ([], None, []) => {
            let manifest = load_manifest(ctx).await?;
            let names: Vec<CborValue> = manifest
                .grp
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
        // Get a group's CID.
        ([name], None, []) => {
            let manifest = load_manifest(ctx).await?;
            let Some(link) = manifest.grp.get(name.as_str()) else {
                return send_crud_error(message, reply_type, ctx, "group-not-found").await;
            };
            let ipfs_path = format!("/ipfs/{}", link.cid);
            send_crud_reply_cbor(message, reply_type, ctx, &CborValue::Text(ipfs_path)).await
        }
        // Set a group by CID. Unrestricted — including "owners", and
        // including setting it to a CID for an empty list.
        ([name], Some(""), [CborValue::Text(raw)]) => {
            let Some(cid) = resolve_ipfs_ref(&ctx.kubo_rpc_url, raw).await? else {
                return send_crud_i18n_error(message, reply_type, ctx, "cidv1-required").await;
            };
            let name = name.clone();
            with_manifest_crud(ctx, |m| {
                m.grp.insert(name.clone(), IpldLink::new(&cid));
                Ok(())
            })
            .await?;
            group_cache_update(ctx, &name, &cid).await;
            // Keep the fast-path owner bypass (`is_owner`/`stats.owners`) in
            // sync immediately, same as the cache above.
            if name == OWNERS_GROUP {
                let members = ctx
                    .group_cache
                    .read()
                    .await
                    .get(OWNERS_GROUP)
                    .cloned()
                    .unwrap_or_default();
                crate::status::grant_owners_in_acl(&ctx.root_acl, &members).await;
                ctx.stats.write().await.owners = members;
            }
            send_crud_ok_cid(message, reply_type, ctx, &cid).await
        }
        // Delete a group — refused for "owners" (the entry must always
        // exist; it may only ever be emptied via SET, never removed).
        ([name], Some(""), []) => {
            if name == OWNERS_GROUP {
                return send_crud_i18n_error(message, reply_type, ctx, "refuse-delete-root").await;
            }
            let name = name.clone();
            let manifest = load_manifest(ctx).await?;
            if !manifest.grp.contains_key(&name) {
                return send_crud_error(message, reply_type, ctx, "group-not-found").await;
            }
            let new_root = with_manifest_crud(ctx, |m| {
                m.grp.remove(&name);
                Ok(())
            })
            .await?;
            ctx.group_cache.write().await.remove(&name);
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        _ => Err(anyhow!("unknown /grp operation")),
    }
}
