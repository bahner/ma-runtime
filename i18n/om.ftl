# ma-runtime – Afaan Oromoo
lang-name = Afaan Oromoo

own-did-published = Sanadni DID koo IPNS irratti maxxanfame
own-did-publish-failed = Sanadni DID koo maxxansuun hin milkoofne
own-did-publish-timeout = Maxxansuu sanadaa DID koo daqiiqaa 2 booda yeroon isaa darbee
started = ma runtime jalqabame
shutdown-requested = Dhaabuun gaafatame
closing-endpoint = iroh endpoint cufamaa jira...
shutdown-complete = Dhaabuun xumurameera
status-listening = Serveriin haala dhaggeeffachaa jira
ipfs-message-rejected = Ergaan IPFS haale
ctrlc-handler-failed = Bulchaan Ctrl-C hin milkoofne
node-connected = Nodiin sirna ilaalaa walitti qabame
received-encrypted-ma-msg = Ergaan ma faalame /ma/ipfs/0.0.1 irratti fudhatame
ping-received = :ping fudhatame, :pong erguuf
did-publish-request-received = Gaaffiin maxxansuu sanadaa DID fudhatame
document-published = Sanadni maxxanfame
did-publish-cid-reply-sent = Deebii CID maxxansuu DID tiif ergame
did-publish-resolve-failed = Ergaa kenna ipfs-publish deebisiisuf ergituu furuun hin milkoofne
ipfs-store-request-received = Gaaffiin kuusaa IPFS fudhatame
ipfs-stored = Qabiyyeen IPFS irratti kuufame
ipfs-store-cid-reply-sent = Deebii CID ergame
ipfs-store-resolve-failed = Ergaa kenna ipfs-store deebisiisuf ergituu furuun hin milkoofne

# Ergaa Dhaabbataa
bootstrap-complete = Bootstrap xumurameera
entity-loaded = Pilagiiniin dhaabbataa fe'ame
entity-load-failed = Pilagiinii dhaabbataa fe'uun hin milkoofne
root-list-entities = #root: tarreeffama dhaabbataa
entity-created = Dhaabbataan uumame
entity-reloaded = Pilagiiniin dhaabbataa irra deebiʼamee fe'ame
entity-deleted = Dhaabbataan haqame
entity-states-saving = Haala dhaabbataa IPFS irratti kuufamaa jira
entity-state-saving = Haala dhaabbataa kuufamaa jira
entity-state-saved = Haala dhaabbataa kuufame
entity-state-empty = Pilagiiniin haala duwwaa deebise, kuusaa irra darbame
entity-states-saved = Haala dhaabbataa kuufame

# Jalqaba jalqabaa / auto-init

# Abbummaa
runtime-claimed = Runtime galmeeffame.

# Waliigaltee iddoo jalqabaa eegame
refuse-delete-root = Iddoo jalqabaa barbaachisaa haquu cimsinaan didduu
runtime-claim-persisted = Abbayyaan qindaa'ina irratti barreeffame.


# Namespace creation (:create)
crud-message-received = Ergaa CRUD fudhatame
crud-acl-updated = ACL geejjibaa bu'uuraa haaromfame

# CRUD validation errors
cidv1-required = gatiin CIDv1 qulqulluu ta'uu qaba ('b' irraa jalqaba; CIDv0 'Qm…' hin fudhatamu)
config-key-protected = murtoo config '%key%' eeggama
config-key-no-delete = murtoo config daemon '%key%' haqamuu hin danda'amu
config-key-not-manifest = murtoo config '%key%' murtoo manifest config beekamaa miti
wrong-crud-protocol = protokoola CRUD dogoggoraa: %type%
entity-name-invalid = maqaan entity UTF-8 maxxansuu danda'u ta'uu qaba
reserved-entity-name = maqaan entity '%name%' kuufameera
genesis-kind-owner-only = Abbaan runtime qofa entity gosa genesis uumuu danda'a

