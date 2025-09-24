// Level 6: Robot Ownership Systems - Automated Test Solutions

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_6_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 6: Robot Ownership Systems",
        level_index: 5,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Robot Registration and Transfer Protocol",
                solution_code: r#"fn main() {
    println!("🤖 LEVEL 6: Robot Ownership Systems - Registration Protocol");

    // Each robot has exactly one owner (Rule 1)
    let robot_ferris = String::from("FERRIS-2024");
    println!("✓ Robot {} registered to Command Center", robot_ferris);

    // Transfer ownership to Field Operations (Rule 2)
    let original_registration = String::from("ALPHA-UNIT-7");
    let field_assignment = original_registration; // Ownership transferred!

    // original_registration is now invalid - robot can't have two owners
    println!("✓ Robot {} assigned to Field Operations", field_assignment);

    // Temporary robot deployment (Rule 3 - scope-based cleanup)
    {
        let scout_bot = String::from("SCOUT-TEMP-1");
        println!("✓ Temporary scout {} deployed for mission", scout_bot);
    } // Scout bot automatically decommissioned when mission ends

    // Basic robot IDs don't transfer ownership (Copy types)
    let robot_id = 42;
    let backup_id = robot_id; // Copy, not transfer
    println!("✓ Robot ID {} logged, backup ID {} stored", robot_id, backup_id);

    // Memory allocation demonstration
    let stack_data = 100; // Robot energy stored on stack
    let heap_data = String::from("Mission Data Alpha"); // Mission data on heap

    println!("✓ Energy level: {}", stack_data);
    println!("✓ Mission payload: {}", heap_data);

    println!("🎯 Robot registration protocol completed!");
}"#,
                completion_indicators: vec![
                    "🤖 LEVEL 6: Robot Ownership Systems - Registration Protocol",
                    "✓ Robot FERRIS-2024 registered to Command Center",
                    "✓ Robot ALPHA-UNIT-7 assigned to Field Operations",
                    "✓ Temporary scout SCOUT-TEMP-1 deployed for mission",
                    "✓ Robot ID 42 logged, backup ID 42 stored",
                    "✓ Energy level: 100",
                    "✓ Mission payload: Mission Data Alpha",
                    "🎯 Robot registration protocol completed!",
                ],
            },

            TaskTest {
                task_number: 2,
                task_name: "Mission Handoff and Resource Transfer",
                solution_code: r#"fn main() {
    println!("🔄 Mission Handoff Protocol - Resource Transfer Systems");

    // Mission data transfer with String
    let mission_briefing = String::from("Sector-7-Recon");
    let active_mission = mission_briefing; // Mission transferred to field team

    println!("✓ Active mission: {}", active_mission);
    // mission_briefing is no longer valid - mission can only have one handler

    // Command function that takes ownership of robot
    fn deploy_robot(robot_name: String) {
        println!("🚀 Deploying {} to field operations", robot_name);
    } // robot_name automatically cleaned up after deployment

    let beta_robot = String::from("BETA-EXPLORER");
    deploy_robot(beta_robot);
    // beta_robot is no longer accessible - fully deployed to field

    // Mission factory that creates and returns new missions
    fn generate_mission() -> String {
        String::from("Deep-Cave-Survey")
    }

    let new_mission = generate_mission();
    println!("✓ New mission generated: {}", new_mission);

    // Transfer robot fleet data
    let robot_fleet = vec!["GAMMA-1", "GAMMA-2", "GAMMA-3"];
    let field_fleet = robot_fleet; // Entire fleet transferred
    println!("✓ Fleet deployed: {:?}", field_fleet);

    // Mission completion confirmation
    fn complete_mission(mission: String) -> String {
        println!("📋 Processing mission: {}", mission);
        mission // Return mission data for archival
    }

    let archive_mission = String::from("Alpha-Site-Survey");
    let completed = complete_mission(archive_mission);
    println!("✅ Mission archived: {}", completed);

    println!("🎯 Mission handoff protocol completed!");
}"#,
                completion_indicators: vec![
                    "🔄 Mission Handoff Protocol - Resource Transfer Systems",
                    "✓ Active mission: Sector-7-Recon",
                    "🚀 Deploying BETA-EXPLORER to field operations",
                    "✓ New mission generated: Deep-Cave-Survey",
                    "✓ Fleet deployed: [\"GAMMA-1\", \"GAMMA-2\", \"GAMMA-3\"]",
                    "📋 Processing mission: Alpha-Site-Survey",
                    "✅ Mission archived: Alpha-Site-Survey",
                    "🎯 Mission handoff protocol completed!",
                ],
            },

            TaskTest {
                task_number: 3,
                task_name: "Shared Resources and Robot Communication",
                solution_code: r#"fn calculate_distance(robot_pos: &String) -> usize {
    robot_pos.len()
}

