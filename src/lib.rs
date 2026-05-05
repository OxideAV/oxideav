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
/// `register(&mut RuntimeContext)` entry point.
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

    macro_rules! enable {
        ($feat:literal, $name:literal, $crat:ident) => {
            #[cfg(feature = $feat)]
            {
                trace($name);
                $crat::register(&mut ctx);
            }
        };
    }

    enable!("basic", "basic", oxideav_basic);
    enable!("ogg", "ogg", oxideav_ogg);
    enable!("vorbis", "vorbis", oxideav_vorbis);
    enable!("opus", "opus", oxideav_opus);
    enable!("flac", "flac", oxideav_flac);
    enable!("mkv", "mkv", oxideav_mkv);
    enable!("mp4", "mp4", oxideav_mp4);
    enable!("avi", "avi", oxideav_avi);
    enable!("flv", "flv", oxideav_flv);
    enable!("iff", "iff", oxideav_iff);
    enable!("amiga_mod", "amiga_mod", oxideav_mod);
    enable!("s3m", "s3m", oxideav_s3m);
    enable!("mp1", "mp1", oxideav_mp1);
    enable!("mp2", "mp2", oxideav_mp2);
    enable!("mp3", "mp3", oxideav_mp3);
    enable!("mjpeg", "mjpeg", oxideav_mjpeg);
    enable!("mpeg1video", "mpeg1video", oxideav_mpeg12video);
    enable!("aac", "aac", oxideav_aac);
    enable!("ac3", "ac3", oxideav_ac3);
    enable!("ac4", "ac4", oxideav_ac4);
    enable!("celt", "celt", oxideav_celt);
    enable!("g711", "g711", oxideav_g711);
    enable!("g722", "g722", oxideav_g722);
    enable!("adpcm", "adpcm", oxideav_adpcm);
    enable!("g7231", "g7231", oxideav_g7231);
    enable!("g728", "g728", oxideav_g728);
    enable!("g729", "g729", oxideav_g729);
    enable!("ilbc", "ilbc", oxideav_ilbc);
    enable!("gsm", "gsm", oxideav_gsm);
    enable!("speex", "speex", oxideav_speex);
    enable!("mpeg4video", "mpeg4video", oxideav_mpeg4video);
    enable!("msmpeg4", "msmpeg4", oxideav_msmpeg4);
    enable!("theora", "theora", oxideav_theora);
    enable!("vp9", "vp9", oxideav_vp9);
    enable!("h265", "h265", oxideav_h265);
    enable!("h266", "h266", oxideav_h266);
    enable!("h264", "h264", oxideav_h264);
    enable!("h263", "h263", oxideav_h263);
    enable!("h261", "h261", oxideav_h261);
    enable!("vp8", "vp8", oxideav_vp8);
    enable!("vp6", "vp6", oxideav_vp6);
    enable!("webp", "webp", oxideav_webp);
    enable!("gif", "gif", oxideav_gif);
    enable!("png", "png", oxideav_png);
    enable!("pbm", "pbm", oxideav_pbm);
    enable!("pdf", "pdf", oxideav_pdf);
    enable!("svg", "svg", oxideav_svg);
    enable!("amv", "amv", oxideav_amv);
    enable!("subtitle", "subtitle", oxideav_subtitle);
    enable!("sub_image", "sub_image", oxideav_sub_image);
    enable!("ass", "ass", oxideav_ass);
    enable!("av1", "av1", oxideav_av1);
    enable!("ffv1", "ffv1", oxideav_ffv1);
    enable!("dirac", "dirac", oxideav_dirac);
    enable!("prores", "prores", oxideav_prores);
    enable!("jpegxl", "jpegxl", oxideav_jpegxl);
    enable!("jpeg2000", "jpeg2000", oxideav_jpeg2000);
    enable!("avif", "avif", oxideav_avif);
    enable!("audio_filter", "audio_filter", oxideav_audio_filter);
    enable!("image_filter", "image_filter", oxideav_image_filter);
    enable!("http", "http", oxideav_http);
    enable!("generator", "generator", oxideav_generator);

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
