# ma-runtime – Fulfulde
lang-name = Fulfulde

own-did-published = Takkaare DID am jannginaa IPNS
own-did-publish-failed = Janngugo takkaare DID am waylaaki
own-did-publish-timeout = Janngugo takkaare DID am dariima caggal miinutuuje 2
started = ma runtime fuɗɗiima
shutdown-requested = Haaɗtugol heɓii
closing-endpoint = Ɗannugo iroh endpoint...
shutdown-complete = Haaɗtugol dariima
status-listening = Sarwiroo haala dengoo
ipfs-message-rejected = Ƈiiɗol IPFS rewindii
ctrlc-handler-failed = Janngiiɗo Ctrl-C waylaaki
node-connected = Nod jokkondiraa e laawol
received-encrypted-ma-msg = Ƈiiɗol ma wuurninaa heɓii to /ma/ipfs/0.0.1
ping-received = :ping heɓii, neldoo :pong
did-publish-request-received = Sarɗol janngugo takkaare DID heɓii
document-published = Takkaare jannginaa
did-publish-cid-reply-sent = Jaabawol CID janngugo DID neldaa
did-publish-resolve-failed = Waylaaki hoolanaago neldoowo faa jaabodoo ipfs-publish
ipfs-store-request-received = Sarɗol haaɗtugol IPFS heɓii
ipfs-stored = Kuutorɗe njaaɗii IPFS
ipfs-store-cid-reply-sent = Jaabawol CID neldaa
ipfs-store-resolve-failed = Waylaaki hoolanaago neldoowo faa jaabodoo ipfs-store

# Neldugo huunde
bootstrap-complete = Bootstrap dariima
entity-loaded = Seɗɗa huunde waɗii
entity-load-failed = Waɗugo seɗɗa huunde waylaaki
root-list-entities = #root: liste huunde
entity-created = Huunde fuɗɗii
entity-reloaded = Seɗɗa huunde waɗitaama
entity-deleted = Huunde momtaa
entity-states-saving = Momtugol ɗeɗɗe huunde IPFS
entity-state-saving = Momtugol ɗeɗɗe huunde
entity-state-saved = Ɗeɗɗe huunde momtaa
entity-state-empty = Seɗɗa yotti ɗeɗɗe jaaje, momtugo yejjitaa
entity-states-saved = Ɗeɗɗe huunde momtaa

# Fuɗɗugo adannde / auto-init

# Jom
runtime-claimed = Runtime faaɓinaa.

# Huunde jalte geɗe eɓɓooje
refuse-delete-root = Rewindoo tiiɗnde momtugol geɗe jalte bardinooje
runtime-claim-persisted = Jom winndirii maantol.


# Namespace creation (:create)
crud-message-received = Tinnde CRUD heɓii
crud-acl-updated = Root transport ACL humpitaama

# CRUD validation errors
cidv1-required = haddi na faa CIDv1 cokoyel (fuuta 'b'; CIDv0 'Qm…' acceppaaka)
config-key-protected = sorol config '%key%' nder keerol
config-key-no-delete = sorol config daemon '%key%' waawaa wanaa
config-key-not-manifest = sorol config '%key%' alaa e sorol manifest config gannduɗi
wrong-crud-protocol = protokol CRUD moƴƴi alaa: %type%
entity-name-invalid = innde entity ngoodha UTF-8 e bindateji
reserved-entity-name = innde entity '%name%' ndi dokkaa
genesis-kind-owner-only = Ko jom runtime tan waawi tagude huunde e juuɗe genesis

# IPv6 config
ipv6-enabled = IPv6 heɓii — jokku IPv4 e IPv6 ɗiɗi
ipv6-disabled = IPv6 haɓɓitaama — IPv4 tan woni seŋtiɗo (restart haɓeteeɗo ngam yuɓɓinde)
ipv6-enable-restart-required = Abbitaama. Restart haɓeteeɗo ngam tabitinde huɓɓinaango ngoo.
ipv6-enable-unchanged = ipv6_enable heɓii tan ko ɗuum boorti — alaa huunde feewi.

