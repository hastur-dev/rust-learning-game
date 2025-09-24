// Level 1: Hello Rust - Automated Test Solutions
// This file contains all the task solutions for automated testing

pub struct LevelTestConfig {
    pub level_name: &'static str,
    pub level_index: usize,
    pub tasks: Vec<TaskTest>,
}

pub struct TaskTest {
    pub task_number: usize,
    pub task_name: &'static str,
    pub solution_code: &'static str,
    pub completion_indicators: Vec<&'static str>,
}

pub fn get_level_1_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 1: Hello Rust",
        level_index: 0,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Basic Println",
                solution_code: r#"fn main() {
    println!("Hello, Rust!");
    println!("This is my first Rust program!");

    // Basic arithmetic
    let result = 5 + 3;
    println!("5 + 3 = {}", result);

    // String creation
    let message = "Welcome to Rust programming!";
    println!("{}", message);

    // Multiple variables
    let x = 10;
    let y = 20;
    println!("x = {}, y = {}", x, y);
    println!("x + y = {}", x + y);
}"#,
                completion_indicators: vec![
                    "Hello, Rust!",
                    "This is my first Rust program!",
                    "5 + 3 = 8",
                    "Welcome to Rust programming!",
                    "x = 10, y = 20",
                    "x + y = 30",
                ],
            },

            TaskTest {
                task_number: 2,
                task_name: "Variables and Printing",
                solution_code: r#"fn main() {
    let name = "Ferris";
    let age = 5;
    let is_awesome = true;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is awesome: {}", is_awesome);

    // Calculations
    let current_year = 2024;
    let birth_year = current_year - age;
    println!("Born in: {}", birth_year);

    // Multiple formats
    println!("Hello, {}! You are {} years old.", name, age);
    println!("{} was created in {}.", name, birth_year);

    // Boolean logic
    if is_awesome {
        println!("{} is indeed awesome!", name);
    }
}"#,
                completion_indicators: vec![
                    "Name: Ferris",
                    "Age: 5",
                    "Is awesome: true",
                    "Born in: 2019",
                    "Hello, Ferris! You are 5 years old.",
                    "Ferris was created in 2019.",
                    "Ferris is indeed awesome!",
                ],
            },

            TaskTest {
                task_number: 3,
                task_name: "Comments and Documentation",
                solution_code: r#"// This is Level 1 Task 3: Comments and Documentation
// Learning how to document Rust code properly

fn main() {
    // Single line comment
    println!("Learning about comments in Rust");

    /*
     * Multi-line comment block
     * Used for longer explanations
     */
    println!("Multi-line comments are useful for detailed explanations");

    // Variables with comments
    let robot_name = "Ferris"; // The Rust mascot
    let robot_type = "Crab"; // A friendly crab

    println!("Meet {}, the {} robot!", robot_name, robot_type);

    // TODO: This is a todo comment for future improvements
    println!("TODO comments help track future work");

    // FIXME: Example of a fixme comment
    println!("FIXME comments mark things that need attention");

    // NOTE: Documentation comments
    println!("Comments make code easier to understand");

    /// This is a documentation comment (though not used in main function typically)
    println!("Documentation comments use three slashes");
}"#,
                completion_indicators: vec![
                    "Learning about comments in Rust",
                    "Multi-line comments are useful for detailed explanations",
                    "Meet Ferris, the Crab robot!",
                    "TODO comments help track future work",
                    "FIXME comments mark things that need attention",
                    "Comments make code easier to understand",
                    "Documentation comments use three slashes",
                ],
            },

            TaskTest {
                task_number: 4,
                task_name: "Basic Error Handling",
                solution_code: r#"fn main() {
    println!("Learning basic error concepts in Rust");

    // Demonstrating expect() for error handling
    let number_str = "42";
    let number: i32 = number_str.parse().expect("Failed to parse number");

    println!("Successfully parsed: {}", number);

    // Safe parsing with match
    let test_strings = ["123", "abc", "456"];

    for s in test_strings.iter() {
        match s.parse::<i32>() {
            Ok(num) => println!("'{}' parsed to: {}", s, num),
            Err(_) => println!("'{}' could not be parsed as a number", s),
        }
    }

    // Using unwrap_or for defaults
    let maybe_number = "invalid".parse::<i32>().unwrap_or(0);
    println!("Parsed with default: {}", maybe_number);

    // Demonstrating panic prevention
    println!("Rust helps us handle errors safely!");

    // Option example
    let numbers = vec![1, 2, 3];
    match numbers.get(1) {
        Some(value) => println!("Found value at index 1: {}", value),
        None => println!("No value found at index 1"),
    }
}"#,
                completion_indicators: vec![
                    "Learning basic error concepts in Rust",
                    "Successfully parsed: 42",
                    "'123' parsed to: 123",
                    "'abc' could not be parsed as a number",
                    "'456' parsed to: 456",
                    "Parsed with default: 0",
                    "Rust helps us handle errors safely!",
                    "Found value at index 1: 2",
                ],
            },

            TaskTest {
                task_number: 5,
                task_name: "Using Rust Tools",
                solution_code: r#"fn main() {
    println!("Exploring Rust's built-in tools and features");

    // Using format! macro
    let formatted = format!("Rust is {} years old!", 14);
    println!("{}", formatted);

    // Using vec! macro
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Vector: {:?}", numbers);

    // Using assert! for testing concepts
    let x = 5;
    let y = 5;
    assert!(x == y, "x and y should be equal");
    println!("Assertion passed: {} equals {}", x, y);

    // Using dbg! macro for debugging
    let calculation = 10 * 2;
    println!("Calculation result: {}", calculation);

    // String methods
    let message = "hello rust";
    let capitalized = message.to_uppercase();
    println!("Original: {}", message);
    println!("Capitalized: {}", capitalized);

    // Basic collections
    let mut items = vec!["apple", "banana", "cherry"];
    items.push("date");
    println!("Items: {:?}", items);
    println!("First item: {}", items[0]);
    println!("Number of items: {}", items.len());

    // Range iteration
    println!("Counting from 1 to 5:");
    for i in 1..=5 {
        println!("  Count: {}", i);
    }

    println!("Level 1 complete! Ready for functions and loops!");
}"#,
                completion_indicators: vec![
                    "Exploring Rust's built-in tools and features",
                    "Rust is 14 years old!",
                    "Vector: [1, 2, 3, 4, 5]",
                    "Assertion passed: 5 equals 5",
                    "Calculation result: 20",
                    "Original: hello rust",
                    "Capitalized: HELLO RUST",
                    "Items: [\"apple\", \"banana\", \"cherry\", \"date\"]",
                    "First item: apple",
                    "Number of items: 4",
                    "Counting from 1 to 5:",
                    "  Count: 1",
                    "  Count: 2",
                    "  Count: 3",
                    "  Count: 4",
                    "  Count: 5",
                    "Level 1 complete! Ready for functions and loops!",
                ],
            },
        ],
    }
}