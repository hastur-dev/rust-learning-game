//! Level 21, Task 4: Result and Option Combinators
//!
//! This module demonstrates advanced usage of Result and Option combinators
//! for functional-style error handling and optional value processing in robot systems.
//!
//! Learning objectives:
//! - Master map, and_then, or_else, and other combinators
//! - Understand functional composition with Results and Options
//! - Learn to chain operations without explicit pattern matching
//! - Use combinators for data transformation and error handling
//! - Implement complex control flow with combinator chains

use std::collections::HashMap;
use std::fmt;

/// Robot sensor measurement with uncertainty
#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    pub value: f64,
    pub unit: String,
    pub accuracy: f64,
    pub timestamp: u64,
}

impl Measurement {
    pub fn new(value: f64, unit: String, accuracy: f64, timestamp: u64) -> Self {
        Self { value, unit, accuracy, timestamp }
    }

    /// Check if measurement is recent enough
    pub fn is_recent(&self, current_time: u64, max_age: u64) -> bool {
        current_time.saturating_sub(self.timestamp) <= max_age
    }

    /// Check if measurement accuracy meets threshold
    pub fn is_accurate_enough(&self, min_accuracy: f64) -> bool {
        self.accuracy >= min_accuracy
    }

    /// Convert to different unit with optional conversion factor
    pub fn convert_unit(&self, new_unit: &str, conversion_factor: Option<f64>) -> Option<Measurement> {
        conversion_factor.map(|factor| Measurement {
            value: self.value * factor,
            unit: new_unit.to_string(),
            accuracy: self.accuracy,
            timestamp: self.timestamp,
        })
    }
}

/// Robot error types for demonstrating Result combinators
#[derive(Debug, Clone, PartialEq)]
pub enum RobotError {
    SensorError(String),
    CalibrationError(String),
    CommunicationError(String),
    OutOfRange { min: f64, max: f64, actual: f64 },
    InsufficientData(String),
    InvalidOperation(String),
}

impl fmt::Display for RobotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RobotError::SensorError(msg) => write!(f, "Sensor error: {}", msg),
            RobotError::CalibrationError(msg) => write!(f, "Calibration error: {}", msg),
            RobotError::CommunicationError(msg) => write!(f, "Communication error: {}", msg),
            RobotError::OutOfRange { min, max, actual } => {
                write!(f, "Value {} out of range [{}, {}]", actual, min, max)
            }
            RobotError::InsufficientData(msg) => write!(f, "Insufficient data: {}", msg),
            RobotError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for RobotError {}

/// Sensor data processor demonstrating Option and Result combinators
#[derive(Debug)]
pub struct SensorProcessor {
    sensors: HashMap<String, SensorInfo>,
    calibration_data: HashMap<String, CalibrationInfo>,
    conversion_factors: HashMap<(String, String), f64>, // (from_unit, to_unit) -> factor
}

#[derive(Debug, Clone)]
struct SensorInfo {
    last_reading: Option<Measurement>,
    is_active: bool,
    error_count: u32,
    calibration_timestamp: Option<u64>,
}

#[derive(Debug, Clone)]
struct CalibrationInfo {
    offset: f64,
    scale: f64,
    timestamp: u64,
    confidence: f64,
}

impl SensorProcessor {
    pub fn new() -> Self {
        let mut processor = Self {
            sensors: HashMap::new(),
            calibration_data: HashMap::new(),
            conversion_factors: HashMap::new(),
        };

        // Initialize some conversion factors
        processor.conversion_factors.insert(("celsius".to_string(), "fahrenheit".to_string()), 1.8);
        processor.conversion_factors.insert(("fahrenheit".to_string(), "celsius".to_string()), 5.0/9.0);
        processor.conversion_factors.insert(("meters".to_string(), "feet".to_string()), 3.28084);
        processor.conversion_factors.insert(("feet".to_string(), "meters".to_string()), 0.3048);
        processor.conversion_factors.insert(("kg".to_string(), "lbs".to_string()), 2.20462);
        processor.conversion_factors.insert(("lbs".to_string(), "kg".to_string()), 0.453592);

        processor
    }

    /// Register a new sensor
    pub fn register_sensor(&mut self, sensor_id: String) {
        self.sensors.insert(sensor_id, SensorInfo {
            last_reading: None,
            is_active: false,
            error_count: 0,
            calibration_timestamp: None,
        });
    }

    /// Add measurement using Option combinators
    pub fn add_measurement(&mut self, sensor_id: &str, value: f64, unit: String, accuracy: f64, timestamp: u64) -> Result<(), RobotError> {
        let measurement = Measurement::new(value, unit, accuracy, timestamp);

        self.sensors.get_mut(sensor_id)
            .ok_or_else(|| RobotError::SensorError(format!("Sensor '{}' not found", sensor_id)))
            .map(|sensor| {
                sensor.last_reading = Some(measurement);
                sensor.is_active = true;
            })
    }

