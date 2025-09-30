use crate::state::{ObscuraState};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum PipelineMessage {
    Log(String),
    Progress(f32),
    Done(String),   // output file path
    Error(String),
}

/// Fake pipeline that simulates parsing, encrypting and writing an output file
pub fn start_fake_pipeline(state: &mut ObscuraState, file_path: String) {
    let (tx, rx) = mpsc::channel();
    state.pipeline_rx = Some(rx);
    state.processing = true;

    let path_clone = file_path.clone();
    state.push_log(format!("Parsing file {} ...", path_clone));

    thread::spawn(move || {
        let _ = tx.send(PipelineMessage::Log("Parsing file...".into()));
        thread::sleep(Duration::from_secs(1));

        let _ = tx.send(PipelineMessage::Log("Encrypting strings...".into()));
        thread::sleep(Duration::from_secs(1));

        let _ = tx.send(PipelineMessage::Log("Done!".into()));

        let output = format!("{}.obscura-log", path_clone);
        std::fs::write(&output, "Pipeline completed").ok();

        let _ = tx.send(PipelineMessage::Done(output));
    });
}
