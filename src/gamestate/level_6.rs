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

            2 => "📋 **TASK 3/5: References and Borrowing**\n\nUse references to borrow values without taking ownership:\n\n```rust\nfn calculate_length(s: &String) -> usize {\n    s.len()\n}\n\nfn change_string(s: &mut String) {\n    s.push_str(\", world!\");\n}\n\nfn main() {\n    // Immutable references (borrowing)\n    let s1 = String::from(\"hello\");\n    let len = calculate_length(&s1);\n    println!(\"The length of '{}' is {}\", s1, len);\n\n    // Mutable references\n    let mut s2 = String::from(\"hello\");\n    change_string(&mut s2);\n    println!(\"Changed string: {}\", s2);\n\n    // Multiple immutable references are allowed\n    let s3 = String::from(\"world\");\n    let r1 = &s3;\n    let r2 = &s3;\n    println!(\"r1: {}, r2: {}\", r1, r2);\n\n    // References to primitive types\n    let x = 5;\n    let r = &x;\n    println!(\"x: {}, reference to x: {}\", x, r);\n}\n```\n\n• **&** creates an immutable reference\n• **&mut** creates a mutable reference\n• **Borrowing** doesn't transfer ownership\n• **Multiple immutable** references allowed\n• **Only one mutable** reference at a time".to_string(),

            3 => "📋 **TASK 4/5: Ownership with Functions**\n\nMaster passing ownership and references to functions:\n\n```rust\nfn print_robot_info(name: &String, energy: &i32) {\n    println!(\"Robot {} has {} energy\", name, energy);\n}\n\nfn recharge_robot(energy: &mut i32) {\n    *energy += 25;\n    println!(\"Recharging... energy now: {}\", energy);\n}\n\nfn create_robot_data() -> (String, i32, bool) {\n    let name = String::from(\"Alpha\");\n    let energy = 75;\n    let active = true;\n    (name, energy, active)\n}\n\nfn main() {\n    let robot_name = String::from(\"Cybertron\");\n    let robot_energy = 100;\n\n    print_robot_info(&robot_name, &robot_energy);\n    // We can still use robot_name and robot_energy here\n    println!(\"Still accessible: {} with {} energy\", robot_name, robot_energy);\n\n    let (name, energy, active) = create_robot_data();\n    println!(\"Created robot: {} (energy: {}, active: {})\", name, energy, active);\n\n    let mut current_energy = 50;\n    recharge_robot(&mut current_energy);\n    println!(\"Final energy: {}\", current_energy);\n}\n```\n\n• **Borrow with &** to avoid moving\n• **Functions can modify** with &mut\n• **Return ownership** from functions\n• **Tuple returns** for multiple values".to_string(),

            4 => "📋 **TASK 5/5: Common Ownership Patterns**\n\nApply common patterns for working with ownership:\n\n```rust\nfn main() {\n    // Clone to avoid move when you need both values\n    let original = String::from(\"original\");\n    let cloned = original.clone();\n    println!(\"Original: {}, Cloned: {}\", original, cloned);\n\n    // Working with collections and ownership\n    let mut robot_names = Vec::new();\n    robot_names.push(String::from(\"Alpha\"));\n    robot_names.push(String::from(\"Beta\"));\n    robot_names.push(String::from(\"Gamma\"));\n\n    // Iterate over references to avoid moving\n    for name in &robot_names {\n        println!(\"Robot: {}\", name);\n    }\n\n    // We can still use robot_names here\n    println!(\"Total robots: {}\", robot_names.len());\n\n    // String slices (&str) don't own their data\n    let full_message = String::from(\"Hello, Rust ownership!\");\n    let slice = &full_message[0..5];\n    println!(\"Full message: {}\", full_message);\n    println!(\"Slice: {}\", slice);\n\n    // Function parameters with different ownership patterns\n    fn analyze_data(owned: String, borrowed: &str, mutable: &mut i32) {\n        println!(\"Owned: {}\", owned);\n        println!(\"Borrowed: {}\", borrowed);\n        *mutable += 10;\n    }\n\n    let owned_string = String::from(\"owned data\");\n    let borrowed_str = \"borrowed data\";\n    let mut mutable_int = 5;\n\n    analyze_data(owned_string, borrowed_str, &mut mutable_int);\n    println!(\"Modified mutable: {}\", mutable_int);\n}\n```\n\n• **Clone** when you need both values\n• **Iterate with &** to avoid moves\n• **String slices** (&str) for borrowing\n• **Mix owned and borrowed** parameters".to_string(),

            _ => "🎉 **Level 6 Complete!**\n\nExcellent! You've mastered Rust's ownership system - the foundation of memory safety:\n• **Three ownership rules** for memory management\n• **Move semantics** and when values become invalid\n• **References and borrowing** to use values without taking ownership\n• **Function ownership patterns** for flexible APIs\n• **Common ownership techniques** for real-world code\n\nYou now understand Rust's unique approach to memory safety without garbage collection! Ownership prevents data races, memory leaks, and use-after-free bugs at compile time.\n\n🚀 Ready for Level 7: Advanced Ownership and Lifetimes!".to_string(),
        }
    }

    pub fn check_level_6_progress(&mut self) {
        if self.level_idx != 5 || self.tutorial_state.current_task >= 5 {
            return; // Only for level 6 and if not completed
        }

        match self.tutorial_state.current_task {
            0 => {
                // Task 1: Basic ownership rules
                if self.println_outputs.iter().any(|output|
                    output.contains("Robot owner: Ferris") ||
                    output.contains("New owner: RustBot") ||
                    output.contains("Temporary robot: TempBot") ||
                    output.contains("x: 5, y: 5") ||
                    output.contains("ownership rules")
                ) {
                    self.tutorial_state.task_completed[0] = true;
                    self.tutorial_state.current_task = 1;
                    println!("✅ Task 1 completed: Basic ownership rules!");
                }
            },
            1 => {
                // Task 2: Move semantics
                if self.println_outputs.iter().any(|output|
                    output.contains("s2: hello") ||
                    output.contains("Function received: world") ||
                    output.contains("Received ownership: transferred") ||
                    output.contains("vec2: [1, 2, 3]") ||
                    output.contains("move semantics")
                ) {
                    self.tutorial_state.task_completed[1] = true;
                    self.tutorial_state.current_task = 2;
                    println!("✅ Task 2 completed: Move semantics!");
                }
            },
            2 => {
                // Task 3: References and borrowing
                if self.println_outputs.iter().any(|output|
                    output.contains("The length of 'hello' is 5") ||
                    output.contains("Changed string: hello, world!") ||
                    output.contains("r1: world, r2: world") ||
                    output.contains("reference to x: 5") ||
                    output.contains("borrowing")
                ) {
                    self.tutorial_state.task_completed[2] = true;
                    self.tutorial_state.current_task = 3;
                    println!("✅ Task 3 completed: References and borrowing!");
                }
            },
            3 => {
                // Task 4: Ownership with functions
                if self.println_outputs.iter().any(|output|
                    output.contains("Robot Cybertron has 100 energy") ||
                    output.contains("Still accessible: Cybertron with 100 energy") ||
                    output.contains("Created robot: Alpha") ||
                    output.contains("Recharging... energy now:") ||
                    output.contains("Final energy:")
                ) {
                    self.tutorial_state.task_completed[3] = true;
                    self.tutorial_state.current_task = 4;
                    println!("✅ Task 4 completed: Ownership with functions!");
                }
            },
            4 => {
                // Task 5: Common ownership patterns
                if self.println_outputs.iter().any(|output|
                    output.contains("Original: original, Cloned: original") ||
                    output.contains("Robot: Alpha") ||
                    output.contains("Total robots: 3") ||
                    output.contains("Full message: Hello, Rust ownership!") ||
                    output.contains("Slice: Hello") ||
                    output.contains("Modified mutable:")
                ) {
                    self.tutorial_state.task_completed[4] = true;
                    self.tutorial_state.current_task = 5;
                    println!("✅ Task 5 completed: Common ownership patterns!");
                    println!("🎉 Level 6 Complete! You've mastered Rust's ownership system!");
                }
            },
            _ => {}
        }
    }
}