    /// Get latest measurement with combinators for validation and transformation
    pub fn get_latest_measurement(&self, sensor_id: &str, current_time: u64) -> Result<Measurement, RobotError> {
        self.sensors.get(sensor_id)
            .ok_or_else(|| RobotError::SensorError(format!("Sensor '{}' not found", sensor_id)))
            .and_then(|sensor| {
                sensor.last_reading.as_ref()
                    .ok_or_else(|| RobotError::InsufficientData(format!("No readings for sensor '{}'", sensor_id)))
                    .cloned()
            })
            .and_then(|measurement| {
                if measurement.is_recent(current_time, 3600) { // 1 hour max age
                    Ok(measurement)
                } else {
                    Err(RobotError::InsufficientData("Measurement too old".to_string()))
                }
            })
    }

    /// Get calibrated measurement using combinator chains
    pub fn get_calibrated_measurement(&self, sensor_id: &str, current_time: u64) -> Result<Measurement, RobotError> {
        self.get_latest_measurement(sensor_id, current_time)
            .and_then(|measurement| {
                self.calibration_data.get(sensor_id)
                    .ok_or_else(|| RobotError::CalibrationError(format!("No calibration for sensor '{}'", sensor_id)))
                    .and_then(|cal_info| {
                        if current_time.saturating_sub(cal_info.timestamp) > 86400 { // 24 hours
                            Err(RobotError::CalibrationError("Calibration too old".to_string()))
                        } else {
                            Ok(cal_info)
                        }
                    })
                    .map(|cal_info| Measurement {
                        value: (measurement.value + cal_info.offset) * cal_info.scale,
                        unit: measurement.unit,
                        accuracy: measurement.accuracy * cal_info.confidence,
                        timestamp: measurement.timestamp,
                    })
            })
    }

    /// Convert measurement to different unit using Option combinators
    pub fn convert_measurement(&self, measurement: &Measurement, target_unit: &str) -> Option<Measurement> {
        if measurement.unit == target_unit {
            return Some(measurement.clone());
        }

        self.conversion_factors.get(&(measurement.unit.clone(), target_unit.to_string()))
            .and_then(|&factor| measurement.convert_unit(target_unit, Some(factor)))
    }

    /// Get measurement in preferred unit with fallback chain
    pub fn get_measurement_in_unit(&self, sensor_id: &str, target_unit: &str, current_time: u64) -> Result<Measurement, RobotError> {
        self.get_calibrated_measurement(sensor_id, current_time)
            .or_else(|_| self.get_latest_measurement(sensor_id, current_time))
            .and_then(|measurement| {
                self.convert_measurement(&measurement, target_unit)
                    .ok_or_else(|| RobotError::InvalidOperation(
                        format!("Cannot convert from '{}' to '{}'", measurement.unit, target_unit)
                    ))
            })
    }

    /// Add calibration data
    pub fn set_calibration(&mut self, sensor_id: &str, offset: f64, scale: f64, timestamp: u64, confidence: f64) -> Result<(), RobotError> {
        if !(0.0..=1.0).contains(&confidence) {
            return Err(RobotError::CalibrationError("Confidence must be between 0 and 1".to_string()));
        }

        self.sensors.get_mut(sensor_id)
            .ok_or_else(|| RobotError::SensorError(format!("Sensor '{}' not found", sensor_id)))
            .map(|sensor| {
                sensor.calibration_timestamp = Some(timestamp);
                self.calibration_data.insert(sensor_id.to_string(), CalibrationInfo {
                    offset, scale, timestamp, confidence
                });
            })
    }

    /// Get sensor status with Option combinators
    pub fn get_sensor_status(&self, sensor_id: &str) -> Option<SensorStatus> {
        self.sensors.get(sensor_id)
            .map(|sensor| SensorStatus {
                is_active: sensor.is_active,
                has_recent_data: sensor.last_reading.as_ref()
                    .map(|reading| reading.is_recent(get_current_time(), 3600))
                    .unwrap_or(false),
                is_calibrated: sensor.calibration_timestamp.is_some(),
                error_count: sensor.error_count,
                last_reading_time: sensor.last_reading.as_ref().map(|r| r.timestamp),
            })
    }

