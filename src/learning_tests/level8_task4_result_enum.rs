// Level 8 Task 4 Test: Result Enum for Error Handling
// Tests if the user code uses Result<T,E> enum for comprehensive error handling

#[cfg(test)]
mod level8_task4_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_robot_error_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("enum RobotError") ||
            analyzer.code.contains("enum Error"),
            "❌ Your code should define a RobotError enum"
        );
    }

    #[test]
    fn test_error_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_insufficient_energy = analyzer.code.contains("InsufficientEnergy");
        let has_path_blocked = analyzer.code.contains("PathBlocked");
        let has_invalid_position = analyzer.code.contains("InvalidPosition");
        assert!(
            has_insufficient_energy && has_path_blocked && has_invalid_position,
            "❌ Your RobotError enum should have InsufficientEnergy, PathBlocked, and InvalidPosition variants"
        );
    }

    #[test]
    fn test_error_variants_with_data() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_required_available = analyzer.code.contains("required") && analyzer.code.contains("available");
        let has_obstacle = analyzer.code.contains("obstacle");
        assert!(
            has_required_available && has_obstacle,
            "❌ Your error variants should carry descriptive data (required/available energy, obstacle type)"
        );
    }

    #[test]
    fn test_has_robot_controller_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct RobotController") ||
            analyzer.code.contains("struct Controller"),
            "❌ Your code should define a RobotController struct"
        );
    }

    #[test]
    fn test_returns_result_type() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("Result<(), RobotError>") ||
            analyzer.code.contains("Result<") ||
            analyzer.code.contains("-> Result"),
            "❌ Your methods should return Result<T, RobotError>"
        );
    }

    #[test]
    fn test_move_to_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn move_to"),
            "❌ Your controller should have a move_to method that returns Result"
        );
    }

    #[test]
    fn test_scan_item_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn scan_item") ||
            analyzer.code.contains("fn scan"),
            "❌ Your controller should have a scan_item method"
        );
    }

    #[test]
    fn test_uses_ok_err_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_ok = analyzer.code.contains("Ok(");
        let has_err = analyzer.code.contains("Err(");
        assert!(
            has_ok && has_err,
            "❌ Your code should use Ok() and Err() variants of Result"
        );
    }

    #[test]
    fn test_error_pattern_matching() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let matches_result = analyzer.code.contains("match") &&
                            (analyzer.code.contains("Ok(") || analyzer.code.contains("Err("));
        assert!(
            matches_result,
            "❌ Your code should use pattern matching with Result (Ok/Err)"
        );
    }

    #[test]
    fn test_energy_checking() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let checks_energy = analyzer.code.contains("energy") &&
                           analyzer.code.contains("energy_cost");
        assert!(
            checks_energy,
            "❌ Your code should check energy requirements before operations"
        );
    }

    #[test]
    fn test_bounds_checking() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let checks_bounds = analyzer.code.contains("map_bounds") ||
                           (analyzer.code.contains("x <") && analyzer.code.contains("y <"));
        assert!(
            checks_bounds,
            "❌ Your code should check position bounds"
        );
    }

    #[test]
    fn test_obstacle_checking() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let checks_obstacles = analyzer.code.contains("blocked_positions") ||
                              analyzer.code.contains("obstacle");
        assert!(
            checks_obstacles,
            "❌ Your code should check for obstacles before moving"
        );
    }

    #[test]
    fn test_mission_execution() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("execute_mission") ||
            analyzer.code.contains("mission"),
            "❌ Your code should have a mission execution function"
        );
    }

    #[test]
    fn test_error_propagation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let propagates_errors = analyzer.code.contains("return Err") ||
                               analyzer.code.contains("?");
        assert!(
            propagates_errors,
            "❌ Your code should propagate errors properly"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output Result handling information
        let has_result_output = result.stdout.contains("successful") ||
                               result.stdout.contains("failed") ||
                               result.stdout.contains("Error") ||
                               result.stdout.contains("Mission");

        assert!(
            has_result_output,
            "❌ Your program should output information about Result handling"
        );
    }
}

