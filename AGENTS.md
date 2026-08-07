# 間 Runtime (`ma`)

## Agent rules

- **Never modify files outside the current workspace without explicit user approval.** Always ask first.

---

A lean daemon that exposes `/ma/ipfs/0.0.1` and `/ma/rpc/0.0.1` on behalf of
clients that cannot reach the Kubo RPC API directly (e.g. browser-based 間
actors). It runs on a host with a Kubo daemon, derives its own `did:ma` identity
at startup, publishes its own DID document, then handles two services over iroh
QUIC transport:

- **`/ma/ipfs/0.0.1`** — optional (enabled by `ipfs_publisher: true` in config, default `true`);
  receives signed IPFS-publish requests and publishes
  `did:ma` DID documents to IPFS/IPNS via Kubo on behalf of the caller.
- **`/ma/rpc/0.0.1`** — receives RPC messages; responds to `:ping` atoms with
  `:pong` replies using the `/ma/rpc/0.0.1` transport.

A minimal status HTTP server runs on `127.0.0.1:5003` (configurable).

## Design principles

- **No backward compatibility.** This is active development. Clean, simple code
  is preferred over compatibility shims for hypothetical users. Remove old forms
  without hesitation when a better design emerges.
- **British English for project-owned language.** Use British English in naming,
  code comments, docs, log/i18n keys, local protocol fields, and schema fields:
  `behaviour`, `authorise`, `authorised`, `authorisation`, `initialise`,
  `initialised`, `serialise`, `colour`, and `licence`. Keep externally mandated
  API names unchanged, such as Rust/serde `Serialize`/`Deserialize`, CSS
  `color`, HTTP/Web/API terms, crate names, and upstream protocol names.
- **Two services, nothing more.** Only `/ma/ipfs/0.0.1` and `/ma/rpc/0.0.1`
  are registered. No gossip, no additional RPC.
- **No local protocol code.** All publish logic, validation, secret-bundle
  handling, config, ACL, and transport are provided by the `ma-core` crate.
  Local code is nothing but glue.
- **Entity lifecycle.** Every entity tracks a `Lifecycle` state (`new` →
  `running` | `error`; `stopped` on shutdown). `Lifecycle::New` is set on
  creation. After `init()` returns `:ok` the state becomes `Running`; a CBOR
  `[:error, reason]` reply sets it to `Error` (plugin still dispatchable for
  debug). `Stopped` is written to the manifest on clean shutdown only.
  Kinds without `init` in their API skip the call and start `Running` directly.
- **No lifecycle shortcuts for native entities.** `Evaluator::Native` means the
  implementation is compiled into the runtime; it does **not** mean the entity
  may bypass manifest loading, lifecycle stages, per-entity queues, ACLs, state
  save/restore, or kind metadata. Native entities must be loaded from
  `RuntimeManifest.entities` just like Extism entities, and statefulness must be
  derived from the referenced `KindNode` (`KindNode::plugin_kind()`), never from
  hardcoded knowledge about a specific native actor such as `#scheduler`.
- **Plugin evaluator.** Each `KindNode` carries an `Evaluator` field (default:
  `Extism`). Only `Extism` is implemented; `Native`, `Bash`, and `Lua` are
  reserved for future use. `load()` returns `Err` if asked to load a kind with
  an unsupported evaluator.
