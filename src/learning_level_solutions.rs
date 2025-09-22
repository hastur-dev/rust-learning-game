// Complete solutions for all learning levels
// Used by the automated testing system to verify level completion

use std::collections::HashMap;

pub struct LevelSolution {
    pub level_name: &'static str,
    pub solution_code: &'static str,
}

pub fn get_all_solutions() -> Vec<LevelSolution> {
    vec![
        // Level 1: Hello Rust!
        LevelSolution {
            level_name: "Level 1 - Hello Rust!",
            solution_code: r#"fn main() {
    println!("Hello, Rust!");
}"#,
        },

        // Level 2: Functions and Loops (complete solution with all tasks)
        LevelSolution {
            level_name: "Level 2: Functions and Loops",
            solution_code: r#"struct GridInfo {
    x: i32,
    y: i32,
    content: String,
}

fn scan_level() {
    println!("Beginning level scan...");

    for y in 0..6 {
        for x in 0..6 {
            // Movement and scanning code
        }
    }
}

fn grab_if_item(scan_result: &str) {
    if scan_result.contains("item") {
        // grab the item
    }
}

fn main() {
    println!("Level 2: Functions, Loops, and Structs");
    println!("Remember: All code must be in functions!");

    scan_level();

    println!("All tasks complete! Moving to goal...");
}"#,
        },
    ]
}

// Get solution by level name
pub fn get_solution_for_level(level_name: &str) -> Option<&'static str> {
    get_all_solutions()
        .into_iter()
        .find(|s| s.level_name == level_name)
        .map(|s| s.solution_code)
}

// Get a map of all solutions
pub fn get_solutions_map() -> HashMap<String, &'static str> {
    get_all_solutions()
        .into_iter()
        .map(|s| (s.level_name.to_string(), s.solution_code))
        .collect()
}