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
        // Poll messages / atualizar estado antes de desenhar UI
        self.state.poll_auth_messages();
        self.state.poll_pipeline_messages();

        // Se qualquer operação assíncrona estiver em andamento, continue repintando
        if self.state.auth_processing || self.state.processing {
            ctx.request_repaint();
        }

        // Pegamos uma cópia do estado atual para usar dentro do closure (evita mover)
        let current_state = self.state.state;

        egui::CentralPanel::default().show(ctx, |ui| {
            match current_state {
                AppState::Login => ui::login::show_login(ui, &mut self.state),
                AppState::Dashboard => ui::dashboard::show_dashboard(ui, &mut self.state),
            }
        });
    }
}
