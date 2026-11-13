# Tables, filters and counts
column-added-by = 追加者
column-album = アルバム
column-artist = アーティスト
column-date-added = 追加日
column-genre = ジャンル
column-index = #
column-length = 長さ
column-modified = 更新日
column-name = 名前
column-owner = 所有者
column-played-at = 再生日時
column-plays = 再生数
column-popularity = 人気度
column-title = タイトル
column-tracks = 曲数
column-year = 年
count-songs =
    { $count ->
       *[other] { $count }曲
    }
count-tracks =
    { $count ->
       *[other] { $count }曲
    }
filter-album = アルバムの曲を絞り込む
filter-downloaded = ダウンロード済み
filter-duration = 長さ
filter-favorites = お気に入りのみ
filter-hide-explicit = 露骨な表現を含む曲を隠す
filter-history = 再生履歴を絞り込む
filter-library = ライブラリを絞り込む
filter-owned = 自分のもの
filter-playable = 再生可能な曲のみ
filter-reset = フィルターをリセット
filter-year = 年
tool-filters = フィルター
tool-sort = 並べ替え

# Library and discovery
genre-empty = ここにはまだ何もありません
history-clear = 履歴をクリア
history-clear-confirm = このデバイスからすべての再生記録が削除されます。この操作は取り消せません。
history-clear-title = 再生履歴をクリア
history-empty = 再生した曲がここに表示されます。
history-not-loaded = 再生履歴を読み込めませんでした。
home-collection-albums = コレクションのアルバム
home-favorite-albums = お気に入りのアルバム
home-quick-picks = クイックピック
home-quick-picks-empty = 何曲かお気に入りに追加するとここに表示されます
home-recently-added = 最近追加した項目
home-recently-played = 最近再生した項目
library-liked-songs = お気に入り
library-local-unconfigured = ローカルライブラリを設定する
library-no-albums = 保存したアルバムはまだありません
library-no-artists = お気に入りのアーティストはまだいません
library-no-catalog-albums = アルバムが見つかりません
library-no-catalog-artists = アーティストが見つかりません
library-no-catalog-songs = 曲が見つかりません
library-no-local-albums = インポートされたアルバムが見つかりません
library-no-local-artists = インポートされたアーティストが見つかりません
library-no-local-playlists = ローカルのプレイリストはまだありません
library-no-local-songs = インポートされた曲が見つかりません
library-no-playlists = プレイリストはまだありません
library-no-songs = お気に入りはまだありません
library-not-loaded = ライブラリを読み込めませんでした
library-part-not-loaded = ライブラリのこの部分を読み込めませんでした
library-scanning = フォルダを確認しています…
library-scanning-progress = 音楽を読み込んでいます… { $read } / { $found }
search-albums-playlists = アルバムとプレイリスト
search-best-match = ベストマッチ
search-browse = すべてのジャンル
search-placeholder = 何を聴きたいですか？
search-recent = 最近の検索
search-results = 検索結果
search-saved =
    { $count ->
       *[other] ライブラリに { $count }曲
    }
search-tag = { $kind } ·
user-eyebrow = プロフィール
user-followers = { $count ->
   *[other] フォロワー { $value }人
}
user-following = { $count ->
   *[other] フォロー中 { $value }人
}
user-playlists = 公開プレイリスト
user-playlists-empty = 公開プレイリストはまだありません
