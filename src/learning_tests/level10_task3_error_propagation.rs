// Level 10 Task 3 Test: Error Propagation with the ? Operator
// Tests if the user code uses the ? operator for clean error propagation

#[cfg(test)]
mod level10_task3_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_uses_question_mark_operator() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_question_mark = analyzer.code.matches("?").count() >= 5;
        assert!(
            uses_question_mark,
            "❌ Your code should use the ? operator for error propagation (at least 5 times)"
        );
    }

    #[test]
    fn test_defines_mission_controller() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_mission_controller = analyzer.code.contains("struct MissionController") ||
                                   analyzer.code.contains("MissionController {");
        assert!(
            has_mission_controller,
            "❌ Your code should define a MissionController struct"
        );
    }

    #[test]
    fn test_mission_controller_has_robots() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_robots_field = analyzer.code.contains("robots: HashMap") ||
                              analyzer.code.contains("robots:");
        assert!(
            has_robots_field,
            "❌ MissionController should have a robots field using HashMap"
        );
    }

    #[test]
    fn test_mission_controller_has_log() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_log_field = analyzer.code.contains("mission_log") ||
                           analyzer.code.contains("log:");
        assert!(
            has_log_field,
            "❌ MissionController should have a mission_log field"
        );
    }

    #[test]
    fn test_implements_add_robot_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_add_robot = analyzer.code.contains("fn add_robot(");
        assert!(
            has_add_robot,
            "❌ MissionController should implement add_robot method"
        );
    }

    #[test]
    fn test_implements_movement_sequence() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_movement_sequence = analyzer.code.contains("execute_movement_sequence") ||
                                  analyzer.code.contains("movement_sequence");
        assert!(
            has_movement_sequence,
            "❌ Your code should implement execute_movement_sequence method"
        );
    }

    #[test]
    fn test_implements_collect_all_items() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_collect_items = analyzer.code.contains("collect_all_items") ||
                               analyzer.code.contains("collect_items");
        assert!(
            has_collect_items,
            "❌ Your code should implement collect_all_items method"
        );
    }

    #[test]
    fn test_implements_navigate_through_doors() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_navigate_doors = analyzer.code.contains("navigate_through_doors") ||
                                analyzer.code.contains("door_navigation");
        assert!(
            has_navigate_doors,
            "❌ Your code should implement navigate_through_doors method"
        );
    }

    #[test]
    fn test_implements_complete_mission() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_complete_mission = analyzer.code.contains("fn complete_mission(");
        assert!(
            has_complete_mission,
            "❌ Your code should implement complete_mission method"
        );
    }

    #[test]
    fn test_uses_ok_or_else() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".ok_or_else("),
            "❌ Your code should use .ok_or_else() for Option to Result conversion"
        );
    }

    #[test]
    fn test_chains_operations_with_question_mark() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_chained_ops = analyzer.code.contains("move_to(") &&
                             analyzer.code.contains(")?;") &&
                             analyzer.code.contains("scan_for_items");
        assert!(
            has_chained_ops,
            "❌ Your code should chain operations using ? operator"
        );
    }

    #[test]
    fn test_parallel_mission_attempt() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_parallel = analyzer.code.contains("parallel_mission") ||
                          analyzer.code.contains("parallel");
        assert!(
            has_parallel,
            "❌ Your code should implement parallel mission attempt functionality"
        );
    }

    #[test]
    fn test_waypoint_processing() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let processes_waypoints = analyzer.code.contains("waypoints") &&
                                analyzer.code.contains("for ") &&
                                analyzer.code.contains("enumerate");
        assert!(
            processes_waypoints,
            "❌ Your code should process waypoints in sequence"
        );
    }

    #[test]
    fn test_diagnostic_checking() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let checks_diagnostic = analyzer.code.contains("system_diagnostic") &&
                               analyzer.code.contains("CRITICAL");
        assert!(
            checks_diagnostic,
            "❌ Your code should check system diagnostics and handle critical states"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_propagation_output = result.stdout.contains("Mission") ||
                                   result.stdout.contains("Assigning") ||
                                   result.stdout.contains("waypoint") ||
                                   result.stdout.contains("completed") ||
                                   result.stdout.contains("Alpha");

        assert!(
            has_propagation_output,
            "❌ Your program should output information about mission execution and error propagation"
        );
    }
}

