use anyhow::{anyhow, Result};
use ciborium::Value as CborValue;
use tracing::warn;

use super::helpers::{
    cidv1_ref, load_manifest, send_crud_data_yaml, send_crud_error, send_crud_i18n_error,
    send_crud_i18n_errorf, send_crud_ok, send_crud_ok_cid, send_crud_ok_path, send_crud_reply_cbor,
    with_manifest_crud,
};
use super::CrudHandlerCtx;

// ── Config key tables ──────────────────────────────────────────────────────────

/// Daemon config fields that may be read/written via CRUD and are saved to
/// `config.yaml` on change.
pub const DAEMON_CONFIG_KEYS_PUB: &[&str] = &[
    "root_cid",
    "kubo_rpc_url",
    "kubo_key_alias",
    "log_level",
    "log_level_stdout",
    "did_resolver_positive_ttl_secs",
    "did_resolver_negative_ttl_secs",
    "log_file",
    "outbox_backoff_attempts",
    "ipv6_enable",
    "wasm_reload_shutdown_timeout_ms",
];

const DAEMON_CONFIG_KEYS: &[&str] = DAEMON_CONFIG_KEYS_PUB;

/// Manifest config keys that may be written via CRUD (stored in IPFS DAG).
const MANIFEST_CONFIG_KEYS: &[&str] = &[
    "root",
    "start",
    "zion",
    "name",
    "description",
    "i18n",
    "did_resolve_attempts",
    "did_resolve_attempt_timeout_secs",
    "did_document_publishing_interval_secs",
    "did_publish_cache_warm_secs",
    "did_document_publishing_timeout_secs",
    "did_document_publishing_lifetime_hours",
    "ipns_publish_lifetime_hours",
    "ipns_publish_timeout_secs",
    "ipns_publish_resolve",
    "ipns_publish_allow_offline",
    "plugin_envelope_queue_capacity",
];

pub const DEFAULT_ZION_SOURCE: &str =
    "/ipns/k51qzi5uqu5djnoah9igllgb3zsn0sfy75glxdjf8glaozybubkm13nuqbfvpm";

pub const DEFAULT_RUNTIME_NAME: &str = "間trix";
pub const DEFAULT_RUNTIME_DESCRIPTION: &str = "A 間 runtime with a lazy owner.";
pub const DEFAULT_WASM_RELOAD_SHUTDOWN_TIMEOUT_MS: u64 = 250;
pub const DEFAULT_PLUGIN_ENVELOPE_QUEUE_CAPACITY: usize = 1024;

pub fn outbox_backoff_attempts(value: Option<&serde_yaml::Value>) -> Result<usize> {
    let Some(value) = value else {
        return Ok(crate::ipfs::DEFAULT_OUTBOX_BACKOFF_ATTEMPTS);
    };
    let attempts = value
        .as_u64()
        .ok_or_else(|| anyhow!("outbox_backoff_attempts must be a positive integer"))?;
    let attempts = usize::try_from(attempts)
        .map_err(|_| anyhow!("outbox_backoff_attempts exceeds this platform's usize range"))?;
    if attempts == 0 {
        return Err(anyhow!("outbox_backoff_attempts must be greater than zero"));
    }
    Ok(attempts)
}

pub fn plugin_envelope_queue_capacity(value: Option<&serde_yaml::Value>) -> Result<usize> {
    let Some(value) = value else {
        return Ok(DEFAULT_PLUGIN_ENVELOPE_QUEUE_CAPACITY);
    };
    let capacity = value
        .as_u64()
        .ok_or_else(|| anyhow!("plugin_envelope_queue_capacity must be a positive integer"))?;
    let capacity = usize::try_from(capacity).map_err(|_| {
        anyhow!("plugin_envelope_queue_capacity exceeds this platform's usize range")
    })?;
    if capacity == 0 {
        return Err(anyhow!(
            "plugin_envelope_queue_capacity must be greater than zero"
        ));
    }
    Ok(capacity)
}

