//! Serialised manifest writer.
//!
//! Every runtime-phase mutation of the IPFS [`RuntimeManifest`] goes through a
//! single [`ManifestWriter`].  It owns the authoritative root CID behind an
//! async mutex, so the read-modify-write cycle (`dag_get` → mutate → `dag_put` →
//! local and optional remote pin replacement) is serialised.
//!
//! This eliminates the last-writer-wins race that occurred when concurrent CRUD
//! sets and `ma_create_entity` calls each read the same old root CID and raced
//! to publish — previously dropping all but the last entity from the manifest
//! on a crash-restart.
//!
//! Startup (`bootstrap::load_entities`) and shutdown (`save_all_entity_states`)
//! run outside the concurrent window and do not use the writer; they mutate the
//! manifest directly.  The writer must therefore be spawned *after* startup has
//! settled the initial root CID, and all concurrent mutations must go through it.

use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::Result;
use ma_core::config::{Config, RemotePinConfig};
use tokio::sync::{Mutex, MutexGuard, RwLock};
use tracing::{info, warn};

use crate::entity::{EntityNode, IpldLink, RuntimeManifest};
use crate::status::SharedStats;

pub type ManifestMutationFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

/// Cloneable handle to the serialised manifest writer.
#[derive(Clone)]
pub struct ManifestWriter {
    inner: Arc<Inner>,
}

struct Inner {
    /// Authoritative current root CID.  Held across the whole read-modify-write
    /// so mutations never race on a stale base.
    current: Mutex<String>,
    kubo_url: String,
    stats: SharedStats,
    config_path: Option<PathBuf>,
    shared_config: Option<Arc<RwLock<Config>>>,
}

impl Inner {
    async fn remote_pin_config(&self) -> Option<RemotePinConfig> {
        let shared_config = self.shared_config.as_ref()?;
        let config = shared_config.read().await;
        let default_name = crate::bootstrap::default_remote_root_pin_name(&config.slug);
        match config.remote_pin_config_with_default_name(default_name) {
            Ok(remote) => remote,
            Err(err) => {
                warn!(error = %err, "remote root pinning is misconfigured");
                None
            }
        }
    }
}

