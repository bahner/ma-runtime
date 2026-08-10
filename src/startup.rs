//! Boot-time helpers: secret-bundle loading and configuration defaults.
//!
//! These are pure functions over [`Config`] used during daemon startup, split
//! out of `main.rs` to keep the entry point focused on orchestration.

use anyhow::{anyhow, Context, Result};
use cid::Cid;
use ma_core::config::{Config, SecretBundle};
use std::{net::SocketAddr, path::Path};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub const DEFAULT_POLL_MS: u64 = 100;
pub const DEFAULT_STATUS_BIND: &str = "127.0.0.1:5003";

pub async fn materialise_plugin_envelope_queue_capacity(
    kubo_url: &str,
    root_cid: &str,
) -> Result<(String, usize)> {
    let mut manifest: crate::entity::RuntimeManifest = crate::kubo::dag_get(kubo_url, root_cid)
        .await
        .context("loading manifest for plugin envelope queue capacity")?;
    let capacity = crate::crud::config::plugin_envelope_queue_capacity(
        manifest.config.get("plugin_envelope_queue_capacity"),
    )?;
    if manifest
        .config
        .contains_key("plugin_envelope_queue_capacity")
    {
        return Ok((root_cid.to_string(), capacity));
    }

    manifest.config.insert(
        "plugin_envelope_queue_capacity".to_string(),
        serde_yaml::Value::from(capacity as u64),
    );
    let new_root_cid = crate::kubo::dag_put(kubo_url, &manifest).await?;
    if let Err(error) = crate::kubo::pin_update(kubo_url, root_cid, &new_root_cid).await {
        tracing::warn!(old = %root_cid, new = %new_root_cid, error = %error, "manifest pin_update failed");
    }
    Ok((new_root_cid, capacity))
}

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

/// Canonicalise an RFC 3339 timestamp to whole-second UTC precision.
///
/// Returns `None` when the input cannot be parsed as RFC 3339.
pub fn canonicalise_rfc3339_utc_seconds(value: &str) -> Option<String> {
    let parsed = OffsetDateTime::parse(value, &Rfc3339).ok()?;
    let whole_seconds = parsed.replace_nanosecond(0).ok()?;
    whole_seconds.format(&Rfc3339).ok()
}

/// Ensure bundle `created_at` is serialised in canonical whole-second RFC 3339 form.
///
/// This preserves the timestamp instant and only changes textual precision.
pub fn canonicalise_bundle_created_at(bundle: &mut SecretBundle) {
    if let Some(canonical) = canonicalise_rfc3339_utc_seconds(&bundle.created_at) {
        bundle.created_at = canonical;
    }
}

/// QA guard for DID-document timestamp consistency before publish.
///
/// `createdAt` is normalised to whole-second RFC 3339 text without changing
/// the represented instant. `updatedAt` is renewed by `SecretBundle::build_document`
/// via `Document::touch()` and therefore stays in canonical whole-second form.
pub fn qa_prepare_bundle_timestamps_for_publish(bundle: &mut SecretBundle) {
    canonicalise_bundle_created_at(bundle);
}

pub fn should_generate_headless_config(config: &Config, bundle_path: &Path) -> bool {
    let config_exists = config
        .config_path
        .as_ref()
        .is_some_and(|path| path.exists());
    !config_exists && !bundle_path.exists()
}

pub fn get_u64_setting(config: &Config, key: &str, default: u64) -> u64 {
    config
        .extra
        .get(key)
        .and_then(serde_yaml::Value::as_u64)
        .unwrap_or(default)
}

pub fn get_bool_setting(config: &Config, key: &str, default: bool) -> bool {
    config
        .extra
        .get(key)
        .and_then(serde_yaml::Value::as_bool)
        .unwrap_or(default)
}