    /// Get aggregated sensor data using combinator chains
    pub fn get_aggregate_data(&self, sensor_ids: &[&str], operation: AggregateOperation, current_time: u64) -> Result<f64, RobotError> {
        let measurements: Result<Vec<_>, _> = sensor_ids.iter()
            .map(|&id| self.get_latest_measurement(id, current_time))
            .collect();

        measurements
            .and_then(|measurements| {
                if measurements.is_empty() {
                    Err(RobotError::InsufficientData("No valid measurements".to_string()))
                } else {
                    Ok(measurements)
                }
            })
            .map(|measurements| {
                let values: Vec<f64> = measurements.iter().map(|m| m.value).collect();
                match operation {
                    AggregateOperation::Average => values.iter().sum::<f64>() / values.len() as f64,
                    AggregateOperation::Max => values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
                    AggregateOperation::Min => values.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
                    AggregateOperation::Sum => values.iter().sum(),
                }
            })
    }

    /// Validate measurement range using Result combinators
    pub fn validate_measurement_range(&self, sensor_id: &str, min: f64, max: f64, current_time: u64) -> Result<bool, RobotError> {
        self.get_latest_measurement(sensor_id, current_time)
            .map(|measurement| measurement.value)
            .and_then(|value| {
                if value < min || value > max {
                    Err(RobotError::OutOfRange { min, max, actual: value })
                } else {
                    Ok(true)
                }
            })
    }

