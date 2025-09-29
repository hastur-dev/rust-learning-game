// Learning Tests for Level 22, Task 1: Anyhow Error Handling Basics
// Introduction to anyhow for simplified error handling in robot systems

use anyhow::{anyhow, bail, ensure, Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io;

// Robot system structs for error handling demonstrations
#[derive(Debug, Clone)]
pub struct Robot {
    pub id: u32,
    pub name: String,
    pub battery: f64,
    pub status: RobotStatus,
    pub sensors: Vec<Sensor>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RobotStatus {
    Active,
    Idle,
    Charging,
    Error,
    Maintenance,
}

#[derive(Debug, Clone)]
pub struct Sensor {
    pub name: String,
    pub value: f64,
    pub operational: bool,
}

impl Robot {
    pub fn new(id: u32, name: String) -> Self {
        Robot {
            id,
            name,
            battery: 100.0,
            status: RobotStatus::Idle,
            sensors: Vec::new(),
        }
    }

    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }
}

// Basic anyhow usage patterns
pub struct RobotManager {
    robots: HashMap<u32, Robot>,
}

impl RobotManager {
    pub fn new() -> Self {
        RobotManager {
            robots: HashMap::new(),
        }
    }

    // Basic anyhow::Result usage
    pub fn add_robot(&mut self, robot: Robot) -> Result<()> {
        if self.robots.contains_key(&robot.id) {
            bail!("Robot with ID {} already exists", robot.id);
        }

        if robot.name.is_empty() {
            return Err(anyhow!("Robot name cannot be empty"));
        }

        self.robots.insert(robot.id, robot);
        Ok(())
    }

    // Using ensure! macro for preconditions
    pub fn activate_robot(&mut self, robot_id: u32) -> Result<()> {
        let robot = self.robots.get_mut(&robot_id)
            .ok_or_else(|| anyhow!("Robot {} not found", robot_id))?;

        ensure!(robot.battery > 10.0, "Robot battery too low: {:.1}%", robot.battery);
        ensure!(robot.status != RobotStatus::Error, "Robot is in error state");
        ensure!(robot.status != RobotStatus::Maintenance, "Robot is in maintenance mode");

        robot.status = RobotStatus::Active;
        Ok(())
    }

    // Converting different error types with ?
    pub fn save_robot_config(&self, robot_id: u32, filename: &str) -> Result<()> {
        let robot = self.get_robot(robot_id)?;
        let config = self.serialize_robot_config(robot)?;

        // std::io::Error automatically converts to anyhow::Error
        fs::write(filename, config)?;
        Ok(())
    }

    // Helper methods that return different error types
    fn get_robot(&self, robot_id: u32) -> Result<&Robot> {
        self.robots.get(&robot_id)
            .ok_or_else(|| anyhow!("Robot {} not found", robot_id))
    }

    fn serialize_robot_config(&self, robot: &Robot) -> Result<String> {
        if robot.sensors.is_empty() {
            bail!("Cannot serialize robot without sensors");
        }

        // Simulate serialization that might fail
        let config = format!("Robot: {}\nBattery: {:.1}\nSensors: {}",
                            robot.name, robot.battery, robot.sensors.len());
        Ok(config)
    }

    // Multiple validation with early return
    pub fn validate_robot_fleet(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        for (id, robot) in &self.robots {
            if robot.battery < 20.0 {
                warnings.push(format!("Robot {} has low battery: {:.1}%", id, robot.battery));
            }

            if robot.sensors.is_empty() {
                warnings.push(format!("Robot {} has no sensors", id));
            }

            if robot.name.len() < 3 {
                bail!("Robot {} has invalid name: '{}'", id, robot.name);
            }

            // Check for non-operational sensors
            let broken_sensors: Vec<&Sensor> = robot.sensors.iter()
                .filter(|s| !s.operational)
                .collect();

            if broken_sensors.len() > robot.sensors.len() / 2 {
                return Err(anyhow!("Robot {} has too many broken sensors: {}/{}",
                                 id, broken_sensors.len(), robot.sensors.len()));
            }
        }

        Ok(warnings)
    }

    pub fn get_robot_count(&self) -> usize {
        self.robots.len()
    }
}

// Error propagation and handling patterns
pub struct RobotOperations;

impl RobotOperations {
    // Chain operations with automatic error propagation
    pub fn perform_maintenance(manager: &mut RobotManager, robot_id: u32) -> Result<String> {
        let robot = manager.robots.get_mut(&robot_id)
            .ok_or_else(|| anyhow!("Robot {} not found", robot_id))?;

        ensure!(robot.status != RobotStatus::Active, "Cannot maintain active robot");

        robot.status = RobotStatus::Maintenance;
        robot.battery = 100.0;

        // Reset all sensors
        for sensor in &mut robot.sensors {
            sensor.operational = true;
            sensor.value = 0.0;
        }

        Ok(format!("Maintenance completed for robot {}", robot_id))
    }

    // Handle file operations with context
    pub fn load_robot_database(filename: &str) -> Result<RobotManager> {
        let content = fs::read_to_string(filename)
            .with_context(|| format!("Failed to read robot database from {}", filename))?;

        Self::parse_robot_database(&content)
            .with_context(|| "Failed to parse robot database")
    }

    // Parse operations that can fail
    fn parse_robot_database(content: &str) -> Result<RobotManager> {
        let mut manager = RobotManager::new();

        for (line_num, line) in content.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            let robot = Self::parse_robot_line(line)
                .with_context(|| format!("Error parsing line {}", line_num + 1))?;

            manager.add_robot(robot)
                .with_context(|| format!("Failed to add robot from line {}", line_num + 1))?;
        }

        Ok(manager)
    }

    fn parse_robot_line(line: &str) -> Result<Robot> {
        let parts: Vec<&str> = line.split(',').collect();

        ensure!(parts.len() >= 3, "Invalid robot line format: expected at least 3 fields");

        let id = parts[0].trim().parse::<u32>()
            .with_context(|| format!("Invalid robot ID: '{}'", parts[0]))?;

        let name = parts[1].trim();
        ensure!(!name.is_empty(), "Robot name cannot be empty");

        let battery = parts[2].trim().parse::<f64>()
            .with_context(|| format!("Invalid battery level: '{}'", parts[2]))?;

        ensure!(battery >= 0.0 && battery <= 100.0,
               "Battery level must be between 0 and 100, got {}", battery);

        let mut robot = Robot::new(id, name.to_string());
        robot.battery = battery;

        Ok(robot)
    }

    // Fallible operations with recovery
    pub fn safe_robot_operation<F, T>(operation: F) -> Result<T>
    where
        F: FnOnce() -> Result<T>,
    {
        operation().with_context(|| "Robot operation failed")
    }

    // Convert standard library errors
    pub fn read_sensor_data(filename: &str) -> Result<Vec<f64>> {
        let content = fs::read_to_string(filename)?;

        let mut values = Vec::new();
        for line in content.lines() {
            let value = line.trim().parse::<f64>()
                .with_context(|| format!("Invalid sensor value: '{}'", line))?;
            values.push(value);
        }

        Ok(values)
    }
}

