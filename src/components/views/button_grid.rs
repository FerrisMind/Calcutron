use iced::widget::{column, row};
use iced::{Center, Element, Fill};

use crate::components::buttons::{digit_button, equals_button, operator_button, plus_minus_button};
use crate::models::{calcutron::AdaptiveSize, message::Message, operation::Operation};

pub fn create_button_grid(adaptive_size: AdaptiveSize) -> Element<'static, Message, iced::Theme> {
    column![
        // Row 1: Percentage, Clear Entry, Clear, Backspace
        row![
            operator_button("percent.svg", Message::Percentage, adaptive_size).width(Fill),
            operator_button("broom.svg", Message::ClearEntry, adaptive_size).width(Fill),
            operator_button("trash-simple.svg", Message::Clear, adaptive_size).width(Fill),
            operator_button("backspace.svg", Message::Backspace, adaptive_size).width(Fill),
        ]
        .spacing(2)
        .align_y(Center)
        .height(Fill),
        // Row 2: Reciprocal, Square, Square Root, Division
        row![
            operator_button("fragmentation.svg", Message::Reciprocal, adaptive_size).width(Fill),
            operator_button("text-superscript.svg", Message::Square, adaptive_size).width(Fill),
            operator_button("radical.svg", Message::SquareRoot, adaptive_size).width(Fill),
            operator_button(
                "divide.svg",
                Message::Operation(Operation::Divide),
                adaptive_size
            )
            .width(Fill),
        ]
        .spacing(1)
        .align_y(Center)
        .height(Fill),
        // Row 3: Number buttons and operations
        row![
            digit_button("number-seven.svg", Message::Digit(7), adaptive_size).width(Fill),
            digit_button("number-eight.svg", Message::Digit(8), adaptive_size).width(Fill),
            digit_button("number-nine.svg", Message::Digit(9), adaptive_size).width(Fill),
            operator_button(
                "x.svg",
                Message::Operation(Operation::Multiply),
                adaptive_size
            )
            .width(Fill),
        ]
        .spacing(1)
        .align_y(Center),
        // Row 4
        row![
            digit_button("number-four.svg", Message::Digit(4), adaptive_size).width(Fill),
            digit_button("number-five.svg", Message::Digit(5), adaptive_size).width(Fill),
            digit_button("number-six.svg", Message::Digit(6), adaptive_size).width(Fill),
            operator_button(
                "minus.svg",
                Message::Operation(Operation::Subtract),
                adaptive_size
            )
            .width(Fill),
        ]
        .spacing(1)
        .align_y(Center),
        // Row 5
        row![
            digit_button("number-one.svg", Message::Digit(1), adaptive_size).width(Fill),
            digit_button("number-two.svg", Message::Digit(2), adaptive_size).width(Fill),
            digit_button("number-three.svg", Message::Digit(3), adaptive_size).width(Fill),
            operator_button(
                "plus.svg",
                Message::Operation(Operation::Add),
                adaptive_size
            )
            .width(Fill),
        ]
        .spacing(1)
        .align_y(Center),
        // Row 6
        row![
            plus_minus_button("plus-minus.svg", Message::PlusMinus, adaptive_size).width(Fill),
            digit_button("number-zero.svg", Message::Digit(0), adaptive_size).width(Fill),
            digit_button("dot.svg", Message::Decimal, adaptive_size).width(Fill),
            equals_button("equals.svg", Message::Equals, adaptive_size).width(Fill),
        ]
        .spacing(1)
        .align_y(Center),
    ]
    .spacing(1)
    .align_x(Center)
    .height(Fill)
    .into()
}
