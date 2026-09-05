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
4. ~~`rpc-send` primitive name~~ — done: renamed to `actor-send` (builtin) and
   `send_rpc` → `send_actor` (SchemeCtx trait) in ma-zscheme 0.7.0.

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

## Phase 6 (operator + zscheme) — done

1. **`ipfs.rs`** now replies to the sender's bare DID over `/ma/inbox/0.0.1`
   (`build_reply_message`), not a `#rpc` fragment.
2. **ma-operator** folds actor verb dispatch into inbox: `send_actor_message*`,
   `send_pong`, no `RPC_PROTOCOL_ID`/`CAP_RPC`/`SESSION_RPC_INBOX`; actor terms
   detected by `content_type == CONTENT_TYPE_TERM`; `AGENTS.md` updated.
3. **zscheme** sends actor calls over inbox (`INBOX_PROTOCOL_ID` +
   `MESSAGE_TYPE_MESSAGE` from the `#inbox` fragment), `rpc_inbox` → `inbox`,
   `poll_ms`; `AGENTS.md` updated.
4. **lambda-ma** needs no functional change (its `ma-send!`/`ma-reply!` host
   functions route over inbox transparently).

## Remaining (cosmetic / follow-ups)

1. ~~ma-operator i18n `rpc-error` keys + translated strings~~ — done: keys renamed to
   `term-error`/`term-error-detail`, English values reworded, "RPC" token
   stripped from translations, stale `rpc_requests` status read removed.
2. ~~`rpc-send` primitive name~~ — done: renamed to `actor-send` (builtin) and
   `send_rpc` → `send_actor` (SchemeCtx trait) in `rust-ma-zscheme` 0.7.0;
   consumers bumped to `ma-zscheme = "0.7"`.
3. ~~`actor-call` error classification~~ — done: timeouts now surface as a
   distinct `"timeout"` error; cancellation remains `"reply channel
   cancelled"`, reply errors carry the actor's reason, and send failures carry
   the transport error.
4. `lambda-ma/AGENTS.md` "RPC and events" wording is generic actor-model
   language; no functional change needed.
