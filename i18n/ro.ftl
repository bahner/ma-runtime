# ma-runtime – Română
lang-name = Română

own-did-published = Documentul DID propriu publicat pe IPNS
own-did-publish-failed = Publicarea documentului DID propriu a eșuat
own-did-publish-timeout = Publicarea documentului DID propriu a expirat după 2 minute
started = ma runtime pornit
shutdown-requested = Oprire solicitată
closing-endpoint = Închiderea endpoint-ului iroh...
shutdown-complete = Oprire finalizată
status-listening = Serverul de stare ascultă
ipfs-message-rejected = Mesaj IPFS respins
ctrlc-handler-failed = Handler-ul Ctrl-C a eșuat
node-connected = Nod conectat la protocol
received-encrypted-ma-msg = Mesaj ma criptat primit pe /ma/ipfs/0.0.1
ping-received = :ping primit, trimit :pong
did-publish-request-received = Cerere de publicare document DID primită
document-published = Document publicat
did-publish-cid-reply-sent = Răspuns CID trimis pentru publicarea DID
did-publish-resolve-failed = Nu s-a putut rezolva expeditorul pentru livrarea răspunsului ipfs-publish
ipfs-store-request-received = Cerere de stocare IPFS primită
ipfs-stored = Conținut stocat pe IPFS
ipfs-store-cid-reply-sent = Răspuns CID trimis
ipfs-store-resolve-failed = Nu s-a putut rezolva expeditorul pentru livrarea răspunsului ipfs-store

# Dispecerizarea entităților
bootstrap-complete = Bootstrap finalizat
entity-loaded = Plugin entitate încărcat
entity-load-failed = Încărcarea plugin-ului entitate a eșuat
root-list-entities = #root: listează entitățile
entity-created = Entitate creată
entity-reloaded = Plugin entitate reîncărcat
entity-deleted = Entitate ștearsă
entity-states-saving = Salvare stări entități în IPFS
entity-state-saving = Salvare stare entitate
entity-state-saved = Stare entitate salvată
entity-state-empty = Plugin-ul a returnat stare goală, salvarea omisă
entity-states-saved = Stări entități salvate

# Prima rulare / auto-init

# Proprietate
runtime-claimed = Runtime înregistrat.

# Elemente rădăcină protejate
refuse-delete-root = Refuz categoric să șterg un element rădăcină obligatoriu
runtime-claim-persisted = Proprietar scris în configurație.


# Namespace creation (:create)
crud-message-received = Mesaj CRUD primit
crud-acl-updated = ACL de transport rădăcină actualizat

# CRUD validation errors
cidv1-required = valoarea trebuie să fie un CIDv1 pur (începe cu 'b'; CIDv0 'Qm…' nu este acceptat)
config-key-protected = cheia de config '%key%' este protejată
config-key-no-delete = cheia de config '%key%' a daemon-ului nu poate fi ștearsă
config-key-not-manifest = cheia de config '%key%' nu este o cheie de manifest config cunoscută
wrong-crud-protocol = protocol CRUD greșit: %type%
entity-name-invalid = numele entității trebuie să fie UTF-8 imprimabil
reserved-entity-name = numele entității '%name%' este rezervat
genesis-kind-owner-only = Doar un proprietar runtime poate crea un entity de tip genesis

# IPv6 config
ipv6-enabled = IPv6 activat — leagă atât IPv4, cât și IPv6
ipv6-disabled = IPv6 dezactivat — se leagă doar IPv4 (este necesară repornirea pentru a reactiva)
ipv6-enable-restart-required = Salvat. Este necesară repornirea pentru ca această modificare să intre în vigoare.
ipv6-enable-unchanged = ipv6_enable este deja setat la acea valoare — fără modificări.