pub fn select_poll_ms(cli_poll_ms: Option<u64>, config: &Config) -> Result<u64> {
    let poll_ms = match cli_poll_ms {
        Some(value) => value,
        None => match config.extra.get("poll_ms") {
            Some(value) => value
                .as_u64()
                .ok_or_else(|| anyhow!("poll_ms in config.yaml must be a positive integer"))?,
            None => DEFAULT_POLL_MS,
        },
    };
    if poll_ms == 0 {
        return Err(anyhow!("poll_ms must be greater than zero"));
    }
    Ok(poll_ms)
}

pub fn select_status_bind(
    cli_status_bind: Option<SocketAddr>,
    config: &Config,
) -> Result<SocketAddr> {
    if let Some(status_bind) = cli_status_bind {
        return Ok(status_bind);
    }
    let status_bind = match config.extra.get("status_bind") {
        Some(value) => value
            .as_str()
            .ok_or_else(|| anyhow!("status_bind in config.yaml must be a socket address"))?,
        None => DEFAULT_STATUS_BIND,
    };
    status_bind
        .parse()
        .with_context(|| format!("invalid status_bind in config.yaml: {status_bind}"))
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

pub fn require_kinds_overlay_base(kinds_cid: Option<&str>, root_cid: Option<&str>) -> Result<()> {
    if kinds_cid.is_some() && root_cid.is_none() {
        return Err(anyhow!(
            "--kinds-cid requires an existing runtime head from --root-cid, --bootstrap, config.yaml, or runtime IPNS"
        ));
    }
    Ok(())
}

pub fn persist_root_cid(config: &mut Config, root_cid: &str) -> Result<()> {
    if root_cid_setting(config).as_deref() == Some(root_cid) {
        return Ok(());
    }
    config.extra.insert(
        serde_yaml::Value::String("root_cid".to_string()),
        serde_yaml::Value::String(root_cid.to_string()),
    );
    config.save()?;
    Ok(())
}

type ManifestConfigMap = std::collections::BTreeMap<String, serde_yaml::Value>;

fn insert_static_manifest_defaults(out: &mut ManifestConfigMap) {
    out.insert(
        "name".to_string(),
        serde_yaml::Value::String(crate::crud::config::DEFAULT_RUNTIME_NAME.to_string()),
    );
    out.insert(
        "description".to_string(),
        serde_yaml::Value::String(crate::crud::config::DEFAULT_RUNTIME_DESCRIPTION.to_string()),
    );
    out.insert(
        "plugin_envelope_queue_capacity".to_string(),
        serde_yaml::Value::from(crate::crud::config::DEFAULT_PLUGIN_ENVELOPE_QUEUE_CAPACITY as u64),
    );
}

fn insert_did_resolver_manifest_settings(out: &mut ManifestConfigMap, config: &Config) {
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
        "did_resolve_attempts".to_string(),
        serde_yaml::Value::from(get_u64_setting(config, "did_resolve_attempts", 5)),
    );
    out.insert(
        "did_resolve_attempt_timeout_secs".to_string(),
        serde_yaml::Value::from(get_u64_setting(
            config,
            "did_resolve_attempt_timeout_secs",
            60,
        )),
    );
}

fn insert_did_publish_manifest_settings(out: &mut ManifestConfigMap, config: &Config) {
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
            3600,
        )),
    );
    out.insert(
        "did_publish_cache_warm_secs".to_string(),
        serde_yaml::Value::from(get_u64_setting(config, "did_publish_cache_warm_secs", 3600)),
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
}

