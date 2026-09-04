# ma-runtime – Patwa
lang-name = Patwa

own-did-published = Owna DID dokument publish to IPNS
own-did-publish-failed = Cyaan publish owna DID dokument
own-did-publish-timeout = Owna DID dokument publish time out afta 2 minit
started = ma runtime staat
shutdown-requested = Shutdown request mek
closing-endpoint = A klooz iroh endpoint...
shutdown-complete = Shutdown don
status-listening = Status server a listen
ipfs-message-rejected = IPFS message reject
ctrlc-handler-failed = Ctrl-C handler fail
node-connected = Node connect to protocol
received-encrypted-ma-msg = Receive encrypt ma-msg pan /ma/ipfs/0.0.1
ping-received = :ping receive, a send :pong
did-publish-request-received = Receive DID dokument publish request
document-published = Dokument publish
did-publish-cid-reply-sent = CID reply fi DID publish send
did-publish-resolve-failed = Cyaan resolve sender fi deliver ipfs-publish reply
ipfs-store-request-received = Receive IPFS store request
ipfs-stored = Content store pon IPFS
ipfs-store-cid-reply-sent = CID reply send
ipfs-store-resolve-failed = Cyaan resolve sender fi deliver ipfs-store reply

# Entity dispatch
bootstrap-complete = Bootstrap don
entity-loaded = Entity plugin load
entity-load-failed = Cyaan load entity plugin
root-list-entities = #root: list entity dem
entity-created = Entity create
entity-reloaded = Entity plugin load op fresh agen
entity-deleted = Entity delete
entity-states-saving = Saving entity state dem to IPFS
entity-state-saving = A save entity state
entity-state-saved = Entity state save
entity-state-empty = Plugin return empty state, skip it
entity-states-saved = Entity state dem save

# First-run auto-init

# Ownership / claim
runtime-claimed = Runtime claim.

# Protected root elements
refuse-delete-root = Steadfastly refuse fi delete required root element
runtime-claim-persisted = Owner write to config.


# Namespace creation (:create)
crud-message-received = CRUD mesij receive
crud-acl-updated = Root transport ACL update

# CRUD validation errors
cidv1-required = di value haffi be a bare CIDv1 (start wid 'b'; CIDv0 'Qm…' nuh accepted)
config-key-protected = config key '%key%' protect
config-key-no-delete = daemon config key '%key%' cyaan delete
config-key-not-manifest = config key '%key%' nuh known manifest config key
wrong-crud-protocol = rong CRUD protokol: %type%
entity-name-invalid = di entity name haffi be printable UTF-8
reserved-entity-name = di entity name '%name%' reserved
genesis-kind-owner-only = Ongle a runtime Owner can mek a entity a genesis kind

# IPv6 config
ipv6-enabled = IPv6 enable — a bind IPv4 an IPv6 baat
ipv6-disabled = IPv6 disable — ongle IPv4 a bind (restart need fi enable it back)
ipv6-enable-restart-required = Sav. Restart need fi dis change fi tek effect.
ipv6-enable-unchanged = ipv6_enable already set to dat value — no change.

