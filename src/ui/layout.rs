use crate::app::Calculator;
use crate::theme::ThemeMode;
use eframe::egui;

use super::{render_actions, render_display, render_keypad};

pub fn render_calculator_ui(calc: &mut Calculator, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("MATH+")
                        .strong()
                        .color(calc.theme_mode.color(
                            ctx,
                            egui::Color32::GRAY,
                            egui::Color32::DARK_GRAY,
                        )),
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

            calc.theme_mode.apply(ctx);

            render_display(calc, ctx, ui);

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            render_keypad(calc, ctx, ui);
            render_actions(calc, ctx, ui);
        });
    });
}
