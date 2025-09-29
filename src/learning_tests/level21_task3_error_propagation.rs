//! Level 21, Task 3: Error Propagation with ? Operator
//!
//! This module demonstrates advanced error propagation techniques in Rust,
//! focusing on the ? operator and how to effectively chain fallible operations
//! in robot control systems.
//!
//! Learning objectives:
//! - Master the ? operator for error propagation
//! - Understand early returns and error bubbling
//! - Learn to combine Result and Option with ?
//! - Handle error conversion and From trait
//! - Implement complex error propagation chains

use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;
use std::io;

/// Comprehensive error types for robot operations
#[derive(Debug, Clone)]
pub enum RobotError {
    // Hardware errors
    MotorFailure { motor_id: u32, code: u16 },
    SensorError { sensor: String, details: String },
    BatteryError { level: f64, required: f64 },

    // Communication errors
    NetworkTimeout { timeout_ms: u64 },
    ProtocolError { expected: String, received: String },

    // Configuration errors
    InvalidConfiguration { field: String, value: String },
    MissingConfiguration(String),

    // Navigation errors
    OutOfBounds { x: f64, y: f64, bounds: String },
    PathNotFound { from: String, to: String },
    ObstacleDetected { position: String },

    // System errors
    InsufficientPermissions(String),
    ResourceUnavailable(String),
    SystemOverload { cpu_usage: f64, memory_usage: f64 },

    // Conversion errors
    ParseError(String),
    IoError(String),
    ConversionError(String),
}

impl fmt::Display for RobotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RobotError::MotorFailure { motor_id, code } => {
                write!(f, "Motor {} failure (code: 0x{:04X})", motor_id, code)
            }
            RobotError::SensorError { sensor, details } => {
                write!(f, "Sensor '{}' error: {}", sensor, details)
            }
            RobotError::BatteryError { level, required } => {
                write!(f, "Battery level {:.1}% insufficient (required: {:.1}%)", level, required)
            }
            RobotError::NetworkTimeout { timeout_ms } => {
                write!(f, "Network timeout after {}ms", timeout_ms)
            }
            RobotError::ProtocolError { expected, received } => {
                write!(f, "Protocol error: expected '{}', received '{}'", expected, received)
            }
            RobotError::InvalidConfiguration { field, value } => {
                write!(f, "Invalid configuration for '{}': '{}'", field, value)
            }
            RobotError::MissingConfiguration(field) => {
                write!(f, "Missing required configuration: '{}'", field)
            }
            RobotError::OutOfBounds { x, y, bounds } => {
                write!(f, "Position ({}, {}) is outside bounds: {}", x, y, bounds)
            }
            RobotError::PathNotFound { from, to } => {
                write!(f, "No path found from '{}' to '{}'", from, to)
            }
            RobotError::ObstacleDetected { position } => {
                write!(f, "Obstacle detected at position: {}", position)
            }
            RobotError::InsufficientPermissions(action) => {
                write!(f, "Insufficient permissions for action: {}", action)
            }
            RobotError::ResourceUnavailable(resource) => {
                write!(f, "Resource unavailable: {}", resource)
            }
            RobotError::SystemOverload { cpu_usage, memory_usage } => {
                write!(f, "System overload: CPU {:.1}%, Memory {:.1}%", cpu_usage, memory_usage)
            }
            RobotError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            RobotError::IoError(msg) => write!(f, "IO error: {}", msg),
            RobotError::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
        }
    }
}

impl std::error::Error for RobotError {}

// Implement From trait for automatic error conversion
impl From<ParseIntError> for RobotError {
    fn from(err: ParseIntError) -> Self {
        RobotError::ParseError(err.to_string())
    }
}

impl From<io::Error> for RobotError {
    fn from(err: io::Error) -> Self {
        RobotError::IoError(err.to_string())
    }
}

