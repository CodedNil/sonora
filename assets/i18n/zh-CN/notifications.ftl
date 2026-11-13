# Notifications and status
get-sonora = 获取 Sonora
notice-never = 不再显示
offline-go-online = 切换为在线
offline-title = 你处于离线状态。
requirement-audio-bridge = { $server ->
    [pulseaudio] PulseAudio 正在运行，但 ALSA 无法连接到它，因此播放的内容都听不到。请安装 pulseaudio-alsa 并重启 Sonora。
   *[pipewire] PipeWire 正在运行，但 ALSA 无法连接到它，因此播放的内容都听不到。请安装 pipewire-alsa 并重启 Sonora。
}
requirement-audio-bridge-title = Sonora 无法连接到你的声音服务器
requirement-audio-output = Sonora 无法打开你的音频设备，因此播放的内容都听不到。请检查设备是否已连接，且未被其他应用占用。
requirement-audio-output-title = 无声音输出
requirement-help = 需要安装什么
requirement-web-engine = 安装 webkit2gtk-4.1 即可在浏览器窗口中登录 YouTube Music、Deezer 或 Apple Music。在此之前，请改为粘贴你的 Cookies。
requirement-web-engine-title = 浏览器登录不可用
toast-code-copied = 代码已复制
toast-download-cancelled = 已取消下载 { $name }
toast-download-failed = 无法列出要下载的歌曲。
toast-downloads-cancelled = { $count ->
    [one] 已取消下载 { $count } 首歌曲
   *[other] 已取消下载 { $count } 首歌曲
}
toast-favorite-albums-removed = 已从你的收藏中移除 { $count ->
        [one] { $count } 张专辑
       *[other] { $count } 张专辑
    }
toast-favorite-artists-removed = 已从你的收藏中移除 { $count ->
        [one] { $count } 位歌手
       *[other] { $count } 位歌手
    }
toast-favorite-removed = 已从你的收藏中移除 { $name }
toast-favorite-songs-removed = 已从你的收藏中移除 { $count ->
        [one] { $count } 首歌曲
       *[other] { $count } 首歌曲
    }
toast-folder-missing = { $name } 已不在磁盘上
toast-folder-removed = 已移除 { $name }
toast-history-clear-failed = 无法清空历史记录
toast-history-remove-failed = 无法从历史记录中移除歌曲
toast-history-removed = { $count ->
    [one] 已从历史记录中移除
   *[other] 已从历史记录中移除 { $count } 首歌曲
}
toast-history-restore-failed = 无法恢复历史记录
toast-jam-ended = Jam 已结束
toast-jam-failed = 无法发起或加入 Jam。
toast-jam-leave-failed = 无法结束或退出 Jam。
toast-jam-left = 你已退出 Jam
toast-jam-needs-connect = 在设置中开启“显示为 Spotify 设备”即可加入 Jam
toast-jam-unreadable = 该链接无法用于加入 Jam。
toast-keys-error = 播放密钥错误
toast-keys-refused = Spotify 可能无法使用
toast-last-album = 最后播放专辑
toast-last-artist = 最后播放歌手
toast-last-playlist = 最后播放该列表
toast-last-track = 最后播放 { $name }
toast-last-tracks = 最后播放 { $count } 首歌曲
toast-library-add-failed = { $name } 无法添加到你的音乐库
toast-library-added = 已添加到你的音乐库
toast-library-albums-removed = 已从你的音乐库中移除 { $count ->
        [one] { $count } 张专辑
       *[other] { $count } 张专辑
    }
toast-library-pin-failed = 无法在 Spotify 中更新固定状态。
toast-library-remove-failed = { $name } 无法从你的音乐库中移除
toast-library-removed-one = 已从你的音乐库中移除 { $name }
toast-library-scanned = { $count ->
    [one] 音乐库扫描完成，找到 { $count } 首歌曲
   *[other] 音乐库扫描完成，找到 { $count } 首歌曲
}
toast-library-songs-removed = 已从你的音乐库中移除 { $count ->
        [one] { $count } 首歌曲
       *[other] { $count } 首歌曲
    }
toast-link-copied = 链接已复制
toast-link-sign-in = 登录 { $name } 以打开此链接
toast-link-unreadable = 该链接指向的内容 Sonora 无法打开
toast-local-delete-failed = 部分歌曲文件无法删除
toast-next-album = 接下来播放专辑
toast-next-artist = 接下来播放歌手
toast-next-playlist = 接下来播放该列表
toast-next-track = 接下来播放 { $name }
toast-next-tracks = 接下来播放 { $count } 首歌曲
toast-offline = 无网络连接。恢复连接前将无法串流。
toast-offline-mode = 你处于离线状态。切换为在线即可串流这首歌曲。
toast-offline-unsupported = { $name } 仅在联网时才能播放下载内容。
toast-open-all-failed = { $count } 个文件无法打开
toast-open-failed = 无法打开 { $name }
toast-open-some-failed = 部分文件无法打开
toast-open-unsupported = { $name } 不是 Sonora 可以播放的音频文件
toast-output-default-failed = 无法选择系统默认设备
toast-output-select-failed = 无法选择 { $name }
toast-playback-stopped = 连续多首歌曲无法播放，播放已停止
toast-playlist-added = 播放列表已添加到你的音乐库
toast-playlist-busy = 另一个更改仍在进行中
toast-playlist-cover-changed = 播放列表封面已更换
toast-playlist-cover-removed = 播放列表封面已移除
toast-playlist-created = 播放列表已创建
toast-playlist-deleted = 播放列表已删除
toast-playlist-exported = 播放列表已导出
toast-playlist-failed = 该更改无法保存
toast-playlist-import-empty = { $name } 中没有任何歌曲在你的音乐库中
toast-playlist-imported-partly = 已导入 { $name }，跳过了 { $count ->
        [one] { $count } 首
       *[other] { $count } 首
    }不在你的音乐库中的歌曲
