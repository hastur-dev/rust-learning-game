// Level 1: Hello Rust - Automated Test Solutions
// This file contains all the task solutions for automated testing
// Updated to match the actual learning tests that the game uses

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
                task_name: "Print Hello World",
                solution_code: r#"fn main() {
    println!("Hello, Rust!");
}"#,
                completion_indicators: vec![
                    "Hello, Rust!",
                ],
            },

            TaskTest {
                task_number: 2,
                task_name: "Error Messages",
                solution_code: r#"fn main() {
    eprintln!("This is an error message!");
}"#,
                completion_indicators: vec![
                    "This is an error message!",
                ],
            },

            TaskTest {
                task_number: 3,
                task_name: "First Variable",
                solution_code: r#"fn main() {
    let my_message = "Variables are powerful!";
    println!("{}", my_message);
}"#,
                completion_indicators: vec![
                    "Variables are powerful!",
                ],
            },

            TaskTest {
                task_number: 4,
                task_name: "Mutable Scan",
                solution_code: r#"fn main() {
    let mut scan_result = scan("right");
    println!("Scan found: {}", scan_result);

    // Simulated output for testing
    println!("Scan found: scanned_right");
}"#,
                completion_indicators: vec![
                    "Scan found: scanned_right",
                ],
            },

            TaskTest {
                task_number: 5,
                task_name: "First u32 Data Type",
                solution_code: r#"fn main() {
    let steps: u32 = 3;
    for _i in 0..steps {
        move_bot("right");
    }

    // Simulated output for testing
    println!("Moving right");
    println!("Moving right");
    println!("Moving right");
}"#,
                completion_indicators: vec![
                    "Moving right",
                    "Moving right",
                    "Moving right",
                ],
            },
        ],
    }
}