/// Robot position with validation
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn parse_from_string(s: &str) -> Result<Self, RobotError> {
        let parts: Vec<&str> = s.trim().split(',').collect();
        if parts.len() != 2 {
            return Err(RobotError::ParseError(
                format!("Expected 'x,y' format, got: '{}'", s)
            ));
        }

        let x = parts[0].trim().parse::<f64>()
            .map_err(|_| RobotError::ParseError(
                format!("Invalid x coordinate: '{}'", parts[0])
            ))?;

        let y = parts[1].trim().parse::<f64>()
            .map_err(|_| RobotError::ParseError(
                format!("Invalid y coordinate: '{}'", parts[1])
            ))?;

        Ok(Position::new(x, y))
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn validate_bounds(&self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Result<(), RobotError> {
        if self.x < min_x || self.x > max_x || self.y < min_y || self.y > max_y {
            Err(RobotError::OutOfBounds {
                x: self.x,
                y: self.y,
                bounds: format!("[{}, {}] x [{}, {}]", min_x, max_x, min_y, max_y),
            })
        } else {
            Ok(())
        }
    }
}

/// Configuration management with error propagation
#[derive(Debug, Clone)]
pub struct RobotConfig {
    settings: HashMap<String, String>,
}

impl RobotConfig {
    pub fn new() -> Self {
        Self {
            settings: HashMap::new(),
        }
    }

    pub fn load_from_string(&mut self, config_data: &str) -> Result<(), RobotError> {
        for line in config_data.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            self.parse_config_line(line)?; // ? propagates parse errors
        }
        Ok(())
    }

    fn parse_config_line(&mut self, line: &str) -> Result<(), RobotError> {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(RobotError::InvalidConfiguration {
                field: "line_format".to_string(),
                value: line.to_string(),
            });
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        if key.is_empty() {
            return Err(RobotError::InvalidConfiguration {
                field: "key".to_string(),
                value: "empty".to_string(),
            });
        }

        self.validate_config_value(key, value)?; // ? propagates validation errors
        self.settings.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn validate_config_value(&self, key: &str, value: &str) -> Result<(), RobotError> {
        match key {
            "max_speed" => {
                let speed: f64 = value.parse()
                    .map_err(|_| RobotError::InvalidConfiguration {
                        field: key.to_string(),
                        value: value.to_string(),
                    })?;

                if speed <= 0.0 || speed > 100.0 {
                    return Err(RobotError::InvalidConfiguration {
                        field: key.to_string(),
                        value: value.to_string(),
                    });
                }
            }
            "battery_capacity" => {
                let capacity: f64 = value.parse()
                    .map_err(|_| RobotError::InvalidConfiguration {
                        field: key.to_string(),
                        value: value.to_string(),
                    })?;

                if capacity <= 0.0 {
                    return Err(RobotError::InvalidConfiguration {
                        field: key.to_string(),
                        value: value.to_string(),
                    });
                }
            }
            "communication_timeout" => {
                let timeout: u64 = value.parse()
                    .map_err(|_| RobotError::InvalidConfiguration {
                        field: key.to_string(),
                        value: value.to_string(),
                    })?;

                if timeout < 100 || timeout > 30000 {
                    return Err(RobotError::InvalidConfiguration {
                        field: key.to_string(),
                        value: value.to_string(),
                    });
                }
            }
            _ => {} // Unknown keys are allowed
        }

        Ok(())
    }

    pub fn get_string(&self, key: &str) -> Result<&str, RobotError> {
        self.settings.get(key)
            .map(|s| s.as_str())
            .ok_or_else(|| RobotError::MissingConfiguration(key.to_string()))
    }

    pub fn get_float(&self, key: &str) -> Result<f64, RobotError> {
        let value_str = self.get_string(key)?; // ? propagates missing config error
        value_str.parse::<f64>()
            .map_err(|_| RobotError::ConversionError(
                format!("Cannot convert '{}' to float for key '{}'", value_str, key)
            ))
    }

    pub fn get_int(&self, key: &str) -> Result<i64, RobotError> {
        let value_str = self.get_string(key)?; // ? propagates missing config error
        value_str.parse::<i64>()
            .map_err(|_| RobotError::ConversionError(
                format!("Cannot convert '{}' to integer for key '{}'", value_str, key)
            ))
    }

    pub fn get_bool(&self, key: &str) -> Result<bool, RobotError> {
        let value_str = self.get_string(key)?; // ? propagates missing config error
        match value_str.to_lowercase().as_str() {
            "true" | "yes" | "1" | "on" => Ok(true),
            "false" | "no" | "0" | "off" => Ok(false),
            _ => Err(RobotError::ConversionError(
                format!("Cannot convert '{}' to boolean for key '{}'", value_str, key)
            )),
        }
    }
}

/// Robot navigation system with error propagation
#[derive(Debug)]
pub struct NavigationSystem {
    current_position: Position,
    target_position: Option<Position>,
    waypoints: Vec<Position>,
    config: RobotConfig,
    obstacles: Vec<Position>,
}

impl NavigationSystem {
    pub fn new(initial_position: Position, config: RobotConfig) -> Self {
        Self {
            current_position: initial_position,
            target_position: None,
            waypoints: Vec::new(),
            config,
            obstacles: Vec::new(),
        }
    }

    /// Set target with full validation chain
    pub fn set_target(&mut self, target_str: &str) -> Result<(), RobotError> {
        let target = Position::parse_from_string(target_str)?; // ? propagates parse errors

        // Get bounds from config - multiple ? operations chained
        let min_x = self.config.get_float("min_x")?;
        let max_x = self.config.get_float("max_x")?;
        let min_y = self.config.get_float("min_y")?;
        let max_y = self.config.get_float("max_y")?;

        target.validate_bounds(min_x, max_x, min_y, max_y)?; // ? propagates bound errors

        self.check_path_obstacles(&self.current_position, &target)?; // ? propagates obstacle errors

        self.target_position = Some(target);
        Ok(())
    }

    /// Add waypoint with validation
    pub fn add_waypoint(&mut self, waypoint_str: &str) -> Result<(), RobotError> {
        let waypoint = Position::parse_from_string(waypoint_str)?; // ? propagates parse errors

        // Validate waypoint bounds
        let min_x = self.config.get_float("min_x")?;
        let max_x = self.config.get_float("max_x")?;
        let min_y = self.config.get_float("min_y")?;
        let max_y = self.config.get_float("max_y")?;

        waypoint.validate_bounds(min_x, max_x, min_y, max_y)?; // ? propagates validation errors

        self.waypoints.push(waypoint);
        Ok(())
    }

    /// Load multiple waypoints from string data
    pub fn load_waypoints_from_data(&mut self, data: &str) -> Result<usize, RobotError> {
        let mut count = 0;
        for line in data.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            self.add_waypoint(line)?; // ? propagates waypoint errors, stops on first error
            count += 1;
        }
        Ok(count)
    }

    /// Check for obstacles in path
    fn check_path_obstacles(&self, from: &Position, to: &Position) -> Result<(), RobotError> {
        let safety_distance = self.config.get_float("safety_distance")?; // ? propagates config errors

        for obstacle in &self.obstacles {
            let distance_to_line = self.point_to_line_distance(obstacle, from, to);
            if distance_to_line < safety_distance {
                return Err(RobotError::ObstacleDetected {
                    position: format!("({:.2}, {:.2})", obstacle.x, obstacle.y),
                });
            }
        }
        Ok(())
    }

    fn point_to_line_distance(&self, point: &Position, line_start: &Position, line_end: &Position) -> f64 {
        let a = line_end.y - line_start.y;
        let b = line_start.x - line_end.x;
        let c = line_end.x * line_start.y - line_start.x * line_end.y;

        (a * point.x + b * point.y + c).abs() / (a * a + b * b).sqrt()
    }

    /// Execute navigation plan with comprehensive error handling
    pub fn execute_navigation_plan(&mut self) -> Result<NavigationResult, RobotError> {
        self.validate_system_ready()?; // ? propagates system validation errors

        let max_speed = self.config.get_float("max_speed")?; // ? propagates config errors
        let battery_capacity = self.config.get_float("battery_capacity")?;

        let total_distance = self.calculate_total_path_distance()?; // ? propagates calculation errors
        let required_battery = self.estimate_battery_consumption(total_distance)?; // ? propagates estimation errors

        if required_battery > battery_capacity {
            return Err(RobotError::BatteryError {
                level: battery_capacity,
                required: required_battery,
            });
        }

        // Execute waypoints first
        let mut waypoint_results = Vec::new();
        for (i, waypoint) in self.waypoints.iter().enumerate() {
            let result = self.move_to_position(waypoint, max_speed)?; // ? stops execution on first waypoint error
            waypoint_results.push(result);
            self.current_position = waypoint.clone();
        }

        // Then move to final target if set
        let target_result = if let Some(target) = &self.target_position {
            Some(self.move_to_position(target, max_speed)?) // ? propagates movement errors
        } else {
            None
        };

        if let Some(target) = &self.target_position {
            self.current_position = target.clone();
        }

        Ok(NavigationResult {
            waypoint_results,
            target_result,
            total_distance,
            battery_consumed: required_battery,
        })
    }

    fn validate_system_ready(&self) -> Result<(), RobotError> {
        // Check if we have targets or waypoints
        if self.target_position.is_none() && self.waypoints.is_empty() {
            return Err(RobotError::InvalidConfiguration {
                field: "navigation_plan".to_string(),
                value: "no_targets".to_string(),
            });
        }

        // Validate required config is present
        self.config.get_float("max_speed")?; // ? propagates missing config
        self.config.get_float("battery_capacity")?;
        self.config.get_float("safety_distance")?;

        Ok(())
    }

    fn calculate_total_path_distance(&self) -> Result<f64, RobotError> {
        let mut total_distance = 0.0;
        let mut current_pos = &self.current_position;

        // Distance through waypoints
        for waypoint in &self.waypoints {
            total_distance += current_pos.distance_to(waypoint);
            current_pos = waypoint;
        }

        // Distance to final target
        if let Some(target) = &self.target_position {
            total_distance += current_pos.distance_to(target);
        }

        Ok(total_distance)
    }

    fn estimate_battery_consumption(&self, distance: f64) -> Result<f64, RobotError> {
        let consumption_rate = self.config.get_float("battery_consumption_rate")?; // ? propagates config errors
        Ok(distance * consumption_rate)
    }

    fn move_to_position(&self, position: &Position, max_speed: f64) -> Result<MovementResult, RobotError> {
        let distance = self.current_position.distance_to(position);
        let travel_time = distance / max_speed;

        // Check for obstacles one more time before moving
        self.check_path_obstacles(&self.current_position, position)?; // ? propagates obstacle errors

        Ok(MovementResult {
            from: self.current_position.clone(),
            to: position.clone(),
            distance,
            travel_time,
        })
    }

    pub fn add_obstacle(&mut self, obstacle_str: &str) -> Result<(), RobotError> {
        let obstacle = Position::parse_from_string(obstacle_str)?; // ? propagates parse errors
        self.obstacles.push(obstacle);
        Ok(())
    }

    pub fn get_current_position(&self) -> &Position {
        &self.current_position
    }

    pub fn clear_waypoints(&mut self) {
        self.waypoints.clear();
    }
}