pub fn wasm_reload_shutdown_timeout(cfg: &ma_core::Config) -> std::time::Duration {
    std::time::Duration::from_millis(
        cfg.extra
            .get("wasm_reload_shutdown_timeout_ms")
            .and_then(serde_yaml::Value::as_u64)
            .unwrap_or(DEFAULT_WASM_RELOAD_SHUTDOWN_TIMEOUT_MS),
    )
}

pub fn default_manifest_config_value(key: &str) -> Option<serde_yaml::Value> {
    match key {
        "zion" => Some(serde_yaml::Value::String(DEFAULT_ZION_SOURCE.to_string())),
        "name" => Some(serde_yaml::Value::String(DEFAULT_RUNTIME_NAME.to_string())),
        "description" => Some(serde_yaml::Value::String(
            DEFAULT_RUNTIME_DESCRIPTION.to_string(),
        )),
        "plugin_envelope_queue_capacity" => Some(serde_yaml::Value::from(
            DEFAULT_PLUGIN_ENVELOPE_QUEUE_CAPACITY as u64,
        )),
        _ => None,
    }
}

/// Keys that are never exposed or writable via CRUD.
/// Any key beginning with `secret` is also blocked dynamically.
const PROTECTED_CONFIG_KEYS: &[&str] = &[
    "slug",
    "secret_bundle",
    "secret_bundle_passphrase",
    "config_path",
];

pub fn is_protected_config_key_pub(key: &str) -> bool {
    PROTECTED_CONFIG_KEYS.contains(&key) || key.starts_with("secret")
}

fn is_protected_config_key(key: &str) -> bool {
    is_protected_config_key_pub(key)
}

/// Read a daemon config field as a `serde_yaml::Value` for CRUD responses.
/// Returns `Value::Null` for unknown or platform-guarded keys.
pub fn daemon_config_key_value_pub(cfg: &ma_core::Config, key: &str) -> serde_yaml::Value {
    match key {
        "root_cid" => cfg
            .extra
            .get("root_cid")
            .cloned()
            .unwrap_or(serde_yaml::Value::Null),
        "kubo_rpc_url" => serde_yaml::Value::String(cfg.kubo_rpc_url.clone()),
        "kubo_key_alias" => serde_yaml::Value::String(cfg.kubo_key_alias.clone()),
        "log_level" => serde_yaml::Value::String(cfg.log_level.clone()),
        "log_level_stdout" => serde_yaml::Value::String(cfg.log_level_stdout.clone()),
        "did_resolver_positive_ttl_secs" => {
            serde_yaml::Value::Number(cfg.did_resolver_positive_ttl_secs.into())
        }
        "did_resolver_negative_ttl_secs" => {
            serde_yaml::Value::Number(cfg.did_resolver_negative_ttl_secs.into())
        }
        "log_file" => cfg.log_file.as_ref().map_or(serde_yaml::Value::Null, |p| {
            serde_yaml::Value::String(p.to_string_lossy().into_owned())
        }),
        "outbox_backoff_attempts" => serde_yaml::Value::Number(
            outbox_backoff_attempts(cfg.extra.get("outbox_backoff_attempts"))
                .unwrap_or(crate::ipfs::DEFAULT_OUTBOX_BACKOFF_ATTEMPTS)
                .into(),
        ),
        "ipv6_enable" => serde_yaml::Value::Bool(
            cfg.extra
                .get("ipv6_enable")
                .and_then(serde_yaml::Value::as_bool)
                .unwrap_or(true),
        ),
        "wasm_reload_shutdown_timeout_ms" => serde_yaml::Value::Number(
            cfg.extra
                .get("wasm_reload_shutdown_timeout_ms")
                .and_then(serde_yaml::Value::as_u64)
                .unwrap_or(DEFAULT_WASM_RELOAD_SHUTDOWN_TIMEOUT_MS)
                .into(),
        ),
        _ => serde_yaml::Value::Null,
    }
}

fn yaml_config_value_to_string(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::Null => None,
        serde_yaml::Value::Bool(value) => Some(value.to_string()),
        serde_yaml::Value::Number(value) => Some(value.to_string()),
        serde_yaml::Value::String(value) => Some(value.clone()),
        value => serde_yaml::to_string(value)
            .ok()
            .map(|s| s.trim_end().to_string()),
    }
}