// Error creation and custom error messages
pub struct ErrorFactory;

impl ErrorFactory {
    // Create errors with formatted messages
    pub fn robot_not_found(robot_id: u32) -> anyhow::Error {
        anyhow!("Robot with ID {} was not found in the system", robot_id)
    }

    pub fn invalid_battery_level(level: f64) -> anyhow::Error {
        anyhow!("Invalid battery level: {:.1}% (must be between 0 and 100)", level)
    }

    pub fn sensor_malfunction(sensor_name: &str, error_code: u32) -> anyhow::Error {
        anyhow!("Sensor '{}' malfunction detected (error code: {})", sensor_name, error_code)
    }

    // Create errors with additional data
    pub fn communication_failure(robot_id: u32, retry_count: u32) -> anyhow::Error {
        anyhow!("Failed to communicate with robot {} after {} retries", robot_id, retry_count)
    }

    // Conditional error creation
    pub fn validate_robot_state(robot: &Robot) -> Result<()> {
        if robot.battery < 0.0 || robot.battery > 100.0 {
            return Err(Self::invalid_battery_level(robot.battery));
        }

        if robot.name.is_empty() {
            bail!("Robot name cannot be empty");
        }

        if robot.sensors.is_empty() {
            bail!("Robot must have at least one sensor");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_basic_anyhow_usage() {
        let mut manager = RobotManager::new();
        let robot = Robot::new(1, "TestBot".to_string());

        // Successful operation
        assert!(manager.add_robot(robot).is_ok());

        // Duplicate robot should fail
        let duplicate_robot = Robot::new(1, "AnotherBot".to_string());
        let result = manager.add_robot(duplicate_robot);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test]
    fn test_ensure_macro() {
        let mut manager = RobotManager::new();
        let mut robot = Robot::new(1, "TestBot".to_string());
        robot.battery = 5.0; // Low battery
        manager.add_robot(robot).unwrap();

        let result = manager.activate_robot(1);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("battery too low"));
    }

    #[test]
    fn test_error_propagation() {
        let mut manager = RobotManager::new();
        let robot = Robot::new(1, "TestBot".to_string());
        manager.add_robot(robot).unwrap();

        // This should fail because robot has no sensors
        let result = manager.save_robot_config(1, "/tmp/test_config.txt");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("without sensors"));
    }

