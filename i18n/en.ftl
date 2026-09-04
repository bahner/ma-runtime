# ma-runtime – English
lang-name = English

own-did-published = Own DID document published to IPNS
own-did-publish-failed = Failed to publish own DID document
own-did-publish-timeout = Own DID document publish timed out after 2 minutes
started = ma runtime started
shutdown-requested = Shutdown requested
closing-endpoint = Closing iroh endpoint...
shutdown-complete = Shutdown complete
status-listening = Status server listening
crud-message-received = Received CRUD message
crud-acl-updated = Root transport ACL updated
ipfs-message-rejected = IPFS message rejected
ctrlc-handler-failed = Ctrl-C handler failed
node-connected = Node connected to protocol
received-encrypted-ma-msg = Received encrypted ma-msg on /ma/ipfs/0.0.1
reply-sent = Reply sent
ping-received = Received :ping, sending :pong
did-publish-request-received = Received DID document publish request
document-published = Document published
did-publish-cid-reply-sent = Sent CID reply for DID publish
did-publish-resolve-failed = Could not resolve sender to deliver ipfs-publish reply
ipfs-store-request-received = Received IPFS store request
ipfs-stored = Stored content on IPFS
ipfs-store-cid-reply-sent = Sent CID reply
ipfs-store-resolve-failed = Could not resolve sender to deliver ipfs-store reply

# Entity dispatch
bootstrap-complete = Bootstrap complete
entity-loaded = Entity plugin loaded
entity-load-failed = Failed to load entity plugin
entity-not-found = Entity not found, ignoring message
entity-dispatched = Message dispatched to entity
root-list-entities = #root: list entities
entity-created = Entity created
entity-reloaded = Entity plugin reloaded
entity-reload-skipped = Entity reload skipped because the reload gate is closed
entity-reload-started = Entity reload started
entity-reload-kind-lookup-failed = Failed to load the manifest for kind lookup while reloading an entity
entity-reload-kind-missing = Kind not found in the manifest; cannot reload entity
entity-reload-kind-fetch-failed = Failed to fetch the kind node while reloading an entity
entity-reload-kind-extends-failed = Failed to resolve the kind extends chain while reloading an entity
entity-reload-manifest-state-update-failed = Failed to update the manifest with the current state before reload; keeping the current plugin
entity-reload-state-persist-failed = Failed to persist the current state before reload; keeping the current plugin
entity-reload-current-node-load-failed = Failed to load the current entity node before reload; keeping the current plugin
entity-reload-failed = Entity failed to reload; unloading it until the next reload
entity-reload-state-produced-failed = Failed to persist the state produced during reload
entity-reloaded-manifest-updated = Updated reloaded entity in manifest
entity-reloaded-manifest-update-failed = Failed to update the reloaded entity in the manifest
bootstrap-remote-root-pin-misconfigured = Remote root pinning is misconfigured
bootstrap-remote-root-pin-confirmed = Remote root pin confirmed
bootstrap-kinds-tree-published = Published runtime kinds tree
bootstrap-kinds-overlay-pin-update-failed = Pin/update failed after the kinds overlay
bootstrap-kinds-overlay-published = Published runtime manifest after the kinds overlay
bootstrap-runtime-manifest-published = Published runtime root manifest
bootstrap-root-pin-replacement-failed = Continuing after a remote root pin replacement failure
bootstrap-root-pin-update-failed = Pin/update failed after bootstrap
bootstrap-kind-published = Published kind node
bootstrap-entity-registering-prepublished = Registering pre-published entity
bootstrap-entity-published = Published entity node
bootstrap-acl-published = Published ACL node
bootstrap-group-published = Published group node
bootstrap-root-acl-published = Published root transport-gate ACL
bootstrap-kind-registry-hydrated = Kind registry hydrated from manifest
bootstrap-lifecycle-manifest-pin-update-failed = Pin/update failed after lifecycle persist
bootstrap-lifecycle-manifest-published = Published updated manifest after lifecycle transitions
bootstrap-lifecycle-manifest-publish-failed = Failed to publish manifest after lifecycle transitions
bootstrap-entity-lifecycle-updated = Updated entity lifecycle in IPFS
bootstrap-entity-lifecycle-update-failed = Failed to write updated entity lifecycle to IPFS
bootstrap-entity-node-shutdown-updated = Updated entity node on shutdown
bootstrap-entity-registry-not-in-manifest = Entity in registry but not in manifest, skipping
plugin-outbox-drain-limit = plugin outbox drain budget exhausted; deferring remaining envelopes
plugin-outbox-congested = plugin outbox congested; envelopes may be dropped if channel fills
plugin-envelope-local-gate-closed = plugin envelope: local dispatch gate closed
plugin-envelope-local-timeout = plugin envelope: local dispatch timed out
plugin-envelope-recipient-invalid = plugin envelope: invalid recipient DID; skipped
plugin-envelope-build-failed = plugin envelope: failed to build message; skipped
plugin-envelope-remote-limit = plugin envelope: remote delivery limit reached; envelope dropped
scheduled-dispatch-error = scheduled dispatch error
scheduled-dispatch-manifest-writer-unavailable = scheduled dispatch: manifest writer is not ready; entity state remains pending
manifest-pin-update-failed = manifest pin_update failed
bootstrap-kind-registry-fetch-log-failed = Failed to fetch kind node for registry
bootstrap-entity-state-update-fetch-failed = Failed to fetch entity node for state update
schedule-stale-dispatch-skipped = scheduled dispatch skipped: stale schedule
schedule-random-reschedule-failed = failed to reschedule random job
schedule-random-create-failed = failed to create next random job
schedule-random-chain-stopped = random schedule chain stopped: superseded by newer definition
schedule-entity-not-found = scheduled dispatch: entity not found
schedule-dispatch-firing = scheduled dispatch firing
bootstrap-kind-registry-extends-failed = Failed to resolve kind extends chain for registry
bootstrap-entity-registry-fetch-failed = Failed to fetch entity node
bootstrap-entity-registry-kind-missing = Kind not found in manifest; skipping entity
bootstrap-entity-registry-kind-fetch-failed = Failed to fetch kind node
bootstrap-entity-registry-kind-extends-failed = Failed to resolve kind extends chain
bootstrap-manifest-fetch-failed = Failed to fetch runtime manifest
bootstrap-minimal-manifest-failed = Failed to bootstrap minimal manifest
bootstrap-entity-state-save-failed = Failed to save entity states
bootstrap-entity-state-shutdown-aborted = shutdown aborted; runtime remains active so state can be saved on a later shutdown attempt
bootstrap-root-cid-shutdown-persist-failed = failed to persist root_cid during shutdown
bootstrap-root-cid-shutdown-publish-succeeded = shutdown runtime_ipns publish succeeded
bootstrap-root-cid-shutdown-publish-failed = shutdown runtime_ipns publish failed
bootstrap-root-cid-shutdown-publish-timeout = shutdown runtime_ipns publish timed out
bootstrap-endpoint-close-timeout = endpoint close timed out after 5 s; forcing exit
bootstrap-endpoint-close-stuck = endpoint still held by in-flight tasks after 10 s; dropping without graceful close
bootstrap-envelope-delivery-failed = plugin envelope delivery failed; dropping envelope
bootstrap-envelope-open-failed = plugin envelope: outbox open failed; dropping envelope
boot-minimal-manifest-not-found = No runtime root CID found in IPNS; bootstrapping minimal manifest
boot-minimal-manifest-bootstrapped = Minimal manifest bootstrapped
boot-kinds-overlay-no-change = Kinds overlay made no manifest changes
boot-kinds-overlay-applied = Kinds overlay applied
boot-load-manifest-for-acl-cache-failed = Failed to load manifest for ACL cache population
boot-root-acl-loaded-from-manifest = Root transport-gate ACL loaded from manifest
boot-root-acl-load-failed = Failed to load root ACL from manifest
boot-group-loaded-into-cache = Group loaded into cache
boot-group-load-failed = Failed to load group at startup
boot-root-acl-loaded-into-cache = Root ACL loaded into cache
boot-root-acl-load-cache-failed = Failed to load root ACL at startup
boot-reconciled-owners-persist-failed = Failed to persist reconciled owners to config.yaml
boot-reconciled-owners-published = Owners reconciled from config.yaml/--owner into manifest at startup
boot-reconciled-owners-manifest-failed = Failed to reconcile owners into manifest at startup
boot-no-root-entity = No root entity registered for default config root
boot-default-root-config-populated = Default config root populated
boot-default-root-config-populate-failed = Failed to populate default config root
boot-entity-load-processed = Entity plugins loaded
plugin-envelope-local-recipient-unknown = Plugin envelope: unknown local recipient; skipped
plugin-envelope-local-dispatch-start = Plugin envelope: local dispatch start
plugin-envelope-local-dispatch-finish = Plugin envelope: local dispatch finish
plugin-envelope-local-dispatch-failed = Plugin envelope: local dispatch failed
plugin-envelope-create-requests-ignored = Plugin envelope: create requests ignored without side-effect context
ma-create-entity-already-exists = ma_create_entity: entity already exists; keeping current entity
ma-create-entity-kind-missing = ma_create_entity: kind not in registry; skipped
ma-create-entity-invalid-behaviour = ma_create_entity: invalid behaviour reference; skipped
crud-message-rejected = CRUD message rejected
inbox-message-rejected = Inbox message rejected
entity-deleted = Entity deleted
entity-states-saving = Saving entity states to IPFS
entity-state-saving = Saving entity state
entity-state-saved = Entity state saved
entity-state-empty = Plugin returned empty state, skipping persist
entity-states-saved = Entity states saved

