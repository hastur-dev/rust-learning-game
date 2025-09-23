//! Level 21, Task 1: Result Type Basics
//!
//! This module demonstrates the fundamentals of Rust's Result type for error handling
//! in robot control systems, showing how to work with fallible operations.
//!
//! Learning objectives:
//! - Understand Result<T, E> for error handling
//! - Learn to create and return Result values
//! - Master pattern matching with Result
//! - Use Result methods for transformation and control flow
//! - Handle errors gracefully in robot operations

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// Custom error types for robot operations
#[derive(Debug, Clone, PartialEq)]
pub enum RobotError {
    InvalidCommand(String),
    HardwareFailure(String),
    OutOfBounds { x: f64, y: f64 },
    BatteryLow { current: f64, minimum: f64 },
    SensorError(String),
    CommunicationTimeout,
    CalibrationFailed,
}

impl fmt::Display for RobotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RobotError::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
            RobotError::HardwareFailure(msg) => write!(f, "Hardware failure: {}", msg),
            RobotError::OutOfBounds { x, y } => write!(f, "Position out of bounds: ({}, {})", x, y),
            RobotError::BatteryLow { current, minimum } => {
                write!(f, "Battery low: {}% (minimum: {}%)", current, minimum)
            }
            RobotError::SensorError(sensor) => write!(f, "Sensor error: {}", sensor),
            RobotError::CommunicationTimeout => write!(f, "Communication timeout"),
            RobotError::CalibrationFailed => write!(f, "Calibration failed"),
        }
    }
}

impl std::error::Error for RobotError {}

/// Position coordinates for robot movement
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl FromStr for Position {
    type Err = RobotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(',').collect();
        if parts.len() != 2 {
            return Err(RobotError::InvalidCommand(
                format!("Position must have format 'x,y', got: {}", s)
            ));
        }

        let x = parts[0].trim().parse::<f64>()
            .map_err(|_| RobotError::InvalidCommand(
                format!("Invalid x coordinate: {}", parts[0])
            ))?;

        let y = parts[1].trim().parse::<f64>()
            .map_err(|_| RobotError::InvalidCommand(
                format!("Invalid y coordinate: {}", parts[1])
            ))?;

        Ok(Position::new(x, y))
    }
}

/// Robot battery management
#[derive(Debug)]
pub struct Battery {
    charge_level: f64,
    capacity: f64,
    minimum_charge: f64,
}

impl Battery {
    pub fn new(capacity: f64, minimum_charge: f64) -> Self {
        Self {
            charge_level: capacity,
            capacity,
            minimum_charge,
        }
    }

    pub fn get_charge_percentage(&self) -> f64 {
        (self.charge_level / self.capacity) * 100.0
    }

    pub fn consume_power(&mut self, amount: f64) -> Result<(), RobotError> {
        if self.charge_level - amount < 0.0 {
            return Err(RobotError::BatteryLow {
                current: self.get_charge_percentage(),
                minimum: 0.0,
            });
        }

        self.charge_level -= amount;

        if self.get_charge_percentage() < self.minimum_charge {
            Err(RobotError::BatteryLow {
                current: self.get_charge_percentage(),
                minimum: self.minimum_charge,
            })
        } else {
            Ok(())
        }
    }

    pub fn charge(&mut self, amount: f64) -> Result<f64, RobotError> {
        if amount < 0.0 {
            return Err(RobotError::InvalidCommand("Charge amount cannot be negative".to_string()));
        }

        let old_level = self.charge_level;
        self.charge_level = (self.charge_level + amount).min(self.capacity);
        Ok(self.charge_level - old_level)
    }

    pub fn is_low(&self) -> bool {
        self.get_charge_percentage() < self.minimum_charge
    }
}

/// Sensor readings for robot navigation
#[derive(Debug, Clone)]
pub struct SensorReading {
    pub sensor_id: String,
    pub value: f64,
    pub timestamp: u64,
    pub valid: bool,
}

impl SensorReading {
    pub fn new(sensor_id: String, value: f64, timestamp: u64) -> Self {
        Self {
            sensor_id,
            value,
            timestamp,
            valid: true,
        }
    }

    pub fn validate_range(&self, min: f64, max: f64) -> Result<(), RobotError> {
        if !self.valid {
            return Err(RobotError::SensorError(
                format!("Sensor {} reading is marked invalid", self.sensor_id)
            ));
        }

        if self.value < min || self.value > max {
            Err(RobotError::SensorError(
                format!("Sensor {} value {} out of range [{}, {}]",
                        self.sensor_id, self.value, min, max)
            ))
        } else {
            Ok(())
        }
    }
}

