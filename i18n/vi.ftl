# ma-runtime – Tiếng Việt
lang-name = Tiếng Việt

own-did-published = Tài liệu DID của mình đã được công bố lên IPNS
own-did-publish-failed = Không thể công bố tài liệu DID của mình
own-did-publish-timeout = Việc công bố tài liệu DID của mình đã hết thời gian sau 2 phút
started = ma runtime đã khởi động
shutdown-requested = Yêu cầu tắt máy đã được gửi
closing-endpoint = Đang đóng iroh endpoint...
shutdown-complete = Tắt máy hoàn tất
status-listening = Máy chủ trạng thái đang lắng nghe
ipfs-message-rejected = Tin nhắn IPFS đã bị từ chối
ctrlc-handler-failed = Trình xử lý Ctrl-C thất bại
node-connected = Nút đã kết nối với giao thức
received-encrypted-ma-msg = Đã nhận tin nhắn ma được mã hóa tại /ma/ipfs/0.0.1
ping-received = Đã nhận :ping, đang gửi :pong
did-publish-request-received = Đã nhận yêu cầu công bố tài liệu DID
document-published = Tài liệu đã được công bố
did-publish-cid-reply-sent = Đã gửi phản hồi CID cho việc công bố DID
did-publish-resolve-failed = Không thể xác định người gửi để gửi phản hồi ipfs-publish
ipfs-store-request-received = Đã nhận yêu cầu lưu trữ IPFS
ipfs-stored = Nội dung đã được lưu trữ trên IPFS
ipfs-store-cid-reply-sent = Đã gửi phản hồi CID
ipfs-store-resolve-failed = Không thể xác định người gửi để gửi phản hồi ipfs-store

# Điều phối thực thể
bootstrap-complete = Bootstrap hoàn tất
entity-loaded = Plugin thực thể đã được tải
entity-load-failed = Không thể tải plugin thực thể
root-list-entities = #root: danh sách thực thể
entity-created = Thực thể đã được tạo
entity-reloaded = Plugin thực thể đã được tải lại
entity-deleted = Thực thể đã bị xóa
entity-states-saving = Đang lưu trạng thái thực thể vào IPFS
entity-state-saving = Đang lưu trạng thái thực thể
entity-state-saved = Đã lưu trạng thái thực thể
entity-state-empty = Plugin trả về trạng thái rỗng, bỏ qua việc lưu
entity-states-saved = Đã lưu các trạng thái thực thể

# Khởi động lần đầu / tự động khởi tạo

# Quyền sở hữu
runtime-claimed = Runtime đã được đăng ký.

# Các phần tử root được bảo vệ
refuse-delete-root = Kiên quyết từ chối xóa phần tử root cần thiết
runtime-claim-persisted = Chủ sở hữu đã được ghi vào cấu hình.


# Namespace creation (:create)
crud-message-received = Nhận được tin nhắn CRUD
crud-acl-updated = Đã cập nhật ACL vận chuyển gốc

# CRUD validation errors
cidv1-required = giá trị phải là CIDv1 thuần túy (bắt đầu bằng 'b'; CIDv0 'Qm…' không được chấp nhận)
config-key-protected = khóa config '%key%' được bảo vệ
config-key-no-delete = khóa config '%key%' của daemon không thể xóa
config-key-not-manifest = khóa config '%key%' không phải là khóa manifest config đã biết
wrong-crud-protocol = giao thức CRUD sai: %type%
entity-name-invalid = tên entity phải là UTF-8 có thể in được
reserved-entity-name = tên entity '%name%' đã được đặt trước
genesis-kind-owner-only = Chỉ chủ sở hữu runtime mới có thể tạo entity loại genesis

# IPv6 config
ipv6-enabled = IPv6 đã bật — đang liên kết cả IPv4 và IPv6
ipv6-disabled = IPv6 bị tắt — chỉ đang liên kết IPv4 (cần restart để bật lại)
ipv6-enable-restart-required = Đã lưu. Cần restart để thay đổi này có hiệu lực.
ipv6-enable-unchanged = ipv6_enable đã được đặt thành giá trị đó — không có thay đổi.