fn update_robot_status(status: &mut String) {
    status.push_str("-UPDATED");
}

fn main() {
    println!("📡 Robot Communication Network - Shared Resource Access");

    // Shared access to robot position data (borrowing)
    let robot_position = String::from("SECTOR-7-GRID-A5");
    let distance_calc = calculate_distance(&robot_position);
    println!("✓ Robot position: {}", robot_position);
    println!("✓ Distance calculation: {} units", distance_calc);

    // Mutable sharing for status updates
    let mut robot_status = String::from("OPERATIONAL");
    update_robot_status(&mut robot_status);
    println!("✓ Updated robot status: {}", robot_status);

    // Multiple read-only access to mission data
    let mission_data = String::from("Cave-Exploration-Alpha");
    let primary_reader = &mission_data;
    let backup_reader = &mission_data;
    println!("✓ Primary mission access: {}", primary_reader);
    println!("✓ Backup mission access: {}", backup_reader);

    // Shared sensor readings
    let sensor_value = 85;
    let sensor_ref = &sensor_value;
    println!("✓ Sensor reading: {}", sensor_value);
    println!("✓ Transmitted value: {}", sensor_ref);

    // Temporary data sharing in subsystem scopes
    {
        let subsystem_data = String::from("Navigation-Module");
        let data_link = &subsystem_data;
        println!("✓ Subsystem online: {}", data_link);
    } // Subsystem and data_link automatically cleaned up

    // Robot fleet coordination
    fn coordinate_robots(robot1: &str, robot2: &str) -> bool {
        println!("🤝 Coordinating {} and {}", robot1, robot2);
        robot1.len() + robot2.len() > 10
    }

    let alpha = String::from("ALPHA-SCOUT");
    let beta = String::from("BETA-MINER");
    let coordination_success = coordinate_robots(&alpha, &beta);

    // Both robots still accessible after coordination
    println!("✓ {} status: Ready", alpha);
    println!("✓ {} status: Ready", beta);
    println!("✓ Coordination success: {}", coordination_success);

    println!("🎯 Robot communication network established!");
}"#,
                completion_indicators: vec![
                    "📡 Robot Communication Network - Shared Resource Access",
                    "✓ Robot position: SECTOR-7-GRID-A5",
                    "✓ Distance calculation: 17 units",
                    "✓ Updated robot status: OPERATIONAL-UPDATED",
                    "✓ Primary mission access: Cave-Exploration-Alpha",
                    "✓ Backup mission access: Cave-Exploration-Alpha",
                    "✓ Sensor reading: 85",
                    "✓ Transmitted value: 85",
                    "✓ Subsystem online: Navigation-Module",
                    "🤝 Coordinating ALPHA-SCOUT and BETA-MINER",
                    "✓ ALPHA-SCOUT status: Ready",
                    "✓ BETA-MINER status: Ready",
                    "✓ Coordination success: true",
                    "🎯 Robot communication network established!",
                ],
            },

            TaskTest {
                task_number: 4,
                task_name: "Command Structure and Robot Deployment",
                solution_code: r#"fn assign_mission(robot_name: &String, energy: &i32) {
    println!("📋 Assigning mission to {} (Energy: {}%)", robot_name, energy);
}

fn recharge_robot(energy: &mut i32) {
    *energy += 25;
    println!("🔋 Robot recharged! Energy now: {}%", energy);
}

fn create_robot_squad() -> (String, i32, bool) {
    let squad_name = String::from("Alpha-Squad");
    let squad_size = 4;
    let is_active = true;
    (squad_name, squad_size, is_active)
}

