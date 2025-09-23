#[cfg(test)]
mod level15_task2_yaml_processing_tests {
    use super::*;
    use serde::{Serialize, Deserialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct AdvancedRobotConfig {
        metadata: ConfigMetadata,
        hardware: HardwareConfig,
        software: SoftwareConfig,
        missions: Vec<MissionTemplate>,
        environment_settings: HashMap<String, EnvironmentValue>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct ConfigMetadata {
        version: String,
        created_by: String,
        last_modified: String,
        description: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct HardwareConfig {
        sensors: Vec<SensorConfig>,
        actuators: Vec<ActuatorConfig>,
        power_management: PowerConfig,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct SoftwareConfig {
        os_version: String,
        runtime_version: String,
        modules: Vec<String>,
        debug_mode: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct SensorConfig {
        name: String,
        #[serde(rename = "type")]
        sensor_type: String,
        enabled: bool,
        calibration: HashMap<String, f64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct ActuatorConfig {
        name: String,
        #[serde(rename = "type")]
        actuator_type: String,
        max_speed: f64,
        precision: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct PowerConfig {
        battery_capacity: u32,
        low_power_threshold: u32,
        charging_rate: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct MissionTemplate {
        name: String,
        priority: String,
        estimated_duration: u32,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(untagged)]
    enum EnvironmentValue {
        Text(String),
        Number(f64),
        Boolean(bool),
        List(Vec<String>),
    }

    fn process_robot_settings_yaml() -> Result<AdvancedRobotConfig, serde_yaml::Error> {
        let yaml_data = r#"
metadata:
  version: "2.1.0"
  created_by: "Engineering Team"
  last_modified: "2024-01-15"
  description: "Advanced robot configuration for exploration missions"

hardware:
  sensors:
    - name: "primary_camera"
      type: "optical"
      enabled: true
      calibration:
        focal_length: 24.5
        aperture: 2.8
        iso_sensitivity: 800
    - name: "lidar_scanner"
      type: "laser"
      enabled: true
      calibration:
        range_max: 100.0
        accuracy: 0.05
        scan_rate: 10.0

  actuators:
    - name: "main_drive"
      type: "servo"
      max_speed: 3.5
      precision: 0.01
    - name: "arm_joint_1"
      type: "stepper"
      max_speed: 1.2
      precision: 0.001

  power_management:
    battery_capacity: 5000
    low_power_threshold: 15
    charging_rate: 2.5

software:
  os_version: "RobotOS 3.2.1"
  runtime_version: "Rust 1.70.0"
  modules:
    - "navigation"
    - "sensor_fusion"
    - "ai_planning"
  debug_mode: false

missions:
  - name: "exploration"
    priority: "high"
    estimated_duration: 1800
  - name: "data_collection"
    priority: "medium"
    estimated_duration: 900

environment_settings:
  temperature_unit: "celsius"
  distance_unit: "meters"
  coordinate_system: "cartesian"
  logging_level: "info"
  max_speed: 5.0
  autonomous_mode: true
        "#;

        let config: AdvancedRobotConfig = serde_yaml::from_str(yaml_data)?;
        println!("Loaded advanced config version: {}", config.metadata.version);

        for sensor in &config.hardware.sensors {
            println!("Sensor '{}' ({}): enabled = {}",
                     sensor.name, sensor.sensor_type, sensor.enabled);
        }

        for actuator in &config.hardware.actuators {
            println!("Actuator '{}' ({}): max_speed = {}",
                     actuator.name, actuator.actuator_type, actuator.max_speed);
        }

        println!("Power management: battery={}mAh, threshold={}%",
                 config.hardware.power_management.battery_capacity,
                 config.hardware.power_management.low_power_threshold);

        Ok(config)
    }

    fn process_mission_config_yaml() -> Result<Vec<MissionTemplate>, serde_yaml::Error> {
        let yaml_data = r#"
- name: "perimeter_scan"
  priority: "high"
  estimated_duration: 600
- name: "sample_collection"
  priority: "medium"
  estimated_duration: 1200
- name: "emergency_response"
  priority: "critical"
  estimated_duration: 300
- name: "maintenance_check"
  priority: "low"
  estimated_duration: 1800
        "#;

        let missions: Vec<MissionTemplate> = serde_yaml::from_str(yaml_data)?;

        println!("Loaded {} mission templates:", missions.len());
        for mission in &missions {
            println!("  - {} ({}): {}s", mission.name, mission.priority, mission.estimated_duration);
        }

        Ok(missions)
    }

    fn process_sensor_calibration_yaml() -> Result<HashMap<String, SensorConfig>, serde_yaml::Error> {
        let yaml_data = r#"
primary_camera:
  name: "primary_camera"
  type: "optical"
  enabled: true
  calibration:
    focal_length: 24.5
    aperture: 2.8
    iso_sensitivity: 800
    zoom_factor: 1.0

lidar_scanner:
  name: "lidar_scanner"
  type: "laser"
  enabled: true
  calibration:
    range_max: 100.0
    accuracy: 0.05
    scan_rate: 10.0
    beam_width: 0.1

thermal_sensor:
  name: "thermal_sensor"
  type: "infrared"
  enabled: false
  calibration:
    temperature_range_min: -20.0
    temperature_range_max: 150.0
    resolution: 0.1

proximity_sensor:
  name: "proximity_sensor"
  type: "ultrasonic"
  enabled: true
  calibration:
    range_min: 0.02
    range_max: 4.0
    frequency: 40000.0
        "#;

        let sensors: HashMap<String, SensorConfig> = serde_yaml::from_str(yaml_data)?;

        println!("Loaded {} sensor configurations:", sensors.len());
        for (key, sensor) in &sensors {
            println!("  - {}: {} ({} calibration params)",
                     key, sensor.sensor_type, sensor.calibration.len());
        }

        Ok(sensors)
    }

    fn process_environment_data_yaml() -> Result<HashMap<String, EnvironmentValue>, serde_yaml::Error> {
        let yaml_data = r#"
# Basic configuration values
temperature_unit: "celsius"
distance_unit: "meters"
coordinate_system: "cartesian"
logging_level: "info"

# Numeric values
max_speed: 5.0
default_timeout: 30.0
scan_frequency: 2.5

# Boolean flags
autonomous_mode: true
collision_avoidance: true
energy_saving: false
debug_mode: false

# List values
supported_formats:
  - "json"
  - "yaml"
  - "xml"

navigation_modes:
  - "manual"
  - "assisted"
  - "autonomous"

error_levels:
  - "debug"
  - "info"
  - "warning"
  - "error"
  - "critical"
        "#;

        let env_data: HashMap<String, EnvironmentValue> = serde_yaml::from_str(yaml_data)?;

        println!("Loaded {} environment settings:", env_data.len());
        for (key, value) in &env_data {
            match value {
                EnvironmentValue::Text(s) => println!("  - {}: \"{}\"", key, s),
                EnvironmentValue::Number(n) => println!("  - {}: {}", key, n),
                EnvironmentValue::Boolean(b) => println!("  - {}: {}", key, b),
                EnvironmentValue::List(l) => println!("  - {}: {:?}", key, l),
            }
        }

        Ok(env_data)
    }

    fn create_complete_config_from_parts() -> Result<AdvancedRobotConfig, Box<dyn std::error::Error>> {
        let base_config = process_robot_settings_yaml()?;
        let additional_missions = process_mission_config_yaml()?;
        let sensor_configs = process_sensor_calibration_yaml()?;
        let env_data = process_environment_data_yaml()?;

        let mut complete_config = base_config;

        // Add additional missions
        complete_config.missions.extend(additional_missions);

        // Update sensor configurations with calibration data
        for (sensor_name, sensor_config) in sensor_configs {
            if let Some(existing_sensor) = complete_config.hardware.sensors
                .iter_mut()
                .find(|s| s.name == sensor_name) {
                existing_sensor.calibration = sensor_config.calibration;
            } else {
                complete_config.hardware.sensors.push(sensor_config);
            }
        }

        // Merge environment settings
        complete_config.environment_settings.extend(env_data);

        println!("Created complete configuration with:");
        println!("  - {} sensors", complete_config.hardware.sensors.len());
        println!("  - {} actuators", complete_config.hardware.actuators.len());
        println!("  - {} missions", complete_config.missions.len());
        println!("  - {} environment settings", complete_config.environment_settings.len());

        Ok(complete_config)
    }

    #[test]
    fn test_basic_yaml_parsing() {
        let result = process_robot_settings_yaml();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.metadata.version, "2.1.0");
        assert_eq!(config.metadata.created_by, "Engineering Team");
        assert!(config.metadata.description.is_some());
    }

    #[test]
    fn test_hardware_configuration() {
        let config = process_robot_settings_yaml().unwrap();

        assert_eq!(config.hardware.sensors.len(), 2);
        assert_eq!(config.hardware.actuators.len(), 2);

        let camera = &config.hardware.sensors[0];
        assert_eq!(camera.name, "primary_camera");
        assert_eq!(camera.sensor_type, "optical");
        assert!(camera.enabled);
        assert_eq!(camera.calibration.len(), 3);

        let power = &config.hardware.power_management;
        assert_eq!(power.battery_capacity, 5000);
        assert_eq!(power.low_power_threshold, 15);
        assert_eq!(power.charging_rate, 2.5);
    }

    #[test]
    fn test_software_configuration() {
        let config = process_robot_settings_yaml().unwrap();

        assert_eq!(config.software.os_version, "RobotOS 3.2.1");
        assert_eq!(config.software.runtime_version, "Rust 1.70.0");
        assert_eq!(config.software.modules.len(), 3);
        assert!(!config.software.debug_mode);

        assert!(config.software.modules.contains(&"navigation".to_string()));
        assert!(config.software.modules.contains(&"sensor_fusion".to_string()));
        assert!(config.software.modules.contains(&"ai_planning".to_string()));
    }

    #[test]
    fn test_mission_templates() {
        let config = process_robot_settings_yaml().unwrap();

        assert_eq!(config.missions.len(), 2);

        let exploration = &config.missions[0];
        assert_eq!(exploration.name, "exploration");
        assert_eq!(exploration.priority, "high");
        assert_eq!(exploration.estimated_duration, 1800);

        let data_collection = &config.missions[1];
        assert_eq!(data_collection.name, "data_collection");
        assert_eq!(data_collection.priority, "medium");
        assert_eq!(data_collection.estimated_duration, 900);
    }

    #[test]
    fn test_environment_settings() {
        let config = process_robot_settings_yaml().unwrap();

        assert!(config.environment_settings.contains_key("temperature_unit"));
        assert!(config.environment_settings.contains_key("distance_unit"));
        assert!(config.environment_settings.contains_key("max_speed"));
        assert!(config.environment_settings.contains_key("autonomous_mode"));

        if let Some(EnvironmentValue::Text(unit)) = config.environment_settings.get("temperature_unit") {
            assert_eq!(unit, "celsius");
        } else {
            panic!("Expected text value for temperature_unit");
        }

        if let Some(EnvironmentValue::Number(speed)) = config.environment_settings.get("max_speed") {
            assert_eq!(*speed, 5.0);
        } else {
            panic!("Expected number value for max_speed");
        }
    }

    #[test]
    fn test_additional_missions() {
        let missions = process_mission_config_yaml().unwrap();

        assert_eq!(missions.len(), 4);

        let perimeter = &missions[0];
        assert_eq!(perimeter.name, "perimeter_scan");
        assert_eq!(perimeter.priority, "high");
        assert_eq!(perimeter.estimated_duration, 600);

        let emergency = &missions[2];
        assert_eq!(emergency.name, "emergency_response");
        assert_eq!(emergency.priority, "critical");
        assert_eq!(emergency.estimated_duration, 300);
    }

    #[test]
    fn test_sensor_calibration() {
        let sensors = process_sensor_calibration_yaml().unwrap();

        assert_eq!(sensors.len(), 4);
        assert!(sensors.contains_key("primary_camera"));
        assert!(sensors.contains_key("lidar_scanner"));
        assert!(sensors.contains_key("thermal_sensor"));
        assert!(sensors.contains_key("proximity_sensor"));

        let thermal = &sensors["thermal_sensor"];
        assert_eq!(thermal.sensor_type, "infrared");
        assert!(!thermal.enabled);
        assert!(thermal.calibration.contains_key("temperature_range_min"));
        assert!(thermal.calibration.contains_key("temperature_range_max"));
    }

    #[test]
    fn test_environment_data_types() {
        let env_data = process_environment_data_yaml().unwrap();

        // Test string values
        if let Some(EnvironmentValue::Text(unit)) = env_data.get("temperature_unit") {
            assert_eq!(unit, "celsius");
        } else {
            panic!("Expected text value");
        }

        // Test numeric values
        if let Some(EnvironmentValue::Number(speed)) = env_data.get("max_speed") {
            assert_eq!(*speed, 5.0);
        } else {
            panic!("Expected number value");
        }

        // Test boolean values
        if let Some(EnvironmentValue::Boolean(mode)) = env_data.get("autonomous_mode") {
            assert!(*mode);
        } else {
            panic!("Expected boolean value");
        }

        // Test list values
        if let Some(EnvironmentValue::List(formats)) = env_data.get("supported_formats") {
            assert_eq!(formats.len(), 3);
            assert!(formats.contains(&"json".to_string()));
            assert!(formats.contains(&"yaml".to_string()));
            assert!(formats.contains(&"xml".to_string()));
        } else {
            panic!("Expected list value");
        }
    }

    #[test]
    fn test_complete_config_assembly() {
        let result = create_complete_config_from_parts();
        assert!(result.is_ok());

        let config = result.unwrap();

        // Should have sensors from both base config and calibration data
        assert!(config.hardware.sensors.len() >= 2);

        // Should have missions from both sources
        assert!(config.missions.len() >= 6); // 2 from base + 4 from additional

        // Should have environment settings from both sources
        assert!(config.environment_settings.len() > 4);
    }

    #[test]
    fn test_yaml_round_trip() {
        let original_config = process_robot_settings_yaml().unwrap();

        // Serialize to YAML
        let yaml_string = serde_yaml::to_string(&original_config).unwrap();

        // Deserialize back from YAML
        let deserialized_config: AdvancedRobotConfig = serde_yaml::from_str(&yaml_string).unwrap();

        // Should be identical
        assert_eq!(original_config, deserialized_config);
    }

    #[test]
    fn test_sensor_type_rename() {
        let config = process_robot_settings_yaml().unwrap();

        // Test that 'type' field is correctly renamed from YAML
        let camera = &config.hardware.sensors[0];
        assert_eq!(camera.sensor_type, "optical");

        let lidar = &config.hardware.sensors[1];
        assert_eq!(lidar.sensor_type, "laser");
    }

    #[test]
    fn test_optional_description() {
        let config = process_robot_settings_yaml().unwrap();

        assert!(config.metadata.description.is_some());
        assert_eq!(config.metadata.description.unwrap(),
                   "Advanced robot configuration for exploration missions");
    }

    #[test]
    fn test_calibration_data_access() {
        let config = process_robot_settings_yaml().unwrap();

        let camera = &config.hardware.sensors[0];
        assert_eq!(camera.calibration.get("focal_length"), Some(&24.5));
        assert_eq!(camera.calibration.get("aperture"), Some(&2.8));
        assert_eq!(camera.calibration.get("iso_sensitivity"), Some(&800.0));

        let lidar = &config.hardware.sensors[1];
        assert_eq!(lidar.calibration.get("range_max"), Some(&100.0));
        assert_eq!(lidar.calibration.get("accuracy"), Some(&0.05));
        assert_eq!(lidar.calibration.get("scan_rate"), Some(&10.0));
    }

    #[test]
    fn test_actuator_configuration() {
        let config = process_robot_settings_yaml().unwrap();

        assert_eq!(config.hardware.actuators.len(), 2);

        let main_drive = &config.hardware.actuators[0];
        assert_eq!(main_drive.name, "main_drive");
        assert_eq!(main_drive.actuator_type, "servo");
        assert_eq!(main_drive.max_speed, 3.5);
        assert_eq!(main_drive.precision, 0.01);

        let arm_joint = &config.hardware.actuators[1];
        assert_eq!(arm_joint.name, "arm_joint_1");
        assert_eq!(arm_joint.actuator_type, "stepper");
        assert_eq!(arm_joint.max_speed, 1.2);
        assert_eq!(arm_joint.precision, 0.001);
    }

    #[test]
    fn test_error_handling() {
        let invalid_yaml = r#"
metadata:
  version: "2.1.0"
  created_by: "Engineering Team"
  last_modified: "2024-01-15"
  # Missing required fields

hardware:
  # Invalid structure
  sensors: "not_an_array"
        "#;

        let result: Result<AdvancedRobotConfig, _> = serde_yaml::from_str(invalid_yaml);
        assert!(result.is_err());
    }
}

// Example usage and reference implementation
fn main() {
    println!("Level 15 Task 2: Process YAML Configuration Files");
    println!("Run with: cargo test level15_task2");
}