pub fn public_plugin_config(
    manifest: &crate::entity::RuntimeManifest,
    cfg: &ma_core::Config,
) -> std::collections::BTreeMap<String, String> {
    let mut out = std::collections::BTreeMap::new();
    for (key, value) in &manifest.config {
        if let Some(value) = yaml_config_value_to_string(value) {
            out.insert(key.clone(), value);
        }
    }
    for key in DAEMON_CONFIG_KEYS {
        let value = daemon_config_key_value_pub(cfg, key);
        if let Some(value) = yaml_config_value_to_string(&value) {
            out.insert((*key).to_string(), value);
        }
    }
    for key in ["name", "description"] {
        if !out.contains_key(key) {
            if let Some(value) = default_manifest_config_value(key)
                .and_then(|value| yaml_config_value_to_string(&value))
            {
                out.insert(key.to_string(), value);
            }
        }
    }
    out
}

/// Apply a YAML value from CRUD to the corresponding `Config` field in memory.
pub fn set_daemon_config_key_pub(cfg: &mut ma_core::Config, key: &str, val: &serde_yaml::Value) {
    match key {
        "kubo_rpc_url" => {
            if let Some(s) = val.as_str() {
                cfg.kubo_rpc_url = s.to_string();
            }
        }
        "kubo_key_alias" => {
            if let Some(s) = val.as_str() {
                cfg.kubo_key_alias = s.to_string();
            }
        }
        "log_level" => {
            if let Some(s) = val.as_str() {
                cfg.log_level = s.to_string();
            }
        }
        "log_level_stdout" => {
            if let Some(s) = val.as_str() {
                cfg.log_level_stdout = s.to_string();
            }
        }
        "did_resolver_positive_ttl_secs" => {
            if let Some(n) = val.as_u64() {
                cfg.did_resolver_positive_ttl_secs = n;
            }
        }
        "did_resolver_negative_ttl_secs" => {
            if let Some(n) = val.as_u64() {
                cfg.did_resolver_negative_ttl_secs = n;
            }
        }
        "log_file" => {
            cfg.log_file = val.as_str().map(std::path::PathBuf::from);
        }
        "outbox_backoff_attempts" => {
            if let Some(n) = val.as_u64() {
                cfg.extra.insert(
                    serde_yaml::Value::String("outbox_backoff_attempts".to_string()),
                    serde_yaml::Value::Number(n.into()),
                );
            }
        }
        "ipv6_enable" => {
            if let Some(b) = val.as_bool() {
                cfg.extra.insert(
                    serde_yaml::Value::String("ipv6_enable".to_string()),
                    serde_yaml::Value::Bool(b),
                );
            }
        }
        "wasm_reload_shutdown_timeout_ms" => {
            if let Some(n) = val.as_u64() {
                cfg.extra.insert(
                    serde_yaml::Value::String("wasm_reload_shutdown_timeout_ms".to_string()),
                    serde_yaml::Value::Number(n.into()),
                );
            }
        }
        "root_cid" => {
            if let Some(s) = val.as_str() {
                cfg.extra.insert(
                    serde_yaml::Value::String("root_cid".to_string()),
                    serde_yaml::Value::String(s.to_string()),
                );
            }
        }
        _ => {}
    }
}

fn set_daemon_config_key(cfg: &mut ma_core::Config, key: &str, val: &serde_yaml::Value) {
    set_daemon_config_key_pub(cfg, key, val);
}

