use crate::history::History;
use crate::history_ui::{HistoryAction, show_history_window};
use crate::shortcuts::handle_keyboard_shortcuts;
use crate::theme::ThemeMode;
use crate::ui::{render_calculator_ui, render_scientific_panel};
use eframe::{App, Frame};

pub struct Calculator {
    pub input: String,
    pub result: f64,
    pub history: History,
    pub show_history: bool,
    pub copied_at: Option<std::time::Instant>,
    pub scientific_mode: bool,
    pub theme_mode: ThemeMode,
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
        }
    }
}

impl App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if self.scientific_mode {
            render_scientific_panel(ctx);
        }

        render_calculator_ui(self, ctx);

        handle_keyboard_shortcuts(
            ctx,
            &mut self.input,
            &mut self.result,
            &mut self.history,
            &mut self.show_history,
            &mut self.copied_at,
        );

        if self.show_history {
            let action = show_history_window(ctx, &self.history, &mut self.show_history);
            if matches!(action, HistoryAction::Clear) {
                self.history.clear();
            }
        }
    }
}
