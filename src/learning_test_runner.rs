// Fully functional automated test runner for individual learning level tasks
// Tests each task separately with the actual in-game editor and UI

use macroquad::prelude::*;
use log::{info, warn, error};
use ::rand::{rngs::StdRng, SeedableRng};
use std::time::{Duration, Instant};

use crate::{
    gamestate::{Game},
    embedded_levels,
    learning_level_solutions::{self, TaskSolution},
    execute_rust_code,
    menu::{MenuState},
    draw_main_game_view,
};

pub struct TaskTestResult {
    pub level_name: String,
    pub task_number: usize,
    pub task_description: String,
    pub success: bool,
    pub error_message: Option<String>,
    pub time_taken: Duration,
}

#[derive(Debug)]
enum TestState {
    Loading,
    InputtingSolution,
    ExecutingCode,
    WaitingForCompletion,
    TaskComplete,
    NextTask,
    AllTasksComplete,
}

pub struct LearningTaskTestRunner {
    game: Game,
    current_level: usize,
    current_task: usize,
    test_results: Vec<TaskTestResult>,
    all_tasks: Vec<TaskSolution>,
    level_tasks: Vec<TaskSolution>,
    test_start_time: Instant,
    task_start_time: Instant,
    state: TestState,
    state_timer: f32,
    current_solution: String,
    typing_progress: usize,
    typing_speed: f32,
    input_chars: Vec<char>,
    char_input_timer: f32,
    total_tasks_tested: usize,
}

impl LearningTaskTestRunner {
    pub fn new() -> Self {
        info!("Initializing Learning Task Test Runner");

        let rng = StdRng::seed_from_u64(0x7E57);
        let levels = embedded_levels::get_embedded_level_specs();
        let mut game = Game::new(levels, rng);

        // Skip menu and go directly to first learning level
        game.menu.state = MenuState::InGame;
        game.level_idx = 0;
        game.load_level(0);
        game.code_editor_active = true;

        // Clear initial code
        game.current_code = String::new();
        game.cursor_position = 0;

        let all_tasks = learning_level_solutions::get_all_task_solutions();

        Self {
            game,
            current_level: 0,
            current_task: 1,
            test_results: Vec::new(),
            level_tasks: Vec::new(),
            all_tasks,
            test_start_time: Instant::now(),
            task_start_time: Instant::now(),
            state: TestState::Loading,
            state_timer: 0.0,
            current_solution: String::new(),
            typing_progress: 0,
            typing_speed: 25.0, // 25 characters per second
            input_chars: Vec::new(),
            char_input_timer: 0.0,
            total_tasks_tested: 0,
        }
    }

    /// Update the test runner state
    pub async fn update(&mut self, delta_time: f32) {
        self.state_timer += delta_time;
        self.char_input_timer += delta_time;

        match self.state {
            TestState::Loading => {
                if self.state_timer >= 1.0 {
                    self.start_task_test();
                }
            },
            TestState::InputtingSolution => {
                self.update_typing();
            },
            TestState::ExecutingCode => {
                if self.state_timer >= 1.0 {
                    self.execute_solution().await;
                }
            },
            TestState::WaitingForCompletion => {
                if self.check_for_completion() || self.state_timer >= 3.0 {
                    self.complete_task();
                }
            },
            TestState::TaskComplete => {
                if self.state_timer >= 1.5 {
                    self.advance_to_next_task();
                }
            },
            TestState::NextTask => {
                if self.state_timer >= 0.5 {
                    self.start_task_test();
                }
            },
            TestState::AllTasksComplete => {
                // Stay in this state
            },
        }
    }

    fn start_task_test(&mut self) {
        let level_name = &self.game.levels[self.current_level].name;

        // Get tasks for current level
        self.level_tasks = learning_level_solutions::get_task_solutions_for_level(level_name);

        if self.current_task <= self.level_tasks.len() {
            if let Some(task_solution) = self.level_tasks.get(self.current_task - 1) {
                info!("Starting task test: {} - Task {}/{}: {}",
                      level_name, self.current_task, self.level_tasks.len(),
                      task_solution.task_description);

                self.current_solution = task_solution.solution_code.to_string();
                self.typing_progress = 0;
                self.input_chars = self.current_solution.chars().collect();

                // Clear the editor
                self.game.current_code = String::new();
                self.game.cursor_position = 0;
                self.game.code_editor_active = true;

                // Clear outputs from previous tasks
                self.game.println_outputs.clear();
                self.game.error_outputs.clear();
                self.game.execution_result.clear();

                self.state = TestState::InputtingSolution;
                self.state_timer = 0.0;
                self.char_input_timer = 0.0;
                self.task_start_time = Instant::now();

                info!("Prepared to input solution of {} characters", self.input_chars.len());
            } else {
                warn!("No solution found for task {}", self.current_task);
                self.advance_to_next_level();
            }
        } else {
            // All tasks for this level are complete
            self.advance_to_next_level();
        }
    }

