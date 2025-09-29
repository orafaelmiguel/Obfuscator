mod app;
mod ui;

use app::Obscura;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Obscura",
        native_options,
        Box::new(|cc| Ok(Box::new(Obscura::new(cc)))),
    )
}