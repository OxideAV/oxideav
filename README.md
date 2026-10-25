# oxideav

Central aggregator crate for oxideav — re-exports and a prebuilt registry

Part of the [oxideav](https://github.com/OxideAV/oxideav-workspace) framework — a pure-Rust media transcoding and streaming stack. Codec, container, and filter crates are implemented from the spec (no C codec libraries linked or wrapped, no `*-sys` crates). Optional hardware-engine crates (`oxideav-videotoolbox` / `-audiotoolbox` / `-vaapi` / `-vdpau` / `-nvidia` / `-vulkan-video`) bridge to OS APIs via runtime `dlsym`; pass `--no-hwaccel` (or omit the `hwaccel` feature) to opt out.

## Usage

```toml
[dependencies]
oxideav = "0.0.1"
```

## Decoding on every core

Codecs run **single-threaded until they are handed a thread budget**.
`oxideav::open_decoder` / `oxideav::open_encoder` build the preferred
implementation from a `RuntimeContext` with an
`ExecutionContext::auto()` budget (every core) already announced:

```rust,ignore
let mut ctx = oxideav::RuntimeContext::new();
oxideav_meta::register_all(&mut ctx);
let mut dec = oxideav::open_decoder(&ctx, &stream.params)?;
```

If you build a codec any other way (`ctx.codecs.first_decoder(..)`,
`oxideav::pipeline::make_decoder_with`, a codec crate's own factory),
add the one line before the first packet:

```rust,ignore
decoder.set_execution_context(&oxideav::ExecutionContext::auto());
```

`ExecutionContext::with_threads(n)` caps the workers,
`ExecutionContext::serial()` asks for one; output is identical at every
budget. The `oxideav::pipeline` helpers (`transcode_simple*`,
`Pipeline`, `Executor`) announce a budget themselves.

## License

MIT — see [LICENSE](https://github.com/OxideAV/oxideav-workspace/blob/master/LICENSE).
