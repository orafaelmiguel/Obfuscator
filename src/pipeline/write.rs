use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::time::Duration;

use crate::pipeline::{PipelineContext, PipelineMessage};
use crate::pipeline::step::PipelineStep;

/// WriteOutputStep: creates a protected output file as a simple copy (PoC).
pub struct WriteOutputStep;

impl WriteOutputStep {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipelineStep for WriteOutputStep {
    fn run(&self, ctx: &PipelineContext, tx: &Sender<PipelineMessage>) {
        let _ = tx.send(PipelineMessage::Log("Write step (mock) started".into()));
        // small progress hint
        let _ = tx.send(PipelineMessage::Progress(0.9));
        std::thread::sleep(Duration::from_millis(150));

        // Build output path: append ".obscura-protected.exe" if input ends with .exe, otherwise add suffix
        let input_path = &ctx.input_path;
        let output_path = if input_path.to_lowercase().ends_with(".exe") {
            format!("{}.obscura-protected.exe", input_path.trim_end_matches(".exe"))
        } else {
            format!("{}.obscura-protected", input_path)
        };

        // Try to copy the file (non-destructive PoC)
        match fs::copy(&input_path, &output_path) {
            Ok(bytes) => {
                let _ = tx.send(PipelineMessage::Log(format!(
                    "Wrote output (copy, {} bytes): {}",
                    bytes, output_path
                )));
                // final progress
                let _ = tx.send(PipelineMessage::Progress(1.0));
                // Notify completion (Done)
                let _ = tx.send(PipelineMessage::Done(output_path));
            }
            Err(e) => {
                let _ = tx.send(PipelineMessage::Error(format!(
                    "Write step failed to copy file: {}",
                    e
                )));
            }
        }

        // small delay to give UI time to show final messages
        std::thread::sleep(Duration::from_millis(80));
        let _ = tx.send(PipelineMessage::Log("Write step (mock) finished".into()));
    }
}
