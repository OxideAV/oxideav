//! `StreamDecoder` and the in-band `Frame::StreamParams` event: the
//! simple pull consumes it and flags the next frame, the opt-in pull
//! yields it as-is, and each change is reported exactly once whether a
//! decoder emits the event or only moves its generation counter.

use oxideav::StreamDecoder;
use oxideav::core::registry::CodecInfo;
use oxideav::core::{
    CodecCapabilities, CodecId, CodecParameters, Decoder, Error, Frame, Packet, PixelFormat,
    Result, TimeBase, VideoFrame, VideoPlane,
};
use oxideav::{ExecutionContext, RuntimeContext};
use std::collections::VecDeque;

const EMITTING: &str = "spe_emit";
const SILENT: &str = "spe_silent";

/// A scripted decoder. Every byte of a packet is one step: `0` is a
/// picture at the current size, a non-zero byte switches the size to
/// that many pixels square. A switch after the first picture bumps the
/// generation and — when `emit` — returns a `StreamParams` event, so a
/// switch byte at the end of a packet leaves the event followed by
/// `NeedMore`. Before the first picture a switch just sets the initial
/// size (no event, no bump).
struct Scripted {
    id: CodecId,
    emit: bool,
    steps: VecDeque<u8>,
    side: u32,
    generation: u64,
    pictures: u64,
    events: usize,
}

impl Scripted {
    fn new(id: &str, emit: bool) -> Self {
        Self {
            id: CodecId::new(id),
            emit,
            steps: VecDeque::new(),
            side: 4,
            generation: 0,
            pictures: 0,
            events: 0,
        }
    }
    fn params(&self) -> CodecParameters {
        let mut p = CodecParameters::video(self.id.clone());
        p.width = Some(self.side);
        p.height = Some(self.side);
        p.pixel_format = Some(PixelFormat::Gray8);
        p
    }
}