# New keys
boot-default-root-config-populate-failed = Kyaan fill up di default config root
boot-default-root-config-populated = Default config root full up
boot-entity-load-processed = Entity plugin dem load up
boot-group-load-failed = Kyaan load di group wen start up
boot-group-loaded-into-cache = Group load inna di cache
boot-kinds-overlay-applied = Kinds overlay put on
boot-kinds-overlay-no-change = Kinds overlay nuh mek no change to di manifest
boot-load-manifest-for-acl-cache-failed = Kyaan load manifest fi fill up ACL cache
boot-minimal-manifest-bootstrapped = Di likkle manifest bootstrap
boot-minimal-manifest-not-found = Cyaan find no runtime root CID inna IPNS; a bootstrap di likkle manifest
boot-no-root-entity = No root entity sign up fi di default config root
boot-reconciled-owners-manifest-failed = Kyaan put di owners dem together inna manifest when start up
boot-reconciled-owners-persist-failed = Kyaan save di reconcile owners dem to config.yaml
boot-reconciled-owners-published = Owners dem reconcile from config.yaml/--owner inna manifest when start up
boot-root-acl-load-cache-failed = Kyaan load di root ACL when start up
boot-root-acl-load-failed = Kyaan load di root ACL from manifest
boot-root-acl-loaded-from-manifest = Root transport gate ACL load from manifest
boot-root-acl-loaded-into-cache = Root ACL load inna cache
bootstrap-acl-published = ACL node publish
bootstrap-endpoint-close-stuck = Endpoint still hold by task dem inna flight after 10 s; drop without nice close
bootstrap-endpoint-close-timeout = Close endpoint run out a time after 5 s; force quit
bootstrap-entity-lifecycle-update-failed = Kyaan write di update entity lifecycle to IPFS
bootstrap-entity-lifecycle-updated = Entity lifecycle update inna IPFS
bootstrap-entity-node-shutdown-updated = Entity node update when shut down
bootstrap-entity-published = Entity node publish
bootstrap-entity-registering-prepublished = Sign up di pre-publish entity
bootstrap-entity-registry-fetch-failed = Kyaan get di entity node
bootstrap-entity-registry-kind-extends-failed = Kyaan work out di kind extends chain
bootstrap-entity-registry-kind-fetch-failed = Kyaan get di kind node
bootstrap-entity-registry-kind-missing = Kind nuh deh inna manifest; skip di entity
bootstrap-entity-registry-not-in-manifest = Entity deh inna registry but not inna manifest; skip
bootstrap-entity-state-save-failed = Kyaan save di entity state dem
bootstrap-entity-state-shutdown-aborted = Shutdown cancel; runtime stay active so state can save pon a later shutdown try
bootstrap-entity-state-update-fetch-failed = Kyaan get di entity node fi update di state
bootstrap-envelope-delivery-failed = Plugin envelope delivery fail; drop di envelope
bootstrap-envelope-open-failed = Plugin envelope: open outbox fail; drop di envelope
bootstrap-group-published = Group node publish
bootstrap-kind-published = Kind node publish
bootstrap-kind-registry-extends-failed = Kyaan work out di kind extends chain fi registry
bootstrap-kind-registry-fetch-log-failed = Kyaan get di kind node fi registry
bootstrap-kind-registry-hydrated = Kind registry water up from manifest
bootstrap-kinds-overlay-pin-update-failed = Pin/update fail after di kinds overlay
bootstrap-kinds-overlay-published = Runtime manifest publish after di kinds overlay
bootstrap-kinds-tree-published = Runtime kinds tree publish
bootstrap-lifecycle-manifest-pin-update-failed = Pin/update fail after save lifecycle
bootstrap-lifecycle-manifest-publish-failed = Kyaan publish manifest after lifecycle changes
bootstrap-lifecycle-manifest-published = Update manifest publish after lifecycle changes
bootstrap-manifest-fetch-failed = Kyaan get di runtime manifest
bootstrap-minimal-manifest-failed = Kyaan bootstrap di likkle manifest
bootstrap-remote-root-pin-confirmed = Far away root pin confirm
bootstrap-remote-root-pin-misconfigured = Far away root pinning set up wrong
bootstrap-root-acl-published = Root transport gate ACL publish
bootstrap-root-cid-shutdown-persist-failed = Kyaan save root_cid when shut down
bootstrap-root-cid-shutdown-publish-failed = Runtime_ipns publish fail when shut down
bootstrap-root-cid-shutdown-publish-succeeded = Runtime_ipns publish work when shut down
bootstrap-root-cid-shutdown-publish-timeout = Runtime_ipns publish run out a time when shut down
bootstrap-root-pin-replacement-failed = Continue after di far away root pin replace fail
bootstrap-root-pin-update-failed = Pin/update fail after bootstrap
bootstrap-runtime-manifest-published = Runtime root manifest publish
crud-message-rejected = CRUD message reject
entity-reload-current-node-load-failed = Kyaan load di current entity node before reload; keep di current plugin
entity-reload-failed = Entity fail fi reload; unload till di next reload
entity-reload-kind-extends-failed = Kyaan work out di kind extends chain while reload di entity
entity-reload-kind-fetch-failed = Kyaan get di kind node while reload di entity
entity-reload-kind-lookup-failed = Kyaan load di manifest fi look up kind while reload di entity
entity-reload-kind-missing = Kind nuh deh inna di manifest; cyaan reload entity
entity-reload-manifest-state-update-failed = Kyaan update di manifest wid di current state before reload; keep di current plugin
entity-reload-skipped = Entity reload skip because di reload gate close
entity-reload-started = Entity reload start
entity-reload-state-persist-failed = Kyaan save di current state before reload; keep di current plugin
entity-reload-state-produced-failed = Kyaan save di state produce during reload
entity-reloaded-manifest-update-failed = Kyaan update di reload entity inna di manifest
entity-reloaded-manifest-updated = Update di reload entity inna manifest
inbox-message-rejected = Inbox message reject
ma-create-entity-already-exists = ma_create_entity: entity already deh; keep di current entity
ma-create-entity-invalid-behaviour = ma_create_entity: behaviour reference no good; skip
ma-create-entity-kind-missing = ma_create_entity: kind not inna registry; skip
manifest-pin-update-failed = Manifest pin_update fail
plugin-envelope-build-failed = Plugin envelope: kyaan build di message; skip
plugin-envelope-create-requests-ignored = Plugin envelope: create request ignore without side-effect context
plugin-envelope-local-dispatch-failed = Plugin envelope: local dispatch fail
plugin-envelope-local-dispatch-finish = Plugin envelope: local dispatch finish op
plugin-envelope-local-dispatch-start = Plugin envelope: local dispatch start op
plugin-envelope-local-gate-closed = Plugin envelope: local dispatch gate close
plugin-envelope-local-recipient-unknown = Plugin envelope: unknown local receiver; skip
plugin-envelope-local-timeout = Plugin envelope: local dispatch run out a time
plugin-envelope-recipient-invalid = Plugin envelope: receiver DID no good; skip
plugin-envelope-remote-limit = Plugin envelope: far away delivery limit reach; envelope drop
plugin-outbox-congested = Plugin outbox congest up; envelope might drop if channel fill
plugin-outbox-drain-limit = Plugin outbox drain budget use up; hold back di rest a di envelope dem
schedule-dispatch-firing = Schedule dispatch fire
schedule-entity-not-found = Schedule dispatch: entity not find
schedule-random-chain-stopped = Random schedule chain stop: newer definition take over
schedule-random-create-failed = Kyaan create next random job
schedule-random-reschedule-failed = Kyaan reschedule di random job
schedule-stale-dispatch-skipped = Schedule dispatch skip: schedule old
scheduled-dispatch-error = Schedule dispatch error
scheduled-dispatch-manifest-writer-unavailable = Schedule dispatch: manifest writer not ready; entity state still waiting
