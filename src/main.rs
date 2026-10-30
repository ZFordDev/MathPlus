// Hide the console window on Windows builds.
// This makes the app feel like a normal GUI program.
#![windows_subsystem = "windows"]

mod app;
mod history;
mod history_ui;
mod logic;
mod shortcuts;
mod theme;
mod ui;

use app::Calculator;
use eframe::egui;

fn load_icon() -> egui::IconData {
    let bytes = include_bytes!("../assets/icon.ico");

    let image = image::load_from_memory(bytes)
        .expect("Failed to load icon")
        .into_rgba8();

    let (width, height) = image.dimensions();

    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([340.0, 700.0])
            .with_min_inner_size([320.0, 700.0])
            .with_resizable(true)
            .with_icon(load_icon()),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "MathPlus",
        options,
        Box::new(|_cc| Ok(Box::<Calculator>::default())),
    );
}