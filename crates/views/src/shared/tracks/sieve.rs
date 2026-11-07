use music::Track;
use ui::{Filter, FlagAxis};

#[derive(Clone, Copy, Default, PartialEq)]
pub(crate) struct TrackSieve {
    pub year: Option<(f32, f32)>,
    pub popularity: Option<(f32, f32)>,
    pub duration: Option<(f32, f32)>,
    /// Drop tracks marked explicit.
    pub hide_explicit: bool,
    pub playable: bool,
    /// Keep only starred tracks. `keeps` cannot see the library, so the source applies it.
    pub favorites: bool,
    /// Keep only downloaded tracks, which the source applies for the same reason.
    pub downloaded: bool,
    pub collaborations: bool,
}

impl TrackSieve {
    pub(crate) fn active(&self) -> bool {
        self.year.is_some()
            || self.popularity.is_some()
            || self.duration.is_some()
            || self.hide_explicit
            || self.playable
            || self.favorites
            || self.downloaded
            || self.collaborations
    }

    /// The flag definitions used by every track table; unavailable library/download controls stay out.
    pub(super) fn flags(&self, catalog: bool, downloads: bool) -> Vec<Filter> {
        let flag = |key, on| {
            Filter::Flag(FlagAxis {
                key,
                label: i18n::lookup(key, None),
                on,
            })
        };
        let mut flags = vec![
            flag("filter-hide-explicit", self.hide_explicit),
            flag("filter-playable", self.playable),
            flag("filter-collaborations", self.collaborations),
        ];
        if catalog {
            flags.push(flag("filter-favorites", self.favorites));
        }
        if downloads {
            flags.push(flag("filter-downloaded", self.downloaded));
        }
        flags
    }

    pub(super) fn keeps(&self, track: &Track) -> bool {
        if self.hide_explicit && track.explicit {
            return false;
        }
        if self.collaborations && track.artist_refs.len() < 2 {
            return false;
        }
        if self.playable && !track.playable {
            return false;
        }
        if self
            .year
            .is_some_and(|bounds| track.year <= 0 || !within(track.year as f32, bounds))
            || self.popularity.is_some_and(|bounds| {
                track.popularity == 0 || !within(track.popularity as f32, bounds)
            })
        {
            return false;
        }

        match self.duration {
            Some((low, high)) => {
                let seconds = track.duration.as_secs_f32();
                seconds >= low - 0.5 && seconds <= high + 0.5
            }
            None => true,
        }
    }
}

fn within(value: f32, (low, high): (f32, f32)) -> bool {
    value >= low - 0.5 && value <= high + 0.5
}

#[cfg(test)]
mod tests {
    use super::TrackSieve;
    use crate::shared::tracks::fixture::track;

    #[test]
    fn duration_bounds_are_inclusive() {
        let sieve = TrackSieve {
            duration: Some((60., 180.)),
            ..TrackSieve::default()
        };

        assert!(sieve.active());
        assert!(sieve.keeps(&track(60, false, true)));
        assert!(sieve.keeps(&track(180, false, true)));
        assert!(sieve.keeps(&track(120, false, true)));
        assert!(!sieve.keeps(&track(59, false, true)));
        assert!(!sieve.keeps(&track(181, false, true)));
    }

    #[test]
    fn every_axis_must_pass() {
        let sieve = TrackSieve {
            duration: Some((60., 180.)),
            hide_explicit: true,
            playable: true,
            favorites: false,
            downloaded: false,
            ..TrackSieve::default()
        };

        assert!(sieve.keeps(&track(120, false, true)));
        assert!(!sieve.keeps(&track(120, true, true)));
        assert!(!sieve.keeps(&track(120, false, false)));
        assert!(!sieve.keeps(&track(400, false, true)));
    }
}
