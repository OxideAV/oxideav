//! `StreamDecoder` exposes the stream parameters merged with what the
//! decoder reports, and follows a mid-stream change once.

use std::sync::atomic::{AtomicUsize, Ordering};

use oxideav::StreamDecoder;
use oxideav::core::registry::CodecInfo;
use oxideav::core::{
    CodecCapabilities, CodecId, CodecParameters, Decoder, Error, Frame, Packet, PixelFormat,
    Result, TimeBase, VideoFrame, VideoPlane,
};
use oxideav::{ExecutionContext, RuntimeContext};

const CODEC: &str = "sp_mock";
/// The first packet decoded at the new size.
const SWITCH_AT: u8 = 3;

static QUERIES: AtomicUsize = AtomicUsize::new(0);

/// Reports 4×4 Gray8 from construction, 8×8 from packet `SWITCH_AT`.
struct Mock {
    pending: Option<u8>,
    big: bool,
}

impl Decoder for Mock {
    fn codec_id(&self) -> &CodecId {
        static ID: std::sync::OnceLock<CodecId> = std::sync::OnceLock::new();
        ID.get_or_init(|| CodecId::new(CODEC))
    }
    fn send_packet(&mut self, p: &Packet) -> Result<()> {
        self.pending = Some(p.data[0]);
        Ok(())
    }
    fn receive_frame(&mut self) -> Result<Frame> {
        let n = self.pending.take().ok_or(Error::NeedMore)?;
        self.big = n >= SWITCH_AT;
        let side = if self.big { 8 } else { 4 };
        Ok(Frame::Video(VideoFrame {
            pts: Some(n as i64),
            planes: vec![VideoPlane {
                stride: side,
                data: vec![0; side * side],
            }],
        }))
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
    fn output_params(&self) -> Option<CodecParameters> {
        QUERIES.fetch_add(1, Ordering::SeqCst);
        let mut p = CodecParameters::video(CodecId::new(CODEC));
        let side = if self.big { 8 } else { 4 };
        p.width = Some(side);
        p.height = Some(side);
        p.pixel_format = Some(PixelFormat::Gray8);
        Some(p)
    }
    fn output_params_generation(&self) -> u64 {
        u64::from(self.big)
    }
    fn output_pixel_format(&self) -> Option<PixelFormat> {
        self.output_params().and_then(|p| p.pixel_format)
    }
}

fn make(_: &CodecParameters) -> Result<Box<dyn Decoder>> {
    Ok(Box::new(Mock {
        pending: None,
        big: false,
    }))
}

fn packet(n: u8) -> Packet {
    Packet::new(0, TimeBase::new(1, 25), vec![n])
}

#[test]
fn stream_params_merge_the_decoder_report_and_follow_a_change() {
    let mut ctx = RuntimeContext::new();
    ctx.codecs.register(
        CodecInfo::new(CodecId::new(CODEC))
            .capabilities(CodecCapabilities::video("sp_mock").with_decode())
            .decoder(make),
    );
    // The container's claim: a coded height, no layout.
    let mut stream = CodecParameters::video(CodecId::new(CODEC));
    stream.width = Some(4);
    stream.height = Some(6);
    stream.bit_rate = Some(777);

    let mut dec = StreamDecoder::open_with(
        &ctx,
        &stream,
        &Default::default(),
        &ExecutionContext::serial(),
    )
    .unwrap();
    // Known before the first frame, container-side fields kept.
    let p = dec.stream_params();
    assert_eq!((p.width, p.height), (Some(4), Some(4)));
    assert_eq!(p.pixel_format, Some(PixelFormat::Gray8));
    assert_eq!(p.bit_rate, Some(777));

    let before = QUERIES.load(Ordering::SeqCst);
    let mut changes = Vec::new();
    for n in 0..8u8 {
        dec.send_packet(&packet(n)).unwrap();
        let Frame::Video(v) = dec.receive_frame().unwrap() else {
            unreachable!()
        };
        if dec.params_changed() {
            changes.push(n);
        }
        // The stream describes the frame just returned.
        let w = dec.stream_params().width.unwrap() as usize;
        assert_eq!(v.planes[0].data.len(), w * w, "frame {n}");
    }
    assert_eq!(changes, vec![SWITCH_AT]);
    assert_eq!(dec.stream_params().height, Some(8));
    // Read on the first frame and at the switch — not once per frame.
    assert_eq!(QUERIES.load(Ordering::SeqCst) - before, 2);

    let (_decoder, params) = dec.into_parts();
    assert_eq!(params.width, Some(8));
}
