use iced::Task;

use crate::models::{calcutron::Calcutron, message::Message};

pub fn handle_clear_entry(calcutron: &mut Calcutron) -> Task<Message> {
    calcutron.display = "0".to_string();
    Task::none()
}

pub fn handle_clear(calcutron: &mut Calcutron) -> Task<Message> {
    calcutron.display = "0".to_string();
    calcutron.history = "".to_string();
    calcutron.first_operand = None;
    calcutron.operation = None;
    calcutron.waiting_for_operand = false;
    Task::none()
}
