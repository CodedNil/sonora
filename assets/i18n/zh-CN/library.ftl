# Tables, filters and counts
column-added-by = 添加者
column-album = 专辑
column-artist = 歌手
column-date-added = 添加日期
column-genre = 流派
column-index = #
column-length = 时长
column-modified = 修改时间
column-name = 名称
column-owner = 所有者
column-played-at = 播放时间
column-plays = 播放次数
column-popularity = 热度
column-title = 标题
column-tracks = 歌曲总数
column-year = 年份
count-songs =
    { $count ->
        [one] { $count } 首歌曲
       *[other] { $count } 首歌曲
    }
count-tracks =
    { $count ->
        [one] { $count } 首歌曲
       *[other] { $count } 首歌曲
    }
filter-album = 筛选专辑歌曲
filter-downloaded = 已下载
filter-duration = 时长
filter-favorites = 仅收藏
filter-hide-explicit = 隐藏露骨内容
filter-history = 筛选收听历史
filter-library = 筛选音乐库
filter-owned = 由你拥有
filter-playable = 仅播放
filter-reset = 重置筛选
filter-year = 年份
tool-filters = 筛选
tool-sort = 排序

# Library and discovery
genre-empty = 这里暂时没有可显示的内容
history-clear = 清空历史记录
history-clear-confirm = 此设备上的所有播放记录将被移除。此操作无法撤销。
history-clear-title = 清空收听历史
history-empty = 你播放的歌曲会显示在这里。
history-not-loaded = 收听历史加载失败。
home-collection-albums = 你音乐库中的专辑
home-favorite-albums = 收藏的专辑
home-quick-picks = 快速推荐
home-quick-picks-empty = 收藏几首歌后，它们会显示在这里
home-recently-added = 最近添加
home-recently-played = 最近播放
library-liked-songs = 收藏
library-local-unconfigured = 配置本地音乐库
library-no-albums = 还没有保存的专辑
library-no-artists = 还没有收藏的歌手
library-no-catalog-albums = 未找到专辑
library-no-catalog-artists = 未找到歌手
library-no-catalog-songs = 未找到歌曲
library-no-local-albums = 未找到导入的专辑
library-no-local-artists = 未找到导入的歌手
library-no-local-playlists = 还没有本地播放列表
library-no-local-songs = 未找到导入的歌曲
library-no-playlists = 还没有播放列表
library-no-songs = 还没有收藏的歌曲
library-not-loaded = 音乐库加载失败
library-part-not-loaded = 音乐库的这部分加载失败
library-scanning = 正在浏览你的文件夹…
library-scanning-progress = 正在读取你的音乐…{ $read } / { $found }
search-albums-playlists = 专辑和播放列表
search-best-match = 最佳匹配
search-browse = 浏览全部
search-placeholder = 你想听什么？
search-recent = 最近搜索
search-results = 结果
search-saved =
    { $count ->
        [one] 音乐库中有 { $count } 首歌曲
       *[other] 音乐库中有 { $count } 首歌曲
    }
search-tag = { $kind } ·
user-eyebrow = 个人资料
user-followers = { $count ->
    [one] { $value } 位关注者
   *[other] { $value } 位关注者
}
user-following = { $count ->
   *[other] 正在关注 { $value } 人
}
user-playlists = 公开播放列表
user-playlists-empty = 还没有公开播放列表