toast-playlist-removed = 播放列表已从你的音乐库中移除
toast-playlist-renamed = 播放列表已重命名
toast-playlist-signed-out = 登录以更改播放列表
toast-playlist-updated = 播放列表已更新
toast-playlist-visibility = 播放列表可见性已更改
toast-playlists-removed = 已从你的音乐库中移除 { $count ->
        [one] { $count } 个播放列表
       *[other] { $count } 个播放列表
    }
toast-queue-cleared = 播放队列已清空
toast-queue-failed = 无法添加到播放队列
toast-queue-local-elsewhere = 本地文件无法加入其他设备的播放队列
toast-queue-removed-track = 已从播放队列中移除 { $name }
toast-queue-removed-tracks = 已从播放队列中移除 { $count } 首歌曲
toast-queued-album = 专辑已添加到播放队列
toast-queued-artist = 歌手已添加到播放队列
toast-queued-playlist = 播放列表已添加到播放队列
toast-queued-track = { $name } 已添加到播放队列
toast-queued-tracks = { $count } 首歌曲已添加到播放队列
toast-radio-failed = 无法开启 { $name } 的电台
toast-restore-failed = { $name } 无法重新登录，Sonora 已在没有它的情况下继续运行
toast-sample-burst = 第 { $name } 条通知
toast-sample-delete = 已删除 { $name }
toast-sample-done = 已保存到你的音乐库
toast-sample-event = 活动已创建
toast-sample-event-detail = 12月3日星期日 上午 9:00
toast-sample-failed = 无法连接到服务器
toast-sample-info = 请在活动开始前 10 分钟到场。
toast-sample-lasting = 此通知会一直保留，直到你关闭它
toast-sample-loading = 正在上传你的歌曲
toast-sample-long = 这条通知比其他通知长得多，因此文字需要换到第二行甚至第三行，同时不能把关闭按钮挤出卡片
toast-sample-plain = 活动已创建
toast-sample-save-failed = { $name } 无法保存
toast-sample-save-failed-detail = 服务器返回了 503 Service Unavailable
toast-sample-saved = 已保存 { $name }
toast-sample-saving = 正在保存 { $name }
toast-sample-undone = 已撤销
toast-sample-warning = 你的会话将在五分钟后结束
toast-scan-failed = 无法扫描你的音乐文件夹
toast-settings-broken = 请修正 settings.json 的第 { $name } 行以保存更改
toast-settings-overwritten = settings.json 已在磁盘上被修改，因此你的上一次更改未保存
toast-settings-save-failed = 设置无法保存
toast-sign-in-to-play = { $name } 仅向已登录用户播放
toast-similar-failed = 找不到与 { $name } 相似的歌曲
toast-sleep-ended = 睡眠定时器已结束
toast-still-throttled = { $name } 仍在限制播放。按播放键重试
toast-tags-failed = 标签无法保存
toast-tags-saved = 已保存 { $name } 的标签
toast-throttled = { $name } 暂时限制了播放。稍后将重试
toast-track-added = 已添加到 { $name }
toast-track-removed = 已从 { $name } 中移除
toast-track-removed-unnamed = 已从播放列表中移除
toast-track-unplayable = { $name } 无法播放
toast-tracks-removed = 已从 { $name } 中移除 { $count ->
        [one] { $count } 首歌曲
       *[other] { $count } 首歌曲
    }
toast-tracks-removed-unnamed = 已从播放列表中移除 { $count ->
        [one] { $count } 首歌曲
       *[other] { $count } 首歌曲
    }
toast-tray-unavailable = 没有可放置图标的系统托盘
toast-unpinned = 已取消固定 { $name }
toast-why = 为什么？
toast-widevine-borrowed = 请尝试更新 Widevine
toast-widevine-download-failed = Widevine 无法下载
toast-widevine-failed = 播放失败
toast-widevine-install-failed = Widevine 无法安装
toast-widevine-remove-failed = Widevine 无法移除
trouble-not-loaded = 无法加载
trouble-offline = 无网络连接
trouble-offline-detail = 请检查网络连接并重试。
trouble-retry = 重试
update-available = Sonora { $version } 已发布
update-detail = 你当前使用的是 { $running }。查看更新内容，或立即更新。
update-detail-notes = 你当前使用的是 { $running }。查看更新内容，然后按你的安装方式更新 Sonora。
update-failed = 更新无法安装。请从发布页面重试。
update-never = 不检查更新
update-notes = 新内容
update-now = 更新
update-working = 正在下载更新…
wake-reason = 正在播放音乐
