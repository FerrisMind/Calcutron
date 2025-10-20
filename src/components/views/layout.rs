use iced::widget::{column, container, horizontal_space, row, text};
use iced::{Element, Length};

use crate::components::views::always_on_top_button::create_always_on_top_button;
use crate::models::message::Message;

pub fn create_main_layout(
    mode_row: Element<'static, Message, iced::Theme>,
    display_container: Element<'static, (), iced::Theme>,
    buttons: Element<'static, Message, iced::Theme>,
    compact_mode: bool,
) -> Element<'static, Message, iced::Theme> {
    if compact_mode {
        // In compact mode, create custom title bar identical to system one
        column![
            // Custom title bar that looks like system title bar
            container(
                row![
                    // Left side - app icon area (empty for now)
                    horizontal_space().width(Length::Fixed(16.0)),
                    // Center - window title
                    text("Calcutron")
                        .size(12)
                        .color(iced::Color::WHITE),
                    // Right side - spacing to push button to edge
                    horizontal_space(),
                    // Control button area
                    create_always_on_top_button(true, true),
                ]
                .spacing(0)
                .align_y(iced::alignment::Vertical::Center)
            )
            .width(Length::Fill)
            .height(Length::Fixed(32.0)) // Standard Windows title bar height
            .style(|_theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(30, 30, 30))), // Dark title bar color for dark theme
                border: iced::border::Border {
                    radius: 0.0.into(),
                    width: 1.0,
                    color: iced::Color::from_rgb8(60, 60, 60), // Dark border color
                },
                ..Default::default()
            })
            .padding([0, 0]),

            // Main calculator content below custom title bar
            container(
                column![
                    display_container.map(|_| Message::Ignore), // Two-line display
                    container(iced::widget::vertical_space().height(Length::Fixed(38.0)))
                        .height(Length::Fixed(38.0)), // Space between display and buttons
                    container(buttons).height(Length::FillPortion(75)), // More space for buttons
                    iced::widget::vertical_space().height(Length::Fixed(4.0)), // Margin below
                ]
                .height(Length::Fill)
            )
            .height(Length::Fill)
            .padding([0, 4])
        ]
        .height(Length::Fill)
        .into()
    } else {
        // Normal mode with mode row
        column![
            mode_row, // Add the mode row here - moved to top
            container(iced::widget::vertical_space().height(Length::Fixed(15.0)))
                .height(Length::Fixed(15.0)), // Increased space between mode row and display
            container(iced::widget::vertical_space().height(Length::Fixed(6.0)))
                .height(Length::Fixed(6.0)), // Additional space to move display lower
            display_container.map(|_| Message::Ignore), // Two-line display with height of 93.5px
            container(iced::widget::vertical_space().height(Length::Fixed(38.0)))
                .height(Length::Fixed(38.0)), // Reduced from 40px to 20px bottom space
            container(buttons).height(Length::FillPortion(65)), // 65% for buttons
            iced::widget::vertical_space().height(Length::Fixed(4.0)), // 4px margin below button block
        ]
        .padding([0, 4]) // Only horizontal padding
        .height(Length::Fill) // Allow the entire content to fill available space
        .into()
    }
}
