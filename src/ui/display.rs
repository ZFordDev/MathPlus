use crate::app::Calculator;
use crate::logic::format_result;
use crate::platform::play_click_sound;
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
                    ui.label(
                        egui::RichText::new(&calc.input)
                            .size(32.0)
                            .monospace()
                            .color(calc.theme_mode.color(
                                ctx,
                                egui::Color32::from_rgb(232, 232, 238),
                                egui::Color32::from_rgb(24, 24, 28),
                            )),
                    );
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
                            ui.output_mut(|o| {
                                o.copied_text = calc.result.to_string();
                            });
                            calc.copied_at = Some(std::time::Instant::now());
                            play_click_sound();
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
