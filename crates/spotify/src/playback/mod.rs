mod audio;

use std::time::Duration;

use music::{PlaybackConfig, PlaybackEvents, PlaybackFactory};

use super::session::Session;

use music::Offer;

pub(super) const QUALITIES: &[Offer] = audio::QUALITIES;

/// Initial interval for adaptive Spotify download pacing.
pub(super) const DOWNLOAD_GAP: Duration = Duration::from_secs(2);

/// Plays Spotify's web audio through Sonora's shared engine.
pub(super) struct Factory {
    session: Session,
}

impl Factory {
    pub(super) fn new(session: Session) -> Self {
        Self { session }
    }
}

impl PlaybackFactory for Factory {
    fn start(&self, config: PlaybackConfig) -> (Box<dyn music::Player>, Box<dyn PlaybackEvents>) {
        engine::start(
            audio::Audio {
                session: self.session.clone(),
                quality: config.quality,
            },
            config,
        )
    }
}
