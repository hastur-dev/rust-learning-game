use super::types::Game;

impl Game {
    // Level 2 specific tutorial system methods
    pub fn get_level_2_task_message(&self) -> String {
        if self.level_idx != 1 {
            return String::new(); // Only for level 2
        }
        
        match self.tutorial_state.current_task {
            0 => "📋 **TASK 1/4: Create Function with Print Statement**\n\nCreate a function called `scan_level()` that contains a print statement:\n\n```rust\nfn scan_level() {\n    println!(\"Beginning level scan...\");\n    // This function will hold our main logic\n}\n```\n\nRemember to:\n• Define the function above `main()`\n• Call it from `main()` with `scan_level();`\n• Functions organize code into reusable blocks!".to_string(),
            
            1 => "📋 **TASK 2/4: Add Nested Loops for Grid Scanning**\n\nInside your `scan_level()` function, add nested loops to scan every tile in the 6x6 grid:\n\n```rust\nfor y in 0..6 {        // 6x6 grid height\n    for x in 0..6 {    // 6x6 grid width\n        // Movement and scanning code here\n        let scan_result = scan(\"current\");\n        println!(\"Scanned ({}, {}): {}\", x, y, scan_result);\n    }\n}\n```\n\nLoops let us repeat code systematically through the entire grid!".to_string(),
            
            2 => "📋 **TASK 3/4: Create GridInfo Struct**\n\nFirst, define a struct above your functions to store grid data:\n\n```rust\nstruct GridInfo {\n    x: i32,\n    y: i32,\n    content: String,\n}\n```\n\nThen inside your loops, collect and track item locations:\n\n```rust\nlet mut item_locations = Vec::new();\n\n// Inside your nested loops:\nif scan_result != \"empty\" && scan_result != \"wall\" {\n    item_locations.push((x, y, scan_result.clone()));\n}\n```\n\nStructs organize related data together!".to_string(),
            
            3 => "📋 **TASK 4/4: Create Item Collection Function**\n\nCreate a second function `grab_if_item()` with an if statement:\n\n```rust\nfn grab_if_item(scan_result: &str) {\n    if scan_result != \"empty\" && scan_result != \"wall\" && scan_result != \"goal\" {\n        grab();\n        println!(\"Grabbed: {}\", scan_result);\n    }\n}\n```\n\nCall this function inside your scanning loop:\n\n```rust\n// Inside your nested loops:\nlet scan_result = scan(\"current\");\ngrab_if_item(&scan_result);\n```\n\nSeparate functions make code more organized and reusable!".to_string(),
            
            _ => "🎉 **Level 2 Complete!**\n\nCongratulations! You've mastered:\n• Function creation and organization\n• Nested loops for systematic processing\n• Structs for data organization\n• Conditional logic with if statements\n\nYou've built a complete grid scanning and item collection system using functions, loops, and structs - the building blocks of larger programs!\n\n🚀 Ready for Level 3: Error Handling and Advanced Patterns!".to_string(),
        }
    }
    
