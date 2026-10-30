use crate::app::Calculator;
use crate::logic::{format_result, format_value};
use eframe::egui;

pub fn render_display(calc: &mut Calculator, ctx: &egui::Context, ui: &mut egui::Ui) {
    let colors = calc.theme_mode.calculator_colors(ctx);
    let display_width = ui.available_width().min(480.0);
    ui.allocate_ui(egui::vec2(display_width, 100.0), |ui| {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_height(95.0);
            ui.set_width(display_width);

            ui.vertical(|ui| {
                ui.add_space(10.0);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut calc.input)
                            .id_salt("expression_input")
                            .font(egui::FontSelection::FontId(egui::FontId::new(
                                22.0,
                                egui::FontFamily::Monospace,
                            )))
                            .text_color(colors.key_foreground)
                            // Plain label look — no boxed input frame.
                            .frame(egui::Frame::NONE)
                            .desired_width(f32::INFINITY)
                            .align(egui::Align2::RIGHT_CENTER)
                            // Enter must not surrender focus — our shortcut
                            // handler evaluates the expression instead.
                            .return_key(None)
                            // Esc clears the expression but must keep focus
                            // in the field (default Esc hands focus back to
                            // the window and the next keystroke goes nowhere).
                            .event_filter(egui::EventFilter {
                                horizontal_arrows: true,
                                vertical_arrows: true,
                                tab: false,
                                escape: true,
                            }),
                    );

                    // Focus the field once at startup so typing works
                    // immediately; afterwards focus is the user's to manage.
                    if calc.focus_input {
                        response.request_focus();
                        calc.focus_input = false;
                    }

                    // Remember whether the field has a text selection, so the
                    // Copy shortcut can prefer it over copying the result.
                    calc.field_has_selection = response.has_focus()
                        && egui::text_edit::TextEditState::load(ctx, response.id)
                            .and_then(|s| s.cursor.char_range())
                            .is_some_and(|r| !r.is_empty());
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.0);

                    let res_text = format_result(calc.result);
                    let just_copied = calc
                        .copied_at
                        .map(|t| t.elapsed().as_millis() < 600)
                        .unwrap_or(false);

                    if just_copied {
                        ui.label(
                            egui::RichText::new("Copied!")
                                .size(11.0)
                                .color(colors.accent_foreground),
                        );
                        ctx.request_repaint();
                    } else {
                        let copy_btn = egui::Button::new(
                            egui::RichText::new("Copy")
                                .size(11.0)
                                .color(colors.muted_foreground),
                        )
                        .fill(egui::Color32::TRANSPARENT)
                        .stroke(egui::Stroke::NONE)
                        .min_size(egui::vec2(18.0, 18.0));

                        let copy_response = ui.add(copy_btn);
                        if copy_response.clicked() {
                            ui.ctx().copy_text(format_value(calc.result));
                            calc.copied_at = Some(std::time::Instant::now());
                        }
                    }

                    ui.label(
                        egui::RichText::new(&res_text)
                            .size(28.0)
                            .color(colors.accent_foreground),
                    );
                });
            });
        });
    });
}