    #[test]
    fn test_validation_with_warnings() {
        let mut manager = RobotManager::new();

        let mut robot1 = Robot::new(1, "Bot1".to_string());
        robot1.battery = 15.0; // Low battery
        robot1.add_sensor(Sensor {
            name: "camera".to_string(),
            value: 100.0,
            operational: true,
        });

        let mut robot2 = Robot::new(2, "Bot2".to_string());
        robot2.add_sensor(Sensor {
            name: "lidar".to_string(),
            value: 50.0,
            operational: true,
        });

        manager.add_robot(robot1).unwrap();
        manager.add_robot(robot2).unwrap();

        let warnings = manager.validate_robot_fleet().unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("low battery"));
    }

    #[test]
    fn test_validation_failure() {
        let mut manager = RobotManager::new();

        let robot = Robot::new(1, "X".to_string()); // Invalid name (too short)
        manager.add_robot(robot).unwrap();

        let result = manager.validate_robot_fleet();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid name"));
    }

    #[test]
    fn test_maintenance_operation() {
        let mut manager = RobotManager::new();
        let mut robot = Robot::new(1, "TestBot".to_string());
        robot.status = RobotStatus::Idle;
        robot.battery = 50.0;
        manager.add_robot(robot).unwrap();

        let result = RobotOperations::perform_maintenance(&mut manager, 1);
        assert!(result.is_ok());

        let robot = manager.robots.get(&1).unwrap();
        assert_eq!(robot.status, RobotStatus::Maintenance);
        assert_eq!(robot.battery, 100.0);
    }

    #[test]
    fn test_maintenance_on_active_robot() {
        let mut manager = RobotManager::new();
        let mut robot = Robot::new(1, "TestBot".to_string());
        robot.status = RobotStatus::Active;
        manager.add_robot(robot).unwrap();

        let result = RobotOperations::perform_maintenance(&mut manager, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot maintain active robot"));
    }

    #[test]
    fn test_database_parsing() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "1, Robot1, 85.5").unwrap();
        writeln!(temp_file, "2, Robot2, 92.0").unwrap();
        writeln!(temp_file, "").unwrap(); // Empty line should be ignored
        writeln!(temp_file, "3, Robot3, 78.2").unwrap();

        let result = RobotOperations::load_robot_database(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());

        let manager = result.unwrap();
        assert_eq!(manager.get_robot_count(), 3);
    }

    #[test]
    fn test_invalid_database_format() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "1, Robot1, 85.5").unwrap();
        writeln!(temp_file, "invalid_line").unwrap(); // Invalid format

        let result = RobotOperations::load_robot_database(temp_file.path().to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("line 2"));
    }

    #[test]
    fn test_sensor_data_reading() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "25.5").unwrap();
        writeln!(temp_file, "30.2").unwrap();
        writeln!(temp_file, "28.7").unwrap();

        let result = RobotOperations::read_sensor_data(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());

        let values = result.unwrap();
        assert_eq!(values, vec![25.5, 30.2, 28.7]);
    }

    #[test]
    fn test_invalid_sensor_data() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "25.5").unwrap();
        writeln!(temp_file, "invalid_number").unwrap();

        let result = RobotOperations::read_sensor_data(temp_file.path().to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid sensor value"));
    }

    #[test]
    fn test_error_factory() {
        let error = ErrorFactory::robot_not_found(42);
        assert!(error.to_string().contains("Robot with ID 42"));

        let error = ErrorFactory::invalid_battery_level(150.0);
        assert!(error.to_string().contains("150.0"));

        let error = ErrorFactory::sensor_malfunction("camera", 0x1234);
        assert!(error.to_string().contains("camera"));
        assert!(error.to_string().contains("4660")); // 0x1234 in decimal
    }

    #[test]
    fn test_robot_state_validation() {
        let mut robot = Robot::new(1, "TestBot".to_string());
        robot.add_sensor(Sensor {
            name: "test".to_string(),
            value: 0.0,
            operational: true,
        });

        // Valid robot
        assert!(ErrorFactory::validate_robot_state(&robot).is_ok());

        // Invalid battery
        robot.battery = -10.0;
        assert!(ErrorFactory::validate_robot_state(&robot).is_err());

        robot.battery = 50.0;

        // Empty name
        robot.name = String::new();
        assert!(ErrorFactory::validate_robot_state(&robot).is_err());

        robot.name = "TestBot".to_string();

        // No sensors
        robot.sensors.clear();
        assert!(ErrorFactory::validate_robot_state(&robot).is_err());
    }
}

