# Rust Game Test Runner

A testing framework for editors and basic game engines to verify code functionality without GUI interaction.

Originally developed to test every single task at [rust-learning-game](https://github.com/hastur-dev/rust-learning-game/) without having to open up the game and just use the grid system.

## Features

- **Headless Code Execution**: Test Rust code without launching GUI applications
- **Grid-Based Game Testing**: Validate robot movement, scanning, and interaction logic
- **Message Simulation**: See what popups and messages would appear during execution
- **Comprehensive Reporting**: Track robot position, turns taken, level completion
- **Task Validation**: Automatically verify tutorial task completion
- **Multi-Level Testing**: Test entire learning progression automatically

## Usage

### As a Library

```rust
use rust_game_test_runner::{TestRunner, GameConfig, TestResult};

let config = GameConfig::new()
    .with_grid_size(6, 6)
    .with_robot_start_position(1, 1);

let code = r#"
fn main() {
    move_bot("right");
    let result = scan("current");
    println!("Found: {}", result);
}
"#;

let runner = TestRunner::new(config);
let result = runner.test_code(code).await?;

println!("Final position: ({}, {})", result.final_position.x, result.final_position.y);
println!("Messages: {:?}", result.messages);
```

### Command Line Interface

```bash
# Test a specific file
cargo run --bin test-runner -- --test-code yourfile.rs

# Test with detailed output
cargo run --bin test-runner -- --test-code yourfile.rs --verbose

# Test multiple files
cargo run --bin test-runner -- --test-dir tests/
```

## Example Output

```
=== RUST GAME TEST RUNNER ===
Testing code from file: example.rs

=== Executing Test Code ===
Robot Action: Move -> Move executed
Robot Action: Scan -> All 4 accessible tiles are empty
Message Popup: 📝 Program Output - Found: empty

=== Test Results ===
✅ Execution successful
📍 Final Position: (2, 1) 
🔄 Turns taken: 1
📋 Messages: 2 popups would be displayed

=== Test Complete ===
```

## Supported Game Functions

- `move_bot(direction)` - Move robot up/down/left/right
- `scan(direction)` - Scan adjacent tiles or current area
- `grab()` - Attempt to grab items at current position
- `println!()`, `eprintln!()`, `panic!()` - Output functions

## Use Cases

Perfect for:
- **Educational Games**: Validate student code without manual testing
- **Game Engine Testing**: Automated validation of game logic
- **CI/CD Pipelines**: Continuous testing of game functionality
- **Editor Integration**: Real-time code validation in IDEs
- **Tutorial Systems**: Automated checking of learning objectives

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-game-test-runner = "0.1.0"
```

## License

Licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.

## Contributing

This project was originally built to test the [Rust Learning Game](https://github.com/hastur-dev/rust-learning-game/). 

Contributions are welcome! Please feel free to submit a Pull Request.