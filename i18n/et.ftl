# ma-runtime – Eesti
lang-name = Eesti

own-did-published = Oma DID dokument avaldatud IPNS-is
own-did-publish-failed = Oma DID dokumendi avaldamine ebaõnnestus
own-did-publish-timeout = Oma DID dokumendi avaldamine aegus 2 minuti pärast
started = ma runtime käivitatud
shutdown-requested = Seiskamine nõutud
closing-endpoint = iroh lõpp-punkti sulgemine...
shutdown-complete = Seiskamine lõpetatud
status-listening = Olekuserver kuulab
ipfs-message-rejected = IPFS sõnum tagasi lükatud
ctrlc-handler-failed = Ctrl-C käsitleja ebaõnnestus
node-connected = Sõlm ühendatud protokolliga
received-encrypted-ma-msg = Krüptitud ma sõnum saadud /ma/ipfs/0.0.1 kaudu
ping-received = :ping saadud, saadan :pong
did-publish-request-received = DID dokumendi avaldamise päring saadud
document-published = Dokument avaldatud
did-publish-cid-reply-sent = CID vastus saadetud DID avaldamise jaoks
did-publish-resolve-failed = Saatjat ei õnnestunud lahendada ipfs-publish vastuse edastamiseks
ipfs-store-request-received = IPFS salvestamise päring saadud
ipfs-stored = Sisu salvestatud IPFS-i
ipfs-store-cid-reply-sent = CID vastus saadetud
ipfs-store-resolve-failed = Saatjat ei õnnestunud lahendada ipfs-store vastuse edastamiseks

# Olemite saatmine
bootstrap-complete = Bootstrap lõpetatud
entity-loaded = Olemite plugin laaditud
entity-load-failed = Olemite plugini laadimine ebaõnnestus
root-list-entities = #root: olemite loend
entity-created = Olem loodud
entity-reloaded = Olemite plugin uuesti laaditud
entity-deleted = Olem kustutatud
entity-states-saving = Olemite olekute salvestamine IPFS-i
entity-state-saving = Olemi oleku salvestamine
entity-state-saved = Olemi olek salvestatud
entity-state-empty = Plugin tagastas tühja oleku, salvestamine vahele jäetud
entity-states-saved = Olemite olekud salvestatud

# Esimene käivitus / auto-init

# Omandiõigus
runtime-claimed = Runtime registreeritud.

# Kaitstud juureelemendid
refuse-delete-root = Keeldun kategooriliselt nõutava juureelemendi kustutamisest
runtime-claim-persisted = Omanik kirjutatud konfiguratsiooni.


# Namespace creation (:create)
crud-message-received = CRUD-sõnum vastu võetud
crud-acl-updated = Juurtranspordi ACL uuendati

# CRUD validation errors
cidv1-required = väärtus peab olema puhas CIDv1 (algab 'b'-ga; CIDv0 'Qm…' ei aktsepteerita)
config-key-protected = konfiguratsioonivõti '%key%' on kaitstud
config-key-no-delete = deemoni konfiguratsioonivõtit '%key%' ei saa kustutada
config-key-not-manifest = konfiguratsioonivõti '%key%' ei ole teadaolev manifest config võti
wrong-crud-protocol = vale CRUD-protokoll: %type%
entity-name-invalid = entity nimi peab olema prinditav UTF-8
reserved-entity-name = entity nimi '%name%' on reserveeritud
genesis-kind-owner-only = Ainult runtime omanik tohib luua genesis-tüüpi olemi

# IPv6 config
ipv6-enabled = IPv6 on lubatud — seob nii IPv4 kui ka IPv6
ipv6-disabled = IPv6 on keelatud — seotakse ainult IPv4 (uuesti lubamiseks on vajalik restart)
ipv6-enable-restart-required = Salvestatud. Muudatuse jõustumiseks on vajalik restart.
ipv6-enable-unchanged = ipv6_enable on juba sellele väärtusele seatud — muudatusi pole.

