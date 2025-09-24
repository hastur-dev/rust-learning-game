use super::types::Game;

impl Game {
    // Level 6 specific tutorial system methods
    pub fn get_level_6_task_message(&self) -> String {
        if self.level_idx != 5 {
            return String::new(); // Only for level 6
        }

        match self.tutorial_state.current_task {
            0 => "🤖 **TASK 1/5: Robot Registration and Transfer Protocol**\n\nWelcome to the Command Center! Learn how robot ownership prevents conflicts in our fleet management system:\n\n```rust\nfn main() {\n    // Each robot has exactly one owner (Rule 1)\n    let robot_ferris = String::from(\"FERRIS-2024\");\n    println!(\"✓ Robot {} registered to Command Center\", robot_ferris);\n\n    // Transfer ownership to Field Operations (Rule 2)\n    let original_registration = String::from(\"ALPHA-UNIT-7\");\n    let field_assignment = original_registration; // Ownership transferred!\n    \n    // original_registration is now invalid - no dual control!\n    println!(\"✓ Robot {} assigned to Field Operations\", field_assignment);\n\n    // Temporary robot deployment (Rule 3)\n    {\n        let scout_bot = String::from(\"SCOUT-TEMP-1\");\n        println!(\"✓ Temporary scout {} deployed\", scout_bot);\n    } // Scout automatically decommissioned\n\n    // Robot IDs are copied, not transferred\n    let robot_id = 42;\n    let backup_id = robot_id; // Copy for redundancy\n    println!(\"✓ Robot ID {} logged, backup {} stored\", robot_id, backup_id);\n}\n```\n\n🎯 **Mission**: Implement secure robot registration that prevents dual ownership conflicts!\n• **Rule 1**: Each robot has exactly one commander\n• **Rule 2**: Only one active assignment at a time\n• **Rule 3**: Auto-cleanup when mission ends".to_string(),

            1 => "🔄 **TASK 2/5: Mission Handoff and Resource Transfer**\n\nLearn how mission data transfers between robot command systems:\n\n```rust\nfn main() {\n    // Mission briefing transfer\n    let mission_briefing = String::from(\"Sector-7-Recon\");\n    let active_mission = mission_briefing; // Mission transferred to field team\n    println!(\"Active mission: {}\", active_mission);\n    // mission_briefing is no longer valid - mission can only have one handler\n\n    // Command function that takes ownership of robot\n    fn deploy_robot(robot_name: String) {\n        println!(\"Deploying {} to field operations\", robot_name);\n    } // robot_name automatically cleaned up after deployment\n\n    let beta_robot = String::from(\"BETA-EXPLORER\");\n    deploy_robot(beta_robot);\n    // beta_robot is no longer accessible - fully deployed to field\n\n    // Mission factory that creates and returns new missions\n    fn generate_mission() -> String {\n        String::from(\"Deep-Cave-Survey\")\n    }\n\n    let new_mission = generate_mission();\n    println!(\"New mission generated: {}\", new_mission);\n\n    // Robot fleet transfer\n    let robot_fleet = vec![\"GAMMA-1\", \"GAMMA-2\", \"GAMMA-3\"];\n    let field_fleet = robot_fleet; // Entire fleet transferred\n    println!(\"Fleet deployed: {:?}\", field_fleet);\n}\n```\n\n🎯 **Mission**: Master resource handoff protocols!\n• **Mission transfers** happen with assignment\n• **Functions deploy** robots by taking ownership\n• **Mission generators** return new assignments\n• **Fleet data** moves as complete units".to_string(),

            2 => "📡 **TASK 3/5: Shared Resources and Robot Communication**\n\nEstablish communication networks where multiple systems can access robot data:\n\n```rust\nfn calculate_distance(robot_pos: &String) -> usize {\n    robot_pos.len()\n}\n\nfn update_robot_status(status: &mut String) {\n    status.push_str(\"-UPDATED\");\n}\n\nfn main() {\n    // Shared access to robot position data (borrowing)\n    let robot_position = String::from(\"SECTOR-7-GRID-A5\");\n    let distance_calc = calculate_distance(&robot_position);\n    println!(\"Robot position: {}\", robot_position);\n    println!(\"Distance calculation: {} units\", distance_calc);\n\n    // Mutable sharing for status updates\n    let mut robot_status = String::from(\"OPERATIONAL\");\n    update_robot_status(&mut robot_status);\n    println!(\"Updated robot status: {}\", robot_status);\n\n    // Multiple read-only access to mission data\n    let mission_data = String::from(\"Cave-Exploration-Alpha\");\n    let primary_reader = &mission_data;\n    let backup_reader = &mission_data;\n    println!(\"Primary mission access: {}\", primary_reader);\n    println!(\"Backup mission access: {}\", backup_reader);\n\n    // Shared sensor readings\n    let sensor_value = 85;\n    let sensor_ref = &sensor_value;\n    println!(\"Sensor reading: {}\", sensor_value);\n    println!(\"Transmitted value: {}\", sensor_ref);\n}\n```\n\n🎯 **Mission**: Build secure communication networks!\n• **& borrows** data without taking control\n• **&mut allows** status updates\n• **Multiple readers** can access data simultaneously\n• **Sensor data** can be shared safely".to_string(),

            3 => "⚡ **TASK 4/5: Command Structure and Robot Deployment**\n\nMaster command hierarchy systems where functions coordinate robot operations:\n\n```rust\nfn assign_mission(robot_name: &String, energy: &i32) {\n    println!(\"Assigning mission to {} (Energy: {}%)\", robot_name, energy);\n}\n\nfn recharge_robot(energy: &mut i32) {\n    *energy += 25;\n    println!(\"Robot recharged! Energy now: {}%\", energy);\n}\n\nfn create_robot_squad() -> (String, i32, bool) {\n    let squad_name = String::from(\"Alpha-Squad\");\n    let squad_size = 4;\n    let is_active = true;\n    (squad_name, squad_size, is_active)\n}\n\nfn main() {\n    let commander_robot = String::from(\"COMMANDER-PRIME\");\n    let robot_energy = 75;\n\n    assign_mission(&commander_robot, &robot_energy);\n    // Robot still under command control after mission assignment\n    println!(\"{} remains under command control\", commander_robot);\n    println!(\"Current energy status: {}%\", robot_energy);\n\n    let (squad_name, squad_size, squad_active) = create_robot_squad();\n    println!(\"Created squad: {} with {} members (Active: {})\", squad_name, squad_size, squad_active);\n\n    let mut field_robot_energy = 40;\n    recharge_robot(&mut field_robot_energy);\n    println!(\"Field robot final energy: {}%\", field_robot_energy);\n}\n```\n\n🎯 **Mission**: Build efficient command structures!\n• **Borrow data** for mission assignments\n• **Functions modify** energy levels with &mut\n• **Squad creation** returns multiple values\n• **Command retains** control of deployed units".to_string(),

            4 => "🌟 **TASK 5/5: Advanced Fleet Management Strategies**\n\nImplement sophisticated robot fleet management using advanced ownership patterns:\n\n```rust\nfn main() {\n    // Fleet duplication strategy when you need both original and copy\n    let master_fleet_id = String::from(\"FLEET-OMEGA-7\");\n    let backup_fleet_id = master_fleet_id.clone();\n    println!(\"Master Fleet: {}\", master_fleet_id);\n    println!(\"Backup Registry: {}\", backup_fleet_id);\n\n    // Robot inventory management with collections\n    let mut robot_inventory = Vec::new();\n    robot_inventory.push(String::from(\"MINING-BOT-A\"));\n    robot_inventory.push(String::from(\"SCOUT-BOT-B\"));\n    robot_inventory.push(String::from(\"REPAIR-BOT-C\"));\n\n    // Iterate over references to avoid moving robots\n    for robot in &robot_inventory {\n        println!(\"🤖 {}\", robot);\n    }\n\n    // Inventory still accessible after iteration\n    println!(\"Total robots in inventory: {}\", robot_inventory.len());\n\n    // Mission data slicing without ownership transfer\n    let full_mission_log = String::from(\"2024-Mission-Deep-Cave-Exploration-Alpha-Squad\");\n    let mission_year = &full_mission_log[0..4];\n    let mission_type = &full_mission_log[13..22];\n    println!(\"Full mission log: {}\", full_mission_log);\n    println!(\"Mission year: {}\", mission_year);\n\n    // Advanced deployment patterns with mixed ownership\n    fn process_deployment(owned_robot: String, borrowed_mission: &str, shared_energy: &mut i32) {\n        println!(\"Deploying {} for mission: {}\", owned_robot, borrowed_mission);\n        *shared_energy -= 10;\n    }\n\n    let deployment_robot = String::from(\"GAMMA-EXPLORER\");\n    let mission_briefing = \"Cave-Survey-Delta\";\n    let mut shared_energy = 95;\n\n    process_deployment(deployment_robot, mission_briefing, &mut shared_energy);\n    println!(\"Mission briefing still available: {}\", mission_briefing);\n    println!(\"Shared energy updated: {}\", shared_energy);\n}\n```\n\n🎯 **Mission**: Master advanced fleet management!\n• **Clone fleets** when you need duplicates\n• **Iterate with &** to preserve inventory\n• **String slices** for mission data access\n• **Mix ownership patterns** for complex operations".to_string(),

            _ => "🎉 **Level 6 Complete!**\n\nExcellent! You've mastered Robot Ownership Systems - the foundation of safe fleet management:\n• **Robot Registration Protocol** - Single ownership prevents conflicts\n• **Mission Handoff Systems** - Resource transfer and deployment\n• **Communication Networks** - Shared access through borrowing\n• **Command Structures** - Coordinated robot operations\n• **Advanced Fleet Management** - Complex ownership strategies\n\nYou now understand Rust's unique approach to memory safety through ownership! Your robot fleet is secure from data races, memory leaks, and use-after-free bugs at compile time.\n\n🚀 Ready for Level 7: Advanced Robot Systems and Lifetimes!".to_string(),
        }
    }

