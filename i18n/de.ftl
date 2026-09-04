# ma-runtime – Deutsch
lang-name = Deutsch

own-did-published = Eigenes DID-Dokument auf IPNS veröffentlicht
own-did-publish-failed = Veröffentlichung des eigenen DID-Dokuments fehlgeschlagen
own-did-publish-timeout = Veröffentlichung des eigenen DID-Dokuments nach 2 Minuten abgebrochen
started = ma runtime gestartet
shutdown-requested = Herunterfahren angefordert
closing-endpoint = iroh-Endpunkt wird geschlossen...
shutdown-complete = Herunterfahren abgeschlossen
status-listening = Statusserver lauscht
ipfs-message-rejected = IPFS-Nachricht abgelehnt
ctrlc-handler-failed = Ctrl-C-Handler fehlgeschlagen
node-connected = Knoten mit Protokoll verbunden
received-encrypted-ma-msg = Verschlüsselte ma-Nachricht auf /ma/ipfs/0.0.1 empfangen
ping-received = :ping empfangen, sende :pong
did-publish-request-received = Anfrage zur Veröffentlichung des DID-Dokuments empfangen
document-published = Dokument veröffentlicht
did-publish-cid-reply-sent = CID-Antwort für DID-Veröffentlichung gesendet
did-publish-resolve-failed = Sender konnte nicht aufgelöst werden, um ipfs-publish-Antwort zu liefern
ipfs-store-request-received = IPFS-Store-Anfrage empfangen
ipfs-stored = Inhalt auf IPFS gespeichert
ipfs-store-cid-reply-sent = CID-Antwort gesendet
ipfs-store-resolve-failed = Sender konnte nicht aufgelöst werden, um ipfs-store-Antwort zu liefern

# Entitätsutsendung
bootstrap-complete = Bootstrap abgeschlossen
entity-loaded = Entitätsplugin geladen
entity-load-failed = Laden des Entitätsplugins fehlgeschlagen
root-list-entities = #root: Entitäten auflisten
entity-created = Entität erstellt
entity-reloaded = Entitätsplugin neu geladen
entity-deleted = Entität gelöscht
entity-states-saving = Entitätszustände werden auf IPFS gespeichert
entity-state-saving = Entitätszustand wird gespeichert
entity-state-saved = Entitätszustand gespeichert
entity-state-empty = Plugin hat leeren Zustand zurückgegeben, Speichern wird übersprungen
entity-states-saved = Entitätszustände gespeichert

# Erster Start / Auto-Init

# Eigentumsrecht
runtime-claimed = Runtime registriert.

# Geschützte Wurzelelemente
refuse-delete-root = Weigere mich strikt, ein erforderliches Wurzelelement zu löschen
runtime-claim-persisted = Eigentümer in Konfiguration geschrieben.


# Namespace creation (:create)
crud-message-received = CRUD-Nachricht empfangen
crud-acl-updated = Root-Transport-ACL aktualisiert

# CRUD validation errors
cidv1-required = der Wert muss ein reiner CIDv1 sein (beginnt mit 'b'; CIDv0 'Qm…' nicht akzeptiert)
config-key-protected = Konfigurationsschlüssel '%key%' ist geschützt
config-key-no-delete = Daemon-Konfigurationsschlüssel '%key%' kann nicht gelöscht werden
config-key-not-manifest = Konfigurationsschlüssel '%key%' ist kein bekannter manifest-config-Schlüssel
wrong-crud-protocol = falsches CRUD-Protokoll: %type%
entity-name-invalid = Entity-Name muss druckbares UTF-8 sein
reserved-entity-name = Entity-Name '%name%' ist reserviert
genesis-kind-owner-only = Nur ein Runtime-Eigentümer darf eine Entity vom Typ genesis erstellen

# IPv6 config
ipv6-enabled = IPv6 aktiviert — bindet sowohl IPv4 als auch IPv6
ipv6-disabled = IPv6 deaktiviert — bindet nur IPv4 (Neustart erforderlich zum Reaktivieren)
ipv6-enable-restart-required = Gespeichert. Neustart erforderlich, damit die Änderung wirksam wird.
ipv6-enable-unchanged = ipv6_enable ist bereits auf diesen Wert gesetzt — keine Änderung.