impl ManifestWriter {
    /// Create a writer rooted at `initial_cid`.
    pub fn new(
        initial_cid: String,
        kubo_url: String,
        stats: SharedStats,
        config_path: Option<PathBuf>,
        shared_config: Option<Arc<RwLock<Config>>>,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                current: Mutex::new(initial_cid),
                kubo_url,
                stats,
                config_path,
                shared_config,
            }),
        }
    }

    async fn persist_root_cid(&self, root_cid: &str) {
        if let Some(ref shared_config) = self.inner.shared_config {
            let mut config = shared_config.write().await;
            config.extra.insert(
                serde_yaml::Value::String("root_cid".to_string()),
                serde_yaml::Value::String(root_cid.to_string()),
            );
            if let Err(e) = config.save() {
                warn!(root_cid = %root_cid, error = %e, "failed to persist root_cid to config.yaml");
            }
        } else if let Some(ref path) = self.inner.config_path {
            if let Err(e) = crate::startup::persist_root_cid_to_config(path, root_cid) {
                warn!(root_cid = %root_cid, error = %e, "failed to persist root_cid to config.yaml");
            }
        }
    }

    async fn publish_manifest_update(
        &self,
        guard: &mut MutexGuard<'_, String>,
        old_cid: String,
        manifest: &RuntimeManifest,
    ) -> Result<String> {
        let inner = &self.inner;
        let new_cid = crate::kubo::dag_put(&inner.kubo_url, manifest).await?;
        if let Err(e) = crate::kubo::pin_update(&inner.kubo_url, &old_cid, &new_cid).await {
            warn!(old = %old_cid, new = %new_cid, error = %e, "manifest pin_update failed");
        }
        self.replace_remote_root_pin(Some(&old_cid), &new_cid).await;

        guard.clone_from(&new_cid);
        inner.stats.write().await.root_cid = Some(new_cid.clone());
        self.persist_root_cid(&new_cid).await;
        Ok(new_cid)
    }

    async fn replace_remote_root_pin(&self, old_cid: Option<&str>, new_cid: &str) {
        let inner = &self.inner;
        let Some(remote) = inner.remote_pin_config().await else {
            return;
        };
        match ma_core::remote_pin_replace(&inner.kubo_url, &remote, old_cid, new_cid).await {
            Ok(outcome) => {
                if let Some(error) = outcome.previous_remove_error {
                    warn!(
                        old = old_cid.unwrap_or(""),
                        new = %new_cid,
                        service = %remote.service,
                        name = %remote.name,
                        error = %error,
                        "remote root pin replacement left the previous pin in place"
                    );
                } else {
                    info!(
                        old = old_cid.unwrap_or(""),
                        new = %new_cid,
                        service = %remote.service,
                        name = %remote.name,
                        "remote root pin replaced"
                    );
                }
            }
            Err(err) => {
                warn!(
                    old = old_cid.unwrap_or(""),
                    new = %new_cid,
                    service = %remote.service,
                    name = %remote.name,
                    error = %err,
                    "remote root pin replacement failed"
                );
            }
        }
    }

    /// Apply `f` to the current manifest, publish it, swap the pin, and return
    /// the new root CID.  Mutations are fully serialised: each observes the
    /// result of the previous one.
    ///
    /// `f` runs synchronously between the fetch and the publish; if it returns
    /// `Err`, nothing is published and the root CID is unchanged.
    // The guard is held across the whole read-modify-write on purpose — that is
    // exactly what serialises concurrent mutations.
    #[allow(clippy::significant_drop_tightening)]
    pub async fn mutate<F>(&self, f: F) -> Result<String>
    where
        F: FnOnce(&mut RuntimeManifest) -> Result<()>,
    {
        let inner = &self.inner;
        let mut guard = inner.current.lock().await;
        let old_cid = guard.clone();

        let mut manifest: RuntimeManifest = crate::kubo::dag_get(&inner.kubo_url, &old_cid).await?;
        f(&mut manifest)?;
        self.publish_manifest_update(&mut guard, old_cid, &manifest)
            .await
    }

    /// Async variant of [`Self::mutate`] for mutations that must validate
    /// against IPFS while the manifest writer lock is held.
    #[allow(clippy::significant_drop_tightening)]
    pub async fn mutate_async<F, T>(&self, f: F) -> Result<(String, T)>
    where
        F: for<'a> FnOnce(&'a mut RuntimeManifest) -> ManifestMutationFuture<'a, T>,
    {
        let inner = &self.inner;
        let mut guard = inner.current.lock().await;
        let old_cid = guard.clone();

        let mut manifest: RuntimeManifest = crate::kubo::dag_get(&inner.kubo_url, &old_cid).await?;
        let value = f(&mut manifest).await?;
        let new_cid = self
            .publish_manifest_update(&mut guard, old_cid, &manifest)
            .await?;
        Ok((new_cid, value))
    }

    /// Publish an updated entity node whose `state` points at `state_cid`, then
    /// publish a manifest that points at that updated entity node.
    #[allow(clippy::significant_drop_tightening)]
    pub async fn set_entity_state(&self, fragment: &str, state_cid: &str) -> Result<String> {
        let inner = &self.inner;
        let mut guard = inner.current.lock().await;
        let old_cid = guard.clone();

        let mut manifest: RuntimeManifest = crate::kubo::dag_get(&inner.kubo_url, &old_cid).await?;
        let entity_link = manifest
            .entities
            .get(fragment)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("entity '{fragment}' is not in the manifest"))?;
        let mut entity_node: EntityNode =
            crate::kubo::dag_get(&inner.kubo_url, &entity_link.cid).await?;
        entity_node.state = Some(IpldLink::new(state_cid));
        let entity_cid = crate::kubo::dag_put(&inner.kubo_url, &entity_node).await?;
        manifest
            .entities
            .insert(fragment.to_string(), IpldLink::new(&entity_cid));

        self.publish_manifest_update(&mut guard, old_cid, &manifest)
            .await
    }

    /// Publish an updated entity node with a new per-entity behaviour link,
    /// then publish a manifest that points at that updated entity node.
    #[allow(clippy::significant_drop_tightening)]
    pub async fn set_entity_behaviour(
        &self,
        fragment: &str,
        behaviour_cid: Option<&str>,
    ) -> Result<String> {
        let inner = &self.inner;
        let mut guard = inner.current.lock().await;
        let old_cid = guard.clone();

        let mut manifest: RuntimeManifest = crate::kubo::dag_get(&inner.kubo_url, &old_cid).await?;
        let entity_link = manifest
            .entities
            .get(fragment)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("entity '{fragment}' is not in the manifest"))?;
        let mut entity_node: EntityNode =
            crate::kubo::dag_get(&inner.kubo_url, &entity_link.cid).await?;
        entity_node.behaviour = behaviour_cid.map(IpldLink::new);
        entity_node.initialised = true;
        let entity_cid = crate::kubo::dag_put(&inner.kubo_url, &entity_node).await?;
        manifest
            .entities
            .insert(fragment.to_string(), IpldLink::new(&entity_cid));

        self.publish_manifest_update(&mut guard, old_cid, &manifest)
            .await
    }

    /// Publish an updated entity node carrying the latest recoverable reload
    /// error. `None` clears a previous reload error after a later successful
    /// reload.
    #[allow(clippy::significant_drop_tightening)]
    pub async fn set_entity_reload_error(
        &self,
        fragment: &str,
        reason: Option<&str>,
    ) -> Result<String> {
        let inner = &self.inner;
        let mut guard = inner.current.lock().await;
        let old_cid = guard.clone();

        let mut manifest: RuntimeManifest = crate::kubo::dag_get(&inner.kubo_url, &old_cid).await?;
        let entity_link = manifest
            .entities
            .get(fragment)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("entity '{fragment}' is not in the manifest"))?;
        let mut entity_node: EntityNode =
            crate::kubo::dag_get(&inner.kubo_url, &entity_link.cid).await?;
        let next_error = reason.map(ToString::to_string);
        if entity_node.reload_error == next_error {
            return Ok(old_cid);
        }
        entity_node.reload_error = next_error;
        let entity_cid = crate::kubo::dag_put(&inner.kubo_url, &entity_node).await?;
        manifest
            .entities
            .insert(fragment.to_string(), IpldLink::new(&entity_cid));

        self.publish_manifest_update(&mut guard, old_cid, &manifest)
            .await
    }
}

