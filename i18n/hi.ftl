# ma-runtime – हिन्दी
lang-name = हिन्दी

own-did-published = स्वयं का DID दस्तावेज़ IPNS पर प्रकाशित किया गया
own-did-publish-failed = स्वयं का DID दस्तावेज़ प्रकाशित करने में विफल
own-did-publish-timeout = स्वयं के DID दस्तावेज़ का प्रकाशन 2 मिनट बाद टाइम आउट हुआ
started = ma runtime शुरू हुआ
shutdown-requested = बंद करने का अनुरोध किया गया
closing-endpoint = iroh एंडपॉइंट बंद किया जा रहा है...
shutdown-complete = बंद करना पूर्ण हुआ
status-listening = स्टेटस सर्वर सुन रहा है
ipfs-message-rejected = IPFS संदेश अस्वीकार किया गया
ctrlc-handler-failed = Ctrl-C हैंडलर विफल रहा
node-connected = नोड प्रोटोकॉल से जुड़ा
received-encrypted-ma-msg = /ma/ipfs/0.0.1 पर एन्क्रिप्टेड ma संदेश प्राप्त हुआ
ping-received = :ping प्राप्त हुआ, :pong भेजा जा रहा है
did-publish-request-received = DID दस्तावेज़ प्रकाशन अनुरोध प्राप्त हुआ
document-published = दस्तावेज़ प्रकाशित किया गया
did-publish-cid-reply-sent = DID प्रकाशन के लिए CID उत्तर भेजा गया
did-publish-resolve-failed = ipfs-publish उत्तर देने के लिए प्रेषक को हल करने में विफल
ipfs-store-request-received = IPFS भंडारण अनुरोध प्राप्त हुआ
ipfs-stored = सामग्री IPFS पर संग्रहीत की गई
ipfs-store-cid-reply-sent = CID उत्तर भेजा गया
ipfs-store-resolve-failed = ipfs-store उत्तर देने के लिए प्रेषक को हल करने में विफल

# एंटिटी डिस्पैच
bootstrap-complete = Bootstrap पूर्ण हुआ
entity-loaded = एंटिटी प्लगइन लोड किया गया
entity-load-failed = एंटिटी प्लगइन लोड करने में विफल
root-list-entities = #root: एंटिटी सूची
entity-created = एंटिटी बनाई गई
entity-reloaded = एंटिटी प्लगइन पुनः लोड किया गया
entity-deleted = एंटिटी हटाई गई
entity-states-saving = एंटिटी स्थितियां IPFS पर सहेजी जा रही हैं
entity-state-saving = एंटिटी स्थिति सहेजी जा रही है
entity-state-saved = एंटिटी स्थिति सहेजी गई
entity-state-empty = प्लगइन ने खाली स्थिति लौटाई, सहेजना छोड़ा गया
entity-states-saved = एंटिटी स्थितियां सहेजी गईं

# पहला प्रारंभ / स्वतः-प्रारंभीकरण

# स्वामित्व
runtime-claimed = Runtime पंजीकृत हुआ।

# संरक्षित रूट तत्व
refuse-delete-root = आवश्यक रूट तत्व को हटाने से दृढ़तापूर्वक इनकार
runtime-claim-persisted = स्वामी कॉन्फ़िगरेशन में लिखा गया।


# Namespace creation (:create)
crud-message-received = CRUD संदेश प्राप्त
crud-acl-updated = रूट ट्रान्सपोर्ट ACL अपडेट हुआ

# CRUD validation errors
cidv1-required = मान एक सादा CIDv1 होनी चाहिए ('b' से शुरू; CIDv0 'Qm…' स्वीकार नहीं)
config-key-protected = config कुंजी '%key%' सुरक्षित है
config-key-no-delete = daemon config कुंजी '%key%' हटाई नहीं जा सकती
config-key-not-manifest = config कुंजी '%key%' एक ज्ञात manifest config कुंजी नहीं है
wrong-crud-protocol = गलत CRUD प्रोटोकॉल: %type%
entity-name-invalid = entity का नाम प्रिंट करने योग्य UTF-8 होना चाहिए
reserved-entity-name = entity का नाम '%name%' आरक्षित है
genesis-kind-owner-only = केवल runtime का स्वामी ही genesis प्रकार की entity बना सकता है

