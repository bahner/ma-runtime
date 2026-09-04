# ma-runtime – Svenska
lang-name = Svenska

own-did-published = Eget DID-dokument publicerat på IPNS
own-did-publish-failed = Misslyckades med att publicera eget DID-dokument
own-did-publish-timeout = Publicering av eget DID-dokument tog för lång tid efter 2 minuter
started = ma runtime startad
shutdown-requested = Avstängning begärd
closing-endpoint = Stänger iroh-slutpunkt...
shutdown-complete = Avstängning slutförd
status-listening = Statusserver lyssnar
ipfs-message-rejected = IPFS-meddelande avvisat
ctrlc-handler-failed = Ctrl-C-hanterare misslyckades
node-connected = Nod ansluten till protokoll
received-encrypted-ma-msg = Krypterat ma-meddelande mottaget på /ma/ipfs/0.0.1
ping-received = :ping mottaget, skickar :pong
did-publish-request-received = Begäran om publicering av DID-dokument mottagen
document-published = Dokument publicerat
did-publish-cid-reply-sent = CID-svar skickat för DID-publicering
did-publish-resolve-failed = Kunde inte lösa upp avsändare för att leverera ipfs-publish-svar
ipfs-store-request-received = IPFS-lagringsbegäran mottagen
ipfs-stored = Innehåll lagrat på IPFS
ipfs-store-cid-reply-sent = CID-svar skickat
ipfs-store-resolve-failed = Kunde inte lösa upp avsändare för att leverera ipfs-store-svar

# Entitetsutsändning
bootstrap-complete = Bootstrap slutförd
entity-loaded = Entitetsplugin laddad
entity-load-failed = Misslyckades med att ladda entitetsplugin
root-list-entities = #root: lista entiteter
entity-created = Entitet skapad
entity-reloaded = Entitetsplugin omladdad
entity-deleted = Entitet borttagen
entity-states-saving = Sparar entitetstillstånd till IPFS
entity-state-saving = Sparar entitetstillstånd
entity-state-saved = Entitetstillstånd sparat
entity-state-empty = Plugin returnerade tomt tillstånd, hoppar över sparning
entity-states-saved = Entitetstillstånd sparade

# Första start / auto-init

# Äganderätt
runtime-claimed = Runtime registrerad.

# Skyddade rotelement
refuse-delete-root = Vägrar bestämt att ta bort ett obligatoriskt rotelement
runtime-claim-persisted = Ägare skriven till konfiguration.


# Namespace creation (:create)
crud-message-received = CRUD-meddelande mottaget
crud-acl-updated = Root-transport-ACL uppdaterad

# CRUD validation errors
cidv1-required = värdet måste vara en ren CIDv1 (börjar med 'b'; CIDv0 'Qm…' accepteras inte)
config-key-protected = config-nyckeln '%key%' är skyddad
config-key-no-delete = daemon-config-nyckeln '%key%' kan inte tas bort
config-key-not-manifest = config-nyckeln '%key%' är inte en känd manifest config-nyckel
wrong-crud-protocol = fel CRUD-protokoll: %type%
entity-name-invalid = entity-namn måste vara utskrivbart UTF-8
reserved-entity-name = entity-namn '%name%' är reserverat
genesis-kind-owner-only = Endast en runtime-ägare får skapa en entity av typen genesis

# IPv6 config
ipv6-enabled = IPv6 aktiverat — lyssnar på både IPv4 och IPv6
ipv6-disabled = IPv6 är inaktiverat — binder endast IPv4 (omstart krävs för att återaktivera)
ipv6-enable-restart-required = Sparat. Omstart krävs för att den här ändringen ska träda i kraft.
ipv6-enable-unchanged = ipv6_enable är redan inställt på det värdet — ingen ändring.

