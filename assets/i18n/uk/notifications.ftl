# Notifications and status
get-sonora = Завантажити Sonora
notice-never = Більше не показувати
offline-go-online = Перейти в онлайн
offline-title = Ви офлайн.
requirement-audio-bridge = { $server ->
    [pulseaudio] PulseAudio працює, але ALSA не має до нього маршруту, тож нічого не чути. Встановіть pulseaudio-alsa і перезапустіть Sonora.
   *[pipewire] PipeWire працює, але ALSA не має до нього маршруту, тож нічого не чути. Встановіть pipewire-alsa і перезапустіть Sonora.
}
requirement-audio-bridge-title = Sonora не може під'єднатися до звукового сервера
requirement-audio-output = Sonora не вдалося відкрити аудіопристрій, тож нічого не чути. Перевірте, чи пристрій підключено й чи не зайнятий він іншим застосунком.
requirement-audio-output-title = Немає виведення звуку
requirement-help = Що встановити
requirement-web-engine = Встановіть webkit2gtk-4.1, щоб входити в YouTube Music, Deezer чи Apple Music у вікні браузера. А поки вставте cookie вручну.
requirement-web-engine-title = Вхід через браузер недоступний
toast-code-copied = Код скопійовано
toast-download-cancelled = Завантаження { $name } скасовано
toast-download-failed = Не вдалося отримати список треків для завантаження.
toast-downloads-cancelled = { $count ->
    [one] Скасовано завантаження { $count } треку
    [few] Скасовано завантаження { $count } треків
   *[other] Скасовано завантаження { $count } треків
}
toast-favorite-albums-removed = Прибрано { $count ->
        [one] { $count } альбом
        [few] { $count } альбоми
       *[other] { $count } альбомів
    } з улюбленого
toast-favorite-artists-removed = Прибрано { $count ->
        [one] { $count } виконавця
        [few] { $count } виконавців
       *[other] { $count } виконавців
    } з улюбленого
toast-favorite-removed = { $name } прибрано з улюбленого
toast-favorite-songs-removed = Прибрано { $count ->
        [one] { $count } трек
        [few] { $count } треки
       *[other] { $count } треків
    } з улюбленого
toast-folder-missing = { $name } більше немає на диску
toast-folder-removed = { $name } вилучено
toast-history-clear-failed = Не вдалося очистити історію
toast-history-remove-failed = Не вдалося вилучити трек з історії
toast-history-removed = { $count ->
    [one] Вилучено { $count } трек з історії
    [few] Вилучено { $count } треки з історії
   *[other] Вилучено { $count } треків з історії
}
toast-history-restore-failed = Не вдалося відновити історію
toast-jam-ended = Jam завершено
toast-jam-failed = Не вдалося почати Jam або приєднатися до нього.
toast-jam-leave-failed = Не вдалося завершити Jam або вийти з нього.
toast-jam-left = Ви вийшли з Jam
toast-jam-needs-connect = Увімкніть «Показувати як пристрій Spotify» в налаштуваннях, щоб приєднатися до Jam
toast-jam-unreadable = Це посилання не веде до Jam, до якого можна приєднатися.
toast-keys-error = Помилка ключа
toast-keys-refused = Spotify може не грати
toast-last-album = Альбом прозвучить наприкінці
toast-last-artist = Виконавець прозвучить наприкінці
toast-last-playlist = Плейлист прозвучить наприкінці
toast-last-track = { $name } прозвучить наприкінці
toast-last-tracks = Наприкінці прозвучать треки ({ $count })
toast-library-add-failed = Не вдалося додати { $name } до медіатеки
toast-library-added = Додано до медіатеки
toast-library-albums-removed = Вилучено { $count ->
        [one] { $count } альбом
        [few] { $count } альбоми
       *[other] { $count } альбомів
    } з медіатеки
toast-library-pin-failed = Не вдалося змінити закріплення у Spotify.
toast-library-remove-failed = Не вдалося вилучити { $name } з медіатеки
toast-library-removed-one = { $name } вилучено з медіатеки
toast-library-scanned = { $count ->
    [one] Медіатеку проскановано, знайдено { $count } трек
    [few] Медіатеку проскановано, знайдено { $count } треки
   *[other] Медіатеку проскановано, знайдено { $count } треків
}
toast-library-songs-removed = Вилучено { $count ->
        [one] { $count } трек
        [few] { $count } треки
       *[other] { $count } треків
    } з медіатеки
toast-link-copied = Посилання скопійовано
toast-link-sign-in = Увійдіть у { $name }, щоб відкрити це посилання
toast-link-unreadable = Sonora не може відкрити це посилання
toast-local-delete-failed = Не вдалося видалити деякі файли треків
toast-next-album = Альбом прозвучить наступним
toast-next-artist = Виконавець прозвучить наступним
toast-next-playlist = Плейлист прозвучить наступним
toast-next-track = { $name } прозвучить наступним
toast-next-tracks = Наступними прозвучать треки ({ $count })
toast-offline = Немає з'єднання. Нічого не відтворюватиметься, доки воно не повернеться.
toast-offline-mode = Ви офлайн. Перейдіть в онлайн, щоб слухати цей трек.
toast-offline-unsupported = { $name } відтворює завантажене лише онлайн.
toast-open-all-failed = Не вдалося відкрити файли ({ $count })
toast-open-failed = Не вдалося відкрити { $name }
toast-open-some-failed = Деякі файли не вдалося відкрити
toast-open-unsupported = { $name } не є аудіофайлом, який може відтворити Sonora
toast-output-default-failed = Не вдалося вибрати системний пристрій за замовчуванням
toast-output-select-failed = Не вдалося вибрати { $name }
toast-playback-stopped = Відтворення зупинено: кілька треків поспіль не вдалося відтворити
toast-playlist-added = Плейлист додано до медіатеки
toast-playlist-busy = Інша зміна ще виконується
toast-playlist-cover-changed = Обкладинку плейлиста змінено
toast-playlist-cover-removed = Обкладинку плейлиста прибрано
toast-playlist-created = Плейлист створено
toast-playlist-deleted = Плейлист видалено
toast-playlist-exported = Плейлист експортовано
toast-playlist-failed = Не вдалося зберегти зміну
toast-playlist-import-empty = Жодного треку з { $name } немає в медіатеці
toast-playlist-imported-partly = Плейлист { $name } імпортовано без { $count ->
        [one] { $count } трека, якого
       *[other] { $count } треків, яких
    } немає в медіатеці