# IPv6 config
ipv6-enabled = IPv6 सक्षम — IPv4 और IPv6 दोनों से बाइंड हो रहा है
ipv6-disabled = IPv6 अक्षम है — केवल IPv4 बाइंड हो रहा है (पुनः सक्षम करने के लिए restart आवश्यक है)
ipv6-enable-restart-required = सहेजा गया। इस परिवर्तन को प्रभावी करने के लिए restart आवश्यक है।
ipv6-enable-unchanged = ipv6_enable पहले से उस मान पर सेट है — कोई परिवर्तन नहीं।

boot-default-root-config-populate-failed = डिफ़ॉल्ट रूट कॉन्फ़िगरेशन भरना विफल रहा
boot-default-root-config-populated = डिफ़ॉल्ट रूट कॉन्फ़िगरेशन भर दिया गया
boot-entity-load-processed = एंटिटी प्लगइन लोड हो गए
boot-group-load-failed = बूट के दौरान ग्रुप लोड करना विफल रहा
boot-group-loaded-into-cache = ग्रुप कैश में लोड हो गया
boot-kinds-overlay-applied = Kinds ओवरले लागू किया गया
boot-kinds-overlay-no-change = Kinds ओवरले ने मैनिफेस्ट में कोई बदलाव नहीं किया
boot-load-manifest-for-acl-cache-failed = ACL कैश भरने के लिए मैनिफेस्ट लोड करना विफल रहा
boot-minimal-manifest-bootstrapped = न्यूनतम मैनिफेस्ट प्रारंभ किया गया
boot-minimal-manifest-not-found = IPNS में रनटाइम रूट CID नहीं मिला; न्यूनतम मैनिफेस्ट प्रारंभ किया जा रहा है
boot-no-root-entity = डिफ़ॉल्ट रूट कॉन्फ़िगरेशन के लिए कोई रूट एंटिटी पंजीकृत नहीं है
boot-reconciled-owners-manifest-failed = बूट के दौरान मैनिफेस्ट में मालिकों का समाधान विफल रहा
boot-reconciled-owners-persist-failed = समाधानित मालिकों को config.yaml में सहेजना विफल रहा
boot-reconciled-owners-published = बूट के दौरान config.yaml/--owner से मैनिफेस्ट में मालिक समाधानित किए गए
boot-root-acl-load-cache-failed = बूट के दौरान रूट ACL लोड करना विफल रहा
boot-root-acl-load-failed = मैनिफेस्ट से रूट ACL लोड करना विफल रहा
boot-root-acl-loaded-from-manifest = रूट ट्रांसपोर्ट ACL मैनिफेस्ट से लोड हो गया
boot-root-acl-loaded-into-cache = रूट ACL कैश में लोड हो गया
bootstrap-acl-published = ACL नोड प्रकाशित हुआ
bootstrap-endpoint-close-stuck = 10 सेकंड बाद भी एंडपॉइंट चलते कार्यों द्वारा रोका गया है; ज़बरदस्ती बंद किया जा रहा है
bootstrap-endpoint-close-timeout = एंडपॉइंट बंद होने में 5 सेकंड में टाइमआउट हुआ; ज़बरदस्ती समाप्त किया जा रहा है
bootstrap-entity-lifecycle-update-failed = एंटिटी का अपडेट किया गया जीवनचक्र IPFS में लिखना विफल रहा
bootstrap-entity-lifecycle-updated = एंटिटी जीवनचक्र IPFS में अपडेट किया गया
bootstrap-entity-node-shutdown-updated = शटडाउन के दौरान एंटिटी नोड अपडेट किया गया
bootstrap-entity-published = एंटिटी नोड प्रकाशित हुआ
bootstrap-entity-registering-prepublished = पूर्व-प्रकाशित एंटिटी पंजीकृत की जा रही है
bootstrap-entity-registry-fetch-failed = एंटिटी नोड प्राप्त करना विफल रहा
bootstrap-entity-registry-kind-extends-failed = Kind एक्सटेंशन चेन हल करना विफल रहा
bootstrap-entity-registry-kind-fetch-failed = Kind नोड प्राप्त करना विफल रहा
bootstrap-entity-registry-kind-missing = मैनिफेस्ट में Kind नहीं मिला; एंटिटी छोड़ी गई
bootstrap-entity-registry-not-in-manifest = एंटिटी रजिस्ट्री में है लेकिन मैनिफेस्ट में नहीं; छोड़ी गई
bootstrap-entity-state-save-failed = एंटिटी अवस्थाएं सहेजना विफल रहा
bootstrap-entity-state-shutdown-aborted = शटडाउन रद्द किया गया; रनटाइम अगले शटडाउन पर अवस्था सहेजने के लिए सक्रिय रहता है
bootstrap-entity-state-update-fetch-failed = अवस्था अपडेट के लिए एंटिटी नोड प्राप्त करना विफल रहा
bootstrap-envelope-delivery-failed = प्लगइन एनवेलप डिलीवरी विफल रही; एनवेलप हटाया गया
bootstrap-envelope-open-failed = प्लगइन एनवेलप: आउटगोइंग मेलबॉक्स खोलना विफल रहा; एनवेलप हटाया गया
bootstrap-group-published = ग्रुप नोड प्रकाशित हुआ
bootstrap-kind-published = Kind नोड प्रकाशित हुआ
bootstrap-kind-registry-extends-failed = रजिस्ट्री के लिए Kind एक्सटेंशन चेन हल करना विफल रहा
bootstrap-kind-registry-fetch-log-failed = रजिस्ट्री के लिए Kind नोड प्राप्त करना विफल रहा
bootstrap-kind-registry-hydrated = Kinds रजिस्ट्री मैनिफेस्ट से भरी गई
bootstrap-kinds-overlay-pin-update-failed = Kinds ओवरले के बाद पिन/अपडेट विफल रहा
bootstrap-kinds-overlay-published = Kinds ओवरले के बाद रनटाइम मैनिफेस्ट प्रकाशित हुआ
bootstrap-kinds-tree-published = रनटाइम Kinds ट्री प्रकाशित हुई
bootstrap-lifecycle-manifest-pin-update-failed = जीवनचक्र दृढ़ता के बाद पिन/अपडेट विफल रहा
bootstrap-lifecycle-manifest-publish-failed = जीवनचक्र संक्रमण के बाद मैनिफेस्ट प्रकाशित करना विफल रहा
bootstrap-lifecycle-manifest-published = जीवनचक्र संक्रमण के बाद अपडेट मैनिफेस्ट प्रकाशित हुआ
bootstrap-manifest-fetch-failed = रनटाइम मैनिफेस्ट प्राप्त करना विफल रहा
bootstrap-minimal-manifest-failed = न्यूनतम मैनिफेस्ट प्रारंभ करना विफल रहा
bootstrap-remote-root-pin-confirmed = रिमोट रूट पिनिंग की पुष्टि हुई
bootstrap-remote-root-pin-misconfigured = रिमोट रूट पिनिंग गलत तरीके से कॉन्फ़िगर की गई है
bootstrap-root-acl-published = रूट ट्रांसपोर्ट ACL प्रकाशित हुआ
bootstrap-root-cid-shutdown-persist-failed = शटडाउन के दौरान root_cid सहेजना विफल रहा
bootstrap-root-cid-shutdown-publish-failed = शटडाउन के दौरान runtime_ipns प्रकाशित करना विफल रहा
bootstrap-root-cid-shutdown-publish-succeeded = शटडाउन के दौरान runtime_ipns प्रकाशन सफल रहा
bootstrap-root-cid-shutdown-publish-timeout = शटडाउन के दौरान runtime_ipns प्रकाशन का समय समाप्त हुआ
bootstrap-root-pin-replacement-failed = रिमोट रूट पिन प्रतिस्थापन त्रुटि के बाद जारी है
bootstrap-root-pin-update-failed = बूटस्ट्रैप के बाद पिन/अपडेट विफल रहा
bootstrap-runtime-manifest-published = रनटाइम रूट मैनिफेस्ट प्रकाशित हुआ
crud-message-rejected = CRUD संदेश अस्वीकृत
entity-reload-current-node-load-failed = रीलोड से पहले वर्तमान एंटिटी नोड प्राप्त करना विफल रहा; वर्तमान प्लगइन बनाए रखा गया
entity-reload-failed = एंटिटी रीलोड विफल रहा; अगले रीलोड तक अक्षम
entity-reload-kind-extends-failed = एंटिटी रीलोड के दौरान Kind एक्सटेंशन चेन हल करना विफल रहा
entity-reload-kind-fetch-failed = एंटिटी रीलोड के दौरान Kind नोड प्राप्त करना विफल रहा
entity-reload-kind-lookup-failed = एंटिटी रीलोड के दौरान Kind खोज के लिए मैनिफेस्ट प्राप्त करना विफल रहा
entity-reload-kind-missing = मैनिफेस्ट में Kind नहीं मिला; एंटिटी रीलोड नहीं हो सकती
entity-reload-manifest-state-update-failed = रीलोड से पहले वर्तमान अवस्था के साथ मैनिफेस्ट अपडेट करना विफल रहा; वर्तमान प्लगइन बनाए रखा गया
entity-reload-skipped = एंटिटी रीलोड छोड़ा गया क्योंकि रीलोड गेट बंद है
entity-reload-started = एंटिटी रीलोड शुरू हुआ
entity-reload-state-persist-failed = रीलोड से पहले वर्तमान अवस्था सहेजना विफल रहा; वर्तमान प्लगइन बनाए रखा गया
entity-reload-state-produced-failed = रीलोड के दौरान उत्पन्न अवस्था सहेजना विफल रहा
entity-reloaded-manifest-update-failed = मैनिफेस्ट में रीलोड की गई एंटिटी अपडेट करना विफल रहा
entity-reloaded-manifest-updated = रीलोड की गई एंटिटी मैनिफेस्ट में अपडेट हुई
inbox-message-rejected = इनबॉक्स संदेश अस्वीकृत
ma-create-entity-already-exists = ma_create_entity: एंटिटी पहले से मौजूद है; वर्तमान एंटिटी बनाए रखी गई
ma-create-entity-invalid-behaviour = ma_create_entity: अमान्य व्यवहार संदर्भ; छोड़ा गया
ma-create-entity-kind-missing = ma_create_entity: रजिस्ट्री में Kind नहीं है; छोड़ा गया
manifest-pin-update-failed = मैनिफेस्ट pin_update विफल रहा
plugin-envelope-build-failed = प्लगइन एनवेलप: संदेश बनाना विफल रहा; छोड़ा गया
plugin-envelope-create-requests-ignored = प्लगइन एनवेलप: साइड-इफ़ेक्ट संदर्भ के बिना बनाने के अनुरोध अनदेखे किए गए
plugin-envelope-local-dispatch-failed = प्लगइन एनवेलप: स्थानीय डिस्पैच विफल रहा
plugin-envelope-local-dispatch-finish = प्लगइन एनवेलप: स्थानीय डिस्पैच पूर्ण हुआ
plugin-envelope-local-dispatch-start = प्लगइन एनवेलप: स्थानीय डिस्पैच शुरू हुआ
plugin-envelope-local-gate-closed = प्लगइन एनवेलप: स्थानीय डिस्पैच गेट बंद है
plugin-envelope-local-recipient-unknown = प्लगइन एनवेलप: अज्ञात स्थानीय प्राप्तकर्ता; छोड़ा गया
plugin-envelope-local-timeout = प्लगइन एनवेलप: स्थानीय डिस्पैच का समय समाप्त हुआ
plugin-envelope-recipient-invalid = प्लगइन एनवेलप: अमान्य प्राप्तकर्ता DID; छोड़ा गया
plugin-envelope-remote-limit = प्लगइन एनवेलप: रिमोट डिलीवरी सीमा पहुंची; एनवेलप हटाया गया
plugin-outbox-congested = प्लगइन आउटबॉक्स भरा है; चैनल भरने पर एनवेलप हटाए जा सकते हैं
plugin-outbox-drain-limit = प्लगइन आउटबॉक्स ड्रेन बजट समाप्त; शेष एनवेलप स्थगित किए गए
schedule-dispatch-firing = निर्धारित डिस्पैच चल रहा है
schedule-entity-not-found = निर्धारित डिस्पैच: एंटिटी नहीं मिली
schedule-random-chain-stopped = यादृच्छिक निर्धारित चेन रोकी गई: नई परिभाषा से बदली गई
schedule-random-create-failed = अगला यादृच्छिक कार्य बनाना विफल रहा
schedule-random-reschedule-failed = यादृच्छिक कार्य पुनर्निर्धारित करना विफल रहा
schedule-stale-dispatch-skipped = निर्धारित डिस्पैच छोड़ा गया: पुराना शेड्यूल
scheduled-dispatch-error = निर्धारित डिस्पैच में त्रुटि
scheduled-dispatch-manifest-writer-unavailable = निर्धारित डिस्पैच: मैनिफेस्ट राइटर तैयार नहीं; एंटिटी अवस्था प्रतीक्षारत है
