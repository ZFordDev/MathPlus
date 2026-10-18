use crate::app::Calculator;
use crate::history::History;
use eframe::egui::{self, Event, Key};
use meval::eval_str;

const ALLOWED_INPUT_CHARS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '+', '-', '*', '/',
];

/// Shortcuts that fired while filtering events, applied once the UI has rendered.
#[derive(Default)]
pub struct PendingShortcuts {
    pub evaluate: bool,
    pub clear: bool,
}

/// Runs before the UI renders: strips events the input field must not see
/// (blocked characters, `=`, `c`) so the TextEdit never inserts them.
/// Key/Copy events are left untouched for `handle_keyboard_shortcuts`.
pub fn filter_events(ctx: &egui::Context) -> PendingShortcuts {
    let mut pending = PendingShortcuts::default();
    let mut kept = Vec::new();

    for event in ctx.input(|i| i.events.clone()) {
        match event {
            Event::Text(text) if text == "=" => pending.evaluate = true,
            Event::Text(text) if text == "c" || text == "C" => pending.clear = true,
            Event::Text(text) => {
                let filtered: String = text
                    .chars()
                    .filter(|c| ALLOWED_INPUT_CHARS.contains(c))
                    .collect();
                if !filtered.is_empty() {
                    kept.push(Event::Text(filtered));
                }
            }
            Event::Paste(text) => {
                let filtered: String = text
                    .chars()
                    .filter(|c| ALLOWED_INPUT_CHARS.contains(c))
                    .collect();
                if !filtered.is_empty() {
                    kept.push(Event::Paste(filtered));
                }
            }
            other => kept.push(other),
        }
    }

    ctx.input_mut(|i| i.events = kept);
    pending
}

pub fn handle_keyboard_shortcuts(
    ctx: &egui::Context,
    calc: &mut Calculator,
    pending: PendingShortcuts,
) {
    if pending.clear {
        clear_input(&mut calc.input, &mut calc.result);
    }
    if pending.evaluate {
        evaluate_input(&calc.input, &mut calc.result, &mut calc.history);
    }

    for event in ctx.input(|i| i.events.clone()) {
        match event {
            // If the input field has a selection, the TextEdit already copied
            // it during render — don't overwrite the clipboard with the result.
            Event::Copy if !calc.field_has_selection => {
                if !calc.result.is_nan() && !(calc.result == 0.0 && calc.input.is_empty()) {
                    ctx.copy_text(calc.result.to_string());
                    calc.copied_at = Some(std::time::Instant::now());
                }
            }
            Event::Key {
                key,
                pressed: true,
                ..
            } => match key {
                Key::Enter => evaluate_input(&calc.input, &mut calc.result, &mut calc.history),
                Key::Escape => clear_input(&mut calc.input, &mut calc.result),
                _ => {}
            },
            _ => {}
        }
    }

    // Show history when the History button is activated or when user presses H.
    if ctx.input(|i| i.key_pressed(Key::H)) {
        calc.show_history = true;
    }
}

fn evaluate_input(input: &str, result: &mut f64, history: &mut History) {
    match eval_str(input) {
        Ok(val) => {
            *result = val;
            history.add_calculation(input.to_string(), val);
        }
        Err(_) => {
            *result = f64::NAN;
        }
    }
}

fn clear_input(input: &mut String, result: &mut f64) {
    input.clear();
    *result = 0.0;
}
