# Notifications and status
get-sonora = Sonora を入手
notice-never = 今後表示しない
offline-go-online = オンラインにする
offline-title = オフラインです。
requirement-audio-bridge = { $server ->
    [pulseaudio] PulseAudio は動作していますが、ALSA から接続できないため音が出ません。pulseaudio-alsa をインストールして Sonora を再起動してください。
   *[pipewire] PipeWire は動作していますが、ALSA から接続できないため音が出ません。pipewire-alsa をインストールして Sonora を再起動してください。
}
requirement-audio-bridge-title = Sonora がサウンドサーバーに接続できません
requirement-audio-output = オーディオデバイスを開けなかったため、音が出ません。デバイスが接続されていて、他のアプリに占有されていないか確認してください。
requirement-audio-output-title = 音声出力がありません
requirement-help = インストールが必要なもの
requirement-web-engine = YouTube Music、Deezer、Apple Music にブラウザウィンドウでサインインするには webkit2gtk-4.1 をインストールしてください。それまでは Cookie を貼り付けてサインインできます。
requirement-web-engine-title = ブラウザでのサインインは利用できません
toast-code-copied = コードをコピーしました
toast-download-cancelled = { $name } のダウンロードをキャンセルしました
toast-download-failed = ダウンロードする曲の一覧を取得できませんでした。
toast-downloads-cancelled = { $count ->
   *[other] { $count }曲のダウンロードをキャンセルしました
}
toast-favorite-albums-removed = お気に入りから{ $count ->
       *[other] { $count }枚のアルバム
    }を削除しました
toast-favorite-artists-removed = お気に入りから{ $count ->
       *[other] { $count }人のアーティスト
    }を削除しました
toast-favorite-removed = { $name } をお気に入りから削除しました
toast-favorite-songs-removed = お気に入りから{ $count ->
       *[other] { $count }曲
    }を削除しました
toast-folder-missing = { $name } はディスク上にありません
toast-folder-removed = { $name } を削除しました
toast-history-clear-failed = 履歴をクリアできませんでした
toast-history-remove-failed = 曲を履歴から削除できませんでした
toast-history-removed = { $count ->
   *[other] { $count }曲を履歴から削除しました
}
toast-history-restore-failed = 履歴を復元できませんでした
toast-jam-ended = Jam を終了しました
toast-jam-failed = Jam を開始または参加できませんでした。
toast-jam-leave-failed = Jam を終了または退出できませんでした。
toast-jam-left = Jam から退出しました
toast-jam-needs-connect = Jam に参加するには、設定で「Spotify デバイスとして表示」をオンにしてください
toast-jam-unreadable = このリンクから参加できる Jam はありません。
toast-keys-error = 再生キーのエラー
toast-keys-refused = Spotify が使えないかも
toast-last-album = アルバムを最後に再生します
toast-last-artist = アーティストを最後に再生します
toast-last-playlist = プレイリストを最後に再生します
toast-last-track = { $name } を最後に再生します
toast-last-tracks = { $count }曲を最後に再生します
toast-library-add-failed = { $name } をライブラリに追加できませんでした
toast-library-added = ライブラリに追加しました
toast-library-albums-removed = ライブラリから{ $count ->
       *[other] { $count }枚のアルバム
    }を削除しました
toast-library-pin-failed = Spotify のピン留めを更新できませんでした。
toast-library-remove-failed = { $name } をライブラリから削除できませんでした
toast-library-removed-one = { $name } をライブラリから削除しました
toast-library-scanned = { $count ->
   *[other] ライブラリをスキャンしました。{ $count }曲が見つかりました
}
toast-library-songs-removed = ライブラリから{ $count ->
       *[other] { $count }曲
    }を削除しました
toast-link-copied = リンクをコピーしました
toast-link-sign-in = このリンクを開くには { $name } にサインインしてください
toast-link-unreadable = このリンクは Sonora で開けません
toast-local-delete-failed = 一部の曲ファイルを削除できませんでした
toast-next-album = アルバムを次に再生します
toast-next-artist = アーティストを次に再生します
toast-next-playlist = プレイリストを次に再生します
toast-next-track = { $name } を次に再生します
toast-next-tracks = { $count }曲を次に再生します
toast-offline = 接続されていません。接続が戻るまでストリーミングできません。
toast-offline-mode = オフラインです。この曲をストリーミングするにはオンラインにしてください。
toast-offline-unsupported = { $name } のダウンロード済みの曲はオンライン時のみ再生できます。
toast-open-all-failed = { $count }個のファイルを開けませんでした
toast-open-failed = { $name } を開けませんでした
toast-open-some-failed = 一部のファイルを開けませんでした
toast-open-unsupported = { $name } は Sonora で再生できる音声ファイルではありません
toast-output-default-failed = システムのデフォルトデバイスを選択できません
toast-output-select-failed = { $name } を選択できません
toast-playback-stopped = 再生できない曲が続いたため、再生を停止しました
toast-playlist-added = プレイリストをライブラリに追加しました
toast-playlist-busy = 別の変更を処理中です
toast-playlist-cover-changed = プレイリストのカバーを変更しました
toast-playlist-cover-removed = プレイリストのカバーを削除しました
toast-playlist-created = プレイリストを作成しました
toast-playlist-deleted = プレイリストを削除しました
toast-playlist-exported = プレイリストをエクスポートしました
toast-playlist-failed = 変更を保存できませんでした
toast-playlist-import-empty = { $name } の曲はどれもライブラリにありません
toast-playlist-imported-partly = { $name } をインポートしました（ライブラリにない{ $count ->
       *[other] { $count }曲
    }を除く）
