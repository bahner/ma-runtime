# ma-runtime – Polski
lang-name = Polski

own-did-published = Własny dokument DID opublikowany w IPNS
own-did-publish-failed = Nie udało się opublikować własnego dokumentu DID
own-did-publish-timeout = Publikacja własnego dokumentu DID przekroczyła limit czasu po 2 minutach
started = ma runtime uruchomiony
shutdown-requested = Żądanie wyłączenia
closing-endpoint = Zamykanie punktu końcowego iroh...
shutdown-complete = Wyłączenie zakończone
status-listening = Serwer statusu nasłuchuje
ipfs-message-rejected = Wiadomość IPFS odrzucona
ctrlc-handler-failed = Błąd procedury obsługi Ctrl-C
node-connected = Węzeł podłączony do protokołu
received-encrypted-ma-msg = Odebrano zaszyfrowaną wiadomość ma na /ma/ipfs/0.0.1
ping-received = Odebrano :ping, wysyłam :pong
did-publish-request-received = Odebrano żądanie publikacji dokumentu DID
document-published = Dokument opublikowany
did-publish-cid-reply-sent = Wysłano odpowiedź CID dla publikacji DID
did-publish-resolve-failed = Nie można rozwiązać nadawcy w celu dostarczenia odpowiedzi ipfs-publish
ipfs-store-request-received = Odebrano żądanie przechowywania IPFS
ipfs-stored = Treść zapisana w IPFS
ipfs-store-cid-reply-sent = Odpowiedź CID wysłana
ipfs-store-resolve-failed = Nie można rozwiązać nadawcy w celu dostarczenia odpowiedzi ipfs-store

# Przekazywanie encji
bootstrap-complete = Bootstrap zakończony
entity-loaded = Wtyczka encji załadowana
entity-load-failed = Nie udało się załadować wtyczki encji
root-list-entities = #root: lista encji
entity-created = Encja utworzona
entity-reloaded = Wtyczka encji załadowana ponownie
entity-deleted = Encja usunięta
entity-states-saving = Zapisywanie stanów encji do IPFS
entity-state-saving = Zapisywanie stanu encji
entity-state-saved = Stan encji zapisany
entity-state-empty = Wtyczka zwróciła pusty stan, pomijanie zapisu
entity-states-saved = Stany encji zapisane

# Pierwsze uruchomienie / auto-init

# Własność
runtime-claimed = Runtime zarejestrowany.

# Chronione elementy główne
refuse-delete-root = Stanowczo odmawiam usunięcia wymaganego elementu głównego
runtime-claim-persisted = Właściciel zapisany w konfiguracji.


# Namespace creation (:create)
crud-message-received = Odebrano wiadomość CRUD
crud-acl-updated = Zaktualizowano główny ACL transportu

# CRUD validation errors
cidv1-required = wartość musi być zwykłym CIDv1 (zaczyna się od 'b'; CIDv0 'Qm…' nie jest akceptowany)
config-key-protected = klucz config '%key%' jest chroniony
config-key-no-delete = klucza config '%key%' demona nie można usunąć
config-key-not-manifest = klucz config '%key%' nie jest znany kluczem manifest config
wrong-crud-protocol = nieprawidłowy protokół CRUD: %type%
entity-name-invalid = nazwa entity musi być drukowalnym UTF-8
reserved-entity-name = nazwa entity '%name%' jest zarezerwowana
genesis-kind-owner-only = Tylko właściciel runtime može utworzyć entity typu genesis

# IPv6 config
ipv6-enabled = IPv6 włączone — nasłuchuje na IPv4 i IPv6
ipv6-disabled = IPv6 wyłączone — wiązany jest tylko IPv4 (wymagany restart w celu ponownego włączenia)
ipv6-enable-restart-required = Zapisano. Do zastosowania tej zmiany wymagany jest restart.
ipv6-enable-unchanged = ipv6_enable jest już ustawione na tę wartość — brak zmian.

