# ma-runtime – मराठी
lang-name = मराठी

own-did-published = स्वतःचा DID दस्तऐवज IPNS वर प्रकाशित झाला
own-did-publish-failed = स्वतःचा DID दस्तऐवज प्रकाशित करण्यात अयशस्वी
own-did-publish-timeout = स्वतःच्या DID दस्तऐवज प्रकाशनाचा वेळ 2 मिनिटांनंतर संपला
started = ma runtime सुरू झाले
shutdown-requested = बंद करण्याची विनंती केली
closing-endpoint = iroh एंडपॉइंट बंद होत आहे...
shutdown-complete = बंद करणे पूर्ण झाले
status-listening = स्टेटस सर्व्हर ऐकत आहे
ipfs-message-rejected = IPFS संदेश नाकारला गेला
ctrlc-handler-failed = Ctrl-C हँडलर अयशस्वी झाला
node-connected = नोड प्रोटोकॉलशी जोडले
received-encrypted-ma-msg = /ma/ipfs/0.0.1 वर एन्क्रिप्टेड ma संदेश प्राप्त झाला
ping-received = :ping प्राप्त, :pong पाठवत आहे
did-publish-request-received = DID दस्तऐवज प्रकाशन विनंती प्राप्त झाली
document-published = दस्तऐवज प्रकाशित झाला
did-publish-cid-reply-sent = DID प्रकाशनासाठी CID उत्तर पाठवले
did-publish-resolve-failed = ipfs-publish उत्तर देण्यासाठी प्रेषक सोडवण्यात अयशस्वी
ipfs-store-request-received = IPFS स्टोरेज विनंती प्राप्त झाली
ipfs-stored = सामग्री IPFS वर संग्रहित केली
ipfs-store-cid-reply-sent = CID उत्तर पाठवले
ipfs-store-resolve-failed = ipfs-store उत्तर देण्यासाठी प्रेषक सोडवण्यात अयशस्वी

# एंटिटी डिस्पॅच
bootstrap-complete = Bootstrap पूर्ण झाले
entity-loaded = एंटिटी प्लगइन लोड झाला
entity-load-failed = एंटिटी प्लगइन लोड करण्यात अयशस्वी
root-list-entities = #root: एंटिटी यादी
entity-created = एंटिटी तयार झाली
entity-reloaded = एंटिटी प्लगइन पुन्हा लोड झाला
entity-deleted = एंटिटी हटवली
entity-states-saving = IPFS वर एंटिटी स्थिती जतन होत आहेत
entity-state-saving = एंटिटी स्थिती जतन होत आहे
entity-state-saved = एंटिटी स्थिती जतन झाली
entity-state-empty = प्लगइनने रिकामी स्थिती परत केली, जतन वगळले
entity-states-saved = एंटिटी स्थिती जतन झाल्या

# पहली सुरुवात / स्वयं-प्रारंभ

# मालकी
runtime-claimed = Runtime नोंदवले.

# संरक्षित मूळ घटक
refuse-delete-root = आवश्यक मूळ घटक हटवण्यास ठामपणे नकार
runtime-claim-persisted = मालक कॉन्फिगरेशनमध्ये लिहिला.


# Namespace creation (:create)
crud-message-received = CRUD संदेश प्राप्त झाला
crud-acl-updated = रूट ट्रान्सपोर्ट ACL अपडेट केले

# CRUD validation errors
cidv1-required = मूल्य एक साधे CIDv1 असणे आवश्यक आहे ('b' पासून सुरू; CIDv0 'Qm…' स्वीकार्य नाही)
config-key-protected = config की '%key%' संरक्षित आहे
config-key-no-delete = daemon config की '%key%' हटविता येत नाही
config-key-not-manifest = config की '%key%' हे ज्ञात manifest config की नाही
wrong-crud-protocol = चुकीचा CRUD प्रोटोकॉल: %type%
entity-name-invalid = entity चे नाव मुद्रण करण्यायोग्य UTF-8 असणे आवश्यक आहे
reserved-entity-name = entity चे नाव '%name%' राखीव आहे
genesis-kind-owner-only = फक्त runtime चा मालकच genesis प्रकारची entity तयार करू शकतो

# IPv6 config
ipv6-enabled = IPv6 सक्षम — IPv4 आणि IPv6 दोन्हींशी बांधणी होत आहे
ipv6-disabled = IPv6 अक्षम झाले — फक्त IPv4 बाइंड होत आहे (पुन्हा सक्षम करण्यासाठी restart आवश्यक आहे)
ipv6-enable-restart-required = जतन केले. हा बदल लागू होण्यासाठी restart आवश्यक आहे.
ipv6-enable-unchanged = ipv6_enable आधीच त्या मूल्यावर सेट आहे — कोणताही बदल नाही.

