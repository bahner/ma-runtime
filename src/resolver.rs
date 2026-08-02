//! Runtime-local DID resolver.
//!
//! Native `ma` already depends on local Kubo, so DID resolution should prefer
//! Kubo RPC over public HTTP gateways.

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use ma_core::{Did, DidDocumentResolver, Document};

#[derive(Clone)]
pub struct KuboDidResolver {
    kubo_rpc_url: Arc<str>,
}

impl KuboDidResolver {
    #[must_use]
    pub fn new(kubo_rpc_url: impl Into<Arc<str>>) -> Self {
        Self {
            kubo_rpc_url: kubo_rpc_url.into(),
        }
    }

    async fn resolve_via_kubo(&self, did: &str) -> Result<Document> {
        let parsed = Did::try_from(did).map_err(|err| anyhow!(err))?;
        let path = format!("/ipns/{}", parsed.ipns);
        let cid = crate::kubo::name_resolve(&self.kubo_rpc_url, &parsed.ipns)
            .await
            .with_context(|| format!("Kubo failed to resolve {path}"))?;
        let bytes = crate::kubo::block_get_bytes(&self.kubo_rpc_url, &cid)
            .await
            .with_context(|| format!("Kubo failed to fetch DID document block {cid}"))?;
        Document::decode(&bytes).map_err(|err| anyhow!(err))
    }
}

#[async_trait]
impl DidDocumentResolver for KuboDidResolver {
    async fn resolve(&self, did: &str) -> ma_core::Result<Document> {
        self.resolve_via_kubo(did)
            .await
            .map_err(|kubo_err| ma_core::Error::Resolution {
                did: did.to_string(),
                detail: format!("Kubo RPC failed: {kubo_err}"),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::KuboDidResolver;
    use ma_core::{DidDocumentResolver, SigningKey};

    #[tokio::test]
    async fn resolves_did_document_through_kubo_rpc() {
        let kubo = crate::testkubo::MockKubo::start().await;
        let did = ma_core::Did::new_identity("k51qzi5uqu5kuboresolver").unwrap();
        let signing_key =
            SigningKey::generate(ma_core::Did::new_url(did.ipns.clone(), Some("sign")).unwrap())
                .unwrap();
        let mut document = ma_core::Document::new(&did, &did);
        let vm = ma_core::VerificationMethod::try_from(&signing_key).unwrap();
        document.verification_method.push(vm.clone());
        document.assertion_method.push(vm.id.clone());
        document.sign(&signing_key, &vm).unwrap();

        kubo.add_bytes_at(&did.ipns, document.encode().unwrap())
            .await;
        let did_resolver = KuboDidResolver::new(kubo.url().to_string());

        let document = did_resolver
            .resolve(&did.base_id())
            .await
            .expect("Kubo RPC should resolve local DID document");
        assert_eq!(document.id, did.base_id());
    }
}
