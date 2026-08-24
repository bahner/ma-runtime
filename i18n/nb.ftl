# ma-runtime – Norsk Bokmål
lang-name = Norsk bokmål

own-did-published = Eget DID-dokument publisert til IPNS
own-did-publish-failed = Kunne ikke publisere eget DID-dokument
own-did-publish-timeout = Timeout ved publisering av eget DID-dokument (2 min)
started = ma runtime startet
shutdown-requested = Avslutning forespurt
closing-endpoint = Lukker iroh-endepunkt ...
shutdown-complete = Avslutning fullført
status-listening = Statusserver lytter
rpc-message-received = Mottok RPC-melding
crud-message-received = Mottok CRUD-melding
crud-acl-updated = Rot-transport-ACL oppdatert
rpc-message-rejected = RPC-melding avvist
ipfs-message-rejected = IPFS-melding avvist
ctrlc-handler-failed = Ctrl-C-behandler feilet
node-connected = Node koblet til protokoll
received-encrypted-ma-msg = Mottok kryptert ma-melding på /ma/ipfs/0.0.1
unknown-rpc-atom = Ukjent RPC-atom, ignorerer
rpc-not-text-atom = RPC-innhold er ikke et tekstatom
rpc-unknown-verb = Ukjent RPC-verb
rpc-reply-sent = RPC-svar sendt
ping-received = Mottok :ping, sender :pong
did-publish-request-received = Mottok forespørsel om publisering av dokument
document-published = Dokument publisert
did-publish-cid-reply-sent = Sendt CID-svar for DID-publisering
did-publish-resolve-failed = Kunne ikke løse opp avsender for ipfs-publish-svar
ipfs-store-request-received = Mottok IPFS store-forespørsel
ipfs-stored = Lagret innhold på IPFS
ipfs-store-cid-reply-sent = Sendt CID-svar
ipfs-store-resolve-failed = Kunne ikke løse opp avsender for ipfs-store-svar

# Enhetsutsending
bootstrap-complete = Bootstrap fullført
entity-loaded = Enhetsplugin lastet
entity-load-failed = Feil ved lasting av enhetsplugin
entity-not-found = Enhet ikke funnet, ignorerer RPC
entity-dispatched = RPC sendt til enhet
entity-replied = Enhet sendte RPC-svar
root-create-entity = #root: opprett enhet
root-list-entities = #root: list enheter
root-delete-entity = #root: slett enhet
root-entity-updated = Runtime-manifest oppdatert
default-config-root-populated = Standard /config/root satt ved oppstart
default-config-root-no-root-entity = Kan ikke sette standard /config/root ved oppstart: #root-enheten er ikke lastet
default-config-root-no-root-cid = Kan ikke sette standard /config/root ved oppstart: ingen manifest-root-CID er tilgjengelig
default-config-root-inspect-failed = Klarte ikke å lese manifestet før standard /config/root ble satt
default-config-root-populate-failed = Klarte ikke å sette standard /config/root ved oppstart
entity-created = Enhet opprettet
entity-reloaded = Enhetsplugin lastet på nytt
entity-deleted = Enhet slettet
entity-states-saving = Lagrer enhetstilstander til IPFS
entity-state-saving = Lagrer enhetstilstand
entity-state-saved = Enhetstilstand lagret
entity-state-empty = Plugin returnerte tom tilstand, hopper over lagring
entity-states-saved = Enhetstilstander lagret
link-set = Lenke satt
ftl-loaded = Språkmeldinger lastet fra IPFS

# Første-gangs auto-oppsett
no-config-found = Ingen konfigurasjon funnet.
initialising-new-identity = Oppretter ny runtime-identitet.
generated-headless-config = Genererte hodeløs konfigurasjon.

# Eierskap / krav
runtime-claimed = Runtime registrert.

# Beskyttede rotelementer
refuse-delete-root = Nekter bestemt å slette påkrevd rotelement
no-root-acl = Ingen rot-ACL er konfigurert — kjøretiden opererer uten tilgangskontroll
acl-owners-access = Innringer fikk tilgang som medlem av +owners
runtime-claim-persisted = Eier skrevet til konfigurasjon.
runtime-already-claimed = Runtime er allerede registrert.