fn maybe_deploy_robot(should_deploy: bool) -> Option<String> {
    if should_deploy {
        Some(String::from("DELTA-RECON"))
    } else {
        None
    }
}

fn main() {
    println!("⚡ Command & Control - Robot Deployment Operations");

    // Command hierarchy - sharing robot data without transferring ownership
    let commander_robot = String::from("COMMANDER-PRIME");
    let robot_energy = 75;

    assign_mission(&commander_robot, &robot_energy);
    // Robot still under command control after mission assignment
    println!("✓ {} remains under command control", commander_robot);
    println!("✓ Current energy status: {}%", robot_energy);

    // Squad creation with multiple return values
    let (squad_name, squad_size, squad_active) = create_robot_squad();
    println!("✓ Created squad: {} with {} members (Active: {})", squad_name, squad_size, squad_active);

    // Energy management with mutable references
    let mut field_robot_energy = 40;
    println!("✓ Field robot initial energy: {}%", field_robot_energy);
    recharge_robot(&mut field_robot_energy);
    println!("✓ Field robot final energy: {}%", field_robot_energy);

    // Conditional robot deployment
    match maybe_deploy_robot(true) {
        Some(robot) => println!("🚀 Successfully deployed: {}", robot),
        None => println!("❌ No robot available for deployment"),
    }

    match maybe_deploy_robot(false) {
        Some(robot) => println!("🚀 Successfully deployed: {}", robot),
        None => println!("⏳ Deployment cancelled - robots on standby"),
    }

    // Multi-robot coordination system
    fn coordinate_mission(leader: &String, support: &String, objective: &String) {
        println!("🎯 Mission coordination:");
        println!("   Leader: {}", leader);
        println!("   Support: {}", support);
        println!("   Objective: {}", objective);
    }

    let leader_bot = String::from("ALPHA-LEAD");
    let support_bot = String::from("BETA-SUPPORT");
    let mission_obj = String::from("Secure-Perimeter");

    coordinate_mission(&leader_bot, &support_bot, &mission_obj);

    // All robots still accessible for further commands
    println!("✓ Command retains control of all deployed units");
    println!("   - {}: Ready for orders", leader_bot);
    println!("   - {}: Awaiting instructions", support_bot);

    println!("🎯 Command structure operational!");
}"#,
                completion_indicators: vec![
                    "⚡ Command & Control - Robot Deployment Operations",
                    "📋 Assigning mission to COMMANDER-PRIME (Energy: 75%)",
                    "✓ COMMANDER-PRIME remains under command control",
                    "✓ Current energy status: 75%",
                    "✓ Created squad: Alpha-Squad with 4 members (Active: true)",
                    "✓ Field robot initial energy: 40%",
                    "🔋 Robot recharged! Energy now: 65%",
                    "✓ Field robot final energy: 65%",
                    "🚀 Successfully deployed: DELTA-RECON",
                    "⏳ Deployment cancelled - robots on standby",
                    "🎯 Mission coordination:",
                    "   Leader: ALPHA-LEAD",
                    "   Support: BETA-SUPPORT",
                    "   Objective: Secure-Perimeter",
                    "✓ Command retains control of all deployed units",
                    "   - ALPHA-LEAD: Ready for orders",
                    "   - BETA-SUPPORT: Awaiting instructions",
                    "🎯 Command structure operational!",
                ],
            },

            TaskTest {
                task_number: 5,
                task_name: "Advanced Robot Fleet Management",
                solution_code: r#"fn main() {
    println!("🌟 Advanced Fleet Management - Ownership Strategies");

    // Fleet duplication strategy when you need both original and copy
    let master_fleet_id = String::from("FLEET-OMEGA-7");
    let backup_fleet_id = master_fleet_id.clone();
    println!("✓ Master Fleet: {}", master_fleet_id);
    println!("✓ Backup Registry: {}", backup_fleet_id);

    // Robot inventory management with collections
    let mut robot_inventory = Vec::new();
    robot_inventory.push(String::from("MINING-BOT-A"));
    robot_inventory.push(String::from("SCOUT-BOT-B"));
    robot_inventory.push(String::from("REPAIR-BOT-C"));

    // Iterate over references to avoid moving robots
    println!("📊 Current robot inventory:");
    for robot in &robot_inventory {
        println!("   🤖 {}", robot);
    }

    // Inventory still accessible after iteration
    println!("✓ Total robots in inventory: {}", robot_inventory.len());

    // Mission data slicing without ownership transfer
    let full_mission_log = String::from("2024-Mission-Deep-Cave-Exploration-Alpha-Squad");
    let mission_year = &full_mission_log[0..4];
    let mission_type = &full_mission_log[13..22];
    println!("✓ Full mission log: {}", full_mission_log);
    println!("✓ Mission year: {}", mission_year);
    println!("✓ Mission type: {}", mission_type);

    // Advanced deployment patterns with mixed ownership
    fn process_deployment(owned_robot: String, borrowed_mission: &str, shared_energy: &mut i32) {
        println!("🚀 Deploying {} for mission: {}", owned_robot, borrowed_mission);
        println!("📊 Processing robot data: {}", borrowed_mission);
        *shared_energy -= 10;
        println!("⚡ Energy consumed: {} remaining", shared_energy);
    }

    let deployment_robot = String::from("GAMMA-EXPLORER");
    let mission_briefing = "Cave-Survey-Delta";
    let mut shared_energy = 95;

    process_deployment(deployment_robot, mission_briefing, &mut shared_energy);
    // deployment_robot is now owned by the function
    println!("✓ Mission briefing still available: {}", mission_briefing);
    println!("✓ Shared energy updated: {}", shared_energy);

    // Multi-level robot command chain
    fn analyze_robot_data(data: &Vec<String>) -> Vec<String> {
        let mut analysis = Vec::new();
        for robot in data {
            analysis.push(format!("{}-ANALYZED", robot));
        }
        analysis
    }

    let robot_data = vec![
        String::from("THETA-1"),
        String::from("THETA-2"),
        String::from("THETA-3")
    ];

    let analysis_results = analyze_robot_data(&robot_data);

    println!("📈 Analysis complete:");
    for (original, analyzed) in robot_data.iter().zip(analysis_results.iter()) {
        println!("   {} -> {}", original, analyzed);
    }

    // Fleet coordination with advanced patterns
    fn coordinate_complex_mission() -> Result<String, String> {
        let mission_success = true;
        if mission_success {
            Ok(String::from("MISSION-ALPHA-SUCCESS"))
        } else {
            Err(String::from("MISSION-COORDINATION-FAILED"))
        }
    }

    match coordinate_complex_mission() {
        Ok(success_msg) => println!("🎉 {}", success_msg),
        Err(error_msg) => println!("⚠️ {}", error_msg),
    }

    println!("🎯 Advanced fleet management systems online!");
}"#,
                completion_indicators: vec![
                    "🌟 Advanced Fleet Management - Ownership Strategies",
                    "✓ Master Fleet: FLEET-OMEGA-7",
                    "✓ Backup Registry: FLEET-OMEGA-7",
                    "📊 Current robot inventory:",
                    "   🤖 MINING-BOT-A",
                    "   🤖 SCOUT-BOT-B",
                    "   🤖 REPAIR-BOT-C",
                    "✓ Total robots in inventory: 3",
                    "✓ Full mission log: 2024-Mission-Deep-Cave-Exploration-Alpha-Squad",
                    "✓ Mission year: 2024",
                    "✓ Mission type: Deep-Cave",
                    "🚀 Deploying GAMMA-EXPLORER for mission: Cave-Survey-Delta",
                    "📊 Processing robot data: Cave-Survey-Delta",
                    "⚡ Energy consumed: 85 remaining",
                    "✓ Mission briefing still available: Cave-Survey-Delta",
                    "✓ Shared energy updated: 85",
                    "📈 Analysis complete:",
                    "   THETA-1 -> THETA-1-ANALYZED",
                    "   THETA-2 -> THETA-2-ANALYZED",
                    "   THETA-3 -> THETA-3-ANALYZED",
                    "🎉 MISSION-ALPHA-SUCCESS",
                    "🎯 Advanced fleet management systems online!",
                ],
            },
        ],
    }
}