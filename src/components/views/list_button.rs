use iced::widget::svg::Handle;
use iced::widget::{button, container, svg};
use iced::{Element, Fill, Length, Theme};

use crate::models::message::Message;

pub fn create_list_button() -> Element<'static, Message, Theme> {
    button(
        container(
            svg(Handle::from_path("static/icons/list.svg"))
                .width(Length::Fixed(40.0)) // Icon size 40x40 - increased
                .height(Length::Fixed(40.0)) // Icon size 40x40 - increased
                .style(
                    |_theme: &Theme, _status: iced::widget::svg::Status| svg::Style {
                        color: Some(iced::Color::WHITE),
                    },
                ),
        )
        .width(Length::Fixed(40.0)) // Container size 40x40 - increased
        .height(Length::Fixed(40.0)) // Container size 40x40 - increased
        .center_x(Fill)
        .center_y(Fill),
    )
    .width(Length::Fixed(42.0)) // Button size 42x42 - proper frame for 40x40 icon
    .height(Length::Fixed(42.0)) // Button size 42x42 - proper frame for 40x40 icon
    .style(|_theme: &Theme, status: iced::widget::button::Status| {
        // Keep hover effect but remove default highlighting
        let background_color = match status {
            iced::widget::button::Status::Hovered => iced::Color::from_rgb8(50, 50, 50), // Hover color
            _ => iced::Color::from_rgb8(35, 35, 35), // Normal color (darker to remove highlighting)
        };

        iced::widget::button::Style {
            background: Some(iced::Background::Color(background_color)),
            text_color: iced::Color::WHITE,
            border: iced::border::Border {
                radius: 5.0.into(),
                ..Default::default()
            },
            shadow: Default::default(),
        }
    })
    .on_press(Message::ShowList)
    .into()
}
