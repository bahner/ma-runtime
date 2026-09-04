# ma-runtime – Føroyskt
lang-name = Føroyskt

own-did-published = Egið DID-skjal birt á IPNS
own-did-publish-failed = Fáast ikki at birta egið DID-skjal
own-did-publish-timeout = Birting av egnum DID-skjali gjørdi timeout eftir 2 minuttir
started = ma runtime startað
shutdown-requested = Stansa búgvin
closing-endpoint = Lokkar iroh-endastøð...
shutdown-complete = Stansing liðug
status-listening = Støðutenar hloyðir
ipfs-message-rejected = IPFS-boð avvísað
ctrlc-handler-failed = Ctrl-C-handsamari brast
node-connected = Knútur knýttur til protokol
received-encrypted-ma-msg = Dulkódað ma-boð móttikið á /ma/ipfs/0.0.1
ping-received = :ping móttikið, sendi :pong
did-publish-request-received = Bøn um birting av DID-skjali móttøkin
document-published = Skjal birt
did-publish-cid-reply-sent = CID-svar sent fyri DID-birting
did-publish-resolve-failed = Ikki mett at finna sendaran til at skaffa ipfs-publish-svar
ipfs-store-request-received = IPFS-goymslubøn móttøkin
ipfs-stored = Innihald goymt á IPFS
ipfs-store-cid-reply-sent = CID-svar sent
ipfs-store-resolve-failed = Ikki mett at finna sendaran til at skaffa ipfs-store-svar

# Eindir send
bootstrap-complete = Bootstrap liðugt
entity-loaded = Eindarplugin lesin inn
entity-load-failed = Innlesing av eindarplugin brast
root-list-entities = #root: lista eindir
entity-created = Eindir stovnað
entity-reloaded = Eindarplugin lesin inn av nýggjum
entity-deleted = Eindir strikað
entity-states-saving = Goymi eindarstøður til IPFS
entity-state-saving = Goymi eindarstøðu
entity-state-saved = Eindarstøða goymd
entity-state-empty = Plugin skilaði tómari støðu, goymsl sleppt
entity-states-saved = Eindarstøður goymd

# Fyrsta keyrsla / sjálvvirk uppsetning

# Eivirði
runtime-claimed = Runtime skráð.

# Vernd rótareimindir
refuse-delete-root = Neitari ákveðið at strika kravd rótareindir
runtime-claim-persisted = Eigari skrivað til stillingar.


# Namespace creation (:create)
crud-message-received = CRUD-boð móttikið
crud-acl-updated = Root-transport-ACL uppfært

# CRUD validation errors
cidv1-required = gildi skal vera eitt reint CIDv1 (byrjar við 'b'; CIDv0 'Qm…' ikki góðteke)
config-key-protected = config-lykillinn '%key%' er verndaður
config-key-no-delete = daemon-config-lykillinn '%key%' kann ikki slettast
config-key-not-manifest = config-lykillinn '%key%' er ikki ein kendur manifest-config-lykill
wrong-crud-protocol = rang CRUD-protokoll: %type%
entity-name-invalid = entity-navnið skal vera prentbært UTF-8
reserved-entity-name = entity-navnið '%name%' er fyrirvara
genesis-kind-owner-only = Bara ein runtime-eigari kann stovna eina entity av slagnum genesis

# IPv6 config
ipv6-enabled = IPv6 virkjað — bindar bæði IPv4 og IPv6
ipv6-disabled = IPv6 er óvirkjað — bindir bert IPv4 (restart krevst fyri at virkja aftur)
ipv6-enable-restart-required = Goymst. Restart krevst, fyri at broytingin tekur verkað.
ipv6-enable-unchanged = ipv6_enable er longu sett til tað virðið — ongar broytingar.

