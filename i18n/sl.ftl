# ma-runtime – Slovenščina
lang-name = Slovenščina

own-did-published = Lastni DID dokument objavljen na IPNS
own-did-publish-failed = Objava lastnega DID dokumenta ni uspela
own-did-publish-timeout = Objava lastnega DID dokumenta je potekla po 2 minutah
started = ma runtime zagnan
shutdown-requested = Zahtevano zaustavitev
closing-endpoint = Zapiranje iroh končne točke...
shutdown-complete = Zaustavitev dokončana
status-listening = Strežnik statusa posluša
ipfs-message-rejected = IPFS sporočilo zavrnjeno
ctrlc-handler-failed = Upravljalnik Ctrl-C je odpovedal
node-connected = Vozlišče povezano s protokolom
received-encrypted-ma-msg = Prejeto šifrirano ma sporočilo na /ma/ipfs/0.0.1
ping-received = Prejet :ping, pošiljam :pong
did-publish-request-received = Prejeta zahteva za objavo DID dokumenta
document-published = Dokument objavljen
did-publish-cid-reply-sent = Poslan CID odgovor za objavo DID
did-publish-resolve-failed = Ni mogoče razrešiti pošiljatelja za dostavo odgovora ipfs-publish
ipfs-store-request-received = Prejeta zahteva za shranjevanje IPFS
ipfs-stored = Vsebina shranjena na IPFS
ipfs-store-cid-reply-sent = CID odgovor poslan
ipfs-store-resolve-failed = Ni mogoče razrešiti pošiljatelja za dostavo odgovora ipfs-store

# Razpošiljanje entitet
bootstrap-complete = Bootstrap dokončan
entity-loaded = Vtičnik entitete naložen
entity-load-failed = Nalaganje vtičnika entitete ni uspelo
root-list-entities = #root: seznam entitet
entity-created = Entiteta ustvarjena
entity-reloaded = Vtičnik entitete ponovno naložen
entity-deleted = Entiteta izbrisana
entity-states-saving = Shranjevanje stanj entitet v IPFS
entity-state-saving = Shranjevanje stanja entitete
entity-state-saved = Stanje entitete shranjeno
entity-state-empty = Vtičnik je vrnil prazno stanje, shranjevanje preskočeno
entity-states-saved = Stanja entitet shranjena

# Prvi zagon / auto-init

# Lastništvo
runtime-claimed = Runtime registriran.

# Zaščiteni korenski elementi
refuse-delete-root = Odločno zavračam brisanje zahtevanega korenskega elementa
runtime-claim-persisted = Lastnik zapisan v konfiguracijo.


# Namespace creation (:create)
crud-message-received = Prejeto CRUD sporočilo
crud-acl-updated = Korenski transportni ACL posodobljen

# CRUD validation errors
cidv1-required = vrednost mora biti goli CIDv1 (začne se z 'b'; CIDv0 'Qm…' ni sprejeto)
config-key-protected = konfiguracijski ključ '%key%' je zaščiten
config-key-no-delete = konfiguracijski ključ '%key%' demona ni mogoče izbrisati
config-key-not-manifest = konfiguracijski ključ '%key%' ni znan ključ manifest config
wrong-crud-protocol = napačen protokol CRUD: %type%
entity-name-invalid = ime entity mora biti tiskljivi UTF-8
reserved-entity-name = ime entity '%name%' je rezervirano
genesis-kind-owner-only = Samo lastnik runtime lahko ustvari entity vrste genesis

# IPv6 config
ipv6-enabled = IPv6 omogočeno — vezano na IPv4 in IPv6
ipv6-disabled = IPv6 je onemogočen — poveže se samo IPv4 (za ponovno omogočitev je potreben ponovni zagon)
ipv6-enable-restart-required = Shranjeno. Za uveljavitev te spremembe je potreben ponovni zagon.
ipv6-enable-unchanged = ipv6_enable je že nastavljeno na to vrednost — brez sprememb.

