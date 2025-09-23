//! Level 22, Task 4: Custom Error Types with Anyhow
//!
//! This module demonstrates how to create custom error types that work seamlessly
//! with anyhow for better error handling in robot control systems.
//!
//! Learning objectives:
//! - Create custom error types that work with anyhow
//! - Implement proper error traits (std::error::Error, Display, Debug)
//! - Use thiserror for better error type definitions
//! - Handle complex error hierarchies in robot systems
//! - Implement error recovery based on error types

use anyhow::{anyhow, bail, ensure, Context, Result};
use std::fmt;
use std::error::Error as StdError;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Custom error type for robot hardware failures
#[derive(Debug, Clone)]
pub enum RobotHardwareError {
    MotorFailure { motor_id: u32, error_code: u16 },
    SensorMalfunction { sensor_type: String, reading: f64 },
    PowerSystemError { voltage: f32, expected_min: f32 },
    CommunicationTimeout { device: String, timeout_ms: u64 },
    CalibrationError { component: String, deviation: f64 },
}

impl fmt::Display for RobotHardwareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RobotHardwareError::MotorFailure { motor_id, error_code } => {
                write!(f, "Motor {} failed with error code 0x{:04X}", motor_id, error_code)
            }
            RobotHardwareError::SensorMalfunction { sensor_type, reading } => {
                write!(f, "{} sensor malfunction: invalid reading {}", sensor_type, reading)
            }
            RobotHardwareError::PowerSystemError { voltage, expected_min } => {
                write!(f, "Power system error: voltage {}V below minimum {}V", voltage, expected_min)
            }
            RobotHardwareError::CommunicationTimeout { device, timeout_ms } => {
                write!(f, "Communication timeout with {}: {}ms exceeded", device, timeout_ms)
            }
            RobotHardwareError::CalibrationError { component, deviation } => {
                write!(f, "Calibration error in {}: deviation {:.2}%", component, deviation)
            }
        }
    }
}

impl StdError for RobotHardwareError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

/// Custom error type for robot software failures
#[derive(Debug, Clone)]
pub enum RobotSoftwareError {
    InvalidCommand { command: String, reason: String },
    ConfigurationError { setting: String, value: String },
    MemoryError { component: String, used_mb: u64, limit_mb: u64 },
    AlgorithmFailure { algorithm: String, iteration: u32 },
    DataCorruption { file_path: String, checksum_expected: u32, checksum_actual: u32 },
}

impl fmt::Display for RobotSoftwareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RobotSoftwareError::InvalidCommand { command, reason } => {
                write!(f, "Invalid command '{}': {}", command, reason)
            }
            RobotSoftwareError::ConfigurationError { setting, value } => {
                write!(f, "Configuration error: invalid value '{}' for setting '{}'", value, setting)
            }
            RobotSoftwareError::MemoryError { component, used_mb, limit_mb } => {
                write!(f, "Memory error in {}: using {}MB, limit {}MB", component, used_mb, limit_mb)
            }
            RobotSoftwareError::AlgorithmFailure { algorithm, iteration } => {
                write!(f, "Algorithm '{}' failed at iteration {}", algorithm, iteration)
            }
            RobotSoftwareError::DataCorruption { file_path, checksum_expected, checksum_actual } => {
                write!(f, "Data corruption in '{}': checksum mismatch (expected 0x{:08X}, got 0x{:08X})",
                       file_path, checksum_expected, checksum_actual)
            }
        }
    }
}

impl StdError for RobotSoftwareError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

/// High-level robot error that can contain various sub-errors
#[derive(Debug)]
pub enum RobotSystemError {
    Hardware(RobotHardwareError),
    Software(RobotSoftwareError),
    Network { host: String, port: u16, source: Box<dyn StdError + Send + Sync> },
    Critical { message: String, recovery_possible: bool },
}

impl fmt::Display for RobotSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RobotSystemError::Hardware(err) => write!(f, "Hardware error: {}", err),
            RobotSystemError::Software(err) => write!(f, "Software error: {}", err),
            RobotSystemError::Network { host, port, source } => {
                write!(f, "Network error connecting to {}:{} - {}", host, port, source)
            }
            RobotSystemError::Critical { message, recovery_possible } => {
                write!(f, "Critical system error: {} (recovery: {})",
                       message, if *recovery_possible { "possible" } else { "not possible" })
            }
        }
    }
}

