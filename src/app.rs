use eframe::{egui, App, CreationContext, Frame};

use crate::ui;

pub enum AppState {
    Login,
    Dashboard,
}

pub struct Obscura {
    pub state: AppState,
    pub email: String,
    pub password: String,
    pub counter: i32, 
}

impl Obscura {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            state: AppState::Login,
            email: String::new(),
            password: String::new(),
            counter: 0,
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
