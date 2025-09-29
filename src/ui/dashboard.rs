use eframe::egui;
use crate::app::Obscura;

pub fn show_dashboard(ui: &mut egui::Ui, app: &mut Obscura) {
    ui.heading("Dashboard");
    ui.separator();

    ui.label(format!("Bem-vindo, {}", app.email));

    ui.horizontal(|ui| {
        if ui.button("−").clicked() {
            app.counter -= 1;
        }
        ui.label(app.counter.to_string());
        if ui.button("+").clicked() {
            app.counter += 1;
        }
    });
}