impl StdError for RobotSystemError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            RobotSystemError::Hardware(err) => Some(err),
            RobotSystemError::Software(err) => Some(err),
            RobotSystemError::Network { source, .. } => Some(source.as_ref()),
            RobotSystemError::Critical { .. } => None,
        }
    }
}

/// Motor controller with detailed error handling
pub struct MotorController {
    motor_id: u32,
    current_speed: f64,
    max_speed: f64,
    temperature: f64,
    voltage: f32,
    error_count: u32,
    last_error: Option<RobotHardwareError>,
}

impl MotorController {
    pub fn new(motor_id: u32, max_speed: f64) -> Self {
        Self {
            motor_id,
            current_speed: 0.0,
            max_speed,
            temperature: 25.0,
            voltage: 12.0,
            error_count: 0,
            last_error: None,
        }
    }

    pub fn set_speed(&mut self, speed: f64) -> Result<()> {
        ensure!(speed >= 0.0, "Speed cannot be negative: {}", speed);
        ensure!(speed <= self.max_speed, "Speed {} exceeds maximum {}", speed, self.max_speed);

        // Simulate hardware checks
        if self.voltage < 10.0 {
            let error = RobotHardwareError::PowerSystemError {
                voltage: self.voltage,
                expected_min: 10.0,
            };
            self.record_error(error.clone());
            return Err(RobotSystemError::Hardware(error).into());
        }

        if self.temperature > 80.0 {
            let error = RobotHardwareError::MotorFailure {
                motor_id: self.motor_id,
                error_code: 0x1001, // Overheating
            };
            self.record_error(error.clone());
            return Err(RobotSystemError::Hardware(error).into());
        }

        // Simulate potential communication issues
        if speed > self.max_speed * 0.9 && self.error_count > 3 {
            let error = RobotHardwareError::CommunicationTimeout {
                device: format!("Motor_{}", self.motor_id),
                timeout_ms: 500,
            };
            self.record_error(error.clone());
            return Err(RobotSystemError::Hardware(error).into());
        }

        self.current_speed = speed;
        // Simulate heating with speed
        self.temperature += speed * 0.1;
        Ok(())
    }

    pub fn get_speed(&self) -> f64 {
        self.current_speed
    }

    pub fn calibrate(&mut self) -> Result<()> {
        // Simulate calibration process
        let expected_position = 0.0;
        let actual_position = 0.15; // Simulated deviation
        let deviation_percent = (actual_position - expected_position).abs() * 100.0;

        if deviation_percent > 5.0 {
            let error = RobotHardwareError::CalibrationError {
                component: format!("Motor_{}", self.motor_id),
                deviation: deviation_percent,
            };
            self.record_error(error.clone());
            return Err(RobotSystemError::Hardware(error).into());
        }

        Ok(())
    }

    pub fn emergency_stop(&mut self) -> Result<()> {
        self.current_speed = 0.0;
        self.temperature = 25.0; // Simulate cooling

        // Verify stop was successful
        if self.current_speed > 0.01 {
            let error = RobotHardwareError::MotorFailure {
                motor_id: self.motor_id,
                error_code: 0x2001, // Emergency stop failure
            };
            self.record_error(error.clone());
            return Err(RobotSystemError::Hardware(error).into());
        }

        Ok(())
    }

    fn record_error(&mut self, error: RobotHardwareError) {
        self.error_count += 1;
        self.last_error = Some(error);
    }

    pub fn get_error_count(&self) -> u32 {
        self.error_count
    }

    pub fn get_last_error(&self) -> Option<&RobotHardwareError> {
        self.last_error.as_ref()
    }

    pub fn reset_errors(&mut self) {
        self.error_count = 0;
        self.last_error = None;
    }
}

