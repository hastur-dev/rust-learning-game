// Level 8: Enums and Advanced Patterns - Automated Test Solutions
// Updated to match actual learning tests

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_8_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 8: Enums and Advanced Patterns",
        level_index: 7,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Basic Enums",
                solution_code: r#"#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
enum RobotCommand {
    Move(Direction),
    Scan,
    Grab,
    OpenDoor,
    Recharge,
    Wait,
}

#[derive(Debug, PartialEq)]
enum ActionResult {
    Success,
    Failed(String),
    Blocked,
    EnergyRequired(u32),
}

fn execute_command(command: RobotCommand) -> ActionResult {
    match command {
        RobotCommand::Move(direction) => {
            match direction {
                Direction::North => {
                    println!("Moving north");
                    ActionResult::Success
                }
                Direction::South => {
                    println!("Moving south");
                    ActionResult::Success
                }
                Direction::East => {
                    println!("Moving east");
                    ActionResult::Success
                }
                Direction::West => {
                    println!("Moving west");
                    ActionResult::Success
                }
            }
        }
        RobotCommand::Scan => {
            println!("Scanning area...");
            ActionResult::Success
        }
        RobotCommand::Grab => {
            println!("Attempting to grab item");
            ActionResult::Success
        }
        RobotCommand::OpenDoor => {
            println!("Opening door");
            ActionResult::EnergyRequired(15)
        }
        RobotCommand::Recharge => {
            println!("Recharging battery");
            ActionResult::Success
        }
        RobotCommand::Wait => {
            println!("Waiting...");
            ActionResult::Success
        }
    }
}