#[derive(Debug)]
pub struct NavigationResult {
    pub waypoint_results: Vec<MovementResult>,
    pub target_result: Option<MovementResult>,
    pub total_distance: f64,
    pub battery_consumed: f64,
}

#[derive(Debug, Clone)]
pub struct MovementResult {
    pub from: Position,
    pub to: Position,
    pub distance: f64,
    pub travel_time: f64,
}

/// Robot command processor demonstrating error propagation chains
#[derive(Debug)]
pub struct CommandProcessor {
    navigation: NavigationSystem,
    permissions: HashMap<String, bool>,
}

impl CommandProcessor {
    pub fn new(initial_position: Position, config: RobotConfig) -> Self {
        let mut permissions = HashMap::new();
        permissions.insert("navigate".to_string(), true);
        permissions.insert("configure".to_string(), false);
        permissions.insert("shutdown".to_string(), false);

        Self {
            navigation: NavigationSystem::new(initial_position, config),
            permissions,
        }
    }

    /// Process command string with full error propagation
    pub fn process_command(&mut self, command: &str) -> Result<String, RobotError> {
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Err(RobotError::ParseError("Empty command".to_string()));
        }

        let cmd = parts[0].to_lowercase();
        self.check_permission(&cmd)?; // ? propagates permission errors

