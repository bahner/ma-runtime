# ma-runtime – Igbo
lang-name = Igbo

own-did-published = Akwụkwọ DID m ejiri bipụta n'ime IPNS
own-did-publish-failed = Ọ dịghị maka ibipụta akwụkwọ DID m
own-did-publish-timeout = Ibipụta akwụkwọ DID m agafewo oge mgbe nkeji 2
started = ma runtime amalitela
shutdown-requested = Ọ chọtara mkpochapụ
closing-endpoint = Na-mechie iroh endpoint...
shutdown-complete = Mkpochapụ emechara
status-listening = Ọkwa ọnọdụ na-ege ntị
ipfs-message-rejected = Ahapụ ozi IPFS
ctrlc-handler-failed = Onye njikwa Ctrl-C dabara
node-connected = Ebe akwụkwọ jikọọ na usoro
received-encrypted-ma-msg = Nwetara ozi ma nke echekwara nzuzo na /ma/ipfs/0.0.1
ping-received = Nwetara :ping, na-eziga :pong
did-publish-request-received = Nwetara arịọ ibipụta akwụkwọ DID
document-published = Akwụkwọ bipụtara
did-publish-cid-reply-sent = Ezigara azịza CID maka ibipụta DID
did-publish-resolve-failed = Enweghị ike iwepụ onye zitere iji nyefe azịza ipfs-publish
ipfs-store-request-received = Nwetara arịọ nchekwa IPFS
ipfs-stored = Echekwara ọdịnaya na IPFS
ipfs-store-cid-reply-sent = Ezigara azịza CID
ipfs-store-resolve-failed = Enweghị ike iwepụ onye zitere iji nyefe azịza ipfs-store

# Nnyefe ihe dị ndụ
bootstrap-complete = Bootstrap emechara
entity-loaded = Ntinye ihe dị ndụ torowaa
entity-load-failed = Enweghị ike itinye ntinye ihe dị ndụ
root-list-entities = #root: ndepụta ihe dị ndụ
entity-created = Emepụtara ihe dị ndụ
entity-reloaded = Etinyegharịrị ntinye ihe dị ndụ
entity-deleted = Ehichapụ ihe dị ndụ
entity-states-saving = Na-echekwa ọnọdụ ihe dị ndụ na IPFS
entity-state-saving = Na-echekwa ọnọdụ ihe dị ndụ
entity-state-saved = Echekwara ọnọdụ ihe dị ndụ
entity-state-empty = Ntinye weghachitere ọnọdụ efu, wepụrụ ịchekwa
entity-states-saved = Echekwara ọnọdụ ihe dị ndụ

# Mmalite nke mbụ / auto-init

# Nwe ihe
runtime-claimed = Derewo runtime.

# Ihe ndabere echekwara
refuse-delete-root = Ọ dịghị maka ihichapụ ihe ndabere dị mkpa n'ikwuọ ókè
runtime-claim-persisted = Odeere nwe onye na nhazi.


# Namespace creation (:create)
crud-message-received = Enwetara ozi CRUD
crud-acl-updated = ACL nkwurita isi emegharịrị

# CRUD validation errors
cidv1-required = uru ahụ kwesịrị ịbụ CIDv1 dị mfe (malite na 'b'; CIDv0 'Qm…' anaghị anabata)
config-key-protected = igodo config '%key%' na-echekwa
config-key-no-delete = igodo config daemon '%key%' enweghị ike ihichapụ ya
config-key-not-manifest = igodo config '%key%' abụghị igodo manifest config ama ama
wrong-crud-protocol = protocol CRUD dị njọ: %type%
entity-name-invalid = aha entity ga abụ UTF-8 enwere ike ị depụta
reserved-entity-name = aha entity '%name%' edobere
genesis-kind-owner-only = Naanị onye nwe ma runtime ka ike ịmepụta entity nke ụdị genesis

# IPv6 config
ipv6-enabled = Enyere IPv6 ikike — na-ejikọ IPv4 na IPv6 abụọ
ipv6-disabled = E mechie IPv6 — naanị IPv4 ka e na-ejikọ (restart dị mkpa iji weghachite ya)
ipv6-enable-restart-required = Echekwara. Restart dị mkpa ka mgbanwe a bata n'ọrụ.
ipv6-enable-unchanged = Etolara ipv6_enable n'uru ahụ — enweghị mgbanwe.