# Namespace creation (:create)

# CRUD validation errors
blob-value-ipfs-path = blob-verdien må være en IPFS-sti (/ipfs/, /ipns/ eller /ipld/)
acl-value-ipfs-path = ACL-verdien må være en IPFS-sti (/ipfs/, /ipns/ eller /ipld/)
kind-value-ipfs-path = kind-verdien må være en IPFS-sti (/ipfs/, /ipns/ eller /ipld/)
kind-not-found = Typen ble ikke funnet
cidv1-required = verdien må være en ren CIDv1 (starter med 'b'; CIDv0 'Qm…' godtas ikke)
config-key-protected = config-nøkkelen '%key%' er beskyttet
config-key-no-delete = daemon-config-nøkkelen '%key%' kan ikke slettes
config-key-not-manifest = config-nøkkelen '%key%' er ikke en kjent manifest config-nøkkel
owners-value-not-list = owners-verdien må være en liste av DID-er, ikke en enkelt verdi
wrong-crud-protocol = feil CRUD-protokoll: %type%
entity-name-invalid = entity-navn må være skrivbart UTF-8
reserved-entity-name = entity-navn '%name%' er reservert
genesis-kind-owner-only = Bare en runtime-eier kan opprette en entity av typen genesis

# IPv6 config
ipv6-enabled = IPv6 aktivert — lytter på både IPv4 og IPv6
ipv6-disabled = IPv6 er deaktivert — binder kun IPv4 (restart kreves for å aktivere på nytt)
ipv6-enable-restart-required = Lagret. Restart kreves for at denne endringen skal tre i kraft.
ipv6-enable-unchanged = ipv6_enable er allerede satt til den verdien — ingen endring.

