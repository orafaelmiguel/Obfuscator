use eframe::egui;
use eframe::egui::ScrollArea;
use crate::app::Obscura;

pub fn show_dashboard(ui: &mut egui::Ui, app: &mut Obscura) {
    ui.horizontal(|ui| {
        ui.heading("Dashboard");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui.button("Sair").clicked() {
                app.state = crate::app::AppState::Login;
                app.push_log("Usuário fez logout");
            }
        });
    });

    ui.separator();

    ui.horizontal(|ui| {
        if ui.button("Carregar arquivo EXE").clicked() {
            app.push_log("Botão 'Carregar arquivo EXE' clicado (fake)");
        }

        if let Some(path) = &app.selected_file {
            ui.label(format!("Arquivo: {}", path));
        } else {
            ui.label("Arquivo: nenhum selecionado");
        }

        ui.add_space(16.0);

        ui.vertical(|ui| {
            ui.label("Opções:");
            ui.horizontal(|ui| {
                ui.checkbox(&mut app.encrypt_strings, "Encrypt strings");
                ui.add_space(6.0);
                ui.checkbox(&mut app.obfuscate_functions, "Obfuscate functions");
            });
        });

        ui.add_space(16.0);

        if ui.add_enabled(app.selected_file.is_some(), egui::Button::new("Proteger")).clicked() {
            app.push_log("Pipeline 'Proteger' iniciado (mock)");
            if app.encrypt_strings {
                app.push_log(" - Encrypt strings selecionado");
            }
            if app.obfuscate_functions {
                app.push_log(" - Obfuscate functions selecionado");
            }
            app.push_log("Pipeline concluído (mock)");
        }
    });

    ui.separator();

    ui.collapsing("Placeholder - Configurações avançadas", |ui| {
        ui.label("Aqui aparecerão controles avançados (control flow, anti-debug, etc.)");
    });

    ui.separator();

    ui.label("Logs:");
    ui.add_space(4.0);

    // Logs section
    ui.horizontal(|ui| {
        if ui.button("Limpar logs").clicked() {
            app.logs.clear();
            app.push_log("Logs limpos pelo usuário");
        }
        ui.add_space(8.0);
        ui.label(format!("Entradas: {}", app.logs.len()));
    });

    ui.add_space(6.0);

    // Mostrar logs com scroll
    ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
        for entry in app.logs.iter().rev() {
            ui.label(entry);
        }
    });
}
