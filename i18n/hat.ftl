# ma-runtime – Kreyòl ayisyen
lang-name = Kreyòl ayisyen

own-did-published = Dokiman DID pwòp li pibliye nan IPNS
own-did-publish-failed = Echèk pou pibliye pwòp dokiman DID
own-did-publish-timeout = Pibliye pwòp dokiman DID ekspire apre 2 minit
started = ma runtime kòmanse
shutdown-requested = Fèmti mande
closing-endpoint = Fèmti pwen iroh...
shutdown-complete = Fèmti konplè
status-listening = Sèvè estati ap koute
ipfs-message-rejected = Mesaj IPFS rejte
ctrlc-handler-failed = Jestyon Ctrl-C echwe
node-connected = Nod konekte ak pwotokòl
received-encrypted-ma-msg = Resevwa mesaj ma-msg chifre sou /ma/ipfs/0.0.1
ping-received = :ping resevwa, ap voye :pong
did-publish-request-received = Resevwa demann pibliye dokiman DID
document-published = Dokiman pibliye
did-publish-cid-reply-sent = Repons CID pou pibliye DID voye
did-publish-resolve-failed = Pa ka rezoud ekspeditiè pou delivre repons ipfs-publish
ipfs-store-request-received = Resevwa demann depo IPFS
ipfs-stored = Kontni estoke sou IPFS
ipfs-store-cid-reply-sent = Repons CID voye
ipfs-store-resolve-failed = Pa ka rezoud ekspeditiè pou delivre repons ipfs-store

# Entity dispatch
bootstrap-complete = Bootstrap konplè
entity-loaded = Plugin antite chaje
entity-load-failed = Echèk pou chaje plugin antite
root-list-entities = #root: liste antite
entity-created = Antite kreye
entity-reloaded = Plugin antite rechaje
entity-deleted = Antite efase
entity-states-saving = Ap sove eta antite nan IPFS
entity-state-saving = Ap sove eta antite
entity-state-saved = Eta antite sove
entity-state-empty = Plugin retounen eta vid, pase
entity-states-saved = Eta antite sove

# First-run auto-init

# Ownership / claim
runtime-claimed = Runtime reklame.

# Protected root elements
refuse-delete-root = Refize efase eleman rasin obligatwa
runtime-claim-persisted = Pwopriyetè ekri nan konfigirasyon.


# Namespace creation (:create)
crud-message-received = Mesaj CRUD resevwa
crud-acl-updated = ACL transpò rasin mete ajou

# CRUD validation errors
cidv1-required = valè a dwe se yon CIDv1 tou nèf (kòmanse ak 'b'; CIDv0 'Qm…' pa aksepte)
config-key-protected = kle config '%key%' la pwoteje
config-key-no-delete = yo pa ka efase kle config '%key%' daemon an
config-key-not-manifest = kle config '%key%' la pa yon kle manifest config ki konnen
wrong-crud-protocol = move protokòl CRUD: %type%
entity-name-invalid = non entity a dwe UTF-8 ki ka enprime
reserved-entity-name = non entity '%name%' rezerve
genesis-kind-owner-only = Se sèlman yon pwopriyetè runtime ki ka kreye yon entity kalite genesis

# IPv6 config
ipv6-enabled = IPv6 aktive — konekte tou de IPv4 ak IPv6
ipv6-disabled = IPv6 dezaktive — IPv4 sèlman k ap konekte (restart nesesè pou reaktive)
ipv6-enable-restart-required = Sove. Restart nesesè pou chanjman sa a antre an vigè.
ipv6-enable-unchanged = ipv6_enable deja mete sou valè sa a — pa gen chanjman.

