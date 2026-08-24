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

pub struct PeriodicDidPublishContext {
    pub stats: SharedStats,
    pub shared_config: std::sync::Arc<RwLock<Config>>,
    pub kubo_url: String,
    pub runtime_slug: String,
    pub ma_base: MaExtension,
    pub runtime_ipns_key: [u8; 32],
    pub bundle_path: PathBuf,
    pub passphrase: String,
    pub interval: Duration,
    pub cache_warm: Duration,
    pub timeout: Duration,
    pub ipns_publish: ipfs::IpnsPublishSettings,
}

pub fn spawn_periodic_did_publish(context: PeriodicDidPublishContext) {
    tokio::spawn(async move {
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
                    warn!(
                        root_cid = %latest_root_cid,
                        error = %err,
                        "{}",
                        crate::i18n::t("bootstrap-root-pin-update-failed")
                    );
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
            warn!(cid = %latest_root_cid, error = %err, "{}", crate::i18n::t("bootstrap-root-pin-update-failed"));
            return None;
        }
    };
    let mut bundle = match SecretBundle::load(&context.bundle_path, &context.passphrase) {
        Ok(bundle) => bundle,
        Err(err) => {
            error!(error = %format!("{err:#}"), "{}", crate::i18n::t("bootstrap-root-pin-update-failed"));
            return None;
        }
    };
    crate::startup::qa_prepare_bundle_timestamps_for_publish(&mut bundle);
    let ma = context
        .ma_base
        .clone()
        .extra("runtime", Ipld::Link(runtime_cid));
    let document = match bundle.build_document(ma) {
        Ok(document) => document,
        Err(err) => {
            error!(error = %format!("{err:#}"), "{}", crate::i18n::t("bootstrap-root-pin-update-failed"));
            return None;
        }
    };
    let doc_cbor = match document.encode() {
        Ok(bytes) => bytes,
        Err(err) => {
            error!(error = %format!("{err:#}"), "{}", crate::i18n::t("bootstrap-root-pin-update-failed"));
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
    let config = context.shared_config.read().await;
    let remote_pin = crate::bootstrap::runtime_remote_pin_config(&config);
    let pin_overwrite = config.pin_overwrite;
    drop(config);
    let publish = tokio::time::timeout(
        context.timeout,
        ipfs::do_publish_own_document(
            context.kubo_url.clone(),
            context.runtime_slug.clone(),
            doc_cbor,
            ipns_key,
            context.ipns_publish,
            remote_pin,
            pin_overwrite,
        ),
    )
    .await;
    match publish {
        Ok(Ok(())) => {
            info!(runtime_cid = %latest_root_cid, cid_changed, "{}", crate::i18n::t("bootstrap-runtime-manifest-published"));
            true
        }
        Ok(Err(err)) => {
            error!(runtime_cid = %latest_root_cid, error = %format!("{err:#}"), "{}", crate::i18n::t("bootstrap-runtime-manifest-published"));
            false
        }
        Err(_) => {
            error!(runtime_cid = %latest_root_cid, "{}", crate::i18n::t("bootstrap-root-pin-update-failed"));
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
            info!(runtime_cid = %latest_root_cid, cid_changed, "{}", crate::i18n::t("bootstrap-root-pin-update-failed"));
            true
        }
        Ok(Err(err)) => {
            error!(runtime_cid = %latest_root_cid, error = %format!("{err:#}"), "{}", crate::i18n::t("bootstrap-root-pin-update-failed"));
            false
        }
        Err(_) => {
            error!(runtime_cid = %latest_root_cid, "{}", crate::i18n::t("bootstrap-root-pin-update-failed"));
            false
        }
    }
}
