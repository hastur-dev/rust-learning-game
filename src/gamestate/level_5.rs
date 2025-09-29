use super::types::Game;

impl Game {
    // Level 5 specific tutorial system methods
    pub fn get_level_5_task_message(&self) -> String {
        if self.level_idx != 4 {
            return String::new(); // Only for level 5
        }

        match self.tutorial_state.current_task {
            0 => "📋 **TASK 1/5: Explicit Type Casting with 'as'**\n\nUse the `as` keyword for explicit type casting, which can potentially lose data:\n\n```rust\n// Basic integer casting\nlet large_number: i64 = 1000;\nlet small_number: i32 = large_number as i32;\n\nprintln!(\"Large (i64): {}\", large_number);\nprintln!(\"Small (i32): {}\", small_number);\n\n// Casting that loses precision\nlet precise_float: f64 = 3.14159265359;\nlet less_precise: f32 = precise_float as f32;\n\nprintln!(\"Precise (f64): {}\", precise_float);\nprintln!(\"Less precise (f32): {}\", less_precise);\n\n// Float to integer (truncates decimal)\nlet pi: f64 = 3.14159;\nlet pi_int: i32 = pi as i32;\n\nprintln!(\"Pi as float: {}\", pi);\nprintln!(\"Pi as integer: {} (decimal part lost)\", pi_int);\n```\n\n• `as` performs explicit casting\n• Casting can lose data or precision\n• Float to int truncates (doesn't round)\n• Integer overflow can wrap around".to_string(),

            1 => "📋 **TASK 2/5: Safe Conversions with From and Into**\n\nUse `From` and `Into` traits for safe, lossless conversions:\n\n```rust\n// From smaller to larger integer types (always safe)\nlet small: i32 = 100;\nlet large: i64 = small.into(); // or i64::from(small)\n\nprintln!(\"Small (i32): {}\", small);\nprintln!(\"Large (i64): {}\", large);\n\n// String conversions\nlet number: i32 = 42;\nlet number_string: String = number.to_string();\nlet formatted: String = format!(\"Number: {}\", number);\n\nprintln!(\"Original number: {}\", number);\nprintln!(\"As string: {}\", number_string);\nprintln!(\"Formatted: {}\", formatted);\n\n// Character to string\nlet ch: char = 'R';\nlet ch_string: String = ch.to_string();\n\nprintln!(\"Character: {}\", ch);\nprintln!(\"As string: {}\", ch_string);\n```\n\n• `From` and `Into` traits for safe conversions\n• `.into()` for automatic type inference\n• `.to_string()` for string conversions\n• Safe conversions don't lose data".to_string(),

            2 => "📋 **TASK 3/5: String Parsing and Error Handling**\n\nParse strings to other types with proper error handling using `Result`:\n\n```rust\n// Basic parsing with expect (panics on failure)\nlet valid_number = \"42\";\nlet parsed: i32 = valid_number.parse().expect(\"Failed to parse number\");\n\nprintln!(\"Valid string: '{}'\", valid_number);\nprintln!(\"Parsed number: {}\", parsed);\n\n// Parsing with match for error handling\nlet strings = [\"123\", \"45.67\", \"not_a_number\", \"0\"];\n\nfor string_val in strings.iter() {\n    match string_val.parse::<i32>() {\n        Ok(number) => println!(\"'{}' -> {} (success)\", string_val, number),\n        Err(error) => println!(\"'{}' -> Error: {}\", string_val, error),\n    }\n}\n\n// Using unwrap_or for default values\nlet inputs = [\"100\", \"invalid\", \"200\"];\n\nfor input in inputs.iter() {\n    let number: i32 = input.parse().unwrap_or(0);\n    println!(\"'{}' -> {} (with default)\", input, number);\n}\n```\n\n• `.parse()` returns `Result<T, E>`\n• `expect()` for panicking on errors\n• `match` for handling parse results\n• `unwrap_or()` for default values".to_string(),

            3 => "📋 **TASK 4/5: Custom Type Conversions**\n\nCreate your own types and implement conversion traits:\n\n```rust\n// Custom types for robot system\nstruct Position {\n    x: i32,\n    y: i32,\n}\n\nstruct RobotState {\n    position: Position,\n    energy: u32,\n}\n\n// Implement conversion from tuple to Position\nimpl From<(i32, i32)> for Position {\n    fn from(coord: (i32, i32)) -> Self {\n        Position {\n            x: coord.0,\n            y: coord.1,\n        }\n    }\n}\n\n// Implement conversion from Position to tuple\nimpl From<Position> for (i32, i32) {\n    fn from(pos: Position) -> Self {\n        (pos.x, pos.y)\n    }\n}\n\nfn main() {\n    // Create Position from tuple\n    let start_coords = (5, 10);\n    let start_position: Position = start_coords.into();\n    \n    println!(\"Position: x={}, y={}\", start_position.x, start_position.y);\n    \n    // Create RobotState using conversions\n    let robot = RobotState {\n        position: (0, 0).into(),  // tuple -> Position\n        energy: 100,\n    };\n}\n```\n\n• Implementing `From` trait for custom types\n• Bidirectional conversions\n• Using conversions in data structures\n• Converting collections of data".to_string(),

            4 => "📋 **TASK 5/5: Type Inference with Conversions**\n\nMaster type inference in conversion contexts and understand its limits:\n\n```rust\nfn main() {\n    // Type inference with numeric conversions\n    let small = 100_i32;\n    let large: i64 = small.into(); // Rust infers i64 from context\n    \n    // Need explicit type when inference is ambiguous\n    let explicit: i64 = small.into();\n    let inferred: i64 = small.into();\n    \n    println!(\"Small: {}\", small);\n    println!(\"Large (inferred): {}\", large);\n    \n    // Collection inference\n    let numbers = vec![1, 2, 3];\n    let converted: Vec<i64> = numbers.into_iter().map(|x| x.into()).collect();\n    \n    println!(\"Converted: {:?}\", converted);\n    \n    // Parsing with inference - requires type annotation\n    let as_i32: i32 = \"123\".parse().expect(\"Parse failed\");\n    let as_f64: f64 = \"123\".parse().expect(\"Parse failed\");\n    \n    println!(\"Parsed as i32: {}\", as_i32);\n    println!(\"Parsed as f64: {}\", as_f64);\n    \n    // Turbofish syntax for explicit types\n    let parsed_with_turbofish = \"456\".parse::<i32>().expect(\"Parse failed\");\n    println!(\"Turbofish parsed: {}\", parsed_with_turbofish);\n}\n```\n\n• Type inference works with conversions\n• Explicit annotations when ambiguous\n• Function parameter inference\n• Turbofish syntax `::<Type>`\n• Collection type specification".to_string(),

            _ => "🎉 **Level 5 Complete!**\n\nOutstanding! You've mastered Rust's type system and conversion mechanisms:\n• **Explicit casting** with `as` keyword and its risks\n• **Safe conversions** with `From`/`Into` traits\n• **String parsing** with proper error handling\n• **Custom type conversions** with trait implementations\n• **Type inference** in conversion contexts\n\nYou now understand how Rust maintains type safety while providing flexible conversion options. You can safely transform data between types without losing information or introducing runtime errors!\n\n🚀 Ready for Level 6: Flow Control and Pattern Matching!".to_string(),
        }
    }

