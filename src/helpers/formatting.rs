// Helper function to format numbers for display
pub fn format_number(value: f64) -> String {
    if value.is_nan() {
        "Error".to_string()
    } else if value.is_infinite() {
        "Infinity".to_string()
    } else {
        // Format to avoid unnecessary decimal places
        if value.fract() == 0.0 && value.abs() < 1e15 {
            format!("{}", value as i64)
        } else if value.abs() >= 1e15 || (value.abs() < 1e-4 && value != 0.0) {
            format!("{:.6e}", value)
        } else {
            format!("{:.10}", value).trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }
}