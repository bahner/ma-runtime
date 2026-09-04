# art-x-lyaric — Dread Talk / Iyaric (Rasta)
# Rastafarian Iyaric dialect, also known as Dread Talk or Lyaric.
# BCP-47 private-use tag: art-x-lyaric
lang-name = Iyaric

own-did-published = I an I DID document publish forward to IPNS
own-did-publish-failed = Iration fail fi publish I an I DID document
own-did-publish-timeout = I an I DID document publish time out after 2 minutes
started = ma runtime a rise
shutdown-requested = Shutdown a call
closing-endpoint = I an I a seal up di iroh endpoint...
shutdown-complete = Shutdown fulfill
status-listening = Status server a listen
ipfs-message-rejected = IPFS message reject by Zion
ctrlc-handler-failed = Ctrl-C handler fall
node-connected = I an I node connect to protocol
received-encrypted-ma-msg = Receive encrypted ma-msg pon /ma/ipfs/0.0.1
ping-received = :ping receive, a send :pong
did-publish-request-received = Receive DID document publish request
document-published = Document publish forward
did-publish-cid-reply-sent = CID reply fi DID publish forward
did-publish-resolve-failed = Cyaan overstand sender fi carry ipfs-publish reply
ipfs-store-request-received = Receive IPFS store request
ipfs-stored = Content store pon IPFS in Zion
ipfs-store-cid-reply-sent = CID reply forward
ipfs-store-resolve-failed = Cyaan overstand sender fi carry ipfs-store reply

# Entity dispatch
bootstrap-complete = Bootstrap complete in Iration
entity-loaded = Entity plugin load
entity-load-failed = Iration fail fi load entity plugin
root-list-entities = #root: list entity dem
entity-created = Entity manifest
entity-reloaded = Entity plugin load up fresh again
entity-deleted = Entity remove
entity-states-saving = I an I a preserve entity states pon IPFS
entity-state-saving = I an I a preserve entity state
entity-state-saved = Entity state save
entity-state-empty = Plugin return empty state, pass over
entity-states-saved = Entity states save

# First-run auto-init

# Ownership / claim
runtime-claimed = Runtime claim in Iration.

# Protected root elements
refuse-delete-root = Steadfastly refuse fi remove required root element
runtime-claim-persisted = Owner write to config in Zion.


# Namespace creation (:create)
crud-message-received = CRUD message come inna Zion
crud-acl-updated = Root transport ACL rise up new

# CRUD validation errors
cidv1-required = di value mus be a bare CIDv1 (start wid 'b'; CIDv0 'Qm…' nah accepted)
config-key-protected = config key '%key%' is inna protection
config-key-no-delete = daemon config key '%key%' cyan delete
config-key-not-manifest = config key '%key%' nah known manifest config key
wrong-crud-protocol = CRUD protocol nuh right: %type%
entity-name-invalid = di entity name must be clean printable UTF-8
reserved-entity-name = di entity name '%name%' ital, it reserved
genesis-kind-owner-only = Only a runtime Owner can create a entity of a genesis kind

# IPv6 config
ipv6-enabled = IPv6 lively up — holdin both IPv4 an IPv6 inna oneness
ipv6-disabled = IPv6 downstrike — only IPv4 a hold (restart needeth fi rise it back)
ipv6-enable-restart-required = Irie-saved. Restart needeth fi dis change fi livity.
ipv6-enable-unchanged = ipv6_enable already set to dat value — no change seen.

