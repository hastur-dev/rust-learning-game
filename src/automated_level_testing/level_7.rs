// Level 7: Advanced Ownership and Lifetimes - Automated Test Solutions

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_7_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 7: Advanced Ownership and Lifetimes",
        level_index: 6,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Lifetime Annotations and References",
                solution_code: r#"fn main() {
    println!("🔗 LEVEL 7: Advanced Ownership and Lifetimes - References");

    // Basic lifetime demonstration
    let robot_name = String::from("ALPHA-PRIME");
    let mission_ref = &robot_name;
    println!("✓ Robot {} reference created", mission_ref);

    // Function with lifetime parameters
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let mission_a = "SCAN-OPERATION";
    let mission_b = "DEEP-EXPLORATION-SURVEY";
    let longest_mission = longest(mission_a, mission_b);
    println!("✓ Longest mission: {}", longest_mission);

    // Lifetime with struct
    struct RobotMission<'a> {
        name: &'a str,
        duration: u32,
    }

    let mission_name = "PERIMETER-PATROL";
    let robot_mission = RobotMission {
        name: mission_name,
        duration: 300,
    };
    println!("✓ Mission {} created ({}s duration)", robot_mission.name, robot_mission.duration);

    // Multiple references with lifetimes
    let primary_target = "SECTOR-7";
    let secondary_target = "BACKUP-ZONE";

    fn analyze_targets<'a>(primary: &'a str, secondary: &'a str) -> (&'a str, &'a str) {
        println!("✓ Analyzing targets: {} and {}", primary, secondary);
        (primary, secondary)
    }

    let (p, s) = analyze_targets(primary_target, secondary_target);
    println!("✓ Target analysis complete: primary={}, secondary={}", p, s);

    println!("🎯 Lifetime annotations and references mastered!");
}"#,
                completion_indicators: vec![
                    "🔗 LEVEL 7: Advanced Ownership and Lifetimes - References",
                    "✓ Robot ALPHA-PRIME reference created",
                    "✓ Longest mission: DEEP-EXPLORATION-SURVEY",
                    "✓ Mission PERIMETER-PATROL created (300s duration)",
                    "✓ Analyzing targets: SECTOR-7 and BACKUP-ZONE",
                    "✓ Target analysis complete: primary=SECTOR-7, secondary=BACKUP-ZONE",
                    "🎯 Lifetime annotations and references mastered!",
                ],
            },

            TaskTest {
                task_number: 2,
                task_name: "Borrowing Checker and Mutable References",
                solution_code: r#"fn main() {
    println!("🔧 LEVEL 7: Borrowing Checker - Mutable References");

    // Mutable borrowing demonstration
    let mut robot_energy = 75;
    println!("✓ Initial robot energy: {}%", robot_energy);

    // Function that borrows mutably
    fn recharge_robot(energy: &mut i32) {
        *energy += 25;
        println!("✓ Robot recharged! Energy now: {}%", energy);
    }

    recharge_robot(&mut robot_energy);
    println!("✓ Final robot energy: {}%", robot_energy);

    // Multiple immutable borrows
    let robot_status = String::from("OPERATIONAL-READY");
    let status_ref1 = &robot_status;
    let status_ref2 = &robot_status;
    println!("✓ Status check 1: {}", status_ref1);
    println!("✓ Status check 2: {}", status_ref2);

    // Scope-based borrowing
    {
        let mut mission_log = Vec::new();
        mission_log.push("STARTUP");
        mission_log.push("PATROL");
        mission_log.push("SCAN");

        for entry in &mission_log {
            println!("✓ Log entry: {}", entry);
        }

        // Mutable borrow after immutable borrows go out of scope
        mission_log.push("COMPLETE");
        println!("✓ Mission log updated: {} entries", mission_log.len());
    }

    // Reference validity demonstration
    let robot_position;
    {
        let temp_coords = String::from("GRID-42-ALPHA");
        // robot_position = &temp_coords; // Would not compile - lifetime issue
        robot_position = temp_coords.clone(); // Safe approach
    }
    println!("✓ Robot position stored: {}", robot_position);

    // Function that doesn't take ownership
    fn check_robot_status(status: &str) -> bool {
        println!("✓ Checking robot status: {}", status);
        status.contains("OPERATIONAL")
    }

    let current_status = "OPERATIONAL-STANDBY";
    let is_operational = check_robot_status(current_status);
    println!("✓ Robot operational: {}", is_operational);
    println!("✓ Status still available: {}", current_status);

    println!("🎯 Borrowing checker patterns mastered!");
}"#,
                completion_indicators: vec![
                    "🔧 LEVEL 7: Borrowing Checker - Mutable References",
                    "✓ Initial robot energy: 75%",
                    "✓ Robot recharged! Energy now: 100%",
                    "✓ Final robot energy: 100%",
                    "✓ Status check 1: OPERATIONAL-READY",
                    "✓ Status check 2: OPERATIONAL-READY",
                    "✓ Log entry: STARTUP",
                    "✓ Log entry: PATROL",
                    "✓ Log entry: SCAN",
                    "✓ Mission log updated: 4 entries",
                    "✓ Robot position stored: GRID-42-ALPHA",
                    "✓ Checking robot status: OPERATIONAL-STANDBY",
                    "✓ Robot operational: true",
                    "✓ Status still available: OPERATIONAL-STANDBY",
                    "🎯 Borrowing checker patterns mastered!",
                ],
            },

            TaskTest {
                task_number: 3,
                task_name: "Advanced Reference Patterns and Dangling Prevention",
                solution_code: r#"fn main() {
    println!("🛡️ LEVEL 7: Advanced References - Dangling Prevention");

    // Safe reference patterns
    fn create_robot_report() -> String {
        String::from("ROBOT-STATUS-REPORT-2024")
    }

    // Return owned data, not references
    let report = create_robot_report();
    println!("✓ Robot report created: {}", report);

    // Reference parameter patterns
    fn process_mission_data(data: &str) -> String {
        format!("PROCESSED-{}", data)
    }

    let mission_data = "SCAN-RESULTS-ALPHA";
    let processed = process_mission_data(mission_data);
    println!("✓ Mission data processed: {}", processed);
    println!("✓ Original data still valid: {}", mission_data);

    // Advanced borrowing with collections
    let mut robot_fleet = vec![
        String::from("ALPHA-1"),
        String::from("BETA-2"),
        String::from("GAMMA-3")
    ];

    // Iterate with references
    for robot in &robot_fleet {
        println!("✓ Robot in fleet: {}", robot);
    }

    // Modify after iteration
    robot_fleet.push(String::from("DELTA-4"));
    println!("✓ Fleet expanded to {} robots", robot_fleet.len());

    // Reference to vector elements
    if let Some(first_robot) = robot_fleet.first() {
        println!("✓ Fleet leader: {}", first_robot);
    }

    // Safe slicing
    let fleet_slice = &robot_fleet[0..2];
    for robot in fleet_slice {
        println!("✓ Command squad robot: {}", robot);
    }

    // Method chaining with references
    let robot_name = String::from("OMEGA-COMMANDER");
    let name_length = robot_name.len();
    let name_upper = robot_name.to_uppercase();
    println!("✓ Robot name: {} (length: {})", robot_name, name_length);
    println!("✓ Robot callsign: {}", name_upper);

    // Struct with references - lifetime safe patterns
    #[derive(Debug)]
    struct RobotConfig {
        id: String,
        energy: i32,
        status: String,
    }

    let config = RobotConfig {
        id: String::from("RC-001"),
        energy: 95,
        status: String::from("ACTIVE"),
    };

    fn display_config(config: &RobotConfig) {
        println!("✓ Config: {} - {}% energy, status: {}",
                config.id, config.energy, config.status);
    }

    display_config(&config);
    println!("✓ Config still owned and valid: {:?}", config);

    println!("🎯 Advanced reference patterns and safety mastered!");
}"#,
                completion_indicators: vec![
                    "🛡️ LEVEL 7: Advanced References - Dangling Prevention",
                    "✓ Robot report created: ROBOT-STATUS-REPORT-2024",
                    "✓ Mission data processed: PROCESSED-SCAN-RESULTS-ALPHA",
                    "✓ Original data still valid: SCAN-RESULTS-ALPHA",
                    "✓ Robot in fleet: ALPHA-1",
                    "✓ Robot in fleet: BETA-2",
                    "✓ Robot in fleet: GAMMA-3",
                    "✓ Fleet expanded to 4 robots",
                    "✓ Fleet leader: ALPHA-1",
                    "✓ Command squad robot: ALPHA-1",
                    "✓ Command squad robot: BETA-2",
                    "✓ Robot name: OMEGA-COMMANDER (length: 15)",
                    "✓ Robot callsign: OMEGA-COMMANDER",
                    "✓ Config: RC-001 - 95% energy, status: ACTIVE",
                    "🎯 Advanced reference patterns and safety mastered!",
                ],
            },

            TaskTest {
                task_number: 4,
                task_name: "Lifetime Elision and Advanced Patterns",
                solution_code: r#"fn main() {
    println!("⚡ LEVEL 7: Lifetime Elision - Advanced Patterns");

    // Functions with lifetime elision (implicit lifetimes)
    fn get_robot_prefix(name: &str) -> &str {
        &name[0..5] // Returns a slice of the input
    }

    let robot_name = "ALPHA-COMMANDER-7";
    let prefix = get_robot_prefix(robot_name);
    println!("✓ Robot prefix: {}", prefix);

    // Method with implicit lifetime on &self
    struct Robot {
        name: String,
        energy: i32,
    }

    impl Robot {
        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_status(&self) -> String {
            format!("{} - Energy: {}%", self.name, self.energy)
        }
    }

    let robot = Robot {
        name: String::from("BETA-SCOUT"),
        energy: 88,
    };

    let robot_name_ref = robot.get_name();
    println!("✓ Robot name reference: {}", robot_name_ref);
    println!("✓ Robot status: {}", robot.get_status());

    // Advanced lifetime patterns with multiple parameters
    fn compare_missions<'a>(mission1: &'a str, mission2: &'a str) -> &'a str {
        if mission1.len() > mission2.len() {
            mission1
        } else {
            mission2
        }
    }

    let mission_alpha = "SCAN-PERIMETER";
    let mission_beta = "DEEP-EXPLORATION-SURVEY-BETA";
    let longer_mission = compare_missions(mission_alpha, mission_beta);
    println!("✓ Longer mission selected: {}", longer_mission);

    // Struct with lifetime parameters
    struct MissionReport<'a> {
        robot_name: &'a str,
        mission_type: &'a str,
        duration: u32,
    }

    impl<'a> MissionReport<'a> {
        fn summary(&self) -> String {
            format!("Robot {} completed {} mission in {}s",
                   self.robot_name, self.mission_type, self.duration)
        }
    }

    let robot_identifier = "GAMMA-RECON";
    let mission_classification = "STEALTH-PATROL";

    let report = MissionReport {
        robot_name: robot_identifier,
        mission_type: mission_classification,
        duration: 450,
    };

    println!("✓ Mission report: {}", report.summary());

    // Static lifetime demonstration
    const ROBOT_PROTOCOL: &'static str = "SECURE-COMM-V2";

    fn get_protocol() -> &'static str {
        ROBOT_PROTOCOL
    }

    println!("✓ Protocol in use: {}", get_protocol());

    // Lifetime bounds with generics
    fn process_robot_data<'a, T>(data: &'a T) -> &'a T
    where
        T: std::fmt::Display,
    {
        println!("✓ Processing data: {}", data);
        data
    }

    let robot_id = 42;
    let processed_id = process_robot_data(&robot_id);
    println!("✓ Processed robot ID: {}", processed_id);

    let status_code = String::from("OPERATIONAL");
    let processed_status = process_robot_data(&status_code);
    println!("✓ Processed status: {}", processed_status);

    println!("🎯 Lifetime elision and advanced patterns mastered!");
}"#,
                completion_indicators: vec![
                    "⚡ LEVEL 7: Lifetime Elision - Advanced Patterns",
                    "✓ Robot prefix: ALPHA",
                    "✓ Robot name reference: BETA-SCOUT",
                    "✓ Robot status: BETA-SCOUT - Energy: 88%",
                    "✓ Longer mission selected: DEEP-EXPLORATION-SURVEY-BETA",
                    "✓ Mission report: Robot GAMMA-RECON completed STEALTH-PATROL mission in 450s",
                    "✓ Protocol in use: SECURE-COMM-V2",
                    "✓ Processing data: 42",
                    "✓ Processed robot ID: 42",
                    "✓ Processing data: OPERATIONAL",
                    "✓ Processed status: OPERATIONAL",
                    "🎯 Lifetime elision and advanced patterns mastered!",
                ],
            },

            TaskTest {
                task_number: 5,
                task_name: "Complex Ownership Transfer and Smart Pointers",
                solution_code: r#"use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("🚀 LEVEL 7: Complex Ownership - Smart Pointers");

    // Reference counting with Rc
    let robot_blueprint = Rc::new(String::from("ADVANCED-COMBAT-UNIT"));
    let factory_ref = Rc::clone(&robot_blueprint);
    let assembly_ref = Rc::clone(&robot_blueprint);

    println!("✓ Blueprint shared across {} locations", Rc::strong_count(&robot_blueprint));
    println!("✓ Factory blueprint: {}", factory_ref);
    println!("✓ Assembly blueprint: {}", assembly_ref);

    // Interior mutability with RefCell
    let robot_state = Rc::new(RefCell::new(String::from("STANDBY")));
    let control_ref = Rc::clone(&robot_state);
    let monitor_ref = Rc::clone(&robot_state);

    // Modify through one reference
    {
        let mut state = control_ref.borrow_mut();
        *state = String::from("ACTIVE");
        println!("✓ Robot state updated via control system");
    }

    // Read through another reference
    {
        let state = monitor_ref.borrow();
        println!("✓ Monitor reports robot state: {}", *state);
    }

    // Box for heap allocation
    let large_sensor_data = Box::new(vec![1, 2, 3, 4, 5]);
    println!("✓ Sensor data stored on heap: {:?}", large_sensor_data);

    // Transfer ownership through Box
    fn process_sensor_data(data: Box<Vec<i32>>) -> Box<Vec<i32>> {
        println!("✓ Processing {} sensor readings", data.len());
        let mut processed = *data; // Unbox
        processed.push(6);
        Box::new(processed) // Re-box
    }

    let processed_data = process_sensor_data(large_sensor_data);
    println!("✓ Processed sensor data: {:?}", processed_data);

    // Advanced ownership patterns with functions
    fn create_robot_network() -> Vec<Rc<String>> {
        vec![
            Rc::new(String::from("ALPHA-NODE")),
            Rc::new(String::from("BETA-NODE")),
            Rc::new(String::from("GAMMA-NODE")),
        ]
    }

    let network_nodes = create_robot_network();
    for (i, node) in network_nodes.iter().enumerate() {
        println!("✓ Network node {}: {} (refs: {})",
                i + 1, node, Rc::strong_count(node));
    }

    // Shared ownership with modification
    let shared_mission_log = Rc::new(RefCell::new(Vec::new()));

    fn log_mission_event(log: Rc<RefCell<Vec<String>>>, event: String) {
        log.borrow_mut().push(event);
        println!("✓ Mission event logged: {}", log.borrow().last().unwrap());
    }

    log_mission_event(Rc::clone(&shared_mission_log), "MISSION-START".to_string());
    log_mission_event(Rc::clone(&shared_mission_log), "TARGET-ACQUIRED".to_string());
    log_mission_event(Rc::clone(&shared_mission_log), "SCAN-COMPLETE".to_string());

    println!("✓ Total mission events: {}", shared_mission_log.borrow().len());

    // Pattern: Factory with shared configuration
    let config = Rc::new(RefCell::new("STEALTH-MODE-V2".to_string()));

    fn create_robot_with_config(id: u32, config: Rc<RefCell<String>>) -> String {
        let config_value = config.borrow();
        format!("ROBOT-{}-{}", id, *config_value)
    }

    let robot_a = create_robot_with_config(1, Rc::clone(&config));
    let robot_b = create_robot_with_config(2, Rc::clone(&config));

    println!("✓ Created robot: {}", robot_a);
    println!("✓ Created robot: {}", robot_b);

    // Update shared config
    {
        let mut config_ref = config.borrow_mut();
        *config_ref = String::from("ASSAULT-MODE-V3");
    }

    let robot_c = create_robot_with_config(3, Rc::clone(&config));
    println!("✓ Created robot with updated config: {}", robot_c);

    println!("🎯 Complex ownership and smart pointers mastered!");
}"#,
                completion_indicators: vec![
                    "🚀 LEVEL 7: Complex Ownership - Smart Pointers",
                    "✓ Blueprint shared across 3 locations",
                    "✓ Factory blueprint: ADVANCED-COMBAT-UNIT",
                    "✓ Assembly blueprint: ADVANCED-COMBAT-UNIT",
                    "✓ Robot state updated via control system",
                    "✓ Monitor reports robot state: ACTIVE",
                    "✓ Sensor data stored on heap: [1, 2, 3, 4, 5]",
                    "✓ Processing 5 sensor readings",
                    "✓ Processed sensor data: [1, 2, 3, 4, 5, 6]",
                    "✓ Network node 1: ALPHA-NODE (refs: 1)",
                    "✓ Network node 2: BETA-NODE (refs: 1)",
                    "✓ Network node 3: GAMMA-NODE (refs: 1)",
                    "✓ Mission event logged: MISSION-START",
                    "✓ Mission event logged: TARGET-ACQUIRED",
                    "✓ Mission event logged: SCAN-COMPLETE",
                    "✓ Total mission events: 3",
                    "✓ Created robot: ROBOT-1-STEALTH-MODE-V2",
                    "✓ Created robot: ROBOT-2-STEALTH-MODE-V2",
                    "✓ Created robot with updated config: ROBOT-3-ASSAULT-MODE-V3",
                    "🎯 Complex ownership and smart pointers mastered!",
                ],
            },
        ],
    }
}