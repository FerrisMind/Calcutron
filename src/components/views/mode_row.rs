use iced::widget::{container, row};
use iced::{Center, Element, Fill, Length, Theme};

use crate::components::views::{
    always_on_top_button::create_always_on_top_button, list_button::create_list_button,
    mode_text::create_mode_text,
};
use crate::models::message::Message;

pub fn create_mode_row(always_on_top: bool) -> Element<'static, Message, Theme> {
    container(
        row![
            // List button
            create_list_button(),
            // Mode text
            create_mode_text().map(|_| Message::Ignore),
            // Always on top toggle button
            create_always_on_top_button(always_on_top),
        ]
        .spacing(2)
        .align_y(Center),
    )
    .width(Fill)
    .height(Length::Fixed(30.0))
    .padding([0, 4])
    .into()
}
