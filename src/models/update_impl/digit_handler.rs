use iced::Task;

use crate::models::{calcutron::Calcutron, message::Message, operation::Operation};

pub fn handle_digit(calcutron: &mut Calcutron, digit: u8) -> Task<Message> {
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
        calcutron.display = digit.to_string();
        calcutron.waiting_for_operand = false;
    } else if calcutron.display == "0" {
        calcutron.display = digit.to_string();
    } else {
        calcutron.display.push_str(&digit.to_string());
    }
    Task::none()
}
