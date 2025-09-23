// Level 10 Task 4 Test: Error Recovery and Fallback Strategies
// Tests if the user code implements sophisticated error recovery mechanisms

#[cfg(test)]
mod level10_task4_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_imports_time_module() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let imports_time = analyzer.code.contains("use std::time") ||
                          analyzer.code.contains("Duration") ||
                          analyzer.code.contains("Instant");
        assert!(
            imports_time,
            "❌ Your code should import std::time module for Duration and Instant"
        );
    }

    #[test]
    fn test_defines_retry_config() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_retry_config = analyzer.code.contains("struct RetryConfig") ||
                              analyzer.code.contains("RetryConfig {");
        assert!(
            has_retry_config,
            "❌ Your code should define a RetryConfig struct"
        );
    }

    #[test]
    fn test_retry_config_fields() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_max_attempts = analyzer.code.contains("max_attempts");
        let has_delay = analyzer.code.contains("delay_between_attempts");
        let has_backoff = analyzer.code.contains("exponential_backoff");

        assert!(
            has_max_attempts && has_delay && has_backoff,
            "❌ RetryConfig should have max_attempts, delay_between_attempts, and exponential_backoff fields"
        );
    }

    #[test]
    fn test_defines_recovery_robot() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_recovery_robot = analyzer.code.contains("struct RecoveryRobot") ||
                               analyzer.code.contains("RecoveryRobot {");
        assert!(
            has_recovery_robot,
            "❌ Your code should define a RecoveryRobot struct"
        );
    }

    #[test]
    fn test_recovery_robot_fields() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_inner = analyzer.code.contains("inner:");
        let has_config = analyzer.code.contains("retry_config:");
        let has_fallback = analyzer.code.contains("fallback_positions:");
        let has_emergency = analyzer.code.contains("emergency_energy:");

        assert!(
            has_inner && has_config && has_fallback && has_emergency,
            "❌ RecoveryRobot should have inner, retry_config, fallback_positions, and emergency_energy fields"
        );
    }

    #[test]
    fn test_implements_retry_with_recovery() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_retry_method = analyzer.code.contains("fn retry_with_recovery") ||
                              analyzer.code.contains("retry_with_recovery");
        assert!(
            has_retry_method,
            "❌ Your code should implement retry_with_recovery method"
        );
    }

    #[test]
    fn test_uses_generic_closures() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_generics = analyzer.code.contains("where") &&
                          analyzer.code.contains("Fn(") &&
                          analyzer.code.contains("-> RobotResult");
        assert!(
            uses_generics,
            "❌ Your retry_with_recovery should use generic closures with where clause"
        );
    }

    #[test]
    fn test_implements_energy_recovery() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_energy_recovery = analyzer.code.contains("attempt_energy_recovery") ||
                                analyzer.code.contains("energy_recovery");
        assert!(
            has_energy_recovery,
            "❌ Your code should implement attempt_energy_recovery method"
        );
    }

    #[test]
    fn test_implements_movement_recovery() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_movement_recovery = analyzer.code.contains("attempt_movement_recovery") ||
                                  analyzer.code.contains("movement_recovery");
        assert!(
            has_movement_recovery,
            "❌ Your code should implement attempt_movement_recovery method"
        );
    }

    #[test]
    fn test_implements_hardware_recovery() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_hardware_recovery = analyzer.code.contains("attempt_hardware_recovery") ||
                                  analyzer.code.contains("hardware_recovery");
        assert!(
            has_hardware_recovery,
            "❌ Your code should implement attempt_hardware_recovery method"
        );
    }

    #[test]
    fn test_uses_sleep_and_backoff() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_sleep = analyzer.code.contains("thread::sleep") ||
                        analyzer.code.contains("sleep(");
        let has_backoff = analyzer.code.contains("exponential_backoff") &&
                        analyzer.code.contains("* 2");
        assert!(
            uses_sleep && has_backoff,
            "❌ Your code should use thread::sleep and implement exponential backoff"
        );
    }

    #[test]
    fn test_fallback_position_strategy() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_fallback = analyzer.code.contains("fallback_positions") &&
                          analyzer.code.contains("for ") &&
                          analyzer.code.contains("&fallback_pos");
        assert!(
            uses_fallback,
            "❌ Your code should iterate through fallback positions for recovery"
        );
    }

    #[test]
    fn test_safe_wrapper_methods() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_safe_move = analyzer.code.contains("safe_move_to");
        let has_safe_scan = analyzer.code.contains("safe_scan_for_items");
        let has_safe_door = analyzer.code.contains("safe_door_operation");

        assert!(
            has_safe_move && has_safe_scan && has_safe_door,
            "❌ Your code should implement safe wrapper methods (safe_move_to, safe_scan_for_items, safe_door_operation)"
        );
    }

    #[test]
    fn test_mission_with_recovery() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_mission_recovery = analyzer.code.contains("execute_mission_with_recovery") ||
                                 analyzer.code.contains("mission_with_recovery");
        assert!(
            has_mission_recovery,
            "❌ Your code should implement execute_mission_with_recovery method"
        );
    }

    #[test]
    fn test_stress_testing() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_stress_test = analyzer.code.contains("stress_test") ||
                            analyzer.code.contains("stress");
        assert!(
            has_stress_test,
            "❌ Your code should implement stress testing functionality"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_recovery_output = result.stdout.contains("Attempt") ||
                                result.stdout.contains("recovery") ||
                                result.stdout.contains("fallback") ||
                                result.stdout.contains("succeeded after") ||
                                result.stdout.contains("Mission completed");

        assert!(
            has_recovery_output,
            "❌ Your program should output information about error recovery operations"
        );
    }
}

