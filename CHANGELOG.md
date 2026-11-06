# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `StreamDecoder`: a decoder bundled with its stream's parameters.
  `stream_params()` returns the container's `CodecParameters` with the
  decoder's own report (oxideav-core 0.1.42 `Decoder::output_params`:
  emitted size, pixel / sample format, rate, channels) merged over
  them — known right after `open` when the decoder's configuration
  record states it, and kept current after every `receive_frame` by an
  `OutputParamsWatch` (one integer compare per frame);
  `params_changed()` flags the frame that changed them (a mid-stream
  switch). Crate docs and README explain why frames carry no format
  fields (stream metadata, stated once).
- `StreamDecoder` follows the in-band `Frame::StreamParams` event
  (oxideav-core 0.1.43) as well as the generation counter, so
  `stream_params()` is truthful whichever a decoder implements and each
  mid-stream change is reported exactly once:
  - `receive_frame()` (simple API) never returns the event: it merges it
    (`OutputParamsWatch::observe_frame`) and returns the next media
    frame with `params_changed()` set. When the decoder answers
    `NeedMore` between the event and that frame, the parameters already
    describe the frame to come and the change is flagged when it comes.
  - `receive_frame_or_event()` (low-level opt-in) yields the decoder's
    event as-is, in position, with `params_changed()` set on the event;
    a counter-only decoder yields no invented event (the change is
    flagged on its first new-shape frame).
  - `params_generation()`: a counter of reported changes, for comparing
    "did the shape change between these two frames" across a queue.
- Crate docs: a decoder used directly (`open_decoder`) returns its
  events from `receive_frame`; skip them with `is_stream_params()` where
  only media is wanted. Two broken / redundant intra-doc links fixed.

## [0.0.5](https://github.com/OxideAV/oxideav/compare/v0.0.4...v0.0.5) - 2026-10-25

### Other

- open_decoder / open_encoder: registry codecs with a thread budget (default auto)
- hardware-engine crates load OS APIs through dlsym, not libloading
- edition 2024, MSRV 1.95

### Added

- `open_decoder` / `open_encoder` (+ `_with` taking `CodecPreferences`
  and an `ExecutionContext`): build the preferred implementation from a
  `RuntimeContext` with a thread budget already announced — default
  `ExecutionContext::auto()`. A codec built straight from the registry
  runs single-threaded until `set_execution_context` is called; the
  crate docs and README now show that one line.
- `ExecutionContext` re-exported at the crate root.

### Changed

- edition 2024, MSRV 1.95.

## [0.0.4](https://github.com/OxideAV/oxideav/compare/v0.0.3...v0.0.4) - 2026-05-06

### Other

- reframe FFI claim — HW-engine crates use OS FFI by necessity
- drop with_all_features* (use oxideav_meta::register_all instead)
