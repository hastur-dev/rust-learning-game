// Level 5: Types and Casting - Automated Test Solutions

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_5_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 5: Types and Casting",
        level_index: 4,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Basic Type Casting",
                solution_code: r#"fn main() {
    println!("Level 5: Types and Casting");

    // Basic numeric casting
    let integer: i32 = 42;
    let float_from_int = integer as f64;
    println!("Integer {} as float: {}", integer, float_from_int);

    // Float to integer casting (truncates)
    let float_value: f64 = 3.14159;
    let int_from_float = float_value as i32;
    println!("Float {} as integer: {}", float_value, int_from_float);

    // Casting between integer types
    let large_int: i64 = 1000000;
    let small_int = large_int as i32;
    println!("i64 {} as i32: {}", large_int, small_int);

    // Unsigned to signed casting
    let unsigned_val: u32 = 255;
    let signed_val = unsigned_val as i32;
    println!("u32 {} as i32: {}", unsigned_val, signed_val);

    // Character to number casting
    let character = 'A';
    let ascii_value = character as u8;
    println!("Character '{}' as ASCII: {}", character, ascii_value);

    // Number to character casting
    let number: u8 = 65;
    let char_from_num = number as char;
    println!("Number {} as character: '{}'", number, char_from_num);

    println!("Basic type casting completed!");
}"#,
                completion_indicators: vec![
                    "Level 5: Types and Casting",
                    "Integer 42 as float: 42",
                    "Float 3.14159 as integer: 3",
                    "i64 1000000 as i32: 1000000",
                    "u32 255 as i32: 255",
                    "Character 'A' as ASCII: 65",
                    "Number 65 as character: 'A'",
                    "Basic type casting completed!",
                ],
            },

            TaskTest {
                task_number: 2,
                task_name: "Safe Parsing and Conversion",
                solution_code: r#"fn main() {
    println!("Level 5: Safe Parsing and Conversion");

    // Safe string to number parsing
    let number_str = "42";
    match number_str.parse::<i32>() {
        Ok(num) => println!("Successfully parsed '{}' to: {}", number_str, num),
        Err(_) => println!("Failed to parse '{}'", number_str),
    }

    // Invalid parsing example
    let invalid_str = "not_a_number";
    match invalid_str.parse::<i32>() {
        Ok(num) => println!("Successfully parsed '{}' to: {}", invalid_str, num),
        Err(_) => println!("Failed to parse '{}'", invalid_str),
    }

    // Using unwrap_or for default values
    let safe_number = "123".parse::<i32>().unwrap_or(0);
    let safe_invalid = "abc".parse::<i32>().unwrap_or(0);
    println!("Safe parsing '123': {}", safe_number);
    println!("Safe parsing 'abc': {}", safe_invalid);

    // Parsing different types
    let bool_str = "true";
    let parsed_bool: bool = bool_str.parse().unwrap_or(false);
    println!("Parsed boolean '{}': {}", bool_str, parsed_bool);

    let float_str = "3.14";
    let parsed_float: f64 = float_str.parse().unwrap_or(0.0);
    println!("Parsed float '{}': {}", float_str, parsed_float);

    // Using expect for error messages
    let expected_number = "456".parse::<i32>().expect("Should be a valid number");
    println!("Expected number: {}", expected_number);

    println!("Safe parsing and conversion completed!");
}"#,
                completion_indicators: vec![
                    "Level 5: Safe Parsing and Conversion",
                    "Successfully parsed '42' to: 42",
                    "Failed to parse 'not_a_number'",
                    "Safe parsing '123': 123",
                    "Safe parsing 'abc': 0",
                    "Parsed boolean 'true': true",
                    "Parsed float '3.14': 3.14",
                    "Expected number: 456",
                    "Safe parsing and conversion completed!",
                ],
            },

            TaskTest {
                task_number: 3,
                task_name: "Working with Aliased Types",
                solution_code: r#"fn main() {
    println!("Level 5: Working with Aliased Types");

    // Type aliases for clarity
    type RobotId = u32;
    type Energy = f64;
    type Position = (i32, i32);

    // Using aliased types
    let robot_id: RobotId = 12345;
    let robot_energy: Energy = 85.5;
    let robot_pos: Position = (10, 20);

    println!("Robot ID: {}", robot_id);
    println!("Robot Energy: {}", robot_energy);
    println!("Robot Position: {:?}", robot_pos);

    // Function using aliased types
    fn create_robot(id: RobotId, energy: Energy, pos: Position) -> String {
        format!("Robot {} at {:?} with {}% energy", id, pos, energy)
    }

    let robot_info = create_robot(robot_id, robot_energy, robot_pos);
    println!("Robot Info: {}", robot_info);

    // Common Rust type aliases
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    type HashMap<K, V> = std::collections::HashMap<K, V>;

    // Using standard library types
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers vector: {:?}", numbers);

    let text: String = String::from("Type aliases make code clearer!");
    println!("Text: {}", text);

    // Complex type alias
    type RobotData = (RobotId, Energy, Position, String);
    let robot_data: RobotData = (robot_id, robot_energy, robot_pos, "Explorer".to_string());
    println!("Complex robot data: {:?}", robot_data);

    println!("Working with aliased types completed!");
}"#,
                completion_indicators: vec![
                    "Level 5: Working with Aliased Types",
                    "Robot ID: 12345",
                    "Robot Energy: 85.5",
                    "Robot Position: (10, 20)",
                    "Robot Info: Robot 12345 at (10, 20) with 85.5% energy",
                    "Numbers vector: [1, 2, 3, 4, 5]",
                    "Text: Type aliases make code clearer!",
                    "Complex robot data: (12345, 85.5, (10, 20), \"Explorer\")",
                    "Working with aliased types completed!",
                ],
            },

            TaskTest {
                task_number: 4,
                task_name: "Type Inference and Turbofish",
                solution_code: r#"fn main() {
    println!("Level 5: Type Inference and Turbofish");

    // Rust can infer types from context
    let inferred = 42; // i32 by default
    let also_inferred = 3.14; // f64 by default
    println!("Inferred integer: {}", inferred);
    println!("Inferred float: {}", also_inferred);

    // When inference isn't enough, use annotations
    let numbers: Vec<i32> = Vec::new();
    println!("Empty vector length: {}", numbers.len());

    // Turbofish syntax (::<>) when inference fails
    let parsed_number = "42".parse::<i32>().unwrap();
    println!("Parsed with turbofish: {}", parsed_number);

    // Collecting with turbofish
    let string_numbers = vec!["1", "2", "3", "4"];
    let collected_numbers: Vec<i32> = string_numbers
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("Collected numbers: {:?}", collected_numbers);

    // Alternative turbofish syntax
    let collected_alt = string_numbers
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("Alternative collection: {:?}", collected_alt);

    // Function that needs type specification
    fn generic_parse<T: std::str::FromStr>(s: &str) -> Option<T> {
        s.parse().ok()
    }

    let result: Option<f64> = generic_parse("3.14");
    println!("Generic parse result: {:?}", result);

    // Using turbofish with generic function
    let turbo_result = generic_parse::<i32>("123");
    println!("Turbofish generic result: {:?}", turbo_result);

    // Multiple type parameters
    let mut map = std::collections::HashMap::<String, i32>::new();
    map.insert("key".to_string(), 42);
    println!("HashMap with turbofish: {:?}", map);

    println!("Type inference and turbofish completed!");
}"#,
                completion_indicators: vec![
                    "Level 5: Type Inference and Turbofish",
                    "Inferred integer: 42",
                    "Inferred float: 3.14",
                    "Empty vector length: 0",
                    "Parsed with turbofish: 42",
                    "Collected numbers: [1, 2, 3, 4]",
                    "Alternative collection: [1, 2, 3, 4]",
                    "Generic parse result: Some(3.14)",
                    "Turbofish generic result: Some(123)",
                    "HashMap with turbofish: {\"key\": 42}",
                    "Type inference and turbofish completed!",
                ],
            },

            TaskTest {
                task_number: 5,
                task_name: "Advanced Casting and Coercion",
                solution_code: r#"fn main() {
    println!("Level 5: Advanced Casting and Coercion");

    // Automatic deref coercion
    let string = String::from("Hello, World!");
    let str_slice: &str = &string; // String -> &str coercion
    println!("String: {}", string);
    println!("Str slice: {}", str_slice);

    // Subtyping with lifetimes (lifetime coercion)
    let longer_lived = String::from("I live longer");
    {
        let shorter_lived = String::from("I live shorter");
        // Both can be used as &str
        print_strings(&longer_lived, &shorter_lived);
    }

    // Casting with potential data loss
    let large_number: u32 = 300;
    let small_number: u8 = large_number as u8; // Truncation occurs
    println!("Large number {} as u8: {}", large_number, small_number);

    // Signed/unsigned casting edge cases
    let negative: i8 = -50;
    let as_unsigned = negative as u8;
    println!("Negative {} as u8: {}", negative, as_unsigned);

    // Floating point precision loss
    let precise_float: f64 = 3.14159265359;
    let less_precise = precise_float as f32;
    println!("f64: {}", precise_float);
    println!("f32: {}", less_precise);

    // Boolean casting (not allowed directly, must be explicit)
    let true_as_num = true as u8;
    let false_as_num = false as u8;
    println!("True as number: {}", true_as_num);
    println!("False as number: {}", false_as_num);

    // Pointer casting (advanced)
    let number = 42i32;
    let number_ptr = &number as *const i32;
    println!("Number address: {:p}", number_ptr);

    // Reference to raw pointer
    let raw_value = unsafe { *number_ptr };
    println!("Value from raw pointer: {}", raw_value);

    println!("Advanced casting and coercion completed!");
}

fn print_strings(s1: &str, s2: &str) {
    println!("String 1: {}", s1);
    println!("String 2: {}", s2);
}"#,
                completion_indicators: vec![
                    "Level 5: Advanced Casting and Coercion",
                    "String: Hello, World!",
                    "Str slice: Hello, World!",
                    "String 1: I live longer",
                    "String 2: I live shorter",
                    "Large number 300 as u8: 44",
                    "Negative -50 as u8: 206",
                    "f64: 3.14159265359",
                    "f32: 3.1415927",
                    "True as number: 1",
                    "False as number: 0",
                    "Number address:",
                    "Value from raw pointer: 42",
                    "Advanced casting and coercion completed!",
                ],
            },
        ],
    }
}