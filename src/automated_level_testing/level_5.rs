// Level 5: Level 5: Type Casting and Conversions - Automated Test Solutions
// Updated to match actual learning tests

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_5_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 5: Type Casting and Conversions",
        level_index: 4,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Casting with 'as'",
                solution_code: r#"fn main() {
    let large_number: i64 = 1000;
    let small_number: i32 = large_number as i32;
    println!("i64 {} -> i32 {}", large_number, small_number);

    let float: f64 = 3.99;
    let integer: i32 = float as i32;
    println!("f64 {} -> i32 {} (truncated)", float, integer);

    // Dangerous cast
    let big: i32 = 300;
    let small: i8 = big as i8;
    println!("i32 {} -> i8 {} (overflow!)", big, small);

    let unsigned: u32 = 42;
    let signed: i32 = unsigned as i32;
    println!("u32 {} -> i32 {}", unsigned, signed);
}"#,
                completion_indicators: vec![
                    "i64 1000 -> i32 1000", "f64 3.99 -> i32 3", "overflow!", "u32 42 -> i32 42"
                ],
            },
            TaskTest {
                task_number: 2,
                task_name: "From and Into",
                solution_code: r#"fn main() {
    let small: i32 = 100;
    let large: i64 = small.into();
    println!("i32 {} -> i64 {} (using into)", small, large);

    let number: i32 = 42;
    let text = String::from("Hello, Rust!");
    println!("Created string: {}", text);

    let large2 = i64::from(small);
    println!("i32 {} -> i64 {} (using from)", small, large2);

    // Array to Vec
    let arr = [1, 2, 3];
    let vec: Vec<i32> = arr.into();
    println!("Array to Vec: {:?}", vec);
}"#,
                completion_indicators: vec![
                    "i32 100 -> i64 100 (using into)", "Created string: Hello, Rust!", "i32 100 -> i64 100 (using from)", "Array to Vec: [1, 2, 3]"
                ],
            },
            TaskTest {
                task_number: 3,
                task_name: "Parsing Strings",
                solution_code: r#"fn main() {
    let valid_number = "42";
    let parsed: i32 = valid_number.parse().expect("Failed to parse");
    println!("Parsed '{}' to {}", valid_number, parsed);

    let invalid = "not_a_number";
    match invalid.parse::<i32>() {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Failed to parse '{}': {}", invalid, e),
    }

    let with_default = "bad".parse::<i32>().unwrap_or(0);
    println!("Parse with default: {}", with_default);

    let float_str = "3.14";
    let float: f64 = float_str.parse().expect("Failed to parse float");
    println!("Parsed float: {}", float);
}"#,
                completion_indicators: vec![
                    "Parsed '42' to 42", "Failed to parse", "Parse with default: 0", "Parsed float: 3.14"
                ],
            },
            TaskTest {
                task_number: 4,
                task_name: "Custom Conversions",
                solution_code: r#"struct Position {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Position {
    fn from(coords: (i32, i32)) -> Self {
        Position { x: coords.0, y: coords.1 }
    }
}

fn main() {
    let start_coords = (5, 10);
    let start_position: Position = start_coords.into();
    println!("Position from tuple: ({}, {})", start_position.x, start_position.y);

    let end_coords = (15, 20);
    let end_position = Position::from(end_coords);
    println!("Position using from: ({}, {})", end_position.x, end_position.y);

    // Multiple conversions
    let positions: Vec<Position> = vec![(1, 1), (2, 2), (3, 3)]
        .into_iter()
        .map(Position::from)
        .collect();

    println!("Created {} positions", positions.len());
}"#,
                completion_indicators: vec![
                    "Position from tuple: (5, 10)", "Position using from: (15, 20)", "Created 3 positions"
                ],
            },
            TaskTest {
                task_number: 5,
                task_name: "Type Inference in Conversions",
                solution_code: r#"fn main() {
    let small = 100_i32;
    let large: i64 = small.into();
    println!("Inferred conversion: {} -> {}", small, large);

    // Parsing with turbofish
    let text = "42";
    let number = text.parse::<i32>().expect("Failed");
    println!("Turbofish parse: {}", number);

    // Collection inference
    let vec = vec![1, 2, 3];
    let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // Function parameter inference
    fn process(value: i64) -> i64 { value * 2 }
    let result = process(50_i32.into());
    println!("Process result: {}", result);
}"#,
                completion_indicators: vec![
                    "Inferred conversion: 100 -> 100", "Turbofish parse: 42", "Doubled: [2, 4, 6]", "Process result: 100"
                ],
            }
        ],
    }
}