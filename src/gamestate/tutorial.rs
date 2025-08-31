use super::types::Game;

impl Game {
    // Tutorial system methods
    pub fn get_tutorial_task_message(&self) -> String {
        if self.level_idx != 0 {
            return String::new(); // Only for level 1
        }
        
        match self.tutorial_state.current_task {
            0 => "Task 1/5: Learning Print Statements\n\nIn Rust, we use println!() to display text. Try typing:\nprintln!(\"Hello, Rust!\");\n\nThen click [ENTER] Run to execute your code.".to_string(),
            1 => "Task 2/5: Error Messages\n\nGreat! Now let's learn about error messages. Try using:\neprintln!(\"This is an error message!\");\n\nError messages are useful for debugging and showing warnings.".to_string(),
            2 => "Task 3/5: Variables in Print Statements\n\nExcellent! Now let's create a variable and print it. Try:\nlet my_message = \"Variables are powerful!\";\nprintln!(\"{}\", my_message);\n\nVariables store data we can reuse.".to_string(),
            3 => "Task 4/5: Mutable Variables and Scan Function\n\nAwesome! Let's learn about mutable variables by using the scan function:\nlet mut scan_result = scan(up);\nprintln!(\"Scan found: {}\", scan_result);\n\nThe 'mut' keyword lets us change variable values.".to_string(),
            4 => "Task 5/5: Data Types and Movement\n\nPerfect! Now let's learn about the u32 integer type by using it for movement:\nlet steps: u32 = 3;\nfor _i in 0..steps {\n    move_bot(\"right\");\n}\n\nu32 is an unsigned 32-bit integer (0 to 4,294,967,295).".to_string(),
            _ => "Tutorial Complete! You've learned the basics of Rust programming!".to_string(),
        }
    }
    
    pub fn check_tutorial_progress(&mut self) {
        if self.level_idx != 0 || self.tutorial_state.current_task >= 5 {
            return; // Only for level 1 and if not completed
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