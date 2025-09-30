use std::sync::mpsc::Sender;
use std::time::Duration;
use crate::pipeline::{PipelineContext, PipelineMessage};
use crate::pipeline::step::PipelineStep;

pub struct EncryptStringsStep;

impl EncryptStringsStep {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipelineStep for EncryptStringsStep {
    fn run(&self, ctx: &PipelineContext, tx: &Sender<PipelineMessage>) {
        let _ = tx.send(PipelineMessage::Log("Encrypting strings (mock)...".into()));
        // Simulate scanning strings
        let parts = 5;
        for i in 0..parts {
            std::thread::sleep(Duration::from_millis(250));
            let progress = 0.15 + (i as f32 + 1.0) / (parts as f32) * 0.25; // next share
            let _ = tx.send(PipelineMessage::Progress(progress));
        }

        // PoC: count ASCII-like strings naive (not real PE parsing)
        let count = 0usize;
        let _ = tx.send(PipelineMessage::Log(format!("Found {} candidate strings (mock)", count)));
    }
}
