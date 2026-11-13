# Sound and equalizer
equalizer-add-filter = Add filter
equalizer-advanced = Advanced
equalizer-bass = Bass
equalizer-copied = Copied the profile as Equalizer APO text
equalizer-decibels = dB
equalizer-delete = Delete profile
equalizer-export = Export
equalizer-export-file = Export file…
equalizer-exported = Exported the profile
equalizer-file-unreadable = Could not read the file
equalizer-file-unwritable = Could not write the file
equalizer-filter = Filter { $number }
equalizer-filter-frequency = Filter { $number } frequency
equalizer-filter-gain = Filter { $number } gain
equalizer-filter-kind = Filter { $number } type
equalizer-filter-limit = A profile holds up to { $max } filters.
equalizer-filter-width = Filter { $number } width
equalizer-graph = Frequency response
equalizer-graph-detail = { $count ->
    [0] No filters
    [one] 1 filter
   *[other] { $count } filters
    }, preamp at { $db } dB
equalizer-hertz = Hz
equalizer-import = Import
equalizer-import-file = Import file…
equalizer-import-nothing = Found no equalizer settings to import
equalizer-import-too-large = Not imported: the text is over 1 MB, too large for an equalizer profile
equalizer-import-too-many = Not imported: { $count ->
        [one] one filter is on
       *[other] { $count } filters are on
    }, more than a profile holds
equalizer-imported-skipped = Imported { $name } without { $count ->
        [one] one line
       *[other] { $count } lines
    }
equalizer-invalid-frequency = Frequency must be between { $low } and { $high } Hz
equalizer-invalid-gain = Gain must be between { $low } and { $high } dB
equalizer-invalid-octaves = Bandwidth must be between { $low } and { $high } octaves
equalizer-invalid-preamp = Preamp must be between { $low } and { $high } dB
equalizer-invalid-q = Q must be between { $low } and { $high }
equalizer-invalid-shelf-octaves = A shelf takes a Q, not a bandwidth
equalizer-kind-all-pass = All pass
equalizer-kind-band-pass = Band pass
equalizer-kind-high-pass = High pass
equalizer-kind-high-shelf = High shelf
equalizer-kind-low-pass = Low pass
equalizer-kind-low-shelf = Low shelf
equalizer-kind-notch = Notch
equalizer-kind-peak = Peak
equalizer-level = Level
equalizer-lower-preamp = Lower preamp
equalizer-mode = Mode
equalizer-no-filters = No filters yet. Add one, pick a preset above, or import a profile.
equalizer-no-gain = —
equalizer-no-gain-spoken = No gain
equalizer-octaves = oct
equalizer-output-port = { $device } · { $port }
equalizer-overload = Clipping by { $db } dB
equalizer-overload-slight = Clipping by under 0.1 dB
equalizer-paste-empty = The clipboard holds no text
equalizer-preamp = Preamp
equalizer-preamp-auto = Automatic
equalizer-preamp-level = Preamp level
equalizer-preamp-manual = Manual
equalizer-profile-default = Default
equalizer-profile-name-hint = Profile name
equalizer-profile-new = Profile { $number }
equalizer-profile-pasted = Pasted profile
equalizer-q = Q
equalizer-redo = Redo
equalizer-remove-filter = Remove filter
equalizer-rename = Rename profile
equalizer-rule-add = Add rule
equalizer-rule-choose-device = Choose a device
equalizer-rule-choose-sound = Choose a sound
equalizer-rule-device = Device
equalizer-rule-disconnected = Not connected
equalizer-rule-other = Any other device
equalizer-rule-playing = Playing now
equalizer-rule-remove = Remove rule
equalizer-rule-replaces = Replaces its rule
equalizer-rules = Device rules
equalizer-save-as = Save as profile
equalizer-simple = Simple
equalizer-skip-expression = a value written as an expression
equalizer-skip-no-frequency = no frequency
equalizer-skip-no-gain = no gain
equalizer-skip-no-preamp = the preamp is not a number
equalizer-skip-no-width = no Q or bandwidth
equalizer-skip-unknown-type = unknown filter type { $name }
equalizer-skip-unreadable = not a filter Sonora can read
equalizer-skip-unsupported = { $name } is not supported
equalizer-skipped-line = Line { $line }: { $reason }
equalizer-skipped-more = and { $count ->
        [one] one more
       *[other] { $count } more
    }
equalizer-status-altered = Not bit-perfect due to:
equalizer-status-bit-perfect = Bit-perfect
equalizer-status-bit-perfect-detail = Bit-perfect from Sonora
equalizer-status-bits = { $from } → { $to } bit
equalizer-status-channels = { $from ->
    [1] Mono
    [2] Stereo
   *[other] { $from } ch
} → { $to ->
    [1] mono
    [2] stereo
   *[other] { $to } ch
}
equalizer-status-direct = Played by Soloist, past Sonora
equalizer-status-equalized = EQ
equalizer-status-float = Float → { $to } bit
equalizer-status-normalized = Normalized
equalizer-status-rate = { $from } → { $to } kHz
equalizer-status-separator = {", "}
equalizer-status-server = The sound server then converts it to { $khz } kHz
equalizer-status-volume = Volume
equalizer-tone-reset = Reset to 0 dB
equalizer-transfer = Import and export
equalizer-treble = Treble
equalizer-value-refused = Kept the previous value
response-graph-db = { $value } dB
response-graph-hz = { $value } Hz
response-graph-khz = { $value } kHz
response-graph-readout = { $frequency } · { $direction ->
        [up] +{ $gain }
        [down] -{ $gain }
       *[flat] { $gain }
    } dB
