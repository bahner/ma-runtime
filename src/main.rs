mod acl;
mod behaviour;
mod bootstrap;
mod crud;
mod entity;
mod eventloop;
mod i18n;
mod inbox;
mod ipfs;
mod kubo;
mod manifest;
mod plugin;
mod republish;
mod routing;
mod rpc;
mod schedule;
mod scheduler_actor;
mod startup;
mod status;

#[cfg(test)]
mod testkubo;

use anyhow::{anyhow, Context, Result};
use cid::Cid;
use clap::Parser;
use ma_core::config::{Config, MaArgs};
use ma_core::ipfs::IpfsDidPublisher;
use ma_core::{ipns_from_secret, Ipld, INBOX_PROTOCOL_ID, IPFS_PROTOCOL_ID};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

use startup::{
    get_bool_setting, get_u64_setting, load_secret_bundle,
    materialise_plugin_envelope_queue_capacity, qa_prepare_bundle_timestamps_for_publish,
    require_kinds_overlay_base, runtime_manifest_config, select_poll_ms, select_root_cid,
    select_status_bind, should_generate_headless_config,
};

const MA_DEFAULT_SLUG: &str = "ma";

/// Generates a fresh headless config + secret bundle, then adds and persists
/// the `runtime_ipns` extra key so the bundle is complete before first use.
fn generate_and_persist_headless_config(ma_args: &MaArgs) -> Result<()> {
    Config::gen_headless(ma_args, MA_DEFAULT_SLUG)?;
    let config = Config::from_args(ma_args, MA_DEFAULT_SLUG)?;
    let mut bundle = load_secret_bundle(&config)?;
    bundle
        .generate_key("runtime_ipns")
        .context("failed to generate 'runtime_ipns' key")?;
    let passphrase = config
        .secret_bundle_passphrase
        .as_deref()
        .ok_or_else(|| anyhow!("secret_bundle_passphrase missing after gen_headless"))?;
    let bundle_path = config.effective_secret_bundle()?;
    bundle
        .save(&bundle_path, passphrase)
        .context("failed to re-save bundle with 'runtime_ipns' key")?;
    Ok(())
}

/// Waits for the Kubo RPC API at `kubo_rpc_url` to become reachable.
async fn wait_for_kubo_ready(kubo_rpc_url: &str, context_msg: &'static str) -> Result<()> {
    let publisher = IpfsDidPublisher::new(kubo_rpc_url)
        .with_context(|| format!("invalid kubo_rpc_url: {kubo_rpc_url}"))?;
    publisher.wait_until_ready(10).await.context(context_msg)?;
    Ok(())
}

/// Publishes a bootstrap YAML manifest tree to IPFS. Shared by
/// `--gen-root-cid` and `--bootstrap`, which run identical logic.
async fn publish_bootstrap_manifest(
    yaml_path: &std::path::Path,
    config: &Config,
    effective_lang_base: Option<&str>,
    cli_root_cid: Option<&str>,
) -> Result<bootstrap::BootstrapResult> {
    wait_for_kubo_ready(&config.kubo_rpc_url, "kubo RPC is not reachable for bootstrap").await?;
    let runtime_config = runtime_manifest_config(config);
    let remote_pin = bootstrap::runtime_remote_pin_config(config);
    bootstrap::run_bootstrap(
        yaml_path,
        &config.kubo_rpc_url,
        runtime_config,
        effective_lang_base.unwrap_or("nb"),
        cli_root_cid,
        remote_pin.as_ref(),
    )
    .await
    .context("bootstrap failed")
}

#[derive(Debug, Parser)]
#[command(name = "ma")]
#[command(about = "間 Runtime daemon — RPC + optional IPFS publisher, powered by ma-core")]
struct Cli {
    #[command(flatten)]
    ma: MaArgs,

    /// DID(s) of the runtime owner(s). Repeat for multiple: --owner <did1> --owner <did2>.
    /// Falls back to `owners:` list in config.yaml.
    #[arg(long)]
    owner: Vec<String>,

    /// Publish FTL lang files + manifest from YAML and print the resulting root CID, then exit.
    #[arg(long)]
    gen_root_cid: Option<PathBuf>,

    /// Publish only runtime.kinds from YAML and print the resulting kinds CID, then exit.
    #[arg(long)]
    gen_kinds_cid: Option<PathBuf>,

    /// Bootstrap from YAML: publish manifest to IPFS and start the daemon using the resulting root CID.
    #[arg(long)]
    bootstrap: Option<PathBuf>,

    /// WARNING: resets runtime head for this process. If wrong, recover old CID from logs.
    #[arg(long, env = "MA_ROOT_CID")]
    root_cid: Option<String>,

    /// Apply a kinds-tree CID over the selected runtime head without replacing entities.
    #[arg(long)]
    kinds_cid: Option<String>,

    /// Poll interval in milliseconds.
    #[arg(long)]
    poll_ms: Option<u64>,

    /// Language for log messages. Falls back to `i18n:` in config.yaml, then "nb".
    #[arg(long, env = "MA_I18N")]
    i18n: Option<String>,

    /// Status web server bind address.
    #[arg(long)]
    status_bind: Option<SocketAddr>,
}