// Reference implementation
fn main() {
    use std::collections::HashMap;

    // Reuse error types from previous task
    #[derive(Debug)]
    enum RobotSystemError {
        Movement {
            attempted_position: (i32, i32),
        },
        Energy {
            current: u32,
            required: u32,
            operation: String,
        },
        Mission {
            objective: String,
            failure_reason: String,
        },
    }

    type RobotResult<T> = Result<T, RobotSystemError>;

    struct AdvancedRobot {
        position: (i32, i32),
        energy: u32,
        systems_online: bool,
    }

    impl AdvancedRobot {
        fn new() -> Self {
            AdvancedRobot {
                position: (0, 0),
                energy: 100,
                systems_online: true,
            }
        }

        fn move_to(&mut self, target: (i32, i32)) -> RobotResult<()> {
            if self.energy < 10 {
                return Err(RobotSystemError::Energy {
                    current: self.energy,
                    required: 10,
                    operation: "movement".to_string(),
                });
            }
            self.position = target;
            self.energy -= 10;
            Ok(())
        }

        fn scan_for_items(&self) -> RobotResult<Vec<(String, (i32, i32))>> {
            if self.energy < 5 {
                return Err(RobotSystemError::Energy {
                    current: self.energy,
                    required: 5,
                    operation: "scan".to_string(),
                });
            }

            let items = vec![
                ("result_core".to_string(), (4, 1)),
                ("error_handler".to_string(), (14, 3)),
            ];
            Ok(items)
        }

        fn system_diagnostic(&self) -> RobotResult<String> {
            if self.energy < 20 {
                Ok("CRITICAL: Low energy".to_string())
            } else {
                Ok("All systems nominal".to_string())
            }
        }

        fn attempt_door_opening(&mut self, _door_pos: (i32, i32)) -> RobotResult<()> {
            if self.energy < 20 {
                return Err(RobotSystemError::Energy {
                    current: self.energy,
                    required: 20,
                    operation: "door opening".to_string(),
                });
            }
            self.energy -= 20;
            Ok(())
        }
    }

    struct MissionController {
        robots: HashMap<String, AdvancedRobot>,
        mission_log: Vec<String>,
    }

    impl MissionController {
        fn new() -> Self {
            MissionController {
                robots: HashMap::new(),
                mission_log: Vec::new(),
            }
        }

        fn add_robot(&mut self, id: String) -> RobotResult<()> {
            let robot = AdvancedRobot::new();

            // Check if robot ID already exists
            if self.robots.contains_key(&id) {
                return Err(RobotSystemError::Mission {
                    objective: "Add Robot".to_string(),
                    failure_reason: format!("Robot {} already exists", id),
                });
            }

            self.robots.insert(id.clone(), robot);
            self.mission_log.push(format!("Robot {} added to mission", id));
            Ok(())
        }

        // Using ? operator for clean error propagation
        fn execute_movement_sequence(&mut self, robot_id: &str, waypoints: Vec<(i32, i32)>) -> RobotResult<()> {
            let robot = self.robots.get_mut(robot_id).ok_or_else(|| {
                RobotSystemError::Mission {
                    objective: "Movement Sequence".to_string(),
                    failure_reason: format!("Robot {} not found", robot_id),
                }
            })?;

            self.mission_log.push(format!("Starting movement sequence for {}", robot_id));

            for (i, waypoint) in waypoints.iter().enumerate() {
                // The ? operator automatically returns the error if move_to fails
                robot.move_to(*waypoint)?;

                self.mission_log.push(format!("{} reached waypoint {}: {:?}",
                                             robot_id, i + 1, waypoint));

                // Check system status after each move
                let diagnostic = robot.system_diagnostic()?;
                if diagnostic.contains("CRITICAL") {
                    return Err(RobotSystemError::Mission {
                        objective: "Movement Sequence".to_string(),
                        failure_reason: format!("Critical system status: {}", diagnostic),
                    });
                }
            }

            Ok(())
        }

        fn collect_all_items(&mut self, robot_id: &str) -> RobotResult<Vec<String>> {
            let robot = self.robots.get_mut(robot_id).ok_or_else(|| {
                RobotSystemError::Mission {
                    objective: "Item Collection".to_string(),
                    failure_reason: format!("Robot {} not found", robot_id),
                }
            })?;

            let mut collected_items = Vec::new();

            // Item locations from level 10
            let item_locations = vec![
                ((4, 1), "result_core"),
                ((14, 3), "error_handler"),
                ((1, 10), "recovery_system"),
                ((8, 11), "fault_tolerance"),
            ];

            for ((x, y), item_name) in item_locations {
                // Move to item location - ? propagates any movement errors
                robot.move_to((x, y))?;

                // Scan for the item - ? propagates scan errors
                let nearby_items = robot.scan_for_items()?;

                // Check if target item was found
                let item_found = nearby_items.iter()
                    .any(|(name, _)| name == item_name);

                if item_found {
                    collected_items.push(item_name.to_string());
                    self.mission_log.push(format!("{} collected {}", robot_id, item_name));
                } else {
                    return Err(RobotSystemError::Mission {
                        objective: "Item Collection".to_string(),
                        failure_reason: format!("Item {} not found at {:?}", item_name, (x, y)),
                    });
                }
            }

            Ok(collected_items)
        }

        fn navigate_through_doors(&mut self, robot_id: &str) -> RobotResult<u32> {
            let robot = self.robots.get_mut(robot_id).ok_or_else(|| {
                RobotSystemError::Mission {
                    objective: "Door Navigation".to_string(),
                    failure_reason: format!("Robot {} not found", robot_id),
                }
            })?;

            let door_positions = vec![(5, 4), (9, 6), (3, 9), (13, 8)];
            let mut doors_opened = 0;

            for door_pos in door_positions {
                // Move near the door - ? propagates movement errors
                let approach_pos = (door_pos.0 - 1, door_pos.1);
                robot.move_to(approach_pos)?;

                // Attempt to open the door - ? propagates door operation errors
                robot.attempt_door_opening(door_pos)?;

                doors_opened += 1;
                self.mission_log.push(format!("{} opened door at {:?}", robot_id, door_pos));
            }

            Ok(doors_opened)
        }

        fn complete_mission(&mut self, robot_id: &str) -> RobotResult<String> {
            // Chain multiple operations using ? operator
            // Each ? automatically returns if an error occurs

            self.mission_log.push(format!("Starting complete mission for {}", robot_id));

            // Execute movement sequence
            let waypoints = vec![(2, 0), (4, 1), (8, 3), (12, 5)];
            self.execute_movement_sequence(robot_id, waypoints)?;

            // Collect items
            let collected_items = self.collect_all_items(robot_id)?;

            // Navigate doors
            let doors_opened = self.navigate_through_doors(robot_id)?;

            // Final movement to goal
            let robot = self.robots.get_mut(robot_id).unwrap(); // Safe because previous operations succeeded
            robot.move_to((15, 11))?;

            // Generate mission report
            let report = format!(
                "Mission completed successfully!\n\
                 Robot: {}\n\
                 Items collected: {} ({})\n\
                 Doors opened: {}\n\
                 Final position: {:?}\n\
                 Remaining energy: {}",
                robot_id,
                collected_items.len(),
                collected_items.join(", "),
                doors_opened,
                robot.position,
                robot.energy
            );

            Ok(report)
        }

        fn parallel_mission_attempt(&mut self) -> RobotResult<Vec<String>> {
            let robot_ids = vec!["Alpha", "Beta", "Gamma"];
            let mut results = Vec::new();

            // Add robots to the mission
            for id in &robot_ids {
                self.add_robot(id.to_string())?; // ? propagates any robot addition errors
            }

            // Attempt missions for each robot
            for robot_id in robot_ids {
                match self.complete_mission(&robot_id) {
                    Ok(report) => {
                        results.push(format!("✅ {}: SUCCESS", robot_id));
                        results.push(report);
                    }
                    Err(e) => {
                        // Convert error to string and continue with other robots
                        results.push(format!("❌ {}: FAILED - {:?}", robot_id, e));

                        // Log the failure
                        self.mission_log.push(format!("{} mission failed: {:?}", robot_id, e));
                    }
                }
            }

            Ok(results)
        }

        fn get_mission_summary(&self) -> String {
            format!("Mission Log ({} entries):\n{}",
                   self.mission_log.len(),
                   self.mission_log.join("\n"))
        }
    }

    println!("🚀 Mission Control - Error Propagation Demo");

    let mut mission_control = MissionController::new();

    // Demonstrate successful mission with ? operator
    println!("\n=== Single Robot Mission ===");
    match mission_control.add_robot("MainRobot".to_string()) {
        Ok(()) => println!("✅ Robot added successfully"),
        Err(e) => println!("❌ Failed to add robot: {:?}", e),
    }

    match mission_control.complete_mission("MainRobot") {
        Ok(report) => {
            println!("✅ Mission completed!");
            println!("{}", report);
        }
        Err(e) => {
            println!("❌ Mission failed: {:?}", e);
            println!("Error propagated through function chain");
        }
    }

    // Demonstrate parallel missions with mixed results
    println!("\n=== Parallel Mission Attempt ===");
    let mut parallel_control = MissionController::new();

    match parallel_control.parallel_mission_attempt() {
        Ok(results) => {
            println!("Parallel mission results:");
            for result in results {
                println!("{}", result);
            }
        }
        Err(e) => println!("❌ Parallel mission setup failed: {:?}", e),
    }

    // Show mission logs
    println!("\n=== Mission Logs ===");
    println!("{}", mission_control.get_mission_summary());

    // Demonstrate error recovery
    println!("\n=== Error Recovery Example ===");
    let recovery_result = (|| -> RobotResult<String> {
        let mut recovery_control = MissionController::new();

        // This might fail, but ? will propagate the error
        recovery_control.add_robot("RecoveryBot".to_string())?;

        // Try the mission, propagating any errors
        let report = recovery_control.complete_mission("RecoveryBot")?;

        Ok(format!("Recovery successful: {}", report))
    })();

    match recovery_result {
        Ok(success_msg) => println!("✅ {}", success_msg),
        Err(e) => println!("❌ Recovery failed: {:?}", e),
    }
}