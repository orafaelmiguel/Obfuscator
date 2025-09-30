use std::sync::mpsc::Sender;
use std::time::Duration;
use crate::pipeline::{PipelineContext, PipelineMessage};
use crate::pipeline::step::PipelineStep;

pub struct WriteOutputStep;

impl WriteOutputStep {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipelineStep for WriteOutputStep {
    fn run(&self, ctx: &PipelineContext, tx: &Sender<PipelineMessage>) {
        let _ = tx.send(PipelineMessage::Log("Writing output (mock)...".into()));
        std::thread::sleep(Duration::from_millis(500));
        // small progress bump
        let _ = tx.send(PipelineMessage::Progress(0.9));
        std::thread::sleep(Duration::from_millis(200));
        let _ = tx.send(PipelineMessage::Log(format!("Output prepared (mock) for {}", ctx.input_path)));
    }
}
