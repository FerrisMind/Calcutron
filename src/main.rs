use iced::widget::{
    button, column, container, row, text, svg,
};
use iced::{Center, Element, Fill, Length, Task, Theme};
use iced::widget::svg::Handle;
use iced::window;

// Define the Rubik font
const RUBIK_FONT: iced::Font = iced::Font::with_name("Rubik");

// Define Calcutron state
#[derive(Default)]
struct Calcutron {
    // Current display value
    display: String,
    // History display value (previous operations)
    history: String,
    // First operand for binary operations
    first_operand: Option<f64>,
    // Current operation
    operation: Option<Operation>,
    // Whether we're waiting for the next operand
    waiting_for_operand: bool,
    // Always on top state
    always_on_top: bool,
    // Window ID
    window_id: Option<window::Id>,
}

// Define operations
#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Define messages
#[derive(Debug, Clone)]
enum Message {
    // Digit buttons (0-9)
    Digit(u8),
    // Decimal point
    Decimal,
    // Operations
    Operation(Operation),
    // Equals button
    Equals,
    // Clear entry (CE)
    ClearEntry,
    // Clear (C)
    Clear,
    // Backspace
    Backspace,
    // Plus/Minus
    PlusMinus,
    // Square root
    SquareRoot,
    // Square (x²)
    Square,
    // Percentage
    Percentage,
    // Reciprocal
    Reciprocal,
    // List button
    ShowList,
    // Toggle always on top
    ToggleAlwaysOnTop,
    // Window event
    WindowEvent(window::Event),
    // Ignore event
    Ignore,
}

impl Calcutron {
    fn new() -> (Self, Task<Message>) {
        let mut calculator = Calcutron::default();
        calculator.display = "0".to_string();
        calculator.history = "".to_string();
        calculator.always_on_top = false;
        // Initialize window_id - this will be set when the window is created
        calculator.window_id = Some(window::Id::unique());
        (calculator, Task::none())
    }

