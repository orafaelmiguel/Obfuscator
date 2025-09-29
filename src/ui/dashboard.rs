use eframe::egui;
use eframe::egui::ScrollArea;
use crate::state::{ObscuraState, AppState};
use crate::pipeline::start_pipeline_mock;
use crate::auth;

pub fn show_dashboard(ui: &mut egui::Ui, state: &mut ObscuraState) {
    // Poll pipeline messages each frame
    state.poll_pipeline_messages();

    ui.horizontal(|ui| {
        ui.heading("Dashboard");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui.button("Logout").clicked() {
                auth::logout(state);
            }
        });
    });

    ui.separator();

    ui.horizontal(|ui| {
        if ui.button("Load EXE file").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Executable", &["exe"])
                .pick_file()
            {
                let path_str = path.display().to_string();
                state.selected_file = Some(path_str.clone());
                state.push_log(format!("Selected file: {}", path_str));
            } else {
                state.push_log("File selection cancelled");
            }
        }

        if let Some(path) = &state.selected_file {
            ui.label(format!("File: {}", path));
        } else {
            ui.label("File: none selected");
        }
    });

    ui.separator();

    ui.horizontal(|ui| {
        if ui.add_enabled(
            state.selected_file.is_some() && !state.processing,
            egui::Button::new("Protect"),
        ).clicked() {
            if let Some(path) = &state.selected_file {
                let rx = start_pipeline_mock(path.clone().into());
                state.pipeline_rx = Some(rx);
                state.processing = true;
                state.progress = 0.0;
                state.push_log("Pipeline started (mock)");
            }
        }

        if state.processing {
            ui.add(egui::ProgressBar::new(state.progress).show_percentage());
        }
    });

    ui.separator();

    ui.collapsing("Logs", |ui| {
        ui.horizontal(|ui| {
            if ui.button("Clear logs").clicked() {
                state.logs.clear();
                state.push_log("Logs cleared");
            }
            ui.label(format!("Entries: {}", state.logs.len()));
        });

        ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            for entry in state.logs.iter().rev() {
                ui.label(entry);
            }
        });
    });
}