// Reference implementation for comparison
#[derive(Debug, Clone)]
enum RobotError {
    InsufficientEnergy { required: u32, available: u32 },
    PathBlocked { obstacle: String },
    InvalidPosition { x: i32, y: i32 },
    ItemNotFound { item_name: String },
    DoorLocked { key_required: String },
    SystemFailure { component: String },
}

#[derive(Debug, Clone)]
struct GameItem {
    name: String,
    value: u32,
    item_type: String,
}

#[derive(Debug)]
struct RobotController {
    position: (i32, i32),
    energy: u32,
    keys: Vec<String>,
    map_bounds: (i32, i32),
}

impl RobotController {
    fn new() -> Self {
        RobotController {
            position: (0, 0),
            energy: 100,
            keys: Vec::new(),
            map_bounds: (14, 12),
        }
    }

    fn move_to(&mut self, x: i32, y: i32) -> Result<(), RobotError> {
        // Check energy
        let energy_cost = ((x - self.position.0).abs() + (y - self.position.1).abs()) as u32 * 5;

        if self.energy < energy_cost {
            return Err(RobotError::InsufficientEnergy {
                required: energy_cost,
                available: self.energy,
            });
        }

        // Check bounds
        if x < 0 || y < 0 || x >= self.map_bounds.0 || y >= self.map_bounds.1 {
            return Err(RobotError::InvalidPosition { x, y });
        }

        // Check for obstacles (simulate some blocked positions)
        let blocked_positions = vec![(4, 3), (9, 6), (3, 9), (11, 4)];
        if blocked_positions.contains(&(x, y)) {
            return Err(RobotError::PathBlocked {
                obstacle: "Enemy".to_string(),
            });
        }

        // Successful move
        self.position = (x, y);
        self.energy -= energy_cost;
        Ok(())
    }

    fn scan_item(&self, item_name: &str) -> Result<GameItem, RobotError> {
        // Simulate item locations and scanning
        let items = [
            ((3, 1), GameItem { name: "enum_core".to_string(), value: 100, item_type: "Data".to_string() }),
            ((12, 3), GameItem { name: "state_machine".to_string(), value: 150, item_type: "Logic".to_string() }),
            ((1, 8), GameItem { name: "option_handler".to_string(), value: 75, item_type: "Safety".to_string() }),
            ((8, 10), GameItem { name: "result_processor".to_string(), value: 125, item_type: "Error".to_string() }),
        ];

        for ((x, y), item) in &items {
            if item.name == item_name {
                // Check if robot is close enough to scan
                let distance = (self.position.0 - x).abs() + (self.position.1 - y).abs();
                if distance <= 2 {
                    return Ok(item.clone());
                } else {
                    return Err(RobotError::ItemNotFound {
                        item_name: item_name.to_string(),
                    });
                }
            }
        }

        Err(RobotError::ItemNotFound {
            item_name: item_name.to_string(),
        })
    }

    fn open_door(&mut self, door_pos: (i32, i32)) -> Result<(), RobotError> {
        // Door positions from level 8
        let doors = vec![(7, 2), (2, 7), (10, 9), (6, 6)];

        if !doors.contains(&door_pos) {
            return Err(RobotError::InvalidPosition { x: door_pos.0, y: door_pos.1 });
        }

        // Check if robot has a key
        if self.keys.is_empty() {
            return Err(RobotError::DoorLocked {
                key_required: "Universal Key".to_string(),
            });
        }

        // Check energy
        if self.energy < 15 {
            return Err(RobotError::InsufficientEnergy {
                required: 15,
                available: self.energy,
            });
        }

        // Check distance to door
        let distance = (self.position.0 - door_pos.0).abs() + (self.position.1 - door_pos.1).abs();
        if distance > 1 {
            return Err(RobotError::PathBlocked {
                obstacle: "Too far from door".to_string(),
            });
        }

        // Success - consume energy and key
        self.energy -= 15;
        self.keys.pop();
        Ok(())
    }

    fn add_key(&mut self, key: String) {
        self.keys.push(key);
    }

