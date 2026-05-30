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
use eframe::Theme;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 580.0])
            .with_resizable(true),
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
