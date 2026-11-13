# Notifications and status
get-sonora = Скачать Sonora
notice-never = Больше не показывать
offline-go-online = Перейти в онлайн
offline-title = Вы офлайн.
requirement-audio-bridge = { $server ->
    [pulseaudio] PulseAudio запущен, но у ALSA нет к нему маршрута, поэтому ничего не слышно. Установите pulseaudio-alsa и перезапустите Sonora.
   *[pipewire] PipeWire запущен, но у ALSA нет к нему маршрута, поэтому ничего не слышно. Установите pipewire-alsa и перезапустите Sonora.
}
requirement-audio-bridge-title = Sonora не может подключиться к звуковому серверу
requirement-audio-output = Sonora не смогла открыть аудиоустройство, поэтому ничего не слышно. Проверьте, что устройство подключено и не занято другим приложением.
requirement-audio-output-title = Нет вывода звука
requirement-help = Что установить
requirement-web-engine = Установите webkit2gtk-4.1, чтобы входить в YouTube Music, Deezer или Apple Music в окне браузера. А пока вставьте cookie вручную.
requirement-web-engine-title = Вход через браузер недоступен
toast-code-copied = Код скопирован
toast-download-cancelled = Отменено скачивание { $name }
toast-download-failed = Не удалось получить список треков для скачивания.
toast-downloads-cancelled = { $count ->
    [one] Отменено скачивание { $count } трека
   *[other] Отменено скачивание { $count } треков
}
toast-favorite-albums-removed = { $count ->
        [one] Удалён { $count } альбом
        [few] Удалено { $count } альбома
       *[other] Удалено { $count } альбомов
    } из избранного
toast-favorite-artists-removed = { $count ->
        [one] Удалён { $count } исполнитель
        [few] Удалено { $count } исполнителя
       *[other] Удалено { $count } исполнителей
    } из избранного
toast-favorite-removed = Из избранного удалено: { $name }
toast-favorite-songs-removed = { $count ->
        [one] Удалён { $count } трек
        [few] Удалено { $count } трека
       *[other] Удалено { $count } треков
    } из избранного
toast-folder-missing = Папки { $name } больше нет на диске
toast-folder-removed = Папка { $name } удалена
toast-history-clear-failed = Не удалось очистить историю
toast-history-remove-failed = Не удалось удалить трек из истории
toast-history-removed = { $count ->
    [one] Из истории удалён { $count } трек
    [few] Из истории удалено { $count } трека
   *[other] Из истории удалено { $count } треков
}
toast-history-restore-failed = Не удалось восстановить историю
toast-jam-ended = Jam завершён
toast-jam-failed = Не удалось начать Jam или присоединиться к нему.
toast-jam-leave-failed = Не удалось завершить или покинуть Jam.
toast-jam-left = Вы покинули Jam
toast-jam-needs-connect = Включите в настройках «Показывать как устройство Spotify», чтобы присоединиться к Jam
toast-jam-unreadable = Эта ссылка не ведёт в Jam, к которому можно присоединиться.
toast-keys-error = Ошибка ключа
toast-keys-refused = Spotify может не играть
toast-last-album = Альбом прозвучит в конце
toast-last-artist = Исполнитель прозвучит в конце
toast-last-playlist = Плейлист прозвучит в конце
toast-last-track = { $name } прозвучит в конце
toast-last-tracks = { $count ->
    [one] { $count } трек прозвучит в конце
    [few] { $count } трека прозвучат в конце
   *[other] { $count } треков прозвучат в конце
}
toast-library-add-failed = Не удалось добавить { $name } в медиатеку
toast-library-added = Добавлено в медиатеку
toast-library-albums-removed = { $count ->
        [one] Удалён { $count } альбом
        [few] Удалено { $count } альбома
       *[other] Удалено { $count } альбомов
    } из медиатеки
toast-library-pin-failed = Не удалось изменить закрепление в Spotify.
toast-library-remove-failed = Не удалось удалить { $name } из медиатеки
toast-library-removed-one = Из медиатеки удалено: { $name }
toast-library-scanned = { $count ->
    [one] Медиатека просканирована, найден { $count } трек
    [few] Медиатека просканирована, найдено { $count } трека
   *[other] Медиатека просканирована, найдено { $count } треков
}
toast-library-songs-removed = { $count ->
        [one] Удалён { $count } трек
        [few] Удалено { $count } трека
       *[other] Удалено { $count } треков
    } из медиатеки