    /// Check sensor health using Option and Result combinators
    pub fn check_sensor_health(&self, sensor_id: &str, current_time: u64) -> Result<HealthStatus, RobotError> {
        let sensor = self.sensors.get(sensor_id)
            .ok_or_else(|| RobotError::SensorError(format!("Sensor '{}' not found", sensor_id)))?;

        let has_recent_data = sensor.last_reading.as_ref()
            .map(|reading| reading.is_recent(current_time, 1800)) // 30 minutes
            .unwrap_or(false);

        let is_well_calibrated = sensor.calibration_timestamp
            .and_then(|cal_time| {
                self.calibration_data.get(sensor_id).map(|cal_info| {
                    current_time.saturating_sub(cal_time) <= 604800 && // 1 week
                    cal_info.confidence >= 0.8
                })
            })
            .unwrap_or(false);

        let accuracy_ok = sensor.last_reading.as_ref()
            .map(|reading| reading.accuracy >= 0.7)
            .unwrap_or(false);

        Ok(HealthStatus {
            overall_healthy: sensor.is_active && has_recent_data && sensor.error_count < 5,
            has_recent_data,
            is_well_calibrated,
            accuracy_acceptable: accuracy_ok,
            error_count: sensor.error_count,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SensorStatus {
    pub is_active: bool,
    pub has_recent_data: bool,
    pub is_calibrated: bool,
    pub error_count: u32,
    pub last_reading_time: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub overall_healthy: bool,
    pub has_recent_data: bool,
    pub is_well_calibrated: bool,
    pub accuracy_acceptable: bool,
    pub error_count: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum AggregateOperation {
    Average,
    Max,
    Min,
    Sum,
}

/// Robot control system demonstrating complex combinator usage
#[derive(Debug)]
pub struct ControlSystem {
    processor: SensorProcessor,
    control_parameters: HashMap<String, f64>,
    safety_limits: HashMap<String, (f64, f64)>, // (min, max)
}

impl ControlSystem {
    pub fn new() -> Self {
        let mut system = Self {
            processor: SensorProcessor::new(),
            control_parameters: HashMap::new(),
            safety_limits: HashMap::new(),
        };

        // Initialize some default parameters and limits
        system.control_parameters.insert("target_temperature".to_string(), 23.0);
        system.control_parameters.insert("max_speed".to_string(), 10.0);
        system.control_parameters.insert("power_limit".to_string(), 1000.0);

        system.safety_limits.insert("temperature".to_string(), (0.0, 50.0));
        system.safety_limits.insert("speed".to_string(), (0.0, 15.0));
        system.safety_limits.insert("battery_voltage".to_string(), (10.0, 14.0));

        system
    }

    /// Register sensor in the system
    pub fn add_sensor(&mut self, sensor_id: String) {
        self.processor.register_sensor(sensor_id);
    }

    /// Add measurement with automatic safety checking
    pub fn add_measurement(&mut self, sensor_id: &str, value: f64, unit: String, accuracy: f64, timestamp: u64) -> Result<SafetyReport, RobotError> {
        self.processor.add_measurement(sensor_id, value, unit, accuracy, timestamp)
            .and_then(|_| self.check_safety_limits(sensor_id, timestamp))
    }

    /// Check safety limits using combinator chains
    fn check_safety_limits(&self, sensor_id: &str, current_time: u64) -> Result<SafetyReport, RobotError> {
        let measurement = self.processor.get_latest_measurement(sensor_id, current_time)?;

        let safety_check = self.safety_limits.get(sensor_id)
            .map(|&(min, max)| {
                if measurement.value >= min && measurement.value <= max {
                    SafetyStatus::Safe
                } else if measurement.value < min {
                    SafetyStatus::BelowMinimum { actual: measurement.value, minimum: min }
                } else {
                    SafetyStatus::AboveMaximum { actual: measurement.value, maximum: max }
                }
            })
            .unwrap_or(SafetyStatus::NoLimitsSet);

        Ok(SafetyReport {
            sensor_id: sensor_id.to_string(),
            measurement: measurement.value,
            status: safety_check,
            timestamp: current_time,
        })
    }

    /// Calculate control output using sensor fusion and combinators
    pub fn calculate_control_output(&self, target_sensor: &str, reference_sensors: &[&str], current_time: u64) -> Result<ControlOutput, RobotError> {
        let target_measurement = self.processor.get_calibrated_measurement(target_sensor, current_time)
            .or_else(|_| self.processor.get_latest_measurement(target_sensor, current_time))?;

        let reference_avg = self.processor.get_aggregate_data(reference_sensors, AggregateOperation::Average, current_time)
            .unwrap_or(target_measurement.value);

        let target_value = self.control_parameters.get("target_temperature")
            .copied()
            .unwrap_or(23.0);

        let error = target_value - target_measurement.value;
        let reference_error = reference_avg - target_measurement.value;

        // Simple PID-like calculation
        let output = error * 0.5 + reference_error * 0.2;

        // Apply safety limits to output
        let max_output = self.control_parameters.get("power_limit")
            .copied()
            .unwrap_or(1000.0);

        let safe_output = output.max(-max_output).min(max_output);

        Ok(ControlOutput {
            raw_output: output,
            limited_output: safe_output,
            error,
            reference_error,
            target_value,
            current_value: target_measurement.value,
            confidence: target_measurement.accuracy,
        })
    }

    /// Get system health report using Option and Result combinators
    pub fn get_system_health(&self, current_time: u64) -> SystemHealthReport {
        let sensor_healths: Vec<_> = self.processor.sensors.keys()
            .map(|sensor_id| {
                self.processor.check_sensor_health(sensor_id, current_time)
                    .map(|health| (sensor_id.clone(), health))
                    .ok()
            })
            .collect();

        let healthy_sensors = sensor_healths.iter()
            .filter_map(|opt| opt.as_ref())
            .filter(|(_, health)| health.overall_healthy)
            .count();

        let total_sensors = sensor_healths.len();

        let overall_healthy = total_sensors > 0 &&
            (healthy_sensors as f64 / total_sensors as f64) >= 0.8;

        SystemHealthReport {
            overall_healthy,
            total_sensors,
            healthy_sensors,
            sensor_details: sensor_healths.into_iter().filter_map(|x| x).collect(),
        }
    }

    /// Perform sensor fusion using combinators
    pub fn fuse_sensor_data(&self, primary_sensor: &str, backup_sensors: &[&str], target_unit: &str, current_time: u64) -> Result<FusedReading, RobotError> {
        // Try primary sensor first
        let primary_result = self.processor.get_measurement_in_unit(primary_sensor, target_unit, current_time);

        primary_result
            .or_else(|_| {
                // If primary fails, try backup sensors
                backup_sensors.iter()
                    .find_map(|&sensor_id| {
                        self.processor.get_measurement_in_unit(sensor_id, target_unit, current_time).ok()
                    })
                    .ok_or_else(|| RobotError::InsufficientData("All sensors failed".to_string()))
            })
            .map(|measurement| FusedReading {
                value: measurement.value,
                unit: measurement.unit,
                source: if primary_result.is_ok() {
                    DataSource::Primary(primary_sensor.to_string())
                } else {
                    DataSource::Backup
                },
                confidence: measurement.accuracy,
                timestamp: measurement.timestamp,
            })
    }

    /// Set control parameter with validation
    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<f64, RobotError> {
        match name {
            "target_temperature" => {
                if (0.0..=40.0).contains(&value) {
                    Ok(self.control_parameters.insert(name.to_string(), value).unwrap_or(0.0))
                } else {
                    Err(RobotError::OutOfRange { min: 0.0, max: 40.0, actual: value })
                }
            }
            "max_speed" => {
                if (0.0..=20.0).contains(&value) {
                    Ok(self.control_parameters.insert(name.to_string(), value).unwrap_or(0.0))
                } else {
                    Err(RobotError::OutOfRange { min: 0.0, max: 20.0, actual: value })
                }
            }
            "power_limit" => {
                if value >= 0.0 {
                    Ok(self.control_parameters.insert(name.to_string(), value).unwrap_or(0.0))
                } else {
                    Err(RobotError::OutOfRange { min: 0.0, max: f64::INFINITY, actual: value })
                }
            }
            _ => Err(RobotError::InvalidOperation(format!("Unknown parameter: {}", name)))
        }
    }
}

#[derive(Debug, Clone)]
pub struct SafetyReport {
    pub sensor_id: String,
    pub measurement: f64,
    pub status: SafetyStatus,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum SafetyStatus {
    Safe,
    BelowMinimum { actual: f64, minimum: f64 },
    AboveMaximum { actual: f64, maximum: f64 },
    NoLimitsSet,
}

#[derive(Debug, Clone)]
pub struct ControlOutput {
    pub raw_output: f64,
    pub limited_output: f64,
    pub error: f64,
    pub reference_error: f64,
    pub target_value: f64,
    pub current_value: f64,
    pub confidence: f64,
}

#[derive(Debug)]
pub struct SystemHealthReport {
    pub overall_healthy: bool,
    pub total_sensors: usize,
    pub healthy_sensors: usize,
    pub sensor_details: Vec<(String, HealthStatus)>,
}

#[derive(Debug, Clone)]
pub struct FusedReading {
    pub value: f64,
    pub unit: String,
    pub source: DataSource,
    pub confidence: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum DataSource {
    Primary(String),
    Backup,
}

/// Utility functions demonstrating combinator patterns
pub mod combinator_utils {
    use super::*;

    /// Chain multiple Option operations
    pub fn chain_optional_operations<T, U, V, F1, F2>(
        initial: Option<T>,
        op1: F1,
        op2: F2,
    ) -> Option<V>
    where
        F1: FnOnce(T) -> Option<U>,
        F2: FnOnce(U) -> Option<V>,
    {
        initial.and_then(op1).and_then(op2)
    }

    /// Apply function to Option if condition is met
    pub fn conditional_map<T, U, F, P>(
        option: Option<T>,
        condition: P,
        transform: F,
    ) -> Option<U>
    where
        F: FnOnce(T) -> U,
        P: FnOnce(&T) -> bool,
    {
        option.and_then(|value| {
            if condition(&value) {
                Some(transform(value))
            } else {
                None
            }
        })
    }

    /// Combine multiple Results into one
    pub fn combine_results<T, E>(results: Vec<Result<T, E>>) -> Result<Vec<T>, Vec<E>> {
        let mut successes = Vec::new();
        let mut errors = Vec::new();

        for result in results {
            match result {
                Ok(value) => successes.push(value),
                Err(error) => errors.push(error),
            }
        }

        if errors.is_empty() {
            Ok(successes)
        } else {
            Err(errors)
        }
    }

    /// Apply function with fallback on error
    pub fn with_fallback<T, E, F>(
        result: Result<T, E>,
        fallback: F,
    ) -> T
    where
        F: FnOnce(E) -> T,
    {
        result.unwrap_or_else(fallback)
    }

    /// Transform error type in Result
    pub fn map_error<T, E1, E2, F>(
        result: Result<T, E1>,
        error_mapper: F,
    ) -> Result<T, E2>
    where
        F: FnOnce(E1) -> E2,
    {
        result.map_err(error_mapper)
    }

    /// Try multiple operations until one succeeds
    pub fn try_sequence<T, E, F>(operations: Vec<F>) -> Result<T, Vec<E>>
    where
        F: FnOnce() -> Result<T, E>,
    {
        let mut errors = Vec::new();

        for operation in operations {
            match operation() {
                Ok(result) => return Ok(result),
                Err(error) => errors.push(error),
            }
        }

        Err(errors)
    }

    /// Validate value and wrap in Result
    pub fn validate<T, E, P>(value: T, predicate: P, error: E) -> Result<T, E>
    where
        P: FnOnce(&T) -> bool,
    {
        if predicate(&value) {
            Ok(value)
        } else {
            Err(error)
        }
    }
}

/// Mock time function for testing
fn get_current_time() -> u64 {
    1000000 // Mock timestamp
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::combinator_utils::*;

    #[test]
    fn test_measurement_creation_and_validation() {
        let measurement = Measurement::new(23.5, "celsius".to_string(), 0.95, 1000);

        assert!(measurement.is_recent(1500, 600));
        assert!(!measurement.is_recent(2000, 600));
        assert!(measurement.is_accurate_enough(0.9));
        assert!(!measurement.is_accurate_enough(0.99));
    }

    #[test]
    fn test_measurement_unit_conversion() {
        let temp = Measurement::new(23.0, "celsius".to_string(), 0.9, 1000);

        let converted = temp.convert_unit("fahrenheit", Some(1.8));
        assert!(converted.is_some());

        let fahrenheit = converted.unwrap();
        assert_eq!(fahrenheit.value, 41.4);
        assert_eq!(fahrenheit.unit, "fahrenheit");

        let no_conversion = temp.convert_unit("kelvin", None);
        assert!(no_conversion.is_none());
    }

    #[test]
    fn test_sensor_processor_basic_operations() {
        let mut processor = SensorProcessor::new();
        processor.register_sensor("temp1".to_string());

        let result = processor.add_measurement("temp1", 25.0, "celsius".to_string(), 0.9, 1000);
        assert!(result.is_ok());

        let measurement = processor.get_latest_measurement("temp1", 1500);
        assert!(measurement.is_ok());
        assert_eq!(measurement.unwrap().value, 25.0);
    }

    #[test]
    fn test_sensor_processor_error_handling() {
        let processor = SensorProcessor::new();

        // Test unknown sensor
        let result = processor.get_latest_measurement("unknown", 1000);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::SensorError(_)));

        // Test sensor with no data
        let mut processor = SensorProcessor::new();
        processor.register_sensor("empty".to_string());

        let result = processor.get_latest_measurement("empty", 1000);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::InsufficientData(_)));
    }

    #[test]
    fn test_calibration_with_combinators() {
        let mut processor = SensorProcessor::new();
        processor.register_sensor("temp1".to_string());

        processor.add_measurement("temp1", 25.0, "celsius".to_string(), 0.9, 1000).unwrap();
        processor.set_calibration("temp1", 2.0, 1.1, 1000, 0.95).unwrap();

        let calibrated = processor.get_calibrated_measurement("temp1", 1500);
        assert!(calibrated.is_ok());

        let measurement = calibrated.unwrap();
        assert_eq!(measurement.value, 29.7); // (25 + 2) * 1.1
        assert_eq!(measurement.accuracy, 0.855); // 0.9 * 0.95
    }

    #[test]
    fn test_unit_conversion_with_combinators() {
        let mut processor = SensorProcessor::new();
        processor.register_sensor("temp1".to_string());

        processor.add_measurement("temp1", 25.0, "celsius".to_string(), 0.9, 1000).unwrap();

        let converted = processor.get_measurement_in_unit("temp1", "fahrenheit", 1500);
        assert!(converted.is_ok());

        let measurement = converted.unwrap();
        assert_eq!(measurement.value, 45.0); // 25 * 1.8
        assert_eq!(measurement.unit, "fahrenheit");
    }

    #[test]
    fn test_sensor_status_with_combinators() {
        let mut processor = SensorProcessor::new();
        processor.register_sensor("temp1".to_string());

        let status = processor.get_sensor_status("temp1");
        assert!(status.is_some());

        let status = status.unwrap();
        assert!(!status.is_active);
        assert!(!status.has_recent_data);
        assert!(!status.is_calibrated);

        processor.add_measurement("temp1", 25.0, "celsius".to_string(), 0.9, 1000).unwrap();

        let status = processor.get_sensor_status("temp1").unwrap();
        assert!(status.is_active);
        assert!(status.has_recent_data);
    }

    #[test]
    fn test_aggregate_operations() {
        let mut processor = SensorProcessor::new();
        processor.register_sensor("temp1".to_string());
        processor.register_sensor("temp2".to_string());
        processor.register_sensor("temp3".to_string());

        processor.add_measurement("temp1", 20.0, "celsius".to_string(), 0.9, 1000).unwrap();
        processor.add_measurement("temp2", 25.0, "celsius".to_string(), 0.9, 1000).unwrap();
        processor.add_measurement("temp3", 30.0, "celsius".to_string(), 0.9, 1000).unwrap();

        let sensors = vec!["temp1", "temp2", "temp3"];

        let avg = processor.get_aggregate_data(&sensors, AggregateOperation::Average, 1500);
        assert!(avg.is_ok());
        assert_eq!(avg.unwrap(), 25.0);

        let max = processor.get_aggregate_data(&sensors, AggregateOperation::Max, 1500);
        assert_eq!(max.unwrap(), 30.0);

        let min = processor.get_aggregate_data(&sensors, AggregateOperation::Min, 1500);
        assert_eq!(min.unwrap(), 20.0);

        let sum = processor.get_aggregate_data(&sensors, AggregateOperation::Sum, 1500);
        assert_eq!(sum.unwrap(), 75.0);
    }

    #[test]
    fn test_control_system_safety_checking() {
        let mut control = ControlSystem::new();
        control.add_sensor("temp1".to_string());

        // Safe measurement
        let report = control.add_measurement("temp1", 25.0, "temperature".to_string(), 0.9, 1000);
        assert!(report.is_ok());

        let safety_report = report.unwrap();
        assert!(matches!(safety_report.status, SafetyStatus::Safe));

        // Unsafe measurement
        let report = control.add_measurement("temp1", 60.0, "temperature".to_string(), 0.9, 1500);
        assert!(report.is_ok());

        let safety_report = report.unwrap();
        assert!(matches!(safety_report.status, SafetyStatus::AboveMaximum { .. }));
    }

    #[test]
    fn test_control_output_calculation() {
        let mut control = ControlSystem::new();
        control.add_sensor("temp1".to_string());
        control.add_sensor("temp2".to_string());

        control.add_measurement("temp1", 20.0, "celsius".to_string(), 0.9, 1000).unwrap();
        control.add_measurement("temp2", 22.0, "celsius".to_string(), 0.9, 1000).unwrap();

        let output = control.calculate_control_output("temp1", &["temp2"], 1500);
        assert!(output.is_ok());

        let control_output = output.unwrap();
        assert!(control_output.error > 0.0); // Should be positive since target is 23.0
        assert_eq!(control_output.current_value, 20.0);
        assert_eq!(control_output.target_value, 23.0);
    }

    #[test]
    fn test_sensor_fusion() {
        let mut control = ControlSystem::new();
        control.add_sensor("primary".to_string());
        control.add_sensor("backup1".to_string());
        control.add_sensor("backup2".to_string());

        // Only backup sensors have data
        control.add_measurement("backup1", 25.0, "celsius".to_string(), 0.9, 1000).unwrap();
        control.add_measurement("backup2", 26.0, "celsius".to_string(), 0.8, 1000).unwrap();

        let fused = control.fuse_sensor_data("primary", &["backup1", "backup2"], "celsius", 1500);
        assert!(fused.is_ok());

        let reading = fused.unwrap();
        assert_eq!(reading.value, 25.0); // Should use first backup
        assert!(matches!(reading.source, DataSource::Backup));
    }

    #[test]
    fn test_system_health_report() {
        let mut control = ControlSystem::new();
        control.add_sensor("temp1".to_string());
        control.add_sensor("temp2".to_string());

        control.add_measurement("temp1", 25.0, "celsius".to_string(), 0.9, 1000).unwrap();
        // temp2 has no data

        let health = control.get_system_health(1500);
        assert_eq!(health.total_sensors, 2);
        assert_eq!(health.healthy_sensors, 1);
        assert!(!health.overall_healthy); // Less than 80% healthy
    }

    #[test]
    fn test_combinator_utils() {
        // Test chain_optional_operations
        let result = chain_optional_operations(
            Some(5),
            |x| if x > 0 { Some(x * 2) } else { None },
            |x| if x < 20 { Some(x + 1) } else { None },
        );
        assert_eq!(result, Some(11));

        // Test conditional_map
        let result = conditional_map(
            Some(10),
            |&x| x > 5,
            |x| x * 2,
        );
        assert_eq!(result, Some(20));

        let result = conditional_map(
            Some(3),
            |&x| x > 5,
            |x| x * 2,
        );
        assert_eq!(result, None);

        // Test combine_results
        let results = vec![Ok(1), Ok(2), Err("error1"), Ok(3), Err("error2")];
        let combined = combine_results(results);
        assert!(combined.is_err());

        let errors = combined.unwrap_err();
        assert_eq!(errors, vec!["error1", "error2"]);

        // Test with_fallback
        let result: i32 = with_fallback(Err("error"), |_| 42);
        assert_eq!(result, 42);

        // Test validate
        let result = validate(10, |&x| x > 5, "too small");
        assert!(result.is_ok());

        let result = validate(3, |&x| x > 5, "too small");
        assert!(result.is_err());
    }

    #[test]
    fn test_parameter_validation() {
        let mut control = ControlSystem::new();

        // Valid parameter
        let result = control.set_parameter("target_temperature", 25.0);
        assert!(result.is_ok());

        // Invalid parameter value
        let result = control.set_parameter("target_temperature", 50.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::OutOfRange { .. }));

        // Unknown parameter
        let result = control.set_parameter("unknown_param", 10.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RobotError::InvalidOperation(_)));
    }

    #[test]
    fn test_complex_combinator_chains() {
        let mut processor = SensorProcessor::new();
        processor.register_sensor("multi_sensor".to_string());

        processor.add_measurement("multi_sensor", 25.0, "celsius".to_string(), 0.9, 1000).unwrap();
        processor.set_calibration("multi_sensor", 2.0, 1.1, 1000, 0.95).unwrap();

        // Complex chain: get calibrated -> convert units -> validate range
        let result = processor.get_calibrated_measurement("multi_sensor", 1500)
            .and_then(|measurement| {
                processor.convert_measurement(&measurement, "fahrenheit")
                    .ok_or_else(|| RobotError::InvalidOperation("Conversion failed".to_string()))
            })
            .and_then(|measurement| {
                if measurement.value > 32.0 && measurement.value < 100.0 {
                    Ok(measurement)
                } else {
                    Err(RobotError::OutOfRange { min: 32.0, max: 100.0, actual: measurement.value })
                }
            });

        assert!(result.is_ok());
        let final_measurement = result.unwrap();
        assert_eq!(final_measurement.unit, "fahrenheit");
        assert!(final_measurement.value > 32.0);
    }
}

/// Public module for student exercises
pub mod exercises {
    use super::*;

