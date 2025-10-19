use iced::Task;

use crate::models::{calcutron::Calcutron, message::Message, operation::Operation};

pub fn handle_decimal(calcutron: &mut Calcutron) -> Task<Message> {
    if calcutron.waiting_for_operand {
        // Move current display to history with operation
        if let Some(op) = calcutron.operation {
            let op_symbol = match op {
                Operation::Add => "+",
                Operation::Subtract => "-",
                Operation::Multiply => "×",
                Operation::Divide => "÷",
            };
            calcutron.history = format!("{} {} ", calcutron.display, op_symbol);
        }
        calcutron.display = "0.".to_string();
        calcutron.waiting_for_operand = false;
    } else if !calcutron.display.contains('.') {
        calcutron.display.push('.');
    }
    Task::none()
}
