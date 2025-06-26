mod app;
mod clipboard;
mod config;
mod generators;
mod controllers;

use eframe;
use app::PasswordGeneratorApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_resizable(false),
        ..Default::default()
    };
    
    eframe::run_native(
        "Password Generator",
        options,
        Box::new(|_cc| Ok(Box::new(PasswordGeneratorApp::new())))
    )
}