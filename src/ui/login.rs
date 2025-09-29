use eframe::egui;
use crate::state::ObscuraState;
use crate::auth;

pub fn show_login(ui: &mut egui::Ui, state: &mut ObscuraState) {
    ui.heading("Login");

    ui.label("Email:");
    ui.text_edit_singleline(&mut state.email);

    ui.label("Password:");
    ui.add(egui::TextEdit::singleline(&mut state.password).password(true));

    if ui.button("Login").clicked() {
        let _ = auth::login(state);
    }
}
