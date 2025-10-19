// Import our modules
use calcutron::models::{calcutron::Calcutron, message::Message};

fn main() -> iced::Result {
    iced::application(Calcutron::title, Calcutron::update, Calcutron::view)
        .subscription(|_state: &Calcutron| {
            iced::event::listen_with(|event, _status, _id| {
                if let iced::Event::Window(window_event) = event {
                    Some(Message::WindowEvent(window_event))
                } else {
                    Some(Message::Ignore)
                }
            })
        })
        .window(iced::window::Settings {
            size: iced::Size::new(320.0, 470.0),
            min_size: Some(iced::Size::new(320.0, 470.0)),
            ..Default::default()
        })
        .font(include_bytes!("../static/fonts/Rubik-Regular.ttf").as_slice())
        .theme(|_state: &Calcutron| iced::Theme::Dark)
        .run_with(Calcutron::new)
}