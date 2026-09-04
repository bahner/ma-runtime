# ma-runtime – Yorùbá
lang-name = Yorùbá

own-did-published = Àkọsílẹ̀ DID ti ara mi ti jẹ̀ tẹ̀jáde sí IPNS
own-did-publish-failed = Títẹ̀jáde àkọsílẹ̀ DID ti ara mi kùnà
own-did-publish-timeout = Títẹ̀jáde àkọsílẹ̀ DID ti ara mi gba àkókò ju ìdá 2 lọ
started = ma runtime ti bẹ̀rẹ̀
shutdown-requested = A ti béèrè ìpadasẹ̀yìn
closing-endpoint = Ó ń pa iroh endpoint...
shutdown-complete = Ìpadasẹ̀yìn ti parí
status-listening = Sèébà ipo ń gbọ́
ipfs-message-rejected = Ìfiránṣẹ́ IPFS ti kọ̀
ctrlc-handler-failed = Olùdarí Ctrl-C ti kùnà
node-connected = Eku-opo ti so mọ́ ìlànà
received-encrypted-ma-msg = Ìfiránṣẹ́ ma tí wọn sọ ní àṣírí ti wọlé ní /ma/ipfs/0.0.1
ping-received = :ping ti wọlé, ń rán :pong
did-publish-request-received = Ìbéèrè títẹ̀jáde àkọsílẹ̀ DID ti wọlé
document-published = Àkọsílẹ̀ ti tẹ̀jáde
did-publish-cid-reply-sent = Ìdáhùn CID fún títẹ̀jáde DID ti rán
did-publish-resolve-failed = Kùnà láti yanjú olùrán fún ìfíjísọ̀ ìdáhùn ipfs-publish
ipfs-store-request-received = Ìbéèrè àpamọ́ IPFS ti wọlé
ipfs-stored = Àkóónú ti fi pamọ́ sí IPFS
ipfs-store-cid-reply-sent = Ìdáhùn CID ti rán
ipfs-store-resolve-failed = Kùnà láti yanjú olùrán fún ìfíjísọ̀ ìdáhùn ipfs-store

# Ìránṣẹ́ ẹ̀dá
bootstrap-complete = Bootstrap ti parí
entity-loaded = Ẹ̀rọ ẹ̀dá ti kojọpọ
entity-load-failed = Kùnà láti kojọpọ ẹ̀rọ ẹ̀dá
root-list-entities = #root: àkójọ ẹ̀dá
entity-created = Ẹ̀dá ti ṣẹ̀dá
entity-reloaded = Ẹ̀rọ ẹ̀dá ti tún kojọpọ
entity-deleted = Ẹ̀dá ti paarẹ́
entity-states-saving = Ó ń fi ipò ẹ̀dá pamọ́ sí IPFS
entity-state-saving = Ó ń fi ipò ẹ̀dá pamọ́
entity-state-saved = Ipò ẹ̀dá ti fi pamọ́
entity-state-empty = Ẹ̀rọ dápadà ipò òfìfo, ìpamọ́ fo
entity-states-saved = Ipò ẹ̀dá ti fi pamọ́

# Ìbẹ̀rẹ̀ àkọ́kọ́ / ìbẹ̀rẹ̀ àdáṣe

# Ìní
runtime-claimed = Runtime ti forúkọsilẹ̀.

# Àwọn ẹ̀dá ìpìlẹ̀ tí a dáàbò bò
refuse-delete-root = Mo kọ̀ pátápátá láti pa ẹ̀dá ìpìlẹ̀ tí a nílò rẹ́
runtime-claim-persisted = Onígbọwọ ti kọ sí ìtọ́nisọ̀.


# Namespace creation (:create)
crud-message-received = A gba ifiranṣẹ CRUD
crud-acl-updated = ACL ọna gbigbe gbongbon ti ni imudojuiwọn

