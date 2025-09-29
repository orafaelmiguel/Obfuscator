use eframe::{egui, App, CreationContext, Frame};

use crate::state::{ObscuraState, AppState};
use crate::ui;

pub struct Obscura {
    pub state: ObscuraState,
}

impl Obscura {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            state: ObscuraState::new(),
        }
    }
}

impl App for Obscura {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state.state {
                AppState::Login => ui::login::show_login(ui, &mut self.state),
                AppState::Dashboard => ui::dashboard::show_dashboard(ui, &mut self.state),
            }
        });
    }
}