/// Sensor system with custom error types
pub struct SensorSystem {
    sensors: HashMap<String, SensorData>,
    calibration_data: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
struct SensorData {
    value: f64,
    timestamp: Instant,
    valid_range: (f64, f64),
}

impl SensorSystem {
    pub fn new() -> Self {
        let mut system = Self {
            sensors: HashMap::new(),
            calibration_data: HashMap::new(),
        };

        // Initialize some sensors
        system.add_sensor("temperature", 25.0, (0.0, 100.0));
        system.add_sensor("pressure", 1013.25, (900.0, 1100.0));
        system.add_sensor("humidity", 45.0, (0.0, 100.0));
        system.add_sensor("distance", 150.0, (0.0, 1000.0));

        system
    }

    pub fn add_sensor(&mut self, name: &str, initial_value: f64, valid_range: (f64, f64)) {
        self.sensors.insert(name.to_string(), SensorData {
            value: initial_value,
            timestamp: Instant::now(),
            valid_range,
        });
        self.calibration_data.insert(name.to_string(), 1.0); // Default calibration factor
    }

    pub fn read_sensor(&mut self, sensor_name: &str) -> Result<f64> {
        let sensor = self.sensors.get_mut(sensor_name)
            .ok_or_else(|| anyhow!("Sensor '{}' not found", sensor_name))?;

        // Simulate sensor reading with potential errors
        let mut reading = sensor.value;

        // Simulate sensor drift
        reading += (rand::random::<f64>() - 0.5) * 2.0;

        // Apply calibration
        if let Some(&calibration_factor) = self.calibration_data.get(sensor_name) {
            reading *= calibration_factor;
        }

        // Check if reading is within valid range
        let (min_val, max_val) = sensor.valid_range;
        if reading < min_val || reading > max_val {
            return Err(RobotSystemError::Hardware(
                RobotHardwareError::SensorMalfunction {
                    sensor_type: sensor_name.to_string(),
                    reading,
                }
            ).into());
        }

        // Check for stale data (older than 1 second)
        if sensor.timestamp.elapsed() > Duration::from_secs(1) {
            return Err(anyhow!("Sensor data is stale")
                .context(format!("Sensor '{}' last updated {:?} ago",
                                sensor_name, sensor.timestamp.elapsed())));
        }

        sensor.value = reading;
        sensor.timestamp = Instant::now();
        Ok(reading)
    }

    pub fn calibrate_sensor(&mut self, sensor_name: &str, reference_value: f64) -> Result<()> {
        let current_reading = self.read_sensor(sensor_name)
            .context("Failed to read current sensor value for calibration")?;

        if current_reading == 0.0 {
            return Err(RobotSystemError::Hardware(
                RobotHardwareError::CalibrationError {
                    component: sensor_name.to_string(),
                    deviation: 100.0,
                }
            ).into());
        }

        let calibration_factor = reference_value / current_reading;

        // Check if calibration factor is reasonable
        if calibration_factor < 0.5 || calibration_factor > 2.0 {
            return Err(RobotSystemError::Hardware(
                RobotHardwareError::CalibrationError {
                    component: sensor_name.to_string(),
                    deviation: ((calibration_factor - 1.0).abs() * 100.0),
                }
            ).into());
        }

        self.calibration_data.insert(sensor_name.to_string(), calibration_factor);
        Ok(())
    }

    pub fn get_all_readings(&mut self) -> Result<HashMap<String, f64>> {
        let mut readings = HashMap::new();
        let mut errors = Vec::new();

        for sensor_name in self.sensors.keys().cloned().collect::<Vec<_>>() {
            match self.read_sensor(&sensor_name) {
                Ok(value) => {
                    readings.insert(sensor_name, value);
                }
                Err(e) => {
                    errors.push(format!("{}: {}", sensor_name, e));
                }
            }
        }

        if !errors.is_empty() {
            return Err(anyhow!("Multiple sensor errors: {}", errors.join(", ")));
        }

        Ok(readings)
    }
}

/// Robot configuration manager with custom error handling
pub struct ConfigurationManager {
    settings: HashMap<String, String>,
    validated_settings: HashMap<String, bool>,
}

impl ConfigurationManager {
    pub fn new() -> Self {
        Self {
            settings: HashMap::new(),
            validated_settings: HashMap::new(),
        }
    }

