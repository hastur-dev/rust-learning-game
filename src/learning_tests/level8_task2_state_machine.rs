// Level 8 Task 2 Test: Robot State Machine with Enums
// Tests if the user code creates state machine using enums for robot behavior

#[cfg(test)]
mod level8_task2_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_robot_state_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("enum RobotState"),
            "❌ Your code should define a RobotState enum"
        );
    }

    #[test]
    fn test_robot_state_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_idle = analyzer.code.contains("Idle");
        let has_exploring = analyzer.code.contains("Exploring");
        let has_collecting = analyzer.code.contains("Collecting");
        assert!(
            has_idle && has_exploring && has_collecting,
            "❌ Your RobotState enum should have Idle, Exploring, and Collecting variants"
        );
    }

    #[test]
    fn test_enum_variants_with_data() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_energy_threshold = analyzer.code.contains("energy_threshold");
        let has_target_item = analyzer.code.contains("target_item");
        assert!(
            has_energy_threshold && has_target_item,
            "❌ Your enum variants should carry data (energy_threshold, target_item)"
        );
    }

    #[test]
    fn test_has_stateful_robot_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct StatefulRobot") ||
            analyzer.code.contains("struct Robot"),
            "❌ Your code should define a StatefulRobot struct"
        );
    }

    #[test]
    fn test_robot_has_current_state_field() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("current_state: RobotState") ||
            analyzer.code.contains("state: RobotState"),
            "❌ Your robot struct should have a current_state field of type RobotState"
        );
    }

    #[test]
    fn test_has_update_state_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn update_state"),
            "❌ Your robot should have an update_state() method"
        );
    }

    #[test]
    fn test_has_execute_behavior_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn execute_behavior") ||
            analyzer.code.contains("fn execute") ||
            analyzer.code.contains("fn run"),
            "❌ Your robot should have an execute_behavior() or similar method"
        );
    }

    #[test]
    fn test_state_transition_logic() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_state_matching = analyzer.code.contains("match") &&
                                analyzer.code.contains("current_state");
        assert!(
            has_state_matching,
            "❌ Your code should use match expressions for state transitions"
        );
    }

    #[test]
    fn test_robot_properties() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_position = analyzer.code.contains("position");
        let has_energy = analyzer.code.contains("energy");
        let has_health = analyzer.code.contains("health");
        assert!(
            has_position && has_energy,
            "❌ Your robot should have position and energy properties"
        );
    }

    #[test]
    fn test_state_based_behavior() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let matches_idle = analyzer.code.contains("RobotState::Idle") ||
                          analyzer.code.contains("Idle =>");
        let matches_exploring = analyzer.code.contains("RobotState::Exploring") ||
                               analyzer.code.contains("Exploring");
        assert!(
            matches_idle && matches_exploring,
            "❌ Your code should handle different behaviors based on robot state"
        );
    }

    #[test]
    fn test_energy_management() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let manages_energy = analyzer.code.contains("energy") &&
                            (analyzer.code.contains("saturating_sub") ||
                             analyzer.code.contains("self.energy -"));
        assert!(
            manages_energy,
            "❌ Your robot should manage energy consumption during actions"
        );
    }

    #[test]
    fn test_simulation_loop() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_simulation = analyzer.code.contains("cycle") ||
                            analyzer.code.contains("for") ||
                            analyzer.code.contains("loop");
        assert!(
            has_simulation,
            "❌ Your code should run a simulation loop to demonstrate state changes"
        );
    }

    #[test]
    fn test_threat_simulation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let simulates_threats = analyzer.code.contains("threat") ||
                               analyzer.code.contains("enemy") ||
                               analyzer.code.contains("Avoiding");
        assert!(
            simulates_threats,
            "❌ Your code should simulate threats or avoiding behavior"
        );
    }

    #[test]
    fn test_status_reporting() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("get_status") ||
            analyzer.code.contains("status"),
            "❌ Your robot should provide status reporting"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output state machine information
        let has_state_output = result.stdout.contains("State") ||
                              result.stdout.contains("Cycle") ||
                              result.stdout.contains("Robot") ||
                              result.stdout.contains("Exploring");

        assert!(
            has_state_output,
            "❌ Your program should output information about state machine execution"
        );
    }
}

// Reference implementation for comparison
#[derive(Debug, Clone, PartialEq)]
enum RobotState {
    Idle,
    Exploring { energy_threshold: u32 },
    Collecting { target_item: String },
    Avoiding { threat_position: (i32, i32) },
    Recharging { progress: u8 },
    EmergencyMode { reason: String },
}

