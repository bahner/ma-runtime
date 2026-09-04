# ma-runtime – አማርኛ
lang-name = አማርኛ

own-did-published = የራሴ DID ሰነድ ወደ IPNS ታትሟል
own-did-publish-failed = የራሴ DID ሰነድ ማሳተም አልተሳካም
own-did-publish-timeout = የራሴ DID ሰነድ ህትመት ከ2 ደቂቃ በኋላ ጊዜ አልፎታል
started = ma runtime ተጀምሯል
shutdown-requested = ማሰናከያ ጥያቄ ቀርቧል
closing-endpoint = iroh endpoint ዝጋ...
shutdown-complete = ማሰናከያ ተጠናቋል
status-listening = የሁኔታ አገልጋይ እያዳመጠ ነው
ipfs-message-rejected = IPFS መልዕክት ተቀባይነት አላገኘም
ctrlc-handler-failed = Ctrl-C ሂደት ቆጣቢ አልተሳካም
node-connected = ኖድ ወደ ፕሮቶኮል ተገናኝቷል
received-encrypted-ma-msg = /ma/ipfs/0.0.1 ላይ ምስጢራዊ ma መልዕክት ደርሷል
ping-received = :ping ደርሷል፣ :pong እየላኩ
did-publish-request-received = DID ሰነድ ህትመት ጥያቄ ደርሷል
document-published = ሰነዱ ታትሟል
did-publish-cid-reply-sent = DID ህትመት CID ምላሽ ተልኳል
did-publish-resolve-failed = ipfs-publish ምላሽ ለማድረስ ላኪን መፍታት አልተሳካም
ipfs-store-request-received = IPFS ማከማቻ ጥያቄ ደርሷል
ipfs-stored = ይዘቱ ወደ IPFS ተከማቸ
ipfs-store-cid-reply-sent = CID ምላሽ ተልኳል
ipfs-store-resolve-failed = ipfs-store ምላሽ ለማድረስ ላኪን መፍታት አልተሳካም

# ሀ/ሰ ላኪ
bootstrap-complete = Bootstrap ተጠናቋል
entity-loaded = ሀ/ሰ ፕለጊን ተጭኗል
entity-load-failed = ሀ/ሰ ፕለጊን መጫን አልተሳካም
root-list-entities = #root: ሀ/ሰዎች ዝርዝር
entity-created = ሀ/ሰ ተፈጥሯል
entity-reloaded = ሀ/ሰ ፕለጊን እንደገና ተጭኗል
entity-deleted = ሀ/ሰ ተሰርዟል
entity-states-saving = ሀ/ሰ ሁኔታዎች ወደ IPFS እየተቀመጡ ነው
entity-state-saving = ሀ/ሰ ሁኔታ እየተቀመጠ ነው
entity-state-saved = ሀ/ሰ ሁኔታ ተቀምጧል
entity-state-empty = ፕለጊን ባዶ ሁኔታ መለሰ፣ ማቀናበርን ዘሏል
entity-states-saved = ሀ/ሰ ሁኔታዎች ተቀምጠዋል

# የመጀመሪያ ጅምር / ራስ-ሰር

# ባለቤትነት
runtime-claimed = Runtime ተመዝግቧል።

# የተጠበቁ ሥር ንጥረ-ነገሮች
refuse-delete-root = አስፈላጊ ሥር ንጥረ-ነገርን ለመሰረዝ በጥብቅ ፈቃደኛ አይደለሁም
runtime-claim-persisted = ባለቤቱ ወደ ቅንብር ተፅፏል።

# Namespace creation (:create)
crud-message-received = CRUD መልዕክት ተቀብሏል
crud-acl-updated = Root transport ACL ታደሰ

# CRUD validation errors
cidv1-required = ዋጋ ጥሬ CIDv1 መሆን አለበት ('b' ይጀምራል; CIDv0 'Qm…' አይፈቀድም)
config-key-protected = የ config ቁልፍ '%key%' ጥበቃ ስር ነው
config-key-no-delete = የ daemon config ቁልፍ '%key%' ሊሰረዝ አይችልም
config-key-not-manifest = የ config ቁልፍ '%key%' የሚታወቅ manifest config ቁልፍ አይደለም
wrong-crud-protocol = ስህተት CRUD ፕሮቶኮል: %type%
entity-name-invalid = የ entity ስም ሊታተም የሚችል UTF-8 መሆን አለበት
reserved-entity-name = የ entity ስም '%name%' የተጠበቀ ነው
genesis-kind-owner-only = የ ma runtime ባለቤት ብቻ ነው የ genesis ዓይነት ሀ/ሰ መፍጠር የሚችለው

