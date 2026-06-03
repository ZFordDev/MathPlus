use eframe::{Theme, egui};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

impl ThemeMode {
    pub fn selected_text(self) -> &'static str {
        match self {
            ThemeMode::System => "System",
            ThemeMode::Light => "Light",
            ThemeMode::Dark => "Dark",
        }
    }

    pub fn is_dark_mode(self, ctx: &egui::Context) -> bool {
        match self {
            ThemeMode::System => ctx.style().visuals.dark_mode,
            ThemeMode::Light => false,
            ThemeMode::Dark => true,
        }
    }

    pub fn apply(self, ctx: &egui::Context) {
        if self != ThemeMode::System {
            let visuals = match self {
                ThemeMode::Light => Theme::Light.egui_visuals(),
                ThemeMode::Dark => Theme::Dark.egui_visuals(),
                ThemeMode::System => unreachable!(),
            };
            ctx.set_visuals(visuals);
        }
    }

    pub fn color(
        self,
        ctx: &egui::Context,
        dark: egui::Color32,
        light: egui::Color32,
    ) -> egui::Color32 {
        if self.is_dark_mode(ctx) { dark } else { light }
    }
}
