//! A decoder bundled with the stream it decodes.
//!
//! Frames carry **no format fields by design**: a
//! [`VideoFrame`](oxideav_core::VideoFrame) holds planes and a pts, an
//! [`AudioFrame`](oxideav_core::AudioFrame) samples and a pts — no
//! width, height, pixel format, sample format, rate or channel count.
//! Those do not change from one frame to the next, so they live once in
//! the stream's [`CodecParameters`] instead of being repeated on every
//! frame.
//!
//! The container's stream parameters are only its claim — some
//! containers state no pixel layout at all, others a coded (not
//! cropped) size. The decoder knows what it emits
//! ([`Decoder::output_params`]); [`StreamDecoder`] merges that answer
//! over the container's parameters as soon as the decoder has it, keeps
//! it current through mid-stream changes (a resolution switch at a
//! keyframe, a sample-rate change in a chained stream) with one integer
//! compare per frame ([`OutputParamsWatch`]), and hands it out through
//! [`StreamDecoder::stream_params`].

use oxideav_core::{
    CodecParameters, Decoder, ExecutionContext, Frame, OutputParamsWatch, Packet, Result,
    RuntimeContext,
};
use oxideav_pipeline::CodecPreferences;

/// A decoder plus its stream's parameters, kept in step with what the
/// decoder actually emits.
///
/// [`stream_params`](Self::stream_params) describes every frame
/// [`receive_frame`](Self::receive_frame) returns: right after opening
/// it holds the container's parameters with whatever the decoder
/// already reported (from its configuration record) merged over them;
/// after each frame it describes that frame.
/// [`params_changed`](Self::params_changed) says whether the frame
/// just returned changed it (its first resolution, or a mid-stream
/// switch) — the cue to reconfigure a renderer, scaler or encoder.
///
/// ```
/// use oxideav::StreamDecoder;
/// use oxideav::core::{Demuxer, Error, Frame, Result};
///
/// fn decode_first_stream(ctx: &oxideav::RuntimeContext, dmx: &mut dyn Demuxer) -> Result<()> {
///     let stream = dmx.streams()[0].clone();
///     let mut dec = StreamDecoder::open(ctx, &stream.params)?;
///     loop {
///         match dmx.next_packet() {
///             Ok(pkt) if pkt.stream_index == stream.index => dec.send_packet(&pkt)?,
///             Ok(_) => continue,
///             Err(Error::Eof) => dec.flush()?,
///             Err(e) => return Err(e),
///         }
///         loop {
///             let frame = match dec.receive_frame() {
///                 Ok(f) => f,
///                 Err(Error::NeedMore) => break,
///                 Err(Error::Eof) => return Ok(()),
///                 Err(e) => return Err(e),
///             };
///             let p = dec.stream_params();
///             if dec.params_changed() {
///                 // First frame, or a mid-stream switch: (re)size the
///                 // output from the stream, not from the frame.
///                 println!("{:?}x{:?} {:?}", p.width, p.height, p.pixel_format);
///             }
///             if let Frame::Video(v) = &frame {
///                 let _ = (&v.planes, p.pixel_format);
///             }
///         }
///     }
/// }
/// ```
pub struct StreamDecoder {
    decoder: Box<dyn Decoder>,
    params: CodecParameters,
    watch: OutputParamsWatch,
    changed: bool,
}

impl StreamDecoder {
    /// Open the preferred decoder for `stream` ([`crate::open_decoder`]:
    /// an [`ExecutionContext::auto`] thread budget announced) and merge
    /// what it already reports into the stream parameters.
    pub fn open(ctx: &RuntimeContext, stream: &CodecParameters) -> Result<Self> {
        Ok(Self::new(crate::open_decoder(ctx, stream)?, stream.clone()))
    }

    /// [`open`](Self::open) under explicit codec preferences and
    /// thread budget ([`crate::open_decoder_with`]).
    pub fn open_with(
        ctx: &RuntimeContext,
        stream: &CodecParameters,
        prefs: &CodecPreferences,
        budget: &ExecutionContext,
    ) -> Result<Self> {
        Ok(Self::new(
            crate::open_decoder_with(ctx, stream, prefs, budget)?,
            stream.clone(),
        ))
    }

    /// Wrap a decoder built some other way. `stream` is the container's
    /// description of what it decodes; the decoder's own report is
    /// merged over it here and after every frame.
    pub fn new(decoder: Box<dyn Decoder>, stream: CodecParameters) -> Self {
        let mut s = Self {
            decoder,
            params: stream,
            watch: OutputParamsWatch::new(),
            changed: false,
        };
        s.watch.prime(&*s.decoder, &mut s.params);
        s
    }

    /// The stream's parameters: the container's, with the decoder's
    /// report (size of the frames as emitted, pixel / sample format,
    /// rate, channels, colour signal) taking precedence. Container-side
    /// fields (extradata, bit rate, tag, language, …) are kept.
    pub fn stream_params(&self) -> &CodecParameters {
        &self.params
    }

    /// Whether the last successful [`receive_frame`](Self::receive_frame)
    /// changed [`stream_params`](Self::stream_params) — its first
    /// resolution, or a mid-stream switch. One integer compare per
    /// frame while nothing changes.
    pub fn params_changed(&self) -> bool {
        self.changed
    }

    /// Feed one packet of this stream.
    pub fn send_packet(&mut self, packet: &Packet) -> Result<()> {
        self.decoder.send_packet(packet)
    }

    /// Pull the next frame; [`stream_params`](Self::stream_params)
    /// describes it on return.
    pub fn receive_frame(&mut self) -> Result<Frame> {
        let frame = self.decoder.receive_frame()?;
        self.changed = self.watch.observe(&*self.decoder, &mut self.params);
        Ok(frame)
    }

    /// Signal end of stream; remaining frames come out of
    /// [`receive_frame`](Self::receive_frame).
    pub fn flush(&mut self) -> Result<()> {
        self.decoder.flush()
    }

    /// Discard decoder state (after a seek). The parameters stay; a
    /// decoder whose shape differs at the new position reports it with
    /// the first frame returned afterwards.
    pub fn reset(&mut self) -> Result<()> {
        self.changed = false;
        self.decoder.reset()
    }

    /// The wrapped decoder.
    pub fn decoder(&self) -> &dyn Decoder {
        &*self.decoder
    }

    /// The wrapped decoder, mutably. Frames pulled through it directly
    /// bypass the watch: call [`receive_frame`](Self::receive_frame)
    /// instead to keep [`stream_params`](Self::stream_params) current.
    pub fn decoder_mut(&mut self) -> &mut dyn Decoder {
        &mut *self.decoder
    }

    /// Take the decoder and the merged stream parameters apart.
    pub fn into_parts(self) -> (Box<dyn Decoder>, CodecParameters) {
        (self.decoder, self.params)
    }
}

impl std::fmt::Debug for StreamDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamDecoder")
            .field("codec_id", self.decoder.codec_id())
            .field("params", &self.params)
            .field("changed", &self.changed)
            .finish()
    }
}