    /// Exercise 1: Implement a sensor data validator
    ///
    /// Create a comprehensive sensor validation system using combinators:
    /// - Chain multiple validation rules (range, accuracy, freshness)
    /// - Use map and and_then for transformation and validation
    /// - Implement conditional validation based on sensor type
    /// - Create validation reports with detailed error information
    ///
    /// Requirements:
    /// - Extensive use of Result and Option combinators
    /// - No explicit pattern matching - use combinators only
    /// - Implement validation rule composition
    pub fn exercise_1_sensor_validator() {
        // TODO: Implement SensorValidator struct using only combinators
        // TODO: Add validation rule chaining with and_then
        // TODO: Create validation report generation with map operations
        println!("Exercise 1: Implement sensor validator using combinators");
    }

    /// Exercise 2: Robot behavior decision engine
    ///
    /// Create a decision engine that uses combinators to determine robot actions:
    /// - Process multiple sensor inputs with Option combinators
    /// - Chain decision rules using Result combinators
    /// - Implement fallback behaviors using or_else
    /// - Calculate confidence scores using map operations
    ///
    /// Requirements:
    /// - Complex combinator chains for decision making
    /// - Fallback strategy implementation with or_else
    /// - Use filter and map for data processing
    pub fn exercise_2_decision_engine() {
        // TODO: Implement DecisionEngine using combinators
        // TODO: Add decision rule chaining with complex logic
        // TODO: Create fallback behavior system using or_else
        println!("Exercise 2: Implement decision engine with combinators");
    }