# First-run auto-init

# Ownership / claim
runtime-claimed = Runtime claimed.

# Protected root elements
refuse-delete-root = Steadfastly refuse to delete required root element
runtime-claim-persisted = Owner written to config.

# Namespace creation (:create)

# CRUD validation errors
cidv1-required = value must be a bare CIDv1 (starts with 'b'; CIDv0 'Qm…' not accepted)
config-key-protected = config key '%key%' is protected
config-key-no-delete = daemon config key '%key%' cannot be deleted
config-key-not-manifest = config key '%key%' is not a known manifest config key
wrong-crud-protocol = wrong CRUD protocol: %type%
entity-name-invalid = entity name must be printable UTF-8
reserved-entity-name = entity name '%name%' is reserved
genesis-kind-owner-only = only a runtime owner may create an entity of a genesis kind

# IPv6 config
ipv6-enabled = IPv6 enabled — binding both IPv4 and IPv6
ipv6-disabled = IPv6 disabled — binding IPv4 only (restart required to re-enable)
ipv6-enable-restart-required = Saved. Restart required for this change to take effect.
ipv6-enable-unchanged = ipv6_enable is already set to that value — no change.

# no-rpc refactor: inbox dispatch + boot keys
boot-generated-headless-config = Generated headless config.
boot-initialising-new-identity = Initialising new runtime identity.
boot-kinds-cid-usage-invalid = invalid kinds CID
boot-no-config-found = No config found.
boot-root-cid-reset = root CID reset
boot-root-cid-usage-invalid = invalid root CID
boot-runtime-ipfs-service-disabled = IPFS publisher service disabled
boot-runtime-ipfs-service-enabled = IPFS publisher service enabled
inbox-message-received = Received inbox message
