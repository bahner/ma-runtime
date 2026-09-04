# ma-runtime – Tok Pisin
lang-name = Tok Pisin

own-did-published = Dokumen bilong DID i bin planim long IPNS
own-did-publish-failed = Planim bilong dokumen DID i no wok
own-did-publish-timeout = Planim bilong dokumen DID i pinis taim bihain 2 minit
started = ma runtime i stat
shutdown-requested = Klospim i bin askim
closing-endpoint = Klospim poin bilong iroh...
shutdown-complete = Klospim i pinis
status-listening = Siva bilong stetes i harim
ipfs-message-rejected = Mesej IPFS i bin rausim
ctrlc-handler-failed = Handler bilong Ctrl-C i no wok
node-connected = Nod i konek long protokol
received-encrypted-ma-msg = Kisim mesej ma-msg i bin haitim long /ma/ipfs/0.0.1
ping-received = :ping i kamap, salim :pong
did-publish-request-received = Kisim askim bilong planim dokumen DID
document-published = Dokumen i bin planim
did-publish-cid-reply-sent = Bekim CID bilong planim DID i bin salim
did-publish-resolve-failed = I no inap painim sende bilong givim bekim ipfs-publish
ipfs-store-request-received = Kisim askim bilong storim IPFS
ipfs-stored = Kontens i storim long IPFS
ipfs-store-cid-reply-sent = Bekim CID i bin salim
ipfs-store-resolve-failed = I no inap painim sende bilong givim bekim ipfs-store

# Entity dispatch
bootstrap-complete = Bootstrap i pinis
entity-loaded = Plugin bilong entiti i lod
entity-load-failed = Lodim plugin bilong entiti i no wok
root-list-entities = #root: soim lis bilong entiti
entity-created = Entiti i wokim
entity-reloaded = Plugin bilong entiti i lod gen
entity-deleted = Entiti i rausim
entity-states-saving = Seivim stet bilong entiti long IPFS
entity-state-saving = Seivim stet bilong entiti
entity-state-saved = Stet bilong entiti i seivim
entity-state-empty = Plugin i givim stet nating, lusim
entity-states-saved = Stet bilong entiti i seivim

# First-run auto-init

# Ownership / claim
runtime-claimed = Runtime i klemim.

# Protected root elements
refuse-delete-root = Tok no long rausim elementis bilong rut
runtime-claim-persisted = Ona i raitim long konfigurasion.


# Namespace creation (:create)
crud-message-received = CRUD mesej i kam pinis
crud-acl-updated = Root transport ACL i nupela pinis

# CRUD validation errors
cidv1-required = namba i mas baim CIDv1 nating ('b' i kirap long en; CIDv0 'Qm…' no orait)
config-key-protected = config ki '%key%' i gat banis
config-key-no-delete = i no inap rausim daemon config ki '%key%'
config-key-not-manifest = config ki '%key%' i no wan manifest config ki we ol save
wrong-crud-protocol = CRUD protokol i rong: %type%
entity-name-invalid = nem bilong entity i mas stap printable UTF-8
reserved-entity-name = nem bilong entity '%name%' i reserved
genesis-kind-owner-only = Ona bilong runtime tasol inap wokim entity bilong genesis kain

# IPv6 config
ipv6-enabled = IPv6 i wok — em i baindim IPv4 na IPv6 tupela
ipv6-disabled = IPv6 i no wok — IPv4 tasol i wok long nau (restart i nidim bilong mekim i wok gen)
ipv6-enable-restart-required = Seivim pinis. Restart i nidim bilong dispela senis i wok.
ipv6-enable-unchanged = ipv6_enable i stap olredi long dispela namba — i no gat senis.

