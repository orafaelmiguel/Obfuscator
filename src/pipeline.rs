use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

#[derive(Debug)]
pub enum PipelineMsg {
    Log(String),
    Progress(f32),
    Done(PathBuf),
    Error(String),
}

/// Starts a mock pipeline in a background thread.
/// Returns an `mpsc::Receiver` to listen for pipeline messages.
pub fn start_pipeline_mock(input_path: PathBuf) -> Receiver<PipelineMsg> {
    let (tx, rx): (Sender<PipelineMsg>, Receiver<PipelineMsg>) = mpsc::channel();

    thread::spawn(move || {
        let steps = vec![
            "Parsing file...",
            "Encrypting strings...",
            "Obfuscating functions...",
            "Rebuilding binary...",
        ];

        for (i, step) in steps.iter().enumerate() {
            if tx.send(PipelineMsg::Log(step.to_string())).is_err() {
                return;
            }

            let progress = (i + 1) as f32 / steps.len() as f32;
            if tx.send(PipelineMsg::Progress(progress)).is_err() {
                return;
            }

            thread::sleep(Duration::from_secs(1));
        }

        // Fake output file
        let out_path = input_path.with_extension("obscura-log");
        match File::create(&out_path) {
            Ok(mut f) => {
                let _ = writeln!(f, "Pipeline finished successfully (mock)");
                let _ = tx.send(PipelineMsg::Log("Output file written".into()));
                let _ = tx.send(PipelineMsg::Done(out_path));
            }
            Err(e) => {
                let _ = tx.send(PipelineMsg::Error(format!(
                    "Failed to write output: {}",
                    e
                )));
            }
        }
    });

    rx
}
