mod media;

pub(super) use media::OfflineFactory;

use std::time::Duration;

use music::{Encoding, Offer, PlaybackConfig, PlaybackEvents, PlaybackFactory, Quality};

use super::session::Session;

/// Initial interval for adaptive Spotify download pacing.
pub(crate) const DOWNLOAD_GAP: Duration = Duration::from_secs(2);
/// Spotify's Vorbis levels, best first.
pub(crate) const QUALITIES: &[Offer] = &[VORBIS_HIGH, VORBIS_NORMAL, VORBIS_LOW];

pub(super) const VORBIS_LOW: Offer = vorbis(Quality::Low, 96);
pub(super) const VORBIS_NORMAL: Offer = vorbis(Quality::Normal, 160);
pub(super) const VORBIS_HIGH: Offer = vorbis(Quality::High, 320);

const fn vorbis(quality: Quality, kbps: u32) -> Offer {
    Offer {
        quality,
        encoding: Encoding::Vorbis { kbps },
        up_to: false,
    }
}

/// Plays Spotify through Sonora's shared audio engine.
pub struct Factory {
    session: Session,
}

impl Factory {
    pub fn new(session: Session) -> Self {
        Self { session }
    }
}

impl PlaybackFactory for Factory {
    fn start(&self, config: PlaybackConfig) -> (Box<dyn music::Player>, Box<dyn PlaybackEvents>) {
        let audio = media::Audio {
            shelf: config.vault.as_ref().map(|vault| vault.shelf("spotify")),
            session: Some(self.session.clone()),
            quality: config.quality,
        };
        engine::start(audio, config)
    }
}
