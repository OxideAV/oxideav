//! Thin facade over [`oxideav_core`].
//!
//! `oxideav` is **no longer an aggregator** — it does NOT depend on any
//! sibling codec / container / filter / source crate. Sibling crates
//! declare their entry points via the [`oxideav_core::register!`]
//! macro; the [`oxideav-meta`](https://crates.io/crates/oxideav-meta)
//! crate's build script then enumerates the enabled sibling deps and
//! emits a `register_all(ctx)` body that calls each.
//!
//! Consumers that want "everything the framework knows about" depend
//! on `oxideav-meta` with the `all` feature (or on a slimmer
//! preset like `audio` / `video` / `image` / `subtitles` / `hwaccel`,
//! or on individual sibling crates) and call
//! [`oxideav_meta::register_all`].
//!
//! # Quick start (everything)
//!
//! ```toml
//! [dependencies]
//! oxideav = "*"
//! oxideav-meta = { version = "*", features = ["all"] }
//! ```
//!
//! ```ignore
//! let mut ctx = oxideav::RuntimeContext::new();
//! oxideav_meta::register_all(&mut ctx);
//! ```
//!
//! # Quick start (selective)
//!
//! ```toml
//! [dependencies]
//! oxideav = "*"
//! oxideav-meta = { version = "*", default-features = false, features = ["h264", "aac", "mp4"] }
//! ```
//!
//! ```ignore
//! let mut ctx = oxideav::RuntimeContext::new();
//! oxideav_meta::register_all(&mut ctx);  // h264 + aac + mp4 only
//! ```
//!
//! # Decoding on every core
//!
//! A codec runs **single-threaded until it is handed a thread budget**
//! (`oxideav-core`'s threading contract): a decoder built straight from
//! the registry (`ctx.codecs.first_decoder(..)`,
//! [`pipeline::make_decoder_with`], a crate's own `make_decoder`) keeps
//! a 1080p AV1 / VP9 / HEVC stream on one core. Either build it through
//! [`open_decoder`] / [`open_encoder`], which announce
//! [`ExecutionContext::auto`] for you, or make the call yourself before
//! the first packet:
//!
//! ```ignore
//! decoder.set_execution_context(&oxideav::ExecutionContext::auto());
//! ```
//!
//! [`ExecutionContext::with_threads`]`(n)` caps the workers,
//! [`ExecutionContext::serial`] asks for one; output is identical at
//! every budget. The helpers in [`pipeline`] (`transcode_simple*`,
//! `Pipeline`, `Executor`) do this themselves.
//!
//! ```ignore
//! let mut ctx = oxideav::RuntimeContext::new();
//! oxideav_meta::register_all(&mut ctx);
//! let mut dmx = /* open a demuxer */;
//! let mut dec = oxideav::open_decoder(&ctx, &dmx.streams()[0].params)?;
//! dec.send_packet(&dmx.next_packet()?)?;
//! let frame = dec.receive_frame()?;
//! ```

pub use oxideav_core as core;
pub use oxideav_core::{ExecutionContext, RuntimeContext};
pub use oxideav_pipeline as pipeline;
pub use oxideav_source as source;

/// Back-compat alias — historically the aggregator exposed `Registries`
/// (codecs + containers). The unified context now lives in
/// `oxideav-core` and bundles all four registries (codec / container /
/// source / filter), so consumers should prefer [`RuntimeContext`]
/// directly. The alias keeps existing call sites (`reg.codecs`,
/// `reg.containers`) compiling unchanged.
pub type Registries = RuntimeContext;

use oxideav_core::{CodecParameters, Decoder, Encoder, Result};
use oxideav_pipeline::CodecPreferences;

/// Build the preferred decoder for `params` from `ctx`'s codec
/// registry (the same choice as [`pipeline::make_decoder`]: highest
/// priority implementation first, falling back to the next one when a
/// factory refuses), with an [`ExecutionContext::auto`] thread budget
/// already announced.
pub fn open_decoder(ctx: &RuntimeContext, params: &CodecParameters) -> Result<Box<dyn Decoder>> {
    open_decoder_with(
        ctx,
        params,
        &CodecPreferences::default(),
        &ExecutionContext::auto(),
    )
}

/// [`open_decoder`] under explicit codec preferences (e.g.
/// `no_hardware`) and thread budget.
pub fn open_decoder_with(
    ctx: &RuntimeContext,
    params: &CodecParameters,
    prefs: &CodecPreferences,
    budget: &ExecutionContext,
) -> Result<Box<dyn Decoder>> {
    let mut decoder = oxideav_pipeline::make_decoder_with(&ctx.codecs, params, prefs)?;
    decoder.set_execution_context(budget);
    Ok(decoder)
}

/// Build the preferred encoder for `params` from `ctx`'s codec
/// registry, with an [`ExecutionContext::auto`] thread budget already
/// announced.
pub fn open_encoder(ctx: &RuntimeContext, params: &CodecParameters) -> Result<Box<dyn Encoder>> {
    open_encoder_with(
        ctx,
        params,
        &CodecPreferences::default(),
        &ExecutionContext::auto(),
    )
}

/// [`open_encoder`] under explicit codec preferences and thread
/// budget.
pub fn open_encoder_with(
    ctx: &RuntimeContext,
    params: &CodecParameters,
    prefs: &CodecPreferences,
    budget: &ExecutionContext,
) -> Result<Box<dyn Encoder>> {
    let mut encoder = oxideav_pipeline::make_encoder_with(&ctx.codecs, params, prefs)?;
    encoder.set_execution_context(budget);
    Ok(encoder)
}
