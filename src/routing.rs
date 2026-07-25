use ma_core::Did;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetRoute {
    LocalFragment(String),
    LocalRuntime,
    Remote(Did),
    Invalid,
}

pub fn classify_target(target: &str, our_did: &str) -> TargetRoute {
    let target = target.trim();
    let Ok(target_did) = Did::try_from(target) else {
        return TargetRoute::Invalid;
    };

    if target_did.base_id() != local_base_did(our_did) {
        return TargetRoute::Remote(target_did);
    }

    match target_did.fragment {
        Some(fragment) if !fragment.is_empty() => TargetRoute::LocalFragment(fragment),
        _ => TargetRoute::LocalRuntime,
    }
}

pub fn local_target_fragment(target: &str, our_did: &str) -> Option<String> {
    match classify_target(target, our_did) {
        TargetRoute::LocalFragment(fragment) => Some(fragment),
        _ => None,
    }
}

pub fn local_actor_url(our_did: &str, fragment: &str) -> String {
    format!("{}#{fragment}", local_base_did(our_did))
}

fn local_base_did(our_did: &str) -> String {
    Did::try_from(our_did).map_or_else(|_| our_did.trim().to_string(), |did| did.base_id())
}

#[cfg(test)]
mod tests {
    use super::{classify_target, local_actor_url, local_target_fragment, TargetRoute};

    #[test]
    fn classifies_local_fragment_target() {
        assert_eq!(
            classify_target("did:ma:abc#construct", "did:ma:abc"),
            TargetRoute::LocalFragment("construct".to_string())
        );
    }

    #[test]
    fn classifies_local_runtime_target_without_fragment() {
        assert_eq!(
            classify_target("did:ma:abc", "did:ma:abc"),
            TargetRoute::LocalRuntime
        );
    }

    #[test]
    fn classifies_remote_target() {
        assert!(matches!(
            classify_target("did:ma:xyz#construct", "did:ma:abc"),
            TargetRoute::Remote(_)
        ));
    }

    #[test]
    fn rejects_bare_fragment_target() {
        assert_eq!(
            classify_target("#construct", "did:ma:abc"),
            TargetRoute::Invalid
        );
    }

    #[test]
    fn extracts_local_target_fragment() {
        assert_eq!(
            local_target_fragment("did:ma:abc#rms", "did:ma:abc").as_deref(),
            Some("rms")
        );
    }

    #[test]
    fn local_actor_url_uses_base_did() {
        assert_eq!(
            local_actor_url("did:ma:abc#root", "room"),
            "did:ma:abc#room"
        );
    }
}
