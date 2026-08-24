use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use ma_core::{Did, DidDocumentResolver, Document};
use tokio::sync::watch;
use tokio::sync::Mutex;

pub const DEFAULT_DID_REFRESH_INTERVAL_SECS: u64 = 86_400;

#[derive(Default)]
pub struct DocumentCache {
    documents: Mutex<HashMap<String, Document>>,
}

impl DocumentCache {
    async fn get(&self, did: &str) -> ma_core::Result<Option<Document>> {
        let base = Did::try_from(did)?.base_id();
        Ok(self.documents.lock().await.get(&base).cloned())
    }

    async fn insert_published(&self, did: &str, document: Document) -> ma_core::Result<()> {
        let base = Did::try_from(did)?.base_id();
        self.documents.lock().await.insert(base, document);
        Ok(())
    }

    async fn merge_resolved(&self, did: &str, document: Document) -> ma_core::Result<Document> {
        let base = Did::try_from(did)?.base_id();
        let mut documents = self.documents.lock().await;
        match documents.get(&base) {
            Some(cached) if cached.updated_at >= document.updated_at => Ok(cached.clone()),
            _ => {
                documents.insert(base, document.clone());
                drop(documents);
                Ok(document)
            }
        }
    }

    async fn dids(&self) -> Vec<String> {
        self.documents.lock().await.keys().cloned().collect()
    }
}

pub struct RuntimeDidResolver {
    cache: DocumentCache,
    resolver: Arc<dyn DidDocumentResolver>,
    refresh_resolver: Arc<dyn DidDocumentResolver>,
}

impl RuntimeDidResolver {
    #[cfg(test)]
    pub fn from_resolver(resolver: Arc<dyn DidDocumentResolver>) -> Self {
        Self::new(Arc::clone(&resolver), resolver)
    }

    pub fn new(
        resolver: Arc<dyn DidDocumentResolver>,
        refresh_resolver: Arc<dyn DidDocumentResolver>,
    ) -> Self {
        Self {
            cache: DocumentCache::default(),
            resolver,
            refresh_resolver,
        }
    }

    /// Any document the runtime has already received and validated locally is
    /// authoritative, even if it has not yet reached the public IPFS/IPNS layer
    /// or a gateway refresh is temporarily unavailable.
    pub async fn insert_known(&self, did: &str, document: Document) -> ma_core::Result<()> {
        self.cache.insert_published(did, document).await
    }

    pub async fn refresh(&self, did: &str) -> ma_core::Result<Document> {
        let document = self.refresh_resolver.resolve(did).await?;
        self.cache.merge_resolved(did, document).await
    }

    pub async fn cached_dids(&self) -> Vec<String> {
        self.cache.dids().await
    }
}

pub fn spawn_refresh_worker(
    resolver: Arc<RuntimeDidResolver>,
    mut interval_secs: watch::Receiver<u64>,
) {
    tokio::spawn(async move {
        let mut refresh_now = *interval_secs.borrow() > 0;
        loop {
            if refresh_now {
                refresh_all(&resolver).await;
            }

            let interval = *interval_secs.borrow();
            if interval == 0 {
                if interval_secs.changed().await.is_err() {
                    return;
                }
                refresh_now = *interval_secs.borrow() > 0;
                continue;
            }

            tokio::select! {
                () = tokio::time::sleep(std::time::Duration::from_secs(interval)) => {
                    refresh_now = true;
                }
                changed = interval_secs.changed() => {
                    if changed.is_err() {
                        return;
                    }
                    refresh_now = *interval_secs.borrow() > 0 && interval == 0;
                }
            }
        }
    });
}

async fn refresh_all(resolver: &Arc<RuntimeDidResolver>) {
    const CONCURRENCY: usize = 8;

    let mut tasks = tokio::task::JoinSet::new();
    for did in resolver.cached_dids().await {
        while tasks.len() >= CONCURRENCY {
            let _ = tasks.join_next().await;
        }
        let resolver = Arc::clone(resolver);
        tasks.spawn(async move {
            let _ = resolver.refresh(&did).await;
        });
    }
    while tasks.join_next().await.is_some() {}
}

#[async_trait]
impl DidDocumentResolver for RuntimeDidResolver {
    async fn resolve(&self, did: &str) -> ma_core::Result<Document> {
        if let Some(document) = self.cache.get(did).await? {
            return Ok(document);
        }
        let document = self.resolver.resolve(did).await?;
        self.cache.merge_resolved(did, document).await
    }

    fn set_cache_ttls(&self, positive_ttl: std::time::Duration, negative_ttl: std::time::Duration) {
        self.resolver.set_cache_ttls(positive_ttl, negative_ttl);
    }

