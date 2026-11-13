use music::Track;

use ui::{Filter, Unit};

use crate::chrome::tools::FilterState;

/// A choice axis over explicit metadata or provider-neutral audio measurements.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Words {
    Genre,
    Mood,
    Tag,
}

impl Words {
    pub(super) fn read<'a>(self, track: &'a Track) -> impl Iterator<Item = &'a str> {
        let words = match self {
            Self::Genre => &track.genres,
            Self::Mood => &track.moods,
            Self::Tag => &track.tags,
        };
        words.iter().map(String::as_str).chain(
            track
                .audio_features
                .as_ref()
                .filter(|_| self == Self::Mood)
                .into_iter()
                .flat_map(music::AudioFeatures::moods)
                .map(|mood| -> &'a str { mood }),
        )
    }
}

pub(super) const FACETS: [(&str, Words); 3] = [
    ("filter-genre", Words::Genre),
    ("filter-mood", Words::Mood),
    ("filter-tag", Words::Tag),
];

/// One numeric track field, used for both its slider and its matching rule.
type Number = fn(&Track) -> Option<f32>;
pub(super) const RANGES: [(&str, Number, Unit); 5] = [
    (
        "filter-duration",
        |track| track.duration.map(|duration| duration.as_secs_f32()),
        Unit::Clock,
    ),
    (
        "filter-year",
        |track| track.year().map(f32::from),
        Unit::Plain,
    ),
    (
        "filter-play-count",
        |track| track.playcount.map(|count| count as f32),
        Unit::Plain,
    ),
    (
        "filter-popularity",
        |track| track.popularity.map(f32::from),
        Unit::Plain,
    ),
    ("filter-bpm", |track| track.bpm.map(f32::from), Unit::Plain),
];

/// Track-specific matching over the shared filter state.
#[derive(Clone, Default, PartialEq)]
pub(crate) struct TrackSieve {
    pub filters: FilterState,
}

impl TrackSieve {
    pub(crate) fn active(&self) -> bool {
        self.filters.active()
    }

    /// Offers the flags supported by this track list.
    pub(super) fn flags(&self, catalog: bool, downloads: bool) -> Vec<Filter> {
        self.filters.flags([
            ("filter-hide-explicit", true),
            ("filter-playable", true),
            ("filter-collaborations", true),
            ("filter-favorites", catalog),
            ("filter-downloaded", downloads),
        ])
    }

    pub(super) fn keeps(&self, track: &Track) -> bool {
        if self.filters.on("filter-hide-explicit") && track.explicit {
            return false;
        }
        if self.filters.on("filter-collaborations") && track.artist_refs.len() < 2 {
            return false;
        }
        if self.filters.on("filter-playable") && !track.playable {
            return false;
        }
        RANGES
            .iter()
            .all(|(key, number, _)| keeps_range(self.filters.range(key), number(track)))
            && FACETS.iter().all(|(key, words)| {
                self.filters.picks(key).is_none_or(|picked| {
                    words
                        .read(track)
                        .any(|word| picked.iter().any(|picked| picked == word))
                })
            })
    }
}

fn keeps_range(range: Option<(f32, f32)>, value: Option<f32>) -> bool {
    range.is_none_or(|bounds| value.is_some_and(|value| within(value, bounds)))
}

fn within(value: f32, (low, high): (f32, f32)) -> bool {
    value >= low - 0.5 && value <= high + 0.5
}

#[cfg(test)]
mod tests {
    use super::TrackSieve;
    use crate::shared::tracks::fixture::track;
    use ui::FilterChange;

    #[test]
    fn duration_bounds_are_inclusive() {
        let mut sieve = TrackSieve::default();
        sieve
            .filters
            .apply(FilterChange::Range("filter-duration", Some((60., 180.))));

        assert!(sieve.active());
        assert!(sieve.keeps(&track(60, false, true)));
        assert!(sieve.keeps(&track(180, false, true)));
        assert!(sieve.keeps(&track(120, false, true)));
        assert!(!sieve.keeps(&track(59, false, true)));
        assert!(!sieve.keeps(&track(181, false, true)));
    }

    #[test]
    fn every_axis_must_pass() {
        let mut sieve = TrackSieve::default();
        sieve
            .filters
            .apply(FilterChange::Range("filter-duration", Some((60., 180.))));
        sieve
            .filters
            .apply(FilterChange::Flag("filter-hide-explicit", true));
        sieve
            .filters
            .apply(FilterChange::Flag("filter-playable", true));

        assert!(sieve.keeps(&track(120, false, true)));
        assert!(!sieve.keeps(&track(120, true, true)));
        assert!(!sieve.keeps(&track(120, false, false)));
        assert!(!sieve.keeps(&track(400, false, true)));
    }
}
