# ma-runtime – Soomaali
lang-name = Soomaali

own-did-published = Dokumeentiyahayga DID ayaa IPNS lagu daabacay
own-did-publish-failed = Daabacaadda dokumeentiga DID ee iyada ah way guul-darroowday
own-did-publish-timeout = Daabacaadda dokumeentiga DID ee iyada ah waxay dhaaftay 2 daqiiqo
started = ma runtime ayaa bilaabmay
shutdown-requested = Codsiga joojinta ayaa la helay
closing-endpoint = iroh endpoint waxaa la xidayaa...
shutdown-complete = Joojinta waa la dhamaystay
status-listening = Serverka xaaladda ayaa dhagaysanaya
ipfs-message-rejected = Farriinta IPFS ayaa la diidday
ctrlc-handler-failed = Maamulaha Ctrl-C ayaa guuldareystay
node-connected = Node-ka ayaa xidhasho la galay protokoolka
received-encrypted-ma-msg = Farriinta ma ee encrypted ayaa la helay /ma/ipfs/0.0.1
ping-received = :ping ayaa la helay, :pong waa la dirayaa
did-publish-request-received = Codsiga daabacaadda dokumeentiga DID ayaa la helay
document-published = Dokumeentigu wuu daabacmay
did-publish-cid-reply-sent = Jawaabta CID ee daabacaadda DID ayaa la diray
did-publish-resolve-failed = Waa laga guuldareystay in la xalliyo dirayaha si loo gaarsiiyo jawaabta ipfs-publish
ipfs-store-request-received = Codsiga kaydinta IPFS ayaa la helay
ipfs-stored = Nuxurku IPFS ayuu ku kaydinmay
ipfs-store-cid-reply-sent = Jawaabta CID ayaa la diray
ipfs-store-resolve-failed = Waa laga guuldareystay in la xalliyo dirayaha si loo gaarsiiyo jawaabta ipfs-store

# Qeybinta shayga
bootstrap-complete = Bootstrap waa la dhamaystay
entity-loaded = Plugin-ka shayga ayaa la raray
entity-load-failed = Raarista plugin-ka shayga way guul-darroowday
root-list-entities = #root: liiska shayada
entity-created = Shayga ayaa la sameeyay
entity-reloaded = Plugin-ka shayga ayaa dib loo raray
entity-deleted = Shayga ayaa la tirtiray
entity-states-saving = Xaaladaha shayada IPFS ayaa lagu kaydiyaa
entity-state-saving = Xaaladda shayga ayaa la kaydiyaa
entity-state-saved = Xaaladda shayga ayaa la kaydiyay
entity-state-empty = Plugin-ku wuxuu soo celiyay xaalad madhan, kaydinta waa la booday
entity-states-saved = Xaaladaha shayada ayaa la kaydiyay

# Bilaabista ugu horreysa / bilaabista otomaatig ah

# Lahaanshaha
runtime-claimed = Runtime ayaa diiwaangeliyay.

# Curiyeyaasha xidiga ee la ilaaliyay
refuse-delete-root = Waxaan si xooggan u diiday in la tirtiro curiyaha xidiga ee loo baahan yahay
runtime-claim-persisted = Milkiilaha ayaa habaynta lagu qoray.


# Namespace creation (:create)
crud-message-received = Fariin CRUD la helay
crud-acl-updated = ACL gaadhsiinta xidiga waa la cusboonaysiiyay

# CRUD validation errors
cidv1-required = qiimaha waa inuu yahay CIDv1 qalin ah (wuxuu ka bilaabmaa 'b'; CIDv0 'Qm…' lama aqbalo)
config-key-protected = furaha config '%key%' waa la ilaaliyo
config-key-no-delete = furaha config '%key%' ee daemon lama tirtiri karo
config-key-not-manifest = furaha config '%key%' maaha furah manifest config la garanayo
wrong-crud-protocol = protokoolka CRUD ee khaldan: %type%
entity-name-invalid = magaca entity waa inuu noqdaa UTF-8 la daabici karo
reserved-entity-name = magaca entity '%name%' waa la kaydiyay
genesis-kind-owner-only = Kaliya milkiilaha runtime ayaa abuuri kara entity nooca genesis ah

