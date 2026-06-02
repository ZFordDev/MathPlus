use meval::eval_str;

pub fn evaluate_expression(input: &str) -> Result<f64, ()> {
    eval_str(input).map_err(|_| ())
}

pub fn format_result(result: f64) -> String {
    if result.is_nan() {
        "Error".to_string()
    } else {
        format!("= {}", result)
    }
}

pub fn format_history_result(result: f64) -> String {
    if result.is_nan() {
        "Error".to_string()
    } else {
        format!("= {:.6}", result)
    }
}