impl Decoder for Scripted {
    fn codec_id(&self) -> &CodecId {
        &self.id
    }
    fn send_packet(&mut self, p: &Packet) -> Result<()> {
        self.steps.extend(p.data.iter().copied());
        Ok(())
    }
    fn receive_frame(&mut self) -> Result<Frame> {
        loop {
            match self.steps.pop_front().ok_or(Error::NeedMore)? {
                0 => {
                    let side = self.side as usize;
                    self.pictures += 1;
                    return Ok(Frame::Video(VideoFrame {
                        pts: Some(self.pictures as i64),
                        planes: vec![VideoPlane {
                            stride: side,
                            data: vec![0; side * side],
                        }],
                    }));
                }
                s if u32::from(s) == self.side => {}
                s => {
                    self.side = u32::from(s);
                    if self.pictures > 0 {
                        self.generation = self.generation.wrapping_add(1);
                        if self.emit {
                            self.events += 1;
                            return Ok(Frame::StreamParams(Box::new(self.params())));
                        }
                    }
                }
            }
        }
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
    fn reset(&mut self) -> Result<()> {
        self.steps.clear();
        Ok(())
    }
    fn output_params(&self) -> Option<CodecParameters> {
        Some(self.params())
    }
    fn output_params_generation(&self) -> u64 {
        self.generation
    }
    fn output_pixel_format(&self) -> Option<PixelFormat> {
        Some(PixelFormat::Gray8)
    }
}

fn pkt(steps: &[u8]) -> Packet {
    Packet::new(0, TimeBase::new(1, 25), steps.to_vec())
}

fn stream(id: &str) -> CodecParameters {
    let mut p = CodecParameters::video(CodecId::new(id));
    p.width = Some(4);
    p.height = Some(6); // a coded size; the decoder corrects it
    p.bit_rate = Some(1234);
    p
}

/// What a simple-API consumer sees per media frame: (side of the
/// picture, side in `stream_params`, `params_changed`, generation).
type Seen = Vec<(usize, u32, bool, u64)>;

/// Drive `dec` with `packets` through the simple pull, asserting no
/// event ever surfaces.
fn simple(dec: &mut StreamDecoder, packets: &[&[u8]]) -> Seen {
    let mut seen = Vec::new();
    for p in packets {
        dec.send_packet(&pkt(p)).unwrap();
        loop {
            let frame = match dec.receive_frame() {
                Ok(f) => f,
                Err(Error::NeedMore) => break,
                Err(e) => panic!("{e:?}"),
            };
            let Frame::Video(v) = frame else {
                panic!("receive_frame surfaced a non-media frame: {frame:?}")
            };
            seen.push((
                v.planes[0].stride,
                dec.stream_params().width.unwrap(),
                dec.params_changed(),
                dec.params_generation(),
            ));
        }
    }
    seen
}

/// The same stream: 3 pictures at 4, switch to 8 (event at a packet
/// end, so `NeedMore` sits between it and its picture), 2 pictures,
/// switch to 2 mid-packet, 2 pictures.
const SCRIPT: &[&[u8]] = &[&[0], &[0, 0], &[8], &[0, 0], &[2, 0], &[0]];

fn expected() -> Seen {
    vec![
        // Coded 6 already corrected to 4 at construction: no change.
        (4, 4, false, 0),
        (4, 4, false, 0),
        (4, 4, false, 0),
        (8, 8, true, 1),
        (8, 8, false, 1),
        (2, 2, true, 2),
        (2, 2, false, 2),
    ]
}

#[test]
fn simple_pull_consumes_the_event_and_flags_the_next_frame() {
    let mut dec = StreamDecoder::new(Box::new(Scripted::new(EMITTING, true)), stream(EMITTING));
    assert_eq!(simple(&mut dec, SCRIPT), expected());
    let p = dec.stream_params();
    assert_eq!(
        (p.width, p.height, p.bit_rate),
        (Some(2), Some(2), Some(1234))
    );
}

#[test]
fn counter_only_decoder_reports_the_same_changes() {
    let mut dec = StreamDecoder::new(Box::new(Scripted::new(SILENT, false)), stream(SILENT));
    assert_eq!(simple(&mut dec, SCRIPT), expected());
}

#[test]
fn simple_pull_flags_a_change_whose_event_preceded_need_more() {
    let mut dec = StreamDecoder::new(Box::new(Scripted::new(EMITTING, true)), stream(EMITTING));
    dec.send_packet(&pkt(&[0, 16])).unwrap();
    assert!(dec.receive_frame().unwrap().pts().is_some());
    // The event is consumed; no picture follows yet.
    assert!(matches!(dec.receive_frame(), Err(Error::NeedMore)));
    // The parameters already describe the picture to come…
    assert_eq!(dec.stream_params().width, Some(16));
    let generation = dec.params_generation();
    // …and the change is flagged when it comes, once.
    dec.send_packet(&pkt(&[0, 0])).unwrap();
    let Frame::Video(v) = dec.receive_frame().unwrap() else {
        unreachable!()
    };
    assert_eq!(v.planes[0].stride, 16);
    assert!(dec.params_changed());
    assert_eq!(dec.params_generation(), generation + 1);
    dec.receive_frame().unwrap();
    assert!(!dec.params_changed());
}

/// The opt-in pull: (kind, side in `stream_params`, `params_changed`).
fn raw(dec: &mut StreamDecoder, packets: &[&[u8]]) -> Vec<(&'static str, u32, bool)> {
    let mut seen = Vec::new();
    for p in packets {
        dec.send_packet(&pkt(p)).unwrap();
        loop {
            let frame = match dec.receive_frame_or_event() {
                Ok(f) => f,
                Err(Error::NeedMore) => break,
                Err(e) => panic!("{e:?}"),
            };
            let side = dec.stream_params().width.unwrap();
            let kind = match &frame {
                Frame::StreamParams(ev) => {
                    // The event's parameters are what the stream now says.
                    assert_eq!(ev.width, Some(side));
                    "event"
                }
                Frame::Video(v) => {
                    assert_eq!(v.planes[0].stride as u32, side);
                    "frame"
                }
                other => panic!("{other:?}"),
            };
            seen.push((kind, side, dec.params_changed()));
        }
    }
    seen
}

#[test]
fn opt_in_pull_yields_the_event_in_position() {
    let mut dec = StreamDecoder::new(Box::new(Scripted::new(EMITTING, true)), stream(EMITTING));
    assert_eq!(
        raw(&mut dec, SCRIPT),
        vec![
            ("frame", 4, false),
            ("frame", 4, false),
            ("frame", 4, false),
            ("event", 8, true),
            ("frame", 8, false),
            ("frame", 8, false),
            ("event", 2, true),
            ("frame", 2, false),
            ("frame", 2, false),
        ]
    );
    assert_eq!(dec.params_generation(), 2);
}

#[test]
fn opt_in_pull_invents_no_event_for_a_counter_only_decoder() {
    let mut dec = StreamDecoder::new(Box::new(Scripted::new(SILENT, false)), stream(SILENT));
    assert_eq!(
        raw(&mut dec, SCRIPT),
        vec![
            ("frame", 4, false),
            ("frame", 4, false),
            ("frame", 4, false),
            ("frame", 8, true),
            ("frame", 8, false),
            ("frame", 2, true),
            ("frame", 2, false),
        ]
    );
}

#[test]
fn mixing_the_two_pulls_reports_each_change_once() {
    let mut dec = StreamDecoder::new(Box::new(Scripted::new(EMITTING, true)), stream(EMITTING));
    dec.send_packet(&pkt(&[0, 8])).unwrap();
    dec.receive_frame().unwrap();
    // Event consumed by the simple pull, its picture taken by the raw one.
    assert!(matches!(dec.receive_frame(), Err(Error::NeedMore)));
    dec.send_packet(&pkt(&[0, 0])).unwrap();
    assert!(dec.receive_frame_or_event().unwrap().pts().is_some());
    assert!(dec.params_changed());
    assert!(!dec.receive_frame_or_event().unwrap().is_stream_params());
    assert!(!dec.params_changed());
    assert_eq!(dec.params_generation(), 1);
}

#[test]
fn reset_keeps_following_changes() {
    let mut dec = StreamDecoder::new(Box::new(Scripted::new(EMITTING, true)), stream(EMITTING));
    simple(&mut dec, &[&[0, 0]]);
    dec.reset().unwrap();
    // The new position is in a segment of another size.
    let seen = simple(&mut dec, &[&[8, 0, 0]]);
    assert_eq!(seen, vec![(8, 8, true, 1), (8, 8, false, 1)]);
}

fn make_emitting(_: &CodecParameters) -> Result<Box<dyn Decoder>> {
    Ok(Box::new(Scripted::new(EMITTING, true)))
}

#[test]
fn registry_opened_decoder_follows_the_event() {
    let mut ctx = RuntimeContext::new();
    ctx.codecs.register(
        CodecInfo::new(CodecId::new(EMITTING))
            .capabilities(CodecCapabilities::video(EMITTING).with_decode())
            .decoder(make_emitting),
    );
    let mut dec = StreamDecoder::open_with(
        &ctx,
        &stream(EMITTING),
        &Default::default(),
        &ExecutionContext::serial(),
    )
    .unwrap();
    // Known before the first picture.
    assert_eq!(dec.stream_params().height, Some(4));
    assert_eq!(simple(&mut dec, SCRIPT), expected());

    // The decoder underneath really emitted the events.
    dec.send_packet(&pkt(&[4, 0])).unwrap();
    assert!(
        dec.decoder_mut()
            .receive_frame()
            .unwrap()
            .is_stream_params()
    );
}