/// Convert a CBOR value to a `serde_yaml::Value` for storage in
/// `RuntimeManifest.config`. Clients send native CBOR — text, integer,
/// boolean, float, null, arrays, maps — and this maps it to the YAML
/// value type that the config tree uses internally.
fn cbor_to_yaml(val: &CborValue) -> serde_yaml::Value {
    match val {
        CborValue::Bool(b) => serde_yaml::Value::Bool(*b),
        CborValue::Integer(i) => u64::try_from(*i).map_or_else(
            |_| {
                i64::try_from(*i).map_or(serde_yaml::Value::Null, |n| {
                    serde_yaml::Value::Number(n.into())
                })
            },
            |n| serde_yaml::Value::Number(n.into()),
        ),
        CborValue::Float(f) => serde_yaml::Value::Number((*f).into()),
        CborValue::Text(s) => serde_yaml::Value::String(s.clone()),
        CborValue::Bytes(b) => {
            serde_yaml::Value::String(b.iter().fold(String::new(), |mut acc, byte| {
                use std::fmt::Write;
                let _ = write!(acc, "{byte:02x}");
                acc
            }))
        }
        CborValue::Array(arr) => {
            // Sequences are sets: preserve first-occurrence order, drop duplicates.
            let mut seen = std::collections::HashSet::new();
            let items: Vec<serde_yaml::Value> = arr
                .iter()
                .map(cbor_to_yaml)
                .filter(|item| {
                    let key = match item {
                        serde_yaml::Value::String(s) => s.clone(),
                        other => format!("{other:?}"),
                    };
                    seen.insert(key)
                })
                .collect();
            serde_yaml::Value::Sequence(items)
        }
        CborValue::Map(pairs) => {
            let mut map = serde_yaml::Mapping::new();
            for (k, v) in pairs {
                if let CborValue::Text(key) = k {
                    map.insert(serde_yaml::Value::String(key.clone()), cbor_to_yaml(v));
                }
            }
            serde_yaml::Value::Mapping(map)
        }
        CborValue::Tag(_, inner) => cbor_to_yaml(inner),
        _ => serde_yaml::Value::Null,
    }
}

// ── Config handler ───────────────────────────────────────────────────────────

/// Handle `.config` (no key segment): GET the combined config root, or
/// reject deletion of the root.
async fn handle_config_root(
    message: &ma_core::Message,
    tail: Option<&str>,
    args: &[CborValue],
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    match (tail, args) {
        (None, []) => {
            let manifest = load_manifest(ctx).await?;
            let mut combined = manifest.config.clone();
            for key in [
                "zion",
                "name",
                "description",
                "plugin_envelope_queue_capacity",
            ] {
                if let Some(default_value) = default_manifest_config_value(key) {
                    combined.entry(key.to_string()).or_insert(default_value);
                }
            }
            {
                let cfg = ctx.shared_config.read().await;
                for key in DAEMON_CONFIG_KEYS {
                    let val = daemon_config_key_value_pub(&cfg, key);
                    if !val.is_null() {
                        combined.insert(key.to_string(), val);
                    }
                }
                drop(cfg);
            }
            send_crud_data_yaml(message, reply_type, ctx, &combined).await
        }
        (Some(""), _) => send_crud_i18n_error(message, reply_type, ctx, "refuse-delete-root").await,
        _ => Err(anyhow!("unknown config operation")),
    }
}

/// GET `.config.<key>`.
async fn handle_config_key_get(
    message: &ma_core::Message,
    key: &str,
    is_daemon_key: bool,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    let val = if is_daemon_key {
        let cfg = ctx.shared_config.read().await;
        daemon_config_key_value_pub(&cfg, key)
    } else {
        let manifest = load_manifest(ctx).await?;
        match manifest.config.get(key) {
            Some(v) => v.clone(),
            None => match default_manifest_config_value(key) {
                Some(default_value) => default_value,
                None => {
                    return send_crud_error(message, reply_type, ctx, "config-not-found").await;
                }
            },
        }
    };
    if let serde_yaml::Value::String(ref s) = val {
        if let Some(cid) = cidv1_ref(s) {
            return send_crud_reply_cbor(message, reply_type, ctx, &CborValue::Text(cid)).await;
        }
    }
    send_crud_data_yaml(message, reply_type, ctx, &val).await
}

