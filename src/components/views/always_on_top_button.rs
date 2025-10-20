use iced::widget::svg::Handle;
use iced::widget::{button, container, svg};
use iced::{Element, Fill, Length, Theme};

use crate::models::message::Message;

pub fn create_always_on_top_button(
    always_on_top: bool,
    compact_mode: bool,
) -> Element<'static, Message, Theme> {
    let icon_path = if always_on_top {
        "static/icons/arrow-square-in.svg"
    } else {
        "static/icons/arrow-square-out.svg"
    };

    let icon_size = if compact_mode { 16.0 } else { 40.0 }; // Small icon for title bar button
    let button_width = if compact_mode { 45.0 } else { icon_size + 4.0 }; // System button width in compact mode
    let button_height = if compact_mode { 32.0 } else { icon_size + 4.0 }; // Title bar height in compact mode

    button(
        container(
            svg(Handle::from_path(icon_path))
                .width(Length::Fixed(icon_size))
                .height(Length::Fixed(icon_size))
                .style(
                    |_theme: &Theme, _status: iced::widget::svg::Status| svg::Style {
                        color: Some(iced::Color::WHITE),
                    },
                ),
        )
        .width(Length::Fixed(icon_size))
        .height(Length::Fixed(icon_size))
        .center_x(Fill)
        .center_y(Fill),
    )
    .width(Length::Fixed(button_width))
    .height(Length::Fixed(button_height))
    .style(
        move |_theme: &Theme, status: iced::widget::button::Status| {
            if compact_mode {
                // System title bar button style for dark theme
                let background_color = match status {
                    iced::widget::button::Status::Hovered => iced::Color::from_rgb8(70, 70, 70), // Dark gray hover for dark theme
                    _ => iced::Color::TRANSPARENT,
                };

                iced::widget::button::Style {
                    background: Some(iced::Background::Color(background_color)),
                    text_color: iced::Color::WHITE,
                    border: iced::border::Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: iced::Color::TRANSPARENT,
                    },
                    shadow: Default::default(),
                }
            } else {
                // Original button style for normal mode
                let background_color = match status {
                    iced::widget::button::Status::Hovered => iced::Color::from_rgb8(70, 70, 70),
                    _ => iced::Color::from_rgb8(35, 35, 35),
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
            }
        },
    )
    .on_press(Message::ToggleAlwaysOnTop)
    .into()
}

pub fn create_close_button(compact_mode: bool) -> Element<'static, Message, Theme> {
    let icon_size = if compact_mode { 16.0 } else { 40.0 };
    let button_width = if compact_mode { 45.0 } else { icon_size + 4.0 };
    let button_height = if compact_mode { 32.0 } else { icon_size + 4.0 };

    button(
        container(
            svg(Handle::from_path("static/icons/x.svg"))
                .width(Length::Fixed(icon_size))
                .height(Length::Fixed(icon_size))
                .style(
                    |_theme: &Theme, _status: iced::widget::svg::Status| svg::Style {
                        color: Some(iced::Color::WHITE),
                    },
                ),
        )
        .width(Length::Fixed(icon_size))
        .height(Length::Fixed(icon_size))
        .center_x(Fill)
        .center_y(Fill),
    )
    .width(Length::Fixed(button_width))
    .height(Length::Fixed(button_height))
    .style(
        move |_theme: &Theme, status: iced::widget::button::Status| {
            if compact_mode {
                let background_color = match status {
                    iced::widget::button::Status::Hovered => iced::Color::from_rgb8(196, 59, 59),
                    _ => iced::Color::TRANSPARENT,
                };

                iced::widget::button::Style {
                    background: Some(iced::Background::Color(background_color)),
                    text_color: iced::Color::WHITE,
                    border: iced::border::Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: iced::Color::TRANSPARENT,
                    },
                    shadow: Default::default(),
                }
            } else {
                let background_color = match status {
                    iced::widget::button::Status::Hovered => iced::Color::from_rgb8(70, 70, 70),
                    _ => iced::Color::from_rgb8(35, 35, 35),
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
            }
        },
    )
    .on_press(Message::CloseWindow)
    .into()
}
