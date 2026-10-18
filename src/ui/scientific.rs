use eframe::egui;

pub fn render_scientific_panel(ui: &mut egui::Ui) {
    egui::Panel::right("scientific_panel")
        .resizable(false)
        .default_size(220.0)
        .min_size(200.0)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Scientific")
                        .size(16.0)
                        .strong()
                        .color(egui::Color32::from_rgb(160, 157, 232)),
                );
                ui.separator();
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Functions coming soon...")
                        .size(13.0)
                        .color(egui::Color32::from_rgb(140, 140, 150)),
                );
            });
        });
}