        match cmd.as_str() {
            "move" => self.process_move_command(&parts[1..]), // ? propagates from process_move_command
            "waypoint" => self.process_waypoint_command(&parts[1..]),
            "navigate" => self.process_navigate_command(),
            "obstacle" => self.process_obstacle_command(&parts[1..]),
            "status" => self.process_status_command(),
            _ => Err(RobotError::ParseError(format!("Unknown command: {}", cmd))),
        }
    }

    fn check_permission(&self, action: &str) -> Result<(), RobotError> {
        if !self.permissions.get(action).unwrap_or(&false) {
            Err(RobotError::InsufficientPermissions(action.to_string()))
        } else {
            Ok(())
        }
    }

    fn process_move_command(&mut self, args: &[&str]) -> Result<String, RobotError> {
        if args.is_empty() {
            return Err(RobotError::ParseError("Move command requires target position".to_string()));
        }

        let target_str = args.join(" ");
        self.navigation.set_target(&target_str)?; // ? propagates all navigation errors

        Ok(format!("Target set to: {}", target_str))
    }

    fn process_waypoint_command(&mut self, args: &[&str]) -> Result<String, RobotError> {
        if args.is_empty() {
            return Err(RobotError::ParseError("Waypoint command requires position".to_string()));
        }

        if args[0] == "clear" {
            self.navigation.clear_waypoints();
            return Ok("Waypoints cleared".to_string());
        }

        if args[0] == "load" {
            if args.len() < 2 {
                return Err(RobotError::ParseError("Load command requires data".to_string()));
            }

            let data = args[1..].join(" ");
            let count = self.navigation.load_waypoints_from_data(&data)?; // ? propagates loading errors
            return Ok(format!("Loaded {} waypoints", count));
        }

        let waypoint_str = args.join(" ");
        self.navigation.add_waypoint(&waypoint_str)?; // ? propagates waypoint errors

        Ok(format!("Waypoint added: {}", waypoint_str))
    }

    fn process_navigate_command(&mut self) -> Result<String, RobotError> {
        let result = self.navigation.execute_navigation_plan()?; // ? propagates all navigation errors

        let mut response = format!(
            "Navigation completed:\n- Total distance: {:.2}\n- Battery consumed: {:.2}",
            result.total_distance, result.battery_consumed
        );

        if !result.waypoint_results.is_empty() {
            response.push_str(&format!("\n- Waypoints visited: {}", result.waypoint_results.len()));
        }

        if result.target_result.is_some() {
            response.push_str("\n- Target reached");
        }

        Ok(response)
    }

    fn process_obstacle_command(&mut self, args: &[&str]) -> Result<String, RobotError> {
        if args.is_empty() {
            return Err(RobotError::ParseError("Obstacle command requires position".to_string()));
        }

        let obstacle_str = args.join(" ");
        self.navigation.add_obstacle(&obstacle_str)?; // ? propagates obstacle parsing errors

        Ok(format!("Obstacle added at: {}", obstacle_str))
    }

    fn process_status_command(&self) -> Result<String, RobotError> {
        let pos = self.navigation.get_current_position();
        Ok(format!("Current position: ({:.2}, {:.2})", pos.x, pos.y))
    }

    pub fn set_permission(&mut self, action: &str, allowed: bool) {
        self.permissions.insert(action.to_string(), allowed);
    }
}

