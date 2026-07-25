//! `/ma/crud/0.0.1` — structured data management service.
//!
//! Single message type `application/vnd.ma.crud.request`. The operation is encoded in
//! a 1- or 2-element CBOR array payload:
//!
//!   GET:    `["/ns/key"]`
//!   SET:    `["/ns/key", value]`   — value is a scalar or a `/ipfs/<cid>` /
//!                                    `/ipns/<key>` reference
//!   DELETE: `["/ns/key", ""]`      — empty string value means delete
//!
//! All replies use `application/vnd.ma.crud.reply`.

mod acl;
pub mod config;
mod entities;
mod grp;
mod helpers;
mod kinds;

use std::sync::Arc;

use anyhow::Result;
use ciborium::Value as CborValue;
use ma_core::{IpfsGatewayResolver, SigningKey, MESSAGE_TYPE_CRUD, MESSAGE_TYPE_CRUD_REPLY};
use tokio::sync::RwLock;

use crate::acl::{check_full, AclCache, AclMap, GroupCache, SharedAcl, CAP_CRUD};
use crate::entity::{KindRegistry, SendEnvelope};
use crate::plugin::EntityRegistry;
use crate::status::SharedStats;

pub use ma_core::CRUD_PROTOCOL_ID;

// ── Handler context ────────────────────────────────────────────────────────────

pub struct CrudHandlerCtx {
    pub our_did: Arc<str>,
    pub signing_key: Arc<SigningKey>,
    pub endpoint: Arc<dyn ma_core::MaEndpoint>,
    pub kubo_rpc_url: Arc<str>,
    pub resolver: Arc<IpfsGatewayResolver>,
    pub stats: SharedStats,
    pub entity_registry: EntityRegistry,
    pub kind_registry: KindRegistry,
    pub shared_config: Arc<RwLock<ma_core::Config>>,
    /// Named ACL cache — maps `"acls.<name>"` to its `AclMap` for
    /// zero-overhead lookup at call time.
    pub acl_cache: AclCache,
    /// Named group cache — maps a group name to its flat DID-member list,
    /// backing the `+<name>` principal syntax in any `AclMap`.
    pub group_cache: GroupCache,
    /// Shared root transport ACL — owner may update at runtime via `:acl: <cid>`.
    pub root_acl: SharedAcl,
    /// Forwarding channel for envelopes produced by entity plugins via `ma_send`.
    pub envelope_tx: tokio::sync::mpsc::UnboundedSender<(String, SendEnvelope)>,
    /// Serialised manifest writer — all manifest mutations go through it.
    pub manifest_writer: crate::manifest::ManifestWriter,
}

// ── Entry point ────────────────────────────────────────────────────────────────

pub async fn handle_crud_message(
    message: &ma_core::Message,
    acl: &AclMap,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    // ACL: require "crud" capability. Owners bypass this gate unconditionally
    // so they can never be locked out of ACL management.
    let owners = ctx.stats.read().await.owners.clone();
    if !crate::acl::is_owner(&owners, &message.from) {
        check_full(acl, &message.from, &[CAP_CRUD], |key| {
            let name = key.strip_prefix('+').unwrap_or(key).to_string();
            async move {
                Ok(ctx
                    .group_cache
                    .read()
                    .await
                    .get(&name)
                    .cloned()
                    .unwrap_or_default())
            }
        })
        .await?;
    }
    dispatch_management(message, ctx).await
}

async fn dispatch_management(message: &ma_core::Message, ctx: &CrudHandlerCtx) -> Result<()> {
    if message.message_type != MESSAGE_TYPE_CRUD {
        return helpers::send_crud_i18n_errorf(
            message,
            MESSAGE_TYPE_CRUD_REPLY,
            ctx,
            "wrong-crud-protocol",
            &[("type", &message.message_type)],
        )
        .await;
    }

    let payload = message.payload();
    if payload.is_empty() {
        let reason = "empty CRUD payload";
        tracing::error!(
            from = %message.from,
            to = %message.to,
            id = %message.id,
            message_type = %message.message_type,
            "CRUD message rejected: empty payload"
        );
        return helpers::send_crud_error(message, MESSAGE_TYPE_CRUD_REPLY, ctx, reason).await;
    }

    let path_owned: String;
    let tail_owned: Option<String>;
    let args: Vec<CborValue>;

    match helpers::decode_crud_payload(&payload)? {
        helpers::CrudOp::Get(path) => {
            path_owned = path;
            tail_owned = None;
            args = vec![];
        }
        helpers::CrudOp::Set(path, value) => {
            path_owned = path;
            tail_owned = Some(String::new());
            args = vec![value];
        }
        helpers::CrudOp::Delete(path) => {
            path_owned = path;
            tail_owned = Some(String::new());
            args = vec![];
        }
    }

    let reply_type = MESSAGE_TYPE_CRUD_REPLY;
    let tail: Option<&str> = tail_owned.as_deref();
    let (ns, rest) = helpers::parse_path(&path_owned)?;

    match ns {
        "entities" => {
            entities::handle_entities_ns(message, &rest, tail, args, reply_type, ctx).await
        }
        "kinds" => kinds::handle_kinds_ns(message, &rest, tail, args, reply_type, ctx).await,
        "config" => config::handle_config_ns(message, &rest, tail, args, reply_type, ctx).await,
        "acl" => acl::handle_root_acl(message, tail, args, reply_type, ctx).await,
        "acls" => acl::handle_root_acls(message, &rest, tail, args, reply_type, ctx).await,
        "grp" => grp::handle_root_grp(message, &rest, tail, args, reply_type, ctx).await,
        // Unknown first segment: treat the full path as a config key path.
        // e.g. :owners → config["owners"], :foo.owners → config["foo"]["owners"]
        _ => {
            let mut full_rest = vec![ns.to_string()];
            full_rest.extend(rest);
            config::handle_config_ns(message, &full_rest, tail, args, reply_type, ctx).await
        }
    }
}
