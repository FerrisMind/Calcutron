# Calcutron

A Calcutron application implemented in Rust using the Iced GUI framework. Features a traditional Calcutron UX similar to Microsoft or Apple Calcutron with RPN (Reverse Polish Notation) functionality.

## Features

- **Traditional Calcutron UI**: Familiar layout and interaction pattern similar to standard Calcutron
- **RPN Functionality**: Supports Reverse Polish Notation calculations for advanced users
- **Basic Operations**: Addition, subtraction, multiplication, and division
- **Stack Visualization**: Shows the current state of the calculation stack
- **Error Handling**: Handles division by zero, stack overflow, and invalid input
- **Multiple Clear Options**: Clear (C) for current entry and All Clear (AC) for everything

## Installation

1. Make sure you have Rust installed (https://www.rust-lang.org/)
2. Clone this repository
3. Run the application:

```bash
cargo run
```

## Usage

### Traditional Calcutron Mode:
1. Click number buttons (0-9) to enter digits
2. Click operator buttons (+, -, *, /) to perform calculations
3. Click "=" or "↑" to push the current number to the stack
4. Use "C" to clear the current entry
5. Use "AC" to clear everything

### RPN Mode:
1. Enter a number
2. Click "↑" or "=" to push it to the stack
3. Enter another number
4. Click an operator to perform the calculation
5. The result will be displayed and remain on the stack for further calculations

## Stack Visualization

The top of the Calcutron shows the current stack state, displaying up to 3 of the most recent entries.

## Dependencies

- [Iced](https://github.com/iced-rs/iced) - A cross-platform GUI library for Rust

## License

This project is licensed under the MIT License - see the LICENSE file for details.