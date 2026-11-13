//! Short H.264 clips, such as a song's Canvas, decoded a frame at a time so they can loop forever
//! without holding every frame in memory. The MP4 is read once into compressed samples, and each
//! pass over them runs a fresh decoder from the first frame.

use std::collections::VecDeque;
use std::io::Cursor;
use std::time::Duration;

use anyhow::{Context as _, Result, bail};
use mp4::{MediaType, Mp4Reader, TrackType};
use openh264::OpenH264API;
use openh264::decoder::{DecodedYUV, Decoder, DecoderConfig, Flush};
use openh264::formats::YUVSource as _;

/// The start code that opens every NAL unit the decoder is fed.
const START: [u8; 4] = [0, 0, 0, 1];
/// The longest a frame is ever held, so a clip with a broken timescale still moves.
const LONGEST: Duration = Duration::from_millis(250);
/// The shortest a frame is held, so a clip claiming a silly frame rate cannot spin a core.
const SHORTEST: Duration = Duration::from_millis(8);

/// A clip held in memory as its compressed samples, ready to be decoded from the start any number
/// of times.
pub struct Clip {
    samples: Vec<Sample>,
}

struct Sample {
    /// The sample's NAL units, each behind a start code.
    units: Vec<u8>,
    duration: Duration,
}

/// One decoded picture, its pixels in the BGRA order GPUI images take.
pub struct Frame {
    pub bgra: Vec<u8>,
    pub width: u32,
    pub height: u32,
    /// How long the picture stays up before the next one.
    pub duration: Duration,
}

/// One pass over a clip, from its first frame to its last.
pub struct Frames<'a> {
    clip: &'a Clip,
    decoder: Decoder,
    at: usize,
    /// The pictures the decoder still held once every sample was fed, in display order.
    tail: Option<VecDeque<Frame>>,
}

impl Clip {
    /// Reads the first H.264 track out of an MP4 file. The samples are copied out, so `bytes`
    /// is only borrowed.
    pub fn open(bytes: &[u8]) -> Result<Self> {
        let size = bytes.len() as u64;
        let mut reader =
            Mp4Reader::read_header(Cursor::new(bytes), size).context("cannot read the mp4")?;
        let Some((&id, track)) = reader.tracks().iter().find(|(_, track)| {
            track.track_type().ok() == Some(TrackType::Video)
                && track.media_type().ok() == Some(MediaType::H264)
        }) else {
            bail!("the clip has no H.264 track");
        };

        let mut header = Vec::new();
        for set in [
            track.sequence_parameter_set()?,
            track.picture_parameter_set()?,
        ] {
            header.extend_from_slice(&START);
            header.extend_from_slice(set);
        }
        let timescale = track.timescale().max(1);
        let count = reader.sample_count(id)?;

        let mut samples = Vec::with_capacity(count as usize);
        for index in 1..=count {
            let Some(sample) = reader.read_sample(id, index)? else {
                continue;
            };
            let seconds = f64::from(sample.duration) / f64::from(timescale);
            // Store the parameter sets with the first packet once, rather than rebuild
            // that packet every time the clip loops.
            annex_b(&sample.bytes, &mut header)?;
            samples.push(Sample {
                units: std::mem::take(&mut header),
                duration: Duration::from_secs_f64(seconds).clamp(SHORTEST, LONGEST),
            });
        }
        if samples.is_empty() {
            bail!("the clip has no frames");
        }
        Ok(Self { samples })
    }

    /// Starts a pass from the first frame. The caller starts another once this one runs dry.
    pub fn frames(&self) -> Result<Frames<'_>> {
        // Flushing after every packet breaks streams with B-frames, whose pictures come out of
        // order, so the decoder keeps them until it has them all and the pass drains it at the end.
        let config = DecoderConfig::new().flush_after_decode(Flush::NoFlush);
        let decoder = Decoder::with_api_config(OpenH264API::from_source(), config)
            .context("cannot start the H.264 decoder")?;
        Ok(Frames {
            clip: self,
            decoder,
            at: 0,
            tail: None,
        })
    }
}

impl Iterator for Frames<'_> {
    type Item = Result<Frame>;

    fn next(&mut self) -> Option<Self::Item> {
        // A sample can hold no picture of its own, so keep feeding until one comes out.
        while let Some(sample) = self.clip.samples.get(self.at) {
            self.at += 1;
            let decoded = match self.decoder.decode(&sample.units) {
                Ok(decoded) => decoded,
                Err(error) => return Some(Err(error).context("cannot decode a frame")),
            };
            if let Some(picture) = decoded {
                return Some(Ok(frame(&picture, sample.duration)));
            }
        }

        if self.tail.is_none() {
            let duration = self
                .clip
                .samples
                .last()
                .map_or(SHORTEST, |sample| sample.duration);
            let tail = match self.decoder.flush_remaining() {
                Ok(pictures) => pictures
                    .iter()
                    .map(|picture| frame(picture, duration))
                    .collect(),
                Err(error) => return Some(Err(error).context("cannot drain the decoder")),
            };
            self.tail = Some(tail);
        }
        self.tail.as_mut()?.pop_front().map(Ok)
    }
}

/// Copies a decoded picture out as BGRA.
fn frame(picture: &DecodedYUV<'_>, duration: Duration) -> Frame {
    let (width, height) = picture.dimensions();
    let mut bgra = vec![0; width * height * 4];
    picture.write_rgba8(&mut bgra);
    for pixel in bgra.as_chunks_mut::<4>().0 {
        pixel.swap(0, 2);
    }
    Frame {
        bgra,
        width: width as u32,
        height: height as u32,
        duration,
    }
}

/// Turns an MP4 sample's length-prefixed NAL units into start-code ones.
fn annex_b(sample: &[u8], units: &mut Vec<u8>) -> Result<()> {
    units.reserve(sample.len());
    let mut rest = sample;
    while rest.len() >= 4 {
        let length = u32::from_be_bytes([rest[0], rest[1], rest[2], rest[3]]) as usize;
        let unit = rest
            .get(4..4 + length)
            .context("a NAL unit runs past its sample")?;
        units.extend_from_slice(&START);
        units.extend_from_slice(unit);
        rest = &rest[4 + length..];
    }
    Ok(())
}
