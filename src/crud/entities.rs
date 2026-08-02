use anyhow::{anyhow, Context, Result};
use ciborium::Value as CborValue;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::info;

use crate::acl::check_full;
use crate::entity::{EntityNode, IpldLink};

use super::helpers::{
    cidv1_ref, load_manifest, runtime_config_snapshot, send_crud_data_cbor, send_crud_error,
    send_crud_i18n_error, send_crud_i18n_errorf, send_crud_ok, send_crud_ok_cid,
    send_crud_reply_cbor, spawn_entity_reload, with_manifest_crud,
};
use super::CrudHandlerCtx;

async fn reload_shutdown_timeout(ctx: &CrudHandlerCtx) -> std::time::Duration {
    let cfg = ctx.shared_config.read().await;
    super::config::wasm_reload_shutdown_timeout(&cfg)
}

fn single_entity_reload_gate() -> Arc<Semaphore> {
    Arc::new(Semaphore::new(1))
}

// ── Management capability helpers ─────────────────────────────────────────────

async fn check_entity_management_cap(
    message: &ma_core::Message,
    ctx: &CrudHandlerCtx,
    caps: &[&str],
) -> Result<()> {
    // Snapshot and drop the read guard before the async check_full call.
    // Holding the guard across an await would block any concurrent write
    // to root_acl (e.g. :acl: update) until the check completes.
    let acl = ctx.root_acl.read().await.clone();
    check_full(&acl, &message.from, caps, |key| {
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
    .await
    .with_context(|| {
        format!(
            "entity management denied for {}: requires {:?}",
            message.from, caps
        )
    })
}

// ── Entities handler ─────────────────────────────────────────────────────────

pub(super) async fn handle_entities_ns(
    message: &ma_core::Message,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    match rest.len() {
        0 => match (tail, args.as_slice()) {
            (None, []) => {
                info!("{}", crate::i18n::t("root-list-entities"));
                let names: Vec<String> = ctx.entity_registry.read().await.keys().cloned().collect();
                send_crud_data_cbor(message, reply_type, ctx, &names).await
            }
            (Some(""), _) => {
                send_crud_i18n_error(message, reply_type, ctx, "refuse-delete-root").await
            }
            _ => Err(anyhow!("unknown entities operation")),
        },
        1 => handle_single_entity(message, &rest[0], tail, args, reply_type, ctx).await,
        2.. => {
            handle_entity_field(message, &rest[0], &rest[1..], tail, args, reply_type, ctx).await
        }
    }
}

#[allow(clippy::too_many_lines)]
async fn handle_single_entity(
    message: &ma_core::Message,
    name: &String,
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    match (tail, args.as_slice()) {
        (None, []) => {
            let manifest = load_manifest(ctx).await?;
            let Some(link) = manifest.entities.get(name.as_str()) else {
                return send_crud_error(message, reply_type, ctx, "entity-not-found").await;
            };
            let entity: EntityNode = crate::kubo::dag_get(&ctx.kubo_rpc_url, &link.cid).await?;
            send_crud_data_cbor(message, reply_type, ctx, &entity).await
        }
        (Some(""), []) => {
            // Delete entity — requires `delete` + `entities` in root ACL.
            check_entity_management_cap(message, ctx, &["delete", "entities"]).await?;
            let name = name.as_str();
            let manifest = load_manifest(ctx).await?;
            if !manifest.entities.contains_key(name) {
                return send_crud_error(message, reply_type, ctx, "entity-not-found").await;
            }
            let removed = ctx.entity_registry.write().await.remove(name);
            if let Some(entity) = removed {
                entity.terminate_worker();
            }
            let new_root = with_manifest_crud(ctx, |m| {
                m.entities.remove(name);
                Ok(())
            })
            .await?;
            info!(name = %name, cid = %new_root, "{}", crate::i18n::t("entity-deleted"));
            send_crud_ok(message, reply_type, ctx).await
        }
        (Some(""), [CborValue::Text(raw)]) => {
            // Upsert entity — caller needs the entity's `kind` as a capability in root ACL.
            // The kind is read from the EntityNode itself; no separate state required.
            let name = name.as_str();
            if name.chars().any(char::is_control) {
                return send_crud_i18n_error(message, reply_type, ctx, "entity-name-invalid").await;
            }
            if crate::entity::RESERVED_ENTITY_NAMES.contains(&name) {
                return send_crud_i18n_errorf(
                    message,
                    reply_type,
                    ctx,
                    "reserved-entity-name",
                    &[("name", name)],
                )
                .await;
            }
            let Some(cid) = cidv1_ref(raw) else {
                return send_crud_i18n_error(message, reply_type, ctx, "cidv1-required").await;
            };
            let cid = cid.as_str();
            let mut entity_node: EntityNode = crate::kubo::dag_get(&ctx.kubo_rpc_url, cid)
                .await
                .with_context(|| format!("fetching entity node from {cid}"))?;
            // ACL gate: caller must hold the entity's kind protocol ID as a capability.
            check_entity_management_cap(message, ctx, &[entity_node.kind.as_str()]).await?;
            // Genesis rule (hardcoded, cross-cutting — see
            // `entity::is_genesis_entity`'s doc comment): entity-level
            // `attributes.genesis` overrides the kind's own, merged at
            // read time. Either way it's true, only owners may create the
            // instance, and it always gets `parent: None`, regardless of
            // what the caller's published EntityNode requested.
            //
            // Prefer the hydrated in-memory kind registry, with a manifest/IPFS
            // fallback for stale or externally-mutated roots.
            let cached_kind = ctx
                .kind_registry
                .read()
                .await
                .get(entity_node.kind.as_str())
                .cloned();
            let kind_node = if let Some(k) = cached_kind {
                Some(k.as_ref().clone())
            } else {
                let manifest = load_manifest(ctx).await?;
                if let Some(link) = manifest.kinds.get_protocol(entity_node.kind.as_str()) {
                    let raw_kind: crate::entity::KindNode =
                        crate::kubo::dag_get(&ctx.kubo_rpc_url, &link.cid)
                            .await
                            .with_context(|| {
                                format!("fetching kind node for '{}'", entity_node.kind)
                            })?;
                    let resolved = if raw_kind.extends.is_some() {
                        crate::entity::resolve_kind_extends(&ctx.kubo_rpc_url, &manifest, raw_kind)
                            .await?
                    } else {
                        raw_kind
                    };
                    Some(resolved)
                } else {
                    None
                }
            };
            if let Some(kind_node) = &kind_node {
                if crate::entity::is_genesis_entity(kind_node, &entity_node) {
                    let owners = ctx.stats.read().await.owners.clone();
                    if !crate::acl::is_owner(&owners, &message.from) {
                        return send_crud_i18n_error(
                            message,
                            reply_type,
                            ctx,
                            "genesis-kind-owner-only",
                        )
                        .await;
                    }
                    entity_node.parent = None;
                }
            }
            with_manifest_crud(ctx, |m| {
                m.entities.insert(name.to_string(), IpldLink::new(cid));
                Ok(())
            })
            .await?;
            let runtime_config = runtime_config_snapshot(ctx).await?;
            let reload_shutdown_timeout = reload_shutdown_timeout(ctx).await;
            spawn_entity_reload(
                name.to_string(),
                entity_node,
                ctx.kind_registry.clone(),
                ctx.stats.clone(),
                Arc::clone(&ctx.kubo_rpc_url),
                Arc::clone(&ctx.our_did),
                ctx.envelope_tx.clone(),
                ctx.entity_registry.clone(),
                ctx.manifest_writer.clone(),
                runtime_config,
                reload_shutdown_timeout,
                single_entity_reload_gate(),
            );
            info!(name = %name, cid = %cid, "{}", crate::i18n::t("entity-created"));
            send_crud_ok_cid(message, reply_type, ctx, cid).await
        }
        _ => Err(anyhow!("unknown entities.{name} operation")),
    }
}

// ── Entity field helpers ───────────────────────────────────────────────────────

pub(super) async fn fetch_entity_node(ctx: &CrudHandlerCtx, name: &str) -> Result<EntityNode> {
    let manifest = load_manifest(ctx).await?;
    let link = manifest
        .entities
        .get(name)
        .ok_or_else(|| anyhow!("entity not found: {name}"))?;
    crate::kubo::dag_get(&ctx.kubo_rpc_url, &link.cid)
        .await
        .with_context(|| format!("fetching entity {name} from {}", link.cid))
}

pub(super) async fn update_entity_node(
    ctx: &CrudHandlerCtx,
    name: &str,
    entity: &EntityNode,
) -> Result<String> {
    let entity_cid = crate::kubo::dag_put(&ctx.kubo_rpc_url, entity)
        .await
        .with_context(|| format!("publishing updated entity {name}"))?;
    with_manifest_crud(ctx, |m| {
        m.entities
            .insert(name.to_string(), IpldLink::new(&entity_cid));
        Ok(())
    })
    .await?;
    Ok(entity_cid)
}

async fn handle_entity_field(
    message: &ma_core::Message,
    name: &String,
    field_path: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    let Some((field, sub_path)) = field_path.split_first() else {
        return Err(anyhow!("empty field path in entity.{name}"));
    };

    // Generic GET — works for any leaf field without field-specific code.
    if tail.is_none() && args.is_empty() && sub_path.is_empty() {
        let entity = fetch_entity_node(ctx, name).await?;
        let mut entity_cbor = Vec::new();
        ciborium::ser::into_writer(&entity, &mut entity_cbor)
            .context("serializing entity for field GET")?;
        let cbor_map: CborValue = ciborium::de::from_reader(entity_cbor.as_slice())
            .context("re-parsing entity CBOR map")?;
        if let CborValue::Map(entries) = cbor_map {
            if let Some((_, value)) = entries
                .into_iter()
                .find(|(k, _)| matches!(k, CborValue::Text(s) if s == field))
            {
                return send_crud_data_cbor(message, reply_type, ctx, &value).await;
            }
        }
        return Err(anyhow!("field '{field}' not found in entity '{name}'"));
    }

    match field.as_str() {
        "acl" => {
            handle_entity_acl_field(message, name, sub_path, tail, args, reply_type, ctx).await
        }
        _ => Err(anyhow!("unknown entity field '{field}' in entity.{name}")),
    }
}

async fn handle_entity_acl_field(
    message: &ma_core::Message,
    name: &String,
    sub_path: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    // Handle entity ACL field via CRUD (e.g. `@runtime/entities/scheduler/acl`).
    // Remote notation: `@runtime/entities/<name>/acl: <acl-name>`
    // GET: `@runtime/entities/<name>/acl`
    // SET: `@runtime/entities/<name>/acl: <acl-name>`
    // DELETE: `@runtime/entities/<name>/acl:`

    if !sub_path.is_empty() {
        return Err(anyhow!(
            "entity field 'acl' sub-path '{}' not yet implemented",
            sub_path.join(".")
        ));
    }
    match (tail, args.as_slice()) {
        (None, []) => {
            let entity = fetch_entity_node(ctx, name).await?;
            send_crud_reply_cbor(message, reply_type, ctx, &CborValue::Text(entity.acl)).await
        }
        (Some(""), [CborValue::Text(acl_name)]) => {
            let manifest = load_manifest(ctx).await?;
            if !manifest.acls.contains_key(acl_name) {
                let available: Vec<&String> = manifest.acls.keys().collect();
                return Err(anyhow!(
                    "ACL name '{acl_name}' not found in manifest; available: {available:?}"
                ));
            }
            let mut entity = fetch_entity_node(ctx, name).await?;
            entity.acl = acl_name.clone();
            let entity_cid = update_entity_node(ctx, name, &entity).await?;
            let runtime_config = runtime_config_snapshot(ctx).await?;
            let reload_shutdown_timeout = reload_shutdown_timeout(ctx).await;
            spawn_entity_reload(
                name.clone(),
                entity.clone(),
                ctx.kind_registry.clone(),
                ctx.stats.clone(),
                Arc::clone(&ctx.kubo_rpc_url),
                Arc::clone(&ctx.our_did),
                ctx.envelope_tx.clone(),
                ctx.entity_registry.clone(),
                ctx.manifest_writer.clone(),
                runtime_config,
                reload_shutdown_timeout,
                single_entity_reload_gate(),
            );
            info!(name = %name, acl_name = %acl_name, entity_cid = %entity_cid, "entity ACL name set");
            send_crud_ok_cid(message, reply_type, ctx, &entity_cid).await
        }
        (Some(""), []) => {
            let mut entity = fetch_entity_node(ctx, name).await?;
            entity.acl = String::new();
            let entity_cid = update_entity_node(ctx, name, &entity).await?;
            let runtime_config = runtime_config_snapshot(ctx).await?;
            let reload_shutdown_timeout = reload_shutdown_timeout(ctx).await;
            spawn_entity_reload(
                name.clone(),
                entity.clone(),
                ctx.kind_registry.clone(),
                ctx.stats.clone(),
                Arc::clone(&ctx.kubo_rpc_url),
                Arc::clone(&ctx.our_did),
                ctx.envelope_tx.clone(),
                ctx.entity_registry.clone(),
                ctx.manifest_writer.clone(),
                runtime_config,
                reload_shutdown_timeout,
                single_entity_reload_gate(),
            );
            info!(name = %name, entity_cid = %entity_cid, "entity ACL cleared");
            send_crud_ok_cid(message, reply_type, ctx, &entity_cid).await
        }
        _ => Err(anyhow!("unknown entities.{name}.acl operation")),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use std::time::Duration;

    use axum::extract::State;
    use axum::http::header;
    use axum::routing::get;
    use axum::Router;
    use ciborium::Value as CborValue;
    use tokio::sync::RwLock;

    use super::handle_entity_acl_field;
    use crate::acl::{new_acl_cache, new_group_cache, new_shared_acl, AclMap};
    use crate::entity::SendEnvelope;
    use crate::entity::{
        new_kind_registry, EntityNode, Evaluator, IpldLink, KindNode, RuntimeManifest,
    };
    use crate::manifest::ManifestWriter;
    use crate::plugin::{new_entity_registry, EntityPlugin};
    use crate::status::Stats;
    use crate::testkubo::MockKubo;

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

    fn entity_node(acl: &str) -> EntityNode {
        EntityNode {
            kind: "/ma/test/0.0.1".to_string(),
            behaviour: None,
            acl: acl.to_string(),
            state: None,
            parent: None,
            label: None,
            attributes: BTreeMap::new(),
            init: None,
            initialised: false,
            reload_error: None,
        }
    }

    fn test_config(kubo_rpc_url: &str) -> ma_core::Config {
        ma_core::Config {
            slug: "ma".to_string(),
            log_level: "info".to_string(),
            log_level_stdout: "info".to_string(),
            did_resolver_positive_ttl_secs: 0,
            did_resolver_negative_ttl_secs: 0,
            log_file: None,
            kubo_rpc_url: kubo_rpc_url.to_string(),
            kubo_key_alias: "ma".to_string(),
            pin_remote: false,
            pin_remote_service: None,
            pin_remote_name: None,
            secret_bundle: None,
            secret_bundle_passphrase: None,
            config_path: None,
            extra: serde_yaml::Mapping::new(),
        }
    }

    async fn spawn_did_gateway(doc_bytes: Vec<u8>) -> String {
        async fn serve_doc(
            State(doc_bytes): State<Arc<Vec<u8>>>,
        ) -> ([(header::HeaderName, &'static str); 1], Vec<u8>) {
            (
                [(header::CONTENT_TYPE, "application/vnd.ipld.dag-cbor")],
                (*doc_bytes).clone(),
            )
        }

        let app = Router::new()
            .route("/ipns/:id", get(serve_doc))
            .with_state(Arc::new(doc_bytes));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        format!("http://{addr}")
    }

    async fn wait_for_entity_acl(
        entity_registry: &crate::plugin::EntityRegistry,
        fragment: &str,
        expected_acl: &str,
    ) {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
        loop {
            let plugin = entity_registry.read().await.get(fragment).cloned();
            if let Some(plugin) = plugin {
                if plugin.acl == expected_acl {
                    return;
                }
            }
            assert!(
                tokio::time::Instant::now() < deadline,
                "timed out waiting for entity ACL '{expected_acl}'"
            );
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    #[allow(clippy::too_many_lines)]
    #[tokio::test(flavor = "multi_thread")]
    async fn entity_acl_crud_set_reloads_running_plugin_acl() {
        let kubo = MockKubo::start().await;
        let wasm_cid = kubo.add_bytes(wat::parse_str(GOOD_WAT).unwrap()).await;
        let kind = kind_node(&wasm_cid);

        let open_acl_cid = crate::kubo::dag_put(kubo.url(), &AclMap::new())
            .await
            .unwrap();
        let locked_acl_cid = crate::kubo::dag_put(kubo.url(), &AclMap::new())
            .await
            .unwrap();
        let entity_cid = crate::kubo::dag_put(kubo.url(), &entity_node("open"))
            .await
            .unwrap();

        let mut manifest = RuntimeManifest::default();
        manifest
            .acls
            .insert("open".to_string(), IpldLink::new(open_acl_cid));
        manifest
            .acls
            .insert("locked".to_string(), IpldLink::new(locked_acl_cid));
        manifest
            .entities
            .insert("room".to_string(), IpldLink::new(entity_cid));
        let root_cid = crate::kubo::dag_put(kubo.url(), &manifest).await.unwrap();

        let stats = Arc::new(RwLock::new(Stats {
            root_cid: Some(root_cid.clone()),
            ..Default::default()
        }));
        let manifest_writer =
            ManifestWriter::new(root_cid, kubo.url().to_string(), stats.clone(), None, None);

        let kind_registry = new_kind_registry();
        kind_registry
            .write()
            .await
            .insert(kind.protocol.clone(), Arc::new(kind.clone()));
        let entity_registry = new_entity_registry();
        let (envelope_tx, _envelope_rx) =
            tokio::sync::mpsc::channel::<(String, SendEnvelope)>(16);

        let runtime_did = ma_core::Did::new_url("k51qzi5uqu5runtime", None::<String>).unwrap();
        let runtime_signing = ma_core::SigningKey::generate(
            ma_core::Did::new_url("k51qzi5uqu5runtime", Some("sign")).unwrap(),
        )
        .unwrap();
        let sender_signing = ma_core::SigningKey::generate(
            ma_core::Did::new_url("k51qzi5uqu5sender", Some("sign")).unwrap(),
        )
        .unwrap();

        let (plugin, _) = EntityPlugin::load(
            "room",
            &entity_node("open"),
            &kind,
            &runtime_did.base_id(),
            kubo.url(),
            envelope_tx.clone(),
            entity_registry.clone(),
            "",
            0,
            BTreeMap::new(),
            None,
        )
        .await
        .unwrap();
        entity_registry
            .write()
            .await
            .insert("room".to_string(), Arc::new(plugin));
        assert_eq!(
            entity_registry.read().await.get("room").unwrap().acl,
            "open"
        );

        let mut endpoint = ma_core::new_ma_endpoint([3u8; 32], false).await.unwrap();
        let _crud_inbox = endpoint.service(ma_core::CRUD_PROTOCOL_ID);
        let sender_did = ma_core::Did::new_url("k51qzi5uqu5sender", None::<String>).unwrap();
        let mut sender_doc = ma_core::Document::new(&sender_did, &sender_did);
        let assertion_vm = ma_core::VerificationMethod::try_from(&sender_signing).unwrap();
        sender_doc.verification_method.push(assertion_vm.clone());
        sender_doc.assertion_method.push(assertion_vm.id.clone());
        sender_doc.set_ma_extension(endpoint.ma_extension());
        sender_doc.sign(&sender_signing, &assertion_vm).unwrap();
        let gateway_url = spawn_did_gateway(sender_doc.encode().unwrap()).await;

        let ctx = super::CrudHandlerCtx {
            our_did: Arc::from(runtime_did.base_id()),
            signing_key: Arc::new(runtime_signing),
            endpoint: Arc::from(endpoint),
            kubo_rpc_url: Arc::from(kubo.url().to_string()),
            resolver: Arc::new(ma_core::IpfsGatewayResolver::new(gateway_url)),
            doc_cache: None,
            did_resolve: crate::ipfs::DidResolveSettings::default(),
            stats: stats.clone(),
            entity_registry: entity_registry.clone(),
            kind_registry: kind_registry.clone(),
            shared_config: Arc::new(RwLock::new(test_config(kubo.url()))),
            acl_cache: new_acl_cache(),
            group_cache: new_group_cache(),
            root_acl: new_shared_acl(AclMap::new()),
            envelope_tx,
            manifest_writer: manifest_writer.clone(),
        };

        let incoming = ma_core::Message::new(
            sender_did.base_id(),
            runtime_did.base_id(),
            ma_core::MESSAGE_TYPE_CRUD,
            ma_core::CONTENT_TYPE_TERM,
            b"set acl",
            &sender_signing,
        )
        .unwrap();

        handle_entity_acl_field(
            &incoming,
            &"room".to_string(),
            &[],
            Some(""),
            vec![CborValue::Text("locked".to_string())],
            ma_core::MESSAGE_TYPE_CRUD_REPLY,
            &ctx,
        )
        .await
        .unwrap();

        wait_for_entity_acl(&entity_registry, "room", "locked").await;

        let updated_root = stats.read().await.root_cid.clone().unwrap();
        let updated_manifest: RuntimeManifest = crate::kubo::dag_get(kubo.url(), &updated_root)
            .await
            .unwrap();
        let updated_link = updated_manifest.entities.get("room").unwrap();
        let updated_entity: EntityNode = crate::kubo::dag_get(kubo.url(), &updated_link.cid)
            .await
            .unwrap();
        assert_eq!(updated_entity.acl, "locked");
        assert_eq!(
            entity_registry.read().await.get("room").unwrap().acl,
            "locked"
        );
    }

    #[allow(clippy::too_many_lines)]
    #[tokio::test(flavor = "multi_thread")]
    async fn entity_acl_crud_clear_reloads_running_plugin_acl() {
        let kubo = MockKubo::start().await;
        let wasm_cid = kubo.add_bytes(wat::parse_str(GOOD_WAT).unwrap()).await;
        let kind = kind_node(&wasm_cid);

        let open_acl_cid = crate::kubo::dag_put(kubo.url(), &AclMap::new())
            .await
            .unwrap();
        let entity_cid = crate::kubo::dag_put(kubo.url(), &entity_node("open"))
            .await
            .unwrap();

        let mut manifest = RuntimeManifest::default();
        manifest
            .acls
            .insert("open".to_string(), IpldLink::new(open_acl_cid));
        manifest
            .entities
            .insert("room".to_string(), IpldLink::new(entity_cid));
        let root_cid = crate::kubo::dag_put(kubo.url(), &manifest).await.unwrap();

        let stats = Arc::new(RwLock::new(Stats {
            root_cid: Some(root_cid.clone()),
            ..Default::default()
        }));
        let manifest_writer =
            ManifestWriter::new(root_cid, kubo.url().to_string(), stats.clone(), None, None);

        let kind_registry = new_kind_registry();
        kind_registry
            .write()
            .await
            .insert(kind.protocol.clone(), Arc::new(kind.clone()));
        let entity_registry = new_entity_registry();
        let (envelope_tx, _envelope_rx) =
            tokio::sync::mpsc::channel::<(String, SendEnvelope)>(16);

        let runtime_did = ma_core::Did::new_url("k51qzi5uqu5runtime", None::<String>).unwrap();
        let runtime_signing = ma_core::SigningKey::generate(
            ma_core::Did::new_url("k51qzi5uqu5runtime", Some("sign")).unwrap(),
        )
        .unwrap();
        let sender_signing = ma_core::SigningKey::generate(
            ma_core::Did::new_url("k51qzi5uqu5sender", Some("sign")).unwrap(),
        )
        .unwrap();

        let (plugin, _) = EntityPlugin::load(
            "room",
            &entity_node("open"),
            &kind,
            &runtime_did.base_id(),
            kubo.url(),
            envelope_tx.clone(),
            entity_registry.clone(),
            "",
            0,
            BTreeMap::new(),
            None,
        )
        .await
        .unwrap();
        entity_registry
            .write()
            .await
            .insert("room".to_string(), Arc::new(plugin));
        assert_eq!(
            entity_registry.read().await.get("room").unwrap().acl,
            "open"
        );

        let mut endpoint = ma_core::new_ma_endpoint([3u8; 32], false).await.unwrap();
        let _crud_inbox = endpoint.service(ma_core::CRUD_PROTOCOL_ID);
        let sender_did = ma_core::Did::new_url("k51qzi5uqu5sender", None::<String>).unwrap();
        let mut sender_doc = ma_core::Document::new(&sender_did, &sender_did);
        let assertion_vm = ma_core::VerificationMethod::try_from(&sender_signing).unwrap();
        sender_doc.verification_method.push(assertion_vm.clone());
        sender_doc.assertion_method.push(assertion_vm.id.clone());
        sender_doc.set_ma_extension(endpoint.ma_extension());
        sender_doc.sign(&sender_signing, &assertion_vm).unwrap();
        let gateway_url = spawn_did_gateway(sender_doc.encode().unwrap()).await;

        let ctx = super::CrudHandlerCtx {
            our_did: Arc::from(runtime_did.base_id()),
            signing_key: Arc::new(runtime_signing),
            endpoint: Arc::from(endpoint),
            kubo_rpc_url: Arc::from(kubo.url().to_string()),
            resolver: Arc::new(ma_core::IpfsGatewayResolver::new(gateway_url)),
            doc_cache: None,
            did_resolve: crate::ipfs::DidResolveSettings::default(),
            stats: stats.clone(),
            entity_registry: entity_registry.clone(),
            kind_registry: kind_registry.clone(),
            shared_config: Arc::new(RwLock::new(test_config(kubo.url()))),
            acl_cache: new_acl_cache(),
            group_cache: new_group_cache(),
            root_acl: new_shared_acl(AclMap::new()),
            envelope_tx,
            manifest_writer: manifest_writer.clone(),
        };

        let incoming = ma_core::Message::new(
            sender_did.base_id(),
            runtime_did.base_id(),
            ma_core::MESSAGE_TYPE_CRUD,
            ma_core::CONTENT_TYPE_TERM,
            b"clear acl",
            &sender_signing,
        )
        .unwrap();

        handle_entity_acl_field(
            &incoming,
            &"room".to_string(),
            &[],
            Some(""),
            vec![],
            ma_core::MESSAGE_TYPE_CRUD_REPLY,
            &ctx,
        )
        .await
        .unwrap();

        wait_for_entity_acl(&entity_registry, "room", "").await;

        let updated_root = stats.read().await.root_cid.clone().unwrap();
        let updated_manifest: RuntimeManifest = crate::kubo::dag_get(kubo.url(), &updated_root)
            .await
            .unwrap();
        let updated_link = updated_manifest.entities.get("room").unwrap();
        let updated_entity: EntityNode = crate::kubo::dag_get(kubo.url(), &updated_link.cid)
            .await
            .unwrap();
        assert_eq!(updated_entity.acl, "");
        assert_eq!(entity_registry.read().await.get("room").unwrap().acl, "");
    }
}
