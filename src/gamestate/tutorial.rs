use super::types::Game;

impl Game {
    // Unified tutorial system methods - routes to appropriate level
    pub fn get_tutorial_task_message(&self) -> String {
        // Only show tutorial messages for learning levels
        if !self.is_learning_level(self.level_idx) {
            return String::new();
        }
        
        match self.level_idx {
            0 => self.get_level_1_task_message(),
            1 => self.get_level_2_task_message(),
            2 => self.get_level_3_task_message(),
            3 => self.get_level_4_task_message(),
            4 => self.get_level_5_task_message(),
            _ => {
                // Generic message for unimplemented learning levels
                format!("🚧 Learning Level {} is under construction!\n\nThis level's tutorial system hasn't been implemented yet.\nCheck back soon for new learning content!", self.level_idx + 1)
            }
        }
    }
    
    pub fn check_tutorial_progress(&mut self) {
        // Only check progress for learning levels
        if !self.is_learning_level(self.level_idx) {
            return;
        }
        
        match self.level_idx {
            0 => self.check_level_1_progress(),
            1 => self.check_level_2_progress(),
            2 => self.check_level_3_progress(),
            3 => self.check_level_4_progress(),
            4 => self.check_level_5_progress(),
            _ => {
                // For unimplemented learning levels, just complete them automatically
                if self.tutorial_state.current_task == 0 {
                    self.tutorial_state.current_task = self.get_max_tasks_for_level(self.level_idx).unwrap_or(1);
                    self.popup_system.show_message(
                        "🚧 Level Under Construction".to_string(),
                        "This learning level hasn't been fully implemented yet. You can continue to the next level!".to_string(),
                        crate::popup::PopupType::Info,
                        Some(3.0)
                    );
                }
            }
        }
    }
    
    // Level 1 specific methods
    fn get_level_1_task_message(&self) -> String {
        
        match self.tutorial_state.current_task {
            0 => "Task 1/5: Learning Print Statements\n\nIn Rust, we use println!() to display text.\n In this game we capture the print statement and turn it into popups.\n Try typing:\nprintln!(\"Hello, Rust!\");\n\nThen hit [SHIFT+ENTER] Run to execute your code.".to_string(),
            1 => "Task 2/5: Error Messages\n\nGreat! Now let's learn about error messages.\n We use this to be able to tell ourselfs that something went wrong in the code, but in this game it's a red popup.\n Try using:\neprintln!(\"This is an error message!\");\n\nError messages are useful for debugging and showing warnings.".to_string(),
            2 => "Task 3/5: Variables in Print Statements\n\nExcellent! Now let's create a variable and print it.\n Variables are pretty much anything, but we're going to show you that you can create one and pass it into anything else we've already shown you.\n Try:\nlet my_message = \"Variables are powerful!\";\nprintln!(\"{}\", my_message);\n\nVariables store data we can reuse.".to_string(),
            3 => "Task 4/5: Mutable Variables and Scan Function\n\nAwesome! Let's learn about mutable variables by using the scan function. \n variables by themselves have to be defined in the code, but mutable variables don't basically if you have a user input or a message then you want to make that a mutable variable.\n this will tell rust that your variable exists, but you don't know what it is yet.\n\nlet mut scan_result = scan(\"right\");\nprintln!(\"Scan found: {}\", scan_result);\n\nThe 'mut' keyword lets us change variable values.".to_string(),
            4 => "Task 5/5: Data Types and Movement\n\nPerfect! Now let's learn about the u32 integer type and data types in general. \n sometimes we want to make sure that a variable is something specific by design, so we have data types to define what that specific thing is. \n learn more about this at the rust website by hitting CTRL+SHIFT+B to open your web browser to teh documentation for this language \n now lets learn it by using it for movement:\nlet steps: u32 = 3;\nfor _i in 0..steps {\n    move_bot(\"right\");\n}\n\nu32 is an unsigned 32-bit integer (0 to 4,294,967,295).".to_string(),
            _ => "Congratulations! You've correctly gone through the first few steps of learning the rust programming language!\n Next we'll teach you more about functions and loops\n Continue onwards by hitting CTRL+SHIFT+N to start the next level".to_string(),
        }
    }
    
    fn check_level_1_progress(&mut self) {
        if self.tutorial_state.current_task >= 5 {
            return; // Level 1 completed
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
                        "Great job! You've successfully used println!() to display text. This is one of the most fundamental operations in programming.".to_string(),
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
                        "Excellent! You've learned about error messages with eprintln!(). This is essential for debugging and showing warnings.".to_string(),
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