use eframe::egui;
use crate::state::ObscuraState;
use crate::pipeline;

pub fn show_dashboard(ui: &mut egui::Ui, state: &mut ObscuraState) {
    ui.with_layout(egui::Layout::top_down(eframe::egui::Align::Center), |ui| {
        ui.add_space(20.0);

        ui.heading(
            egui::RichText::new("Dashboard")
                .size(28.0)
                .strong(),
        );

        ui.add_space(30.0);

        // --- File Section ---
        egui::Frame::default()
            .corner_radius(12)
            .stroke(egui::Stroke::NONE)
            .show(ui, |ui| {
                ui.set_width(600.0);
                ui.vertical(|ui| {
                    ui.heading("📂 File");
                    ui.add_space(10.0);

                    if ui.button("Load EXE file").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Executable", &["exe"])
                            .pick_file()
                        {
                            let path_str = path.display().to_string();
                            state.selected_file = Some(path_str.clone());
                            state.push_log(format!("Loaded file: {}", path_str));
                        }
                    }

                    ui.add_space(8.0);

                    if let Some(path) = &state.selected_file {
                        ui.label(
                            egui::RichText::new(format!("Selected: {}", path))
                                .color(ui.visuals().hyperlink_color),
                        );
                    } else {
                        ui.label(egui::RichText::new("No file selected").italics());
                    }
                });
            });

        ui.add_space(20.0);

        // --- Log Section ---
        egui::Frame::default()
            .corner_radius(12)
            .stroke(egui::Stroke::NONE)
            .show(ui, |ui| {
                ui.set_width(600.0);
                ui.vertical(|ui| {
                    ui.heading("📝 Logs");
                    ui.add_space(10.0);

                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            for log in &state.logs {
                                ui.label(log);
                            }
                        });
                });
            });

        ui.add_space(20.0);

        // --- Actions Section ---
        egui::Frame::default()
            .corner_radius(12)
            .stroke(egui::Stroke::NONE)
            .show(ui, |ui| {
                ui.set_width(600.0);
                ui.vertical_centered(|ui| {
                    ui.heading("⚡ Actions");
                    ui.add_space(10.0);

                    // progress bar durante processamento
                    if state.processing {
                        ui.label("Processing pipeline...");
                        ui.add_space(5.0);
                        ui.add(
                            egui::ProgressBar::new(state.progress)
                                .show_percentage()
                        );
                        ui.add_space(8.0);
                        
                        // botão cancel
                        if ui.button("❌ Cancel").clicked() {
                            if let Some(flag) = &state.cancel_flag {
                                flag.store(true, std::sync::atomic::Ordering::Relaxed);
                                state.push_log("Cancellation requested...");
                            }
                        }
                        
                        ui.add_space(10.0);
                    }

                    // botão protect file (desabilitado durante processamento)
                    let protect_enabled = !state.processing;
                    if ui
                        .add_enabled(protect_enabled, egui::Button::new("Protect File").min_size([200.0, 40.0].into()))
                        .clicked()
                    {
                        if let Some(path) = state.selected_file.clone() {
                            state.push_log(format!("Starting pipeline for {}", path));

                            // iniciar pipeline modular (nova API)
                            pipeline::start_pipeline(state, path);
                            state.processing = true;
                            state.progress = 0.0;
                        } else {
                            state.push_log("No file selected. Cannot run pipeline.");
                        }
                    }

                    ui.add_space(15.0);

                    // mostrar last output se disponível
                    if let Some(output_path) = &state.last_output {
                        ui.separator();
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(format!("Last output: {}", output_path))
                                .color(ui.visuals().hyperlink_color)
                        );
                        ui.add_space(5.0);
                        
                        // botão para abrir pasta do output
                        if ui.button("📁 Open output folder").clicked() {
                            if let Some(parent) = std::path::Path::new(output_path).parent() {
                                if let Err(e) = open::that(parent) {
                                    state.push_log(format!("Failed to open folder: {}", e));
                                } else {
                                    state.push_log(format!("Opened folder: {}", parent.display()));
                                }
                            }
                        }
                    }
                });
            });
    });
}