    pub fn check_level_6_progress(&mut self) {
        if self.level_idx != 5 || self.tutorial_state.current_task >= 5 {
            return; // Only for level 6 and if not completed
        }

        match self.tutorial_state.current_task {
            0 => {
                // Task 1: Robot Registration and Transfer Protocol
                if self.println_outputs.iter().any(|output|
                    output.contains("Robot FERRIS-2024 registered to Command Center") ||
                    output.contains("Robot ALPHA-UNIT-7 assigned to Field Operations") ||
                    output.contains("Temporary scout SCOUT-TEMP-1 deployed") ||
                    output.contains("Robot ID 42 logged, backup ID 42 stored") ||
                    output.contains("registration protocol") ||
                    output.contains("Robot Registration and Transfer Protocol")
                ) {
                    self.tutorial_state.task_completed[0] = true;
                    self.tutorial_state.current_task = 1;
                    println!("✅ Task 1 completed: Robot Registration and Transfer Protocol!");
                }
            },
            1 => {
                // Task 2: Mission Handoff and Resource Transfer
                if self.println_outputs.iter().any(|output|
                    output.contains("Active mission: Sector-7-Recon") ||
                    output.contains("Deploying BETA-EXPLORER to field operations") ||
                    output.contains("New mission generated: Deep-Cave-Survey") ||
                    output.contains("Fleet deployed: [\"GAMMA-1\", \"GAMMA-2\", \"GAMMA-3\"]") ||
                    output.contains("Mission Handoff Protocol") ||
                    output.contains("handoff protocol")
                ) {
                    self.tutorial_state.task_completed[1] = true;
                    self.tutorial_state.current_task = 2;
                    println!("✅ Task 2 completed: Mission Handoff and Resource Transfer!");
                }
            },
            2 => {
                // Task 3: Shared Resources and Robot Communication
                if self.println_outputs.iter().any(|output|
                    output.contains("Robot position: SECTOR-7-GRID-A5") ||
                    output.contains("Distance calculation: 17 units") ||
                    output.contains("Updated robot status: OPERATIONAL-UPDATED") ||
                    output.contains("Primary mission access: Cave-Exploration-Alpha") ||
                    output.contains("Backup mission access: Cave-Exploration-Alpha") ||
                    output.contains("communication network") ||
                    output.contains("Robot Communication Network")
                ) {
                    self.tutorial_state.task_completed[2] = true;
                    self.tutorial_state.current_task = 3;
                    println!("✅ Task 3 completed: Shared Resources and Robot Communication!");
                }
            },
            3 => {
                // Task 4: Command Structure and Robot Deployment
                if self.println_outputs.iter().any(|output|
                    output.contains("Assigning mission to COMMANDER-PRIME (Energy: 75%)") ||
                    output.contains("COMMANDER-PRIME remains under command control") ||
                    output.contains("Created squad: Alpha-Squad with 4 members") ||
                    output.contains("Robot recharged! Energy now:") ||
                    output.contains("Field robot final energy:") ||
                    output.contains("Command Structure") ||
                    output.contains("command structure")
                ) {
                    self.tutorial_state.task_completed[3] = true;
                    self.tutorial_state.current_task = 4;
                    println!("✅ Task 4 completed: Command Structure and Robot Deployment!");
                }
            },
            4 => {
                // Task 5: Advanced Fleet Management Strategies
                if self.println_outputs.iter().any(|output|
                    output.contains("Master Fleet: FLEET-OMEGA-7") ||
                    output.contains("Backup Registry: FLEET-OMEGA-7") ||
                    output.contains("🤖 MINING-BOT-A") ||
                    output.contains("🤖 SCOUT-BOT-B") ||
                    output.contains("Total robots in inventory: 3") ||
                    output.contains("Mission year: 2024") ||
                    output.contains("Deploying GAMMA-EXPLORER for mission") ||
                    output.contains("Advanced Fleet Management") ||
                    output.contains("fleet management")
                ) {
                    self.tutorial_state.task_completed[4] = true;
                    self.tutorial_state.current_task = 5;
                    println!("✅ Task 5 completed: Advanced Fleet Management Strategies!");
                    println!("🎉 Level 6 Complete! You've mastered Robot Ownership Systems!");
                }
            },
            _ => {}
        }
    }
}