boot-default-root-config-populate-failed = Fylling av vanligari rótarkonfiguration miseydnaðist
boot-default-root-config-populated = Vanlig rótarkonfiguration fyllst
boot-entity-load-processed = Viðgerðir hjá einingum hlaðnar
boot-group-load-failed = Hleðsla av group við uppræsing miseydnaðist
boot-group-loaded-into-cache = Group hlaðin í skyndiminni
boot-kinds-overlay-applied = Kinds-yvirlag brúkt
boot-kinds-overlay-no-change = Kinds-yvirlag broytti ikki manifest
boot-load-manifest-for-acl-cache-failed = Hleðsla av manifest til at fylla ACL-skyndiminni miseydnaðist
boot-minimal-manifest-bootstrapped = Minimalt manifest initialisert
boot-minimal-manifest-not-found = Rót-CID av umhvørvi funnin ikki í IPNS; initialiserar minimalt manifest
boot-no-root-entity = Einki rót-entity skrásett hjá vanligari rótarkonfiguration
boot-reconciled-owners-manifest-failed = Samanlegging av eigarum í manifest við uppræsing miseydnaðist
boot-reconciled-owners-persist-failed = Goymur av samanlagðum eigarum í config.yaml miseydnaðist
boot-reconciled-owners-published = Eigarar samanlagðir úr config.yaml/--owner til manifest við uppræsing
boot-root-acl-load-cache-failed = Hleðsla av rót-ACL við uppræsing miseydnaðist
boot-root-acl-load-failed = Hleðsla av rót-ACL úr manifest miseydnaðist
boot-root-acl-loaded-from-manifest = Rót-ferðslu-ACL hlaðin úr manifest
boot-root-acl-loaded-into-cache = Rót-ACL hlaðin í skyndiminni
bootstrap-acl-published = ACL-hnútur birtur
bootstrap-endpoint-close-stuck = Endapunktur er enn í gangi eftir 10 s; tvangslokar
bootstrap-endpoint-close-timeout = Lokað á endapunkt eftir 5 s; tvangsstoppar
bootstrap-entity-lifecycle-update-failed = Skriving av uppfærðum lívstíð hjá einingu til IPFS miseydnaðist
bootstrap-entity-lifecycle-updated = Lívstíður hjá einingu uppfærður í IPFS
bootstrap-entity-node-shutdown-updated = Hnútur hjá einingu uppfærður við stoppan
bootstrap-entity-published = Hnútur hjá einingu birtur
bootstrap-entity-registering-prepublished = Skrásettur fyribirtingar eining
bootstrap-entity-registry-fetch-failed = Heinting av hnút hjá einingu miseydnaðist
bootstrap-entity-registry-kind-extends-failed = Uppgerð av kinds-útbreiðslukeðju miseydnaðist
bootstrap-entity-registry-kind-fetch-failed = Heinting av kinds-hnút miseydnaðist
bootstrap-entity-registry-kind-missing = Kind funnin ikki í manifest; eining slept yvir
bootstrap-entity-registry-not-in-manifest = Eining í skráseting men ikki í manifest; slept yvir
bootstrap-entity-state-save-failed = Goymur av støðum hjá einingum miseydnaðist
bootstrap-entity-state-shutdown-aborted = Stoppan avlýst; umhvørvi verður virkt til at goyma støðu við næstu stoppan
bootstrap-entity-state-update-fetch-failed = Heinting av hnút hjá einingu til uppfæring av støðu miseydnaðist
bootstrap-envelope-delivery-failed = Afhending av umslagi frá viðgerð miseydnaðist; umslag kastað
bootstrap-envelope-open-failed = Umslag frá viðgerð: opna av útgangspósthúsi miseydnaðist; umslag kastað
bootstrap-group-published = Hnútur hjá group birtur
bootstrap-kind-published = Kinds-hnútur birtur
bootstrap-kind-registry-extends-failed = Uppgerð av kinds-útbreiðslukeðju fyri skrásetingar miseydnaðist
bootstrap-kind-registry-fetch-log-failed = Heinting av kinds-hnút fyri skrásetingar miseydnaðist
bootstrap-kind-registry-hydrated = Kinds-skrásetingin fyllst úr manifest
bootstrap-kinds-overlay-pin-update-failed = Pinna/uppfæra miseydnaðist eftir kinds-yvirlag
bootstrap-kinds-overlay-published = Manifest umhvørvis birt eftir kinds-yvirlag
bootstrap-kinds-tree-published = Kinds-træ umhvørvis birt
bootstrap-lifecycle-manifest-pin-update-failed = Pinna/uppfæra miseydnaðist eftir goymur av lívstíð
bootstrap-lifecycle-manifest-publish-failed = Birting av manifest eftir lívstíðsbroyting miseydnaðist
bootstrap-lifecycle-manifest-published = Uppfært manifest birt eftir lívstíðsbroyting
bootstrap-manifest-fetch-failed = Heinting av manifest umhvørvis miseydnaðist
bootstrap-minimal-manifest-failed = Initialisering av minimalt manifest miseydnaðist
bootstrap-remote-root-pin-confirmed = Fjarlægt rót-pinna staðfest
bootstrap-remote-root-pin-misconfigured = Fjarlægt rót-pinna er rangt stillt
bootstrap-root-acl-published = Rót-ferðslu-ACL birt
bootstrap-root-cid-shutdown-persist-failed = Goymur av root_cid við stoppan miseydnaðist
bootstrap-root-cid-shutdown-publish-failed = Birting av runtime_ipns við stoppan miseydnaðist
bootstrap-root-cid-shutdown-publish-succeeded = Birting av runtime_ipns við stoppan eydnaðist
bootstrap-root-cid-shutdown-publish-timeout = Birting av runtime_ipns við stoppan tók ov langan tíð
bootstrap-root-pin-replacement-failed = Heldur fram eftir villa í útskiftingi av fjarlægu rót-pinni
bootstrap-root-pin-update-failed = Pinna/uppfæra miseydnaðist eftir uppræsing
bootstrap-runtime-manifest-published = Rót-manifest umhvørvis birt
crud-message-rejected = CRUD-boð avvist
entity-reload-current-node-load-failed = Heinting av núgvurandum hnút hjá einingu fyri endurhleðslu miseydnaðist; núgvurandi viðgerð goymд
entity-reload-failed = Endurhleðsla hjá einingu miseydnaðist; óvirk til næstu endurhleðslu
entity-reload-kind-extends-failed = Uppgerð av kinds-útbreiðslukeðju við endurhleðslu einingar miseydnaðist
entity-reload-kind-fetch-failed = Heinting av kinds-hnút við endurhleðslu einingar miseydnaðist
entity-reload-kind-lookup-failed = Heinting av manifest til kinds-leiting við endurhleðslu einingar miseydnaðist
entity-reload-kind-missing = Kind funnin ikki í manifest; eining kann ikki endurhlaðast
entity-reload-manifest-state-update-failed = Uppfæring av manifest við núgvurandi støðu fyri endurhleðslu miseydnaðist; núgvurandi viðgerð goymð
entity-reload-skipped = Endurhleðsla hjá einingu slept yvir tí at endurhleðsluglogg er lokað
entity-reload-started = Endurhleðsla hjá einingu byrjaði
entity-reload-state-persist-failed = Goymur av núgvurandi støðu fyri endurhleðslu miseydnaðist; núgvurandi viðgerð goymð
entity-reload-state-produced-failed = Goymur av støðu framleiðdari við endurhleðslu miseydnaðist
entity-reloaded-manifest-update-failed = Uppfæring av endurhladdari einingu í manifest miseydnaðist
entity-reloaded-manifest-updated = Endurhlaðin eining uppfærð í manifest
inbox-message-rejected = Boð úr innkomandi avvist
ma-create-entity-already-exists = ma_create_entity: eining er longu til; núgvurandi eining goymð
ma-create-entity-invalid-behaviour = ma_create_entity: ógildug áhvíslan til hegðan; slept yvir
ma-create-entity-kind-missing = ma_create_entity: kinds vantar í skrásetingini; slept yvir
manifest-pin-update-failed = Manifest pin_update miseydnaðist
plugin-envelope-build-failed = Umslag frá viðgerð: uppgerð av boðum miseydnaðist; slept yvir
plugin-envelope-create-requests-ignored = Umslag frá viðgerð: beiðnir um stovnan hundsaðar uttan hliðarverkan
plugin-envelope-local-dispatch-failed = Umslag frá viðgerð: staðbundin sending miseydnaðist
plugin-envelope-local-dispatch-finish = Umslag frá viðgerð: staðbundin sending lokið
plugin-envelope-local-dispatch-start = Umslag frá viðgerð: staðbundin sending byrjaði
plugin-envelope-local-gate-closed = Umslag frá viðgerð: staðbundin sendingarhlið er lokað
plugin-envelope-local-recipient-unknown = Umslag frá viðgerð: ókenndur staðbundinn móttakari; slept yvir
plugin-envelope-local-timeout = Umslag frá viðgerð: staðbundin sending tók ov langan tíð
plugin-envelope-recipient-invalid = Umslag frá viðgerð: ógildugur DID hjá móttakara; slept yvir
plugin-envelope-remote-limit = Umslag frá viðgerð: fjarlægar afhendingarmørkur náð; umslag kastað
plugin-outbox-congested = Útgangspósthús hjá viðgerð yvirhlaðið; umslög kunnu kasast, um kannalin fyllast
plugin-outbox-drain-limit = Tømingarútreiðsl hjá útgangspósthúsi viðgerðar útrásin; eftirstandandi umslög seinkað
schedule-dispatch-firing = Ætlað sending í gongd
schedule-entity-not-found = Ætlað sending: eining ikki funnin
schedule-random-chain-stopped = Tilvildarlig ætlað keðja stoppað: skipt út við nýggjari definitión
schedule-random-create-failed = Stovning av næsta tilvildarliga verkefni miseydnaðist
schedule-random-reschedule-failed = Endurætlan av tilvildarligum verkefni miseydnaðist
schedule-stale-dispatch-skipped = Ætlað sending slept yvir: gámaður ætlan
scheduled-dispatch-error = Villa við ætlaðari sendingu
scheduled-dispatch-manifest-writer-unavailable = Ætlað sending: manifest-skrivar er ikki klárt; støðan hjá einingu bíðar