toast-link-copied = Ссылка скопирована
toast-link-sign-in = Войдите в { $name }, чтобы открыть эту ссылку
toast-link-unreadable = Sonora не может открыть эту ссылку
toast-local-delete-failed = Не удалось удалить часть файлов треков
toast-next-album = Альбом прозвучит следующим
toast-next-artist = Исполнитель прозвучит следующим
toast-next-playlist = Плейлист прозвучит следующим
toast-next-track = { $name } прозвучит следующим
toast-next-tracks = { $count ->
    [one] { $count } трек прозвучит следующим
    [few] { $count } трека прозвучат следующими
   *[other] { $count } треков прозвучат следующими
}
toast-offline = Нет соединения. Ничего не будет воспроизводиться, пока оно не вернётся.
toast-offline-mode = Вы офлайн. Перейдите в онлайн, чтобы слушать эту песню.
toast-offline-unsupported = { $name } играет скачанное только онлайн.
toast-open-all-failed = { $count ->
    [one] Не удалось открыть { $count } файл
    [few] Не удалось открыть { $count } файла
   *[other] Не удалось открыть { $count } файлов
}
toast-open-failed = Не удалось открыть { $name }
toast-open-some-failed = Не удалось открыть часть файлов
toast-open-unsupported = Sonora не может воспроизвести { $name }
toast-output-default-failed = Не удалось выбрать системное устройство по умолчанию
toast-output-select-failed = Не удалось выбрать { $name }
toast-playback-stopped = Воспроизведение остановлено: несколько треков подряд не удалось воспроизвести
toast-playlist-added = Плейлист добавлен в медиатеку
toast-playlist-busy = Другое изменение ещё выполняется
toast-playlist-cover-changed = Обложка плейлиста изменена
toast-playlist-cover-removed = Обложка плейлиста убрана
toast-playlist-created = Плейлист создан
toast-playlist-deleted = Плейлист удалён
toast-playlist-exported = Плейлист экспортирован
toast-playlist-failed = Не удалось сохранить изменение
toast-playlist-import-empty = Ни одного трека из { $name } нет в медиатеке
toast-playlist-imported-partly = Плейлист { $name } импортирован без { $count ->
        [one] { $count } трека, которого
       *[other] { $count } треков, которых
    } нет в медиатеке
toast-playlist-removed = Плейлист удалён из медиатеки
toast-playlist-renamed = Плейлист переименован
toast-playlist-signed-out = Войдите, чтобы менять плейлисты
toast-playlist-updated = Плейлист обновлён
toast-playlist-visibility = Видимость плейлиста изменена
toast-playlists-removed = { $count ->
        [one] Удалён { $count } плейлист
        [few] Удалено { $count } плейлиста
       *[other] Удалено { $count } плейлистов
    } из медиатеки
