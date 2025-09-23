// Level 10 Task 1 Test: Result Basics and Error Propagation
// Tests if the user code uses Result<T, E> for error handling

#[cfg(test)]
mod level10_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_defines_robot_error_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_robot_error = analyzer.code.contains("enum RobotError") ||
                             analyzer.code.contains("RobotError {");
        assert!(
            has_robot_error,
            "❌ Your code should define a RobotError enum"
        );
    }

    #[test]
    fn test_error_enum_has_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_energy_error = analyzer.code.contains("InsufficientEnergy");
        let has_path_error = analyzer.code.contains("PathBlocked");
        let has_command_error = analyzer.code.contains("InvalidCommand");
        let has_system_error = analyzer.code.contains("SystemFailure");

        assert!(
            has_energy_error && has_path_error && has_command_error && has_system_error,
            "❌ Your RobotError enum should have InsufficientEnergy, PathBlocked, InvalidCommand, and SystemFailure variants"
        );
    }

    #[test]
    fn test_defines_robot_result_type() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_result_type = analyzer.code.contains("type RobotResult") ||
                             analyzer.code.contains("Result<");
        assert!(
            has_result_type,
            "❌ Your code should define a RobotResult type alias"
        );
    }

    #[test]
    fn test_move_robot_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_move_function = analyzer.code.contains("fn move_robot");
        assert!(
            has_move_function,
            "❌ Your code should define a move_robot function"
        );
    }

    #[test]
    fn test_move_robot_returns_result() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let returns_result = analyzer.code.contains("-> RobotResult") ||
                           analyzer.code.contains("-> Result<");
        assert!(
            returns_result,
            "❌ Your move_robot function should return a Result type"
        );
    }

    #[test]
    fn test_uses_ok_and_err() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_ok = analyzer.code.contains("Ok(");
        let uses_err = analyzer.code.contains("Err(");
        assert!(
            uses_ok && uses_err,
            "❌ Your code should use Ok() and Err() for Result values"
        );
    }

    #[test]
    fn test_energy_check() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let checks_energy = analyzer.code.contains("energy <") ||
                          analyzer.code.contains("energy >");
        assert!(
            checks_energy,
            "❌ Your code should check energy levels before operations"
        );
    }

    #[test]
    fn test_boundary_checking() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let checks_bounds = analyzer.code.contains("< 0") ||
                          analyzer.code.contains(">= 16") ||
                          analyzer.code.contains(">= 12");
        assert!(
            checks_bounds,
            "❌ Your code should check level boundaries"
        );
    }

    #[test]
    fn test_pattern_matching() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_match = analyzer.code.contains("match ") &&
                        analyzer.code.contains("Ok(") &&
                        analyzer.code.contains("Err(");
        assert!(
            uses_match,
            "❌ Your code should use pattern matching for Result handling"
        );
    }

    #[test]
    fn test_scan_area_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_scan_function = analyzer.code.contains("fn scan_area");
        assert!(
            has_scan_function,
            "❌ Your code should define a scan_area function"
        );
    }

    #[test]
    fn test_direction_validation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let validates_direction = analyzer.code.contains("north") ||
                                analyzer.code.contains("south") ||
                                analyzer.code.contains("east") ||
                                analyzer.code.contains("west");
        assert!(
            validates_direction,
            "❌ Your code should validate movement directions"
        );
    }

    #[test]
    fn test_error_with_data() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_error_data = analyzer.code.contains("required:") &&
                           analyzer.code.contains("available:");
        assert!(
            has_error_data,
            "❌ Your error variants should include data fields"
        );
    }

    #[test]
    fn test_unwrap_or_usage() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".unwrap_or("),
            "❌ Your code should use .unwrap_or() for default values"
        );
    }

    #[test]
    fn test_map_err_usage() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".map_err("),
            "❌ Your code should use .map_err() for error transformation"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_error_output = result.stdout.contains("Move failed") ||
                              result.stdout.contains("Invalid command") ||
                              result.stdout.contains("Low energy") ||
                              result.stdout.contains("Path blocked") ||
                              result.stdout.contains("Error");

        assert!(
            has_error_output,
            "❌ Your program should output information about error handling operations"
        );
    }
}