# Nouvo kle yo
boot-default-root-config-populate-failed = Pa kapab ranpli rasin config default la
boot-default-root-config-populated = Rasin config default la ranpli
boot-entity-load-processed = Plugin entity yo chaje
boot-group-load-failed = Pa kapab chaje gwoup lan pandan démarrage
boot-group-loaded-into-cache = Gwoup lan chaje nan cache
boot-kinds-overlay-applied = Kinds overlay aplike
boot-kinds-overlay-no-change = Kinds overlay pa fè okenn chanjman nan manifest
boot-load-manifest-for-acl-cache-failed = Pa kapab chaje manifest pou ranpli cache ACL
boot-minimal-manifest-bootstrapped = Manifest minimòm lan bootstrap
boot-minimal-manifest-not-found = Pa jwenn okenn runtime root CID nan IPNS; ap bootstrap manifest minimòm nan
boot-no-root-entity = Pa gen root entity anrejistre pou rasin config default la
boot-reconciled-owners-manifest-failed = Pa kapab rekonsiliasyon pwopriyetè yo nan manifest pandan démarrage
boot-reconciled-owners-persist-failed = Pa kapab konsève pwopriyetè rekonsiliasyon yo nan config.yaml
boot-reconciled-owners-published = Pwopriyetè rekonsiliasyon depi config.yaml/--owner nan manifest pandan démarrage
boot-root-acl-load-cache-failed = Pa kapab chaje ACL rasin pandan démarrage
boot-root-acl-load-failed = Pa kapab chaje ACL rasin depi manifest
boot-root-acl-loaded-from-manifest = ACL pò transpò rasin chaje depi manifest
boot-root-acl-loaded-into-cache = ACL rasin chaje nan cache
bootstrap-acl-published = Noèd ACL pibliye
bootstrap-endpoint-close-stuck = endpoint toujou kenbe pa tach an vol apre 10 s; lage san fèmti elegant
bootstrap-endpoint-close-timeout = Fèmti endpoint ekspire apre 5 s; fòse sòti
bootstrap-entity-lifecycle-update-failed = Pa kapab ekri miz ajou lifecycle entity nan IPFS
bootstrap-entity-lifecycle-updated = Lifecycle entity mete ajou nan IPFS
bootstrap-entity-node-shutdown-updated = Noèd entity mete ajou pandan fermeture
bootstrap-entity-published = Noèd entity pibliye
bootstrap-entity-registering-prepublished = Anrejistre entity ki deja pibliye
bootstrap-entity-registry-fetch-failed = Pa kapab jwenn noèd entity
bootstrap-entity-registry-kind-extends-failed = Pa kapab rezoud chèn extends kind
bootstrap-entity-registry-kind-fetch-failed = Pa kapab jwenn noèd kind
bootstrap-entity-registry-kind-missing = Kind pa jwenn nan manifest; pase entity
bootstrap-entity-registry-not-in-manifest = Entity nan registry men pa nan manifest; pase
bootstrap-entity-state-save-failed = Pa kapab sove eta entity yo
bootstrap-entity-state-shutdown-aborted = Fermeture anile; runtime rete aktif pou ka sove eta nan yon eseye fermeture apre
bootstrap-entity-state-update-fetch-failed = Pa kapab jwenn noèd entity pou miz ajou eta
bootstrap-envelope-delivery-failed = Livrezon envelope plugin echwe; lage envelope
bootstrap-envelope-open-failed = Envelope plugin: ouvèti outbox echwe; lage envelope
bootstrap-group-published = Noèd gwoup pibliye
bootstrap-kind-published = Noèd kind pibliye
bootstrap-kind-registry-extends-failed = Pa kapab rezoud chèn extends kind pou registry
bootstrap-kind-registry-fetch-log-failed = Pa kapab jwenn noèd kind pou registry
bootstrap-kind-registry-hydrated = Registry kind idrate depi manifest
bootstrap-kinds-overlay-pin-update-failed = Pin/update echwe apre kinds overlay
bootstrap-kinds-overlay-published = Manifest runtime pibliye apre kinds overlay
bootstrap-kinds-tree-published = Pyebwa kinds runtime pibliye
bootstrap-lifecycle-manifest-pin-update-failed = Pin/update echwe apre konsèvasyon lifecycle
bootstrap-lifecycle-manifest-publish-failed = Pa kapab pibliye manifest apre tranzisyon lifecycle
bootstrap-lifecycle-manifest-published = Manifest ajou pibliye apre tranzisyon lifecycle
bootstrap-manifest-fetch-failed = Pa kapab jwenn manifest runtime
bootstrap-minimal-manifest-failed = Pa kapab bootstrap manifest minimòm
bootstrap-remote-root-pin-confirmed = Pin rasin an distans konfime
bootstrap-remote-root-pin-misconfigured = Pinning rasin an distans mal konfigire
bootstrap-root-acl-published = ACL pò transpò rasin pibliye
bootstrap-root-cid-shutdown-persist-failed = Pa kapab konsève root_cid pandan fermeture
bootstrap-root-cid-shutdown-publish-failed = Piblikasyon runtime_ipns pandan fermeture echwe
bootstrap-root-cid-shutdown-publish-succeeded = Piblikasyon runtime_ipns pandan fermeture reyisi
bootstrap-root-cid-shutdown-publish-timeout = Piblikasyon runtime_ipns pandan fermeture ekspire
bootstrap-root-pin-replacement-failed = Ap kontinye apre echèk ranplasman pin rasin an distans
bootstrap-root-pin-update-failed = Pin/update echwe apre bootstrap
bootstrap-runtime-manifest-published = Manifest rasin runtime pibliye
crud-message-rejected = Mesaj CRUD rejte
entity-reload-current-node-load-failed = Pa kapab chaje noèd entity aktyèl anvan rechajman; konsève plugin aktyèl la
entity-reload-failed = Entity pa kapab rechaje; deplwaye jiskaske rechajman pwochen an
entity-reload-kind-extends-failed = Pa kapab rezoud chèn extends kind pandan rechajman entity
entity-reload-kind-fetch-failed = Pa kapab jwenn noèd kind pandan rechajman entity
entity-reload-kind-lookup-failed = Pa kapab chaje manifest pou rechèche kind pandan rechajman entity
entity-reload-kind-missing = Kind pa jwenn nan manifest; pa kapab rechaje entity
entity-reload-manifest-state-update-failed = Pa kapab mete ajou manifest avèk eta aktyèl anvan rechajman; konsève plugin aktyèl la
entity-reload-skipped = Rechajman entity pase paske pòt rechajman fèmen
entity-reload-started = Rechajman entity kòmanse
entity-reload-state-persist-failed = Pa kapab konsève eta aktyèl anvan rechajman; konsève plugin aktyèl la
entity-reload-state-produced-failed = Pa kapab konsève eta pwodwi pandan rechajman
entity-reloaded-manifest-update-failed = Pa kapab mete ajou entity rechaje nan manifest
entity-reloaded-manifest-updated = Entity rechaje mete ajou nan manifest
inbox-message-rejected = Mesaj inbox rejte
ma-create-entity-already-exists = ma_create_entity: entity deja egziste; konsève entity aktyèl la
ma-create-entity-invalid-behaviour = ma_create_entity: referans behaviour ki pa valid; pase
ma-create-entity-kind-missing = ma_create_entity: kind pa nan registry; pase
manifest-pin-update-failed = Manifest pin_update echwe
plugin-envelope-build-failed = Envelope plugin: pa kapab bati mesaj; pase
plugin-envelope-create-requests-ignored = Envelope plugin: demann kreye inyore san kontèks side-effect
plugin-envelope-local-dispatch-failed = Envelope plugin: distribisyon lokal echwe
plugin-envelope-local-dispatch-finish = Envelope plugin: distribisyon lokal fini
plugin-envelope-local-dispatch-start = Envelope plugin: distribisyon lokal kòmanse
plugin-envelope-local-gate-closed = Envelope plugin: pòt distribisyon lokal fèmen
plugin-envelope-local-recipient-unknown = Envelope plugin: destinatè lokal enkoni; pase
plugin-envelope-local-timeout = Envelope plugin: distribisyon lokal ekspire
plugin-envelope-recipient-invalid = Envelope plugin: DID destinatè ki pa valid; pase
plugin-envelope-remote-limit = Envelope plugin: limit livrezon an distans rive; envelope lage
plugin-outbox-congested = Outbox plugin kongeste; envelope ka lage si kanal ranpli
plugin-outbox-drain-limit = Bidjè drain outbox plugin epwize; ap remèt envelope ki rete yo
schedule-dispatch-firing = Distribisyon pwograme ap tire
schedule-entity-not-found = Distribisyon pwograme: entity pa jwenn
schedule-random-chain-stopped = Chèn pwogram aza rete: ranplase pa yon definisyon pi resan
schedule-random-create-failed = Pa kapab kreye travay aza pwochen an
schedule-random-reschedule-failed = Pa kapab replanifye travay aza
schedule-stale-dispatch-skipped = Distribisyon pwograme pase: pwogram date
scheduled-dispatch-error = Erè distribisyon pwograme
scheduled-dispatch-manifest-writer-unavailable = Distribisyon pwograme: ekrivèn manifest pa prè; eta entity rete an atant
