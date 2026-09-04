# ma-runtime – Nynorsk
lang-name = Nynorsk

own-did-published = Eige DID-dokument publisert til IPNS
own-did-publish-failed = Kunne ikkje publisere eige DID-dokument
own-did-publish-timeout = Tidsavbrot ved publisering av eige DID-dokument (2 min)
started = ma runtime starta
shutdown-requested = Avslutning beden
closing-endpoint = Lukkar iroh-endepunkt...
shutdown-complete = Avslutning fullført
status-listening = Statustenar lyttar
crud-message-received = Mottok CRUD-melding
crud-acl-updated = Rot-transport-ACL oppdatert
ipfs-message-rejected = IPFS-melding avvist
ctrlc-handler-failed = Ctrl-C-handsamar feila
node-connected = Node kopla til protokoll
received-encrypted-ma-msg = Mottok kryptert ma-melding på /ma/ipfs/0.0.1
ping-received = Mottok :ping, sender :pong
did-publish-request-received = Mottok førespurnad om publisering av DID-dokument
document-published = Dokument publisert
did-publish-cid-reply-sent = Sendt CID-svar for DID-publisering
did-publish-resolve-failed = Klarte ikkje løyse opp avsendaren for ipfs-publish-svar
ipfs-store-request-received = Mottok IPFS store-førespurnad
ipfs-stored = Lagra innhald på IPFS
ipfs-store-cid-reply-sent = Sendt CID-svar
ipfs-store-resolve-failed = Klarte ikkje løyse opp avsendaren for ipfs-store-svar

# Einingsutsending
bootstrap-complete = Bootstrap fullført
entity-loaded = Einingsplugin lasta
entity-load-failed = Feil ved lasting av einingsplugin
root-list-entities = #root: list einingar
entity-created = Eining oppretta
entity-reloaded = Einingsplugin lasta på nytt
entity-deleted = Eining sletta
entity-states-saving = Lagrar einingstilstandar til IPFS
entity-state-saving = Lagrar einingstilstand
entity-state-saved = Einingstilstand lagra
entity-state-empty = Plugin returnerte tom tilstand, hoppar over lagring
entity-states-saved = Einingstilstandar lagra

# Første gongs auto-oppsett

# Eigarskap
runtime-claimed = Runtime registrert.

# Verna rotelément
refuse-delete-root = Nektar bestemt å slette påkravd rotelement
runtime-claim-persisted = Eigar skrive til konfigurasjon.


# Namespace creation (:create)

# CRUD validation errors
cidv1-required = verdien må vere ein rein CIDv1 (startar med 'b'; CIDv0 'Qm…' vert ikkje godtatt)
config-key-protected = config-nøkkelen '%key%' er verna
config-key-no-delete = daemon-config-nøkkelen '%key%' kan ikkje slettast
config-key-not-manifest = config-nøkkelen '%key%' er ikkje ein kjend manifest config-nøkkel
wrong-crud-protocol = feil CRUD-protokoll: %type%
entity-name-invalid = entity-namn må vere skrivbart UTF-8
reserved-entity-name = entity-namn '%name%' er reservert
genesis-kind-owner-only = Berre ein runtime-eigar kan opprette ein entity av typen genesis

# IPv6 config
ipv6-enabled = IPv6 aktivert — bind til både IPv4 og IPv6
ipv6-disabled = IPv6 er deaktivert — bind berre IPv4 (restart krevst for å aktivere på nytt)
ipv6-enable-restart-required = Lagra. Restart krevst for at denne endringa skal tre i kraft.
ipv6-enable-unchanged = ipv6_enable er allereie sett til den verdien — ingen endring.

