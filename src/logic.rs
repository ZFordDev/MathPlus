use meval::eval_str;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum EvaluationMode {
    #[default]
    Chained,
    Standard,
}

impl EvaluationMode {
    pub fn selected_text(self) -> &'static str {
        match self {
            Self::Chained => "Chained",
            Self::Standard => "Standard",
        }
    }
}

pub fn evaluate_expression(input: &str, mode: EvaluationMode) -> Result<f64, ()> {
    match mode {
        EvaluationMode::Chained => evaluate_left_to_right(input),
        EvaluationMode::Standard => eval_str(input).map_err(|_| ()),
    }
}

fn evaluate_left_to_right(input: &str) -> Result<f64, ()> {
    let bytes = input.as_bytes();
    let mut index = 0;
    let mut result = None;
    let mut pending_operator = None;

    // Apply each operator as it appears instead of using standard precedence.
    while index < bytes.len() {
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index == bytes.len() {
            break;
        }

        let number_start = index;
        if matches!(bytes[index], b'+' | b'-') {
            index += 1;
        }
        let integer_start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        let has_integer_digits = index > integer_start;
        let mut has_fraction_digits = false;
        if index < bytes.len() && bytes[index] == b'.' {
            index += 1;
            let fraction_start = index;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                index += 1;
            }
            has_fraction_digits = index > fraction_start;
        }
        if !has_integer_digits && !has_fraction_digits {
            return Err(());
        }

        let operand = input[number_start..index].parse::<f64>().map_err(|_| ())?;

        result = Some(match (result, pending_operator) {
            (None, None) => operand,
            (Some(current), Some(operator)) => match operator {
                b'+' => current + operand,
                b'-' => current - operand,
                b'*' => current * operand,
                b'/' => current / operand,
                _ => return Err(()),
            },
            _ => return Err(()),
        });
        pending_operator = None;

        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index < bytes.len() {
            if !matches!(bytes[index], b'+' | b'-' | b'*' | b'/') {
                return Err(());
            }
            pending_operator = Some(bytes[index]);
            index += 1;
        }
    }

    if pending_operator.is_some() {
        Err(())
    } else {
        result.ok_or(())
    }
}

pub fn format_result(result: f64) -> String {
    if result.is_nan() {
        "Error".to_string()
    } else {
        format!("= {}", format_value(result))
    }
}

pub fn format_history_result(result: f64) -> String {
    format_result(result)
}

pub fn format_value(result: f64) -> String {
    if !result.is_finite() || result == 0.0 {
        return result.to_string();
    }

    // Round only the rendered value; calculations retain the original f64.
    let scientific = format!("{:.14e}", result.abs());
    let (mantissa, exponent) = scientific.split_once('e').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
    let sign = if result.is_sign_negative() { "-" } else { "" };

    // Scientific notation keeps very large and small values compact.
    if !(-6..15).contains(&exponent) {
        return format!("{sign}{mantissa}e{exponent}");
    }

    let digits = mantissa.replace('.', "");
    let decimal_position = 1 + exponent;
    if decimal_position <= 0 {
        format!(
            "{sign}0.{}{digits}",
            "0".repeat((-decimal_position) as usize)
        )
    } else if decimal_position as usize >= digits.len() {
        format!(
            "{sign}{digits}{}",
            "0".repeat(decimal_position as usize - digits.len())
        )
    } else {
        let decimal_position = decimal_position as usize;
        format!(
            "{sign}{}.{}",
            &digits[..decimal_position],
            &digits[decimal_position..]
        )
    }
}
