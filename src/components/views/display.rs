use iced::widget::{container, column, text};
use iced::{Element, Fill, Length, Theme};

// Define the Rubik font
const RUBIK_FONT: iced::Font = iced::Font::with_name("Rubik");

pub fn create_history_display(history: String) -> Element<'static, (), Theme> {
    container(
        text(history)
            .size(14) // Reduced font size from 16 to 14
            .font(RUBIK_FONT) // Apply Rubik font
    )
    .width(Fill)
    .height(Length::Fixed(28.0)) // Adjusted height for better vertical centering
    .padding([0, 10]) // Removed vertical padding for better vertical centering
    .align_x(iced::alignment::Horizontal::Right) // Right align the text
    .align_y(iced::alignment::Vertical::Center) // Vertically center the text
    .style(|_theme: &Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgba8(26, 34, 39, 0.9))), // #1a2227 with 10% transparency (90% opacity)
        border: iced::border::Border {
            radius: 5.0.into(), // Simple rounded corners
            ..Default::default()
        },
        ..container::Style::default()
    })
    .into()
}

pub fn create_main_display(display: String) -> Element<'static, (), Theme> {
    container(
        text(display)
            .size(46) // Reverted to original font size for main display
            .font(RUBIK_FONT) // Apply Rubik font
    )
    .width(Fill)
    .height(Length::Fixed(60.0)) // Reverted to original height: 2/3 of 93.5px = 62.3px
    .padding([0, 10]) // Kept vertical centering improvement (removed vertical padding)
    .align_x(iced::alignment::Horizontal::Right) // Right align the text
    .align_y(iced::alignment::Vertical::Center) // Vertically center the text
    .style(|_theme: &Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgba8(26, 34, 39, 0.9))), // #1a2227 with 10% transparency (90% opacity)
        border: iced::border::Border {
            radius: 5.0.into(), // Simple rounded corners
            ..Default::default()
        },
        ..container::Style::default()
    })
    .into()
}

pub fn create_display_container(history_display: Element<'static, (), Theme>, main_display: Element<'static, (), Theme>) -> Element<'static, (), Theme> {
    column![
        history_display,
        main_display
    ]
    .spacing(2) // Spacing between the two lines
    .height(Length::Fixed(93.5)) // Total height: 93.5px
    .padding(0) // No padding on the container
    .into()
}