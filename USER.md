# USER.md — `no-rpc` refactor notes

Notes for "after the fact" review. The work spans `ma-spec`, `rust-ma-core`,
and `rust-ma-runtime`; the bulk lives in `rust-ma-runtime`.

## Decisions made on your behalf

1. **`:ping` stays transport-level.** Handled on the inbox's unfragmented path
   → `:pong` directly, never routed to an entity. Rationale: the AGENTS notes
   say liveness is ma-core protocol that must survive a wedged actor queue.
2. **Unfragmented config-read verbs removed.** The old `:entities`/`:config`
   dot-path grammar was already gone from the code; the remaining unfragmented
   `:name`/`:description` config reads were removed (config is CRUD's job).
   Only `:ping` remains unfragmented. The CRUD `entities`/`config`
   *capabilities* are untouched.
3. **Per-actor verb ACL on all actor messages.** The old RPC-only
   `enforce_entity_rpc_acl` generalises to all fragment-addressed inbox
   messages, capability derived from the content term's head.
4. **Reply correlation is `reply_to` only.** `type = rpc` / `rpc.reply`
   values are removed; nothing else carries the correlation.
5. **`ma_reply` routes over inbox.** lambda-ma `.ma` sources are unchanged —
   only the runtime's host-function routing changes.
6. **Path override via `cargo --config 'patch.crates-io.ma-core.path=…'`**,
   not committed. `Cargo.toml`/`Cargo.lock` stay on registry sources.
7. **Native `output` reply and `#root :publish` kept.** A native actor's
   `DispatchResult.output` is still delivered as a correlated reply (the
   `gen_server:handle_call` pattern), and `#root :publish` still replies. What
   was dropped is the *fabricated* error reply for a crashed actor.

## Questions for review

1. `"ping"` capability: I **removed** it and made `:ping` ungated transport
   liveness (like a health check). Worth confirming you want it ungated, or
   whether a `ping` capability should gate it.
2. Runtime-level failures on the actor channel (unknown entity, ACL denied,
   malformed term): I went with **drop + log** (no bounce), to avoid
   reintroducing the reply expectation. Confirm this rather than a
   mailer-daemon-style bounce.
3. `actor-call` timeout value (phase 6, out of scope here): needs a default
   and error classification (`reply-error` vs `timeout` vs `transport-error`).
4. `rpc-send` primitive name (zscheme-v1.md §8.1): still named `rpc-send`; it
   now sends via inbox but the name hints at the removed service. Consider
   renaming (`actor-call`/`call`) in phase 6.

## Deferred cleanup (cosmetic)

1. i18n log keys still reference "RPC": `rpc-message-received`,
   `rpc-message-rejected`, `rpc-not-text-atom`, `rpc-unknown-verb`,
   `unknown-rpc-atom` are now dead; `entity-dispatched`, `entity-replied`,
   `rpc-reply-sent`, `plugin-envelope-local-reply-dropped` are still used but
   their translated values say "RPC". Deferred: touches ~20+ locale files with
   genuine translations (and the AGENTS forbids copying English into them).
2. `boot.rs` `Cli` doc string still says "RPC + optional IPFS publisher".
3. acl.rs test function names (`wildcard_rpc_allows_rpc`,
   `wildcard_rpc_denies_ipfs`) still say "rpc" but now test `inbox`.
