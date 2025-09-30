use std::sync::mpsc::Sender;
use std::time::Duration;
use crate::pipeline::{PipelineContext, PipelineMessage};
use crate::pipeline::step::PipelineStep;

pub struct ObfuscateFunctionsStep;

impl ObfuscateFunctionsStep {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipelineStep for ObfuscateFunctionsStep {
    fn run(&self, _ctx: &PipelineContext, tx: &Sender<PipelineMessage>) {
        let _ = tx.send(PipelineMessage::Log("Obfuscating functions (mock)...".into()));
        let parts = 4;
        for i in 0..parts {
            std::thread::sleep(Duration::from_millis(300));
            let progress = 0.45 + (i as f32 + 1.0) / (parts as f32) * 0.25;
            let _ = tx.send(PipelineMessage::Progress(progress));
        }
        let _ = tx.send(PipelineMessage::Log("Obfuscation step completed (mock)".into()));
    }
}