/// Utility functions demonstrating error propagation patterns
pub mod propagation_utils {
    use super::*;

    /// Chain multiple operations with early returns
    pub fn validate_robot_config(config_data: &str) -> Result<RobotConfig, RobotError> {
        let mut config = RobotConfig::new();
        config.load_from_string(config_data)?; // ? propagates parsing errors

        // Validate required fields exist and are reasonable
        let max_speed = config.get_float("max_speed")?; // ? propagates missing/conversion errors
        let battery_capacity = config.get_float("battery_capacity")?;
        let safety_distance = config.get_float("safety_distance")?;

        if max_speed <= 0.0 {
            return Err(RobotError::InvalidConfiguration {
                field: "max_speed".to_string(),
                value: "must be positive".to_string(),
            });
        }

        if battery_capacity <= 0.0 {
            return Err(RobotError::InvalidConfiguration {
                field: "battery_capacity".to_string(),
                value: "must be positive".to_string(),
            });
        }

        if safety_distance < 0.0 {
            return Err(RobotError::InvalidConfiguration {
                field: "safety_distance".to_string(),
                value: "must be non-negative".to_string(),
            });
        }

        Ok(config)
    }

    /// Parse and validate a complete navigation plan
    pub fn parse_navigation_plan(plan_data: &str) -> Result<NavigationPlan, RobotError> {
        let lines: Vec<&str> = plan_data.lines().collect();
        if lines.is_empty() {
            return Err(RobotError::ParseError("Empty navigation plan".to_string()));
        }

        // First line should be start position
        let start_position = Position::parse_from_string(lines[0].trim())?; // ? propagates parse errors

        let mut waypoints = Vec::new();
        let mut target = None;

        for line in &lines[1..] {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("target:") {
                let target_str = line[7..].trim();
                target = Some(Position::parse_from_string(target_str)?); // ? propagates parse errors
            } else if line.starts_with("waypoint:") {
                let waypoint_str = line[9..].trim();
                waypoints.push(Position::parse_from_string(waypoint_str)?); // ? propagates parse errors
            } else {
                // Assume it's a waypoint without prefix
                waypoints.push(Position::parse_from_string(line)?); // ? propagates parse errors
            }
        }

        if target.is_none() && waypoints.is_empty() {
            return Err(RobotError::ParseError("Navigation plan must have target or waypoints".to_string()));
        }

        Ok(NavigationPlan {
            start_position,
            waypoints,
            target,
        })
    }

