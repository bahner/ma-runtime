# ma-runtime – اردو
lang-name = اردو

own-did-published = اپنا DID دستاویز IPNS پر شائع کیا گیا
own-did-publish-failed = اپنا DID دستاویز شائع کرنے میں ناکامی
own-did-publish-timeout = اپنے DID دستاویز کی اشاعت 2 منٹ بعد ٹائم آؤٹ ہوئی
started = ma runtime شروع ہوا
shutdown-requested = بند کرنے کی درخواست کی گئی
closing-endpoint = iroh اینڈ پوائنٹ بند ہو رہا ہے...
shutdown-complete = بند کرنا مکمل ہوا
status-listening = اسٹیٹس سرور سن رہا ہے
ipfs-message-rejected = IPFS پیغام مسترد کیا گیا
ctrlc-handler-failed = Ctrl-C ہینڈلر ناکام رہا
node-connected = نوڈ پروٹوکول سے جڑا
received-encrypted-ma-msg = /ma/ipfs/0.0.1 پر خفیہ کردہ ma پیغام موصول ہوا
ping-received = :ping موصول ہوا، :pong بھیج رہے ہیں
did-publish-request-received = DID دستاویز اشاعت کی درخواست موصول ہوئی
document-published = دستاویز شائع ہوا
did-publish-cid-reply-sent = DID اشاعت کے لیے CID جواب بھیجا گیا
did-publish-resolve-failed = ipfs-publish جواب پہنچانے کے لیے بھیجنے والے کو حل کرنے میں ناکامی
ipfs-store-request-received = IPFS اسٹوریج کی درخواست موصول ہوئی
ipfs-stored = مواد IPFS پر محفوظ کیا گیا
ipfs-store-cid-reply-sent = CID جواب بھیجا گیا
ipfs-store-resolve-failed = ipfs-store جواب پہنچانے کے لیے بھیجنے والے کو حل کرنے میں ناکامی

# اینٹٹی ڈسپیچ
bootstrap-complete = Bootstrap مکمل ہوا
entity-loaded = اینٹٹی پلگ ان لوڈ ہوا
entity-load-failed = اینٹٹی پلگ ان لوڈ کرنے میں ناکامی
root-list-entities = #root: اینٹٹی فہرست
entity-created = اینٹٹی بنائی گئی
entity-reloaded = اینٹٹی پلگ ان دوبارہ لوڈ ہوا
entity-deleted = اینٹٹی حذف ہوئی
entity-states-saving = IPFS پر اینٹٹی حالتیں محفوظ ہو رہی ہیں
entity-state-saving = اینٹٹی حالت محفوظ ہو رہی ہے
entity-state-saved = اینٹٹی حالت محفوظ ہوئی
entity-state-empty = پلگ ان نے خالی حالت واپس کی، محفوظ کرنا چھوڑ دیا
entity-states-saved = اینٹٹی حالتیں محفوظ ہوئیں

# پہلی شروعات / خودکار آغاز

# ملکیت
runtime-claimed = Runtime رجسٹر ہوا۔

# محفوظ روٹ عناصر
refuse-delete-root = ضروری روٹ عنصر حذف کرنے سے قطعی انکار
runtime-claim-persisted = مالک ترتیبات میں لکھا گیا۔


# Namespace creation (:create)
crud-message-received = CRUD پیغام موصول ہوا
crud-acl-updated = روٹ ٹرانسپورٹ ACL اپ ڈیٹ ہوئی

# CRUD validation errors
cidv1-required = قدر کو ایک خام CIDv1 ہونا چاہیے ('b' سے شروع ہوتی ہے؛ CIDv0 'Qm…' قبول نہیں)
config-key-protected = config کی چابی '%key%' محفوظ ہے
config-key-no-delete = daemon config کی چابی '%key%' کو حذف نہیں کیا جا سکتا
config-key-not-manifest = config کی چابی '%key%' ایک معروف manifest config چابی نہیں ہے
wrong-crud-protocol = غلط CRUD پروٹوکول: %type%
entity-name-invalid = entity کا نام قابلِ پرنٹ UTF-8 ہونا چاہیے
reserved-entity-name = entity کا نام '%name%' محفوظ ہے
genesis-kind-owner-only = صرف runtime کا مالک genesis قسم کی entity بنا سکتا ہے

# IPv6 config
ipv6-enabled = IPv6 فعال ہے — IPv4 اور IPv6 دونوں سے منسلک ہو رہا ہے
ipv6-disabled = IPv6 بند ہے — صرف IPv4 سے منسلک ہو رہا ہے (دوبارہ فعال کرنے کے لیے restart ضروری ہے)
ipv6-enable-restart-required = محفوظ ہو گیا۔ یہ تبدیلی نافذ کرنے کے لیے restart ضروری ہے۔
ipv6-enable-unchanged = ipv6_enable پہلے سے اس قدر پر مقرر ہے — کوئی تبدیلی نہیں۔

