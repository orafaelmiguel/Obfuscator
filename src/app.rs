use eframe::{egui, App, CreationContext, Frame};
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use crate::ui;

pub enum AppState {
    Login,
    Dashboard,
}

pub enum PipelineMessage {
    Log(String),
    Progress(f32),
    Done(PathBuf),
    Error(String),
}

pub struct Obscura {
    pub state: AppState,
    pub email: String,
    pub password: String,

    // OBF camps
    pub logs: Vec<String>,
    pub selected_file: Option<String>,
    pub encrypt_strings: bool,
    pub obfuscate_functions: bool,

    //pipeline_tx: Option<mpsc::Sender<PipelineMessage>>,
    pub processing: bool,
    pub progress: f32,
    pub pipeline_rx: Option<Receiver<PipelineMessage>>
}

impl Obscura {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
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
                        self.push_log(format!("Pipeline finalizado. Arquivo gerado: {}", output_path.display()));
                        self.processing = false;
                        self.progress = 1.0;
                        self.selected_file = Some(output_path.to_string_lossy().into_owned());
                    }
                    PipelineMessage::Error(e) => {
                        self.push_log(format!("Erro no pipeline: {}", e));
                        self.processing = false;
                    }
                }
            }

            if self.processing {
                self.pipeline_rx = Some(rx);
            }
        }
    }

    pub fn start_pipeline_mock(&mut self) {
        if self.processing {
            self.push_log("Pipeline já em execução");
            return;
        }

        let (tx, rx) = mpsc::channel::<PipelineMessage>();
        self.pipeline_rx = Some(rx);
        self.processing = true;
        self.progress = 0.0;

        let selected = self.selected_file.clone();
        let encrypt = self.encrypt_strings;
        let obfuscate = self.obfuscate_functions;

        thread::spawn(move || {
            let send = |tx: &mpsc::Sender<PipelineMessage>, m: PipelineMessage| {
                let _ = tx.send(m); 
            };

            // step 1
            send(&tx, PipelineMessage::Log("Parsing arquivo...".into()));
            std::thread::sleep(std::time::Duration::from_millis(700));
            send(&tx, PipelineMessage::Progress(0.15));

            // step 2
            if let Some(ref path) = selected {
                send(&tx, PipelineMessage::Log(format!("Arquivo detectado: {}", path)));
            } else {
                send(&tx, PipelineMessage::Log("Nenhum arquivo selecionado; usando input simulado".into()));
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
            send(&tx, PipelineMessage::Progress(0.3));

            // step 3
            if encrypt {
                send(&tx, PipelineMessage::Log("Encrypting strings...".into()));
                std::thread::sleep(std::time::Duration::from_millis(900));
                send(&tx, PipelineMessage::Progress(0.55));
            }

            // step 4
            if obfuscate {
                send(&tx, PipelineMessage::Log("Obfuscating functions...".into()));
                std::thread::sleep(std::time::Duration::from_millis(900));
                send(&tx, PipelineMessage::Progress(0.75));
            }

            // step 5
            send(&tx, PipelineMessage::Log("Finalizando e reconstruindo binário (simulado)...".into()));
            std::thread::sleep(std::time::Duration::from_millis(500));
            send(&tx, PipelineMessage::Progress(0.95));

            let out_path = if let Some(ref path_str) = selected {
                let p = std::path::Path::new(path_str);
                if let Some(parent) = p.parent() {
                    parent.join("output.obscura-log.txt")
                } else {
                    std::env::temp_dir().join("output.obscura-log.txt")
                }
            } else {
                std::env::temp_dir().join("output.obscura-log.txt")
            };

            // log content
            let content = format!(
                "Obscura Defender (mock)\nsteps:\n - parsed\n - encrypt: {}\n - obfuscate: {}\n",
                encrypt, obfuscate
            );

            match std::fs::write(&out_path, content.as_bytes()) {
                Ok(_) => {
                    send(&tx, PipelineMessage::Log(format!("Arquivo de saída criado: {}", out_path.display())));
                    // final sign
                    let _ = tx.send(PipelineMessage::Done(out_path));
                }
                Err(e) => {
                    let _ = tx.send(PipelineMessage::Error(format!("Falha ao escrever arquivo de saída: {}", e)));
                }
            }
        });

        self.push_log("Pipeline mock iniciado");
    }   
}

impl App for Obscura {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.poll_pipeline_messages();

        if self.processing {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                AppState::Login => ui::login::show_login(ui, self),
                AppState::Dashboard => ui::dashboard::show_dashboard(ui, self),
            }
        });
    }
}