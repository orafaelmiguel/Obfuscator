use std::sync::mpsc::Sender;
use std::time::Duration;
use crate::pipeline::{PipelineContext, PipelineMessage};
use crate::pipeline::step::PipelineStep;

pub struct ParseStep;

impl ParseStep {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipelineStep for ParseStep {
    fn run(&self, ctx: &PipelineContext, tx: &Sender<PipelineMessage>) {
        let _ = tx.send(PipelineMessage::Log(format!("Parsing file: {}", ctx.input_path)));
        // Simulate work and progressive updates
        let parts = 4;
        for i in 0..parts {
            std::thread::sleep(Duration::from_millis(300));
            let progress = (i as f32 + 1.0) / (parts as f32) * 0.15; // small share of total
            let _ = tx.send(PipelineMessage::Progress(progress));
        }

        // For PoC, we can attempt a quick file read and report size
        match std::fs::metadata(&ctx.input_path) {
            Ok(meta) => {
                let size = meta.len();
                let _ = tx.send(PipelineMessage::Log(format!("Parsed: size = {} bytes", size)));
            }
            Err(e) => {
                let _ = tx.send(PipelineMessage::Log(format!("Parsed: failed to read metadata: {}", e)));
            }
        }
    }
}
