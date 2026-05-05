//! Aggregator crate for oxideav.
//!
//! Depend on this crate to pull in codecs and containers from the wider
//! oxideav ecosystem, gated by Cargo features. Each format crate maps to
//! exactly one feature here:
//!
//! ```toml
//! [dependencies]
//! oxideav = { version = "*", features = ["basic", "ogg", "vorbis", "flac"] }
//! ```

// `oxideav::codec` / `oxideav::container` previously aliased the
// shim crates; those have been archived. The trait + registry types
// they hosted (Decoder / Encoder / CodecRegistry / Demuxer / Muxer /
// ContainerRegistry / ReadSeek / WriteSeek) now live in oxideav-core
// directly. Consumers should import from `oxideav::core` (the alias
// below), or from `oxideav_core` with a `pub use oxideav_core::*;`.
pub use oxideav_core as core;
pub use oxideav_pipeline as pipeline;
pub use oxideav_source as source;

pub use oxideav_core::RuntimeContext;

#[cfg(feature = "generator")]
pub use oxideav_generator as generator;
#[cfg(feature = "http")]
pub use oxideav_http as http;

#[cfg(feature = "aac")]
pub use oxideav_aac as aac;
#[cfg(feature = "ac3")]
pub use oxideav_ac3 as ac3;
#[cfg(feature = "ac4")]
pub use oxideav_ac4 as ac4;
#[cfg(feature = "adpcm")]
pub use oxideav_adpcm as adpcm;
#[cfg(feature = "amv")]
pub use oxideav_amv as amv;
#[cfg(feature = "ass")]
pub use oxideav_ass as ass;
#[cfg(feature = "audio_filter")]
pub use oxideav_audio_filter as audio_filter;
#[cfg(feature = "av1")]
pub use oxideav_av1 as av1;
#[cfg(feature = "avi")]
pub use oxideav_avi as avi;
#[cfg(feature = "avif")]
pub use oxideav_avif as avif;
#[cfg(feature = "basic")]
pub use oxideav_basic as basic;
#[cfg(feature = "celt")]
pub use oxideav_celt as celt;
#[cfg(feature = "dirac")]
pub use oxideav_dirac as dirac;
#[cfg(feature = "ffv1")]
pub use oxideav_ffv1 as ffv1;
#[cfg(feature = "flac")]
pub use oxideav_flac as flac;
#[cfg(feature = "flv")]
pub use oxideav_flv as flv;
#[cfg(feature = "g711")]
pub use oxideav_g711 as g711;
#[cfg(feature = "g722")]
pub use oxideav_g722 as g722;
#[cfg(feature = "g7231")]
pub use oxideav_g7231 as g7231;
#[cfg(feature = "g728")]
pub use oxideav_g728 as g728;
#[cfg(feature = "g729")]
pub use oxideav_g729 as g729;
#[cfg(feature = "gif")]
pub use oxideav_gif as gif;
#[cfg(feature = "gsm")]
pub use oxideav_gsm as gsm;
#[cfg(feature = "h261")]
pub use oxideav_h261 as h261;
#[cfg(feature = "h263")]
pub use oxideav_h263 as h263;
#[cfg(feature = "h264")]
pub use oxideav_h264 as h264;
#[cfg(feature = "h265")]
pub use oxideav_h265 as h265;
#[cfg(feature = "h266")]
pub use oxideav_h266 as h266;
#[cfg(feature = "iff")]
pub use oxideav_iff as iff;
#[cfg(feature = "ilbc")]
pub use oxideav_ilbc as ilbc;
#[cfg(feature = "image_filter")]
pub use oxideav_image_filter as image_filter;
#[cfg(feature = "jpeg2000")]
pub use oxideav_jpeg2000 as jpeg2000;
#[cfg(feature = "jpegxl")]
pub use oxideav_jpegxl as jpegxl;
#[cfg(feature = "mjpeg")]
pub use oxideav_mjpeg as mjpeg;
#[cfg(feature = "mkv")]
pub use oxideav_mkv as mkv;
#[cfg(feature = "amiga_mod")]
pub use oxideav_mod as amiga_mod;
#[cfg(feature = "mp1")]
pub use oxideav_mp1 as mp1;
#[cfg(feature = "mp2")]
pub use oxideav_mp2 as mp2;
#[cfg(feature = "mp3")]
pub use oxideav_mp3 as mp3;
#[cfg(feature = "mp4")]
pub use oxideav_mp4 as mp4;
#[cfg(feature = "mpeg1video")]
pub use oxideav_mpeg12video as mpeg12video;
#[cfg(feature = "mpeg4video")]
pub use oxideav_mpeg4video as mpeg4video;
#[cfg(feature = "msmpeg4")]
pub use oxideav_msmpeg4 as msmpeg4;
#[cfg(feature = "ogg")]
pub use oxideav_ogg as ogg;
#[cfg(feature = "opus")]
pub use oxideav_opus as opus;
#[cfg(feature = "pbm")]
pub use oxideav_pbm as pbm;
#[cfg(feature = "pdf")]
pub use oxideav_pdf as pdf;
#[cfg(feature = "pixfmt")]
pub use oxideav_pixfmt as pixfmt;
#[cfg(feature = "png")]
pub use oxideav_png as png;
#[cfg(feature = "prores")]
pub use oxideav_prores as prores;
#[cfg(feature = "raster")]
pub use oxideav_raster as raster;
#[cfg(feature = "s3m")]
pub use oxideav_s3m as s3m;
#[cfg(feature = "scene")]
pub use oxideav_scene as scene;
#[cfg(feature = "speex")]
pub use oxideav_speex as speex;
#[cfg(feature = "sub_image")]
pub use oxideav_sub_image as sub_image;
#[cfg(feature = "subtitle")]
pub use oxideav_subtitle as subtitle;
#[cfg(feature = "svg")]
pub use oxideav_svg as svg;
#[cfg(feature = "theora")]
pub use oxideav_theora as theora;
#[cfg(feature = "vorbis")]
pub use oxideav_vorbis as vorbis;
#[cfg(feature = "vp6")]
pub use oxideav_vp6 as vp6;
#[cfg(feature = "vp8")]
pub use oxideav_vp8 as vp8;
#[cfg(feature = "vp9")]
pub use oxideav_vp9 as vp9;
#[cfg(feature = "webp")]
pub use oxideav_webp as webp;

