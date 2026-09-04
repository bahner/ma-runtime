# ma-runtime – Български
lang-name = Български

own-did-published = Собственият DID документ е публикуван в IPNS
own-did-publish-failed = Неуспешна публикация на собствения DID документ
own-did-publish-timeout = Публикацията на собствения DID документ изтече след 2 минути
started = ma runtime стартиран
shutdown-requested = Заявено изключване
closing-endpoint = Затваряне на iroh крайната точка...
shutdown-complete = Изключването завършено
status-listening = Сървърът за статус слуша
ipfs-message-rejected = IPFS съобщението е отхвърлено
ctrlc-handler-failed = Неуспех на манипулатора Ctrl-C
node-connected = Възелът е свързан с протокол
received-encrypted-ma-msg = Получено криптирано ma съобщение на /ma/ipfs/0.0.1
ping-received = Получен :ping, изпращам :pong
did-publish-request-received = Получена заявка за публикуване на DID документ
document-published = Документът е публикуван
did-publish-cid-reply-sent = Изпратен CID отговор за публикуване на DID
did-publish-resolve-failed = Неуспешно разрешаване на подателя за доставяне на ipfs-publish отговор
ipfs-store-request-received = Получена заявка за съхранение в IPFS
ipfs-stored = Съдържанието е съхранено в IPFS
ipfs-store-cid-reply-sent = CID отговорът е изпратен
ipfs-store-resolve-failed = Неуспешно разрешаване на подателя за доставяне на ipfs-store отговор

# Изпращане на същности
bootstrap-complete = Bootstrap завършен
entity-loaded = Плъгинът на същността е зареден
entity-load-failed = Неуспешно зареждане на плъгина на същността
root-list-entities = #root: списък на същностите
entity-created = Същността е създадена
entity-reloaded = Плъгинът на същността е презареден
entity-deleted = Същността е изтрита
entity-states-saving = Записване на състоянията на същностите в IPFS
entity-state-saving = Записване на състоянието на същността
entity-state-saved = Състоянието на същността е записано
entity-state-empty = Плъгинът върна празно състояние, записването е пропуснато
entity-states-saved = Състоянията на същностите са записани

# Първо стартиране / авто-инициализация

# Собственост
runtime-claimed = Runtime е регистриран.

# Защитени корени елементи
refuse-delete-root = Категорично отказвам да изтрия задължителен корен елемент
runtime-claim-persisted = Собственикът е записан в конфигурацията.


# Namespace creation (:create)
crud-message-received = Получено CRUD съобщение
crud-acl-updated = Основният транспортен ACL е актуализиран

# CRUD validation errors
cidv1-required = стойността трябва да е чист CIDv1 (започва с 'b'; CIDv0 'Qm…' не се приема)
config-key-protected = конфигурационният ключ '%key%' е защитен
config-key-no-delete = конфигурационният ключ '%key%' на демона не може да бъде изтрит
config-key-not-manifest = конфигурационният ключ '%key%' не е познат ключ на manifest config
wrong-crud-protocol = грешен CRUD протокол: %type%
entity-name-invalid = Името на entity трябва да бъде отпечатваем UTF-8
reserved-entity-name = Името на entity '%name%' е запазено
genesis-kind-owner-only = Само собственик на ma runtime може да създаде entity от тип genesis

# IPv6 config
ipv6-enabled = IPv6 е активиран — свързва се с IPv4 и IPv6 едновременно
ipv6-disabled = IPv6 е деактивиран — свързва се само с IPv4 (необходимо е рестартиране за повторно активиране)
ipv6-enable-restart-required = Запазено. Необходимо е рестартиране, за да влезе промяната в сила.
ipv6-enable-unchanged = ipv6_enable вече е зададен на тази стойност — без промяна.