    /// Exercise 3: Multi-sensor data fusion
    ///
    /// Create an advanced sensor fusion system using combinators:
    /// - Combine data from multiple sensors with different reliability
    /// - Implement weighted averaging using map and fold operations
    /// - Handle missing sensors gracefully with Option combinators
    /// - Create confidence metrics for fused data
    ///
    /// Requirements:
    /// - Advanced use of iterator combinators
    /// - Option and Result composition for missing data handling
    /// - Weighted fusion algorithms using combinators
    pub fn exercise_3_advanced_sensor_fusion() {
        // TODO: Implement AdvancedSensorFusion using combinators
        // TODO: Add weighted averaging with iterator combinators
        // TODO: Create confidence calculation system
        println!("Exercise 3: Implement advanced sensor fusion with combinators");
    }

    /// Exercise 4: Robot task scheduler with priorities
    ///
    /// Create a task scheduling system using functional combinators:
    /// - Parse and validate task definitions using Result combinators
    /// - Implement priority-based scheduling with Option chaining
    /// - Handle resource conflicts using and_then chains
    /// - Create execution reports using map transformations
    ///
    /// Requirements:
    /// - Functional composition for task processing
    /// - Priority queue operations using combinators
    /// - Resource allocation with combinator-based conflict resolution
    pub fn exercise_4_task_scheduler() {
        // TODO: Implement TaskScheduler using functional combinators
        // TODO: Add priority-based scheduling with Option chains
        // TODO: Create resource conflict resolution system
        println!("Exercise 4: Implement task scheduler with combinators");
    }

    /// Exercise 5: Predictive maintenance system
    ///
    /// Create a predictive maintenance system using advanced combinators:
    /// - Process historical sensor data with iterator combinators
    /// - Predict failures using Result chains for error propagation
    /// - Calculate maintenance schedules using Option compositions
    /// - Generate maintenance reports using map and filter operations
    ///
    /// Requirements:
    /// - Complex data processing pipelines using combinators
    /// - Predictive algorithms implemented with functional composition
    /// - Report generation using advanced combinator patterns
    pub fn exercise_5_predictive_maintenance() {
        // TODO: Implement PredictiveMaintenance using advanced combinators
        // TODO: Add failure prediction with Result chains
        // TODO: Create maintenance scheduling with Option compositions
        println!("Exercise 5: Implement predictive maintenance with combinators");
    }
}