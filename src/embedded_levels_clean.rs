use crate::level::{YamlLevelConfig, LevelSpec, ItemConfig};
use rand::{rngs::StdRng, SeedableRng};

// Embedded learning levels - these are core levels included in the executable
pub fn get_embedded_learning_levels() -> Vec<YamlLevelConfig> {
    vec![
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
    ]
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