/// DELETE `.config.<key>:`.
async fn handle_config_key_delete(
    message: &ma_core::Message,
    key: &str,
    is_daemon_key: bool,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    if is_daemon_key {
        return send_crud_i18n_errorf(
            message,
            reply_type,
            ctx,
            "config-key-no-delete",
            &[("key", key)],
        )
        .await;
    }
    let key = key.to_string();
    let manifest = load_manifest(ctx).await?;
    if !manifest.config.contains_key(&key) {
        return send_crud_error(message, reply_type, ctx, "config-not-found").await;
    }
    with_manifest_crud(ctx, |m| {
        m.config.remove(&key);
        Ok(())
    })
    .await?;
    send_crud_ok(message, reply_type, ctx).await
}

/// SET `.config.<key>: <value>`.
async fn handle_config_key_set(
    message: &ma_core::Message,
    key: &str,
    yaml_val: serde_yaml::Value,
    is_daemon_key: bool,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    let key = key.to_string();
    // ipv6_enable is stored in config.extra; detect changes and
    // require a restart for the new value to take effect.
    if key == "ipv6_enable" {
        let new_val = yaml_val.as_bool().unwrap_or(true);
        let current_val = ctx
            .shared_config
            .read()
            .await
            .extra
            .get("ipv6_enable")
            .and_then(serde_yaml::Value::as_bool)
            .unwrap_or(true);
        if new_val == current_val {
            return send_crud_ok_path(
                message,
                reply_type,
                ctx,
                &crate::i18n::t("ipv6-enable-unchanged"),
            )
            .await;
        }
        set_daemon_config_key(&mut *ctx.shared_config.write().await, &key, &yaml_val);
        let save_result = ctx.shared_config.read().await.save();
        if let Err(e) = save_result {
            warn!(key = %key, error = %e, "failed to save config.yaml after CRUD update");
        }
        return send_crud_ok_path(
            message,
            reply_type,
            ctx,
            &crate::i18n::t("ipv6-enable-restart-required"),
        )
        .await;
    }
    if is_daemon_key {
        if key == "outbox_backoff_attempts" {
            let attempts = outbox_backoff_attempts(Some(&yaml_val))?;
            if let Some(doc_cache) = &ctx.doc_cache {
                doc_cache.set_backoff_attempts(attempts);
            }
        }
        set_daemon_config_key(&mut *ctx.shared_config.write().await, &key, &yaml_val);
        let save_result = ctx.shared_config.read().await.save();
        if let Err(e) = save_result {
            warn!(key = %key, error = %e, "failed to save config.yaml after CRUD update");
        }
        return send_crud_ok(message, reply_type, ctx).await;
    }
    // Manifest config key — only known keys may be written.
    if !MANIFEST_CONFIG_KEYS.contains(&key.as_str()) {
        return send_crud_i18n_errorf(
            message,
            reply_type,
            ctx,
            "config-key-not-manifest",
            &[("key", key.as_str())],
        )
        .await;
    }
    if key == "plugin_envelope_queue_capacity" {
        plugin_envelope_queue_capacity(Some(&yaml_val))?;
    }
    let link_cid = if let serde_yaml::Value::String(ref s) = yaml_val {
        cidv1_ref(s)
    } else {
        None
    };
    let new_root = with_manifest_crud(ctx, |m| {
        m.config.insert(key.clone(), yaml_val.clone());
        Ok(())
    })
    .await?;
    // Language hot-swap: reload FTL messages immediately.
    if key == "i18n" {
        if let serde_yaml::Value::String(ref lang) = yaml_val {
            crate::i18n::switch_lang(lang, &ctx.kubo_rpc_url).await;
        }
    }
    send_crud_ok_cid(
        message,
        reply_type,
        ctx,
        link_cid.as_deref().unwrap_or(&new_root),
    )
    .await
}