/// Basic robot controller demonstrating Result usage
#[derive(Debug)]
pub struct RobotController {
    position: Position,
    battery: Battery,
    boundaries: (Position, Position), // (min, max)
    sensor_readings: HashMap<String, SensorReading>,
    calibrated: bool,
    max_speed: f64,
}

impl RobotController {
    pub fn new(initial_position: Position, battery_capacity: f64) -> Self {
        Self {
            position: initial_position,
            battery: Battery::new(battery_capacity, 20.0), // 20% minimum charge
            boundaries: (Position::new(-100.0, -100.0), Position::new(100.0, 100.0)),
            sensor_readings: HashMap::new(),
            calibrated: false,
            max_speed: 10.0,
        }
    }

    /// Move robot to a new position, returning Result for error handling
    pub fn move_to(&mut self, target: Position) -> Result<Position, RobotError> {
        // Check if target is within boundaries
        self.validate_position(&target)?;

        // Calculate power consumption based on distance
        let distance = self.position.distance_to(&target);
        let power_needed = distance * 0.1; // 0.1 units per distance unit

        // Check and consume battery power
        self.battery.consume_power(power_needed)?;

        // Update position
        let old_position = self.position.clone();
        self.position = target;

        Ok(old_position)
    }

    /// Validate that a position is within robot boundaries
    pub fn validate_position(&self, pos: &Position) -> Result<(), RobotError> {
        let (min_bounds, max_bounds) = &self.boundaries;

        if pos.x < min_bounds.x || pos.x > max_bounds.x ||
           pos.y < min_bounds.y || pos.y > max_bounds.y {
            Err(RobotError::OutOfBounds { x: pos.x, y: pos.y })
        } else {
            Ok(())
        }
    }

    /// Set robot boundaries, returning the old boundaries
    pub fn set_boundaries(&mut self, min: Position, max: Position) -> Result<(Position, Position), RobotError> {
        // Validate that min is actually less than max
        if min.x >= max.x || min.y >= max.y {
            return Err(RobotError::InvalidCommand(
                "Minimum boundaries must be less than maximum boundaries".to_string()
            ));
        }

        // Check if current position would be valid with new boundaries
        if self.position.x < min.x || self.position.x > max.x ||
           self.position.y < min.y || self.position.y > max.y {
            return Err(RobotError::OutOfBounds {
                x: self.position.x,
                y: self.position.y,
            });
        }

        let old_boundaries = self.boundaries.clone();
        self.boundaries = (min, max);
        Ok(old_boundaries)
    }

    /// Calibrate robot systems
    pub fn calibrate(&mut self) -> Result<(), RobotError> {
        // Simulate calibration process that might fail
        if self.battery.get_charge_percentage() < 50.0 {
            return Err(RobotError::BatteryLow {
                current: self.battery.get_charge_percentage(),
                minimum: 50.0,
            });
        }

        // Consume power for calibration
        self.battery.consume_power(5.0)?;

        // Simulate calibration that might fail
        if rand::random() < 0.1 {
            Err(RobotError::CalibrationFailed)
        } else {
            self.calibrated = true;
            Ok(())
        }
    }

    /// Add sensor reading with validation
    pub fn add_sensor_reading(&mut self, sensor_id: String, value: f64, timestamp: u64) -> Result<(), RobotError> {
        let reading = SensorReading::new(sensor_id.clone(), value, timestamp);

        // Validate sensor reading based on sensor type
        match sensor_id.as_str() {
            "temperature" => reading.validate_range(-40.0, 85.0)?,
            "distance" => reading.validate_range(0.0, 1000.0)?,
            "battery_voltage" => reading.validate_range(0.0, 24.0)?,
            "gyroscope" => reading.validate_range(-180.0, 180.0)?,
            _ => {} // Unknown sensors are allowed without validation
        }

        self.sensor_readings.insert(sensor_id, reading);
        Ok(())
    }

    /// Get sensor reading with error handling
    pub fn get_sensor_reading(&self, sensor_id: &str) -> Result<&SensorReading, RobotError> {
        self.sensor_readings.get(sensor_id)
            .ok_or_else(|| RobotError::SensorError(
                format!("Sensor '{}' not found", sensor_id)
            ))
    }

