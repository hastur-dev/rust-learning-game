// Fully functional automated test runner for individual learning level tasks
// Tests each task separately with the actual in-game editor and UI

use macroquad::prelude::*;
use log::{info, warn, error};
use ::rand::{rngs::StdRng, SeedableRng};
use std::time::{Duration, Instant};

use crate::{
    gamestate::{Game},
    embedded_levels,
    automated_level_testing::{self, LevelTestConfig, TaskTest},
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
    all_level_configs: Vec<LevelTestConfig>,
    current_level_config: Option<LevelTestConfig>,
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
    start_level: usize,    // NEW: Level to start testing from
    max_levels: usize,     // NEW: Maximum number of levels to test
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

        let all_level_configs = automated_level_testing::get_all_level_tests();

        Self {
            game,
            current_level: 0,
            current_task: 1,
            test_results: Vec::new(),
            current_level_config: None,
            all_level_configs,
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
            start_level: 0,        // Default: start from level 0
            max_levels: 6,         // Default: test up to 6 levels
        }
    }

    pub fn new_with_options(start_level: usize, max_levels: usize) -> Self {
        info!("Initializing Learning Task Test Runner (start: {}, max: {})", start_level, max_levels);

        let rng = StdRng::seed_from_u64(0x7E57);
        let levels = embedded_levels::get_embedded_level_specs();
        let mut game = Game::new(levels, rng);

        // Skip menu and go directly to the specified starting level
        game.menu.state = MenuState::InGame;
        game.level_idx = start_level;

        // Safely load the level if it exists
        if start_level < game.levels.len() {
            game.load_level(start_level);
            info!("Loaded level {} for testing", start_level);
        } else {
            warn!("Start level {} is beyond available levels ({}), using level 0", start_level, game.levels.len());
            game.level_idx = 0;
            game.load_level(0);
        }

        game.code_editor_active = true;

        // Clear initial code
        game.current_code = String::new();
        game.cursor_position = 0;

        let all_level_configs = automated_level_testing::get_all_level_tests();

        let current_level = start_level.min(game.levels.len().saturating_sub(1));

        Self {
            game,
            current_level,
            current_task: 1,
            test_results: Vec::new(),
            current_level_config: None,
            all_level_configs,
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
            start_level,
            max_levels,
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
        // Safe access to level name with bounds checking
        let level_name = if self.current_level < self.game.levels.len() {
            self.game.levels[self.current_level].name.clone()
        } else {
            error!("Invalid level index {} (max: {})", self.current_level, self.game.levels.len());
            format!("Unknown Level {}", self.current_level)
        };

        // Get level configuration for current level
        info!("Getting level config for level index: {}", self.current_level);
        self.current_level_config = automated_level_testing::get_level_tests(self.current_level);

        if let Some(ref level_config) = self.current_level_config {
            info!("Found level config: {} with {} tasks", level_config.level_name, level_config.tasks.len());

            if self.current_task <= level_config.tasks.len() {
                if let Some(task_test) = level_config.tasks.get(self.current_task - 1) {
                    info!("Starting task test: {} - Task {}/{}: {}",
                          level_config.level_name, self.current_task, level_config.tasks.len(),
                          task_test.task_name);

                    self.current_solution = task_test.solution_code.to_string();
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
                    warn!("No task found for task number {}", self.current_task);
                    self.advance_to_next_level();
                }
            } else {
                // All tasks for this level are complete
                self.advance_to_next_level();
            }
        } else {
            warn!("No level config found for level index {}", self.current_level);
            self.advance_to_next_level();
        }
    }

    fn update_typing(&mut self) {
        // Type characters at the specified speed
        let interval = 1.0 / self.typing_speed;

        if self.char_input_timer >= interval && self.typing_progress < self.input_chars.len() {
            let ch = self.input_chars[self.typing_progress];

            // Safely add character to the game's code by appending to the end
            // This avoids UTF-8 boundary issues that can occur with cursor_position
            self.game.current_code.push(ch);
            self.game.cursor_position = self.game.current_code.len();

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
        // Use completion indicators from the test configuration
        if let Some(ref level_config) = self.current_level_config {
            if let Some(task_test) = level_config.tasks.get(self.current_task - 1) {
                // Check if all completion indicators are present in the output
                let all_outputs = self.game.println_outputs.join("\n");

                let mut indicators_found = 0;
                let total_indicators = task_test.completion_indicators.len();

                for indicator in &task_test.completion_indicators {
                    if all_outputs.contains(indicator) {
                        indicators_found += 1;
                    }
                }

                info!("Completion check: {}/{} indicators found for task {}",
                      indicators_found, total_indicators, self.current_task);

                // Task is complete if all indicators are found
                return indicators_found == total_indicators && total_indicators > 0;
            }
        }

        // Fallback to basic completion check
        !self.game.println_outputs.is_empty() &&
        !self.game.execution_result.contains("error") &&
        !self.game.execution_result.contains("Error")
    }

    fn complete_task(&mut self) {
        self.record_test_success();

        // Update the game's task completion state for GUI display
        // Convert 1-based task number to 0-based index
        let task_index = self.current_task - 1;
        if task_index < 5 {  // Ensure we don't exceed the array bounds
            self.game.tutorial_state.task_completed[task_index] = true;
            // CRITICAL FIX: Also update current_task to match - this is what makes GUI transitions work
            self.game.tutorial_state.current_task = self.current_task;
            info!("Marked task {} as completed in GUI (index {}) and updated current_task to {}",
                  self.current_task, task_index, self.current_task);
        }

        self.state = TestState::TaskComplete;
        self.state_timer = 0.0;
        self.total_tasks_tested += 1;
    }

    fn record_test_success(&mut self) {
        let duration = self.task_start_time.elapsed();
        let level_name = if self.current_level < self.game.levels.len() {
            self.game.levels[self.current_level].name.clone()
        } else {
            format!("Unknown Level {}", self.current_level)
        };
        let task_desc = if let Some(ref level_config) = self.current_level_config {
            if let Some(task_test) = level_config.tasks.get(self.current_task - 1) {
                task_test.task_name.to_string()
            } else {
                format!("Task {}", self.current_task)
            }
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
        let level_name = if self.current_level < self.game.levels.len() {
            self.game.levels[self.current_level].name.clone()
        } else {
            format!("Unknown Level {}", self.current_level)
        };
        let task_desc = if let Some(ref level_config) = self.current_level_config {
            if let Some(task_test) = level_config.tasks.get(self.current_task - 1) {
                task_test.task_name.to_string()
            } else {
                format!("Task {}", self.current_task)
            }
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
        if let Some(ref level_config) = self.current_level_config {
            if self.current_task <= level_config.tasks.len() {
                info!("Advancing to task {}/{}", self.current_task, level_config.tasks.len());
                self.state = TestState::NextTask;
                self.state_timer = 0.0;
            } else {
                // All tasks for this level complete, move to next level
                self.advance_to_next_level();
            }
        } else {
            // No level config, move to next level
            self.advance_to_next_level();
        }
    }

    fn advance_to_next_level(&mut self) {
        self.current_level += 1;
        self.current_task = 1;

        // Check if we should continue testing based on our limits
        let max_level_to_test = (self.start_level + self.max_levels).min(self.game.levels.len());

        if self.current_level < self.game.levels.len() && self.current_level < max_level_to_test {
            info!("Advancing to level {} ({})", self.current_level + 1, self.current_level);
            self.game.level_idx = self.current_level;

            // Safety check before loading level
            if self.current_level < self.game.levels.len() {
                self.game.load_level(self.current_level);
                info!("Successfully loaded level {}", self.current_level);
            } else {
                error!("Cannot load level {} - beyond available levels ({})", self.current_level, self.game.levels.len());
                self.state = TestState::AllTasksComplete;
                return;
            }

            self.game.println_outputs.clear();
            self.game.error_outputs.clear();

            // Reset task completion state for new level
            self.game.tutorial_state.task_completed = [false; 5];
            // CRITICAL FIX: Reset the game's current_task to 0 (corresponds to task 1 in 0-based indexing)
            self.game.tutorial_state.current_task = 0;
            info!("Reset task completion state and current_task for new level");

            self.state = TestState::NextTask;
            self.state_timer = 0.0;
        } else {
            info!("All levels and tasks tested! (tested {} levels starting from {})",
                  self.current_level - self.start_level, self.start_level);
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

        // Safe access to level name with bounds checking
        let level_name = if self.current_level < self.game.levels.len() {
            &self.game.levels[self.current_level].name
        } else {
            "Unknown Level"
        };
        let total_tasks_for_level = if let Some(ref level_config) = self.current_level_config {
            level_config.tasks.len()
        } else {
            0
        };

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
        let total_expected_tasks = 24; // Level 1: 5 tasks + Level 2: 4 tasks + Level 3: 5 tasks + Level 4: 5 tasks + Level 5: 5 tasks = 24 total tasks
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
    run_test_loop(test_runner).await;
}

/// Run the automated learning level task tests with options
pub async fn run_learning_level_tests_with_options(start_level: usize, max_levels: usize) {
    info!("Starting automated learning level task tests (start: {}, max: {})", start_level, max_levels);

    let mut test_runner = LearningTaskTestRunner::new_with_options(start_level, max_levels);

    // Main test loop
    run_test_loop(test_runner).await;
}

async fn run_test_loop(mut test_runner: LearningTaskTestRunner) {

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