impl std::fmt::Debug for ManifestWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManifestWriter").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::sync::RwLock;

    use super::ManifestWriter;
    use crate::entity::{IpldLink, RuntimeManifest};
    use crate::status::Stats;
    use crate::testkubo::MockKubo;

    /// Publish an empty manifest to the mock and return `(root_cid, stats)`.
    async fn seed(kubo: &MockKubo) -> (String, Arc<RwLock<Stats>>) {
        let initial = crate::kubo::dag_put(kubo.url(), &RuntimeManifest::default())
            .await
            .unwrap();
        let stats = Arc::new(RwLock::new(Stats {
            root_cid: Some(initial.clone()),
            ..Default::default()
        }));
        (initial, stats)
    }

    // Directly reproduces the last-writer-wins race the writer was built to fix:
    // 25 concurrent inserts must all survive.  Before serialisation each task
    // read the same base CID and clobbered the others — the final manifest would
    // hold a single entity.
    #[tokio::test]
    async fn concurrent_mutations_are_not_lost() {
        let kubo = MockKubo::start().await;
        let (initial, stats) = seed(&kubo).await;
        let writer =
            ManifestWriter::new(initial, kubo.url().to_string(), stats.clone(), None, None);

        let mut handles = Vec::new();
        for i in 0..25u32 {
            let w = writer.clone();
            handles.push(tokio::spawn(async move {
                w.mutate(move |m| {
                    m.entities.insert(format!("e{i}"), IpldLink::new("bafyx"));
                    Ok(())
                })
                .await
            }));
        }
        for h in handles {
            h.await.unwrap().unwrap();
        }

        let final_cid = stats.read().await.root_cid.clone().unwrap();
        let m: RuntimeManifest = crate::kubo::dag_get(kubo.url(), &final_cid).await.unwrap();
        assert_eq!(m.entities.len(), 25, "every concurrent insert must survive");
    }

    #[tokio::test]
    async fn sequential_mutations_chain() {
        let kubo = MockKubo::start().await;
        let (initial, stats) = seed(&kubo).await;
        let writer =
            ManifestWriter::new(initial, kubo.url().to_string(), stats.clone(), None, None);

        for name in ["a", "b", "c"] {
            let name = name.to_string();
            writer
                .mutate(move |m| {
                    m.entities.insert(name, IpldLink::new("bafyx"));
                    Ok(())
                })
                .await
                .unwrap();
        }

        let cid = stats.read().await.root_cid.clone().unwrap();
        let m: RuntimeManifest = crate::kubo::dag_get(kubo.url(), &cid).await.unwrap();
        assert_eq!(m.entities.len(), 3);
    }

    #[tokio::test]
    async fn set_entity_state_updates_entity_node_and_manifest_root() {
        let kubo = MockKubo::start().await;
        let entity_node = crate::entity::EntityNode {
            kind: "/ma/test/0.0.1".to_string(),
            behaviour: None,
            acl: String::new(),
            state: None,
            parent: None,
            label: None,
            attributes: std::collections::BTreeMap::new(),
            init: None,
            initialised: false,
            reload_error: None,
        };
        let entity_cid = crate::kubo::dag_put(kubo.url(), &entity_node)
            .await
            .unwrap();
        let mut manifest = RuntimeManifest::default();
        manifest
            .entities
            .insert("room".to_string(), IpldLink::new(entity_cid));
        let initial = crate::kubo::dag_put(kubo.url(), &manifest).await.unwrap();
        let stats = Arc::new(RwLock::new(Stats {
            root_cid: Some(initial.clone()),
            ..Default::default()
        }));
        let writer =
            ManifestWriter::new(initial, kubo.url().to_string(), stats.clone(), None, None);

        let root_cid = writer.set_entity_state("room", "bafystate").await.unwrap();

        assert_eq!(
            stats.read().await.root_cid.as_deref(),
            Some(root_cid.as_str())
        );
        let updated_manifest: RuntimeManifest =
            crate::kubo::dag_get(kubo.url(), &root_cid).await.unwrap();
        let updated_link = updated_manifest.entities.get("room").unwrap();
        let updated_node: crate::entity::EntityNode =
            crate::kubo::dag_get(kubo.url(), &updated_link.cid)
                .await
                .unwrap();
        assert_eq!(updated_node.state.unwrap().cid, "bafystate");
    }

    #[tokio::test]
    async fn set_entity_behaviour_marks_entity_initialised() {
        let kubo = MockKubo::start().await;
        let entity_node = crate::entity::EntityNode {
            kind: "/ma/room/0.0.1".to_string(),
            behaviour: None,
            acl: "open".to_string(),
            state: None,
            parent: None,
            label: None,
            attributes: std::collections::BTreeMap::new(),
            init: Some("(set-prop! \"name\" \"Construct\")".to_string()),
            initialised: false,
            reload_error: None,
        };
        let entity_cid = crate::kubo::dag_put(kubo.url(), &entity_node)
            .await
            .unwrap();
        let mut manifest = RuntimeManifest::default();
        manifest
            .entities
            .insert("construct".to_string(), IpldLink::new(entity_cid));
        let initial = crate::kubo::dag_put(kubo.url(), &manifest).await.unwrap();
        let stats = Arc::new(RwLock::new(Stats {
            root_cid: Some(initial.clone()),
            ..Default::default()
        }));
        let writer =
            ManifestWriter::new(initial, kubo.url().to_string(), stats.clone(), None, None);

        let root_cid = writer
            .set_entity_behaviour("construct", Some("bafybehaviour"))
            .await
            .unwrap();

        let updated_manifest: RuntimeManifest =
            crate::kubo::dag_get(kubo.url(), &root_cid).await.unwrap();
        let updated_link = updated_manifest.entities.get("construct").unwrap();
        let updated_node: crate::entity::EntityNode =
            crate::kubo::dag_get(kubo.url(), &updated_link.cid)
                .await
                .unwrap();
        assert!(updated_node.initialised);
        assert_eq!(updated_node.behaviour.unwrap().cid, "bafybehaviour");
    }

    #[tokio::test]
    async fn set_entity_reload_error_sets_and_clears_error() {
        let kubo = MockKubo::start().await;
        let entity_node = crate::entity::EntityNode {
            kind: "/ma/room/0.0.1".to_string(),
            behaviour: None,
            acl: "open".to_string(),
            state: None,
            parent: None,
            label: None,
            attributes: std::collections::BTreeMap::new(),
            init: None,
            initialised: true,
            reload_error: None,
        };
        let entity_cid = crate::kubo::dag_put(kubo.url(), &entity_node)
            .await
            .unwrap();
        let mut manifest = RuntimeManifest::default();
        manifest
            .entities
            .insert("construct".to_string(), IpldLink::new(entity_cid));
        let initial = crate::kubo::dag_put(kubo.url(), &manifest).await.unwrap();
        let stats = Arc::new(RwLock::new(Stats {
            root_cid: Some(initial.clone()),
            ..Default::default()
        }));
        let writer = ManifestWriter::new(initial, kubo.url().to_string(), stats, None, None);

        let failed_root = writer
            .set_entity_reload_error("construct", Some("kind fetch failed"))
            .await
            .unwrap();
        let failed_manifest: RuntimeManifest = crate::kubo::dag_get(kubo.url(), &failed_root)
            .await
            .unwrap();
        let failed_link = failed_manifest.entities.get("construct").unwrap();
        let failed_node: crate::entity::EntityNode =
            crate::kubo::dag_get(kubo.url(), &failed_link.cid)
                .await
                .unwrap();
        assert_eq!(
            failed_node.reload_error.as_deref(),
            Some("kind fetch failed")
        );

        let cleared_root = writer
            .set_entity_reload_error("construct", None)
            .await
            .unwrap();
        let cleared_manifest: RuntimeManifest = crate::kubo::dag_get(kubo.url(), &cleared_root)
            .await
            .unwrap();
        let cleared_link = cleared_manifest.entities.get("construct").unwrap();
        let cleared_node: crate::entity::EntityNode =
            crate::kubo::dag_get(kubo.url(), &cleared_link.cid)
                .await
                .unwrap();
        assert_eq!(cleared_node.reload_error, None);
    }

    #[tokio::test]
    async fn failed_mutation_does_not_advance_root() {
        let kubo = MockKubo::start().await;
        let (initial, stats) = seed(&kubo).await;
        let writer = ManifestWriter::new(
            initial.clone(),
            kubo.url().to_string(),
            stats.clone(),
            None,
            None,
        );

        let result = writer.mutate(|_m| Err(anyhow::anyhow!("boom"))).await;
        assert!(result.is_err());
        assert_eq!(
            stats.read().await.root_cid.as_deref(),
            Some(initial.as_str()),
            "a failed mutation must not advance the root CID"
        );
    }
}
