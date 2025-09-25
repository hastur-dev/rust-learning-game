// Level 6: Flow Control and Conditionals - Automated Test Solutions

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_6_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 6: Flow Control and Conditionals",
        level_index: 5,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "If/Else Conditionals and Expressions",
                solution_code: r#"fn main() {
    println!("Level 6: Flow Control and Conditionals");

    let energy = 75;
    let position = (5, 3);

    // Basic if/else statements
    if energy > 50 {
        println!("Robot has sufficient energy: {}", energy);
    } else {
        println!("Robot needs recharging: {}", energy);
    }

    // If/else expressions (return values)
    let status = if energy > 80 {
        "Excellent"
    } else if energy > 50 {
        "Good"
    } else if energy > 20 {
        "Low"
    } else {
        "Critical"
    };

    println!("Energy status: {}", status);

    // Complex conditions with logical operators
    let x = position.0;
    let y = position.1;

    if x > 0 && y > 0 {
        println!("Robot is in positive quadrant: ({}, {})", x, y);
    } else if x == 0 || y == 0 {
        println!("Robot is on an axis: ({}, {})", x, y);
    } else {
        println!("Robot position: ({}, {})", x, y);
    }

    // If let for pattern matching
    let maybe_value: Option<i32> = Some(42);
    if let Some(value) = maybe_value {
        println!("Found value: {}", value);
    } else {
        println!("No value found");
    }
}"#,
                completion_indicators: vec![
                    "Level 6: Flow Control and Conditionals",
                    "Robot has sufficient energy: 75",
                    "Energy status: Good",
                    "Robot is in positive quadrant: (5, 3)",
                    "Found value: 42",
                ],
            },

            TaskTest {
                task_number: 2,
                task_name: "Loops - loop, while, and for",
                solution_code: r#"fn main() {
    println!("=== Infinite loop with break ===");
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Loop iteration: {}", counter);

        if counter >= 3 {
            println!("Breaking out of infinite loop");
            break;
        }
    }

    println!("=== While loop ===");
    let mut energy = 100;
    while energy > 0 {
        println!("Energy remaining: {}", energy);
        energy -= 25;

        if energy == 25 {
            println!("Low energy warning!");
            continue;
        }
    }

    println!("=== For loop with range ===");
    for i in 1..=5 {
        println!("For loop step: {}", i);
    }

    println!("=== For loop with collection ===");
    let positions = vec![(0, 0), (1, 2), (3, 4), (5, 6)];
    for (index, (x, y)) in positions.iter().enumerate() {
        println!("Position {}: ({}, {})", index, x, y);
    }
}"#,
                completion_indicators: vec![
                    "=== Infinite loop with break ===",
                    "Loop iteration: 1",
                    "Loop iteration: 2",
                    "Loop iteration: 3",
                    "Breaking out of infinite loop",
                    "=== While loop ===",
                    "Energy remaining: 100",
                    "Low energy warning!",
                    "=== For loop with range ===",
                    "For loop step: 1",
                    "For loop step: 5",
                    "=== For loop with collection ===",
                    "Position 0: (0, 0)",
                ],
            },

            TaskTest {
                task_number: 3,
                task_name: "Loop Control - break, continue, and labels",
                solution_code: r#"fn main() {
    println!("=== Finding prime numbers with continue ===");
    for num in 2..20 {
        let mut is_prime = true;

        for i in 2..num {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }

        if !is_prime {
            continue;
        }

        println!("{} is prime", num);
    }

    println!("=== Grid search with labeled breaks ===");
    let target = (2, 3);
    let mut found = false;

    'search: for row in 0..5 {
        for col in 0..5 {
            println!("Checking ({}, {})", row, col);

            if (row, col) == target {
                println!("Found target at ({}, {})!", row, col);
                found = true;
                break 'search;
            }

            if row == col {
                println!("Skipping diagonal position ({}, {})", row, col);
                continue;
            }
        }
    }

    if !found {
        println!("Target not found");
    }
}"#,
                completion_indicators: vec![
                    "=== Finding prime numbers with continue ===",
                    "2 is prime",
                    "3 is prime",
                    "5 is prime",
                    "7 is prime",
                    "11 is prime",
                    "13 is prime",
                    "17 is prime",
                    "19 is prime",
                    "=== Grid search with labeled breaks ===",
                    "Found target at (2, 3)!",
                ],
            },

            TaskTest {
                task_number: 4,
                task_name: "Match Expressions and Pattern Matching",
                solution_code: r#"fn main() {
    println!("=== Basic match with integers ===");
    let robot_mode = 2;

    let mode_name = match robot_mode {
        1 => "Exploration",
        2 => "Collection",
        3 => "Return Home",
        _ => "Unknown Mode",
    };

    println!("Robot mode {}: {}", robot_mode, mode_name);

    println!("=== Match with ranges ===");
    let energy_level = 45;

    match energy_level {
        81..=100 => println!("Energy: Excellent ({}%)", energy_level),
        61..=80 => println!("Energy: Good ({}%)", energy_level),
        41..=60 => println!("Energy: Moderate ({}%)", energy_level),
        21..=40 => println!("Energy: Low ({}%)", energy_level),
        1..=20 => println!("Energy: Critical ({}%)", energy_level),
        0 => println!("Energy: Depleted"),
        _ => println!("Energy: Invalid reading ({})", energy_level),
    }

    println!("=== Match with tuples ===");
    let position = (3, 4);

    match position {
        (0, 0) => println!("At origin"),
        (0, y) => println!("On Y-axis at y={}", y),
        (x, 0) => println!("On X-axis at x={}", x),
        (x, y) if x == y => println!("On diagonal at ({}, {})", x, y),
        (x, y) if x > y => println!("Above diagonal at ({}, {})", x, y),
        (x, y) => println!("Below diagonal at ({}, {})", x, y),
    }
}"#,
                completion_indicators: vec![
                    "=== Basic match with integers ===",
                    "Robot mode 2: Collection",
                    "=== Match with ranges ===",
                    "Energy: Moderate (45%)",
                    "=== Match with tuples ===",
                    "Below diagonal at (3, 4)",
                ],
            },

            TaskTest {
                task_number: 5,
                task_name: "Advanced Flow Control Patterns",
                solution_code: r#"fn main() {
    println!("=== Robot pathfinding simulation ===");

    let grid_size = 5;
    let obstacles = vec![(1, 1), (2, 3), (3, 1)];
    let mut robot_pos = (0, 0);
    let target = (4, 4);
    let mut steps = 0;
    let max_steps = 20;

    'pathfinding: loop {
        steps += 1;

        if steps > max_steps {
            println!("Pathfinding failed: too many steps");
            break 'pathfinding;
        }

        println!("Step {}: Robot at ({}, {})", steps, robot_pos.0, robot_pos.1);

        if robot_pos == target {
            println!("Target reached in {} steps!", steps);
            break 'pathfinding;
        }

        // Simple pathfinding - move towards target
        let mut next_moves = Vec::new();

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_pos = (robot_pos.0 + dx, robot_pos.1 + dy);

            if new_pos.0 >= 0 && new_pos.0 < grid_size &&
               new_pos.1 >= 0 && new_pos.1 < grid_size {

                let is_obstacle = obstacles.iter().any(|&obs| obs == new_pos);

                if !is_obstacle {
                    next_moves.push(new_pos);
                }
            }
        }

        if next_moves.is_empty() {
            println!("No valid moves available!");
            break 'pathfinding;
        }

        // Move towards target (simple strategy)
        robot_pos = next_moves[0];

        match robot_pos {
            pos if pos == target => println!("Will reach target next!"),
            _ => println!("Moving towards target..."),
        }
    }
}"#,
                completion_indicators: vec![
                    "=== Robot pathfinding simulation ===",
                    "Step 1: Robot at (0, 0)",
                    "Moving towards target...",
                    "Target reached in",
                ],
            },
        ],
    }
}