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

## Deferred cleanup — now done

1. **i18n keys**: dead RPC/inbox keys removed, remaining "RPC" values
   reworded, `rpc-reply-sent` renamed to `reply-sent`, and `MESSAGE_IDS`
   regenerated to exactly match code usage (160 keys). Missing `boot-*` and
   `inbox-message-received` keys added. All 65 locale files pruned.
2. **Stale "RPC" code comments/log strings** (boot.rs `about`, dispatch.rs
   debug strings, acl.rs test names, bootstrap.rs, crud/config.rs,
   eventloop.rs) reworded to the actor/inbox model.
3. **`rpc_requests` status field** removed (it was dead, always 0).
4. **`rust-ma-runtime/AGENTS.md`** updated to the inbox-only actor model.

## Remaining (phase 6 — out of scope here)

1. `ipfs.rs` still builds IPFS-service replies addressed to the sender's
   `#rpc` fragment (`build_rpc_reply_message`). That is the client-side reply
   convention zion/zscheme still rely on; it changes with the phase 6 zion
   refactor, not before.
2. `ma-zion/AGENTS.md` and `zscheme/AGENTS.md` still describe the RPC
   transport (`RPC_PROTOCOL_ID`, `CAP_RPC`, `send_rpc`, `SESSION_RPC_INBOX`,
   `rpc-send` primitive). Those repos are unrefactored; update the notes when
   phase 6 lands.
3. `lambda-ma/AGENTS.md` "RPC and events" wording is generic actor-model
   language (ma-reply!, technical replies); no functional change needed.
4. zscheme's `rpc-send` primitive name and zion's `actor-call` timeout/error
   classification (see "Questions for review").
