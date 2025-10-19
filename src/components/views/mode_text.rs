use iced::widget::{container, text};
use iced::{Element, Fill, Length, Theme};

// Define the Rubik font
const RUBIK_FONT: iced::Font = iced::Font::with_name("Rubik");

pub fn create_mode_text() -> Element<'static, (), Theme> {
    container(
        text("Базовый")
            .size(20) // Match Calcutron button font size
            .font(RUBIK_FONT)
            .style(|_theme: &Theme| text::Style {
                color: Some(iced::Color::WHITE),
            }),
    )
    .width(Length::Shrink)
    .height(Length::Fixed(30.0))
    .center_y(Fill)
    .into()
}
