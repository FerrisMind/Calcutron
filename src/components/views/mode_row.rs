use iced::widget::{Space, container, row};
use iced::{Center, Element, Fill, Length, Theme};

use crate::components::views::{
    always_on_top_button::create_always_on_top_button, list_button::create_list_button,
    mode_text::create_mode_text,
};
use crate::models::message::Message;

pub fn create_mode_row(
    always_on_top: bool,
    compact_mode: bool,
) -> Element<'static, Message, Theme> {
    if compact_mode {
        // In compact mode, don't show any mode row - button is overlayed
        container(iced::widget::vertical_space().height(Length::Fixed(0.0)))
            .width(Fill)
            .height(Length::Fixed(0.0))
            .into()
    } else {
        // In normal mode, show all controls
        container(
            row![
                // List button
                create_list_button(),
                // Mode text
                create_mode_text().map(|_| Message::Ignore),
                // Small space to move the always-on-top button slightly to the right
                Space::with_width(Length::Fixed(12.0)),
                // Always on top toggle button
                create_always_on_top_button(always_on_top, compact_mode),
            ]
            .spacing(2)
            .align_y(Center),
        )
        .width(Fill)
        .height(Length::Fixed(46.0))
        .padding([0, 4])
        .into()
    }
}
