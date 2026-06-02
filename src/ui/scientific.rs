use eframe::egui;

pub fn render_scientific_panel(ctx: &egui::Context) {
    egui::SidePanel::right("scientific_panel")
        .resizable(false)
        .default_width(220.0)
        .min_width(200.0)
        .show(ctx, |ui| {
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
