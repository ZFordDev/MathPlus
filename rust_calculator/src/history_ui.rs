use crate::history::History;
use eframe::egui;

pub enum HistoryAction {
    None,
    Clear,
}

pub fn show_history_window(
    ctx: &egui::Context,
    history: &History,
    show: &mut bool,
) -> HistoryAction {
    let mut action = HistoryAction::None;

    egui::Window::new("Calculation History")
        .open(show)
        .resizable(true)
        .default_size([300.0, 400.0])
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Recent Calculations");

                if history.get_calculations().is_empty() {
                    ui.label("No calculations yet.");
                } else {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (i, calc) in history.get_calculations().iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}.", i + 1));
                                ui.label(&calc.input);
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        let result_text = if calc.result.is_nan() {
                                            "Error".to_string()
                                        } else {
                                            format!("= {:.6}", calc.result)
                                        };
                                        ui.label(
                                            egui::RichText::new(result_text)
                                                .color(egui::Color32::LIGHT_BLUE),
                                        );
                                    },
                                );
                            });
                            ui.separator();
                        }
                    });
                }

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("Clear History").clicked() {
                        action = HistoryAction::Clear;
                    }
                    // Close button removed - window closes via X button or outside click
                });
            });
        });

    action
}