    /// Execute multiple robot commands in sequence
    pub fn execute_command_sequence(processor: &mut CommandProcessor, commands: &[&str]) -> Result<Vec<String>, RobotError> {
        let mut results = Vec::new();

        for command in commands {
            let result = processor.process_command(command)?; // ? stops on first error
            results.push(result);
        }

        Ok(results)
    }

    /// Try to execute commands with error recovery
    pub fn execute_with_recovery(processor: &mut CommandProcessor, commands: &[&str]) -> (Vec<String>, Vec<RobotError>) {
        let mut results = Vec::new();
        let mut errors = Vec::new();

        for command in commands {
            match processor.process_command(command) {
                Ok(result) => results.push(result),
                Err(error) => errors.push(error),
            }
        }

        (results, errors)
    }
}

#[derive(Debug)]
pub struct NavigationPlan {
    pub start_position: Position,
    pub waypoints: Vec<Position>,
    pub target: Option<Position>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::propagation_utils::*;

    #[test]
    fn test_position_parsing_with_propagation() {
        // Test successful parsing
        let pos = Position::parse_from_string("10.5, 20.3").unwrap();
        assert_eq!(pos.x, 10.5);
        assert_eq!(pos.y, 20.3);

        // Test error propagation
        let result = Position::parse_from_string("invalid");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::ParseError(_)));

        let result = Position::parse_from_string("10.0,abc");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::ParseError(_)));
    }

    #[test]
    fn test_position_bounds_validation() {
        let pos = Position::new(50.0, 75.0);

        // Test valid bounds
        assert!(pos.validate_bounds(0.0, 100.0, 0.0, 100.0).is_ok());

        // Test invalid bounds
        let result = pos.validate_bounds(0.0, 40.0, 0.0, 100.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::OutOfBounds { .. }));
    }

    #[test]
    fn test_config_loading_with_propagation() {
        let mut config = RobotConfig::new();

        let config_data = "max_speed=50.0\nbattery_capacity=100.0\nsafety_distance=5.0";
        assert!(config.load_from_string(config_data).is_ok());

        // Test invalid config
        let invalid_config = "max_speed=invalid_number";
        let result = config.load_from_string(invalid_config);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::InvalidConfiguration { .. }));
    }

    #[test]
    fn test_config_value_retrieval() {
        let mut config = RobotConfig::new();
        config.load_from_string("max_speed=50.0\nverbose=true\nretries=3").unwrap();

        // Test successful retrieval with conversion
        assert_eq!(config.get_float("max_speed").unwrap(), 50.0);
        assert_eq!(config.get_bool("verbose").unwrap(), true);
        assert_eq!(config.get_int("retries").unwrap(), 3);

        // Test missing config error propagation
        let result = config.get_float("missing_key");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::MissingConfiguration(_)));
    }

    #[test]
    fn test_navigation_system_target_setting() {
        let mut config = RobotConfig::new();
        config.load_from_string(
            "max_speed=10.0\nbattery_capacity=100.0\nsafety_distance=2.0\n\
             min_x=-50.0\nmax_x=50.0\nmin_y=-50.0\nmax_y=50.0"
        ).unwrap();

        let initial_pos = Position::new(0.0, 0.0);
        let mut nav = NavigationSystem::new(initial_pos, config);

        // Test successful target setting
        assert!(nav.set_target("10.0, 20.0").is_ok());

        // Test out of bounds error propagation
        let result = nav.set_target("100.0, 100.0");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::OutOfBounds { .. }));
    }

    #[test]
    fn test_waypoint_loading_with_propagation() {
        let mut config = RobotConfig::new();
        config.load_from_string(
            "max_speed=10.0\nbattery_capacity=100.0\nsafety_distance=2.0\n\
             min_x=-50.0\nmax_x=50.0\nmin_y=-50.0\nmax_y=50.0"
        ).unwrap();

        let initial_pos = Position::new(0.0, 0.0);
        let mut nav = NavigationSystem::new(initial_pos, config);

        let waypoint_data = "10.0, 10.0\n20.0, 20.0\n# comment\n30.0, 30.0";
        let count = nav.load_waypoints_from_data(waypoint_data).unwrap();
        assert_eq!(count, 3);

        // Test error propagation on invalid waypoint
        let invalid_data = "10.0, 10.0\ninvalid_waypoint\n30.0, 30.0";
        let result = nav.load_waypoints_from_data(invalid_data);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::ParseError(_)));
    }

    #[test]
    fn test_navigation_execution_with_propagation() {
        let mut config = RobotConfig::new();
        config.load_from_string(
            "max_speed=10.0\nbattery_capacity=100.0\nsafety_distance=2.0\n\
             battery_consumption_rate=0.1\n\
             min_x=-50.0\nmax_x=50.0\nmin_y=-50.0\nmax_y=50.0"
        ).unwrap();

        let initial_pos = Position::new(0.0, 0.0);
        let mut nav = NavigationSystem::new(initial_pos, config);

        nav.set_target("10.0, 10.0").unwrap();
        nav.add_waypoint("5.0, 5.0").unwrap();

        let result = nav.execute_navigation_plan();
        assert!(result.is_ok());

        let nav_result = result.unwrap();
        assert_eq!(nav_result.waypoint_results.len(), 1);
        assert!(nav_result.target_result.is_some());
    }

    #[test]
    fn test_obstacle_detection_propagation() {
        let mut config = RobotConfig::new();
        config.load_from_string(
            "max_speed=10.0\nbattery_capacity=100.0\nsafety_distance=5.0\n\
             min_x=-50.0\nmax_x=50.0\nmin_y=-50.0\nmax_y=50.0"
        ).unwrap();

        let initial_pos = Position::new(0.0, 0.0);
        let mut nav = NavigationSystem::new(initial_pos, config);

        // Add obstacle in the path
        nav.add_obstacle("5.0, 5.0").unwrap();

        // Try to set target that would pass through obstacle
        let result = nav.set_target("10.0, 10.0");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::ObstacleDetected { .. }));
    }

    #[test]
    fn test_command_processor_with_propagation() {
        let mut config = RobotConfig::new();
        config.load_from_string(
            "max_speed=10.0\nbattery_capacity=100.0\nsafety_distance=2.0\n\
             battery_consumption_rate=0.1\n\
             min_x=-50.0\nmax_x=50.0\nmin_y=-50.0\nmax_y=50.0"
        ).unwrap();

        let initial_pos = Position::new(0.0, 0.0);
        let mut processor = CommandProcessor::new(initial_pos, config);

        // Test successful command
        let result = processor.process_command("move 10.0, 10.0");
        assert!(result.is_ok());

        // Test permission error propagation
        let result = processor.process_command("configure max_speed 20.0");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::InsufficientPermissions(_)));

        // Test invalid command error propagation
        let result = processor.process_command("invalid_command");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::ParseError(_)));
    }

    #[test]
    fn test_command_sequence_execution() {
        let mut config = RobotConfig::new();
        config.load_from_string(
            "max_speed=10.0\nbattery_capacity=100.0\nsafety_distance=2.0\n\
             battery_consumption_rate=0.1\n\
             min_x=-50.0\nmax_x=50.0\nmin_y=-50.0\nmax_y=50.0"
        ).unwrap();

        let initial_pos = Position::new(0.0, 0.0);
        let mut processor = CommandProcessor::new(initial_pos, config);

        let commands = vec![
            "waypoint 5.0, 5.0",
            "move 10.0, 10.0",
            "navigate",
        ];

        let result = execute_command_sequence(&mut processor, &commands);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn test_error_propagation_with_recovery() {
        let mut config = RobotConfig::new();
        config.load_from_string(
            "max_speed=10.0\nbattery_capacity=100.0\nsafety_distance=2.0\n\
             min_x=-50.0\nmax_x=50.0\nmin_y=-50.0\nmax_y=50.0"
        ).unwrap();

        let initial_pos = Position::new(0.0, 0.0);
        let mut processor = CommandProcessor::new(initial_pos, config);

        let commands = vec![
            "move 10.0, 10.0",      // Should succeed
            "invalid_command",       // Should fail
            "status",               // Should succeed
            "move 200.0, 200.0",    // Should fail (out of bounds)
        ];

        let (results, errors) = execute_with_recovery(&mut processor, &commands);
        assert_eq!(results.len(), 2); // Two successful commands
        assert_eq!(errors.len(), 2);  // Two failed commands
    }

    #[test]
    fn test_config_validation_utility() {
        let valid_config = "max_speed=10.0\nbattery_capacity=100.0\nsafety_distance=2.0";
        let result = validate_robot_config(valid_config);
        assert!(result.is_ok());

        let invalid_config = "max_speed=-5.0\nbattery_capacity=100.0\nsafety_distance=2.0";
        let result = validate_robot_config(invalid_config);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::InvalidConfiguration { .. }));
    }

    #[test]
    fn test_navigation_plan_parsing() {
        let plan_data = "0.0, 0.0\nwaypoint: 5.0, 5.0\nwaypoint: 10.0, 10.0\ntarget: 20.0, 20.0";
        let result = parse_navigation_plan(plan_data);
        assert!(result.is_ok());

        let plan = result.unwrap();
        assert_eq!(plan.start_position, Position::new(0.0, 0.0));
        assert_eq!(plan.waypoints.len(), 2);
        assert!(plan.target.is_some());

        // Test error propagation
        let invalid_plan = "invalid_start_position";
        let result = parse_navigation_plan(invalid_plan);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::ParseError(_)));
    }

    #[test]
    fn test_error_conversion_traits() {
        // Test ParseIntError conversion
        let parse_error: RobotError = "abc".parse::<i32>().unwrap_err().into();
        assert!(matches!(parse_error, RobotError::ParseError(_)));
    }
}

