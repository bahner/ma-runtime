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

    fn test_did(seed: u8) -> String {
        format!(
            "did:ma:{}",
            ma_core::ipns_from_secret([seed; 32]).expect("test IPNS identifier")
        )
    }

    #[test]
    fn classifies_local_fragment_target() {
        let local = test_did(1);
        assert_eq!(
            classify_target(&format!("{local}#concourse"), &local),
            TargetRoute::LocalFragment("concourse".to_string())
        );
    }

    #[test]
    fn classifies_local_runtime_target_without_fragment() {
        let local = test_did(1);
        assert_eq!(classify_target(&local, &local), TargetRoute::LocalRuntime);
    }

    #[test]
    fn classifies_remote_target() {
        let local = test_did(1);
        let remote = test_did(2);
        assert!(matches!(
            classify_target(&format!("{remote}#concourse"), &local),
            TargetRoute::Remote(_)
        ));
    }

    #[test]
    fn rejects_bare_fragment_target() {
        let local = test_did(1);
        assert_eq!(classify_target("#concourse", &local), TargetRoute::Invalid);
    }

    #[test]
    fn extracts_local_target_fragment() {
        let local = test_did(1);
        assert_eq!(
            local_target_fragment(&format!("{local}#rms"), &local).as_deref(),
            Some("rms")
        );
    }

    #[test]
    fn local_actor_url_uses_base_did() {
        let local = test_did(1);
        assert_eq!(
            local_actor_url(&format!("{local}#root"), "room"),
            format!("{local}#room")
        );
    }
}
