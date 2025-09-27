// Level 4: Level 4: Variable Bindings and Mutability - Automated Test Solutions
// Updated to match actual learning tests

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_4_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 4: Variable Bindings and Mutability",
        level_index: 3,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Immutable Variables",
                solution_code: r#"fn main() {
    let robot_name = "Ferris";
    let robot_id = 1234;
    let max_energy = 100;

    println!("Robot: {}", robot_name);
    println!("ID: {}", robot_id);
    println!("Max Energy: {}", max_energy);

    // Use in calculations
    let half_energy = max_energy / 2;
    println!("Half energy: {}", half_energy);

    if robot_id > 1000 {
        println!("High-level robot detected");
    }
}"#,
                completion_indicators: vec![
                    "Robot: Ferris", "ID: 1234", "Max Energy: 100", "Half energy: 50", "High-level robot"
                ],
            },
            TaskTest {
                task_number: 2,
                task_name: "Mutable Variables",
                solution_code: r#"fn main() {
    let mut robot_position = 0;
    let mut energy_level = 100;
    let mut items_collected = 0;

    println!("Starting position: {}", robot_position);
    println!("Starting energy: {}", energy_level);

    // Modify variables
    robot_position += 5;
    energy_level -= 10;
    items_collected += 1;

    println!("After move - Position: {}", robot_position);
    println!("After move - Energy: {}", energy_level);
    println!("Items: {}", items_collected);

    // Loop with mutations
    for _ in 0..3 {
        robot_position += 1;
        energy_level -= 5;
    }

    println!("Final position: {}", robot_position);
    println!("Final energy: {}", energy_level);
}"#,
                completion_indicators: vec![
                    "Starting position: 0", "Starting energy: 100", "After move - Position: 5", "Final position: 8", "Final energy"
                ],
            },
            TaskTest {
                task_number: 3,
                task_name: "Shadowing",
                solution_code: r#"fn main() {
    let robot_data = "12345";
    println!("Robot data (string): {}", robot_data);

    let robot_data: i32 = robot_data.parse().expect("Failed to parse");
    println!("Robot data (number): {}", robot_data);

    let robot_data = robot_data * 2;
    println!("Robot data (doubled): {}", robot_data);

    {
        let robot_data = "inner scope";
        println!("Robot data (inner): {}", robot_data);
    }

    println!("Robot data (outer): {}", robot_data);
}"#,
                completion_indicators: vec![
                    "Robot data (string): 12345", "Robot data (number): 12345", "Robot data (doubled): 24690", "Robot data (inner): inner scope", "Robot data (outer): 24690"
                ],
            },
            TaskTest {
                task_number: 4,
                task_name: "Scope",
                solution_code: r#"fn main() {
    let outer_variable = "I'm in the outer scope";

    println!("Outer: {}", outer_variable);

    {
        let inner_variable = "I'm in the inner scope";
        println!("Inner: {}", inner_variable);
        println!("Can access outer: {}", outer_variable);
    }

    // inner_variable is not accessible here
    println!("Back to outer: {}", outer_variable);

    helper_function();
}

fn helper_function() {
    let function_scope = "I'm in function scope";
    println!("Function: {}", function_scope);
}"#,
                completion_indicators: vec![
                    "Outer: I'm in the outer scope", "Inner: I'm in the inner scope", "Can access outer:", "Back to outer:", "Function: I'm in function scope"
                ],
            },
            TaskTest {
                task_number: 5,
                task_name: "Constants",
                solution_code: r#"const MAX_ENERGY: u32 = 100;
const MIN_ENERGY: u32 = 0;
const ROBOT_VERSION: f32 = 2.5;

fn main() {
    println!("Maximum energy: {}", MAX_ENERGY);
    println!("Minimum energy: {}", MIN_ENERGY);
    println!("Robot version: {}", ROBOT_VERSION);

    let current_energy = 75;
    let energy_percentage = (current_energy as f32 / MAX_ENERGY as f32) * 100.0;
    println!("Energy at {}%", energy_percentage);

    const SPEED_MULTIPLIER: f32 = 1.5;
    println!("Speed multiplier: {}", SPEED_MULTIPLIER);
}"#,
                completion_indicators: vec![
                    "Maximum energy: 100", "Minimum energy: 0", "Robot version: 2.5", "Energy at 75%", "Speed multiplier: 1.5"
                ],
            }
        ],
    }
}