boot-default-root-config-populate-failed = Попълването на конфигурацията по подразбиране не бе успешно
boot-default-root-config-populated = Конфигурацията по подразбиране е попълнена
boot-entity-load-processed = Приставките на обектите са заредени
boot-group-load-failed = Зареждането на групата при стартиране не бе успешно
boot-group-loaded-into-cache = Групата е заредена в кеша
boot-kinds-overlay-applied = Наслагването на видовете е приложено
boot-kinds-overlay-no-change = Наслагването на видовете не промени манифеста
boot-load-manifest-for-acl-cache-failed = Зареждането на манифеста за попълване на кеша на ACL не бе успешно
boot-minimal-manifest-bootstrapped = Минималният манифест е инициализиран
boot-minimal-manifest-not-found = Коренният CID на средата за изпълнение не е намерен в IPNS; инициализиране на минимален манифест
boot-no-root-entity = Няма регистриран корен обект за конфигурацията по подразбиране
boot-reconciled-owners-manifest-failed = Съгласуването на собствениците в манифеста при стартиране не бе успешно
boot-reconciled-owners-persist-failed = Записването на съгласуваните собственици в config.yaml не бе успешно
boot-reconciled-owners-published = Собствениците са съгласувани от config.yaml/--owner в манифеста при стартиране
boot-root-acl-load-cache-failed = Зареждането на коренния ACL при стартиране не бе успешно
boot-root-acl-load-failed = Зареждането на коренния ACL от манифеста не бе успешно
boot-root-acl-loaded-from-manifest = Коренният транспортен ACL е зареден от манифеста
boot-root-acl-loaded-into-cache = Коренният ACL е зареден в кеша
bootstrap-acl-published = Възелът на ACL е публикуван
bootstrap-endpoint-close-stuck = Крайната точка все още се задържа от текущи задачи след 10 с; принудително затваряне
bootstrap-endpoint-close-timeout = Затварянето на крайната точка изтече след 5 с; принудително прекратяване
bootstrap-entity-lifecycle-update-failed = Записването на обновения жизнен цикъл на обекта в IPFS не бе успешно
bootstrap-entity-lifecycle-updated = Жизненият цикъл на обекта е обновен в IPFS
bootstrap-entity-node-shutdown-updated = Възелът на обекта е обновен при изключване
bootstrap-entity-published = Възелът на обекта е публикуван
bootstrap-entity-registering-prepublished = Регистриране на предварително публикуван обект
bootstrap-entity-registry-fetch-failed = Извличането на възела на обекта не бе успешно
bootstrap-entity-registry-kind-extends-failed = Разрешаването на веригата на разширения на вида не бе успешно
bootstrap-entity-registry-kind-fetch-failed = Извличането на възела на вида не бе успешно
bootstrap-entity-registry-kind-missing = Видът не е намерен в манифеста; обектът е пропуснат
bootstrap-entity-registry-not-in-manifest = Обектът е в регистъра, но не в манифеста; пропуснат
bootstrap-entity-state-save-failed = Записването на състоянията на обектите не бе успешно
bootstrap-entity-state-shutdown-aborted = Изключването е прекратено; средата остава активна за запазване на състоянието при следващото изключване
bootstrap-entity-state-update-fetch-failed = Извличането на възела на обекта за обновяване на състоянието не бе успешно
bootstrap-envelope-delivery-failed = Доставката на плика на приставката не бе успешна; пликът е изхвърлен
bootstrap-envelope-open-failed = Плик на приставката: отварянето на изходящата кутия не бе успешно; пликът е изхвърлен
bootstrap-group-published = Възелът на групата е публикуван
bootstrap-kind-published = Възелът на вида е публикуван
bootstrap-kind-registry-extends-failed = Разрешаването на веригата на разширения на вида за регистъра не бе успешно
bootstrap-kind-registry-fetch-log-failed = Извличането на възела на вида за регистъра не бе успешно
bootstrap-kind-registry-hydrated = Регистърът на видовете е попълнен от манифеста
bootstrap-kinds-overlay-pin-update-failed = Закачането/обновяването не бе успешно след наслагването на видовете
bootstrap-kinds-overlay-published = Манифестът на средата е публикуван след наслагването на видовете
bootstrap-kinds-tree-published = Дървото на видовете на средата е публикувано
bootstrap-lifecycle-manifest-pin-update-failed = Закачането/обновяването не бе успешно след запазването на жизнения цикъл
bootstrap-lifecycle-manifest-publish-failed = Публикуването на манифеста след преходите на жизнения цикъл не бе успешно
bootstrap-lifecycle-manifest-published = Обновеният манифест е публикуван след преходите на жизнения цикъл
bootstrap-manifest-fetch-failed = Извличането на манифеста на средата не бе успешно
bootstrap-minimal-manifest-failed = Инициализацията на минималния манифест не бе успешна
bootstrap-remote-root-pin-confirmed = Дистанционното коренно закачане е потвърдено
bootstrap-remote-root-pin-misconfigured = Дистанционното коренно закачане е неправилно конфигурирано
bootstrap-root-acl-published = Коренният транспортен ACL е публикуван
bootstrap-root-cid-shutdown-persist-failed = Запазването на root_cid при изключване не бе успешно
bootstrap-root-cid-shutdown-publish-failed = Публикуването на runtime_ipns при изключване не бе успешно
bootstrap-root-cid-shutdown-publish-succeeded = Публикуването на runtime_ipns при изключване бе успешно
bootstrap-root-cid-shutdown-publish-timeout = Публикуването на runtime_ipns при изключване изтече
bootstrap-root-pin-replacement-failed = Продължаване след грешка при замяната на дистанционното коренно закачане
bootstrap-root-pin-update-failed = Закачането/обновяването не бе успешно след начална зареждане
bootstrap-runtime-manifest-published = Коренният манифест на средата е публикуван
crud-message-rejected = CRUD съобщението е отхвърлено
entity-reload-current-node-load-failed = Извличането на текущия възел на обекта преди презареждане не бе успешно; текущата приставка е запазена
entity-reload-failed = Презареждането на обекта не бе успешно; деактивиран до следващото презареждане
entity-reload-kind-extends-failed = Разрешаването на веригата на разширения на вида при презареждане на обекта не бе успешно
entity-reload-kind-fetch-failed = Извличането на възела на вида при презареждане на обекта не бе успешно
entity-reload-kind-lookup-failed = Извличането на манифеста за търсене на вид при презареждане на обекта не бе успешно
entity-reload-kind-missing = Видът не е намерен в манифеста; обектът не може да бъде презареден
entity-reload-manifest-state-update-failed = Обновяването на манифеста с текущото състояние преди презареждане не бе успешно; текущата приставка е запазена
entity-reload-skipped = Презареждането на обекта е пропуснато, защото портата за презареждане е затворена
entity-reload-started = Презареждането на обекта е започнало
entity-reload-state-persist-failed = Запазването на текущото състояние преди презареждане не бе успешно; текущата приставка е запазена
entity-reload-state-produced-failed = Запазването на състоянието, произведено при презареждане, не бе успешно
entity-reloaded-manifest-update-failed = Обновяването на презаредения обект в манифеста не бе успешно
entity-reloaded-manifest-updated = Презареденият обект е обновен в манифеста
inbox-message-rejected = Съобщението от входящата кутия е отхвърлено
ma-create-entity-already-exists = ma_create_entity: обектът вече съществува; текущият обект е запазен
ma-create-entity-invalid-behaviour = ma_create_entity: невалидна препратка към поведение; пропуснато
ma-create-entity-kind-missing = ma_create_entity: видът липсва от регистъра; пропуснато
manifest-pin-update-failed = Манифестът pin_update не бе успешен
plugin-envelope-build-failed = Плик на приставката: изграждането на съобщението не бе успешно; пропуснато
plugin-envelope-create-requests-ignored = Плик на приставката: заявките за създаване се игнорират без контекст на страничен ефект
plugin-envelope-local-dispatch-failed = Плик на приставката: локалното изпращане не бе успешно
plugin-envelope-local-dispatch-finish = Плик на приставката: локалното изпращане е завършено
plugin-envelope-local-dispatch-start = Плик на приставката: локалното изпращане е започнало
plugin-envelope-local-gate-closed = Плик на приставката: портата за локално изпращане е затворена
plugin-envelope-local-recipient-unknown = Плик на приставката: неизвестен локален получател; пропуснато
plugin-envelope-local-timeout = Плик на приставката: локалното изпращане изтече
plugin-envelope-recipient-invalid = Плик на приставката: невалиден DID на получателя; пропуснато
plugin-envelope-remote-limit = Плик на приставката: лимитът за дистанционна доставка е достигнат; пликът е изхвърлен
plugin-outbox-congested = Изходящата кутия на приставката е претоварена; пликовете могат да бъдат изхвърлени ако каналът се запълни
plugin-outbox-drain-limit = Бюджетът за изпразване на изходящата кутия на приставката е изчерпан; останалите пликове са отложени
schedule-dispatch-firing = Планираното изпращане е в ход
schedule-entity-not-found = Планирано изпращане: обектът не е намерен
schedule-random-chain-stopped = Случайната планирана верига е спряна: заменена с по-нова дефиниция
schedule-random-create-failed = Създаването на следващата случайна задача не бе успешно
schedule-random-reschedule-failed = Преплануването на случайната задача не бе успешно
schedule-stale-dispatch-skipped = Планираното изпращане е пропуснато: остарял план
scheduled-dispatch-error = Грешка при планираното изпращане
scheduled-dispatch-manifest-writer-unavailable = Планирано изпращане: писателят на манифест не е готов; състоянието на обекта чака
