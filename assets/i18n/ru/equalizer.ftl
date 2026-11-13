# Sound and equalizer
equalizer-add-filter = Добавить фильтр
equalizer-advanced = Расширенный
equalizer-bass = Басы
equalizer-copied = Профиль скопирован как текст Equalizer APO
equalizer-decibels = дБ
equalizer-delete = Удалить профиль
equalizer-export = Экспорт
equalizer-export-file = Экспорт в файл…
equalizer-exported = Профиль экспортирован
equalizer-file-unreadable = Не удалось прочитать файл
equalizer-file-unwritable = Не удалось записать файл
equalizer-filter-limit = В профиле может быть до { $max } фильтров.
equalizer-hertz = Гц
equalizer-import = Импорт
equalizer-import-file = Импорт из файла…
equalizer-import-nothing = Не найдено настроек эквалайзера для импорта
equalizer-import-too-large = Не импортировано: текст больше 1 МБ, слишком много для профиля эквалайзера
equalizer-import-too-many = Не импортировано: { $count ->
        [one] включён { $count } фильтр
        [few] включено { $count } фильтра
       *[other] включено { $count } фильтров
    }, больше, чем вмещает профиль
equalizer-imported-skipped = Импортировано: { $name }, без { $count ->
        [one] { $count } строки
        [few] { $count } строк
       *[other] { $count } строк
    }
equalizer-invalid-frequency = Частота должна быть от { $low } до { $high } Гц
equalizer-invalid-gain = Усиление должно быть от { $low } до { $high } дБ
equalizer-invalid-octaves = Ширина полосы должна быть от { $low } до { $high } октав
equalizer-invalid-preamp = Предусиление должно быть от { $low } до { $high } дБ
equalizer-invalid-q = Q должно быть от { $low } до { $high }
equalizer-invalid-shelf-octaves = Для полки задаётся Q, а не ширина полосы
equalizer-kind-all-pass = Всепропускающий
equalizer-kind-band-pass = Полосовой
equalizer-kind-high-pass = Фильтр ВЧ
equalizer-kind-high-shelf = Высокая полка
equalizer-kind-low-pass = Фильтр НЧ
equalizer-kind-low-shelf = Низкая полка
equalizer-kind-notch = Режекторный
equalizer-kind-peak = Пиковый
equalizer-level = Уровень
equalizer-lower-preamp = Снизить предусиление
equalizer-no-filters = Фильтров пока нет. Добавьте фильтр, выберите пресет выше или импортируйте профиль.
equalizer-no-gain = —
equalizer-octaves = окт.
equalizer-output-port = { $device } · { $port }
equalizer-overload = Клиппинг на { $db } дБ
equalizer-overload-slight = Клиппинг меньше 0,1 дБ
equalizer-paste-empty = В буфере обмена нет текста
equalizer-preamp = Предусиление
equalizer-preamp-auto = Автоматически
equalizer-preamp-manual = Вручную
equalizer-profile-default = По умолчанию
equalizer-profile-name-hint = Название профиля
equalizer-profile-new = Профиль { $number }
equalizer-profile-pasted = Вставленный профиль
equalizer-q = Q
equalizer-redo = Повторить
equalizer-remove-filter = Удалить фильтр
equalizer-rename = Переименовать профиль
equalizer-rule-add = Добавить правило
equalizer-rule-choose-device = Выберите устройство
equalizer-rule-choose-sound = Выберите звучание
equalizer-rule-disconnected = Не подключено
equalizer-rule-other = Любое другое устройство
equalizer-rule-playing = Сейчас играет
equalizer-rule-remove = Удалить правило
equalizer-rule-replaces = Заменяет его правило
equalizer-rules = Правила для устройств
equalizer-save-as = Сохранить как профиль
equalizer-simple = Простой
equalizer-skip-expression = значение записано выражением
equalizer-skip-no-frequency = нет частоты
equalizer-skip-no-gain = нет усиления
equalizer-skip-no-preamp = предусиление не число
equalizer-skip-no-width = нет Q или ширины полосы
equalizer-skip-unknown-type = неизвестный тип фильтра { $name }
equalizer-skip-unreadable = не распознано как фильтр
equalizer-skip-unsupported = { $name } не поддерживается
equalizer-skipped-line = Строка { $line }: { $reason }
equalizer-skipped-more = и ещё { $count ->
        [one] { $count } строка
        [few] { $count } строки
       *[other] { $count } строк
    }
equalizer-status-altered = Не bit-perfect из-за:
equalizer-status-bit-perfect = Bit-perfect
equalizer-status-bit-perfect-detail = Bit-perfect на выходе Sonora
equalizer-status-bits = { $from } → { $to } бит
equalizer-status-channels = { $from ->
    [1] Моно
    [2] Стерео
   *[other] { $from } кан.
} → { $to ->
    [1] моно
    [2] стерео
   *[other] { $to } кан.
}
equalizer-status-direct = Играет Soloist, мимо Sonora
equalizer-status-equalized = EQ
equalizer-status-float = Float → { $to } бит
equalizer-status-normalized = Нормализация
equalizer-status-rate = { $from } → { $to } кГц
equalizer-status-separator = {", "}
equalizer-status-server = Затем звуковой сервер переводит его в { $khz } кГц
equalizer-status-volume = Громкость
equalizer-tone-reset = Сбросить до 0 дБ
equalizer-transfer = Импорт и экспорт
equalizer-treble = Высокие
equalizer-value-refused = Оставлено прежнее значение
response-graph-db = { $value } дБ
response-graph-hz = { $value } Гц
response-graph-khz = { $value } кГц
response-graph-readout = { $frequency } · { $direction ->
        [up] +{ $gain }
        [down] -{ $gain }
       *[flat] { $gain }
    } дБ