/// Endpoint plus the per-protocol message inboxes registered on it.
struct EndpointServices {
    endpoint: Arc<dyn ma_core::MaEndpoint>,
    rpc_messages: ma_core::Inbox<ma_core::Message>,
    inbox_messages: ma_core::Inbox<ma_core::Message>,
    ipfs_messages: Option<ma_core::Inbox<ma_core::Message>>,
    crud_messages: Option<ma_core::Inbox<ma_core::Message>>,
}

/// Builds the iroh endpoint and registers the RPC/inbox/IPFS/CRUD services on
/// it. Service registration requires `&mut endpoint`, so this returns the
/// endpoint already converted to the shared `Arc<dyn MaEndpoint>` used by the
/// rest of the daemon.
async fn setup_endpoint_services(
    secrets: &ma_core::config::SecretBundle,
    shared_resolver: &Arc<dyn ma_core::DidDocumentResolver>,
    ipv6_enabled: bool,
    ipfs_publisher_enabled: bool,
    crud_enabled: bool,
) -> Result<EndpointServices> {
    if ipv6_enabled {
        info!("{}", i18n::t("ipv6-enabled"));
    } else {
        info!("{}", i18n::t("ipv6-disabled"));
    }
    let mut endpoint = ma_core::new_ma_endpoint(
        secrets.iroh_secret_key,
        secrets.encryption_key()?,
        Arc::clone(shared_resolver),
        ipv6_enabled,
    )
    .await?;
    let rpc_messages = endpoint.service(rpc::RPC_PROTOCOL_ID);
    let inbox_messages = endpoint.service(INBOX_PROTOCOL_ID);
    let ipfs_messages = ipfs_publisher_enabled.then(|| endpoint.service(IPFS_PROTOCOL_ID));
    let crud_messages = crud_enabled.then(|| endpoint.service(crud::CRUD_PROTOCOL_ID));

    // Convert endpoint to Arc so it can be shared across tokio::spawn tasks.
    // All service() registrations are complete at this point.
    let endpoint: Arc<dyn ma_core::MaEndpoint> = Arc::from(endpoint);

    Ok(EndpointServices {
        endpoint,
        rpc_messages,
        inbox_messages,
        ipfs_messages,
        crud_messages,
    })
}

/// Own DID document details needed for logging, RPC identity, and periodic
/// republishing.
struct OwnIdentity {
    our_did: String,
    ma_base: ma_core::MaExtension,
    did_publish_timeout_secs: u64,
    did_publish_interval_secs: u64,
    ipns_publish: ipfs::IpnsPublishSettings,
    did_resolve: ipfs::DidResolveSettings,
}

/// Builds the runtime's own DID document (linking `ma.runtime` to `root_cid`
/// when known), spawns a best-effort background task to publish it to
/// Kubo/IPNS, and returns the identity fields the rest of startup needs.
fn build_and_publish_own_identity(
    endpoint: &Arc<dyn ma_core::MaEndpoint>,
    secrets: &ma_core::config::SecretBundle,
    config: &Config,
    root_cid: Option<&str>,
) -> Result<OwnIdentity> {
    // Først: bygg ma-extension uten runtime-link for å få DID
    let ma_base = endpoint
        .ma_extension()
        .kind("runtime")
        .extra("protocol", Ipld::String("/ma/runtime/0.0.1".to_string()));
    secrets
        .build_document(ma_base.clone())
        .context("failed to build own DID document (base)")?;

    // ma.runtime skal være en ekte CID-link (bafy...) for direkte DAG-traversering.
    let ma = if let Some(rc) = root_cid {
        let runtime_cid = Cid::try_from(rc).with_context(|| format!("invalid root_cid: {rc}"))?;
        ma_base.clone().extra("runtime", Ipld::Link(runtime_cid))
    } else {
        ma_base.clone()
    };
    let our_document = secrets
        .build_document(ma)
        .context("failed to build own DID document")?;

    let our_did = our_document.id.clone();

    let did_publish_timeout_secs =
        get_u64_setting(config, "did_document_publishing_timeout_secs", 120);
    let did_publish_lifetime_hours =
        get_u64_setting(config, "did_document_publishing_lifetime_hours", 8760);
    let did_publish_interval_secs =
        get_u64_setting(config, "did_document_publishing_interval_secs", 3600);
    let ipns_publish = ipfs::IpnsPublishSettings {
        lifetime_hours: get_u64_setting(
            config,
            "ipns_publish_lifetime_hours",
            did_publish_lifetime_hours,
        ),
        allow_offline: get_bool_setting(config, "ipns_publish_allow_offline", true),
        resolve: get_bool_setting(config, "ipns_publish_resolve", false),
        timeout_secs: get_u64_setting(
            config,
            "ipns_publish_timeout_secs",
            did_publish_timeout_secs,
        ),
    };
    let did_resolve = ipfs::DidResolveSettings {
        attempts: usize::try_from(get_u64_setting(config, "did_resolve_attempts", 5))
            .unwrap_or(usize::MAX),
        attempt_timeout_secs: get_u64_setting(config, "did_resolve_attempt_timeout_secs", 60),
    };

    let doc_cbor = our_document
        .encode()
        .context("failed to encode own DID document")?;

    let ipns_key = secrets.ipns_secret_key.to_vec();
    let kubo_url_clone = config.kubo_rpc_url.clone();
    let runtime_slug = config.slug.clone();
    let did_remote_pin = bootstrap::runtime_remote_pin_config(config);
    let did_pin_overwrite = config.pin_overwrite;
    let did_for_log = our_did.clone();
    tokio::spawn(async move {
        let result = tokio::time::timeout(
            Duration::from_secs(did_publish_timeout_secs),
            ipfs::do_publish_own_document(
                kubo_url_clone,
                runtime_slug,
                doc_cbor,
                ipns_key,
                ipns_publish,
                did_remote_pin,
                did_pin_overwrite,
            ),
        )
        .await;
        match result {
            Ok(Ok(())) => info!(did = %did_for_log, "{}", i18n::t("own-did-published")),
            Ok(Err(err)) => {
                error!(did = %did_for_log, error = %format!("{err:#}"), "{}", i18n::t("own-did-publish-failed"));
            }
            Err(_) => {
                error!(did = %did_for_log, "{}", i18n::t("own-did-publish-timeout"));
            }
        }
    });

    Ok(OwnIdentity {
        our_did,
        ma_base,
        did_publish_timeout_secs,
        did_publish_interval_secs,
        ipns_publish,
        did_resolve,
    })
}