# Maande keewɗe
boot-default-root-config-populate-failed = Tinnuɗo heɓaani haaɗde config root deftere
boot-default-root-config-populated = Config root deftere heɓaama
boot-entity-load-processed = Entity plugin ɗe loggaama
boot-group-load-failed = Tinnuɗo heɓaani loga laaɓol sabu fuɗɗannde
boot-group-loaded-into-cache = Laaɓol loggaama e cache
boot-kinds-overlay-applied = Kinds overlay waɗaama
boot-kinds-overlay-no-change = Kinds overlay waɗaani goowna e manifest
boot-load-manifest-for-acl-cache-failed = Tinnuɗo heɓaani loga manifest ngam heɓɓinde cache ACL
boot-minimal-manifest-bootstrapped = Manifest tonngu bootstrapped
boot-minimal-manifest-not-found = Heɓaano root CID e runtime to IPNS; bootstrapping manifest tonngu
boot-no-root-entity = Maa jeyaa entity root ngam heɓde config root deftere
boot-reconciled-owners-manifest-failed = Tinnuɗo heɓaani tiiɗnude jom-galle e manifest sabu fuɗɗannde
boot-reconciled-owners-persist-failed = Tinnuɗo heɓaani mooɓtude jom-galle ɗe tiiɗnaama e config.yaml
boot-reconciled-owners-published = Jom-galle tiiɗnaama fes config.yaml/--owner e manifest sabu fuɗɗannde
boot-root-acl-load-cache-failed = Tinnuɗo heɓaani loga ACL root sabu fuɗɗannde
boot-root-acl-load-failed = Tinnuɗo heɓaani loga ACL root fes manifest
boot-root-acl-loaded-from-manifest = ACL tiiɗnande galooje root loggaama fes manifest
boot-root-acl-loaded-into-cache = ACL root loggaama e cache
bootstrap-acl-published = Noode ACL winndinaa
bootstrap-endpoint-close-stuck = Endpoint tan jeyaama e tiitoonde sabu 10 s; wantinaama sanne waylo
bootstrap-endpoint-close-timeout = Woosnude endpoint taaɓii sabu 5 s; wannginii
bootstrap-entity-lifecycle-update-failed = Tinnuɗo heɓaani winndude lifecycle entity e IPFS
bootstrap-entity-lifecycle-updated = Lifecycle entity heɓɓinaama e IPFS
bootstrap-entity-node-shutdown-updated = Noode entity heɓɓinaama sabu woosnude
bootstrap-entity-published = Noode entity winndinaa
bootstrap-entity-registering-prepublished = Winndinude entity winndaama daga yonta
bootstrap-entity-registry-fetch-failed = Tinnuɗo heɓaani heɓde noode entity
bootstrap-entity-registry-kind-extends-failed = Tinnuɗo heɓaani wuurnude silsila kind
bootstrap-entity-registry-kind-fetch-failed = Tinnuɗo heɓaani heɓde noode kind
bootstrap-entity-registry-kind-missing = Kind heɓaani e manifest; entity faawtaa
bootstrap-entity-registry-not-in-manifest = Entity e registry kono heɓaani e manifest; faawtaa
bootstrap-entity-state-save-failed = Tinnuɗo heɓaani mooɓtude dariibe entity
bootstrap-entity-state-shutdown-aborted = Woosnude hotorii; runtime jeyii kadi ngam mooɓtude dariibe e woosnude taata
bootstrap-entity-state-update-fetch-failed = Tinnuɗo heɓaani heɓde noode entity ngam heɓɓinannde dariibe
bootstrap-envelope-delivery-failed = Welnude envelope plugin tinnii; envelope wantinaa
bootstrap-envelope-open-failed = Envelope plugin: yaltude outbox tinnii; envelope wantinaa
bootstrap-group-published = Noode laaɓol winndinaa
bootstrap-kind-published = Noode kind winndinaa
bootstrap-kind-registry-extends-failed = Tinnuɗo heɓaani wuurnude silsila kind ngam registry
bootstrap-kind-registry-fetch-log-failed = Tinnuɗo heɓaani heɓde noode kind ngam registry
bootstrap-kind-registry-hydrated = Registry kind heɓɓinaama fes manifest
bootstrap-kinds-overlay-pin-update-failed = Pin/update tinnii caggal kinds overlay
bootstrap-kinds-overlay-published = Manifest runtime winndinaa caggal kinds overlay
bootstrap-kinds-tree-published = Maran kinds runtime winndinaa
bootstrap-lifecycle-manifest-pin-update-failed = Pin/update tinnii caggal mooɓtude lifecycle
bootstrap-lifecycle-manifest-publish-failed = Tinnuɗo heɓaani winndinde manifest caggal wanngine lifecycle
bootstrap-lifecycle-manifest-published = Manifest heɓɓinaama winndinaa caggal wanngine lifecycle
bootstrap-manifest-fetch-failed = Tinnuɗo heɓaani heɓde manifest runtime
bootstrap-minimal-manifest-failed = Tinnuɗo heɓaani bootagol manifest tonngu
bootstrap-remote-root-pin-confirmed = Pin root juural tiiɗtinaama
bootstrap-remote-root-pin-misconfigured = Pinning root juural heɓɓinaama taaɓol
bootstrap-root-acl-published = ACL tiiɗnande galooje root winndinaa
bootstrap-root-cid-shutdown-persist-failed = Tinnuɗo heɓaani mooɓtude root_cid e sabu woosnude
bootstrap-root-cid-shutdown-publish-failed = Winndineede runtime_ipns sabu woosnude tinnii
bootstrap-root-cid-shutdown-publish-succeeded = Winndineede runtime_ipns sabu woosnude tiinii
bootstrap-root-cid-shutdown-publish-timeout = Winndineede runtime_ipns sabu woosnude taaɓii waqtiiji
bootstrap-root-pin-replacement-failed = Wonkii caggal tinnuɗo jaaynde pin root juural
bootstrap-root-pin-update-failed = Pin/update tinnii caggal bootstrap
bootstrap-runtime-manifest-published = Manifest root runtime winndinaa
crud-message-rejected = Tindol CRUD faawtaa
entity-reload-current-node-load-failed = Tinnuɗo heɓaani loga noode entity hannde aroode laɓɓude; plugin hannde artaa
entity-reload-failed = Entity tinnii laɓɓude; wantineede haa laɓɓol goo-ɓanndu
entity-reload-kind-extends-failed = Tinnuɗo heɓaani wuurnude silsila kind sabu laɓɓude entity
entity-reload-kind-fetch-failed = Tinnuɗo heɓaani heɓde noode kind sabu laɓɓude entity
entity-reload-kind-lookup-failed = Tinnuɗo heɓaani loga manifest ngam faaɓde kind sabu laɓɓude entity
entity-reload-kind-missing = Kind heɓaani e manifest; heɓaani laɓɓude entity
entity-reload-manifest-state-update-failed = Tinnuɗo heɓaani heɓɓinande manifest dariibe hannde aroode laɓɓude; plugin hannde artaa
entity-reload-skipped = Laɓɓol entity faawtaa ngam portol laɓɓol wooftii
entity-reload-started = Laɓɓol entity fuɗɗii
entity-reload-state-persist-failed = Tinnuɗo heɓaani mooɓtude dariibe hannde aroode laɓɓude; plugin hannde artaa
entity-reload-state-produced-failed = Tinnuɗo heɓaani mooɓtude dariibe waɗama sabu laɓɓude
entity-reloaded-manifest-update-failed = Tinnuɗo heɓaani heɓɓinande entity laɓɓama e manifest
entity-reloaded-manifest-updated = Entity laɓɓama heɓɓinaama e manifest
inbox-message-rejected = Tindol inbox faawtaa
ma-create-entity-already-exists = ma_create_entity: entity jeyaa daga yonta; entity hannde artaa
ma-create-entity-invalid-behaviour = ma_create_entity: toɓde behaviour taaɓol; faawtaa
ma-create-entity-kind-missing = ma_create_entity: kind heɓaani e registry; faawtaa
manifest-pin-update-failed = Manifest pin_update tinnii
plugin-envelope-build-failed = Envelope plugin: tinnuɗo heɓaani heɓde tindol; faawtaa
plugin-envelope-create-requests-ignored = Envelope plugin: ɗiɗɗo ɓetde faawtaa sanne tonngol side-effect
plugin-envelope-local-dispatch-failed = Envelope plugin: laɓɓol ɗemngal tinnii
plugin-envelope-local-dispatch-finish = Envelope plugin: laɓɓol ɗemngal tiinii
plugin-envelope-local-dispatch-start = Envelope plugin: laɓɓol ɗemngal fuɗɗii
plugin-envelope-local-gate-closed = Envelope plugin: portol laɓɓol ɗemngal wooftii
plugin-envelope-local-recipient-unknown = Envelope plugin: heɓaaɗo ɗemngal maa anndaa; faawtaa
plugin-envelope-local-timeout = Envelope plugin: laɓɓol ɗemngal taaɓii
plugin-envelope-recipient-invalid = Envelope plugin: DID heɓaaɗo taaɓol; faawtaa
plugin-envelope-remote-limit = Envelope plugin: haaɗni welnude juural heɓɗo; envelope wantinaa
plugin-outbox-congested = Outbox plugin ɓurɗo heɓɓinde; envelope maa wantee so canel ɓuri wuurnande
plugin-outbox-drain-limit = Miiɗaari drain outbox plugin fewjii; envelope koo-ɗo jey faawtaa
schedule-dispatch-firing = Laɓɓol haalanaado fuɗɗi
schedule-entity-not-found = Laɓɓol haalanaado: entity heɓaani
schedule-random-chain-stopped = Silsila jaddi haalanaado haaɗi: jaaynii fes toɓde taata
schedule-random-create-failed = Tinnuɗo heɓaani waɗde tiinde taaɓol jaddi
schedule-random-reschedule-failed = Tinnuɗo heɓaani haalande laabi taaɓol jaddi
schedule-stale-dispatch-skipped = Laɓɓol haalanaado faawtaa: haalanaado mawɓe
scheduled-dispatch-error = Juumre laɓɓol haalanaado
scheduled-dispatch-manifest-writer-unavailable = Laɓɓol haalanaado: winndoowo manifest maa woodi; dariibe entity jeyaa kadi gila