    fn title(&self) -> String {
        String::from("Calcutron")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Digit(digit) => {
                if self.waiting_for_operand {
                    // Move current display to history with operation
                    if let Some(op) = self.operation {
                        let op_symbol = match op {
                            Operation::Add => "+",
                            Operation::Subtract => "-",
                            Operation::Multiply => "×",
                            Operation::Divide => "÷",
                        };
                        self.history = format!("{} {} ", self.display, op_symbol);
                    }
                    self.display = digit.to_string();
                    self.waiting_for_operand = false;
                } else {
                    if self.display == "0" {
                        self.display = digit.to_string();
                    } else {
                        self.display.push_str(&digit.to_string());
                    }
                }
                Task::none()
            }
            Message::Decimal => {
                if self.waiting_for_operand {
                    // Move current display to history with operation
                    if let Some(op) = self.operation {
                        let op_symbol = match op {
                            Operation::Add => "+",
                            Operation::Subtract => "-",
                            Operation::Multiply => "×",
                            Operation::Divide => "÷",
                        };
                        self.history = format!("{} {} ", self.display, op_symbol);
                    }
                    self.display = "0.".to_string();
                    self.waiting_for_operand = false;
                } else {
                    if !self.display.contains('.') {
                        self.display.push('.');
                    }
                }
                Task::none()
            }
            Message::Operation(op) => {
                if let Ok(value) = self.display.parse::<f64>() {
                    if let Some(first) = self.first_operand {
                        if let Some(current_op) = self.operation {
                            // Perform the previous operation
                            match self.calculate(first, value, current_op) {
                                Ok(result) => {
                                    self.display = format_number(result);
                                }
                                Err(error) => {
                                    self.display = error;
                                    self.first_operand = None;
                                    self.operation = None;
                                    self.waiting_for_operand = true;
                                    return Task::none();
                                }
                            }
                            // Update history with the operation
                            let op_symbol = match current_op {
                                Operation::Add => "+",
                                Operation::Subtract => "-",
                                Operation::Multiply => "×",
                                Operation::Divide => "÷",
                            };
                            self.history = format!("{} {} {} = ", first, op_symbol, value);
                            self.first_operand = Some(result);
                        }
                    } else {
                        self.first_operand = Some(value);
                        // Update history with the operation
                        let op_symbol = match op {
                            Operation::Add => "+",
                            Operation::Subtract => "-",
                            Operation::Multiply => "×",
                            Operation::Divide => "÷",
                        };
                        self.history = format!("{} {} ", value, op_symbol);
                    }
                    self.operation = Some(op);
                    self.waiting_for_operand = true;
                }
                Task::none()
            }
            Message::Equals => {
                if let (Ok(value), Some(first), Some(op)) = (
                    self.display.parse::<f64>(),
                    self.first_operand,
                    self.operation,
                ) {
                    match self.calculate(first, value, op) {
                        Ok(result) => {
                            // Update history with the full expression
                            let op_symbol = match op {
                                Operation::Add => "+",
                                Operation::Subtract => "-",
                                Operation::Multiply => "×",
                                Operation::Divide => "÷",
                            };
                            self.history = format!("{} {} {} = ", first, op_symbol, value);
                            self.display = format_number(result);
                            self.first_operand = None;
                            self.operation = None;
                            self.waiting_for_operand = true;
                        }
                        Err(error) => {
                            self.display = error;
                            self.first_operand = None;
                            self.operation = None;
                            self.waiting_for_operand = true;
                        }
                    }
                }
                Task::none()
            }
            Message::ClearEntry => {
                self.display = "0".to_string();
                Task::none()
            }
            Message::Clear => {
                self.display = "0".to_string();
                self.history = "".to_string();
                self.first_operand = None;
                self.operation = None;
                self.waiting_for_operand = false;
                Task::none()
            }
            Message::Backspace => {
                if self.display.len() > 1 {
                    self.display.pop();
                } else {
                    self.display = "0".to_string();
                }
                Task::none()
            }
            Message::PlusMinus => {
                if let Ok(value) = self.display.parse::<f64>() {
                    self.display = format_number(-value);
                }
                Task::none()
            }
            Message::SquareRoot => {
                if let Ok(value) = self.display.parse::<f64>() {
                    if value >= 0.0 {
                        self.display = format_number(value.sqrt());
                    } else {
                        self.display = "Invalid input".to_string();
                    }
                }
                Task::none()
            }
            Message::Square => {
                if let Ok(value) = self.display.parse::<f64>() {
                    self.display = format_number(value * value);
                }
                Task::none()
            }
            Message::Percentage => {
                if let Ok(value) = self.display.parse::<f64>() {
                    self.display = format_number(value / 100.0);
                }
                Task::none()
            }
            Message::Reciprocal => {
                if let Ok(value) = self.display.parse::<f64>() {
                    if value != 0.0 {
                        self.display = format_number(1.0 / value);
                    } else {
                        self.display = "Cannot divide by zero".to_string();
                    }
                }
                Task::none()
            }
            Message::ShowList => {
                // TODO: Implement list functionality
                Task::none()
            }
            Message::ToggleAlwaysOnTop => {
                if let Some(window_id) = self.window_id {
                    self.always_on_top = !self.always_on_top;
                    return window::change_level(
                        window_id,
                        if self.always_on_top {
                            window::Level::AlwaysOnTop
                        } else {
                            window::Level::Normal
                        }
                    );
                }
                Task::none()
            }
            Message::WindowEvent(event) => {
                // Handle window events
                match event {
                    window::Event::CloseRequested => {
                        // We don't have the window ID here, so we can't close it directly
                        Task::none()
                    }
                    _ => Task::none()
                }
            }
            Message::Ignore => {
                // Do nothing
                Task::none()
            }
        }
    }

    fn calculate(&self, first: f64, second: f64, op: Operation) -> Result<f64, String> {
        match op {
            Operation::Add => Ok(first + second),
            Operation::Subtract => Ok(first - second),
            Operation::Multiply => Ok(first * second),
            Operation::Divide => {
                if second != 0.0 {
                    Ok(first / second)
                } else {
                    // Handle division by zero consistently
                    Err("Cannot divide by zero".to_string())
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Create the mode display row with list and always on top buttons
        let mode_row = container(
            row![
                // List button
                button(
                    container(svg(Handle::from_path("static/icons/list.svg"))
                        .width(Length::Fixed(32.0)) // Icon size 28x28
                        .height(Length::Fixed(32.0)) // Icon size 28x28
                        .style(|_theme: &Theme, _status: iced::widget::svg::Status| {
                            svg::Style { color: Some(iced::Color::WHITE) }
                        }))
                        .width(Length::Fixed(32.0)) // Container size 28x28
                        .height(Length::Fixed(32.0)) // Container size 28x28
                        .center_x(Fill)
                        .center_y(Fill)
                )
                .width(Length::Fixed(38.0)) // Button size 34x34
                .height(Length::Fixed(38.0)) // Button size 34x34
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
                .on_press(Message::ShowList),
                
                // Mode text
                container(text("Базовый")
                    .size(20) // Match Calcutron button font size
                    .font(RUBIK_FONT)
                    .style(|_theme: &Theme| {
                        text::Style { color: Some(iced::Color::WHITE) }
                    }))
                .width(Length::Shrink)
                .height(Length::Fixed(30.0))
                .center_y(Fill),
                
                // Always on top toggle button
                button(
                    container(svg(Handle::from_path(
                        if self.always_on_top {
                            "static/icons/arrow-square-in.svg"
                        } else {
                            "static/icons/arrow-square-out.svg"
                        }
                    ))
                        .width(Length::Fixed(32.0)) // Icon size 28x28
                        .height(Length::Fixed(32.0)) // Icon size 28x28
                        .style(|_theme: &Theme, _status: iced::widget::svg::Status| {
                            svg::Style { color: Some(iced::Color::WHITE) }
                        }))
                        .width(Length::Fixed(32.0)) // Container size 28x28
                        .height(Length::Fixed(32.0)) // Container size 28x28
                        .center_x(Fill)
                        .center_y(Fill)
                )
                .width(Length::Fixed(38.0)) // Button size 34x34
                .height(Length::Fixed(38.0)) // Button size 34x34
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
                .on_press(Message::ToggleAlwaysOnTop),
            ]
            .spacing(2)
            .align_y(Center)
        )
        .width(Fill)
        .height(Length::Fixed(30.0))
        .padding([0, 4]);

        // Create the history display (smaller font, top line) - 1/3 of total height
        let history_display = container(
            text(&self.history)
                .size(14) // Reduced font size from 16 to 14
                .font(RUBIK_FONT) // Apply Rubik font
        )
        .width(Fill)
        .height(Length::Fixed(28.0)) // Adjusted height for better vertical centering
        .padding([0, 10]) // Removed vertical padding for better vertical centering
        .align_x(iced::alignment::Horizontal::Right) // Right align the text
        .align_y(iced::alignment::Vertical::Center) // Vertically center the text
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba8(26, 34, 39, 0.9))), // #1a2227 with 10% transparency (90% opacity)
            border: iced::border::Border {
                radius: 5.0.into(), // Simple rounded corners
                ..Default::default()
            },
            ..container::Style::default()
        });

        // Create the main display (larger font, bottom line) - 2/3 of total height
        let main_display = container(
            text(&self.display)
                .size(46) // Reverted to original font size for main display
                .font(RUBIK_FONT) // Apply Rubik font
        )
        .width(Fill)
        .height(Length::Fixed(60.0)) // Reverted to original height: 2/3 of 93.5px = 62.3px
        .padding([0, 10]) // Kept vertical centering improvement (removed vertical padding)
        .align_x(iced::alignment::Horizontal::Right) // Right align the text
        .align_y(iced::alignment::Vertical::Center) // Vertically center the text
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba8(26, 34, 39, 0.9))), // #1a2227 with 10% transparency (90% opacity)
            border: iced::border::Border {
                radius: 5.0.into(), // Simple rounded corners
                ..Default::default()
            },
            ..container::Style::default()
        });

        // Combine both displays into a single display container with specified height
        let display_container = column![
            history_display,
            main_display
        ]
        .spacing(2) // Spacing between the two lines
        .height(Length::Fixed(93.5)) // Total height: 93.5px
        .padding(0); // No padding on the container

        // Create the buttons with proper centering
        let buttons = column![
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
            ].spacing(2).align_y(Center),
            
            // Row 4
            row![
                digit_button("number-four.svg", Message::Digit(4)).padding(15).width(Fill),
                digit_button("number-five.svg", Message::Digit(5)).padding(15).width(Fill),
                digit_button("number-six.svg", Message::Digit(6)).padding(15).width(Fill),
                operator_button("minus.svg", Message::Operation(Operation::Subtract)).padding(15).width(Fill),
            ].spacing(2).align_y(Center),
            
            // Row 5
            row![
                digit_button("number-one.svg", Message::Digit(1)).padding(15).width(Fill),
                digit_button("number-two.svg", Message::Digit(2)).padding(15).width(Fill),
                digit_button("number-three.svg", Message::Digit(3)).padding(15).width(Fill),
                operator_button("plus.svg", Message::Operation(Operation::Add)).padding(15).width(Fill),
            ].spacing(2).align_y(Center),
            
            // Row 6
            row![
                plus_minus_button("plus-minus.svg", Message::PlusMinus).padding(15).width(Fill),
                digit_button("number-zero.svg", Message::Digit(0)).padding(15).width(Fill),
                digit_button("dot.svg", Message::Decimal).padding(15).width(Fill),
                equals_button("equals.svg", Message::Equals).padding(15).width(Fill),
            ].spacing(2).align_y(Center),
        ]
        .spacing(2)
        .align_x(Center)
        .height(Fill);

        // Create the main layout with the specified proportions
        let content = column![
            container(iced::widget::vertical_space().height(Length::Fixed(15.0))).height(Length::Fixed(15.0)), // Increased top space to move mode row lower
            mode_row, // Add the mode row here
            container(iced::widget::vertical_space().height(Length::Fixed(15.0))).height(Length::Fixed(15.0)), // Increased space between mode row and display
            container(iced::widget::vertical_space().height(Length::Fixed(6.0))).height(Length::Fixed(6.0)), // Additional space to move display lower
            display_container, // Two-line display with height of 93.5px
            container(iced::widget::vertical_space().height(Length::Fixed(38.0))).height(Length::Fixed(38.0)), // Reduced from 40px to 20px bottom space
            container(buttons).height(Length::FillPortion(65)), // 65% for buttons
            iced::widget::vertical_space().height(Length::Fixed(4.0)), // 4px margin below button block
        ]
        .padding([0, 4]) // Only horizontal padding
        .height(Length::Fill); // Allow the entire content to fill available space

        container(content)
            .width(Length::Fill) // Allow width to adapt to window size
            .height(Length::Fill) // Allow height to adapt to window size
            .center_x(Fill)
            .center_y(Fill)
            .style(|_theme: &Theme| container::Style {
                // Remove the background entirely to allow window transparency to show through
                background: None,
                border: iced::border::Border {
                    radius: 5.0.into(),
                    ..Default::default()
                },
                ..container::Style::default()
            })
            .into()
    }
}

