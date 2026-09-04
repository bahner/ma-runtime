# ma-runtime – Čeština
lang-name = Čeština

own-did-published = Vlastní DID dokument publikován na IPNS
own-did-publish-failed = Publikování vlastního DID dokumentu selhalo
own-did-publish-timeout = Publikování vlastního DID dokumentu vypršelo po 2 minutách
started = ma runtime spuštěn
shutdown-requested = Vypnutí požadováno
closing-endpoint = Uzavírání iroh endpointu...
shutdown-complete = Vypnutí dokončeno
status-listening = Stavový server naslouchá
ipfs-message-rejected = IPFS zpráva odmítnuta
ctrlc-handler-failed = Obsluha Ctrl-C selhala
node-connected = Uzel připojen k protokolu
received-encrypted-ma-msg = Přijata zašifrovaná ma zpráva na /ma/ipfs/0.0.1
ping-received = Přijat :ping, odesílám :pong
did-publish-request-received = Přijat požadavek na publikování DID dokumentu
document-published = Dokument publikován
did-publish-cid-reply-sent = Odeslána CID odpověď pro publikování DID
did-publish-resolve-failed = Nelze přeložit odesílatele pro doručení odpovědi ipfs-publish
ipfs-store-request-received = Přijat požadavek na uložení IPFS
ipfs-stored = Obsah uložen na IPFS
ipfs-store-cid-reply-sent = CID odpověď odeslána
ipfs-store-resolve-failed = Nelze přeložit odesílatele pro doručení odpovědi ipfs-store

# Odeslání entit
bootstrap-complete = Bootstrap dokončen
entity-loaded = Plugin entity načten
entity-load-failed = Načtení pluginu entity selhalo
root-list-entities = #root: seznam entit
entity-created = Entita vytvořena
entity-reloaded = Plugin entity znovu načten
entity-deleted = Entita smazána
entity-states-saving = Ukládání stavů entit do IPFS
entity-state-saving = Ukládání stavu entity
entity-state-saved = Stav entity uložen
entity-state-empty = Plugin vrátil prázdný stav, ukládání přeskočeno
entity-states-saved = Stavy entit uloženy

# První spuštění / auto-init

# Vlastnictví
runtime-claimed = Runtime registrován.

# Chráněné kořenové prvky
refuse-delete-root = Důrazně odmítám smazat požadovaný kořenový prvek
runtime-claim-persisted = Vlastník zapsán do konfigurace.


# Namespace creation (:create)
crud-message-received = Přijata zpráva CRUD
crud-acl-updated = Kořenový transportní ACL aktualizován

# CRUD validation errors
cidv1-required = hodnota musí být holý CIDv1 (začíná 'b'; CIDv0 'Qm…' není přijat)
config-key-protected = konfigurační klíč '%key%' je chráněný
config-key-no-delete = konfigurační klíč '%key%' démona nelze smazat
config-key-not-manifest = konfigurační klíč '%key%' není známým klíčem manifest config
wrong-crud-protocol = nesprávný protokol CRUD: %type%
entity-name-invalid = název entity musí být tisknutelné UTF-8
reserved-entity-name = název entity '%name%' je vyhrazený
genesis-kind-owner-only = Entitu typu genesis smí vytvořit pouze vlastník runtime

# IPv6 config
ipv6-enabled = IPv6 povoleno — naslouchá na IPv4 i IPv6
ipv6-disabled = IPv6 je zakázáno — váže se pouze IPv4 (pro opětovné povolení je nutný restart)
ipv6-enable-restart-required = Uloženo. Pro uplatnění této změny je nutný restart.
ipv6-enable-unchanged = ipv6_enable je již nastaveno na tuto hodnotu — žádná změna.

