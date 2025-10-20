use iced::{Subscription, Task, window};
extern crate image;

use crate::components::views::{
    button_grid::create_button_grid,
    display::{create_display_container, create_history_display, create_main_display},
    layout::create_main_layout,
    mode_row::create_mode_row,
};
use crate::models::{message::Message, operation::Operation};

/// Adaptive layout sizes based on window dimensions (following Windows Calculator approach)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AdaptiveSize {
    Large,  // >=780x814
    Medium, // >=468x500
    Small,  // >=260 height
    Tiny,   // default
}

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
    // Window size for adaptive layout
    pub window_size: iced::Size,
    // Saved window size before entering always-on-top mode
    pub saved_window_size: Option<iced::Size>,
    // Current display mode (affects UI layout)
    pub compact_mode: bool,
    // Application icon
    pub app_icon: Option<winit::window::Icon>,
}

impl Default for Calcutron {
    fn default() -> Self {
        Calcutron::new().0
    }
}

/// Load application icon from ICO file
fn load_app_icon() -> Option<winit::window::Icon> {
    let icon_bytes = include_bytes!("../../static/app_icons/icon.ico");
    match image::load_from_memory_with_format(icon_bytes, image::ImageFormat::Ico) {
        Ok(img) => {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            winit::window::Icon::from_rgba(rgba.into_raw(), width, height).ok()
        }
        Err(e) => {
            eprintln!("Failed to load app icon: {}", e);
            None
        }
    }
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
            window_size: iced::Size::new(320.0, 470.0),
            saved_window_size: None,
            compact_mode: false,
            app_icon: load_app_icon(),
        };
        (calculator, Task::none())
    }

    pub fn title(&self) -> String {
        String::from("Calcutron")
    }

    /// Get the adaptive layout size based on window dimensions
    pub fn get_adaptive_size(&self) -> AdaptiveSize {
        let width = self.window_size.width;
        let height = self.window_size.height;

        if width >= 780.0 && height >= 814.0 {
            AdaptiveSize::Large
        } else if width >= 468.0 && height >= 500.0 {
            AdaptiveSize::Medium
        } else if height >= 260.0 {
            AdaptiveSize::Small
        } else {
            AdaptiveSize::Tiny
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        use crate::models::update_impl::{
            backspace_handler::handle_backspace,
            clear_handler::{handle_clear, handle_clear_entry},
            decimal_handler::handle_decimal,
            digit_handler::handle_digit,
            equals_handler::handle_equals,
            operation_handler::handle_operation,
            special_operation_handler::{
                handle_percentage, handle_plus_minus, handle_reciprocal, handle_square,
                handle_square_root,
            },
            window_handler::{handle_toggle_always_on_top, handle_window_event},
        };

        match message {
            Message::Digit(digit) => handle_digit(self, digit),
            Message::Decimal => handle_decimal(self),
            Message::Operation(op) => handle_operation(self, op),
            Message::Equals => handle_equals(self),
            Message::ClearEntry => handle_clear_entry(self),
            Message::Clear => handle_clear(self),
            Message::Backspace => handle_backspace(self),
            Message::PlusMinus => handle_plus_minus(self),
            Message::SquareRoot => handle_square_root(self),
            Message::Square => handle_square(self),
            Message::Percentage => handle_percentage(self),
            Message::Reciprocal => handle_reciprocal(self),
            Message::ShowList => {
                // TODO: Implement list functionality
                Task::none()
            }
            Message::CloseWindow => self.window_id.map(window::close).unwrap_or_else(Task::none),
            Message::BeginWindowDrag => self.window_id.map(window::drag).unwrap_or_else(Task::none),
            Message::ToggleAlwaysOnTop => handle_toggle_always_on_top(self),
            Message::WindowEvent(id, event) => handle_window_event(self, id, event),
            Message::Ignore => {
                // Do nothing
                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        // Create the mode display row with list and always on top buttons
        let mode_row = create_mode_row(self.always_on_top, self.compact_mode);

        // Create the history display (smaller font, top line) - 1/3 of total height
        let history_display = create_history_display(self.history.clone());

        // Create the main display (larger font, bottom line) - 2/3 of total height
        let main_display = create_main_display(self.display.clone());

        // Combine both displays into a single display container with specified height
        let display_container = create_display_container(history_display, main_display);

        // Create the buttons with proper centering
        let buttons = create_button_grid(self.get_adaptive_size());

        // Create the main layout with the specified proportions
        let content = create_main_layout(
            mode_row,
            display_container,
            buttons,
            self.always_on_top,
            self.compact_mode,
        );

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

    pub fn subscription(&self) -> Subscription<Message> {
        window::events().map(|(id, event)| Message::WindowEvent(id, event))
    }
}
