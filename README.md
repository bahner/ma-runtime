# 間

---

## Background

### The Name

*Ma* (間) is a Japanese concept — the space between things, the interval that
gives structure its meaning. In architecture it is the silence that makes the
room. In music it is the rest that makes the phrase. In `did:ma` it is the
space between actors where messages travel.

### The Inspiration

The inspiration for 間 stems from Alan Kay's 1997 OOPSLA keynote,
*"The Computer Revolution Hasn't Happened Yet"*, where he argued that the
mainstream had fixated on the wrong part of object-oriented programming.

> I have apologized profusely over the last 20 years for making up the term
> "object-oriented", because as soon as it started to be misapplied, I
> realized I should've used a much more process-oriented term for it.
> Now the Japanese have an interesting word, which is called 間 - spelled in
> English just m a - ma. And 間 is the stuff in between what we call
> "objects". It's the stuff we don't see because we are focused on the
> "nounness" of things rather than the "processness" of things.

Kay envisioned objects as autonomous entities — like biological cells or
computers on a network — that communicate exclusively by sending messages.
Each object has its own address. You never reach inside; you send a message
and let the receiver decide what to do with it.

`did:ma` takes this literally. Every entity has a DID — a globally resolvable
address. Every interaction is a signed, one-way message. There are no method
calls, no shared state, no synchronous return channels. You send a message to
a DID and move on.

### All Objects Have a URL

Kay also insisted that every object must be addressable — a real identity on
the network, not a pointer in local memory.

> I do not know of anybody yet who has realized that at the very least each
> object should have a URL.

DIDs can be converted to URLs by appending fragments. The 間 framework provides
a URL for all entities for other entities to send messages to.

### Hewitt's Actor Model

Carl Hewitt introduced the Actor Model in 1973 as a mathematical theory of
concurrent computation. An actor is the fundamental unit: an independent
entity with its own identity, state, and behaviour. Actors communicate
exclusively by sending messages. Upon receiving a message, an actor may:

1. Send messages to other actors it knows about.
2. Create new actors.
3. Update its own local state.

Nothing else. No shared memory. No locks. No synchronous return values. The
model is simpler than it looks: all concurrency, all distribution, all
coordination reduces to these three primitives.

`did:ma` is a direct implementation of this. Each entity is an actor — it has
a DID as its identity, an inbox as its message interface, and private state
that nothing outside can touch. Sending to a DID is sending to an actor.
The runtime is the actor system.

### Erlang's Lessons

If Hewitt wrote the theory, Joe Armstrong and the Erlang team at Ericsson
proved it works at scale. `did:ma` borrows liberally from Erlang and its
cousin Elixir — message passing over shared state, strict validation instead
of defensive error handling, location transparency through addressable
identity. The first `did:ma` prototype was in fact written in Elixir, where
Erlang atom IDs served directly as process addresses. The nanoid fragments in
DID URLs (`did:ma:<ipns-key>#<nanoid>`) are the same idea with a global scope.

The wire protocol carries that lineage visibly. RPC verbs are CBOR text atoms
that look exactly like Erlang atoms: `:ping`, `:pong`, `:ok`, `:error`. Replies
follow the same tuple conventions Erlang programmers recognise immediately:
`[:ok, payload]`, `[:error, reason]`. The pattern matching discipline is the
same too — a message is either one of those shapes or it is rejected.

### MUDs and LambdaMOO

Before the web, there were MUDs — Multi-User Dungeons. Text-based shared
worlds where players inhabited rooms, carried objects, and talked to each
other through typed commands. In 1990, Pavel Curtis at Xerox PARC created
LambdaMOO, a programmable MUD where every object in the world was a
first-class entity with properties, verbs, and a unique identity. Users
could build rooms, script objects, and extend the world from inside the world
itself.

LambdaMOO got something deeply right: a shared, persistent, programmable
space where identity matters and objects are real. It also demonstrated what
happens when you centralise everything on one server — it doesn't scale, it
doesn't federate, and the operator holds all the keys.

`did:ma` inherits the ambition — a world of addressable, programmable objects
inhabited by identified actors — but distributes it. No single server owns
the world. Identities are self-sovereign DIDs. Objects are content-addressed.
The rooms still exist, but they live on IPFS, not in a single process on a
single machine.

### Standing on the Shoulders of Protocols

None of this would work without the infrastructure that others built first.

**IPFS** provides content-addressed, immutable storage. Every DID document,
every published object, every piece of world state is a DAG node identified
by its content hash. If you have the CID, you have the data — regardless of
who serves it.

**IPNS** provides mutable pointers over immutable storage. A DID resolves
through IPNS: `did:ma:<ipns-key>` points to the latest version of a DID
document, which is itself an immutable IPFS object. Identity is the key.
The document can change. The address stays.

