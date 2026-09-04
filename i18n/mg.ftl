# ma-runtime – Malagasy
lang-name = Malagasy

own-did-published = Ny antontan-taratasy DID ahy dia navoaka tao amin'ny IPNS
own-did-publish-failed = Tsy vitanay namoaka ny antontan-taratasy DID ahy
own-did-publish-timeout = Ny famoahana antontan-taratasy DID ahy dia afaka fotoana rehefa afaka 2 minitra
started = Nanomboka ny ma runtime
shutdown-requested = Nangataka ny fitoahana
closing-endpoint = Mikatona iroh endpoint...
shutdown-complete = Vita ny fitoahana
status-listening = Mangataka ny mpandrindra fandinihana
ipfs-message-rejected = Nandàvana ny hafatra IPFS
ctrlc-handler-failed = Tsy nahomby ny mpitantana Ctrl-C
node-connected = Ny fehezan-teny dia nampifandray amin'ny drafitra
received-encrypted-ma-msg = Voaray hafatra ma voasokafana tao /ma/ipfs/0.0.1
ping-received = Voaray :ping, alefa :pong
did-publish-request-received = Voaray fangatahana famoahana antontan-taratasy DID
document-published = Navoaka ny antontan-taratasy
did-publish-cid-reply-sent = Nalefa ny valiny CID ho an'ny famoahana DID
did-publish-resolve-failed = Tsy vitanay namaha ny mpandefitra mba hanondro ny valiny ipfs-publish
ipfs-store-request-received = Voaray fangatahana fitahirizana IPFS
ipfs-stored = Voatahiry tao amin'ny IPFS ny votoatiny
ipfs-store-cid-reply-sent = Nalefa ny valiny CID
ipfs-store-resolve-failed = Tsy vitanay namaha ny mpandefitra mba hanondro ny valiny ipfs-store

# Fanatiterahan'ny entity
bootstrap-complete = Vita ny Bootstrap
entity-loaded = Loaded ny plugin entity
entity-load-failed = Tsy vitanay ny nanidy ny plugin entity
root-list-entities = #root: lisitry ny entity
entity-created = Noforonina ny entity
entity-reloaded = Naverina nalaina ny plugin entity
entity-deleted = Nafana ny entity
entity-states-saving = Mitahiry ny toe-javatra entity any amin'ny IPFS
entity-state-saving = Mitahiry ny toe-java-tsy-misy entity
entity-state-saved = Voatahiry ny toe-java-tsy-misy entity
entity-state-empty = Naverina tsy misy ny plugin, navela ny fitahirizana
entity-states-saved = Voatahiry ny toe-javatra entity

# Fiantombohana voalohany / auto-init

# Fananana
runtime-claimed = Voasoratra ny runtime.

# Singa fototra voaaro
refuse-delete-root = Mandà mafy ny mamafa ny singa fototra ilaina
runtime-claim-persisted = Voasoratra tao amin'ny fanakianana ny tompon'andraikitra.


# Namespace creation (:create)
crud-message-received = Voaraisina ny hafatra CRUD
crud-acl-updated = Navao ny ACL fitaterana fototra

