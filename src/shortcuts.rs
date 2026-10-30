use crate::app::Calculator;
use crate::logic::{EvaluationMode, evaluate_expression, format_value};
use eframe::egui::{self, Event, Key};

const ALLOWED_INPUT_CHARS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '+', '-', '*', '/',
];

/// Shortcuts that fired while filtering events, applied once the UI has rendered.
#[derive(Default)]
pub struct PendingShortcuts {
    pub evaluate: bool,
    pub clear: bool,
    pub operator_input: bool,
}

/// Runs before the UI renders: strips events the input field must not see
/// (blocked characters, `=`, `c`) so the TextEdit never inserts them.
/// Key/Copy events are left untouched for `handle_keyboard_shortcuts`.
pub fn filter_events(ctx: &egui::Context, calc: &mut Calculator) -> PendingShortcuts {
    let mut pending = PendingShortcuts::default();
    let mut kept = Vec::new();

    for event in ctx.input(|i| i.events.clone()) {
        match event {
            Event::Text(text) if text == "=" => pending.evaluate = true,
            Event::Text(text) if text == "c" || text == "C" => pending.clear = true,
            Event::Text(text) => {
                prepare_for_next_input(calc, &text);
                if text.chars().any(|c| matches!(c, '+' | '-' | '*' | '/')) {
                    pending.operator_input = true;
                }
                let filtered: String = text
                    .chars()
                    .filter(|c| ALLOWED_INPUT_CHARS.contains(c))
                    .collect();
                if !filtered.is_empty() {
                    kept.push(Event::Text(filtered));
                }
            }
            Event::Paste(text) => {
                prepare_for_next_input(calc, &text);
                if text.chars().any(|c| matches!(c, '+' | '-' | '*' | '/')) {
                    pending.operator_input = true;
                }
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
    let operator_input = pending.operator_input;
    if pending.clear {
        clear_input(&mut calc.input, &mut calc.result);
        calc.just_evaluated = false;
    }
    if pending.evaluate {
        evaluate_input(calc);
    }

    for event in ctx.input(|i| i.events.clone()) {
        match event {
            // If the input field has a selection, the TextEdit already copied
            // it during render — don't overwrite the clipboard with the result.
            Event::Copy if !calc.field_has_selection => {
                if !calc.result.is_nan() && !(calc.result == 0.0 && calc.input.is_empty()) {
                    ctx.copy_text(format_value(calc.result));
                    calc.copied_at = Some(std::time::Instant::now());
                }
            }
            Event::Key {
                key, pressed: true, ..
            } => match key {
                Key::Enter => evaluate_input(calc),
                Key::Escape => {
                    clear_input(&mut calc.input, &mut calc.result);
                    calc.just_evaluated = false;
                }
                _ => {}
            },
            _ => {}
        }
    }

    if operator_input
        && !calc.scientific_mode
        && calc.evaluation_mode == EvaluationMode::Chained
        && let Some(operator_index) = calc
            .input
            .char_indices()
            .next_back()
            .filter(|(_, c)| matches!(c, '+' | '-' | '*' | '/'))
            .map(|(index, _)| index)
    {
        let expression = &calc.input[..operator_index];
        if let Ok(value) = evaluate_expression(expression, EvaluationMode::Chained) {
            calc.result = value;
        }
    }

    // Show history when the History button is activated or when user presses H.
    if ctx.input(|i| i.key_pressed(Key::H)) {
        calc.show_history = true;
    }
}

fn evaluate_input(calc: &mut Calculator) {
    let mode = if calc.scientific_mode {
        EvaluationMode::Standard
    } else {
        calc.evaluation_mode
    };
    match evaluate_expression(&calc.input, mode) {
        Ok(val) => {
            calc.result = val;
            calc.history.add_calculation(calc.input.clone(), val);
            calc.just_evaluated = true;
        }
        Err(_) => {
            calc.result = f64::NAN;
            calc.just_evaluated = false;
        }
    }
}

fn clear_input(input: &mut String, result: &mut f64) {
    input.clear();
    *result = 0.0;
}

fn prepare_for_next_input(calc: &mut Calculator, text: &str) {
    if !calc.just_evaluated {
        return;
    }

    if !calc.scientific_mode && calc.evaluation_mode == EvaluationMode::Chained {
        match text.chars().next() {
            Some('+' | '-' | '*' | '/') => {
                calc.input = format_value(calc.result);
            }
            Some('0'..='9' | '.') => {
                calc.input.clear();
                calc.result = 0.0;
            }
            _ => {}
        }
    }
    calc.just_evaluated = false;
}