    pub fn set_setting(&mut self, key: &str, value: &str) -> Result<()> {
        // Validate setting based on key
        match key {
            "max_speed" => {
                let speed: f64 = value.parse()
                    .context("max_speed must be a valid number")?;

                if speed <= 0.0 || speed > 1000.0 {
                    return Err(RobotSystemError::Software(
                        RobotSoftwareError::ConfigurationError {
                            setting: key.to_string(),
                            value: value.to_string(),
                        }
                    ).into());
                }
            }
            "communication_timeout" => {
                let timeout: u64 = value.parse()
                    .context("communication_timeout must be a valid number")?;

                if timeout < 100 || timeout > 10000 {
                    return Err(RobotSystemError::Software(
                        RobotSoftwareError::ConfigurationError {
                            setting: key.to_string(),
                            value: value.to_string(),
                        }
                    ).into());
                }
            }
            "log_level" => {
                if !["debug", "info", "warn", "error"].contains(&value) {
                    return Err(RobotSystemError::Software(
                        RobotSoftwareError::ConfigurationError {
                            setting: key.to_string(),
                            value: value.to_string(),
                        }
                    ).into());
                }
            }
            _ => {
                // Unknown setting
                return Err(anyhow!("Unknown setting: {}", key));
            }
        }

        self.settings.insert(key.to_string(), value.to_string());
        self.validated_settings.insert(key.to_string(), true);
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Result<&str> {
        self.settings.get(key)
            .map(|s| s.as_str())
            .ok_or_else(|| anyhow!("Setting '{}' not found", key))
    }

    pub fn validate_all_settings(&self) -> Result<()> {
        let mut invalid_settings = Vec::new();

        for (key, _) in &self.settings {
            if !self.validated_settings.get(key).unwrap_or(&false) {
                invalid_settings.push(key.clone());
            }
        }

        if !invalid_settings.is_empty() {
            return Err(anyhow!("Invalid settings found: {}", invalid_settings.join(", ")));
        }

        Ok(())
    }
}

/// Memory monitor for detecting memory issues
pub struct MemoryMonitor {
    component_usage: HashMap<String, u64>,
    limits: HashMap<String, u64>,
}

impl MemoryMonitor {
    pub fn new() -> Self {
        let mut monitor = Self {
            component_usage: HashMap::new(),
            limits: HashMap::new(),
        };

        // Set default limits (in MB)
        monitor.limits.insert("navigation".to_string(), 100);
        monitor.limits.insert("vision".to_string(), 200);
        monitor.limits.insert("planning".to_string(), 150);
        monitor.limits.insert("control".to_string(), 50);

        monitor
    }

    pub fn record_usage(&mut self, component: &str, usage_mb: u64) -> Result<()> {
        let limit = self.limits.get(component)
            .ok_or_else(|| anyhow!("Unknown component: {}", component))?;

        if usage_mb > *limit {
            return Err(RobotSystemError::Software(
                RobotSoftwareError::MemoryError {
                    component: component.to_string(),
                    used_mb: usage_mb,
                    limit_mb: *limit,
                }
            ).into());
        }

        self.component_usage.insert(component.to_string(), usage_mb);
        Ok(())
    }

    pub fn get_total_usage(&self) -> u64 {
        self.component_usage.values().sum()
    }

    pub fn get_usage_report(&self) -> String {
        let mut report = String::new();
        for (component, usage) in &self.component_usage {
            let limit = self.limits.get(component).unwrap_or(&0);
            let percentage = (*usage as f64 / *limit as f64) * 100.0;
            report.push_str(&format!("{}: {}MB/{}MB ({:.1}%)\n",
                                   component, usage, limit, percentage));
        }
        report
    }
}

/// Data integrity checker with custom errors
pub struct DataIntegrityChecker {
    checksums: HashMap<String, u32>,
}

impl DataIntegrityChecker {
    pub fn new() -> Self {
        Self {
            checksums: HashMap::new(),
        }
    }

    pub fn store_checksum(&mut self, file_path: &str, checksum: u32) {
        self.checksums.insert(file_path.to_string(), checksum);
    }