# IPv6 config
ipv6-enabled = IPv6 ነቅቷል — IPv4 እና IPv6 ሁለቱንም እያያዘ
ipv6-disabled = IPv6 ተሰናክሏል — IPv4 ብቻ እየተሳሰረ ነው (እንደገና ለማስቻል restart ያስፈልጋል)
ipv6-enable-restart-required = ተቀምጧል። ይህ ለውጥ ሥራ ላይ እንዲውል restart ያስፈልጋል።
ipv6-enable-unchanged = ipv6_enable ቀድሞውኑ ወደዚያ ዋጋ ተቀምጧል — ምንም ለውጥ የለም።

# አዲስ ቁልፎች
boot-default-root-config-populate-failed = ነባሪ ውቅረት ሥር ሊሞላ አልቻለም
boot-default-root-config-populated = ነባሪ ውቅረት ሥር ተሞልቷል
boot-entity-load-processed = Entity ፕለጊኖች ተጭነዋል
boot-group-load-failed = ቡድኑ በጅምር ላይ ሊጫን አልቻለም
boot-group-loaded-into-cache = ቡድኑ ወደ ካሽ ተጭኗል
boot-kinds-overlay-applied = Kinds overlay ተተግብሯል
boot-kinds-overlay-no-change = Kinds overlay ምንም ለውጥ ወደ manifest አላደረገም
boot-load-manifest-for-acl-cache-failed = Manifest ለ ACL ካሽ ሙሌት ሊጫን አልቻለም
boot-minimal-manifest-bootstrapped = ትንሹ manifest ተጀምሯል
boot-minimal-manifest-not-found = ምንም runtime root CID በ IPNS አልተገኘም፤ ትንሹ manifest ይጀምራል
boot-no-root-entity = ለነባሪ ውቅረት ሥር ምንም root entity አልተመዘገበም
boot-reconciled-owners-manifest-failed = ባለቤቶችን ወደ manifest ማዋሃድ በጅምር ላይ ተሳናው
boot-reconciled-owners-persist-failed = ማዋሃጃ ባለቤቶችን ወደ config.yaml ማስቀመጥ ተሳናው
boot-reconciled-owners-published = ባለቤቶች ከ config.yaml/--owner ወደ manifest በጅምር ላይ ተዋህደዋል
boot-root-acl-load-cache-failed = Root ACL በጅምር ላይ ሊጫን አልቻለም
boot-root-acl-load-failed = Root ACL ከ manifest ሊጫን አልቻለም
boot-root-acl-loaded-from-manifest = Root transport ACL ከ manifest ተጭኗል
boot-root-acl-loaded-into-cache = Root ACL ወደ ካሽ ተጭኗል
bootstrap-acl-published = ACL ኖድ ታትሟል
bootstrap-endpoint-close-stuck = endpoint አሁንም ከ 10 ሰ በኋላ በሚሄዱ ተግባራት ተይዟል፤ ያለ ጥንቃቄ ይጣላል
bootstrap-endpoint-close-timeout = endpoint መዝጋት ከ 5 ሰ በኋላ አለቀ፤ ዋናው ይጠናቀቃል
bootstrap-entity-lifecycle-update-failed = ወደ IPFS የ entity lifecycle ዝማኔ ሊፃፍ አልቻለም
bootstrap-entity-lifecycle-updated = Entity lifecycle በ IPFS ዘምኗል
bootstrap-entity-node-shutdown-updated = Entity ኖድ በማጥፋት ላይ ዘምኗል
bootstrap-entity-published = Entity ኖድ ታትሟል
bootstrap-entity-registering-prepublished = አስቀድሞ የታተመ entity ይመዘገባል
bootstrap-entity-registry-fetch-failed = Entity ኖድ ሊወሰድ አልቻለም
bootstrap-entity-registry-kind-extends-failed = Kind ሰንሰለት ሊፈታ አልቻለም
bootstrap-entity-registry-kind-fetch-failed = Kind ኖድ ሊወሰድ አልቻለም
bootstrap-entity-registry-kind-missing = Kind በ manifest ውስጥ አልተገኘም፤ entity ይዘለላል
bootstrap-entity-registry-not-in-manifest = Entity በ registry ውስጥ አለ ነገር ግን በ manifest ውስጥ የለም፤ ይዘለላል
bootstrap-entity-state-save-failed = Entity ሁኔቶችን ማስቀመጥ ተሳናው
bootstrap-entity-state-shutdown-aborted = ማጥፋት ተቋርጧል፤ runtime ንቁ ሆኖ ይቆያል ሁኔቱ ወደፊት ሊቀመጥ ይችላል
bootstrap-entity-state-update-fetch-failed = Entity ኖድ ለሁኔት ዝማኔ ሊወሰድ አልቻለም
bootstrap-envelope-delivery-failed = Plugin envelope ርክክብ ተሳናው፤ envelope ይጣላል
bootstrap-envelope-open-failed = Plugin envelope፡ outbox ሊከፈት አልቻለም፤ envelope ይጣላል
bootstrap-group-published = ቡድን ኖድ ታትሟል
bootstrap-kind-published = Kind ኖድ ታትሟል
bootstrap-kind-registry-extends-failed = Kind ሰንሰለት ለ registry ሊፈታ አልቻለም
bootstrap-kind-registry-fetch-log-failed = Kind ኖድ ለ registry ሊወሰድ አልቻለም
bootstrap-kind-registry-hydrated = Kind registry ከ manifest ተሞልቷል
bootstrap-kinds-overlay-pin-update-failed = Kinds overlay ከኋላ pin/ዝማኔ ተሳናው
bootstrap-kinds-overlay-published = Runtime manifest ከ kinds overlay ከኋላ ታትሟል
bootstrap-kinds-tree-published = Runtime kinds ዛፍ ታትሟል
bootstrap-lifecycle-manifest-pin-update-failed = Lifecycle ከኋላ pin/ዝማኔ ተሳናው
bootstrap-lifecycle-manifest-publish-failed = Manifest ከ lifecycle ሽግግሮች ከኋላ ሊታተም አልቻለም
bootstrap-lifecycle-manifest-published = ዘምኗል manifest ከ lifecycle ሽግግሮች ከኋላ ታትሟል
bootstrap-manifest-fetch-failed = Runtime manifest ሊወሰድ አልቻለም
bootstrap-minimal-manifest-failed = ትንሹ manifest ሊጀምር አልቻለም
bootstrap-remote-root-pin-confirmed = የርቀት root pin ተረጋግጧል
bootstrap-remote-root-pin-misconfigured = የርቀት root pinning በሳሳ ቅንጅት ተዘጋጅቷል
bootstrap-root-acl-published = Root transport ACL ታትሟል
bootstrap-root-cid-shutdown-persist-failed = root_cid በማጥፋት ላይ ሊቀመጥ አልቻለም
bootstrap-root-cid-shutdown-publish-failed = runtime_ipns ህትመት በማጥፋት ላይ ተሳናው
bootstrap-root-cid-shutdown-publish-succeeded = runtime_ipns ህትመት በማጥፋት ላይ ተሳካ
bootstrap-root-cid-shutdown-publish-timeout = runtime_ipns ህትመት በማጥፋት ላይ አለቀ
bootstrap-root-pin-replacement-failed = ከ የርቀት root pin ለውጥ ስህተት ከኋላ ቀጠለ
bootstrap-root-pin-update-failed = Bootstrap ከኋላ pin/ዝማኔ ተሳናው
bootstrap-runtime-manifest-published = Runtime root manifest ታትሟል
crud-message-rejected = CRUD መልዕክት ውድቅ ሆነ
entity-reload-current-node-load-failed = ወቅታዊ entity ኖድ ከድጋሚ ጭነት በፊት ሊጫን አልቻለም፤ ወቅታዊ plugin ቆይቷል
entity-reload-failed = Entity ሊደጋጭ አልቻለም፤ እስከ ቀጣዩ ድጋሚ ጭነት ይዘጋል
entity-reload-kind-extends-failed = Kind ሰንሰለት ​​ entity ሲደጋጭ ሊፈታ አልቻለም
entity-reload-kind-fetch-failed = Kind ኖድ entity ሲደጋጭ ሊወሰድ አልቻለም
entity-reload-kind-lookup-failed = Manifest ለ kind ፍለጋ entity ሲደጋጭ ሊጫን አልቻለም
entity-reload-kind-missing = Kind በ manifest ውስጥ አልተገኘም፤ entity ሊደጋጭ አይቻልም
entity-reload-manifest-state-update-failed = Manifest ወቅታዊ ሁኔት ከድጋሚ ጭነት በፊት ሊዘምን አልቻለም፤ ወቅታዊ plugin ቆይቷል
entity-reload-skipped = Entity ድጋሚ ጭነት ዘሏል ምክንያቱም ድጋሚ ጭነት በር ተዘግቷል
entity-reload-started = Entity ድጋሚ ጭነት ጀምሯል
entity-reload-state-persist-failed = ወቅታዊ ሁኔት ከድጋሚ ጭነት በፊት ሊቀመጥ አልቻለም፤ ወቅታዊ plugin ቆይቷል
entity-reload-state-produced-failed = ድጋሚ ጭነት ወቅት የተፈጠረ ሁኔት ሊቀመጥ አልቻለም
entity-reloaded-manifest-update-failed = የደጋጭ entity manifest ሊዘምን አልቻለም
entity-reloaded-manifest-updated = የደጋጭ entity manifest ዘምኗል
inbox-message-rejected = Inbox መልዕክት ውድቅ ሆነ
ma-create-entity-already-exists = ma_create_entity፡ entity አስቀድሞ አለ፤ ወቅታዊ entity ቆይቷል
ma-create-entity-invalid-behaviour = ma_create_entity፡ ዋጋ ያጣ behaviour ማጣቀሻ፤ ዘሏል
ma-create-entity-kind-missing = ma_create_entity፡ kind በ registry ውስጥ የለም፤ ዘሏል
manifest-pin-update-failed = manifest pin_update ተሳናው
plugin-envelope-build-failed = Plugin envelope፡ መልዕክት ሊፈጠር አልቻለም፤ ዘሏል
plugin-envelope-create-requests-ignored = Plugin envelope፡ ፍጠር ጥያቄዎች ያለ side-effect context ይሰናከላሉ
plugin-envelope-local-dispatch-failed = Plugin envelope፡ ቦታዊ ርክክብ ተሳናው
plugin-envelope-local-dispatch-finish = Plugin envelope፡ ቦታዊ ርክክብ ተጠናቀቀ
plugin-envelope-local-dispatch-start = Plugin envelope፡ ቦታዊ ርክክብ ጀምሯል
plugin-envelope-local-gate-closed = Plugin envelope፡ ቦታዊ ርክክብ በር ተዘግቷል
plugin-envelope-local-recipient-unknown = Plugin envelope፡ ያልታወቀ ቦታዊ ተቀባይ፤ ዘሏል
plugin-envelope-local-timeout = Plugin envelope፡ ቦታዊ ርክክብ አለቀ
plugin-envelope-recipient-invalid = Plugin envelope፡ ዋጋ ያጣ ተቀባይ DID፤ ዘሏል
plugin-envelope-remote-limit = Plugin envelope፡ የርቀት ርክክብ ወሰን ደርሷል፤ envelope ይጣላል
plugin-outbox-congested = Plugin outbox ጥቅጥቅ ብሏል፤ ቻናሉ ሲሞላ envelope ሊጣሉ ይችላሉ
plugin-outbox-drain-limit = Plugin outbox drain budget ሞልቷል፤ ቀሪ envelope ዘግይቷል
schedule-dispatch-firing = የታቀደ ርክክብ ይፋ ሆኗል
schedule-entity-not-found = የታቀደ ርክክብ፡ entity አልተገኘም
schedule-random-chain-stopped = ዘፈቀደ የጊዜ ሰሌዳ ሰንሰለት ቆሟል፡ በቅርብ ትርጓሜ ተተክቷል
schedule-random-create-failed = ቀጣዩ ዘፈቀደ ሥራ ሊፈጠር አልቻለም
schedule-random-reschedule-failed = ዘፈቀደ ሥራ እንደገና ሊታቀድ አልቻለም
schedule-stale-dispatch-skipped = የታቀደ ርክክብ ዘሏል፡ ያረጀ ዕቅድ
scheduled-dispatch-error = የታቀደ ርክክብ ስህተት
scheduled-dispatch-manifest-writer-unavailable = የታቀደ ርክክብ፡ manifest writer ዝግጁ አይደለም፤ entity ሁኔት ባልተፈታ ሁኔቱ ይቆያል