fn insert_ipns_publish_manifest_settings(out: &mut ManifestConfigMap, config: &Config) {
    out.insert(
        "ipns_publish_lifetime_hours".to_string(),
        serde_yaml::Value::from(get_u64_setting(config, "ipns_publish_lifetime_hours", 8760)),
    );
    out.insert(
        "ipns_publish_timeout_secs".to_string(),
        serde_yaml::Value::from(get_u64_setting(
            config,
            "ipns_publish_timeout_secs",
            get_u64_setting(config, "did_document_publishing_timeout_secs", 120),
        )),
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
}

pub fn runtime_manifest_config(config: &Config) -> ManifestConfigMap {
    let mut out = ManifestConfigMap::new();
    insert_static_manifest_defaults(&mut out);
    insert_did_resolver_manifest_settings(&mut out, config);
    insert_did_publish_manifest_settings(&mut out, config);
    insert_ipns_publish_manifest_settings(&mut out, config);
    out
}

#[cfg(test)]
mod tests {
    use super::{
        canonicalise_bundle_created_at, canonicalise_rfc3339_utc_seconds, persist_root_cid,
        qa_prepare_bundle_timestamps_for_publish, require_kinds_overlay_base, root_cid_setting,
        runtime_manifest_config, select_poll_ms, select_root_cid, select_status_bind,
        should_generate_headless_config, DEFAULT_POLL_MS, DEFAULT_STATUS_BIND,
    };

    #[test]
    fn canonicalises_subsecond_rfc3339_to_whole_seconds() {
        assert_eq!(
            canonicalise_rfc3339_utc_seconds("2026-07-19T19:45:24.489Z").as_deref(),
            Some("2026-07-19T19:45:24Z")
        );
    }

    #[test]
    fn leaves_whole_second_rfc3339_unchanged() {
        assert_eq!(
            canonicalise_rfc3339_utc_seconds("2026-08-08T08:02:37Z").as_deref(),
            Some("2026-08-08T08:02:37Z")
        );
    }

    #[test]
    fn rejects_invalid_rfc3339() {
        assert_eq!(canonicalise_rfc3339_utc_seconds("not-a-timestamp"), None);
    }

    #[test]
    fn canonicalises_bundle_created_at_only() {
        let bundle = ma_core::config::SecretBundle::generate();
        let mut bundle = bundle;
        bundle.created_at = "2026-07-19T19:45:24.489Z".to_string();

        canonicalise_bundle_created_at(&mut bundle);

        assert_eq!(bundle.created_at, "2026-07-19T19:45:24Z");
    }

    #[test]
    fn qa_prepare_normalises_created_at() {
        let bundle = ma_core::config::SecretBundle::generate();
        let mut bundle = bundle;
        bundle.created_at = "2026-07-19T19:45:24.489Z".to_string();

        qa_prepare_bundle_timestamps_for_publish(&mut bundle);

        assert_eq!(bundle.created_at, "2026-07-19T19:45:24Z");
    }

    #[test]
    fn qa_prepare_and_build_document_keep_second_precision_for_both_timestamps() {
        let mut bundle = ma_core::config::SecretBundle::generate();
        bundle.created_at = "2026-07-19T19:45:24.489Z".to_string();

        qa_prepare_bundle_timestamps_for_publish(&mut bundle);

        let ma = ma_core::MaExtension::new().kind("runtime");
        let document = bundle.build_document(ma).unwrap();

        assert_eq!(document.created_at, "2026-07-19T19:45:24Z");
        assert!(document.created_at.ends_with('Z'));
        assert!(!document.created_at.contains('.'));

        assert!(document.updated_at.ends_with('Z'));
        assert!(!document.updated_at.contains('.'));
    }

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
    fn kinds_overlay_requires_an_existing_runtime_head() {
        let err = require_kinds_overlay_base(Some("bafykinds"), None).unwrap_err();

        assert!(err
            .to_string()
            .contains("--kinds-cid requires an existing runtime head"));
    }

    #[test]
    fn kinds_overlay_accepts_an_existing_runtime_head() {
        require_kinds_overlay_base(Some("bafykinds"), Some("bafyroot")).unwrap();
        require_kinds_overlay_base(None, None).unwrap();
    }

    #[test]
    fn runtime_startup_settings_default_when_unset() {
        let config = ma_core::config::Config::from_yaml_str("{}\n").unwrap();

        assert_eq!(select_poll_ms(None, &config).unwrap(), DEFAULT_POLL_MS);
        assert_eq!(
            select_status_bind(None, &config).unwrap().to_string(),
            DEFAULT_STATUS_BIND
        );
    }

    #[test]
    fn runtime_startup_settings_use_yaml_when_cli_is_absent() {
        let config =
            ma_core::config::Config::from_yaml_str("poll_ms: 250\nstatus_bind: 127.0.0.1:9000\n")
                .unwrap();

        assert_eq!(select_poll_ms(None, &config).unwrap(), 250);
        assert_eq!(
            select_status_bind(None, &config).unwrap().to_string(),
            "127.0.0.1:9000"
        );
    }

    #[test]
    fn cli_runtime_startup_settings_override_yaml() {
        let config =
            ma_core::config::Config::from_yaml_str("poll_ms: 250\nstatus_bind: 127.0.0.1:9000\n")
                .unwrap();

        assert_eq!(select_poll_ms(Some(50), &config).unwrap(), 50);
        assert_eq!(
            select_status_bind(Some("127.0.0.1:9100".parse().unwrap()), &config)
                .unwrap()
                .to_string(),
            "127.0.0.1:9100"
        );
    }

    #[test]
    fn invalid_runtime_startup_settings_error() {
        let zero_poll = ma_core::config::Config::from_yaml_str("poll_ms: 0\n").unwrap();
        assert!(select_poll_ms(None, &zero_poll).is_err());

        let invalid_status =
            ma_core::config::Config::from_yaml_str("status_bind: not-an-address\n").unwrap();
        assert!(select_status_bind(None, &invalid_status).is_err());
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

        let mut config = ma_core::config::Config::from_yaml_str(
            "owners:\n  - did:ma:alice\ni18n: art-x-lyaric\nlog_level: info\n",
        )
        .unwrap();
        config.config_path = Some(path.clone());
        persist_root_cid(&mut config, "bafyroot").unwrap();

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

    #[test]
    fn does_not_rewrite_config_when_root_cid_is_unchanged() {
        let path = std::env::temp_dir().join(format!(
            "ma-runtime-unchanged-root-cid-test-{}.yaml",
            std::process::id()
        ));
        let original = "# Keep this comment and formatting.\nroot_cid: bafyroot\nlog_level: info\n";
        std::fs::write(&path, original).unwrap();

        let mut config = ma_core::config::Config::from_yaml_str(original).unwrap();
        config.config_path = Some(path.clone());
        persist_root_cid(&mut config, "bafyroot").unwrap();

        assert_eq!(std::fs::read_to_string(&path).unwrap(), original);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn does_not_autogenerate_when_config_exists_but_bundle_is_missing() {
        let dir = std::env::temp_dir().join(format!(
            "ma-runtime-existing-config-test-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("sky.yaml");
        let bundle_path = dir.join("sky.bin");
        std::fs::write(&config_path, "secret_bundle_passphrase: test\n").unwrap();

        let mut config =
            ma_core::config::Config::from_yaml_str("secret_bundle_passphrase: test\n").unwrap();
        config.config_path = Some(config_path.clone());

        assert!(!should_generate_headless_config(&config, &bundle_path));

        let _ = std::fs::remove_file(config_path);
        let _ = std::fs::remove_dir(dir);
    }

    #[test]
    fn autogenerates_only_when_config_and_bundle_are_missing() {
        let dir = std::env::temp_dir().join(format!(
            "ma-runtime-missing-config-test-{}",
            std::process::id()
        ));
        let config_path = dir.join("sky.yaml");
        let bundle_path = dir.join("sky.bin");

        let mut config = ma_core::config::Config::from_yaml_str("{}\n").unwrap();
        config.config_path = Some(config_path);

        assert!(should_generate_headless_config(&config, &bundle_path));
    }

    #[test]
    fn runtime_manifest_config_includes_ipns_publish_timeout() {
        let config = ma_core::config::Config::from_yaml_str(
            "did_document_publishing_timeout_secs: 240\n\
             ipns_publish_timeout_secs: 180\n\
             did_resolve_attempts: 3\n\
             did_resolve_attempt_timeout_secs: 90\n\
             ipns_publish_allow_offline: false\n\
             ipns_publish_resolve: true\n",
        )
        .unwrap();

        let manifest_config = runtime_manifest_config(&config);

        assert_eq!(
            manifest_config
                .get("ipns_publish_timeout_secs")
                .and_then(serde_yaml::Value::as_u64),
            Some(180)
        );
        assert_eq!(
            manifest_config
                .get("ipns_publish_allow_offline")
                .and_then(serde_yaml::Value::as_bool),
            Some(false)
        );
        assert_eq!(
            manifest_config
                .get("ipns_publish_resolve")
                .and_then(serde_yaml::Value::as_bool),
            Some(true)
        );
        assert_eq!(
            manifest_config
                .get("did_resolve_attempts")
                .and_then(serde_yaml::Value::as_u64),
            Some(3)
        );
        assert_eq!(
            manifest_config
                .get("did_resolve_attempt_timeout_secs")
                .and_then(serde_yaml::Value::as_u64),
            Some(90)
        );
    }

    #[tokio::test]
    async fn materialises_default_plugin_envelope_queue_capacity() {
        let kubo = crate::testkubo::MockKubo::start().await;
        let manifest = crate::entity::RuntimeManifest::default();
        let root_cid = crate::kubo::dag_put(kubo.url(), &manifest).await.unwrap();

        let (new_root_cid, capacity) =
            super::materialise_plugin_envelope_queue_capacity(kubo.url(), &root_cid)
                .await
                .unwrap();

        assert_eq!(
            capacity,
            crate::crud::config::DEFAULT_PLUGIN_ENVELOPE_QUEUE_CAPACITY
        );
        assert_ne!(new_root_cid, root_cid);
        let updated: crate::entity::RuntimeManifest =
            crate::kubo::dag_get(kubo.url(), &new_root_cid)
                .await
                .unwrap();
        assert_eq!(
            updated
                .config
                .get("plugin_envelope_queue_capacity")
                .and_then(serde_yaml::Value::as_u64),
            Some(crate::crud::config::DEFAULT_PLUGIN_ENVELOPE_QUEUE_CAPACITY as u64)
        );
    }

    #[tokio::test]
    async fn honours_and_validates_manifest_plugin_envelope_queue_capacity() {
        let kubo = crate::testkubo::MockKubo::start().await;
        let mut manifest = crate::entity::RuntimeManifest::default();
        manifest.config.insert(
            "plugin_envelope_queue_capacity".to_string(),
            serde_yaml::Value::from(7),
        );
        let root_cid = crate::kubo::dag_put(kubo.url(), &manifest).await.unwrap();
        let (unchanged_root, capacity) =
            super::materialise_plugin_envelope_queue_capacity(kubo.url(), &root_cid)
                .await
                .unwrap();
        assert_eq!(unchanged_root, root_cid);
        assert_eq!(capacity, 7);

        manifest.config.insert(
            "plugin_envelope_queue_capacity".to_string(),
            serde_yaml::Value::from(0),
        );
        let invalid_root = crate::kubo::dag_put(kubo.url(), &manifest).await.unwrap();
        assert!(
            super::materialise_plugin_envelope_queue_capacity(kubo.url(), &invalid_root)
                .await
                .is_err()
        );
    }

    #[test]
    fn ipns_publish_timeout_defaults_to_outer_publish_timeout() {
        let config =
            ma_core::config::Config::from_yaml_str("did_document_publishing_timeout_secs: 240\n")
                .unwrap();

        let manifest_config = runtime_manifest_config(&config);

        assert_eq!(
            manifest_config
                .get("ipns_publish_timeout_secs")
                .and_then(serde_yaml::Value::as_u64),
            Some(240)
        );
    }

    #[test]
    fn runtime_manifest_config_defaults_root_publish_to_hourly() {
        let config = ma_core::config::Config::from_yaml_str("{}\n").unwrap();

        let manifest_config = runtime_manifest_config(&config);

        assert_eq!(
            manifest_config
                .get("did_document_publishing_interval_secs")
                .and_then(serde_yaml::Value::as_u64),
            Some(3600)
        );
        assert_eq!(
            manifest_config
                .get("did_publish_cache_warm_secs")
                .and_then(serde_yaml::Value::as_u64),
            Some(3600)
        );
    }
}
