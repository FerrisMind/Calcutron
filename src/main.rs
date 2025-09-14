use iced::widget::{
    button, column, container, row, text,
};
use iced::{Center, Element, Fill, Length, Task, Theme};

// Define calculator state
#[derive(Default)]
struct Calculator {
    // Current display value
    display: String,
    // First operand for binary operations
    first_operand: Option<f64>,
    // Current operation
    operation: Option<Operation>,
    // Whether we're waiting for the next operand
    waiting_for_operand: bool,
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
}

impl Calculator {
    fn new() -> (Self, Task<Message>) {
        let mut calculator = Calculator::default();
        calculator.display = "0".to_string();
        (calculator, Task::none())
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Digit(digit) => {
                if self.waiting_for_operand {
                    self.display = digit.to_string();
                    self.waiting_for_operand = false;
                } else {
                    if self.display == "0" {
                        self.display = digit.to_string();
                    } else {
                        self.display.push_str(&digit.to_string());
                    }
                }
            }
            Message::Decimal => {
                if self.waiting_for_operand {
                    self.display = "0.".to_string();
                    self.waiting_for_operand = false;
                } else {
                    if !self.display.contains('.') {
                        self.display.push('.');
                    }
                }
            }
            Message::Operation(op) => {
                if let Ok(value) = self.display.parse::<f64>() {
                    if let Some(first) = self.first_operand {
                        if let Some(current_op) = self.operation {
                            // Perform the previous operation
                            let result = self.calculate(first, value, current_op);
                            self.display = format_number(result);
                            self.first_operand = Some(result);
                        }
                    } else {
                        self.first_operand = Some(value);
                    }
                    self.operation = Some(op);
                    self.waiting_for_operand = true;
                }
            }
            Message::Equals => {
                if let (Ok(value), Some(first), Some(op)) = (
                    self.display.parse::<f64>(),
                    self.first_operand,
                    self.operation,
                ) {
                    let result = self.calculate(first, value, op);
                    self.display = format_number(result);
                    self.first_operand = None;
                    self.operation = None;
                    self.waiting_for_operand = true;
                }
            }
            Message::ClearEntry => {
                self.display = "0".to_string();
            }
            Message::Clear => {
                self.display = "0".to_string();
                self.first_operand = None;
                self.operation = None;
                self.waiting_for_operand = false;
            }
            Message::Backspace => {
                if self.display.len() > 1 {
                    self.display.pop();
                } else {
                    self.display = "0".to_string();
                }
            }
            Message::PlusMinus => {
                if let Ok(value) = self.display.parse::<f64>() {
                    self.display = format_number(-value);
                }
            }
            Message::SquareRoot => {
                if let Ok(value) = self.display.parse::<f64>() {
                    if value >= 0.0 {
                        self.display = format_number(value.sqrt());
                    } else {
                        self.display = "Invalid input".to_string();
                    }
                }
            }
            Message::Square => {
                if let Ok(value) = self.display.parse::<f64>() {
                    self.display = format_number(value * value);
                }
            }
            Message::Percentage => {
                if let Ok(value) = self.display.parse::<f64>() {
                    self.display = format_number(value / 100.0);
                }
            }
            Message::Reciprocal => {
                if let Ok(value) = self.display.parse::<f64>() {
                    if value != 0.0 {
                        self.display = format_number(1.0 / value);
                    } else {
                        self.display = "Cannot divide by zero".to_string();
                    }
                }
            }
        }
        
        Task::none()
    }

    fn calculate(&self, first: f64, second: f64, op: Operation) -> f64 {
        match op {
            Operation::Add => first + second,
            Operation::Subtract => first - second,
            Operation::Multiply => first * second,
            Operation::Divide => {
                if second != 0.0 {
                    first / second
                } else {
                    // Handle division by zero
                    f64::NAN
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Create the display
        let display = container(
            text(&self.display)
                .size(32)
                .align_x(Center)
        )
        .width(Fill)
        .height(Length::Fixed(60.0))
        .padding(10)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.1, 0.1, 0.1))),
            border: iced::border::Border {
                radius: 5.0.into(),
                ..Default::default()
            },
            ..container::Style::default()
        });

        // Create the buttons with proper centering
        let buttons = column![
            // Row 1: Percentage, Square Root, Clear buttons
            row![
                centered_button("%").on_press(Message::Percentage).padding(10).width(Fill),
                centered_button("√").on_press(Message::SquareRoot).padding(10).width(Fill),
                centered_button("x²").on_press(Message::Square).padding(10).width(Fill),
                centered_button("1/x").on_press(Message::Reciprocal).padding(10).width(Fill),
            ].spacing(2).align_y(Center),
            
            // Row 2: Clear and operation buttons
            row![
                centered_button("CE").on_press(Message::ClearEntry).padding(10).width(Fill),
                centered_button("C").on_press(Message::Clear).padding(10).width(Fill),
                centered_button("←").on_press(Message::Backspace).padding(10).width(Fill),
                centered_button("/").on_press(Message::Operation(Operation::Divide)).padding(10).width(Fill),
            ].spacing(2).align_y(Center),
            
            // Row 3: Number buttons and operations
            row![
                centered_button("7").on_press(Message::Digit(7)).padding(15).width(Fill),
                centered_button("8").on_press(Message::Digit(8)).padding(15).width(Fill),
                centered_button("9").on_press(Message::Digit(9)).padding(15).width(Fill),
                centered_button("*").on_press(Message::Operation(Operation::Multiply)).padding(15).width(Fill),
            ].spacing(2).align_y(Center),
            
            // Row 4
            row![
                centered_button("4").on_press(Message::Digit(4)).padding(15).width(Fill),
                centered_button("5").on_press(Message::Digit(5)).padding(15).width(Fill),
                centered_button("6").on_press(Message::Digit(6)).padding(15).width(Fill),
                centered_button("-").on_press(Message::Operation(Operation::Subtract)).padding(15).width(Fill),
            ].spacing(2).align_y(Center),
            
            // Row 5
            row![
                centered_button("1").on_press(Message::Digit(1)).padding(15).width(Fill),
                centered_button("2").on_press(Message::Digit(2)).padding(15).width(Fill),
                centered_button("3").on_press(Message::Digit(3)).padding(15).width(Fill),
                centered_button("+").on_press(Message::Operation(Operation::Add)).padding(15).width(Fill),
            ].spacing(2).align_y(Center),
            
            // Row 6
            row![
                centered_button("±").on_press(Message::PlusMinus).padding(15).width(Fill),
                centered_button("0").on_press(Message::Digit(0)).padding(15).width(Fill),
                centered_button(".").on_press(Message::Decimal).padding(15).width(Fill),
                centered_button("=").on_press(Message::Equals).padding(15).width(Fill),
            ].spacing(2).align_y(Center),
        ]
        .spacing(2);

        // Create the main layout
        let content = column![
            display,
            buttons,
        ]
        .spacing(5)
        .padding(10);

        container(content)
            .width(Length::Fixed(300.0))
            .height(Length::Fixed(400.0))
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }
}

// Helper function to create centered buttons
fn centered_button(label: &str) -> button::Button<'_, Message> {
    button(text(label).width(Fill).center())
        .width(Fill)
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
            format!("{}", value as i64)
        } else if value.abs() >= 1e15 || (value.abs() < 1e-4 && value != 0.0) {
            format!("{:.6e}", value)
        } else {
            format!("{:.10}", value).trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }
}

fn main() -> iced::Result {
    iced::application(Calculator::title, Calculator::update, Calculator::view)
        .theme(|_state: &Calculator| Theme::Dark)
        .run_with(Calculator::new)
}