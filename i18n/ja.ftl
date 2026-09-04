# ma-runtime – 日本語
lang-name = 日本語

own-did-published = 自身の DID ドキュメントを IPNS に公開しました
own-did-publish-failed = 自身の DID ドキュメントの公開に失敗しました
own-did-publish-timeout = 自身の DID ドキュメントの公開が 2 分後にタイムアウトしました
started = ma runtime が起動しました
shutdown-requested = シャットダウンが要求されました
closing-endpoint = iroh エンドポイントを閉じています...
shutdown-complete = シャットダウンが完了しました
status-listening = ステータスサーバーがリッスン中です
ipfs-message-rejected = IPFS メッセージが拒否されました
ctrlc-handler-failed = Ctrl-C ハンドラーが失敗しました
node-connected = ノードがプロトコルに接続しました
received-encrypted-ma-msg = /ma/ipfs/0.0.1 で暗号化された ma メッセージを受信しました
ping-received = :ping を受信しました、:pong を送信します
did-publish-request-received = DID ドキュメントの公開リクエストを受信しました
document-published = ドキュメントを公開しました
did-publish-cid-reply-sent = DID 公開の CID 返信を送信しました
did-publish-resolve-failed = ipfs-publish 返信の配信のために送信者を解決できませんでした
ipfs-store-request-received = IPFS ストレージリクエストを受信しました
ipfs-stored = コンテンツを IPFS に保存しました
ipfs-store-cid-reply-sent = CID 返信を送信しました
ipfs-store-resolve-failed = ipfs-store 返信の配信のために送信者を解決できませんでした

# エンティティのディスパッチ
bootstrap-complete = Bootstrap が完了しました
entity-loaded = エンティティプラグインを読み込みました
entity-load-failed = エンティティプラグインの読み込みに失敗しました
root-list-entities = #root: エンティティ一覧
entity-created = エンティティが作成されました
entity-reloaded = エンティティプラグインを再読み込みしました
entity-deleted = エンティティが削除されました
entity-states-saving = エンティティの状態を IPFS に保存しています
entity-state-saving = エンティティの状態を保存しています
entity-state-saved = エンティティの状態が保存されました
entity-state-empty = プラグインが空の状態を返しました、保存をスキップします
entity-states-saved = エンティティの状態が保存されました

# 初回起動 / 自動初期化

# 所有権
runtime-claimed = Runtime が登録されました。

# 保護されたルート要素
refuse-delete-root = 必要なルート要素の削除を断固として拒否します
runtime-claim-persisted = 所有者が設定に書き込まれました。


# Namespace creation (:create)
crud-message-received = CRUDメッセージを受信
crud-acl-updated = ルートトランスポートACLを更新

# CRUD validation errors
cidv1-required = 値はベアCIDv1でなければなりません（'b'で始まる; CIDv0 'Qm…'は受け付けません）
config-key-protected = configキー '%key%' は保護されています
config-key-no-delete = daemonのconfigキー '%key%' は削除できません
config-key-not-manifest = configキー '%key%' は既知のmanifest configキーではありません
wrong-crud-protocol = 不正なCRUDプロトコル: %type%
entity-name-invalid = entity名は印刷可能なUTF-8でなければなりません
reserved-entity-name = entity名 '%name%' は予約済みです
genesis-kind-owner-only = genesis 種別の entity を作成できるのは runtime の所有者のみです

# IPv6 config
ipv6-enabled = IPv6 有効 — IPv4 と IPv6 の両方にバインド中
ipv6-disabled = IPv6 が無効になりました — IPv4 のみをバインドしています（再有効化には restart が必要です）
ipv6-enable-restart-required = 保存しました。この変更を反映するには restart が必要です。
ipv6-enable-unchanged = ipv6_enable はすでにその値に設定されています — 変更はありません。