boot-default-root-config-populate-failed = Zapolnitev privzete korenske konfiguracije ni uspela
boot-default-root-config-populated = Privzeta koreninska konfiguracija zapolnjena
boot-entity-load-processed = Vtičniki entitet naloženi
boot-group-load-failed = Nalaganje skupine pri zagonu ni uspelo
boot-group-loaded-into-cache = Skupina naložena v predpomnilnik
boot-kinds-overlay-applied = Prekrivanje kinds uporabljeno
boot-kinds-overlay-no-change = Prekrivanje kinds ni spremenilo manifesta
boot-load-manifest-for-acl-cache-failed = Nalaganje manifesta za zapolnitev predpomnilnika ACL ni uspelo
boot-minimal-manifest-bootstrapped = Minimalni manifest inicializiran
boot-minimal-manifest-not-found = Korenski CID okolja izvajanja ni bil najden v IPNS; inicializacija minimalnega manifesta
boot-no-root-entity = Nobena koreninska entiteta ni registrirana za privzeto korensko konfiguracijo
boot-reconciled-owners-manifest-failed = Usklajevanje lastnikov v manifestu pri zagonu ni uspelo
boot-reconciled-owners-persist-failed = Shranjevanje usklajenih lastnikov v config.yaml ni uspelo
boot-reconciled-owners-published = Lastniki usklajeni iz config.yaml/--owner v manifest pri zagonu
boot-root-acl-load-cache-failed = Nalaganje korenskega ACL pri zagonu ni uspelo
boot-root-acl-load-failed = Nalaganje korenskega ACL iz manifesta ni uspelo
boot-root-acl-loaded-from-manifest = Korenski transportni ACL naložen iz manifesta
boot-root-acl-loaded-into-cache = Korenski ACL naložen v predpomnilnik
bootstrap-acl-published = Vozlišče ACL objavljeno
bootstrap-endpoint-close-stuck = Endpoint je zadržan z zagotavljanjem tekočih nalog po 10 s; prisilno zapiranje
bootstrap-endpoint-close-timeout = Zapiranje endpointa je poteklo po 5 s; prisilna prekinitev
bootstrap-entity-lifecycle-update-failed = Pisanje posodobljenega življenjskega cikla entitete v IPFS ni uspelo
bootstrap-entity-lifecycle-updated = Življenjski cikel entitete posodobljen v IPFS
bootstrap-entity-node-shutdown-updated = Vozlišče entitete posodobljeno ob zaustavitvi
bootstrap-entity-published = Vozlišče entitete objavljeno
bootstrap-entity-registering-prepublished = Registracija vnaprej objavljene entitete
bootstrap-entity-registry-fetch-failed = Pridobivanje vozlišča entitete ni uspelo
bootstrap-entity-registry-kind-extends-failed = Razreševanje verige razširitev kind ni uspelo
bootstrap-entity-registry-kind-fetch-failed = Pridobivanje vozlišča kind ni uspelo
bootstrap-entity-registry-kind-missing = Kind ni bil najden v manifestu; entiteta preskočena
bootstrap-entity-registry-not-in-manifest = Entiteta v registru, a ne v manifestu; preskočena
bootstrap-entity-state-save-failed = Shranjevanje stanj entitet ni uspelo
bootstrap-entity-state-shutdown-aborted = Zaustavitev prekinjena; okolje ostane aktivno za shranjevanje stanja ob naslednji zaustavitvi
bootstrap-entity-state-update-fetch-failed = Pridobivanje vozlišča entitete za posodobitev stanja ni uspelo
bootstrap-envelope-delivery-failed = Dostava ovojnice vtičnika ni uspela; ovojnica zavržena
bootstrap-envelope-open-failed = Ovojnica vtičnika: odpiranje odhodnega nabiralnika ni uspelo; ovojnica zavržena
bootstrap-group-published = Vozlišče skupine objavljeno
bootstrap-kind-published = Vozlišče kind objavljeno
bootstrap-kind-registry-extends-failed = Razreševanje verige razširitev kind za register ni uspelo
bootstrap-kind-registry-fetch-log-failed = Pridobivanje vozlišča kind za register ni uspelo
bootstrap-kind-registry-hydrated = Register kinds zapolnjen iz manifesta
bootstrap-kinds-overlay-pin-update-failed = Pripenjanje/posodobitev ni uspelo po prekrivanju kinds
bootstrap-kinds-overlay-published = Manifest okolja objavljen po prekrivanju kinds
bootstrap-kinds-tree-published = Drevo kinds okolja objavljeno
bootstrap-lifecycle-manifest-pin-update-failed = Pripenjanje/posodobitev ni uspelo po shranjevanju življenjskega cikla
bootstrap-lifecycle-manifest-publish-failed = Objava manifesta po prehodih življenjskega cikla ni uspela
bootstrap-lifecycle-manifest-published = Posodobljeni manifest objavljen po prehodih življenjskega cikla
bootstrap-manifest-fetch-failed = Pridobivanje manifesta okolja ni uspelo
bootstrap-minimal-manifest-failed = Inicializacija minimalnega manifesta ni uspela
bootstrap-remote-root-pin-confirmed = Oddaljeno korensko pripenjanje potrjeno
bootstrap-remote-root-pin-misconfigured = Oddaljeno korensko pripenjanje je napačno konfigurirano
bootstrap-root-acl-published = Korenski transportni ACL objavljen
bootstrap-root-cid-shutdown-persist-failed = Shranjevanje root_cid ob zaustavitvi ni uspelo
bootstrap-root-cid-shutdown-publish-failed = Objava runtime_ipns ob zaustavitvi ni uspela
bootstrap-root-cid-shutdown-publish-succeeded = Objava runtime_ipns ob zaustavitvi je uspela
bootstrap-root-cid-shutdown-publish-timeout = Objava runtime_ipns ob zaustavitvi je potekla
bootstrap-root-pin-replacement-failed = Nadaljevanje po napaki zamenjave oddaljenega korenskega pripenjanja
bootstrap-root-pin-update-failed = Pripenjanje/posodobitev ni uspelo po zagonu
bootstrap-runtime-manifest-published = Korenski manifest okolja objavljen
crud-message-rejected = Sporočilo CRUD zavrnjeno
entity-reload-current-node-load-failed = Pridobivanje trenutnega vozlišča entitete pred ponovnim nalaganjem ni uspelo; trenutni vtičnik ohranjen
entity-reload-failed = Ponovno nalaganje entitete ni uspelo; onemogočena do naslednjega ponovnega nalaganja
entity-reload-kind-extends-failed = Razreševanje verige razširitev kind pri ponovnem nalaganju entitete ni uspelo
entity-reload-kind-fetch-failed = Pridobivanje vozlišča kind pri ponovnem nalaganju entitete ni uspelo
entity-reload-kind-lookup-failed = Pridobivanje manifesta za iskanje kind pri ponovnem nalaganju entitete ni uspelo
entity-reload-kind-missing = Kind ni bil najden v manifestu; entitete ni mogoče znova naložiti
entity-reload-manifest-state-update-failed = Posodobitev manifesta s trenutnim stanjem pred ponovnim nalaganjem ni uspela; trenutni vtičnik ohranjen
entity-reload-skipped = Ponovno nalaganje entitete preskočeno, ker je vrata ponovnega nalaganja zaprta
entity-reload-started = Ponovno nalaganje entitete se je začelo
entity-reload-state-persist-failed = Shranjevanje trenutnega stanja pred ponovnim nalaganjem ni uspelo; trenutni vtičnik ohranjen
entity-reload-state-produced-failed = Shranjevanje stanja, ki je nastalo pri ponovnem nalaganju, ni uspelo
entity-reloaded-manifest-update-failed = Posodobitev znova naložene entitete v manifestu ni uspela
entity-reloaded-manifest-updated = Znova naložena entiteta posodobljena v manifestu
inbox-message-rejected = Sporočilo nabiralnika zavrnjeno
ma-create-entity-already-exists = ma_create_entity: entiteta že obstaja; trenutna entiteta ohranjena
ma-create-entity-invalid-behaviour = ma_create_entity: neveljavna referenca na vedenje; preskočeno
ma-create-entity-kind-missing = ma_create_entity: kind manjka v registru; preskočeno
manifest-pin-update-failed = pin_update manifesta ni uspel
plugin-envelope-build-failed = Ovojnica vtičnika: gradnja sporočila ni uspela; preskočeno
plugin-envelope-create-requests-ignored = Ovojnica vtičnika: zahtevki za ustvarjanje prezrti brez konteksta stranskega učinka
plugin-envelope-local-dispatch-failed = Ovojnica vtičnika: lokalno pošiljanje ni uspelo
plugin-envelope-local-dispatch-finish = Ovojnica vtičnika: lokalno pošiljanje zaključeno
plugin-envelope-local-dispatch-start = Ovojnica vtičnika: lokalno pošiljanje se je začelo
plugin-envelope-local-gate-closed = Ovojnica vtičnika: vrata lokalnega pošiljanja so zaprta
plugin-envelope-local-recipient-unknown = Ovojnica vtičnika: neznan lokalni prejemnik; preskočeno
plugin-envelope-local-timeout = Ovojnica vtičnika: lokalno pošiljanje je poteklo
plugin-envelope-recipient-invalid = Ovojnica vtičnika: neveljaven DID prejemnika; preskočeno
plugin-envelope-remote-limit = Ovojnica vtičnika: dosežena omejitev oddaljenega dostavljanja; ovojnica zavržena
plugin-outbox-congested = Odhodni nabiralnik vtičnika zasičen; ovojnice so lahko zavržene, če se kanal zapolni
plugin-outbox-drain-limit = Proračun praznjenja odhodnega nabiralnika vtičnika izčrpan; preostale ovojnice odložene
schedule-dispatch-firing = Načrtovano pošiljanje v teku
schedule-entity-not-found = Načrtovano pošiljanje: entiteta ni bila najdena
schedule-random-chain-stopped = Naključna načrtovana veriga ustavljena: nadomeščena z novejšo definicijo
schedule-random-create-failed = Ustvarjanje naslednje naključne naloge ni uspelo
schedule-random-reschedule-failed = Prerazporeditev naključne naloge ni uspela
schedule-stale-dispatch-skipped = Načrtovano pošiljanje preskočeno: zastarel razpored
scheduled-dispatch-error = Napaka pri načrtovanem pošiljanju
scheduled-dispatch-manifest-writer-unavailable = Načrtovano pošiljanje: pisec manifesta ni pripravljen; stanje entitete čaka
