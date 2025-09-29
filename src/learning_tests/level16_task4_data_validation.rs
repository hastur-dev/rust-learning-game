// Learning Tests for Level 16, Task 4: Data Validation and Schema Compliance
// Implementing comprehensive data validation during deserialization

use serde::{Serialize, Deserialize, de::{self, Deserializer, Visitor}, Serializer};
use serde_json;
use std::fmt;
use std::error::Error;

// Data Processing Error types
#[derive(Debug)]
pub enum DataProcessingError {
    JsonError(serde_json::Error),
    ValidationError(String),
    CorruptedData(String),
    UnsupportedFormat(String),
    MigrationError(String),
}

impl fmt::Display for DataProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataProcessingError::JsonError(e) => write!(f, "JSON processing error: {}", e),
            DataProcessingError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            DataProcessingError::CorruptedData(msg) => write!(f, "Data corruption detected: {}", msg),
            DataProcessingError::UnsupportedFormat(format) => write!(f, "Unsupported format: {}", format),
            DataProcessingError::MigrationError(msg) => write!(f, "Migration error: {}", msg),
        }
    }
}

impl Error for DataProcessingError {}

impl From<serde_json::Error> for DataProcessingError {
    fn from(error: serde_json::Error) -> Self {
        DataProcessingError::JsonError(error)
    }
}

// Validated Robot Configuration with comprehensive validation
#[derive(Debug, Clone, PartialEq)]
pub struct ValidatedRobotConfig {
    pub id: u32,
    pub name: String,
    pub coordinates: (f64, f64),
    pub sensors: Vec<String>,
}

impl<'de> Deserialize<'de> for ValidatedRobotConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawConfig {
            id: u32,
            name: String,
            coordinates: (f64, f64),
            sensors: Vec<String>,
        }

        let raw = RawConfig::deserialize(deserializer)?;

        // Comprehensive validation logic
        if raw.id == 0 {
            return Err(de::Error::custom("Robot ID cannot be zero"));
        }

        if raw.name.trim().is_empty() {
            return Err(de::Error::custom("Robot name cannot be empty"));
        }

        if raw.name.len() > 50 {
            return Err(de::Error::custom("Robot name too long (max 50 characters)"));
        }

        if raw.coordinates.0.abs() > 1000.0 || raw.coordinates.1.abs() > 1000.0 {
            return Err(de::Error::custom("Coordinates out of valid range (-1000 to 1000)"));
        }

        if raw.sensors.is_empty() {
            return Err(de::Error::custom("Robot must have at least one sensor"));
        }

        let valid_sensors = ["camera", "lidar", "ultrasonic", "gyroscope", "accelerometer"];
        for sensor in &raw.sensors {
            if !valid_sensors.contains(&sensor.as_str()) {
                return Err(de::Error::custom(format!("Unknown sensor type: {}", sensor)));
            }
        }

        Ok(ValidatedRobotConfig {
            id: raw.id,
            name: raw.name,
            coordinates: raw.coordinates,
            sensors: raw.sensors,
        })
    }
}

impl Serialize for ValidatedRobotConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ValidatedRobotConfig", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("coordinates", &self.coordinates)?;
        state.serialize_field("sensors", &self.sensors)?;
        state.end()
    }
}

// Extended validation with ranges and constraints
#[derive(Debug, Clone)]
pub struct AdvancedRobotConfig {
    pub id: u32,
    pub name: String,
    pub coordinates: (f64, f64),
    pub sensors: Vec<String>,
    pub battery_level: f32,
    pub firmware_version: String,
}