# IPv6 config
ipv6-enabled = IPv6 waxa la shiday — waxay xidhaa IPv4 iyo IPv6 labadaba
ipv6-disabled = IPv6 waa la dami — IPv4 kaliya ayaa la xidaya (dib u bilaabid ayaa loo baahan yahay si dib loogu shido)
ipv6-enable-restart-required = Waxa la keydsaday. Dib u bilaabid ayaa loo baahan yahay si isbeddelkani saamayn u yeesho.
ipv6-enable-unchanged = ipv6_enable horay ayuu u dejiyay qiimahan — wax is beddel ah ma jiro.

entity-reload-skipped = Dib-u-rarista entity waa la gudbay maxaa yeelay albaabka dib-u-rarista waa xiddan yahay
entity-reload-started = Dib-u-rarista entity waa la bilaabay
entity-reload-kind-lookup-failed = Kuma guulaysan rarida manifest si loo raadiyo kind inta lagu jiro dib-u-rarista entity
entity-reload-kind-missing = Kind lama helin manifest; ma suurtagalin dib-u-rarista entity
entity-reload-kind-fetch-failed = Kuma guulaysan helitaanka kind node inta lagu jiro dib-u-rarista entity
entity-reload-kind-extends-failed = Kuma guulaysan xallinta silsilada extends ee kind inta lagu jiro dib-u-rarista entity
entity-reload-manifest-state-update-failed = Kuma guulaysan cusboonaysiinta manifest xaaladda hadda jirta ka hor dib-u-rarista; plugin hadda jirta waa la hayaa
entity-reload-state-persist-failed = Kuma guulaysan kaydinta xaaladda hadda jirta ka hor dib-u-rarista; plugin hadda jirta waa la hayaa
entity-reload-current-node-load-failed = Kuma guulaysan rarida entity node hadda jirta ka hor dib-u-rarista; plugin hadda jirta waa la hayaa
entity-reload-failed = Entity kuma guulaysan dib-u-rarista; waxaa la saarayaa ilaa dib-u-rarista xigta
entity-reload-state-produced-failed = Kuma guulaysan kaydinta xaaladda la soo saaray inta lagu jiro dib-u-rarista
entity-reloaded-manifest-updated = Entity dib-u-rar waa la cusboonaysiiyay ee manifest
entity-reloaded-manifest-update-failed = Kuma guulaysan cusboonaysiinta entity dib-u-rar ee manifest
bootstrap-remote-root-pin-misconfigured = Remote root pinning si xun ayaa loo dejiyay
bootstrap-remote-root-pin-confirmed = Remote root pin waa la xaqiijiyay
bootstrap-kinds-tree-published = Runtime kinds geed waa la daabacay
bootstrap-kinds-overlay-pin-update-failed = Pin/update way ku guul-darraysatay ka dib kinds overlay
bootstrap-kinds-overlay-published = Runtime manifest waa la daabacay ka dib kinds overlay
bootstrap-runtime-manifest-published = Runtime root manifest waa la daabacay
bootstrap-root-pin-replacement-failed = Waxaa lagu sii wadaa ka dib guul-darrada beddelka pin-ka root ee fog
bootstrap-root-pin-update-failed = Pin/update way ku guul-darraysatay ka dib bootstrap
bootstrap-kind-published = Kind node waa la daabacay
bootstrap-entity-registering-prepublished = Waxaa la diiwaangelinayaa entity horay loo daabacay
bootstrap-entity-published = Entity node waa la daabacay
bootstrap-acl-published = ACL node waa la daabacay
bootstrap-group-published = Koox node waa la daabacay
bootstrap-root-acl-published = Root transport-gate ACL waa la daabacay
bootstrap-kind-registry-hydrated = Kind registry waxaa laga buuxiyay manifest
bootstrap-lifecycle-manifest-pin-update-failed = Pin/update way ku guul-darraysatay ka dib kaydinta lifecycle
bootstrap-lifecycle-manifest-published = Manifest la cusboonaysiiyay waa la daabacay ka dib u-wareejimaha lifecycle
bootstrap-lifecycle-manifest-publish-failed = Kuma guulaysan daabacaadda manifest ka dib u-wareejimaha lifecycle
bootstrap-entity-lifecycle-updated = Entity lifecycle waa la cusboonaysiiyay ee IPFS
bootstrap-entity-lifecycle-update-failed = Kuma guulaysan qorida cusboonaysiinta lifecycle entity ee IPFS
bootstrap-entity-node-shutdown-updated = Entity node waa la cusboonaysiiyay markii la xidhay
bootstrap-entity-registry-not-in-manifest = Entity waxay ku jirtaa registry laakiin kuma jirto manifest; waa la gudbay
plugin-outbox-drain-limit = Miisaaniyadda drain ee plugin outbox waa la dhammaystay; envelopes hartay waa la dhaafsiyay
plugin-outbox-congested = Plugin outbox waa buuxsamay; envelopes waxa laga yaabaa in la siiyo haddii kanaalku buuxo
plugin-envelope-local-gate-closed = Plugin envelope: albaabka dirsanaynta maxalliga waa xiddan yahay
plugin-envelope-local-timeout = Plugin envelope: dirsanaynta maxalliga waxay dhammaysay waqtiga
plugin-envelope-recipient-invalid = Plugin envelope: DID qaabilaha ma sax; waa la gudbay
plugin-envelope-build-failed = Plugin envelope: kuma guulaysan dhisidda fariinta; waa la gudbay
plugin-envelope-remote-limit = Plugin envelope: xadka dhiibidda fog waa la gaary; envelope waa la siday
scheduled-dispatch-error = Khalad dirsanaynta la jadwaliyay
scheduled-dispatch-manifest-writer-unavailable = Dirsanaynta la jadwaliyay: qoraha manifest ma diyaar; xaaladda entity wali sugaysaa
manifest-pin-update-failed = Manifest pin_update way ku guul-darraysatay
bootstrap-kind-registry-fetch-log-failed = Kuma guulaysan helitaanka kind node ee registry
bootstrap-entity-state-update-fetch-failed = Kuma guulaysan helitaanka entity node si loo cusboonaysiiyey xaaladda
schedule-stale-dispatch-skipped = Dirsanaynta la jadwaliyay waa la gudbay: jadwal duug ah
schedule-random-reschedule-failed = Kuma guulaysan dib-u-jadwalinta shaqada random
schedule-random-create-failed = Kuma guulaysan abuurida shaqada random ee xigta
schedule-random-chain-stopped = Silsilada jadwalka random waa joojisay: qeexitaan cusub ayaa beddel u noqday
schedule-entity-not-found = Dirsanaynta la jadwaliyay: entity lama helin
schedule-dispatch-firing = Dirsanaynta la jadwaliyay waxay socotaa
bootstrap-kind-registry-extends-failed = Kuma guulaysan xallinta silsilada extends ee kind ee registry
bootstrap-entity-registry-fetch-failed = Kuma guulaysan helitaanka entity node
bootstrap-entity-registry-kind-missing = Kind lama helin manifest; entity waa la gudbay
bootstrap-entity-registry-kind-fetch-failed = Kuma guulaysan helitaanka kind node
bootstrap-entity-registry-kind-extends-failed = Kuma guulaysan xallinta silsilada extends ee kind
bootstrap-manifest-fetch-failed = Kuma guulaysan helitaanka runtime manifest
bootstrap-minimal-manifest-failed = Kuma guulaysan bilowga manifest yar
bootstrap-entity-state-save-failed = Kuma guulaysan kaydinta xaaladaha entity
bootstrap-entity-state-shutdown-aborted = Xidnimaynta waa la joojiyay; runtime wali firfircoon yahay si xaaladda loo kaydin karo xidnimaynta xigta
bootstrap-root-cid-shutdown-persist-failed = Kuma guulaysan kaydinta root_cid inta lagu jiro xidnimaynta
bootstrap-root-cid-shutdown-publish-succeeded = Daabacaadda runtime_ipns inta lagu jiro xidnimaynta waa lagu guulaystay
bootstrap-root-cid-shutdown-publish-failed = Daabacaadda runtime_ipns inta lagu jiro xidnimaynta way ku guul-darraysatay
bootstrap-root-cid-shutdown-publish-timeout = Daabacaadda runtime_ipns inta lagu jiro xidnimaynta waxay dhammaysay waqtiga
bootstrap-endpoint-close-timeout = Xidnimaynta endpoint waxay dhammaysay 5 s ka dib; waxaa la xoojinayaa bixitaanka
bootstrap-endpoint-close-stuck = Endpoint waxaa haysta hawlaha duulaya ka dib 10 s; la siday iyada oo aan xidnimayn hab-wanaag
bootstrap-envelope-delivery-failed = Dhiibimaynta plugin envelope way ku guul-darraysatay; envelope waa la siday
bootstrap-envelope-open-failed = Plugin envelope: furnimaynta outbox way ku guul-darraysatay; envelope waa la siday
boot-minimal-manifest-not-found = Lama helin runtime root CID ee IPNS; waxaa la bilaabayaa manifest yar
boot-minimal-manifest-bootstrapped = Manifest yar ayaa la bilaabay
boot-kinds-overlay-no-change = Kinds overlay kuma baddelin manifest
boot-kinds-overlay-applied = Kinds overlay waa la fuliyay
boot-load-manifest-for-acl-cache-failed = Kuma guulaysan rarida manifest si loo buuxiyo ACL cache
boot-root-acl-loaded-from-manifest = Root transport-gate ACL waxaa laga rariyay manifest
boot-root-acl-load-failed = Kuma guulaysan rarida root ACL manifest
boot-group-loaded-into-cache = Kooxda waxaa lagu riyay cache
boot-group-load-failed = Kuma guulaysan rarida kooxda markii bilowga
boot-root-acl-loaded-into-cache = Root ACL waxaa lagu riyay cache
boot-root-acl-load-cache-failed = Kuma guulaysan rarida root ACL markii bilowga
boot-reconciled-owners-persist-failed = Kuma guulaysan kaydinta milkiilayaasha la midaynay ee config.yaml
boot-reconciled-owners-published = Milkiilayaasha waxaa laga soo midaynay config.yaml/--owner manifest markii bilowga
boot-reconciled-owners-manifest-failed = Kuma guulaysan midaynta milkiilayaasha manifest markii bilowga
boot-no-root-entity = Lama diiwaangelin root entity loogu talagalay default config root
boot-default-root-config-populated = Default config root waa la buuxiyay
boot-default-root-config-populate-failed = Kuma guulaysan buuxinta default config root
boot-entity-load-processed = Entity plugin waa la rariyay
plugin-envelope-local-recipient-unknown = Plugin envelope: qaabilaha maxalliga aan la garanayn; waa la gudbay
plugin-envelope-local-dispatch-start = Plugin envelope: dirsanaynta maxalliga waa la bilaabay
plugin-envelope-local-dispatch-finish = Plugin envelope: dirsanaynta maxalliga waa la dhammaystay
plugin-envelope-local-dispatch-failed = Plugin envelope: dirsanaynta maxalliga waxay ku guul-darraysatay
plugin-envelope-create-requests-ignored = Plugin envelope: codsiyada abuurista waa la iska indho tiray iyada oo aan lahayn context side-effect
ma-create-entity-already-exists = ma_create_entity: entity horay ayay u jirtay; entity hadda jirta waa la hayaa
ma-create-entity-kind-missing = ma_create_entity: kind kuma jiro registry; waa la gudbay
ma-create-entity-invalid-behaviour = ma_create_entity: tixraaca behaviour ma sax; waa la gudbay
crud-message-rejected = Fariinta CRUD waa la diiday
inbox-message-rejected = Fariinta inbox waa la diiday
