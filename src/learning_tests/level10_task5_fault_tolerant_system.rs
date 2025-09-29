// Level 10 Task 5 Test: Complete Fault-Tolerant Robot System
// Tests if the user code combines all error handling techniques into a production-ready system

#[cfg(test)]
mod level10_task5_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_defines_system_error_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_system_error = analyzer.code.contains("enum SystemError") ||
                              analyzer.code.contains("SystemError {");
        assert!(
            has_system_error,
            "❌ Your code should define a SystemError enum"
        );
    }

    #[test]
    fn test_system_error_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_critical = analyzer.code.contains("Critical");
        let has_recoverable = analyzer.code.contains("Recoverable");
        let has_warning = analyzer.code.contains("Warning");

        assert!(
            has_critical && has_recoverable && has_warning,
            "❌ SystemError should have Critical, Recoverable, and Warning variants"
        );
    }

    #[test]
    fn test_defines_critical_error_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_critical_error = analyzer.code.contains("enum CriticalError") ||
                                analyzer.code.contains("CriticalError {");
        assert!(
            has_critical_error,
            "❌ Your code should define a CriticalError enum"
        );
    }

    #[test]
    fn test_defines_recoverable_error_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_recoverable_error = analyzer.code.contains("enum RecoverableError") ||
                                  analyzer.code.contains("RecoverableError {");
        assert!(
            has_recoverable_error,
            "❌ Your code should define a RecoverableError enum"
        );
    }

    #[test]
    fn test_defines_impact_level_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_impact_level = analyzer.code.contains("enum ImpactLevel") ||
                              analyzer.code.contains("ImpactLevel {");
        assert!(
            has_impact_level,
            "❌ Your code should define an ImpactLevel enum"
        );
    }

    #[test]
    fn test_defines_mission_objective_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_mission_objective = analyzer.code.contains("struct MissionObjective") ||
                                   analyzer.code.contains("MissionObjective {");
        assert!(
            has_mission_objective,
            "❌ Your code should define a MissionObjective struct"
        );
    }

    #[test]
    fn test_defines_objective_type_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_objective_type = analyzer.code.contains("enum ObjectiveType") ||
                               analyzer.code.contains("ObjectiveType {");
        assert!(
            has_objective_type,
            "❌ Your code should define an ObjectiveType enum"
        );
    }

    #[test]
    fn test_defines_priority_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_priority = analyzer.code.contains("enum Priority") ||
                          analyzer.code.contains("Priority {");
        assert!(
            has_priority,
            "❌ Your code should define a Priority enum"
        );
    }

    #[test]
    fn test_defines_fault_tolerant_system() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_fault_tolerant = analyzer.code.contains("struct FaultTolerantRobotSystem") ||
                               analyzer.code.contains("FaultTolerantRobotSystem {");
        assert!(
            has_fault_tolerant,
            "❌ Your code should define a FaultTolerantRobotSystem struct"
        );
    }

    #[test]
    fn test_system_has_required_fields() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_robot = analyzer.code.contains("robot:");
        let has_error_log = analyzer.code.contains("error_log:");
        let has_mission_queue = analyzer.code.contains("mission_queue:");
        let has_system_health = analyzer.code.contains("system_health:");
        let has_error_budget = analyzer.code.contains("error_budget:");

        assert!(
            has_robot && has_error_log && has_mission_queue && has_system_health && has_error_budget,
            "❌ FaultTolerantRobotSystem should have robot, error_log, mission_queue, system_health, and error_budget fields"
        );
    }

    #[test]
    fn test_implements_log_error_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_log_error = analyzer.code.contains("fn log_error(");
        assert!(
            has_log_error,
            "❌ Your system should implement log_error method"
        );
    }

    #[test]
    fn test_implements_execute_objective_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_execute_objective = analyzer.code.contains("fn execute_objective(");
        assert!(
            has_execute_objective,
            "❌ Your system should implement execute_objective method"
        );
    }

    #[test]
    fn test_implements_execute_mission_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_execute_mission = analyzer.code.contains("fn execute_mission(");
        assert!(
            has_execute_mission,
            "❌ Your system should implement execute_mission method"
        );
    }

    #[test]
    fn test_implements_emergency_recovery() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_emergency_recovery = analyzer.code.contains("emergency_recovery") ||
                                   analyzer.code.contains("emergency");
        assert!(
            has_emergency_recovery,
            "❌ Your system should implement emergency recovery functionality"
        );
    }

    #[test]
    fn test_implements_error_conversion() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_conversion = analyzer.code.contains("convert_robot_error_to_system_error") ||
                           analyzer.code.contains("convert");
        assert!(
            has_conversion,
            "❌ Your system should implement error type conversion"
        );
    }

    #[test]
    fn test_priority_based_mission_planning() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_priority = analyzer.code.contains("priority") &&
                          analyzer.code.contains("Critical") &&
                          analyzer.code.contains("insert(position");
        assert!(
            uses_priority,
            "❌ Your system should implement priority-based mission planning"
        );
    }

    #[test]
    fn test_system_health_monitoring() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let monitors_health = analyzer.code.contains("system_health") &&
                            analyzer.code.contains("< 20.0");
        assert!(
            monitors_health,
            "❌ Your system should monitor system health and react to critical levels"
        );
    }

    #[test]
    fn test_error_budget_management() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let manages_budget = analyzer.code.contains("error_budget") &&
                           analyzer.code.contains("== 0");
        assert!(
            manages_budget,
            "❌ Your system should manage error budget and react when exhausted"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_fault_tolerant_output = result.stdout.contains("Fault-Tolerant") ||
                                       result.stdout.contains("objective") ||
                                       result.stdout.contains("Mission") ||
                                       result.stdout.contains("System health") ||
                                       result.stdout.contains("Error budget") ||
                                       result.stdout.contains("CRITICAL");

        assert!(
            has_fault_tolerant_output,
            "❌ Your program should output information about fault-tolerant system operations"
        );
    }
}

