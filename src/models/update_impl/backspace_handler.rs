use iced::Task;

use crate::models::{calcutron::Calcutron, message::Message};

pub fn handle_backspace(calcutron: &mut Calcutron) -> Task<Message> {
    if calcutron.display.len() > 1 {
        calcutron.display.pop();
    } else {
        calcutron.display = "0".to_string();
    }
    Task::none()
}