/// Resolves the runtime root CID once Kubo is reachable: falls back to
/// resolving it via the runtime's own IPNS record, then to bootstrapping a
/// minimal manifest if none exists yet, then applies any `--kinds-cid`
/// overlay on top of whatever root was found or already supplied.
async fn resolve_startup_root_cid(
    config: &Config,
    runtime_ipns_id: &str,
    kinds_cid: Option<&str>,
    mut root_cid: Option<String>,
) -> Result<Option<String>> {
    if root_cid.is_none() {
        root_cid =
            ipfs::resolve_runtime_root_cid_by_ipns_id(&config.kubo_rpc_url, runtime_ipns_id)
                .await
                .context("failed to resolve runtime root CID from IPNS")?;
        require_kinds_overlay_base(kinds_cid, root_cid.as_deref())?;
        if root_cid.is_none() {
            warn!("No runtime root CID found in IPNS; bootstrapping minimal manifest");
            match status::bootstrap_minimal_manifest(&config.kubo_rpc_url, &[]).await {
                Ok(cid) => {
                    info!(cid = %cid, "Minimal manifest bootstrapped");
                    root_cid = Some(cid);
                }
                Err(e) => warn!(error = %format!("{e:#}"), "Failed to bootstrap minimal manifest"),
            }
        }
    }

    if let (Some(rc), Some(kinds_cid)) = (root_cid.as_deref(), kinds_cid) {
        let result = bootstrap::apply_kinds_overlay(rc, kinds_cid, &config.kubo_rpc_url)
            .await
            .context("applying kinds CID overlay failed")?;
        if result.root_cid == rc {
            info!(changed = 0, "Kinds overlay made no manifest changes");
        } else {
            info!(root_cid = %result.root_cid, changed = result.changed_protocols.len(), "Kinds overlay applied");
            root_cid = Some(result.root_cid);
        }
    }

    Ok(root_cid)
}

/// State produced by loading entity plugins and starting the native
/// scheduler backend.
struct EntityStartup {
    entity_registry: plugin::EntityRegistry,
    kind_registry: entity::KindRegistry,
    startup_epoch: u64,
    startup_iroh_node_id: String,
    /// Kept alive for the remainder of the process; the scheduler runs its
    /// jobs on a background task owned by this handle.
    sched: Arc<tokio_cron_scheduler::JobScheduler>,
    scheduler_manifest_writer: Arc<tokio::sync::RwLock<Option<manifest::ManifestWriter>>>,
    root_cid: Option<String>,
}

/// Starts the native cron/interval scheduler backend and, if `root_cid` is
/// known, loads all entity plugins referenced by the manifest.
async fn load_entities_and_scheduler(
    config: &Config,
    our_did: &str,
    endpoint: &Arc<dyn ma_core::MaEndpoint>,
    envelope_tx: &tokio::sync::mpsc::Sender<(String, entity::SendEnvelope)>,
    mut root_cid: Option<String>,
) -> Result<EntityStartup> {
    let entity_registry = plugin::new_entity_registry();
    let kind_registry = entity::new_kind_registry();
    let startup_epoch = status::now_unix_secs();
    let startup_iroh_node_id = endpoint.id();

    // ── Native scheduler backend ─────────────────────────────────────────────
    let sched = Arc::new(
        tokio_cron_scheduler::JobScheduler::new()
            .await
            .context("creating job scheduler")?,
    );
    sched.start().await.context("starting job scheduler")?;
    let scheduler_manifest_writer = Arc::new(tokio::sync::RwLock::new(None));

    let sched_ctx = schedule::SchedulerCtx {
        entity_registry: entity_registry.clone(),
        manifest_writer: Arc::clone(&scheduler_manifest_writer),
        kubo_rpc_url: config.kubo_rpc_url.clone(),
        our_did: our_did.to_string(),
    };
    let mut native_factories = plugin::NativeFactories::new();
    native_factories.insert(
        scheduler_actor::SCHEDULER_KIND.to_string(),
        scheduler_actor::native_factory(Arc::clone(&sched), sched_ctx),
    );

    if let Some(ref rc) = root_cid {
        let (count, updated_root) = bootstrap::load_entities(bootstrap::LoadEntitiesArgs {
            root_cid: rc,
            kubo_url: &config.kubo_rpc_url,
            daemon_config: config,
            our_did,
            registry: &entity_registry,
            kind_registry: &kind_registry,
            native_factories: &native_factories,
            envelope_tx: envelope_tx.clone(),
            iroh_node_id: &startup_iroh_node_id,
            started_at: startup_epoch,
        })
        .await;
        info!(count = %count, "Entity plugins loaded");
        if let Some(new_rc) = updated_root {
            root_cid = Some(new_rc);
        }
    }

    Ok(EntityStartup {
        entity_registry,
        kind_registry,
        startup_epoch,
        startup_iroh_node_id,
        sched,
        scheduler_manifest_writer,
        root_cid,
    })
}

