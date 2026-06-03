// Hide the console window on Windows builds.
// This makes the app feel like a normal GUI program.
#![windows_subsystem = "windows"]

mod app;
mod history;
mod history_ui;
mod logic;
mod platform;
mod shortcuts;
mod theme;
mod ui;

use app::Calculator;
use eframe::{egui, Theme};

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
            .with_inner_size([300.0, 580.0])
            .with_resizable(true)
            .with_icon(load_icon()),
        follow_system_theme: true,
        default_theme: Theme::Dark,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "MathPlus",
        options,
        Box::new(|_cc| Box::<Calculator>::default()),
    );
}