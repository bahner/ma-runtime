//! Boot-time helpers: secret-bundle loading and configuration defaults.
//!
//! These are pure functions over [`Config`] used during daemon startup, split
//! out of `main.rs` to keep the entry point focused on orchestration.

use anyhow::{anyhow, Context, Result};
use cid::Cid;
use ma_core::config::{Config, SecretBundle};
use std::path::Path;

pub fn load_secret_bundle(config: &Config) -> Result<SecretBundle> {
    let passphrase = config
        .secret_bundle_passphrase
        .as_deref()
        .ok_or_else(|| anyhow!("secret_bundle_passphrase is required (env or config)"))?;
    let bundle_path = config.effective_secret_bundle()?;
    SecretBundle::load(&bundle_path, passphrase).with_context(|| {
        format!(
            "failed to load secret bundle from {}",
            bundle_path.display()
        )
    })
}

pub fn get_u64_setting(config: &Config, key: &str, default: u64) -> u64 {
    config
        .extra
        .get(key)
        .and_then(serde_yaml::Value::as_u64)
        .unwrap_or(default)
}

pub fn root_cid_setting(config: &Config) -> Option<String> {
    config
        .extra
        .get("root_cid")
        .and_then(serde_yaml::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

pub fn select_root_cid(
    cli_root_cid: Option<String>,
    bootstrap_root_cid: Option<String>,
    config: &Config,
) -> Result<Option<String>> {
    if cli_root_cid.is_some() {
        return Ok(cli_root_cid);
    }
    if bootstrap_root_cid.is_some() {
        return Ok(bootstrap_root_cid);
    }
    let config_root_cid = root_cid_setting(config);
    if let Some(ref cid) = config_root_cid {
        Cid::try_from(cid.as_str())
            .with_context(|| format!("invalid root_cid in config.yaml: {cid}"))?;
    }
    Ok(config_root_cid)
}

pub fn persist_root_cid_to_config(path: &Path, root_cid: &str) -> Result<()> {
    let yaml_text = if path.exists() {
        std::fs::read_to_string(path)?
    } else {
        String::new()
    };
    let yaml_to_parse = if yaml_text.trim().is_empty() {
        "{}".to_string()
    } else {
        yaml_text
    };
    let mut config = Config::from_yaml_str(&yaml_to_parse)?;
    config.config_path = Some(path.to_path_buf());
    config.extra.insert(
        serde_yaml::Value::String("root_cid".to_string()),
        serde_yaml::Value::String(root_cid.to_string()),
    );
    config.save()?;
    Ok(())
}

pub fn runtime_manifest_config(
    config: &Config,
) -> std::collections::BTreeMap<String, serde_yaml::Value> {
    let mut out = std::collections::BTreeMap::new();

    out.insert(
        "name".to_string(),
        serde_yaml::Value::String(crate::crud::config::DEFAULT_RUNTIME_NAME.to_string()),
    );
    out.insert(
        "description".to_string(),
        serde_yaml::Value::String(crate::crud::config::DEFAULT_RUNTIME_DESCRIPTION.to_string()),
    );

    out.insert(
        "did_resolver_positive_ttl_secs".to_string(),
        serde_yaml::Value::from(get_u64_setting(
            config,
            "did_resolver_positive_ttl_secs",
            60,
        )),
    );
    out.insert(
        "did_resolver_negative_ttl_secs".to_string(),
        serde_yaml::Value::from(get_u64_setting(
            config,
            "did_resolver_negative_ttl_secs",
            10,
        )),
    );
    out.insert(
        "wasm_reload_shutdown_timeout_ms".to_string(),
        serde_yaml::Value::from(get_u64_setting(
            config,
            "wasm_reload_shutdown_timeout_ms",
            crate::crud::config::DEFAULT_WASM_RELOAD_SHUTDOWN_TIMEOUT_MS,
        )),
    );
    out.insert(
        "did_document_publishing_interval_secs".to_string(),
        serde_yaml::Value::from(get_u64_setting(
            config,
            "did_document_publishing_interval_secs",
            300,
        )),
    );
    out.insert(
        "did_document_publishing_timeout_secs".to_string(),
        serde_yaml::Value::from(get_u64_setting(
            config,
            "did_document_publishing_timeout_secs",
            120,
        )),
    );
    out.insert(
        "did_document_publishing_lifetime_hours".to_string(),
        serde_yaml::Value::from(get_u64_setting(
            config,
            "did_document_publishing_lifetime_hours",
            8760,
        )),
    );
    out.insert(
        "ipns_publish_lifetime_hours".to_string(),
        serde_yaml::Value::from(get_u64_setting(config, "ipns_publish_lifetime_hours", 8760)),
    );
    out.insert(
        "ipns_publish_resolve".to_string(),
        serde_yaml::Value::from(
            config
                .extra
                .get("ipns_publish_resolve")
                .and_then(serde_yaml::Value::as_bool)
                .unwrap_or(false),
        ),
    );
    out.insert(
        "ipns_publish_allow_offline".to_string(),
        serde_yaml::Value::from(
            config
                .extra
                .get("ipns_publish_allow_offline")
                .and_then(serde_yaml::Value::as_bool)
                .unwrap_or(true),
        ),
    );
    out
}

#[cfg(test)]
mod tests {
    use super::{persist_root_cid_to_config, root_cid_setting, select_root_cid};

    #[test]
    fn cli_root_cid_overrides_invalid_config_root_cid() {
        let config =
            ma_core::config::Config::from_yaml_str("root_cid: definitely-not-a-cid\n").unwrap();

        let selected = select_root_cid(Some("cli-root".to_string()), None, &config).unwrap();

        assert_eq!(selected.as_deref(), Some("cli-root"));
    }

    #[test]
    fn invalid_config_root_cid_errors_when_config_is_used() {
        let config =
            ma_core::config::Config::from_yaml_str("root_cid: definitely-not-a-cid\n").unwrap();

        let err = select_root_cid(None, None, &config).unwrap_err();

        assert!(err.to_string().contains("invalid root_cid in config.yaml"));
    }

    #[test]
    fn persists_root_cid_without_dropping_existing_config() {
        let path = std::env::temp_dir().join(format!(
            "ma-runtime-root-cid-test-{}.yaml",
            std::process::id()
        ));
        std::fs::write(
            &path,
            "owners:\n  - did:ma:alice\ni18n: art-x-lyaric\nlog_level: info\n",
        )
        .unwrap();

        persist_root_cid_to_config(&path, "bafyroot").unwrap();

        let saved = std::fs::read_to_string(&path).unwrap();
        let config = ma_core::config::Config::from_yaml_str(&saved).unwrap();
        assert_eq!(root_cid_setting(&config).as_deref(), Some("bafyroot"));
        assert_eq!(
            config.extra.get("i18n").and_then(serde_yaml::Value::as_str),
            Some("art-x-lyaric")
        );
        assert!(matches!(
            config.extra.get("owners"),
            Some(serde_yaml::Value::Sequence(_))
        ));

        let _ = std::fs::remove_file(path);
    }
}