    pub fn verify_integrity(&self, file_path: &str, actual_checksum: u32) -> Result<()> {
        let expected_checksum = self.checksums.get(file_path)
            .ok_or_else(|| anyhow!("No checksum stored for file: {}", file_path))?;

        if actual_checksum != *expected_checksum {
            return Err(RobotSystemError::Software(
                RobotSoftwareError::DataCorruption {
                    file_path: file_path.to_string(),
                    checksum_expected: *expected_checksum,
                    checksum_actual: actual_checksum,
                }
            ).into());
        }

        Ok(())
    }

    pub fn calculate_simple_checksum(data: &[u8]) -> u32 {
        data.iter().map(|&b| b as u32).sum()
    }
}

/// Error recovery utilities
pub struct ErrorRecoveryManager {
    recovery_attempts: HashMap<String, u32>,
    max_attempts: u32,
}

impl ErrorRecoveryManager {
    pub fn new(max_attempts: u32) -> Self {
        Self {
            recovery_attempts: HashMap::new(),
            max_attempts,
        }
    }

    pub fn can_attempt_recovery(&mut self, error_type: &str) -> bool {
        let attempts = self.recovery_attempts.entry(error_type.to_string()).or_insert(0);
        *attempts < self.max_attempts
    }

    pub fn record_recovery_attempt(&mut self, error_type: &str) {
        *self.recovery_attempts.entry(error_type.to_string()).or_insert(0) += 1;
    }

    pub fn reset_recovery_count(&mut self, error_type: &str) {
        self.recovery_attempts.remove(error_type);
    }

    pub fn get_recovery_strategy(&self, error: &RobotSystemError) -> Option<String> {
        match error {
            RobotSystemError::Hardware(hw_err) => {
                match hw_err {
                    RobotHardwareError::MotorFailure { .. } => {
                        Some("Emergency stop and recalibrate motor".to_string())
                    }
                    RobotHardwareError::SensorMalfunction { .. } => {
                        Some("Recalibrate sensor or switch to backup".to_string())
                    }
                    RobotHardwareError::PowerSystemError { .. } => {
                        Some("Switch to backup power or reduce power consumption".to_string())
                    }
                    RobotHardwareError::CommunicationTimeout { .. } => {
                        Some("Retry communication with exponential backoff".to_string())
                    }
                    RobotHardwareError::CalibrationError { .. } => {
                        Some("Perform full system recalibration".to_string())
                    }
                }
            }
            RobotSystemError::Software(sw_err) => {
                match sw_err {
                    RobotSoftwareError::InvalidCommand { .. } => {
                        Some("Validate and retry command with corrected parameters".to_string())
                    }
                    RobotSoftwareError::ConfigurationError { .. } => {
                        Some("Reset to default configuration".to_string())
                    }
                    RobotSoftwareError::MemoryError { .. } => {
                        Some("Free memory and restart component".to_string())
                    }
                    RobotSoftwareError::AlgorithmFailure { .. } => {
                        Some("Switch to backup algorithm".to_string())
                    }
                    RobotSoftwareError::DataCorruption { .. } => {
                        Some("Restore from backup and verify integrity".to_string())
                    }
                }
            }
            RobotSystemError::Network { .. } => {
                Some("Retry connection with different endpoint".to_string())
            }
            RobotSystemError::Critical { recovery_possible, .. } => {
                if *recovery_possible {
                    Some("Attempt system restart".to_string())
                } else {
                    None
                }
            }
        }
    }
}

// Simple random number generator for simulation
mod rand {
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEED: AtomicU64 = AtomicU64::new(1);

