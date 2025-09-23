// Level 10 Task 2 Test: Custom Error Types and Error Traits
// Tests if the user code implements proper error traits

#[cfg(test)]
mod level10_task2_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_imports_error_traits() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let imports_fmt = analyzer.code.contains("use std::fmt");
        let imports_error = analyzer.code.contains("use std::error::Error");
        assert!(
            imports_fmt && imports_error,
            "❌ Your code should import std::fmt and std::error::Error"
        );
    }

    #[test]
    fn test_defines_robot_system_error() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_system_error = analyzer.code.contains("enum RobotSystemError") ||
                              analyzer.code.contains("RobotSystemError {");
        assert!(
            has_system_error,
            "❌ Your code should define a RobotSystemError enum"
        );
    }

    #[test]
    fn test_system_error_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_movement = analyzer.code.contains("Movement");
        let has_energy = analyzer.code.contains("Energy");
        let has_communication = analyzer.code.contains("Communication");
        let has_hardware = analyzer.code.contains("Hardware");
        let has_mission = analyzer.code.contains("Mission");

        assert!(
            has_movement && has_energy && has_communication && has_hardware && has_mission,
            "❌ Your RobotSystemError should have Movement, Energy, Communication, Hardware, and Mission variants"
        );
    }

    #[test]
    fn test_defines_movement_error() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_movement_error = analyzer.code.contains("enum MovementError") ||
                                analyzer.code.contains("MovementError {");
        assert!(
            has_movement_error,
            "❌ Your code should define a MovementError enum"
        );
    }

    #[test]
    fn test_implements_display_trait() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let implements_display = analyzer.code.contains("impl fmt::Display for RobotSystemError") ||
                               analyzer.code.contains("impl Display for RobotSystemError");
        assert!(
            implements_display,
            "❌ Your code should implement Display trait for RobotSystemError"
        );
    }

    #[test]
    fn test_implements_error_trait() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let implements_error = analyzer.code.contains("impl Error for RobotSystemError") ||
                             analyzer.code.contains("impl std::error::Error for RobotSystemError");
        assert!(
            implements_error,
            "❌ Your code should implement Error trait for RobotSystemError"
        );
    }

    #[test]
    fn test_display_fmt_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_fmt_method = analyzer.code.contains("fn fmt(&self, f: &mut fmt::Formatter)");
        assert!(
            has_fmt_method,
            "❌ Your Display implementation should have a fmt method"
        );
    }

    #[test]
    fn test_error_helper_functions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_helpers = analyzer.code.contains("movement_blocked") ||
                         analyzer.code.contains("insufficient_energy") ||
                         analyzer.code.contains("hardware_failure");
        assert!(
            has_helpers,
            "❌ Your code should implement error helper functions"
        );
    }

    #[test]
    fn test_advanced_robot_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_advanced_robot = analyzer.code.contains("struct AdvancedRobot") ||
                               analyzer.code.contains("AdvancedRobot {");
        assert!(
            has_advanced_robot,
            "❌ Your code should define an AdvancedRobot struct"
        );
    }

    #[test]
    fn test_robot_has_required_fields() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_position = analyzer.code.contains("position:");
        let has_energy = analyzer.code.contains("energy:");
        let has_health = analyzer.code.contains("health:");
        let has_systems = analyzer.code.contains("systems_online:");

        assert!(
            has_position && has_energy && has_health && has_systems,
            "❌ AdvancedRobot should have position, energy, health, and systems_online fields"
        );
    }

    #[test]
    fn test_robot_implements_methods() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_move_to = analyzer.code.contains("fn move_to(");
        let has_scan = analyzer.code.contains("fn scan_for_items(");
        let has_diagnostic = analyzer.code.contains("fn system_diagnostic(");

        assert!(
            has_move_to && has_scan && has_diagnostic,
            "❌ AdvancedRobot should implement move_to, scan_for_items, and system_diagnostic methods"
        );
    }

    #[test]
    fn test_error_description_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_description = analyzer.code.contains("fn description(&self)");
        assert!(
            has_description,
            "❌ Your Error implementation should have a description method"
        );
    }

    #[test]
    fn test_structured_error_data() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_structured_data = analyzer.code.contains("current:") &&
                                analyzer.code.contains("required:") &&
                                analyzer.code.contains("target:");
        assert!(
            has_structured_data,
            "❌ Your error types should include structured data fields"
        );
    }

    #[test]
    fn test_energy_cost_calculation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let calculates_cost = analyzer.code.contains("calculate_move_cost") ||
                            analyzer.code.contains("energy_cost");
        assert!(
            calculates_cost,
            "❌ Your code should calculate energy costs for operations"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_error_trait_output = result.stdout.contains("CRITICAL") ||
                                   result.stdout.contains("RECOVERABLE") ||
                                   result.stdout.contains("failure") ||
                                   result.stdout.contains("Error type") ||
                                   result.stdout.contains("description");

        assert!(
            has_error_trait_output,
            "❌ Your program should output information about custom error types and traits"
        );
    }
}