boot-default-root-config-populate-failed = Nepodařilo se naplnit výchozí kořen konfigurace
boot-default-root-config-populated = Výchozí kořen konfigurace naplněn
boot-entity-load-processed = Zásuvné moduly entit načteny
boot-group-load-failed = Nepodařilo se načíst skupinu při spuštění
boot-group-loaded-into-cache = Skupina načtena do mezipaměti
boot-kinds-overlay-applied = Překrytí kinds aplikováno
boot-kinds-overlay-no-change = Překrytí kinds neprovedlo žádné změny v manifestu
boot-load-manifest-for-acl-cache-failed = Nepodařilo se načíst manifest pro naplnění mezipaměti ACL
boot-minimal-manifest-bootstrapped = Minimální manifest inicializován
boot-minimal-manifest-not-found = V IPNS nebylo nalezeno kořenové CID runtime; inicializace minimálního manifestu
boot-no-root-entity = Není registrována žádná kořenová entita pro výchozí kořen konfigurace
boot-reconciled-owners-manifest-failed = Nepodařilo se odsouhlasit vlastníky v manifestu při spuštění
boot-reconciled-owners-persist-failed = Nepodařilo se uložit odsouhlasené vlastníky do config.yaml
boot-reconciled-owners-published = Vlastníci odsouhlaseni z config.yaml/--owner do manifestu při spuštění
boot-root-acl-load-cache-failed = Nepodařilo se načíst kořenové ACL při spuštění
boot-root-acl-load-failed = Nepodařilo se načíst kořenové ACL z manifestu
boot-root-acl-loaded-from-manifest = Kořenové přenosové ACL načteno z manifestu
boot-root-acl-loaded-into-cache = Kořenové ACL načteno do mezipaměti
bootstrap-acl-published = Uzel ACL publikován
bootstrap-endpoint-close-stuck = Endpoint je stále držen probíhajícími úlohami po 10 s; vynucené uzavření
bootstrap-endpoint-close-timeout = Uzavření endpointu vypršelo po 5 s; vynucené ukončení
bootstrap-entity-lifecycle-update-failed = Nepodařilo se zapsat aktualizovaný životní cyklus entity do IPFS
bootstrap-entity-lifecycle-updated = Životní cyklus entity aktualizován v IPFS
bootstrap-entity-node-shutdown-updated = Uzel entity aktualizován při vypnutí
bootstrap-entity-published = Uzel entity publikován
bootstrap-entity-registering-prepublished = Registrace předem publikované entity
bootstrap-entity-registry-fetch-failed = Nepodařilo se načíst uzel entity
bootstrap-entity-registry-kind-extends-failed = Nepodařilo se přeložit řetězec rozšíření kind
bootstrap-entity-registry-kind-fetch-failed = Nepodařilo se načíst uzel kind
bootstrap-entity-registry-kind-missing = Kind nenalezen v manifestu; entita přeskočena
bootstrap-entity-registry-not-in-manifest = Entita v registru ale ne v manifestu, přeskočena
bootstrap-entity-state-save-failed = Nepodařilo se uložit stavy entit
bootstrap-entity-state-shutdown-aborted = Vypnutí přerušeno; runtime zůstává aktivní, aby bylo možné uložit stav při dalším pokusu o vypnutí
bootstrap-entity-state-update-fetch-failed = Nepodařilo se načíst uzel entity pro aktualizaci stavu
bootstrap-envelope-delivery-failed = Doručení obálky zásuvného modulu selhalo; obálka zahozena
bootstrap-envelope-open-failed = Obálka zásuvného modulu: otevření odchozí pošty selhalo; obálka zahozena
bootstrap-group-published = Uzel skupiny publikován
bootstrap-kind-published = Uzel kind publikován
bootstrap-kind-registry-extends-failed = Nepodařilo se přeložit řetězec rozšíření kind pro registr
bootstrap-kind-registry-fetch-log-failed = Nepodařilo se načíst uzel kind pro registr
bootstrap-kind-registry-hydrated = Registr kinds naplněn z manifestu
bootstrap-kinds-overlay-pin-update-failed = Připnutí/aktualizace selhalo po překrytí kinds
bootstrap-kinds-overlay-published = Manifest runtime publikován po překrytí kinds
bootstrap-kinds-tree-published = Strom kinds runtime publikován
bootstrap-lifecycle-manifest-pin-update-failed = Připnutí/aktualizace selhalo po perzistenci životního cyklu
bootstrap-lifecycle-manifest-publish-failed = Nepodařilo se publikovat manifest po přechodech životního cyklu
bootstrap-lifecycle-manifest-published = Aktualizovaný manifest publikován po přechodech životního cyklu
bootstrap-manifest-fetch-failed = Nepodařilo se načíst manifest runtime
bootstrap-minimal-manifest-failed = Nepodařilo se inicializovat minimální manifest
bootstrap-remote-root-pin-confirmed = Vzdálené kořenové připnutí potvrzeno
bootstrap-remote-root-pin-misconfigured = Vzdálené kořenové připínání je špatně nakonfigurováno
bootstrap-root-acl-published = Kořenové přenosové ACL publikováno
bootstrap-root-cid-shutdown-persist-failed = Nepodařilo se perzistovat root_cid při vypnutí
bootstrap-root-cid-shutdown-publish-failed = Publikace runtime_ipns při vypnutí selhala
bootstrap-root-cid-shutdown-publish-succeeded = Publikace runtime_ipns při vypnutí proběhla úspěšně
bootstrap-root-cid-shutdown-publish-timeout = Publikace runtime_ipns při vypnutí vypršela
bootstrap-root-pin-replacement-failed = Pokračování po chybě nahrazení vzdáleného kořenového připnutí
bootstrap-root-pin-update-failed = Připnutí/aktualizace selhalo po bootstrapu
bootstrap-runtime-manifest-published = Kořenový manifest runtime publikován
crud-message-rejected = Zpráva CRUD odmítnuta
entity-reload-current-node-load-failed = Nepodařilo se načíst aktuální uzel entity před znovunačtením; aktuální zásuvný modul zachován
entity-reload-failed = Znovunačtení entity selhalo; deaktivována do dalšího znovunačtení
entity-reload-kind-extends-failed = Nepodařilo se přeložit řetězec rozšíření kind při znovunačtení entity
entity-reload-kind-fetch-failed = Nepodařilo se načíst uzel kind při znovunačtení entity
entity-reload-kind-lookup-failed = Nepodařilo se načíst manifest pro vyhledávání kind při znovunačtení entity
entity-reload-kind-missing = Kind nenalezen v manifestu; entitu nelze znovu načíst
entity-reload-manifest-state-update-failed = Nepodařilo se aktualizovat manifest s aktuálním stavem před znovunačtením; aktuální zásuvný modul zachován
entity-reload-skipped = Znovunačtení entity přeskočeno, protože brána znovunačtení je zavřena
entity-reload-started = Znovunačtení entity zahájeno
entity-reload-state-persist-failed = Nepodařilo se perzistovat aktuální stav před znovunačtením; aktuální zásuvný modul zachován
entity-reload-state-produced-failed = Nepodařilo se perzistovat stav vzniklý při znovunačtení
entity-reloaded-manifest-update-failed = Nepodařilo se aktualizovat znovu načtenou entitu v manifestu
entity-reloaded-manifest-updated = Znovu načtená entita aktualizována v manifestu
inbox-message-rejected = Zpráva z doručené pošty odmítnuta
ma-create-entity-already-exists = ma_create_entity: entita již existuje; aktuální entita zachována
ma-create-entity-invalid-behaviour = ma_create_entity: neplatný odkaz na behaviour; přeskočeno
ma-create-entity-kind-missing = ma_create_entity: kind chybí v registru; přeskočeno
manifest-pin-update-failed = pin_update manifestu selhalo
plugin-envelope-build-failed = Obálka zásuvného modulu: sestavení zprávy selhalo; přeskočeno
plugin-envelope-create-requests-ignored = Obálka zásuvného modulu: požadavky na vytvoření ignorovány bez kontextu vedlejšího efektu
plugin-envelope-local-dispatch-failed = Obálka zásuvného modulu: místní odesílání selhalo
plugin-envelope-local-dispatch-finish = Obálka zásuvného modulu: místní odesílání dokončeno
plugin-envelope-local-dispatch-start = Obálka zásuvného modulu: místní odesílání zahájeno
plugin-envelope-local-gate-closed = Obálka zásuvného modulu: brána místního odesílání zavřena
plugin-envelope-local-recipient-unknown = Obálka zásuvného modulu: neznámý místní příjemce; přeskočeno
plugin-envelope-local-timeout = Obálka zásuvného modulu: místní odesílání vypršelo
plugin-envelope-recipient-invalid = Obálka zásuvného modulu: neplatné DID příjemce; přeskočeno
plugin-envelope-remote-limit = Obálka zásuvného modulu: dosažen limit vzdáleného doručování; obálka zahozena
plugin-outbox-congested = Odchozí pošta zásuvného modulu přetížena; obálky mohou být zahozeny, pokud se kanál zaplní
plugin-outbox-drain-limit = Rozpočet vyprazdňování odchozí pošty zásuvného modulu vyčerpán; zbývající obálky odloženy
schedule-dispatch-firing = Plánované odesílání probíhá
schedule-entity-not-found = Plánované odesílání: entita nenalezena
schedule-random-chain-stopped = Náhodný plánovaný řetězec zastaven: nahrazen novější definicí
schedule-random-create-failed = Nepodařilo se vytvořit další náhodnou úlohu
schedule-random-reschedule-failed = Nepodařilo se přeplánovat náhodnou úlohu
schedule-stale-dispatch-skipped = Plánované odesílání přeskočeno: zastaralý plán
scheduled-dispatch-error = Chyba při plánovaném odesílání
scheduled-dispatch-manifest-writer-unavailable = Plánované odesílání: zapisovač manifestu není připraven; stav entity čeká