boot-default-root-config-populate-failed = デフォルト設定ルートの入力に失敗しました
boot-default-root-config-populated = デフォルト設定ルートを入力しました
boot-entity-load-processed = エンティティプラグインを読み込みました
boot-group-load-failed = 起動時にグループの読み込みに失敗しました
boot-group-loaded-into-cache = グループをキャッシュに読み込みました
boot-kinds-overlay-applied = kindsオーバーレイを適用しました
boot-kinds-overlay-no-change = kindsオーバーレイはマニフェストに変更を加えませんでした
boot-load-manifest-for-acl-cache-failed = ACLキャッシュ入力用マニフェストの読み込みに失敗しました
boot-minimal-manifest-bootstrapped = 最小マニフェストを初期化しました
boot-minimal-manifest-not-found = IPNSでランタイムルートCIDが見つかりません。最小マニフェストを初期化します
boot-no-root-entity = デフォルト設定ルート用のルートエンティティが登録されていません
boot-reconciled-owners-manifest-failed = 起動時にオーナーをマニフェストに調整できませんでした
boot-reconciled-owners-persist-failed = 調整済みオーナーをconfig.yamlに保存できませんでした
boot-reconciled-owners-published = 起動時にconfig.yaml/--ownerからオーナーをマニフェストに調整しました
boot-root-acl-load-cache-failed = 起動時にルートACLの読み込みに失敗しました
boot-root-acl-load-failed = マニフェストからルートACLの読み込みに失敗しました
boot-root-acl-loaded-from-manifest = マニフェストからルートトランスポートACLを読み込みました
boot-root-acl-loaded-into-cache = ルートACLをキャッシュに読み込みました
bootstrap-acl-published = ACLノードを公開しました
bootstrap-endpoint-close-stuck = 10秒後もエンドポイントが実行中タスクに保持されています。強制終了します
bootstrap-endpoint-close-timeout = 5秒後にエンドポイントのクローズがタイムアウトしました。強制終了します
bootstrap-entity-lifecycle-update-failed = 更新されたエンティティのライフサイクルをIPFSに書き込めませんでした
bootstrap-entity-lifecycle-updated = エンティティのライフサイクルをIPFSで更新しました
bootstrap-entity-node-shutdown-updated = シャットダウン時にエンティティノードを更新しました
bootstrap-entity-published = エンティティノードを公開しました
bootstrap-entity-registering-prepublished = 事前公開済みエンティティを登録中
bootstrap-entity-registry-fetch-failed = エンティティノードの取得に失敗しました
bootstrap-entity-registry-kind-extends-failed = kind拡張チェーンの解決に失敗しました
bootstrap-entity-registry-kind-fetch-failed = kindノードの取得に失敗しました
bootstrap-entity-registry-kind-missing = マニフェストにkindが見つかりません。エンティティをスキップします
bootstrap-entity-registry-not-in-manifest = エンティティはレジストリに存在しますがマニフェストには存在しません。スキップします
bootstrap-entity-state-save-failed = エンティティの状態の保存に失敗しました
bootstrap-entity-state-shutdown-aborted = シャットダウンを中止しました。次のシャットダウン試行で状態を保存できるようにランタイムはアクティブのままです
bootstrap-entity-state-update-fetch-failed = 状態更新用エンティティノードの取得に失敗しました
bootstrap-envelope-delivery-failed = プラグインエンベロープの配信に失敗しました。エンベロープを破棄します
bootstrap-envelope-open-failed = プラグインエンベロープ: アウトボックスを開けませんでした。エンベロープを破棄します
bootstrap-group-published = グループノードを公開しました
bootstrap-kind-published = kindノードを公開しました
bootstrap-kind-registry-extends-failed = レジストリ用kind拡張チェーンの解決に失敗しました
bootstrap-kind-registry-fetch-log-failed = レジストリ用kindノードの取得に失敗しました
bootstrap-kind-registry-hydrated = マニフェストからkindsレジストリを読み込みました
bootstrap-kinds-overlay-pin-update-failed = kindsオーバーレイ後にピン/更新が失敗しました
bootstrap-kinds-overlay-published = kindsオーバーレイ後にランタイムマニフェストを公開しました
bootstrap-kinds-tree-published = ランタイムkindsツリーを公開しました
bootstrap-lifecycle-manifest-pin-update-failed = ライフサイクル永続化後にピン/更新が失敗しました
bootstrap-lifecycle-manifest-publish-failed = ライフサイクル遷移後にマニフェストの公開に失敗しました
bootstrap-lifecycle-manifest-published = ライフサイクル遷移後に更新済みマニフェストを公開しました
bootstrap-manifest-fetch-failed = ランタイムマニフェストの取得に失敗しました
bootstrap-minimal-manifest-failed = 最小マニフェストの初期化に失敗しました
bootstrap-remote-root-pin-confirmed = リモートルートピンを確認しました
bootstrap-remote-root-pin-misconfigured = リモートルートピンニングの設定が正しくありません
bootstrap-root-acl-published = ルートトランスポートACLを公開しました
bootstrap-root-cid-shutdown-persist-failed = シャットダウン中にroot_cidを永続化できませんでした
bootstrap-root-cid-shutdown-publish-failed = シャットダウン中のruntime_ipns公開に失敗しました
bootstrap-root-cid-shutdown-publish-succeeded = シャットダウン中のruntime_ipns公開に成功しました
bootstrap-root-cid-shutdown-publish-timeout = シャットダウン中のruntime_ipns公開がタイムアウトしました
bootstrap-root-pin-replacement-failed = リモートルートピン置換エラー後も続行します
bootstrap-root-pin-update-failed = ブートストラップ後にピン/更新が失敗しました
bootstrap-runtime-manifest-published = ランタイムルートマニフェストを公開しました
crud-message-rejected = CRUDメッセージを拒否しました
entity-reload-current-node-load-failed = リロード前に現在のエンティティノードを読み込めませんでした。現在のプラグインを維持します
entity-reload-failed = エンティティのリロードに失敗しました。次のリロードまで無効化します
entity-reload-kind-extends-failed = エンティティリロード中にkind拡張チェーンの解決に失敗しました
entity-reload-kind-fetch-failed = エンティティリロード中にkindノードの取得に失敗しました
entity-reload-kind-lookup-failed = エンティティリロード中にkind検索用マニフェストの読み込みに失敗しました
entity-reload-kind-missing = マニフェストにkindが見つかりません。エンティティをリロードできません
entity-reload-manifest-state-update-failed = リロード前に現在の状態でマニフェストを更新できませんでした。現在のプラグインを維持します
entity-reload-skipped = リロードゲートが閉じているためエンティティのリロードをスキップしました
entity-reload-started = エンティティのリロードを開始しました
entity-reload-state-persist-failed = リロード前に現在の状態を永続化できませんでした。現在のプラグインを維持します
entity-reload-state-produced-failed = リロード中に生成された状態を永続化できませんでした
entity-reloaded-manifest-update-failed = リロード済みエンティティのマニフェスト更新に失敗しました
entity-reloaded-manifest-updated = リロード済みエンティティをマニフェストで更新しました
inbox-message-rejected = 受信ボックスのメッセージを拒否しました
ma-create-entity-already-exists = ma_create_entity: エンティティは既に存在します。現在のエンティティを維持します
ma-create-entity-invalid-behaviour = ma_create_entity: 無効なbehaviour参照。スキップします
ma-create-entity-kind-missing = ma_create_entity: レジストリにkindがありません。スキップします
manifest-pin-update-failed = マニフェストpin_updateが失敗しました
plugin-envelope-build-failed = プラグインエンベロープ: メッセージの構築に失敗しました。スキップします
plugin-envelope-create-requests-ignored = プラグインエンベロープ: 副作用コンテキストなしで作成リクエストを無視します
plugin-envelope-local-dispatch-failed = プラグインエンベロープ: ローカルディスパッチに失敗しました
plugin-envelope-local-dispatch-finish = プラグインエンベロープ: ローカルディスパッチが完了しました
plugin-envelope-local-dispatch-start = プラグインエンベロープ: ローカルディスパッチを開始しました
plugin-envelope-local-gate-closed = プラグインエンベロープ: ローカルディスパッチゲートが閉じています
plugin-envelope-local-recipient-unknown = プラグインエンベロープ: 不明なローカル受信者。スキップします
plugin-envelope-local-timeout = プラグインエンベロープ: ローカルディスパッチがタイムアウトしました
plugin-envelope-recipient-invalid = プラグインエンベロープ: 無効な受信者DID。スキップします
plugin-envelope-remote-limit = プラグインエンベロープ: リモート配信制限に達しました。エンベロープを破棄します
plugin-outbox-congested = プラグインアウトボックスが輻輳しています。チャンネルがいっぱいになるとエンベロープが破棄される可能性があります
plugin-outbox-drain-limit = プラグインアウトボックスのドレインバジェットを使い切りました。残りのエンベロープを延期します
schedule-dispatch-firing = スケジュールされたディスパッチを実行中
schedule-entity-not-found = スケジュールされたディスパッチ: エンティティが見つかりません
schedule-random-chain-stopped = ランダムスケジュールチェーンが停止しました: より新しい定義に置き換えられました
schedule-random-create-failed = 次のランダムジョブの作成に失敗しました
schedule-random-reschedule-failed = ランダムジョブの再スケジュールに失敗しました
schedule-stale-dispatch-skipped = スケジュールされたディスパッチをスキップしました: 古いスケジュール
scheduled-dispatch-error = スケジュールされたディスパッチでエラーが発生しました
scheduled-dispatch-manifest-writer-unavailable = スケジュールされたディスパッチ: マニフェストライターの準備ができていません。エンティティの状態は保留中です
