use crate::app::Calculator;
use crate::logic::evaluate_expression;
use crate::platform::play_click_sound;
use eframe::egui;

pub fn render_actions(calc: &mut Calculator, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.add_space(15.0);

    let equals_btn = egui::Button::new(egui::RichText::new("=").size(24.0).strong().color(
        calc.theme_mode.color(
            ctx,
            egui::Color32::from_rgb(212, 210, 247),
            egui::Color32::from_rgb(245, 245, 255),
        ),
    ))
    .fill(calc.theme_mode.color(
        ctx,
        egui::Color32::from_rgb(37, 37, 53),
        egui::Color32::from_rgb(70, 100, 170),
    ))
    .rounding(egui::Rounding::same(12.0))
    .min_size(egui::vec2(250.0, 50.0));

    if ui.add(equals_btn).clicked() {
        match evaluate_expression(&calc.input) {
            Ok(val) => {
                calc.result = val;
                calc.history.add_calculation(calc.input.clone(), val);
            }
            Err(_) => calc.result = f64::NAN,
        }
        play_click_sound();
    }

    ui.add_space(10.0);

    let history_btn = egui::Button::new(egui::RichText::new("History").size(18.0).strong())
        .fill(calc.theme_mode.color(
            ctx,
            egui::Color32::from_rgb(100, 100, 100),
            egui::Color32::from_rgb(210, 210, 210),
        ))
        .stroke(egui::Stroke::new(
            0.5,
            calc.theme_mode.color(
                ctx,
                egui::Color32::from_rgba_premultiplied(255, 255, 255, 10),
                egui::Color32::from_rgba_premultiplied(0, 0, 0, 15),
            ),
        ))
        .min_size(egui::vec2(250.0, 35.0));

    if ui.add(history_btn).clicked() {
        calc.show_history = true;
        play_click_sound();
    }

    ui.add_space(6.0);

    let sci_label = if calc.scientific_mode {
        "Basic Mode"
    } else {
        "Scientific Mode"
    };

    let sci_btn = egui::Button::new(egui::RichText::new(sci_label).size(18.0).strong())
        .fill(if calc.scientific_mode {
            calc.theme_mode.color(
                ctx,
                egui::Color32::from_rgb(60, 80, 120),
                egui::Color32::from_rgb(100, 130, 190),
            )
        } else {
            calc.theme_mode.color(
                ctx,
                egui::Color32::from_rgb(100, 100, 100),
                egui::Color32::from_rgb(210, 210, 210),
            )
        })
        .stroke(egui::Stroke::new(
            0.5,
            calc.theme_mode.color(
                ctx,
                egui::Color32::from_rgba_premultiplied(255, 255, 255, 10),
                egui::Color32::from_rgba_premultiplied(0, 0, 0, 15),
            ),
        ))
        .min_size(egui::vec2(250.0, 35.0));

    if ui.add(sci_btn).clicked() {
        calc.scientific_mode = !calc.scientific_mode;
        play_click_sound();
    }
}
