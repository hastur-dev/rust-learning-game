// TEMPLATE FOR NEW LEARNING LEVELS
// Copy this file and rename it to level_X.rs (where X is your level number)
// Follow the comments to customize for your specific level

use super::types::Game;

impl Game {
    // STEP 1: Update the level index and task messages
    // Change "level_X" to your actual level number (e.g., level_3, level_4, etc.)
    pub fn get_level_X_task_message(&self) -> String {
        // STEP 2: Update the level_idx check to match your level
        if self.level_idx != X {  // <- Replace X with your level index (2, 3, 4, etc.)
            return String::new();
        }
        
        match self.tutorial_state.current_task {
            // STEP 3: Define your tasks with clear instructions
            0 => "📋 **TASK 1/Y: Your First Task**\n\nDescription of what the student should do.\n\n```rust\n// Example code they should write\nfn example() {\n    println!(\"Hello!\");\n}\n```\n\nExplanation of the concept being taught.".to_string(),
            
            1 => "📋 **TASK 2/Y: Your Second Task**\n\nDescription of the next step.\n\n```rust\n// More example code\nfor i in 0..5 {\n    // loop body\n}\n```\n\nMore explanation.".to_string(),
            
            // Add more tasks as needed...
            
            _ => "🎉 **Level X Complete!**\n\nCongratulations! You've mastered:\n• Concept 1\n• Concept 2\n• Concept 3\n\nYou've learned [summary of what this level taught]!\n\n🚀 Ready for the next challenge!".to_string(),
        }
    }
    
    // STEP 4: Implement the progress checking logic
    pub fn check_level_X_progress(&mut self) {
        // Update the level_idx and max tasks
        if self.level_idx != X || self.tutorial_state.current_task >= Y {  // <- Replace X and Y
            return;
        }
        
        match self.tutorial_state.current_task {
            0 => {
                // STEP 5: Define the completion condition for Task 1
                if self.check_your_first_task_condition() && !self.tutorial_state.task_completed[0] {
                    self.tutorial_state.task_completed[0] = true;
                    self.tutorial_state.current_task = 1;
                    self.popup_system.show_message(
                        "Task 1 Complete! ✓".to_string(),
                        "Great! You've completed the first task. [Explanation of what they accomplished]".to_string(),
                        crate::popup::PopupType::Success,
                        Some(4.0)
                    );
                }
            },
            1 => {
                // Task 2 completion condition
                if self.check_your_second_task_condition() && !self.tutorial_state.task_completed[1] {
                    self.tutorial_state.task_completed[1] = true;
                    self.tutorial_state.current_task = 2;
                    self.popup_system.show_message(
                        "Task 2 Complete! ✓".to_string(),
                        "Excellent! [Explanation of second task achievement]".to_string(),
                        crate::popup::PopupType::Success,
                        Some(4.0)
                    );
                }
            },
            // Add more task completions...
            
            // Final task (adjust index to your last task)
            Y-1 => {  // <- Replace Y-1 with your last task index
                if self.check_your_final_task_condition() && !self.tutorial_state.task_completed[Y-1] {
                    self.tutorial_state.task_completed[Y-1] = true;
                    self.tutorial_state.current_task = Y;  // <- Replace Y
                    self.finished = true; // Complete the level
                    self.popup_system.show_congratulations(
                        "🎉 Level X Complete!".to_string(),  // <- Replace X
                        "Congratulations! You've mastered [list of concepts]:\n• Concept 1\n• Concept 2\n• Concept 3\n\nYou've successfully [summary of achievement]!".to_string(),
                        Some("Next: Level [X+1] will introduce [next concepts].".to_string())
                    );
                }
            },
            _ => {}
        }
    }
    
    // STEP 6: Implement the specific checking functions for each task
    fn check_your_first_task_condition(&self) -> bool {
        let code = &self.current_code;
        // Add your specific detection logic here
        // Examples:
        // - code.contains("fn my_function") for function definitions
        // - code.contains("for ") && code.contains("in ") for loops
        // - code.contains("struct ") for struct definitions
        // - code.contains("println!(") for print statements
        false // Replace with your actual condition
    }
    
    fn check_your_second_task_condition(&self) -> bool {
        let code = &self.current_code;
        // Add your second task detection logic
        false // Replace with your actual condition
    }
    
    // Add more checking functions as needed...
    
    fn check_your_final_task_condition(&self) -> bool {
        let code = &self.current_code;
        // Add your final task detection logic
        false // Replace with your actual condition
    }
}

/* 
STEP 7: Integration Instructions

1. After creating your level_X.rs file:
   
   a) Add it to src/gamestate/mod.rs:
      pub mod level_X;
   
   b) Add your level to the learning level config in src/gamestate/types.rs:
      LearningLevelConfig {
          level_idx: X,
          max_tasks: Y,
          name: "Level X: Your Level Name".to_string(),
      },
   
   c) Add your level to the routing in src/gamestate/tutorial.rs:
      X => self.get_level_X_task_message(),
      
      And:
      X => self.check_level_X_progress(),

2. Create your level YAML file in learning_levels/0X_your_level.yaml following the existing format

3. Test by running the game and navigating to your level!

TIPS:
- Keep task messages clear and instructional
- Provide code examples in the task descriptions
- Make detection conditions specific enough to avoid false positives
- Test each task completion condition thoroughly
- Use descriptive task names and helpful success messages

EXAMPLES OF COMMON DETECTION PATTERNS:
- Functions: code.contains("fn ") && code.contains("function_name")
- Loops: code.contains("for ") || code.contains("while ")
- Structs: code.contains("struct ") && code.contains("struct_name")
- Variables: code.contains("let ") && code.contains("variable_name")
- Error handling: code.contains("Result") || code.contains("match")
- Print statements: code.contains("println!(") && code.contains("{}")
*/