impl<'de> Deserialize<'de> for AdvancedRobotConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawAdvancedConfig {
            id: u32,
            name: String,
            coordinates: (f64, f64),
            sensors: Vec<String>,
            battery_level: f32,
            firmware_version: String,
        }

        let raw = RawAdvancedConfig::deserialize(deserializer)?;

        // ID validation
        if raw.id == 0 || raw.id > 999999 {
            return Err(de::Error::custom("Robot ID must be between 1 and 999999"));
        }

        // Name validation
        if raw.name.trim().is_empty() {
            return Err(de::Error::custom("Robot name cannot be empty"));
        }
        if raw.name.len() > 50 {
            return Err(de::Error::custom("Robot name too long (max 50 characters)"));
        }
        if !raw.name.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || c == '-' || c == '_') {
            return Err(de::Error::custom("Robot name contains invalid characters"));
        }

        // Coordinate validation
        if raw.coordinates.0.abs() > 1000.0 || raw.coordinates.1.abs() > 1000.0 {
            return Err(de::Error::custom("Coordinates out of valid range (-1000 to 1000)"));
        }
        if raw.coordinates.0.is_nan() || raw.coordinates.1.is_nan() {
            return Err(de::Error::custom("Coordinates cannot be NaN"));
        }

        // Sensor validation
        if raw.sensors.is_empty() {
            return Err(de::Error::custom("Robot must have at least one sensor"));
        }
        if raw.sensors.len() > 10 {
            return Err(de::Error::custom("Robot cannot have more than 10 sensors"));
        }

        let valid_sensors = ["camera", "lidar", "ultrasonic", "gyroscope", "accelerometer", "temperature", "pressure"];
        for sensor in &raw.sensors {
            if !valid_sensors.contains(&sensor.as_str()) {
                return Err(de::Error::custom(format!("Unknown sensor type: {}", sensor)));
            }
        }

        // Battery level validation
        if raw.battery_level < 0.0 || raw.battery_level > 100.0 {
            return Err(de::Error::custom("Battery level must be between 0.0 and 100.0"));
        }

        // Firmware version validation
        if !raw.firmware_version.matches('.').count() == 2 {
            return Err(de::Error::custom("Firmware version must be in format x.y.z"));
        }

        Ok(AdvancedRobotConfig {
            id: raw.id,
            name: raw.name,
            coordinates: raw.coordinates,
            sensors: raw.sensors,
            battery_level: raw.battery_level,
            firmware_version: raw.firmware_version,
        })
    }
}

