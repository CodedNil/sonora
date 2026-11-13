//! Symphonia decoding shared by every provider. The small Source adapter keeps HE-AAC and Opus
//! registration in Sonora while Rodio stays on a published release.

use std::io::{Read, Seek, SeekFrom};
use std::marker::PhantomData;
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use rodio::{ChannelCount, SampleRate, Source, source::SeekError};
use symphonia::core::audio::{Channels, SampleBuffer, SignalSpec};
use symphonia::core::codecs::{CodecRegistry, Decoder as Codec, DecoderOptions};
use symphonia::core::errors::{Error, Result};
use symphonia::core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia::core::io::{MediaSource, MediaSourceStream};
use symphonia::core::probe::Hint;
use tracing::debug;

static CODECS: LazyLock<CodecRegistry> = LazyLock::new(|| {
    let mut codecs = CodecRegistry::new();
    symphonia::default::register_enabled_codecs(&mut codecs);
    codecs.register_all::<symphonia_adapter_libopus::OpusDecoder>();
    codecs.register_all::<crate::aac::Aac>();
    codecs
});

/// A packet decoder exposed as a Rodio source, with seeking and format changes handled by Symphonia.
pub(crate) struct Decoder<R> {
    format: Box<dyn FormatReader>,
    codec: Box<dyn Codec>,
    track: u32,
    samples: SampleBuffer<f32>,
    cursor: usize,
    spec: SignalSpec,
    duration: Option<Duration>,
    reader: PhantomData<R>,
}

/// The stream information providers already know, forwarded to Symphonia's demuxer.
pub(crate) struct Builder<R> {
    data: Option<R>,
    length: Option<u64>,
    seekable: bool,
    gapless: bool,
    hint: Hint,
}

struct Input<R> {
    data: R,
    length: Option<u64>,
    seekable: bool,
}

impl<R: Read + Seek + Send + Sync + 'static> Decoder<R> {
    pub(crate) fn builder() -> Builder<R> {
        Builder {
            data: None,
            length: None,
            seekable: false,
            gapless: true,
            hint: Hint::new(),
        }
    }
}

impl<R> Decoder<R> {
    /// Advances to the next usable packet on the selected track, allowing damaged packets to be skipped.
    fn fill(&mut self) -> Result<bool> {
        loop {
            let packet = match self.format.next_packet() {
                Ok(packet) => packet,
                Err(Error::IoError(error)) if error.kind() == std::io::ErrorKind::UnexpectedEof => {
                    return Ok(false);
                }
                Err(Error::ResetRequired) => {
                    let track = self
                        .format
                        .tracks()
                        .iter()
                        .find(|track| track.id == self.track)
                        .ok_or(Error::Unsupported("the selected audio track disappeared"))?;
                    self.codec = CODECS.make(&track.codec_params, &DecoderOptions::default())?;
                    continue;
                }
                Err(error) => return Err(error),
            };
            if packet.track_id() != self.track {
                continue;
            }
            let decoded = match self.codec.decode(&packet) {
                Ok(decoded) => decoded,
                Err(Error::DecodeError(_)) => continue,
                Err(error) => return Err(error),
            };
            if decoded.frames() == 0 {
                continue;
            }
            if self.spec != *decoded.spec()
                || self.samples.capacity() < decoded.capacity() * decoded.spec().channels.count()
            {
                self.spec = *decoded.spec();
                self.samples = SampleBuffer::new(decoded.capacity() as u64, self.spec);
            }
            self.samples.copy_interleaved_ref(decoded);
            self.cursor = 0;
            return Ok(true);
        }
    }
}

