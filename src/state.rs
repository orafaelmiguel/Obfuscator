use std::sync::mpsc::Receiver;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::pipeline::PipelineMsg;

pub enum AppState {
    Login,
    Dashboard,
}

pub struct ObscuraState {
    pub state: AppState,
    pub email: String,
    pub password: String,

    pub counter: i32,
    pub logs: Vec<String>,

    pub selected_file: Option<String>,
    pub encrypt_strings: bool,
    pub obfuscate_functions: bool,

    pub processing: bool,
    pub progress: f32,
    pub pipeline_rx: Option<Receiver<PipelineMsg>>,
}

impl ObscuraState {
    pub fn new() -> Self {
        Self {
            state: AppState::Login,
            email: String::new(),
            password: String::new(),
            counter: 0,
            logs: Vec::new(),
            selected_file: None,
            encrypt_strings: true,
            obfuscate_functions: true,
            processing: false,
            progress: 0.0,
            pipeline_rx: None,
        }
    }

    pub fn push_log<S: Into<String>>(&mut self, msg: S) {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or_default();
        let entry = format!("[{}] {}", ts, msg.into());
        self.logs.push(entry);
        if self.logs.len() > 2000 {
            let remove = self.logs.len() - 2000;
            self.logs.drain(0..remove);
        }
    }

    /// Polls messages from the pipeline receiver and updates state/logs accordingly.
    pub fn poll_pipeline_messages(&mut self) {
        if let Some(rx) = self.pipeline_rx.take() {
            for msg in rx.try_iter() {
                match msg {
                    PipelineMsg::Log(s) => self.push_log(s),
                    PipelineMsg::Progress(p) => {
                        self.progress = p.clamp(0.0, 1.0);
                    }
                    PipelineMsg::Done(output_path) => {
                        self.push_log(format!("Pipeline finished. Output: {}", output_path.display()));
                        self.processing = false;
                        self.progress = 1.0;
                    }
                    PipelineMsg::Error(e) => {
                        self.push_log(format!("Pipeline error: {}", e));
                        self.processing = false;
                    }
                }
            }

            if self.processing {
                self.pipeline_rx = Some(rx);
            }
        }
    }
}
