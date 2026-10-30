use crate::theme::ThemeMode;
use eframe::egui;

pub fn render_scientific_panel(ui: &mut egui::Ui, theme_mode: ThemeMode) {
    let colors = theme_mode.calculator_colors(ui.ctx());

    egui::Panel::right("scientific_panel")
        .resizable(false)
        .default_size(220.0)
        .min_size(200.0)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Scientific")
                        .size(16.0)
                        .strong()
                        .color(colors.scientific_title),
                );
                ui.separator();
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Functions coming soon...")
                        .size(13.0)
                        .color(colors.secondary_foreground),
                );
            });
        });
}
