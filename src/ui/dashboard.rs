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

                    if state.processing {
                        ui.add_enabled(
                            false,
                            egui::Button::new("Processing...")
                                .min_size([200.0, 40.0].into()),
                        );
                    } else if ui
                        .add_sized([200.0, 40.0], egui::Button::new("Protect File"))
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
                });
            });
    });
}
