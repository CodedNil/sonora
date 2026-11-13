# Sound and equalizer
equalizer-add-filter = フィルターを追加
equalizer-advanced = 詳細
equalizer-bass = 低音
equalizer-copied = プロファイルを Equalizer APO 形式でコピーしました
equalizer-decibels = dB
equalizer-delete = プロファイルを削除
equalizer-export = エクスポート
equalizer-export-file = ファイルにエクスポート…
equalizer-exported = プロファイルをエクスポートしました
equalizer-file-unreadable = ファイルを読み込めませんでした
equalizer-file-unwritable = ファイルを書き込めませんでした
equalizer-filter-limit = プロファイルに入れられるフィルターは { $max } 個までです。
equalizer-hertz = Hz
equalizer-import = インポート
equalizer-import-file = ファイルをインポート…
equalizer-import-nothing = インポートできるイコライザー設定が見つかりませんでした
equalizer-import-too-large = インポートできません: テキストが 1 MB を超えており、イコライザープロファイルには大きすぎます
equalizer-import-too-many = インポートできません: 有効なフィルターが{ $count ->
       *[other] { $count }個
    }あり、プロファイルの上限を超えています
equalizer-imported-skipped = { $name } をインポートしました（{ $count ->
       *[other] { $count }行
    }を除く）
equalizer-invalid-frequency = 周波数は { $low }～{ $high } Hz の範囲で指定してください
equalizer-invalid-gain = ゲインは { $low }～{ $high } dB の範囲で指定してください
equalizer-invalid-octaves = 帯域幅は { $low }～{ $high } オクターブの範囲で指定してください
equalizer-invalid-preamp = プリアンプは { $low }～{ $high } dB の範囲で指定してください
equalizer-invalid-q = Q は { $low }～{ $high } の範囲で指定してください
equalizer-invalid-shelf-octaves = シェルフには帯域幅ではなく Q を指定します
equalizer-kind-all-pass = オールパス
equalizer-kind-band-pass = バンドパス
equalizer-kind-high-pass = ハイパス
equalizer-kind-high-shelf = ハイシェルフ
equalizer-kind-low-pass = ローパス
equalizer-kind-low-shelf = ローシェルフ
equalizer-kind-notch = ノッチ
equalizer-kind-peak = ピーク
equalizer-level = レベル
equalizer-lower-preamp = プリアンプを下げる
equalizer-no-filters = フィルターはまだありません。追加するか、上のプリセットを選ぶか、プロファイルをインポートしてください。
equalizer-no-gain = —
equalizer-octaves = oct
equalizer-output-port = { $device } · { $port }
equalizer-overload = { $db } dB のクリッピング
equalizer-overload-slight = 0.1 dB 未満のクリッピング
equalizer-paste-empty = クリップボードにテキストがありません
equalizer-preamp = プリアンプ
equalizer-preamp-auto = 自動
equalizer-preamp-manual = 手動
equalizer-profile-default = デフォルト
equalizer-profile-name-hint = プロファイル名
equalizer-profile-new = プロファイル { $number }
equalizer-profile-pasted = 貼り付けたプロファイル
equalizer-q = Q
equalizer-redo = やり直す
equalizer-remove-filter = フィルターを削除
equalizer-rename = プロファイル名を変更
equalizer-rule-add = ルールを追加
equalizer-rule-choose-device = デバイスを選択
equalizer-rule-choose-sound = サウンドを選択
equalizer-rule-disconnected = 未接続
equalizer-rule-other = その他のデバイス
equalizer-rule-playing = 再生中
equalizer-rule-remove = ルールを削除
equalizer-rule-replaces = 既存のルールを置き換えます
equalizer-rules = デバイスごとのルール
equalizer-save-as = プロファイルとして保存
equalizer-simple = シンプル
equalizer-skip-expression = 式で書かれた値です
equalizer-skip-no-frequency = 周波数がありません
equalizer-skip-no-gain = ゲインがありません
equalizer-skip-no-preamp = プリアンプが数値ではありません
equalizer-skip-no-width = Q または帯域幅がありません
equalizer-skip-unknown-type = 不明なフィルタータイプ { $name }
equalizer-skip-unreadable = Sonora が読み込めるフィルターではありません
equalizer-skip-unsupported = { $name } はサポートされていません
equalizer-skipped-line = { $line } 行目: { $reason }
equalizer-skipped-more = ほか{ $count ->
       *[other] { $count }件
    }
equalizer-status-altered = ビットパーフェクトでない理由:
equalizer-status-bit-perfect = ビットパーフェクト
equalizer-status-bit-perfect-detail = Sonora からはビットパーフェクト
equalizer-status-bits = { $from } → { $to }ビット
equalizer-status-channels = { $from ->
    [1] モノラル
    [2] ステレオ
   *[other] { $from }ch
} → { $to ->
    [1] モノラル
    [2] ステレオ
   *[other] { $to }ch
}
equalizer-status-equalized = EQ
equalizer-status-float = 浮動小数点 → { $to }ビット
equalizer-status-normalized = 平準化
equalizer-status-rate = { $from } → { $to } kHz
equalizer-status-separator = {", "}
equalizer-status-server = その後サウンドサーバーが { $khz } kHz に変換します
equalizer-status-volume = 音量
equalizer-tone-reset = 0 dB にリセット
equalizer-transfer = インポートとエクスポート
equalizer-treble = 高音
equalizer-value-refused = 以前の値を維持しました
response-graph-db = { $value } dB
response-graph-hz = { $value } Hz
response-graph-khz = { $value } kHz
response-graph-readout = { $frequency } · { $direction ->
        [up] +{ $gain }
        [down] -{ $gain }
       *[flat] { $gain }
    } dB
