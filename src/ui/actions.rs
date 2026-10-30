use crate::app::Calculator;
use crate::logic::{EvaluationMode, evaluate_expression};
use eframe::egui;

pub fn render_actions(
    calc: &mut Calculator,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    scientific_panel_visible: bool,
) {
    ui.add_space(15.0);
    let button_width = ui.available_width().min(480.0);
    let colors = calc.theme_mode.calculator_colors(ctx);

    let equals_btn = egui::Button::new(
        egui::RichText::new("=")
            .size(24.0)
            .strong()
            .color(colors.equals_foreground),
    )
    .fill(colors.equals_background)
    .corner_radius(egui::CornerRadius::same(super::BUTTON_CORNER_RADIUS))
    .min_size(egui::vec2(button_width, 50.0));

    let equals_response = ui.add(equals_btn);
    if equals_response.clicked() {
        let mode = if calc.scientific_mode {
            EvaluationMode::Standard
        } else {
            calc.evaluation_mode
        };
        match evaluate_expression(&calc.input, mode) {
            Ok(val) => {
                calc.result = val;
                calc.history.add_calculation(calc.input.clone(), val);
                calc.just_evaluated = true;
            }
            Err(_) => {
                calc.result = f64::NAN;
                calc.just_evaluated = false;
            }
        }
    }

    ui.add_space(10.0);

    let history_btn = egui::Button::new(egui::RichText::new("History").size(18.0).strong())
        .fill(colors.action_background)
        .stroke(egui::Stroke::new(0.5, colors.subtle_border))
        .corner_radius(egui::CornerRadius::same(super::BUTTON_CORNER_RADIUS))
        .min_size(egui::vec2(button_width, 35.0));

    let history_response = ui.add(history_btn);
    if history_response.clicked() {
        calc.show_history = true;
    }

    ui.add_space(6.0);

    let sci_label = if calc.scientific_mode {
        "Basic Mode"
    } else {
        "Scientific Mode"
    };

    let sci_btn = egui::Button::new(egui::RichText::new(sci_label).size(18.0).strong())
        .fill(if calc.scientific_mode {
            colors.scientific_active_background
        } else {
            colors.action_background
        })
        .stroke(egui::Stroke::new(0.5, colors.subtle_border))
        .corner_radius(egui::CornerRadius::same(super::BUTTON_CORNER_RADIUS))
        .min_size(egui::vec2(button_width, 35.0));
    let sci_response = ui.add(sci_btn);
    let sci_response = if calc.scientific_mode && !scientific_panel_visible {
        sci_response.on_hover_text("Widen the window to view the scientific panel.")
    } else {
        sci_response
    };

    if sci_response.clicked() {
        calc.scientific_mode = !calc.scientific_mode;
    }
}