// Helper function to create digit buttons with custom styling
fn digit_button(icon_path: &str, message: Message) -> button::Button<'_, Message> {
    let icon_handle = Handle::from_path(format!("static/icons/{}", icon_path));
    button(
        container(svg(icon_handle)
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .style(|_theme: &Theme, _status: iced::widget::svg::Status| {
                // Use the same color as the display text (white in dark theme)
                svg::Style { color: Some(iced::Color::WHITE) }
            }))
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .center_x(Fill)
            .center_y(Fill)
    )
        .width(Fill)
        .height(Fill)
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
fn operator_button(icon_path: &str, message: Message) -> button::Button<'_, Message> {
    let icon_handle = Handle::from_path(format!("static/icons/{}", icon_path));
    button(
        container(svg(icon_handle)
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .style(|_theme: &Theme, _status: iced::widget::svg::Status| {
                // Use the same color as the display text (white in dark theme)
                svg::Style { color: Some(iced::Color::WHITE) }
            }))
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .center_x(Fill)
            .center_y(Fill)
    )
        .width(Fill)
        .height(Fill)
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
fn equals_button(icon_path: &str, message: Message) -> button::Button<'_, Message> {
    let icon_handle = Handle::from_path(format!("static/icons/{}", icon_path));
    button(
        container(svg(icon_handle)
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0)))
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .center_x(Fill)
            .center_y(Fill)
    )
        .width(Fill)
        .height(Fill)
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
fn plus_minus_button(icon_path: &str, message: Message) -> button::Button<'_, Message> {
    let icon_handle = Handle::from_path(format!("static/icons/{}", icon_path));
    button(
        container(svg(icon_handle)
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .style(|_theme: &Theme, _status: iced::widget::svg::Status| {
                // Use the same color as the display text (white in dark theme)
                svg::Style { color: Some(iced::Color::WHITE) }
            }))
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .center_x(Fill)
            .center_y(Fill)
    )
        .width(Fill)
        .height(Fill)
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