// Reference implementation
fn main() {
    #[derive(Debug)]
    enum RobotError {
        InsufficientEnergy { required: u32, available: u32 },
        PathBlocked { position: (i32, i32), obstacle: String },
        InvalidCommand { command: String },
        SystemFailure { component: String },
    }

    type RobotResult<T> = Result<T, RobotError>;

    fn move_robot(current_pos: (i32, i32), direction: &str, energy: u32) -> RobotResult<(i32, i32)> {
        // Check energy requirements
        if energy < 10 {
            return Err(RobotError::InsufficientEnergy {
                required: 10,
                available: energy,
            });
        }

        // Calculate new position
        let new_pos = match direction {
            "north" => (current_pos.0, current_pos.1 - 1),
            "south" => (current_pos.0, current_pos.1 + 1),
            "east" => (current_pos.0 + 1, current_pos.1),
            "west" => (current_pos.0 - 1, current_pos.1),
            _ => return Err(RobotError::InvalidCommand {
                command: direction.to_string(),
            }),
        };

        // Check for obstacles at new position
        let obstacles = vec![(6, 3), (12, 5), (4, 8)]; // Enemy positions
        if obstacles.contains(&new_pos) {
            return Err(RobotError::PathBlocked {
                position: new_pos,
                obstacle: "Enemy".to_string(),
            });
        }

        // Check bounds
        if new_pos.0 < 0 || new_pos.0 >= 16 || new_pos.1 < 0 || new_pos.1 >= 12 {
            return Err(RobotError::PathBlocked {
                position: new_pos,
                obstacle: "Boundary".to_string(),
            });
        }

        Ok(new_pos)
    }

    fn scan_area(pos: (i32, i32), energy: u32) -> RobotResult<Vec<String>> {
        if energy < 5 {
            return Err(RobotError::InsufficientEnergy {
                required: 5,
                available: energy,
            });
        }

        let mut scan_results = Vec::new();

        // Simulate scanning around position
        let scan_range = 2;
        for dx in -scan_range..=scan_range {
            for dy in -scan_range..=scan_range {
                let scan_pos = (pos.0 + dx, pos.1 + dy);

                // Check item positions
                let items = vec![(4, 1), (14, 3), (1, 10), (8, 11)];
                if items.contains(&scan_pos) {
                    scan_results.push(format!("Item at {:?}", scan_pos));
                }

                // Check enemy positions
                let enemies = vec![(6, 3), (12, 5), (4, 8), (14, 9), (2, 6), (10, 2)];
                if enemies.contains(&scan_pos) {
                    scan_results.push(format!("Enemy at {:?}", scan_pos));
                }

                // Check door positions
                let doors = vec![(5, 4), (9, 6), (3, 9), (13, 8), (7, 2), (11, 10)];
                if doors.contains(&scan_pos) {
                    scan_results.push(format!("Door at {:?}", scan_pos));
                }
            }
        }

        Ok(scan_results)
    }

    println!("=== Robot Error Handling System ===");

    let mut robot_pos = (0, 0);
    let mut robot_energy = 100;

    println!("Initial position: {:?}, Energy: {}", robot_pos, robot_energy);

    // Test successful operations
    match move_robot(robot_pos, "east", robot_energy) {
        Ok(new_pos) => {
            robot_pos = new_pos;
            robot_energy -= 10;
            println!("✅ Moved to {:?}, Energy: {}", robot_pos, robot_energy);
        }
        Err(e) => println!("❌ Move failed: {:?}", e),
    }

    // Test error conditions
    println!("\n=== Testing Error Conditions ===");

    // Test invalid command
    match move_robot(robot_pos, "invalid", robot_energy) {
        Ok(new_pos) => println!("Moved to {:?}", new_pos),
        Err(RobotError::InvalidCommand { command }) => {
            println!("❌ Invalid command: {}", command);
        }
        Err(e) => println!("❌ Other error: {:?}", e),
    }

    // Test low energy
    robot_energy = 5;
    match move_robot(robot_pos, "north", robot_energy) {
        Ok(_) => println!("Move successful"),
        Err(RobotError::InsufficientEnergy { required, available }) => {
            println!("❌ Low energy: need {}, have {}", required, available);
        }
        Err(e) => println!("❌ Other error: {:?}", e),
    }

    // Test blocked path
    robot_energy = 50;
    match move_robot((5, 3), "east", robot_energy) {
        Ok(_) => println!("Move successful"),
        Err(RobotError::PathBlocked { position, obstacle }) => {
            println!("❌ Path blocked at {:?} by {}", position, obstacle);
        }
        Err(e) => println!("❌ Other error: {:?}", e),
    }

    // Test scanning
    println!("\n=== Scanning Operations ===");
    match scan_area(robot_pos, robot_energy) {
        Ok(results) => {
            println!("Scan successful, found {} objects:", results.len());
            for result in results.iter().take(5) {
                println!("  - {}", result);
            }
        }
        Err(e) => println!("❌ Scan failed: {:?}", e),
    }

    // Test scan with low energy
    match scan_area(robot_pos, 3) {
        Ok(results) => println!("Scan found {} objects", results.len()),
        Err(RobotError::InsufficientEnergy { required, available }) => {
            println!("❌ Cannot scan: need {}, have {}", required, available);
        }
        Err(e) => println!("❌ Scan error: {:?}", e),
    }

    // Error handling patterns
    println!("\n=== Error Handling Patterns ===");

    // Using unwrap_or for defaults
    let safe_move = move_robot(robot_pos, "south", robot_energy)
        .unwrap_or(robot_pos);
    println!("Safe move result: {:?}", safe_move);

    // Using map_err for error transformation
    let move_result = move_robot(robot_pos, "west", 5)
        .map_err(|e| format!("Movement failed: {:?}", e));

    match move_result {
        Ok(pos) => println!("Move successful: {:?}", pos),
        Err(msg) => println!("Move failed: {}", msg),
    }
}