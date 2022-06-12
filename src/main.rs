#![windows_subsystem = "windows"]

use eframe::egui;
use rand::prelude::*;
use rand::distributions::Alphanumeric;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn main() {

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 768.0)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "Password Generator 0.1.0",
        native_options,
        Box::new(|cc| Box::new(PasswordGenerator::new(cc)))
    )
}

#[derive(Default)]
struct PasswordGenerator {}

impl PasswordGenerator {
    fn new(__cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for PasswordGenerator {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Datei", |ui| {
                    if ui.button("Beenden...").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(64.0);
                ui.label("Test");
            });
        });
    }
}

    // let gen_passwd = |length| -> String {
    //     let password = thread_rng().sample_iter(Alphanumeric).take(length).map(char::from).collect();
    //     password
    // };
