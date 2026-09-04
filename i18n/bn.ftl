# ma-runtime – বাংলা
lang-name = বাংলা

own-did-published = নিজের DID দলিল IPNS-এ প্রকাশিত হয়েছে
own-did-publish-failed = নিজের DID দলিল প্রকাশ করতে ব্যর্থ
own-did-publish-timeout = নিজের DID দলিল প্রকাশ ২ মিনিট পরে টাইমআউট হয়েছে
started = ma runtime শুরু হয়েছে
shutdown-requested = বন্ধ করার অনুরোধ করা হয়েছে
closing-endpoint = iroh এন্ডপয়েন্ট বন্ধ হচ্ছে...
shutdown-complete = বন্ধ করা সম্পন্ন হয়েছে
status-listening = স্ট্যাটাস সার্ভার শুনছে
ipfs-message-rejected = IPFS বার্তা প্রত্যাখ্যাত হয়েছে
ctrlc-handler-failed = Ctrl-C হ্যান্ডলার ব্যর্থ হয়েছে
node-connected = নোড প্রোটোকলে সংযুক্ত হয়েছে
received-encrypted-ma-msg = /ma/ipfs/0.0.1-এ এনক্রিপ্টেড ma বার্তা প্রাপ্ত হয়েছে
ping-received = :ping প্রাপ্ত হয়েছে, :pong পাঠানো হচ্ছে
did-publish-request-received = DID দলিল প্রকাশের অনুরোধ প্রাপ্ত হয়েছে
document-published = দলিল প্রকাশিত হয়েছে
did-publish-cid-reply-sent = DID প্রকাশের জন্য CID উত্তর পাঠানো হয়েছে
did-publish-resolve-failed = ipfs-publish উত্তর দিতে প্রেরক সমাধান করতে ব্যর্থ
ipfs-store-request-received = IPFS স্টোরেজ অনুরোধ প্রাপ্ত হয়েছে
ipfs-stored = কন্টেন্ট IPFS-এ সংরক্ষিত হয়েছে
ipfs-store-cid-reply-sent = CID উত্তর পাঠানো হয়েছে
ipfs-store-resolve-failed = ipfs-store উত্তর দিতে প্রেরক সমাধান করতে ব্যর্থ

# এন্টিটি ডিসপ্যাচ
bootstrap-complete = Bootstrap সম্পন্ন হয়েছে
entity-loaded = এন্টিটি প্লাগইন লোড হয়েছে
entity-load-failed = এন্টিটি প্লাগইন লোড করতে ব্যর্থ
root-list-entities = #root: এন্টিটি তালিকা
entity-created = এন্টিটি তৈরি হয়েছে
entity-reloaded = এন্টিটি প্লাগইন পুনরায় লোড হয়েছে
entity-deleted = এন্টিটি মুছে গেছে
entity-states-saving = IPFS-এ এন্টিটি অবস্থা সংরক্ষণ হচ্ছে
entity-state-saving = এন্টিটি অবস্থা সংরক্ষণ হচ্ছে
entity-state-saved = এন্টিটি অবস্থা সংরক্ষিত হয়েছে
entity-state-empty = প্লাগইন খালি অবস্থা ফেরত দিয়েছে, সংরক্ষণ এড়িয়ে গেছে
entity-states-saved = এন্টিটি অবস্থাসমূহ সংরক্ষিত হয়েছে

# প্রথম স্টার্টআপ / স্বয়ংক্রিয়-ইনিট

# মালিকানা
runtime-claimed = Runtime নিবন্ধিত হয়েছে।

# সুরক্ষিত রুট উপাদান
refuse-delete-root = প্রয়োজনীয় রুট উপাদান মুছতে দৃঢ়ভাবে অস্বীকার করুন
runtime-claim-persisted = মালিক কনফিগারেশনে লেখা হয়েছে।


# Namespace creation (:create)
crud-message-received = CRUD বার্তা পাওয়া গেছে
crud-acl-updated = রুট ট্রান্সপোর্ট ACL আপডেট হয়েছে

