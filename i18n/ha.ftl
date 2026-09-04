# ma-runtime – Hausa
lang-name = Hausa

own-did-published = Takardana ta DID an buga zuwa IPNS
own-did-publish-failed = Kasa bugawa takarda ta DID
own-did-publish-timeout = Bugawa na takarda ta DID ta kare bayan minti 2
started = ma runtime ya fara
shutdown-requested = An nemi kashe
closing-endpoint = Ana rufe iroh endpoint...
shutdown-complete = Kashe ya kammala
status-listening = Sabar matsayi yana sauraro
ipfs-message-rejected = An ƙi sako na IPFS
ctrlc-handler-failed = Mai kula da Ctrl-C ya kasa
node-connected = Nod ya haɗa da yarjejeniya
received-encrypted-ma-msg = An karbi sako na ma da aka ɓoye a /ma/ipfs/0.0.1
ping-received = An karbi :ping, ana aika :pong
did-publish-request-received = An karbi buƙata don buga takarda ta DID
document-published = An buga takarda
did-publish-cid-reply-sent = An aika amsa ta CID don bugawa ta DID
did-publish-resolve-failed = Kasa warware mai aika don isar da amsa ta ipfs-publish
ipfs-store-request-received = An karbi buƙata don ajiya ta IPFS
ipfs-stored = An ajiye abun ciki a IPFS
ipfs-store-cid-reply-sent = An aika amsa ta CID
ipfs-store-resolve-failed = Kasa warware mai aika don isar da amsa ta ipfs-store

# Rarraba abubuwa
bootstrap-complete = Bootstrap ya kammala
entity-loaded = Plugin na abu ya loda
entity-load-failed = Kasa loda plugin na abu
root-list-entities = #root: jerin abubuwa
entity-created = An ƙirƙiri abu
entity-reloaded = An sake loda plugin na abu
entity-deleted = An share abu
entity-states-saving = Ana ajiye yanayin abubuwa zuwa IPFS
entity-state-saving = Ana ajiye yanayin abu
entity-state-saved = An ajiye yanayin abu
entity-state-empty = Plugin ya mayar da yanayi maras komai, an tsallake ajiyewa
entity-states-saved = An ajiye yanayin abubuwa

# Farawa na farko / farawa ta atomatik

# Mallakarwa
runtime-claimed = An yi rajistar runtime.

# Abubuwa na asali da aka kare
refuse-delete-root = Ina ƙin a gaba ɗaya share abin da ake bukata na asali
runtime-claim-persisted = An rubuta mai shi zuwa saiti.


# Namespace creation (:create)
crud-message-received = An karɓi saƙon CRUD
crud-acl-updated = An sabunta ACL na jigilar tushe

# CRUD validation errors
cidv1-required = ƙima dole ne ta zama CIDv1 na asali (tana farawa da 'b'; CIDv0 'Qm…' ba a karba)
config-key-protected = maɓallin config '%key%' yana ƙarƙashin kariya
config-key-no-delete = ba za a iya share maɓallin config '%key%' na daemon ba
config-key-not-manifest = maɓallin config '%key%' ba shi ne maɓallin manifest config da aka sani ba
wrong-crud-protocol = kuskuren CRUD protocol: %type%
entity-name-invalid = sunan entity dole ne ya kasance UTF-8 da za a buga
reserved-entity-name = sunan entity '%name%' ya keɓe
genesis-kind-owner-only = Sai mai shi na runtime zai iya ƙirƙiri entity na nau'in genesis

# IPv6 config
ipv6-enabled = An kunna IPv6 — yana ɗaurewa IPv4 da IPv6
ipv6-disabled = An kashe IPv6 — ana ɗaure IPv4 kawai (ana buƙatar sake farawa don sake kunna)
ipv6-enable-restart-required = An adana. Ana buƙatar sake farawa don wannan canjin ya yi aiki.
ipv6-enable-unchanged = ipv6_enable an riga an saita shi zuwa wannan ƙima — babu canji.

