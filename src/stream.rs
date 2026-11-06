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
//!
//! # Mid-stream changes: the event and the counter
//!
//! A decoder announces a mid-stream change of its output shape through
//! its generation counter ([`Decoder::output_params_generation`]) and —
//! if it emits it — through the in-band [`Frame::StreamParams`] event,
//! returned immediately before the first frame of the new shape. A
//! `StreamDecoder` follows both, so it is truthful whichever a decoder
//! implements, and reports each change exactly once.
//!
//! The two pull methods differ only in whether the event reaches you:
//!
//! * [`StreamDecoder::receive_frame`] — the simple API — **never**
//!   returns the event: it folds it into
//!   [`stream_params`](StreamDecoder::stream_params) and returns the
//!   next media frame. [`params_changed`](StreamDecoder::params_changed)
//!   (and a moved [`params_generation`](StreamDecoder::params_generation))
//!   says the shape changed before that frame. Code written before the
//!   event existed sees only media frames.
//! * [`StreamDecoder::receive_frame_or_event`] — the low-level opt-in —
//!   returns the decoder's events as-is, in position, for a consumer
//!   that forwards the frame stream (a filter graph, an encoder that
//!   re-configures) and wants the change in the stream itself. A
//!   decoder that only moves its counter emits no event, and none is
//!   invented: the change is flagged on the first frame of the new shape
//!   instead.

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
    /// The last successful pull changed `params`.
    changed: bool,
    /// An event was folded into `params` by `receive_frame` but the
    /// frame of the new shape has not been returned yet (the decoder
    /// answered `NeedMore` / `Eof` / an error in between).
    pending: bool,
    /// Number of changes reported so far.
    generation: u64,
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
            pending: false,
            generation: 0,
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

    /// Whether the last successful pull changed
    /// [`stream_params`](Self::stream_params) — its first resolution, or
    /// a mid-stream switch. One integer compare per frame while nothing
    /// changes.
    ///
    /// After [`receive_frame`](Self::receive_frame) it is `true` on the
    /// first frame of a new shape, whether the decoder announced the
    /// change with a [`Frame::StreamParams`] event (consumed on the way)
    /// or only through its generation counter. After
    /// [`receive_frame_or_event`](Self::receive_frame_or_event) it is
    /// `true` on the event itself (and `false` on the frame after it),
    /// or on the first frame of the new shape when the decoder emits no
    /// event.
    pub fn params_changed(&self) -> bool {
        self.changed
    }

    /// A counter of the changes reported so far: it moves exactly when
    /// [`params_changed`](Self::params_changed) is `true`. Keep the
    /// value seen with a frame and compare it with a later one to ask
    /// "did the shape change between these two" without tracking every
    /// pull — e.g. across a queue between a decode thread and a
    /// renderer. Starts at `0` — the shape known at construction (from
    /// the decoder's configuration record) is the baseline; a shape the
    /// decoder resolves only at the first frame counts as a change.
    pub fn params_generation(&self) -> u64 {
        self.generation
    }

    /// Feed one packet of this stream.
    pub fn send_packet(&mut self, packet: &Packet) -> Result<()> {
        self.decoder.send_packet(packet)
    }

    /// Pull the next **media** frame; [`stream_params`](Self::stream_params)
    /// describes it on return.
    ///
    /// A [`Frame::StreamParams`] event from the decoder is never returned
    /// here: it is merged into [`stream_params`](Self::stream_params)
    /// and the frame after it is returned, with
    /// [`params_changed`](Self::params_changed) set. If the decoder
    /// answers [`Error::NeedMore`](oxideav_core::Error::NeedMore) between
    /// the event and that frame, the error is returned as usual (the
    /// parameters then already describe the frame to come) and the change
    /// is flagged on the next frame returned. Use
    /// [`receive_frame_or_event`](Self::receive_frame_or_event) to see
    /// the event itself.
    pub fn receive_frame(&mut self) -> Result<Frame> {
        loop {
            let frame = self.decoder.receive_frame()?;
            if frame.is_stream_params() {
                self.pending |= self.absorb_event(&frame);
                continue;
            }
            let changed = self.watch.observe(&*self.decoder, &mut self.params)
                | std::mem::take(&mut self.pending);
            self.set_changed(changed);
            return Ok(frame);
        }
    }

    /// Low-level pull: the next frame **or** the decoder's
    /// [`Frame::StreamParams`] event, as-is and in position (after the
    /// last frame of the old shape, before the first of the new one).
    ///
    /// On an event, [`stream_params`](Self::stream_params) already
    /// describes the new shape (the event's parameters merged over the
    /// stream, container-side fields kept) and
    /// [`params_changed`](Self::params_changed) is `true`; the frame after
    /// it reports no second change. A decoder that announces changes only
    /// through its generation counter yields no event here — the change
    /// is flagged on the first frame of the new shape, as with
    /// [`receive_frame`](Self::receive_frame).
    ///
    /// Opt-in: forward the event to stages that understand it (filters
    /// re-plan, an encoder that cannot re-configure rejects it with
    /// [`Error::Unsupported`](oxideav_core::Error::Unsupported)), and
    /// skip it ([`Frame::is_stream_params`]) where only media is wanted.
    pub fn receive_frame_or_event(&mut self) -> Result<Frame> {
        let frame = self.decoder.receive_frame()?;
        let changed = if frame.is_stream_params() {
            self.absorb_event(&frame) | std::mem::take(&mut self.pending)
        } else {
            self.watch.observe(&*self.decoder, &mut self.params) | std::mem::take(&mut self.pending)
        };
        self.set_changed(changed);
        Ok(frame)
    }

    /// Fold an event into the parameters; `true` when that changed
    /// them. The watch then records the decoder's (already bumped)
    /// generation, so the frame after the event costs one compare and
    /// reports no second change.
    fn absorb_event(&mut self, event: &Frame) -> bool {
        let merged = self.watch.observe_frame(event, &mut self.params);
        let read = self.watch.observe(&*self.decoder, &mut self.params);
        merged | read
    }

    fn set_changed(&mut self, changed: bool) {
        self.changed = changed;
        if changed {
            self.generation = self.generation.wrapping_add(1);
        }
    }

    /// Signal end of stream; remaining frames come out of
    /// [`receive_frame`](Self::receive_frame).
    pub fn flush(&mut self) -> Result<()> {
        self.decoder.flush()
    }

    /// Discard decoder state (after a seek). The parameters stay; a
    /// decoder whose shape differs at the new position reports it with
    /// the first frame returned afterwards (by event or counter, as
    /// during playback). A change already folded in from an event whose
    /// frame never came out is still flagged on that first frame.
    pub fn reset(&mut self) -> Result<()> {
        self.changed = false;
        self.decoder.reset()
    }

    /// The wrapped decoder.
    pub fn decoder(&self) -> &dyn Decoder {
        &*self.decoder
    }

    /// The wrapped decoder, mutably. Frames pulled through it directly
    /// bypass the watch (and include the decoder's raw
    /// [`Frame::StreamParams`] events): call
    /// [`receive_frame`](Self::receive_frame) instead to keep
    /// [`stream_params`](Self::stream_params) current.
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
            .field("generation", &self.generation)
            .finish()
    }
}