/// Loads the manifest's transport-gate ACL, named groups, and root verb-ACL
/// library into their in-memory caches. Returns the `owners` group's members
/// (if present), reused by owner resolution so the manifest is only fetched
/// once at startup.
async fn populate_acl_and_group_caches(
    config: &Config,
    root_cid: Option<&str>,
    acl: &acl::SharedAcl,
    acl_cache: &acl::AclCache,
    group_cache: &acl::GroupCache,
) -> Vec<String> {
    let mut manifest_owners: Vec<String> = Vec::new();
    let Some(rc) = root_cid else {
        return manifest_owners;
    };
    let manifest: Result<entity::RuntimeManifest, _> = kubo::dag_get(&config.kubo_rpc_url, rc).await;
    let m = match manifest {
        Ok(m) => m,
        Err(e) => {
            warn!(error = %e, "failed to load manifest for ACL cache population");
            return manifest_owners;
        }
    };

    // Load the manifest ACL as the transport gate.
    if let Some(ref link) = m.acl {
        match acl::load_acl_from_cid(&config.kubo_rpc_url, &link.cid).await {
            Ok(manifest_acl) => {
                info!(cid = %link.cid, "Root transport-gate ACL loaded from manifest");
                *acl.write().await = manifest_acl;
            }
            Err(e) => {
                warn!(cid = %link.cid, error = %e, "failed to load root ACL from manifest");
            }
        }
    }

    // Named groups: "/grp/<name>", flat Vec<String> of member DIDs.
    // "owners" is one ordinary entry here, no special resolution.
    let mut group_entries = Vec::new();
    for (name, link) in &m.grp {
        match kubo::dag_get::<Vec<String>>(&config.kubo_rpc_url, &link.cid).await {
            Ok(members) => {
                info!(name = %name, cid = %link.cid, "Group loaded into cache");
                group_entries.push((name.clone(), members));
            }
            Err(e) => {
                warn!(name = %name, cid = %link.cid, error = %e, "failed to load group at startup");
            }
        }
    }
    {
        let mut cache = group_cache.write().await;
        for (name, members) in group_entries {
            if name == "owners" {
                manifest_owners.clone_from(&members);
            }
            cache.insert(name, members);
        }
    }

    let mut entries = Vec::new();
    // Root verb-ACL library: "acls.<name>"
    for (acl_name, link) in &m.acls {
        let cache_key = format!("acls.{acl_name}");
        match acl::load_acl_from_cid(&config.kubo_rpc_url, &link.cid).await {
            Ok(acl_map) => {
                info!(key = %cache_key, cid = %link.cid, "Root ACL loaded into cache");
                entries.push((cache_key, acl_map));
            }
            Err(e) => {
                warn!(key = %cache_key, cid = %link.cid, error = %e, "failed to load root ACL at startup");
            }
        }
    }
    let mut cache = acl_cache.write().await;
    for (key, acl_map) in entries {
        cache.insert(key, acl_map);
    }

    manifest_owners
}

/// Result of reconciling owners from config.yaml, `--owner`, and the
/// manifest's `/grp/owners` group.
struct ResolvedOwners {
    owners: Vec<String>,
    /// True if config.yaml/`--owner` introduced an owner not already in the
    /// manifest, meaning the manifest needs reconciling once the manifest
    /// writer exists further down in startup.
    needs_manifest_sync: bool,
}

/// Reconciles owners from config.yaml, `--owner` CLI args, and the
/// manifest's `/grp/owners` group (union), best-effort persists the merged
/// list back to config.yaml, and seeds the live transport-gate ACL with
/// wildcard permissions for every known owner.
async fn resolve_and_seed_owners(
    config: &Config,
    cli_owners: &[String],
    manifest_owners: &[String],
    acl: &acl::SharedAcl,
) -> ResolvedOwners {
    let owners_from_config: Vec<String> = match config.extra.get("owners") {
        Some(serde_yaml::Value::Sequence(seq)) => seq
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect(),
        Some(serde_yaml::Value::String(s)) => vec![s.clone()],
        _ => vec![],
    };
    let mut resolved_owners: Vec<String> = owners_from_config.clone();
    for o in cli_owners {
        if !resolved_owners.contains(o) {
            resolved_owners.push(o.clone());
        }
    }
    for o in manifest_owners {
        if !resolved_owners.contains(o) {
            resolved_owners.push(o.clone());
        }
    }

    let needs_manifest_sync = resolved_owners.iter().any(|o| !manifest_owners.contains(o));

    // Best-effort persist the reconciled list back to config.yaml so a
    // future restart has the right fallback even if Kubo/the manifest is
    // briefly unreachable.
    if resolved_owners != owners_from_config {
        if let Some(ref path) = config.config_path {
            if let Err(e) = status::persist_owners_to_config(path, &resolved_owners) {
                warn!(error = %e, "failed to persist reconciled owners to config.yaml");
            }
        }
    }

    // Seed the live ACL with wildcard permissions for every known owner so
    // they can use RPC immediately (before any manifest ACL is published).
    if !resolved_owners.is_empty() {
        status::grant_owners_in_acl(acl, &resolved_owners).await;
    }

    ResolvedOwners {
        owners: resolved_owners,
        needs_manifest_sync,
    }
}

