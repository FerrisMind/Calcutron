use iced::Task;

use crate::models::{calcutron::Calcutron, message::Message, operation::Operation};
use crate::helpers::{formatting::format_number, calculation::calculate};

pub fn handle_operation(calcutron: &mut Calcutron, op: Operation) -> Task<Message> {
    if let Ok(value) = calcutron.display.parse::<f64>() {
        if let Some(first) = calcutron.first_operand {
            if let Some(current_op) = calcutron.operation {
                // Perform the previous operation
                let result = calculate(first, value, current_op);
                calcutron.display = format_number(result);
                // Update history with the operation
                let op_symbol = match current_op {
                    Operation::Add => "+",
                    Operation::Subtract => "-",
                    Operation::Multiply => "×",
                    Operation::Divide => "÷",
                };
                calcutron.history = format!("{} {} {} = ", first, op_symbol, value);
                calcutron.first_operand = Some(result);
            }
        } else {
            calcutron.first_operand = Some(value);
            // Update history with the operation
            let op_symbol = match op {
                Operation::Add => "+",
                Operation::Subtract => "-",
                Operation::Multiply => "×",
                Operation::Divide => "÷",
            };
            calcutron.history = format!("{} {} ", value, op_symbol);
        }
        calcutron.operation = Some(op);
        calcutron.waiting_for_operand = true;
    }
    Task::none()
}