// Validation processor function
pub fn process_validation_test_data() -> Result<ValidatedRobotConfig, DataProcessingError> {
    let test_cases = vec![
        r#"{"id": 0, "name": "Test", "coordinates": [0.0, 0.0], "sensors": ["camera"]}"#,
        r#"{"id": 123, "name": "", "coordinates": [0.0, 0.0], "sensors": ["camera"]}"#,
        r#"{"id": 123, "name": "Valid Robot", "coordinates": [2000.0, 0.0], "sensors": ["camera"]}"#,
        r#"{"id": 123, "name": "Valid Robot", "coordinates": [0.0, 0.0], "sensors": []}"#,
        r#"{"id": 123, "name": "Valid Robot", "coordinates": [0.0, 0.0], "sensors": ["invalid_sensor"]}"#,
        r#"{"id": 123, "name": "Valid Robot", "coordinates": [10.0, 20.0], "sensors": ["camera", "lidar"]}"#,
    ];

    for (i, test_case) in test_cases.iter().enumerate() {
        println!("Testing case {}: ", i + 1);
        match serde_json::from_str::<ValidatedRobotConfig>(test_case) {
            Ok(config) => println!("✓ Valid: {:?}", config),
            Err(e) => println!("✗ Invalid: {}", e),
        }
    }

    // Return the last valid case
    serde_json::from_str(test_cases.last().unwrap())
        .map_err(DataProcessingError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_robot_config_basic() {
        let json = r#"{"id": 123, "name": "Test Robot", "coordinates": [10.0, 20.0], "sensors": ["camera", "lidar"]}"#;
        let config: ValidatedRobotConfig = serde_json::from_str(json).unwrap();

        assert_eq!(config.id, 123);
        assert_eq!(config.name, "Test Robot");
        assert_eq!(config.coordinates, (10.0, 20.0));
        assert_eq!(config.sensors, vec!["camera", "lidar"]);
    }

    #[test]
    fn test_zero_id_validation() {
        let json = r#"{"id": 0, "name": "Test", "coordinates": [0.0, 0.0], "sensors": ["camera"]}"#;
        let result = serde_json::from_str::<ValidatedRobotConfig>(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Robot ID cannot be zero"));
    }

    #[test]
    fn test_empty_name_validation() {
        let json = r#"{"id": 123, "name": "", "coordinates": [0.0, 0.0], "sensors": ["camera"]}"#;
        let result = serde_json::from_str::<ValidatedRobotConfig>(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Robot name cannot be empty"));
    }

    #[test]
    fn test_long_name_validation() {
        let long_name = "A".repeat(51);
        let json = format!(r#"{{"id": 123, "name": "{}", "coordinates": [0.0, 0.0], "sensors": ["camera"]}}"#, long_name);
        let result = serde_json::from_str::<ValidatedRobotConfig>(&json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Robot name too long"));
    }

    #[test]
    fn test_coordinates_out_of_range() {
        let json = r#"{"id": 123, "name": "Test", "coordinates": [2000.0, 0.0], "sensors": ["camera"]}"#;
        let result = serde_json::from_str::<ValidatedRobotConfig>(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Coordinates out of valid range"));
    }

    #[test]
    fn test_empty_sensors_validation() {
        let json = r#"{"id": 123, "name": "Test", "coordinates": [0.0, 0.0], "sensors": []}"#;
        let result = serde_json::from_str::<ValidatedRobotConfig>(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Robot must have at least one sensor"));
    }

    #[test]
    fn test_invalid_sensor_type() {
        let json = r#"{"id": 123, "name": "Test", "coordinates": [0.0, 0.0], "sensors": ["invalid_sensor"]}"#;
        let result = serde_json::from_str::<ValidatedRobotConfig>(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown sensor type"));
    }

    #[test]
    fn test_valid_all_sensor_types() {
        let json = r#"{"id": 123, "name": "Test", "coordinates": [0.0, 0.0], "sensors": ["camera", "lidar", "ultrasonic", "gyroscope", "accelerometer"]}"#;
        let config: ValidatedRobotConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.sensors.len(), 5);
    }

    #[test]
    fn test_serialize_validated_config() {
        let config = ValidatedRobotConfig {
            id: 123,
            name: "Test Robot".to_string(),
            coordinates: (10.0, 20.0),
            sensors: vec!["camera".to_string(), "lidar".to_string()],
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: ValidatedRobotConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_advanced_robot_config_battery_validation() {
        let json = r#"{"id": 123, "name": "Test", "coordinates": [0.0, 0.0], "sensors": ["camera"], "battery_level": 150.0, "firmware_version": "1.0.0"}"#;
        let result = serde_json::from_str::<AdvancedRobotConfig>(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Battery level must be between 0.0 and 100.0"));
    }

    #[test]
    fn test_advanced_robot_config_firmware_validation() {
        let json = r#"{"id": 123, "name": "Test", "coordinates": [0.0, 0.0], "sensors": ["camera"], "battery_level": 75.0, "firmware_version": "invalid"}"#;
        let result = serde_json::from_str::<AdvancedRobotConfig>(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Firmware version must be in format x.y.z"));
    }

    #[test]
    fn test_process_validation_test_data_function() {
        // This test runs the validation processor function
        let result = process_validation_test_data();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.id, 123);
        assert_eq!(config.name, "Valid Robot");
        assert_eq!(config.coordinates, (10.0, 20.0));
        assert_eq!(config.sensors, vec!["camera", "lidar"]);
    }

    #[test]
    fn test_coordinate_boundary_conditions() {
        // Test exact boundary values
        let json = r#"{"id": 123, "name": "Test", "coordinates": [1000.0, -1000.0], "sensors": ["camera"]}"#;
        let config: ValidatedRobotConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.coordinates, (1000.0, -1000.0));

        // Test just over boundary
        let json = r#"{"id": 123, "name": "Test", "coordinates": [1000.1, 0.0], "sensors": ["camera"]}"#;
        let result = serde_json::from_str::<ValidatedRobotConfig>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_name_whitespace_handling() {
        let json = r#"{"id": 123, "name": "   ", "coordinates": [0.0, 0.0], "sensors": ["camera"]}"#;
        let result = serde_json::from_str::<ValidatedRobotConfig>(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Robot name cannot be empty"));
    }

    #[test]
    fn test_sensor_duplicates_allowed() {
        let json = r#"{"id": 123, "name": "Test", "coordinates": [0.0, 0.0], "sensors": ["camera", "camera", "lidar"]}"#;
        let config: ValidatedRobotConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.sensors.len(), 3);
        assert_eq!(config.sensors[0], "camera");
        assert_eq!(config.sensors[1], "camera");
        assert_eq!(config.sensors[2], "lidar");
    }

    #[test]
    fn test_mixed_valid_invalid_sensors() {
        let json = r#"{"id": 123, "name": "Test", "coordinates": [0.0, 0.0], "sensors": ["camera", "invalid", "lidar"]}"#;
        let result = serde_json::from_str::<ValidatedRobotConfig>(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown sensor type: invalid"));
    }
}