boot-default-root-config-populate-failed = Kunde inte fylla standardkonfigurationroten
boot-default-root-config-populated = Standardkonfigurationsroten fylld
boot-entity-load-processed = Entitets-plugins laddade
boot-group-load-failed = Kunde inte ladda grupp vid start
boot-group-loaded-into-cache = Grupp laddad i cache
boot-kinds-overlay-applied = Kinds-overlay tillämpad
boot-kinds-overlay-no-change = Kinds-overlay gjorde inga ändringar i manifestet
boot-load-manifest-for-acl-cache-failed = Kunde inte ladda manifest för ACL-cachefyllning
boot-minimal-manifest-bootstrapped = Minimalt manifest initialiserat
boot-minimal-manifest-not-found = Ingen runtime-rot-CID hittad i IPNS; initialiserar minimalt manifest
boot-no-root-entity = Ingen rotentitet registrerad för standardkonfigurationsroten
boot-reconciled-owners-manifest-failed = Kunde inte stämma av ägare i manifest vid start
boot-reconciled-owners-persist-failed = Kunde inte spara avstämda ägare i config.yaml
boot-reconciled-owners-published = Ägare avstämda från config.yaml/--owner i manifest vid start
boot-root-acl-load-cache-failed = Kunde inte ladda rot-ACL vid start
boot-root-acl-load-failed = Kunde inte ladda rot-ACL från manifest
boot-root-acl-loaded-from-manifest = Rot-transport-ACL laddad från manifest
boot-root-acl-loaded-into-cache = Rot-ACL laddad i cache
bootstrap-acl-published = ACL-nod publicerad
bootstrap-endpoint-close-stuck = Endpoint hålls fortfarande av pågående uppgifter efter 10 s; tvångsstängning
bootstrap-endpoint-close-timeout = Stängning av endpoint tidde ut efter 5 s; tvångsvstängning
bootstrap-entity-lifecycle-update-failed = Kunde inte skriva uppdaterad entitetslivscykel till IPFS
bootstrap-entity-lifecycle-updated = Entitetslivscykeln uppdaterad i IPFS
bootstrap-entity-node-shutdown-updated = Entitetsnod uppdaterad vid avstängning
bootstrap-entity-published = Entitetsnod publicerad
bootstrap-entity-registering-prepublished = Registrerar förpublicerad entitet
bootstrap-entity-registry-fetch-failed = Kunde inte hämta entitetsnod
bootstrap-entity-registry-kind-extends-failed = Kunde inte lösa kinds-utvidningskedja
bootstrap-entity-registry-kind-fetch-failed = Kunde inte hämta kinds-nod
bootstrap-entity-registry-kind-missing = Kind hittades inte i manifest; hoppar över entitet
bootstrap-entity-registry-not-in-manifest = Entitet i registret men inte i manifest, hoppas över
bootstrap-entity-state-save-failed = Kunde inte spara entitetstillstånd
bootstrap-entity-state-shutdown-aborted = Avstängning avbruten; runtime förblir aktiv så tillståndet kan sparas vid nästa avstängningsförsök
bootstrap-entity-state-update-fetch-failed = Kunde inte hämta entitetsnod för tillståndsuppdatering
bootstrap-envelope-delivery-failed = Plugin-kuvertleverans misslyckades; kuvert kasserat
bootstrap-envelope-open-failed = Plugin-kuvert: öppning av utbox misslyckades; kuvert kasserat
bootstrap-group-published = Gruppnod publicerad
bootstrap-kind-published = Kinds-nod publicerad
bootstrap-kind-registry-extends-failed = Kunde inte lösa kinds-utvidningskedja för registret
bootstrap-kind-registry-fetch-log-failed = Kunde inte hämta kinds-nod för registret
bootstrap-kind-registry-hydrated = Kinds-register hydratiserat från manifest
bootstrap-kinds-overlay-pin-update-failed = Pin/uppdatering misslyckades efter kinds-overlay
bootstrap-kinds-overlay-published = Runtime-manifest publicerat efter kinds-overlay
bootstrap-kinds-tree-published = Runtime kinds-träd publicerat
bootstrap-lifecycle-manifest-pin-update-failed = Pin/uppdatering misslyckades efter livscykelspersistering
bootstrap-lifecycle-manifest-publish-failed = Kunde inte publicera manifest efter livscykelövergångar
bootstrap-lifecycle-manifest-published = Uppdaterat manifest publicerat efter livscykelövergångar
bootstrap-manifest-fetch-failed = Kunde inte hämta runtime-manifest
bootstrap-minimal-manifest-failed = Kunde inte initialisera minimalt manifest
bootstrap-remote-root-pin-confirmed = Extern rot-pin bekräftad
bootstrap-remote-root-pin-misconfigured = Extern rot-pinning är felkonfigurerad
bootstrap-root-acl-published = Rot-transport-ACL publicerad
bootstrap-root-cid-shutdown-persist-failed = Kunde inte persistera root_cid vid avstängning
bootstrap-root-cid-shutdown-publish-failed = runtime_ipns-publicering vid avstängning misslyckades
bootstrap-root-cid-shutdown-publish-succeeded = runtime_ipns-publicering vid avstängning lyckades
bootstrap-root-cid-shutdown-publish-timeout = runtime_ipns-publicering vid avstängning tidde ut
bootstrap-root-pin-replacement-failed = Fortsätter efter fel vid ersättning av extern rot-pin
bootstrap-root-pin-update-failed = Pin/uppdatering misslyckades efter bootstrap
bootstrap-runtime-manifest-published = Runtime rot-manifest publicerat
crud-message-rejected = CRUD-meddelande avvisat
entity-reload-current-node-load-failed = Kunde inte ladda nuvarande entitetsnod före omladdning; behåller nuvarande plugin
entity-reload-failed = Entitet kunde inte laddas om; inaktiveras till nästa omladdning
entity-reload-kind-extends-failed = Kunde inte lösa kinds-utvidningskedja vid entitetsomladdning
entity-reload-kind-fetch-failed = Kunde inte hämta kinds-nod vid entitetsomladdning
entity-reload-kind-lookup-failed = Kunde inte ladda manifest för kinds-sökning vid entitetsomladdning
entity-reload-kind-missing = Kind hittades inte i manifest; kan inte ladda om entitet
entity-reload-manifest-state-update-failed = Kunde inte uppdatera manifest med nuvarande tillstånd före omladdning; behåller nuvarande plugin
entity-reload-skipped = Entitetsomladdning hoppad över eftersom omladdningsporten är stängd
entity-reload-started = Entitetsomladdning startad
entity-reload-state-persist-failed = Kunde inte persistera nuvarande tillstånd före omladdning; behåller nuvarande plugin
entity-reload-state-produced-failed = Kunde inte persistera tillstånd producerat under omladdning
entity-reloaded-manifest-update-failed = Kunde inte uppdatera omladdad entitet i manifest
entity-reloaded-manifest-updated = Omladdad entitet uppdaterad i manifest
inbox-message-rejected = Inkorgsmeddelande avvisat
ma-create-entity-already-exists = ma_create_entity: entitet finns redan; behåller nuvarande entitet
ma-create-entity-invalid-behaviour = ma_create_entity: ogiltig behaviour-referens; hoppad över
ma-create-entity-kind-missing = ma_create_entity: kind inte i registret; hoppad över
manifest-pin-update-failed = Manifest pin_update misslyckades
plugin-envelope-build-failed = Plugin-kuvert: kunde inte bygga meddelande; hoppad över
plugin-envelope-create-requests-ignored = Plugin-kuvert: skapandeförfrågningar ignorerade utan bieffektskontext
plugin-envelope-local-dispatch-failed = Plugin-kuvert: lokal dispatch misslyckades
plugin-envelope-local-dispatch-finish = Plugin-kuvert: lokal dispatch avslutad
plugin-envelope-local-dispatch-start = Plugin-kuvert: lokal dispatch startad
plugin-envelope-local-gate-closed = Plugin-kuvert: lokal dispatch-port stängd
plugin-envelope-local-recipient-unknown = Plugin-kuvert: okänd lokal mottagare; hoppad över
plugin-envelope-local-timeout = Plugin-kuvert: lokal dispatch tidde ut
plugin-envelope-recipient-invalid = Plugin-kuvert: ogiltig mottagare-DID; hoppad över
plugin-envelope-remote-limit = Plugin-kuvert: extern leveransgräns nådd; kuvert kasserat
plugin-outbox-congested = Plugin-utbox överbelastad; kuvert kan kasseras om kanalen fylls
plugin-outbox-drain-limit = Plugin-utbox tömningsbudget uttömd; kvarvarande kuvert uppskjutna
schedule-dispatch-firing = Schemalagd dispatch utförs
schedule-entity-not-found = Schemalagd dispatch: entitet hittades inte
schedule-random-chain-stopped = Slumpmässig schemakedja stoppad: ersatt av nyare definition
schedule-random-create-failed = Kunde inte skapa nästa slumpmässiga jobb
schedule-random-reschedule-failed = Kunde inte omplanera slumpmässigt jobb
schedule-stale-dispatch-skipped = Schemalagd dispatch hoppad över: inaktuellt schema
scheduled-dispatch-error = Fel vid schemalagd dispatch
scheduled-dispatch-manifest-writer-unavailable = Schemalagd dispatch: manifestskrivare är inte redo; entitetstillstånd väntar
