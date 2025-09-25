// Level 2: Functions and Loops - Automated Test Solutions

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_2_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 2: Functions and Loops",
        level_index: 1,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Create Function with Print Statement",
                solution_code: r#"fn scan_level() {
    println!("Beginning level scan...");
    println!("Initializing robot systems...");
}

fn main() {
    println!("Level 2: Functions and Loops");
    scan_level();
    println!("Scan function completed successfully!");
}"#,
                completion_indicators: vec![
                    "Level 2: Functions and Loops",
                    "Beginning level scan...",
                    "Initializing robot systems...",
                    "Scan function completed successfully!",
                ],
            },

            TaskTest {
                task_number: 2,
                task_name: "Add Nested Loops for Grid Scanning with Robot Gamma",
                solution_code: r#"fn scan_level() {
    println!("Beginning level scan...");
    println!("Robot Gamma (◆) cloning demonstration active");

    for y in 0..6 {
        for x in 0..6 {
            let scan_result = format!("tile_{}_{}", x, y);
            println!("Scanned ({}, {}): {}", x, y, scan_result);

            // Robot Gamma demonstrates cloning at position (2,2)
            if x == 2 && y == 2 {
                println!("Robot Gamma detected at position (2,2) - cloning demo active");
                let original_data = String::from("robot_data");
                let cloned_data = original_data.clone();
                println!("Robot Gamma cloned data: {} -> {}", original_data, cloned_data);
            }
        }
    }

    println!("Grid scan complete!");
}

fn main() {
    println!("Level 2: Functions and Loops");
    scan_level();
    println!("All scanning operations completed!");
}"#,
                completion_indicators: vec![
                    "Level 2: Functions and Loops",
                    "Beginning level scan...",
                    "Scanned (0, 0): tile_0_0",
                    "Scanned (5, 5): tile_5_5",
                    "Grid scan complete!",
                    "All scanning operations completed!",
                ],
            },

            TaskTest {
                task_number: 3,
                task_name: "Create GridInfo Struct",
                solution_code: r#"struct GridInfo {
    x: i32,
    y: i32,
    content: String,
}

fn scan_level() {
    println!("Beginning level scan...");
    let mut item_locations = Vec::new();

    for y in 0..6 {
        for x in 0..6 {
            let scan_result = if x == 2 && y == 2 {
                "energy_cell"
            } else if x == 4 && y == 1 {
                "repair_kit"
            } else {
                "empty"
            };

            if scan_result != "empty" {
                item_locations.push((x, y, scan_result.to_string()));
                println!("Found item at ({}, {}): {}", x, y, scan_result);
            }
        }
    }

    println!("Scan complete! Found {} items", item_locations.len());

    for (x, y, item) in item_locations {
        println!("Item location: ({}, {}) contains {}", x, y, item);
    }
}

fn main() {
    println!("Level 2: Functions and Loops");
    scan_level();
    println!("Struct-based scanning completed!");
}"#,
                completion_indicators: vec![
                    "Level 2: Functions and Loops",
                    "Beginning level scan...",
                    "Found item at (2, 2): energy_cell",
                    "Found item at (4, 1): repair_kit",
                    "Scan complete! Found 2 items",
                    "Item location: (2, 2) contains energy_cell",
                    "Item location: (4, 1) contains repair_kit",
                    "Struct-based scanning completed!",
                ],
            },

            TaskTest {
                task_number: 4,
                task_name: "Create Item Collection Function",
                solution_code: r#"struct GridInfo {
    x: i32,
    y: i32,
    content: String,
}

fn grab_if_item(scan_result: &str, x: i32, y: i32) -> bool {
    if scan_result != "empty" && scan_result != "wall" && scan_result != "goal" {
        println!("Grabbed: {} at ({}, {})", scan_result, x, y);
        return true;
    }
    false
}

fn scan_level() {
    println!("Beginning level scan...");
    let mut items_collected = 0;

    for y in 0..6 {
        for x in 0..6 {
            let scan_result = if x == 2 && y == 2 {
                "energy_cell"
            } else if x == 4 && y == 1 {
                "repair_kit"
            } else if x == 1 && y == 4 {
                "upgrade_module"
            } else {
                "empty"
            };

            if grab_if_item(scan_result, x, y) {
                items_collected += 1;
            }
        }
    }

    println!("Scan complete! Collected {} items total", items_collected);
}

fn main() {
    println!("Level 2: Functions and Loops");
    scan_level();
    println!("Function-based item collection completed!");
}"#,
                completion_indicators: vec![
                    "Level 2: Functions and Loops",
                    "Beginning level scan...",
                    "Grabbed: energy_cell at (2, 2)",
                    "Grabbed: repair_kit at (4, 1)",
                    "Grabbed: upgrade_module at (1, 4)",
                    "Scan complete! Collected 3 items total",
                    "Function-based item collection completed!",
                ],
            },
        ],
    }
}