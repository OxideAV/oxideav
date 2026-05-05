//! Thin facade over [`oxideav_core`].
//!
//! `oxideav` is **no longer an aggregator** — it does NOT depend on any
//! sibling codec / container / filter / source crate. Instead, each
//! sibling crate self-registers into [`oxideav_core::REGISTRARS`] (a
//! linkme distributed slice) at link time. Consumers pull whichever
//! siblings they want into their own `[dependencies]`, then call
//! [`with_all_features`] to materialise everything that ended up
//! linked.
//!
//! For convenience there is a sibling [`oxideav-format-all`](https://crates.io/crates/oxideav-format-all)
//! virtual crate that depends on every codec / container / filter /
//! source crate the framework knows about — `oxideav-cli`, `oxideplay`,
//! and similar "want everything" consumers depend on it.
//!
//! # Quick start (everything)
//!
//! ```toml
//! [dependencies]
//! oxideav = "*"
//! oxideav-format-all = "*"
//! ```
//!
//! ```ignore
//! let ctx = oxideav::with_all_features();
//! ```
//!
//! # Quick start (selective)
//!
//! ```toml
//! [dependencies]
//! oxideav = "*"
//! oxideav-h264 = "*"
//! oxideav-aac = "*"
//! oxideav-mp4 = "*"
//! ```
//!
//! ```ignore
//! // Only h264 + aac + mp4 self-register; the slice walker sees them.
//! let ctx = oxideav::with_all_features();
//! ```

pub use oxideav_core as core;
pub use oxideav_core::RuntimeContext;
pub use oxideav_pipeline as pipeline;
pub use oxideav_source as source;

/// Build a [`RuntimeContext`] populated with every sibling crate that's
/// linked into the binary.
///
/// Delegates to [`RuntimeContext::with_all_features`], which walks the
/// linkme distributed slice [`oxideav_core::REGISTRARS`] populated by
/// each sibling's [`oxideav_core::register!`] macro call.
pub fn with_all_features() -> RuntimeContext {
    RuntimeContext::with_all_features()
}

/// Build a [`RuntimeContext`] populated with every sibling crate, while
/// invoking `trace(name)` immediately before each registrar fires.
/// Useful for diagnosing register-time hangs.
pub fn with_all_features_traced<F: FnMut(&str)>(trace: F) -> RuntimeContext {
    RuntimeContext::with_all_features_traced(trace)
}

/// Build a [`RuntimeContext`] populated with every sibling crate whose
/// name `filter` returns `true` for. Used by CLIs to implement opt-outs
/// like `--no-hwaccel` (skip `videotoolbox` / `audiotoolbox`).
pub fn with_all_features_filtered<F: FnMut(&str) -> bool>(filter: F) -> RuntimeContext {
    RuntimeContext::with_all_features_filtered(filter)
}

/// Back-compat alias — historically the aggregator exposed `Registries`
/// (codecs + containers). The unified context now lives in
/// `oxideav-core` and bundles all four registries (codec / container /
/// source / filter), so consumers should prefer [`RuntimeContext`]
/// directly. The alias keeps existing call sites (`reg.codecs`,
/// `reg.containers`) compiling unchanged.
pub type Registries = RuntimeContext;

/// Convenience trait so call sites can write
/// `RuntimeContext::with_all_features()` (matching the historical
/// `Registries::with_all_features()` shape) without depending on the
/// free function's path.
pub trait RuntimeContextExt {
    /// See [`crate::with_all_features`].
    fn with_all_features() -> Self;
}

impl RuntimeContextExt for RuntimeContext {
    fn with_all_features() -> Self {
        with_all_features()
    }
}