pub(super) async fn handle_config_ns(
    message: &ma_core::Message,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx,
) -> Result<()> {
    // No key segment — operate on config root.
    if rest.is_empty() {
        return handle_config_root(message, tail, &args, reply_type, ctx).await;
    }

    let [key] = rest else {
        return Err(anyhow!("unknown config operation"));
    };

    if is_protected_config_key(key.as_str()) {
        return send_crud_i18n_errorf(
            message,
            reply_type,
            ctx,
            "config-key-protected",
            &[("key", key.as_str())],
        )
        .await;
    }

    let is_daemon_key = DAEMON_CONFIG_KEYS.contains(&key.as_str());
    match (tail, args.as_slice()) {
        (None, []) => handle_config_key_get(message, key, is_daemon_key, reply_type, ctx).await,
        (Some(""), []) => {
            handle_config_key_delete(message, key, is_daemon_key, reply_type, ctx).await
        }
        (Some(""), [value]) => {
            let yaml_val = cbor_to_yaml(value);
            handle_config_key_set(message, key, yaml_val, is_daemon_key, reply_type, ctx).await
        }
        _ => Err(anyhow!("unknown config.{key} operation")),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        cbor_to_yaml, default_manifest_config_value, is_protected_config_key_pub,
        outbox_backoff_attempts, plugin_envelope_queue_capacity, public_plugin_config,
        set_daemon_config_key_pub, wasm_reload_shutdown_timeout,
        DEFAULT_PLUGIN_ENVELOPE_QUEUE_CAPACITY, DEFAULT_RUNTIME_DESCRIPTION, DEFAULT_RUNTIME_NAME,
        DEFAULT_WASM_RELOAD_SHUTDOWN_TIMEOUT_MS,
    };
    use ciborium::Value as CborValue;
    use ma_core::Config;

    fn test_config() -> Config {
        Config {
            slug: "ma".to_string(),
            log_level: "info".to_string(),
            log_level_stdout: "warn".to_string(),
            did_resolver_positive_ttl_secs: 60,
            did_resolver_negative_ttl_secs: 10,
            log_file: None,
            kubo_rpc_url: "http://127.0.0.1:5001".to_string(),
            kubo_key_alias: "ma".to_string(),
            pin_remote: false,
            pin_remote_service: None,
            pin_remote_name: None,
            pin_overwrite: true,
            secret_bundle: None,
            secret_bundle_passphrase: Some("secret".to_string()),
            config_path: None,
            extra: serde_yaml::Mapping::new(),
        }
    }

    #[test]
    fn protects_secret_and_reserved_keys() {
        assert!(is_protected_config_key_pub("slug"));
        assert!(is_protected_config_key_pub("secret_bundle"));
        assert!(is_protected_config_key_pub("secret_bundle_passphrase"));
        assert!(is_protected_config_key_pub("config_path"));
        assert!(is_protected_config_key_pub("secret_future_field"));
    }

    #[test]
    fn allows_normal_keys() {
        assert!(!is_protected_config_key_pub("kubo_rpc_url"));
        assert!(!is_protected_config_key_pub("log_level"));
        assert!(!is_protected_config_key_pub("owners"));
    }

    #[test]
    fn cbor_scalars_map_to_yaml() {
        assert_eq!(cbor_to_yaml(&CborValue::Bool(true)).as_bool(), Some(true));
        assert_eq!(
            cbor_to_yaml(&CborValue::Text("hi".into())).as_str(),
            Some("hi")
        );
        assert_eq!(
            cbor_to_yaml(&CborValue::Integer(42_i64.into())).as_u64(),
            Some(42)
        );
    }

    #[test]
    fn cbor_sequence_dedups_preserving_order() {
        let arr = CborValue::Array(vec![
            CborValue::Text("a".into()),
            CborValue::Text("b".into()),
            CborValue::Text("a".into()),
        ]);
        let serde_yaml::Value::Sequence(items) = cbor_to_yaml(&arr) else {
            panic!("expected a YAML sequence");
        };
        assert_eq!(items.len(), 2, "duplicates should be dropped");
        assert_eq!(items[0].as_str(), Some("a"));
        assert_eq!(items[1].as_str(), Some("b"));
    }

    #[test]
    fn daemon_config_controls_wasm_reload_shutdown_timeout() {
        let mut cfg = test_config();
        assert_eq!(
            wasm_reload_shutdown_timeout(&cfg),
            std::time::Duration::from_millis(DEFAULT_WASM_RELOAD_SHUTDOWN_TIMEOUT_MS)
        );

        set_daemon_config_key_pub(
            &mut cfg,
            "wasm_reload_shutdown_timeout_ms",
            &serde_yaml::Value::Number(17_u64.into()),
        );

        assert_eq!(
            wasm_reload_shutdown_timeout(&cfg),
            std::time::Duration::from_millis(17)
        );
        assert_eq!(
            super::daemon_config_key_value_pub(&cfg, "wasm_reload_shutdown_timeout_ms").as_u64(),
            Some(17)
        );
    }

    #[test]
    fn outbox_backoff_attempts_requires_a_positive_integer() {
        assert_eq!(
            outbox_backoff_attempts(None).unwrap(),
            crate::ipfs::DEFAULT_OUTBOX_BACKOFF_ATTEMPTS
        );
        assert_eq!(
            outbox_backoff_attempts(Some(&serde_yaml::Value::Number(3_u64.into()))).unwrap(),
            3
        );
        assert!(outbox_backoff_attempts(Some(&serde_yaml::Value::Number(0_u64.into()))).is_err());
    }

    #[test]
    fn public_plugin_config_includes_public_runtime_keys_only() {
        let mut manifest = crate::entity::RuntimeManifest::default();
        manifest.config.insert(
            "root".to_string(),
            serde_yaml::Value::String("did:ma:test#root".to_string()),
        );
        manifest.config.insert(
            "start".to_string(),
            serde_yaml::Value::String("did:ma:test#construct".to_string()),
        );
        manifest
            .config
            .insert("enabled".to_string(), serde_yaml::Value::Bool(true));
        manifest
            .config
            .insert("absent".to_string(), serde_yaml::Value::Null);

        let view = public_plugin_config(&manifest, &test_config());

        assert_eq!(
            view.get("root").map(String::as_str),
            Some("did:ma:test#root")
        );
        assert_eq!(
            view.get("start").map(String::as_str),
            Some("did:ma:test#construct")
        );
        assert_eq!(view.get("enabled").map(String::as_str), Some("true"));
        assert_eq!(
            view.get("kubo_rpc_url").map(String::as_str),
            Some("http://127.0.0.1:5001")
        );
        assert_eq!(
            view.get("name").map(String::as_str),
            Some(DEFAULT_RUNTIME_NAME)
        );
        assert_eq!(
            view.get("description").map(String::as_str),
            Some(DEFAULT_RUNTIME_DESCRIPTION)
        );
        assert!(!view.contains_key("absent"));
        assert!(!view.contains_key("secret_bundle_passphrase"));
        assert!(!view.contains_key("config_path"));
    }

    #[test]
    fn default_manifest_config_value_exposes_runtime_metadata_defaults() {
        assert_eq!(
            default_manifest_config_value("name").and_then(|v| v.as_str().map(str::to_string)),
            Some(DEFAULT_RUNTIME_NAME.to_string())
        );
        assert_eq!(
            default_manifest_config_value("description")
                .and_then(|v| v.as_str().map(str::to_string)),
            Some(DEFAULT_RUNTIME_DESCRIPTION.to_string())
        );
        assert_eq!(
            default_manifest_config_value("plugin_envelope_queue_capacity")
                .and_then(|v| v.as_u64()),
            Some(DEFAULT_PLUGIN_ENVELOPE_QUEUE_CAPACITY as u64)
        );
    }

    #[test]
    fn plugin_envelope_queue_capacity_requires_positive_platform_integer() {
        assert_eq!(
            plugin_envelope_queue_capacity(None).unwrap(),
            DEFAULT_PLUGIN_ENVELOPE_QUEUE_CAPACITY
        );
        assert_eq!(
            plugin_envelope_queue_capacity(Some(&serde_yaml::Value::from(17))).unwrap(),
            17
        );
        assert!(plugin_envelope_queue_capacity(Some(&serde_yaml::Value::from(0))).is_err());
        assert!(plugin_envelope_queue_capacity(Some(&serde_yaml::Value::from("17"))).is_err());
        assert!(plugin_envelope_queue_capacity(Some(&serde_yaml::Value::from(-1))).is_err());
    }
}
