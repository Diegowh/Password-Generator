mod app;
mod clipboard;
mod config;
mod generators;
mod controllers;

use app::PasswordGeneratorApp;
use eframe;
use egui::IconData;

// Embed the icon at compile time
const ICON_BYTES: &[u8] = include_bytes!("assets/icon.png");

fn load_icon() -> IconData {
    let image = image::load_from_memory(ICON_BYTES)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

fn main() -> Result<(), eframe::Error> {
    let icon = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_resizable(false)
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "Password Generator",
        options,
        Box::new(|_cc| Ok(Box::new(PasswordGeneratorApp::new()))),
    )
}