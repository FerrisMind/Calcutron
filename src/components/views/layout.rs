use iced::widget::{column, container};
use iced::{Element, Length};

use crate::models::message::Message;

pub fn create_main_layout(mode_row: Element<'static, Message, iced::Theme>, display_container: Element<'static, (), iced::Theme>, buttons: Element<'static, Message, iced::Theme>) -> Element<'static, Message, iced::Theme> {
    column![
        container(iced::widget::vertical_space().height(Length::Fixed(15.0))).height(Length::Fixed(15.0)), // Increased top space to move mode row lower
        mode_row.map(|_| Message::Ignore), // Add the mode row here
        container(iced::widget::vertical_space().height(Length::Fixed(15.0))).height(Length::Fixed(15.0)), // Increased space between mode row and display
        container(iced::widget::vertical_space().height(Length::Fixed(6.0))).height(Length::Fixed(6.0)), // Additional space to move display lower
        display_container.map(|_| Message::Ignore), // Two-line display with height of 93.5px
        container(iced::widget::vertical_space().height(Length::Fixed(38.0))).height(Length::Fixed(38.0)), // Reduced from 40px to 20px bottom space
        container(buttons).height(Length::FillPortion(65)), // 65% for buttons
        iced::widget::vertical_space().height(Length::Fixed(4.0)), // 4px margin below button block
    ]
    .padding([0, 4]) // Only horizontal padding
    .height(Length::Fill) // Allow the entire content to fill available space
    .into()
}