toast-playlist-removed = Плейлист вилучено з медіатеки
toast-playlist-renamed = Плейлист перейменовано
toast-playlist-signed-out = Увійдіть, щоб змінювати плейлисти
toast-playlist-updated = Плейлист оновлено
toast-playlist-visibility = Видимість плейлиста змінено
toast-playlists-removed = Вилучено { $count ->
        [one] { $count } плейлист
        [few] { $count } плейлисти
       *[other] { $count } плейлистів
    } з медіатеки
toast-queue-cleared = Чергу очищено
toast-queue-failed = Не вдалося додати до черги
toast-queue-local-elsewhere = Локальні файли не можна додати до черги на іншому пристрої
toast-queue-removed-track = { $name } вилучено з черги
toast-queue-removed-tracks = З черги вилучено треки ({ $count })
toast-queued-album = Альбом додано до черги
toast-queued-artist = Виконавця додано до черги
toast-queued-playlist = Плейлист додано до черги
toast-queued-track = { $name } додано до черги
toast-queued-tracks = До черги додано треки ({ $count })
toast-radio-failed = Не вдалося запустити радіо за { $name }
toast-restore-failed = Не вдалося знову увійти в { $name }, тож Sonora працює без нього
toast-sample-burst = Сповіщення номер { $name }
toast-sample-delete = { $name } видалено
toast-sample-done = Збережено в медіатеку
toast-sample-event = Подію створено
toast-sample-event-detail = Неділя, 3 грудня, 9:00
toast-sample-failed = Не вдалося зв'язатися із сервером
toast-sample-info = Прийдіть за 10 хвилин до початку події.
toast-sample-lasting = Це сповіщення лишається, доки ви його не закриєте
toast-sample-loading = Вивантажуємо ваші треки
toast-sample-long = Це сповіщення каже значно більше за інші, тож текст має переноситися на другий, а може й на третій рядок, не виштовхуючи кнопку закриття з картки
toast-sample-plain = Подію створено
toast-sample-save-failed = Не вдалося зберегти { $name }
toast-sample-save-failed-detail = Сервер відповів 503 Service Unavailable
toast-sample-saved = { $name } збережено
toast-sample-saving = Зберігаємо { $name }
toast-sample-undone = Скасовано
toast-sample-warning = Ваш сеанс завершиться за п'ять хвилин
toast-scan-failed = Не вдалося просканувати ваші папки з музикою
toast-settings-broken = Виправте рядок { $name } у settings.json, щоб зберегти зміни
toast-settings-overwritten = settings.json змінився на диску, тож вашу останню зміну не збережено
toast-settings-save-failed = Не вдалося зберегти налаштування
toast-sign-in-to-play = { $name } віддає музику лише тим, хто увійшов в акаунт
toast-similar-failed = Не вдалося знайти треки, схожі на { $name }
toast-sleep-ended = Таймер сну спрацював
toast-still-throttled = { $name } досі обмежує відтворення. Натисніть «Відтворити», щоб спробувати ще раз
toast-tags-failed = Не вдалося зберегти теги
toast-tags-saved = Теги «{ $name }» збережено
toast-throttled = { $name } поки обмежує відтворення. Повторна спроба за мить
toast-track-added = Додано до { $name }
toast-track-removed = Видалено з { $name }
toast-track-removed-unnamed = Вилучено з плейлиста
toast-track-unplayable = Не вдалося відтворити { $name }
toast-tracks-removed = Вилучено { $count ->
        [one] { $count } трек
        [few] { $count } треки
       *[other] { $count } треків
    } з { $name }
toast-tracks-removed-unnamed = Вилучено { $count ->
        [one] { $count } трек
        [few] { $count } треки
       *[other] { $count } треків
    } з плейлиста
toast-tray-unavailable = Немає області сповіщень, куди можна додати піктограму
toast-unpinned = { $name } відкріплено
toast-why = Чому?
toast-widevine-borrowed = Спробуйте оновити Widevine
toast-widevine-download-failed = Не вдалося завантажити Widevine
toast-widevine-failed = Не вдалося відтворити
toast-widevine-install-failed = Не вдалося встановити Widevine
toast-widevine-remove-failed = Не вдалося видалити Widevine
trouble-not-loaded = Не вдалося завантажити
trouble-offline = Немає з'єднання
trouble-offline-detail = Перевірте підключення до інтернету та спробуйте ще раз.
trouble-retry = Повторити
update-available = Вийшла Sonora { $version }
update-detail = У вас { $running }. Подивіться, що змінилося, або оновіться зараз.
update-detail-notes = У вас { $running }. Подивіться, що змінилося, і оновіть Sonora так само, як встановлювали.
update-failed = Не вдалося встановити оновлення. Спробуйте зі сторінки релізів.
update-never = Не перевіряти оновлення
update-notes = Що нового
update-now = Оновити
update-working = Завантажуємо оновлення…
wake-reason = Грає музика
