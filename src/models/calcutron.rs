use iced::{Task, window};

use crate::models::{operation::Operation, message::Message};
use crate::components::{
    views::{
        mode_row::create_mode_row,
        display::{create_history_display, create_main_display, create_display_container},
        button_grid::create_button_grid,
        layout::create_main_layout
    }
};

#[derive(Default)]
pub struct Calcutron {
    // Current display value
    pub display: String,
    // History display value (previous operations)
    pub history: String,
    // First operand for binary operations
    pub first_operand: Option<f64>,
    // Current operation
    pub operation: Option<Operation>,
    // Whether we're waiting for the next operand
    pub waiting_for_operand: bool,
    // Always on top state
    pub always_on_top: bool,
    // Window ID
    pub window_id: Option<window::Id>,
}

impl Calcutron {
    pub fn new() -> (Self, Task<Message>) {
        let calculator = Calcutron { 
            display: "0".to_string(), 
            history: "".to_string(), 
            always_on_top: false,
            first_operand: None,
            operation: None,
            waiting_for_operand: false,
            window_id: None,
        };
        (calculator, Task::none())
    }

    pub fn title(&self) -> String {
        String::from("Calcutron")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        use crate::models::update_impl::{
            digit_handler::handle_digit,
            decimal_handler::handle_decimal,
            operation_handler::handle_operation,
            equals_handler::handle_equals,
            clear_handler::{handle_clear_entry, handle_clear},
            backspace_handler::handle_backspace,
            special_operation_handler::{handle_plus_minus, handle_square_root, handle_square, handle_percentage, handle_reciprocal},
            window_handler::{handle_toggle_always_on_top, handle_window_event}
        };

        match message {
            Message::Digit(digit) => {
                handle_digit(self, digit)
            }
            Message::Decimal => {
                handle_decimal(self)
            }
            Message::Operation(op) => {
                handle_operation(self, op)
            }
            Message::Equals => {
                handle_equals(self)
            }
            Message::ClearEntry => {
                handle_clear_entry(self)
            }
            Message::Clear => {
                handle_clear(self)
            }
            Message::Backspace => {
                handle_backspace(self)
            }
            Message::PlusMinus => {
                handle_plus_minus(self)
            }
            Message::SquareRoot => {
                handle_square_root(self)
            }
            Message::Square => {
                handle_square(self)
            }
            Message::Percentage => {
                handle_percentage(self)
            }
            Message::Reciprocal => {
                handle_reciprocal(self)
            }
            Message::ShowList => {
                // TODO: Implement list functionality
                Task::none()
            }
            Message::ToggleAlwaysOnTop => {
                handle_toggle_always_on_top(self)
            }
            Message::WindowEvent(event) => {
                handle_window_event(self, event)
            }
            Message::Ignore => {
                // Do nothing
                Task::none()
            }
        }
    }
    
    pub fn view(&self) -> iced::Element<'_, Message> {
        // Create the mode display row with list and always on top buttons
        let mode_row = create_mode_row(self.always_on_top);
        
        // Create the history display (smaller font, top line) - 1/3 of total height
        let history_display = create_history_display(self.history.clone());
        
        // Create the main display (larger font, bottom line) - 2/3 of total height
        let main_display = create_main_display(self.display.clone());
        
        // Combine both displays into a single display container with specified height
        let display_container = create_display_container(history_display, main_display);
        
        // Create the buttons with proper centering
        let buttons = create_button_grid();
        
        // Create the main layout with the specified proportions
        let content = create_main_layout(mode_row, display_container, buttons);
        
        iced::widget::container(content)
            .width(iced::Fill) // Allow width to adapt to window size
            .height(iced::Fill) // Allow height to adapt to window size
            .center_x(iced::Fill)
            .center_y(iced::Fill)
            .style(|_theme: &iced::Theme| iced::widget::container::Style {
                // Remove the background entirely to allow window transparency to show through
                background: None,
                border: iced::border::Border {
                    radius: 5.0.into(),
                    ..Default::default()
                },
                ..iced::widget::container::Style::default()
            })
            .into()
    }
}