/// Inputs needed to build the shared stats/config/manifest-writer instances
/// and spawn the status server and periodic DID republishing task (see
/// [`finalize_startup`]).
struct FinalizeArgs<'a> {
    config: &'a Config,
    status_bind: SocketAddr,
    acl: &'a acl::SharedAcl,
    group_cache: &'a acl::GroupCache,
    entity_registry: &'a plugin::EntityRegistry,
    scheduler_manifest_writer: &'a Arc<tokio::sync::RwLock<Option<manifest::ManifestWriter>>>,
    our_did: &'a str,
    ma_base: ma_core::MaExtension,
    root_cid: Option<String>,
    startup_iroh_node_id: String,
    startup_epoch: u64,
    ipfs_publisher_enabled: bool,
    resolved_owners: Vec<String>,
    owners_need_manifest_sync: bool,
    runtime_ipns_key: [u8; 32],
    did_publish_interval_secs: u64,
    did_publish_timeout_secs: u64,
    ipns_publish: ipfs::IpnsPublishSettings,
}

/// The shared runtime state produced by [`finalize_startup`], consumed by
/// `eventloop::run`.
struct FinalizedStartup {
    stats: status::SharedStats,
    shared_config: Arc<tokio::sync::RwLock<Config>>,
    manifest_writer: manifest::ManifestWriter,
}

/// Builds the shared stats and config handles, spawns the status server,
/// creates the serialised manifest writer, reconciles any config.yaml/
/// `--owner` owners into the manifest, populates the default `.config` root,
/// and spawns the periodic DID-document republishing task.
/// Publishes the merged owners list (from `stats.owners`) and links it into
/// the manifest at `/grp/owners`, updating the in-memory group cache to
/// match. Unlike the old inline `manifest.owners` field, `/grp/owners` is an
/// IPLD link to its own document, so the merged list must be published
/// first. Best-effort: logs and returns on any failure.
async fn reconcile_owners_into_manifest(
    config: &Config,
    stats: &status::SharedStats,
    manifest_writer: &manifest::ManifestWriter,
    group_cache: &acl::GroupCache,
) {
    let merged_owners = stats.read().await.owners.clone();
    if merged_owners.is_empty() {
        return;
    }
    let owners_cid = match kubo::dag_put(&config.kubo_rpc_url, &merged_owners).await {
        Ok(cid) => cid,
        Err(e) => {
            warn!(error = %e, "failed to publish reconciled owners list at startup");
            return;
        }
    };
    let link_cid = owners_cid.clone();
    match manifest_writer
        .mutate(move |m| {
            m.grp
                .insert("owners".to_string(), entity::IpldLink::new(&link_cid));
            Ok(())
        })
        .await
    {
        Ok(cid) => {
            group_cache
                .write()
                .await
                .insert("owners".to_string(), merged_owners);
            info!(cid = %cid, "owners reconciled from config.yaml/--owner into manifest at startup");
        }
        Err(e) => {
            warn!(error = %e, "failed to reconcile owners into manifest at startup");
        }
    }
}