// Reference implementation
fn main() {
    use std::collections::{HashMap, VecDeque};
    use std::fmt;

    // Complete error taxonomy for the robot system
    #[derive(Debug, Clone)]
    pub enum SystemError {
        // Critical errors that stop the mission
        Critical {
            component: String,
            error: CriticalError,
            recovery_impossible: bool,
        },
        // Recoverable errors that can be retried
        Recoverable {
            operation: String,
            error: RecoverableError,
            retry_count: u32,
        },
        // Warning-level errors that don't stop execution
        Warning {
            source: String,
            message: String,
            impact: ImpactLevel,
        },
    }

    #[derive(Debug, Clone)]
    pub enum CriticalError {
        PowerSystemFailure,
        NavigationSystemOffline,
        CommunicationLost,
        HardwareDestroyed,
    }

    #[derive(Debug, Clone)]
    pub enum RecoverableError {
        InsufficientEnergy { required: u32, available: u32 },
        PathBlocked { position: (i32, i32), obstacle: String },
        TemporaryHardwareGlitch { component: String },
        ResourceUnavailable { resource: String },
    }

    #[derive(Debug, Clone)]
    pub enum ImpactLevel {
        Low,       // Minor performance impact
        Medium,    // Noticeable but not critical
        High,      // Significant impact on mission
    }

    impl fmt::Display for SystemError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                SystemError::Critical { component, error, recovery_impossible } => {
                    write!(f, "CRITICAL: {:?} failure in {} (Recovery: {})",
                           error, component,
                           if *recovery_impossible { "impossible" } else { "attempting" })
                }
                SystemError::Recoverable { operation, error, retry_count } => {
                    write!(f, "RECOVERABLE: {} failed (attempt {}): {:?}",
                           operation, retry_count, error)
                }
                SystemError::Warning { source, message, impact } => {
                    write!(f, "WARNING [{:?}]: {} - {}", impact, source, message)
                }
            }
        }
    }

    type SystemResult<T> = Result<T, SystemError>;

    #[derive(Debug, Clone)]
    struct MissionObjective {
        id: String,
        objective_type: ObjectiveType,
        target_position: (i32, i32),
        priority: Priority,
        required_resources: Vec<String>,
        estimated_energy_cost: u32,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum ObjectiveType {
        Navigate,
        CollectItem(String),
        OpenDoor,
        ScanArea,
        EmergencyEvacuation,
    }

    #[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
    enum Priority {
        Low = 1,
        Medium = 2,
        High = 3,
        Critical = 4,
    }

    // Simplified robot types for compilation
    #[derive(Debug)]
    struct RecoveryRobot {
        inner: AdvancedRobot,
    }

    #[derive(Debug)]
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
    }

    impl RecoveryRobot {
        fn new() -> Self {
            RecoveryRobot {
                inner: AdvancedRobot::new(),
            }
        }

        fn safe_move_to(&mut self, _target: (i32, i32)) -> Result<(), SystemError> {
            if self.inner.energy < 10 {
                return Err(SystemError::Recoverable {
                    operation: "movement".to_string(),
                    error: RecoverableError::InsufficientEnergy {
                        required: 10,
                        available: self.inner.energy,
                    },
                    retry_count: 1,
                });
            }
            self.inner.energy -= 10;
            Ok(())
        }

        fn safe_scan_for_items(&self) -> Result<Vec<(String, (i32, i32))>, SystemError> {
            if self.inner.energy < 5 {
                return Err(SystemError::Recoverable {
                    operation: "scan".to_string(),
                    error: RecoverableError::InsufficientEnergy {
                        required: 5,
                        available: self.inner.energy,
                    },
                    retry_count: 1,
                });
            }
            Ok(vec![("test_item".to_string(), (1, 1))])
        }

        fn attempt_energy_recovery(&mut self) -> Result<(), SystemError> {
            self.inner.energy += 30;
            Ok(())
        }

        fn attempt_hardware_recovery(&mut self) -> Result<(), SystemError> {
            self.inner.systems_online = true;
            self.inner.health = 100;
            Ok(())
        }

        fn attempt_movement_recovery(&mut self) -> Result<(), SystemError> {
            self.inner.position = (1, 1);
            Ok(())
        }
    }

    struct FaultTolerantRobotSystem {
        robot: RecoveryRobot,
        error_log: Vec<SystemError>,
        mission_queue: VecDeque<MissionObjective>,
        completed_objectives: Vec<String>,
        system_health: f32,
        error_budget: u32, // Maximum recoverable errors before mission abort
        warning_count: u32,
    }

    impl FaultTolerantRobotSystem {
        fn new() -> Self {
            FaultTolerantRobotSystem {
                robot: RecoveryRobot::new(),
                error_log: Vec::new(),
                mission_queue: VecDeque::new(),
                completed_objectives: Vec::new(),
                system_health: 100.0,
                error_budget: 10,
                warning_count: 0,
            }
        }

        fn add_mission_objective(&mut self, objective: MissionObjective) {
            // Insert based on priority
            let position = self.mission_queue.iter()
                .position(|obj| obj.priority < objective.priority)
                .unwrap_or(self.mission_queue.len());

            self.mission_queue.insert(position, objective);
        }

        fn log_error(&mut self, error: SystemError) {
            match &error {
                SystemError::Critical { .. } => {
                    self.system_health -= 25.0;
                }
                SystemError::Recoverable { .. } => {
                    self.system_health -= 5.0;
                    if self.error_budget > 0 {
                        self.error_budget -= 1;
                    }
                }
                SystemError::Warning { impact, .. } => {
                    self.warning_count += 1;
                    let health_impact = match impact {
                        ImpactLevel::Low => 1.0,
                        ImpactLevel::Medium => 2.5,
                        ImpactLevel::High => 5.0,
                    };
                    self.system_health -= health_impact;
                }
            }

            self.error_log.push(error);
            self.system_health = self.system_health.max(0.0);
        }

        fn execute_objective(&mut self, objective: MissionObjective) -> SystemResult<String> {
            println!("🎯 Executing objective: {} (Priority: {:?})", objective.id, objective.priority);

            // Pre-flight checks
            if self.system_health < 20.0 {
                return Err(SystemError::Critical {
                    component: "System Health".to_string(),
                    error: CriticalError::PowerSystemFailure,
                    recovery_impossible: true,
                });
            }

            if self.error_budget == 0 {
                return Err(SystemError::Critical {
                    component: "Error Budget".to_string(),
                    error: CriticalError::NavigationSystemOffline,
                    recovery_impossible: false,
                });
            }

            // Check energy requirements
            if self.robot.inner.energy < objective.estimated_energy_cost {
                let error = SystemError::Recoverable {
                    operation: format!("Execute {}", objective.id),
                    error: RecoverableError::InsufficientEnergy {
                        required: objective.estimated_energy_cost,
                        available: self.robot.inner.energy,
                    },
                    retry_count: 1,
                };

                self.log_error(error.clone());

                // Attempt energy recovery
                if let Err(_recovery_error) = self.robot.attempt_energy_recovery() {
                    return Err(SystemError::Critical {
                        component: "Energy Recovery".to_string(),
                        error: CriticalError::PowerSystemFailure,
                        recovery_impossible: true,
                    });
                }
            }

            // Execute the objective based on type
            let result = match &objective.objective_type {
                ObjectiveType::Navigate => {
                    match self.robot.safe_move_to(objective.target_position) {
                        Ok(()) => Ok(format!("Navigated to {:?}", objective.target_position)),
                        Err(system_error) => {
                            self.log_error(system_error.clone());
                            Err(system_error)
                        }
                    }
                }
                ObjectiveType::CollectItem(item_name) => {
                    match self.robot.safe_move_to(objective.target_position) {
                        Ok(()) => {
                            match self.robot.safe_scan_for_items() {
                                Ok(items) => {
                                    if items.iter().any(|(name, _)| name == item_name) {
                                        Ok(format!("Collected item: {}", item_name))
                                    } else {
                                        let error = SystemError::Warning {
                                            source: "Item Collection".to_string(),
                                            message: format!("Item {} not found at location", item_name),
                                            impact: ImpactLevel::Medium,
                                        };
                                        self.log_error(error.clone());
                                        Err(error)
                                    }
                                }
                                Err(system_error) => {
                                    self.log_error(system_error.clone());
                                    Err(system_error)
                                }
                            }
                        }
                        Err(system_error) => {
                            self.log_error(system_error.clone());
                            Err(system_error)
                        }
                    }
                }
                ObjectiveType::ScanArea => {
                    match self.robot.safe_scan_for_items() {
                        Ok(items) => Ok(format!("Scanned area, found {} objects", items.len())),
                        Err(system_error) => {
                            self.log_error(system_error.clone());
                            Err(system_error)
                        }
                    }
                }
                ObjectiveType::EmergencyEvacuation => {
                    // Find nearest safe position
                    let safe_positions = vec![(0, 0), (15, 11), (1, 1), (14, 10)];
                    for safe_pos in safe_positions {
                        if let Ok(()) = self.robot.safe_move_to(safe_pos) {
                            return Ok(format!("Emergency evacuation to {:?}", safe_pos));
                        }
                    }

                    Err(SystemError::Critical {
                        component: "Emergency Systems".to_string(),
                        error: CriticalError::NavigationSystemOffline,
                        recovery_impossible: true,
                    })
                }
                _ => Ok("Objective completed".to_string()),
            };

            match &result {
                Ok(success_msg) => {
                    self.completed_objectives.push(objective.id.clone());
                    println!("✅ {}", success_msg);
                }
                Err(e) => {
                    println!("❌ Objective {} failed: {}", objective.id, e);
                }
            }

            result
        }

        fn execute_mission(&mut self) -> SystemResult<String> {
            let mut mission_summary = Vec::new();
            let mut objectives_completed = 0;
            let mut objectives_failed = 0;

            mission_summary.push("🚀 Starting fault-tolerant mission execution".to_string());

            while let Some(objective) = self.mission_queue.pop_front() {
                // Check if we should abort the mission
                if self.system_health < 10.0 {
                    mission_summary.push("🚨 Mission aborted due to critical system health".to_string());
                    break;
                }

                if self.error_budget == 0 {
                    mission_summary.push("⚠️ Error budget exhausted, switching to critical objectives only".to_string());

                    // Only execute critical priority objectives
                    if objective.priority < Priority::Critical {
                        continue;
                    }
                }

                match self.execute_objective(objective) {
                    Ok(result) => {
                        objectives_completed += 1;
                        mission_summary.push(format!("✅ {}", result));
                    }
                    Err(SystemError::Critical { recovery_impossible: true, .. }) => {
                        objectives_failed += 1;
                        mission_summary.push("🚨 Critical unrecoverable error - aborting mission".to_string());
                        break;
                    }
                    Err(error) => {
                        objectives_failed += 1;
                        mission_summary.push(format!("❌ Objective failed: {}", error));

                        // For critical errors that might be recoverable, try emergency procedures
                        if let SystemError::Critical { recovery_impossible: false, .. } = error {
                            if let Ok(()) = self.emergency_recovery() {
                                mission_summary.push("🔧 Emergency recovery successful".to_string());
                            } else {
                                mission_summary.push("💀 Emergency recovery failed".to_string());
                                break;
                            }
                        }
                    }
                }
            }

            let final_report = format!(
                "📊 Mission Summary:\n\
                 Objectives completed: {}\n\
                 Objectives failed: {}\n\
                 System health: {:.1}%\n\
                 Error budget remaining: {}\n\
                 Warnings: {}\n\
                 Final position: {:?}\n\
                 \n{}",
                objectives_completed,
                objectives_failed,
                self.system_health,
                self.error_budget,
                self.warning_count,
                self.robot.inner.position,
                mission_summary.join("\n")
            );

            Ok(final_report)
        }

        fn emergency_recovery(&mut self) -> SystemResult<()> {
            println!("🚨 Initiating emergency recovery procedures...");

            // Attempt system health recovery
            self.robot.attempt_hardware_recovery()?;

            // Attempt energy recovery
            self.robot.attempt_energy_recovery()?;

            // Move to safe position
            self.robot.attempt_movement_recovery()?;

            // Boost system health if recovery was successful
            self.system_health += 20.0;
            self.system_health = self.system_health.min(100.0);

            println!("✅ Emergency recovery completed");
            Ok(())
        }

        fn generate_error_report(&self) -> String {
            let mut report = String::from("📋 System Error Report:\n\n");

            let critical_errors = self.error_log.iter()
                .filter(|e| matches!(e, SystemError::Critical { .. }))
                .count();

            let recoverable_errors = self.error_log.iter()
                .filter(|e| matches!(e, SystemError::Recoverable { .. }))
                .count();

            report.push_str(&format!("Critical errors: {}\n", critical_errors));
            report.push_str(&format!("Recoverable errors: {}\n", recoverable_errors));
            report.push_str(&format!("Warnings: {}\n\n", self.warning_count));

            report.push_str("Recent errors:\n");
            for error in self.error_log.iter().rev().take(5) {
                report.push_str(&format!("  - {}\n", error));
            }

            report
        }
    }

    println!("🛡️ Complete Fault-Tolerant Robot System Demo");

    let mut system = FaultTolerantRobotSystem::new();

    // Add mission objectives
    let objectives = vec![
        MissionObjective {
            id: "nav_to_first_item".to_string(),
            objective_type: ObjectiveType::Navigate,
            target_position: (4, 1),
            priority: Priority::High,
            required_resources: vec!["navigation".to_string()],
            estimated_energy_cost: 25,
        },
        MissionObjective {
            id: "collect_result_core".to_string(),
            objective_type: ObjectiveType::CollectItem("result_core".to_string()),
            target_position: (4, 1),
            priority: Priority::Critical,
            required_resources: vec!["scanner".to_string()],
            estimated_energy_cost: 15,
        },
        MissionObjective {
            id: "emergency_position".to_string(),
            objective_type: ObjectiveType::EmergencyEvacuation,
            target_position: (15, 11),
            priority: Priority::Critical,
            required_resources: vec!["navigation".to_string()],
            estimated_energy_cost: 40,
        },
    ];

    for objective in objectives {
        system.add_mission_objective(objective);
    }

    // Execute the mission
    println!("\n=== Mission Execution ===");
    match system.execute_mission() {
        Ok(report) => {
            println!("{}", report);
        }
        Err(e) => {
            println!("💀 Complete mission failure: {}", e);
        }
    }

    // Generate error report
    println!("\n=== Error Analysis ===");
    println!("{}", system.generate_error_report());

    println!("\n🏁 Fault-tolerant system demonstration complete!");
}