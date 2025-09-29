use eframe::egui;
use crate::app::{Obscura, AppState};

pub fn show_login(ui: &mut egui::Ui, app: &mut Obscura) {
    ui.heading("Login");
    ui.separator();

    ui.label("Email:");
    ui.text_edit_singleline(&mut app.email);

    ui.label("Senha:");
    ui.add(egui::TextEdit::singleline(&mut app.password).password(true));

    if ui.button("Entrar").clicked() {
        // Mock: qualquer email/senha funciona
        app.state = AppState::Dashboard;
    }
}