boot-default-root-config-populate-failed = Vaikimisi juureseadistuse täitmine ebaõnnestus
boot-default-root-config-populated = Vaikimisi juureseadistus täidetud
boot-entity-load-processed = Olemite pluginad laaditud
boot-group-load-failed = Grupi laadimine käivitamisel ebaõnnestus
boot-group-loaded-into-cache = Grupp laaditud vahemällu
boot-kinds-overlay-applied = Kinds-katmine rakendatud
boot-kinds-overlay-no-change = Kinds-katmine ei muutnud manifesti
boot-load-manifest-for-acl-cache-failed = Manifesti laadimine ACL vahemälu täitmiseks ebaõnnestus
boot-minimal-manifest-bootstrapped = Minimaalne manifest lähtestatud
boot-minimal-manifest-not-found = Käituskeskkonna juure CID ei leitud IPNS-st; minimaalse manifesti lähtestamine
boot-no-root-entity = Vaikimisi juureseadistuse jaoks pole registreeritud juurolemi
boot-reconciled-owners-manifest-failed = Omanike kooskõlastamine manifestis käivitamisel ebaõnnestus
boot-reconciled-owners-persist-failed = Kooskõlastatud omanike salvestamine config.yaml-i ebaõnnestus
boot-reconciled-owners-published = Omanikud kooskõlastatud config.yaml/--owner allikatest manifesti käivitamisel
boot-root-acl-load-cache-failed = Juur-ACL laadimine käivitamisel ebaõnnestus
boot-root-acl-load-failed = Juur-ACL laadimine manifestist ebaõnnestus
boot-root-acl-loaded-from-manifest = Juurülekandeprotokoll ACL laaditud manifestist
boot-root-acl-loaded-into-cache = Juur-ACL laaditud vahemällu
bootstrap-acl-published = ACL sõlm avaldatud
bootstrap-endpoint-close-stuck = Lõpp-punkt on jätkuvalt aktiivne 10 s pärast; sundlõpetus
bootstrap-endpoint-close-timeout = Lõpp-punkti sulgemine aegus 5 s pärast; sundlõpetus
bootstrap-entity-lifecycle-update-failed = Olemi uuendatud elutsükli kirjutamine IPFS-i ebaõnnestus
bootstrap-entity-lifecycle-updated = Olemi elutsükkel uuendatud IPFS-is
bootstrap-entity-node-shutdown-updated = Olemi sõlm uuendatud seiskamise ajal
bootstrap-entity-published = Olemi sõlm avaldatud
bootstrap-entity-registering-prepublished = Eelnevalt avaldatud olemi registreerimine
bootstrap-entity-registry-fetch-failed = Olemi sõlme toomine ebaõnnestus
bootstrap-entity-registry-kind-extends-failed = Kinds laiendusahela lahendamine ebaõnnestus
bootstrap-entity-registry-kind-fetch-failed = Kind sõlme toomine ebaõnnestus
bootstrap-entity-registry-kind-missing = Kind ei leitud manifestist; olem jäeti vahele
bootstrap-entity-registry-not-in-manifest = Olem registris aga mitte manifestis; jäeti vahele
bootstrap-entity-state-save-failed = Olemiseisundite salvestamine ebaõnnestus
bootstrap-entity-state-shutdown-aborted = Seiskamine katkestatud; käituskeskkond jääb aktiivseks, et salvestada olek järgmisel seiskamisel
bootstrap-entity-state-update-fetch-failed = Olemi sõlme toomine oleku uuendamiseks ebaõnnestus
bootstrap-envelope-delivery-failed = Plugina ümbriku kättetoimetamine ebaõnnestus; ümbrik kõrvaldatud
bootstrap-envelope-open-failed = Plugina ümbrik: väljamineva postkasti avamine ebaõnnestus; ümbrik kõrvaldatud
bootstrap-group-published = Gruppisõlm avaldatud
bootstrap-kind-published = Kind-sõlm avaldatud
bootstrap-kind-registry-extends-failed = Kinds laiendusahela lahendamine registri jaoks ebaõnnestus
bootstrap-kind-registry-fetch-log-failed = Kind-sõlme toomine registri jaoks ebaõnnestus
bootstrap-kind-registry-hydrated = Kindide register täidetud manifestist
bootstrap-kinds-overlay-pin-update-failed = Kinnistamine/uuendamine ebaõnnestus pärast kinds-katmist
bootstrap-kinds-overlay-published = Käituskeskkonna manifest avaldatud pärast kinds-katmist
bootstrap-kinds-tree-published = Käituskeskkonna kindide puu avaldatud
bootstrap-lifecycle-manifest-pin-update-failed = Kinnistamine/uuendamine ebaõnnestus pärast elutsükli säilitamist
bootstrap-lifecycle-manifest-publish-failed = Manifesti avaldamine pärast elutsüklisiirdeid ebaõnnestus
bootstrap-lifecycle-manifest-published = Uuendatud manifest avaldatud pärast elutsüklisiirdeid
bootstrap-manifest-fetch-failed = Käituskeskkonna manifesti toomine ebaõnnestus
bootstrap-minimal-manifest-failed = Minimaalse manifesti lähtestamine ebaõnnestus
bootstrap-remote-root-pin-confirmed = Kauguurkinnistus kinnitatud
bootstrap-remote-root-pin-misconfigured = Kaugjuurekinnistus on valesti konfigureeritud
bootstrap-root-acl-published = Juurülekandeprotokoll ACL avaldatud
bootstrap-root-cid-shutdown-persist-failed = root_cid säilitamine seiskamise ajal ebaõnnestus
bootstrap-root-cid-shutdown-publish-failed = runtime_ipns avaldamine seiskamise ajal ebaõnnestus
bootstrap-root-cid-shutdown-publish-succeeded = runtime_ipns avaldamine seiskamise ajal õnnestus
bootstrap-root-cid-shutdown-publish-timeout = runtime_ipns avaldamine seiskamise ajal aegus
bootstrap-root-pin-replacement-failed = Jätkamine pärast kaugjuurekinnistuse asendamise tõrget
bootstrap-root-pin-update-failed = Kinnistamine/uuendamine ebaõnnestus pärast käivitamist
bootstrap-runtime-manifest-published = Käituskeskkonna juurmanifest avaldatud
crud-message-rejected = CRUD sõnum lükati tagasi
entity-reload-current-node-load-failed = Praeguse olemi sõlme toomine enne uuslaadimist ebaõnnestus; praegune plugin säilitatud
entity-reload-failed = Olemi uususlaadimine ebaõnnestus; keelatud kuni järgmise uuslaadimiseni
entity-reload-kind-extends-failed = Kind laiendusahela lahendamine olemi uuslaadimise ajal ebaõnnestus
entity-reload-kind-fetch-failed = Kind-sõlme toomine olemi uuslaadimise ajal ebaõnnestus
entity-reload-kind-lookup-failed = Manifesti toomine kind-otsinguks olemi uuslaadimise ajal ebaõnnestus
entity-reload-kind-missing = Kind ei leitud manifestist; olemi ei saa uuslaadimatult laadida
entity-reload-manifest-state-update-failed = Manifesti uuendamine praeguse olekuga enne uuslaadimist ebaõnnestus; praegune plugin säilitatud
entity-reload-skipped = Olemi uususlaadimine jäeti vahele, kuna uuslaadimisvärav on suletud
entity-reload-started = Olemi uususlaadimine alustatud
entity-reload-state-persist-failed = Praeguse oleku säilitamine enne uuslaadimist ebaõnnestus; praegune plugin säilitatud
entity-reload-state-produced-failed = Uuslaadimise käigus toodetud oleku säilitamine ebaõnnestus
entity-reloaded-manifest-update-failed = Uuslaaditud olemi uuendamine manifestis ebaõnnestus
entity-reloaded-manifest-updated = Uuslaaditud olem uuendatud manifestis
inbox-message-rejected = Postkasti sõnum lükati tagasi
ma-create-entity-already-exists = ma_create_entity: olem on juba olemas; praegune olem säilitatud
ma-create-entity-invalid-behaviour = ma_create_entity: vigane käitumisviide; jäeti vahele
ma-create-entity-kind-missing = ma_create_entity: kind puudub registrist; jäeti vahele
manifest-pin-update-failed = Manifesti pin_update ebaõnnestus
plugin-envelope-build-failed = Plugina ümbrik: sõnumi koostamine ebaõnnestus; jäeti vahele
plugin-envelope-create-requests-ignored = Plugina ümbrik: loomispäringud eiratakse kõrvaltoime kontekstita
plugin-envelope-local-dispatch-failed = Plugina ümbrik: kohalik saatmine ebaõnnestus
plugin-envelope-local-dispatch-finish = Plugina ümbrik: kohalik saatmine lõpetatud
plugin-envelope-local-dispatch-start = Plugina ümbrik: kohalik saatmine alustatud
plugin-envelope-local-gate-closed = Plugina ümbrik: kohaliku saatmise värav suletud
plugin-envelope-local-recipient-unknown = Plugina ümbrik: tundmatu kohalik saaja; jäeti vahele
plugin-envelope-local-timeout = Plugina ümbrik: kohalik saatmine aegus
plugin-envelope-recipient-invalid = Plugina ümbrik: vigane saaja DID; jäeti vahele
plugin-envelope-remote-limit = Plugina ümbrik: kaugkätte limiit saavutatud; ümbrik kõrvaldatud
plugin-outbox-congested = Plugina väljaminev postkast ülekoormusel; ümbrikke võidakse kõrvaldada, kui kanal täitub
plugin-outbox-drain-limit = Plugina väljamineva postkasti tühjendamise eelarve ammendunud; allesjäänud ümbrikud lükatud edasi
schedule-dispatch-firing = Ajastatud saatmine käimas
schedule-entity-not-found = Ajastatud saatmine: olem ei leitud
schedule-random-chain-stopped = Juhuslik ajastusahel peatatud: asendatud uuema definitsiooniga
schedule-random-create-failed = Järgmise juhusliku ülesande loomine ebaõnnestus
schedule-random-reschedule-failed = Juhusliku ülesande ümberajastamine ebaõnnestus
schedule-stale-dispatch-skipped = Ajastatud saatmine jäeti vahele: aegunud ajakava
scheduled-dispatch-error = Viga ajastatud saatmisel
scheduled-dispatch-manifest-writer-unavailable = Ajastatud saatmine: manifesti kirjutaja ei ole valmis; olemi olek ootab
