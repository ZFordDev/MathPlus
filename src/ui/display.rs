use crate::app::Calculator;
use crate::logic::{format_result, format_value};
use eframe::egui;

pub fn render_display(calc: &mut Calculator, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.allocate_ui(egui::vec2(260.0, 100.0), |ui| {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_height(95.0);
            ui.set_width(260.0);

            ui.vertical(|ui| {
                ui.add_space(10.0);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut calc.input)
                            .id_salt("expression_input")
                            .font(egui::FontSelection::FontId(egui::FontId::new(
                                32.0,
                                egui::FontFamily::Monospace,
                            )))
                            .text_color(calc.theme_mode.color(
                                ctx,
                                egui::Color32::from_rgb(232, 232, 238),
                                egui::Color32::from_rgb(24, 24, 28),
                            ))
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
                        ui.label(egui::RichText::new("Copied!").size(11.0).color(
                            calc.theme_mode.color(
                                ctx,
                                egui::Color32::from_rgb(160, 157, 232),
                                egui::Color32::from_rgb(70, 90, 170),
                            ),
                        ));
                        ctx.request_repaint();
                    } else {
                        let copy_btn =
                            egui::Button::new(egui::RichText::new("📋").size(13.0).color(
                                calc.theme_mode.color(
                                    ctx,
                                    egui::Color32::from_rgb(110, 110, 120),
                                    egui::Color32::from_rgb(90, 90, 100),
                                ),
                            ))
                            .fill(egui::Color32::TRANSPARENT)
                            .stroke(egui::Stroke::NONE)
                            .min_size(egui::vec2(18.0, 18.0));

                        if ui.add(copy_btn).clicked() {
                            ui.ctx().copy_text(format_value(calc.result));
                            calc.copied_at = Some(std::time::Instant::now());
                        }
                    }

                    ui.label(egui::RichText::new(&res_text).size(18.0).color(
                        calc.theme_mode.color(
                            ctx,
                            egui::Color32::from_rgb(160, 157, 232),
                            egui::Color32::from_rgb(70, 90, 170),
                        ),
                    ));
                });
            });
        });
    });
}