**IPLD** provides the data model. DID documents and messages are dag-cbor —
a canonical, deterministic CBOR serialisation that slots directly into the
IPFS content-addressing stack.

**iroh** provides the transport. Where IPFS gives us storage and naming,
iroh gives us direct peer-to-peer connectivity over QUIC — fast, encrypted,
NAT-traversing connections between endpoints identified by public keys. The
iroh endpoint model maps cleanly onto `did:ma` services: each endpoint
advertises protocol IDs, accepts connections, and routes them to service
handlers.

These are not dependencies bolted on after the fact. `did:ma` was designed
around them. IPFS/IPNS is the verifiable data registry. IPLD is the wire
format. iroh is the transport. The protocol is the composition.

---

## What it does

When you run `ma`, a lean Tokio daemon comes alive. It derives its own
`did:ma:` identity from a key bundle, publishes a signed DID document to IPFS,
and registers three services on an iroh QUIC endpoint:

- **`/ma/rpc/0.0.1`** — routes messages to named Wasm plugins (entities) and
  handles liveness probes.
- **`/ma/crud/0.0.1`** — manages the live entity system: create, update,
  delete entities and kinds; read and write runtime config.
- **`/ma/ipfs/0.0.1`** — receives signed publish requests from browser actors
  and forwards them to Kubo, so a tab can push a new DID document to IPNS
  without touching the Kubo API directly.

The entity system is where the interesting work happens. Each entity is a
named Wasm plugin whose behaviour lives in an IPFS-addressed blob. State is
persisted back to IPFS on graceful shutdown. The entire live runtime — kinds,
entities, config, ACL — is a content-addressed IPLD DAG rooted at a single
CID that is published to IPNS on each change. You can fork a runtime by
publishing a different root. You can audit the history by traversing the DAG.
The runtime is just a view over a graph, and the graph is permanent.

```
your browser ──iroh QUIC──► /ma/rpc   ──► Wasm entity dispatch
             ──iroh QUIC──► /ma/crud  ──► entity + config management
             ──iroh QUIC──► /ma/ipfs  ──► Kubo → IPNS publish
```

---

## Getting started

### Prerequisites

