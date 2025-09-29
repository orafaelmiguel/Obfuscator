use eframe::egui;

struct Obscura {
    counter: i32,
}

impl Obscura {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { counter: 0 }
    }
}

impl eframe::App for Obscura {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Obscura");
            ui.horizontal(|ui| {
                if ui.button("−").clicked() {
                    self.counter -= 1;
                }
                ui.label(self.counter.to_string());
                if ui.button("+").clicked() {
                    self.counter += 1;
                }
            });
            ui.separator();
            ui.label("Hi");
        });
    }
}


fn main() {
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
    ).unwrap();
}

