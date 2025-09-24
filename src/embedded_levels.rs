use crate::level::{YamlLevelConfig, LevelSpec, ItemConfig, TaskConfig};
use rand::{rngs::StdRng, SeedableRng};
use std::fs;

// Function to load tasks from separate YAML files
fn load_level_tasks(level_number: u32) -> Option<Vec<TaskConfig>> {
    let _task_file_path = format!("learning_levels/{:02}_*_tasks.yaml", level_number);
    
    // Try specific filenames we know exist
    let possible_paths = match level_number {
        1 => vec!["learning_levels/01_hello_rust_tasks.yaml"],
        2 => vec!["learning_levels/02_functions_and_loops_tasks.yaml"],
        3 => vec!["learning_levels/03_primitives_data_types_tasks.yaml"],
        4 => vec!["learning_levels/04_variable_bindings_mutability_tasks.yaml"],
        5 => vec!["learning_levels/05_types_casting_tasks.yaml"],
        6 => vec!["learning_levels/06_ownership_basics_tasks.yaml"],
        _ => vec![]
    };
    
    for path in possible_paths {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(config) = serde_yaml::from_str::<YamlLevelConfig>(&content) {
                return config.tasks;
            }
        }
    }
    
    None
}

// Embedded learning levels - these are core levels included in the executable
pub fn get_embedded_learning_levels() -> Vec<YamlLevelConfig> {
    let levels = vec![
        // Level 1: Hello Rust!
        YamlLevelConfig {
            name: "Level 1 - Hello Rust!".to_string(),
            grid_size: "12x8".to_string(),
            obstacles: Some(3),
            doors: None,
            enemies: None,
            items: Some(vec![
                ItemConfig {
                    name: "hello_world_tip".to_string(),
                    item_file: "items/hello_world.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((10, 6)),
                },
                ItemConfig {
                    name: "goal_item".to_string(),
                    item_file: "items/level_complete.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((8, 2)),
                }
            ]),
            tasks: load_level_tasks(1),
            income_per_square: Some(1),
            start_position: Some((1, 1)),
            max_turns: Some(0),
            fog_of_war: Some(true),
            message: Some("Welcome to Rust Robot Programming! 🦀 Your goal: Navigate to collect all items and reach the goal. Use basic movement commands (move, grab, scan) to explore. This level introduces Rust basics and the println! macro for output.".to_string()),
            hint_message: Some("Use println!(\"message\") to display text. The exclamation mark means it's a macro, not a function!".to_string()),
            rust_docs_url: Some("https://doc.rust-lang.org/rust-by-example/hello.html".to_string()),
            starting_code: Some(r#"// Welcome to Rust! Let's start with the classic Hello World program.
// Your task: Make this program print "Hello, Rust!" to complete the level.

fn main() {
    // TODO: Add your println! statement here
    // println!("Hello, Rust!");
}
"#.to_string()),
            next_level_hint: Some("Next: Learn about functions, loops, and organizing code!".to_string()),
            achievement_message: Some("🎉 Congratulations! You've completed your first Rust program!".to_string()),
            completion_condition: None,
            completion_flag: Some("println:Hello, Rust!".to_string()),
            completion_message: None,
        },
        
        // Level 2: Functions and Loops
        YamlLevelConfig {
            name: "Level 2: Functions and Loops".to_string(),
            grid_size: "6x6".to_string(),
            obstacles: Some(0),
            doors: None, 
            enemies: None,
            items: Some(vec![
                ItemConfig {
                    name: "key".to_string(),
                    item_file: "items/key.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((3, 0)),
                },
                ItemConfig {
                    name: "goal_item".to_string(),
                    item_file: "items/level_complete.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((5, 5)),
                }
            ]),
            tasks: load_level_tasks(2),
            income_per_square: Some(1),
            start_position: Some((0, 0)),
            max_turns: Some(150),
            fog_of_war: Some(false),
            message: Some("🎯 **LEVEL 2: Functions, Loops, and Structs** - Learn to organize your code effectively and process data systematically!".to_string()),
            hint_message: Some("Create functions to organize your code, use loops to repeat actions, and structs to organize data. All code must be in functions!".to_string()),
            rust_docs_url: Some("https://doc.rust-lang.org/book/ch03-03-how-functions-work.html".to_string()),
            starting_code: Some(r#"// Level 2: Functions, Loops, and Structs
// All code must be organized into functions!

// TODO: Task 3 - Define your GridInfo struct here
// struct GridInfo {
//     x: i32,
//     y: i32, 
//     content: String,
// }

// TODO: Task 1 - Create scan_level function with print statement
// fn scan_level() {
//     println!("Beginning level scan...");
//     
//     // TODO: Task 2 - Add nested loops here
//     // for y in 0..6 {
//     //     for x in 0..6 {
//     //         // Movement and scanning code
//     //     }
//     // }
// }

// TODO: Task 4 - Create grab_if_item function
// fn grab_if_item(scan_result: &str) {
//     // Add if statement to check for items and grab them
// }

fn main() {
    println!("Level 2: Functions, Loops, and Structs");
    println!("Remember: All code must be in functions!");
    
    // TODO: Call your scan_level() function here
    // scan_level();
    
    // Navigate to goal when done
    println!("All tasks complete! Moving to goal...");
    // You'll need movement code to reach (5,5)
}
"#.to_string()),
            next_level_hint: Some("Next level: Error handling and advanced movement patterns".to_string()),
            achievement_message: Some("Perfect! You've mastered function organization, loops, structs, and conditional logic!".to_string()),
            completion_condition: None,
            completion_flag: Some("items_collected:2".to_string()),
            completion_message: None,
        },

        // Level 3: Primitives and Data Types
        YamlLevelConfig {
            name: "Level 3: Primitives and Data Types".to_string(),
            grid_size: "8x6".to_string(),
            obstacles: Some(2),
            doors: None,
            enemies: None,
            items: Some(vec![
                ItemConfig {
                    name: "integer_token".to_string(),
                    item_file: "items/integer.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((2, 1)),
                },
                ItemConfig {
                    name: "float_token".to_string(),
                    item_file: "items/float.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((5, 2)),
                },
                ItemConfig {
                    name: "goal_item".to_string(),
                    item_file: "items/level_complete.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((7, 5)),
                }
            ]),
            tasks: load_level_tasks(3),
            income_per_square: Some(1),
            start_position: Some((0, 0)),
            max_turns: Some(100),
            fog_of_war: Some(false),
            message: Some("🔢 **LEVEL 3: Primitives and Data Types** - Master Rust's fundamental data types: integers, floats, booleans, characters, and type inference!".to_string()),
            hint_message: Some("Learn about i32/u32, f64, bool, char, and how Rust infers types. Each type has specific properties and uses.".to_string()),
            rust_docs_url: Some("https://doc.rust-lang.org/book/ch03-02-data-types.html".to_string()),
            starting_code: Some(r#"// Level 3: Primitives and Data Types
// Learn about Rust's fundamental data types

fn main() {
    println!("Level 3: Primitives and Data Types");

    // TODO: Task 1 - Work with integers (i32, u32, i64, u8)
    // let signed: i32 = -42;
    // let unsigned: u32 = 255;

    // TODO: Task 2 - Floating point numbers (f64, f32)
    // let pi: f64 = 3.141592653589793;
    // let large_num: f64 = 1.23e6;

    // TODO: Task 3 - Boolean values and logic
    // let is_rust_awesome: bool = true;
    // let both_true = is_rust_awesome && false;

    // TODO: Task 4 - Character type and Unicode
    // let heart: char = '♥';
    // let crab: char = '🦀';

    // TODO: Task 5 - Type inference and annotations
    // let inferred_int = 42;
    // let explicit_u64: u64 = 1000;

    println!("Complete all tasks to master Rust data types!");
}
"#.to_string()),
            next_level_hint: Some("Next: Variable bindings, mutability, and memory safety!".to_string()),
            achievement_message: Some("🎉 Excellent! You understand Rust's type system and primitives!".to_string()),
            completion_condition: None,
            completion_flag: Some("goal".to_string()),
            completion_message: None,
        },

        // Level 4: Variable Bindings and Mutability
        YamlLevelConfig {
            name: "Level 4: Variable Bindings and Mutability".to_string(),
            grid_size: "9x7".to_string(),
            obstacles: Some(3),
            doors: None,
            enemies: None,
            items: Some(vec![
                ItemConfig {
                    name: "immutable_token".to_string(),
                    item_file: "items/immutable.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((2, 1)),
                },
                ItemConfig {
                    name: "mutable_token".to_string(),
                    item_file: "items/mutable.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((6, 2)),
                },
                ItemConfig {
                    name: "shadow_token".to_string(),
                    item_file: "items/shadow.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((1, 5)),
                },
                ItemConfig {
                    name: "scope_token".to_string(),
                    item_file: "items/scope.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((7, 5)),
                },
                ItemConfig {
                    name: "goal_item".to_string(),
                    item_file: "items/level_complete.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((8, 6)),
                }
            ]),
            tasks: load_level_tasks(4),
            income_per_square: Some(1),
            start_position: Some((0, 0)),
            max_turns: Some(120),
            fog_of_war: Some(false),
            message: Some("🔒 **LEVEL 4: Variable Bindings and Mutability** - Learn Rust's memory safety through immutable-by-default variables and explicit mutability!".to_string()),
            hint_message: Some("Variables are immutable by default (`let x = 5;`). Use `mut` for mutable variables (`let mut y = 10;`). Shadowing allows redefining variables with `let`.".to_string()),
            rust_docs_url: Some("https://doc.rust-lang.org/rust-by-example/variable_bindings.html".to_string()),
            starting_code: Some(r#"// Level 4: Variable Bindings and Mutability
// Learn Rust's memory safety through variable binding rules

// TODO: Task 5 - Add constants here (outside functions)
// const MAX_ENERGY: i32 = 1000;
// const ROBOT_NAME: &str = "Ferris";

fn main() {
    println!("Level 4: Variable Bindings and Mutability");

    // TODO: Task 1 - Create immutable variables
    // let robot_name = "Ferris";
    // let robot_id = 12345;
    // println!("Robot: {} (ID: {})", robot_name, robot_id);

    // TODO: Task 2 - Create mutable variables
    // let mut energy_level = 100;
    // let mut position = 0;
    // energy_level -= 10;
    // position += 5;
    // println!("Energy: {}, Position: {}", energy_level, position);

    // TODO: Task 3 - Demonstrate shadowing
    // let data = "123";
    // let data: i32 = data.parse().expect("Parse error");
    // let data = data * 2;
    // println!("Transformed data: {}", data);

    // TODO: Task 4 - Show scope with blocks
    // {
    //     let inner_value = "Inside block";
    //     println!("Inner: {}", inner_value);
    // }

    // TODO: Task 5 - Use constants
    // println!("Max energy: {}", MAX_ENERGY);
    // println!("Robot name: {}", ROBOT_NAME);

    println!("Complete all tasks to master variable bindings!");
}

// TODO: Task 4 - Helper function for scope demonstration
// fn calculate_something() -> i32 {
//     let local_value = 42;
//     local_value * 2
// }
"#.to_string()),
            next_level_hint: Some("Next: Type casting, conversions, and type transformations!".to_string()),
            achievement_message: Some("🎉 Fantastic! You've mastered Rust's variable binding system and memory safety!".to_string()),
            completion_condition: None,
            completion_flag: Some("goal".to_string()),
            completion_message: None,
        },

        // Level 5: Types and Casting
        YamlLevelConfig {
            name: "Level 5: Types and Casting".to_string(),
            grid_size: "10x8".to_string(),
            obstacles: Some(4),
            doors: None,
            enemies: None,
            items: Some(vec![
                ItemConfig {
                    name: "casting_tool".to_string(),
                    item_file: "items/casting.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((3, 2)),
                },
                ItemConfig {
                    name: "conversion_tool".to_string(),
                    item_file: "items/conversion.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((7, 3)),
                },
                ItemConfig {
                    name: "parse_tool".to_string(),
                    item_file: "items/parsing.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((2, 6)),
                },
                ItemConfig {
                    name: "inference_tool".to_string(),
                    item_file: "items/inference.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((8, 6)),
                },
                ItemConfig {
                    name: "goal_item".to_string(),
                    item_file: "items/level_complete.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((9, 7)),
                }
            ]),
            tasks: load_level_tasks(5),
            income_per_square: Some(1),
            start_position: Some((0, 0)),
            max_turns: Some(150),
            fog_of_war: Some(true),
            message: Some("🔄 **LEVEL 5: Types and Casting** - Master Rust's type conversion system - from explicit casting to safe conversions! Learn how Rust prevents data loss and maintains type safety during conversions.".to_string()),
            hint_message: Some("Type conversion tips: `as` keyword for explicit casting (can lose data), `.into()` for automatic conversions (From/Into traits), `.parse()` for string to number conversions. Rust prevents lossy conversions by default.".to_string()),
            rust_docs_url: Some("https://doc.rust-lang.org/rust-by-example/cast.html".to_string()),
            starting_code: Some(r#"// Level 5: Types and Casting
// Master Rust's type conversion system

fn main() {
    println!("Level 5: Types and Casting");

    // TODO: Task 1 - Explicit casting with 'as'
    // let large_number: i64 = 1000;
    // let small_number: i32 = large_number as i32;
    // println!("Cast: {} -> {}", large_number, small_number);

    // TODO: Task 2 - Safe conversions with From/Into
    // let small: i32 = 100;
    // let large: i64 = small.into();
    // println!("Safe conversion: {} -> {}", small, large);

    // TODO: Task 3 - String parsing
    // let number_str = "42";
    // match number_str.parse::<i32>() {
    //     Ok(num) => println!("Parsed: {}", num),
    //     Err(e) => println!("Error: {}", e),
    // }

    // TODO: Task 4 - Custom conversions
    // let position: Position = (10, 20).into();
    // println!("Position: x={}, y={}", position.x, position.y);

    // TODO: Task 5 - Type inference with conversions
    // let value = 500_i32;
    // let converted: i64 = value.into();
    // println!("Inferred conversion: {} -> {}", value, converted);

    println!("Complete all tasks to master type conversions!");
}

// TODO: Task 4 - Define custom types here
// struct Position {
//     x: i32,
//     y: i32,
// }
//
// impl From<(i32, i32)> for Position {
//     fn from(coord: (i32, i32)) -> Self {
//         Position { x: coord.0, y: coord.1 }
//     }
// }
"#.to_string()),
            next_level_hint: Some("Next: Flow control, conditionals, and pattern matching!".to_string()),
            achievement_message: Some("🎉 Superb! You've mastered Rust's type system and conversion mechanisms!".to_string()),
            completion_condition: None,
            completion_flag: Some("goal".to_string()),
            completion_message: None,
        },

        // Level 6: Ownership Basics
        YamlLevelConfig {
            name: "Level 6: Ownership Basics".to_string(),
            grid_size: "18x13".to_string(),
            obstacles: Some(6),
            doors: None,
            enemies: None,
            items: Some(vec![
                ItemConfig {
                    name: "ownership_rules_tip".to_string(),
                    item_file: "items/ownership_rules.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((6, 4)),
                },
                ItemConfig {
                    name: "move_semantics_tip".to_string(),
                    item_file: "items/move_semantics.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((12, 7)),
                },
                ItemConfig {
                    name: "stack_heap_tip".to_string(),
                    item_file: "items/stack_heap.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((16, 4)),
                },
                ItemConfig {
                    name: "goal_item".to_string(),
                    item_file: "items/level_complete.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((9, 11)),
                }
            ]),
            tasks: load_level_tasks(6),
            income_per_square: Some(1),
            start_position: Some((1, 1)),
            max_turns: Some(0),
            fog_of_war: Some(true),
            message: Some("🔒 **LEVEL 6: Ownership Basics** - Welcome to Rust's most unique feature: Ownership! Understanding ownership is crucial for memory safety without garbage collection. Learn the three rules and how values move between scopes.".to_string()),
            hint_message: Some("Ownership Rules: 1) Each value has one owner, 2) Only one owner at a time, 3) Value is dropped when owner goes out of scope. Assignment moves ownership by default.".to_string()),
            rust_docs_url: Some("https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html".to_string()),
            starting_code: Some(r#"// Level 6: Ownership Basics
// Learn Rust's unique ownership system for memory safety

fn main() {
    println!("Level 6: Ownership Basics");

    // TODO: Task 1 - Basic ownership rules
    // let robot_name = String::from("Ferris");
    // let new_owner = robot_name; // moves ownership
    // println!("New owner: {}", new_owner);

    // TODO: Task 2 - Move semantics
    // let s1 = String::from("hello");
    // let s2 = s1; // s1 is moved to s2
    // println!("s2: {}", s2);

    // TODO: Task 3 - References and borrowing
    // let s = String::from("hello");
    // let len = calculate_length(&s); // borrowing with &
    // println!("Length of '{}' is {}", s, len);

    // TODO: Task 4 - Ownership with functions
    // let name = String::from("Robot");
    // print_info(&name); // borrow instead of move
    // println!("Still can use: {}", name);

    // TODO: Task 5 - Common ownership patterns
    // let original = String::from("data");
    // let cloned = original.clone(); // explicit copy
    // println!("Both: {} and {}", original, cloned);

    println!("Complete all tasks to master ownership!");
}

// TODO: Task 3 - Add calculate_length function
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// TODO: Task 4 - Add print_info function
// fn print_info(name: &String) {
//     println!("Robot name: {}", name);
// }
"#.to_string()),
            next_level_hint: Some("Next: More advanced ownership concepts like lifetimes and borrowing checker!".to_string()),
            achievement_message: Some("🎉 Outstanding! You've mastered Rust's ownership system - the foundation of memory safety!".to_string()),
            completion_condition: None,
            completion_flag: Some("goal".to_string()),
            completion_message: None,
        },
    ];

    levels
}

pub fn get_embedded_level_specs() -> Vec<LevelSpec> {
    let mut levels = Vec::new();
    let mut rng = StdRng::seed_from_u64(0xC0FFEE);
    
    // Use the new embedded learning levels
    let learning_configs = get_embedded_learning_levels();
    for config in learning_configs {
        if let Ok(level_spec) = config.to_level_spec(&mut rng) {
            levels.push(level_spec);
        }
    }
    
    levels
}