// Helper function to format numbers for display
fn format_number(value: f64) -> String {
    if value.is_nan() {
        "Error".to_string()
    } else if value.is_infinite() {
        "Infinity".to_string()
    } else {
        // Format to avoid unnecessary decimal places
        if value.fract() == 0.0 && value.abs() < 1e15 {
            // Use integer formatting for whole numbers
            (value as i64).to_string()
        } else if value.abs() >= 1e15 || (value.abs() < 1e-4 && value != 0.0) {
            // Use scientific notation for very large or very small numbers
            format!("{:.6e}", value)
        } else {
            // Use a more efficient approach for regular decimal numbers
            let formatted = format!("{:.10}", value);
            // Trim trailing zeros and decimal point more efficiently
            let trimmed = formatted.trim_end_matches('0');
            if trimmed.ends_with('.') {
                &trimmed[..trimmed.len() - 1]
            } else {
                trimmed
            }.to_string()
        }
    }
}

fn main() -> iced::Result {
    iced::application(Calcutron::title, Calcutron::update, Calcutron::view)
        .subscription(|_state: &Calcutron| {
            iced::event::listen_with(|event, _status, _id| {
                if let iced::Event::Window(window_event) = event {
                    Some(Message::WindowEvent(window_event))
                } else {
                    Some(Message::Ignore)
                }
            })
        })
        .window(iced::window::Settings {
            size: iced::Size::new(320.0, 470.0),
            min_size: Some(iced::Size::new(320.0, 470.0)),
            ..Default::default()
        })
        .font(include_bytes!("../static/fonts/Rubik-Regular.ttf").as_slice())
        .theme(|_state: &Calcutron| Theme::Dark)
        .run_with(Calcutron::new)
}