boot-default-root-config-populate-failed = Popularea configurației rădăcină implicite a eșuat
boot-default-root-config-populated = Configurația rădăcină implicită populată
boot-entity-load-processed = Plugin-urile entităților încărcate
boot-group-load-failed = Încărcarea grupului la pornire a eșuat
boot-group-loaded-into-cache = Grup încărcat în cache
boot-kinds-overlay-applied = Suprapunerea kinds aplicată
boot-kinds-overlay-no-change = Suprapunerea kinds nu a modificat manifestul
boot-load-manifest-for-acl-cache-failed = Încărcarea manifestului pentru popularea cache-ului ACL a eșuat
boot-minimal-manifest-bootstrapped = Manifest minimal inițializat
boot-minimal-manifest-not-found = Nu s-a găsit CID rădăcină runtime în IPNS; se inițializează manifestul minimal
boot-no-root-entity = Nicio entitate rădăcină înregistrată pentru configurația rădăcină implicită
boot-reconciled-owners-manifest-failed = Reconcilierea proprietarilor în manifest la pornire a eșuat
boot-reconciled-owners-persist-failed = Salvarea proprietarilor reconciliați în config.yaml a eșuat
boot-reconciled-owners-published = Proprietari reconciliați din config.yaml/--owner în manifest la pornire
boot-root-acl-load-cache-failed = Încărcarea ACL rădăcină la pornire a eșuat
boot-root-acl-load-failed = Încărcarea ACL rădăcină din manifest a eșuat
boot-root-acl-loaded-from-manifest = ACL-ul de transport rădăcină încărcat din manifest
boot-root-acl-loaded-into-cache = ACL rădăcină încărcat în cache
bootstrap-acl-published = Nod ACL publicat
bootstrap-endpoint-close-stuck = Endpoint-ul este reținut de sarcini în desfășurare după 10 s; închidere forțată
bootstrap-endpoint-close-timeout = Închiderea endpoint-ului a expirat după 5 s; terminare forțată
bootstrap-entity-lifecycle-update-failed = Scrierea ciclului de viață actualizat al entității în IPFS a eșuat
bootstrap-entity-lifecycle-updated = Ciclul de viață al entității actualizat în IPFS
bootstrap-entity-node-shutdown-updated = Nod entitate actualizat la oprire
bootstrap-entity-published = Nod entitate publicat
bootstrap-entity-registering-prepublished = Înregistrare entitate pre-publicată
bootstrap-entity-registry-fetch-failed = Obținerea nodului entitate a eșuat
bootstrap-entity-registry-kind-extends-failed = Rezolvarea lanțului de extensii kind a eșuat
bootstrap-entity-registry-kind-fetch-failed = Obținerea nodului kind a eșuat
bootstrap-entity-registry-kind-missing = Kind negăsit în manifest; entitate omisă
bootstrap-entity-registry-not-in-manifest = Entitate în registru dar nu în manifest, omisă
bootstrap-entity-state-save-failed = Salvarea stărilor entităților a eșuat
bootstrap-entity-state-shutdown-aborted = Oprire anulată; runtime rămâne activ pentru a salva starea la următoarea oprire
bootstrap-entity-state-update-fetch-failed = Obținerea nodului entitate pentru actualizarea stării a eșuat
bootstrap-envelope-delivery-failed = Livrarea plicului plugin-ului a eșuat; plic eliminat
bootstrap-envelope-open-failed = Plic plugin: deschiderea cutiei poștale de ieșire a eșuat; plic eliminat
bootstrap-group-published = Nod grup publicat
bootstrap-kind-published = Nod kind publicat
bootstrap-kind-registry-extends-failed = Rezolvarea lanțului de extensii kind pentru registru a eșuat
bootstrap-kind-registry-fetch-log-failed = Obținerea nodului kind pentru registru a eșuat
bootstrap-kind-registry-hydrated = Registrul kinds completat din manifest
bootstrap-kinds-overlay-pin-update-failed = Pin/actualizare eșuat după suprapunerea kinds
bootstrap-kinds-overlay-published = Manifest runtime publicat după suprapunerea kinds
bootstrap-kinds-tree-published = Arborele kinds runtime publicat
bootstrap-lifecycle-manifest-pin-update-failed = Pin/actualizare eșuat după persistarea ciclului de viață
bootstrap-lifecycle-manifest-publish-failed = Publicarea manifestului după tranzițiile ciclului de viață a eșuat
bootstrap-lifecycle-manifest-published = Manifest actualizat publicat după tranzițiile ciclului de viață
bootstrap-manifest-fetch-failed = Obținerea manifestului runtime a eșuat
bootstrap-minimal-manifest-failed = Inițializarea manifestului minimal a eșuat
bootstrap-remote-root-pin-confirmed = Pinning rădăcină la distanță confirmat
bootstrap-remote-root-pin-misconfigured = Pinning-ul rădăcină la distanță este configurat incorect
bootstrap-root-acl-published = ACL-ul de transport rădăcină publicat
bootstrap-root-cid-shutdown-persist-failed = Persistarea root_cid la oprire a eșuat
bootstrap-root-cid-shutdown-publish-failed = Publicarea runtime_ipns la oprire a eșuat
bootstrap-root-cid-shutdown-publish-succeeded = Publicarea runtime_ipns la oprire a reușit
bootstrap-root-cid-shutdown-publish-timeout = Publicarea runtime_ipns la oprire a expirat
bootstrap-root-pin-replacement-failed = Continuare după eroarea înlocuirii pin-ului rădăcină la distanță
bootstrap-root-pin-update-failed = Pin/actualizare eșuat după bootstrap
bootstrap-runtime-manifest-published = Manifest rădăcină runtime publicat
crud-message-rejected = Mesaj CRUD respins
entity-reload-current-node-load-failed = Obținerea nodului curent al entității înainte de reîncărcare a eșuat; plugin-ul curent păstrat
entity-reload-failed = Reîncărcarea entității a eșuat; dezactivată până la următoarea reîncărcare
entity-reload-kind-extends-failed = Rezolvarea lanțului de extensii kind la reîncărcarea entității a eșuat
entity-reload-kind-fetch-failed = Obținerea nodului kind la reîncărcarea entității a eșuat
entity-reload-kind-lookup-failed = Obținerea manifestului pentru căutarea kind la reîncărcarea entității a eșuat
entity-reload-kind-missing = Kind negăsit în manifest; entitatea nu poate fi reîncărcată
entity-reload-manifest-state-update-failed = Actualizarea manifestului cu starea curentă înainte de reîncărcare a eșuat; plugin-ul curent păstrat
entity-reload-skipped = Reîncărcarea entității omisă deoarece poarta de reîncărcare este închisă
entity-reload-started = Reîncărcarea entității a început
entity-reload-state-persist-failed = Persistarea stării curente înainte de reîncărcare a eșuat; plugin-ul curent păstrat
entity-reload-state-produced-failed = Persistarea stării produse la reîncărcare a eșuat
entity-reloaded-manifest-update-failed = Actualizarea entității reîncărcate în manifest a eșuat
entity-reloaded-manifest-updated = Entitate reîncărcată actualizată în manifest
inbox-message-rejected = Mesaj inbox respins
ma-create-entity-already-exists = ma_create_entity: entitatea există deja; entitatea curentă păstrată
ma-create-entity-invalid-behaviour = ma_create_entity: referință behaviour invalidă; omis
ma-create-entity-kind-missing = ma_create_entity: kind lipsă din registru; omis
manifest-pin-update-failed = pin_update manifest eșuat
plugin-envelope-build-failed = Plic plugin: construirea mesajului a eșuat; omis
plugin-envelope-create-requests-ignored = Plic plugin: cererile de creare ignorate fără context de efect secundar
plugin-envelope-local-dispatch-failed = Plic plugin: expedierea locală a eșuat
plugin-envelope-local-dispatch-finish = Plic plugin: expediere locală finalizată
plugin-envelope-local-dispatch-start = Plic plugin: expediere locală pornită
plugin-envelope-local-gate-closed = Plic plugin: poarta de expediere locală este închisă
plugin-envelope-local-recipient-unknown = Plic plugin: destinatar local necunoscut; omis
plugin-envelope-local-timeout = Plic plugin: expedierea locală a expirat
plugin-envelope-recipient-invalid = Plic plugin: DID destinatar invalid; omis
plugin-envelope-remote-limit = Plic plugin: limita livrării la distanță atinsă; plic eliminat
plugin-outbox-congested = Outbox plugin congestionat; plicurile pot fi eliminate dacă canalul se umple
plugin-outbox-drain-limit = Bugetul de golire al outbox-ului plugin epuizat; plicurile rămase amânate
schedule-dispatch-firing = Expediere programată în curs
schedule-entity-not-found = Expediere programată: entitate negăsită
schedule-random-chain-stopped = Lanț aleatoriu programat oprit: înlocuit de o definiție mai nouă
schedule-random-create-failed = Crearea următoarei sarcini aleatorii a eșuat
schedule-random-reschedule-failed = Reprogramarea sarcinii aleatorii a eșuat
schedule-stale-dispatch-skipped = Expediere programată omisă: program depășit
scheduled-dispatch-error = Eroare la expedierea programată
scheduled-dispatch-manifest-writer-unavailable = Expediere programată: scriitorul manifestului nu este pregătit; starea entității în așteptare