- **ACL groups via the manifest `grp` registry.** Groups in `AclMap`s use the
  flat `+<name>` principal syntax (e.g. `+gurus`, `+owners`) — no nested path,
  no `#fragment`. `<name>` is looked up directly in `manifest.grp` (CRUD-
  addressed as `/grp/<name>`), an IPLD link to a plain `Vec<String>` of member
  DIDs. Resolution is a synchronous, in-memory `GroupCache` lookup (see
  `acl.rs`'s `GroupCache`/`new_group_cache`) — no actor dispatch, no async
  RPC round-trip. `"owners"` is one ordinary entry in `grp`, resolved exactly
  like any other group; it is protected only against deletion (see
  `crud/grp.rs`), never resolved specially. There is no `#fragment`-actor-
  probe mechanism (the old `+#<fragment>`/`ma-set`/`query_actor_group`
  design) — it was removed as a fundamentally wrong conflation of "group"
  (a `+`-prefixed principal) with "local entity address" (a bare `#fragment`
  principal, which remains valid and unrelated, e.g. `#idjit: null`).
- **Keys in memory only.** IPNS private key material arriving in a request is
  used once and immediately zeroized (`zeroize`). The daemon's own keys live in
  an encrypted `SecretBundle` on disk, decrypted into memory at startup and
  never written out again. The ipns key bytes are also zeroized after the own
  DID document is published.
- **Own identity published at startup.** The daemon derives its `did:ma` from
  `ipns_secret_key` via `libp2p-identity`, builds a signed `Document` with
  `did_signing_key` and `did_encryption_key`, and publishes it to Kubo before
  accepting any connections.
- **Strict input validation.** Every incoming CBOR envelope on `/ma/ipfs/0.0.1`
  carrying an identity-publish request is validated by
  `validate_identity_publish_request`, which parses the signed message, checks
  content-type, validates and verifies the DID document (including its proof
  signature), and asserts that the sender's IPNS identity matches the
  document's DID.
- **Replay protection.** A `ReplayGuard` (sliding 120-second window) is applied
  to `/ma/ipfs/0.0.1` messages before any processing.
- **ACL with deny-wins semantics.** An explicit `null` entry in the `AclMap`
  denies a principal and overrides any wildcard allow. Capabilities are plain
  strings in YAML sequences — `/ma/rpc/0.0.1` requires `"rpc"`,
  `/ma/ipfs/0.0.1` requires `"ipfs"` for generic content storage
  (`application/vnd.ma.ipfs.request`) or `"identity-publish"` for DID-document
  publishing (`application/vnd.ma.identity.publish.request`) — these are two
  independent capabilities gating two independent message types on the same
  protocol.
- **Manifest is the source of truth; ACLs are derivatives.** `RuntimeManifest`
  paths are canonical. ACLs must always be derived from and kept in sync with
  manifest data, never the reverse. Concretely:
  - `/grp/owners` (an entry in `manifest.grp`, an IPLD link to a `Vec<String>`
    document) is the authoritative owners list. `stats.owners` (the in-memory
    fast-path used by `is_owner()`) and the live root `AclMap` (via
    `grant_owners_in_acl`) are derived from it and must be updated whenever
    it changes.
  - On bootstrap: owners are written to `grp.owners` (`BootstrapRuntime.grp`)
    and, if present, injected into the published root ACL.
  - On startup: owners are merged from config.yaml, `/grp/owners`, and
    `--owner` CLI args (manifest takes precedence; the merged result is
    published back to `/grp/owners` if it introduced anything new).
  - On CRUD SET `/grp/owners`: `grant_owners_in_acl` and `stats.owners` are
    updated immediately (hot-swap, no restart needed) — see `crud/grp.rs`.
  - `/grp/owners` can never be deleted via CRUD (may be set to an empty
    list, but the entry itself must always exist) — enforced in
    `crud/grp.rs`.
  - Never read the ACL to discover owners — read `/grp/owners`.
- **Never default or fall back to open ACLs.** An empty `AclMap` (no entries)
  denies everyone. Code must never construct or substitute an open ACL
  (`{"*": ["*"]}`) as a fallback for a missing or unreadable ACL document.
  A missing ACL is a configuration error — fail loudly rather than silently
  opening access. The `:acl:` delete verb on the root ACL is therefore a no-op;
  to change the ACL, replace it with a new published CID via `:acl: <cid>`.
- **Actors communicate exclusively via message passing.** One entity plugin must
  never invoke another entity's handler directly. All inter-entity communication
  goes through the per-entity message queue. `ma_send` (fire-and-forget) is the
  only inter-actor primitive. There is no `ma_call` — synchronous request-reply
  between actors is not supported. Replies are ordinary messages matched by
  `reply_to` message-ID. This is not a style preference; it is the only
  architecture that scales to hundreds of thousands of entities.
- **Per-entity message queues.** Each entity has a dedicated `tokio::mpsc`
  channel. The RPC handler routes incoming messages non-blockingly to the correct
  entity channel. Each entity processes its own queue sequentially in a spawned
  tokio task. Entities never block one another.

## Dependencies

Only published crates — **never local paths**:

```toml
anyhow = "1"
axum = { version = "0.7", default-features = false, features = ["http1", "tokio"] }
ciborium = "0.2"
clap = { version = "4", features = ["derive"] }
directories = "5"
ma-core = { version = "0.13", default-features = false, features = ["config", "kubo", "iroh", "acl"] }
serde_json = "1"
serde_yaml = "0.9"
tokio = { version = "1", features = ["macros", "rt-multi-thread", "signal", "time", "sync"] }
tracing = "0.1"
zeroize = "1"
```

`ma-core 0.13` exposes everything this daemon uses for DID handling, so no
direct `ma-did` dependency is required.

## Configuration

The default slug is `ma`. Config, secret bundle, and log file
follow XDG paths via ma-core:

| File | Default path |
|------|-------------|
| Config | `$XDG_CONFIG_HOME/ma/ma.yaml` |
| Secret bundle | `$XDG_CONFIG_HOME/ma/ma.bin` |
| ACL | `$XDG_CONFIG_HOME/ma/ma.acl` (optional) |
| Log | `$XDG_DATA_HOME/ma/ma.log` |

`secret_bundle_passphrase` must be set (env `MA_SECRET_BUNDLE_PASSPHRASE`, or in the YAML config).

`kubo_rpc_url` defaults to `http://127.0.0.1:5001`.

### Config key categories

**`slug`** may be stored in `config.yaml`. The config path is selected before
YAML is loaded: `--config` wins, then `--slug` / `MA_SLUG`, then the default
slug. After that selected file is read, its `slug:` value sets the effective
runtime slug and derived defaults. YAML `slug` must never select, redirect, or
reload a config file; CLI/environment slug overrides it.

**Protected keys** — never exposed or writable via `:config.*` RPC:

| Key | Reason |
|-----|--------|
| `slug` | Runtime identity/default paths; set in config.yaml or before startup, never through RPC |
| `secret_bundle` | Key material path — must not leak |
| `secret_bundle_passphrase` | Secret — must never be exposed |
| `config_path` | Internal path — not user-settable via RPC |
| any key starting with `secret` | Blanket guard for future secret fields |

**Daemon config keys** — readable and writable via `:config.<key>` RPC;
changes take effect immediately in memory and are saved to `config.yaml`:

| Key | Type | Description |
|-----|------|-------------|
| `kubo_rpc_url` | string | Kubo RPC API URL (effective on next IPFS call) |
| `kubo_key_alias` | string | IPNS key alias in Kubo |
| `log_level` | string | Log level for the log file |
| `log_level_stdout` | string | Log level for stdout |
| `did_resolver_positive_ttl_secs` | u64 | Cache TTL for resolved DIDs |
| `did_resolver_negative_ttl_secs` | u64 | Cache TTL for failed DID lookups |
| `log_file` | string or null | Path to log file |
| `root_cid` | string | Runtime manifest head CID; read at startup and updated whenever the head changes |

**Manifest config keys** — stored in the IPFS DAG (`manifest.config`), not in
`config.yaml`. These persist across restarts only because they live in IPFS.

| Key | Type | Description |
|-----|------|-------------|
| `i18n` | string | Active language BCP-47 tag (e.g. `nb`, `zh-Hans`) |
| other | any | Free-form runtime metadata |

Setting `i18n` via `:config.i18n: nb` takes effect immediately (calls
`switch_lang()` to reload FTL translations in memory) and persists to IPFS.

Startup head resolution order is `--root-cid`, `--bootstrap`, `root_cid` from
`config.yaml`, then runtime IPNS. Runtime manifest mutations advance the
in-memory head without rewriting `config.yaml`; the current `root_cid` is
saved after a successful periodic republish and during graceful shutdown.

### IPFS publisher toggle

Add to `~/.config/ma/ma.yaml` to disable the IPFS publisher service:

```yaml
ipfs_publisher: false
```

The key lives in `config.extra` (a `serde_yaml::Mapping`). Default is `true`
(enabled) when the key is absent.

### First-time setup

```sh
ma --gen-headless-config
```

Generates a fresh `SecretBundle` with four random 32-byte keys (`iroh_secret_key`,
`ipns_secret_key`, `did_signing_key`, `did_encryption_key`), encrypts it with a
random passphrase, and writes both config and bundle to the XDG paths with mode
`0600`.

### Runtime

```sh
ma
# or with an explicit ACL file:
ma --acl-file /etc/ma/acl.yaml
# or with a custom local status bind address:
ma --status-bind 127.0.0.1:5004
```

## CLI flags

| Flag | Default | Description |
|------|---------|-------------|
| `--acl-file <PATH>` | — | ACL YAML file; defaults to open (`*`) if omitted |
| `--poll-ms <MS>` | `100` | Service poll interval |
| `--status-bind <ADDR>` | `127.0.0.1:5003` | Status web server bind address |
| `--gen-headless-config` | — | Generate config + secret bundle and exit |

## ACL format

The ACL YAML must contain an `acl:` map from principal to a YAML sequence of
capability strings. The default when no file is supplied is open
(`"*": [rpc, ipfs, identity-publish]`).

```yaml
acl:
  "*": [rpc, ipfs, identity-publish]  # everyone: RPC + IPFS store + DID publish
  "did:ma:alice": ["*"]       # alice: all capabilities
  "did:ma:bob": [rpc]         # bob: RPC only, no IPFS store or DID publish
  "did:ma:eve":               # null = explicit deny
```

Built-in capability strings:

| Capability | Required by |
|------------|-------------|
| `"rpc"` | `/ma/rpc/0.0.1` |
| `"ipfs"` | `/ma/ipfs/0.0.1` (generic content storage) |
| `"identity-publish"` | `/ma/ipfs/0.0.1` (DID-document publishing) |
| `"read"` | (reserved — future read-only access) |
| `"create"` | Create namespaces/entities |
| `"update"` | Update namespaces/entities |
| `"delete"` | Delete namespaces/entities |
| `"*"` | Wildcard — grants all capabilities when used in an Allow set |

Arbitrary capability strings are valid — entity and namespace ACLs may use
verb names or sub-namespace names as capabilities.

Rules:

- **Deny always wins** over wildcard allow. An explicit `null` entry (bare key,
  `key: ~`, or `key: null` in YAML) is an explicit deny.
- Direct principal lookup wins over the `"*"` wildcard.
- Fragment stripped from DID-URLs before lookup.
- ACL is checked on every incoming message on both services.

## Status web server

Runs on `127.0.0.1:5003` (or `--status-bind`). Two endpoints:

| Endpoint | Content | Description |
|----------|---------|-------------|
| `GET /` | `text/html` | Human-friendly status page |
| `GET /status.json` | `application/json` | Compact JSON status object |

The JSON object contains:

```json
{
  "did": "did:ma:<ipns>",
  "endpoint_id": "<iroh-id>",
  "uptime_secs": 42,
  "ipfs_publisher": true,
  "ipfs_requests": 0,
  "rpc_requests": 0,
  "pings_received": 0,
  "started_at": 1234567890
}
```

## Wire format rules

**All data over iroh transport is CBOR. No JSON is sent between peers.**

- RPC requests: CBOR atom (`:verb`) or array `[":verb", arg1, arg2, …]`.
- RPC replies: CBOR atom (`:pong`, `:ok`, `:error`) or tuple `[":ok", payload]` / `[":error", reason]`.
- Entity content in replies: CBOR-encoded `EntityNode` (same structure as
  stored in IPFS DAG-CBOR), never JSON.
- Entity definitions written by users in zion use **YAML** as the human-readable
  format. YAML is stored to IPFS via `dag_put` (DAG-CBOR), and the resulting
  CID is the canonical reference.

The one exception: Kubo's HTTP RPC API (`/api/v0/…`) speaks JSON. That is an
internal implementation detail of `crate::kubo` and is invisible to peers.

**Extism host-function traffic is a separate wire boundary from the above,
and is just raw bytes — not CBOR by default.** `extism-pdk`'s `#[host_fn]`
macro (guest side) serialises `String`/`Vec<u8>` arguments and return values
via `ToBytes`/`FromBytes`, both **identity** for those two types (raw UTF-8
bytes / raw bytes, no envelope of any kind). Any CBOR seen on a host
function's `input`/return bytes (e.g. `ma_set_state`) is a **calling-
convention choice made by that specific guest library** (Python actors
manually CBOR-encode via `cbor2` to match what `python-ma-actors`' actor
library expects), not something Extism or this runtime imposes. A Rust
guest calling a host function via the `#[host_fn]` macro (e.g. the
`lambda-ma/scheme-actor` guest) gets raw bytes both ways unless it manually
CBOR-encodes/decodes itself — **the host-side implementation of any given
host function must match whatever encoding its actual callers use**, not
assume CBOR uniformly. Mixing this up (host expects CBOR, guest sends raw
bytes, or vice versa) produces a wasm trap at call time with a generic
"error while executing" backtrace, not a clear encoding-mismatch error —
confirmed by hitting this exact bug once with `ma_ipfs_include` (host
originally assumed CBOR input; the Rust guest macro sends raw UTF-8; fixed
by reading `input` as `String::from_utf8` on the host side instead of
`from_cbor_bytes`).

## RPC protocol

Content types are defined in ma-spec, not ma-core — they are string literals:

| Direction | Content-Type |
|-----------|--------------|
| Request | `application/vnd.ma.rpc.request` |
| Reply | `application/vnd.ma.rpc.reply` |

RPC verbs are CBOR-encoded text strings beginning with `:`.

### Dot-path grammar

Unfragmented RPC messages (addressed to `did:ma:<ipns>`, no fragment) use a
dot-path grammar rooted in four namespaces:

```
:entities[.<name>][:<verb>]  — entity management
:kinds[.<family>[.<impl>]]   — kind/protocol registry (read-only)
:config[.<key>]              — runtime config
:ping                        — liveness check
```

| Pattern | Meaning |
|---------|---------|
| `:entities` | list all entity names |
| `:entities.<name>` | get EntityNode (as CBOR) |
| `:entities.<name>:` | delete entity |
| `:entities.<name>: <cid>` | upsert entity by CID (fetches DAG-CBOR from IPFS) |
| `:entities.<name>:edit` | return current EntityNode for client-side editing |
| `:ping` | reply `:pong` |

Fragment-addressed messages (`did:ma:<ipns>#<name>`) are routed directly to
the named entity plugin (Wasm `on_message`).

### `:edit` verb

`:entities.<name>:edit` returns the current `EntityNode` as CBOR. The **client**
(zion) is responsible for opening an editor so the user can modify it. After
editing, the client publishes the updated node to IPFS (`dag_put`), then sends
`:entities.<name>: <new-cid>` to register it. The runtime never initiates an
editor session; it only stores and retrieves by CID.

### `:ping`

Replies with `:pong` to `did:ma:<sender_ipns>#ping`. The reply sets `reply_to`
to the originating message's ID and is delivered via `endpoint.outbox()`.

### `#scheduler` — native schedule actor

`#scheduler` is a compiled-in native actor (not Wasm). Plugins register timed
dispatches by sending a CBOR array to `did:ma:<ipns>#scheduler` via `ma_send`.

**Wire format** — 4 required elements, optional extra args at position 5+:

```
[":cron",     spec_str,     target_frag, verb_or_array, extra_args…]
[":interval", duration_str, target_frag, verb_or_array, extra_args…]
[":at",       timestamp_ms, target_frag, verb_or_array, extra_args…]
[":random",   max_secs_int, target_frag, verb_or_array, extra_args…]
```

| Field | Type | Description |
|-------|------|-------------|
| type | text atom | `:cron`, `:interval`, `:at`, or `:random` |
| spec | text / integer | cron string, duration string, Unix ms timestamp, or max_secs integer |
| target_frag | text | bare fragment name (`"myentity"`) or full DID-URL (`did:ma:…#myentity`) |
| verb_or_array | text atom or array | `":verb"` or `[":verb", arg1, …]` |
| extra_args… | any CBOR | optional positional args appended after the verb |

**Schedule types:**

| Type | Spec | Behaviour |
|------|------|-----------|
| `:cron` | 6-field cron `"sec min hour day month weekday"` | Fires on schedule indefinitely |
| `:interval` | Human duration: `"1h"`, `"30m"`, `"5s"`, `"2d12h"` | Fires every N seconds indefinitely |
| `:at` | Unix timestamp in milliseconds (integer) | Fires once after the computed delay |
| `:random` | Max seconds (integer) | Fires after 1–N random seconds, then self-reschedules |

**Examples:**

```cbor
; Fire :tick on myentity every minute
[":cron", "0 * * * * *", "myentity", ":tick"]

; Fire [:grow, "small plant+=1", "bigplant+=4"] on garden every 30 minutes
[":interval", "30m", "garden", ":grow", "small plant+=1", "bigplant+=4"]

; Same, using array form for verb
[":interval", "30m", "garden", [":grow", "small plant+=1", "bigplant+=4"]]

; One-shot at a specific time
[":at", 1748700000000, "myentity", ":wake"]

; Random re-trigger within 5 minutes
[":random", 300, "dog", ":scratch"]
```

**ACL:** Scheduled dispatch bypasses all ACL checks. The runtime is the trusted
caller. Schedules are not persisted — they must be re-registered on each startup,
typically from a plugin's `init()` when `lifecycle == "new"` or always on init.

## ma-core API used

| Purpose | Call |
|---------|------|
| Config + CLI | `Config::from_args(&args, MA_DEFAULT_SLUG)` |
| First-time config | `Config::gen_headless(&args, MA_DEFAULT_SLUG)` |
| Key material | `SecretBundle::load(path, passphrase)` |
| IPNS derivation | `libp2p_identity::ed25519::SecretKey::try_from_bytes` → `Keypair` → `PeerId::to_base58()` |
| Own DID document | `Document::new`, `SigningKey::from_private_key_bytes`, `EncryptionKey::from_private_key_bytes`, `VerificationMethod::new`, `document.sign`, `document.marshal` |
| iroh endpoint | `ma_core::new_ma_endpoint(iroh_secret_key)` |
| Register service | `endpoint.service("/ma/ipfs/0.0.1")` + `endpoint.service("/ma/rpc/0.0.1")` |
| Kubo publisher | `IpfsDidPublisher::new(kubo_rpc_url)` |
| Kubo readiness | `publisher.wait_until_ready(attempts)` |
| Request validation | `validate_ipfs_publish_request(message_cbor)` |
| Publish | `publisher.publish_document(did_doc_json, ipns_key_b64)` |
| Replay guard | `ReplayGuard::default()` + `check_and_insert(&headers)` |
| ACL | `AclMap` (serde-deserialised from YAML) + `check_cap(acl, caller, cap)` |
| Outbox (pong) | `endpoint.outbox(&resolver, &sender_did, "/ma/rpc/0.0.1").await` → `outbox.send(&msg)` |
| Resolver | `IpfsGatewayResolver::new(kubo_rpc_url)` |

## Security notes

- `application/vnd.ma.identity.publish.request` payloads **must** be encrypted envelopes per
  the ma-spec (messaging-format.md §2.2.1). The iroh transport provides the
  encrypted channel; `validate_ipfs_publish_request` enforces content-type.
- The IPNS private key embedded in each `/ma/ipfs/0.0.1` request is the
  sender's full publishing authority over their DID. It is used once and
  zeroized immediately after the Kubo call.
- The daemon's own `ipns_secret_key` bytes are zeroized immediately after the
  own DID document is published at startup.
- The daemon carries no signing or encryption keys of its own beyond those
  needed for transport and its own DID identity — it cannot impersonate any
  other `did:ma` identity.
- All files written by ma-core (config, bundle) use mode `0600`.
- The `iroh_secret_key` is only for the iroh QUIC transport layer; it is
  distinct from `ipns_secret_key` which roots the `did:ma` identity.

## Internationalisation — `src/i18n.rs` + `lang/`

Translation strings use `key = value` lines only. No attributes, selectors, or
substitutions — all runtime keys are plain declarative log messages with no
`{ $var }` placeholders.

- `lang/en.ftl` — **canonical source**; defines all keys.
- `lang/*.ftl` — all other supported locales.
- Missing keys fall back to the key name string.
- Technical terms kept verbatim: DID, IPFS, IPNS, RPC, ACL, iroh, CID,
  `#root`, `/ma/ipfs/0.0.1`, `:ping`, `:pong`, Bootstrap, headless, Plugin,
  manifest.
- **When adding or changing any logged string**, update `lang/en.ftl` first,
  then add/update the same key in every `lang/*.ftl` file that exists.
  Never leave a key missing from any locale file.
- **NEVER copy English text into non-English locale files.** Every non-`en.ftl`
  file must have a genuine translation for every key. English text in a
  non-English `.ftl` file is worse than a missing key (which falls back to the
  key name string); it silently misleads users who do not read English.
  Translate properly or leave the key absent — never paste the English value.

### `lang-name` key

Every `lang/*.ftl` file **must** contain a `lang-name` key whose value is the
language's own name for itself (autonym), e.g. `lang-name = Norsk bokmål`.

### Adding a new language

1. Create `lang/<code>.ftl` with all keys from `lang/en.ftl` translated,
   including `lang-name = <autonym>`.
2. Rebuild (`cargo build`). `build.rs` scans `lang/*.ftl` and regenerates
   `BUNDLED_LANGS` automatically — no manual code change required.

`BUNDLED_LANGS` is written to `$OUT_DIR/bundled_langs.rs` and `include!`-ed
into `src/i18n.rs`. All FTL files in `lang/` are compiled into the binary.

### Notable constructed / special languages

| Code | Language | Notes |
|------|----------|---------|
| `art-x-lyaric` | Dread Talk (Rasta) | BCP-47 private-use tag for Lyaric / Rastafarian Iyaric dialect |
| `qbc` | Belter Creole | The conlang from *The Expanse* (Belter lang); ISO 639-3 code `qbc` |