# CRUD validation errors
cidv1-required = ny sanda dia tokony ho CIDv1 tsotra (manomboka amin'ny 'b'; CIDv0 'Qm…' tsy voaray)
config-key-protected = ny fanalahidin'ny config '%key%' dia voaro
config-key-no-delete = ny fanalahidin'ny daemon config '%key%' dia tsy azo esorina
config-key-not-manifest = ny fanalahidin'ny config '%key%' dia tsy fanalahidy manifest config fantatra
wrong-crud-protocol = diso ny CRUD protocol: %type%
entity-name-invalid = ny anaran'ny entity dia tsy maintsy UTF-8 azo atonta
reserved-entity-name = ny anaran'ny entity '%name%' dia voatokana
genesis-kind-owner-only = Ny tompon'andraikitry ny runtime ihany no afaka mamorona entity amin'ny karazana genesis

# IPv6 config
ipv6-enabled = IPv6 voalefaka — mampifandray IPv4 sy IPv6
ipv6-disabled = Voarara ny IPv6 — IPv4 ihany no mifamatotra (restart no ilaina mba hamerenana azy)
ipv6-enable-restart-required = Voatahiry. Restart no ilaina mba hisy fiantraikany io fanovana io.
ipv6-enable-unchanged = Efa voapetraka amin'io sanda io ny ipv6_enable — tsy misy fanovana.

entity-reload-skipped = Fanavaozana entity novimbina satria voakatona ny vavahady fanavaozana
entity-reload-started = Fanavaozana entity nanomboka
entity-reload-kind-lookup-failed = Tsy afaka nampiditra manifest ho fitadiavana kind nandritra ny fanavaozana entity
entity-reload-kind-missing = Kind tsy hita ao amin ny manifest; tsy afaka namerenana ny entity
entity-reload-kind-fetch-failed = Tsy afaka naka kind node nandritra ny fanavaozana entity
entity-reload-kind-extends-failed = Tsy afaka namaha ny fihelezana extends kind nandritra ny fanavaozana entity
entity-reload-manifest-state-update-failed = Tsy afaka nanavaoza manifest amin ny state ankehitriny mialoha ny fanavaozana; mitazona plugin ankehitriny
entity-reload-state-persist-failed = Tsy afaka nitahiry ny state ankehitriny mialoha ny fanavaozana; mitazona plugin ankehitriny
entity-reload-current-node-load-failed = Tsy afaka nampiditra entity node ankehitriny mialoha ny fanavaozana; mitazona plugin ankehitriny
entity-reload-failed = Entity tsy nahavita fanavaozana; alaina ambara ny fanavaozana manaraka
entity-reload-state-produced-failed = Tsy afaka nitahiry state vokatra nandritra ny fanavaozana
entity-reloaded-manifest-updated = Entity navokatra voasoratra tao amin ny manifest
entity-reloaded-manifest-update-failed = Tsy afaka nanavaoza entity navokatra ao amin ny manifest
bootstrap-remote-root-pin-misconfigured = Remote root pinning voarindra diso
bootstrap-remote-root-pin-confirmed = Remote root pin voamarina
bootstrap-kinds-tree-published = Runtime kinds tree nopetraka
bootstrap-kinds-overlay-pin-update-failed = Tsy nahomby ny Pin/update aorian ny kinds overlay
bootstrap-kinds-overlay-published = Runtime manifest nopetraka aorian ny kinds overlay
bootstrap-runtime-manifest-published = Runtime root manifest nopetraka
bootstrap-root-pin-replacement-failed = Mitohy aorian ny tsy fahombiazan ny fanaovana solon ny remote root pin
bootstrap-root-pin-update-failed = Tsy nahomby ny Pin/update aorian ny bootstrap
bootstrap-kind-published = Kind node nopetraka
bootstrap-entity-registering-prepublished = Mansoratra entity efa nopetraka
bootstrap-entity-published = Entity node nopetraka
bootstrap-acl-published = Nopetraka ny ACL node
bootstrap-group-published = Vondrona node nopetraka
bootstrap-root-acl-published = Root transport-gate ACL nopetraka
bootstrap-kind-registry-hydrated = Kind registry voatezaina avy amin ny manifest
bootstrap-lifecycle-manifest-pin-update-failed = Tsy nahomby ny Pin/update aorian ny fitehirizana lifecycle
bootstrap-lifecycle-manifest-published = Manifest novàvana nopetraka aorian ny fiovana lifecycle
bootstrap-lifecycle-manifest-publish-failed = Tsy afaka namoaka manifest aorian ny fiovana lifecycle
bootstrap-entity-lifecycle-updated = Lifecycle entity navokatra tao amin ny IPFS
bootstrap-entity-lifecycle-update-failed = Tsy afaka namorona fanovana lifecycle entity tao amin ny IPFS
bootstrap-entity-node-shutdown-updated = Entity node navokatra tamin ny fikatsoana
bootstrap-entity-registry-not-in-manifest = Entity ao amin ny registry fa tsy ao amin ny manifest; afoin
plugin-outbox-drain-limit = Lany ny volan-dalana drain plugin outbox; ampitahena ny envelope sisa
plugin-outbox-congested = Plugin outbox mihevitra; envelope mety ho latsaka raha feno ny fandriana
plugin-envelope-local-gate-closed = Envelope plugin: voakatona ny vavahady fandefasana eo an-toerana
plugin-envelope-local-timeout = Envelope plugin: nifarana ny fotoana fandefasana eo an-toerana
plugin-envelope-recipient-invalid = Envelope plugin: tsy manan-danja ny DID mpandray; afoin
plugin-envelope-build-failed = Envelope plugin: tsy afaka nanamboatra hafatra; afoin
plugin-envelope-remote-limit = Envelope plugin: tonga ny fetra fandefasana lavitra; latsaka ny envelope
scheduled-dispatch-error = Fahadisoan ny fandefasana voatokana
scheduled-dispatch-manifest-writer-unavailable = Fandefasana voatokana: mpanoratra manifest tsy vonona; state entity mbola miandry
manifest-pin-update-failed = Tsy nahomby ny manifest pin_update
bootstrap-kind-registry-fetch-log-failed = Tsy afaka naka kind node ho registry
bootstrap-entity-state-update-fetch-failed = Tsy afaka naka entity node ho fanavaozana state
schedule-stale-dispatch-skipped = Fandefasana voatokana novimbina: fandaminana tranainy
schedule-random-reschedule-failed = Tsy afaka namerina ny fandaminana asa filatsa-kazo
schedule-random-create-failed = Tsy afaka namorona asa filatsa-kazo manaraka
schedule-random-chain-stopped = Filatsa-kazo voatokana mijanona: voasolo amin ny famaritana vaovao
schedule-entity-not-found = Fandefasana voatokana: entity tsy hita
schedule-dispatch-firing = Fandefasana voatokana mandeha
bootstrap-kind-registry-extends-failed = Tsy afaka namaha ny fihelezana extends kind ho registry
bootstrap-entity-registry-fetch-failed = Tsy afaka naka entity node
bootstrap-entity-registry-kind-missing = Kind tsy hita ao amin ny manifest; afoin ny entity
bootstrap-entity-registry-kind-fetch-failed = Tsy afaka naka kind node
bootstrap-entity-registry-kind-extends-failed = Tsy afaka namaha ny fihelezana extends kind
bootstrap-manifest-fetch-failed = Tsy afaka naka runtime manifest
bootstrap-minimal-manifest-failed = Tsy afaka natomboky ny manifest kely
bootstrap-entity-state-save-failed = Tsy afaka nitahiry ny state entity
bootstrap-entity-state-shutdown-aborted = Fiatoana nohazonina; runtime mbola mavitrika mba hahafahana mitahiry state amin ny fiatoana manaraka
bootstrap-root-cid-shutdown-persist-failed = Tsy afaka nitahiry root_cid nandritra ny fiatoana
bootstrap-root-cid-shutdown-publish-succeeded = Nahomby ny famoahana runtime_ipns nandritra ny fiatoana
bootstrap-root-cid-shutdown-publish-failed = Tsy nahomby ny famoahana runtime_ipns nandritra ny fiatoana
bootstrap-root-cid-shutdown-publish-timeout = Nifarana ny fotoana famoahana runtime_ipns nandritra ny fiatoana
bootstrap-endpoint-close-timeout = Nifarana ny fotoana fikatsoana endpoint aorian ny 5 s; voatery hivoaka
bootstrap-endpoint-close-stuck = Endpoint mbola voatana amin ny asana manidina aorian ny 10 s; alefa tsy misy fikatsoana tsara
bootstrap-envelope-delivery-failed = Tsy nahomby ny fandefasana envelope plugin; latsaka ny envelope
bootstrap-envelope-open-failed = Envelope plugin: tsy afaka nanokatra outbox; latsaka ny envelope
boot-minimal-manifest-not-found = Tsy hita runtime root CID ao amin ny IPNS; manomboka manifest kely
boot-minimal-manifest-bootstrapped = Manifest kely no niandoha
boot-kinds-overlay-no-change = Kinds overlay tsy nanova ny manifest
boot-kinds-overlay-applied = Kinds overlay nampiharina
boot-load-manifest-for-acl-cache-failed = Tsy afaka nampiditra manifest ho feno ny cache ACL
boot-root-acl-loaded-from-manifest = Root transport-gate ACL nampiasaina avy amin ny manifest
boot-root-acl-load-failed = Tsy afaka nampiditra root ACL avy amin ny manifest
boot-group-loaded-into-cache = Vondrona nampiasaina ao amin ny cache
boot-group-load-failed = Tsy afaka nampiditra ny vondrona tamin ny fianirandana
boot-root-acl-loaded-into-cache = Root ACL nampiasaina ao amin ny cache
boot-root-acl-load-cache-failed = Tsy afaka nampiditra root ACL tamin ny fianirandana
boot-reconciled-owners-persist-failed = Tsy afaka nitahiry ny tompo nampifanarahana tao amin ny config.yaml
boot-reconciled-owners-published = Ny tompo nampifanarahana avy amin ny config.yaml/--owner dia ao amin ny manifest tamin ny fianirandana
boot-reconciled-owners-manifest-failed = Tsy afaka nampifanaraka ny tompon ny manifest tamin ny fianirandana
boot-no-root-entity = Tsy nisy root entity voasoratra ho amin ny fakana config default root
boot-default-root-config-populated = Ny fakana config default root dia efa feno
boot-default-root-config-populate-failed = Tsy afaka nameno ny fakana config default root
boot-entity-load-processed = Plugin entity no nampiasaina
plugin-envelope-local-recipient-unknown = Envelope plugin: tsy fantatra ny mpandray eo an-toerana; afoin
plugin-envelope-local-dispatch-start = Envelope plugin: fandefasana eo an-toerana nanomboka
plugin-envelope-local-dispatch-finish = Envelope plugin: fandefasana eo an-toerana vita
plugin-envelope-local-dispatch-failed = Envelope plugin: fandefasana eo an-toerana tsy nahomby
plugin-envelope-create-requests-ignored = Envelope plugin: fangatahana famoronana tsy raisina tsy misy context side-effect
ma-create-entity-already-exists = ma_create_entity: entity efa misy; mitazona entity ankehitriny
ma-create-entity-kind-missing = ma_create_entity: kind tsy ao amin ny registry; afoin
ma-create-entity-invalid-behaviour = ma_create_entity: tsy manan-danja ny reference behaviour; afoin
crud-message-rejected = Hafatra CRUD nolavina
inbox-message-rejected = Hafatra inbox nolavina
