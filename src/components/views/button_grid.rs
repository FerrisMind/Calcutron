use iced::widget::{column, row};
use iced::{Center, Element, Fill};

use crate::models::{message::Message, operation::Operation};
use crate::components::buttons::{digit_button, operator_button, equals_button, plus_minus_button};

pub fn create_button_grid() -> Element<'static, Message, iced::Theme> {
    column![
        // Row 1: Percentage, Clear Entry, Clear, Backspace
        row![
            operator_button("percent.svg", Message::Percentage).padding(15).width(Fill),
            operator_button("broom.svg", Message::ClearEntry).padding(15).width(Fill),
            operator_button("trash-simple.svg", Message::Clear).padding(15).width(Fill),
            operator_button("backspace.svg", Message::Backspace).padding(15).width(Fill),
        ].spacing(2).align_y(Center),
        
        // Row 2: Reciprocal, Square, Square Root, Division
        row![
            operator_button("fragmentation.svg", Message::Reciprocal).padding(15).width(Fill),
            operator_button("text-superscript.svg", Message::Square).padding(15).width(Fill),
            operator_button("radical.svg", Message::SquareRoot).padding(15).width(Fill),
            operator_button("divide.svg", Message::Operation(Operation::Divide)).padding(15).width(Fill),
        ].spacing(2).align_y(Center),
        
        
        // Row 3: Number buttons and operations
        row![
            digit_button("number-seven.svg", Message::Digit(7)).padding(15).width(Fill),
            digit_button("number-eight.svg", Message::Digit(8)).padding(15).width(Fill),
            digit_button("number-nine.svg", Message::Digit(9)).padding(15).width(Fill),
            operator_button("x.svg", Message::Operation(Operation::Multiply)).padding(15).width(Fill),
        ].spacing(3).align_y(Center),
        
        // Row 4
        row![
            digit_button("number-four.svg", Message::Digit(4)).padding(15).width(Fill),
            digit_button("number-five.svg", Message::Digit(5)).padding(15).width(Fill),
            digit_button("number-six.svg", Message::Digit(6)).padding(15).width(Fill),
            operator_button("minus.svg", Message::Operation(Operation::Subtract)).padding(15).width(Fill),
        ].spacing(3).align_y(Center),
        
        // Row 5
        row![
            digit_button("number-one.svg", Message::Digit(1)).padding(15).width(Fill),
            digit_button("number-two.svg", Message::Digit(2)).padding(15).width(Fill),
            digit_button("number-three.svg", Message::Digit(3)).padding(15).width(Fill),
            operator_button("plus.svg", Message::Operation(Operation::Add)).padding(15).width(Fill),
        ].spacing(3).align_y(Center),
        
        // Row 6
        row![
            plus_minus_button("plus-minus.svg", Message::PlusMinus).padding(15).width(Fill),
            digit_button("number-zero.svg", Message::Digit(0)).padding(15).width(Fill),
            digit_button("dot.svg", Message::Decimal).padding(15).width(Fill),
            equals_button("equals.svg", Message::Equals).padding(15).width(Fill),
        ].spacing(3).align_y(Center),
    ]
    .spacing(2)
    .align_x(Center)
    .height(Fill)
    .into()
}