    /// Execute a command string, demonstrating Result composition
    pub fn execute_command(&mut self, command: &str) -> Result<String, RobotError> {
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Err(RobotError::InvalidCommand("Empty command".to_string()));
        }

        match parts[0].to_lowercase().as_str() {
            "move" => {
                if parts.len() != 2 {
                    return Err(RobotError::InvalidCommand(
                        "Move command requires position (x,y)".to_string()
                    ));
                }

                let target = Position::from_str(parts[1])?;
                let old_pos = self.move_to(target)?;
                Ok(format!("Moved from ({}, {}) to ({}, {})",
                          old_pos.x, old_pos.y, self.position.x, self.position.y))
            }
            "calibrate" => {
                self.calibrate()?;
                Ok("Calibration completed successfully".to_string())
            }
            "status" => {
                Ok(self.get_status())
            }
            "charge" => {
                if parts.len() != 2 {
                    return Err(RobotError::InvalidCommand(
                        "Charge command requires amount".to_string()
                    ));
                }

                let amount = parts[1].parse::<f64>()
                    .map_err(|_| RobotError::InvalidCommand(
                        format!("Invalid charge amount: {}", parts[1])
                    ))?;

                let charged = self.battery.charge(amount)?;
                Ok(format!("Charged {:.2} units, battery now at {:.1}%",
                          charged, self.battery.get_charge_percentage()))
            }
            _ => Err(RobotError::InvalidCommand(
                format!("Unknown command: {}", parts[0])
            ))
        }
    }

    pub fn get_status(&self) -> String {
        format!(
            "Robot Status:\n\
             - Position: ({:.2}, {:.2})\n\
             - Battery: {:.1}%\n\
             - Calibrated: {}\n\
             - Sensors: {}",
            self.position.x,
            self.position.y,
            self.battery.get_charge_percentage(),
            self.calibrated,
            self.sensor_readings.len()
        )
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_battery_level(&self) -> f64 {
        self.battery.get_charge_percentage()
    }

    pub fn is_calibrated(&self) -> bool {
        self.calibrated
    }
}

/// Utility functions demonstrating Result operations
pub mod result_utils {
    use super::*;

    /// Safe division that returns Result
    pub fn safe_divide(a: f64, b: f64) -> Result<f64, RobotError> {
        if b == 0.0 {
            Err(RobotError::InvalidCommand("Division by zero".to_string()))
        } else {
            Ok(a / b)
        }
    }

    /// Parse multiple coordinates from a string
    pub fn parse_coordinates(input: &str) -> Result<Vec<Position>, RobotError> {
        let mut positions = Vec::new();

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let pos = Position::from_str(line)?;
            positions.push(pos);
        }

        if positions.is_empty() {
            Err(RobotError::InvalidCommand("No valid coordinates found".to_string()))
        } else {
            Ok(positions)
        }
    }

    /// Calculate average position from a list, with error handling
    pub fn average_position(positions: &[Position]) -> Result<Position, RobotError> {
        if positions.is_empty() {
            return Err(RobotError::InvalidCommand("Cannot average empty position list".to_string()));
        }

        let sum_x: f64 = positions.iter().map(|p| p.x).sum();
        let sum_y: f64 = positions.iter().map(|p| p.y).sum();
        let count = positions.len() as f64;

        Ok(Position::new(sum_x / count, sum_y / count))
    }

    /// Validate a path of positions
    pub fn validate_path(positions: &[Position], max_distance: f64) -> Result<f64, RobotError> {
        if positions.len() < 2 {
            return Err(RobotError::InvalidCommand("Path must have at least 2 positions".to_string()));
        }

        let mut total_distance = 0.0;

        for window in positions.windows(2) {
            let distance = window[0].distance_to(&window[1]);
            if distance > max_distance {
                return Err(RobotError::InvalidCommand(
                    format!("Distance {:.2} exceeds maximum {:.2}", distance, max_distance)
                ));
            }
            total_distance += distance;
        }

        Ok(total_distance)
    }

    /// Try to execute multiple operations, collecting errors
    pub fn execute_operations<F>(operations: Vec<F>) -> Result<Vec<String>, Vec<RobotError>>
    where
        F: FnOnce() -> Result<String, RobotError>,
    {
        let mut results = Vec::new();
        let mut errors = Vec::new();

        for operation in operations {
            match operation() {
                Ok(result) => results.push(result),
                Err(error) => errors.push(error),
            }
        }

        if errors.is_empty() {
            Ok(results)
        } else {
            Err(errors)
        }
    }
}

