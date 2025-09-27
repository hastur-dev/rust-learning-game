// Level 2: Level 2: Functions and Loops - Automated Test Solutions
// Updated to match actual learning tests

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_2_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 2: Functions and Loops",
        level_index: 1,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Function with Print Statement",
                solution_code: r#"fn scan_level() {
    println!("Beginning level scan...");
}

fn main() {
    scan_level();
}"#,
                completion_indicators: vec![
                    "Beginning level scan..."
                ],
            },
            TaskTest {
                task_number: 2,
                task_name: "Nested Loops",
                solution_code: r#"fn scan_level() {
    println!("Beginning level scan...");

    for y in 0..6 {
        for x in 0..6 {
            println!("Scanning position ({}, {})", x, y);
        }
    }
}

fn main() {
    scan_level();
}"#,
                completion_indicators: vec![
                    "Beginning level scan...", "Scanning position (0, 0)", "Scanning position (5, 5)"
                ],
            },
            TaskTest {
                task_number: 3,
                task_name: "Struct with Vec",
                solution_code: r#"struct GridInfo {
    x: i32,
    y: i32,
    content: String,
}

fn scan_level() {
    println!("Beginning level scan...");

    let mut found_items = Vec::new();

    for y in 0..6 {
        for x in 0..6 {
            let grid_info = GridInfo {
                x: x,
                y: y,
                content: format!("tile_{}_{}", x, y),
            };

            if grid_info.content.contains("3") {
                found_items.push(grid_info);
            }
        }
    }

    println!("Found {} locations", found_items.len());
}

fn main() {
    scan_level();
}"#,
                completion_indicators: vec![
                    "Beginning level scan...", "Found", "locations"
                ],
            },
            TaskTest {
                task_number: 4,
                task_name: "Conditional Logic",
                solution_code: r#"struct GridInfo {
    x: i32,
    y: i32,
    content: String,
}

fn grab_if_item(scan_result: &str) -> bool {
    if scan_result != "empty" && scan_result != "wall" {
        println!("Grabbing: {}", scan_result);
        return true;
    }
    false
}

fn scan_level() {
    println!("Beginning level scan...");

    let mut found_items = Vec::new();
    let mut item_count = 0;

    for y in 0..6 {
        for x in 0..6 {
            let grid_info = GridInfo {
                x: x,
                y: y,
                content: format!("tile_{}_{}", x, y),
            };

            if grab_if_item(&grid_info.content) {
                item_count += 1;
            }

            if grid_info.content.contains("3") {
                found_items.push(grid_info);
            }
        }
    }

    println!("Found {} locations", found_items.len());
    println!("Grabbed {} items", item_count);
}

fn main() {
    scan_level();
}"#,
                completion_indicators: vec![
                    "Beginning level scan...", "Grabbing:", "Found", "locations", "Grabbed", "items"
                ],
            }
        ],
    }
}