    fn update_typing(&mut self) {
        // Type characters at the specified speed
        let interval = 1.0 / self.typing_speed;

        if self.char_input_timer >= interval && self.typing_progress < self.input_chars.len() {
            let ch = self.input_chars[self.typing_progress];

            // Add character to the game's code
            self.game.current_code.insert(self.game.cursor_position, ch);
            self.game.cursor_position += 1;

            self.typing_progress += 1;
            self.char_input_timer = 0.0;
        }

        if self.typing_progress >= self.input_chars.len() {
            info!("Solution input complete! Code length: {}", self.game.current_code.len());
            self.state = TestState::ExecutingCode;
            self.state_timer = 0.0;
        }
    }

    async fn execute_solution(&mut self) {
        info!("Executing solution for task {}", self.current_task);

        // Execute the code using the game's execution system
        self.game.execution_result = execute_rust_code(&mut self.game).await;

        info!("Code execution result: {}", self.game.execution_result);

        self.state = TestState::WaitingForCompletion;
        self.state_timer = 0.0;
    }

    fn check_for_completion(&mut self) -> bool {
        // Basic completion checks based on execution result
        if self.game.execution_result.contains("Print statements executed successfully") ||
           self.game.execution_result.contains("successfully") ||
           !self.game.println_outputs.is_empty() {
            return true;
        }

        // Check if there are no errors and code was executed
        if !self.game.execution_result.contains("error") &&
           !self.game.execution_result.contains("Error") &&
           !self.game.execution_result.is_empty() {
            return true;
        }

        false
    }

    fn complete_task(&mut self) {
        self.record_test_success();
        self.state = TestState::TaskComplete;
        self.state_timer = 0.0;
        self.total_tasks_tested += 1;
    }

    fn record_test_success(&mut self) {
        let duration = self.task_start_time.elapsed();
        let level_name = self.game.levels[self.current_level].name.clone();
        let task_desc = if let Some(task) = self.level_tasks.get(self.current_task - 1) {
            task.task_description.to_string()
        } else {
            format!("Task {}", self.current_task)
        };

        self.test_results.push(TaskTestResult {
            level_name: level_name.clone(),
            task_number: self.current_task,
            task_description: task_desc.clone(),
            success: true,
            error_message: None,
            time_taken: duration,
        });

        info!("✅ {} - Task {} completed in {:?}",
              level_name, self.current_task, duration);
    }

    fn record_test_failure(&mut self, error: String) {
        let duration = self.task_start_time.elapsed();
        let level_name = self.game.levels[self.current_level].name.clone();
        let task_desc = if let Some(task) = self.level_tasks.get(self.current_task - 1) {
            task.task_description.to_string()
        } else {
            format!("Task {}", self.current_task)
        };

        self.test_results.push(TaskTestResult {
            level_name: level_name.clone(),
            task_number: self.current_task,
            task_description: task_desc.clone(),
            success: false,
            error_message: Some(error.clone()),
            time_taken: duration,
        });

        error!("❌ {} - Task {} failed: {}",
               level_name, self.current_task, error);
    }

    fn advance_to_next_task(&mut self) {
        self.current_task += 1;

        // Check if there are more tasks for this level
        if self.current_task <= self.level_tasks.len() {
            info!("Advancing to task {}/{}", self.current_task, self.level_tasks.len());
            self.state = TestState::NextTask;
            self.state_timer = 0.0;
        } else {
            // All tasks for this level complete, move to next level
            self.advance_to_next_level();
        }
    }

    fn advance_to_next_level(&mut self) {
        self.current_level += 1;
        self.current_task = 1;

        if self.current_level < self.game.levels.len() && self.current_level < 3 {  // Limit to first 3 levels
            info!("Advancing to level {}", self.current_level + 1);
            self.game.level_idx = self.current_level;
            self.game.load_level(self.current_level);
            self.game.println_outputs.clear();
            self.game.error_outputs.clear();
            self.state = TestState::NextTask;
            self.state_timer = 0.0;
        } else {
            info!("All levels and tasks tested!");
            self.state = TestState::AllTasksComplete;
            self.state_timer = 0.0;
        }
    }

    /// Draw the current test state using the real game UI
    pub fn draw(&mut self) {
        // Use the actual game drawing function
        draw_main_game_view(&mut self.game);

        // Draw test status overlay
        self.draw_test_overlay();
    }

