// Level 4: Variable Bindings and Mutability - Automated Test Solutions

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_4_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 4: Variable Bindings and Mutability",
        level_index: 3,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Basic Variable Binding",
                solution_code: r#"fn main() {
    println!("Level 4: Variable Bindings and Mutability");

    // Basic variable binding
    let robot_name = "Ferris";
    let robot_level = 4;
    let is_active = true;

    println!("Robot Name: {}", robot_name);
    println!("Robot Level: {}", robot_level);
    println!("Is Active: {}", is_active);

    // Binding with type annotations
    let energy: f64 = 100.0;
    let health: u32 = 95;

    println!("Energy: {}", energy);
    println!("Health: {}", health);

    // Multiple bindings in one statement
    let (x, y) = (10, 20);
    println!("Position: ({}, {})", x, y);

    println!("Variable binding completed!");
}"#,
                completion_indicators: vec![
                    "Level 4: Variable Bindings and Mutability",
                    "Robot Name: Ferris",
                    "Robot Level: 4",
                    "Is Active: true",
                    "Energy: 100",
                    "Health: 95",
                    "Position: (10, 20)",
                    "Variable binding completed!",
                ],
            },

            TaskTest {
                task_number: 2,
                task_name: "Mutable Variables",
                solution_code: r#"fn main() {
    println!("Level 4: Mutable Variables");

    // Mutable variables can be changed
    let mut robot_energy = 100;
    let mut robot_position = (0, 0);

    println!("Initial energy: {}", robot_energy);
    println!("Initial position: {:?}", robot_position);

    // Modify the variables
    robot_energy -= 25;
    robot_position.0 = 5;
    robot_position.1 = 3;

    println!("Energy after move: {}", robot_energy);
    println!("New position: {:?}", robot_position);

    // Multiple mutations
    let mut counter = 0;
    counter += 1;
    counter *= 2;
    counter -= 1;

    println!("Counter value: {}", counter);

    // String mutations
    let mut message = String::from("Hello");
    message.push_str(", Rust!");
    println!("Message: {}", message);

    println!("Mutable variables completed!");
}"#,
                completion_indicators: vec![
                    "Level 4: Mutable Variables",
                    "Initial energy: 100",
                    "Initial position: (0, 0)",
                    "Energy after move: 75",
                    "New position: (5, 3)",
                    "Counter value: 1",
                    "Message: Hello, Rust!",
                    "Mutable variables completed!",
                ],
            },

            TaskTest {
                task_number: 3,
                task_name: "Variable Shadowing",
                solution_code: r#"fn main() {
    println!("Level 4: Variable Shadowing");

    // Original variable
    let x = 5;
    println!("Original x: {}", x);

    // Shadow with same name but different value
    let x = x + 1;
    println!("Shadowed x: {}", x);

    // Shadow with different type
    let x = "now I'm a string";
    println!("Shadowed x (string): {}", x);

    // Shadowing in inner scope
    {
        let x = 100;
        println!("Inner scope x: {}", x);
    }
    println!("Outer scope x: {}", x);

    // Multiple shadowing example
    let spaces = "   ";
    println!("Spaces as string: '{}'", spaces);
    let spaces = spaces.len();
    println!("Spaces as length: {}", spaces);

    // Shadowing vs mutation example
    let robot_name = "R2D2";
    println!("Robot: {}", robot_name);
    let robot_name = format!("{}-Upgraded", robot_name);
    println!("Upgraded Robot: {}", robot_name);

    println!("Variable shadowing completed!");
}"#,
                completion_indicators: vec![
                    "Level 4: Variable Shadowing",
                    "Original x: 5",
                    "Shadowed x: 6",
                    "Shadowed x (string): now I'm a string",
                    "Inner scope x: 100",
                    "Outer scope x: now I'm a string",
                    "Spaces as string: '   '",
                    "Spaces as length: 3",
                    "Robot: R2D2",
                    "Upgraded Robot: R2D2-Upgraded",
                    "Variable shadowing completed!",
                ],
            },

            TaskTest {
                task_number: 4,
                task_name: "Scope and Lifetimes",
                solution_code: r#"fn main() {
    println!("Level 4: Scope and Lifetimes");

    // Variables live within their scope
    let outer_var = "I'm in outer scope";
    println!("Outer variable: {}", outer_var);

    // Inner scope
    {
        let inner_var = "I'm in inner scope";
        println!("Inner variable: {}", inner_var);
        println!("Can access outer: {}", outer_var);

        // Shadow outer variable
        let outer_var = "Shadowed in inner scope";
        println!("Shadowed outer: {}", outer_var);
    }

    // inner_var is no longer accessible here
    println!("Back to outer: {}", outer_var);

    // Multiple nested scopes
    let level = 1;
    {
        let level = 2;
        println!("Level 2 scope: {}", level);
        {
            let level = 3;
            println!("Level 3 scope: {}", level);
        }
        println!("Back to level 2: {}", level);
    }
    println!("Back to level 1: {}", level);

    // Variable dropping demonstration
    {
        let temp_data = vec![1, 2, 3, 4, 5];
        println!("Temp data: {:?}", temp_data);
    } // temp_data is dropped here

    println!("Scope and lifetimes completed!");
}"#,
                completion_indicators: vec![
                    "Level 4: Scope and Lifetimes",
                    "Outer variable: I'm in outer scope",
                    "Inner variable: I'm in inner scope",
                    "Can access outer: I'm in outer scope",
                    "Shadowed outer: Shadowed in inner scope",
                    "Back to outer: I'm in outer scope",
                    "Level 2 scope: 2",
                    "Level 3 scope: 3",
                    "Back to level 2: 2",
                    "Back to level 1: 1",
                    "Temp data: [1, 2, 3, 4, 5]",
                    "Scope and lifetimes completed!",
                ],
            },

            TaskTest {
                task_number: 5,
                task_name: "Constants and Static Variables",
                solution_code: r#"// Constants are always immutable and must have type annotations
const MAX_ENERGY: u32 = 1000;
const GAME_VERSION: &str = "1.0.0";

// Static variables have 'static lifetime
static ROBOT_COUNT: u32 = 42;

fn main() {
    println!("Level 4: Constants and Static Variables");

    // Using constants
    println!("Maximum energy: {}", MAX_ENERGY);
    println!("Game version: {}", GAME_VERSION);
    println!("Robot count: {}", ROBOT_COUNT);

    // Constants can be used in any scope
    {
        println!("Max energy in inner scope: {}", MAX_ENERGY);

        // Local constant (uncommon but possible)
        const LOCAL_MULTIPLIER: u32 = 2;
        let boosted_energy = MAX_ENERGY * LOCAL_MULTIPLIER;
        println!("Boosted energy: {}", boosted_energy);
    }

    // Constants vs variables
    let current_energy = 750;
    let energy_percentage = (current_energy as f32 / MAX_ENERGY as f32) * 100.0;
    println!("Energy percentage: {:.1}%", energy_percentage);

    // Math with constants
    const DAMAGE_PER_HIT: u32 = 50;
    let hits_to_deplete = MAX_ENERGY / DAMAGE_PER_HIT;
    println!("Hits to deplete energy: {}", hits_to_deplete);

    // String constants
    println!("Running {} with {} robots", GAME_VERSION, ROBOT_COUNT);

    println!("Constants and static variables completed!");
}"#,
                completion_indicators: vec![
                    "Level 4: Constants and Static Variables",
                    "Maximum energy: 1000",
                    "Game version: 1.0.0",
                    "Robot count: 42",
                    "Max energy in inner scope: 1000",
                    "Boosted energy: 2000",
                    "Energy percentage: 75.0%",
                    "Hits to deplete energy: 20",
                    "Running 1.0.0 with 42 robots",
                    "Constants and static variables completed!",
                ],
            },
        ],
    }
}