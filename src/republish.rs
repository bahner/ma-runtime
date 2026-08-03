//! Periodic DID-document republishing task.
//!
//! Republishes the runtime's own DID document (and `runtime_ipns` root) from the
//! in-memory runtime head: immediately when the root CID changes, otherwise at
//! most once per cache-warm interval.  Split out of `main.rs` to keep the entry
//! point focused on orchestration.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use cid::Cid;
use ma_core::config::{Config, SecretBundle};
use ma_core::{Ipld, MaExtension};
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use crate::ipfs;
use crate::status::SharedStats;

struct PeriodicDidPublishContext {
    stats: SharedStats,
    shared_config: std::sync::Arc<RwLock<Config>>,
    kubo_url: String,
    runtime_slug: String,
    ma_base: MaExtension,
    runtime_ipns_key: [u8; 32],
    bundle_path: PathBuf,
    passphrase: String,
    interval: Duration,
    cache_warm: Duration,
    timeout: Duration,
    ipns_publish: ipfs::IpnsPublishSettings,
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_periodic_did_publish(
    refresh_stats: SharedStats,
    refresh_config: std::sync::Arc<RwLock<Config>>,
    refresh_kubo_url: String,
    refresh_runtime_slug: String,
    refresh_ma_base: MaExtension,
    refresh_runtime_ipns_key: [u8; 32],
    refresh_bundle_path: PathBuf,
    refresh_passphrase: String,
    did_publish_interval_secs: u64,
    did_publish_cache_warm_secs: u64,
    did_publish_timeout_secs: u64,
    ipns_publish: ipfs::IpnsPublishSettings,
) {
    tokio::spawn(async move {
        let context = PeriodicDidPublishContext {
            stats: refresh_stats,
            shared_config: refresh_config,
            kubo_url: refresh_kubo_url,
            runtime_slug: refresh_runtime_slug,
            ma_base: refresh_ma_base,
            runtime_ipns_key: refresh_runtime_ipns_key,
            bundle_path: refresh_bundle_path,
            passphrase: refresh_passphrase,
            interval: Duration::from_secs(did_publish_interval_secs),
            cache_warm: Duration::from_secs(did_publish_cache_warm_secs),
            timeout: Duration::from_secs(did_publish_timeout_secs),
            ipns_publish,
        };
        let mut ticker = tokio::time::interval(context.interval);
        let mut last_published_cid: Option<String> = None;
        let mut last_published_at = Instant::now()
            .checked_sub(context.cache_warm)
            .unwrap_or_else(Instant::now);
        loop {
            ticker.tick().await;
            let Some(latest_root_cid) = context.stats.read().await.root_cid.clone() else {
                continue;
            };
            let cid_changed = last_published_cid.as_deref() != Some(latest_root_cid.as_str());
            let cache_warm_elapsed = last_published_at.elapsed() >= context.cache_warm;
            if !cid_changed && !cache_warm_elapsed {
                continue;
            }
            if publish_current_root(&context, &latest_root_cid, cid_changed).await {
                let mut config = context.shared_config.write().await;
                if let Err(err) = crate::startup::persist_root_cid(&mut config, &latest_root_cid) {
                    warn!(root_cid = %latest_root_cid, error = %err, "failed to persist root_cid after periodic republish");
                }
                last_published_cid = Some(latest_root_cid);
                last_published_at = Instant::now();
            }
        }
    });
}

async fn publish_current_root(
    context: &PeriodicDidPublishContext,
    latest_root_cid: &str,
    cid_changed: bool,
) -> bool {
    let Some((doc_cbor, ipns_key)) = build_did_publish_payload(context, latest_root_cid) else {
        return false;
    };
    let did_ok =
        publish_did_document(context, latest_root_cid, cid_changed, doc_cbor, ipns_key).await;
    let ipns_ok = publish_runtime_ipns(context, latest_root_cid, cid_changed).await;

    did_ok && ipns_ok
}

fn build_did_publish_payload(
    context: &PeriodicDidPublishContext,
    latest_root_cid: &str,
) -> Option<(Vec<u8>, Vec<u8>)> {
    let runtime_cid = match Cid::try_from(latest_root_cid) {
        Ok(cid) => cid,
        Err(err) => {
            warn!(cid = %latest_root_cid, error = %err, "invalid root_cid for periodic DID publish");
            return None;
        }
    };
    let bundle = match SecretBundle::load(&context.bundle_path, &context.passphrase) {
        Ok(bundle) => bundle,
        Err(err) => {
            error!(error = %format!("{err:#}"), "periodic DID load secret bundle failed");
            return None;
        }
    };
    let ma = context
        .ma_base
        .clone()
        .extra("runtime", Ipld::Link(runtime_cid));
    let document = match bundle.build_document(ma) {
        Ok(document) => document,
        Err(err) => {
            error!(error = %format!("{err:#}"), "periodic DID build failed");
            return None;
        }
    };
    let doc_cbor = match document.encode() {
        Ok(bytes) => bytes,
        Err(err) => {
            error!(error = %format!("{err:#}"), "periodic DID encode failed");
            return None;
        }
    };
    let ipns_key = bundle.ipns_secret_key.to_vec();

    Some((doc_cbor, ipns_key))
}

async fn publish_did_document(
    context: &PeriodicDidPublishContext,
    latest_root_cid: &str,
    cid_changed: bool,
    doc_cbor: Vec<u8>,
    ipns_key: Vec<u8>,
) -> bool {
    let publish = tokio::time::timeout(
        context.timeout,
        ipfs::do_publish_own_document(
            context.kubo_url.clone(),
            context.runtime_slug.clone(),
            doc_cbor,
            ipns_key,
            context.ipns_publish,
        ),
    )
    .await;
    match publish {
        Ok(Ok(())) => {
            info!(runtime_cid = %latest_root_cid, cid_changed, "periodic DID publish succeeded");
            true
        }
        Ok(Err(err)) => {
            error!(runtime_cid = %latest_root_cid, error = %format!("{err:#}"), "periodic DID publish failed");
            false
        }
        Err(_) => {
            error!(runtime_cid = %latest_root_cid, "periodic DID publish timed out");
            false
        }
    }
}

async fn publish_runtime_ipns(
    context: &PeriodicDidPublishContext,
    latest_root_cid: &str,
    cid_changed: bool,
) -> bool {
    match tokio::time::timeout(
        context.timeout,
        ipfs::publish_runtime_root_cid(
            &context.kubo_url,
            &context.runtime_slug,
            &context.runtime_ipns_key,
            latest_root_cid,
            context.ipns_publish,
        ),
    )
    .await
    {
        Ok(Ok(_)) => {
            info!(runtime_cid = %latest_root_cid, cid_changed, "periodic runtime_ipns publish succeeded");
            true
        }
        Ok(Err(err)) => {
            error!(runtime_cid = %latest_root_cid, error = %format!("{err:#}"), "periodic runtime_ipns publish failed");
            false
        }
        Err(_) => {
            error!(runtime_cid = %latest_root_cid, "periodic runtime_ipns publish timed out");
            false
        }
    }
}
