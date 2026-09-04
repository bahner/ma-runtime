# ma-runtime – Latviešu
lang-name = Latviešu

own-did-published = Savs DID dokuments publicēts IPNS
own-did-publish-failed = Neizdevās publicēt savu DID dokumentu
own-did-publish-timeout = Sava DID dokumenta publicēšana pārsniedza 2 min. taimauta
started = ma runtime palaists
shutdown-requested = Izslēgšana pieprasīta
closing-endpoint = Aizver iroh galapunktu...
shutdown-complete = Izslēgšana pabeigta
status-listening = Statusa serveris klausās
ipfs-message-rejected = IPFS ziņojums noraidīts
ctrlc-handler-failed = Ctrl-C apstrādātājs neizdevās
node-connected = Mezgls pievienojies protokolam
received-encrypted-ma-msg = Saņemts šifrēts ma ziņojums uz /ma/ipfs/0.0.1
ping-received = Saņemts :ping, sūtu :pong
did-publish-request-received = Saņemts DID dokumenta publicēšanas pieprasījums
document-published = Dokuments publicēts
did-publish-cid-reply-sent = CID atbilde nosūtīta DID publicēšanai
did-publish-resolve-failed = Neizdevās atrisināt sūtītāju ipfs-publish atbildes piegādei
ipfs-store-request-received = Saņemts IPFS glabāšanas pieprasījums
ipfs-stored = Saturs saglabāts IPFS
ipfs-store-cid-reply-sent = CID atbilde nosūtīta
ipfs-store-resolve-failed = Neizdevās atrisināt sūtītāju ipfs-store atbildes piegādei

# Entitāšu nosūtīšana
bootstrap-complete = Bootstrap pabeigts
entity-loaded = Entitātes spraudnis ielādēts
entity-load-failed = Entitātes spraudņa ielāde neizdevās
root-list-entities = #root: entitāšu saraksts
entity-created = Entitāte izveidota
entity-reloaded = Entitātes spraudnis pārlādēts
entity-deleted = Entitāte dzēsta
entity-states-saving = Saglabā entitāšu stāvokļus IPFS
entity-state-saving = Saglabā entitātes stāvokli
entity-state-saved = Entitātes stāvoklis saglabāts
entity-state-empty = Spraudnis atgrieza tukšu stāvokli, saglabāšana izlaista
entity-states-saved = Entitāšu stāvokļi saglabāti

# Pirmā palaišana / auto-init

# Īpašumtiesības
runtime-claimed = Runtime reģistrēts.

# Aizsargātie saknes elementi
refuse-delete-root = Kategoriski atsakos dzēst nepieciešamo saknes elementu
runtime-claim-persisted = Īpašnieks ierakstīts konfigurācijā.


# Namespace creation (:create)
crud-message-received = Saņemts CRUD ziņojums
crud-acl-updated = Saknes transporta ACL atjaunināts

# CRUD validation errors
cidv1-required = vērtībai jābūt neapstrādātam CIDv1 (sākas ar 'b'; CIDv0 'Qm…' netiek pieņemts)
config-key-protected = konfigurācijas atslēga '%key%' ir aizsargāta
config-key-no-delete = daemon konfigurācijas atslēgu '%key%' nevar dzēst
config-key-not-manifest = konfigurācijas atslēga '%key%' nav zināma manifest config atslēga
wrong-crud-protocol = nepareizs CRUD protokols: %type%
entity-name-invalid = entity nosaukumam jābūt drukājamam UTF-8
reserved-entity-name = entity nosaukums '%name%' ir rezervēts
genesis-kind-owner-only = Tikai runtime īpašnieks var izveidot genesis tipa entity

# IPv6 config
ipv6-enabled = IPv6 iespējots — saista gan IPv4, gan IPv6
ipv6-disabled = IPv6 ir atspējots — tiek piesaistīts tikai IPv4 (atkārtotai iespējošanai nepieciešams restart)
ipv6-enable-restart-required = Saglabāts. Lai šīs izmaiņas stātos spēkā, nepieciešams restart.
ipv6-enable-unchanged = ipv6_enable jau ir iestatīts uz šo vērtību — nav izmaiņu.