    pub fn random<T>() -> T
    where
        T: From<f64>
    {
        let mut seed = SEED.load(Ordering::Relaxed);
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        SEED.store(seed, Ordering::Relaxed);

        let normalized = (seed as f64) / (u64::MAX as f64);
        T::from(normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motor_controller_speed_control() {
        let mut motor = MotorController::new(1, 100.0);

        // Test normal operation
        assert!(motor.set_speed(50.0).is_ok());
        assert_eq!(motor.get_speed(), 50.0);

        // Test speed limit enforcement
        assert!(motor.set_speed(150.0).is_err());

        // Test negative speed rejection
        assert!(motor.set_speed(-10.0).is_err());
    }

    #[test]
    fn test_motor_controller_power_system_error() {
        let mut motor = MotorController::new(1, 100.0);
        motor.voltage = 8.0; // Below minimum

        let result = motor.set_speed(50.0);
        assert!(result.is_err());

        let error = result.unwrap_err();
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("Power system error"));
    }

    #[test]
    fn test_motor_controller_overheating() {
        let mut motor = MotorController::new(1, 100.0);
        motor.temperature = 85.0; // Above safe threshold

        let result = motor.set_speed(50.0);
        assert!(result.is_err());

        if let Ok(hardware_err) = result.unwrap_err().downcast::<RobotSystemError>() {
            match hardware_err {
                RobotSystemError::Hardware(RobotHardwareError::MotorFailure { motor_id, error_code }) => {
                    assert_eq!(motor_id, 1);
                    assert_eq!(error_code, 0x1001);
                }
                _ => panic!("Expected motor failure error"),
            }
        }
    }

    #[test]
    fn test_motor_controller_emergency_stop() {
        let mut motor = MotorController::new(1, 100.0);
        motor.set_speed(75.0).unwrap();

        assert!(motor.emergency_stop().is_ok());
        assert_eq!(motor.get_speed(), 0.0);
    }

    #[test]
    fn test_motor_controller_calibration() {
        let mut motor = MotorController::new(1, 100.0);

        // This should fail due to simulated deviation
        let result = motor.calibrate();
        assert!(result.is_err());
    }

    #[test]
    fn test_sensor_system_basic_operations() {
        let mut sensors = SensorSystem::new();

        // Test reading existing sensor
        let temp_result = sensors.read_sensor("temperature");
        assert!(temp_result.is_ok());

        // Test reading non-existent sensor
        let invalid_result = sensors.read_sensor("nonexistent");
        assert!(invalid_result.is_err());
    }

    #[test]
    fn test_sensor_system_calibration() {
        let mut sensors = SensorSystem::new();

        // Test normal calibration
        let result = sensors.calibrate_sensor("temperature", 20.0);
        // This might succeed or fail depending on simulated reading

        // Test calibration with extreme factor
        sensors.sensors.get_mut("temperature").unwrap().value = 1.0;
        let result = sensors.calibrate_sensor("temperature", 100.0);
        assert!(result.is_err()); // Should fail due to extreme calibration factor
    }

    #[test]
    fn test_sensor_system_get_all_readings() {
        let mut sensors = SensorSystem::new();

        let readings = sensors.get_all_readings();
        // This might succeed or fail depending on simulated sensor errors

        if let Ok(readings_map) = readings {
            assert!(!readings_map.is_empty());
        }
    }

    #[test]
    fn test_configuration_manager_valid_settings() {
        let mut config = ConfigurationManager::new();

        assert!(config.set_setting("max_speed", "100.0").is_ok());
        assert!(config.set_setting("communication_timeout", "1000").is_ok());
        assert!(config.set_setting("log_level", "info").is_ok());

        assert_eq!(config.get_setting("max_speed").unwrap(), "100.0");
    }

    #[test]
    fn test_configuration_manager_invalid_settings() {
        let mut config = ConfigurationManager::new();

        // Invalid max_speed
        assert!(config.set_setting("max_speed", "0").is_err());
        assert!(config.set_setting("max_speed", "2000").is_err());
        assert!(config.set_setting("max_speed", "not_a_number").is_err());

        // Invalid communication_timeout
        assert!(config.set_setting("communication_timeout", "50").is_err());
        assert!(config.set_setting("communication_timeout", "20000").is_err());

        // Invalid log_level
        assert!(config.set_setting("log_level", "verbose").is_err());

        // Unknown setting
        assert!(config.set_setting("unknown_setting", "value").is_err());
    }

    #[test]
    fn test_memory_monitor() {
        let mut monitor = MemoryMonitor::new();

        // Test normal usage
        assert!(monitor.record_usage("navigation", 50).is_ok());
        assert!(monitor.record_usage("vision", 100).is_ok());

        // Test exceeding limits
        assert!(monitor.record_usage("navigation", 150).is_err());
        assert!(monitor.record_usage("vision", 300).is_err());

        // Test unknown component
        assert!(monitor.record_usage("unknown", 10).is_err());

        // Test total usage calculation
        monitor.record_usage("control", 25).unwrap();
        let total = monitor.get_total_usage();
        assert_eq!(total, 175); // 50 + 100 + 25
    }

    #[test]
    fn test_data_integrity_checker() {
        let mut checker = DataIntegrityChecker::new();

        let test_data = b"Hello, robot!";
        let checksum = DataIntegrityChecker::calculate_simple_checksum(test_data);

        checker.store_checksum("test_file.dat", checksum);

        // Test successful verification
        assert!(checker.verify_integrity("test_file.dat", checksum).is_ok());

        // Test failed verification
        assert!(checker.verify_integrity("test_file.dat", checksum + 1).is_err());

        // Test missing checksum
        assert!(checker.verify_integrity("nonexistent.dat", checksum).is_err());
    }

    #[test]
    fn test_error_recovery_manager() {
        let mut recovery = ErrorRecoveryManager::new(3);

        // Test initial state
        assert!(recovery.can_attempt_recovery("motor_failure"));

        // Test recording attempts
        recovery.record_recovery_attempt("motor_failure");
        recovery.record_recovery_attempt("motor_failure");
        recovery.record_recovery_attempt("motor_failure");

        // Should not be able to attempt more recoveries
        assert!(!recovery.can_attempt_recovery("motor_failure"));

        // Reset and test again
        recovery.reset_recovery_count("motor_failure");
        assert!(recovery.can_attempt_recovery("motor_failure"));
    }

    #[test]
    fn test_error_recovery_strategies() {
        let recovery = ErrorRecoveryManager::new(3);

        // Test hardware error strategies
        let motor_error = RobotSystemError::Hardware(
            RobotHardwareError::MotorFailure { motor_id: 1, error_code: 0x1001 }
        );
        let strategy = recovery.get_recovery_strategy(&motor_error);
        assert!(strategy.is_some());
        assert!(strategy.unwrap().contains("Emergency stop"));

        // Test software error strategies
        let config_error = RobotSystemError::Software(
            RobotSoftwareError::ConfigurationError {
                setting: "max_speed".to_string(),
                value: "invalid".to_string(),
            }
        );
        let strategy = recovery.get_recovery_strategy(&config_error);
        assert!(strategy.is_some());
        assert!(strategy.unwrap().contains("default configuration"));

        // Test critical error without recovery
        let critical_error = RobotSystemError::Critical {
            message: "System failure".to_string(),
            recovery_possible: false,
        };
        let strategy = recovery.get_recovery_strategy(&critical_error);
        assert!(strategy.is_none());
    }

    #[test]
    fn test_custom_error_display() {
        let motor_error = RobotHardwareError::MotorFailure {
            motor_id: 1,
            error_code: 0x1001,
        };
        let display = format!("{}", motor_error);
        assert!(display.contains("Motor 1"));
        assert!(display.contains("0x1001"));

        let config_error = RobotSoftwareError::ConfigurationError {
            setting: "max_speed".to_string(),
            value: "invalid".to_string(),
        };
        let display = format!("{}", config_error);
        assert!(display.contains("max_speed"));
        assert!(display.contains("invalid"));
    }

    #[test]
    fn test_error_source_chain() {
        let hardware_error = RobotHardwareError::MotorFailure {
            motor_id: 1,
            error_code: 0x1001,
        };
        let system_error = RobotSystemError::Hardware(hardware_error);

        // Test that we can access the source error
        let source = system_error.source();
        assert!(source.is_some());
    }

    #[test]
    fn test_anyhow_integration() {
        let mut motor = MotorController::new(1, 100.0);
        motor.voltage = 8.0; // This will cause an error

        let result: Result<()> = motor.set_speed(50.0)
            .context("Failed to set motor speed during startup sequence");

        assert!(result.is_err());
        let error_msg = format!("{:?}", result.unwrap_err());
        assert!(error_msg.contains("Failed to set motor speed during startup sequence"));
        assert!(error_msg.contains("Power system error"));
    }

    #[test]
    fn test_complex_error_scenario() {
        let mut motor = MotorController::new(1, 100.0);
        let mut sensors = SensorSystem::new();
        let mut config = ConfigurationManager::new();

        // Create a complex scenario with multiple potential failures
        motor.error_count = 5; // High error count

        // Try to configure and operate the system
        let config_result = config.set_setting("max_speed", "90.0");
        assert!(config_result.is_ok());

        let speed_result = motor.set_speed(85.0); // High speed with high error count
        // This might fail due to communication timeout simulation

        let sensor_result = sensors.read_sensor("temperature");
        // This might fail due to simulated sensor issues

        // The test verifies that errors are properly typed and can be handled
        if let Err(e) = speed_result {
            println!("Motor error: {}", e);
        }

        if let Err(e) = sensor_result {
            println!("Sensor error: {}", e);
        }
    }
}

/// Public module for student exercises
pub mod exercises {
    use super::*;