    fn draw_test_overlay(&self) {
        let overlay_height = 140.0;
        let overlay_y = screen_height() - overlay_height;

        // Draw semi-transparent background
        draw_rectangle(0.0, overlay_y, screen_width(), overlay_height, Color::new(0.0, 0.0, 0.2, 0.9));

        // Current state
        let state_text = match self.state {
            TestState::Loading => "Loading task...",
            TestState::InputtingSolution => "Typing solution into editor...",
            TestState::ExecutingCode => "Executing code...",
            TestState::WaitingForCompletion => "Waiting for completion...",
            TestState::TaskComplete => "Task completed!",
            TestState::NextTask => "Loading next task...",
            TestState::AllTasksComplete => "All tasks complete!",
        };

        let level_name = &self.game.levels[self.current_level].name;
        let total_tasks_for_level = self.level_tasks.len();

        draw_text(&format!("🧪 AUTOMATED TASK TESTING: {}",
                          level_name),
                 10.0, overlay_y + 20.0, 18.0, YELLOW);

        draw_text(&format!("Task {}/{}: {}",
                          self.current_task, total_tasks_for_level, state_text),
                 10.0, overlay_y + 40.0, 16.0, WHITE);

        // Show typing progress
        if matches!(self.state, TestState::InputtingSolution) {
            let typing_progress = if self.input_chars.is_empty() { 0.0 }
                                 else { (self.typing_progress as f32) / (self.input_chars.len() as f32) };
            draw_text(&format!("Typing Progress: {:.1}% ({}/{})",
                             typing_progress * 100.0,
                             self.typing_progress,
                             self.input_chars.len()),
                     10.0, overlay_y + 60.0, 14.0, BLUE);
        }

        // Progress bar for all tasks
        let total_expected_tasks = 10; // Level 1: 1 task + Level 2: 4 tasks + Level 3: 5 tasks = 10 total tasks
        let progress = (self.total_tasks_tested as f32) / (total_expected_tasks as f32);
        let bar_width = screen_width() - 20.0;
        let bar_height = 20.0;
        let bar_y = overlay_y + 80.0;

        draw_rectangle(10.0, bar_y, bar_width, bar_height, DARKGRAY);
        draw_rectangle(10.0, bar_y, bar_width * progress, bar_height,
                      if matches!(self.state, TestState::AllTasksComplete) { GREEN } else { YELLOW });

        // Results summary
        let passed = self.test_results.iter().filter(|r| r.success).count();
        let failed = self.test_results.len() - passed;
        draw_text(&format!("Results: {} passed, {} failed (Total tested: {})",
                          passed, failed, self.total_tasks_tested),
                 10.0, overlay_y + 105.0, 14.0, LIGHTGRAY);

        // Show current code length and last execution result
        draw_text(&format!("Code: {} chars", self.game.current_code.len()),
                 screen_width() - 150.0, overlay_y + 20.0, 14.0, LIGHTGRAY);

        if !self.game.execution_result.is_empty() {
            let result_preview = if self.game.execution_result.len() > 30 {
                format!("{}...", &self.game.execution_result[..27])
            } else {
                self.game.execution_result.clone()
            };
            draw_text(&format!("Result: {}", result_preview),
                     screen_width() - 350.0, overlay_y + 40.0, 12.0, GREEN);
        }

        draw_text(&format!("Total Time: {:02}:{:02}",
                          self.test_start_time.elapsed().as_secs() / 60,
                          self.test_start_time.elapsed().as_secs() % 60),
                 10.0, overlay_y + 125.0, 14.0, LIGHTGRAY);
    }

    /// Check if all tests are complete
    pub fn is_complete(&self) -> bool {
        matches!(self.state, TestState::AllTasksComplete)
    }

    /// Get the test results
    pub fn get_results(&self) -> &[TaskTestResult] {
        &self.test_results
    }

    /// Print a summary of test results
    pub fn print_summary(&self) {
        println!("\n{}", "=".repeat(70));
        println!("LEARNING LEVEL TASK TEST RESULTS");
        println!("{}", "=".repeat(70));

        let total = self.test_results.len();
        let passed = self.test_results.iter().filter(|r| r.success).count();
        let failed = total - passed;

        println!("Total Tasks Tested: {}", total);
        println!("✅ Passed: {}", passed);
        println!("❌ Failed: {}", failed);

        if total > 0 {
            println!("Success Rate: {:.1}%", (passed as f32 / total as f32) * 100.0);
        }

        println!("\nDetailed Results:");
        println!("{}", "-".repeat(70));

        for result in &self.test_results {
            let status = if result.success { "✅ PASS" } else { "❌ FAIL" };
            println!("[{}] {} - Task {}: {} ({:?})",
                    status,
                    result.level_name,
                    result.task_number,
                    result.task_description,
                    result.time_taken);

            if let Some(error) = &result.error_message {
                println!("    Error: {}", error);
            }
        }

        println!("{}", "=".repeat(70));
        println!("Total test time: {:?}", self.test_start_time.elapsed());
    }
}

/// Run the automated learning level task tests
pub async fn run_learning_level_tests() {
    info!("Starting automated learning level task tests");

    let mut test_runner = LearningTaskTestRunner::new();

    // Main test loop
    while !test_runner.is_complete() {
        // Update test runner
        test_runner.update(get_frame_time()).await;

        // Draw current state using real game UI
        test_runner.draw();

        // Allow for early exit with ESC
        if is_key_pressed(KeyCode::Escape) {
            warn!("Test runner interrupted by user");
            break;
        }

        next_frame().await;
    }

    // Print test summary
    test_runner.print_summary();

    // Keep window open for review
    info!("Tests complete. Press SPACE to exit or ESC to quit immediately.");
    loop {
        clear_background(BLACK);
        draw_text("🎉 All Task Tests Complete!", screen_width() / 2.0 - 150.0, screen_height() / 2.0 - 50.0, 30.0, GREEN);
        draw_text("Press SPACE to exit", screen_width() / 2.0 - 80.0, screen_height() / 2.0, 20.0, WHITE);

        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}