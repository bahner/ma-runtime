# ma-runtime – 한국어
lang-name = 한국어

own-did-published = 자신의 DID 문서가 IPNS에 게시되었습니다
own-did-publish-failed = 자신의 DID 문서 게시에 실패했습니다
own-did-publish-timeout = 자신의 DID 문서 게시가 2분 후에 시간 초과되었습니다
started = ma runtime이 시작되었습니다
shutdown-requested = 종료가 요청되었습니다
closing-endpoint = iroh 엔드포인트를 닫는 중...
shutdown-complete = 종료가 완료되었습니다
status-listening = 상태 서버가 수신 대기 중입니다
ipfs-message-rejected = IPFS 메시지가 거부되었습니다
ctrlc-handler-failed = Ctrl-C 핸들러가 실패했습니다
node-connected = 노드가 프로토콜에 연결되었습니다
received-encrypted-ma-msg = /ma/ipfs/0.0.1에서 암호화된 ma 메시지를 수신했습니다
ping-received = :ping 수신, :pong 전송 중
did-publish-request-received = DID 문서 게시 요청을 수신했습니다
document-published = 문서가 게시되었습니다
did-publish-cid-reply-sent = DID 게시를 위한 CID 응답을 전송했습니다
did-publish-resolve-failed = ipfs-publish 응답 전달을 위한 발신자 확인에 실패했습니다
ipfs-store-request-received = IPFS 저장 요청을 수신했습니다
ipfs-stored = 콘텐츠가 IPFS에 저장되었습니다
ipfs-store-cid-reply-sent = CID 응답을 전송했습니다
ipfs-store-resolve-failed = ipfs-store 응답 전달을 위한 발신자 확인에 실패했습니다

# 엔티티 디스패치
bootstrap-complete = Bootstrap이 완료되었습니다
entity-loaded = 엔티티 플러그인이 로드되었습니다
entity-load-failed = 엔티티 플러그인 로드에 실패했습니다
root-list-entities = #root: 엔티티 목록
entity-created = 엔티티가 생성되었습니다
entity-reloaded = 엔티티 플러그인이 다시 로드되었습니다
entity-deleted = 엔티티가 삭제되었습니다
entity-states-saving = 엔티티 상태를 IPFS에 저장하는 중
entity-state-saving = 엔티티 상태를 저장하는 중
entity-state-saved = 엔티티 상태가 저장되었습니다
entity-state-empty = 플러그인이 빈 상태를 반환했습니다, 저장을 건너뜁니다
entity-states-saved = 엔티티 상태가 저장되었습니다

# 첫 번째 시작 / 자동 초기화

# 소유권
runtime-claimed = Runtime이 등록되었습니다.

# 보호된 루트 요소
refuse-delete-root = 필요한 루트 요소 삭제를 단호히 거부합니다
runtime-claim-persisted = 소유자가 설정에 기록되었습니다.


# Namespace creation (:create)
crud-message-received = CRUD 메시지 수신됨
crud-acl-updated = 루트 전송 ACL 업데이트됨

# CRUD validation errors
cidv1-required = 값은 순수한 CIDv1이어야 합니다 ('b'로 시작; CIDv0 'Qm…' 허용되지 않음)
config-key-protected = config 키 '%key%'은 보호되어 있습니다
config-key-no-delete = daemon config 키 '%key%'은 삭제할 수 없습니다
config-key-not-manifest = config 키 '%key%'은 알려진 manifest config 키가 아닙니다
wrong-crud-protocol = 잘못된 CRUD 프로토콜: %type%
entity-name-invalid = entity 이름은 출력 가능한 UTF-8이어야 합니다
reserved-entity-name = entity 이름 '%name%'은 예약되어 있습니다
genesis-kind-owner-only = genesis 종류의 entity는 runtime 소유자만 생성할 수 있습니다