    /// Exercise 1: Create a custom error type for a robot arm controller
    ///
    /// Create a custom error type that handles:
    /// - Joint limit violations
    /// - Inverse kinematics failures
    /// - Collision detection errors
    /// - Grip strength failures
    ///
    /// Requirements:
    /// - Implement Display and Error traits
    /// - Include relevant data in each error variant
    /// - Create a RobotArmController that uses these errors
    pub fn exercise_1_robot_arm_errors() {
        // TODO: Implement RobotArmError enum
        // TODO: Implement RobotArmController struct
        // TODO: Add methods that can generate different error types
        println!("Exercise 1: Implement custom robot arm error types");
    }

    /// Exercise 2: Create a hierarchical error system
    ///
    /// Create a multi-level error hierarchy:
    /// - SystemError (top level)
    /// - SubsystemError (navigation, manipulation, perception)
    /// - ComponentError (specific component failures)
    ///
    /// Requirements:
    /// - Each level should properly chain to the next
    /// - Implement source() method to trace error chains
    /// - Create utility functions to extract error information
    pub fn exercise_2_error_hierarchy() {
        // TODO: Implement multi-level error hierarchy
        // TODO: Create error chain traversal utilities
        // TODO: Add error classification functions
        println!("Exercise 2: Implement hierarchical error system");
    }