/// Simple random number generator for testing
mod rand {
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEED: AtomicU64 = AtomicU64::new(1);

    pub fn random() -> f64 {
        let mut seed = SEED.load(Ordering::Relaxed);
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        SEED.store(seed, Ordering::Relaxed);

        (seed as f64) / (u64::MAX as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::result_utils::*;

    #[test]
    fn test_position_creation_and_parsing() {
        let pos = Position::new(10.0, 20.0);
        assert_eq!(pos.x, 10.0);
        assert_eq!(pos.y, 20.0);

        // Test successful parsing
        let parsed = Position::from_str("15.5, 25.5").unwrap();
        assert_eq!(parsed.x, 15.5);
        assert_eq!(parsed.y, 25.5);

        // Test parsing errors
        assert!(Position::from_str("10").is_err());
        assert!(Position::from_str("10,abc").is_err());
        assert!(Position::from_str("").is_err());
    }

    #[test]
    fn test_position_distance() {
        let pos1 = Position::new(0.0, 0.0);
        let pos2 = Position::new(3.0, 4.0);
        assert_eq!(pos1.distance_to(&pos2), 5.0);
    }

    #[test]
    fn test_battery_operations() {
        let mut battery = Battery::new(100.0, 20.0);

        // Test initial state
        assert_eq!(battery.get_charge_percentage(), 100.0);
        assert!(!battery.is_low());

        // Test power consumption
        assert!(battery.consume_power(50.0).is_ok());
        assert_eq!(battery.get_charge_percentage(), 50.0);

        // Test low battery warning
        assert!(battery.consume_power(35.0).is_err());
        assert!(battery.is_low());

        // Test charging
        let charged = battery.charge(30.0).unwrap();
        assert_eq!(charged, 30.0);
        assert!(!battery.is_low());

        // Test invalid charge
        assert!(battery.charge(-10.0).is_err());
    }

    #[test]
    fn test_sensor_reading_validation() {
        let reading = SensorReading::new("test_sensor".to_string(), 25.0, 1000);

        // Test valid range
        assert!(reading.validate_range(0.0, 50.0).is_ok());

        // Test invalid range
        assert!(reading.validate_range(30.0, 50.0).is_err());
        assert!(reading.validate_range(0.0, 20.0).is_err());

        // Test invalid sensor reading
        let mut invalid_reading = reading.clone();
        invalid_reading.valid = false;
        assert!(invalid_reading.validate_range(0.0, 50.0).is_err());
    }

    #[test]
    fn test_robot_controller_movement() {
        let mut robot = RobotController::new(Position::new(0.0, 0.0), 100.0);

        // Test successful movement
        let target = Position::new(10.0, 10.0);
        let old_pos = robot.move_to(target).unwrap();
        assert_eq!(old_pos, Position::new(0.0, 0.0));
        assert_eq!(robot.get_position(), &Position::new(10.0, 10.0));

        // Test out of bounds movement
        let out_of_bounds = Position::new(200.0, 200.0);
        assert!(robot.move_to(out_of_bounds).is_err());
    }

    #[test]
    fn test_robot_controller_boundaries() {
        let mut robot = RobotController::new(Position::new(0.0, 0.0), 100.0);

        // Test valid boundary setting
        let new_min = Position::new(-50.0, -50.0);
        let new_max = Position::new(50.0, 50.0);
        let old_boundaries = robot.set_boundaries(new_min, new_max).unwrap();
        assert_eq!(old_boundaries.0, Position::new(-100.0, -100.0));

        // Test invalid boundary setting (min >= max)
        let invalid_min = Position::new(60.0, 60.0);
        let invalid_max = Position::new(50.0, 50.0);
        assert!(robot.set_boundaries(invalid_min, invalid_max).is_err());

        // Test boundary setting that would make current position invalid
        let restrictive_min = Position::new(10.0, 10.0);
        let restrictive_max = Position::new(20.0, 20.0);
        assert!(robot.set_boundaries(restrictive_min, restrictive_max).is_err());
    }

    #[test]
    fn test_robot_controller_calibration() {
        let mut robot = RobotController::new(Position::new(0.0, 0.0), 100.0);

        // Test successful calibration
        let result = robot.calibrate();
        // Result depends on random simulation, but should not panic

        // Test calibration with low battery
        robot.battery.charge_level = 30.0; // Set low charge
        assert!(robot.calibrate().is_err());
    }

    #[test]
    fn test_robot_controller_sensors() {
        let mut robot = RobotController::new(Position::new(0.0, 0.0), 100.0);

        // Test adding valid sensor reading
        assert!(robot.add_sensor_reading("temperature".to_string(), 25.0, 1000).is_ok());

        // Test adding invalid sensor reading
        assert!(robot.add_sensor_reading("temperature".to_string(), 100.0, 1001).is_err());

        // Test getting sensor reading
        let reading = robot.get_sensor_reading("temperature").unwrap();
        assert_eq!(reading.value, 25.0);

        // Test getting non-existent sensor
        assert!(robot.get_sensor_reading("non_existent").is_err());
    }

    #[test]
    fn test_robot_controller_commands() {
        let mut robot = RobotController::new(Position::new(0.0, 0.0), 100.0);

        // Test valid move command
        let result = robot.execute_command("move 10.0,20.0");
        assert!(result.is_ok());
        assert_eq!(robot.get_position(), &Position::new(10.0, 20.0));

        // Test invalid move command
        assert!(robot.execute_command("move").is_err());
        assert!(robot.execute_command("move invalid").is_err());

        // Test status command
        let status = robot.execute_command("status").unwrap();
        assert!(status.contains("Robot Status"));

        // Test charge command
        let charge_result = robot.execute_command("charge 10.0");
        assert!(charge_result.is_ok());

        // Test invalid charge command
        assert!(robot.execute_command("charge").is_err());
        assert!(robot.execute_command("charge abc").is_err());

        // Test unknown command
        assert!(robot.execute_command("unknown").is_err());

        // Test empty command
        assert!(robot.execute_command("").is_err());
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0).unwrap(), 5.0);
        assert!(safe_divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_parse_coordinates() {
        let input = "10.0, 20.0\n30.0, 40.0\n";
        let positions = parse_coordinates(input).unwrap();
        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0], Position::new(10.0, 20.0));
        assert_eq!(positions[1], Position::new(30.0, 40.0));

        // Test empty input
        assert!(parse_coordinates("").is_err());

        // Test invalid input
        assert!(parse_coordinates("invalid").is_err());
    }

