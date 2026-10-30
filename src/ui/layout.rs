use crate::app::Calculator;
use crate::theme::ThemeMode;
use eframe::egui;

use super::{render_actions, render_display, render_keypad};

pub fn render_calculator_ui(
    calc: &mut Calculator,
    ui: &mut egui::Ui,
    scientific_panel_visible: bool,
) {
    let ctx = ui.ctx().clone();
    let colors = calc.theme_mode.calculator_colors(&ctx);

    egui::CentralPanel::default().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("MATH+")
                        .strong()
                        .color(colors.brand_foreground),
                );

                ui.add_space(16.0);
                egui::ComboBox::from_label("Theme")
                    .selected_text(calc.theme_mode.selected_text())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut calc.theme_mode, ThemeMode::System, "System");
                        ui.selectable_value(&mut calc.theme_mode, ThemeMode::Light, "Light");
                        ui.selectable_value(&mut calc.theme_mode, ThemeMode::Dark, "Dark");
                    });
            });

            calc.theme_mode.apply(&ctx);

            render_display(calc, &ctx, ui);

            ui.add_space(18.0);

            render_keypad(calc, &ctx, ui);
            render_actions(calc, &ctx, ui, scientific_panel_visible);
        });
    });
}
