use std::sync::mpsc::Receiver;
use std::sync::{Arc, atomic::AtomicBool};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::pipeline::PipelineMessage;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppState {
    Login,
    Dashboard,
}

pub enum AuthMsg {
    Success(String /* token */),
    Error(String /* message */),
}

pub struct ObscuraState {
    pub state: AppState,
    pub email: String,
    pub password: String,

    pub logs: Vec<String>,

    pub selected_file: Option<String>,
    pub encrypt_strings: bool,
    pub obfuscate_functions: bool,

    pub processing: bool,
    pub progress: f32,
    pub pipeline_rx: Option<Receiver<PipelineMessage>>,
    pub last_output: Option<String>,
    pub cancel_flag: Option<Arc<AtomicBool>>,

    // Authentication
    pub token: Option<String>,
    pub auth_processing: bool,
    pub auth_rx: Option<Receiver<AuthMsg>>,
    pub last_auth_error: Option<String>,
}

impl ObscuraState {
    pub fn new() -> Self {
        Self {
            state: AppState::Login,
            email: String::new(),
            password: String::new(),
            logs: Vec::new(),
            selected_file: None,
            encrypt_strings: true,
            obfuscate_functions: true,
            processing: false,
            progress: 0.0,
            pipeline_rx: None,
            last_output: None,
            cancel_flag: None,
            token: None,
            auth_processing: false,
            auth_rx: None,
            last_auth_error: None,
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

    pub fn poll_pipeline_messages(&mut self) {
        if let Some(rx) = self.pipeline_rx.take() {
            for msg in rx.try_iter() {
                match msg {
                    PipelineMessage::Log(s) => self.push_log(s),
                    PipelineMessage::Progress(p) => {
                        self.progress = p.clamp(0.0, 1.0);
                    }
                    PipelineMessage::Done(output_path) => {
                        self.push_log(format!("Pipeline finished. Output: {}", output_path));
                        self.last_output = Some(output_path.clone());
                        self.processing = false;
                        self.progress = 1.0;
                        self.cancel_flag = None;
                    }
                    PipelineMessage::Error(e) => {
                        self.push_log(format!("Pipeline error: {}", e));
                        self.processing = false;
                        self.cancel_flag = None;
                    }
                }
            }

            if self.processing {
                self.pipeline_rx = Some(rx);
            }
        }
    }

    pub fn poll_auth_messages(&mut self) {
    if let Some(rx) = self.auth_rx.take() {
        for msg in rx.try_iter() {
            match msg {
                AuthMsg::Success(tok) => {
                    self.push_log("Authentication successful");
                    self.token = Some(tok);
                    self.state = AppState::Dashboard;
                    self.auth_processing = false;
                    self.last_auth_error = None; 
                }
                AuthMsg::Error(e) => {
                    self.push_log(format!("Authentication failed: {}", e));
                    self.auth_processing = false;
                    self.last_auth_error = Some(e); 
                }
            }
        }

        if self.auth_processing {
            self.auth_rx = Some(rx);
        }
    }
}

}