/// Public module for student exercises
pub mod exercises {
    use super::*;

    /// Exercise 1: Implement a robot mission validator
    ///
    /// Create a comprehensive mission validation system:
    /// - Parse mission files with complex syntax
    /// - Validate mission parameters (time, resources, capabilities)
    /// - Check for conflicts with existing missions
    /// - Use ? operator extensively for error propagation
    ///
    /// Requirements:
    /// - Chain multiple validation steps with ? operator
    /// - Implement custom From traits for error conversion
    /// - Handle complex error propagation scenarios
    pub fn exercise_1_mission_validator() {
        // TODO: Implement MissionValidator struct
        // TODO: Add complex validation chains with ? operator
        // TODO: Create custom error conversion implementations
        println!("Exercise 1: Implement mission validator with error propagation");
    }

    /// Exercise 2: Robot communication protocol handler
    ///
    /// Create a communication system that handles protocol parsing:
    /// - Parse binary protocol messages
    /// - Validate message checksums and headers
    /// - Handle protocol versioning and compatibility
    /// - Implement robust error propagation for network issues
    ///
    /// Requirements:
    /// - Use ? operator for parsing and validation chains
    /// - Implement error propagation across network boundaries
    /// - Handle protocol errors with specific error types
    pub fn exercise_2_protocol_handler() {
        // TODO: Implement ProtocolHandler struct
        // TODO: Add message parsing with ? propagation
        // TODO: Create network error handling system
        println!("Exercise 2: Implement protocol handler with error propagation");
    }

