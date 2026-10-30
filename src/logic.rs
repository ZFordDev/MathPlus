use meval::eval_str;

pub fn evaluate_expression(input: &str) -> Result<f64, ()> {
    eval_str(input).map_err(|_| ())
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
