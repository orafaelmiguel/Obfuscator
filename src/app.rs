use eframe::{egui, App, CreationContext, Frame};

pub struct Obscura {
    pub counter: i32,
}

impl Obscura {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self { counter: 0 }
    }
}

impl App for Obscura {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
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