boot-default-root-config-populate-failed = Standardkonfiguration konnte nicht befüllt werden
boot-default-root-config-populated = Standardkonfigurationswurzel befüllt
boot-entity-load-processed = Entity-Plugins geladen
boot-group-load-failed = Gruppe konnte beim Start nicht geladen werden
boot-group-loaded-into-cache = Gruppe in Cache geladen
boot-kinds-overlay-applied = Kinds-Overlay angewendet
boot-kinds-overlay-no-change = Kinds-Overlay hat keine Manifest-Änderungen vorgenommen
boot-load-manifest-for-acl-cache-failed = Manifest für ACL-Cache-Befüllung konnte nicht geladen werden
boot-minimal-manifest-bootstrapped = Minimales Manifest bootstrapped
boot-minimal-manifest-not-found = Kein Runtime-Root-CID in IPNS gefunden; minimales Manifest wird gebootstrapped
boot-no-root-entity = Kein Root-Entity für Standardkonfigurationswurzel registriert
boot-reconciled-owners-manifest-failed = Abgleich der Eigentümer ins Manifest beim Start fehlgeschlagen
boot-reconciled-owners-persist-failed = Abgeglichene Eigentümer konnten nicht in config.yaml gespeichert werden
boot-reconciled-owners-published = Eigentümer aus config.yaml/--owner beim Start ins Manifest abgeglichen
boot-root-acl-load-cache-failed = Root-ACL konnte beim Start nicht geladen werden
boot-root-acl-load-failed = Root-ACL konnte nicht aus Manifest geladen werden
boot-root-acl-loaded-from-manifest = Root-Transport-ACL aus Manifest geladen
boot-root-acl-loaded-into-cache = Root-ACL in Cache geladen
bootstrap-acl-published = ACL-Knoten veröffentlicht
bootstrap-endpoint-close-stuck = Endpoint wird noch von laufenden Aufgaben gehalten nach 10 s; erzwungenes Schließen
bootstrap-endpoint-close-timeout = Endpoint-Schließen nach 5 s abgelaufen; Beenden erzwungen
bootstrap-entity-lifecycle-update-failed = Entity-Lebenszyklus konnte nicht in IPFS geschrieben werden
bootstrap-entity-lifecycle-updated = Entity-Lebenszyklus in IPFS aktualisiert
bootstrap-entity-node-shutdown-updated = Entity-Knoten beim Herunterfahren aktualisiert
bootstrap-entity-published = Entity-Knoten veröffentlicht
bootstrap-entity-registering-prepublished = Vor-veröffentlichtes Entity wird registriert
bootstrap-entity-registry-fetch-failed = Entity-Knoten konnte nicht abgerufen werden
bootstrap-entity-registry-kind-extends-failed = Kind-Erweiterungskette konnte nicht aufgelöst werden
bootstrap-entity-registry-kind-fetch-failed = Kind-Knoten konnte nicht abgerufen werden
bootstrap-entity-registry-kind-missing = Kind nicht im Manifest gefunden; Entity wird übersprungen
bootstrap-entity-registry-not-in-manifest = Entity im Registry aber nicht im Manifest, wird übersprungen
bootstrap-entity-state-save-failed = Entity-Zustände konnten nicht gespeichert werden
bootstrap-entity-state-shutdown-aborted = Herunterfahren abgebrochen; Runtime bleibt aktiv damit der Zustand beim nächsten Versuch gespeichert werden kann
bootstrap-entity-state-update-fetch-failed = Entity-Knoten für Zustandsupdate konnte nicht abgerufen werden
bootstrap-envelope-delivery-failed = Plugin-Envelope-Zustellung fehlgeschlagen; Envelope wird verworfen
bootstrap-envelope-open-failed = Plugin-Envelope: Outbox konnte nicht geöffnet werden; Envelope wird verworfen
bootstrap-group-published = Gruppen-Knoten veröffentlicht
bootstrap-kind-published = Kind-Knoten veröffentlicht
bootstrap-kind-registry-extends-failed = Kind-Erweiterungskette für Registry konnte nicht aufgelöst werden
bootstrap-kind-registry-fetch-log-failed = Kind-Knoten für Registry konnte nicht abgerufen werden
bootstrap-kind-registry-hydrated = Kind-Registry aus Manifest befüllt
bootstrap-kinds-overlay-pin-update-failed = Pin/Update nach Kinds-Overlay fehlgeschlagen
bootstrap-kinds-overlay-published = Runtime-Manifest nach Kinds-Overlay veröffentlicht
bootstrap-kinds-tree-published = Runtime-Kinds-Baum veröffentlicht
bootstrap-lifecycle-manifest-pin-update-failed = Pin/Update nach Lebenszyklus-Persistierung fehlgeschlagen
bootstrap-lifecycle-manifest-publish-failed = Manifest nach Lebenszyklus-Übergängen konnte nicht veröffentlicht werden
bootstrap-lifecycle-manifest-published = Aktualisiertes Manifest nach Lebenszyklus-Übergängen veröffentlicht
bootstrap-manifest-fetch-failed = Runtime-Manifest konnte nicht abgerufen werden
bootstrap-minimal-manifest-failed = Minimales Manifest konnte nicht bootstrapped werden
bootstrap-remote-root-pin-confirmed = Remote-Root-Pin bestätigt
bootstrap-remote-root-pin-misconfigured = Remote-Root-Pinning ist fehlerhaft konfiguriert
bootstrap-root-acl-published = Root-Transport-ACL veröffentlicht
bootstrap-root-cid-shutdown-persist-failed = root_cid konnte beim Herunterfahren nicht persistiert werden
bootstrap-root-cid-shutdown-publish-failed = runtime_ipns-Veröffentlichung beim Herunterfahren fehlgeschlagen
bootstrap-root-cid-shutdown-publish-succeeded = runtime_ipns-Veröffentlichung beim Herunterfahren erfolgreich
bootstrap-root-cid-shutdown-publish-timeout = runtime_ipns-Veröffentlichung beim Herunterfahren abgelaufen
bootstrap-root-pin-replacement-failed = Weiter nach einem Fehler beim Remote-Root-Pin-Ersatz
bootstrap-root-pin-update-failed = Pin/Update nach Bootstrap fehlgeschlagen
bootstrap-runtime-manifest-published = Runtime-Root-Manifest veröffentlicht
crud-message-rejected = CRUD-Nachricht abgelehnt
entity-reload-current-node-load-failed = Aktuellen Entity-Knoten vor Neuladen konnte nicht geladen werden; aktuelles Plugin bleibt
entity-reload-failed = Entity konnte nicht neu geladen werden; wird bis zum nächsten Neuladen deaktiviert
entity-reload-kind-extends-failed = Kind-Erweiterungskette beim Entity-Neuladen konnte nicht aufgelöst werden
entity-reload-kind-fetch-failed = Kind-Knoten beim Entity-Neuladen konnte nicht abgerufen werden
entity-reload-kind-lookup-failed = Manifest für Kind-Suche beim Entity-Neuladen konnte nicht geladen werden
entity-reload-kind-missing = Kind nicht im Manifest; Entity kann nicht neu geladen werden
entity-reload-manifest-state-update-failed = Manifest konnte vor Neuladen nicht mit aktuellem Zustand aktualisiert werden; aktuelles Plugin bleibt
entity-reload-skipped = Entity-Neuladen übersprungen, da das Neuladen-Gate geschlossen ist
entity-reload-started = Entity-Neuladen gestartet
entity-reload-state-persist-failed = Aktuellen Zustand vor Neuladen konnte nicht persistiert werden; aktuelles Plugin bleibt
entity-reload-state-produced-failed = Beim Neuladen erzeugter Zustand konnte nicht persistiert werden
entity-reloaded-manifest-update-failed = Neu geladenes Entity konnte nicht im Manifest aktualisiert werden
entity-reloaded-manifest-updated = Neu geladenes Entity im Manifest aktualisiert
inbox-message-rejected = Inbox-Nachricht abgelehnt
ma-create-entity-already-exists = ma_create_entity: Entity existiert bereits; aktuelles Entity bleibt
ma-create-entity-invalid-behaviour = ma_create_entity: ungültige Behaviour-Referenz; übersprungen
ma-create-entity-kind-missing = ma_create_entity: Kind nicht im Registry; übersprungen
manifest-pin-update-failed = Manifest pin_update fehlgeschlagen
plugin-envelope-build-failed = Plugin-Envelope: Nachricht konnte nicht erstellt werden; übersprungen
plugin-envelope-create-requests-ignored = Plugin-Envelope: Erstellungsanfragen werden ohne Side-Effect-Kontext ignoriert
plugin-envelope-local-dispatch-failed = Plugin-Envelope: lokaler Dispatch fehlgeschlagen
plugin-envelope-local-dispatch-finish = Plugin-Envelope: lokaler Dispatch abgeschlossen
plugin-envelope-local-dispatch-start = Plugin-Envelope: lokaler Dispatch gestartet
plugin-envelope-local-gate-closed = Plugin-Envelope: lokales Dispatch-Gate geschlossen
plugin-envelope-local-recipient-unknown = Plugin-Envelope: unbekannter lokaler Empfänger; übersprungen
plugin-envelope-local-timeout = Plugin-Envelope: lokaler Dispatch abgelaufen
plugin-envelope-recipient-invalid = Plugin-Envelope: ungültige Empfänger-DID; übersprungen
plugin-envelope-remote-limit = Plugin-Envelope: Remote-Zustelllimit erreicht; Envelope verworfen
plugin-outbox-congested = Plugin-Outbox überlastet; Envelopes können verworfen werden wenn Kanal voll ist
plugin-outbox-drain-limit = Plugin-Outbox-Drain-Budget erschöpft; verbleibende Envelopes werden zurückgestellt
schedule-dispatch-firing = Geplanter Dispatch wird ausgeführt
schedule-entity-not-found = Geplanter Dispatch: Entity nicht gefunden
schedule-random-chain-stopped = Zufällige Zeitplan-Kette gestoppt: durch neuere Definition abgelöst
schedule-random-create-failed = Nächsten zufälligen Job konnte nicht erstellt werden
schedule-random-reschedule-failed = Zufälligen Job konnte nicht neu geplant werden
schedule-stale-dispatch-skipped = Geplanter Dispatch übersprungen: veralteter Zeitplan
scheduled-dispatch-error = Fehler beim geplanten Dispatch
scheduled-dispatch-manifest-writer-unavailable = Geplanter Dispatch: Manifest-Writer ist nicht bereit; Entity-Zustand bleibt ausstehend
