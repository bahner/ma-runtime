# ma-runtime – فارسی
lang-name = فارسی

own-did-published = سند DID خود runtime در IPNS منتشر شد
own-did-publish-failed = انتشار سند DID خود runtime ناموفق بود
own-did-publish-timeout = مهلت انتشار سند DID خود runtime پس از ۲ دقیقه پایان یافت
started = زمان اجرای ma آغاز شد
shutdown-requested = درخواست خاموشی دریافت شد
closing-endpoint = در حال بستن نقطه پایانی iroh...
shutdown-complete = خاموشی کامل شد
status-listening = سرور وضعیت در حال گوش دادن است
rpc-message-received = پیام RPC دریافت شد
rpc-message-rejected = پیام RPC رد شد
crud-message-received = پیام CRUD دریافت شد
crud-acl-updated = ACL انتقال ریشه به‌روزرسانی شد
ipfs-message-rejected = پیام IPFS رد شد
ctrlc-handler-failed = مدیریت‌کننده Ctrl-C ناموفق بود
node-connected = گره به پروتکل وصل شد
received-encrypted-ma-msg = پیام رمزگذاری‌شده ma-msg روی /ma/ipfs/0.0.1 دریافت شد
unknown-rpc-atom = اتم RPC ناشناخته است؛ نادیده گرفته می‌شود
rpc-not-text-atom = بار RPC یک اتم متنی نیست
rpc-unknown-verb = فعل RPC ناشناخته است
rpc-reply-sent = پاسخ RPC فرستاده شد
ping-received = :ping دریافت شد؛ در حال فرستادن :pong
did-publish-request-received = درخواست انتشار سند DID دریافت شد
document-published = سند منتشر شد
did-publish-cid-reply-sent = پاسخ CID برای انتشار DID فرستاده شد
did-publish-resolve-failed = حل فرستنده برای رساندن پاسخ ipfs-publish ناموفق بود
ipfs-store-request-received = درخواست ذخیره‌سازی IPFS دریافت شد
ipfs-stored = محتوا در IPFS ذخیره شد
ipfs-store-cid-reply-sent = پاسخ CID فرستاده شد
ipfs-store-resolve-failed = حل فرستنده برای رساندن پاسخ ipfs-store ناموفق بود

# Entity dispatch
bootstrap-complete = Bootstrap کامل شد
entity-loaded = افزونه موجودیت بارگذاری شد
entity-load-failed = بارگذاری افزونه موجودیت ناموفق بود
entity-not-found = موجودیت پیدا نشد؛ RPC نادیده گرفته می‌شود
entity-dispatched = RPC به موجودیت فرستاده شد
entity-replied = موجودیت پاسخ RPC فرستاد
root-create-entity = #root: ایجاد موجودیت
root-list-entities = #root: فهرست موجودیت‌ها
root-delete-entity = #root: حذف موجودیت
root-entity-updated = manifest زمان اجرا به‌روزرسانی شد
default-config-root-populated = /config/root پیش‌فرض هنگام آغاز به کار پر شد
default-config-root-no-root-entity = نمی‌توان /config/root پیش‌فرض را هنگام آغاز به کار پر کرد: موجودیت #root بارگذاری نشده است
default-config-root-no-root-cid = نمی‌توان /config/root پیش‌فرض را هنگام آغاز به کار پر کرد: CID ریشه manifest در دسترس نیست
default-config-root-inspect-failed = بررسی manifest پیش از پر کردن /config/root پیش‌فرض ناموفق بود
default-config-root-populate-failed = پر کردن /config/root پیش‌فرض هنگام آغاز به کار ناموفق بود
entity-created = موجودیت ایجاد شد
entity-reloaded = افزونه موجودیت دوباره بارگذاری شد
entity-deleted = موجودیت حذف شد
entity-states-saving = در حال ذخیره حالت‌های موجودیت‌ها در IPFS
entity-state-saving = در حال ذخیره حالت موجودیت
entity-state-saved = حالت موجودیت ذخیره شد
entity-state-empty = افزونه حالت خالی برگرداند؛ ذخیره‌سازی رد می‌شود
entity-states-saved = حالت‌های موجودیت‌ها ذخیره شدند
link-set = پیوند تنظیم شد
ftl-loaded = پیام‌های زبان از IPFS بارگذاری شدند

# First-run auto-init
no-config-found = پیکربندی پیدا نشد.
initialising-new-identity = در حال مقداردهی اولیه هویت تازه runtime.
generated-headless-config = پیکربندی headless ساخته شد.

# Ownership / claim
runtime-claimed = runtime مطالبه شد.

# Protected root elements
refuse-delete-root = با قاطعیت از حذف عنصر ریشه ضروری خودداری می‌شود
no-root-acl = ACL ریشه پیکربندی نشده است — runtime بدون کنترل دسترسی کار می‌کند
acl-owners-access = به فراخواننده به عنوان عضو +owners دسترسی داده شد
runtime-claim-persisted = مالک در پیکربندی نوشته شد.
runtime-already-claimed = runtime پیش‌تر مطالبه شده است.

# Namespace creation (:create)

# CRUD validation errors
blob-value-ipfs-path = مقدار blob باید مسیر IPFS باشد (/ipfs/، /ipns/ یا /ipld/)
acl-value-ipfs-path = مقدار ACL باید مسیر IPFS باشد (/ipfs/، /ipns/ یا /ipld/)
kind-value-ipfs-path = مقدار kind باید مسیر IPFS باشد (/ipfs/، /ipns/ یا /ipld/)
cidv1-required = مقدار باید CIDv1 خام باشد (با 'b' آغاز شود؛ CIDv0 با 'Qm…' پذیرفته نیست)
kind-not-found = Kind پیدا نشد
config-key-protected = کلید config با نام '%key%' محافظت‌شده است
config-key-no-delete = کلید config مربوط به daemon با نام '%key%' حذف‌شدنی نیست
config-key-not-manifest = کلید config با نام '%key%' یک کلید manifest config شناخته‌شده نیست
owners-value-not-list = مقدار owners باید فهرستی از DIDها باشد، نه یک مقدار تنها
wrong-crud-protocol = پروتکل CRUD نادرست است: %type%
entity-name-invalid = نام موجودیت باید UTF-8 قابل چاپ باشد
reserved-entity-name = نام موجودیت '%name%' رزرو شده است
genesis-kind-owner-only = فقط مالک runtime می‌تواند موجودیتی از kind پیدایشی بسازد

# IPv6 config
ipv6-enabled = IPv6 فعال شد — اتصال هم به IPv4 و هم IPv6 انجام می‌شود
ipv6-disabled = IPv6 غیرفعال شد — فقط IPv4 متصل می‌شود (برای فعال‌سازی دوباره، راه‌اندازی مجدد لازم است)
ipv6-enable-restart-required = ذخیره شد. برای اثر کردن این تغییر، راه‌اندازی مجدد لازم است.
ipv6-enable-unchanged = ipv6_enable از قبل روی همین مقدار تنظیم شده است — تغییری انجام نشد.
