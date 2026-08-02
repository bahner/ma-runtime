//! Behaviour content fetching (ma-runtime-v1.md §14.2.2), and the
//! `#!/ipfs/<cid>`/`#!/ipns/<key>` reference resolver backing the
//! `ma_ipfs_include` host function (ma-scheme-v1.md §11.1).
//!
//! **The runtime performs no recursive expansion of any kind here.** An
//! earlier draft of this module scanned fetched behaviour text for `#!`
//! directive lines and spliced them in recursively, before ever handing
//! the result to a plugin's `set_behaviour` export. That mechanism has
//! been removed: multi-piece library composition is now entirely a
//! ma-scheme-level concern, handled by the dialect's own
//! `ma-include-ipfs` primitive, which calls `ma_ipfs_include` once per
//! reference it encounters and performs its own recursion/depth/cycle
//! tracking on the guest side (see `lambda-ma/scheme-actor`). This module is
//! now a single, flat,
//! reference-to-bytes resolver — nothing more.

use anyhow::{anyhow, Context, Result};

use crate::kubo::{cat_bytes, dag_resolve};

/// Fetch `EntityNode.behaviour`'s content as raw bytes — a single, flat
/// fetch, no scanning, no recursion. Used by `EntityPlugin::load` to
/// obtain the text passed to `set_behaviour` on every load. (An earlier
/// draft also exposed this as a live `ma_get_behaviour` host function;
/// that primitive has been removed entirely — ma-scheme-v1.md §11 — a
/// script reads its own behaviour reference from config instead, and this
/// function is now purely an internal load-time helper.)
pub async fn fetch_behaviour(kubo_url: &str, cid: &str) -> Result<Vec<u8>> {
    cat_bytes(kubo_url, cid)
        .await
        .with_context(|| format!("fetching behaviour content from {cid}"))
}

/// Resolve a single `ma-include-ipfs` reference (ma-scheme-v1.md §11.1) —
/// a literal token of the form `#!/ipfs/<cid>` or `#!/ipns/<key>` — to its
/// raw content bytes. Backs the `ma_ipfs_include` host function.
///
/// A single fetch: `/ipfs/<cid>` resolves trivially (content-addressed,
/// `dag_resolve` is a no-op pass-through); `/ipns/<key>` is resolved via
/// Kubo's name resolution first. No recursion, no directive scanning of
/// any kind — the caller (the ma-scheme guest) is entirely responsible
/// for recursing into whatever content this returns, and for its own
/// depth/cycle guard.
pub async fn resolve_ipfs_include(kubo_url: &str, reference: &str) -> Result<Vec<u8>> {
    let path = reference
        .strip_prefix("#!")
        .filter(|r| r.starts_with("/ipfs/") || r.starts_with("/ipns/"))
        .ok_or_else(|| {
            anyhow!(
                "ma_ipfs_include: reference {reference:?} must be #!/ipfs/<cid> or #!/ipns/<key>"
            )
        })?;
    let cid = dag_resolve(kubo_url, path)
        .await
        .with_context(|| format!("resolving ma-include-ipfs reference {reference}"))?;
    cat_bytes(kubo_url, &cid).await.with_context(|| {
        format!("fetching content for ma-include-ipfs reference {reference} ({cid})")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkubo::MockKubo;

    #[tokio::test]
    async fn fetch_behaviour_returns_raw_content_unmodified() {
        let kubo = MockKubo::start().await;
        let cid = kubo
            .add_bytes(b"(define (on-message msg) msg)\n".to_vec())
            .await;
        let out = fetch_behaviour(kubo.url(), &cid).await.unwrap();
        assert_eq!(out, b"(define (on-message msg) msg)\n");
    }

    #[tokio::test]
    async fn resolve_ipfs_include_fetches_ipfs_reference() {
        let kubo = MockKubo::start().await;
        let cid = kubo.add_bytes(b"(define (helper) 1)\n".to_vec()).await;
        let reference = format!("#!/ipfs/{cid}");
        let out = resolve_ipfs_include(kubo.url(), &reference).await.unwrap();
        assert_eq!(out, b"(define (helper) 1)\n");
    }

    #[tokio::test]
    async fn resolve_ipfs_include_fetches_ipns_reference() {
        // MockKubo's dag_resolve only strips /ipfs/, passing /ipns/<key>
        // straight through as if it were already a bare CID — enough to
        // exercise this function's /ipns/ code path without a real Kubo's
        // name resolution.
        let kubo = MockKubo::start().await;
        let cid = kubo.add_bytes(b"(define x 42)\n".to_vec()).await;
        let reference = format!("#!/ipns/{cid}");
        let out = resolve_ipfs_include(kubo.url(), &reference).await.unwrap();
        assert_eq!(out, b"(define x 42)\n");
    }

    #[tokio::test]
    async fn resolve_ipfs_include_rejects_missing_hash_bang_prefix() {
        let kubo = MockKubo::start().await;
        assert!(resolve_ipfs_include(kubo.url(), "/ipfs/bafybei")
            .await
            .is_err());
    }

    #[tokio::test]
    async fn resolve_ipfs_include_rejects_unknown_reference_kind() {
        let kubo = MockKubo::start().await;
        assert!(resolve_ipfs_include(kubo.url(), "#!/other/thing")
            .await
            .is_err());
    }
}