# CRUD validation errors
cidv1-required = মানটি অবশ্যই একটি খাঁটি CIDv1 হতে হবে ('b' দিয়ে শুরু; CIDv0 'Qm…' গৃহীত নয়)
config-key-protected = config কী '%key%' সুরক্ষিত
config-key-no-delete = daemon config কী '%key%' মুছে ফেলা যাবে না
config-key-not-manifest = config কী '%key%' একটি পরিচিত manifest config কী নয়
wrong-crud-protocol = ভুল CRUD প্রোটোকল: %type%
entity-name-invalid = entity নামটি অবশ্যই মুদ্রণযোগ্য UTF-8 হতে হবে
reserved-entity-name = entity নাম '%name%' সংরক্ষিত
genesis-kind-owner-only = শুধুমাত্র ma runtime-এর মালিক genesis ধরনের entity তৈরি করতে পারেন

# IPv6 config
ipv6-enabled = IPv6 সক্রিয় — IPv4 এবং IPv6 উভয়ই বাঁধা হচ্ছে
ipv6-disabled = IPv6 নিষ্ক্রিয় — শুধুমাত্র IPv4 বাঁধাই হচ্ছে (পুনরায় সক্রিয় করতে restart প্রয়োজন)
ipv6-enable-restart-required = সংরক্ষিত হয়েছে। এই পরিবর্তন কার্যকর হতে restart প্রয়োজন।
ipv6-enable-unchanged = ipv6_enable ইতিমধ্যে সেই মানে সেট করা আছে — কোনো পরিবর্তন নেই।

