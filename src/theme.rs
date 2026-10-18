use eframe::egui::{self, Theme};

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
            ThemeMode::System => ctx.theme() == Theme::Dark,
            ThemeMode::Light => false,
            ThemeMode::Dark => true,
        }
    }

    pub fn apply(self, ctx: &egui::Context) {
        if self != ThemeMode::System {
            let theme = match self {
                ThemeMode::Light => Theme::Light,
                ThemeMode::Dark => Theme::Dark,
                ThemeMode::System => unreachable!(),
            };
            ctx.set_theme(theme);
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
