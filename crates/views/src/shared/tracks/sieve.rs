use music::Track;
use ui::{Filter, FilterChange, FlagAxis};

#[derive(Clone, Copy, Default, PartialEq)]
pub(crate) struct TrackSieve {
    pub year: Option<(f32, f32)>,
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
            || self.duration.is_some()
            || self.hide_explicit
            || self.playable
            || self.favorites
            || self.downloaded
            || self.collaborations
    }

    /// The flag definitions used by every track table; unavailable library/download controls stay out.
    pub(super) fn flags(&self, catalog: bool, downloads: bool) -> Vec<Filter> {
        let mut flags = vec![
            ("filter-hide-explicit", self.hide_explicit),
            ("filter-playable", self.playable),
            ("filter-collaborations", self.collaborations),
        ];
        if catalog {
            flags.push(("filter-favorites", self.favorites));
        }
        if downloads {
            flags.push(("filter-downloaded", self.downloaded));
        }
        flags
            .into_iter()
            .map(|(key, on)| {
                Filter::Flag(FlagAxis {
                    key,
                    label: i18n::lookup(key, None),
                    on,
                })
            })
            .collect()
    }

    /// Applies every track filter in one place, including both range controls.
    pub(super) fn change(&mut self, change: FilterChange) -> bool {
        match change {
            FilterChange::Range("filter-year", value) => self.year = Some(value),
            FilterChange::Range("filter-duration", value) => self.duration = Some(value),
            FilterChange::Flag(key, value) => {
                let flag = match key {
                    "filter-hide-explicit" => &mut self.hide_explicit,
                    "filter-playable" => &mut self.playable,
                    "filter-collaborations" => &mut self.collaborations,
                    "filter-favorites" => &mut self.favorites,
                    "filter-downloaded" => &mut self.downloaded,
                    _ => return false,
                };
                *flag = value;
            }
            FilterChange::Reset => *self = Self::default(),
            _ => return false,
        }
        true
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
        if let Some((low, high)) = self.year
            && (track.year <= 0
                || (track.year as f32) < low - 0.5
                || (track.year as f32) > high + 0.5)
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
