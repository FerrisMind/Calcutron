use iced::window;

use crate::models::operation::Operation;

#[derive(Debug, Clone)]
pub enum Message {
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
    // Close window
    CloseWindow,
    // Begin window drag
    BeginWindowDrag,
    // Toggle always on top
    ToggleAlwaysOnTop,
    // Window event with identifier
    WindowEvent(window::Id, window::Event),
    // Ignore event
    Ignore,
}
