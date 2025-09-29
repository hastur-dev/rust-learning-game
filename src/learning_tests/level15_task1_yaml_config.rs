// Level 15 Task 1 Test: Define Advanced Robot Configuration with YAML
// Tests that user creates complex configuration structures for YAML

#[cfg(test)]
mod level15_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_serde_yaml_import() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_serde_yaml = analyzer.code.contains("serde_yaml");
        assert!(
            has_serde_yaml,
            "❌ You need to use serde_yaml for YAML processing"
        );
    }

    #[test]
    fn test_has_advanced_robot_config_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct AdvancedRobotConfig"),
            "❌ You need to define an AdvancedRobotConfig struct"
        );
    }

    #[test]
    fn test_has_config_metadata_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct ConfigMetadata"),
            "❌ You need to define a ConfigMetadata struct"
        );
    }

    #[test]
    fn test_has_hardware_config_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct HardwareConfig"),
            "❌ You need to define a HardwareConfig struct"
        );
    }

    #[test]
    fn test_has_sensor_config_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct SensorConfig"),
            "❌ You need to define a SensorConfig struct"
        );
    }

    #[test]
    fn test_uses_hashmap() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_hashmap = analyzer.code.contains("HashMap") &&
                         analyzer.code.contains("std::collections::HashMap");
        assert!(
            has_hashmap,
            "❌ You should use HashMap for key-value configuration data"
        );
    }

    #[test]
    fn test_has_serde_rename_attribute() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("#[serde(rename =") &&
            analyzer.code.contains("\"type\""),
            "❌ You should use #[serde(rename = \"type\")] for the sensor_type field"
        );
    }

    #[test]
    fn test_advanced_config_has_required_fields() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_metadata = analyzer.code.contains("metadata:");
        let has_hardware = analyzer.code.contains("hardware:");
        let has_software = analyzer.code.contains("software:");
        let has_missions = analyzer.code.contains("missions:");
        let has_environment = analyzer.code.contains("environment_settings:");

        assert!(has_metadata, "❌ AdvancedRobotConfig should have a 'metadata' field");
        assert!(has_hardware, "❌ AdvancedRobotConfig should have a 'hardware' field");
        assert!(has_software, "❌ AdvancedRobotConfig should have a 'software' field");
        assert!(has_missions, "❌ AdvancedRobotConfig should have a 'missions' field");
        assert!(has_environment, "❌ AdvancedRobotConfig should have an 'environment_settings' field");
    }

    #[test]
    fn test_metadata_has_version_field() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_version = analyzer.code.contains("version: String");
        assert!(
            has_version,
            "❌ ConfigMetadata should have a 'version: String' field"
        );
    }

    #[test]
    fn test_hardware_has_sensor_vector() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_sensors = analyzer.code.contains("sensors: Vec<SensorConfig>");
        assert!(
            has_sensors,
            "❌ HardwareConfig should have a 'sensors: Vec<SensorConfig>' field"
        );
    }

    #[test]
    fn test_sensor_config_has_calibration() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_calibration = analyzer.code.contains("calibration: HashMap");
        assert!(
            has_calibration,
            "❌ SensorConfig should have a 'calibration: HashMap' field"
        );
    }

    #[test]
    fn test_uses_option_type() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_option = analyzer.code.contains("Option<");
        assert!(
            has_option,
            "❌ You should use Option<> for optional fields like description"
        );
    }

    #[test]
    fn test_all_structs_have_serde_derives() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let serialize_count = analyzer.code.matches("Serialize").count();
        let deserialize_count = analyzer.code.matches("Deserialize").count();

        assert!(
            serialize_count >= 4,
            "❌ Multiple structs should derive Serialize (found {})",
            serialize_count
        );
        assert!(
            deserialize_count >= 4,
            "❌ Multiple structs should derive Deserialize (found {})",
            deserialize_count
        );
    }
}

// Reference implementation
fn main() {
    println!("Level 15 Task 1: YAML Config Structures");
    // Reference pattern for advanced YAML configuration structures
}

// Reference advanced configuration pattern
// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct AdvancedRobotConfig {
//     metadata: ConfigMetadata,
//     hardware: HardwareConfig,
//     software: SoftwareConfig,
//     missions: Vec<MissionTemplate>,
//     environment_settings: HashMap<String, EnvironmentValue>,
// }