# Oppstart, bootstrap, avslutning og innboks
bootstrap-manifest-fetch-failed = Klarte ikkje å hente runtime-manifestet
bootstrap-kind-registry-hydrated = Kind-registeret vart fylt frå manifestet
bootstrap-minimal-manifest-failed = Klarte ikkje å opprette eit minimalt manifest
bootstrap-entity-state-save-failed = Klarte ikkje å lagre einingstilstandar
bootstrap-entity-state-shutdown-aborted = avslutning avbroten; runtime held fram slik at tilstanden kan lagrast ved eit seinare avslutningsforsøk
bootstrap-root-cid-shutdown-persist-failed = klarte ikkje å lagre root_cid under avslutninga
bootstrap-root-cid-shutdown-publish-succeeded = publisering av runtime_ipns under avslutninga fullført
bootstrap-root-cid-shutdown-publish-failed = publisering av runtime_ipns under avslutninga feila
bootstrap-root-cid-shutdown-publish-timeout = tidsavbrot ved publisering av runtime_ipns under avslutninga
bootstrap-endpoint-close-timeout = tidsavbrot ved lukking av endepunktet etter 5 s; tvingar avslutning
bootstrap-endpoint-close-stuck = endepunktet blir framleis halde av aktive oppgåver etter 10 s; forkastar det utan kontrollert lukking
bootstrap-envelope-delivery-failed = levering av plugin-konvolutt feila; forkastar konvolutten
bootstrap-envelope-open-failed = plugin-konvolutt: klarte ikkje å opne utboks; forkastar konvolutten
boot-minimal-manifest-not-found = Fann ingen runtime-root-CID i IPNS; opprettar eit minimalt manifest
boot-minimal-manifest-bootstrapped = Minimalt manifest oppretta
boot-kinds-overlay-no-change = Kind-overlagringa endra ikkje manifestet
boot-kinds-overlay-applied = Kind-overlagring lagt på
boot-load-manifest-for-acl-cache-failed = Klarte ikkje å laste manifestet for å fylle ACL-hurtigbufferet
boot-root-acl-loaded-from-manifest = Rotens transport-ACL lasta frå manifestet
boot-root-acl-load-failed = Klarte ikkje å laste rotens ACL frå manifestet
boot-group-loaded-into-cache = Gruppe lasta inn i hurtigbufferet
boot-group-load-failed = Klarte ikkje å laste gruppa ved oppstart
boot-root-acl-loaded-into-cache = Rotens ACL lasta inn i hurtigbufferet
boot-root-acl-load-cache-failed = Klarte ikkje å laste rotens ACL ved oppstart
boot-reconciled-owners-persist-failed = Klarte ikkje å lagre dei samordna eigarane i config.yaml
boot-reconciled-owners-published = Eigarar frå config.yaml/--owner samordna inn i manifestet ved oppstart
boot-reconciled-owners-manifest-failed = Klarte ikkje å samordne eigarar inn i manifestet ved oppstart
boot-no-root-entity = Ingen root-eining er registrert for standard konfigurasjonsrot
boot-default-root-config-populated = Standard konfigurasjonsrot sett
boot-default-root-config-populate-failed = Klarte ikkje å setje standard konfigurasjonsrot
boot-entity-load-processed = Einingsplugin lasta
bootstrap-entity-lifecycle-updated = Livssyklusen til eininga vart oppdatert i IPFS
bootstrap-entity-lifecycle-update-failed = Klarte ikkje å skrive den oppdaterte livssyklusen til eininga til IPFS
bootstrap-entity-node-shutdown-updated = Einingsnoden vart oppdatert under avslutninga
bootstrap-entity-registry-not-in-manifest = Eining finst i registeret, men ikkje i manifestet; hoppar over
plugin-outbox-drain-limit = tømmingsgrensa for plugin-utboksen er nådd; utset resten av konvoluttane
plugin-outbox-congested = plugin-utboksen er overbelasta; konvoluttar kan gå tapt om kanalen blir full
plugin-envelope-local-gate-closed = plugin-konvolutt: lokal distribusjonsport er stengd
plugin-envelope-local-timeout = plugin-konvolutt: lokal distribusjon fekk tidsavbrot
plugin-envelope-recipient-invalid = plugin-konvolutt: ugyldig mottakar-DID; hoppar over
plugin-envelope-build-failed = plugin-konvolutt: klarte ikkje å byggje melding; hoppar over
plugin-envelope-remote-limit = plugin-konvolutt: grensa for ekstern levering er nådd; konvolutten blir forkasta
scheduled-dispatch-error = feil ved planlagd distribusjon
scheduled-dispatch-manifest-writer-unavailable = planlagd distribusjon: manifest-skrivaren er ikkje klar; einingstilstanden blir ståande uendra
manifest-pin-update-failed = oppdatering av manifest-pinnen feila
bootstrap-kind-registry-fetch-log-failed = Klarte ikkje å hente kind-noden til registeret
bootstrap-entity-state-update-fetch-failed = Klarte ikkje å hente einingsnoden for tilstandsoppdatering
schedule-stale-dispatch-skipped = planlagd distribusjon hoppa over: forelda plan
schedule-random-reschedule-failed = klarte ikkje å planleggje den tilfeldige jobben på nytt
schedule-random-create-failed = klarte ikkje å opprette neste tilfeldige jobb
schedule-random-chain-stopped = kjeda for tilfeldig planlegging stoppa: erstatta av ein nyare definisjon
schedule-entity-not-found = planlagd distribusjon: eining ikkje funnen
schedule-dispatch-firing = planlagd distribusjon utløyser

