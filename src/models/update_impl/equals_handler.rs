use iced::Task;

use crate::helpers::{calculation::calculate, formatting::format_number};
use crate::models::{calcutron::Calcutron, message::Message, operation::Operation};

pub fn handle_equals(calcutron: &mut Calcutron) -> Task<Message> {
    if let (Ok(value), Some(first), Some(op)) = (
        calcutron.display.parse::<f64>(),
        calcutron.first_operand,
        calcutron.operation,
    ) {
        let result = calculate(first, value, op);
        // Update history with the full expression
        let op_symbol = match op {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "×",
            Operation::Divide => "÷",
        };
        calcutron.history = format!("{} {} {} = ", first, op_symbol, value);
        calcutron.display = format_number(result);
        calcutron.first_operand = None;
        calcutron.operation = None;
        calcutron.waiting_for_operand = true;
    }
    Task::none()
}
