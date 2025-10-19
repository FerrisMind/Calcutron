use iced::Task;

use crate::helpers::formatting::format_number;
use crate::models::{calcutron::Calcutron, message::Message};

pub fn handle_plus_minus(calcutron: &mut Calcutron) -> Task<Message> {
    if let Ok(value) = calcutron.display.parse::<f64>() {
        calcutron.display = format_number(-value);
    }
    Task::none()
}

pub fn handle_square_root(calcutron: &mut Calcutron) -> Task<Message> {
    if let Ok(value) = calcutron.display.parse::<f64>() {
        if value >= 0.0 {
            calcutron.display = format_number(value.sqrt());
        } else {
            calcutron.display = "Invalid input".to_string();
        }
    }
    Task::none()
}

pub fn handle_square(calcutron: &mut Calcutron) -> Task<Message> {
    if let Ok(value) = calcutron.display.parse::<f64>() {
        calcutron.display = format_number(value * value);
    }
    Task::none()
}

pub fn handle_percentage(calcutron: &mut Calcutron) -> Task<Message> {
    if let Ok(value) = calcutron.display.parse::<f64>() {
        calcutron.display = format_number(value / 100.0);
    }
    Task::none()
}

pub fn handle_reciprocal(calcutron: &mut Calcutron) -> Task<Message> {
    if let Ok(value) = calcutron.display.parse::<f64>() {
        if value != 0.0 {
            calcutron.display = format_number(1.0 / value);
        } else {
            calcutron.display = "Cannot divide by zero".to_string();
        }
    }
    Task::none()
}