// Student exercises
pub mod exercises {
    use super::*;

    // Exercise 1: Implement a configuration validator
    pub struct ConfigValidator;

    impl ConfigValidator {
        pub fn validate_robot_config(config: &str) -> Result<HashMap<String, String>> {
            // TODO: Parse and validate robot configuration string
            // Format: "key1=value1;key2=value2;..."
            // Required keys: name, battery, sensors
            // Use ensure! for validation
            unimplemented!("Implement config validation")
        }

        pub fn validate_sensor_config(config: &HashMap<String, String>) -> Result<Vec<Sensor>> {
            // TODO: Convert sensor configuration to Sensor objects
            // Validate sensor names and values
            unimplemented!("Implement sensor validation")
        }
    }

    // Exercise 2: Implement a robot fleet loader
    pub struct FleetLoader;

    impl FleetLoader {
        pub fn load_from_json(filename: &str) -> Result<RobotManager> {
            // TODO: Load robot fleet from JSON file
            // Handle file I/O errors and JSON parsing errors
            // Use .with_context() for error messages
            unimplemented!("Implement JSON fleet loader")
        }

        pub fn load_from_csv(filename: &str) -> Result<RobotManager> {
            // TODO: Load robot fleet from CSV file
            // Handle malformed CSV data
            unimplemented!("Implement CSV fleet loader")
        }

        pub fn save_fleet(manager: &RobotManager, filename: &str) -> Result<()> {
            // TODO: Save robot fleet to file
            // Handle write permissions and disk space errors
            unimplemented!("Implement fleet saving")
        }
    }

    // Exercise 3: Implement a robot command executor
    pub struct CommandExecutor;

    impl CommandExecutor {
        pub fn execute_command(manager: &mut RobotManager, robot_id: u32, command: &str) -> Result<String> {
            // TODO: Execute commands like "move", "charge", "scan"
            // Use anyhow for error handling
            // Commands: "move x y", "charge", "scan sensor_name"
            unimplemented!("Implement command execution")
        }

        pub fn batch_execute(manager: &mut RobotManager, commands: &[(u32, String)]) -> Result<Vec<String>> {
            // TODO: Execute multiple commands, stop on first error
            // Return results for successful commands
            unimplemented!("Implement batch execution")
        }
    }

    // Exercise 4: Implement a robot health checker
    pub struct HealthChecker;

    impl HealthChecker {
        pub fn comprehensive_health_check(robot: &Robot) -> Result<HealthReport> {
            // TODO: Perform comprehensive health check
            // Check battery, sensors, status, etc.
            // Return detailed health report
            unimplemented!("Implement health check")
        }

        pub fn fleet_health_summary(manager: &RobotManager) -> Result<FleetHealthSummary> {
            // TODO: Generate fleet-wide health summary
            // Include statistics and warnings
            unimplemented!("Implement fleet health summary")
        }
    }

    pub struct HealthReport {
        pub robot_id: u32,
        pub overall_status: String,
        pub warnings: Vec<String>,
        pub errors: Vec<String>,
    }

    pub struct FleetHealthSummary {
        pub total_robots: usize,
        pub healthy_robots: usize,
        pub robots_with_warnings: usize,
        pub robots_with_errors: usize,
        pub critical_issues: Vec<String>,
    }

    // Exercise 5: Implement a backup and recovery system
    pub struct BackupSystem;

    impl BackupSystem {
        pub fn create_backup(manager: &RobotManager, backup_path: &str) -> Result<String> {
            // TODO: Create backup of robot fleet
            // Include timestamp and metadata
            unimplemented!("Implement backup creation")
        }

        pub fn restore_from_backup(backup_path: &str) -> Result<RobotManager> {
            // TODO: Restore robot fleet from backup
            // Validate backup integrity
            unimplemented!("Implement backup restoration")
        }

        pub fn verify_backup(backup_path: &str) -> Result<BackupInfo> {
            // TODO: Verify backup file integrity
            // Return backup metadata
            unimplemented!("Implement backup verification")
        }
    }

    pub struct BackupInfo {
        pub timestamp: String,
        pub robot_count: usize,
        pub checksum: String,
        pub valid: bool,
    }
}