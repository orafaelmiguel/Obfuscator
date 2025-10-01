use crate::state::ObscuraState;
use std::sync::mpsc;
use std::thread;

mod step;
pub mod parse;
pub mod encrypt;
pub mod obfuscate;
pub mod write;

use step::PipelineStep;
use parse::ParseStep;
use encrypt::EncryptStringsStep;
use obfuscate::ObfuscateFunctionsStep;
use write::WriteOutputStep;

#[derive(Debug, Clone)]
pub enum PipelineMessage {
    Log(String),
    Progress(f32),
    Done(String),   // output file path
    Error(String),
}

pub struct PipelineContext {
    pub input_path: String,
}

impl PipelineContext {
    pub fn new(input_path: String) -> Self {
        Self { input_path }
    }
}

/// Runs the full pipeline (Parse → Encrypt → Obfuscate → WriteOutput).
pub fn start_pipeline(state: &mut ObscuraState, file_path: String) {
    let (tx, rx) = mpsc::channel();
    state.pipeline_rx = Some(rx);
    state.processing = true;
    state.progress = 0.0;

    let path_clone = file_path.clone();

    thread::spawn(move || {
        let mut ctx = PipelineContext::new(path_clone);

        // define steps sequence
        let steps: Vec<Box<dyn PipelineStep>> = vec![
            Box::new(ParseStep),
            Box::new(EncryptStringsStep),
            Box::new(ObfuscateFunctionsStep),
            Box::new(WriteOutputStep),
        ];

        let total = steps.len();
        for (i, step) in steps.into_iter().enumerate() {
            let progress = (i as f32) / (total as f32);
            let _ = tx.send(PipelineMessage::Progress(progress));

            if let Err(e) = step.run(&mut ctx, &tx) {
                let _ = tx.send(PipelineMessage::Error(e.to_string()));
                return;
            }
        }

        // final progress and Done already sent by WriteOutputStep
        let _ = tx.send(PipelineMessage::Progress(1.0));
    });
}
