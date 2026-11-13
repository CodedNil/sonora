# Sound and equalizer
equalizer-add-filter = Додати фільтр
equalizer-advanced = Розширений
equalizer-bass = Баси
equalizer-copied = Профіль скопійовано як текст Equalizer APO
equalizer-decibels = дБ
equalizer-delete = Видалити профіль
equalizer-export = Експорт
equalizer-export-file = Експортувати файл…
equalizer-exported = Профіль експортовано
equalizer-file-unreadable = Не вдалося прочитати файл
equalizer-file-unwritable = Не вдалося записати файл
equalizer-filter-limit = Профіль вміщує до { $max } фільтрів.
equalizer-hertz = Гц
equalizer-import = Імпорт
equalizer-import-file = Імпортувати файл…
equalizer-import-nothing = Не знайдено налаштувань еквалайзера для імпорту
equalizer-import-too-large = Не імпортовано: текст понад 1 МБ, завеликий для профілю еквалайзера
equalizer-import-too-many = Не імпортовано: { $count ->
        [one] увімкнено { $count } фільтр
        [few] увімкнено { $count } фільтри
       *[other] увімкнено { $count } фільтрів
    }, більше, ніж вміщує профіль
equalizer-imported-skipped = Імпортовано { $name } без { $count ->
        [one] { $count } рядка
        [few] { $count } рядків
       *[other] { $count } рядків
    }
equalizer-invalid-frequency = Частота має бути від { $low } до { $high } Гц
equalizer-invalid-gain = Підсилення має бути від { $low } до { $high } дБ
equalizer-invalid-octaves = Ширина смуги має бути від { $low } до { $high } октав
equalizer-invalid-preamp = Передпідсилення має бути від { $low } до { $high } дБ
equalizer-invalid-q = Q має бути від { $low } до { $high }
equalizer-invalid-shelf-octaves = Полиця задається через Q, а не ширину смуги
equalizer-kind-all-pass = Фазовий
equalizer-kind-band-pass = Смуговий
equalizer-kind-high-pass = Фільтр ВЧ
equalizer-kind-high-shelf = Полиця ВЧ
equalizer-kind-low-pass = Фільтр НЧ
equalizer-kind-low-shelf = Полиця НЧ
equalizer-kind-notch = Режекторний
equalizer-kind-peak = Піковий
equalizer-level = Рівень
equalizer-lower-preamp = Знизити передпідсилення
equalizer-no-filters = Фільтрів ще немає. Додайте фільтр, виберіть пресет вище або імпортуйте профіль.
equalizer-no-gain = —
equalizer-octaves = окт
equalizer-output-port = { $device } · { $port }
equalizer-overload = Перевантаження на { $db } дБ
equalizer-overload-slight = Перевантаження менш ніж на 0,1 дБ
equalizer-paste-empty = У буфері обміну немає тексту
equalizer-preamp = Передпідсилення
equalizer-preamp-auto = Автоматично
equalizer-preamp-manual = Вручну
equalizer-profile-default = За замовчуванням
equalizer-profile-name-hint = Назва профілю
equalizer-profile-new = Профіль { $number }
equalizer-profile-pasted = Вставлений профіль
equalizer-q = Q
equalizer-redo = Повторити
equalizer-remove-filter = Видалити фільтр
equalizer-rename = Перейменувати профіль
equalizer-rule-add = Додати правило
equalizer-rule-choose-device = Виберіть пристрій
equalizer-rule-choose-sound = Виберіть звук
equalizer-rule-disconnected = Не підключено
equalizer-rule-other = Будь-який інший пристрій
equalizer-rule-playing = Грає зараз
equalizer-rule-remove = Видалити правило
equalizer-rule-replaces = Замінює його правило
equalizer-rules = Правила пристроїв
equalizer-save-as = Зберегти як профіль
equalizer-simple = Простий
equalizer-skip-expression = значення записано виразом
equalizer-skip-no-frequency = немає частоти
equalizer-skip-no-gain = немає підсилення
equalizer-skip-no-preamp = передпідсилення не є числом
equalizer-skip-no-width = немає Q або ширини смуги
equalizer-skip-unknown-type = невідомий тип фільтра { $name }
equalizer-skip-unreadable = фільтр, який Sonora не може прочитати
equalizer-skip-unsupported = { $name } не підтримується
equalizer-skipped-line = Рядок { $line }: { $reason }
equalizer-skipped-more = і ще { $count ->
        [one] { $count }
       *[other] { $count }
    }
equalizer-status-altered = Не bit-perfect через:
equalizer-status-bit-perfect = Bit-perfect
equalizer-status-bit-perfect-detail = Sonora передає сигнал bit-perfect
equalizer-status-bits = { $from } → { $to } біт
equalizer-status-channels = { $from ->
    [1] Моно
    [2] Стерео
   *[other] { $from } кан.
} → { $to ->
    [1] моно
    [2] стерео
   *[other] { $to } кан.
}
equalizer-status-equalized = EQ
equalizer-status-float = Float → { $to } біт
equalizer-status-normalized = Нормалізація
equalizer-status-rate = { $from } → { $to } кГц
equalizer-status-separator = {", "}
equalizer-status-server = Далі звуковий сервер перетворює його на { $khz } кГц
equalizer-status-volume = Гучність
equalizer-tone-reset = Скинути до 0 дБ
equalizer-transfer = Імпорт і експорт
equalizer-treble = Високі
equalizer-value-refused = Залишено попереднє значення
response-graph-db = { $value } дБ
response-graph-hz = { $value } Гц
response-graph-khz = { $value } кГц
response-graph-readout = { $frequency } · { $direction ->
        [up] +{ $gain }
        [down] -{ $gain }
       *[flat] { $gain }
    } дБ