#[derive(Debug)]
struct StatefulRobot {
    position: (i32, i32),
    energy: u32,
    health: u32,
    current_state: RobotState,
    items_collected: Vec<String>,
}

impl StatefulRobot {
    fn new() -> Self {
        StatefulRobot {
            position: (0, 0),
            energy: 100,
            health: 100,
            current_state: RobotState::Idle,
            items_collected: Vec::new(),
        }
    }

    fn update_state(&mut self) {
        // State transition logic
        self.current_state = match &self.current_state {
            RobotState::Idle => {
                if self.energy < 30 {
                    RobotState::Recharging { progress: 0 }
                } else if self.health < 50 {
                    RobotState::EmergencyMode { reason: "Low health".to_string() }
                } else {
                    RobotState::Exploring { energy_threshold: 20 }
                }
            }
            RobotState::Exploring { energy_threshold } => {
                if self.energy < *energy_threshold {
                    RobotState::Recharging { progress: 0 }
                } else {
                    // Simulate finding item
                    RobotState::Collecting { target_item: "enum_core".to_string() }
                }
            }
            RobotState::Collecting { target_item } => {
                self.items_collected.push(target_item.clone());
                println!("Collected: {}", target_item);
                RobotState::Exploring { energy_threshold: 30 }
            }
            RobotState::Avoiding { threat_position } => {
                println!("Avoided threat at {:?}", threat_position);
                RobotState::Exploring { energy_threshold: 40 }
            }
            RobotState::Recharging { progress } => {
                if *progress >= 100 {
                    self.energy = 100;
                    println!("Recharge complete");
                    RobotState::Idle
                } else {
                    RobotState::Recharging { progress: progress + 20 }
                }
            }
            RobotState::EmergencyMode { reason } => {
                println!("Emergency: {}", reason);
                if self.health >= 50 && self.energy >= 30 {
                    RobotState::Idle
                } else {
                    RobotState::Recharging { progress: 0 }
                }
            }
        };
    }

    fn execute_behavior(&mut self) {
        println!("State: {:?}", self.current_state);

        match &self.current_state {
            RobotState::Idle => {
                println!("Robot is idle, waiting for next action");
            }
            RobotState::Exploring { energy_threshold } => {
                println!("Exploring area (energy threshold: {})", energy_threshold);
                self.energy = self.energy.saturating_sub(5);
                self.position.0 += 1; // Simulate movement
            }
            RobotState::Collecting { target_item } => {
                println!("Collecting item: {}", target_item);
                self.energy = self.energy.saturating_sub(3);
            }
            RobotState::Avoiding { threat_position } => {
                println!("Avoiding threat at {:?}", threat_position);
                self.position.0 -= 1; // Move away
                self.energy = self.energy.saturating_sub(8);
            }
            RobotState::Recharging { progress } => {
                println!("Recharging... {}%", progress);
                self.energy += 10;
            }
            RobotState::EmergencyMode { reason } => {
                println!("EMERGENCY MODE: {}", reason);
                // Emergency actions
                if self.health < 50 {
                    self.health += 10;
                }
            }
        }
    }

    fn simulate_threat(&mut self, threat_pos: (i32, i32)) {
        self.current_state = RobotState::Avoiding { threat_position: threat_pos };
        self.health = self.health.saturating_sub(15);
    }

    fn get_status(&self) -> String {
        format!("Pos: {:?}, Energy: {}, Health: {}, Items: {}, State: {:?}",
               self.position, self.energy, self.health,
               self.items_collected.len(), self.current_state)
    }
}

fn main() {
    let mut robot = StatefulRobot::new();

    println!("=== Robot State Machine Simulation ===");

    // Run simulation for 15 cycles
    for cycle in 1..=15 {
        println!("\n--- Cycle {} ---", cycle);
        println!("Status: {}", robot.get_status());

        robot.execute_behavior();
        robot.update_state();

        // Simulate threats occasionally
        if cycle == 8 {
            println!("⚠️ Enemy detected!");
            robot.simulate_threat((4, 3));
        }

        if cycle == 12 {
            println!("⚠️ Another threat!");
            robot.simulate_threat((9, 6));
        }
    }

    println!("\n=== Final Status ===");
    println!("{}", robot.get_status());
    println!("Items collected: {:?}", robot.items_collected);
}