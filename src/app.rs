use crate::history::History;
use crate::history_ui::{HistoryAction, show_history_window};
use crate::shortcuts::{filter_events, handle_keyboard_shortcuts};
use crate::theme::ThemeMode;
use crate::ui::{render_calculator_ui, render_scientific_panel};
use eframe::{App, Frame};

const SCIENTIFIC_PANEL_MIN_WINDOW_WIDTH: f32 = 560.0;

pub struct Calculator {
    pub input: String,
    pub result: f64,
    pub history: History,
    pub show_history: bool,
    pub copied_at: Option<std::time::Instant>,
    pub scientific_mode: bool,
    pub theme_mode: ThemeMode,
    /// One-shot: focus the expression field on the first frame only.
    pub focus_input: bool,
    /// Set by the display each frame; lets the Copy shortcut prefer a
    /// text selection in the field over copying the result.
    pub field_has_selection: bool,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            input: String::new(),
            result: 0.0,
            history: History::new(100),
            show_history: false,
            copied_at: None,
            scientific_mode: false,
            theme_mode: ThemeMode::default(),
            focus_input: true,
            field_has_selection: false,
        }
    }
}

impl App for Calculator {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        let ctx = ui.ctx().clone();

        // Strip blocked keystrokes before the TextEdit renders, so it never
        // sees characters the calculator handles itself (=, c) or rejects.
        let pending = filter_events(&ctx);

        let scientific_panel_visible =
            self.scientific_mode && ui.available_width() >= SCIENTIFIC_PANEL_MIN_WINDOW_WIDTH;
        if scientific_panel_visible {
            render_scientific_panel(ui, self.theme_mode);
        }

        render_calculator_ui(self, ui, scientific_panel_visible);

        handle_keyboard_shortcuts(&ctx, self, pending);

        if self.show_history {
            let action =
                show_history_window(&ctx, &self.history, &mut self.show_history, self.theme_mode);
            if matches!(action, HistoryAction::Clear) {
                self.history.clear();
            }
        }
    }
}
