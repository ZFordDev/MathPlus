use eframe::egui::{self, Theme};

pub struct CalculatorColors {
    pub key_background: egui::Color32,
    pub key_foreground: egui::Color32,
    pub operator_background: egui::Color32,
    pub operator_foreground: egui::Color32,
    pub clear_background: egui::Color32,
    pub clear_foreground: egui::Color32,
    pub accent_foreground: egui::Color32,
    pub muted_foreground: egui::Color32,
    pub equals_background: egui::Color32,
    pub equals_foreground: egui::Color32,
    pub action_background: egui::Color32,
    pub scientific_active_background: egui::Color32,
    pub subtle_border: egui::Color32,
    pub history_accent: egui::Color32,
    pub brand_foreground: egui::Color32,
    pub scientific_title: egui::Color32,
    pub secondary_foreground: egui::Color32,
}

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

    pub fn calculator_colors(self, ctx: &egui::Context) -> CalculatorColors {
        let is_dark = self.is_dark_mode(ctx);
        let accent_foreground = if is_dark {
            egui::Color32::from_rgb(138, 180, 248)
        } else {
            egui::Color32::from_rgb(26, 115, 232)
        };
        let muted_foreground = if is_dark {
            egui::Color32::from_rgb(154, 160, 166)
        } else {
            egui::Color32::from_rgb(95, 99, 104)
        };

        CalculatorColors {
            key_background: if is_dark {
                egui::Color32::from_rgb(48, 52, 55)
            } else {
                egui::Color32::from_rgb(245, 246, 247)
            },
            key_foreground: if is_dark {
                egui::Color32::from_rgb(232, 234, 237)
            } else {
                egui::Color32::from_rgb(32, 33, 36)
            },
            operator_background: if is_dark {
                egui::Color32::from_rgb(60, 64, 67)
            } else {
                egui::Color32::from_rgb(232, 240, 254)
            },
            operator_foreground: accent_foreground,
            clear_background: if is_dark {
                egui::Color32::from_rgb(95, 33, 32)
            } else {
                egui::Color32::from_rgb(252, 232, 230)
            },
            clear_foreground: if is_dark {
                egui::Color32::from_rgb(242, 139, 130)
            } else {
                egui::Color32::from_rgb(217, 48, 37)
            },
            accent_foreground,
            muted_foreground,
            equals_background: accent_foreground,
            equals_foreground: if is_dark {
                egui::Color32::from_rgb(32, 33, 36)
            } else {
                egui::Color32::WHITE
            },
            action_background: if is_dark {
                egui::Color32::from_rgb(60, 64, 67)
            } else {
                egui::Color32::from_rgb(218, 220, 224)
            },
            scientific_active_background: if is_dark {
                egui::Color32::from_rgb(74, 78, 95)
            } else {
                egui::Color32::from_rgb(210, 227, 252)
            },
            subtle_border: if is_dark {
                egui::Color32::from_rgba_premultiplied(255, 255, 255, 14)
            } else {
                egui::Color32::from_rgba_premultiplied(0, 0, 0, 18)
            },
            history_accent: accent_foreground,
            brand_foreground: muted_foreground,
            scientific_title: accent_foreground,
            secondary_foreground: muted_foreground,
        }
    }
}
