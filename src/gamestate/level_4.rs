use super::types::Game;

impl Game {
    // Level 4 specific tutorial system methods
    pub fn get_level_4_task_message(&self) -> String {
        if self.level_idx != 3 {
            return String::new(); // Only for level 4
        }

        match self.tutorial_state.current_task {
            0 => "📋 **TASK 1/5: Immutable Variable Bindings**\n\nLearn Rust's default immutability - variables can't be changed unless explicitly made mutable:\n\n```rust\nlet robot_name = \"Ferris\";\nlet robot_id = 12345;\nlet energy_level = 100;\n\nprintln!(\"Robot name: {}\", robot_name);\nprintln!(\"Robot ID: {}\", robot_id);\nprintln!(\"Energy level: {}\", energy_level);\n\n// This would cause an error:\n// robot_id = 54321; // Can't modify immutable variable!\n\nlet calculated_value = robot_id * 2;\nprintln!(\"Calculated value: {}\", calculated_value);\n```\n\n• Variables are **immutable by default** for safety\n• Use `let` to create immutable bindings\n• Can still use immutable variables in calculations".to_string(),

            1 => "📋 **TASK 2/5: Mutable Variable Bindings**\n\nWhen you need to change variables, use the `mut` keyword:\n\n```rust\nlet mut robot_position = 0;\nlet mut energy_level = 100;\nlet mut is_active = true;\n\nprintln!(\"Initial position: {}\", robot_position);\nprintln!(\"Initial energy: {}\", energy_level);\n\n// Now we can modify them!\nrobot_position += 5;\nenergy_level -= 10;\nis_active = false;\n\nprintln!(\"New position: {}\", robot_position);\nprintln!(\"New energy: {}\", energy_level);\n\n// Use in loops\nfor i in 1..=3 {\n    robot_position += i;\n    energy_level -= 5;\n    println!(\"Step {}: position = {}, energy = {}\", i, robot_position, energy_level);\n}\n```\n\n• Add `mut` after `let` to make variables changeable\n• Explicit mutability prevents accidental changes".to_string(),

            2 => "📋 **TASK 3/5: Variable Shadowing**\n\nShadowing lets you redefine variables with the same name, even changing their type:\n\n```rust\nlet robot_data = \"12345\";\nprintln!(\"Robot data as string: {}\", robot_data);\n\n// Shadow with a different type!\nlet robot_data: i32 = robot_data.parse().expect(\"Failed to parse\");\nprintln!(\"Robot data as number: {}\", robot_data);\n\n// Shadow again with calculation\nlet robot_data = robot_data * 2 + 100;\nprintln!(\"Robot data calculated: {}\", robot_data);\n\nlet value = 10;\nlet value = value + 5;  // Shadow with new calculation\nlet value = format!(\"The answer is {}\", value);  // Shadow with different type\nprintln!(\"Final value: {}\", value);\n```\n\n• Shadowing creates a new variable with the same name\n• Can change type when shadowing\n• Different from mutation - creates new binding".to_string(),

            3 => "📋 **TASK 4/5: Variable Scope and Blocks**\n\nVariables have scope - they only exist within their code block:\n\n```rust\nlet outer_variable = \"I'm in the outer scope\";\nprintln!(\"Outer scope: {}\", outer_variable);\n\n{\n    let inner_variable = \"I'm in the inner scope\";\n    println!(\"Inner scope: {}\", inner_variable);\n    \n    // Can access outer variables from inner scope\n    println!(\"Accessing outer from inner: {}\", outer_variable);\n    \n    // Can shadow outer variables\n    let outer_variable = \"I'm shadowing the outer variable\";\n    println!(\"Shadowed in inner: {}\", outer_variable);\n}\n\n// inner_variable is no longer accessible here!\nprintln!(\"Back to outer scope: {}\", outer_variable);\n\n// Functions have their own scope too\nfn calculate_something() -> i32 {\n    let local_value = 42;\n    local_value * 2\n}\n\nlet result = calculate_something();\nprintln!(\"Function result: {}\", result);\n```\n\n• Variables live within their `{ }` block\n• Inner scopes can access outer variables\n• Variables are dropped when leaving scope".to_string(),

            4 => "📋 **TASK 5/5: Constants and Naming Conventions**\n\nConstants are compile-time values that never change, with specific naming rules:\n\n```rust\n// Constants use SCREAMING_SNAKE_CASE\nconst MAX_ENERGY: i32 = 1000;\nconst ROBOT_NAME: &str = \"Ferris\";\nconst PI: f64 = 3.141592653589793;\n\nfn main() {\n    println!(\"Maximum energy: {}\", MAX_ENERGY);\n    println!(\"Robot name: {}\", ROBOT_NAME);\n    \n    // Variables use snake_case\n    let snake_case_variable = \"variables use snake_case\";\n    let another_example = 42;\n    \n    // Constants vs variables\n    let immutable_var = 100;           // Runtime value\n    const COMPILE_TIME: i32 = 50 + 50; // Compile-time constant\n    \n    println!(\"Variable: {}\", snake_case_variable);\n    println!(\"Compile-time constant: {}\", COMPILE_TIME);\n    \n    {\n        const BLOCK_CONSTANT: i32 = 999;\n        println!(\"Block constant: {}\", BLOCK_CONSTANT);\n    }\n}\n```\n\n• Constants: `const NAME: type = value;`\n• Variables: `let name = value;`\n• Constants must be compile-time computable".to_string(),

            _ => "🎉 **Level 4 Complete!**\n\nCongratulations! You've mastered Rust's variable binding system:\n• **Immutable by default** - variables can't change unless marked `mut`\n• **Explicit mutability** with `mut` keyword for safety\n• **Variable shadowing** for type transformation\n• **Scope rules** for memory management\n• **Constants vs variables** and naming conventions\n\nYou now understand Rust's memory safety philosophy: make dangerous operations explicit and prevent common bugs through the type system!\n\n🚀 Ready for Level 5: Type Casting and Conversions!".to_string(),
        }
    }