    fn cache_ttls(&self) -> Option<(std::time::Duration, std::time::Duration)> {
        self.resolver.cache_ttls()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use ma_core::{Did, Document};
    use tokio::sync::Notify;

    use super::*;

    const TEST_DID: &str = "did:ma:k51qzi5uqu5dj9807pbuod1pplf0vxh8m4lfy3ewl9qbm2s8dsf9ugdf9gedhr";

    struct StubResolver {
        document: Document,
        calls: AtomicUsize,
        called: Notify,
    }

    struct RejectingResolver;

    #[async_trait]
    impl DidDocumentResolver for StubResolver {
        async fn resolve(&self, _did: &str) -> ma_core::Result<Document> {
            self.calls.fetch_add(1, Ordering::Relaxed);
            self.called.notify_one();
            Ok(self.document.clone())
        }
    }

    #[async_trait]
    impl DidDocumentResolver for RejectingResolver {
        async fn resolve(&self, _did: &str) -> ma_core::Result<Document> {
            Err(ma_core::Error::Resolution {
                did: _did.to_string(),
                detail: "remote DID resolution intentionally failed".to_string(),
            })
        }
    }

    fn document(updated_at: &str) -> Document {
        let identity = Did::try_from(TEST_DID).unwrap();
        let mut document = Document::new(&identity, &identity);
        document.updated_at = updated_at.to_string();
        document
    }

    fn stub(document: Document) -> Arc<StubResolver> {
        Arc::new(StubResolver {
            document,
            calls: AtomicUsize::new(0),
            called: Notify::new(),
        })
    }

    #[tokio::test]
    async fn cache_hit_skips_resolver_and_normalises_fragment() {
        let normal = stub(document("2026-08-24T10:00:00Z"));
        let refresh = stub(document("2026-08-24T10:00:00Z"));
        let resolver = RuntimeDidResolver::new(normal.clone(), refresh);
        resolver
            .insert_known(&format!("{TEST_DID}#rpc"), document("2026-08-24T11:00:00Z"))
            .await
            .unwrap();

        let document = resolver.resolve(TEST_DID).await.unwrap();

        assert_eq!(document.updated_at, "2026-08-24T11:00:00Z");
        assert_eq!(normal.calls.load(Ordering::Relaxed), 0);
    }

    #[tokio::test]
    async fn known_local_document_is_used_even_when_remote_resolution_fails() {
        let normal = Arc::new(RejectingResolver);
        let refresh = Arc::new(RejectingResolver);
        let resolver = RuntimeDidResolver::new(normal, refresh);
        let received = document("2026-08-24T11:00:00Z");
        resolver.insert_known(TEST_DID, received.clone()).await.unwrap();

        let resolved = resolver.resolve(TEST_DID).await.unwrap();

        assert_eq!(resolved.updated_at, "2026-08-24T11:00:00Z");
    }

    #[tokio::test]
    async fn refresh_only_replaces_with_newer_document() {
        let normal = stub(document("2026-08-24T10:00:00Z"));
        let refresh = stub(document("2026-08-24T09:00:00Z"));
        let resolver = RuntimeDidResolver::new(normal, refresh.clone());
        resolver
            .insert_known(TEST_DID, document("2026-08-24T10:00:00Z"))
            .await
            .unwrap();

        let document = resolver.refresh(TEST_DID).await.unwrap();

        assert_eq!(document.updated_at, "2026-08-24T10:00:00Z");
        assert_eq!(refresh.calls.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn refresh_replaces_with_newer_document() {
        let normal = stub(document("2026-08-24T10:00:00Z"));
        let refresh = stub(document("2026-08-24T11:00:00Z"));
        let resolver = RuntimeDidResolver::new(normal, refresh);
        resolver
            .insert_known(TEST_DID, document("2026-08-24T10:00:00Z"))
            .await
            .unwrap();

        let document = resolver.refresh(TEST_DID).await.unwrap();

        assert_eq!(document.updated_at, "2026-08-24T11:00:00Z");
    }

    #[tokio::test]
    async fn worker_refreshes_immediately_when_enabled() {
        let normal = stub(document("2026-08-24T10:00:00Z"));
        let refresh = stub(document("2026-08-24T11:00:00Z"));
        let resolver = Arc::new(RuntimeDidResolver::new(normal, refresh.clone()));
        resolver
            .insert_known(TEST_DID, document("2026-08-24T10:00:00Z"))
            .await
            .unwrap();
        let (sender, receiver) = watch::channel(DEFAULT_DID_REFRESH_INTERVAL_SECS);

        spawn_refresh_worker(resolver, receiver);
        tokio::time::timeout(std::time::Duration::from_secs(1), refresh.called.notified())
            .await
            .unwrap();

        assert_eq!(refresh.calls.load(Ordering::Relaxed), 1);
        drop(sender);
    }

    #[tokio::test]
    async fn worker_stays_disabled_until_reenabled() {
        let normal = stub(document("2026-08-24T10:00:00Z"));
        let refresh = stub(document("2026-08-24T11:00:00Z"));
        let resolver = Arc::new(RuntimeDidResolver::new(normal, refresh.clone()));
        resolver
            .insert_known(TEST_DID, document("2026-08-24T10:00:00Z"))
            .await
            .unwrap();
        let (sender, receiver) = watch::channel(0);

        spawn_refresh_worker(resolver, receiver);
        tokio::task::yield_now().await;
        assert_eq!(refresh.calls.load(Ordering::Relaxed), 0);

        sender.send_replace(DEFAULT_DID_REFRESH_INTERVAL_SECS);
        tokio::time::timeout(std::time::Duration::from_secs(1), refresh.called.notified())
            .await
            .unwrap();
        assert_eq!(refresh.calls.load(Ordering::Relaxed), 1);
    }
}
