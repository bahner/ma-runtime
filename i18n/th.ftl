# ma-runtime – ภาษาไทย
lang-name = ภาษาไทย

own-did-published = เผยแพร่เอกสาร DID ของตนเองไปยัง IPNS แล้ว
own-did-publish-failed = ล้มเหลวในการเผยแพร่เอกสาร DID ของตนเอง
own-did-publish-timeout = การเผยแพร่เอกสาร DID ของตนเองหมดเวลาหลังจาก 2 นาที
started = ma runtime เริ่มทำงานแล้ว
shutdown-requested = ได้รับคำขอปิดระบบ
closing-endpoint = กำลังปิด iroh endpoint...
shutdown-complete = การปิดระบบเสร็จสมบูรณ์
status-listening = เซิร์ฟเวอร์สถานะกำลังรับฟัง
ipfs-message-rejected = ปฏิเสธข้อความ IPFS แล้ว
ctrlc-handler-failed = ตัวจัดการ Ctrl-C ล้มเหลว
node-connected = โหนดเชื่อมต่อกับโปรโตคอลแล้ว
received-encrypted-ma-msg = ได้รับข้อความ ma ที่เข้ารหัสแล้วที่ /ma/ipfs/0.0.1
ping-received = ได้รับ :ping กำลังส่ง :pong
did-publish-request-received = ได้รับคำขอเผยแพร่เอกสาร DID
document-published = เผยแพร่เอกสารแล้ว
did-publish-cid-reply-sent = ส่งการตอบกลับ CID สำหรับการเผยแพร่ DID แล้ว
did-publish-resolve-failed = ไม่สามารถแก้ไขผู้ส่งเพื่อส่งการตอบกลับ ipfs-publish
ipfs-store-request-received = ได้รับคำขอจัดเก็บ IPFS
ipfs-stored = จัดเก็บเนื้อหาไปยัง IPFS แล้ว
ipfs-store-cid-reply-sent = ส่งการตอบกลับ CID แล้ว
ipfs-store-resolve-failed = ไม่สามารถแก้ไขผู้ส่งเพื่อส่งการตอบกลับ ipfs-store

# การส่งเอนทิตี
bootstrap-complete = Bootstrap เสร็จสมบูรณ์
entity-loaded = โหลดปลั๊กอินเอนทิตีแล้ว
entity-load-failed = ล้มเหลวในการโหลดปลั๊กอินเอนทิตี
root-list-entities = #root: รายการเอนทิตี
entity-created = สร้างเอนทิตีแล้ว
entity-reloaded = โหลดปลั๊กอินเอนทิตีใหม่แล้ว
entity-deleted = ลบเอนทิตีแล้ว
entity-states-saving = กำลังบันทึกสถานะเอนทิตีไปยัง IPFS
entity-state-saving = กำลังบันทึกสถานะเอนทิตี
entity-state-saved = บันทึกสถานะเอนทิตีแล้ว
entity-state-empty = ปลั๊กอินส่งคืนสถานะว่าง ข้ามการบันทึก
entity-states-saved = บันทึกสถานะเอนทิตีแล้ว

# การเริ่มต้นครั้งแรก / การเริ่มต้นอัตโนมัติ

# ความเป็นเจ้าของ
runtime-claimed = ลงทะเบียน runtime แล้ว

# องค์ประกอบ root ที่ได้รับการปกป้อง
refuse-delete-root = ปฏิเสธอย่างเด็ดขาดที่จะลบองค์ประกอบ root ที่จำเป็น
runtime-claim-persisted = เขียนเจ้าของลงในการกำหนดค่าแล้ว


# Namespace creation (:create)
crud-message-received = ได้รับข้อความ CRUD
crud-acl-updated = อัปเดต ACL การขนส่งรูทแล้ว

# CRUD validation errors
cidv1-required = ค่าต้องเป็น CIDv1 บริสุทธิ์ (เริ่มด้วย 'b'; CIDv0 'Qm…' ไม่ยอมรับ)
config-key-protected = คีย์ config '%key%' ได้รับการป้องกัน
config-key-no-delete = ไม่สามารถลบคีย์ config '%key%' ของ daemon ได้
config-key-not-manifest = คีย์ config '%key%' ไม่ใช่คีย์ manifest config ที่รู้จัก
wrong-crud-protocol = CRUD protocol ผิดพลาด: %type%
entity-name-invalid = ชื่อ entity ต้องเป็น UTF-8 ที่พิมพ์ได้
reserved-entity-name = ชื่อ entity '%name%' ถูกสงวนไว้
genesis-kind-owner-only = มีเพียงเจ้าของ runtime เท่านั้นที่สามารถสร้าง entity ประเภท genesis ได้