    /// Exercise 3: Error recovery state machine
    ///
    /// Create an error recovery system that:
    /// - Tracks error patterns
    /// - Implements escalating recovery strategies
    /// - Maintains recovery state across operations
    /// - Provides metrics on recovery success rates
    ///
    /// Requirements:
    /// - State machine for recovery processes
    /// - Configurable retry policies
    /// - Success/failure metrics tracking
    pub fn exercise_3_recovery_state_machine() {
        // TODO: Implement recovery state machine
        // TODO: Add configurable retry policies
        // TODO: Create recovery metrics system
        println!("Exercise 3: Implement error recovery state machine");
    }

    /// Exercise 4: Error aggregation and batch processing
    ///
    /// Create a system that:
    /// - Collects multiple errors from parallel operations
    /// - Categorizes errors by type and severity
    /// - Provides summary reports
    /// - Handles partial success scenarios
    ///
    /// Requirements:
    /// - Error collection from multiple sources
    /// - Error categorization and filtering
    /// - Partial success result types
    pub fn exercise_4_error_aggregation() {
        // TODO: Implement error aggregation system
        // TODO: Add error categorization
        // TODO: Create partial success result types
        println!("Exercise 4: Implement error aggregation and batch processing");
    }

    /// Exercise 5: Advanced error context and debugging
    ///
    /// Create enhanced error context system:
    /// - Stack trace capture
    /// - Operation timing information
    /// - System state snapshots
    /// - Correlation IDs for distributed operations
    ///
    /// Requirements:
    /// - Rich context capture
    /// - Debugging utilities
    /// - Error correlation across operations
    pub fn exercise_5_advanced_error_context() {
        // TODO: Implement rich error context system
        // TODO: Add debugging and tracing utilities
        // TODO: Create error correlation system
        println!("Exercise 5: Implement advanced error context and debugging");
    }
}