boot-default-root-config-populate-failed = ڈیفالٹ روٹ کنفیگریشن بھرنا ناکام رہا
boot-default-root-config-populated = ڈیفالٹ روٹ کنفیگریشن بھر دی گئی
boot-entity-load-processed = انٹیٹی پلگ ان لوڈ ہو گئے
boot-group-load-failed = بوٹ کے دوران گروپ لوڈ کرنا ناکام رہا
boot-group-loaded-into-cache = گروپ کیش میں لوڈ ہو گیا
boot-kinds-overlay-applied = Kinds اوورلے لاگو ہو گیا
boot-kinds-overlay-no-change = Kinds اوورلے نے مینیفیسٹ میں کوئی تبدیلی نہیں کی
boot-load-manifest-for-acl-cache-failed = ACL کیش بھرنے کے لیے مینیفیسٹ لوڈ کرنا ناکام رہا
boot-minimal-manifest-bootstrapped = کم سے کم مینیفیسٹ شروع ہو گیا
boot-minimal-manifest-not-found = IPNS میں رن ٹائم روٹ CID نہیں ملا؛ کم سے کم مینیفیسٹ شروع کیا جا رہا ہے
boot-no-root-entity = ڈیفالٹ روٹ کنفیگریشن کے لیے کوئی روٹ انٹیٹی رجسٹر نہیں ہے
boot-reconciled-owners-manifest-failed = بوٹ کے دوران مینیفیسٹ میں مالکان کی مطابقت ناکام رہی
boot-reconciled-owners-persist-failed = مطابق مالکان کو config.yaml میں محفوظ کرنا ناکام رہا
boot-reconciled-owners-published = بوٹ کے دوران config.yaml/--owner سے مینیفیسٹ میں مالکان مطابق کیے گئے
boot-root-acl-load-cache-failed = بوٹ کے دوران روٹ ACL لوڈ کرنا ناکام رہا
boot-root-acl-load-failed = مینیفیسٹ سے روٹ ACL لوڈ کرنا ناکام رہا
boot-root-acl-loaded-from-manifest = روٹ ٹرانسپورٹ ACL مینیفیسٹ سے لوڈ ہو گیا
boot-root-acl-loaded-into-cache = روٹ ACL کیش میں لوڈ ہو گیا
bootstrap-acl-published = ACL نوڈ شائع ہو گیا
bootstrap-endpoint-close-stuck = 10 سیکنڈ بعد بھی اینڈ پوائنٹ چلتے کاموں سے روکا ہوا ہے؛ زبردستی بند کیا جا رہا ہے
bootstrap-endpoint-close-timeout = اینڈ پوائنٹ بند کرنے میں 5 سیکنڈ بعد ٹائم آؤٹ ہو گیا؛ زبردستی ختم کیا جا رہا ہے
bootstrap-entity-lifecycle-update-failed = انٹیٹی کا اپ ڈیٹ شدہ لائف سائیکل IPFS میں لکھنا ناکام رہا
bootstrap-entity-lifecycle-updated = انٹیٹی لائف سائیکل IPFS میں اپ ڈیٹ ہو گیا
bootstrap-entity-node-shutdown-updated = شٹ ڈاؤن کے دوران انٹیٹی نوڈ اپ ڈیٹ ہو گیا
bootstrap-entity-published = انٹیٹی نوڈ شائع ہو گیا
bootstrap-entity-registering-prepublished = پہلے سے شائع شدہ انٹیٹی رجسٹر ہو رہی ہے
bootstrap-entity-registry-fetch-failed = انٹیٹی نوڈ لانا ناکام رہا
bootstrap-entity-registry-kind-extends-failed = Kind ایکسٹینشن چین حل کرنا ناکام رہا
bootstrap-entity-registry-kind-fetch-failed = Kind نوڈ لانا ناکام رہا
bootstrap-entity-registry-kind-missing = مینیفیسٹ میں Kind نہیں ملا؛ انٹیٹی چھوڑ دی گئی
bootstrap-entity-registry-not-in-manifest = انٹیٹی رجسٹری میں ہے لیکن مینیفیسٹ میں نہیں؛ چھوڑ دی گئی
bootstrap-entity-state-save-failed = انٹیٹی حالتیں محفوظ کرنا ناکام رہا
bootstrap-entity-state-shutdown-aborted = شٹ ڈاؤن منسوخ ہو گیا؛ اگلے شٹ ڈاؤن میں حالت محفوظ کرنے کے لیے رن ٹائم فعال رہتا ہے
bootstrap-entity-state-update-fetch-failed = حالت اپ ڈیٹ کے لیے انٹیٹی نوڈ لانا ناکام رہا
bootstrap-envelope-delivery-failed = پلگ ان لفافے کی ترسیل ناکام رہی؛ لفافہ حذف کر دیا گیا
bootstrap-envelope-open-failed = پلگ ان لفافہ: آؤٹ گوئنگ میل باکس کھولنا ناکام رہا؛ لفافہ حذف کر دیا گیا
bootstrap-group-published = گروپ نوڈ شائع ہو گیا
bootstrap-kind-published = Kind نوڈ شائع ہو گیا
bootstrap-kind-registry-extends-failed = رجسٹری کے لیے Kind ایکسٹینشن چین حل کرنا ناکام رہا
bootstrap-kind-registry-fetch-log-failed = رجسٹری کے لیے Kind نوڈ لانا ناکام رہا
bootstrap-kind-registry-hydrated = Kinds رجسٹری مینیفیسٹ سے بھر دی گئی
bootstrap-kinds-overlay-pin-update-failed = Kinds اوورلے کے بعد پن/اپ ڈیٹ ناکام رہا
bootstrap-kinds-overlay-published = Kinds اوورلے کے بعد رن ٹائم مینیفیسٹ شائع ہو گیا
bootstrap-kinds-tree-published = رن ٹائم Kinds ٹری شائع ہو گئی
bootstrap-lifecycle-manifest-pin-update-failed = لائف سائیکل مستقل مزاجی کے بعد پن/اپ ڈیٹ ناکام رہا
bootstrap-lifecycle-manifest-publish-failed = لائف سائیکل منتقلی کے بعد مینیفیسٹ شائع کرنا ناکام رہا
bootstrap-lifecycle-manifest-published = لائف سائیکل منتقلی کے بعد اپ ڈیٹ شدہ مینیفیسٹ شائع ہو گیا
bootstrap-manifest-fetch-failed = رن ٹائم مینیفیسٹ لانا ناکام رہا
bootstrap-minimal-manifest-failed = کم سے کم مینیفیسٹ شروع کرنا ناکام رہا
bootstrap-remote-root-pin-confirmed = ریموٹ روٹ پننگ کی تصدیق ہو گئی
bootstrap-remote-root-pin-misconfigured = ریموٹ روٹ پننگ غلط طریقے سے کنفیگر کی گئی ہے
bootstrap-root-acl-published = روٹ ٹرانسپورٹ ACL شائع ہو گیا
bootstrap-root-cid-shutdown-persist-failed = شٹ ڈاؤن کے دوران root_cid محفوظ کرنا ناکام رہا
bootstrap-root-cid-shutdown-publish-failed = شٹ ڈاؤن کے دوران runtime_ipns شائع کرنا ناکام رہا
bootstrap-root-cid-shutdown-publish-succeeded = شٹ ڈاؤن کے دوران runtime_ipns اشاعت کامیاب رہی
bootstrap-root-cid-shutdown-publish-timeout = شٹ ڈاؤن کے دوران runtime_ipns اشاعت کا وقت ختم ہو گیا
bootstrap-root-pin-replacement-failed = ریموٹ روٹ پن متبادل کی خرابی کے بعد جاری ہے
bootstrap-root-pin-update-failed = بوٹ سٹریپ کے بعد پن/اپ ڈیٹ ناکام رہا
bootstrap-runtime-manifest-published = رن ٹائم روٹ مینیفیسٹ شائع ہو گیا
crud-message-rejected = CRUD پیغام مسترد ہو گیا
entity-reload-current-node-load-failed = ریلوڈ سے پہلے موجودہ انٹیٹی نوڈ لانا ناکام رہا؛ موجودہ پلگ ان برقرار رکھا گیا
entity-reload-failed = انٹیٹی ریلوڈ ناکام رہا؛ اگلے ریلوڈ تک غیر فعال
entity-reload-kind-extends-failed = انٹیٹی ریلوڈ کے دوران Kind ایکسٹینشن چین حل کرنا ناکام رہا
entity-reload-kind-fetch-failed = انٹیٹی ریلوڈ کے دوران Kind نوڈ لانا ناکام رہا
entity-reload-kind-lookup-failed = انٹیٹی ریلوڈ کے دوران Kind تلاش کے لیے مینیفیسٹ لانا ناکام رہا
entity-reload-kind-missing = مینیفیسٹ میں Kind نہیں ملا؛ انٹیٹی ریلوڈ نہیں ہو سکتی
entity-reload-manifest-state-update-failed = ریلوڈ سے پہلے موجودہ حالت کے ساتھ مینیفیسٹ اپ ڈیٹ کرنا ناکام رہا؛ موجودہ پلگ ان برقرار رکھا گیا
entity-reload-skipped = ریلوڈ گیٹ بند ہونے کی وجہ سے انٹیٹی ریلوڈ چھوڑ دیا گیا
entity-reload-started = انٹیٹی ریلوڈ شروع ہو گیا
entity-reload-state-persist-failed = ریلوڈ سے پہلے موجودہ حالت محفوظ کرنا ناکام رہا؛ موجودہ پلگ ان برقرار رکھا گیا
entity-reload-state-produced-failed = ریلوڈ کے دوران پیدا شدہ حالت محفوظ کرنا ناکام رہا
entity-reloaded-manifest-update-failed = مینیفیسٹ میں ریلوڈ شدہ انٹیٹی اپ ڈیٹ کرنا ناکام رہا
entity-reloaded-manifest-updated = ریلوڈ شدہ انٹیٹی مینیفیسٹ میں اپ ڈیٹ ہو گئی
inbox-message-rejected = ان باکس پیغام مسترد ہو گیا
ma-create-entity-already-exists = ma_create_entity: انٹیٹی پہلے سے موجود ہے؛ موجودہ انٹیٹی برقرار رکھی گئی
ma-create-entity-invalid-behaviour = ma_create_entity: غیر درست رویے کا حوالہ؛ چھوڑ دیا گیا
ma-create-entity-kind-missing = ma_create_entity: رجسٹری میں Kind نہیں ہے؛ چھوڑ دیا گیا
manifest-pin-update-failed = مینیفیسٹ pin_update ناکام رہا
plugin-envelope-build-failed = پلگ ان لفافہ: پیغام بنانا ناکام رہا؛ چھوڑ دیا گیا
plugin-envelope-create-requests-ignored = پلگ ان لفافہ: سائیڈ ایفیکٹ سیاق و سباق کے بغیر بنانے کی درخواستیں نظر انداز کی گئیں
plugin-envelope-local-dispatch-failed = پلگ ان لفافہ: مقامی ڈسپیچ ناکام رہا
plugin-envelope-local-dispatch-finish = پلگ ان لفافہ: مقامی ڈسپیچ مکمل ہو گیا
plugin-envelope-local-dispatch-start = پلگ ان لفافہ: مقامی ڈسپیچ شروع ہو گیا
plugin-envelope-local-gate-closed = پلگ ان لفافہ: مقامی ڈسپیچ گیٹ بند ہے
plugin-envelope-local-recipient-unknown = پلگ ان لفافہ: نامعلوم مقامی وصول کنندہ؛ چھوڑ دیا گیا
plugin-envelope-local-timeout = پلگ ان لفافہ: مقامی ڈسپیچ کا وقت ختم ہو گیا
plugin-envelope-recipient-invalid = پلگ ان لفافہ: غیر درست وصول کنندہ DID؛ چھوڑ دیا گیا
plugin-envelope-remote-limit = پلگ ان لفافہ: ریموٹ ڈیلیوری حد پہنچ گئی؛ لفافہ حذف کر دیا گیا
plugin-outbox-congested = پلگ ان آؤٹ باکس بھرا ہوا ہے؛ چینل بھرنے پر لفافے حذف ہو سکتے ہیں
plugin-outbox-drain-limit = پلگ ان آؤٹ باکس نکاسی بجٹ ختم ہو گیا؛ باقی لفافے ملتوی کیے گئے
schedule-dispatch-firing = مقررہ ڈسپیچ جاری ہے
schedule-entity-not-found = مقررہ ڈسپیچ: انٹیٹی نہیں ملی
schedule-random-chain-stopped = بے ترتیب مقررہ چین رک گئی: نئی تعریف سے بدل دی گئی
schedule-random-create-failed = اگلا بے ترتیب کام بنانا ناکام رہا
schedule-random-reschedule-failed = بے ترتیب کام دوبارہ مقرر کرنا ناکام رہا
schedule-stale-dispatch-skipped = مقررہ ڈسپیچ چھوڑ دیا گیا: پرانا شیڈول
scheduled-dispatch-error = مقررہ ڈسپیچ میں خرابی
scheduled-dispatch-manifest-writer-unavailable = مقررہ ڈسپیچ: مینیفیسٹ رائٹر تیار نہیں؛ انٹیٹی حالت انتظار میں ہے
