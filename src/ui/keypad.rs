use crate::app::Calculator;
use eframe::egui;

const KEYPAD_ROWS: &[&[&str]] = &[
    &["C", "CE", "/", "*"],
    &["7", "8", "9", "-"],
    &["4", "5", "6", "+"],
    &["1", "2", "3", "."],
    &["0", "", "", ""],
];

pub fn render_keypad(calc: &mut Calculator, ctx: &egui::Context, ui: &mut egui::Ui) {
    let colors = calc.theme_mode.calculator_colors(ctx);

    ui.vertical_centered(|ui| {
        let keypad_width = ui.available_width().min(480.0);
        let cell_width = (keypad_width - 30.0) / 4.0;
        ui.allocate_ui(egui::vec2(keypad_width, 320.0), |ui| {
            egui::Grid::new("calc_grid")
                .spacing([10.0, 10.0])
                .min_col_width(cell_width)
                .show(ui, |ui| {
                    for row in KEYPAD_ROWS.iter() {
                        for &label in row.iter() {
                            let (fill, text_color) = match label {
                                "+" | "-" | "*" | "/" => {
                                    (colors.operator_background, colors.operator_foreground)
                                }
                                "C" | "CE" => (colors.clear_background, colors.clear_foreground),
                                _ => (colors.key_background, colors.key_foreground),
                            };

                            let btn = egui::Button::new(
                                egui::RichText::new(label)
                                    .size(20.0)
                                    .strong()
                                    .color(text_color),
                            )
                            .fill(fill)
                            .corner_radius(egui::CornerRadius::same(super::BUTTON_CORNER_RADIUS))
                            .stroke(egui::Stroke::new(0.5, colors.subtle_border))
                            .min_size(egui::vec2(cell_width, 55.0));

                            if label.is_empty() {
                                ui.add_sized(egui::vec2(cell_width, 55.0), egui::Label::new(""));
                            } else {
                                let response = ui.add(btn);

                                if response.clicked() {
                                    match label {
                                        "C" => {
                                            calc.input.clear();
                                            calc.result = 0.0;
                                        }
                                        "CE" => {
                                            if clear_current_entry(&mut calc.input) {
                                                calc.result = 0.0;
                                            }
                                        }
                                        _ => calc.input.push_str(label),
                                    }
                                }
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    });
}

fn clear_current_entry(input: &mut String) -> bool {
    let Some((last_index, last_char)) = input.char_indices().next_back() else {
        return false;
    };
    if !last_char.is_ascii_digit() && last_char != '.' {
        return false;
    }

    let mut entry_start = last_index;
    for (index, ch) in input.char_indices().rev() {
        if ch.is_ascii_digit() || ch == '.' {
            entry_start = index;
        } else {
            break;
        }
    }

    // Include a leading minus when it is unary and belongs to this entry.
    if entry_start > 0 && input[..entry_start].ends_with('-') {
        let before_minus = &input[..entry_start - 1];
        if before_minus.is_empty()
            || matches!(
                before_minus.chars().next_back(),
                Some('+' | '-' | '*' | '/')
            )
        {
            entry_start -= 1;
        }
    }

    input.truncate(entry_start);
    true
}