# New keys
boot-default-root-config-populate-failed = Faild fi populate di default config root
boot-default-root-config-populated = Default config root fill up
boot-entity-load-processed = Entity plugin dem loaded
boot-group-load-failed = Faild fi load di group pon startup
boot-group-loaded-into-cache = Group loaded inna di cache
boot-kinds-overlay-applied = Kinds overlay put on
boot-kinds-overlay-no-change = Kinds overlay nuh mek no manifest changes
boot-load-manifest-for-acl-cache-failed = Faild fi load manifest fi ACL cache population
boot-minimal-manifest-bootstrapped = Minimal manifest bootstrap up
boot-minimal-manifest-not-found = No runtime root CID find inna IPNS; bootstrapping minimal manifest
boot-no-root-entity = No root entity registered fi default config root
boot-reconciled-owners-manifest-failed = Faild fi reconcile owners inna manifest pon startup
boot-reconciled-owners-persist-failed = Faild fi persist reconciled owners to config.yaml
boot-reconciled-owners-published = Owners reconciled from config.yaml/--owner inna manifest pon startup
boot-root-acl-load-cache-failed = Faild fi load root ACL pon startup
boot-root-acl-load-failed = Faild fi load root ACL from manifest
boot-root-acl-loaded-from-manifest = Root transport ACL loaded from manifest
boot-root-acl-loaded-into-cache = Root ACL loaded inna cache
bootstrap-acl-published = ACL node publish forward
bootstrap-endpoint-close-stuck = endpoint still held by in-flight task dem after 10 s; a drop it without a graceful close
bootstrap-endpoint-close-timeout = endpoint close time out after 5 s; a force di exit
bootstrap-entity-lifecycle-update-failed = Faild fi write updated entity lifecycle to IPFS
bootstrap-entity-lifecycle-updated = Entity lifecycle updated inna IPFS
bootstrap-entity-node-shutdown-updated = Entity node updated pon shutdown
bootstrap-entity-published = Entity node published
bootstrap-entity-registering-prepublished = A register di pre-published entity
bootstrap-entity-registry-fetch-failed = Faild fi fetch entity node
bootstrap-entity-registry-kind-extends-failed = Faild fi resolve kind extends chain
bootstrap-entity-registry-kind-fetch-failed = Faild fi fetch kind node
bootstrap-entity-registry-kind-missing = Kind not found inna manifest; skipping entity
bootstrap-entity-registry-not-in-manifest = Entity inna registry but not inna manifest; skipping
bootstrap-entity-state-save-failed = Faild fi save entity states
bootstrap-entity-state-shutdown-aborted = shutdown aborted; runtime remains active so state can be saved pon a later shutdown attempt
bootstrap-entity-state-update-fetch-failed = Faild fi fetch entity node fi state update
bootstrap-envelope-delivery-failed = Plugin envelope delivery faild; dropping envelope
bootstrap-envelope-open-failed = Plugin envelope: outbox open faild; dropping envelope
bootstrap-group-published = Group node publish forward
bootstrap-kind-published = Kind node publish forward
bootstrap-kind-registry-extends-failed = Faild fi resolve kind extends chain fi registry
bootstrap-kind-registry-fetch-log-failed = Faild fi fetch kind node fi registry
bootstrap-kind-registry-hydrated = Kind registry fill up from manifest
bootstrap-kinds-overlay-pin-update-failed = Pin/update faild after di kinds overlay
bootstrap-kinds-overlay-published = Published runtime manifest after di kinds overlay
bootstrap-kinds-tree-published = Runtime kinds tree publish forward
bootstrap-lifecycle-manifest-pin-update-failed = Pin/update faild after lifecycle persist
bootstrap-lifecycle-manifest-publish-failed = Faild fi publish manifest after lifecycle transitions
bootstrap-lifecycle-manifest-published = Updated manifest publish forward after lifecycle transitions
bootstrap-manifest-fetch-failed = Faild fi fetch runtime manifest
bootstrap-minimal-manifest-failed = Faild fi bootstrap minimal manifest
bootstrap-remote-root-pin-confirmed = Remote root pin confirm
bootstrap-remote-root-pin-misconfigured = Remote root pinning nuh set up right
bootstrap-root-acl-published = Published root transport ACL
bootstrap-root-cid-shutdown-persist-failed = Faild fi persist root_cid during shutdown
bootstrap-root-cid-shutdown-publish-failed = Shutdown runtime_ipns publish faild
bootstrap-root-cid-shutdown-publish-succeeded = Shutdown runtime_ipns publish succeed
bootstrap-root-cid-shutdown-publish-timeout = Shutdown runtime_ipns publish time out
bootstrap-root-pin-replacement-failed = I an I a carry on after di remote root pin replacement fail
bootstrap-root-pin-update-failed = Pin/update faild after bootstrap
bootstrap-runtime-manifest-published = Runtime root manifest publish forward
crud-message-rejected = CRUD message reject by Zion
entity-reload-current-node-load-failed = Faild fi load di current entity node before reload; keeping di current plugin
entity-reload-failed = Entity faild fi reload; unloading til di next reload
entity-reload-kind-extends-failed = Faild fi resolve di kind extends chain while reloading an entity
entity-reload-kind-fetch-failed = Faild fi fetch di kind node while reloading an entity
entity-reload-kind-lookup-failed = Faild fi load di manifest fi kind lookup while reloading an entity
entity-reload-kind-missing = Kind not found inna di manifest; cannot reload entity
entity-reload-manifest-state-update-failed = Faild fi update di manifest with di current state before reload; keeping di current plugin
entity-reload-skipped = Entity reload skipped because di reload gate is closed
entity-reload-started = Entity reload a start
entity-reload-state-persist-failed = Faild fi persist di current state before reload; keeping di current plugin
entity-reload-state-produced-failed = Faild fi persist di state produced during reload
entity-reloaded-manifest-update-failed = Faild fi update di reloaded entity inna di manifest
entity-reloaded-manifest-updated = Updated reloaded entity inna manifest
inbox-message-rejected = Inbox message reject by Zion
ma-create-entity-already-exists = ma_create_entity: entity already deh; keeping di current entity
ma-create-entity-invalid-behaviour = ma_create_entity: behaviour reference nuh valid; pass over
ma-create-entity-kind-missing = ma_create_entity: kind not inna registry; skipped
manifest-pin-update-failed = Manifest pin_update faild
plugin-envelope-build-failed = Plugin envelope: faild fi build message; skipped
plugin-envelope-create-requests-ignored = Plugin envelope: create request dem pass over without side-effect context
plugin-envelope-local-dispatch-failed = Plugin envelope: local dispatch faild
plugin-envelope-local-dispatch-finish = Plugin envelope: local dispatch a finish
plugin-envelope-local-dispatch-start = Plugin envelope: local dispatch a start
plugin-envelope-local-gate-closed = Plugin envelope: local dispatch gate close
plugin-envelope-local-recipient-unknown = Plugin envelope: local recipient nuh known; pass over
plugin-envelope-local-timeout = Plugin envelope: local dispatch time out
plugin-envelope-recipient-invalid = Plugin envelope: recipient DID nuh valid; pass over
plugin-envelope-remote-limit = Plugin envelope: remote delivery limit reach; envelope drop
plugin-outbox-congested = Plugin outbox congested; envelope dem fit drop if channel fill
plugin-outbox-drain-limit = Plugin outbox drain budget done; a put off di remaining envelope dem
schedule-dispatch-firing = Scheduled dispatch a fire
schedule-entity-not-found = Scheduled dispatch: entity nuh find
schedule-random-chain-stopped = Random schedule chain stop: a newer definition replace it
schedule-random-create-failed = Faild fi create next random job
schedule-random-reschedule-failed = Faild fi reschedule random job
schedule-stale-dispatch-skipped = Scheduled dispatch pass over: stale schedule
scheduled-dispatch-error = Error pon di scheduled dispatch
scheduled-dispatch-manifest-writer-unavailable = Scheduled dispatch: manifest writer nuh ready; entity state still a wait