boot-default-root-config-populate-failed = Không thể điền cấu hình root mặc định
boot-default-root-config-populated = Đã điền cấu hình root mặc định
boot-entity-load-processed = Đã tải các plugin entity
boot-group-load-failed = Không thể tải nhóm trong quá trình khởi động
boot-group-loaded-into-cache = Đã tải nhóm vào bộ nhớ đệm
boot-kinds-overlay-applied = Đã áp dụng Kinds overlay
boot-kinds-overlay-no-change = Kinds overlay không thay đổi manifest
boot-load-manifest-for-acl-cache-failed = Không thể tải manifest để điền ACL cache
boot-minimal-manifest-bootstrapped = Đã khởi động manifest tối thiểu
boot-minimal-manifest-not-found = Không tìm thấy runtime root CID trong IPNS; đang khởi động manifest tối thiểu
boot-no-root-entity = Không có root entity nào được đăng ký cho cấu hình root mặc định
boot-reconciled-owners-manifest-failed = Không thể đồng bộ hóa owners trong manifest khi khởi động
boot-reconciled-owners-persist-failed = Không thể lưu owners đã đồng bộ vào config.yaml
boot-reconciled-owners-published = Đã đồng bộ owners từ config.yaml/--owner vào manifest khi khởi động
boot-root-acl-load-cache-failed = Không thể tải root ACL khi khởi động
boot-root-acl-load-failed = Không thể tải root ACL từ manifest
boot-root-acl-loaded-from-manifest = Đã tải root transport ACL từ manifest
boot-root-acl-loaded-into-cache = Đã tải root ACL vào bộ nhớ đệm
bootstrap-acl-published = Đã xuất bản ACL node
bootstrap-endpoint-close-stuck = Endpoint bị chặn bởi các tác vụ đang chạy sau 10 giây; đang buộc đóng
bootstrap-endpoint-close-timeout = Đóng endpoint hết thời gian sau 5 giây; đang buộc kết thúc
bootstrap-entity-lifecycle-update-failed = Không thể ghi lifecycle đã cập nhật của entity vào IPFS
bootstrap-entity-lifecycle-updated = Đã cập nhật lifecycle entity trong IPFS
bootstrap-entity-node-shutdown-updated = Đã cập nhật entity node khi tắt hệ thống
bootstrap-entity-published = Đã xuất bản entity node
bootstrap-entity-registering-prepublished = Đang đăng ký entity đã xuất bản trước
bootstrap-entity-registry-fetch-failed = Không thể lấy entity node
bootstrap-entity-registry-kind-extends-failed = Không thể giải quyết chuỗi mở rộng Kind
bootstrap-entity-registry-kind-fetch-failed = Không thể lấy Kind node
bootstrap-entity-registry-kind-missing = Không tìm thấy Kind trong manifest; bỏ qua entity
bootstrap-entity-registry-not-in-manifest = Entity có trong registry nhưng không có trong manifest; bỏ qua
bootstrap-entity-state-save-failed = Không thể lưu trạng thái entity
bootstrap-entity-state-shutdown-aborted = Tắt hệ thống bị hủy; runtime vẫn hoạt động để lưu trạng thái trong lần tắt tiếp theo
bootstrap-entity-state-update-fetch-failed = Không thể lấy entity node để cập nhật trạng thái
bootstrap-envelope-delivery-failed = Gửi plugin envelope thất bại; đã xóa envelope
bootstrap-envelope-open-failed = Plugin envelope: không thể mở outgoing mailbox; đã xóa envelope
bootstrap-group-published = Đã xuất bản group node
bootstrap-kind-published = Đã xuất bản Kind node
bootstrap-kind-registry-extends-failed = Không thể giải quyết chuỗi mở rộng Kind cho registry
bootstrap-kind-registry-fetch-log-failed = Không thể lấy Kind node cho registry
bootstrap-kind-registry-hydrated = Kinds registry đã được điền từ manifest
bootstrap-kinds-overlay-pin-update-failed = pin/update thất bại sau Kinds overlay
bootstrap-kinds-overlay-published = Đã xuất bản runtime manifest sau Kinds overlay
bootstrap-kinds-tree-published = Đã xuất bản runtime Kinds tree
bootstrap-lifecycle-manifest-pin-update-failed = pin/update thất bại sau khi duy trì lifecycle
bootstrap-lifecycle-manifest-publish-failed = Không thể xuất bản manifest sau khi chuyển đổi lifecycle
bootstrap-lifecycle-manifest-published = Đã xuất bản manifest đã cập nhật sau khi chuyển đổi lifecycle
bootstrap-manifest-fetch-failed = Không thể lấy runtime manifest
bootstrap-minimal-manifest-failed = Không thể khởi động manifest tối thiểu
bootstrap-remote-root-pin-confirmed = Đã xác nhận remote root pinning
bootstrap-remote-root-pin-misconfigured = Remote root pinning được cấu hình không đúng
bootstrap-root-acl-published = Đã xuất bản root transport ACL
bootstrap-root-cid-shutdown-persist-failed = Không thể lưu root_cid khi tắt hệ thống
bootstrap-root-cid-shutdown-publish-failed = Không thể xuất bản runtime_ipns khi tắt hệ thống
bootstrap-root-cid-shutdown-publish-succeeded = Xuất bản runtime_ipns thành công khi tắt hệ thống
bootstrap-root-cid-shutdown-publish-timeout = Xuất bản runtime_ipns hết thời gian khi tắt hệ thống
bootstrap-root-pin-replacement-failed = Tiếp tục sau khi thay thế remote root pin thất bại
bootstrap-root-pin-update-failed = pin/update thất bại sau bootstrap
bootstrap-runtime-manifest-published = Đã xuất bản runtime root manifest
crud-message-rejected = Từ chối tin nhắn CRUD
entity-reload-current-node-load-failed = Không thể lấy entity node hiện tại trước khi reload; giữ plugin hiện tại
entity-reload-failed = Reload entity thất bại; vô hiệu hóa đến lần reload tiếp theo
entity-reload-kind-extends-failed = Không thể giải quyết chuỗi mở rộng Kind trong quá trình reload entity
entity-reload-kind-fetch-failed = Không thể lấy Kind node trong quá trình reload entity
entity-reload-kind-lookup-failed = Không thể lấy manifest để tra cứu Kind trong quá trình reload entity
entity-reload-kind-missing = Không tìm thấy Kind trong manifest; không thể reload entity
entity-reload-manifest-state-update-failed = Không thể cập nhật manifest với trạng thái hiện tại trước khi reload; giữ plugin hiện tại
entity-reload-skipped = Bỏ qua reload entity vì reload gate đang đóng
entity-reload-started = Đã bắt đầu reload entity
entity-reload-state-persist-failed = Không thể lưu trạng thái hiện tại trước khi reload; giữ plugin hiện tại
entity-reload-state-produced-failed = Không thể lưu trạng thái được tạo ra trong quá trình reload
entity-reloaded-manifest-update-failed = Không thể cập nhật entity đã reload trong manifest
entity-reloaded-manifest-updated = Đã cập nhật entity đã reload trong manifest
inbox-message-rejected = Từ chối tin nhắn inbox
ma-create-entity-already-exists = ma_create_entity: entity đã tồn tại; giữ entity hiện tại
ma-create-entity-invalid-behaviour = ma_create_entity: tham chiếu behaviour không hợp lệ; bỏ qua
ma-create-entity-kind-missing = ma_create_entity: Kind không có trong registry; bỏ qua
manifest-pin-update-failed = Manifest pin_update thất bại
plugin-envelope-build-failed = Plugin envelope: không thể xây dựng tin nhắn; bỏ qua
plugin-envelope-create-requests-ignored = Plugin envelope: các yêu cầu tạo không có ngữ cảnh side-effect bị bỏ qua
plugin-envelope-local-dispatch-failed = Plugin envelope: local dispatch thất bại
plugin-envelope-local-dispatch-finish = Plugin envelope: local dispatch hoàn thành
plugin-envelope-local-dispatch-start = Plugin envelope: local dispatch bắt đầu
plugin-envelope-local-gate-closed = Plugin envelope: local dispatch gate đang đóng
plugin-envelope-local-recipient-unknown = Plugin envelope: người nhận local không xác định; bỏ qua
plugin-envelope-local-timeout = Plugin envelope: local dispatch hết thời gian
plugin-envelope-recipient-invalid = Plugin envelope: DID người nhận không hợp lệ; bỏ qua
plugin-envelope-remote-limit = Plugin envelope: đã đạt giới hạn gửi từ xa; đã bỏ envelope
plugin-outbox-congested = Plugin outbox bị tắc nghẽn; envelope có thể bị bỏ nếu channel đầy
plugin-outbox-drain-limit = Ngân sách xả plugin outbox đã cạn; các envelope còn lại bị trì hoãn
schedule-dispatch-firing = Scheduled dispatch đang chạy
schedule-entity-not-found = Scheduled dispatch: không tìm thấy entity
schedule-random-chain-stopped = Chuỗi schedule ngẫu nhiên đã dừng: bị thay thế bởi định nghĩa mới
schedule-random-create-failed = Không thể tạo tác vụ ngẫu nhiên tiếp theo
schedule-random-reschedule-failed = Không thể đặt lại lịch tác vụ ngẫu nhiên
schedule-stale-dispatch-skipped = Bỏ qua scheduled dispatch: schedule đã cũ
scheduled-dispatch-error = Lỗi trong scheduled dispatch
scheduled-dispatch-manifest-writer-unavailable = Scheduled dispatch: manifest writer chưa sẵn sàng; trạng thái entity đang chờ
