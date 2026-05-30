use crate::app::Calculator;
use crate::platform::play_click_sound;
use eframe::egui;

const KEYPAD_ROWS: [[&str; 4]; 4] = [
    ["7", "8", "9", "/"],
    ["4", "5", "6", "*"],
    ["1", "2", "3", "-"],
    ["C", "0", ".", "+"],
];

pub fn render_keypad(calc: &mut Calculator, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.allocate_ui(egui::vec2(260.0, 260.0), |ui| {
            egui::Grid::new("calc_grid")
                .spacing([10.0, 10.0])
                .min_col_width(55.0)
                .show(ui, |ui| {
                    for row in KEYPAD_ROWS.iter() {
                        for &label in row.iter() {
                            let (fill, text_color) = match label {
                                "+" | "-" | "*" | "/" => (
                                    calc.theme_mode.color(
                                        ctx,
                                        egui::Color32::from_rgb(37, 37, 53),
                                        egui::Color32::from_rgb(205, 210, 250),
                                    ),
                                    calc.theme_mode.color(
                                        ctx,
                                        egui::Color32::from_rgb(160, 157, 232),
                                        egui::Color32::from_rgb(35, 45, 75),
                                    ),
                                ),
                                "C" => (
                                    calc.theme_mode.color(
                                        ctx,
                                        egui::Color32::from_rgb(46, 28, 28),
                                        egui::Color32::from_rgb(255, 230, 230),
                                    ),
                                    calc.theme_mode.color(
                                        ctx,
                                        egui::Color32::from_rgb(232, 128, 128),
                                        egui::Color32::from_rgb(145, 35, 50),
                                    ),
                                ),
                                _ => (
                                    calc.theme_mode.color(
                                        ctx,
                                        egui::Color32::from_rgb(42, 42, 48),
                                        egui::Color32::from_rgb(236, 236, 240),
                                    ),
                                    calc.theme_mode.color(
                                        ctx,
                                        egui::Color32::from_rgb(232, 232, 238),
                                        egui::Color32::from_rgb(24, 24, 28),
                                    ),
                                ),
                            };

                            let btn = egui::Button::new(
                                egui::RichText::new(label)
                                    .size(20.0)
                                    .strong()
                                    .color(text_color),
                            )
                            .fill(fill)
                            .rounding(egui::Rounding::same(10.0))
                            .stroke(egui::Stroke::new(
                                0.5,
                                calc.theme_mode.color(
                                    ctx,
                                    egui::Color32::from_rgba_premultiplied(255, 255, 255, 10),
                                    egui::Color32::from_rgba_premultiplied(0, 0, 0, 15),
                                ),
                            ))
                            .min_size(egui::vec2(55.0, 55.0));

                            if ui.add(btn).clicked() {
                                match label {
                                    "C" => {
                                        calc.input.clear();
                                        calc.result = 0.0;
                                    }
                                    _ => calc.input.push_str(label),
                                }
                                play_click_sound();
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    });
}
