//! # Rust Game Test Runner
//!
//! A testing framework for editors and basic game engines to verify code functionality
//! without GUI interaction. Originally developed for the Rust Learning Game.

use serde::{Deserialize, Serialize};

pub mod parser;
pub mod executor;
pub mod grid;
pub mod robot;

pub use parser::*;
pub use executor::*;
pub use grid::*;
pub use robot::*;

/// Configuration for game testing environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub grid_width: usize,
    pub grid_height: usize,
    pub robot_start_x: i32,
    pub robot_start_y: i32,
    pub enable_logging: bool,
}

impl GameConfig {
    /// Create a new game configuration with default values
    pub fn new() -> Self {
        Self {
            grid_width: 6,
            grid_height: 6,
            robot_start_x: 1,
            robot_start_y: 1,
            enable_logging: false,
        }
    }

    /// Set the grid size
    pub fn with_grid_size(mut self, width: usize, height: usize) -> Self {
        self.grid_width = width;
        self.grid_height = height;
        self
    }

    /// Set the robot starting position
    pub fn with_robot_start_position(mut self, x: i32, y: i32) -> Self {
        self.robot_start_x = x;
        self.robot_start_y = y;
        self
    }

    /// Enable detailed logging
    pub fn with_logging(mut self, enabled: bool) -> Self {
        self.enable_logging = enabled;
        self
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Position on the game grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Types of messages that can appear during game execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// Standard output from println!
    Stdout,
    /// Error output from eprintln!
    Stderr,
    /// Panic messages
    Panic,
    /// Robot function results
    RobotAction,
    /// Tutorial or info messages
    Info,
}

/// A message generated during code execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMessage {
    pub message_type: MessageType,
    pub title: String,
    pub content: String,
}

impl GameMessage {
    pub fn stdout(content: String) -> Self {
        Self {
            message_type: MessageType::Stdout,
            title: "📝 Program Output".to_string(),
            content,
        }
    }

    pub fn stderr(content: String) -> Self {
        Self {
            message_type: MessageType::Stderr,
            title: "🔴 Error Output".to_string(),
            content,
        }
    }

    pub fn robot_action(content: String) -> Self {
        Self {
            message_type: MessageType::RobotAction,
            title: "🤖 Robot Action Results".to_string(),
            content,
        }
    }
}

/// Result of executing test code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Whether the code executed successfully
    pub success: bool,
    /// Final position of the robot
    pub final_position: Position,
    /// Number of turns taken
    pub turns_taken: u32,
    /// Messages generated during execution
    pub messages: Vec<GameMessage>,
    /// Raw execution output
    pub execution_output: String,
    /// Error message if execution failed
    pub error: Option<String>,
}

/// Main test runner for game code
pub struct TestRunner {
    config: GameConfig,
}

impl TestRunner {
    /// Create a new test runner with the given configuration
    pub fn new(config: GameConfig) -> Self {
        Self { config }
    }

    /// Test the given Rust code and return results
    pub async fn test_code(&self, code: &str) -> Result<TestResult, Box<dyn std::error::Error>> {
        let mut game_state = GameState::new(&self.config);
        let mut executor = CodeExecutor::new();

        // Parse the code into function calls
        let function_calls = parse_rust_code(code)?;
        
        // Extract print statements
        let print_outputs = extract_print_statements(code);
        
        // Process print outputs first
        let mut messages = Vec::new();
        for output in print_outputs {
            if let Some(msg) = parse_print_output(&output) {
                messages.push(msg);
            }
        }

        // Execute robot function calls
        let mut robot_results = Vec::new();
        for call in function_calls {
            let result = executor.execute_function(&mut game_state, call);
            robot_results.push(result.clone());
            
            if self.config.enable_logging {
                log::info!("Executed function: {}", result);
            }
        }

        // Add robot action messages if any
        if !robot_results.is_empty() {
            let meaningful_results: Vec<String> = robot_results
                .into_iter()
                .filter(|r| !r.is_empty() && !r.contains("executed"))
                .collect();
            
            if !meaningful_results.is_empty() {
                messages.push(GameMessage::robot_action(meaningful_results.join("\n")));
            }
        }

        Ok(TestResult {
            success: true,
            final_position: game_state.robot_position,
            turns_taken: game_state.turns,
            messages,
            execution_output: format!("{:?}", function_calls),
            error: None,
        })
    }
}

/// Internal game state for testing
#[derive(Debug)]
pub struct GameState {
    pub robot_position: Position,
    pub turns: u32,
    pub grid: TestGrid,
}

impl GameState {
    pub fn new(config: &GameConfig) -> Self {
        Self {
            robot_position: Position::new(config.robot_start_x, config.robot_start_y),
            turns: 0,
            grid: TestGrid::new(config.grid_width, config.grid_height),
        }
    }
}

/// Parse print output into a message
fn parse_print_output(output: &str) -> Option<GameMessage> {
    if let Some(content) = output.strip_prefix("stdout: ") {
        Some(GameMessage::stdout(content.to_string()))
    } else if let Some(content) = output.strip_prefix("stderr: ") {
        Some(GameMessage::stderr(content.to_string()))
    } else {
        None
    }
}
