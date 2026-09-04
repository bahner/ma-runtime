# ma-runtime – Italiano
lang-name = Italiano

own-did-published = Documento DID proprio pubblicato su IPNS
own-did-publish-failed = Pubblicazione del documento DID proprio fallita
own-did-publish-timeout = Pubblicazione del documento DID proprio scaduta dopo 2 minuti
started = ma runtime avviato
shutdown-requested = Spegnimento richiesto
closing-endpoint = Chiusura dell'endpoint iroh...
shutdown-complete = Spegnimento completato
status-listening = Server di stato in ascolto
ipfs-message-rejected = Messaggio IPFS rifiutato
ctrlc-handler-failed = Handler Ctrl-C fallito
node-connected = Nodo connesso al protocollo
received-encrypted-ma-msg = Messaggio ma cifrato ricevuto su /ma/ipfs/0.0.1
ping-received = :ping ricevuto, invio :pong
did-publish-request-received = Richiesta di pubblicazione documento DID ricevuta
document-published = Documento pubblicato
did-publish-cid-reply-sent = Risposta CID inviata per la pubblicazione DID
did-publish-resolve-failed = Impossibile risolvere il mittente per consegnare la risposta ipfs-publish
ipfs-store-request-received = Richiesta di archiviazione IPFS ricevuta
ipfs-stored = Contenuto archiviato su IPFS
ipfs-store-cid-reply-sent = Risposta CID inviata
ipfs-store-resolve-failed = Impossibile risolvere il mittente per consegnare la risposta ipfs-store

# Dispatch delle entità
bootstrap-complete = Bootstrap completato
entity-loaded = Plugin entità caricato
entity-load-failed = Caricamento del plugin entità fallito
root-list-entities = #root: elenca entità
entity-created = Entità creata
entity-reloaded = Plugin entità ricaricato
entity-deleted = Entità eliminata
entity-states-saving = Salvataggio degli stati delle entità su IPFS
entity-state-saving = Salvataggio dello stato dell'entità
entity-state-saved = Stato dell'entità salvato
entity-state-empty = Il plugin ha restituito uno stato vuoto, salvataggio ignorato
entity-states-saved = Stati delle entità salvati

# Primo avvio / auto-init

# Proprietà
runtime-claimed = Runtime registrato.

# Elementi radice protetti
refuse-delete-root = Mi rifiuto categoricamente di eliminare un elemento radice richiesto
runtime-claim-persisted = Proprietario scritto nella configurazione.


# Namespace creation (:create)
crud-message-received = Messaggio CRUD ricevuto
crud-acl-updated = ACL di trasporto radice aggiornata

# CRUD validation errors
cidv1-required = il valore deve essere un CIDv1 puro (inizia con 'b'; CIDv0 'Qm…' non accettato)
config-key-protected = la chiave config '%key%' è protetta
config-key-no-delete = la chiave config '%key%' del daemon non può essere eliminata
config-key-not-manifest = la chiave config '%key%' non è una chiave manifest config nota
wrong-crud-protocol = protocollo CRUD errato: %type%
entity-name-invalid = il nome entity deve essere UTF-8 stampabile
reserved-entity-name = il nome entity '%name%' è riservato
genesis-kind-owner-only = Solo un proprietario del runtime può creare un entity di tipo genesis

# IPv6 config
ipv6-enabled = IPv6 abilitato — in ascolto su IPv4 e IPv6
ipv6-disabled = IPv6 disabilitato — si associa solo IPv4 (restart necessario per riabilitarlo)
ipv6-enable-restart-required = Salvato. È necessario un restart affinché questa modifica abbia effetto.
ipv6-enable-unchanged = ipv6_enable è già impostato su quel valore — nessuna modifica.