toast-playlist-removed = プレイリストをライブラリから削除しました
toast-playlist-renamed = プレイリストの名前を変更しました
toast-playlist-signed-out = プレイリストを変更するにはサインインしてください
toast-playlist-updated = プレイリストを更新しました
toast-playlist-visibility = プレイリストの公開設定を変更しました
toast-playlists-removed = ライブラリから{ $count ->
       *[other] { $count }件のプレイリスト
    }を削除しました
toast-queue-cleared = 再生キューをクリアしました
toast-queue-failed = 再生キューに追加できませんでした
toast-queue-local-elsewhere = ローカルファイルは他のデバイスの再生キューに追加できません
toast-queue-removed-track = { $name } を再生キューから削除しました
toast-queue-removed-tracks = { $count }曲を再生キューから削除しました
toast-queued-album = アルバムを再生キューに追加しました
toast-queued-artist = アーティストを再生キューに追加しました
toast-queued-playlist = プレイリストを再生キューに追加しました
toast-queued-track = { $name } を再生キューに追加しました
toast-queued-tracks = { $count }曲を再生キューに追加しました
toast-radio-failed = { $name } のラジオを開始できませんでした
toast-restore-failed = { $name } に再サインインできなかったため、Sonora はそれなしで続行しました
toast-sample-burst = トースト { $name } 番
toast-sample-delete = { $name } を削除しました
toast-sample-done = ライブラリに保存しました
toast-sample-event = イベントを作成しました
toast-sample-event-detail = 12月3日（日）午前9:00
toast-sample-failed = サーバーに接続できませんでした
toast-sample-info = イベントの10分前に到着してください。
toast-sample-lasting = これは閉じるまで表示されたままです
toast-sample-loading = 曲をアップロードしています
toast-sample-long = このトーストはほかよりずっと長い文章なので、閉じるボタンをカードの外に押し出さずに、2行目、場合によっては3行目まで折り返す必要があります
toast-sample-plain = イベントを作成しました
toast-sample-save-failed = { $name } を保存できませんでした
toast-sample-save-failed-detail = サーバーの応答: 503 Service Unavailable
toast-sample-saved = { $name } を保存しました
toast-sample-saving = { $name } を保存中
toast-sample-undone = 元に戻しました
toast-sample-warning = セッションは5分後に終了します
toast-scan-failed = 音楽フォルダをスキャンできませんでした
toast-settings-broken = 変更を保存するには settings.json の { $name } 行目を修正してください
toast-settings-overwritten = settings.json がディスク上で変更されたため、直前の変更は保存されませんでした
toast-settings-save-failed = 設定を保存できませんでした
toast-sign-in-to-play = { $name } のストリーミングにはサインインが必要です
toast-similar-failed = { $name } に似た曲が見つかりませんでした
toast-sleep-ended = スリープタイマーが終了しました
toast-still-throttled = { $name } はまだ再生を制限しています。再生を押してもう一度お試しください
toast-tags-failed = タグを保存できませんでした
toast-tags-saved = { $name } のタグを保存しました
toast-throttled = { $name } が一時的に再生を制限しています。まもなく再試行します
toast-track-added = { $name } に追加しました
toast-track-removed = { $name } から削除しました
toast-track-removed-unnamed = プレイリストから削除しました
toast-track-unplayable = { $name } を再生できませんでした
toast-tracks-removed = { $name } から{ $count ->
       *[other] { $count }曲
    }を削除しました
toast-tracks-removed-unnamed = プレイリストから{ $count ->
       *[other] { $count }曲
    }を削除しました
toast-tray-unavailable = アイコンを置けるシステムトレイがありません
toast-unpinned = { $name } のピン留めを解除しました
toast-why = なぜ？
toast-widevine-borrowed = Widevine を更新してみてください
toast-widevine-download-failed = Widevine をダウンロードできませんでした
toast-widevine-failed = 再生に失敗しました
toast-widevine-install-failed = Widevine をインストールできませんでした
toast-widevine-remove-failed = Widevine を削除できませんでした
trouble-not-loaded = 読み込めませんでした
trouble-offline = 接続されていません
trouble-offline-detail = インターネット接続を確認してもう一度お試しください。
trouble-retry = 再試行
update-available = Sonora { $version } が公開されました
update-detail = 現在のバージョンは { $running } です。変更点を読むか、今すぐ更新できます。
update-detail-notes = 現在のバージョンは { $running } です。変更点を読んでから、インストールした方法で Sonora を更新してください。
update-failed = 更新をインストールできませんでした。リリースページからもう一度お試しください。
update-never = 更新を確認しない
update-notes = 変更点
update-now = 更新
update-working = 更新をダウンロードしています…
wake-reason = 音楽を再生中
