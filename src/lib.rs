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

pub use oxideav_core as core;
pub use oxideav_core::RuntimeContext;
pub use oxideav_pipeline as pipeline;
pub use oxideav_source as source;

/// Back-compat alias — historically the aggregator exposed `Registries`
/// (codecs + containers). The unified context now lives in
/// `oxideav-core` and bundles all four registries (codec / container /
/// source / filter), so consumers should prefer [`RuntimeContext`]
/// directly. The alias keeps existing call sites (`reg.codecs`,
/// `reg.containers`) compiling unchanged.
pub type Registries = RuntimeContext;
