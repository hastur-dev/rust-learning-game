use super::types::Game;

impl Game {
    // Tutorial system methods
    pub fn get_tutorial_task_message(&self) -> String {
        if self.level_idx != 0 {
            return String::new(); // Only for level 1
        }
        
        match self.tutorial_state.current_task {
            0 => "Task 1/5: Now we're going to learn about functions.\n these are things to store your code inside of that you might have already seen with the fn main()\n these functions allow you to store your code into an easily reachable place so that they can be used later. So lets make a function and call it inside of main\n fn move_5_times() {\n for i in 0..5 {\n move_bot(\"down\")n\ }\n}".to_string(),
            1 => "Task 2/5: Blockers\n fn move_5_times() {\n for i in 0..5 {\n scan("down")\n if scan == obstacle {\n move_bot(\"right\")\n }\n move_bot(\"down\")n\ }\n}".to_string(),
            2 => "Task 3/5: Structs\n ".to_string(),
            3 => "Task 4/5: Mutable Variables and Scan Function\n\nAwesome! Let's learn about mutable variables by using the scan function. \n variables by themselves have to be defined in the code, but mutable variables don't basically if you have a user input or a message then you want to make that a mutable variable.\n this will tell rust that your variable exists, but you don't know what it is yet.\n\nlet mut scan_result = scan(\"right\");\nprintln!(\"Scan found: {}\", scan_result);\n\nThe 'mut' keyword lets us change variable values.".to_string(),
            4 => "Task 5/5: Data Types and Movement\n\nPerfect! Now let's learn about the u32 integer type and data types in general. \n sometimes we want to make sure that a variable is something specific by design, so we have data types to define what that specific thing is. \n learn more about this at the rust website by hitting CTRL+SHIFT+B to open your web browser to teh documentation for this language \n now lets learn it by using it for movement:\nlet steps: u32 = 3;\nfor _i in 0..steps {\n    move_bot(\"right\");\n}\n\nu32 is an unsigned 32-bit integer (0 to 4,294,967,295).".to_string(),
            _ => "Congratulations! You've correctly gone through the first few steps of learning the rust programming language!\n Next we'll teach you more about functions and loops\n Continue onwards by hitting CTRL+SHIFT+N to start the next level".to_string(),
        }
    }
    
    pub fn check_tutorial_progress(&mut self) {
        if self.level_idx != 1 || self.tutorial_state.current_task >= 5 {
            return; // Only for level 2 and if not completed
        }
        
        match self.tutorial_state.current_task {
            0 => {
                // Task 1: Any println output completes
                // Debug: Show what println outputs we have (commented out)
                // if self.level_idx == 0 {
                //     let debug_msg = format!("DEBUG: println_outputs = {:?}, task_completed[0] = {}", 
                //                           self.println_outputs, self.tutorial_state.task_completed[0]);
                //     self.execution_result = debug_msg;
                // }
                
                if !self.println_outputs.is_empty() && !self.tutorial_state.task_completed[0] {
                    self.tutorial_state.task_completed[0] = true;
                    self.tutorial_state.current_task = 1;
                    self.popup_system.show_message(
                        "Task 1 Complete! ✓".to_string(),
                        "Great! You might have hit a blocker or the wall when attempting to move downward\n those are obstacles that are there to add complexity to some of the tasks.\n Lets write some code to check if we're hitting one of those obstacles so that our robot knows to move around it..".to_string(),
                        crate::popup::PopupType::Success,
                        Some(4.0)
                    );
                }
            },
            1 => {
                // Task 2: Any error output completes
                if !self.error_outputs.is_empty() && !self.tutorial_state.task_completed[1] {
                    self.tutorial_state.task_completed[1] = true;
                    self.tutorial_state.current_task = 2;
                    self.popup_system.show_message(
                        "Task 2 Complete! ✓".to_string(),
                        "You did it again!\n Now we've learned how to dodge nonsense.\n A useful skill in life that we can now use in rust. So lets move onto the next point, Structs!\n we want to be able to scan the area and collect what's around the level we're currently on. So we'll move around the level and store the information about the level in something called a Struct".to_string(),
                        crate::popup::PopupType::Success,
                        Some(4.0)
                    );
                }
            },
            2 => {
                // Task 3: Variable used in print statement
                if self.check_variable_in_print() && !self.tutorial_state.task_completed[2] {
                    self.tutorial_state.task_completed[2] = true;
                    self.tutorial_state.current_task = 3;
                    self.popup_system.show_message(
                        "Task 3 Complete! ✓".to_string(),
                        "Outstanding! You've created a variable and used it in a print statement. Variables are the building blocks of all programs.".to_string(),
                        crate::popup::PopupType::Success,
                        Some(4.0)
                    );
                }
            },
            3 => {
                // Task 4: Scan output stored in mutable variable
                if self.check_mutable_scan_usage() && !self.tutorial_state.task_completed[3] {
                    self.tutorial_state.task_completed[3] = true;
                    self.tutorial_state.current_task = 4;
                    self.popup_system.show_message(
                        "Task 4 Complete! ✓".to_string(),
                        "Fantastic! You've learned about mutable variables using 'mut' and used the scan function. Mutability is crucial for changing data.".to_string(),
                        crate::popup::PopupType::Success,
                        Some(4.0)
                    );
                }
            },
            4 => {
                // Task 5: u32 integer used for movement
                if self.check_u32_movement() && !self.tutorial_state.task_completed[4] {
                    self.tutorial_state.task_completed[4] = true;
                    self.tutorial_state.current_task = 5;
                    self.finished = true; // Complete the level
                    self.popup_system.show_congratulations(
                        "🎉 Tutorial Complete!".to_string(),
                        "Congratulations! You've mastered the fundamentals of Rust programming:\n• Print statements with println!()\n• Error messages with eprintln!()\n• Variables and string interpolation\n• Mutable variables with 'mut'\n• Data types like u32 integers\n• Control flow with loops\n\nYou're now ready for more advanced programming challenges!".to_string(),
                        Some("Next: You'll learn about functions, error handling, and more advanced Rust concepts.".to_string())
                    );
                }
            },
            _ => {}
        }
    }
    
    fn check_variable_in_print(&self) -> bool {
        // Check if code contains variable declaration and usage in print
        let code = &self.current_code;
        let has_let = code.contains("let ");
        let has_println_with_format = code.contains("println!(") && (code.contains("{}") || code.contains("{"));
        has_let && has_println_with_format
    }
    
    fn check_mutable_scan_usage(&self) -> bool {
        // Check if code contains mutable variable with scan function
        let code = &self.current_code;
        let has_mut = code.contains("let mut ");
        let has_scan = code.contains("scan(");
        let has_print_with_scan = has_scan && (code.contains("println!(") || code.contains("eprintln!("));
        has_mut && has_print_with_scan
    }
    
    fn check_u32_movement(&self) -> bool {
        // Check if code contains u32 type annotation and movement
        let code = &self.current_code;
        let has_u32 = code.contains(": u32");
        let has_move = code.contains("move_bot(") || code.contains("move("); // Support both new and legacy
        let has_loop = code.contains("for ") || code.contains("while ");
        has_u32 && has_move && (has_loop || self.turns >= 3)
    }
}