entity-reload-skipped = Rilodim entity i bin skipim bikos get bilong rilod i klosim
entity-reload-started = Rilodim entity i bin stat pinis
entity-reload-kind-lookup-failed = I no inap lod manifest bilong painim kind taim rilodim entity
entity-reload-kind-missing = Kind i no stap long manifest; i no inap rilodim entity
entity-reload-kind-fetch-failed = I no inap kisim kind node taim rilodim entity
entity-reload-kind-extends-failed = I no inap ripsolvim jein bilong kind extends taim rilodim entity
entity-reload-manifest-state-update-failed = I no inap apdetim manifest wantaim stet nau bipo rilod; i haptaim plugin nau
entity-reload-state-persist-failed = I no inap haptaim stet nau bipo rilod; i haptaim plugin nau
entity-reload-current-node-load-failed = I no inap lod entity node nau bipo rilod; i haptaim plugin nau
entity-reload-failed = Entity i bin pailing bilong rilod; i kisautim em inap nekis rilod
entity-reload-state-produced-failed = I no inap haptaim stet i bin wokim taim rilod
entity-reloaded-manifest-updated = Entity i bin rilod i bin apdetim long manifest
entity-reloaded-manifest-update-failed = I no inap apdetim entity i bin rilod long manifest
bootstrap-remote-root-pin-misconfigured = Remote root pinning i bin setapim rongwei
bootstrap-remote-root-pin-confirmed = Remote root pin i bin konfomim pinis
bootstrap-kinds-tree-published = Runtime kinds diwai i bin pabalasim pinis
bootstrap-kinds-overlay-pin-update-failed = Pin/update i bin pailing bihain kinds overlay
bootstrap-kinds-overlay-published = Runtime manifest i bin pabalasim bihain kinds overlay
bootstrap-runtime-manifest-published = Runtime root manifest i bin pabalasim pinis
bootstrap-root-pin-replacement-failed = I bihainim bihain pailing bilong senisim remote root pin
bootstrap-root-pin-update-failed = Pin/update i bin pailing bihain bootstrap
bootstrap-kind-published = Kind node i bin pabalasim pinis
bootstrap-entity-registering-prepublished = I bin rejistaim entity i bin pabalasim pinis bipo
bootstrap-entity-published = Entity node i bin pabalasim pinis
bootstrap-acl-published = ACL node i bin pabalasim pinis
bootstrap-group-published = Grup node i bin pabalasim pinis
bootstrap-root-acl-published = Root transport-gate ACL i bin pabalasim pinis
bootstrap-kind-registry-hydrated = Kind registry i bin baim long manifest
bootstrap-lifecycle-manifest-pin-update-failed = Pin/update i bin pailing bihain savim lifecycle
bootstrap-lifecycle-manifest-published = Manifest i bin apdetim bihain ol senisim bilong lifecycle
bootstrap-lifecycle-manifest-publish-failed = I no inap pabalasim manifest bihain ol senisim bilong lifecycle
bootstrap-entity-lifecycle-updated = Entity lifecycle i bin apdetim long IPFS
bootstrap-entity-lifecycle-update-failed = I no inap raitim apdet bilong entity lifecycle long IPFS
bootstrap-entity-node-shutdown-updated = Entity node i bin apdetim taim i sutdaun
bootstrap-entity-registry-not-in-manifest = Entity i stap long registry tasol i no stap long manifest; i skipim
plugin-outbox-drain-limit = Bajit bilong drenim plugin outbox i bin pinis; i deferrim ol envelope i stap yet
plugin-outbox-congested = Plugin outbox i fulap; ol envelope inap dropim sapos kanal i fulap
plugin-envelope-local-gate-closed = Plugin envelope: get bilong lokal dispatch i klosim
plugin-envelope-local-timeout = Plugin envelope: lokal dispatch i bin pinis taim
plugin-envelope-recipient-invalid = Plugin envelope: DID bilong risipient i no stret; i skipim
plugin-envelope-build-failed = Plugin envelope: i no inap bildim mesej; i skipim
plugin-envelope-remote-limit = Plugin envelope: limit bilong deliveri i go long hap i bin ritim; i dropim envelope
scheduled-dispatch-error = Erra bilong dispacim i bin skelim
scheduled-dispatch-manifest-writer-unavailable = Dispacim i bin skelim: raitman bilong manifest i no redi; stet bilong entity i stap wet
manifest-pin-update-failed = Manifest pin_update i bin pailing
bootstrap-kind-registry-fetch-log-failed = I no inap kisim kind node bilong registry
bootstrap-entity-state-update-fetch-failed = I no inap kisim entity node bilong apdet stet
schedule-stale-dispatch-skipped = Dispacim i bin skelim i skipim: ol skelim i bin plaua pinis
schedule-random-reschedule-failed = I no inap skelim gen rendem wok
schedule-random-create-failed = I no inap mekim nekis rendem wok
schedule-random-chain-stopped = Jein bilong skelim rendem i bin stopim: niuwa definisin i bin kisautim em
schedule-entity-not-found = Dispacim i bin skelim: entity i no painim
schedule-dispatch-firing = Dispacim i bin skelim i paiim
bootstrap-kind-registry-extends-failed = I no inap ripsolvim jein bilong kind extends bilong registry
bootstrap-entity-registry-fetch-failed = I no inap kisim entity node
bootstrap-entity-registry-kind-missing = Kind i no stap long manifest; i skipim entity
bootstrap-entity-registry-kind-fetch-failed = I no inap kisim kind node
bootstrap-entity-registry-kind-extends-failed = I no inap ripsolvim jein bilong kind extends
bootstrap-manifest-fetch-failed = I no inap kisim runtime manifest
bootstrap-minimal-manifest-failed = I no inap stat liklik manifest
bootstrap-entity-state-save-failed = I no inap sevim ol stet bilong entity
bootstrap-entity-state-shutdown-aborted = Sutdaun i bin kanselim; runtime i stap gut yet bai stet inap sevim long nekis sutdaun
bootstrap-root-cid-shutdown-persist-failed = I no inap haptaim root_cid taim sutdaun
bootstrap-root-cid-shutdown-publish-succeeded = Pabalasim runtime_ipns taim sutdaun i bin gutpela pinis
bootstrap-root-cid-shutdown-publish-failed = Pabalasim runtime_ipns taim sutdaun i bin pailing
bootstrap-root-cid-shutdown-publish-timeout = Pabalasim runtime_ipns taim sutdaun i bin pinis taim
bootstrap-endpoint-close-timeout = Taim bilong klosim endpoint i pinis bihain 5 s; i fosis bilong kisim autpaut
bootstrap-endpoint-close-stuck = Endpoint i stap hap bai ol wok i flai bihain 10 s; i bin dropim em i go nogat klos gut
bootstrap-envelope-delivery-failed = Deliveri bilong plugin envelope i bin pailing; i dropim envelope
bootstrap-envelope-open-failed = Plugin envelope: oponim outbox i bin pailing; i dropim envelope
boot-minimal-manifest-not-found = Nogat runtime root CID long IPNS; em i stat liklik manifest
boot-minimal-manifest-bootstrapped = Liklik manifest i bin stat pinis
boot-kinds-overlay-no-change = Kinds overlay i no senisim manifest
boot-kinds-overlay-applied = Kinds overlay i bin aplaim pinis
boot-load-manifest-for-acl-cache-failed = I no inap lod manifest bilong baim ACL cache
boot-root-acl-loaded-from-manifest = Root transport-gate ACL i bin lod long manifest
boot-root-acl-load-failed = I no inap lod root ACL long manifest
boot-group-loaded-into-cache = Grup i bin go insait long cache
boot-group-load-failed = I no inap lod grup taim sistem i stat
boot-root-acl-loaded-into-cache = Root ACL i bin go insait long cache
boot-root-acl-load-cache-failed = I no inap lod root ACL taim stat
boot-reconciled-owners-persist-failed = I no inap sevim ol owner i bin rekonsail long config.yaml
boot-reconciled-owners-published = Ol owner i bin rekonsail long config.yaml/--owner i go long manifest taim stat
boot-reconciled-owners-manifest-failed = I no inap bringim ol owner i go wantaim long manifest taim stat
boot-no-root-entity = Nogat root entity i bin rejista bilong default config root
boot-default-root-config-populated = Default config root i bin baim pinis
boot-default-root-config-populate-failed = I no inap baim default config root
boot-entity-load-processed = Entity plugin i bin lod pinis
plugin-envelope-local-recipient-unknown = Plugin envelope: lokal risipient i no save; i skipim
plugin-envelope-local-dispatch-start = Plugin envelope: lokal dispatch i bin stat
plugin-envelope-local-dispatch-finish = Plugin envelope: lokal dispatch i bin pinis
plugin-envelope-local-dispatch-failed = Plugin envelope: lokal dispatch i bin pailing
plugin-envelope-create-requests-ignored = Plugin envelope: ol rikwest bilong mekim i bin ananisim nogat side-effect context
ma-create-entity-already-exists = ma_create_entity: entity i stap pinis; i haptaim entity nau
ma-create-entity-kind-missing = ma_create_entity: kind i no stap long registry; i skipim
ma-create-entity-invalid-behaviour = ma_create_entity: referens bilong behaviour i no stret; i skipim
crud-message-rejected = CRUD mesej i bin rijijektim pinis
inbox-message-rejected = Mesej bilong inbox i bin rijijektim pinis