# IPv6 config
ipv6-enabled = เปิดใช้งาน IPv6 แล้ว — กำลังผูกกับ IPv4 และ IPv6 ทั้งสอง
ipv6-disabled = IPv6 ถูกปิดใช้งาน — กำลังผูกเฉพาะ IPv4 (ต้องการ restart เพื่อเปิดใช้งานอีกครั้ง)
ipv6-enable-restart-required = บันทึกแล้ว ต้องการ restart เพื่อให้การเปลี่ยนแปลงนี้มีผล
ipv6-enable-unchanged = ipv6_enable ถูกตั้งค่าเป็นค่านั้นอยู่แล้ว — ไม่มีการเปลี่ยนแปลง

boot-default-root-config-populate-failed = ไม่สามารถเติมค่าการตั้งค่า root เริ่มต้นได้
boot-default-root-config-populated = เติมค่าการตั้งค่า root เริ่มต้นแล้ว
boot-entity-load-processed = โหลดปลั๊กอิน entity แล้ว
boot-group-load-failed = ไม่สามารถโหลด group ระหว่างการบูตได้
boot-group-loaded-into-cache = โหลด group ลงใน cache แล้ว
boot-kinds-overlay-applied = ใช้ Kinds overlay แล้ว
boot-kinds-overlay-no-change = Kinds overlay ไม่ได้เปลี่ยนแปลง manifest
boot-load-manifest-for-acl-cache-failed = ไม่สามารถโหลด manifest เพื่อเติม ACL cache ได้
boot-minimal-manifest-bootstrapped = Bootstrap manifest ขั้นต่ำแล้ว
boot-minimal-manifest-not-found = ไม่พบ runtime root CID ใน IPNS; กำลัง bootstrap manifest ขั้นต่ำ
boot-no-root-entity = ไม่มี root entity ที่ลงทะเบียนสำหรับการตั้งค่า root เริ่มต้น
boot-reconciled-owners-manifest-failed = ไม่สามารถประสาน owners ใน manifest ระหว่างการบูตได้
boot-reconciled-owners-persist-failed = ไม่สามารถบันทึก owners ที่ประสานแล้วลงใน config.yaml ได้
boot-reconciled-owners-published = ประสาน owners จาก config.yaml/--owner ลงใน manifest ระหว่างการบูตแล้ว
boot-root-acl-load-cache-failed = ไม่สามารถโหลด root ACL ระหว่างการบูตได้
boot-root-acl-load-failed = ไม่สามารถโหลด root ACL จาก manifest ได้
boot-root-acl-loaded-from-manifest = โหลด root transport ACL จาก manifest แล้ว
boot-root-acl-loaded-into-cache = โหลด root ACL ลงใน cache แล้ว
bootstrap-acl-published = เผยแพร่ ACL node แล้ว
bootstrap-endpoint-close-stuck = endpoint ถูกบล็อกโดยงานที่กำลังทำงานอยู่หลังจาก 10 วิ; บังคับปิด
bootstrap-endpoint-close-timeout = การปิด endpoint หมดเวลาหลังจาก 5 วิ; บังคับสิ้นสุด
bootstrap-entity-lifecycle-update-failed = ไม่สามารถเขียน lifecycle ที่อัปเดตของ entity ลงใน IPFS ได้
bootstrap-entity-lifecycle-updated = อัปเดต lifecycle ของ entity ใน IPFS แล้ว
bootstrap-entity-node-shutdown-updated = อัปเดต entity node ระหว่างการปิดระบบแล้ว
bootstrap-entity-published = เผยแพร่ entity node แล้ว
bootstrap-entity-registering-prepublished = กำลังลงทะเบียน entity ที่เผยแพร่ล่วงหน้าแล้ว
bootstrap-entity-registry-fetch-failed = ไม่สามารถดึง entity node ได้
bootstrap-entity-registry-kind-extends-failed = ไม่สามารถ resolve ห่วงโซ่การขยาย Kind ได้
bootstrap-entity-registry-kind-fetch-failed = ไม่สามารถดึง Kind node ได้
bootstrap-entity-registry-kind-missing = ไม่พบ Kind ใน manifest; ข้าม entity แล้ว
bootstrap-entity-registry-not-in-manifest = entity อยู่ใน registry แต่ไม่อยู่ใน manifest; ข้ามแล้ว
bootstrap-entity-state-save-failed = ไม่สามารถบันทึกสถานะ entity ได้
bootstrap-entity-state-shutdown-aborted = การปิดระบบถูกยกเลิก; runtime ยังคงทำงานอยู่เพื่อบันทึกสถานะในการปิดครั้งถัดไป
bootstrap-entity-state-update-fetch-failed = ไม่สามารถดึง entity node สำหรับการอัปเดตสถานะได้
bootstrap-envelope-delivery-failed = การส่ง plugin envelope ล้มเหลว; ลบ envelope แล้ว
bootstrap-envelope-open-failed = plugin envelope: ไม่สามารถเปิด outgoing mailbox ได้; ลบ envelope แล้ว
bootstrap-group-published = เผยแพร่ group node แล้ว
bootstrap-kind-published = เผยแพร่ Kind node แล้ว
bootstrap-kind-registry-extends-failed = ไม่สามารถ resolve ห่วงโซ่การขยาย Kind สำหรับ registry ได้
bootstrap-kind-registry-fetch-log-failed = ไม่สามารถดึง Kind node สำหรับ registry ได้
bootstrap-kind-registry-hydrated = Kinds registry ถูกเติมข้อมูลจาก manifest แล้ว
bootstrap-kinds-overlay-pin-update-failed = pin/update ล้มเหลวหลังจาก Kinds overlay
bootstrap-kinds-overlay-published = เผยแพร่ runtime manifest หลังจาก Kinds overlay แล้ว
bootstrap-kinds-tree-published = เผยแพร่ runtime Kinds tree แล้ว
bootstrap-lifecycle-manifest-pin-update-failed = pin/update ล้มเหลวหลังจากการคงอยู่ของ lifecycle
bootstrap-lifecycle-manifest-publish-failed = ไม่สามารถเผยแพร่ manifest หลังจากการเปลี่ยนแปลง lifecycle ได้
bootstrap-lifecycle-manifest-published = เผยแพร่ manifest ที่อัปเดตหลังจากการเปลี่ยนแปลง lifecycle แล้ว
bootstrap-manifest-fetch-failed = ไม่สามารถดึง runtime manifest ได้
bootstrap-minimal-manifest-failed = ไม่สามารถ bootstrap manifest ขั้นต่ำได้
bootstrap-remote-root-pin-confirmed = ยืนยัน remote root pinning แล้ว
bootstrap-remote-root-pin-misconfigured = remote root pinning ตั้งค่าไม่ถูกต้อง
bootstrap-root-acl-published = เผยแพร่ root transport ACL แล้ว
bootstrap-root-cid-shutdown-persist-failed = ไม่สามารถบันทึก root_cid ระหว่างการปิดระบบได้
bootstrap-root-cid-shutdown-publish-failed = ไม่สามารถเผยแพร่ runtime_ipns ระหว่างการปิดระบบได้
bootstrap-root-cid-shutdown-publish-succeeded = เผยแพร่ runtime_ipns ระหว่างการปิดระบบสำเร็จแล้ว
bootstrap-root-cid-shutdown-publish-timeout = การเผยแพร่ runtime_ipns ระหว่างการปิดระบบหมดเวลา
bootstrap-root-pin-replacement-failed = กำลังดำเนินการต่อหลังจากความล้มเหลวในการแทนที่ remote root pin
bootstrap-root-pin-update-failed = pin/update ล้มเหลวหลังจาก bootstrap
bootstrap-runtime-manifest-published = เผยแพร่ runtime root manifest แล้ว
crud-message-rejected = ปฏิเสธข้อความ CRUD
entity-reload-current-node-load-failed = ไม่สามารถดึง entity node ปัจจุบันก่อน reload ได้; รักษา plugin ปัจจุบันไว้
entity-reload-failed = การ reload entity ล้มเหลว; ปิดการใช้งานจนกว่าจะ reload ครั้งถัดไป
entity-reload-kind-extends-failed = ไม่สามารถ resolve ห่วงโซ่การขยาย Kind ระหว่าง entity reload ได้
entity-reload-kind-fetch-failed = ไม่สามารถดึง Kind node ระหว่าง entity reload ได้
entity-reload-kind-lookup-failed = ไม่สามารถดึง manifest สำหรับการค้นหา Kind ระหว่าง entity reload ได้
entity-reload-kind-missing = ไม่พบ Kind ใน manifest; ไม่สามารถ reload entity ได้
entity-reload-manifest-state-update-failed = ไม่สามารถอัปเดต manifest ด้วยสถานะปัจจุบันก่อน reload ได้; รักษา plugin ปัจจุบันไว้
entity-reload-skipped = ข้าม entity reload เนื่องจาก reload gate ปิดอยู่
entity-reload-started = เริ่มต้น entity reload แล้ว
entity-reload-state-persist-failed = ไม่สามารถบันทึกสถานะปัจจุบันก่อน reload ได้; รักษา plugin ปัจจุบันไว้
entity-reload-state-produced-failed = ไม่สามารถบันทึกสถานะที่ผลิตระหว่าง reload ได้
entity-reloaded-manifest-update-failed = ไม่สามารถอัปเดต entity ที่ reload แล้วใน manifest ได้
entity-reloaded-manifest-updated = อัปเดต entity ที่ reload แล้วใน manifest แล้ว
inbox-message-rejected = ปฏิเสธข้อความ inbox
ma-create-entity-already-exists = ma_create_entity: entity มีอยู่แล้ว; รักษา entity ปัจจุบันไว้
ma-create-entity-invalid-behaviour = ma_create_entity: การอ้างอิง behaviour ไม่ถูกต้อง; ข้ามแล้ว
ma-create-entity-kind-missing = ma_create_entity: ไม่มี Kind ใน registry; ข้ามแล้ว
manifest-pin-update-failed = manifest pin_update ล้มเหลว
plugin-envelope-build-failed = plugin envelope: ไม่สามารถสร้างข้อความได้; ข้ามแล้ว
plugin-envelope-create-requests-ignored = plugin envelope: ละเว้นคำขอสร้างโดยไม่มีบริบท side-effect
plugin-envelope-local-dispatch-failed = plugin envelope: local dispatch ล้มเหลว
plugin-envelope-local-dispatch-finish = plugin envelope: local dispatch เสร็จสิ้นแล้ว
plugin-envelope-local-dispatch-start = plugin envelope: local dispatch เริ่มต้นแล้ว
plugin-envelope-local-gate-closed = plugin envelope: local dispatch gate ปิดอยู่
plugin-envelope-local-recipient-unknown = plugin envelope: ผู้รับ local ที่ไม่รู้จัก; ข้ามแล้ว
plugin-envelope-local-timeout = plugin envelope: local dispatch หมดเวลา
plugin-envelope-recipient-invalid = plugin envelope: DID ผู้รับไม่ถูกต้อง; ข้ามแล้ว
plugin-envelope-remote-limit = plugin envelope: ถึงขีดจำกัดการส่งระยะไกล; ทิ้ง envelope แล้ว
plugin-outbox-congested = plugin outbox แออัด; envelopes อาจถูกทิ้งหาก channel เต็ม
plugin-outbox-drain-limit = งบประมาณการดึง plugin outbox หมดแล้ว; envelope ที่เหลือถูกเลื่อนออกไป
schedule-dispatch-firing = กำลังดำเนิน scheduled dispatch
schedule-entity-not-found = scheduled dispatch: ไม่พบ entity
schedule-random-chain-stopped = ห่วงโซ่ random schedule หยุดแล้ว: ถูกแทนที่ด้วยนิยามใหม่
schedule-random-create-failed = ไม่สามารถสร้างงาน random ถัดไปได้
schedule-random-reschedule-failed = ไม่สามารถ reschedule งาน random ได้
schedule-stale-dispatch-skipped = ข้าม scheduled dispatch: schedule ล้าสมัย
scheduled-dispatch-error = เกิดข้อผิดพลาดใน scheduled dispatch
scheduled-dispatch-manifest-writer-unavailable = scheduled dispatch: manifest writer ไม่พร้อม; สถานะ entity รอการประมวลผล