# CRUD validation errors
cidv1-required = iye gbọdọ jẹ CIDv1 bere (bẹrẹ pẹlu 'b'; CIDv0 'Qm…' ko gba)
config-key-protected = bọtini config '%key%' ni aabo
config-key-no-delete = bọtini config '%key%' ti daemon ko le parẹ
config-key-not-manifest = bọtini config '%key%' kii ṣe bọtini manifest config ti a mọ
wrong-crud-protocol = ilana CRUD ti ko tọ: %type%
entity-name-invalid = orúkọ entity gbọdọ jẹ UTF-8 tí a lè tẹ
reserved-entity-name = orúkọ entity '%name%' jẹ ìpamọ́genesis-kind-owner-only = Onígọ̀wọ́ runtime nìkan ló lè ṣẹ̀dá entity onírúurun genesis
# IPv6 config
ipv6-enabled = IPv6 ti ṣiṣẹ — n so IPv4 àti IPv6 papọ̀
ipv6-disabled = A ti pa IPv6 — IPv4 nikan ni a n sopọ (restart nilo lati mu pada wọle)
ipv6-enable-restart-required = Ti fi pamọ. Restart nilo lati fi iyipada yii sinu ẹrọ.
ipv6-enable-unchanged = ipv6_enable ti tẹlẹ ṣeto si iye yẹn — ko si iyipada.

