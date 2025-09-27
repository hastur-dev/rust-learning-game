// Level 6: Level 6: Control Flow - Automated Test Solutions
// Updated to match actual learning tests

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_6_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 6: Control Flow",
        level_index: 5,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Conditionals",
                solution_code: r#"fn main() {
    let energy = 75;
    let position = (5, 3);

    // Basic if/else
    if energy > 50 {
        println!("Energy level: High");
    } else {
        println!("Energy level: Low");
    }

    // Multiple conditions
    if energy >= 80 {
        println!("Status: Excellent");
    } else if energy >= 50 {
        println!("Status: Good");
    } else if energy >= 20 {
        println!("Status: Low");
    } else {
        println!("Status: Critical");
    }

    // Complex conditions
    if position.0 > 0 && position.1 > 0 && energy > 25 {
        println!("Can move diagonally");
    }

    // If as expression
    let status = if energy > 50 { "operational" } else { "needs charging" };
    println!("Robot is {}", status);
}"#,
                completion_indicators: vec![
                    "Energy level: High", "Status: Good", "Can move diagonally", "Robot is operational"
                ],
            },
            TaskTest {
                task_number: 2,
                task_name: "Loops",
                solution_code: r#"fn main() {
    println!("=== Infinite loop with break ===");
    let mut count = 0;
    loop {
        println!("Loop iteration: {}", count);
        count += 1;
        if count >= 3 {
            break;
        }
    }

    println!("=== While loop ===");
    let mut energy = 100;
    while energy > 50 {
        println!("Energy: {}", energy);
        energy -= 20;
    }

    println!("=== For loop with range ===");
    for i in 0..5 {
        println!("Position: {}", i);
    }

    println!("=== Labeled break ===");
    'outer: for x in 0..3 {
        for y in 0..3 {
            if x == 1 && y == 1 {
                println!("Breaking at ({}, {})", x, y);
                break 'outer;
            }
        }
    }
}"#,
                completion_indicators: vec![
                    "Loop iteration:", "Energy:", "Position:", "Breaking at (1, 1)"
                ],
            },
            TaskTest {
                task_number: 3,
                task_name: "Loop Control",
                solution_code: r#"fn main() {
    println!("=== Finding prime numbers with continue ===");
    for num in 2..20 {
        let mut is_prime = true;

        for divisor in 2..num {
            if num % divisor == 0 {
                is_prime = false;
                break;
            }
        }

        if !is_prime {
            continue;
        }

        println!("Found prime: {}", num);
    }

    println!("=== Loop with value return ===");
    let result = loop {
        let value = 42;
        if value > 40 {
            break value * 2;
        }
    };
    println!("Loop returned: {}", result);
}"#,
                completion_indicators: vec![
                    "Found prime:", "Loop returned: 84"
                ],
            },
            TaskTest {
                task_number: 4,
                task_name: "Match Expressions",
                solution_code: r#"fn main() {
    println!("=== Basic match with integers ===");
    let number = 42;
    match number {
        0 => println!("Zero"),
        1..=10 => println!("Small"),
        11..=50 => println!("Medium"),
        _ => println!("Large"),
    }

    println!("=== Match with tuples ===");
    let position = (2, 3);
    match position {
        (0, 0) => println!("Origin"),
        (0, _) => println!("On Y axis"),
        (_, 0) => println!("On X axis"),
        (x, y) if x == y => println!("On diagonal"),
        (x, y) => println!("At ({}, {})", x, y),
    }

    println!("=== Match with Option ===");
    let item: Option<&str> = Some("key");
    match item {
        Some("key") => println!("Found the key!"),
        Some(other) => println!("Found: {}", other),
        None => println!("Nothing found"),
    }
}"#,
                completion_indicators: vec![
                    "Medium", "At (2, 3)", "Found the key!"
                ],
            },
            TaskTest {
                task_number: 5,
                task_name: "Advanced Flow",
                solution_code: r#"fn main() {
    println!("=== Robot pathfinding simulation ===");

    let mut robot_pos = (0, 0);
    let target = (3, 3);
    let mut energy = 100;
    let mut path = Vec::new();

    'pathfinding: loop {
        // Check energy
        if energy <= 0 {
            println!("Out of energy!");
            break 'pathfinding;
        }

        // Determine next move
        let next_move = match (robot_pos, target) {
            (pos, tgt) if pos == tgt => {
                println!("Target reached!");
                break 'pathfinding;
            },
            ((x, y), (tx, ty)) if x < tx => (x + 1, y),
            ((x, y), (tx, ty)) if y < ty => (x, y + 1),
            _ => robot_pos,
        };

        // Move robot
        robot_pos = next_move;
        energy -= 10;
        path.push(robot_pos);

        // Status update
        for (i, pos) in path.iter().enumerate() {
            if i == path.len() - 1 {
                println!("Current position: {:?}, Energy: {}", pos, energy);
            }
        }

        // Check obstacles
        while energy > 50 && robot_pos.0 < target.0 {
            robot_pos.0 += 1;
            energy -= 5;
            path.push(robot_pos);
        }
    }

    println!("Path taken: {:?}", path);
    println!("Final energy: {}", energy);
}"#,
                completion_indicators: vec![
                    "Robot pathfinding simulation", "Current position:", "Target reached!", "Path taken:", "Final energy:"
                ],
            }
        ],
    }
}