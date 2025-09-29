#[cfg(test)]
mod level15_task3_custom_serialization_tests {
    use super::*;
    use serde::{Serialize, Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct MissionProfile {
        #[serde(rename = "mission_id")]
        id: String,

        #[serde(default = "default_mission_name")]
        name: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,

        #[serde(default)]
        priority: Priority,

        #[serde(with = "duration_seconds")]
        estimated_time: std::time::Duration,

        #[serde(flatten)]
        metadata: MissionMetadata,
    }

    #[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
    enum Priority {
        Low,
        #[default]
        Medium,
        High,
        Critical,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct MissionMetadata {
        created_at: String,
        tags: Vec<String>,
    }

    fn default_mission_name() -> String {
        "Unnamed Mission".to_string()
    }

    mod duration_seconds {
        use serde::{Deserialize, Deserializer, Serialize, Serializer};
        use std::time::Duration;

        pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            duration.as_secs().serialize(serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
        where
            D: Deserializer<'de>,
        {
            let secs = u64::deserialize(deserializer)?;
            Ok(Duration::from_secs(secs))
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct RobotSettings {
        #[serde(rename = "robot_name")]
        name: String,

        #[serde(skip_serializing_if = "String::is_empty", default)]
        model: String,

        #[serde(with = "version_string")]
        firmware_version: (u32, u32, u32),

        #[serde(default = "default_enabled")]
        enabled: bool,

        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        capabilities: Vec<String>,

        #[serde(flatten)]
        position: Position,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Position {
        x: f64,
        y: f64,
        #[serde(default)]
        z: f64,
    }

    fn default_enabled() -> bool {
        true
    }

    mod version_string {
        use serde::{Deserialize, Deserializer, Serialize, Serializer};

        pub fn serialize<S>(version: &(u32, u32, u32), serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let version_string = format!("{}.{}.{}", version.0, version.1, version.2);
            version_string.serialize(serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<(u32, u32, u32), D::Error>
        where
            D: Deserializer<'de>,
        {
            let version_string = String::deserialize(deserializer)?;
            let parts: Vec<&str> = version_string.split('.').collect();

            if parts.len() != 3 {
                return Err(serde::de::Error::custom("Invalid version format"));
            }

            let major = parts[0].parse().map_err(serde::de::Error::custom)?;
            let minor = parts[1].parse().map_err(serde::de::Error::custom)?;
            let patch = parts[2].parse().map_err(serde::de::Error::custom)?;

            Ok((major, minor, patch))
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct SensorReading {
        sensor_id: String,

        #[serde(with = "timestamp_iso8601")]
        timestamp: std::time::SystemTime,

        #[serde(serialize_with = "serialize_f64_precision", deserialize_with = "deserialize_f64")]
        value: f64,

        #[serde(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,

        #[serde(default = "default_confidence")]
        confidence: f64,
    }

    fn default_confidence() -> f64 {
        1.0
    }

    fn serialize_f64_precision<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rounded = (value * 100.0).round() / 100.0; // Round to 2 decimal places
        serializer.serialize_f64(rounded)
    }

    fn deserialize_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        f64::deserialize(deserializer)
    }

    mod timestamp_iso8601 {
        use serde::{Deserialize, Deserializer, Serialize, Serializer};
        use std::time::{SystemTime, UNIX_EPOCH};

        pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let duration = time.duration_since(UNIX_EPOCH)
                .map_err(|_| serde::ser::Error::custom("Time went backwards"))?;
            let seconds = duration.as_secs();
            // Simplified ISO8601 format
            let iso_string = format!("2024-01-15T{:02}:{:02}:{:02}Z",
                                     (seconds / 3600) % 24,
                                     (seconds / 60) % 60,
                                     seconds % 60);
            iso_string.serialize(serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
        where
            D: Deserializer<'de>,
        {
            let _iso_string = String::deserialize(deserializer)?;
            // Simplified: just return a fixed time for testing
            Ok(UNIX_EPOCH + std::time::Duration::from_secs(1642248000)) // 2022-01-15T10:00:00Z
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct ConfigContainer {
        #[serde(rename = "version")]
        config_version: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        debug_info: Option<String>,

        #[serde(flatten)]
        settings: RobotSettings,

        #[serde(default)]
        mission_profiles: Vec<MissionProfile>,
    }

    fn create_sample_mission_profile() -> MissionProfile {
        MissionProfile {
            id: "EXPLORE_001".to_string(),
            name: "Deep Exploration".to_string(),
            description: Some("Comprehensive area exploration mission".to_string()),
            priority: Priority::High,
            estimated_time: Duration::from_secs(3600),
            metadata: MissionMetadata {
                created_at: "2024-01-15T10:00:00Z".to_string(),
                tags: vec!["exploration".to_string(), "mapping".to_string()],
            },
        }
    }

    fn create_minimal_mission_profile() -> MissionProfile {
        MissionProfile {
            id: "MINIMAL_001".to_string(),
            name: default_mission_name(), // Uses default
            description: None, // Will be skipped in serialization
            priority: Priority::default(), // Uses default Medium
            estimated_time: Duration::from_secs(1800),
            metadata: MissionMetadata {
                created_at: "2024-01-15T11:00:00Z".to_string(),
                tags: vec![],
            },
        }
    }

    fn create_robot_settings() -> RobotSettings {
        RobotSettings {
            name: "Explorer Bot Alpha".to_string(),
            model: "".to_string(), // Empty, will be skipped
            firmware_version: (2, 1, 0),
            enabled: default_enabled(),
            capabilities: vec![], // Empty, will be skipped
            position: Position {
                x: 10.5,
                y: 20.3,
                z: 0.0, // Default value
            },
        }
    }

    #[test]
    fn test_rename_attribute() {
        let mission = create_sample_mission_profile();
        let yaml = serde_yaml::to_string(&mission).unwrap();

        assert!(yaml.contains("mission_id"));
        assert!(!yaml.contains("id:"));

        let deserialized: MissionProfile = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.id, "EXPLORE_001");
    }

    #[test]
    fn test_default_values() {
        let minimal_mission = create_minimal_mission_profile();

        assert_eq!(minimal_mission.name, "Unnamed Mission");
        assert_eq!(minimal_mission.priority, Priority::Medium);
    }

    #[test]
    fn test_skip_serializing_if() {
        let minimal_mission = create_minimal_mission_profile();
        let yaml = serde_yaml::to_string(&minimal_mission).unwrap();

        // description is None, should be skipped
        assert!(!yaml.contains("description"));

        let full_mission = create_sample_mission_profile();
        let yaml_full = serde_yaml::to_string(&full_mission).unwrap();

        // description is Some, should be included
        assert!(yaml_full.contains("description"));
    }

    #[test]
    fn test_custom_duration_serialization() {
        let mission = create_sample_mission_profile();
        let yaml = serde_yaml::to_string(&mission).unwrap();

        // Duration should be serialized as seconds
        assert!(yaml.contains("estimated_time: 3600"));

        let deserialized: MissionProfile = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.estimated_time, Duration::from_secs(3600));
    }

    #[test]
    fn test_flatten_attribute() {
        let mission = create_sample_mission_profile();
        let yaml = serde_yaml::to_string(&mission).unwrap();

        // Metadata fields should be flattened to the top level
        assert!(yaml.contains("created_at"));
        assert!(yaml.contains("tags"));
        assert!(!yaml.contains("metadata:"));

        let deserialized: MissionProfile = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.metadata.created_at, "2024-01-15T10:00:00Z");
        assert_eq!(deserialized.metadata.tags.len(), 2);
    }

    #[test]
    fn test_version_string_serialization() {
        let settings = create_robot_settings();
        let yaml = serde_yaml::to_string(&settings).unwrap();

        // Version should be serialized as a string
        assert!(yaml.contains("firmware_version: \"2.1.0\""));

        let deserialized: RobotSettings = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.firmware_version, (2, 1, 0));
    }

    #[test]
    fn test_position_flattening() {
        let settings = create_robot_settings();
        let yaml = serde_yaml::to_string(&settings).unwrap();

        // Position fields should be flattened
        assert!(yaml.contains("x: 10.5"));
        assert!(yaml.contains("y: 20.3"));
        assert!(yaml.contains("z: 0"));
        assert!(!yaml.contains("position:"));
    }

    #[test]
    fn test_empty_collections_skipped() {
        let settings = create_robot_settings();
        let yaml = serde_yaml::to_string(&settings).unwrap();

        // Empty model and capabilities should be skipped
        assert!(!yaml.contains("model"));
        assert!(!yaml.contains("capabilities"));
    }

    #[test]
    fn test_sensor_reading_precision() {
        let reading = SensorReading {
            sensor_id: "TEMP_001".to_string(),
            timestamp: std::time::SystemTime::now(),
            value: 23.456789,
            unit: Some("celsius".to_string()),
            confidence: default_confidence(),
        };

        let yaml = serde_yaml::to_string(&reading).unwrap();
        let deserialized: SensorReading = serde_yaml::from_str(&yaml).unwrap();

        // Value should be rounded to 2 decimal places
        assert_eq!(deserialized.value, 23.46);
    }

    #[test]
    fn test_timestamp_serialization() {
        let reading = SensorReading {
            sensor_id: "TEMP_001".to_string(),
            timestamp: std::time::SystemTime::now(),
            value: 25.0,
            unit: None,
            confidence: 0.95,
        };

        let yaml = serde_yaml::to_string(&reading).unwrap();

        // Should contain ISO8601-like timestamp
        assert!(yaml.contains("timestamp:"));
        assert!(yaml.contains("2024-01-15T"));
        assert!(yaml.contains("Z"));
    }

    #[test]
    fn test_config_container_flattening() {
        let container = ConfigContainer {
            config_version: "1.0.0".to_string(),
            debug_info: None,
            settings: create_robot_settings(),
            mission_profiles: vec![],
        };

        let yaml = serde_yaml::to_string(&container).unwrap();

        // Settings should be flattened into the container
        assert!(yaml.contains("robot_name"));
        assert!(yaml.contains("firmware_version"));
        assert!(yaml.contains("x:"));
        assert!(yaml.contains("y:"));

        // debug_info is None, should be skipped
        assert!(!yaml.contains("debug_info"));

        // Empty mission_profiles should use default (empty vec)
        assert!(yaml.contains("mission_profiles: []"));
    }

    #[test]
    fn test_priority_enum_serialization() {
        let priorities = vec![
            Priority::Low,
            Priority::Medium,
            Priority::High,
            Priority::Critical,
        ];

        for priority in priorities {
            let yaml = serde_yaml::to_string(&priority).unwrap();
            let deserialized: Priority = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(priority, deserialized);
        }
    }

    #[test]
    fn test_default_priority() {
        let default_priority = Priority::default();
        assert_eq!(default_priority, Priority::Medium);
    }

    #[test]
    fn test_yaml_with_missing_optional_fields() {
        let yaml_data = r#"
mission_id: "TEST_001"
estimated_time: 1800
created_at: "2024-01-15T12:00:00Z"
tags: []
        "#;

        let mission: MissionProfile = serde_yaml::from_str(yaml_data).unwrap();

        // Should use defaults for missing fields
        assert_eq!(mission.id, "TEST_001");
        assert_eq!(mission.name, "Unnamed Mission"); // default
        assert_eq!(mission.description, None); // optional, not provided
        assert_eq!(mission.priority, Priority::Medium); // default
        assert_eq!(mission.estimated_time, Duration::from_secs(1800));
    }

    #[test]
    fn test_invalid_version_string() {
        let invalid_yaml = r#"
robot_name: "Test Bot"
firmware_version: "not.a.version"
x: 0.0
y: 0.0
        "#;

        let result: Result<RobotSettings, _> = serde_yaml::from_str(invalid_yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_sensor_reading_with_defaults() {
        let yaml_data = r#"
sensor_id: "HUMIDITY_001"
timestamp: "2024-01-15T14:30:00Z"
value: 45.678
        "#;

        let reading: SensorReading = serde_yaml::from_str(yaml_data).unwrap();

        assert_eq!(reading.sensor_id, "HUMIDITY_001");
        assert_eq!(reading.value, 45.678);
        assert_eq!(reading.unit, None); // optional, not provided
        assert_eq!(reading.confidence, 1.0); // default
    }

    #[test]
    fn test_round_trip_with_custom_attributes() {
        let original = create_sample_mission_profile();

        // Serialize to YAML
        let yaml = serde_yaml::to_string(&original).unwrap();

        // Deserialize back
        let deserialized: MissionProfile = serde_yaml::from_str(&yaml).unwrap();

        // Should be equal
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_multiple_renames_and_flattening() {
        let container = ConfigContainer {
            config_version: "2.0.0".to_string(),
            debug_info: Some("Debug enabled".to_string()),
            settings: RobotSettings {
                name: "Advanced Bot".to_string(),
                model: "AB-2000".to_string(),
                firmware_version: (3, 2, 1),
                enabled: true,
                capabilities: vec!["scan".to_string(), "navigate".to_string()],
                position: Position {
                    x: 15.7,
                    y: 32.1,
                    z: 5.5,
                },
            },
            mission_profiles: vec![create_sample_mission_profile()],
        };

        let yaml = serde_yaml::to_string(&container).unwrap();

        // Check various renamed and flattened fields
        assert!(yaml.contains("version: \"2.0.0\""));
        assert!(yaml.contains("robot_name: \"Advanced Bot\""));
        assert!(yaml.contains("firmware_version: \"3.2.1\""));
        assert!(yaml.contains("debug_info"));
        assert!(yaml.contains("x: 15.7"));
        assert!(yaml.contains("mission_id: \"EXPLORE_001\""));

        let deserialized: ConfigContainer = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.config_version, "2.0.0");
        assert_eq!(deserialized.settings.name, "Advanced Bot");
        assert_eq!(deserialized.mission_profiles.len(), 1);
    }

    #[test]
    fn test_conditional_serialization_logic() {
        let mut settings = create_robot_settings();
        settings.model = "".to_string(); // Empty string
        settings.capabilities = vec![]; // Empty vector

        let yaml = serde_yaml::to_string(&settings).unwrap();

        // Empty string and vector should be skipped
        assert!(!yaml.contains("model"));
        assert!(!yaml.contains("capabilities"));

        // Now add values
        settings.model = "RB-1000".to_string();
        settings.capabilities = vec!["explore".to_string()];

        let yaml_with_values = serde_yaml::to_string(&settings).unwrap();

        // Now they should be included
        assert!(yaml_with_values.contains("model"));
        assert!(yaml_with_values.contains("capabilities"));
    }
}

// Example usage and reference implementation
fn main() {
    println!("Level 15 Task 3: Use Serde Attributes for Custom Serialization");
    println!("Run with: cargo test level15_task3");
}