    fn recharge(&mut self) -> Result<(), RobotError> {
        if self.energy >= 90 {
            return Err(RobotError::SystemFailure {
                component: "Battery already near full".to_string(),
            });
        }

        self.energy = 100;
        Ok(())
    }

    fn get_status(&self) -> String {
        format!("Pos: {:?}, Energy: {}, Keys: {}",
               self.position, self.energy, self.keys.len())
    }
}

// Helper functions for Result handling
fn handle_movement_result(result: Result<(), RobotError>) -> String {
    match result {
        Ok(()) => "Movement successful".to_string(),
        Err(RobotError::InsufficientEnergy { required, available }) => {
            format!("Low energy: need {}, have {}", required, available)
        }
        Err(RobotError::PathBlocked { obstacle }) => {
            format!("Path blocked by: {}", obstacle)
        }
        Err(RobotError::InvalidPosition { x, y }) => {
            format!("Invalid position: ({}, {})", x, y)
        }
        Err(e) => format!("Movement error: {:?}", e),
    }
}

fn execute_mission(robot: &mut RobotController) -> Result<Vec<String>, Vec<RobotError>> {
    let mut mission_log = Vec::new();
    let mut errors = Vec::new();

    // Mission: Navigate to collect all items
    let destinations = vec![(3, 1), (12, 3), (1, 8), (8, 10)];
    let item_names = vec!["enum_core", "state_machine", "option_handler", "result_processor"];

    robot.add_key("Universal Key".to_string());

    for (i, &dest) in destinations.iter().enumerate() {
        // Try to move to destination
        match robot.move_to(dest.0, dest.1) {
            Ok(()) => {
                mission_log.push(format!("Reached destination: {:?}", dest));

                // Try to scan for item
                match robot.scan_item(item_names[i]) {
                    Ok(item) => {
                        mission_log.push(format!("Found item: {}", item.name));
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            }
            Err(e) => {
                errors.push(e.clone());

                // Try alternative path
                let alt_x = if dest.0 > 0 { dest.0 - 1 } else { dest.0 + 1 };
                match robot.move_to(alt_x, dest.1) {
                    Ok(()) => {
                        mission_log.push(format!("Used alternative path to near {:?}", dest));
                    }
                    Err(alt_e) => {
                        errors.push(alt_e);
                    }
                }
            }
        }

        // Try to open nearby door if present
        if let Err(e) = robot.open_door((dest.0, dest.1)) {
            // Door opening is optional, don't add to critical errors
            println!("Door operation: {:?}", e);
        }
    }

    if errors.is_empty() {
        Ok(mission_log)
    } else {
        Err(errors)
    }
}

fn main() {
    println!("=== Result Enum Error Handling System ===");

    let mut robot = RobotController::new();
    println!("Initial status: {}", robot.get_status());

    // Test individual operations
    println!("\n--- Testing Movement ---");
    let move_result = robot.move_to(3, 1);
    println!("Move result: {}", handle_movement_result(move_result));
    println!("Status: {}", robot.get_status());

    // Test scanning
    println!("\n--- Testing Scanning ---");
    match robot.scan_item("enum_core") {
        Ok(item) => println!("Scan successful: {:?}", item),
        Err(e) => println!("Scan failed: {:?}", e),
    }

    // Test door opening
    println!("\n--- Testing Door Operations ---");
    match robot.open_door((7, 2)) {
        Ok(()) => println!("Door opened successfully"),
        Err(e) => println!("Door operation failed: {:?}", e),
    }

    // Test recharging
    robot.energy = 20; // Simulate low energy
    match robot.recharge() {
        Ok(()) => println!("Recharge successful"),
        Err(e) => println!("Recharge failed: {:?}", e),
    }

    println!("\n--- Executing Full Mission ---");
    match execute_mission(&mut robot) {
        Ok(log) => {
            println!("✅ Mission completed successfully!");
            for entry in log {
                println!("  - {}", entry);
            }
        }
        Err(errors) => {
            println!("❌ Mission completed with {} errors:", errors.len());
            for error in errors {
                println!("  - {:?}", error);
            }
        }
    }

    println!("\nFinal status: {}", robot.get_status());
}