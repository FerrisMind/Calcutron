// Import the Calcutron application
use calcutron::models::calcutron::Calcutron;

// Main application entry point
fn main() -> iced::Result {
    iced::application("Calcutron", Calcutron::update, Calcutron::view)
        .window(iced::window::Settings {
            size: iced::Size::new(320.0, 470.0),
            min_size: Some(iced::Size::new(320.0, 470.0)),
            ..Default::default()
        })
        .font(include_bytes!("../static/fonts/Rubik-Regular.ttf").as_slice())
        .theme(|_state: &Calcutron| iced::Theme::Dark)
        .subscription(Calcutron::subscription)
        .run()
}