    #[test]
    fn test_average_position() {
        let positions = vec![
            Position::new(0.0, 0.0),
            Position::new(10.0, 10.0),
            Position::new(20.0, 20.0),
        ];

        let avg = average_position(&positions).unwrap();
        assert_eq!(avg.x, 10.0);
        assert_eq!(avg.y, 10.0);

        // Test empty list
        assert!(average_position(&[]).is_err());
    }

    #[test]
    fn test_validate_path() {
        let positions = vec![
            Position::new(0.0, 0.0),
            Position::new(3.0, 4.0),  // Distance: 5.0
            Position::new(6.0, 8.0),  // Distance: 5.0
        ];

        // Test valid path
        let distance = validate_path(&positions, 10.0).unwrap();
        assert_eq!(distance, 10.0);

        // Test path with excessive distance
        assert!(validate_path(&positions, 3.0).is_err());

        // Test path with too few positions
        assert!(validate_path(&positions[0..1], 10.0).is_err());
    }

    #[test]
    fn test_execute_operations() {
        let operations = vec![
            || Ok("Success 1".to_string()),
            || Err(RobotError::InvalidCommand("Error 1".to_string())),
            || Ok("Success 2".to_string()),
            || Err(RobotError::CalibrationFailed),
        ];

        let result = execute_operations(operations);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);

        // Test all successful operations
        let successful_operations = vec![
            || Ok("Success 1".to_string()),
            || Ok("Success 2".to_string()),
        ];

        let result = execute_operations(successful_operations);
        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_error_display() {
        let error = RobotError::InvalidCommand("test".to_string());
        assert!(format!("{}", error).contains("Invalid command: test"));

        let error = RobotError::OutOfBounds { x: 10.0, y: 20.0 };
        assert!(format!("{}", error).contains("Position out of bounds: (10, 20)"));

        let error = RobotError::BatteryLow { current: 15.0, minimum: 20.0 };
        assert!(format!("{}", error).contains("Battery low: 15% (minimum: 20%)"));
    }

    #[test]
    fn test_result_composition() {
        let mut robot = RobotController::new(Position::new(0.0, 0.0), 100.0);

        // Test chaining operations with Results
        let result = Position::from_str("10.0, 20.0")
            .and_then(|pos| robot.move_to(pos))
            .map(|old_pos| format!("Moved from ({}, {})", old_pos.x, old_pos.y));

        assert!(result.is_ok());
        assert!(result.unwrap().contains("Moved from (0, 0)"));
    }