boot-default-root-config-populate-failed = Impossibile popolare la radice di configurazione predefinita
boot-default-root-config-populated = Radice di configurazione predefinita popolata
boot-entity-load-processed = Plugin entità caricati
boot-group-load-failed = Impossibile caricare il gruppo all'avvio
boot-group-loaded-into-cache = Gruppo caricato nella cache
boot-kinds-overlay-applied = Overlay dei kinds applicato
boot-kinds-overlay-no-change = L'overlay dei kinds non ha apportato modifiche al manifest
boot-load-manifest-for-acl-cache-failed = Impossibile caricare il manifest per il popolamento della cache ACL
boot-minimal-manifest-bootstrapped = Manifest minimo inizializzato
boot-minimal-manifest-not-found = Nessun CID radice di runtime trovato in IPNS; inizializzazione manifest minimo
boot-no-root-entity = Nessuna entità radice registrata per la radice di configurazione predefinita
boot-reconciled-owners-manifest-failed = Impossibile riconciliare i proprietari nel manifest all'avvio
boot-reconciled-owners-persist-failed = Impossibile persistere i proprietari riconciliati in config.yaml
boot-reconciled-owners-published = Proprietari riconciliati da config.yaml/--owner nel manifest all'avvio
boot-root-acl-load-cache-failed = Impossibile caricare l'ACL radice all'avvio
boot-root-acl-load-failed = Impossibile caricare l'ACL radice dal manifest
boot-root-acl-loaded-from-manifest = ACL di trasporto radice caricata dal manifest
boot-root-acl-loaded-into-cache = ACL radice caricata nella cache
bootstrap-acl-published = Nodo ACL pubblicato
bootstrap-endpoint-close-stuck = L'endpoint è ancora trattenuto da task in esecuzione dopo 10 s; chiusura forzata
bootstrap-endpoint-close-timeout = Chiusura endpoint scaduta dopo 5 s; uscita forzata
bootstrap-entity-lifecycle-update-failed = Impossibile scrivere il ciclo di vita dell'entità aggiornato in IPFS
bootstrap-entity-lifecycle-updated = Ciclo di vita dell'entità aggiornato in IPFS
bootstrap-entity-node-shutdown-updated = Nodo entità aggiornato allo spegnimento
bootstrap-entity-published = Nodo entità pubblicato
bootstrap-entity-registering-prepublished = Registrazione entità pre-pubblicata
bootstrap-entity-registry-fetch-failed = Impossibile recuperare il nodo entità
bootstrap-entity-registry-kind-extends-failed = Impossibile risolvere la catena di estensione del kind
bootstrap-entity-registry-kind-fetch-failed = Impossibile recuperare il nodo kind
bootstrap-entity-registry-kind-missing = Kind non trovato nel manifest; entità saltata
bootstrap-entity-registry-not-in-manifest = Entità nel registro ma non nel manifest, saltata
bootstrap-entity-state-save-failed = Impossibile salvare gli stati delle entità
bootstrap-entity-state-shutdown-aborted = Spegnimento interrotto; il runtime rimane attivo per consentire il salvataggio dello stato al prossimo tentativo
bootstrap-entity-state-update-fetch-failed = Impossibile recuperare il nodo entità per l'aggiornamento dello stato
bootstrap-envelope-delivery-failed = Consegna envelope plugin fallita; envelope scartato
bootstrap-envelope-open-failed = Envelope plugin: apertura outbox fallita; envelope scartato
bootstrap-group-published = Nodo gruppo pubblicato
bootstrap-kind-published = Nodo kind pubblicato
bootstrap-kind-registry-extends-failed = Impossibile risolvere la catena di estensione del kind per il registro
bootstrap-kind-registry-fetch-log-failed = Impossibile recuperare il nodo kind per il registro
bootstrap-kind-registry-hydrated = Registro dei kinds idratato dal manifest
bootstrap-kinds-overlay-pin-update-failed = Pin/aggiornamento fallito dopo l'overlay dei kinds
bootstrap-kinds-overlay-published = Manifest di runtime pubblicato dopo l'overlay dei kinds
bootstrap-kinds-tree-published = Albero dei kinds di runtime pubblicato
bootstrap-lifecycle-manifest-pin-update-failed = Pin/aggiornamento fallito dopo la persistenza del ciclo di vita
bootstrap-lifecycle-manifest-publish-failed = Impossibile pubblicare il manifest dopo le transizioni del ciclo di vita
bootstrap-lifecycle-manifest-published = Manifest aggiornato pubblicato dopo le transizioni del ciclo di vita
bootstrap-manifest-fetch-failed = Impossibile recuperare il manifest di runtime
bootstrap-minimal-manifest-failed = Impossibile inizializzare il manifest minimo
bootstrap-remote-root-pin-confirmed = Pin radice remoto confermato
bootstrap-remote-root-pin-misconfigured = Il pinning radice remoto è configurato in modo errato
bootstrap-root-acl-published = ACL di trasporto radice pubblicata
bootstrap-root-cid-shutdown-persist-failed = Impossibile persistere root_cid durante lo spegnimento
bootstrap-root-cid-shutdown-publish-failed = Pubblicazione runtime_ipns durante lo spegnimento fallita
bootstrap-root-cid-shutdown-publish-succeeded = Pubblicazione runtime_ipns durante lo spegnimento riuscita
bootstrap-root-cid-shutdown-publish-timeout = Pubblicazione runtime_ipns durante lo spegnimento scaduta
bootstrap-root-pin-replacement-failed = Continuazione dopo un errore di sostituzione del pin radice remoto
bootstrap-root-pin-update-failed = Pin/aggiornamento fallito dopo il bootstrap
bootstrap-runtime-manifest-published = Manifest radice di runtime pubblicato
crud-message-rejected = Messaggio CRUD rifiutato
entity-reload-current-node-load-failed = Impossibile caricare il nodo entità corrente prima del ricaricamento; plugin corrente mantenuto
entity-reload-failed = Impossibile ricaricare l'entità; disabilitata fino al prossimo ricaricamento
entity-reload-kind-extends-failed = Impossibile risolvere la catena di estensione del kind durante il ricaricamento
entity-reload-kind-fetch-failed = Impossibile recuperare il nodo kind durante il ricaricamento
entity-reload-kind-lookup-failed = Impossibile caricare il manifest per la ricerca del kind durante il ricaricamento
entity-reload-kind-missing = Kind non trovato nel manifest; impossibile ricaricare l'entità
entity-reload-manifest-state-update-failed = Impossibile aggiornare il manifest con lo stato corrente prima del ricaricamento; plugin corrente mantenuto
entity-reload-skipped = Ricaricamento entità saltato perché il gate di ricaricamento è chiuso
entity-reload-started = Ricaricamento entità avviato
entity-reload-state-persist-failed = Impossibile persistere lo stato corrente prima del ricaricamento; plugin corrente mantenuto
entity-reload-state-produced-failed = Impossibile persistere lo stato prodotto durante il ricaricamento
entity-reloaded-manifest-update-failed = Impossibile aggiornare l'entità ricaricata nel manifest
entity-reloaded-manifest-updated = Entità ricaricata aggiornata nel manifest
inbox-message-rejected = Messaggio inbox rifiutato
ma-create-entity-already-exists = ma_create_entity: entità già esistente; entità corrente mantenuta
ma-create-entity-invalid-behaviour = ma_create_entity: riferimento behaviour non valido; saltato
ma-create-entity-kind-missing = ma_create_entity: kind assente dal registro; saltato
manifest-pin-update-failed = pin_update del manifest fallito
plugin-envelope-build-failed = Envelope plugin: costruzione messaggio fallita; saltato
plugin-envelope-create-requests-ignored = Envelope plugin: richieste di creazione ignorate senza contesto di effetto collaterale
plugin-envelope-local-dispatch-failed = Envelope plugin: dispatch locale fallito
plugin-envelope-local-dispatch-finish = Envelope plugin: dispatch locale completato
plugin-envelope-local-dispatch-start = Envelope plugin: dispatch locale avviato
plugin-envelope-local-gate-closed = Envelope plugin: gate dispatch locale chiuso
plugin-envelope-local-recipient-unknown = Envelope plugin: destinatario locale sconosciuto; saltato
plugin-envelope-local-timeout = Envelope plugin: dispatch locale scaduto
plugin-envelope-recipient-invalid = Envelope plugin: DID destinatario non valido; saltato
plugin-envelope-remote-limit = Envelope plugin: limite consegna remota raggiunto; envelope scartato
plugin-outbox-congested = Outbox plugin congestionata; gli envelope potrebbero essere scartati se il canale si riempie
plugin-outbox-drain-limit = Budget di svuotamento outbox plugin esaurito; envelope rimanenti differiti
schedule-dispatch-firing = Dispatch programmato in esecuzione
schedule-entity-not-found = Dispatch programmato: entità non trovata
schedule-random-chain-stopped = Catena di pianificazione casuale fermata: sostituita da definizione più recente
schedule-random-create-failed = Impossibile creare il prossimo job casuale
schedule-random-reschedule-failed = Impossibile ripianificare il job casuale
schedule-stale-dispatch-skipped = Dispatch programmato saltato: pianificazione obsoleta
scheduled-dispatch-error = Errore nel dispatch programmato
scheduled-dispatch-manifest-writer-unavailable = Dispatch programmato: il writer del manifest non è pronto; stato entità in attesa
