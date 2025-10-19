use iced::Task;

use crate::models::{calcutron::Calcutron, message::Message};

pub fn handle_toggle_always_on_top(calcutron: &mut Calcutron) -> Task<Message> {
    if let Some(window_id) = calcutron.window_id {
        calcutron.always_on_top = !calcutron.always_on_top;
        return iced::window::change_level(
            window_id,
            if calcutron.always_on_top {
                iced::window::Level::AlwaysOnTop
            } else {
                iced::window::Level::Normal
            },
        );
    }
    Task::none()
}

pub fn handle_window_event(calcutron: &mut Calcutron, event: iced::window::Event) -> Task<Message> {
    // Handle window events
    match event {
        iced::window::Event::CloseRequested => {
            // We don't have the window ID here, so we can't close it directly
            Task::none()
        }
        iced::window::Event::Resized(size) => {
            calcutron.window_size = size;
            Task::none()
        }
        _ => Task::none(),
    }
}