    pub fn check_level_5_progress(&mut self) {
        if self.level_idx != 4 || self.tutorial_state.current_task >= 5 {
            return; // Only for level 5 and if not completed
        }

        match self.tutorial_state.current_task {
            0 => {
                // Task 1: Explicit type casting with 'as'
                if self.println_outputs.iter().any(|output|
                    output.contains("Large (i64):") ||
                    output.contains("Small (i32):") ||
                    output.contains("Precise (f64):") ||
                    output.contains("Less precise (f32):") ||
                    output.contains("Pi as float:") ||
                    output.contains("Pi as integer:") ||
                    output.contains("decimal part lost") ||
                    output.contains("wrapped around") ||
                    output.contains("overflow")
                ) {
                    self.tutorial_state.task_completed[0] = true;
                    self.tutorial_state.current_task = 1;
                    println!("✅ Task 1 completed: Explicit type casting with 'as'!");
                }
            },
            1 => {
                // Task 2: Safe conversions with From/Into
                if self.println_outputs.iter().any(|output|
                    output.contains("Small (i32):") ||
                    output.contains("Large (i64):") ||
                    output.contains("As string:") ||
                    output.contains("Formatted:") ||
                    output.contains("Character:") ||
                    output.contains("Chain:") ||
                    output.contains("From example:") ||
                    output.contains("Into example:")
                ) {
                    self.tutorial_state.task_completed[1] = true;
                    self.tutorial_state.current_task = 2;
                    println!("✅ Task 2 completed: Safe conversions with From/Into!");
                }
            },
            2 => {
                // Task 3: String parsing and error handling
                if self.println_outputs.iter().any(|output|
                    output.contains("Valid string:") ||
                    output.contains("Parsed number:") ||
                    output.contains("(success)") ||
                    output.contains("Error:") ||
                    output.contains("(float)") ||
                    output.contains("Invalid float") ||
                    output.contains("(with default)") ||
                    output.contains("Inferred parse:") ||
                    output.contains("Explicit parse:")
                ) {
                    self.tutorial_state.task_completed[2] = true;
                    self.tutorial_state.current_task = 3;
                    println!("✅ Task 3 completed: String parsing and error handling!");
                }
            },
            3 => {
                // Task 4: Custom type conversions
                if self.println_outputs.iter().any(|output|
                    output.contains("Start coordinates:") ||
                    output.contains("Position: x=") ||
                    output.contains("End coordinates:") ||
                    output.contains("Robot created at:") ||
                    output.contains("Robot energy:") ||
                    output.contains("Movement chain:") ||
                    output.contains("Converted") ||
                    output.contains("coordinates to positions")
                ) {
                    self.tutorial_state.task_completed[3] = true;
                    self.tutorial_state.current_task = 4;
                    println!("✅ Task 4 completed: Custom type conversions!");
                }
            },
            4 => {
                // Task 5: Type inference with conversions
                if self.println_outputs.iter().any(|output|
                    output.contains("Small:") ||
                    output.contains("Large (inferred):") ||
                    output.contains("Explicit:") ||
                    output.contains("Converted: [") ||
                    output.contains("Parsed as i32:") ||
                    output.contains("Parsed as f64:") ||
                    output.contains("Processing:") ||
                    output.contains("Turbofish parsed:") ||
                    output.contains("Explicit collection:")
                ) {
                    self.tutorial_state.task_completed[4] = true;
                    self.tutorial_state.current_task = 5;
                    println!("✅ Task 5 completed: Type inference with conversions!");
                }
            },
            _ => {}
        }
    }
}