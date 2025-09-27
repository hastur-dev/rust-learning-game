// Level 3: Level 3: Primitive Data Types - Automated Test Solutions
// Updated to match actual learning tests

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_3_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 3: Primitive Data Types",
        level_index: 2,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Integers",
                solution_code: r#"fn main() {
    let signed: i32 = -42;
    let large_signed: i64 = -9223372036854775808;
    let unsigned: u32 = 42;
    let large_unsigned: u64 = 18446744073709551615;
    let small: u8 = 255;

    println!("Signed i32: {}", signed);
    println!("Large signed i64: {}", large_signed);
    println!("Unsigned u32: {}", unsigned);
    println!("Large unsigned u64: {}", large_unsigned);
    println!("Small u8: {}", small);
}"#,
                completion_indicators: vec![
                    "Signed i32: -42", "Large signed i64:", "Unsigned u32: 42", "Large unsigned u64:", "Small u8: 255"
                ],
            },
            TaskTest {
                task_number: 2,
                task_name: "Floats",
                solution_code: r#"fn main() {
    let pi: f64 = 3.141592653589793;
    let e = 2.718281828459045;
    let small_float: f32 = 0.1;
    let scientific = 1.23e6;

    println!("Pi (f64): {}", pi);
    println!("E: {}", e);
    println!("Small float (f32): {}", small_float);
    println!("Scientific notation: {}", scientific);
}"#,
                completion_indicators: vec![
                    "Pi (f64): 3.14", "E: 2.71", "Small float", "Scientific notation"
                ],
            },
            TaskTest {
                task_number: 3,
                task_name: "Booleans",
                solution_code: r#"fn main() {
    let is_rust_awesome: bool = true;
    let is_difficult: bool = false;

    let and_result = is_rust_awesome && !is_difficult;
    let or_result = is_rust_awesome || is_difficult;
    let comparison = 5 > 3;

    println!("Rust is awesome: {}", is_rust_awesome);
    println!("Rust is difficult: {}", is_difficult);
    println!("AND result: {}", and_result);
    println!("OR result: {}", or_result);
    println!("5 > 3: {}", comparison);
}"#,
                completion_indicators: vec![
                    "Rust is awesome: true", "Rust is difficult: false", "AND result", "OR result", "5 > 3"
                ],
            },
            TaskTest {
                task_number: 4,
                task_name: "Characters",
                solution_code: r#"fn main() {
    let letter: char = 'A';
    let digit: char = '7';
    let emoji: char = '🦀';
    let unicode: char = '世';

    println!("Letter: {}", letter);
    println!("Digit: {}", digit);
    println!("Emoji: {}", emoji);
    println!("Unicode: {}", unicode);
    println!("Size of char: {} bytes", std::mem::size_of::<char>());
}"#,
                completion_indicators: vec![
                    "Letter: A", "Digit: 7", "Emoji: 🦀", "Unicode:", "Size of char: 4 bytes"
                ],
            },
            TaskTest {
                task_number: 5,
                task_name: "Type Inference",
                solution_code: r#"fn main() {
    // Type inference - Rust figures out the types
    let inferred_int = 42;
    let inferred_float = 3.14;
    let explicit_u8: u8 = 255;

    // Parsing requires type annotation
    let parsed: i32 = "123".parse().expect("Failed to parse");

    println!("Inferred int: {}", inferred_int);
    println!("Inferred float: {}", inferred_float);
    println!("Explicit u8: {}", explicit_u8);
    println!("Parsed i32: {}", parsed);
}"#,
                completion_indicators: vec![
                    "Inferred int: 42", "Inferred float: 3.14", "Explicit u8: 255", "Parsed i32: 123"
                ],
            }
        ],
    }
}