impl<R: Read + Seek + Send + Sync + 'static> Builder<R> {
    pub(crate) fn with_data(mut self, data: R) -> Self {
        self.data = Some(data);
        self
    }

    pub(crate) fn with_byte_len(mut self, length: u64) -> Self {
        self.length = Some(length);
        self
    }

    pub(crate) fn with_seekable(mut self, seekable: bool) -> Self {
        self.seekable = seekable;
        self
    }

    pub(crate) fn with_gapless(mut self, gapless: bool) -> Self {
        self.gapless = gapless;
        self
    }

    pub(crate) fn build(self) -> Result<Decoder<R>> {
        let data = self
            .data
            .ok_or(Error::Unsupported("the audio stream is missing"))?;
        let input = Input {
            data,
            length: self.length,
            seekable: self.seekable,
        };
        let media = MediaSourceStream::new(Box::new(input), Default::default());
        let options = FormatOptions {
            enable_gapless: self.gapless,
            ..Default::default()
        };
        let format = symphonia::default::get_probe()
            .format(&self.hint, media, &options, &Default::default())?
            .format;
        let (track, codec) = format
            .tracks()
            .iter()
            .find_map(|track| {
                CODECS
                    .make(&track.codec_params, &DecoderOptions::default())
                    .ok()
                    .map(|codec| (track, codec))
            })
            .ok_or(Error::Unsupported("no supported audio track"))?;
        let duration = track
            .codec_params
            .time_base
            .zip(track.codec_params.n_frames)
            .map(|(base, frames)| Duration::from(base.calc_time(frames)))
            .filter(|duration| !duration.is_zero());
        let spec = SignalSpec::new(
            track.codec_params.sample_rate.unwrap_or(48_000),
            track.codec_params.channels.unwrap_or(Channels::FRONT_LEFT),
        );
        let track = track.id;
        let mut decoder = Decoder {
            format,
            codec,
            track,
            samples: SampleBuffer::new(0, spec),
            cursor: 0,
            spec,
            duration,
            reader: PhantomData,
        };
        decoder.fill()?;
        Ok(decoder)
    }
}

impl<R: Read> Read for Input<R> {
    fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
        self.data.read(bytes)
    }
}

impl<R: Seek> Seek for Input<R> {
    fn seek(&mut self, at: SeekFrom) -> std::io::Result<u64> {
        self.data.seek(at)
    }
}

impl<R: Read + Seek + Send + Sync> MediaSource for Input<R> {
    fn is_seekable(&self) -> bool {
        self.seekable
    }
    fn byte_len(&self) -> Option<u64> {
        self.length
    }
}

impl<R> Iterator for Decoder<R> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.samples.len() {
            match self.fill() {
                Ok(true) => {}
                Ok(false) => return None,
                Err(error) => {
                    debug!("decode: the audio stream ended: {error}");
                    return None;
                }
            }
        }
        let sample = self.samples.samples()[self.cursor];
        self.cursor += 1;
        Some(sample)
    }
}

impl<R> Source for Decoder<R> {
    fn current_span_len(&self) -> Option<usize> {
        Some(self.samples.len().saturating_sub(self.cursor))
    }

    fn channels(&self) -> ChannelCount {
        ChannelCount::new(self.spec.channels.count() as u16).expect("decoded audio has channels")
    }

    fn sample_rate(&self) -> SampleRate {
        SampleRate::new(self.spec.rate).expect("decoded audio has a sample rate")
    }

    fn total_duration(&self) -> Option<Duration> {
        self.duration
    }

    fn try_seek(&mut self, at: Duration) -> std::result::Result<(), SeekError> {
        let seek = |this: &mut Self| -> Result<()> {
            let at = this.duration.map_or(at, |duration| at.min(duration));
            let channel = this.cursor % this.spec.channels.count();
            let base = this
                .codec
                .codec_params()
                .time_base
                .ok_or(Error::Unsupported(
                    "sample-accurate seeking needs a time base",
                ))?;
            let landed = this.format.seek(
                SeekMode::Accurate,
                SeekTo::Time {
                    time: at.into(),
                    track_id: Some(this.track),
                },
            )?;
            this.codec.reset();
            this.cursor = this.samples.len();
            let skipped = base.calc_time(landed.required_ts.saturating_sub(landed.actual_ts));
            let frames = ((skipped.seconds as f64 + skipped.frac) * f64::from(this.spec.rate))
                .round() as usize;
            for _ in 0..frames
                .saturating_mul(this.spec.channels.count())
                .saturating_add(channel)
            {
                if this.next().is_none() {
                    break;
                }
            }
            Ok(())
        };
        seek(self).map_err(|error| SeekError::Other(Arc::new(error)))
    }
}
