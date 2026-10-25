//! `open_decoder` / `open_encoder` hand the codec a thread budget
//! before it sees any data (default auto, explicit via `_with`).

use std::sync::atomic::{AtomicUsize, Ordering};

use oxideav::core::registry::CodecInfo;
use oxideav::core::{
    CodecCapabilities, CodecId, CodecParameters, Decoder, Encoder, Error, Frame, Packet, Result,
};
use oxideav::pipeline::CodecPreferences;
use oxideav::{ExecutionContext, RuntimeContext};

static DEC: AtomicUsize = AtomicUsize::new(0);
static ENC: AtomicUsize = AtomicUsize::new(0);

struct Probe(CodecParameters);

impl Decoder for Probe {
    fn codec_id(&self) -> &CodecId {
        &self.0.codec_id
    }
    fn send_packet(&mut self, _: &Packet) -> Result<()> {
        Ok(())
    }
    fn receive_frame(&mut self) -> Result<Frame> {
        Err(Error::NeedMore)
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
    fn reset(&mut self) -> Result<()> {
        Ok(())
    }
    fn set_execution_context(&mut self, ctx: &ExecutionContext) {
        DEC.store(ctx.threads, Ordering::SeqCst);
    }
}

impl Encoder for Probe {
    fn codec_id(&self) -> &CodecId {
        &self.0.codec_id
    }
    fn output_params(&self) -> &CodecParameters {
        &self.0
    }
    fn send_frame(&mut self, _: &Frame) -> Result<()> {
        Ok(())
    }
    fn receive_packet(&mut self) -> Result<Packet> {
        Err(Error::NeedMore)
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
    fn set_execution_context(&mut self, ctx: &ExecutionContext) {
        ENC.store(ctx.threads, Ordering::SeqCst);
    }
}

fn make_dec(p: &CodecParameters) -> Result<Box<dyn Decoder>> {
    Ok(Box::new(Probe(p.clone())))
}
fn make_enc(p: &CodecParameters) -> Result<Box<dyn Encoder>> {
    Ok(Box::new(Probe(p.clone())))
}

#[test]
fn open_codecs_carry_a_budget() {
    let mut ctx = RuntimeContext::new();
    ctx.codecs.register(
        CodecInfo::new(CodecId::new("probe_budget"))
            .capabilities(
                CodecCapabilities::video("probe_budget")
                    .with_decode()
                    .with_encode(),
            )
            .decoder(make_dec)
            .encoder(make_enc),
    );
    let params = CodecParameters::video(CodecId::new("probe_budget"));
    let auto = ExecutionContext::auto().threads;

    oxideav::open_decoder(&ctx, &params).unwrap();
    oxideav::open_encoder(&ctx, &params).unwrap();
    assert_eq!(DEC.load(Ordering::SeqCst), auto);
    assert_eq!(ENC.load(Ordering::SeqCst), auto);

    let prefs = CodecPreferences::default();
    let three = ExecutionContext::with_threads(3);
    oxideav::open_decoder_with(&ctx, &params, &prefs, &three).unwrap();
    oxideav::open_encoder_with(&ctx, &params, &prefs, &three).unwrap();
    assert_eq!(DEC.load(Ordering::SeqCst), 3);
    assert_eq!(ENC.load(Ordering::SeqCst), 3);
}