fn main() {
    println!("Robot Command System Demo");

    // Create various robot commands
    let commands = vec![
        RobotCommand::Move(Direction::East),
        RobotCommand::Move(Direction::East),
        RobotCommand::Move(Direction::North),
        RobotCommand::Scan,
        RobotCommand::Grab,
        RobotCommand::OpenDoor,
        RobotCommand::Move(Direction::South),
        RobotCommand::Recharge,
    ];

    // Execute commands and handle results
    for (i, command) in commands.iter().enumerate() {
        println!("\n--- Command {} ---", i + 1);
        println!("Executing: {:?}", command);

        let result = execute_command(command.clone());
        println!("Result: {:?}", result);

        match result {
            ActionResult::Success => println!("✅ Command completed successfully"),
            ActionResult::Failed(reason) => println!("❌ Command failed: {}", reason),
            ActionResult::Blocked => println!("🚧 Path blocked, trying alternative"),
            ActionResult::EnergyRequired(amount) => {
                println!("⚡ Command requires {} energy", amount);
            }
        }
    }

    // Direction utilities
    let current_direction = Direction::North;
    let opposite = match current_direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    };

    println!("\nDirection: {:?}, Opposite: {:?}", current_direction, opposite);
}"#,
                completion_indicators: vec![
                    "Robot Command System Demo", "Moving east", "Command completed successfully", "Direction: North"
                ],
            },
            TaskTest {
                task_number: 2,
                task_name: "State Machine",
                solution_code: r#"#[derive(Debug, Clone, PartialEq)]
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
}"#,
                completion_indicators: vec![
                    "Robot State Machine Simulation", "State: Idle", "Cycle 1", "Exploring area", "Collected: enum_core"
                ],
            },
            TaskTest {
                task_number: 3,
                task_name: "Option Enum",
                solution_code: r#"#[derive(Debug, Clone)]
struct GameItem {
    name: String,
    value: u32,
    item_type: String,
}

#[derive(Debug)]
struct ItemScanner {
    position: (i32, i32),
    scan_range: u32,
}

impl ItemScanner {
    fn new(x: i32, y: i32) -> Self {
        ItemScanner {
            position: (x, y),
            scan_range: 2,
        }
    }

    // Simulate scanning for items - returns Option
    fn scan_for_item(&self, search_pos: (i32, i32)) -> Option<GameItem> {
        let distance = ((self.position.0 - search_pos.0).abs() +
                       (self.position.1 - search_pos.1).abs()) as u32;

        if distance <= self.scan_range {
            // Simulate finding different items based on position
            match search_pos {
                (3, 1) => Some(GameItem {
                    name: "Enum Core".to_string(),
                    value: 100,
                    item_type: "Data".to_string(),
                }),
                (12, 3) => Some(GameItem {
                    name: "State Machine".to_string(),
                    value: 150,
                    item_type: "Logic".to_string(),
                }),
                (1, 8) => Some(GameItem {
                    name: "Option Handler".to_string(),
                    value: 75,
                    item_type: "Safety".to_string(),
                }),
                (8, 10) => Some(GameItem {
                    name: "Result Processor".to_string(),
                    value: 125,
                    item_type: "Error".to_string(),
                }),
                _ => None, // No item at this position
            }
        } else {
            None // Out of scan range
        }
    }

    fn find_nearest_item(&self, positions: Vec<(i32, i32)>) -> Option<(GameItem, (i32, i32))> {
        let mut nearest_item = None;
        let mut nearest_distance = u32::MAX;

        for pos in positions {
            if let Some(item) = self.scan_for_item(pos) {
                let distance = ((self.position.0 - pos.0).abs() +
                               (self.position.1 - pos.1).abs()) as u32;

                if distance < nearest_distance {
                    nearest_distance = distance;
                    nearest_item = Some((item, pos));
                }
            }
        }

        nearest_item
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.position = (x, y);
        println!("Scanner moved to ({}, {})", x, y);
    }
}

#[derive(Debug)]
struct ItemCollector {
    inventory: Vec<GameItem>,
    max_capacity: usize,
}

impl ItemCollector {
    fn new(capacity: usize) -> Self {
        ItemCollector {
            inventory: Vec::new(),
            max_capacity: capacity,
        }
    }

    fn collect_item(&mut self, item_option: Option<GameItem>) -> Option<String> {
        match item_option {
            Some(item) => {
                if self.inventory.len() < self.max_capacity {
                    let item_name = item.name.clone();
                    self.inventory.push(item);
                    Some(format!("Collected: {}", item_name))
                } else {
                    Some("Inventory full!".to_string())
                }
            }
            None => Some("No item to collect".to_string()),
        }
    }

    fn find_item_by_name(&self, name: &str) -> Option<&GameItem> {
        self.inventory.iter().find(|item| item.name == name)
    }

    fn use_item(&mut self, name: &str) -> Option<GameItem> {
        if let Some(index) = self.inventory.iter().position(|item| item.name == name) {
            Some(self.inventory.remove(index))
        } else {
            None
        }
    }

    fn get_total_value(&self) -> u32 {
        self.inventory.iter().map(|item| item.value).sum()
    }
}

fn main() {
    println!("=== Option Enum Item Collection System ===");

    let mut scanner = ItemScanner::new(0, 0);
    let mut collector = ItemCollector::new(10);

    // Item positions from level 8
    let item_positions = vec![(3, 1), (12, 3), (1, 8), (8, 10), (5, 5)];

    println!("Starting scan from position {:?}", scanner.position);

    // Scan from starting position
    for pos in &item_positions {
        match scanner.scan_for_item(*pos) {
            Some(item) => {
                println!("Found item at {:?}: {:?}", pos, item);
                if let Some(message) = collector.collect_item(Some(item)) {
                    println!("  {}", message);
                }
            }
            None => {
                println!("No item detected at {:?}", pos);
            }
        }
    }

    println!("\n--- Moving closer to items ---");

    // Move scanner and try again
    scanner.move_to(2, 2);

    // Find nearest item
    match scanner.find_nearest_item(item_positions.clone()) {
        Some((item, position)) => {
            println!("Nearest item: {:?} at {:?}", item, position);
            if let Some(message) = collector.collect_item(Some(item)) {
                println!("  {}", message);
            }
        }
        None => {
            println!("No items in range");
        }
    }

    // Move to collect more items
    for pos in item_positions {
        scanner.move_to(pos.0, pos.1);

        if let Some(item) = scanner.scan_for_item(pos) {
            if let Some(message) = collector.collect_item(Some(item)) {
                println!("  {}", message);
            }
        }
    }

    println!("\n=== Inventory Management ===");
    println!("Items in inventory: {}", collector.inventory.len());
    println!("Total value: {}", collector.get_total_value());

    // Try to find specific items
    match collector.find_item_by_name("Option Handler") {
        Some(item) => println!("Found: {:?}", item),
        None => println!("Option Handler not found"),
    }

    // Use an item
    match collector.use_item("Enum Core") {
        Some(used_item) => println!("Used: {:?}", used_item),
        None => println!("Enum Core not available"),
    }

    println!("Items remaining: {}", collector.inventory.len());

    // Demonstrate Option chaining
    let result = scanner.scan_for_item((1, 8))
        .and_then(|item| collector.collect_item(Some(item)))
        .unwrap_or_else(|| "No item found or collection failed".to_string());

    println!("Chained operation: {}", result);
}"#,
                completion_indicators: vec![
                    "Option Enum Item Collection System", "Found item at", "No item detected at", "Scanner moved to", "Collected:"
                ],
            },
            TaskTest {
                task_number: 4,
                task_name: "Result Enum",
                solution_code: r#"#[derive(Debug, Clone)]
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
}"#,
                completion_indicators: vec![
                    "Result Enum Error Handling System", "Movement successful", "Scan successful", "Mission completed successfully"
                ],
            },
            TaskTest {
                task_number: 5,
                task_name: "Advanced Enums",
                solution_code: r#"#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
enum RobotError {
    InsufficientEnergy { required: u32, available: u32 },
    PathBlocked { obstacle: String },
    InvalidPosition { x: i32, y: i32 },
    SystemFailure { component: String },
}

#[derive(Debug)]
struct RobotController {
    position: (i32, i32),
    energy: u32,
    map_bounds: (i32, i32),
}

impl RobotController {
    fn new() -> Self {
        RobotController {
            position: (0, 0),
            energy: 100,
            map_bounds: (14, 12),
        }
    }

    fn move_to(&mut self, x: i32, y: i32) -> Result<(), RobotError> {
        let energy_cost = ((x - self.position.0).abs() + (y - self.position.1).abs()) as u32 * 5;

        if self.energy < energy_cost {
            return Err(RobotError::InsufficientEnergy {
                required: energy_cost,
                available: self.energy,
            });
        }

        if x < 0 || y < 0 || x >= self.map_bounds.0 || y >= self.map_bounds.1 {
            return Err(RobotError::InvalidPosition { x, y });
        }

        let blocked_positions = vec![(4, 3), (9, 6), (3, 9), (11, 4)];
        if blocked_positions.contains(&(x, y)) {
            return Err(RobotError::PathBlocked {
                obstacle: "Enemy".to_string(),
            });
        }

        self.position = (x, y);
        self.energy -= energy_cost;
        Ok(())
    }

    fn recharge(&mut self) -> Result<(), RobotError> {
        self.energy = 100;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum AIState {
    Initializing,
    Planning { objectives: Vec<String> },
    Executing { current_objective: String, steps_remaining: u32 },
    Adapting { problem: RobotError, retry_count: u32 },
    Completed { success: bool, items_collected: u32 },
}

#[derive(Debug, Clone)]
enum DecisionFactor {
    EnergyLevel(u32),
    ThreatLevel(u32),
    ItemsNearby(Vec<String>),
    PathClearance(bool),
    MissionProgress(f32),
}

#[derive(Debug, Clone)]
enum AIAction {
    Move(Direction),
    Scan { radius: u32 },
    Collect { item: String },
    Avoid { threat_pos: (i32, i32) },
    Recharge,
    Wait { duration: u32 },
    Abort { reason: String },
}

struct AdvancedRobotAI {
    controller: RobotController,
    current_state: AIState,
    decision_factors: Vec<DecisionFactor>,
    action_history: Vec<AIAction>,
    objectives: Vec<String>,
    max_retries: u32,
}

impl AdvancedRobotAI {
    fn new() -> Self {
        AdvancedRobotAI {
            controller: RobotController::new(),
            current_state: AIState::Initializing,
            decision_factors: Vec::new(),
            action_history: Vec::new(),
            objectives: vec![
                "Collect enum_core".to_string(),
                "Collect state_machine".to_string(),
                "Collect option_handler".to_string(),
                "Collect result_processor".to_string(),
                "Reach goal".to_string(),
            ],
            max_retries: 3,
        }
    }

    fn analyze_situation(&mut self) {
        self.decision_factors.clear();

        // Analyze energy
        self.decision_factors.push(DecisionFactor::EnergyLevel(self.controller.energy));

        // Analyze threats (simulate enemy detection)
        let enemy_positions = vec![(4, 3), (9, 6), (3, 9), (11, 4)];
        let mut threat_level = 0;

        for enemy_pos in enemy_positions {
            let distance = (self.controller.position.0 - enemy_pos.0).abs() +
                          (self.controller.position.1 - enemy_pos.1).abs();
            if distance <= 3 {
                threat_level += 10 / (distance + 1) as u32;
            }
        }

        self.decision_factors.push(DecisionFactor::ThreatLevel(threat_level));

        // Analyze items nearby
        let item_positions = vec![(3, 1), (12, 3), (1, 8), (8, 10)];
        let mut nearby_items = Vec::new();

        for item_pos in item_positions {
            let distance = (self.controller.position.0 - item_pos.0).abs() +
                          (self.controller.position.1 - item_pos.1).abs();
            if distance <= 4 {
                nearby_items.push(format!("item_at_{:?}", item_pos));
            }
        }

        self.decision_factors.push(DecisionFactor::ItemsNearby(nearby_items));

        // Calculate mission progress
        let completed_objectives = 5 - self.objectives.len();
        let progress = completed_objectives as f32 / 5.0;
        self.decision_factors.push(DecisionFactor::MissionProgress(progress));
    }

    fn decide_next_action(&self) -> AIAction {
        // Decision-making based on current state and factors
        match &self.current_state {
            AIState::Initializing => AIAction::Scan { radius: 3 },
            AIState::Planning { objectives } => {
                if objectives.is_empty() {
                    AIAction::Move(Direction::East) // Head to goal
                } else {
                    AIAction::Move(Direction::North) // Start first objective
                }
            }
            AIState::Executing { current_objective, steps_remaining } => {
                // Check decision factors
                for factor in &self.decision_factors {
                    match factor {
                        DecisionFactor::EnergyLevel(energy) if *energy < 30 => {
                            return AIAction::Recharge;
                        }
                        DecisionFactor::ThreatLevel(threat) if *threat > 20 => {
                            return AIAction::Avoid { threat_pos: (4, 3) };
                        }
                        DecisionFactor::ItemsNearby(items) if !items.is_empty() => {
                            return AIAction::Collect { item: items[0].clone() };
                        }
                        _ => {}
                    }
                }

                if *steps_remaining > 0 {
                    AIAction::Move(Direction::East)
                } else {
                    AIAction::Scan { radius: 2 }
                }
            }
            AIState::Adapting { problem, retry_count } => {
                match problem {
                    RobotError::InsufficientEnergy { .. } => AIAction::Recharge,
                    RobotError::PathBlocked { .. } => AIAction::Move(Direction::South),
                    _ => {
                        if *retry_count >= self.max_retries {
                            AIAction::Abort { reason: "Too many failures".to_string() }
                        } else {
                            AIAction::Move(Direction::West)
                        }
                    }
                }
            }
            AIState::Completed { .. } => AIAction::Wait { duration: 0 },
        }
    }

    fn execute_action(&mut self, action: AIAction) -> Result<(), RobotError> {
        println!("Executing: {:?}", action);
        self.action_history.push(action.clone());

        match action {
            AIAction::Move(direction) => {
                let (dx, dy) = match direction {
                    Direction::North => (0, 1),
                    Direction::South => (0, -1),
                    Direction::East => (1, 0),
                    Direction::West => (-1, 0),
                };

                let new_x = self.controller.position.0 + dx;
                let new_y = self.controller.position.1 + dy;

                self.controller.move_to(new_x, new_y)
            }
            AIAction::Scan { radius } => {
                println!("Scanning with radius {}", radius);
                Ok(()) // Simulate successful scan
            }
            AIAction::Collect { item } => {
                println!("Successfully collected: {}", item);
                self.objectives.retain(|obj| !obj.contains(&item));
                Ok(())
            }
            AIAction::Avoid { threat_pos } => {
                // Move away from threat
                let away_x = if self.controller.position.0 < threat_pos.0 {
                    self.controller.position.0 - 1
                } else {
                    self.controller.position.0 + 1
                };

                self.controller.move_to(away_x, self.controller.position.1)
            }
            AIAction::Recharge => self.controller.recharge(),
            AIAction::Wait { duration } => {
                println!("Waiting for {} cycles", duration);
                Ok(())
            }
            AIAction::Abort { reason } => {
                println!("Mission aborted: {}", reason);
                Err(RobotError::SystemFailure { component: reason })
            }
        }
    }

    fn update_state(&mut self, action_result: Result<(), RobotError>) {
        self.current_state = match (&self.current_state, action_result) {
            (AIState::Initializing, Ok(())) => {
                AIState::Planning { objectives: self.objectives.clone() }
            }
            (AIState::Planning { .. }, Ok(())) => {
                if self.objectives.is_empty() {
                    AIState::Completed { success: true, items_collected: 4 }
                } else {
                    AIState::Executing {
                        current_objective: self.objectives[0].clone(),
                        steps_remaining: 10,
                    }
                }
            }
            (AIState::Executing { steps_remaining, .. }, Ok(())) => {
                if *steps_remaining <= 1 {
                    AIState::Planning { objectives: self.objectives.clone() }
                } else {
                    AIState::Executing {
                        current_objective: self.objectives.get(0).unwrap_or(&"Complete".to_string()).clone(),
                        steps_remaining: steps_remaining - 1,
                    }
                }
            }
            (_, Err(error)) => {
                let retry_count = match &self.current_state {
                    AIState::Adapting { retry_count, .. } => retry_count + 1,
                    _ => 1,
                };

                if retry_count > self.max_retries {
                    AIState::Completed { success: false, items_collected: 0 }
                } else {
                    AIState::Adapting {
                        problem: error,
                        retry_count,
                    }
                }
            }
            (AIState::Adapting { retry_count, .. }, Ok(())) => {
                AIState::Planning { objectives: self.objectives.clone() }
            }
            (state, _) => state.clone(),
        };
    }

    fn run_ai_cycle(&mut self) -> bool {
        println!("\n--- AI Cycle ---");
        println!("State: {:?}", self.current_state);
        println!("Position: {:?}", self.controller.position);
        println!("Energy: {}", self.controller.energy);
        println!("Objectives remaining: {}", self.objectives.len());

        // Analyze situation
        self.analyze_situation();

        // Decide action
        let action = self.decide_next_action();

        // Execute action
        let result = self.execute_action(action);

        // Update state
        self.update_state(result);

        // Check if mission is complete
        matches!(self.current_state, AIState::Completed { .. })
    }
}

fn main() {
    println!("🤖 Advanced Robot AI System - Level 8 Mission");

    let mut ai = AdvancedRobotAI::new();

    // Run AI for up to 25 cycles
    for cycle in 1..=25 {
        println!("\n🔄 === Cycle {} ===", cycle);

        if ai.run_ai_cycle() {
            println!("\n🎯 Mission Complete!");
            match &ai.current_state {
                AIState::Completed { success, items_collected } => {
                    if *success {
                        println!("✅ Success! Collected {} items", items_collected);
                    } else {
                        println!("❌ Mission failed after collecting {} items", items_collected);
                    }
                }
                _ => unreachable!(),
            }
            break;
        }
    }

    println!("\n📊 Final Statistics:");
    println!("Actions taken: {}", ai.action_history.len());
    println!("Objectives remaining: {}", ai.objectives.len());
    println!("Final position: {:?}", ai.controller.position);
    println!("Final energy: {}", ai.controller.energy);

    println!("\n📝 Action History:");
    for (i, action) in ai.action_history.iter().enumerate().take(10) {
        println!("  {}: {:?}", i + 1, action);
    }
}"#,
                completion_indicators: vec![
                    "Advanced Robot AI System", "AI Cycle", "State: Initializing", "Executing:", "Mission Complete"
                ],
            }
        ],
    }
}