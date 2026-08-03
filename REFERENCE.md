# 間 Runtime (`ma`)

A lean Tokio daemon that bridges browser-based `did:ma:` identities to Kubo
(IPFS) and manages a Wasm entity/plugin system over iroh QUIC transport.

---

## Design philosophy

間 is built around one idea: **secure messaging and actor interaction should
be fast and lightweight**, not just correct.

The cryptographic primitive choices follow the same school of thought as
[WireGuard](https://www.wireguard.com/) — pick the fastest algorithms that are
still thoroughly secure, keep the code small, and avoid complexity:

| Concern | 間 choice | Why |
|---------|-----------|-----|
| Wire format | CBOR | Compact, schema-free, no string parsing |
| Hashing | BLAKE3 | Fast and modern — same school as WireGuard |
| Key agreement | X25519 | Small, fast, well-understood |
| Signatures | Ed25519 | Small keys, fast verification, no surprises |
| Transport | iroh QUIC | Hole-punching, NAT traversal, end-to-end |
| Identity | W3C DID | Decentralised, self-describing, auditable |

The result is a system that is not [DIDComm](https://identity.foundation/didcomm-messaging/spec/)
— it does not implement that standard — but shares its security model:
sender-authenticated, end-to-end encrypted messages rooted in
[W3C DID](https://www.w3.org/TR/did-core/) identities. The wire format is
leaner and the transport is faster. No JSON parsing in the hot path. No round
trips that are not necessary.

**Why Rust?** [iroh](https://iroh.computer/) is an IPFS spinoff — a leaner,
faster successor to the original libp2p QUIC stack. At the moment iroh is only
available as a Rust crate, so the language choice is not ideological: it is
simply the price of admission for the best transport available.

`ma` has not been independently audited. The cryptographic design is sound,
but treat it as a well-engineered tool under active development, not a
certified security product.

---

## Design principles

- **No backward compatibility.** This is active development. Clean, simple
  code is preferred over compatibility shims for hypothetical users. Remove
  old forms without hesitation when a better design emerges.
- **No local protocol code.** All publish logic, validation, secret-bundle
  handling, config, ACL, and transport are provided by `ma-core`. Local code
  is nothing but glue.
- **Entity lifecycle.** Every entity tracks a `Lifecycle` state:
  `New` → `Running` | `Error`; `Stopped` on graceful shutdown. `New` is set
  on creation. After `init()` returns `:ok` the state becomes `Running`; a
  CBOR `[:error, reason]` reply sets it to `Error` (plugin remains
  dispatchable for debugging). `Stopped` is written to the manifest only on
  clean shutdown. Kinds whose `api` list does not include `init` skip the
  call and start `Running` directly.
- **Plugin evaluators.** Each `KindNode` carries an `Evaluator` field
  (default: `Extism`). Only `Extism` is implemented; `Native`, `Bash`, and
  `Lua` are reserved for future use. `load()` returns `Err` if asked to load
  a kind with an unsupported evaluator.
- **Manifest is the source of truth; ACLs are derivatives.**
  `/grp/owners` (an entry in `manifest.grp`) is the authoritative owners list.
  The in-memory root `AclMap` and stats are derived from it and must be
  updated whenever it changes. Never read the ACL to discover owners — read
  `/grp/owners`.
- **Never fall back to an open ACL.** An empty `AclMap` (no entries) denies
  everyone. Code must never substitute `{"*": ["*"]}` as a fallback for a
  missing ACL. A missing ACL is a configuration error — fail loudly. The
  `:acl:` delete verb on the root ACL is therefore a no-op; replace it with
  a new CID via `:acl: <cid>`.
- **Keys in memory only.** IPNS private key material arriving in a request
  is used once and immediately zeroized. The daemon's own keys live in an
  encrypted `SecretBundle` on disk, decrypted into memory at startup and
  never written out again.
- **Strict input validation.** Every incoming CBOR envelope on
  `/ma/ipfs/0.0.1` is validated by `validate_ipfs_publish_request`: checks
  content-type, DID document proof signature, and that the sender's IPNS
  identity matches the document's DID.
- **Replay protection.** A `ReplayGuard` (sliding 120-second window) is
  applied to `/ma/ipfs/0.0.1` messages before any processing.

---

## Overview

`ma` exposes three iroh QUIC services on behalf of clients that cannot reach the
Kubo RPC API directly (e.g. the `zion` browser actor):

| Service | Protocol ID | Purpose |
|---------|-------------|---------|
| IPFS publisher | `/ma/ipfs/0.0.1` | Receives signed DID-document publish requests and forwards them to Kubo |
| RPC | `/ma/rpc/0.0.1` | Entity/plugin dispatch, ping/pong |
| CRUD | `/ma/crud/0.0.1` | Structured data management (entities, kinds, config, namespaces) |

The daemon derives its own `did:ma:` identity at startup, publishes its own DID
document to IPFS/IPNS, and then enters the main event loop.

---

## Architecture

```
zion (browser WASM) ──iroh QUIC──► /ma/rpc/0.0.1
                                       │
                                       ├─ (unfragmented) → ping, name, description
                                       └─ #<name>        → Wasm plugin dispatch
                   ──iroh QUIC──► /ma/crud/0.0.1
                                       │
                                       ├─ :entities.*    → entity management
                                       ├─ :kinds.*       → kind registry
                                       ├─ :config.*      → runtime config
                                       └─ :namespaces.*  → namespace management
                   ──iroh QUIC──► /ma/ipfs/0.0.1
                                       │
                                       └─ publish DID document to Kubo/IPNS
```

Entity Wasm plugins are loaded from IPFS at startup.  State is persisted back
to IPFS on graceful shutdown and before a hot reload. Reload saves update the
authoritative manifest state link before the replacement plugin is loaded, so
unchanged actor state survives both the replacement and a later restart.

Plugin `ma_send` and `ma_reply` envelopes cross a bounded, in-memory queue.
They remain fire-and-forget: when the queue is full, the envelope is dropped
and the runtime logs an `ERROR` containing the source fragment and configured
capacity. The queue is not durable across process restarts.

---

## Wire format

**All data exchanged between peers over iroh transport is CBOR. Never JSON.**

| Direction | Format |
|-----------|--------|
| RPC request (atom) | CBOR text string: `:verb` |
| RPC request (with args) | CBOR array: `[":verb", arg1, arg2, …]` |
| RPC reply (success) | CBOR atom `:ok`, or tuple `[":ok", payload]` |
| RPC reply (error) | CBOR atom `:error`, or tuple `[":error", reason]` |
| Entity content in replies | CBOR-encoded `EntityNode` (DAG-CBOR structure, never JSON) |
| IPFS identity-publish request | CBOR-encoded signed envelope (`application/vnd.ma.identity.publish.request`) |
| IPFS store request | CBOR-encoded signed envelope (`application/vnd.ma.ipfs.request`) |

The one exception: the Kubo HTTP RPC API (`/api/v0/…`) speaks JSON internally.
That is a private implementation detail of the `kubo` sub-crate and is never
exposed to peers.

Entity definitions authored in zion use **YAML** as the human-readable format.
YAML is stored to IPFS via `dag_put` (DAG-CBOR), and the resulting CID is the
canonical reference.

---

## Prerequisites

- Rust (latest stable)
- [IPFS Desktop](https://docs.ipfs.tech/install/ipfs-desktop/) running locally
  (it includes Kubo on `http://127.0.0.1:5001`; set `kubo_rpc_url` in config if
  you use another Kubo endpoint)

---

## Building

```sh
cargo build --release
```

Binary: `target/release/ma`.

---

## First-time setup

### 1. Generate config and secret bundle

On the very first start `ma` detects a missing secret bundle and generates a
full headless config automatically — no manual step required. You can also
trigger generation explicitly:

```sh
ma --gen-headless-config
```

Writes four random 32-byte keys (plus a `runtime_ipns` key) encrypted with a
random passphrase to:

| File | Default path |
|------|-------------|
| Config | `$XDG_CONFIG_HOME/ma/ma.yaml` |
| Secret bundle | `$XDG_CONFIG_HOME/ma/ma.bin` |

The passphrase is printed once and also written into `ma.yaml` automatically,
so `ma` can restart without any manual input.  Store it in a password manager
as a backup.  If you prefer not to keep it in the config file, remove the
`secret_bundle_passphrase:` line and supply it via `MA_SECRET_BUNDLE_PASSPHRASE`.

### 2. Minimal runtime (no bootstrap needed)

On first start, if no `root_cid` is found in IPNS and none is given via
`--root-cid`, the daemon automatically publishes a minimal `RuntimeManifest`
to Kubo and uses it as the runtime head. This manifest contains:

- `config.owners` — empty (no owner yet)
- `acl` — absent (deny-all for all incoming transport until claimed)
- `entities`, `kinds`, `i18n` — all empty

The daemon is immediately functional for CRUD operations. To establish
ownership, run `.my.ma:claim` from zion (or `POST /claim` with your DID).
This updates the live transport ACL so you can issue RPC/CRUD requests.

If claiming remotely is not practical, set owners directly in `config.yaml`
or via `--owner` (repeatable) before starting the daemon:

```yaml
# ~/.config/ma/ma.yaml
owners:
  - did:ma:<your-ipns>
  - did:ma:<another-ipns>
```

```sh
ma --owner did:ma:<your-ipns> --owner did:ma:<another-ipns>
```

Owners listed this way are granted `["*"]` in the live transport ACL at
startup, before any manifest ACL is loaded.

To add a permanent ACL to the manifest:

```
@ma:acl: <cid>   # set an AclMap CID as the root transport-gate ACL
```

### 3. Bootstrap a world manifest (optional)

Run once after Kubo is ready to set up the entity system from a world/profile
bootstrap. For λ-間:

```sh
ma --gen-root-cid ../lambda-ma/dist/lambda-ma.yaml
```

This publishes kind nodes, entity nodes, and the root `RuntimeManifest` as
IPLD DAG-CBOR objects to Kubo, then prints the generated `root_cid` and exits.

Locale files from `i18n/` are embedded in the binary at build time via
`src/i18n.yaml` (generated by `make src/i18n.yaml`). To rebuild the locale
map after editing any `.ftl` file:

```sh
make src/i18n.yaml
make release
```

Recommended runtime model:

- Runtime head CID is read from `root_cid` in `config.yaml` at startup, falling
  back to runtime IPNS if the config key is absent.
- Runtime manifest mutations advance the head in memory without rewriting
  `config.yaml`. The current head is saved after a successful periodic
  republish and during graceful shutdown.
- Publish the current root explicitly with `@runtime#root:publish`, or rely on
  the configured periodic republish interval.
- `--root-cid <cid>` overrides both config and IPNS for the running process.

Warning for `--root-cid`:

- It resets the runtime head; the reset is persisted at the next successful
  periodic republish or graceful shutdown.
- If you pass the wrong CID, retrieve the previous CID from runtime logs and restart with that value.

On subsequent starts, the daemon restores runtime head from `config.yaml`, with
IPNS as a fallback.

#### `bootstrap.yaml` format

```yaml
runtime:
  owner: did:ma:<your-ipns>   # DID of the operator; shown by `ma --status`

  # Kinds: protocol families → implementations → metadata
  kinds:
    stateless:
      ping:
        protocol: /ma/stateless/ping/0.0.1
        wasi: false             # Rust/native Extism plugin
        api:
          - on_message          # fn on_message() — process incoming message
        host_functions:
          - ma_send             # full-control outbound envelope
      python:
        protocol: /ma/stateless/python/0.0.1
        wasi: true              # Python/WASI plugin
        api:
          - on_message
        host_functions:
          - ma_reply            # reply to sender
          - ma_send
    stateful:
      python:
        protocol: /ma/stateful/python/0.0.1
        wasi: true
        api:
          - init                # called on load with persisted state bytes
          - handle_call
          - save_state          # janitor snapshot trigger
        host_functions:
          - ma_reply
          - ma_send
          - ma_set_state        # persist state to IPFS via runtime

  # Entities: named plugin instances
  entities:
    ping:
      kind: /ma/stateless/ping/0.0.1
      behaviour_cid: bafkrei…   # CID of compiled .wasm — `ipfs add plugin.wasm`
      acl: ""                  # "" = deny-all; use a named ACL ref for open access
    fortune:
      kind: /ma/stateless/python/0.0.1
      behaviour_cid: Qm…
      acl: ""
      # owner: did:ma:<other>  # optional; falls back to runtime.owner
```

---

## Running

```sh
ma
# or
ma --acl-file /etc/ma/acl.yaml --status-bind 0.0.0.0:5003
```

---

## CLI flags

| Flag | Default | Description |
|------|---------|-------------|
| `--gen-root-cid <YAML>` | — | Publish bootstrap manifest to IPFS, print `root_cid`, then exit |
| `--gen-kinds-cid <YAML>` | — | Publish only `runtime.kinds` from bootstrap YAML to IPFS, print the kinds CID, then exit |
| `--root-cid <CID>` | — | WARNING: resets runtime head for this process; recover prior CID from logs if mistaken |
| `--kinds-cid <CID>` | — | Apply a kinds-tree CID over the selected runtime head without replacing entities or entity state |
| `--owner <DID>` | — | DID(s) of the runtime owner(s); repeat for multiple; falls back to `owners:` in config |
| `--acl-file <PATH>` | — | ACL YAML; open (`*`) if omitted |
| `--poll-ms <MS>` | `100` | Service poll interval |
| `--i18n <LANG>` | — | Log language (e.g. `nb`, `en`). Also `MA_I18N` env var. Falls back to `i18n:` in config or `nb`. |
| `--status-bind <ADDR>` | `127.0.0.1:5003` | Status HTTP server address |
| `--gen-headless-config` | — | Generate config + bundle, then exit |
| `--slug <SLUG>` | `ma` | Config/bundle slug (also `MA_SLUG` env). Never put in `ma.yaml` — see [Configuration keys](#configuration-keys-mayaml). |

---

## Startup sequence

1. Parse CLI, load config.
2. If `--gen-headless-config`: write config + bundle (including `runtime_ipns` key), exit.
3. If secret bundle is missing: auto-generate headless config and continue.
4. If `--gen-root-cid <yaml>`: wait for Kubo, publish IPLD manifest, print `root_cid`, exit.
5. If `--gen-kinds-cid <yaml>`: wait for Kubo, publish only `runtime.kinds`, print the kinds CID, exit.
6. Load ACL (deny-all until manifest is loaded later).
7. Load `SecretBundle`.
8. Create iroh QUIC endpoint, register services (RPC, IPFS publisher if enabled, CRUD if enabled).
9. Build own DID document, spawn background publish to IPNS.
10. Wait for Kubo readiness (10 attempts).
11. Resolve `root_cid` from `--root-cid` CLI or runtime IPNS. If none found, publish a
    minimal empty `RuntimeManifest` to Kubo and use that CID as the runtime head.
12. If `--kinds-cid <cid>` is present, read that CID as a `KindTree` overlay,
  apply it additively to the selected manifest, and publish a new runtime head
  whose `entities` links are preserved.
13. Fetch locale from `RuntimeManifest.config.i18n`; fall back to embedded FTL if unavailable.
14. Load Wasm entity plugins from IPFS manifest.
15. Start `#scheduler` native actor.
16. Load named ACLs into cache from manifest; replace transport-gate ACL from manifest ACL.
17. Resolve owners from CLI `--owner` and `config.extra.owners`; seed live ACL.
18. Start status HTTP server.
19. Main event loop: drain RPC, IPFS, and CRUD inboxes every `poll_ms`.
20. On `Ctrl-C`: save entity states to IPFS, update manifest CID in config, close endpoint.

---

## Entity / plugin system

### Concept

All entities are created and managed via CRUD messages to the runtime.
Entity behaviour lives in Wasm blobs stored on IPFS.

### IPLD schema (DAG-CBOR on Kubo)

```
RuntimeManifest { owner, kinds: {family→implementation→CID}, entities: {name→CID→EntityNode}, acl: CID?, acls: {name→CID} }
KindNode        { protocol }
EntityNode      { name, kind, behaviour: {"//":CID}, state: {"//":CID}?, owner, acl: {"//":CID}, wasi: bool }
```

### Management CRUD — dot-path protocol

Send CBOR to the **bare** runtime DID (`did:ma:<runtime>`, no fragment) on
`/ma/crud/0.0.1`. Operations are encoded in the CBOR payload:

```
GET:    [":get",    ".ns.key"]
SET:    [".ns.key", value]
DELETE: [":delete", ".ns.key"]
```

#### Entities

| Path | GET | SET | DELETE |
|------|-----|-----|--------|
| `:entities` | List all entity names | — | — |
| `:entities.<name>` | Get EntityNode | — | — |
| `:entities.<name>` (SET) | — | Create/update entity by CID | — |
| `:entities.<name>` (DELETE) | — | — | Delete entity |

#### Kinds

| Path | GET | SET | DELETE |
|------|-----|-----|--------|
| `:kinds` | List all kinds | — | — |
| `:kinds.<family>` | List implementations | — | — |
| `:kinds.<family>.<impl>` | Get kind ref | Set kind ref CID | Delete kind ref |

#### Config

| Path | GET | SET | DELETE |
|------|-----|-----|--------|
| `:config` | Get entire config map | — | — |
| `:config.<key>` | Get value | Set value | Delete key |

#### Root RPC metadata lookups

Send a CBOR text atom to the bare runtime DID on `/ma/rpc/0.0.1`:

| Verb | Reply | Backing config key | Default |
|------|-------|--------------------|---------|
| `:ping` | `:pong` | — | — |
| `:name` | runtime name string | `config.name` | `間trix` |
| `:description` | runtime description string | `config.description` | `A 間 runtime with a lazy owner.` |

`name` and `description` are read via root RPC for display/discovery, but are
still changed through CRUD (`:config.name:` and `:config.description:`).

#### From zion terminal

```
# list entities
@sky:entities

# get entity
@sky:entities.ping

# create / update entity (CID is the EntityNode dag-cbor CID)
@sky:entities.ping: bafyreiXXXXXXXX

# delete entity
@sky:entities.ping:

# set a kind
@sky:kinds.ma-entity.wasi: bafyreiYYYYYYYY

# set a config key
@sky:config.poll_interval_ms: 250

# set runtime name/description (reflected by @sky:name / @sky:description)
@sky:config.name: "間trix"
@sky:config.description: "A 間 runtime with a lazy owner."
```

### Plugin ABI

Each Wasm plugin must export:

| Export | Description |
|--------|-------------|
| `init(state: Bytes)` | Called at load with persisted state (empty bytes on first load) |
| `on_message(PluginInput JSON) → PluginOutput JSON` | Handle an incoming RPC message |
| `get_state() → Bytes` | Return current state for IPFS persistence |

**`PluginInput`:**
```json
{
  "msg": {
    "id": "…", "from": "did:ma:…", "to": "did:ma:…#name",
    "created_at": 1234567890123456789,
    "expires": 1234567890,
    "reply_to": null,
    "content_type": "application/cbor",
    "content": [/* bytes */]
  },
  "ctx": { "self": "did:ma:…#name", "owner": "did:ma:…" }
}
```

**`PluginOutput`:**
```json
{ "reply": { "content_type": "application/json", "content": [/* bytes */] } }
```
`"reply": null` for no reply.

### Entity lifecycle states

| State | Meaning |
|-------|---------|
| `New` | Just created; `init()` has not yet been called |
| `Running` | `init()` returned `:ok`, or kind has no `init` in its API |
| `Error` | `init()` or a dispatch returned `[:error, reason]`; still dispatchable for debugging |
| `Stopped` | Written to manifest on clean shutdown only |

---

## `#scheduler` — native schedule actor

`#scheduler` is a compiled-in native actor (not Wasm). Plugins register timed
dispatches by sending a CBOR array to `did:ma:<runtime>#scheduler`.

The scheduler also accepts `:help` and returns a short text summary of the
supported schedule types and wire format.

Schedules are **not persisted** — they must be re-registered on each startup,
typically from a plugin's `:start` handler.

### Wire format

Four required elements; optional extra args at position 5+:

```
[name, ":cron",     spec_str,     verb_or_array, extra_args…]
[name, ":interval", duration_str, verb_or_array, extra_args…]
[name, ":at",       timestamp_ms, verb_or_array, extra_args…]
[name, ":random",   max_secs_int, verb_or_array, extra_args…]
```

| Field | Type | Description |
|-------|------|-------------|
| name | text | caller-defined deterministic schedule name |
| type | CBOR text atom | `:cron`, `:interval`, `:at`, or `:random` |
| spec | text / integer | cron string, duration string, Unix ms timestamp, or max_secs integer |
| verb_or_array | text atom or array | `":verb"` or `[":verb", arg1, …]` |
| extra_args… | any CBOR | optional positional args appended after the verb |

Dispatch target is always the caller (`msg.from`). Schedules are keyed by
`(msg.from, name)`, so re-registering the same name overwrites the previous
definition for that caller.

Overwrite is strict latest-wins: callbacks from superseded definitions MUST NOT
dispatch and MUST NOT self-reschedule.

### Schedule types

| Type | Spec | Behaviour |
|------|------|----------|
| `:cron` | 6-field cron string `"sec min hour day month weekday"` | Fires on schedule indefinitely |
| `:interval` | Human duration: `"1h"`, `"30m"`, `"5s"`, `"2d12h"` | Fires every N seconds indefinitely |
| `:at` | Unix timestamp in milliseconds (integer) | Fires once after the computed delay |
| `:random` | Max seconds (integer) | Fires after 1–N random seconds, then self-reschedules |

### Examples

```cbor
; Fire :tick on myentity every minute
["tick", ":cron", "0 * * * * *", ":tick"]

; Fire [:grow, "small plant+=1"] on garden every 30 minutes
["grow", ":interval", "30m", ":grow", "small plant+=1"]

; Same, using array form for verb+args
["grow", ":interval", "30m", [":grow", "small plant+=1"]]

; One-shot at a specific Unix timestamp (ms)
["wake-once", ":at", 1748700000000, ":wake"]

; Random re-trigger within 5 minutes
["scratch", ":random", 300, ":scratch"]

; Overwrite by reusing same name for same sender
["scratch", ":random", 30, ":scratch"]
```

### ACL

Scheduled dispatch **bypasses all ACL checks**. The runtime is the trusted
caller. A typical scheduler ACL exposes `:help` publicly while keeping
schedule registration local-only:

```yaml
scheduler:
  "*": [":help"]
  "#": [":cron", ":interval", ":at", ":random"]
```

This allows foreign DIDs to ask `#scheduler:help` directly without letting
them register schedules on that runtime.

---

## ACL

Capabilities are plain strings in YAML sequences. The default when no
`--acl-file` is given allows everyone to use both services.

```yaml
acl:
  "*": [rpc, ipfs, identity-publish]  # everyone: RPC + IPFS store + DID publish (default)
  "did:ma:alice": [owner]     # alice: full access
  "did:ma:bob": [rpc]         # bob: RPC only, no IPFS store or DID publish
  "did:ma:eve":               # null = explicit deny
```

Built-in capabilities:

| Capability | Required by |
|------------|-------------|
| `rpc` | `/ma/rpc/0.0.1` |
| `ipfs` | `/ma/ipfs/0.0.1` generic content storage |
| `identity-publish` | `/ma/ipfs/0.0.1` DID-document publishing |
| `"*"` | Wildcard — grants all capabilities when used in an Allow set |
| `create` / `update` / `delete` | Namespace / entity management |

Rules: deny always wins; direct match wins over wildcard; fragment stripped
from DID-URLs before lookup.

### Group resolution

ACL entries can reference a named group registry entry via the flat
`+<name>` syntax — no nested path, no `#fragment`:

```yaml
acl:
  "+admins": ["*"]     # anyone listed in /grp/admins gets all capabilities
  "+owners": ["*"]     # anyone listed in /grp/owners (real, not cosmetic)
  "did:ma:eve":        # explicit deny overrides group membership
```

`<name>` is looked up directly in `manifest.grp` (CRUD-addressed as
`/grp/<name>`), an IPLD link to a plain `Vec<String>` of member DIDs.
Resolution is a synchronous, in-memory cache lookup (`GroupCache`) — no
actor dispatch, no async RPC round-trip. `"owners"` is one ordinary entry
in `grp`, resolved exactly like any other group; it is protected only
against deletion (may be set to an empty list, but the entry itself can
never be removed via CRUD), never resolved specially.

---

## Status web server

| Endpoint | Content-Type | Description |
|----------|-------------|-------------|
| `GET /` | `text/html` | Human-friendly page |
| `GET /status.json` | `application/json` | Machine-readable status |
| `GET /bootstrap.yaml` | `text/yaml` | Bootstrap template for this runtime |
| `GET /zion` | `text/html` | Redirects to `/zion/` |
| `GET /zion/` | `text/html` | Local Zion app served from the configured `/config/zion` IPFS/IPNS source |
| `GET /zion/*path` | varies | Local Zion assets proxied from IPFS |
| `POST /claim` | — | Claim ownership: body `{"did":"did:ma:…"}` |

For a first local setup, start IPFS Desktop, start `ma`, then visit
`http://localhost:5003/zion`. This is the recommended path because the browser
app is loaded through the runtime's local-only status server instead of a remote
web origin.

### CORS policy (default-deny except allowlist)

Status endpoints are served with a CORS allowlist (not `*`).

Default allowed origins:

- `http://localhost:8000`
- `http://127.0.0.1:8000`
- `https://zion.bahner.com`

Override in config extra as either a YAML list or comma-separated string:

```yaml
status_cors_allowed_origins:
  - http://localhost:8000
  - http://127.0.0.1:8000
  - https://zion.bahner.com
```

### `status.json`

```json
{
  "did": "did:ma:<ipns>",
  "ipns": "/ipns/<ipns>",
  "endpoint_id": "<iroh-id>",
  "uptime_secs": 42,
  "ipfs_publisher": true,
  "ipfs_requests": 0,
  "rpc_requests": 0,
  "pings_received": 0,
  "started_at": 1234567890,
  "entity_names": ["greeter", "counter"],
  "runtime": {"/": "bafyreiXXXX"}
}
```

---

## Runtime tuning keys (config extra)

These keys are safe/anonymous operational settings and are also mirrored into
the runtime manifest `config` node.

```yaml
did_resolver_positive_ttl_secs: 60
did_resolver_negative_ttl_secs: 10
did_resolve_attempts: 5
did_resolve_attempt_timeout_secs: 60
wasm_reload_shutdown_timeout_ms: 250

did_document_publishing_interval_secs: 3600
did_publish_cache_warm_secs: 3600
did_document_publishing_timeout_secs: 120
did_document_publishing_lifetime_hours: 8760

ipns_publish_lifetime_hours: 8760
ipns_publish_timeout_secs: 120
ipns_publish_allow_offline: true
ipns_publish_resolve: false

status_cors_allowed_origins:
  - http://localhost:8000
  - http://127.0.0.1:8000
  - https://zion.bahner.com
```

### IPNS safety notes

- Keep `ipns_secret_key` private and encrypted in `SecretBundle`.
- Prefer `ipns_publish_resolve: false` and explicit republish intervals to
  reduce publish-time dependency on network resolution.
- Use bounded timeouts for publish operations. `did_document_publishing_timeout_secs`
  is the outer task budget; `ipns_publish_timeout_secs` is the Kubo
  `name/publish` request budget used by the shared IPNS publish helper.
- DID outbox resolution uses `did_resolve_attempts` raced attempts. Each
  attempt has its own `did_resolve_attempt_timeout_secs` budget.
- Runtime manifest changes advance the local root CID immediately. The current
  root CID is advertised through runtime IPNS on the periodic interval above and
  when an owner sends `:publish` to `#root`; clean shutdown also attempts one
  final runtime IPNS publish.

---

## Configuration keys (`ma.yaml`)

### `slug`

`slug` is **CLI/env-only** (`--slug` / `MA_SLUG`). It is never written to
`config.yaml` — the runtime needs the slug to locate the config file in the
first place, creating an unsolvable catch-22 if the slug were stored there.

### Protected keys

These keys are never readable or writable via `:config.*` RPC:

| Key | Reason |
|-----|--------|
| `slug` | CLI/env-only (catch-22, see above) |
| `secret_bundle` | Key material path — must not leak |
| `secret_bundle_passphrase` | Secret — must never be exposed via RPC |
| `config_path` | Internal path — not user-settable via RPC |
| any key starting with `secret` | Blanket guard for future secret fields |

### Daemon config keys

Readable and writable via `:config.<key>` RPC. Changes take effect immediately
in memory and are saved to `config.yaml`:

| Key | Type | Description |
|-----|------|-------------|
| `kubo_rpc_url` | string | Kubo RPC URL (default `http://127.0.0.1:5001`) |
| `secret_bundle_passphrase` | string | Bundle passphrase. Also `MA_SECRET_BUNDLE_PASSPHRASE` env. Written automatically by `--gen-headless-config`. |
| `ipfs_publisher` | bool | Enable `/ma/ipfs/0.0.1` (default `true`) |
| `crud_service` | bool | Enable `/ma/crud/0.0.1` (default `true`) |
| `owners` | list | Owner DIDs granted `["*"]` in the live ACL at startup |
| `log_level` | string | Log level for the log file |
| `log_level_stdout` | string | Log level for stdout |
| `log_file` | string or null | Path to log file |
| `did_resolver_positive_ttl_secs` | u64 | Cache TTL for resolved DIDs |
| `did_resolver_negative_ttl_secs` | u64 | Cache TTL for failed DID lookups |
| `wasm_reload_shutdown_timeout_ms` | u64 | Short per-actor budget for reload-time `:shutdown` state flush before the replacement proceeds (default `250`) |

### Manifest config keys

Stored in the IPFS DAG (`manifest.config`), not in `config.yaml`. These
persist across restarts because they live in IPFS.

| Key | Type | Description |
|-----|------|-------------|
| `name` | string | Runtime display name, used by root RPC `:name` (default `間trix`). |
| `description` | string | Runtime description text, used by root RPC `:description` (default `A 間 runtime with a lazy owner.`). |
| `i18n` | string | Active language BCP-47 tag (e.g. `nb`, `zh-Hans`). Setting via `:config.i18n: nb` reloads translations immediately. |
| `i18n_cid` | string | IPFS CID of the compiled locale DAG-CBOR node (set by `make src/i18n.yaml`) |
| `plugin_envelope_queue_capacity` | positive integer | Capacity of the global in-memory plugin envelope queue (default `1024`). Missing values are materialised in the manifest at startup. Changes take effect after restarting the runtime. |
| other | any | Free-form runtime metadata |

Runtime-owned IPNS keys use deterministic Kubo aliases with the local runtime
slug so multiple local runtimes can share one Kubo daemon. The runtime DID
document uses `ma-runtime-<slug>-<hash>`; the internal `runtime_ipns` manifest
root uses `ma-runtime-<slug>-runtime-<hash>`.

---

## Internationalisation

Log and status messages are translated using a lightweight subset of
[Fluent](https://projectfluent.org/) (FTL): `key = value` lines only. No
attributes, selectors, or `{ $var }` substitutions — all runtime log strings
are plain declarative messages.

FTL files live in `i18n/`. They are compiled into the binary at build time via
`make src/i18n.yaml`. The active language is read from
`RuntimeManifest.config.i18n` at startup; if absent, falls back to `nb`.

### Changing the runtime language

```
# from the zion terminal (takes effect immediately, persists to manifest):
@ma:config.i18n: nb

# or via CLI flag / env var (current process only):
ma --i18n nb
MA_I18N=nb ma
```

### FTL file conventions

- `i18n/en.ftl` is the **canonical source** — it defines every key.
- All other `i18n/*.ftl` files mirror the same key set with translated values.
- Technical terms are kept verbatim: DID, IPFS, IPNS, RPC, ACL, iroh, CID,
  `#root`, `/ma/ipfs/0.0.1`, `:ping`, `:pong`, Bootstrap, Plugin, manifest.
- Missing keys fall back to the key name string.
- **Never copy English text into non-English locale files.** Translate
  properly or leave the key absent — silently pasting English values into a
  non-English file misleads users.
- Every `i18n/*.ftl` file **must** contain a `lang-name` key whose value is
  the language's own name for itself (autonym), e.g. `lang-name = Norsk bokmål`.

### Adding a new language

1. Create `i18n/<code>.ftl` with all keys from `i18n/en.ftl` translated,
   including `lang-name = <autonym>`.
2. Rebuild (`make src/i18n.yaml && make release`). `build.rs` scans
   `i18n/*.ftl` and regenerates `BUNDLED_LANGS` automatically — no manual
   code change required.

### BCP-47 private-use and special language tags

| Code | Language | Notes |
|------|----------|-------|
| `art-x-lyaric` | Dread Talk (Rasta) | BCP-47 private-use tag for Lyaric / Rastafarian Iyaric dialect |
| `qbc` | Belter Creole | The conlang from *The Expanse*; ISO 639-3 code `qbc` |

Tags containing `-x-` are passed through verbatim. Use the full tag as both
the filename stem and the entry in `BUNDLED_LANGS`.

---

## Security notes

- **IPNS keys are zero-use.** The sender's IPNS private key carried in each
  `/ma/ipfs/0.0.1` request is used once for the Kubo publish call and
  immediately zeroized.
- **DID document pinning keeps only the newest CID, best-effort.** Incoming
  DID-document publish requests try to move the local pin from the old resolved
  CID to the newly published CID, but pin cleanup failures do not fail the
  publish. This is local-only; runtime remote pin configuration is not applied
  to delegated DID documents. Generic IPFS store requests are not directly
  pinned.
- **Replay protection** via a 120-second sliding window on `/ma/ipfs/0.0.1`.
- **Input validation** via `validate_ipfs_publish_request` (ma-core): checks
  content-type, DID document proof, sender identity match.
- **Wasm sandboxing** via extism/Wasmtime.
- **No auto-execution** of arbitrary CIDs — plugins load only from
  `behaviour_cid` values in the signed IPLD manifest.
- Config and bundle files are created with mode `0600`.

---

## Makefile

```sh
make build          # cargo build
make release        # cargo build --release
make check          # cargo check
make run            # run the daemon
make src/i18n.yaml  # publish i18n/*.ftl to IPFS and embed CIDs into the binary
make install        # install binary to $PREFIX/bin (default ~/.local/bin)
```