# Oppstart, bootstrap, avslutning og innboks
bootstrap-manifest-fetch-failed = Klarte ikke å hente runtime-manifestet
bootstrap-kind-registry-hydrated = Kind-registeret ble fylt fra manifestet
bootstrap-minimal-manifest-failed = Klarte ikke å opprette et minimalt manifest
bootstrap-entity-state-save-failed = Klarte ikke å lagre enhetstilstander
bootstrap-entity-state-shutdown-aborted = avslutning avbrutt; runtime fortsetter slik at tilstanden kan lagres ved et senere avslutningsforsøk
bootstrap-root-cid-shutdown-persist-failed = klarte ikke å lagre root_cid under avslutningen
bootstrap-root-cid-shutdown-publish-succeeded = publisering av runtime_ipns ved avslutning fullført
bootstrap-root-cid-shutdown-publish-failed = publisering av runtime_ipns ved avslutning feilet
bootstrap-root-cid-shutdown-publish-timeout = tidsavbrudd ved publisering av runtime_ipns under avslutningen
bootstrap-endpoint-close-timeout = tidsavbrudd ved lukking av endepunktet etter 5 s; tvinger avslutning
bootstrap-endpoint-close-stuck = endepunktet holdes fortsatt av aktive oppgaver etter 10 s; forkaster det uten kontrollert lukking
bootstrap-envelope-delivery-failed = levering av plugin-konvolutt feilet; forkaster konvolutten
bootstrap-envelope-open-failed = plugin-konvolutt: klarte ikke å åpne utboks; forkaster konvolutten
boot-minimal-manifest-not-found = Fant ingen runtime-root-CID i IPNS; oppretter et minimalt manifest
boot-minimal-manifest-bootstrapped = Minimalt manifest opprettet
boot-kinds-overlay-no-change = Kind-overlagringen endret ikke manifestet
boot-kinds-overlay-applied = Kind-overlagring lagt på
boot-load-manifest-for-acl-cache-failed = Klarte ikke å laste manifestet for å fylle ACL-hurtigbufferet
boot-root-acl-loaded-from-manifest = Rotens transport-ACL lastet fra manifestet
boot-root-acl-load-failed = Klarte ikke å laste rotens ACL fra manifestet
boot-group-loaded-into-cache = Gruppe lastet inn i hurtigbufferet
boot-group-load-failed = Klarte ikke å laste gruppen ved oppstart
boot-root-acl-loaded-into-cache = Rotens ACL lastet inn i hurtigbufferet
boot-root-acl-load-cache-failed = Klarte ikke å laste rotens ACL ved oppstart
boot-reconciled-owners-persist-failed = Klarte ikke å lagre de samordnede eierne i config.yaml
boot-reconciled-owners-published = Eiere fra config.yaml/--owner samordnet inn i manifestet ved oppstart
boot-reconciled-owners-manifest-failed = Klarte ikke å samordne eiere inn i manifestet ved oppstart
boot-no-root-entity = Ingen root-enhet er registrert for standard konfigurasjonsrot
boot-default-root-config-populated = Standard konfigurasjonsrot satt
boot-default-root-config-populate-failed = Klarte ikke å sette standard konfigurasjonsrot
boot-default-root-config-skip = Standard konfigurasjonsrot er allerede satt
boot-entity-load-processed = Enhetspluginer lastet
boot-runtime-kinds-overlay-failed = Klarte ikke å legge på kind-CID-overlagringen
boot-runtime-manifest-load-failed = Klarte ikke å laste manifestet for å fylle ACL-hurtigbufferet
boot-runtime-root-cid-resolve-failed = Klarte ikke å løse runtime-root-CID fra IPNS
inbox-empty-payload-dropped = innboks: tom nyttelast forkastet
inbox-unfragmented-dropped = innboks: ufragmentert melding forkastet
inbox-unknown-entity = innboks: ukjent enhetsfragment; forkastet
inbox-dispatching = innboks: sender til enhet
inbox-behaviour-ignored = innboks: forespørsler om ma_set_behaviour ignoreres ved innboksdistribusjon uten svar
bootstrap-entity-lifecycle-updated = Enhetens livssyklus oppdatert i IPFS
bootstrap-entity-lifecycle-update-failed = Klarte ikke å skrive den oppdaterte enhetslivssyklusen til IPFS
bootstrap-entity-node-shutdown-updated = Enhetsnoden oppdatert under avslutningen
bootstrap-entity-registry-not-in-manifest = Enhet finnes i registeret, men ikke i manifestet; hopper over
plugin-outbox-drain-limit = plugin-utboksens tømmingsgrense er nådd; utsetter resten av konvoluttene
plugin-outbox-congested = plugin-utboksen er overbelastet; konvolutter kan gå tapt hvis kanalen fylles
plugin-envelope-local-reply-dropped = plugin-konvolutt: lokalt RPC-svar forkastet (ingen lokal svarmottaker)
plugin-envelope-local-gate-closed = plugin-konvolutt: lokal distribusjonsport er stengt
plugin-envelope-local-timeout = plugin-konvolutt: lokal distribusjon fikk tidsavbrudd
plugin-envelope-recipient-invalid = plugin-konvolutt: ugyldig mottaker-DID; hopper over
plugin-envelope-build-failed = plugin-konvolutt: klarte ikke å bygge melding; hopper over
plugin-envelope-remote-limit = plugin-konvolutt: grensen for ekstern levering er nådd; konvolutten forkastes
scheduled-dispatch-error = feil ved planlagt distribusjon
scheduled-dispatch-manifest-writer-unavailable = planlagt distribusjon: manifest-skriveren er ikke klar; enhetstilstanden forblir ubehandlet
manifest-pin-update-failed = oppdatering av manifestets pin feilet
bootstrap-kind-registry-fetch-log-failed = Klarte ikke å hente kind-noden til registeret
bootstrap-entity-state-update-fetch-failed = Klarte ikke å hente enhetsnoden for tilstandsoppdatering
schedule-stale-dispatch-skipped = planlagt distribusjon hoppet over: foreldet plan
schedule-random-reschedule-failed = klarte ikke å planlegge den tilfeldige jobben på nytt
schedule-random-create-failed = klarte ikke å opprette neste tilfeldige jobb
schedule-random-chain-stopped = kjeden for tilfeldig planlegging stoppet: erstattet av en nyere definisjon
schedule-entity-not-found = planlagt distribusjon: enhet ikke funnet
schedule-dispatch-firing = planlagt distribusjon utløses