bootstrap-acl-published = ACL-node publisert
bootstrap-entity-published = Entity-node publisert
bootstrap-entity-registering-prepublished = Registrerer førehandsutgjeven entity
bootstrap-entity-registry-fetch-failed = Klarte ikkje hente entity-node
bootstrap-entity-registry-kind-extends-failed = Klarte ikkje løyse opp kind-utvidingskjeda
bootstrap-entity-registry-kind-fetch-failed = Klarte ikkje hente kind-node
bootstrap-entity-registry-kind-missing = Kind ikkje funne i manifest; hoppar over entity
bootstrap-group-published = Gruppe-node publisert
bootstrap-kind-published = Kind-node publisert
bootstrap-kind-registry-extends-failed = Klarte ikkje løyse opp kind-utvidingskjeda for registeret
bootstrap-kinds-overlay-pin-update-failed = Pin/oppdatering mislukkast etter kinds-overlag
bootstrap-kinds-overlay-published = Runtime-manifest publisert etter kinds-overlag
bootstrap-kinds-tree-published = Runtime kinds-tre publisert
bootstrap-lifecycle-manifest-pin-update-failed = Pin/oppdatering mislukkast etter livssyklus-lagring
bootstrap-lifecycle-manifest-publish-failed = Klarte ikkje publisere manifest etter livssyklus-overgangar
bootstrap-lifecycle-manifest-published = Oppdatert manifest publisert etter livssyklus-overgangar
bootstrap-remote-root-pin-confirmed = Ekstern rot-pin stadfesta
bootstrap-remote-root-pin-misconfigured = Ekstern rot-pinning er feilkonfigurert
bootstrap-root-acl-published = Rot transport-ACL publisert
bootstrap-root-pin-replacement-failed = Held fram etter ein feil ved erstatning av ekstern rot-pin
bootstrap-root-pin-update-failed = Pin/oppdatering mislukkast etter oppstart
bootstrap-runtime-manifest-published = Runtime rot-manifest publisert
crud-message-rejected = CRUD-melding avvist
entity-reload-current-node-load-failed = Klarte ikkje laste gjeldande entity-node før omlasting; beheld gjeldande plugin
entity-reload-failed = Entity klarte ikkje omlaste; deaktiverast til neste omlasting
entity-reload-kind-extends-failed = Klarte ikkje løyse opp kind-utvidingskjeda under entity-omlasting
entity-reload-kind-fetch-failed = Klarte ikkje hente kind-node under entity-omlasting
entity-reload-kind-lookup-failed = Klarte ikkje laste manifest for kind-oppslag under entity-omlasting
entity-reload-kind-missing = Kind ikkje funne i manifest; kan ikkje laste entity på nytt
entity-reload-manifest-state-update-failed = Klarte ikkje oppdatere manifest med gjeldande tilstand før omlasting; beheld gjeldande plugin
entity-reload-skipped = Entity-omlasting hoppa over fordi omlastings-porten er lukka
entity-reload-started = Entity-omlasting starta
entity-reload-state-persist-failed = Klarte ikkje persistere gjeldande tilstand før omlasting; beheld gjeldande plugin
entity-reload-state-produced-failed = Klarte ikkje persistere tilstand produsert under omlasting
entity-reloaded-manifest-update-failed = Klarte ikkje oppdatere omlasta entity i manifest
entity-reloaded-manifest-updated = Omlasta entity oppdatert i manifest
inbox-message-rejected = Innboks-melding avvist
ma-create-entity-already-exists = ma_create_entity: entity eksisterer allereie; beheld gjeldande entity
ma-create-entity-invalid-behaviour = ma_create_entity: ugyldig behaviour-referanse; hoppa over
ma-create-entity-kind-missing = ma_create_entity: kind ikkje i registeret; hoppa over
plugin-envelope-create-requests-ignored = Plugin-konvolutt: opprettingsførespurnader ignorert utan bieffekt-kontekst
plugin-envelope-local-dispatch-failed = Plugin-konvolutt: lokal utsending mislukkast
plugin-envelope-local-dispatch-finish = Plugin-konvolutt: lokal utsending fullført
plugin-envelope-local-dispatch-start = Plugin-konvolutt: lokal utsending starta
plugin-envelope-local-recipient-unknown = Plugin-konvolutt: ukjend lokal mottakar; hoppa over