boot-default-root-config-populate-failed = ডিফল্ট রুট কনফিগারেশন পূরণ করতে ব্যর্থ হয়েছে
boot-default-root-config-populated = ডিফল্ট রুট কনফিগারেশন পূরণ হয়েছে
boot-entity-load-processed = এনটিটি প্লাগইন লোড হয়েছে
boot-group-load-failed = বুট করার সময় গ্রুপ লোড করতে ব্যর্থ হয়েছে
boot-group-loaded-into-cache = গ্রুপ ক্যাশে লোড হয়েছে
boot-kinds-overlay-applied = Kinds ওভারলে প্রয়োগ হয়েছে
boot-kinds-overlay-no-change = Kinds ওভারলে ম্যানিফেস্টে কোনো পরিবর্তন করেনি
boot-load-manifest-for-acl-cache-failed = ACL ক্যাশ পূরণ করতে ম্যানিফেস্ট লোড করতে ব্যর্থ হয়েছে
boot-minimal-manifest-bootstrapped = ন্যূনতম ম্যানিফেস্ট শুরু হয়েছে
boot-minimal-manifest-not-found = IPNS-এ রানটাইম রুট CID পাওয়া যায়নি; ন্যূনতম ম্যানিফেস্ট শুরু করা হচ্ছে
boot-no-root-entity = ডিফল্ট রুট কনফিগারেশনের জন্য কোনো রুট এনটিটি নিবন্ধিত নেই
boot-reconciled-owners-manifest-failed = বুট করার সময় ম্যানিফেস্টে মালিকদের সমন্বয় করতে ব্যর্থ হয়েছে
boot-reconciled-owners-persist-failed = সমন্বিত মালিকদের config.yaml-এ সংরক্ষণ করতে ব্যর্থ হয়েছে
boot-reconciled-owners-published = বুট করার সময় config.yaml/--owner থেকে ম্যানিফেস্টে মালিকরা সমন্বিত হয়েছে
boot-root-acl-load-cache-failed = বুট করার সময় রুট ACL লোড করতে ব্যর্থ হয়েছে
boot-root-acl-load-failed = ম্যানিফেস্ট থেকে রুট ACL লোড করতে ব্যর্থ হয়েছে
boot-root-acl-loaded-from-manifest = রুট ট্রান্সপোর্ট ACL ম্যানিফেস্ট থেকে লোড হয়েছে
boot-root-acl-loaded-into-cache = রুট ACL ক্যাশে লোড হয়েছে
bootstrap-acl-published = ACL নোড প্রকাশিত হয়েছে
bootstrap-endpoint-close-stuck = 10 সেকেন্ড পরেও এন্ডপয়েন্ট চলমান কাজের কারণে আটকে আছে; জোর করে বন্ধ করা হচ্ছে
bootstrap-endpoint-close-timeout = এন্ডপয়েন্ট বন্ধ করতে 5 সেকেন্ড পরে টাইমআউট হয়েছে; জোর করে সমাপ্ত করা হচ্ছে
bootstrap-entity-lifecycle-update-failed = আপডেট করা এনটিটি জীবনচক্র IPFS-এ লিখতে ব্যর্থ হয়েছে
bootstrap-entity-lifecycle-updated = এনটিটি জীবনচক্র IPFS-এ আপডেট হয়েছে
bootstrap-entity-node-shutdown-updated = শাটডাউনের সময় এনটিটি নোড আপডেট হয়েছে
bootstrap-entity-published = এনটিটি নোড প্রকাশিত হয়েছে
bootstrap-entity-registering-prepublished = পূর্ব-প্রকাশিত এনটিটি নিবন্ধন করা হচ্ছে
bootstrap-entity-registry-fetch-failed = এনটিটি নোড আনতে ব্যর্থ হয়েছে
bootstrap-entity-registry-kind-extends-failed = Kind এক্সটেনশন চেইন সমাধান করতে ব্যর্থ হয়েছে
bootstrap-entity-registry-kind-fetch-failed = Kind নোড আনতে ব্যর্থ হয়েছে
bootstrap-entity-registry-kind-missing = ম্যানিফেস্টে Kind পাওয়া যায়নি; এনটিটি বাদ দেওয়া হয়েছে
bootstrap-entity-registry-not-in-manifest = এনটিটি রেজিস্ট্রিতে আছে কিন্তু ম্যানিফেস্টে নেই; বাদ দেওয়া হয়েছে
bootstrap-entity-state-save-failed = এনটিটি অবস্থা সংরক্ষণ করতে ব্যর্থ হয়েছে
bootstrap-entity-state-shutdown-aborted = শাটডাউন বাতিল করা হয়েছে; পরবর্তী শাটডাউনে অবস্থা সংরক্ষণ করতে রানটাইম সক্রিয় থাকে
bootstrap-entity-state-update-fetch-failed = অবস্থা আপডেটের জন্য এনটিটি নোড আনতে ব্যর্থ হয়েছে
bootstrap-envelope-delivery-failed = প্লাগইন এনভেলপ ডেলিভারি ব্যর্থ হয়েছে; এনভেলপ বাদ দেওয়া হয়েছে
bootstrap-envelope-open-failed = প্লাগইন এনভেলপ: আউটগোইং মেইলবক্স খুলতে ব্যর্থ হয়েছে; এনভেলপ বাদ দেওয়া হয়েছে
bootstrap-group-published = গ্রুপ নোড প্রকাশিত হয়েছে
bootstrap-kind-published = Kind নোড প্রকাশিত হয়েছে
bootstrap-kind-registry-extends-failed = রেজিস্ট্রির জন্য Kind এক্সটেনশন চেইন সমাধান করতে ব্যর্থ হয়েছে
bootstrap-kind-registry-fetch-log-failed = রেজিস্ট্রির জন্য Kind নোড আনতে ব্যর্থ হয়েছে
bootstrap-kind-registry-hydrated = Kinds রেজিস্ট্রি ম্যানিফেস্ট থেকে পূরণ হয়েছে
bootstrap-kinds-overlay-pin-update-failed = Kinds ওভারলের পরে পিন/আপডেট ব্যর্থ হয়েছে
bootstrap-kinds-overlay-published = Kinds ওভারলের পরে রানটাইম ম্যানিফেস্ট প্রকাশিত হয়েছে
bootstrap-kinds-tree-published = রানটাইম Kinds ট্রি প্রকাশিত হয়েছে
bootstrap-lifecycle-manifest-pin-update-failed = জীবনচক্র স্থায়িত্বের পরে পিন/আপডেট ব্যর্থ হয়েছে
bootstrap-lifecycle-manifest-publish-failed = জীবনচক্র রূপান্তরের পরে ম্যানিফেস্ট প্রকাশ করতে ব্যর্থ হয়েছে
bootstrap-lifecycle-manifest-published = জীবনচক্র রূপান্তরের পরে আপডেট ম্যানিফেস্ট প্রকাশিত হয়েছে
bootstrap-manifest-fetch-failed = রানটাইম ম্যানিফেস্ট আনতে ব্যর্থ হয়েছে
bootstrap-minimal-manifest-failed = ন্যূনতম ম্যানিফেস্ট শুরু করতে ব্যর্থ হয়েছে
bootstrap-remote-root-pin-confirmed = রিমোট রুট পিনিং নিশ্চিত হয়েছে
bootstrap-remote-root-pin-misconfigured = রিমোট রুট পিনিং ভুলভাবে কনফিগার করা হয়েছে
bootstrap-root-acl-published = রুট ট্রান্সপোর্ট ACL প্রকাশিত হয়েছে
bootstrap-root-cid-shutdown-persist-failed = শাটডাউনের সময় root_cid সংরক্ষণ করতে ব্যর্থ হয়েছে
bootstrap-root-cid-shutdown-publish-failed = শাটডাউনের সময় runtime_ipns প্রকাশ করতে ব্যর্থ হয়েছে
bootstrap-root-cid-shutdown-publish-succeeded = শাটডাউনের সময় runtime_ipns প্রকাশ সফল হয়েছে
bootstrap-root-cid-shutdown-publish-timeout = শাটডাউনের সময় runtime_ipns প্রকাশের সময় শেষ হয়েছে
bootstrap-root-pin-replacement-failed = রিমোট রুট পিন প্রতিস্থাপন ত্রুটির পরে অব্যাহত রয়েছে
bootstrap-root-pin-update-failed = বুটস্ট্র্যাপের পরে পিন/আপডেট ব্যর্থ হয়েছে
bootstrap-runtime-manifest-published = রানটাইম রুট ম্যানিফেস্ট প্রকাশিত হয়েছে
crud-message-rejected = CRUD বার্তা প্রত্যাখ্যাত হয়েছে
entity-reload-current-node-load-failed = রিলোডের আগে বর্তমান এনটিটি নোড আনতে ব্যর্থ হয়েছে; বর্তমান প্লাগইন বজায় রাখা হয়েছে
entity-reload-failed = এনটিটি রিলোড ব্যর্থ হয়েছে; পরবর্তী রিলোড পর্যন্ত নিষ্ক্রিয়
entity-reload-kind-extends-failed = এনটিটি রিলোডের সময় Kind এক্সটেনশন চেইন সমাধান করতে ব্যর্থ হয়েছে
entity-reload-kind-fetch-failed = এনটিটি রিলোডের সময় Kind নোড আনতে ব্যর্থ হয়েছে
entity-reload-kind-lookup-failed = এনটিটি রিলোডের সময় Kind খোঁজার জন্য ম্যানিফেস্ট আনতে ব্যর্থ হয়েছে
entity-reload-kind-missing = ম্যানিফেস্টে Kind পাওয়া যায়নি; এনটিটি রিলোড করা যাবে না
entity-reload-manifest-state-update-failed = রিলোডের আগে বর্তমান অবস্থা দিয়ে ম্যানিফেস্ট আপডেট করতে ব্যর্থ হয়েছে; বর্তমান প্লাগইন বজায় রাখা হয়েছে
entity-reload-skipped = রিলোড গেট বন্ধ থাকায় এনটিটি রিলোড বাদ দেওয়া হয়েছে
entity-reload-started = এনটিটি রিলোড শুরু হয়েছে
entity-reload-state-persist-failed = রিলোডের আগে বর্তমান অবস্থা সংরক্ষণ করতে ব্যর্থ হয়েছে; বর্তমান প্লাগইন বজায় রাখা হয়েছে
entity-reload-state-produced-failed = রিলোডের সময় উৎপন্ন অবস্থা সংরক্ষণ করতে ব্যর্থ হয়েছে
entity-reloaded-manifest-update-failed = ম্যানিফেস্টে রিলোড করা এনটিটি আপডেট করতে ব্যর্থ হয়েছে
entity-reloaded-manifest-updated = রিলোড করা এনটিটি ম্যানিফেস্টে আপডেট হয়েছে
inbox-message-rejected = ইনবক্স বার্তা প্রত্যাখ্যাত হয়েছে
ma-create-entity-already-exists = ma_create_entity: এনটিটি ইতিমধ্যে বিদ্যমান; বর্তমান এনটিটি বজায় রাখা হয়েছে
ma-create-entity-invalid-behaviour = ma_create_entity: অবৈধ আচরণ রেফারেন্স; বাদ দেওয়া হয়েছে
ma-create-entity-kind-missing = ma_create_entity: রেজিস্ট্রিতে Kind নেই; বাদ দেওয়া হয়েছে
manifest-pin-update-failed = ম্যানিফেস্ট pin_update ব্যর্থ হয়েছে
plugin-envelope-build-failed = প্লাগইন এনভেলপ: বার্তা তৈরি করতে ব্যর্থ হয়েছে; বাদ দেওয়া হয়েছে
plugin-envelope-create-requests-ignored = প্লাগইন এনভেলপ: সাইড-ইফেক্ট প্রসঙ্গ ছাড়া তৈরির অনুরোধ উপেক্ষা করা হয়েছে
plugin-envelope-local-dispatch-failed = প্লাগইন এনভেলপ: স্থানীয় ডিসপ্যাচ ব্যর্থ হয়েছে
plugin-envelope-local-dispatch-finish = প্লাগইন এনভেলপ: স্থানীয় ডিসপ্যাচ সম্পন্ন হয়েছে
plugin-envelope-local-dispatch-start = প্লাগইন এনভেলপ: স্থানীয় ডিসপ্যাচ শুরু হয়েছে
plugin-envelope-local-gate-closed = প্লাগইন এনভেলপ: স্থানীয় ডিসপ্যাচ গেট বন্ধ
plugin-envelope-local-recipient-unknown = প্লাগইন এনভেলপ: অজানা স্থানীয় প্রাপক; বাদ দেওয়া হয়েছে
plugin-envelope-local-timeout = প্লাগইন এনভেলপ: স্থানীয় ডিসপ্যাচের সময় শেষ হয়েছে
plugin-envelope-recipient-invalid = প্লাগইন এনভেলপ: অবৈধ প্রাপক DID; বাদ দেওয়া হয়েছে
plugin-envelope-remote-limit = প্লাগইন এনভেলপ: রিমোট ডেলিভারি সীমা পৌঁছেছে; এনভেলপ বাদ দেওয়া হয়েছে
plugin-outbox-congested = প্লাগইন আউটবক্স ভরা; চ্যানেল পূর্ণ হলে এনভেলপ বাদ দেওয়া হতে পারে
plugin-outbox-drain-limit = প্লাগইন আউটবক্স ড্রেন বাজেট শেষ; বাকি এনভেলপগুলো স্থগিত
schedule-dispatch-firing = নির্ধারিত ডিসপ্যাচ চলছে
schedule-entity-not-found = নির্ধারিত ডিসপ্যাচ: এনটিটি পাওয়া যায়নি
schedule-random-chain-stopped = যাদৃচ্ছিক নির্ধারিত চেইন বন্ধ হয়েছে: নতুন সংজ্ঞা দিয়ে প্রতিস্থাপিত
schedule-random-create-failed = পরবর্তী যাদৃচ্ছিক কাজ তৈরি করতে ব্যর্থ হয়েছে
schedule-random-reschedule-failed = যাদৃচ্ছিক কাজ পুনর্নির্ধারণ করতে ব্যর্থ হয়েছে
schedule-stale-dispatch-skipped = নির্ধারিত ডিসপ্যাচ বাদ দেওয়া হয়েছে: পুরনো সময়সূচি
scheduled-dispatch-error = নির্ধারিত ডিসপ্যাচে ত্রুটি
scheduled-dispatch-manifest-writer-unavailable = নির্ধারিত ডিসপ্যাচ: ম্যানিফেস্ট রাইটার প্রস্তুত নয়; এনটিটি অবস্থা অপেক্ষায়