    pub fn check_level_4_progress(&mut self) {
        if self.level_idx != 3 || self.tutorial_state.current_task >= 5 {
            return; // Only for level 4 and if not completed
        }

        match self.tutorial_state.current_task {
            0 => {
                // Task 1: Immutable variable bindings
                if self.println_outputs.iter().any(|output|
                    output.contains("Robot name:") ||
                    output.contains("Robot ID:") ||
                    output.contains("Energy level:") ||
                    output.contains("Ferris") ||
                    output.contains("12345") ||
                    output.contains("Calculated value:")
                ) {
                    self.tutorial_state.task_completed[0] = true;
                    self.tutorial_state.current_task = 1;
                    println!("✅ Task 1 completed: Immutable variable bindings!");
                }
            },
            1 => {
                // Task 2: Mutable variable bindings
                if self.println_outputs.iter().any(|output|
                    output.contains("Initial position:") ||
                    output.contains("New position:") ||
                    output.contains("New energy:") ||
                    output.contains("Step") ||
                    (output.contains("position") && output.contains("energy"))
                ) {
                    self.tutorial_state.task_completed[1] = true;
                    self.tutorial_state.current_task = 2;
                    println!("✅ Task 2 completed: Mutable variable bindings!");
                }
            },
            2 => {
                // Task 3: Variable shadowing
                if self.println_outputs.iter().any(|output|
                    output.contains("Robot data as string:") ||
                    output.contains("Robot data as number:") ||
                    output.contains("Robot data calculated:") ||
                    output.contains("Final value:") ||
                    output.contains("shadowing") ||
                    output.contains("The answer is")
                ) {
                    self.tutorial_state.task_completed[2] = true;
                    self.tutorial_state.current_task = 3;
                    println!("✅ Task 3 completed: Variable shadowing!");
                }
            },
            3 => {
                // Task 4: Variable scope and blocks
                if self.println_outputs.iter().any(|output|
                    output.contains("Outer scope:") ||
                    output.contains("Inner scope:") ||
                    output.contains("Back to outer scope:") ||
                    output.contains("Function result:") ||
                    output.contains("outer scope") ||
                    output.contains("inner scope")
                ) {
                    self.tutorial_state.task_completed[3] = true;
                    self.tutorial_state.current_task = 4;
                    println!("✅ Task 4 completed: Variable scope and blocks!");
                }
            },
            4 => {
                // Task 5: Constants and naming conventions
                if self.println_outputs.iter().any(|output|
                    output.contains("Maximum energy:") ||
                    output.contains("Robot name:") ||
                    output.contains("Compile-time constant:") ||
                    output.contains("Block constant:") ||
                    output.contains("1000") ||
                    output.contains("snake_case") ||
                    output.contains("Variable:")
                ) {
                    self.tutorial_state.task_completed[4] = true;
                    self.tutorial_state.current_task = 5;
                    println!("✅ Task 5 completed: Constants and naming conventions!");
                }
            },
            _ => {}
        }
    }
}