entity-reload-skipped = Atunbere entity ti fo nitori ilẹkun atunbere ti wa ni pipade
entity-reload-started = Atunbere entity ti bẹrẹ
entity-reload-kind-lookup-failed = Ko le gba manifest fun wiwa kind lakoko atunbere entity
entity-reload-kind-missing = Kind ko si ninu manifest; ko le tun entity gba
entity-reload-kind-fetch-failed = Ko le gba kind node lakoko atunbere entity
entity-reload-kind-extends-failed = Ko le yanjú ẹwọn extends kind lakoko atunbere entity
entity-reload-manifest-state-update-failed = Ko le ṣe imudojuiwọn manifest pẹlu ipo lọwọlọwọ ṣaaju atunbere; n ṣetọju plugin lọwọlọwọ
entity-reload-state-persist-failed = Ko le fipamọ ipo lọwọlọwọ ṣaaju atunbere; n ṣetọju plugin lọwọlọwọ
entity-reload-current-node-load-failed = Ko le gba entity node lọwọlọwọ ṣaaju atunbere; n ṣetọju plugin lọwọlọwọ
entity-reload-failed = Entity ti kuna atunbere; n yọ kuro titi di atunbere to n bọ
entity-reload-state-produced-failed = Ko le fipamọ ipo ti ṣelọpọ lakoko atunbere
entity-reloaded-manifest-updated = Entity ti tun gba ti ṣe imudojuiwọn ninu manifest
entity-reloaded-manifest-update-failed = Ko le ṣe imudojuiwọn entity ti tun gba ninu manifest
bootstrap-remote-root-pin-misconfigured = Remote root pinning ti ṣeto ni aṣiṣe
bootstrap-remote-root-pin-confirmed = Remote root pin ti jẹrisi
bootstrap-kinds-tree-published = Igi kinds runtime ti tẹjade
bootstrap-kinds-overlay-pin-update-failed = Pin/update ti kuna lẹhin kinds overlay
bootstrap-kinds-overlay-published = Runtime manifest ti tẹjade lẹhin kinds overlay
bootstrap-runtime-manifest-published = Runtime root manifest ti tẹjade
bootstrap-root-pin-replacement-failed = N tẹsiwaju lẹhin ikuna rirọpo remote root pin
bootstrap-root-pin-update-failed = Pin/update ti kuna lẹhin bootstrap
bootstrap-kind-published = Kind node ti tẹjade
bootstrap-entity-registering-prepublished = N ṣe forukọsilẹ entity ti tẹjade tẹlẹ
bootstrap-entity-published = Entity node ti tẹjade
bootstrap-acl-published = ACL node ti tẹjade
bootstrap-group-published = Ẹgbẹ node ti tẹjade
bootstrap-root-acl-published = Root transport-gate ACL ti tẹjade
bootstrap-kind-registry-hydrated = Kind registry ti kun lati manifest
bootstrap-lifecycle-manifest-pin-update-failed = Pin/update ti kuna lẹhin fipamọ lifecycle
bootstrap-lifecycle-manifest-published = Manifest ti ṣe imudojuiwọn ti tẹjade lẹhin awọn iyipada lifecycle
bootstrap-lifecycle-manifest-publish-failed = Ko le tẹjade manifest lẹhin awọn iyipada lifecycle
bootstrap-entity-lifecycle-updated = Lifecycle entity ti ṣe imudojuiwọn ninu IPFS
bootstrap-entity-lifecycle-update-failed = Ko le kọ imudojuiwọn lifecycle entity si IPFS
bootstrap-entity-node-shutdown-updated = Entity node ti ṣe imudojuiwọn nigba pipade
bootstrap-entity-registry-not-in-manifest = Entity wa ninu registry ṣugbọn ko wa ninu manifest; n fo
plugin-outbox-drain-limit = Isuna ipadanu plugin outbox ti pari; n ṣe idaduro awọn envelope to ku
plugin-outbox-congested = Plugin outbox ti kún; awọn envelope le sọnu ti ikanni ba kun
plugin-envelope-local-gate-closed = Envelope plugin: ilẹkun fifiranṣẹ agbegbe ti wa ni pipade
plugin-envelope-local-timeout = Envelope plugin: fifiranṣẹ agbegbe ti pari akoko
plugin-envelope-recipient-invalid = Envelope plugin: DID olugba ko wulo; ti fo
plugin-envelope-build-failed = Envelope plugin: ko le kọ ifiranṣẹ; ti fo
plugin-envelope-remote-limit = Envelope plugin: opin jiṣẹ ti o jinna ti de; envelope ti ju silẹ
scheduled-dispatch-error = Aṣiṣe fifiranṣẹ ti ṣeto
scheduled-dispatch-manifest-writer-unavailable = Fifiranṣẹ ti ṣeto: akọwe manifest ko ṣetan; ipo entity si tun n duro
manifest-pin-update-failed = Manifest pin_update ti kuna
bootstrap-kind-registry-fetch-log-failed = Ko le gba kind node fun registry
bootstrap-entity-state-update-fetch-failed = Ko le gba entity node fun imudojuiwọn ipo
schedule-stale-dispatch-skipped = Fifiranṣẹ ti ṣeto ti fo: iṣeto atijọ
schedule-random-reschedule-failed = Ko le tun ṣeto iṣẹ aileto
schedule-random-create-failed = Ko le ṣẹda iṣẹ aileto to n bọ
schedule-random-chain-stopped = Ẹwọn iṣeto aileto ti duro: ti rọpo nipasẹ asọye tuntun
schedule-entity-not-found = Fifiranṣẹ ti ṣeto: entity ko ri
schedule-dispatch-firing = Fifiranṣẹ ti ṣeto n jọ
bootstrap-kind-registry-extends-failed = Ko le yanjú ẹwọn extends kind fun registry
bootstrap-entity-registry-fetch-failed = Ko le gba entity node
bootstrap-entity-registry-kind-missing = Kind ko wa ninu manifest; n fo entity
bootstrap-entity-registry-kind-fetch-failed = Ko le gba kind node
bootstrap-entity-registry-kind-extends-failed = Ko le yanjú ẹwọn extends kind
bootstrap-manifest-fetch-failed = Ko le gba runtime manifest
bootstrap-minimal-manifest-failed = Ko le bẹrẹ manifest kekere
bootstrap-entity-state-save-failed = Ko le fipamọ awọn ipo entity
bootstrap-entity-state-shutdown-aborted = Pipade ti fagile; runtime si tun n ṣiṣẹ ki ipo le fipamọ nigba pipade to n bọ
bootstrap-root-cid-shutdown-persist-failed = Ko le fipamọ root_cid lakoko pipade
bootstrap-root-cid-shutdown-publish-succeeded = Titẹjade runtime_ipns lakoko pipade ti ṣaṣeyọri
bootstrap-root-cid-shutdown-publish-failed = Titẹjade runtime_ipns lakoko pipade ti kuna
bootstrap-root-cid-shutdown-publish-timeout = Titẹjade runtime_ipns lakoko pipade ti pari akoko
bootstrap-endpoint-close-timeout = Pipade endpoint ti pari akoko lẹhin iṣẹju marun; fipa mu lati jade
bootstrap-endpoint-close-stuck = Endpoint si tun wa ninu awọn iṣẹ ti n fo lẹhin iṣẹju mẹwa; ti ju silẹ laisi pipade ọlọ́gbọ́n
bootstrap-envelope-delivery-failed = Jiṣẹ envelope plugin ti kuna; envelope ti ju silẹ
bootstrap-envelope-open-failed = Envelope plugin: ṣiṣi outbox ti kuna; envelope ti ju silẹ
boot-minimal-manifest-not-found = Ko ri runtime root CID ninu IPNS; n bẹrẹ manifest kekere
boot-minimal-manifest-bootstrapped = Manifest kekere ti bẹrẹ
boot-kinds-overlay-no-change = Kinds overlay ko yi manifest pada
boot-kinds-overlay-applied = Kinds overlay ti lo
boot-load-manifest-for-acl-cache-failed = Ko le gba manifest lati kun ACL cache
boot-root-acl-loaded-from-manifest = Root transport-gate ACL ti gba lati manifest
boot-root-acl-load-failed = Ko le gba root ACL lati manifest
boot-group-loaded-into-cache = Ẹgbẹ ti gba si cache
boot-group-load-failed = Ko le gba ẹgbẹ nigba ibẹrẹ
boot-root-acl-loaded-into-cache = Root ACL ti gba si cache
boot-root-acl-load-cache-failed = Ko le gba root ACL nigba ibẹrẹ
boot-reconciled-owners-persist-failed = Ko le fipamọ awọn oniwun ti ṣe irẹpọ si config.yaml
boot-reconciled-owners-published = Awọn oniwun ti ṣe irẹpọ lati config.yaml/--owner si manifest nigba ibẹrẹ
boot-reconciled-owners-manifest-failed = Ko le ṣe irẹpọ awọn oniwun si manifest nigba ibẹrẹ
boot-no-root-entity = Ko si root entity ti forukọsilẹ fun default config root
boot-default-root-config-populated = Default config root ti kun
boot-default-root-config-populate-failed = Ko le kun default config root
boot-entity-load-processed = Plugin entity ti gba
plugin-envelope-local-recipient-unknown = Envelope plugin: olugba agbegbe ti a ko mọ; ti fo
plugin-envelope-local-dispatch-start = Envelope plugin: fifiranṣẹ agbegbe ti bẹrẹ
plugin-envelope-local-dispatch-finish = Envelope plugin: fifiranṣẹ agbegbe ti pari
plugin-envelope-local-dispatch-failed = Envelope plugin: fifiranṣẹ agbegbe ti kuna
plugin-envelope-create-requests-ignored = Envelope plugin: awọn ibeere ẹda ti fojufo laisi ọrọ-ọrọ side-effect
ma-create-entity-already-exists = ma_create_entity: entity ti wa tẹlẹ; n ṣetọju entity lọwọlọwọ
ma-create-entity-kind-missing = ma_create_entity: kind ko wa ninu registry; ti fo
ma-create-entity-invalid-behaviour = ma_create_entity: itọkasi behaviour ko wulo; ti fo
crud-message-rejected = Ifiranṣẹ CRUD ti kọ
inbox-message-rejected = Ifiranṣẹ inbox ti kọ
genesis-kind-owner-only = Oniwun runtime nikan le ṣẹda entity ti genesis kind
