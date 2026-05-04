use eframe::egui::{self, Event, Key};
use crate::history::History;
use meval::eval_str;

const ALLOWED_INPUT_CHARS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    '.', '+', '-', '*', '/',
];

pub fn handle_keyboard_shortcuts(
    ctx: &egui::Context,
    input: &mut String,
    result: &mut f64,
    history: &mut History,
    show_history: &mut bool,
) {
    for event in ctx.input(|i| i.events.clone()) {
        match event {
            Event::Text(text) => {
                if text.len() == 1 {
                    let ch = text.chars().next().unwrap();
                    if ALLOWED_INPUT_CHARS.contains(&ch) {
                        input.push(ch);
                        continue;
                    }

                    match ch {
                        'c' | 'C' => clear_input(input, result),
                        '=' => evaluate_input(input, result, history),
                        _ => {}
                    }
                }
            }
            Event::Key { key, pressed, .. } if pressed => match key {
                Key::Enter => evaluate_input(input, result, history),
                Key::Backspace => {
                    input.pop();
                }
                Key::Escape => clear_input(input, result),
                _ => {}
            },
            _ => {}
        }
    }

    // Show history when the History button is activated or when user presses H.
    if ctx.input(|i| i.key_pressed(Key::H)) {
        *show_history = true;
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