    pub fn check_level_2_progress(&mut self) {
        if self.level_idx != 1 || self.tutorial_state.current_task >= 4 {
            return; // Only for level 2 and if not completed
        }
        
        match self.tutorial_state.current_task {
            0 => {
                // Task 1: Function definition with print statement
                if self.check_function_with_print() && !self.tutorial_state.task_completed[0] {
                    self.tutorial_state.task_completed[0] = true;
                    self.tutorial_state.current_task = 1;
                    self.popup_system.show_message(
                        "Task 1 Complete! ✓".to_string(),
                        "Excellent! You've created a function with a print statement. Functions are the foundation of organized, reusable code in Rust!".to_string(),
                        crate::popup::PopupType::Success,
                        Some(4.0)
                    );
                }
            },
            1 => {
                // Task 2: Nested loops for grid scanning
                if self.check_nested_loops() && !self.tutorial_state.task_completed[1] {
                    self.tutorial_state.task_completed[1] = true;
                    self.tutorial_state.current_task = 2;
                    self.popup_system.show_message(
                        "Task 2 Complete! ✓".to_string(),
                        "Perfect! You've implemented nested loops for systematic grid scanning. Loops are essential for processing data collections efficiently!".to_string(),
                        crate::popup::PopupType::Success,
                        Some(4.0)
                    );
                }
            },
            2 => {
                // Task 3: Struct definition and usage
                if self.check_struct_usage() && !self.tutorial_state.task_completed[2] {
                    self.tutorial_state.task_completed[2] = true;
                    self.tutorial_state.current_task = 3;
                    self.popup_system.show_message(
                        "Task 3 Complete! ✓".to_string(),
                        "Outstanding! You've defined and used a struct to organize grid data. Structs are Rust's way of creating custom data types for complex information!".to_string(),
                        crate::popup::PopupType::Success,
                        Some(4.0)
                    );
                }
            },
            3 => {
                // Task 4: Second function with conditional logic
                if self.check_grab_function() && !self.tutorial_state.task_completed[3] {
                    self.tutorial_state.task_completed[3] = true;
                    self.tutorial_state.current_task = 4;
                    self.finished = true; // Complete the level
                    self.popup_system.show_congratulations(
                        "🎉 Level 2 Complete!".to_string(),
                        "Congratulations! You've mastered advanced Rust concepts:\n• Functions and code organization\n• Nested loops for data processing\n• Structs for custom data types\n• Conditional logic with if statements\n• Systematic problem-solving approach\n\nYou've built a complete grid scanning and item collection system - a real programming accomplishment!".to_string(),
                        Some("Next: Level 3 will introduce error handling, pattern matching, and more advanced Rust features.".to_string())
                    );
                }
            },
            _ => {}
        }
    }
    
    // Level 2 specific checking functions
    fn check_function_with_print(&self) -> bool {
        let code = &self.current_code;
        let has_function_def = code.contains("fn ") && 
                              (code.contains("scan_level") || code.contains("fn scan_level"));
        let has_println = code.contains("println!(");
        let has_function_call = code.contains("scan_level();") || code.contains("scan_level ()");
        
        // Check if function is called from within main()
        let has_main_function = code.contains("fn main()");
        let main_calls_function = if has_main_function {
            // Find main function and check if it calls scan_level
            if let Some(main_start) = code.find("fn main()") {
                let after_main = &code[main_start..];
                if let Some(main_brace_start) = after_main.find('{') {
                    let main_body_start = main_start + main_brace_start + 1;
                    // Find the closing brace for main function
                    let mut brace_count = 1;
                    let mut main_end = main_body_start;
                    let chars: Vec<char> = code.chars().collect();
                    
                    for (i, &ch) in chars.iter().enumerate().skip(main_body_start) {
                        match ch {
                            '{' => brace_count += 1,
                            '}' => {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    main_end = i;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    if main_end > main_body_start {
                        let main_body = &code[main_body_start..main_end];
                        main_body.contains("scan_level();") || main_body.contains("scan_level ()")
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            // If no main function, accept any function call for now
            has_function_call
        };
        
        has_function_def && has_println && main_calls_function
    }
    
    fn check_nested_loops(&self) -> bool {
        let code = &self.current_code;
        let has_outer_loop = code.contains("for ") && code.contains("0..6");
        let has_inner_loop = code.matches("for ").count() >= 2; // At least 2 for loops
        let has_scan = code.contains("scan(") || code.contains("scan (");
        
        has_outer_loop && has_inner_loop && has_scan
    }
    
    fn check_struct_usage(&self) -> bool {
        let code = &self.current_code;
        let has_struct_def = code.contains("struct ") && 
                            (code.contains("GridInfo") || code.contains("grid_info") || code.contains("GridData"));
        let has_fields = code.contains("x:") && code.contains("y:") && code.contains("content:");
        let has_vec = code.contains("Vec::new()") || code.contains("vec!");
        
        has_struct_def && has_fields && has_vec
    }
    
    fn check_grab_function(&self) -> bool {
        let code = &self.current_code;
        let has_grab_function = code.contains("fn ") && code.contains("grab_if_item");
        let has_if_statement = code.contains("if ") && (code.contains("!=") || code.contains("=="));
        let has_grab_call = code.contains("grab();") || code.contains("grab ()");
        let has_function_param = code.contains("scan_result") || code.contains("&str");
        
        has_grab_function && has_if_statement && has_grab_call && has_function_param
    }
}