# IPv6 config
ipv6-enabled = IPv6 활성화됨 — IPv4 및 IPv6 모두 바인딩 중
ipv6-disabled = IPv6 비활성화됨 — IPv4만 바인딩 중 (재활성화하려면 restart 필요)
ipv6-enable-restart-required = 저장되었습니다. 이 변경 사항이 적용되려면 restart가 필요합니다.
ipv6-enable-unchanged = ipv6_enable은 이미 해당 값으로 설정되어 있습니다 — 변경 없음.

boot-default-root-config-populate-failed = 기본 설정 루트를 채우는 데 실패했습니다
boot-default-root-config-populated = 기본 설정 루트가 채워졌습니다
boot-entity-load-processed = 엔티티 플러그인을 로드했습니다
boot-group-load-failed = 시작 시 그룹 로드에 실패했습니다
boot-group-loaded-into-cache = 그룹을 캐시에 로드했습니다
boot-kinds-overlay-applied = kinds 오버레이가 적용되었습니다
boot-kinds-overlay-no-change = kinds 오버레이가 매니페스트에 변경을 가하지 않았습니다
boot-load-manifest-for-acl-cache-failed = ACL 캐시 채우기용 매니페스트 로드에 실패했습니다
boot-minimal-manifest-bootstrapped = 최소 매니페스트를 초기화했습니다
boot-minimal-manifest-not-found = IPNS에서 런타임 루트 CID를 찾을 수 없습니다. 최소 매니페스트를 초기화합니다
boot-no-root-entity = 기본 설정 루트용 루트 엔티티가 등록되지 않았습니다
boot-reconciled-owners-manifest-failed = 시작 시 소유자를 매니페스트에 조정하는 데 실패했습니다
boot-reconciled-owners-persist-failed = 조정된 소유자를 config.yaml에 저장하는 데 실패했습니다
boot-reconciled-owners-published = 시작 시 config.yaml/--owner의 소유자를 매니페스트에 조정했습니다
boot-root-acl-load-cache-failed = 시작 시 루트 ACL 로드에 실패했습니다
boot-root-acl-load-failed = 매니페스트에서 루트 ACL 로드에 실패했습니다
boot-root-acl-loaded-from-manifest = 매니페스트에서 루트 전송 ACL을 로드했습니다
boot-root-acl-loaded-into-cache = 루트 ACL을 캐시에 로드했습니다
bootstrap-acl-published = ACL 노드를 게시했습니다
bootstrap-endpoint-close-stuck = 10초 후에도 엔드포인트가 실행 중인 작업에 의해 유지되고 있습니다. 강제 종료합니다
bootstrap-endpoint-close-timeout = 5초 후 엔드포인트 닫기가 시간 초과되었습니다. 강제 종료합니다
bootstrap-entity-lifecycle-update-failed = 업데이트된 엔티티 수명 주기를 IPFS에 쓰는 데 실패했습니다
bootstrap-entity-lifecycle-updated = 엔티티 수명 주기를 IPFS에서 업데이트했습니다
bootstrap-entity-node-shutdown-updated = 종료 시 엔티티 노드를 업데이트했습니다
bootstrap-entity-published = 엔티티 노드를 게시했습니다
bootstrap-entity-registering-prepublished = 사전 게시된 엔티티를 등록 중입니다
bootstrap-entity-registry-fetch-failed = 엔티티 노드를 가져오는 데 실패했습니다
bootstrap-entity-registry-kind-extends-failed = kind 확장 체인 확인에 실패했습니다
bootstrap-entity-registry-kind-fetch-failed = kind 노드를 가져오는 데 실패했습니다
bootstrap-entity-registry-kind-missing = 매니페스트에서 kind를 찾을 수 없습니다. 엔티티를 건너뜁니다
bootstrap-entity-registry-not-in-manifest = 엔티티가 레지스트리에는 있지만 매니페스트에는 없습니다. 건너뜁니다
bootstrap-entity-state-save-failed = 엔티티 상태 저장에 실패했습니다
bootstrap-entity-state-shutdown-aborted = 종료가 중단되었습니다. 다음 종료 시도에서 상태를 저장할 수 있도록 런타임이 활성 상태를 유지합니다
bootstrap-entity-state-update-fetch-failed = 상태 업데이트를 위한 엔티티 노드 가져오기에 실패했습니다
bootstrap-envelope-delivery-failed = 플러그인 엔벨로프 전달에 실패했습니다. 엔벨로프를 삭제합니다
bootstrap-envelope-open-failed = 플러그인 엔벨로프: 아웃박스를 열지 못했습니다. 엔벨로프를 삭제합니다
bootstrap-group-published = 그룹 노드를 게시했습니다
bootstrap-kind-published = kind 노드를 게시했습니다
bootstrap-kind-registry-extends-failed = 레지스트리용 kind 확장 체인 확인에 실패했습니다
bootstrap-kind-registry-fetch-log-failed = 레지스트리용 kind 노드 가져오기에 실패했습니다
bootstrap-kind-registry-hydrated = 매니페스트에서 kinds 레지스트리를 채웠습니다
bootstrap-kinds-overlay-pin-update-failed = kinds 오버레이 후 핀/업데이트에 실패했습니다
bootstrap-kinds-overlay-published = kinds 오버레이 후 런타임 매니페스트를 게시했습니다
bootstrap-kinds-tree-published = 런타임 kinds 트리를 게시했습니다
bootstrap-lifecycle-manifest-pin-update-failed = 수명 주기 지속 후 핀/업데이트에 실패했습니다
bootstrap-lifecycle-manifest-publish-failed = 수명 주기 전환 후 매니페스트 게시에 실패했습니다
bootstrap-lifecycle-manifest-published = 수명 주기 전환 후 업데이트된 매니페스트를 게시했습니다
bootstrap-manifest-fetch-failed = 런타임 매니페스트 가져오기에 실패했습니다
bootstrap-minimal-manifest-failed = 최소 매니페스트 초기화에 실패했습니다
bootstrap-remote-root-pin-confirmed = 원격 루트 핀이 확인되었습니다
bootstrap-remote-root-pin-misconfigured = 원격 루트 핀이 잘못 구성되어 있습니다
bootstrap-root-acl-published = 루트 전송 ACL을 게시했습니다
bootstrap-root-cid-shutdown-persist-failed = 종료 시 root_cid 지속에 실패했습니다
bootstrap-root-cid-shutdown-publish-failed = 종료 시 runtime_ipns 게시에 실패했습니다
bootstrap-root-cid-shutdown-publish-succeeded = 종료 시 runtime_ipns 게시에 성공했습니다
bootstrap-root-cid-shutdown-publish-timeout = 종료 시 runtime_ipns 게시가 시간 초과되었습니다
bootstrap-root-pin-replacement-failed = 원격 루트 핀 교체 실패 후 계속합니다
bootstrap-root-pin-update-failed = 부트스트랩 후 핀/업데이트에 실패했습니다
bootstrap-runtime-manifest-published = 런타임 루트 매니페스트를 게시했습니다
crud-message-rejected = CRUD 메시지가 거부되었습니다
entity-reload-current-node-load-failed = 다시 로드 전에 현재 엔티티 노드를 로드하는 데 실패했습니다. 현재 플러그인을 유지합니다
entity-reload-failed = 엔티티 다시 로드에 실패했습니다. 다음 다시 로드까지 비활성화합니다
entity-reload-kind-extends-failed = 엔티티 다시 로드 중 kind 확장 체인 확인에 실패했습니다
entity-reload-kind-fetch-failed = 엔티티 다시 로드 중 kind 노드 가져오기에 실패했습니다
entity-reload-kind-lookup-failed = 엔티티 다시 로드 중 kind 조회를 위한 매니페스트 로드에 실패했습니다
entity-reload-kind-missing = 매니페스트에서 kind를 찾을 수 없습니다. 엔티티를 다시 로드할 수 없습니다
entity-reload-manifest-state-update-failed = 다시 로드 전에 현재 상태로 매니페스트를 업데이트하는 데 실패했습니다. 현재 플러그인을 유지합니다
entity-reload-skipped = 다시 로드 게이트가 닫혀 있어 엔티티 다시 로드를 건너뜁니다
entity-reload-started = 엔티티 다시 로드가 시작되었습니다
entity-reload-state-persist-failed = 다시 로드 전에 현재 상태를 지속하는 데 실패했습니다. 현재 플러그인을 유지합니다
entity-reload-state-produced-failed = 다시 로드 중 생성된 상태를 지속하는 데 실패했습니다
entity-reloaded-manifest-update-failed = 다시 로드된 엔티티의 매니페스트 업데이트에 실패했습니다
entity-reloaded-manifest-updated = 다시 로드된 엔티티를 매니페스트에서 업데이트했습니다
inbox-message-rejected = 받은 편지함 메시지가 거부되었습니다
ma-create-entity-already-exists = ma_create_entity: 엔티티가 이미 존재합니다. 현재 엔티티를 유지합니다
ma-create-entity-invalid-behaviour = ma_create_entity: 잘못된 behaviour 참조. 건너뜁니다
ma-create-entity-kind-missing = ma_create_entity: 레지스트리에 kind가 없습니다. 건너뜁니다
manifest-pin-update-failed = 매니페스트 pin_update가 실패했습니다
plugin-envelope-build-failed = 플러그인 엔벨로프: 메시지 구성에 실패했습니다. 건너뜁니다
plugin-envelope-create-requests-ignored = 플러그인 엔벨로프: 사이드 이펙트 컨텍스트 없이 생성 요청이 무시됩니다
plugin-envelope-local-dispatch-failed = 플러그인 엔벨로프: 로컬 디스패치에 실패했습니다
plugin-envelope-local-dispatch-finish = 플러그인 엔벨로프: 로컬 디스패치가 완료되었습니다
plugin-envelope-local-dispatch-start = 플러그인 엔벨로프: 로컬 디스패치가 시작되었습니다
plugin-envelope-local-gate-closed = 플러그인 엔벨로프: 로컬 디스패치 게이트가 닫혀 있습니다
plugin-envelope-local-recipient-unknown = 플러그인 엔벨로프: 알 수 없는 로컬 수신자. 건너뜁니다
plugin-envelope-local-timeout = 플러그인 엔벨로프: 로컬 디스패치가 시간 초과되었습니다
plugin-envelope-recipient-invalid = 플러그인 엔벨로프: 잘못된 수신자 DID. 건너뜁니다
plugin-envelope-remote-limit = 플러그인 엔벨로프: 원격 전달 한도에 도달했습니다. 엔벨로프를 삭제합니다
plugin-outbox-congested = 플러그인 아웃박스가 혼잡합니다. 채널이 가득 차면 엔벨로프가 삭제될 수 있습니다
plugin-outbox-drain-limit = 플러그인 아웃박스 드레인 예산이 소진되었습니다. 남은 엔벨로프를 지연합니다
schedule-dispatch-firing = 예약된 디스패치 실행 중
schedule-entity-not-found = 예약된 디스패치: 엔티티를 찾을 수 없습니다
schedule-random-chain-stopped = 랜덤 일정 체인이 중지되었습니다. 더 새로운 정의로 대체되었습니다
schedule-random-create-failed = 다음 랜덤 작업 생성에 실패했습니다
schedule-random-reschedule-failed = 랜덤 작업 재예약에 실패했습니다
schedule-stale-dispatch-skipped = 예약된 디스패치를 건너뜁니다. 오래된 일정
scheduled-dispatch-error = 예약된 디스패치 오류
scheduled-dispatch-manifest-writer-unavailable = 예약된 디스패치: 매니페스트 쓰기 도구가 준비되지 않았습니다. 엔티티 상태가 보류 중입니다