# Sabbin maɓallan
boot-default-root-config-populate-failed = An kasa cika tushen config na tsoho
boot-default-root-config-populated = An cika tushen config na tsoho
boot-entity-load-processed = An loda plugin na entity
boot-group-load-failed = An kasa loda ƙungiyar a lokacin farawa
boot-group-loaded-into-cache = An loda ƙungiyar zuwa cache
boot-kinds-overlay-applied = An aiwatar da kinds overlay
boot-kinds-overlay-no-change = Kinds overlay bai canza manifest ba
boot-load-manifest-for-acl-cache-failed = An kasa loda manifest don yada cache ACL
boot-minimal-manifest-bootstrapped = An kaddamar da manifest mafi ƙaranci
boot-minimal-manifest-not-found = Ba a sami runtime root CID a IPNS ba; ana kaddamar da manifest mafi ƙaranci
boot-no-root-entity = Ba a yi rajista ga root entity don tushen config na tsoho ba
boot-reconciled-owners-manifest-failed = An kasa sulhunta masu gida zuwa manifest a lokacin farawa
boot-reconciled-owners-persist-failed = An kasa adana masu gida da aka sulhunta zuwa config.yaml
boot-reconciled-owners-published = An sulhunta masu gida daga config.yaml/--owner zuwa manifest a farawa
boot-root-acl-load-cache-failed = An kasa loda ACL na asali a lokacin farawa
boot-root-acl-load-failed = An kasa loda ACL na asali daga manifest
boot-root-acl-loaded-from-manifest = An loda ACL na ƙofar jigilar root daga manifest
boot-root-acl-loaded-into-cache = An loda ACL na asali zuwa cache
bootstrap-acl-published = An buga node na ACL
bootstrap-endpoint-close-stuck = Har yanzu endpoint yana riƙe da ayyuka bayan daƙiƙa 10; ana jefar da shi ba da la'akari ba
bootstrap-endpoint-close-timeout = Rufe endpoint ya ƙare bayan daƙiƙa 5; ana tilasta fita
bootstrap-entity-lifecycle-update-failed = An kasa rubuta sabunta lifecycle na entity zuwa IPFS
bootstrap-entity-lifecycle-updated = An sabunta lifecycle na entity a IPFS
bootstrap-entity-node-shutdown-updated = An sabunta node na entity a lokacin kashe
bootstrap-entity-published = An buga node na entity
bootstrap-entity-registering-prepublished = Ana yin rajista da entity da aka riga aka buga
bootstrap-entity-registry-fetch-failed = An kasa dawo da node na entity
bootstrap-entity-registry-kind-extends-failed = An kasa warware sarkar extends na kind
bootstrap-entity-registry-kind-fetch-failed = An kasa dawo da node na kind
bootstrap-entity-registry-kind-missing = Ba a sami kind a manifest ba; ana barin entity
bootstrap-entity-registry-not-in-manifest = Entity yana a registry amma ba a manifest ba; ana bari
bootstrap-entity-state-save-failed = An kasa adana jihohin entity
bootstrap-entity-state-shutdown-aborted = An soke kashe; runtime yana ci gaba da aiki don a iya adana jiha a ƙoƙarin kashe na gaba
bootstrap-entity-state-update-fetch-failed = An kasa dawo da node na entity don sabunta jiha
bootstrap-envelope-delivery-failed = Isar da envelope na plugin ta gaza; ana jefar da envelope
bootstrap-envelope-open-failed = Envelope na plugin: buɗe outbox ya gaza; ana jefar da envelope
bootstrap-group-published = An buga node na ƙungiya
bootstrap-kind-published = An buga node na kind
bootstrap-kind-registry-extends-failed = An kasa warware sarkar extends na kind don registry
bootstrap-kind-registry-fetch-log-failed = An kasa dawo da node na kind don registry
bootstrap-kind-registry-hydrated = An cika registry na kind daga manifest
bootstrap-kinds-overlay-pin-update-failed = Pin/update ya gaza bayan kinds overlay
bootstrap-kinds-overlay-published = An buga manifest na runtime bayan kinds overlay
bootstrap-kinds-tree-published = An buga itacen kinds na runtime
bootstrap-lifecycle-manifest-pin-update-failed = Pin/update ya gaza bayan adana lifecycle
bootstrap-lifecycle-manifest-publish-failed = An kasa buga manifest bayan canje-canjen lifecycle
bootstrap-lifecycle-manifest-published = An buga manifest da aka sabunta bayan canje-canjen lifecycle
bootstrap-manifest-fetch-failed = An kasa dawo da manifest na runtime
bootstrap-minimal-manifest-failed = An kasa kaddamar da manifest mafi ƙaranci
bootstrap-remote-root-pin-confirmed = An tabbatar da pin na asali na nesa
bootstrap-remote-root-pin-misconfigured = Pinning na asali na nesa an saita shi da kuskure
bootstrap-root-acl-published = An buga ACL na ƙofar jigilar root
bootstrap-root-cid-shutdown-persist-failed = An kasa adana root_cid a lokacin kashe
bootstrap-root-cid-shutdown-publish-failed = Buga runtime_ipns a lokacin kashe ya gaza
bootstrap-root-cid-shutdown-publish-succeeded = Buga runtime_ipns a lokacin kashe ya yi nasara
bootstrap-root-cid-shutdown-publish-timeout = Buga runtime_ipns a lokacin kashe ya ƙare
bootstrap-root-pin-replacement-failed = Ana ci gaba bayan kuskuren maye gurbin pin na asali na nesa
bootstrap-root-pin-update-failed = Pin/update ya gaza bayan bootstrap
bootstrap-runtime-manifest-published = An buga manifest na tushe na runtime
crud-message-rejected = An ƙi sakon CRUD
entity-reload-current-node-load-failed = An kasa loda node na entity na yanzu kafin sake loda; ana kiyaye plugin na yanzu
entity-reload-failed = Entity ya gaza sake loda; ana cirewa har sai sake loda na gaba
entity-reload-kind-extends-failed = An kasa warware sarkar extends na kind yayin sake loda entity
entity-reload-kind-fetch-failed = An kasa dawo da node na kind yayin sake loda entity
entity-reload-kind-lookup-failed = An kasa loda manifest don nemo kind yayin sake loda entity
entity-reload-kind-missing = Ba a sami kind a manifest ba; ba za a iya sake loda entity ba
entity-reload-manifest-state-update-failed = An kasa sabunta manifest da jihar yanzu kafin sake loda; ana kiyaye plugin na yanzu
entity-reload-skipped = An tsallake sake loda entity saboda ƙofar sake loda ta rufe
entity-reload-started = An fara sake loda entity
entity-reload-state-persist-failed = An kasa adana jihar yanzu kafin sake loda; ana kiyaye plugin na yanzu
entity-reload-state-produced-failed = An kasa adana jihar da aka samar a lokacin sake loda
entity-reloaded-manifest-update-failed = An kasa sabunta entity da aka sake loda a manifest
entity-reloaded-manifest-updated = An sabunta entity da aka sake loda a manifest
inbox-message-rejected = An ƙi sakon inbox
ma-create-entity-already-exists = ma_create_entity: entity ya riga ya wanzu; ana kiyaye entity na yanzu
ma-create-entity-invalid-behaviour = ma_create_entity: reference na behaviour mara inganci; an tsallake
ma-create-entity-kind-missing = ma_create_entity: kind ba a registry ba; an tsallake
manifest-pin-update-failed = Manifest pin_update ya gaza
plugin-envelope-build-failed = Envelope na plugin: an kasa gina sakon; an tsallake
plugin-envelope-create-requests-ignored = Envelope na plugin: an yi watsi da buƙatun ƙirƙira ba tare da mahallin side-effect ba
plugin-envelope-local-dispatch-failed = Envelope na plugin: isar da gida ya gaza
plugin-envelope-local-dispatch-finish = Envelope na plugin: isar da gida ta ƙare
plugin-envelope-local-dispatch-start = Envelope na plugin: isar da gida ta fara
plugin-envelope-local-gate-closed = Envelope na plugin: ƙofar isar da gida ta rufe
plugin-envelope-local-recipient-unknown = Envelope na plugin: mai karɓa na gida da ba a sani ba; an tsallake
plugin-envelope-local-timeout = Envelope na plugin: isar da gida ya ƙare
plugin-envelope-recipient-invalid = Envelope na plugin: DID mai karɓa mara inganci; an tsallake
plugin-envelope-remote-limit = Envelope na plugin: an kai iyakar isar nesa; an jefar da envelope
plugin-outbox-congested = Outbox na plugin yana cika; za a iya jefar da envelope idan tashoshi suka cika
plugin-outbox-drain-limit = Kasafin drain na outbox na plugin ya ƙare; ana jinkirta envelope da suka rage
schedule-dispatch-firing = Isar da jadawalin tana harbi
schedule-entity-not-found = Isar da jadawali: ba a sami entity ba
schedule-random-chain-stopped = Sarkar jadawali na bazuwar ta tsaya: wani ma'ana ya maye gurbin shi
schedule-random-create-failed = An kasa ƙirƙira aikin bazuwar na gaba
schedule-random-reschedule-failed = An kasa sake jadawali aikin bazuwar
schedule-stale-dispatch-skipped = An tsallake isar da jadawali: jadawali tsoho
scheduled-dispatch-error = Kuskuren isar da jadawali
scheduled-dispatch-manifest-writer-unavailable = Isar da jadawali: marubucin manifest ba ya shirye; jihar entity tana jiran