/// Back-compat alias — historically the aggregator exposed
/// `Registries` (codecs + containers). The unified context now lives
/// in `oxideav-core` and bundles all four registries (codec /
/// container / source / filter), so consumers should prefer
/// [`RuntimeContext`] directly. The alias keeps existing call sites
/// (`reg.codecs`, `reg.containers`) compiling unchanged.
pub type Registries = RuntimeContext;

/// Build a [`RuntimeContext`] populated with every codec / container /
/// source / filter that's enabled at build time. Each `cfg(feature =
/// "X")` block delegates to the sibling crate's unified
/// `register(&mut RuntimeContext)` entry point where it exists, and
/// falls back to the historical `register_codecs` /
/// `register_containers` / `register(&mut CodecRegistry)` /
/// `register(codecs, containers)` shapes otherwise.
pub fn with_all_features() -> RuntimeContext {
    with_all_features_traced(|name| log::debug!(target: "oxideav::register", "{name}"))
}

/// Same as [`with_all_features`], but invokes `trace(name)` immediately
/// before each crate's `register()` call. Useful for debugging startup
/// hangs — when `register()` blocks, the last name passed to `trace`
/// names the offending crate.
///
/// The default path (`with_all_features`) passes a no-op closure that
/// the compiler inlines and erases, so there's no runtime cost when the
/// trace isn't wanted.
pub fn with_all_features_traced<F: FnMut(&str)>(mut trace: F) -> RuntimeContext {
    #[allow(unused_mut)]
    let mut ctx = RuntimeContext::new();

    // Built-in source drivers (file://). Always installed.
    trace("source");
    oxideav_source::register(&mut ctx);

    #[cfg(feature = "basic")]
    {
        trace("basic");
        oxideav_basic::register_codecs(&mut ctx.codecs);
        oxideav_basic::register_containers(&mut ctx.containers);
    }
    #[cfg(feature = "ogg")]
    {
        trace("ogg");
        oxideav_ogg::register(&mut ctx.containers);
    }
    #[cfg(feature = "vorbis")]
    {
        trace("vorbis");
        oxideav_vorbis::register(&mut ctx.codecs);
    }
    #[cfg(feature = "opus")]
    {
        trace("opus");
        oxideav_opus::register(&mut ctx.codecs);
    }
    #[cfg(feature = "flac")]
    {
        trace("flac");
        oxideav_flac::register_codecs(&mut ctx.codecs);
        oxideav_flac::register_containers(&mut ctx.containers);
    }
    #[cfg(feature = "mkv")]
    {
        trace("mkv");
        oxideav_mkv::register(&mut ctx.containers);
    }
    #[cfg(feature = "mp4")]
    {
        trace("mp4");
        oxideav_mp4::register(&mut ctx.containers);
    }
    #[cfg(feature = "avi")]
    {
        trace("avi");
        oxideav_avi::register(&mut ctx.containers);
    }
    #[cfg(feature = "flv")]
    {
        trace("flv");
        oxideav_flv::register(&mut ctx.containers);
    }
    #[cfg(feature = "iff")]
    {
        trace("iff");
        oxideav_iff::register(&mut ctx.containers);
    }
    #[cfg(feature = "amiga_mod")]
    {
        trace("amiga_mod");
        oxideav_mod::register_codecs(&mut ctx.codecs);
        oxideav_mod::register_containers(&mut ctx.containers);
    }
    #[cfg(feature = "s3m")]
    {
        trace("s3m");
        oxideav_s3m::register_codecs(&mut ctx.codecs);
        oxideav_s3m::register_containers(&mut ctx.containers);
    }
    #[cfg(feature = "mp1")]
    {
        trace("mp1");
        oxideav_mp1::register(&mut ctx.codecs);
    }
    #[cfg(feature = "mp2")]
    {
        trace("mp2");
        oxideav_mp2::register(&mut ctx.codecs);
    }
    #[cfg(feature = "mp3")]
    {
        trace("mp3");
        oxideav_mp3::register_codecs(&mut ctx.codecs);
        oxideav_mp3::register_containers(&mut ctx.containers);
    }
    #[cfg(feature = "mjpeg")]
    {
        trace("mjpeg");
        oxideav_mjpeg::register(&mut ctx.codecs);
        oxideav_mjpeg::register_containers(&mut ctx.containers);
    }
    #[cfg(feature = "mpeg1video")]
    {
        trace("mpeg1video");
        oxideav_mpeg12video::register(&mut ctx.codecs);
    }
    #[cfg(feature = "aac")]
    {
        trace("aac");
        oxideav_aac::register(&mut ctx.codecs);
    }
    #[cfg(feature = "ac3")]
    {
        trace("ac3");
        oxideav_ac3::register(&mut ctx.codecs);
    }
    #[cfg(feature = "ac4")]
    {
        trace("ac4");
        oxideav_ac4::register(&mut ctx.codecs);
    }
    #[cfg(feature = "celt")]
    {
        trace("celt");
        oxideav_celt::register(&mut ctx.codecs);
    }
    #[cfg(feature = "g711")]
    {
        trace("g711");
        oxideav_g711::register(&mut ctx.codecs);
    }
    #[cfg(feature = "g722")]
    {
        trace("g722");
        oxideav_g722::register(&mut ctx.codecs);
    }
    #[cfg(feature = "adpcm")]
    {
        trace("adpcm");
        oxideav_adpcm::register(&mut ctx.codecs);
    }
    #[cfg(feature = "g7231")]
    {
        trace("g7231");
        oxideav_g7231::register(&mut ctx.codecs);
    }
    #[cfg(feature = "g728")]
    {
        trace("g728");
        oxideav_g728::register(&mut ctx.codecs);
    }
    #[cfg(feature = "g729")]
    {
        trace("g729");
        oxideav_g729::register(&mut ctx.codecs);
    }
    #[cfg(feature = "ilbc")]
    {
        trace("ilbc");
        oxideav_ilbc::register(&mut ctx.codecs);
    }
    #[cfg(feature = "gsm")]
    {
        trace("gsm");
        oxideav_gsm::register(&mut ctx.codecs);
    }
    #[cfg(feature = "speex")]
    {
        trace("speex");
        oxideav_speex::register(&mut ctx.codecs);
    }
    #[cfg(feature = "mpeg4video")]
    {
        trace("mpeg4video");
        oxideav_mpeg4video::register(&mut ctx.codecs);
    }
    #[cfg(feature = "msmpeg4")]
    {
        trace("msmpeg4");
        oxideav_msmpeg4::register(&mut ctx.codecs);
    }
    #[cfg(feature = "theora")]
    {
        trace("theora");
        oxideav_theora::register(&mut ctx.codecs);
    }
    #[cfg(feature = "vp9")]
    {
        trace("vp9");
        oxideav_vp9::register(&mut ctx.codecs);
    }
    #[cfg(feature = "h265")]
    {
        trace("h265");
        oxideav_h265::register(&mut ctx.codecs);
    }
    #[cfg(feature = "h266")]
    {
        trace("h266");
        oxideav_h266::register(&mut ctx.codecs);
    }
    #[cfg(feature = "h264")]
    {
        trace("h264");
        oxideav_h264::register(&mut ctx.codecs);
    }
    #[cfg(feature = "h263")]
    {
        trace("h263");
        oxideav_h263::register(&mut ctx.codecs);
    }
    #[cfg(feature = "h261")]
    {
        trace("h261");
        oxideav_h261::register(&mut ctx.codecs);
    }
    #[cfg(feature = "vp8")]
    {
        trace("vp8");
        oxideav_vp8::register(&mut ctx.codecs, &mut ctx.containers);
    }
    #[cfg(feature = "vp6")]
    {
        trace("vp6");
        oxideav_vp6::register(&mut ctx.codecs);
    }
    #[cfg(feature = "webp")]
    {
        trace("webp");
        oxideav_webp::register(&mut ctx.codecs, &mut ctx.containers);
    }
    #[cfg(feature = "gif")]
    {
        trace("gif");
        oxideav_gif::register(&mut ctx.codecs, &mut ctx.containers);
    }
    #[cfg(feature = "png")]
    {
        trace("png");
        oxideav_png::register(&mut ctx.codecs, &mut ctx.containers);
    }
    #[cfg(feature = "pbm")]
    {
        trace("pbm");
        oxideav_pbm::register(&mut ctx.codecs, &mut ctx.containers);
    }
    #[cfg(feature = "pdf")]
    {
        trace("pdf");
        oxideav_pdf::register(&mut ctx.codecs, &mut ctx.containers);
    }
    #[cfg(feature = "svg")]
    {
        trace("svg");
        oxideav_svg::register(&mut ctx.codecs, &mut ctx.containers);
    }
    #[cfg(feature = "amv")]
    {
        trace("amv");
        oxideav_amv::register(&mut ctx.codecs, &mut ctx.containers);
    }
    #[cfg(feature = "subtitle")]
    {
        trace("subtitle");
        oxideav_subtitle::register(&mut ctx.codecs, &mut ctx.containers);
    }
    #[cfg(feature = "sub_image")]
    {
        trace("sub_image");
        oxideav_sub_image::register(&mut ctx.codecs, &mut ctx.containers);
    }
    #[cfg(feature = "ass")]
    {
        trace("ass");
        oxideav_ass::register(&mut ctx.codecs, &mut ctx.containers);
    }
    #[cfg(feature = "av1")]
    {
        trace("av1");
        oxideav_av1::register(&mut ctx.codecs);
    }
    #[cfg(feature = "ffv1")]
    {
        trace("ffv1");
        oxideav_ffv1::register(&mut ctx.codecs);
    }
    #[cfg(feature = "dirac")]
    {
        trace("dirac");
        oxideav_dirac::register(&mut ctx.codecs);
    }
    #[cfg(feature = "prores")]
    {
        trace("prores");
        oxideav_prores::register(&mut ctx.codecs);
    }
    #[cfg(feature = "jpegxl")]
    {
        trace("jpegxl");
        oxideav_jpegxl::register(&mut ctx.codecs);
        oxideav_jpegxl::register_containers(&mut ctx.containers);
    }
    #[cfg(feature = "jpeg2000")]
    {
        trace("jpeg2000");
        oxideav_jpeg2000::register(&mut ctx.codecs);
        oxideav_jpeg2000::register_containers(&mut ctx.containers);
    }
    #[cfg(feature = "avif")]
    {
        trace("avif");
        oxideav_avif::register(&mut ctx.codecs);
    }
    #[cfg(feature = "audio_filter")]
    {
        trace("audio_filter");
        oxideav_audio_filter::register(&mut ctx);
    }
    #[cfg(feature = "image_filter")]
    {
        trace("image_filter");
        oxideav_image_filter::register(&mut ctx);
    }
    #[cfg(feature = "http")]
    {
        trace("http");
        oxideav_http::register(&mut ctx.sources);
    }
    #[cfg(feature = "generator")]
    {
        trace("generator");
        oxideav_generator::register_source(&mut ctx.sources);
        oxideav_generator::register_filters(&mut ctx);
    }

    ctx
}

impl RuntimeContextExt for RuntimeContext {
    fn with_all_features() -> Self {
        with_all_features()
    }
}

/// Convenience trait so call sites can write
/// `RuntimeContext::with_all_features()` (matching the historical
/// `Registries::with_all_features()` shape) without depending on the
/// free function's path.
pub trait RuntimeContextExt {
    /// See [`crate::with_all_features`].
    fn with_all_features() -> Self;
}
