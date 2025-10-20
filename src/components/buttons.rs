use iced::widget::svg::Handle;
use iced::widget::{button, container, svg};
use iced::{Length, Theme};

use crate::models::{calcutron::AdaptiveSize, message::Message};

// Helper function to get fixed icon size
fn get_icon_size(_adaptive_size: AdaptiveSize) -> f32 {
    16.0 // Fixed size 20px
}

// Helper function to create digit buttons with custom styling
pub fn digit_button(
    icon_path: &str,
    message: Message,
    adaptive_size: crate::models::calcutron::AdaptiveSize,
) -> iced::widget::Container<'_, Message, iced::Theme> {
    let icon_handle = Handle::from_path(format!("static/icons/{}", icon_path));
    container(
        button(
            container(
                svg(icon_handle)
                    .width(Length::Fixed(get_icon_size(adaptive_size)))
                    .height(Length::Fixed(get_icon_size(adaptive_size)))
                    .style(|_theme: &Theme, _status: iced::widget::svg::Status| {
                        // Use the same color as the display text (white in dark theme)
                        svg::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }),
            )
            .width(iced::Fill)
            .height(iced::Fill)
            .center_x(iced::Fill)
            .center_y(iced::Fill),
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
        .on_press(message),
    )
    .width(iced::Fill)
    .height(iced::Fill)
    .padding(1.5) // 1.5px padding for digit buttons (3px total spacing between buttons)
}

// Helper function to create operator buttons with custom styling (inverse of digit buttons)
pub fn operator_button(
    icon_path: &str,
    message: Message,
    adaptive_size: crate::models::calcutron::AdaptiveSize,
    padding: f32,
) -> iced::widget::Container<'_, Message, iced::Theme> {
    let icon_handle = Handle::from_path(format!("static/icons/{}", icon_path));
    container(
        button(
            container(
                svg(icon_handle)
                    .width(Length::Fixed(get_icon_size(adaptive_size)))
                    .height(Length::Fixed(get_icon_size(adaptive_size)))
                    .style(|_theme: &Theme, _status: iced::widget::svg::Status| {
                        // Use the same color as the display text (white in dark theme)
                        svg::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }),
            )
            .width(iced::Fill)
            .height(iced::Fill)
            .center_x(iced::Fill)
            .center_y(iced::Fill),
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
        .on_press(message),
    )
    .width(iced::Fill)
    .height(iced::Fill)
    .padding(padding)
}

// Helper function to create the equals button with custom styling (keeping original icon color)
pub fn equals_button(
    icon_path: &str,
    message: Message,
    adaptive_size: crate::models::calcutron::AdaptiveSize,
    padding: f32,
) -> iced::widget::Container<'_, Message, iced::Theme> {
    let icon_handle = Handle::from_path(format!("static/icons/{}", icon_path));
    container(
        button(
            container(
                svg(icon_handle)
                    .width(Length::Fixed(get_icon_size(adaptive_size)))
                    .height(Length::Fixed(get_icon_size(adaptive_size))),
            )
            .width(iced::Fill)
            .height(iced::Fill)
            .center_x(iced::Fill)
            .center_y(iced::Fill),
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
        .on_press(message),
    )
    .width(iced::Fill)
    .height(iced::Fill)
    .padding(padding)
}

// Helper function to create the plusminus button with swapped colors
pub fn plus_minus_button(
    icon_path: &str,
    message: Message,
    adaptive_size: crate::models::calcutron::AdaptiveSize,
    padding: f32,
) -> iced::widget::Container<'_, Message, iced::Theme> {
    let icon_handle = Handle::from_path(format!("static/icons/{}", icon_path));
    container(
        button(
            container(
                svg(icon_handle)
                    .width(Length::Fixed(get_icon_size(adaptive_size)))
                    .height(Length::Fixed(get_icon_size(adaptive_size)))
                    .style(|_theme: &Theme, _status: iced::widget::svg::Status| {
                        // Use the same color as the display text (white in dark theme)
                        svg::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }),
            )
            .width(iced::Fill)
            .height(iced::Fill)
            .center_x(iced::Fill)
            .center_y(iced::Fill),
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
        .on_press(message),
    )
    .width(iced::Fill)
    .height(iced::Fill)
    .padding(padding)
}
