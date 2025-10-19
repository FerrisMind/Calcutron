use crate::models::operation::Operation;

pub fn calculate(first: f64, second: f64, op: Operation) -> f64 {
    match op {
        Operation::Add => first + second,
        Operation::Subtract => first - second,
        Operation::Multiply => first * second,
        Operation::Divide => {
            if second != 0.0 {
                first / second
            } else {
                // Handle division by zero
                f64::NAN
            }
        }
    }
}