boot-default-root-config-populate-failed = Nie udało się wypełnić domyślnego katalogu głównego konfiguracji
boot-default-root-config-populated = Domyślny katalog główny konfiguracji wypełniony
boot-entity-load-processed = Wtyczki encji załadowane
boot-group-load-failed = Nie udało się załadować grupy przy uruchomieniu
boot-group-loaded-into-cache = Grupa załadowana do pamięci podręcznej
boot-kinds-overlay-applied = Nakładka kinds zastosowana
boot-kinds-overlay-no-change = Nakładka kinds nie wprowadziła żadnych zmian w manifeście
boot-load-manifest-for-acl-cache-failed = Nie udało się załadować manifestu do wypełnienia pamięci podręcznej ACL
boot-minimal-manifest-bootstrapped = Minimalny manifest zainicjalizowany
boot-minimal-manifest-not-found = Nie znaleoperatoro głównego CID środowiska uruchomieniowego w IPNS; inicjalizowanie minimalnego manifestu
boot-no-root-entity = Brak głównej encji zarejestrowanej dla domyślnego katalogu głównego konfiguracji
boot-reconciled-owners-manifest-failed = Nie udało się uzgodnić właścicieli w manifeście przy uruchomieniu
boot-reconciled-owners-persist-failed = Nie udało się zapisać uzgodnionych właścicieli do config.yaml
boot-reconciled-owners-published = Właściciele uzgodnieni z config.yaml/--owner w manifeście przy uruchomieniu
boot-root-acl-load-cache-failed = Nie udało się załadować głównego ACL przy uruchomieniu
boot-root-acl-load-failed = Nie udało się załadować głównego ACL z manifestu
boot-root-acl-loaded-from-manifest = Główny ACL transportu załadowany z manifestu
boot-root-acl-loaded-into-cache = Główny ACL załadowany do pamięci podręcznej
bootstrap-acl-published = Węzeł ACL opublikowany
bootstrap-endpoint-close-stuck = Endpoint nadal jest przetrzymywany przez uruchomione zadania po 10 s; wymuszone zamknięcie
bootstrap-endpoint-close-timeout = Zamykanie endpointu przekroczyło limit czasu po 5 s; wymuszone wyjście
bootstrap-entity-lifecycle-update-failed = Nie udało się zapisać zaktualizowanego cyklu życia encji do IPFS
bootstrap-entity-lifecycle-updated = Cykl życia encji zaktualizowany w IPFS
bootstrap-entity-node-shutdown-updated = Węzeł encji zaktualizowany podczas zamykania
bootstrap-entity-published = Węzeł encji opublikowany
bootstrap-entity-registering-prepublished = Rejestrowanie wstępnie opublikowanej encji
bootstrap-entity-registry-fetch-failed = Nie udało się pobrać węzła encji
bootstrap-entity-registry-kind-extends-failed = Nie udało się rozwiązać łańcucha rozszerzeń kind
bootstrap-entity-registry-kind-fetch-failed = Nie udało się pobrać węzła kind
bootstrap-entity-registry-kind-missing = Nie znaleoperatoro kind w manifeście; pominięto encję
bootstrap-entity-registry-not-in-manifest = Encja w rejestrze, ale nie w manifeście, pominięta
bootstrap-entity-state-save-failed = Nie udało się zapisać stanów encji
bootstrap-entity-state-shutdown-aborted = Zamykanie przerwane; środowisko uruchomieniowe pozostaje aktywne, aby stan mógł zostać zapisany przy następnej próbie zamknięcia
bootstrap-entity-state-update-fetch-failed = Nie udało się pobrać węzła encji do aktualizacji stanu
bootstrap-envelope-delivery-failed = Dostarczenie koperty wtyczki nie powiodło się; koperta odrzucona
bootstrap-envelope-open-failed = Koperta wtyczki: otwarcie skrzynki wychodzącej nie powiodło się; koperta odrzucona
bootstrap-group-published = Węzeł grupy opublikowany
bootstrap-kind-published = Węzeł kind opublikowany
bootstrap-kind-registry-extends-failed = Nie udało się rozwiązać łańcucha rozszerzeń kind dla rejestru
bootstrap-kind-registry-fetch-log-failed = Nie udało się pobrać węzła kind dla rejestru
bootstrap-kind-registry-hydrated = Rejestr kinds zasilony z manifestu
bootstrap-kinds-overlay-pin-update-failed = Przypinanie/aktualizacja nie powiodło się po nakładce kinds
bootstrap-kinds-overlay-published = Manifest środowiska uruchomieniowego opublikowany po nakładce kinds
bootstrap-kinds-tree-published = Drzewo kinds środowiska uruchomieniowego opublikowane
bootstrap-lifecycle-manifest-pin-update-failed = Przypinanie/aktualizacja nie powiodło się po utrwaleniu cyklu życia
bootstrap-lifecycle-manifest-publish-failed = Nie udało się opublikować manifestu po przejściach cyklu życia
bootstrap-lifecycle-manifest-published = Zaktualizowany manifest opublikowany po przejściach cyklu życia
bootstrap-manifest-fetch-failed = Nie udało się pobrać manifestu środowiska uruchomieniowego
bootstrap-minimal-manifest-failed = Nie udało się zainicjalizować minimalnego manifestu
bootstrap-remote-root-pin-confirmed = Zdalne przypięcie główne potwierdzone
bootstrap-remote-root-pin-misconfigured = Zdalne przypinanie główne jest błędnie skonfigurowane
bootstrap-root-acl-published = Główny ACL transportu opublikowany
bootstrap-root-cid-shutdown-persist-failed = Nie udało się utrwalić root_cid podczas zamykania
bootstrap-root-cid-shutdown-publish-failed = Publikacja runtime_ipns podczas zamykania nie powiodła się
bootstrap-root-cid-shutdown-publish-succeeded = Publikacja runtime_ipns podczas zamykania powiodła się
bootstrap-root-cid-shutdown-publish-timeout = Publikacja runtime_ipns podczas zamykania przekroczyła limit czasu
bootstrap-root-pin-replacement-failed = Kontynuowanie po błędzie zastępowania zdalnego przypięcia głównego
bootstrap-root-pin-update-failed = Przypinanie/aktualizacja nie powiodło się po bootstrapie
bootstrap-runtime-manifest-published = Główny manifest środowiska uruchomieniowego opublikowany
crud-message-rejected = Wiadomość CRUD odrzucona
entity-reload-current-node-load-failed = Nie udało się załadować bieżącego węzła encji przed przeładowaniem; zachowano bieżącą wtyczkę
entity-reload-failed = Przeładowanie encji nie powiodło się; wyłączona do następnego przeładowania
entity-reload-kind-extends-failed = Nie udało się rozwiązać łańcucha rozszerzeń kind podczas przeładowania encji
entity-reload-kind-fetch-failed = Nie udało się pobrać węzła kind podczas przeładowania encji
entity-reload-kind-lookup-failed = Nie udało się załadować manifestu do wyszukiwania kind podczas przeładowania encji
entity-reload-kind-missing = Nie znaleoperatoro kind w manifeście; nie można przeładować encji
entity-reload-manifest-state-update-failed = Nie udało się zaktualizować manifestu z bieżącym stanem przed przeładowaniem; zachowano bieżącą wtyczkę
entity-reload-skipped = Przeładowanie encji pominięte, ponieważ brama przeładowania jest zamknięta
entity-reload-started = Przeładowanie encji rozpoczęte
entity-reload-state-persist-failed = Nie udało się utrwalić bieżącego stanu przed przeładowaniem; zachowano bieżącą wtyczkę
entity-reload-state-produced-failed = Nie udało się utrwalić stanu wygenerowanego podczas przeładowania
entity-reloaded-manifest-update-failed = Nie udało się zaktualizować przeładowanej encji w manifeście
entity-reloaded-manifest-updated = Przeładowana encja zaktualizowana w manifeście
inbox-message-rejected = Wiadomość ze skrzynki odbiorczej odrzucona
ma-create-entity-already-exists = ma_create_entity: encja już istnieje; zachowano bieżącą encję
ma-create-entity-invalid-behaviour = ma_create_entity: nieprawidłowe odwołanie do behaviour; pominięto
ma-create-entity-kind-missing = ma_create_entity: kind nieobecny w rejestrze; pominięto
manifest-pin-update-failed = pin_update manifestu nie powiodło się
plugin-envelope-build-failed = Koperta wtyczki: nie udało się zbudować wiadomości; pominięto
plugin-envelope-create-requests-ignored = Koperta wtyczki: żądania tworzenia zignorowane bez kontekstu efektu ubocznego
plugin-envelope-local-dispatch-failed = Koperta wtyczki: lokalna wysyłka nie powiodła się
plugin-envelope-local-dispatch-finish = Koperta wtyczki: lokalna wysyłka zakończona
plugin-envelope-local-dispatch-start = Koperta wtyczki: lokalna wysyłka rozpoczęta
plugin-envelope-local-gate-closed = Koperta wtyczki: brama lokalnej wysyłki zamknięta
plugin-envelope-local-recipient-unknown = Koperta wtyczki: nieznany lokalny odbiorca; pominięto
plugin-envelope-local-timeout = Koperta wtyczki: lokalna wysyłka przekroczyła limit czasu
plugin-envelope-recipient-invalid = Koperta wtyczki: nieprawidłowy DID odbiorcy; pominięto
plugin-envelope-remote-limit = Koperta wtyczki: osiągnięto limit zdalnego dostarczania; koperta odrzucona
plugin-outbox-congested = Skrzynka wychodząca wtyczki przeciążona; koperty mogą być odrzucane jeśli kanał się zapełni
plugin-outbox-drain-limit = Budżet opróżniania skrzynki wychodzącej wtyczki wyczerpany; pozostałe koperty odłożone
schedule-dispatch-firing = Zaplanowana wysyłka wykonywana
schedule-entity-not-found = Zaplanowana wysyłka: nie znaleoperatoro encji
schedule-random-chain-stopped = Łańcuch losowego harmonogramu zatrzymany: zastąpiony nowszą definicją
schedule-random-create-failed = Nie udało się utworzyć następnego losowego zadania
schedule-random-reschedule-failed = Nie udało się przeplanować losowego zadania
schedule-stale-dispatch-skipped = Zaplanowana wysyłka pominięta: przestarzały harmonogram
scheduled-dispatch-error = Błąd zaplanowanej wysyłki
scheduled-dispatch-manifest-writer-unavailable = Zaplanowana wysyłka: moduł zapisu manifestu nie jest gotowy; stan encji oczekuje
