// Level 6 Task 5 Test: Advanced Flow Control Patterns
// Tests if the user code combines multiple control flow constructs

#[cfg(test)]
mod level6_task5_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_multiple_loop_types() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_for = analyzer.code.contains("for ");
        let has_loop = analyzer.code.contains("loop {") || analyzer.code.contains("loop{");
        let has_while = analyzer.code.contains("while ");

        assert!(
            has_for && (has_loop || has_while),
            "❌ Your code should use multiple types of loops (for, loop, or while)"
        );
    }

    #[test]
    fn test_has_complex_conditionals() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let if_count = analyzer.code.matches("if ").count();
        assert!(
            if_count >= 3,
            "❌ Your code should have multiple conditional statements for complex logic"
        );
    }

    #[test]
    fn test_has_match_expression() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("match "),
            "❌ Your code should include match expressions in the advanced flow control"
        );
    }

    #[test]
    fn test_has_labeled_loops() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_labels = analyzer.code.contains("'") &&
                        (analyzer.code.contains(": for") || analyzer.code.contains(": loop"));
        assert!(
            has_labels,
            "❌ Your code should use labeled loops for complex control flow"
        );
    }

    #[test]
    fn test_has_break_and_continue() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_break = analyzer.code.contains("break");
        let has_continue = analyzer.code.contains("continue");
        assert!(
            has_break && has_continue,
            "❌ Your code should use both break and continue for flow control"
        );
    }

    #[test]
    fn test_has_collection_operations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_collections = analyzer.code.contains("vec!") ||
                             analyzer.code.contains("Vec::") ||
                             analyzer.code.contains(".iter()") ||
                             analyzer.code.contains(".enumerate()");
        assert!(
            has_collections,
            "❌ Your code should work with collections (vectors, arrays, etc.)"
        );
    }

    #[test]
    fn test_has_tuple_destructuring() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_tuples = analyzer.code.contains("(") && analyzer.code.contains(",") && analyzer.code.contains(")");
        assert!(
            has_tuples,
            "❌ Your code should use tuples and destructuring for complex data"
        );
    }

    #[test]
    fn test_simulates_pathfinding_or_algorithm() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_algorithm = analyzer.code.contains("target") ||
                           analyzer.code.contains("path") ||
                           analyzer.code.contains("search") ||
                           analyzer.code.contains("grid") ||
                           analyzer.code.contains("obstacle") ||
                           analyzer.code.contains("resource") ||
                           analyzer.code.contains("simulation");
        assert!(
            has_algorithm,
            "❌ Your code should simulate a complex algorithm like pathfinding or resource management"
        );
    }

    #[test]
    fn test_has_distance_or_calculation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_math = analyzer.code.contains(".abs()") ||
                      analyzer.code.contains("distance") ||
                      analyzer.code.contains("+ ") ||
                      analyzer.code.contains("- ") ||
                      analyzer.code.contains(".min_by") ||
                      analyzer.code.contains(".sum()");
        assert!(
            has_math,
            "❌ Your code should include mathematical calculations or distance computations"
        );
    }

    #[test]
    fn test_has_option_or_result_handling() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_option_result = analyzer.code.contains("Some(") ||
                               analyzer.code.contains("None") ||
                               analyzer.code.contains("if let") ||
                               analyzer.code.contains("unwrap") ||
                               analyzer.code.contains("expect");
        assert!(
            has_option_result,
            "❌ Your code should handle Option or Result types in the complex flow"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output evidence of complex simulation
        let has_complex_output = result.stdout.contains("step") ||
                               result.stdout.contains("target") ||
                               result.stdout.contains("path") ||
                               result.stdout.contains("robot") ||
                               result.stdout.contains("resource") ||
                               result.stdout.contains("cycle") ||
                               result.stdout.contains("simulation");

        assert!(
            has_complex_output,
            "❌ Your program should output results from complex flow control simulation"
        );
    }

    #[test]
    fn test_demonstrates_real_world_pattern() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let complex_pattern_count = analyzer.code.matches("for ").count() +
                                   analyzer.code.matches("if ").count() +
                                   analyzer.code.matches("match ").count() +
                                   analyzer.code.matches("while ").count();
        assert!(
            complex_pattern_count >= 8,
            "❌ Your code should combine many control flow constructs (8+ for/if/match/while statements)"
        );
    }
}

// Reference implementation for comparison
fn main() {
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

        // Check if we reached the target
        if robot_pos == target {
            println!("Target reached in {} steps!", steps);
            break 'pathfinding;
        }

        // Determine next move
        let mut next_moves = Vec::new();

        // Try all four directions
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_pos = (robot_pos.0 + dx, robot_pos.1 + dy);

            // Check bounds
            if new_pos.0 >= 0 && new_pos.0 < grid_size &&
               new_pos.1 >= 0 && new_pos.1 < grid_size {

                // Check for obstacles
                let is_obstacle = obstacles.iter().any(|&obs| obs == new_pos);

                if !is_obstacle {
                    next_moves.push(new_pos);
                }
            }
        }

        // Choose best move (closest to target)
        if next_moves.is_empty() {
            println!("No valid moves available!");
            break 'pathfinding;
        }

        let best_move = next_moves.iter().min_by_key(|&&(x, y)| {
            let distance = (target.0 - x).abs() + (target.1 - y).abs();
            distance
        });

        if let Some(&new_pos) = best_move {
            robot_pos = new_pos;

            match new_pos {
                pos if pos == target => {
                    println!("Will reach target on next iteration!");
                }
                (x, y) if x == target.0 || y == target.1 => {
                    println!("Aligned with target!");
                }
                _ => {
                    let remaining_distance = (target.0 - new_pos.0).abs() +
                                            (target.1 - new_pos.1).abs();
                    println!("Distance to target: {}", remaining_distance);
                }
            }
        }

        // Add some delay simulation
        if steps % 3 == 0 {
            println!("Recalculating path...");
            continue 'pathfinding;
        }
    }

    println!("\n=== Resource management simulation ===");
    let mut resources = vec![("Energy", 100), ("Fuel", 80), ("Repair", 60)];

    for cycle in 1..=5 {
        println!("--- Cycle {} ---", cycle);

        for (resource_name, amount) in resources.iter_mut() {
            match resource_name.as_ref() {
                "Energy" => {
                    *amount -= 15;
                    if *amount < 20 {
                        println!("⚠️ {} critically low: {}", resource_name, amount);
                    }
                }
                "Fuel" => {
                    *amount -= 10;
                    if *amount <= 0 {
                        println!("🚨 {} depleted!", resource_name);
                        *amount = 0;
                    }
                }
                "Repair" => {
                    if cycle % 2 == 0 {
                        *amount += 20;
                        println!("🔧 {} replenished: {}", resource_name, amount);
                    }
                }
                _ => println!("Unknown resource: {}", resource_name),
            }
        }

        // Check if all resources are too low
        let total_resources: i32 = resources.iter().map(|(_, amount)| *amount).sum();
        if total_resources < 50 {
            println!("💀 Critical resource shortage! Emergency stop!");
            break;
        }
    }

    println!("Final resources: {:?}", resources);
}