boot-default-root-config-populate-failed = डिफॉल्ट रूट कॉन्फिगरेशन भरण्यात अयशस्वी
boot-default-root-config-populated = डिफॉल्ट रूट कॉन्फिगरेशन भरले गेले
boot-entity-load-processed = एंटिटी प्लगिन लोड झाले
boot-group-load-failed = बूट दरम्यान ग्रूप लोड करण्यात अयशस्वी
boot-group-loaded-into-cache = ग्रूप कॅशमध्ये लोड झाले
boot-kinds-overlay-applied = Kinds ओव्हरले लागू केले
boot-kinds-overlay-no-change = Kinds ओव्हरलेने मॅनिफेस्टमध्ये बदल केला नाही
boot-load-manifest-for-acl-cache-failed = ACL कॅश भरण्यासाठी मॅनिफेस्ट लोड करण्यात अयशस्वी
boot-minimal-manifest-bootstrapped = किमान मॅनिफेस्ट बूटस्ट्रॅप झाले
boot-minimal-manifest-not-found = IPNS मध्ये रनटाइम रूट CID आढळला नाही; किमान मॅनिफेस्ट बूटस्ट्रॅप करत आहे
boot-no-root-entity = डिफॉल्ट रूट कॉन्फिगरेशनसाठी कोणतीही रूट एंटिटी नोंदणीकृत नाही
boot-reconciled-owners-manifest-failed = बूट दरम्यान मॅनिफेस्टमध्ये मालकांशी जुळवण्यात अयशस्वी
boot-reconciled-owners-persist-failed = जुळवलेले मालक config.yaml मध्ये जतन करण्यात अयशस्वी
boot-reconciled-owners-published = बूट दरम्यान config.yaml/--owner मधून मॅनिफेस्टमध्ये मालक जुळवले गेले
boot-root-acl-load-cache-failed = बूट दरम्यान रूट ACL लोड करण्यात अयशस्वी
boot-root-acl-load-failed = मॅनिफेस्टमधून रूट ACL लोड करण्यात अयशस्वी
boot-root-acl-loaded-from-manifest = रूट ट्रान्स्पोर्ट ACL मॅनिफेस्टमधून लोड झाले
boot-root-acl-loaded-into-cache = रूट ACL कॅशमध्ये लोड झाले
bootstrap-acl-published = ACL नोड प्रकाशित झाले
bootstrap-endpoint-close-stuck = 10 से नंतरही एंडपॉइंट चालू असलेल्या कार्यांनी अडकले आहे; जबरदस्तीने बंद करत आहे
bootstrap-endpoint-close-timeout = एंडपॉइंट बंद करणे 5 से नंतर टाइमआउट झाले; जबरदस्तीने समाप्त करत आहे
bootstrap-entity-lifecycle-update-failed = एंटिटीचे अपडेट केलेले जीवन चक्र IPFS मध्ये लिहिण्यात अयशस्वी
bootstrap-entity-lifecycle-updated = एंटिटी जीवन चक्र IPFS मध्ये अपडेट झाले
bootstrap-entity-node-shutdown-updated = शटडाउन दरम्यान एंटिटी नोड अपडेट झाले
bootstrap-entity-published = एंटिटी नोड प्रकाशित झाले
bootstrap-entity-registering-prepublished = आधी प्रकाशित एंटिटी नोंदवत आहे
bootstrap-entity-registry-fetch-failed = एंटिटी नोड आणण्यात अयशस्वी
bootstrap-entity-registry-kind-extends-failed = Kind विस्तार साखळी रिझोल्व्ह करण्यात अयशस्वी
bootstrap-entity-registry-kind-fetch-failed = Kind नोड आणण्यात अयशस्वी
bootstrap-entity-registry-kind-missing = मॅनिफेस्टमध्ये Kind आढळला नाही; एंटिटी वगळली
bootstrap-entity-registry-not-in-manifest = एंटिटी रेजिस्ट्रीमध्ये आहे पण मॅनिफेस्टमध्ये नाही; वगळली
bootstrap-entity-state-save-failed = एंटिटी स्थिती जतन करण्यात अयशस्वी
bootstrap-entity-state-shutdown-aborted = शटडाउन रद्द झाले; पुढील शटडाउनमध्ये स्थिती जतन करण्यासाठी रनटाइम सक्रिय आहे
bootstrap-entity-state-update-fetch-failed = स्थिती अपडेटसाठी एंटिटी नोड आणण्यात अयशस्वी
bootstrap-envelope-delivery-failed = प्लगिन एनव्हलप वितरण अयशस्वी; एनव्हलप हटवले
bootstrap-envelope-open-failed = प्लगिन एनव्हलप: आउटगोइंग मेलबॉक्स उघडण्यात अयशस्वी; एनव्हलप हटवले
bootstrap-group-published = ग्रूप नोड प्रकाशित झाले
bootstrap-kind-published = Kind नोड प्रकाशित झाले
bootstrap-kind-registry-extends-failed = रेजिस्ट्रीसाठी Kind विस्तार साखळी रिझोल्व्ह करण्यात अयशस्वी
bootstrap-kind-registry-fetch-log-failed = रेजिस्ट्रीसाठी Kind नोड आणण्यात अयशस्वी
bootstrap-kind-registry-hydrated = Kinds रेजिस्ट्री मॅनिफेस्टमधून भरली गेली
bootstrap-kinds-overlay-pin-update-failed = Kinds ओव्हरलेनंतर पिन/अपडेट अयशस्वी
bootstrap-kinds-overlay-published = Kinds ओव्हरलेनंतर रनटाइम मॅनिफेस्ट प्रकाशित झाले
bootstrap-kinds-tree-published = रनटाइम Kinds वृक्ष प्रकाशित झाला
bootstrap-lifecycle-manifest-pin-update-failed = जीवन चक्र स्थिरतेनंतर पिन/अपडेट अयशस्वी
bootstrap-lifecycle-manifest-publish-failed = जीवन चक्र संक्रमणानंतर मॅनिफेस्ट प्रकाशित करण्यात अयशस्वी
bootstrap-lifecycle-manifest-published = जीवन चक्र संक्रमणानंतर अपडेट केलेले मॅनिफेस्ट प्रकाशित झाले
bootstrap-manifest-fetch-failed = रनटाइम मॅनिफेस्ट आणण्यात अयशस्वी
bootstrap-minimal-manifest-failed = किमान मॅनिफेस्ट बूटस्ट्रॅप करण्यात अयशस्वी
bootstrap-remote-root-pin-confirmed = रिमोट रूट पिनिंगची पुष्टी झाली
bootstrap-remote-root-pin-misconfigured = रिमोट रूट पिनिंग चुकीचे कॉन्फिगर केले आहे
bootstrap-root-acl-published = रूट ट्रान्स्पोर्ट ACL प्रकाशित झाले
bootstrap-root-cid-shutdown-persist-failed = शटडाउन दरम्यान root_cid जतन करण्यात अयशस्वी
bootstrap-root-cid-shutdown-publish-failed = शटडाउन दरम्यान runtime_ipns प्रकाशित करण्यात अयशस्वी
bootstrap-root-cid-shutdown-publish-succeeded = शटडाउन दरम्यान runtime_ipns प्रकाशन यशस्वी
bootstrap-root-cid-shutdown-publish-timeout = शटडाउन दरम्यान runtime_ipns प्रकाशन टाइमआउट झाले
bootstrap-root-pin-replacement-failed = रिमोट रूट पिन बदलण्यात त्रुटीनंतर सुरू आहे
bootstrap-root-pin-update-failed = बूटस्ट्रॅपनंतर पिन/अपडेट अयशस्वी
bootstrap-runtime-manifest-published = रनटाइम रूट मॅनिफेस्ट प्रकाशित झाले
crud-message-rejected = CRUD संदेश नाकारला
entity-reload-current-node-load-failed = रीलोडपूर्वी वर्तमान एंटिटी नोड आणण्यात अयशस्वी; वर्तमान प्लगिन कायम ठेवले
entity-reload-failed = एंटिटी रीलोड अयशस्वी; पुढील रीलोडपर्यंत अक्षम
entity-reload-kind-extends-failed = एंटिटी रीलोड दरम्यान Kind विस्तार साखळी रिझोल्व्ह करण्यात अयशस्वी
entity-reload-kind-fetch-failed = एंटिटी रीलोड दरम्यान Kind नोड आणण्यात अयशस्वी
entity-reload-kind-lookup-failed = एंटिटी रीलोड दरम्यान Kind शोधासाठी मॅनिफेस्ट आणण्यात अयशस्वी
entity-reload-kind-missing = मॅनिफेस्टमध्ये Kind आढळला नाही; एंटिटी रीलोड करता येत नाही
entity-reload-manifest-state-update-failed = रीलोडपूर्वी वर्तमान स्थितीसह मॅनिफेस्ट अपडेट करण्यात अयशस्वी; वर्तमान प्लगिन कायम ठेवले
entity-reload-skipped = रीलोड गेट बंद असल्याने एंटिटी रीलोड वगळले
entity-reload-started = एंटिटी रीलोड सुरू झाले
entity-reload-state-persist-failed = रीलोडपूर्वी वर्तमान स्थिती जतन करण्यात अयशस्वी; वर्तमान प्लगिन कायम ठेवले
entity-reload-state-produced-failed = रीलोड दरम्यान तयार केलेली स्थिती जतन करण्यात अयशस्वी
entity-reloaded-manifest-update-failed = मॅनिफेस्टमध्ये रीलोड केलेली एंटिटी अपडेट करण्यात अयशस्वी
entity-reloaded-manifest-updated = रीलोड केलेली एंटिटी मॅनिफेस्टमध्ये अपडेट झाली
inbox-message-rejected = इनबॉक्स संदेश नाकारला
ma-create-entity-already-exists = ma_create_entity: एंटिटी आधीच अस्तित्वात आहे; वर्तमान एंटिटी कायम ठेवली
ma-create-entity-invalid-behaviour = ma_create_entity: अवैध वर्तन संदर्भ; वगळले
ma-create-entity-kind-missing = ma_create_entity: रेजिस्ट्रीमध्ये Kind नाही; वगळले
manifest-pin-update-failed = मॅनिफेस्ट pin_update अयशस्वी
plugin-envelope-build-failed = प्लगिन एनव्हलप: संदेश तयार करण्यात अयशस्वी; वगळले
plugin-envelope-create-requests-ignored = प्लगिन एनव्हलप: साइड-इफेक्ट संदर्भाशिवाय तयार करण्याच्या विनंत्या दुर्लक्षित केल्या
plugin-envelope-local-dispatch-failed = प्लगिन एनव्हलप: स्थानिक डिस्पॅच अयशस्वी
plugin-envelope-local-dispatch-finish = प्लगिन एनव्हलप: स्थानिक डिस्पॅच पूर्ण झाले
plugin-envelope-local-dispatch-start = प्लगिन एनव्हलप: स्थानिक डिस्पॅच सुरू झाले
plugin-envelope-local-gate-closed = प्लगिन एनव्हलप: स्थानिक डिस्पॅच गेट बंद आहे
plugin-envelope-local-recipient-unknown = प्लगिन एनव्हलप: अज्ञात स्थानिक प्राप्तकर्ता; वगळले
plugin-envelope-local-timeout = प्लगिन एनव्हलप: स्थानिक डिस्पॅच टाइमआउट झाले
plugin-envelope-recipient-invalid = प्लगिन एनव्हलप: अवैध प्राप्तकर्ता DID; वगळले
plugin-envelope-remote-limit = प्लगिन एनव्हलप: रिमोट वितरण मर्यादा गाठली; एनव्हलप सोडले
plugin-outbox-congested = प्लगिन आउटबॉक्स भरलेले आहे; चॅनेल भरल्यास एनव्हलप सोडले जाऊ शकतात
plugin-outbox-drain-limit = प्लगिन आउटबॉक्स ड्रेन बजेट संपले; उर्वरित एनव्हलप पुढे ढकलले
schedule-dispatch-firing = शेड्यूल केलेले डिस्पॅच होत आहे
schedule-entity-not-found = शेड्यूल केलेले डिस्पॅच: एंटिटी आढळली नाही
schedule-random-chain-stopped = यादृच्छिक शेड्यूल साखळी थांबली: नवीन व्याख्येने बदलली
schedule-random-create-failed = पुढील यादृच्छिक कार्य तयार करण्यात अयशस्वी
schedule-random-reschedule-failed = यादृच्छिक कार्य पुनर्शेड्यूल करण्यात अयशस्वी
schedule-stale-dispatch-skipped = शेड्यूल केलेले डिस्पॅच वगळले: जुने शेड्यूल
scheduled-dispatch-error = शेड्यूल केलेल्या डिस्पॅचमध्ये त्रुटी
scheduled-dispatch-manifest-writer-unavailable = शेड्यूल केलेले डिस्पॅच: मॅनिफेस्ट लेखक तयार नाही; एंटिटी स्थिती प्रतीक्षेत
