# ma-runtime – Русский
lang-name = Русский

own-did-published = Собственный DID-документ опубликован в IPNS
own-did-publish-failed = Не удалось опубликовать собственный DID-документ
own-did-publish-timeout = Публикация собственного DID-документа прервана по истечении 2 минут
started = ma runtime запущен
shutdown-requested = Запрошено завершение работы
closing-endpoint = Закрытие конечной точки iroh...
shutdown-complete = Завершение работы выполнено
status-listening = Сервер статуса ожидает подключений
ipfs-message-rejected = IPFS-сообщение отклонено
ctrlc-handler-failed = Сбой обработчика Ctrl-C
node-connected = Узел подключён к протоколу
received-encrypted-ma-msg = Получено зашифрованное ma-сообщение на /ma/ipfs/0.0.1
ping-received = Получен :ping, отправляю :pong
did-publish-request-received = Получен запрос на публикацию DID-документа
document-published = Документ опубликован
did-publish-cid-reply-sent = Отправлен CID-ответ для публикации DID
did-publish-resolve-failed = Не удалось определить отправителя для доставки ответа ipfs-publish
ipfs-store-request-received = Получен запрос на сохранение в IPFS
ipfs-stored = Содержимое сохранено в IPFS
ipfs-store-cid-reply-sent = CID-ответ отправлен
ipfs-store-resolve-failed = Не удалось определить отправителя для доставки ответа ipfs-store

# Диспетчеризация сущностей
bootstrap-complete = Bootstrap завершён
entity-loaded = Плагин сущности загружен
entity-load-failed = Не удалось загрузить плагин сущности
root-list-entities = #root: список сущностей
entity-created = Сущность создана
entity-reloaded = Плагин сущности перезагружен
entity-deleted = Сущность удалена
entity-states-saving = Сохранение состояний сущностей в IPFS
entity-state-saving = Сохранение состояния сущности
entity-state-saved = Состояние сущности сохранено
entity-state-empty = Плагин вернул пустое состояние, сохранение пропущено
entity-states-saved = Состояния сущностей сохранены

# Первый запуск / авто-инициализация

# Владение
runtime-claimed = Runtime зарегистрирован.

# Защищённые корневые элементы
refuse-delete-root = Категорически отказываюсь удалять обязательный корневой элемент
runtime-claim-persisted = Владелец записан в конфигурацию.


# Namespace creation (:create)
crud-message-received = Получено CRUD-сообщение
crud-acl-updated = Корневой транспортный ACL обновлён

# CRUD validation errors
cidv1-required = значение должно быть голым CIDv1 (начинается с 'b'; CIDv0 'Qm…' не принимается)
config-key-protected = ключ config '%key%' защищён
config-key-no-delete = ключ config '%key%' демона не может быть удалён
config-key-not-manifest = ключ config '%key%' не является известным ключом manifest config
wrong-crud-protocol = неверный протокол CRUD: %type%
entity-name-invalid = имя entity должно быть печатным UTF-8
reserved-entity-name = имя entity '%name%' зарезервировано
genesis-kind-owner-only = Только владелец runtime может создать entity типа genesis

# IPv6 config
ipv6-enabled = IPv6 включён — привязка к IPv4 и IPv6 одновременно
ipv6-disabled = IPv6 отключён — привязывается только IPv4 (для повторного включения необходим restart)
ipv6-enable-restart-required = Сохранено. Для применения этого изменения необходим restart.
ipv6-enable-unchanged = ipv6_enable уже установлен в это значение — изменений нет.