async fn finalize_startup(args: FinalizeArgs<'_>) -> Result<FinalizedStartup> {
    let FinalizeArgs {
        config,
        status_bind,
        acl,
        group_cache,
        entity_registry,
        scheduler_manifest_writer,
        our_did,
        ma_base,
        root_cid,
        startup_iroh_node_id,
        startup_epoch,
        ipfs_publisher_enabled,
        resolved_owners,
        owners_need_manifest_sync,
        runtime_ipns_key,
        did_publish_interval_secs,
        did_publish_timeout_secs,
        ipns_publish,
    } = args;

    // ── Shared stats ───────────────────────────────────────────────────────────
    let entity_names: Vec<String> = entity_registry.read().await.keys().cloned().collect();
    let stats = Arc::new(tokio::sync::RwLock::new(status::Stats {
        our_did: our_did.to_string(),
        endpoint_id: startup_iroh_node_id,
        started_at: startup_epoch,
        ipfs_publisher_enabled,
        entity_names,
        root_cid: root_cid.clone(),
        kubo_rpc_url: config.kubo_rpc_url.clone(),
        owners: resolved_owners,
        config_path: config.config_path.clone(),
        ..Default::default()
    }));

    // Shared daemon config (enables runtime RPC writes + config.yaml save-back).
    let shared_config: Arc<tokio::sync::RwLock<Config>> =
        Arc::new(tokio::sync::RwLock::new(config.clone()));

    status::spawn_status_server(stats.clone(), acl.clone(), status_bind);

    // Serialised manifest writer — all runtime-phase manifest mutations (CRUD
    // sets, ma_create_entity) go through this to avoid last-writer-wins races.
    let manifest_writer = manifest::ManifestWriter::new(
        root_cid.unwrap_or_default(),
        config.kubo_rpc_url.clone(),
        stats.clone(),
    );
    *scheduler_manifest_writer.write().await = Some(manifest_writer.clone());

    // Reconcile owners into the manifest if config.yaml/--owner introduced
    // any that weren't already there (see "Resolve owners" above).
    if owners_need_manifest_sync {
        reconcile_owners_into_manifest(config, &stats, &manifest_writer, group_cache).await;
    }

    populate_default_config_root(
        &manifest_writer,
        entity_registry,
        &stats,
        &config.kubo_rpc_url,
        our_did,
    )
    .await;

    // Periodic DID-document republishing from the in-memory runtime head.
    let did_publish_cache_warm_secs = get_u64_setting(config, "did_publish_cache_warm_secs", 3600);
    let refresh_passphrase = config
        .secret_bundle_passphrase
        .clone()
        .ok_or_else(|| anyhow!("secret_bundle_passphrase is required for periodic DID publish"))?;
    republish::spawn_periodic_did_publish(republish::PeriodicDidPublishContext {
        stats: stats.clone(),
        shared_config: Arc::clone(&shared_config),
        kubo_url: config.kubo_rpc_url.clone(),
        runtime_slug: config.slug.clone(),
        ma_base,
        runtime_ipns_key,
        bundle_path: config.effective_secret_bundle()?,
        passphrase: refresh_passphrase,
        interval: std::time::Duration::from_secs(did_publish_interval_secs),
        cache_warm: std::time::Duration::from_secs(did_publish_cache_warm_secs),
        timeout: std::time::Duration::from_secs(did_publish_timeout_secs),
        ipns_publish,
    });

    Ok(FinalizedStartup {
        stats,
        shared_config,
        manifest_writer,
    })
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() -> Result<()> {
    let mut cli = Cli::parse();
    cli.root_cid = cli.root_cid.take().and_then(|root_cid| {
        let root_cid = root_cid.trim();
        (!root_cid.is_empty()).then(|| root_cid.to_string())
    });

    if cli.ma.gen_headless_config {
        generate_and_persist_headless_config(&cli.ma)?;
        return Ok(());
    }

    let config = Config::from_args(&cli.ma, MA_DEFAULT_SLUG)?;
    config.init_logging()?;

    // Compute from CLI / config.yaml only — manifest not loaded yet.
    // For runtime startup the manifest fallback is applied later.
    let effective_lang_base: Option<String> = cli.i18n.clone().or_else(|| {
        config
            .extra
            .get("i18n")
            .and_then(|v| v.as_str())
            .map(String::from)
    });

    // ── Auto-generate headless config on first run ────────────────────────────
    // If the secret bundle is missing (true first-run state), generate a full
    // headless config automatically so the daemon works out of the box without
    // manual configuration.
    let bundle_path = config.effective_secret_bundle()?;
    let mut config = if should_generate_headless_config(&config, &bundle_path) {
        warn!("No config found.");
        warn!("Initialising new runtime identity.");
        generate_and_persist_headless_config(&cli.ma)?;
        warn!("Generated headless config.");
        Config::from_args(&cli.ma, MA_DEFAULT_SLUG)?
    } else {
        if !bundle_path.exists() {
            let config_path = config.config_path.as_ref().map_or_else(
                || "<unknown>".to_string(),
                |path| path.display().to_string(),
            );
            return Err(anyhow!(
                "secret bundle not found: {} (loaded config from {}; set secret_bundle in that config or pass --secret-bundle)",
                bundle_path.display(),
                config_path
            ));
        }
        config
    };

    let poll_ms = select_poll_ms(cli.poll_ms, &config)?;
    let status_bind = select_status_bind(cli.status_bind, &config)?;

    // ── gen-root-cid: publish bootstrap tree + lang files to IPFS, print root CID, exit ──
    if let Some(ref yaml_path) = cli.gen_root_cid {
        let result = publish_bootstrap_manifest(
            yaml_path,
            &config,
            effective_lang_base.as_deref(),
            cli.root_cid.as_deref(),
        )
        .await?;
        println!("{}", result.root_cid);
        return Ok(());
    }

    // ── gen-kinds-cid: publish only runtime.kinds to IPFS, print CID, exit ──
    if let Some(ref yaml_path) = cli.gen_kinds_cid {
        wait_for_kubo_ready(
            &config.kubo_rpc_url,
            "kubo RPC is not reachable for kinds bootstrap",
        )
        .await?;
        let kinds_cid = bootstrap::run_kinds_bootstrap(yaml_path, &config.kubo_rpc_url)
            .await
            .context("kinds bootstrap failed")?;
        println!("{kinds_cid}");
        return Ok(());
    }

    // ── bootstrap: publish manifest from YAML, use resulting CID, then continue ──
    let bootstrap_root_cid: Option<String> = if let Some(ref yaml_path) = cli.bootstrap {
        let result = publish_bootstrap_manifest(
            yaml_path,
            &config,
            effective_lang_base.as_deref(),
            cli.root_cid.as_deref(),
        )
        .await?;
        info!(root_cid = %result.root_cid, "Bootstrap complete");
        Some(result.root_cid)
    } else {
        None
    };

    if let Some(ref cid) = cli.root_cid {
        Cid::try_from(cid.as_str()).with_context(|| format!("invalid --root-cid CID: {cid}"))?;
        info!(root_cid = %cid, "runtime head reset for this session");
    }
    if let Some(ref cid) = cli.kinds_cid {
        Cid::try_from(cid.as_str()).with_context(|| format!("invalid --kinds-cid CID: {cid}"))?;
    }

    // ── gen-lang-cid has been replaced by `make src/i18n.yaml` ────────────

    let acl = acl::new_shared_acl(acl::AclMap::new()); // deny-all until manifest loads

    let ipfs_publisher_enabled = config
        .extra
        .get("ipfs_publisher")
        .and_then(serde_yaml::value::Value::as_bool)
        .unwrap_or(true);

    let ipv6_enabled = config
        .extra
        .get("ipv6_enable")
        .and_then(serde_yaml::value::Value::as_bool)
        .unwrap_or(true);

    let mut secrets = load_secret_bundle(&config)?;
    qa_prepare_bundle_timestamps_for_publish(&mut secrets);
    let shared_resolver: Arc<dyn ma_core::DidDocumentResolver> =
        Arc::new(config.ipfs_gateway_resolver());

    // ── Runtime IPNS key (separate from the DID-document IPNS key) ───────────
    let runtime_ipns_key: [u8; 32] = secrets
        .get_key("runtime_ipns")
        .copied()
        .ok_or_else(|| anyhow!("secret bundle is missing extra key 'runtime_ipns'"))?;
    let runtime_ipns_id = ipns_from_secret(runtime_ipns_key)
        .context("failed to derive runtime IPNS id from 'runtime_ipns' key")?;

    // ── iroh endpoint ──────────────────────────────────────────────────────────
    let crud_enabled = config
        .extra
        .get("crud_service")
        .and_then(serde_yaml::value::Value::as_bool)
        .unwrap_or(true);
    let EndpointServices {
        endpoint,
        rpc_messages,
        inbox_messages,
        ipfs_messages,
        crud_messages,
    } = setup_endpoint_services(
        &secrets,
        &shared_resolver,
        ipv6_enabled,
        ipfs_publisher_enabled,
        crud_enabled,
    )
    .await?;

    // ── Own DID document (ma extension uses protocol + runtime link) ─────────
    // root_cid priority: --root-cid CLI > --bootstrap generated CID > config.yaml > IPNS resolution.
    let mut root_cid = select_root_cid(cli.root_cid.clone(), bootstrap_root_cid, &config)?;
    if let Some(ref root_cid) = root_cid {
        startup::persist_root_cid(&mut config, root_cid)
            .context("failed to persist selected root_cid")?;
    }
    let lang_cid = config
        .extra
        .get("i18n_cid")
        .and_then(|v| v.as_str())
        .map(String::from)
        .or_else(|| i18n::default_lang_cid().map(String::from));

    let OwnIdentity {
        our_did,
        ma_base,
        did_publish_timeout_secs,
        did_publish_interval_secs,
        ipns_publish,
        did_resolve,
    } = build_and_publish_own_identity(&endpoint, &secrets, &config, root_cid.as_deref())?;

    // ── Wait for Kubo ──────────────────────────────────────────────────────────
    wait_for_kubo_ready(&config.kubo_rpc_url, "kubo RPC is not reachable").await?;

    root_cid = resolve_startup_root_cid(
        &config,
        &runtime_ipns_id,
        cli.kinds_cid.as_deref(),
        root_cid,
    )
    .await?;

    let plugin_envelope_queue_capacity = if let Some(current_root) = root_cid.as_deref() {
        let (materialised_root, capacity) =
            materialise_plugin_envelope_queue_capacity(&config.kubo_rpc_url, current_root)
                .await
                .context("loading plugin envelope queue capacity")?;
        root_cid = Some(materialised_root);
        capacity
    } else {
        crud::config::DEFAULT_PLUGIN_ENVELOPE_QUEUE_CAPACITY
    };
    let outbox_backoff_attempts =
        crud::config::outbox_backoff_attempts(config.extra.get("outbox_backoff_attempts"))
            .context("invalid outbox_backoff_attempts in config.yaml")?;

    // ── i18n: fetch lang via RuntimeManifest.i18n from IPFS ────────────
    // Priority: --i18n / MA_I18N > config.extra["i18n"] > manifest config.i18n (CID
    // reverse-lookup) > "nb". The active FTL is cached in memory here.
    let effective_lang = if let Some(ref lang) = effective_lang_base {
        lang.clone()
    } else if let Some(rc) = root_cid.as_deref() {
        i18n::resolve_active_lang(&config.kubo_rpc_url, rc)
            .await
            .unwrap_or_else(|| "nb".to_string())
    } else {
        "nb".to_string()
    };
    i18n::init(
        &effective_lang,
        &config.kubo_rpc_url,
        lang_cid.as_deref(),
        root_cid.as_deref(),
    )
    .await;

    // ── Optional IPFS publisher service ───────────────────────────────────────
    let ipfs_state = if ipfs_publisher_enabled {
        let messages = ipfs_messages.expect("ipfs inbox exists when publisher is enabled");
        info!("IPFS publisher service enabled");
        Some(ipfs::IpfsServiceState::new(
            messages,
            outbox_backoff_attempts,
        ))
    } else {
        info!("IPFS publisher service disabled (set ipfs_publisher: true in config to enable)");
        None
    };

    // ── Load entity plugins from IPFS ──────────────────────────────────────────
    // Channel for envelopes produced by entity plugins via ma_send/ma_reply.
    // Plugins send fire-and-forget; the main event loop drains and delivers.
    let (envelope_tx, envelope_rx) = tokio::sync::mpsc::channel::<(String, entity::SendEnvelope)>(
        plugin_envelope_queue_capacity,
    );
    let EntityStartup {
        entity_registry,
        kind_registry,
        startup_epoch,
        startup_iroh_node_id,
        sched: _sched,
        scheduler_manifest_writer,
        root_cid: new_root_cid,
    } = load_entities_and_scheduler(&config, &our_did, &endpoint, &envelope_tx, root_cid).await?;
    root_cid = new_root_cid;

    // ── Load named ACLs and groups into cache ──────────────────────────────────
    let acl_cache = acl::new_acl_cache();
    let group_cache = acl::new_group_cache();
    let manifest_owners =
        populate_acl_and_group_caches(&config, root_cid.as_deref(), &acl, &acl_cache, &group_cache)
            .await;

    // ── Signing key ────────────────────────────────────────────────────────────
    let signing_key = secrets
        .signing_key()
        .context("failed to derive signing key")?;

    // ── Resolve owners: config.yaml + --owner CLI + /grp/owners (union) ──
    // The manifest is the source of truth for owners; config.yaml and
    // --owner are additional seed sources reconciled into it here so a
    // restart never silently forgets owners set only via CRUD (see
    // AGENTS.md "Manifest is the source of truth; ACLs are derivatives").
    let ResolvedOwners {
        owners: resolved_owners,
        needs_manifest_sync: owners_need_manifest_sync,
    } = resolve_and_seed_owners(&config, &cli.owner, &manifest_owners, &acl).await;

    // ── Shared stats, manifest writer, and periodic DID republishing ──────────
    let FinalizedStartup {
        stats,
        shared_config,
        manifest_writer,
    } = finalize_startup(FinalizeArgs {
        config: &config,
        status_bind,
        acl: &acl,
        group_cache: &group_cache,
        entity_registry: &entity_registry,
        scheduler_manifest_writer: &scheduler_manifest_writer,
        our_did: &our_did,
        ma_base,
        root_cid,
        startup_iroh_node_id,
        startup_epoch,
        ipfs_publisher_enabled,
        resolved_owners,
        owners_need_manifest_sync,
        runtime_ipns_key,
        did_publish_interval_secs,
        did_publish_timeout_secs,
        ipns_publish,
    })
    .await?;

    info!(
        did = %our_did,
        endpoint_id = %endpoint.id(),
        kubo_rpc_url = %config.kubo_rpc_url,
        status_bind = %status_bind,
        "{}", i18n::t("started")
    );

    let remote_pin = bootstrap::runtime_remote_pin_config(&config);

    // ── Main event loop + graceful shutdown ─────────────────────────────────────
    eventloop::run(eventloop::RunArgs {
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
        runtime_slug: config.slug.clone(),
        remote_pin,
        did_publish_timeout_secs,
        ipns_publish,
        did_resolve,
        poll_ms,
    })
    .await
}

async fn populate_default_config_root(
    manifest_writer: &manifest::ManifestWriter,
    entity_registry: &plugin::EntityRegistry,
    stats: &status::SharedStats,
    kubo_rpc_url: &str,
    our_did: &str,
) {
    if !entity_registry.read().await.contains_key("root") {
        warn!("{}", i18n::t("default-config-root-no-root-entity"));
        return;
    }

    let Some(root_cid) = stats.read().await.root_cid.clone() else {
        warn!("{}", i18n::t("default-config-root-no-root-cid"));
        return;
    };
    match kubo::dag_get::<entity::RuntimeManifest>(kubo_rpc_url, &root_cid).await {
        Ok(manifest)
            if manifest.config.contains_key("root") && manifest.config.contains_key("start") =>
        {
            return;
        }
        Ok(_) => {}
        Err(e) => {
            warn!(error = %format!("{e:#}"), "{}", i18n::t("default-config-root-inspect-failed"));
            return;
        }
    }

    let default_root = format!("{our_did}#root");
    let default_start = format!("{our_did}#construct");
    match manifest_writer
        .mutate(move |m| {
            m.config
                .entry("root".to_string())
                .or_insert_with(|| serde_yaml::Value::String(default_root));
            m.config
                .entry("start".to_string())
                .or_insert_with(|| serde_yaml::Value::String(default_start));
            Ok(())
        })
        .await
    {
        Ok(cid) => info!(cid = %cid, "{}", i18n::t("default-config-root-populated")),
        Err(e) => {
            warn!(error = %format!("{e:#}"), "{}", i18n::t("default-config-root-populate-failed"));
        }
    }
}