    /// Exercise 3: Resource allocation system
    ///
    /// Create a system for allocating robot resources:
    /// - Validate resource availability and constraints
    /// - Handle resource conflicts and dependencies
    /// - Implement rollback on allocation failures
    /// - Use error propagation for complex allocation chains
    ///
    /// Requirements:
    /// - Chain resource validation with ? operator
    /// - Implement transactional resource allocation
    /// - Handle complex dependency error propagation
    pub fn exercise_3_resource_allocation() {
        // TODO: Implement ResourceAllocator struct
        // TODO: Add validation chains with ? operator
        // TODO: Create transactional allocation system
        println!("Exercise 3: Implement resource allocation with error propagation");
    }

    /// Exercise 4: Robot behavior tree executor
    ///
    /// Create a behavior tree execution system:
    /// - Parse behavior tree definitions from files
    /// - Execute behavior nodes with error handling
    /// - Handle behavior failures and recovery strategies
    /// - Propagate errors through behavior tree hierarchy
    ///
    /// Requirements:
    /// - Use ? operator for behavior tree parsing and execution
    /// - Implement hierarchical error propagation
    /// - Handle behavior-specific error types
    pub fn exercise_4_behavior_tree_executor() {
        // TODO: Implement BehaviorTreeExecutor struct
        // TODO: Add behavior parsing with ? propagation
        // TODO: Create hierarchical error handling
        println!("Exercise 4: Implement behavior tree executor with error propagation");
    }

    /// Exercise 5: Distributed robot coordination
    ///
    /// Create a system for coordinating multiple robots:
    /// - Handle communication with multiple robot nodes
    /// - Validate coordination protocols and consensus
    /// - Handle network partitions and failures gracefully
    /// - Implement complex error propagation across distributed system
    ///
    /// Requirements:
    /// - Use ? operator for distributed operation chains
    /// - Implement consensus error propagation
    /// - Handle network-specific error scenarios
    pub fn exercise_5_distributed_coordination() {
        // TODO: Implement DistributedCoordinator struct
        // TODO: Add consensus protocols with ? propagation
        // TODO: Create distributed error handling system
        println!("Exercise 5: Implement distributed coordination with error propagation");
    }
}