toast-queue-cleared = Очередь очищена
toast-queue-failed = Не удалось добавить в очередь
toast-queue-local-elsewhere = Локальные файлы нельзя добавить в очередь на другом устройстве
toast-queue-removed-track = { $name } убран из очереди
toast-queue-removed-tracks = { $count ->
    [one] Из очереди убран { $count } трек
    [few] Из очереди убрано { $count } трека
   *[other] Из очереди убрано { $count } треков
}
toast-queued-album = Альбом добавлен в очередь
toast-queued-artist = Исполнитель добавлен в очередь
toast-queued-playlist = Плейлист добавлен в очередь
toast-queued-track = { $name } добавлен в очередь
toast-queued-tracks = { $count ->
    [one] { $count } трек добавлен в очередь
    [few] { $count } трека добавлены в очередь
   *[other] { $count } треков добавлено в очередь
}
toast-radio-failed = Не удалось запустить радио по { $name }
toast-restore-failed = Не удалось снова войти в { $name }, Sonora продолжила работу без него
toast-sample-burst = Уведомление номер { $name }
toast-sample-delete = Удалено: { $name }
toast-sample-done = Сохранено в медиатеку
toast-sample-event = Событие создано
toast-sample-event-detail = Воскресенье, 3 декабря, 9:00
toast-sample-failed = Не удалось связаться с сервером
toast-sample-info = Приходите за 10 минут до начала.
toast-sample-lasting = Это уведомление не исчезнет, пока вы его не закроете
toast-sample-loading = Загружаем ваши треки
toast-sample-long = Это уведомление говорит куда больше остальных, поэтому текст должен переноситься на вторую, а то и третью строку, не выталкивая кнопку закрытия из карточки
toast-sample-plain = Событие создано
toast-sample-save-failed = Не удалось сохранить { $name }
toast-sample-save-failed-detail = Сервер ответил 503 Service Unavailable
toast-sample-saved = Сохранено: { $name }
toast-sample-saving = Сохранение { $name }
toast-sample-undone = Отменено
toast-sample-warning = Сессия закончится через пять минут
toast-scan-failed = Не удалось просканировать папки с музыкой
toast-session-ended = Сессия завершена
toast-session-failed = Не удалось начать сессию или присоединиться к ней.
toast-session-full = В этой сессии нет мест.
toast-session-kicked = Хост удалил вас из сессии
toast-session-left = Вы покинули сессию
toast-session-lost = Связь с сессией потеряна
toast-session-needs-provider = Войдите в стриминговый сервис, чтобы начать сессию.
toast-session-not-found = Сессии с таким кодом нет.
toast-session-provider = Эта сессия играет из { $name }. Войдите в { $name }, чтобы присоединиться.
toast-session-rate-limited = Слишком много неверных кодов. Попробуйте через несколько минут.
toast-session-too-many = Из вашей сети уже запущено максимальное число сессий. Завершите одну, чтобы начать новую.
toast-session-unreachable = Не удалось связаться с Sonora Session. Попробуйте позже.
toast-sessions-server-empty = Сначала введите адрес вашего сервера
toast-sessions-server-malformed = Это не похоже на адрес сервера
toast-sessions-server-saved = Сервер сессий сохранён
toast-sessions-server-silent = По этому адресу не отвечает сервер Sonora Sessions, поэтому он не сохранён
toast-settings-broken = Исправьте строку { $name } в settings.json, чтобы сохранять изменения
toast-settings-overwritten = settings.json изменён на диске, поэтому последнее изменение не сохранено
toast-settings-save-failed = Не удалось сохранить настройки
toast-sign-in-to-play = { $name } отдаёт музыку только тем, кто вошёл в аккаунт
toast-similar-failed = Не удалось найти треки, похожие на { $name }
toast-sleep-ended = Таймер сна сработал
toast-soloist-ready = Spotify теперь играет через Soloist
toast-soloist-refused = Soloist больше не может войти
toast-soloist-remove-failed = Не удалось удалить Soloist
toast-still-throttled = { $name } всё ещё ограничивает воспроизведение. Включите трек, чтобы попробовать снова
toast-tags-failed = Не удалось сохранить теги
toast-tags-saved = Теги «{ $name }» сохранены
toast-throttled = { $name } пока ограничивает воспроизведение. Скоро попробуем снова
toast-track-added = Добавлено в { $name }
toast-track-removed = Удалено из { $name }
toast-track-removed-unnamed = Удалено из плейлиста
toast-track-unplayable = Не удалось воспроизвести { $name }
toast-tracks-removed = { $count ->
        [one] Удалён { $count } трек
        [few] Удалено { $count } трека
       *[other] Удалено { $count } треков
    } из { $name }
toast-tracks-removed-unnamed = { $count ->
        [one] Удалён { $count } трек
        [few] Удалено { $count } трека
       *[other] Удалено { $count } треков
    } из плейлиста
toast-tray-unavailable = Системного трея нет, значок некуда поместить
toast-unpinned = Откреплено: { $name }
toast-why = Почему?
toast-widevine-borrowed = Попробуйте обновить Widevine
toast-widevine-download-failed = Не удалось скачать Widevine
toast-widevine-failed = Ошибка воспроизведения
toast-widevine-install-failed = Не удалось установить Widevine
toast-widevine-remove-failed = Не удалось удалить Widevine
trouble-not-loaded = Не удалось загрузить
trouble-offline = Нет соединения
trouble-offline-detail = Проверьте подключение к интернету и попробуйте снова.
trouble-retry = Повторить
update-available = Вышла Sonora { $version }
update-detail = У вас { $running }. Посмотрите, что изменилось, или обновитесь сейчас.
update-detail-notes = У вас { $running }. Посмотрите, что изменилось, и обновите Sonora так же, как устанавливали.
update-failed = Не удалось установить обновление. Попробуйте со страницы релизов.
update-never = Не проверять обновления
update-notes = Что нового
update-now = Обновить
update-working = Загружаем обновление…
wake-reason = Играет музыка
