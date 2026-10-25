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

## Stream parameters, not frame fields

Frames carry no format fields **by design**: a `VideoFrame` holds
planes and a pts, an `AudioFrame` samples and a pts. Width, height,
pixel format, sample format, rate and channel layout are stated once,
in the stream's `CodecParameters` — not repeated on every frame.

The container's parameters are only its claim (some leave the pixel
layout out, some state a coded rather than a cropped size). The decoder
knows what it emits, and `oxideav::StreamDecoder` merges its report
over the stream parameters, keeping them current through mid-stream
changes with one integer compare per frame:

```rust,ignore
let mut dec = oxideav::StreamDecoder::open(&ctx, &stream.params)?;
dec.send_packet(&pkt)?;
let frame = dec.receive_frame()?;
let p = dec.stream_params();  // width / height / pixel_format of `frame`
if dec.params_changed() {
    // a mid-stream switch: reconfigure the renderer / scaler
}
```

## License

MIT — see [LICENSE](https://github.com/OxideAV/oxideav-workspace/blob/master/LICENSE).