// Reference implementation
fn main() {
    use std::time::{Duration, Instant};

    #[derive(Debug)]
    enum RobotSystemError {
        Energy { current: u32, required: u32, operation: String },
        Movement { attempted_position: (i32, i32) },
        Hardware { component: String },
        Mission { objective: String, failure_reason: String },
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
            Ok(vec![("test_item".to_string(), (1, 1))])
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

    #[derive(Debug, Clone)]
    struct RetryConfig {
        max_attempts: u32,
        delay_between_attempts: Duration,
        exponential_backoff: bool,
    }

    impl Default for RetryConfig {
        fn default() -> Self {
            RetryConfig {
                max_attempts: 3,
                delay_between_attempts: Duration::from_millis(100),
                exponential_backoff: true,
            }
        }
    }

    #[derive(Debug)]
    struct RecoveryRobot {
        inner: AdvancedRobot,
        retry_config: RetryConfig,
        fallback_positions: Vec<(i32, i32)>,
        emergency_energy: u32,
    }

    impl RecoveryRobot {
        fn new() -> Self {
            RecoveryRobot {
                inner: AdvancedRobot::new(),
                retry_config: RetryConfig::default(),
                fallback_positions: vec![(1, 1), (5, 5), (10, 8), (2, 9)],
                emergency_energy: 30,
            }
        }

        fn retry_with_recovery<F, T>(&mut self, operation: F, operation_name: &str) -> RobotResult<T>
        where
            F: Fn(&mut AdvancedRobot) -> RobotResult<T>,
        {
            let mut last_error = None;
            let mut delay = self.retry_config.delay_between_attempts;

            for attempt in 1..=self.retry_config.max_attempts {
                println!("Attempt {} of {} for {}", attempt, self.retry_config.max_attempts, operation_name);

                match operation(&mut self.inner) {
                    Ok(result) => {
                        if attempt > 1 {
                            println!("✅ {} succeeded after {} attempts", operation_name, attempt);
                        }
                        return Ok(result);
                    }
                    Err(e) => {
                        println!("❌ {} attempt {} failed: {:?}", operation_name, attempt, e);
                        last_error = Some(e);

                        // Don't sleep on the last attempt
                        if attempt < self.retry_config.max_attempts {
                            // Try recovery strategies based on error type
                            match &last_error {
                                Some(RobotSystemError::Energy { .. }) => {
                                    println!("🔋 Attempting energy recovery...");
                                    let _ = self.attempt_energy_recovery();
                                }
                                Some(RobotSystemError::Movement { .. }) => {
                                    println!("🚧 Attempting movement recovery...");
                                    let _ = self.attempt_movement_recovery();
                                }
                                Some(RobotSystemError::Hardware { .. }) => {
                                    println!("🔧 Attempting hardware recovery...");
                                    let _ = self.attempt_hardware_recovery();
                                }
                                _ => {
                                    println!("⏳ Generic recovery delay...");
                                }
                            }

                            // Sleep with exponential backoff
                            std::thread::sleep(delay);

                            if self.retry_config.exponential_backoff {
                                delay = Duration::from_millis(delay.as_millis() as u64 * 2);
                            }
                        }
                    }
                }
            }

            // All attempts failed, return the last error
            Err(last_error.unwrap_or_else(|| RobotSystemError::Mission {
                objective: operation_name.to_string(),
                failure_reason: "All retry attempts failed".to_string(),
            }))
        }

        fn attempt_energy_recovery(&mut self) -> RobotResult<()> {
            if self.inner.energy < self.emergency_energy {
                println!("🔌 Emergency energy boost activated");
                self.inner.energy += self.emergency_energy;
                Ok(())
            } else {
                println!("⚡ Energy levels acceptable, no recovery needed");
                Ok(())
            }
        }

        fn attempt_movement_recovery(&mut self) -> RobotResult<()> {
            println!("🗺️ Trying fallback positions...");

            for &fallback_pos in &self.fallback_positions {
                if let Ok(()) = self.inner.move_to(fallback_pos) {
                    println!("✅ Moved to fallback position: {:?}", fallback_pos);
                    return Ok(());
                }
            }

            Err(RobotSystemError::Mission {
                objective: "Movement Recovery".to_string(),
                failure_reason: "All fallback positions failed".to_string(),
            })
        }

        fn attempt_hardware_recovery(&mut self) -> RobotResult<()> {
            println!("🔧 Running system diagnostics and repair...");

            // Simulate hardware recovery
            if !self.inner.systems_online {
                self.inner.systems_online = true;
                println!("✅ Systems brought back online");
            }

            if self.inner.health < 50 {
                self.inner.health += 25;
                println!("✅ System health improved to {}", self.inner.health);
            }

            Ok(())
        }

        fn safe_move_to(&mut self, target: (i32, i32)) -> RobotResult<()> {
            self.retry_with_recovery(
                |robot| robot.move_to(target),
                &format!("move to {:?}", target)
            )
        }

        fn safe_scan_for_items(&mut self) -> RobotResult<Vec<(String, (i32, i32))>> {
            self.retry_with_recovery(
                |robot| robot.scan_for_items(),
                "area scan"
            )
        }

        fn safe_door_operation(&mut self, door_pos: (i32, i32)) -> RobotResult<()> {
            self.retry_with_recovery(
                |robot| robot.attempt_door_opening(door_pos),
                &format!("open door at {:?}", door_pos)
            )
        }

        fn execute_mission_with_recovery(&mut self) -> RobotResult<String> {
            let mut mission_report = Vec::new();
            let start_time = Instant::now();

            mission_report.push("🚀 Starting fault-tolerant mission...".to_string());

            // Phase 1: Navigate to first item with recovery
            match self.safe_move_to((4, 1)) {
                Ok(()) => mission_report.push("✅ Reached first item location".to_string()),
                Err(e) => {
                    mission_report.push(format!("❌ Failed to reach first item: {:?}", e));
                    return Ok(mission_report.join("\n"));
                }
            }

            // Phase 2: Scan for items with recovery
            match self.safe_scan_for_items() {
                Ok(items) => {
                    mission_report.push(format!("✅ Scanned area, found {} items", items.len()));
                }
                Err(e) => {
                    mission_report.push(format!("❌ Scan failed: {:?}", e));
                    // Continue mission despite scan failure
                }
            }

            // Phase 3: Attempt door operations with recovery
            let door_positions = vec![(5, 4), (9, 6)];
            let mut doors_opened = 0;

            for door_pos in door_positions {
                // Move near door first
                if let Ok(()) = self.safe_move_to((door_pos.0 - 1, door_pos.1)) {
                    // Try to open door
                    match self.safe_door_operation(door_pos) {
                        Ok(()) => {
                            doors_opened += 1;
                            mission_report.push(format!("✅ Opened door at {:?}", door_pos));
                        }
                        Err(e) => {
                            mission_report.push(format!("❌ Failed to open door at {:?}: {:?}", door_pos, e));
                        }
                    }
                }
            }

            // Phase 4: Final navigation with multiple fallbacks
            let goal_position = (15, 11);
            let intermediate_positions = vec![(10, 8), (12, 9), (14, 10)];

            let mut reached_goal = false;

            // Try direct path first
            if let Ok(()) = self.safe_move_to(goal_position) {
                reached_goal = true;
                mission_report.push("✅ Reached goal via direct path".to_string());
            } else {
                // Try intermediate positions
                for intermediate in intermediate_positions {
                    if let Ok(()) = self.safe_move_to(intermediate) {
                        if let Ok(()) = self.safe_move_to(goal_position) {
                            reached_goal = true;
                            mission_report.push(format!("✅ Reached goal via intermediate position {:?}", intermediate));
                            break;
                        }
                    }
                }
            }

            if !reached_goal {
                mission_report.push("❌ Unable to reach goal position".to_string());
            }

            let mission_time = start_time.elapsed();
            mission_report.push(format!("⏱️ Mission completed in {:.2} seconds", mission_time.as_secs_f32()));
            mission_report.push(format!("📊 Final status: Position {:?}, Energy: {}, Health: {}",
                                       self.inner.position, self.inner.energy, self.inner.health));

            Ok(mission_report.join("\n"))
        }

        fn stress_test_with_recovery(&mut self) -> RobotResult<Vec<String>> {
            let mut results = Vec::new();

            // Simulate various stress conditions
            results.push("🧪 Starting stress test with recovery...".to_string());

            // Test 1: Low energy operations
            self.inner.energy = 15;
            match self.safe_move_to((5, 5)) {
                Ok(()) => results.push("✅ Low energy movement recovered".to_string()),
                Err(e) => results.push(format!("❌ Low energy test failed: {:?}", e)),
            }

            // Test 2: Hardware failure simulation
            self.inner.systems_online = false;
            self.inner.health = 25;
            match self.safe_scan_for_items() {
                Ok(_) => results.push("✅ Hardware failure recovered".to_string()),
                Err(e) => results.push(format!("❌ Hardware recovery failed: {:?}", e)),
            }

            // Test 3: Multiple consecutive failures
            self.inner.energy = 5;
            self.inner.health = 10;
            match self.safe_move_to((12, 8)) {
                Ok(()) => results.push("✅ Multiple failure recovery succeeded".to_string()),
                Err(e) => results.push(format!("❌ Multiple failure recovery failed: {:?}", e)),
            }

            Ok(results)
        }
    }

    println!("🛡️ Advanced Error Recovery System Demo");

    let mut recovery_robot = RecoveryRobot::new();

    // Demonstrate normal mission with recovery
    println!("\n=== Mission with Automatic Recovery ===");
    match recovery_robot.execute_mission_with_recovery() {
        Ok(report) => {
            println!("Mission Report:");
            println!("{}", report);
        }
        Err(e) => println!("❌ Mission completely failed: {:?}", e),
    }

    // Reset robot for stress test
    let mut stress_robot = RecoveryRobot::new();

    println!("\n=== Stress Test with Recovery ===");
    match stress_robot.stress_test_with_recovery() {
        Ok(results) => {
            for result in results {
                println!("{}", result);
            }
        }
        Err(e) => println!("❌ Stress test failed: {:?}", e),
    }

    // Demonstrate custom retry configuration
    println!("\n=== Custom Recovery Configuration ===");
    let mut custom_robot = RecoveryRobot::new();
    custom_robot.retry_config = RetryConfig {
        max_attempts: 5,
        delay_between_attempts: Duration::from_millis(50),
        exponential_backoff: false,
    };

    // Force a challenging scenario
    custom_robot.inner.energy = 8;
    custom_robot.inner.health = 20;

    match custom_robot.safe_move_to((8, 8)) {
        Ok(()) => println!("✅ Custom recovery configuration succeeded"),
        Err(e) => println!("❌ Custom recovery failed: {:?}", e),
    }
}