- Rust (latest stable)
- [IPFS Desktop](https://docs.ipfs.tech/install/ipfs-desktop/) running locally
  (it includes Kubo on `http://127.0.0.1:5001`)

### Build

```sh
cargo build --release
# binary: target/release/ma
```

### Docker Compose

Start Kubo and an isolated runtime with:

```sh
docker compose up
```

The runtime is available at <http://localhost:5003/operator/>. Its `docker.yaml`
and `docker.bin` live in the persistent `ma-config` Docker volume. Back them
up, including while the runtime is stopped, with a temporary container:

```sh
docker run --rm -v ma_ma-config:/state:ro alpine \
  tar -C /state -czf - docker.yaml docker.bin > ma-docker-backup.tgz
```

To seed the Docker runtime from an existing root CID on its first start, set
`MA_ROOT_CID` for that one invocation:

```sh
MA_ROOT_CID=bafy... docker compose up
```

Stop it normally after the initial publish, then start later runs with plain
`docker compose up`. The persistent Docker identity resolves its runtime head
from IPNS; do not leave `MA_ROOT_CID` set after seeding.

To mirror runtime heads to a remote pinning service, provide the following
environment variables when starting Compose:

```sh
MA_PIN_REMOTE=true \
MA_PIN_REMOTE_SERVICE=pinata \
MA_PIN_REMOTE_NAME=ma-runtime-docker \
MA_PIN_OVERWRITE=true \
MA_PIN_REMOTE_API_URL=https://api.pinata.cloud/psa \
MA_PIN_REMOTE_API_SECRET=... \
docker compose up
```

`MA_PIN_REMOTE=true` enables remote replication. `MA_PIN_REMOTE_SERVICE` is
the Kubo service name, `MA_PIN_REMOTE_NAME` is the optional label on the remote
pin, `MA_PIN_REMOTE_API_URL` is the service API endpoint, and
`MA_PIN_REMOTE_API_SECRET` is its API key. `MA_PIN_OVERWRITE` defaults to
`true`: the fresh pin is protected under a temporary in-flight name while a
detached best-effort job removes all old pins with the same exact name, then
renames the pin to the requested name. Cleanup does not delay publication.
The `ma` container registers the named service with Kubo if
it is absent. Existing services are preserved.

### First run

On the first start `ma` detects a missing secret bundle and generates a
complete headless config automatically. No manual setup required. You can
also trigger generation explicitly:

```sh
ma --gen-headless-config
```

Four random 32-byte keys are generated, encrypted with a random passphrase, and
written to `$XDG_CONFIG_HOME/ma/ma.yaml` and `ma.bin`. The passphrase is
printed once and stored in the config file so subsequent restarts are
unattended. Back it up in a password manager; if you prefer it not to live in
the file, remove the `secret_bundle_passphrase:` line and export
`MA_SECRET_BUNDLE_PASSPHRASE` instead.

### Claim ownership

On first start with no prior manifest, `ma` publishes an empty
`RuntimeManifest` and waits. To grant yourself access:

```sh
# from the operator terminal:
.ma!claim
# or from anywhere with your DID:
curl -X POST http://127.0.0.1:5003/claim \
  -H 'Content-Type: application/json' \
  -d '{"did":"did:ma:<your-ipns>"}'
```

Or pre-seed ownership in `ma.yaml` before starting:

```yaml
owners:
  - did:ma:<your-ipns>
```

### Bootstrap (optional)

To pre-populate entity kinds and a runtime manifest from a YAML description,
use a world/profile bootstrap such as λ-間's generated file:

```sh
ma --gen-root-cid ../lambda-ma/dist/lambda-ma.yaml
# prints a root_cid — add it to ma.yaml or pass via --root-cid
```

See [REFERENCE.md](REFERENCE.md) for the full bootstrap YAML schema. The local
[bootstrap.example.yaml](bootstrap.example.yaml) is a minimal schema skeleton;
concrete kind descriptors are maintained by the profile that owns them.

### Run

```sh
ma
# with options:
ma --owner did:ma:<your-ipns>
```

For persistent local startup settings, add them to `ma.yaml`:

```yaml
slug: ma
poll_ms: 100
status_bind: 127.0.0.1:5003
```

`--slug`, `--poll-ms`, and `--status-bind` override the corresponding YAML
values for one invocation. A YAML `slug` affects runtime defaults after the
config file is selected; it cannot select the config file itself.

Once IPFS Desktop and `ma` are running, the simplest and safest way to begin is
to open `http://localhost:5003/operator` in your browser. The runtime serves Operator
through its local-only status server, so you do not need a remote browser origin
for first setup. The default Operator files are nevertheless selected through the
ma maintainer's IPNS key. Treat that source only as bootstrap help, not as a
trust root. Publish Operator under an IPNS key you control and replace
`/config/operator` with your own `/ipns/...` path; the commands are in
[HOWTO.md](HOWTO.md#publish-operator-under-your-own-key). The status page remains
available at `http://127.0.0.1:5003`.

### Wasm memory reservation

`ma` runs each entity plugin through Extism/Wasmtime. Wasmtime normally
reserves a large virtual address range for each Wasm linear memory so it can
grow quickly; this can make `top` show a high `VIRT` value even when actual
resident memory (`RSS`) is small. That reservation is address space, not heap
already in use.

The default runtime reservation is tuned for ordinary entities such as rooms,
avatars, and small script actors:

```sh
MA_WASM_MEMORY_RESERVATION_BYTES=67108864              # default: 64 MiB
MA_WASM_MEMORY_RESERVATION_FOR_GROWTH_BYTES=1048576    # default: 1 MiB
```

These settings are not hard memory limits. If a plugin grows beyond the
reserved range, Wasmtime can relocate and grow the linear memory; the tradeoff
is lower virtual memory usage versus less room for growth before relocation.
Most deployments should leave the defaults alone. Operators running unusually
large Wasm entities can raise the values, or restore Wasmtime's typical 4 GiB
reservation with `MA_WASM_MEMORY_RESERVATION_BYTES=4294967296`.

---

## Security model

`ma` never sees your private keys. Your `SecretBundle` is encrypted at rest;
the plaintext keys live in memory only for the duration of a running session.
The daemon's own IPNS key is zeroized from memory immediately after publishing
its own DID document at startup.

Incoming `/ma/ipfs/0.0.1` requests carry the sender's IPNS private key for
one-shot publication. That key is used exactly once and zeroized immediately
after the Kubo call. `ma` cannot impersonate any `did:ma:` identity other than
its own.

Transport ACL uses deny-wins semantics: an explicit `null` entry for a
principal overrides any wildcard allow. Replay protection on `/ma/ipfs/0.0.1`
rejects duplicate message IDs within a 120-second window. Wasm plugins are
sandboxed by Wasmtime via Extism.

No plugin is executed from an arbitrary CID at runtime. Only `behaviour_cid`
values present in the signed, IPNS-published IPLD manifest are loaded.

---

## Reference

Full configuration keys, CLI flags, ACL format, IPLD schema, startup sequence,
plugin ABI, and operational tuning are in [REFERENCE.md](REFERENCE.md).

---

## Makefile

```sh
make build          # cargo build
make release        # cargo build --release
make check          # cargo check
make run            # run the daemon
make install        # install binary to $PREFIX/bin (default ~/.local/bin)
make src/i18n.yaml  # publish i18n/*.ftl to IPFS, embed CIDs into binary
```