boot-default-root-config-populate-failed = Невозможно заполнить корневую конфигурацию по умолчанию
boot-default-root-config-populated = Корневая конфигурация по умолчанию заполнена
boot-entity-load-processed = Плагины сущностей загружены
boot-group-load-failed = Не удалось загрузить группу при запуске
boot-group-loaded-into-cache = Группа загружена в кэш
boot-kinds-overlay-applied = Наложение kinds применено
boot-kinds-overlay-no-change = Наложение kinds не внесло изменений в манифест
boot-load-manifest-for-acl-cache-failed = Не удалось загрузить манифест для заполнения кэша ACL
boot-minimal-manifest-bootstrapped = Минимальный манифест инициализирован
boot-minimal-manifest-not-found = Корневой CID среды выполнения не найден в IPNS; инициализация минимального манифеста
boot-no-root-entity = Корневая сущность для корневой конфигурации по умолчанию не зарегистрирована
boot-reconciled-owners-manifest-failed = Не удалось согласовать владельцев в манифесте при запуске
boot-reconciled-owners-persist-failed = Не удалось сохранить согласованных владельцев в config.yaml
boot-reconciled-owners-published = Владельцы из config.yaml/--owner согласованы в манифесте при запуске
boot-root-acl-load-cache-failed = Не удалось загрузить корневой ACL при запуске
boot-root-acl-load-failed = Не удалось загрузить корневой ACL из манифеста
boot-root-acl-loaded-from-manifest = Корневой транспортный ACL загружен из манифеста
boot-root-acl-loaded-into-cache = Корневой ACL загружен в кэш
bootstrap-acl-published = Узел ACL опубликован
bootstrap-endpoint-close-stuck = Endpoint всё ещё удерживается выполняемыми задачами через 10 с; принудительное закрытие
bootstrap-endpoint-close-timeout = Закрытие endpoint истекло через 5 с; принудительный выход
bootstrap-entity-lifecycle-update-failed = Не удалось записать обновлённый жизненный цикл сущности в IPFS
bootstrap-entity-lifecycle-updated = Жизненный цикл сущности обновлён в IPFS
bootstrap-entity-node-shutdown-updated = Узел сущности обновлён при завершении работы
bootstrap-entity-published = Узел сущности опубликован
bootstrap-entity-registering-prepublished = Регистрация предварительно опубликованной сущности
bootstrap-entity-registry-fetch-failed = Не удалось получить узел сущности
bootstrap-entity-registry-kind-extends-failed = Не удалось разрешить цепочку расширений kind
bootstrap-entity-registry-kind-fetch-failed = Не удалось получить узел kind
bootstrap-entity-registry-kind-missing = Kind не найден в манифесте; сущность пропущена
bootstrap-entity-registry-not-in-manifest = Сущность есть в реестре, но не в манифесте, пропускается
bootstrap-entity-state-save-failed = Не удалось сохранить состояния сущностей
bootstrap-entity-state-shutdown-aborted = Завершение работы прервано; среда выполнения остаётся активной, чтобы состояние можно было сохранить при следующей попытке
bootstrap-entity-state-update-fetch-failed = Не удалось получить узел сущности для обновления состояния
bootstrap-envelope-delivery-failed = Доставка конверта плагина не удалась; конверт отброшен
bootstrap-envelope-open-failed = Конверт плагина: не удалось открыть исходящую почту; конверт отброшен
bootstrap-group-published = Узел группы опубликован
bootstrap-kind-published = Узел kind опубликован
bootstrap-kind-registry-extends-failed = Не удалось разрешить цепочку расширений kind для реестра
bootstrap-kind-registry-fetch-log-failed = Не удалось получить узел kind для реестра
bootstrap-kind-registry-hydrated = Реестр kinds наполнен из манифеста
bootstrap-kinds-overlay-pin-update-failed = Пин/обновление не удалось после наложения kinds
bootstrap-kinds-overlay-published = Манифест среды выполнения опубликован после наложения kinds
bootstrap-kinds-tree-published = Дерево kinds среды выполнения опубликовано
bootstrap-lifecycle-manifest-pin-update-failed = Пин/обновление не удалось после сохранения жизненного цикла
bootstrap-lifecycle-manifest-publish-failed = Не удалось опубликовать манифест после переходов жизненного цикла
bootstrap-lifecycle-manifest-published = Обновлённый манифест опубликован после переходов жизненного цикла
bootstrap-manifest-fetch-failed = Не удалось получить манифест среды выполнения
bootstrap-minimal-manifest-failed = Не удалось инициализировать минимальный манифест
bootstrap-remote-root-pin-confirmed = Удалённый корневой пин подтверждён
bootstrap-remote-root-pin-misconfigured = Удалённый корневой пин настроен неверно
bootstrap-root-acl-published = Корневой транспортный ACL опубликован
bootstrap-root-cid-shutdown-persist-failed = Не удалось сохранить root_cid при завершении работы
bootstrap-root-cid-shutdown-publish-failed = Публикация runtime_ipns при завершении работы не удалась
bootstrap-root-cid-shutdown-publish-succeeded = Публикация runtime_ipns при завершении работы выполнена успешно
bootstrap-root-cid-shutdown-publish-timeout = Публикация runtime_ipns при завершении работы истекла
bootstrap-root-pin-replacement-failed = Продолжение после ошибки замены удалённого корневого пина
bootstrap-root-pin-update-failed = Пин/обновление не удалось после начальной загрузки
bootstrap-runtime-manifest-published = Корневой манифест среды выполнения опубликован
crud-message-rejected = Сообщение CRUD отклонено
entity-reload-current-node-load-failed = Не удалось загрузить текущий узел сущности перед перезагрузкой; текущий плагин сохранён
entity-reload-failed = Не удалось перезагрузить сущность; отключена до следующей перезагрузки
entity-reload-kind-extends-failed = Не удалось разрешить цепочку расширений kind при перезагрузке сущности
entity-reload-kind-fetch-failed = Не удалось получить узел kind при перезагрузке сущности
entity-reload-kind-lookup-failed = Не удалось загрузить манифест для поиска kind при перезагрузке сущности
entity-reload-kind-missing = Kind не найден в манифесте; невозможно перезагрузить сущность
entity-reload-manifest-state-update-failed = Не удалось обновить манифест с текущим состоянием перед перезагрузкой; текущий плагин сохранён
entity-reload-skipped = Перезагрузка сущности пропущена, так как шлюз перезагрузки закрыт
entity-reload-started = Перезагрузка сущности начата
entity-reload-state-persist-failed = Не удалось сохранить текущее состояние перед перезагрузкой; текущий плагин сохранён
entity-reload-state-produced-failed = Не удалось сохранить состояние, созданное при перезагрузке
entity-reloaded-manifest-update-failed = Не удалось обновить перезагруженную сущность в манифесте
entity-reloaded-manifest-updated = Перезагруженная сущность обновлена в манифесте
inbox-message-rejected = Входящее сообщение отклонено
ma-create-entity-already-exists = ma_create_entity: сущность уже существует; текущая сущность сохранена
ma-create-entity-invalid-behaviour = ma_create_entity: недопустимая ссылка на behaviour; пропущено
ma-create-entity-kind-missing = ma_create_entity: kind отсутствует в реестре; пропущено
manifest-pin-update-failed = Ошибка pin_update манифеста
plugin-envelope-build-failed = Конверт плагина: не удалось создать сообщение; пропущено
plugin-envelope-create-requests-ignored = Конверт плагина: запросы на создание игнорируются без контекста побочных эффектов
plugin-envelope-local-dispatch-failed = Конверт плагина: локальная диспетчеризация не удалась
plugin-envelope-local-dispatch-finish = Конверт плагина: локальная диспетчеризация завершена
plugin-envelope-local-dispatch-start = Конверт плагина: локальная диспетчеризация начата
plugin-envelope-local-gate-closed = Конверт плагина: шлюз локальной диспетчеризации закрыт
plugin-envelope-local-recipient-unknown = Конверт плагина: неизвестный локальный получатель; пропущено
plugin-envelope-local-timeout = Конверт плагина: локальная диспетчеризация истекла
plugin-envelope-recipient-invalid = Конверт плагина: недопустимый DID получателя; пропущено
plugin-envelope-remote-limit = Конверт плагина: достигнут лимит удалённой доставки; конверт отброшен
plugin-outbox-congested = Исходящая почта плагина перегружена; конверты могут быть отброшены при заполнении канала
plugin-outbox-drain-limit = Бюджет опустошения исходящей почты плагина исчерпан; оставшиеся конверты отложены
schedule-dispatch-firing = Запуск запланированной диспетчеризации
schedule-entity-not-found = Запланированная диспетчеризация: сущность не найдена
schedule-random-chain-stopped = Цепочка случайного расписания остановлена: заменена более новым определением
schedule-random-create-failed = Не удалось создать следующее случайное задание
schedule-random-reschedule-failed = Не удалось перепланировать случайное задание
schedule-stale-dispatch-skipped = Запланированная диспетчеризация пропущена: устаревшее расписание
scheduled-dispatch-error = Ошибка запланированной диспетчеризации
scheduled-dispatch-manifest-writer-unavailable = Запланированная диспетчеризация: записывающий манифест не готов; состояние сущности ожидает
