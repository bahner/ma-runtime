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
crud-message-received = Mottok CRUD-melding
crud-acl-updated = Rot-transport-ACL oppdatert
ipfs-message-rejected = IPFS-melding avvist
ctrlc-handler-failed = Ctrl-C-behandler feilet
node-connected = Node koblet til protokoll
received-encrypted-ma-msg = Mottok kryptert ma-melding på /ma/ipfs/0.0.1
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
root-list-entities = #root: list enheter
entity-created = Enhet opprettet
entity-reloaded = Enhetsplugin lastet på nytt
entity-deleted = Enhet slettet
entity-states-saving = Lagrer enhetstilstander til IPFS
entity-state-saving = Lagrer enhetstilstand
entity-state-saved = Enhetstilstand lagret
entity-state-empty = Plugin returnerte tom tilstand, hopper over lagring
entity-states-saved = Enhetstilstander lagret

# Første-gangs auto-oppsett

# Eierskap / krav
runtime-claimed = Runtime registrert.

# Beskyttede rotelementer
refuse-delete-root = Nekter bestemt å slette påkrevd rotelement
runtime-claim-persisted = Eier skrevet til konfigurasjon.


# Namespace creation (:create)

# CRUD validation errors
cidv1-required = verdien må være en ren CIDv1 (starter med 'b'; CIDv0 'Qm…' godtas ikke)
config-key-protected = config-nøkkelen '%key%' er beskyttet
config-key-no-delete = daemon-config-nøkkelen '%key%' kan ikke slettes
config-key-not-manifest = config-nøkkelen '%key%' er ikke en kjent manifest config-nøkkel
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
boot-entity-load-processed = Enhetspluginer lastet
bootstrap-entity-lifecycle-updated = Enhetens livssyklus oppdatert i IPFS
bootstrap-entity-lifecycle-update-failed = Klarte ikke å skrive den oppdaterte enhetslivssyklusen til IPFS
bootstrap-entity-node-shutdown-updated = Enhetsnoden oppdatert under avslutningen
bootstrap-entity-registry-not-in-manifest = Enhet finnes i registeret, men ikke i manifestet; hopper over
plugin-outbox-drain-limit = plugin-utboksens tømmingsgrense er nådd; utsetter resten av konvoluttene
plugin-outbox-congested = plugin-utboksen er overbelastet; konvolutter kan gå tapt hvis kanalen fylles
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

bootstrap-acl-published = ACL-node publisert
bootstrap-entity-published = Entity-node publisert
bootstrap-entity-registering-prepublished = Registrerer forhånds-publisert entity
bootstrap-entity-registry-fetch-failed = Klarte ikke hente entity-node
bootstrap-entity-registry-kind-extends-failed = Klarte ikke løse opp kind-utvidelseskjeden
bootstrap-entity-registry-kind-fetch-failed = Klarte ikke hente kind-node
bootstrap-entity-registry-kind-missing = Kind ikke funnet i manifest; hopper over entity
bootstrap-group-published = Gruppe-node publisert
bootstrap-kind-published = Kind-node publisert
bootstrap-kind-registry-extends-failed = Klarte ikke løse opp kind-utvidelseskjeden for registeret
bootstrap-kinds-overlay-pin-update-failed = Pin/oppdatering mislyktes etter kinds-overlag
bootstrap-kinds-overlay-published = Runtime-manifest publisert etter kinds-overlag
bootstrap-kinds-tree-published = Runtime kinds-tre publisert
bootstrap-lifecycle-manifest-pin-update-failed = Pin/oppdatering mislyktes etter livssyklus-lagring
bootstrap-lifecycle-manifest-publish-failed = Klarte ikke publisere manifest etter livssyklus-overganger
bootstrap-lifecycle-manifest-published = Oppdatert manifest publisert etter livssyklus-overganger
bootstrap-remote-root-pin-confirmed = Ekstern rot-pin bekreftet
bootstrap-remote-root-pin-misconfigured = Ekstern rot-pinning er feilkonfigurert
bootstrap-root-acl-published = Root transport-ACL publisert
bootstrap-root-pin-replacement-failed = Fortsetter etter en feil ved erstatning av ekstern rot-pin
bootstrap-root-pin-update-failed = Pin/oppdatering mislyktes etter oppstart
bootstrap-runtime-manifest-published = Runtime rot-manifest publisert
crud-message-rejected = CRUD-melding avvist
entity-reload-current-node-load-failed = Klarte ikke laste gjeldende entity-node før omlasting; beholder gjeldende plugin
entity-reload-failed = Entity klarte ikke omlaste; deaktiveres inntil neste omlasting
entity-reload-kind-extends-failed = Klarte ikke løse opp kind-utvidelseskjeden under entity-omlasting
entity-reload-kind-fetch-failed = Klarte ikke hente kind-node under entity-omlasting
entity-reload-kind-lookup-failed = Klarte ikke laste manifest for kind-oppslag under entity-omlasting
entity-reload-kind-missing = Kind ikke funnet i manifest; kan ikke laste entity på nytt
entity-reload-manifest-state-update-failed = Klarte ikke oppdatere manifest med gjeldende tilstand før omlasting; beholder gjeldende plugin
entity-reload-skipped = Entity-omlasting hoppet over fordi omlastings-porten er lukket
entity-reload-started = Entity-omlasting startet
entity-reload-state-persist-failed = Klarte ikke persistere gjeldende tilstand før omlasting; beholder gjeldende plugin
entity-reload-state-produced-failed = Klarte ikke persistere tilstand produsert under omlasting
entity-reloaded-manifest-update-failed = Klarte ikke oppdatere omlastet entity i manifest
entity-reloaded-manifest-updated = Omlastet entity oppdatert i manifest
inbox-message-rejected = Innboks-melding avvist
ma-create-entity-already-exists = ma_create_entity: entity eksisterer allerede; beholder gjeldende entity
ma-create-entity-invalid-behaviour = ma_create_entity: ugyldig behaviour-referanse; hoppet over
ma-create-entity-kind-missing = ma_create_entity: kind ikke i registeret; hoppet over
plugin-envelope-create-requests-ignored = Plugin-konvolutt: opprettingsforespørsler ignorert uten bieffekt-kontekst
plugin-envelope-local-dispatch-failed = Plugin-konvolutt: lokal utsendelse mislyktes
plugin-envelope-local-dispatch-finish = Plugin-konvolutt: lokal utsendelse fullført
plugin-envelope-local-dispatch-start = Plugin-konvolutt: lokal utsendelse startet
plugin-envelope-local-recipient-unknown = Plugin-konvolutt: ukjent lokal mottaker; hoppet over
