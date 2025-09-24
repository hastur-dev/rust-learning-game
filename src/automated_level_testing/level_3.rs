// Level 3: Primitives and Data Types - Automated Test Solutions

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_3_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 3: Primitives and Data Types",
        level_index: 2,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Work with Integer Types",
                solution_code: r#"fn main() {
    println!("Level 3: Primitives and Data Types");

    // Signed integers (can be negative)
    let signed: i32 = -42;
    let large_signed: i64 = -1_000_000;

    // Unsigned integers (only positive)
    let unsigned: u32 = 255;
    let small_unsigned: u8 = 200;

    println!("Signed i32: {}", signed);
    println!("Large signed i64: {}", large_signed);
    println!("Unsigned u32: {}", unsigned);
    println!("Small unsigned u8: {}", small_unsigned);

    // Basic arithmetic
    let sum = signed + 100;
    let product = unsigned * 2;

    println!("Signed + 100: {}", sum);
    println!("Unsigned * 2: {}", product);

    println!("Integer types completed!");
}"#,
                completion_indicators: vec![
                    "Level 3: Primitives and Data Types",
                    "Signed i32: -42",
                    "Large signed i64: -1000000",
                    "Unsigned u32: 255",
                    "Small unsigned u8: 200",
                    "Signed + 100: 58",
                    "Unsigned * 2: 510",
                    "Integer types completed!",
                ],
            },

            TaskTest {
                task_number: 2,
                task_name: "Floating Point Numbers",
                solution_code: r#"fn main() {
    println!("Level 3: Floating Point Numbers");

    // f64 is the default (double precision)
    let pi: f64 = 3.141592653589793;
    let e = 2.71828; // Type inferred as f64

    // f32 is single precision (less precise)
    let pi_f32: f32 = 3.14159;

    // Scientific notation
    let large_num: f64 = 1.23e6; // 1,230,000
    let small_num: f64 = 1.23e-6; // 0.00000123

    println!("Pi (f64): {}", pi);
    println!("E (f64): {}", e);
    println!("Pi (f32): {}", pi_f32);
    println!("Large number: {}", large_num);
    println!("Small number: {}", small_num);

    // Float arithmetic
    let radius = 5.0;
    let area = pi * radius * radius;
    println!("Circle area (radius {}): {}", radius, area);

    println!("Floating point numbers completed!");
}"#,
                completion_indicators: vec![
                    "Level 3: Floating Point Numbers",
                    "Pi (f64): 3.141592653589793",
                    "E (f64): 2.71828",
                    "Pi (f32): 3.14159",
                    "Large number: 1230000",
                    "Small number: 0.00000123",
                    "Circle area (radius 5): 78.53981633974483",
                    "Floating point numbers completed!",
                ],
            },

            TaskTest {
                task_number: 3,
                task_name: "Boolean Values and Logic",
                solution_code: r#"fn main() {
    println!("Level 3: Boolean Values and Logic");

    // Basic boolean values
    let is_rust_awesome: bool = true;
    let is_difficult: bool = false;

    // Boolean operations
    let both_true = is_rust_awesome && is_difficult; // AND
    let either_true = is_rust_awesome || is_difficult; // OR
    let not_difficult = !is_difficult; // NOT

    println!("Rust is awesome: {}", is_rust_awesome);
    println!("Is difficult: {}", is_difficult);
    println!("Both true: {}", both_true);
    println!("Either true: {}", either_true);
    println!("Not difficult: {}", not_difficult);

    // Comparison operations
    let x = 10;
    let y = 20;
    let is_greater = x > y;
    let is_equal = x == y;
    let is_not_equal = x != y;

    println!("{} > {}: {}", x, y, is_greater);
    println!("{} == {}: {}", x, y, is_equal);
    println!("{} != {}: {}", x, y, is_not_equal);

    println!("Boolean logic completed!");
}"#,
                completion_indicators: vec![
                    "Level 3: Boolean Values and Logic",
                    "Rust is awesome: true",
                    "Is difficult: false",
                    "Both true: false",
                    "Either true: true",
                    "Not difficult: true",
                    "10 > 20: false",
                    "10 == 20: false",
                    "10 != 20: true",
                    "Boolean logic completed!",
                ],
            },

            TaskTest {
                task_number: 4,
                task_name: "Character Type and Unicode",
                solution_code: r#"fn main() {
    println!("Level 3: Character Type and Unicode");

    // Basic ASCII characters
    let letter: char = 'A';
    let digit: char = '7';
    let symbol: char = '$';

    // Unicode characters
    let heart: char = '♥';
    let lambda: char = 'λ';

    // Emoji (also Unicode!)
    let crab: char = '🦀';  // Rust's mascot
    let robot: char = '🤖';

    println!("Letter: {}", letter);
    println!("Digit: {}", digit);
    println!("Symbol: {}", symbol);
    println!("Heart: {}", heart);
    println!("Lambda: {}", lambda);
    println!("Crab (Rust): {}", crab);
    println!("Robot: {}", robot);

    // Characters are 4 bytes (full Unicode support)
    println!("Size of char: {} bytes", std::mem::size_of::<char>());

    println!("Character types and Unicode completed!");
}"#,
                completion_indicators: vec![
                    "Level 3: Character Type and Unicode",
                    "Letter: A",
                    "Digit: 7",
                    "Symbol: $",
                    "Heart: ♥",
                    "Lambda: λ",
                    "Crab (Rust): 🦀",
                    "Robot: 🤖",
                    "Size of char: 4 bytes",
                    "Character types and Unicode completed!",
                ],
            },

            TaskTest {
                task_number: 5,
                task_name: "Type Inference and Annotations",
                solution_code: r#"fn main() {
    println!("Level 3: Type Inference and Annotations");

    // Type inference - Rust figures out the types
    let inferred_int = 42;        // i32 by default
    let inferred_float = 3.14;    // f64 by default
    let inferred_bool = true;     // bool
    let inferred_char = 'R';      // char

    println!("Inferred integer: {} (type: i32)", inferred_int);
    println!("Inferred float: {} (type: f64)", inferred_float);
    println!("Inferred bool: {} (type: bool)", inferred_bool);
    println!("Inferred char: {} (type: char)", inferred_char);

    // Explicit type annotations
    let explicit_u64: u64 = 1000;
    let explicit_f32: f32 = 2.5;
    let explicit_i8: i8 = -128;

    println!("Explicit u64: {}", explicit_u64);
    println!("Explicit f32: {}", explicit_f32);
    println!("Explicit i8: {}", explicit_i8);

    // Type annotations needed for ambiguous cases
    let parsed_number: i32 = "42".parse().expect("Failed to parse");
    println!("Parsed number: {}", parsed_number);

    // Suffix notation (alternative to annotations)
    let suffix_u32 = 100u32;
    let suffix_f32 = 3.14f32;
    println!("Suffix u32: {}", suffix_u32);
    println!("Suffix f32: {}", suffix_f32);

    println!("Type inference and annotations completed!");
}"#,
                completion_indicators: vec![
                    "Level 3: Type Inference and Annotations",
                    "Inferred integer: 42 (type: i32)",
                    "Inferred float: 3.14 (type: f64)",
                    "Inferred bool: true (type: bool)",
                    "Inferred char: R (type: char)",
                    "Explicit u64: 1000",
                    "Explicit f32: 2.5",
                    "Explicit i8: -128",
                    "Parsed number: 42",
                    "Suffix u32: 100",
                    "Suffix f32: 3.14",
                    "Type inference and annotations completed!",
                ],
            },
        ],
    }
}