# IPv6 config
ipv6-enabled = IPv6 dandeessifame — IPv4 fi IPv6 lachuu walqabsiisa
ipv6-disabled = IPv6 dhabamsiifame — IPv4 qofatu hidhame (deebi'uuf restart barbaachisa)
ipv6-enable-restart-required = Kuusame. Jijjiirraan kun hojii irra ooluuf restart barbaachisa.
ipv6-enable-unchanged = ipv6_enable durumaan gara gatii sanaatti qindaa'eera — jijjiirama hin jiru.

entity-reload-skipped = Entity haaromsi irra darbeera sababii haaromsa karaa cufaameef
entity-reload-started = Entity haaromsi jalqabameera
entity-reload-kind-lookup-failed = Entity haaromsaa yeroo kind barbaaduuf manifest fe'uu hin dandeenye
entity-reload-kind-missing = Kind manifest keessatti argamuu baate; entity haaromsuun hin danda'u
entity-reload-kind-fetch-failed = Entity haaromsaa yeroo kind node fudhuun hin dandeenye
entity-reload-kind-extends-failed = Entity haaromsaa yeroo kind extends hidhata furuun hin dandeenye
entity-reload-manifest-state-update-failed = Haaromsa dura haala ammaa waliin manifest haaromsuun hin dandeenye; plugin ammaa kuusameera
entity-reload-state-persist-failed = Haaromsa dura haala ammaa kuusuun hin dandeenye; plugin ammaa kuusameera
entity-reload-current-node-load-failed = Haaromsa dura entity node ammaa fe'uu hin dandeenye; plugin ammaa kuusameera
entity-reload-failed = Entity haaromsuun hin dandeenye; haaromsa ittaanuu hanga haquun qabame
entity-reload-state-produced-failed = Haaromsa yeroo uumame haala kuusuun hin dandeenye
entity-reloaded-manifest-updated = Entity haaromfame manifest keessatti haaromfameera
entity-reloaded-manifest-update-failed = Entity haaromfame manifest keessatti haaromsuun hin dandeenye
bootstrap-remote-root-pin-misconfigured = Remote root pinning dogoggoraan qindaaheera
bootstrap-remote-root-pin-confirmed = Remote root pin mirkanaaye
bootstrap-kinds-tree-published = Runtime kinds muka maxxanfameera
bootstrap-kinds-overlay-pin-update-failed = Kinds overlay booda Pin/update hin dandeenye
bootstrap-kinds-overlay-published = Runtime manifest kinds overlay booda maxxanfameera
bootstrap-runtime-manifest-published = Runtime root manifest maxxanfameera
bootstrap-root-pin-replacement-failed = Remote root pin bakka buusuun hin dandeenye booda itti fufameera
bootstrap-root-pin-update-failed = Bootstrap booda Pin/update hin dandeenye
bootstrap-kind-published = Kind node maxxanfameera
bootstrap-entity-registering-prepublished = Duraan maxxanfame entity galmeessaa jira
bootstrap-entity-published = Entity node maxxanfameera
bootstrap-acl-published = ACL node maxxanfameera
bootstrap-group-published = Garee node maxxanfameera
bootstrap-root-acl-published = Root transport-gate ACL maxxanfameera
bootstrap-kind-registry-hydrated = Kind registry manifest irraa guutameera
bootstrap-lifecycle-manifest-pin-update-failed = Lifecycle kuufama booda Pin/update hin dandeenye
bootstrap-lifecycle-manifest-published = Manifest haaromfame lifecycle jijjiirama booda maxxanfameera
bootstrap-lifecycle-manifest-publish-failed = Lifecycle jijjiirama booda manifest maxxansuun hin dandeenye
bootstrap-entity-lifecycle-updated = Entity lifecycle IPFS keessatti haaromfameera
bootstrap-entity-lifecycle-update-failed = Entity lifecycle haaromsa IPFS keessa barreessuu hin dandeenye
bootstrap-entity-node-shutdown-updated = Entity node cufinarratti haaromfameera
bootstrap-entity-registry-not-in-manifest = Entity registry keessa jira garuu manifest keessa hin jiru; irra darbame
plugin-outbox-drain-limit = Plugin outbox drain baajeti fixame; envelope hafan booda dabarfame
plugin-outbox-congested = Plugin outbox cufameera; karaan yoo guutame envelope gatamuu danda'a
plugin-envelope-local-gate-closed = Plugin envelope: naannoo darbumsaa karaa cufaameera
plugin-envelope-local-timeout = Plugin envelope: naannoo darbumsaan yeroo xumure
plugin-envelope-recipient-invalid = Plugin envelope: fudhattuu DID sirri miti; irra darbeera
plugin-envelope-build-failed = Plugin envelope: ergaa ijaarsuun hin dandeenye; irra darbeera
plugin-envelope-remote-limit = Plugin envelope: fagoo dhiheessuu daangaa gahame; envelope gatame
scheduled-dispatch-error = Yeroo qabame darbumsaa dogoggora
scheduled-dispatch-manifest-writer-unavailable = Yeroo qabame darbumsaa: manifest barreessaan qophaa'uu baate; entity haalli eeggachaa jira
manifest-pin-update-failed = Manifest pin_update hin dandeenye
bootstrap-kind-registry-fetch-log-failed = Kind node registry dhaaf fudhuun hin dandeenye
bootstrap-entity-state-update-fetch-failed = Entity node haala haaromsuuf fudhuun hin dandeenye
schedule-stale-dispatch-skipped = Yeroo qabame darbumsaan irra darbeera: yeroo qabame baay'ee durii
schedule-random-reschedule-failed = Hojii tasaa yeroo irra deebi'uun hin dandeenye
schedule-random-create-failed = Hojii tasaa ittaanuu uumuun hin dandeenye
schedule-random-chain-stopped = Yeroo tasaa hidhata dhaabate: hiikoo haarawaan bakka buufame
schedule-entity-not-found = Yeroo qabame darbumsaa: entity hin argamne
schedule-dispatch-firing = Yeroo qabame darbumsaan dhaabataa jira
bootstrap-kind-registry-extends-failed = Kind extends hidhata registry dhaaf furuun hin dandeenye
bootstrap-entity-registry-fetch-failed = Entity node fudhuun hin dandeenye
bootstrap-entity-registry-kind-missing = Kind manifest keessatti argamuu baate; entity irra darbame
bootstrap-entity-registry-kind-fetch-failed = Kind node fudhuun hin dandeenye
bootstrap-entity-registry-kind-extends-failed = Kind extends hidhata furuun hin dandeenye
bootstrap-manifest-fetch-failed = Runtime manifest fudhuun hin dandeenye
bootstrap-minimal-manifest-failed = Manifest xiqqaa jalqabuun hin dandeenye
bootstrap-entity-state-save-failed = Entity haala kuusuun hin dandeenye
bootstrap-entity-state-shutdown-aborted = Cufini haqame; runtime hojii irra jira haalli yeroo cufina ittaanuttis kuufamuu danda'u
bootstrap-root-cid-shutdown-persist-failed = Cufina yeroo root_cid kuusuun hin dandeenye
bootstrap-root-cid-shutdown-publish-succeeded = Cufina yeroo runtime_ipns maxxansuun milkaaye
bootstrap-root-cid-shutdown-publish-failed = Cufina yeroo runtime_ipns maxxansuun hin dandeenye
bootstrap-root-cid-shutdown-publish-timeout = Cufina yeroo runtime_ipns maxxansuun yeroo xumure
bootstrap-endpoint-close-timeout = Endpoint cufuun sekoondii 5 booda yeroo xumure; bahuuf dirqamame
bootstrap-endpoint-close-stuck = Endpoint hanga daqiiqaa 10 booda hojii irra jiraniif qabameera; cufuu salphaa malee gatame
bootstrap-envelope-delivery-failed = Plugin envelope dhiheessuu hin dandeenye; envelope gatame
bootstrap-envelope-open-failed = Plugin envelope: outbox banuun hin dandeenye; envelope gatame
boot-minimal-manifest-not-found = Runtime root CID IPNS keessatti argamuu baate; manifest xiqqaa jalqabaa
boot-minimal-manifest-bootstrapped = Manifest xiqqaan jalqabame
boot-kinds-overlay-no-change = Kinds overlay manifest hin jijjiirne
boot-kinds-overlay-applied = Kinds overlay hojii irra ooleera
boot-load-manifest-for-acl-cache-failed = Manifest ACL cache guutuf fe'uu hin dandeenye
boot-root-acl-loaded-from-manifest = Root transport-gate ACL manifest irraa fe'ameera
boot-root-acl-load-failed = Root ACL manifest irraa fe'uu hin dandeenye
boot-group-loaded-into-cache = Gareen cache keessa galame
boot-group-load-failed = Garee jalqaba irratti fe'uu hin dandeenye
boot-root-acl-loaded-into-cache = Root ACL cache keessa galameera
boot-root-acl-load-cache-failed = Root ACL jalqaba irratti fe'uu hin dandeenye
boot-reconciled-owners-persist-failed = Maxxantota walsimsiisamee config.yaml keessa kuusuu hin dandeenye
boot-reconciled-owners-published = Maxxantoti config.yaml/--owner irraa manifest keessatti walsimsiisamee jalqabarratti
boot-reconciled-owners-manifest-failed = Maxxantota manifest waliin walsimsiisuun hin dandeenye jalqabarratti
boot-no-root-entity = Root entity default config root dhaaf galmeeffamuu baate
boot-default-root-config-populated = Default config root guutameera
boot-default-root-config-populate-failed = Default config root guuttachuu hin dandeenye
boot-entity-load-processed = Plugin entity fe'amame
plugin-envelope-local-recipient-unknown = Plugin envelope: naannoo fudhattuu hin beekamne; irra darbeera
plugin-envelope-local-dispatch-start = Plugin envelope: naannoo darbumsaan jalqabame
plugin-envelope-local-dispatch-finish = Plugin envelope: naannoo darbumsaan xumurame
plugin-envelope-local-dispatch-failed = Plugin envelope: naannoo darbumsaa hin dandeenye
plugin-envelope-create-requests-ignored = Plugin envelope: uumuu gaaffiin side-effect context malee hin fudhatamu
ma-create-entity-already-exists = ma_create_entity: entity dursee jira; entity ammaa kuusameera
ma-create-entity-kind-missing = ma_create_entity: kind registry keessa hin jiru; irra darbeera
ma-create-entity-invalid-behaviour = ma_create_entity: behaviour wabii sirri miti; irra darbeera
crud-message-rejected = CRUD ergaan diddame
inbox-message-rejected = Inbox ergaan diddame