    #[test]
    fn test_error_propagation() {
        let mut robot = RobotController::new(Position::new(0.0, 0.0), 100.0);

        // Test that errors propagate correctly through function calls
        let result = robot.execute_command("move 200.0,200.0"); // Out of bounds
        assert!(result.is_err());

        match result.unwrap_err() {
            RobotError::OutOfBounds { .. } => {}, // Expected
            _ => panic!("Expected OutOfBounds error"),
        }
    }
}

/// Public module for student exercises
pub mod exercises {
    use super::*;

    /// Exercise 1: Implement a robot task scheduler
    ///
    /// Create a task scheduler that manages robot operations:
    /// - Tasks can succeed or fail (return Result)
    /// - Failed tasks should be retried up to N times
    /// - Tasks have dependencies (some tasks must complete before others)
    /// - The scheduler should handle task timeouts
    ///
    /// Requirements:
    /// - Use Result types for all fallible operations
    /// - Implement proper error handling and propagation
    /// - Create meaningful error types for different failure modes
    pub fn exercise_1_task_scheduler() {
        // TODO: Implement TaskScheduler struct
        // TODO: Add Task struct with dependencies and retry logic
        // TODO: Implement scheduling algorithm with error handling
        println!("Exercise 1: Implement robot task scheduler with Result types");
    }

    /// Exercise 2: Robot fleet management
    ///
    /// Create a system to manage multiple robots:
    /// - Add/remove robots from the fleet
    /// - Send commands to specific robots or groups
    /// - Aggregate results from multiple robot operations
    /// - Handle partial failures gracefully
    ///
    /// Requirements:
    /// - Use Result for all robot operations
    /// - Implement error aggregation for fleet operations
    /// - Handle communication failures between robots
    pub fn exercise_2_fleet_management() {
        // TODO: Implement RobotFleet struct
        // TODO: Add methods for fleet-wide operations
        // TODO: Implement error aggregation and partial success handling
        println!("Exercise 2: Implement robot fleet management with Result handling");
    }

    /// Exercise 3: Configuration management system
    ///
    /// Create a robust configuration system:
    /// - Load configuration from files with validation
    /// - Support different configuration formats (JSON, TOML, etc.)
    /// - Validate configuration values and provide meaningful errors
    /// - Support configuration hot-reloading with error recovery
    ///
    /// Requirements:
    /// - Comprehensive error types for different failure modes
    /// - Validation functions that return Results
    /// - Graceful fallback to default values when appropriate
    pub fn exercise_3_configuration_system() {
        // TODO: Implement ConfigurationManager struct
        // TODO: Add validation functions with detailed error messages
        // TODO: Implement hot-reloading with error recovery
        println!("Exercise 3: Implement configuration management with Result validation");
    }

    /// Exercise 4: Sensor fusion system
    ///
    /// Create a sensor fusion system that combines multiple sensor readings:
    /// - Validate individual sensor readings
    /// - Detect and handle sensor failures
    /// - Combine multiple sensor inputs with confidence weighting
    /// - Provide fallback readings when sensors fail
    ///
    /// Requirements:
    /// - Result-based sensor reading validation
    /// - Error handling for sensor communication failures
    /// - Graceful degradation when sensors are unavailable
    pub fn exercise_4_sensor_fusion() {
        // TODO: Implement SensorFusion struct
        // TODO: Add sensor validation and error detection
        // TODO: Implement fusion algorithms with error handling
        println!("Exercise 4: Implement sensor fusion system with Result handling");
    }

    /// Exercise 5: Robot mission planner
    ///
    /// Create a mission planning system:
    /// - Parse mission descriptions from various formats
    /// - Validate mission feasibility (battery, boundaries, capabilities)
    /// - Generate step-by-step execution plans
    /// - Handle mission replanning when errors occur
    ///
    /// Requirements:
    /// - Comprehensive mission validation with detailed error reporting
    /// - Result-based mission parsing and planning
    /// - Error recovery and replanning capabilities
    pub fn exercise_5_mission_planner() {
        // TODO: Implement MissionPlanner struct
        // TODO: Add mission parsing and validation
        // TODO: Implement replanning logic with error handling
        println!("Exercise 5: Implement robot mission planner with Result-based validation");
    }
}