# Igodo ọhụrụ
boot-default-root-config-populate-failed = Enweghị ike ịzụlite ngọpụta config ndụ
boot-default-root-config-populated = Azụlitere ngọpụta config ndụ
boot-entity-load-processed = Ebugara plugin entity
boot-group-load-failed = Enweghị ike ibugye otu n'oge mmalite
boot-group-loaded-into-cache = Ebugara otu n'ime cache
boot-kinds-overlay-applied = Etinye kinds overlay
boot-kinds-overlay-no-change = Kinds overlay emeghị mgbanwe ọ bụla na manifest
boot-load-manifest-for-acl-cache-failed = Enweghị ike ibugye manifest maka ịzụlite cache ACL
boot-minimal-manifest-bootstrapped = Edeghachara manifest kacha nta
boot-minimal-manifest-not-found = Ahụghị runtime root CID na IPNS; na-edeghachi manifest kacha nta
boot-no-root-entity = Ọ dịghị root entity dere maka ngọpụta config ndụ
boot-reconciled-owners-manifest-failed = Enweghị ike ijikọta ndị nwe n'ime manifest n'oge mmalite
boot-reconciled-owners-persist-failed = Enweghị ike ichekwa ndị nwe ijikọtara na config.yaml
boot-reconciled-owners-published = Ijikọtara ndị nwe site na config.yaml/--owner n'ime manifest n'oge mmalite
boot-root-acl-load-cache-failed = Enweghị ike ibugye ACL ndụ n'oge mmalite
boot-root-acl-load-failed = Enweghị ike ibugye ACL ndụ site na manifest
boot-root-acl-loaded-from-manifest = Ebugara ACL ọnụ ụzọ nnyefe ndụ site na manifest
boot-root-acl-loaded-into-cache = Ebugara ACL ndụ n'ime cache
bootstrap-acl-published = Bipụtara node ACL
bootstrap-endpoint-close-stuck = Endpoint ka nọ n'aka ọrụ ndị na-eme n'ikuku n'oge nkeji 10; na-ahapụ n'enweghị mmechi dị mfe
bootstrap-endpoint-close-timeout = Imechi endpoint gachara n'oge nkeji 5; na-agbakwunye ịpụ
bootstrap-entity-lifecycle-update-failed = Enweghị ike dee mmelite lifecycle entity na IPFS
bootstrap-entity-lifecycle-updated = Emeziero lifecycle entity na IPFS
bootstrap-entity-node-shutdown-updated = Emeziero node entity n'oge ịkwụsị
bootstrap-entity-published = Bipụtara node entity
bootstrap-entity-registering-prepublished = Na-edekọta entity e bipụtara tupu ugbu a
bootstrap-entity-registry-fetch-failed = Enweghị ike nweta node entity
bootstrap-entity-registry-kind-extends-failed = Enweghị ike idozi usoro extends kind
bootstrap-entity-registry-kind-fetch-failed = Enweghị ike nweta node kind
bootstrap-entity-registry-kind-missing = Ahụghị kind na manifest; na-agbagharị entity
bootstrap-entity-registry-not-in-manifest = Entity dị na registry mana ọ dịghị na manifest; na-agbagharị
bootstrap-entity-state-save-failed = Enweghị ike ichekwa ọnọdụ entity
bootstrap-entity-state-shutdown-aborted = Kagbuo ịkwụsị; runtime ka na-arụ ọrụ ka e nwee ike ichekwa ọnọdụ n'oge ịkwụsị ọzọ
bootstrap-entity-state-update-fetch-failed = Enweghị ike nweta node entity maka mmelite ọnọdụ
bootstrap-envelope-delivery-failed = Nnyefe envelope plugin dara ada; na-ahapụ envelope
bootstrap-envelope-open-failed = Envelope plugin: imepe outbox dara ada; na-ahapụ envelope
bootstrap-group-published = Bipụtara node otu
bootstrap-kind-published = Bipụtara node kind
bootstrap-kind-registry-extends-failed = Enweghị ike idozi usoro extends kind maka registry
bootstrap-kind-registry-fetch-log-failed = Enweghị ike nweta node kind maka registry
bootstrap-kind-registry-hydrated = Emeziero registry kind site na manifest
bootstrap-kinds-overlay-pin-update-failed = Pin/update dara ada ka kinds overlay gasịrị
bootstrap-kinds-overlay-published = Bipụtara manifest runtime ka kinds overlay gasịrị
bootstrap-kinds-tree-published = Bipụtara osisi kinds runtime
bootstrap-lifecycle-manifest-pin-update-failed = Pin/update dara ada ka ichekwa lifecycle gasịrị
bootstrap-lifecycle-manifest-publish-failed = Enweghị ike ibipụta manifest ka mgbanwe lifecycle gasịrị
bootstrap-lifecycle-manifest-published = Bipụtara manifest emeziri ka mgbanwe lifecycle gasịrị
bootstrap-manifest-fetch-failed = Enweghị ike nweta manifest runtime
bootstrap-minimal-manifest-failed = Enweghị ike ịdeghachi manifest kacha nta
bootstrap-remote-root-pin-confirmed = Akwado pin ndụ dị anya
bootstrap-remote-root-pin-misconfigured = E tọgbuo pinning ndụ dị anya nke ọma
bootstrap-root-acl-published = Bipụtara ACL ọnụ ụzọ nnyefe ndụ
bootstrap-root-cid-shutdown-persist-failed = Enweghị ike ichekwa root_cid n'oge ịkwụsị
bootstrap-root-cid-shutdown-publish-failed = Ibipụta runtime_ipns n'oge ịkwụsị dara ada
bootstrap-root-cid-shutdown-publish-succeeded = Ibipụta runtime_ipns n'oge ịkwụsị ga nke ọma
bootstrap-root-cid-shutdown-publish-timeout = Ibipụta runtime_ipns n'oge ịkwụsị gachara
bootstrap-root-pin-replacement-failed = Na-aga n'ihu ka nsogbu ọgbanwee pin ndụ dị anya gasịrị
bootstrap-root-pin-update-failed = Pin/update dara ada ka bootstrap gasịrị
bootstrap-runtime-manifest-published = Bipụtara manifest ndụ runtime
crud-message-rejected = Anụọ ozi CRUD
entity-reload-current-node-load-failed = Enweghị ike ibugye node entity ugbu a tupu ibugye ọzọ; na-echekwa plugin ugbu a
entity-reload-failed = Entity dara ada ịtọghachi; na-ewepụ ruo ịtọghachi ọzọ
entity-reload-kind-extends-failed = Enweghị ike idozi usoro extends kind n'oge ịtọghachi entity
entity-reload-kind-fetch-failed = Enweghị ike nweta node kind n'oge ịtọghachi entity
entity-reload-kind-lookup-failed = Enweghị ike ibugye manifest maka ịchọ kind n'oge ịtọghachi entity
entity-reload-kind-missing = Ahụghị kind na manifest; enweghị ike ịtọghachi entity
entity-reload-manifest-state-update-failed = Enweghị ike meziero manifest na ọnọdụ ugbu a tupu ibugye ọzọ; na-echekwa plugin ugbu a
entity-reload-skipped = Agbagharịrị ịtọghachi entity n'ihi na ọnụ ụzọ ịtọghachi mechiri
entity-reload-started = Ebidoro ịtọghachi entity
entity-reload-state-persist-failed = Enweghị ike ichekwa ọnọdụ ugbu a tupu ibugye ọzọ; na-echekwa plugin ugbu a
entity-reload-state-produced-failed = Enweghị ike ichekwa ọnọdụ e mepụtara n'oge ịtọghachi
entity-reloaded-manifest-update-failed = Enweghị ike meziero entity etọghachiri na manifest
entity-reloaded-manifest-updated = Emeziero entity etọghachiri na manifest
inbox-message-rejected = Anụọ ozi inbox
ma-create-entity-already-exists = ma_create_entity: entity dịrị checheye; na-echekwa entity ugbu a
ma-create-entity-invalid-behaviour = ma_create_entity: ntụaka behaviour na-ezighị ezi; agbagharịrị
ma-create-entity-kind-missing = ma_create_entity: kind adịghị na registry; agbagharịrị
manifest-pin-update-failed = Manifest pin_update dara ada
plugin-envelope-build-failed = Envelope plugin: enweghị ike ịrụ ozi; agbagharịrị
plugin-envelope-create-requests-ignored = Envelope plugin: na-echefu arịọ ịmepụta n'enweghị ọnọdụ side-effect
plugin-envelope-local-dispatch-failed = Envelope plugin: nnyefe mpaghara dara ada
plugin-envelope-local-dispatch-finish = Envelope plugin: nnyefe mpaghara mechiri
plugin-envelope-local-dispatch-start = Envelope plugin: nnyefe mpaghara malitere
plugin-envelope-local-gate-closed = Envelope plugin: ọnụ ụzọ nnyefe mpaghara mechiri
plugin-envelope-local-recipient-unknown = Envelope plugin: onye na-anata mpaghara a naghị ama; agbagharịrị
plugin-envelope-local-timeout = Envelope plugin: nnyefe mpaghara gachara
plugin-envelope-recipient-invalid = Envelope plugin: DID onye na-anata na-ezighị ezi; agbagharịrị
plugin-envelope-remote-limit = Envelope plugin: eruo oke nnyefe dị anya; ahapụrụ envelope
plugin-outbox-congested = Outbox plugin dị oke; enwere ike ihapụ envelope ọ bụrụ na usoro eju
plugin-outbox-drain-limit = Agwụrụ ego drain outbox plugin; na-emechaa envelope ndị fọdụrụ
schedule-dispatch-firing = Na-etinye nnyefe ndọrọ ndọrọ ọchịchị
schedule-entity-not-found = Nnyefe ndọrọ ndọrọ ọchịchị: ahụghị entity
schedule-random-chain-stopped = Usoro ndọrọ ndọrọ ọchịchị azọ kwụsiri: ihe ọzọ dị ọhụrụ wechiela ya
schedule-random-create-failed = Enweghị ike ịmepụta ọrụ azọ ọzọ
schedule-random-reschedule-failed = Enweghị ike itinye ọrụ azọ n'ndọrọ ndọrọ ọchịchị ọzọ
schedule-stale-dispatch-skipped = Agbagharịrị nnyefe ndọrọ ndọrọ ọchịchị: ndọrọ ndọrọ ọchịchị ochie
scheduled-dispatch-error = Njehie nnyefe ndọrọ ndọrọ ọchịchị
scheduled-dispatch-manifest-writer-unavailable = Nnyefe ndọrọ ndọrọ ọchịchị: onye na-ede manifest anaghị atọ; ọnọdụ entity ka nọ n'oge nche
