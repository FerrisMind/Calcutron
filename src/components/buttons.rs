use iced::widget::{button, container, svg};
use iced::{Length, Theme};
use iced::widget::svg::Handle;

use crate::models::message::Message;

// Helper function to create digit buttons with custom styling
pub fn digit_button(icon_path: &str, message: Message) -> button::Button<'_, Message> {
    let icon_handle = Handle::from_path(format!("static/icons/{}", icon_path));
    button(
        container(svg(icon_handle)
            .width(Length::Fixed(22.0))
            .height(Length::Fixed(22.0))
            .style(|_theme: &Theme, _status: iced::widget::svg::Status| {
                // Use the same color as the display text (white in dark theme)
                svg::Style { color: Some(iced::Color::WHITE) }
            }))
            .width(Length::Fixed(22.0))
            .height(Length::Fixed(22.0))
            .center_x(iced::Fill)
            .center_y(iced::Fill)
    )
        .width(iced::Fill)
        .height(iced::Fill)
        .style(|_theme: &Theme, status: iced::widget::button::Status| {
            let background_color = match status {
                iced::widget::button::Status::Hovered => iced::Color::from_rgb8(50, 50, 50), // #323232 (hover)
                _ => iced::Color::from_rgb8(59, 59, 59), // #3b3b3b (normal)
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
        .on_press(message)
}

// Helper function to create operator buttons with custom styling (inverse of digit buttons)
pub fn operator_button(icon_path: &str, message: Message) -> button::Button<'_, Message> {
    let icon_handle = Handle::from_path(format!("static/icons/{}", icon_path));
    button(
        container(svg(icon_handle)
            .width(Length::Fixed(22.0))
            .height(Length::Fixed(22.0))
            .style(|_theme: &Theme, _status: iced::widget::svg::Status| {
                // Use the same color as the display text (white in dark theme)
                svg::Style { color: Some(iced::Color::WHITE) }
            }))
            .width(Length::Fixed(22.0))
            .height(Length::Fixed(22.0))
            .center_x(iced::Fill)
            .center_y(iced::Fill)
    )
        .width(iced::Fill)
        .height(iced::Fill)
        .style(|_theme: &Theme, status: iced::widget::button::Status| {
            let background_color = match status {
                iced::widget::button::Status::Hovered => iced::Color::from_rgb8(59, 59, 59), // #3b3b3b (hover)
                _ => iced::Color::from_rgb8(50, 50, 50), // #323232 (normal)
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
        .on_press(message)
}

// Helper function to create the equals button with custom styling (keeping original icon color)
pub fn equals_button(icon_path: &str, message: Message) -> button::Button<'_, Message> {
    let icon_handle = Handle::from_path(format!("static/icons/{}", icon_path));
    button(
        container(svg(icon_handle)
            .width(Length::Fixed(22.0))
            .height(Length::Fixed(22.0)))
            .width(Length::Fixed(22.0))
            .height(Length::Fixed(22.0))
            .center_x(iced::Fill)
            .center_y(iced::Fill)
    )
        .width(iced::Fill)
        .height(iced::Fill)
        .style(|_theme: &Theme, status: iced::widget::button::Status| {
            let background_color = match status {
                iced::widget::button::Status::Hovered => iced::Color::from_rgb8(71, 177, 232), // #47b1e8 (hover)
                _ => iced::Color::from_rgb8(76, 194, 255), // #4cc2ff (normal)
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
        .on_press(message)
}

// Helper function to create the plusminus button with swapped colors
pub fn plus_minus_button(icon_path: &str, message: Message) -> button::Button<'_, Message> {
    let icon_handle = Handle::from_path(format!("static/icons/{}", icon_path));
    button(
        container(svg(icon_handle)
            .width(Length::Fixed(22.0))
            .height(Length::Fixed(22.0))
            .style(|_theme: &Theme, _status: iced::widget::svg::Status| {
                // Use the same color as the display text (white in dark theme)
                svg::Style { color: Some(iced::Color::WHITE) }
            }))
            .width(Length::Fixed(22.0))
            .height(Length::Fixed(22.0))
            .center_x(iced::Fill)
            .center_y(iced::Fill)
    )
        .width(iced::Fill)
        .height(iced::Fill)
        .style(|_theme: &Theme, status: iced::widget::button::Status| {
            let background_color = match status {
                iced::widget::button::Status::Hovered => iced::Color::from_rgb8(50, 50, 50), // #323232 (hover - was normal)
                _ => iced::Color::from_rgb8(59, 59, 59), // #3b3b3b (normal - was hover)
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
        .on_press(message)
}