// Reference implementation
fn main() {
    use std::fmt;
    use std::error::Error;

    #[derive(Debug, Clone)]
    pub enum RobotSystemError {
        Movement {
            reason: MovementError,
            attempted_position: (i32, i32),
        },
        Energy {
            current: u32,
            required: u32,
            operation: String,
        },
        Communication {
            target: String,
            error_code: u32,
        },
        Hardware {
            component: String,
            diagnostic: String,
        },
        Mission {
            objective: String,
            failure_reason: String,
        },
    }

    #[derive(Debug, Clone)]
    pub enum MovementError {
        ObstacleDetected(String),
        BoundaryViolation,
        InvalidDirection,
        PathCalculationFailed,
    }

    // Implement Display trait for user-friendly error messages
    impl fmt::Display for RobotSystemError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                RobotSystemError::Movement { reason, attempted_position } => {
                    write!(f, "Movement error at {:?}: {}", attempted_position, reason)
                }
                RobotSystemError::Energy { current, required, operation } => {
                    write!(f, "Insufficient energy for {}: have {}, need {}",
                           operation, current, required)
                }
                RobotSystemError::Communication { target, error_code } => {
                    write!(f, "Communication failed with {}: error code {}",
                           target, error_code)
                }
                RobotSystemError::Hardware { component, diagnostic } => {
                    write!(f, "Hardware failure in {}: {}", component, diagnostic)
                }
                RobotSystemError::Mission { objective, failure_reason } => {
                    write!(f, "Mission '{}' failed: {}", objective, failure_reason)
                }
            }
        }
    }

    impl fmt::Display for MovementError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                MovementError::ObstacleDetected(obstacle) => {
                    write!(f, "obstacle detected: {}", obstacle)
                }
                MovementError::BoundaryViolation => {
                    write!(f, "attempted to move outside level bounds")
                }
                MovementError::InvalidDirection => {
                    write!(f, "invalid movement direction specified")
                }
                MovementError::PathCalculationFailed => {
                    write!(f, "unable to calculate valid path")
                }
            }
        }
    }

    // Implement Error trait
    impl Error for RobotSystemError {
        fn description(&self) -> &str {
            match self {
                RobotSystemError::Movement { .. } => "robot movement error",
                RobotSystemError::Energy { .. } => "robot energy error",
                RobotSystemError::Communication { .. } => "robot communication error",
                RobotSystemError::Hardware { .. } => "robot hardware error",
                RobotSystemError::Mission { .. } => "robot mission error",
            }
        }
    }

    impl Error for MovementError {}

    // Helper functions for creating specific errors
    impl RobotSystemError {
        pub fn movement_blocked(pos: (i32, i32), obstacle: &str) -> Self {
            RobotSystemError::Movement {
                reason: MovementError::ObstacleDetected(obstacle.to_string()),
                attempted_position: pos,
            }
        }

        pub fn insufficient_energy(current: u32, required: u32, operation: &str) -> Self {
            RobotSystemError::Energy {
                current,
                required,
                operation: operation.to_string(),
            }
        }

        pub fn hardware_failure(component: &str, diagnostic: &str) -> Self {
            RobotSystemError::Hardware {
                component: component.to_string(),
                diagnostic: diagnostic.to_string(),
            }
        }
    }

    type RobotResult<T> = Result<T, RobotSystemError>;

    struct AdvancedRobot {
        position: (i32, i32),
        energy: u32,
        health: u32,
        systems_online: bool,
    }

    impl AdvancedRobot {
        fn new() -> Self {
            AdvancedRobot {
                position: (0, 0),
                energy: 100,
                health: 100,
                systems_online: true,
            }
        }

        fn move_to(&mut self, target: (i32, i32)) -> RobotResult<()> {
            // Check system status
            if !self.systems_online {
                return Err(RobotSystemError::hardware_failure(
                    "Navigation System",
                    "Systems offline"
                ));
            }

            // Check energy
            let energy_cost = self.calculate_move_cost(target);
            if self.energy < energy_cost {
                return Err(RobotSystemError::insufficient_energy(
                    self.energy,
                    energy_cost,
                    "movement"
                ));
            }

            // Check bounds
            if target.0 < 0 || target.0 >= 16 || target.1 < 0 || target.1 >= 12 {
                return Err(RobotSystemError::Movement {
                    reason: MovementError::BoundaryViolation,
                    attempted_position: target,
                });
            }

            // Check for obstacles
            let obstacles = vec![
                ((6, 3), "Enemy Guard"),
                ((12, 5), "Enemy Chaser"),
                ((4, 8), "Enemy Spiral"),
                ((14, 9), "Enemy Patrol"),
            ];

            for (pos, obstacle_type) in obstacles {
                if pos == target {
                    return Err(RobotSystemError::movement_blocked(target, obstacle_type));
                }
            }

            // Successful move
            self.position = target;
            self.energy -= energy_cost;
            Ok(())
        }

        fn calculate_move_cost(&self, target: (i32, i32)) -> u32 {
            let distance = (self.position.0 - target.0).abs() + (self.position.1 - target.1).abs();
            (distance as u32) * 5
        }

        fn scan_for_items(&self) -> RobotResult<Vec<(String, (i32, i32))>> {
            if self.energy < 15 {
                return Err(RobotSystemError::insufficient_energy(
                    self.energy,
                    15,
                    "area scan"
                ));
            }

            if !self.systems_online {
                return Err(RobotSystemError::hardware_failure(
                    "Scanner Array",
                    "Scanner offline"
                ));
            }

            // Simulate scanning for items
            let items = vec![
                ("result_core".to_string(), (4, 1)),
                ("error_handler".to_string(), (14, 3)),
                ("recovery_system".to_string(), (1, 10)),
                ("fault_tolerance".to_string(), (8, 11)),
            ];

            // Filter items within scan range
            let scan_range = 5;
            let nearby_items: Vec<_> = items.into_iter()
                .filter(|(_, pos)| {
                    let distance = (self.position.0 - pos.0).abs() + (self.position.1 - pos.1).abs();
                    distance <= scan_range
                })
                .collect();

            Ok(nearby_items)
        }

        fn attempt_door_opening(&mut self, door_pos: (i32, i32)) -> RobotResult<()> {
            if self.energy < 20 {
                return Err(RobotSystemError::insufficient_energy(
                    self.energy,
                    20,
                    "door opening"
                ));
            }

            let distance = (self.position.0 - door_pos.0).abs() + (self.position.1 - door_pos.1).abs();
            if distance > 1 {
                return Err(RobotSystemError::Mission {
                    objective: "Open Door".to_string(),
                    failure_reason: "Too far from door".to_string(),
                });
            }

            // Simulate potential hardware failure
            if self.health < 30 {
                return Err(RobotSystemError::hardware_failure(
                    "Door Interface",
                    "Low system health affecting door controls"
                ));
            }

            self.energy -= 20;
            Ok(())
        }

        fn system_diagnostic(&self) -> RobotResult<String> {
            let mut diagnostics = Vec::new();

            if self.energy < 20 {
                diagnostics.push("CRITICAL: Low energy");
            }

            if self.health < 50 {
                diagnostics.push("WARNING: System health degraded");
            }

            if !self.systems_online {
                return Err(RobotSystemError::hardware_failure(
                    "Diagnostic System",
                    "Cannot run diagnostics - systems offline"
                ));
            }

            let status = if diagnostics.is_empty() {
                "All systems nominal".to_string()
            } else {
                diagnostics.join(", ")
            };

            Ok(status)
        }
    }

    println!("🤖 Advanced Robot Error System Demo");

    let mut robot = AdvancedRobot::new();

    // Demonstrate successful operations
    println!("\n=== Successful Operations ===");
    match robot.move_to((3, 1)) {
        Ok(()) => println!("✅ Moved to (3, 1)"),
        Err(e) => println!("❌ Move failed: {}", e),
    }

    match robot.scan_for_items() {
        Ok(items) => {
            println!("✅ Scan complete, found {} items:", items.len());
            for (item, pos) in items {
                println!("  - {} at {:?}", item, pos);
            }
        }
        Err(e) => println!("❌ Scan failed: {}", e),
    }

    // Demonstrate different error types
    println!("\n=== Error Demonstrations ===");

    // Energy error
    robot.energy = 5;
    match robot.move_to((10, 10)) {
        Ok(()) => println!("Move successful"),
        Err(e) => {
            println!("❌ {}", e);
            println!("   Error type: {}", e.description());
        }
    }

    // Movement error - boundary violation
    robot.energy = 100;
    match robot.move_to((20, 20)) {
        Ok(()) => println!("Move successful"),
        Err(e) => println!("❌ {}", e),
    }

    // Movement error - obstacle
    match robot.move_to((6, 3)) {
        Ok(()) => println!("Move successful"),
        Err(e) => println!("❌ {}", e),
    }

    // Hardware error
    robot.systems_online = false;
    match robot.scan_for_items() {
        Ok(_) => println!("Scan successful"),
        Err(e) => println!("❌ {}", e),
    }

    // System diagnostic
    robot.systems_online = true;
    robot.health = 25;
    match robot.system_diagnostic() {
        Ok(status) => println!("✅ Diagnostic: {}", status),
        Err(e) => println!("❌ Diagnostic failed: {}", e),
    }

    println!("\n=== Error Recovery Demonstration ===");

    // Attempt recovery
    robot.energy = 100;
    robot.health = 100;
    robot.systems_online = true;

    match robot.system_diagnostic() {
        Ok(status) => println!("✅ System recovered: {}", status),
        Err(e) => println!("❌ Recovery failed: {}", e),
    }
}