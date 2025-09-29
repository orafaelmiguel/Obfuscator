use eframe::{egui, App, CreationContext, Frame};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ui;

pub enum AppState {
    Login,
    Dashboard,
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
        }
    }

    pub fn push_log<S: Into<String>>(&mut self, msg: S) {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or_default();
        let entry = format!("[{}] {}", ts, msg.into());
        self.logs.push(entry);
        // logs size (small)
        if self.logs.len() > 2000 {
            let remove = self.logs.len() - 2000;
            self.logs.drain(0..remove);
        }
    }
}

impl App for Obscura {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                AppState::Login => ui::login::show_login(ui, self),
                AppState::Dashboard => ui::dashboard::show_dashboard(ui, self),
            }
        });
    }
}