boot-default-root-config-populate-failed = Noklusējuma saknes konfigurācijas aizpildīšana neizdevās
boot-default-root-config-populated = Noklusējuma saknes konfigurācija aizpildīta
boot-entity-load-processed = Entitāšu spraudņi ielādēti
boot-group-load-failed = Grupas ielāde startēšanas laikā neizdevās
boot-group-loaded-into-cache = Grupa ielādēta kešatmiņā
boot-kinds-overlay-applied = Kinds pārklājums pielietots
boot-kinds-overlay-no-change = Kinds pārklājums manifestu nemainīja
boot-load-manifest-for-acl-cache-failed = Manifesta ielāde ACL kešatmiņas aizpildīšanai neizdevās
boot-minimal-manifest-bootstrapped = Minimālais manifests inicializēts
boot-minimal-manifest-not-found = Izpildvides saknes CID nav atrasts IPNS; tiek inicializēts minimālais manifests
boot-no-root-entity = Nav reģistrētas saknes entitātes noklusējuma saknes konfigurācijai
boot-reconciled-owners-manifest-failed = Īpašnieku saskaņošana manifestā startēšanas laikā neizdevās
boot-reconciled-owners-persist-failed = Saskaņoto īpašnieku saglabāšana config.yaml neizdevās
boot-reconciled-owners-published = Īpašnieki saskaņoti no config.yaml/--owner uz manifestu startēšanas laikā
boot-root-acl-load-cache-failed = Saknes ACL ielāde startēšanas laikā neizdevās
boot-root-acl-load-failed = Saknes ACL ielāde no manifesta neizdevās
boot-root-acl-loaded-from-manifest = Saknes transporta ACL ielādēts no manifesta
boot-root-acl-loaded-into-cache = Saknes ACL ielādēts kešatmiņā
bootstrap-acl-published = ACL mezgls publicēts
bootstrap-endpoint-close-stuck = Galapunkts joprojām tiek turēts notiekošo uzdevumu dēļ pēc 10 s; piespiedu aizvēršana
bootstrap-endpoint-close-timeout = Galapunkta aizvēršana iestājās 5 s laikā; piespiedu pārtraukšana
bootstrap-entity-lifecycle-update-failed = Atjaunotā entitātes dzīves cikla rakstīšana IPFS neizdevās
bootstrap-entity-lifecycle-updated = Entitātes dzīves cikls atjaunināts IPFS
bootstrap-entity-node-shutdown-updated = Entitātes mezgls atjaunināts izslēgšanas laikā
bootstrap-entity-published = Entitātes mezgls publicēts
bootstrap-entity-registering-prepublished = Iepriekš publicētas entitātes reģistrēšana
bootstrap-entity-registry-fetch-failed = Entitātes mezgla iegūšana neizdevās
bootstrap-entity-registry-kind-extends-failed = Kind paplašinājumu ķēdes atrisināšana neizdevās
bootstrap-entity-registry-kind-fetch-failed = Kind mezgla iegūšana neizdevās
bootstrap-entity-registry-kind-missing = Kind nav atrasts manifestā; entitāte izlaista
bootstrap-entity-registry-not-in-manifest = Entitāte reģistrā, bet ne manifestā; izlaista
bootstrap-entity-state-save-failed = Entitāšu stāvokļu saglabāšana neizdevās
bootstrap-entity-state-shutdown-aborted = Izslēgšana atcelta; izpildvide paliek aktīva, lai saglabātu stāvokli nākamajā izslēgšanā
bootstrap-entity-state-update-fetch-failed = Entitātes mezgla iegūšana stāvokļa atjaunināšanai neizdevās
bootstrap-envelope-delivery-failed = Spraudņa aploksnes piegāde neizdevās; aploksne izmesta
bootstrap-envelope-open-failed = Spraudņa aploksne: izejošās pastkastes atvēršana neizdevās; aploksne izmesta
bootstrap-group-published = Grupas mezgls publicēts
bootstrap-kind-published = Kind mezgls publicēts
bootstrap-kind-registry-extends-failed = Kind paplašinājumu ķēdes atrisināšana reģistram neizdevās
bootstrap-kind-registry-fetch-log-failed = Kind mezgla iegūšana reģistram neizdevās
bootstrap-kind-registry-hydrated = Kinds reģistrs aizpildīts no manifesta
bootstrap-kinds-overlay-pin-update-failed = Piespraušana/atjaunināšana neizdevās pēc kinds pārklājuma
bootstrap-kinds-overlay-published = Izpildvides manifests publicēts pēc kinds pārklājuma
bootstrap-kinds-tree-published = Izpildvides kinds koks publicēts
bootstrap-lifecycle-manifest-pin-update-failed = Piespraušana/atjaunināšana neizdevās pēc dzīves cikla saglabāšanas
bootstrap-lifecycle-manifest-publish-failed = Manifesta publicēšana pēc dzīves cikla pārejām neizdevās
bootstrap-lifecycle-manifest-published = Atjauninātais manifests publicēts pēc dzīves cikla pārejām
bootstrap-manifest-fetch-failed = Izpildvides manifesta iegūšana neizdevās
bootstrap-minimal-manifest-failed = Minimālā manifesta inicializēšana neizdevās
bootstrap-remote-root-pin-confirmed = Attālā saknes piespraušana apstiprināta
bootstrap-remote-root-pin-misconfigured = Attālā saknes piespraušana ir nepareizi konfigurēta
bootstrap-root-acl-published = Saknes transporta ACL publicēts
bootstrap-root-cid-shutdown-persist-failed = root_cid saglabāšana izslēgšanas laikā neizdevās
bootstrap-root-cid-shutdown-publish-failed = runtime_ipns publicēšana izslēgšanas laikā neizdevās
bootstrap-root-cid-shutdown-publish-succeeded = runtime_ipns publicēšana izslēgšanas laikā izdevās
bootstrap-root-cid-shutdown-publish-timeout = runtime_ipns publicēšana izslēgšanas laikā iestājās
bootstrap-root-pin-replacement-failed = Turpinās pēc attālās saknes piespraušanas nomaiņas kļūdas
bootstrap-root-pin-update-failed = Piespraušana/atjaunināšana neizdevās pēc sāknēšanas
bootstrap-runtime-manifest-published = Izpildvides saknes manifests publicēts
crud-message-rejected = CRUD ziņojums noraidīts
entity-reload-current-node-load-failed = Pašreizējā entitātes mezgla iegūšana pirms atkārtotas ielādes neizdevās; pašreizējais spraudnis saglabāts
entity-reload-failed = Entitātes atkārtota ielāde neizdevās; atspējots līdz nākamajai atkārtotajai ielādei
entity-reload-kind-extends-failed = Kind paplašinājumu ķēdes atrisināšana entitātes atkārtotas ielādes laikā neizdevās
entity-reload-kind-fetch-failed = Kind mezgla iegūšana entitātes atkārtotas ielādes laikā neizdevās
entity-reload-kind-lookup-failed = Manifesta iegūšana kind meklēšanai entitātes atkārtotas ielādes laikā neizdevās
entity-reload-kind-missing = Kind nav atrasts manifestā; entitāti nevar atkārtoti ielādēt
entity-reload-manifest-state-update-failed = Manifesta atjaunināšana ar pašreizējo stāvokli pirms atkārtotas ielādes neizdevās; pašreizējais spraudnis saglabāts
entity-reload-skipped = Entitātes atkārtota ielāde izlaista, jo atkārtotas ielādes vārti ir slēgti
entity-reload-started = Entitātes atkārtota ielāde sākta
entity-reload-state-persist-failed = Pašreizējā stāvokļa saglabāšana pirms atkārtotas ielādes neizdevās; pašreizējais spraudnis saglabāts
entity-reload-state-produced-failed = Atkārtotas ielādes laikā radītā stāvokļa saglabāšana neizdevās
entity-reloaded-manifest-update-failed = Atkārtoti ielādētas entitātes atjaunināšana manifestā neizdevās
entity-reloaded-manifest-updated = Atkārtoti ielādēta entitāte atjaunināta manifestā
inbox-message-rejected = Ienākošās pastkastes ziņojums noraidīts
ma-create-entity-already-exists = ma_create_entity: entitāte jau pastāv; pašreizējā entitāte saglabāta
ma-create-entity-invalid-behaviour = ma_create_entity: nederīga uzvedības atsauce; izlaists
ma-create-entity-kind-missing = ma_create_entity: kind nav reģistrā; izlaists
manifest-pin-update-failed = Manifesta pin_update neizdevās
plugin-envelope-build-failed = Spraudņa aploksne: ziņojuma veidošana neizdevās; izlaists
plugin-envelope-create-requests-ignored = Spraudņa aploksne: izveides pieprasījumi ignorēti bez blakusefektu konteksta
plugin-envelope-local-dispatch-failed = Spraudņa aploksne: lokālā nosūtīšana neizdevās
plugin-envelope-local-dispatch-finish = Spraudņa aploksne: lokālā nosūtīšana pabeigta
plugin-envelope-local-dispatch-start = Spraudņa aploksne: lokālā nosūtīšana sākta
plugin-envelope-local-gate-closed = Spraudņa aploksne: lokālās nosūtīšanas vārti ir slēgti
plugin-envelope-local-recipient-unknown = Spraudņa aploksne: nezināms lokālais saņēmējs; izlaists
plugin-envelope-local-timeout = Spraudņa aploksne: lokālā nosūtīšana iestājās
plugin-envelope-recipient-invalid = Spraudņa aploksne: nederīgs saņēmēja DID; izlaists
plugin-envelope-remote-limit = Spraudņa aploksne: attālinātās piegādes limits sasniegts; aploksne izmesta
plugin-outbox-congested = Spraudņa izejošā pastkaste ir pārslogota; aploksnes var tikt izmestas, ja kanāls piepildīsies
plugin-outbox-drain-limit = Spraudņa izejošās pastkastes iztukšošanas budžets izsmelts; atlikušās aploksnes atliktas
schedule-dispatch-firing = Plānotā nosūtīšana notiek
schedule-entity-not-found = Plānotā nosūtīšana: entitāte nav atrasta
schedule-random-chain-stopped = Nejaušā plānotā ķēde apturēta: aizstāta ar jaunāku definīciju
schedule-random-create-failed = Nākamā nejaušā uzdevuma izveide neizdevās
schedule-random-reschedule-failed = Nejaušā uzdevuma pārplānošana neizdevās
schedule-stale-dispatch-skipped = Plānotā nosūtīšana izlaista: novecojis grafiks
scheduled-dispatch-error = Kļūda plānotās nosūtīšanas laikā
scheduled-dispatch-manifest-writer-unavailable = Plānotā nosūtīšana: manifesta rakstītājs nav gatavs; entitātes stāvoklis gaida
