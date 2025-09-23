#[cfg(test)]
mod level15_task5_comprehensive_yaml_export_tests {
    use super::*;
    use serde::{Serialize, Deserialize};
    use std::collections::HashMap;
    use std::time::Duration;

    // Re-use types from previous tasks
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

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    #[serde(untagged)]
    enum EnvironmentValue {
        Text(String),
        Number(f64),
        Boolean(bool),
        List(Vec<String>),
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct NavigationMap {
        name: String,
        dimensions: (u32, u32),
        waypoints: Vec<Waypoint>,
        obstacles: Vec<Obstacle>,
        safe_zones: Vec<Zone>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct Waypoint {
        id: String,
        position: (f64, f64),
        #[serde(default)]
        waypoint_type: WaypointType,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    enum WaypointType {
        Checkpoint,
        RestArea,
        DataCollection,
        Charging,
    }

    impl Default for WaypointType {
        fn default() -> Self {
            WaypointType::Checkpoint
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    #[serde(untagged)]
    enum Obstacle {
        Rectangle { x: f64, y: f64, width: f64, height: f64 },
        Circle { center: (f64, f64), radius: f64 },
        Polygon { vertices: Vec<(f64, f64)> },
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct Zone {
        name: String,
        bounds: ZoneBounds,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct ZoneBounds {
        min: (f64, f64),
        max: (f64, f64),
    }

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
    struct RobotSystemConfig {
        #[serde(flatten)]
        basic_config: AdvancedRobotConfig,

        navigation: NavigationMap,

        mission_profiles: Vec<MissionProfile>,

        performance_tuning: PerformanceTuning,

        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        error_log: Vec<ErrorEntry>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct PerformanceTuning {
        cpu_priority: i32,
        memory_limit_mb: u32,
        cache_size: u32,
        optimization_level: OptimizationLevel,
    }

    #[derive(Serialize, Deserialize, Debug)]
    enum OptimizationLevel {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "basic")]
        Basic,
        #[serde(rename = "aggressive")]
        Aggressive,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct ErrorEntry {
        timestamp: String,
        level: String,
        message: String,
    }

    // Export system structures
    #[derive(Serialize, Deserialize, Debug)]
    struct ExportManifest {
        export_id: String,
        generated_at: String,
        format_version: String,
        total_configs: u32,
        export_formats: Vec<String>,
        file_metadata: HashMap<String, FileMetadata>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct FileMetadata {
        size_bytes: u32,
        checksum: String,
        last_modified: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct ConfigurationBundle {
        bundle_info: BundleInfo,
        system_configs: Vec<RobotSystemConfig>,
        global_settings: GlobalSystemSettings,
        deployment_profiles: Vec<DeploymentProfile>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct BundleInfo {
        bundle_id: String,
        name: String,
        version: String,
        created_by: String,
        description: String,
        target_environment: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct GlobalSystemSettings {
        logging_level: String,
        max_concurrent_robots: u32,
        communication_protocol: String,
        security_settings: SecuritySettings,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct SecuritySettings {
        encryption_enabled: bool,
        authentication_required: bool,
        allowed_operations: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct DeploymentProfile {
        profile_name: String,
        target_robots: Vec<String>,
        configuration_overrides: HashMap<String, serde_yaml::Value>,
        activation_conditions: Vec<String>,
    }

    fn create_sample_basic_config() -> AdvancedRobotConfig {
        let mut env_settings = HashMap::new();
        env_settings.insert("temperature_unit".to_string(), EnvironmentValue::Text("celsius".to_string()));
        env_settings.insert("max_speed".to_string(), EnvironmentValue::Number(5.0));
        env_settings.insert("autonomous_mode".to_string(), EnvironmentValue::Boolean(true));

        AdvancedRobotConfig {
            metadata: ConfigMetadata {
                version: "3.0.0".to_string(),
                created_by: "System Generator".to_string(),
                last_modified: "2024-01-15T15:30:00Z".to_string(),
                description: Some("Comprehensive robot system configuration".to_string()),
            },
            hardware: HardwareConfig {
                sensors: vec![
                    SensorConfig {
                        name: "primary_camera".to_string(),
                        sensor_type: "optical".to_string(),
                        enabled: true,
                        calibration: {
                            let mut cal = HashMap::new();
                            cal.insert("focal_length".to_string(), 24.5);
                            cal.insert("aperture".to_string(), 2.8);
                            cal
                        },
                    },
                    SensorConfig {
                        name: "lidar_scanner".to_string(),
                        sensor_type: "laser".to_string(),
                        enabled: true,
                        calibration: {
                            let mut cal = HashMap::new();
                            cal.insert("range_max".to_string(), 100.0);
                            cal.insert("accuracy".to_string(), 0.05);
                            cal
                        },
                    },
                ],
                actuators: vec![
                    ActuatorConfig {
                        name: "main_drive".to_string(),
                        actuator_type: "servo".to_string(),
                        max_speed: 3.5,
                        precision: 0.01,
                    },
                ],
                power_management: PowerConfig {
                    battery_capacity: 5000,
                    low_power_threshold: 15,
                    charging_rate: 2.5,
                },
            },
            software: SoftwareConfig {
                os_version: "RobotOS 4.0.0".to_string(),
                runtime_version: "Rust 1.75.0".to_string(),
                modules: vec!["navigation".to_string(), "perception".to_string(), "planning".to_string()],
                debug_mode: false,
            },
            missions: vec![
                MissionTemplate {
                    name: "exploration".to_string(),
                    priority: "high".to_string(),
                    estimated_duration: 3600,
                },
                MissionTemplate {
                    name: "maintenance".to_string(),
                    priority: "low".to_string(),
                    estimated_duration: 1800,
                },
            ],
            environment_settings: env_settings,
        }
    }

    fn create_sample_navigation_map() -> NavigationMap {
        NavigationMap {
            name: "Production Facility Map".to_string(),
            dimensions: (100, 80),
            waypoints: vec![
                Waypoint {
                    id: "HOME".to_string(),
                    position: (0.0, 0.0),
                    waypoint_type: WaypointType::Checkpoint,
                },
                Waypoint {
                    id: "STATION_A".to_string(),
                    position: (25.0, 40.0),
                    waypoint_type: WaypointType::DataCollection,
                },
                Waypoint {
                    id: "CHARGING_BAY".to_string(),
                    position: (90.0, 10.0),
                    waypoint_type: WaypointType::Charging,
                },
            ],
            obstacles: vec![
                Obstacle::Rectangle { x: 20.0, y: 20.0, width: 10.0, height: 5.0 },
                Obstacle::Circle { center: (50.0, 50.0), radius: 8.0 },
                Obstacle::Polygon {
                    vertices: vec![(70.0, 60.0), (80.0, 65.0), (75.0, 75.0), (65.0, 70.0)],
                },
            ],
            safe_zones: vec![
                Zone {
                    name: "Entry Zone".to_string(),
                    bounds: ZoneBounds {
                        min: (0.0, 0.0),
                        max: (10.0, 10.0),
                    },
                },
                Zone {
                    name: "Maintenance Area".to_string(),
                    bounds: ZoneBounds {
                        min: (85.0, 5.0),
                        max: (95.0, 15.0),
                    },
                },
            ],
        }
    }

    fn create_sample_mission_profiles() -> Vec<MissionProfile> {
        vec![
            MissionProfile {
                id: "EXPLORE_DEEP".to_string(),
                name: "Deep Exploration Mission".to_string(),
                description: Some("Comprehensive area mapping and exploration".to_string()),
                priority: Priority::High,
                estimated_time: Duration::from_secs(7200),
                metadata: MissionMetadata {
                    created_at: "2024-01-15T10:00:00Z".to_string(),
                    tags: vec!["exploration".to_string(), "mapping".to_string(), "autonomous".to_string()],
                },
            },
            MissionProfile {
                id: "QUICK_SCAN".to_string(),
                name: "Quick Area Scan".to_string(),
                description: None,
                priority: Priority::Medium,
                estimated_time: Duration::from_secs(1800),
                metadata: MissionMetadata {
                    created_at: "2024-01-15T11:30:00Z".to_string(),
                    tags: vec!["scan".to_string(), "quick".to_string()],
                },
            },
            MissionProfile {
                id: "EMERGENCY_RESPONSE".to_string(),
                name: "Emergency Response Protocol".to_string(),
                description: Some("Rapid response to emergency situations".to_string()),
                priority: Priority::Critical,
                estimated_time: Duration::from_secs(900),
                metadata: MissionMetadata {
                    created_at: "2024-01-15T12:00:00Z".to_string(),
                    tags: vec!["emergency".to_string(), "critical".to_string(), "response".to_string()],
                },
            },
        ]
    }

    fn create_performance_tuning() -> PerformanceTuning {
        PerformanceTuning {
            cpu_priority: 15,
            memory_limit_mb: 1024,
            cache_size: 128,
            optimization_level: OptimizationLevel::Aggressive,
        }
    }

    fn export_complete_system_config() -> Result<RobotSystemConfig, Box<dyn std::error::Error>> {
        let system_config = RobotSystemConfig {
            basic_config: create_sample_basic_config(),
            navigation: create_sample_navigation_map(),
            mission_profiles: create_sample_mission_profiles(),
            performance_tuning: create_performance_tuning(),
            error_log: vec![], // Empty log
        };

        Ok(system_config)
    }

    fn create_configuration_bundle() -> ConfigurationBundle {
        let system_config = export_complete_system_config().unwrap();

        ConfigurationBundle {
            bundle_info: BundleInfo {
                bundle_id: "BUNDLE_001".to_string(),
                name: "Production Robot Fleet Configuration".to_string(),
                version: "1.0.0".to_string(),
                created_by: "Fleet Management System".to_string(),
                description: "Complete configuration bundle for production robot fleet deployment".to_string(),
                target_environment: "production".to_string(),
            },
            system_configs: vec![system_config],
            global_settings: GlobalSystemSettings {
                logging_level: "info".to_string(),
                max_concurrent_robots: 10,
                communication_protocol: "MQTT".to_string(),
                security_settings: SecuritySettings {
                    encryption_enabled: true,
                    authentication_required: true,
                    allowed_operations: vec![
                        "move".to_string(),
                        "scan".to_string(),
                        "collect".to_string(),
                        "report".to_string(),
                    ],
                },
            },
            deployment_profiles: vec![
                DeploymentProfile {
                    profile_name: "standard_deployment".to_string(),
                    target_robots: vec!["ROBOT_001".to_string(), "ROBOT_002".to_string()],
                    configuration_overrides: HashMap::new(),
                    activation_conditions: vec!["system_ready".to_string(), "environment_safe".to_string()],
                },
            ],
        }
    }

    fn generate_export_manifest() -> ExportManifest {
        let mut file_metadata = HashMap::new();
        file_metadata.insert("system_config.yaml".to_string(), FileMetadata {
            size_bytes: 4096,
            checksum: "sha256:abc123def456".to_string(),
            last_modified: "2024-01-15T16:00:00Z".to_string(),
        });
        file_metadata.insert("bundle_config.yaml".to_string(), FileMetadata {
            size_bytes: 8192,
            checksum: "sha256:def456ghi789".to_string(),
            last_modified: "2024-01-15T16:05:00Z".to_string(),
        });

        ExportManifest {
            export_id: "EXPORT_20240115_001".to_string(),
            generated_at: "2024-01-15T16:10:00Z".to_string(),
            format_version: "2.0".to_string(),
            total_configs: 2,
            export_formats: vec!["yaml".to_string(), "json".to_string()],
            file_metadata,
        }
    }

    #[test]
    fn test_complete_system_config_export() {
        let result = export_complete_system_config();
        assert!(result.is_ok());

        let config = result.unwrap();

        // Test basic config is included
        assert_eq!(config.basic_config.metadata.version, "3.0.0");
        assert_eq!(config.basic_config.hardware.sensors.len(), 2);
        assert_eq!(config.basic_config.missions.len(), 2);

        // Test navigation map
        assert_eq!(config.navigation.name, "Production Facility Map");
        assert_eq!(config.navigation.waypoints.len(), 3);
        assert_eq!(config.navigation.obstacles.len(), 3);

        // Test mission profiles
        assert_eq!(config.mission_profiles.len(), 3);

        // Test performance tuning
        assert_eq!(config.performance_tuning.cpu_priority, 15);
        assert_eq!(config.performance_tuning.optimization_level, OptimizationLevel::Aggressive);
    }

    #[test]
    fn test_yaml_export_formatting() {
        let config = export_complete_system_config().unwrap();
        let yaml_output = serde_yaml::to_string(&config).unwrap();

        // Should contain flattened basic config fields
        assert!(yaml_output.contains("version: \"3.0.0\""));
        assert!(yaml_output.contains("created_by: \"System Generator\""));

        // Should contain navigation map
        assert!(yaml_output.contains("navigation:"));
        assert!(yaml_output.contains("Production Facility Map"));

        // Should contain mission profiles
        assert!(yaml_output.contains("mission_profiles:"));
        assert!(yaml_output.contains("EXPLORE_DEEP"));

        // Should contain performance tuning
        assert!(yaml_output.contains("performance_tuning:"));
        assert!(yaml_output.contains("cpu_priority: 15"));

        // Error log should be skipped (empty)
        assert!(!yaml_output.contains("error_log"));
    }

    #[test]
    fn test_json_comparison_export() {
        let config = export_complete_system_config().unwrap();

        let yaml_output = serde_yaml::to_string(&config).unwrap();
        let json_output = serde_json::to_string_pretty(&config).unwrap();

        // Both should contain the same data
        assert!(yaml_output.contains("Production Facility Map"));
        assert!(json_output.contains("Production Facility Map"));

        assert!(yaml_output.contains("EXPLORE_DEEP"));
        assert!(json_output.contains("EXPLORE_DEEP"));

        // YAML should be more readable
        assert!(yaml_output.len() < json_output.len() * 2); // YAML is typically more compact
    }

    #[test]
    fn test_configuration_bundle() {
        let bundle = create_configuration_bundle();

        assert_eq!(bundle.bundle_info.bundle_id, "BUNDLE_001");
        assert_eq!(bundle.bundle_info.target_environment, "production");
        assert_eq!(bundle.system_configs.len(), 1);
        assert_eq!(bundle.deployment_profiles.len(), 1);

        // Test global settings
        assert_eq!(bundle.global_settings.max_concurrent_robots, 10);
        assert!(bundle.global_settings.security_settings.encryption_enabled);
        assert_eq!(bundle.global_settings.security_settings.allowed_operations.len(), 4);
    }

    #[test]
    fn test_bundle_yaml_export() {
        let bundle = create_configuration_bundle();
        let yaml_output = serde_yaml::to_string(&bundle).unwrap();

        // Should contain bundle information
        assert!(yaml_output.contains("bundle_info:"));
        assert!(yaml_output.contains("BUNDLE_001"));
        assert!(yaml_output.contains("Production Robot Fleet"));

        // Should contain system configs
        assert!(yaml_output.contains("system_configs:"));

        // Should contain global settings
        assert!(yaml_output.contains("global_settings:"));
        assert!(yaml_output.contains("max_concurrent_robots: 10"));

        // Should contain deployment profiles
        assert!(yaml_output.contains("deployment_profiles:"));
        assert!(yaml_output.contains("standard_deployment"));
    }

    #[test]
    fn test_export_manifest_generation() {
        let manifest = generate_export_manifest();

        assert_eq!(manifest.export_id, "EXPORT_20240115_001");
        assert_eq!(manifest.total_configs, 2);
        assert_eq!(manifest.export_formats.len(), 2);
        assert_eq!(manifest.file_metadata.len(), 2);

        assert!(manifest.export_formats.contains(&"yaml".to_string()));
        assert!(manifest.export_formats.contains(&"json".to_string()));

        assert!(manifest.file_metadata.contains_key("system_config.yaml"));
        assert!(manifest.file_metadata.contains_key("bundle_config.yaml"));
    }

    #[test]
    fn test_manifest_yaml_export() {
        let manifest = generate_export_manifest();
        let yaml_output = serde_yaml::to_string(&manifest).unwrap();

        assert!(yaml_output.contains("export_id: EXPORT_20240115_001"));
        assert!(yaml_output.contains("format_version: \"2.0\""));
        assert!(yaml_output.contains("total_configs: 2"));
        assert!(yaml_output.contains("file_metadata:"));
        assert!(yaml_output.contains("system_config.yaml"));
        assert!(yaml_output.contains("bundle_config.yaml"));
    }

    #[test]
    fn test_deployment_profile_configuration() {
        let bundle = create_configuration_bundle();
        let profile = &bundle.deployment_profiles[0];

        assert_eq!(profile.profile_name, "standard_deployment");
        assert_eq!(profile.target_robots.len(), 2);
        assert!(profile.target_robots.contains(&"ROBOT_001".to_string()));
        assert!(profile.target_robots.contains(&"ROBOT_002".to_string()));
        assert_eq!(profile.activation_conditions.len(), 2);
    }

    #[test]
    fn test_security_settings_export() {
        let bundle = create_configuration_bundle();
        let security = &bundle.global_settings.security_settings;

        assert!(security.encryption_enabled);
        assert!(security.authentication_required);
        assert_eq!(security.allowed_operations.len(), 4);

        let yaml_output = serde_yaml::to_string(&bundle).unwrap();
        assert!(yaml_output.contains("encryption_enabled: true"));
        assert!(yaml_output.contains("authentication_required: true"));
        assert!(yaml_output.contains("allowed_operations:"));
    }

    #[test]
    fn test_mission_profile_attributes() {
        let profiles = create_sample_mission_profiles();

        // Test mission with description
        let deep_exploration = &profiles[0];
        assert!(deep_exploration.description.is_some());
        assert_eq!(deep_exploration.priority, Priority::High);
        assert_eq!(deep_exploration.metadata.tags.len(), 3);

        // Test mission without description
        let quick_scan = &profiles[1];
        assert!(quick_scan.description.is_none());
        assert_eq!(quick_scan.priority, Priority::Medium);

        // Test critical mission
        let emergency = &profiles[2];
        assert_eq!(emergency.priority, Priority::Critical);
        assert_eq!(emergency.estimated_time, Duration::from_secs(900));
    }

    #[test]
    fn test_flattened_basic_config() {
        let config = export_complete_system_config().unwrap();
        let yaml_output = serde_yaml::to_string(&config).unwrap();

        // Basic config fields should be flattened to top level
        assert!(yaml_output.contains("version: \"3.0.0\"")); // from metadata
        assert!(yaml_output.contains("created_by: \"System Generator\""));
        assert!(yaml_output.contains("hardware:"));
        assert!(yaml_output.contains("software:"));
        assert!(yaml_output.contains("missions:"));

        // Should not have a separate "basic_config:" section
        assert!(!yaml_output.contains("basic_config:"));
    }

    #[test]
    fn test_performance_optimization_levels() {
        let levels = vec![
            OptimizationLevel::None,
            OptimizationLevel::Basic,
            OptimizationLevel::Aggressive,
        ];

        for level in levels {
            let yaml = serde_yaml::to_string(&level).unwrap();
            let deserialized: OptimizationLevel = serde_yaml::from_str(&yaml).unwrap();

            match level {
                OptimizationLevel::None => assert!(yaml.contains("none")),
                OptimizationLevel::Basic => assert!(yaml.contains("basic")),
                OptimizationLevel::Aggressive => assert!(yaml.contains("aggressive")),
            }
        }
    }

    #[test]
    fn test_empty_error_log_skipping() {
        let config = export_complete_system_config().unwrap();
        let yaml_output = serde_yaml::to_string(&config).unwrap();

        // Empty error log should be skipped
        assert!(!yaml_output.contains("error_log"));

        // Create config with error log
        let mut config_with_errors = config;
        config_with_errors.error_log = vec![
            ErrorEntry {
                timestamp: "2024-01-15T10:00:00Z".to_string(),
                level: "warning".to_string(),
                message: "Low battery".to_string(),
            },
        ];

        let yaml_with_errors = serde_yaml::to_string(&config_with_errors).unwrap();
        assert!(yaml_with_errors.contains("error_log:"));
        assert!(yaml_with_errors.contains("Low battery"));
    }

    #[test]
    fn test_round_trip_system_config() {
        let original_config = export_complete_system_config().unwrap();

        // Serialize to YAML
        let yaml_output = serde_yaml::to_string(&original_config).unwrap();

        // Deserialize back
        let deserialized_config: RobotSystemConfig = serde_yaml::from_str(&yaml_output).unwrap();

        // Compare key fields (full comparison is complex due to flattening)
        assert_eq!(original_config.basic_config.metadata.version,
                   deserialized_config.basic_config.metadata.version);
        assert_eq!(original_config.navigation.name,
                   deserialized_config.navigation.name);
        assert_eq!(original_config.mission_profiles.len(),
                   deserialized_config.mission_profiles.len());
        assert_eq!(original_config.performance_tuning.cpu_priority,
                   deserialized_config.performance_tuning.cpu_priority);
    }

    #[test]
    fn test_comprehensive_export_formats() {
        let config = export_complete_system_config().unwrap();
        let bundle = create_configuration_bundle();
        let manifest = generate_export_manifest();

        // Test all exports work
        let config_yaml = serde_yaml::to_string(&config).unwrap();
        let config_json = serde_json::to_string_pretty(&config).unwrap();

        let bundle_yaml = serde_yaml::to_string(&bundle).unwrap();
        let bundle_json = serde_json::to_string_pretty(&bundle).unwrap();

        let manifest_yaml = serde_yaml::to_string(&manifest).unwrap();
        let manifest_json = serde_json::to_string_pretty(&manifest).unwrap();

        // All should be valid and contain expected content
        assert!(config_yaml.contains("Production Facility Map"));
        assert!(config_json.contains("Production Facility Map"));

        assert!(bundle_yaml.contains("BUNDLE_001"));
        assert!(bundle_json.contains("BUNDLE_001"));

        assert!(manifest_yaml.contains("EXPORT_20240115_001"));
        assert!(manifest_json.contains("EXPORT_20240115_001"));
    }
}

// Example usage and reference implementation
fn main() {
    println!("Level 15 Task 5: Create Comprehensive YAML Export System");
    println!("Run with: cargo test level15_task5");
}