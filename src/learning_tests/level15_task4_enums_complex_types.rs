#[cfg(test)]
mod level15_task4_enums_complex_types_tests {
    use super::*;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    #[serde(tag = "type", content = "data")]
    enum EnvironmentValue {
        Text(String),
        Number(f64),
        Boolean(bool),
        List(Vec<String>),
        Coordinates { x: f64, y: f64, z: f64 },
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

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(tag = "kind")]
    enum RobotTask {
        #[serde(rename = "movement")]
        Move { destination: (f64, f64), speed: f64 },

        #[serde(rename = "scanning")]
        Scan { area: String, resolution: f64 },

        #[serde(rename = "collection")]
        Collect { item_type: String, quantity: u32 },

        #[serde(rename = "communication")]
        Transmit { message: String, target: Option<String> },
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct RobotMission {
        id: String,
        name: String,
        tasks: Vec<RobotTask>,
        priority: MissionPriority,
        environment_config: std::collections::HashMap<String, EnvironmentValue>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    enum MissionPriority {
        Low,
        Medium,
        High,
        Critical,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "snake_case")]
    enum SensorType {
        OpticalCamera,
        LidarScanner,
        ThermalImaging,
        UltrasonicRange,
        MotionDetector,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct SensorConfiguration {
        sensor_type: SensorType,
        enabled: bool,
        settings: SensorSettings,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    enum SensorSettings {
        Camera {
            resolution: String,
            fps: u32,
            auto_focus: bool,
        },
        Lidar {
            range_meters: f64,
            scan_rate_hz: f64,
            point_density: u32,
        },
        Range {
            max_distance: f64,
            beam_angle: f64,
        },
        Generic {
            parameters: std::collections::HashMap<String, serde_yaml::Value>,
        },
    }

    fn process_navigation_map_yaml() -> Result<NavigationMap, serde_yaml::Error> {
        let yaml_data = r#"
name: "Laboratory Floor Map"
dimensions: [50, 30]
waypoints:
  - id: "START"
    position: [0.0, 0.0]
    waypoint_type: "Checkpoint"
  - id: "LAB_CENTER"
    position: [25.0, 15.0]
    waypoint_type: "DataCollection"
  - id: "CHARGE_STATION"
    position: [45.0, 5.0]
    waypoint_type: "Charging"

obstacles:
  - x: 10.0
    y: 10.0
    width: 5.0
    height: 3.0
  - center: [30.0, 20.0]
    radius: 2.5
  - vertices:
      - [35.0, 10.0]
      - [40.0, 12.0]
      - [38.0, 18.0]
      - [33.0, 16.0]

safe_zones:
  - name: "Entry Zone"
    bounds:
      min: [0.0, 0.0]
      max: [5.0, 5.0]
  - name: "Staging Area"
    bounds:
      min: [40.0, 25.0]
      max: [50.0, 30.0]
        "#;

        let map: NavigationMap = serde_yaml::from_str(yaml_data)?;
        println!("Loaded map '{}' with {} waypoints", map.name, map.waypoints.len());
        Ok(map)
    }

    fn create_complex_robot_mission() -> RobotMission {
        let mut env_config = std::collections::HashMap::new();
        env_config.insert("temperature_unit".to_string(),
                          EnvironmentValue::Text("celsius".to_string()));
        env_config.insert("max_speed".to_string(),
                          EnvironmentValue::Number(5.0));
        env_config.insert("autonomous_mode".to_string(),
                          EnvironmentValue::Boolean(true));
        env_config.insert("waypoint_names".to_string(),
                          EnvironmentValue::List(vec!["START".to_string(), "GOAL".to_string()]));
        env_config.insert("home_position".to_string(),
                          EnvironmentValue::Coordinates { x: 0.0, y: 0.0, z: 0.0 });

        RobotMission {
            id: "COMPLEX_001".to_string(),
            name: "Multi-Task Exploration".to_string(),
            tasks: vec![
                RobotTask::Move {
                    destination: (10.0, 15.0),
                    speed: 2.5,
                },
                RobotTask::Scan {
                    area: "sector_alpha".to_string(),
                    resolution: 0.1,
                },
                RobotTask::Collect {
                    item_type: "sample".to_string(),
                    quantity: 3,
                },
                RobotTask::Transmit {
                    message: "Mission status: in progress".to_string(),
                    target: Some("base_station".to_string()),
                },
            ],
            priority: MissionPriority::High,
            environment_config: env_config,
        }
    }

    fn create_sensor_configurations() -> Vec<SensorConfiguration> {
        vec![
            SensorConfiguration {
                sensor_type: SensorType::OpticalCamera,
                enabled: true,
                settings: SensorSettings::Camera {
                    resolution: "1920x1080".to_string(),
                    fps: 30,
                    auto_focus: true,
                },
            },
            SensorConfiguration {
                sensor_type: SensorType::LidarScanner,
                enabled: true,
                settings: SensorSettings::Lidar {
                    range_meters: 100.0,
                    scan_rate_hz: 10.0,
                    point_density: 1000,
                },
            },
            SensorConfiguration {
                sensor_type: SensorType::UltrasonicRange,
                enabled: false,
                settings: SensorSettings::Range {
                    max_distance: 5.0,
                    beam_angle: 15.0,
                },
            },
        ]
    }

    #[test]
    fn test_environment_value_enum() {
        let values = vec![
            EnvironmentValue::Text("hello".to_string()),
            EnvironmentValue::Number(42.5),
            EnvironmentValue::Boolean(true),
            EnvironmentValue::List(vec!["a".to_string(), "b".to_string()]),
            EnvironmentValue::Coordinates { x: 1.0, y: 2.0, z: 3.0 },
        ];

        for value in values {
            let yaml = serde_yaml::to_string(&value).unwrap();
            let deserialized: EnvironmentValue = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(value, deserialized);
        }
    }

    #[test]
    fn test_tagged_enum_serialization() {
        let coord_value = EnvironmentValue::Coordinates { x: 10.0, y: 20.0, z: 30.0 };
        let yaml = serde_yaml::to_string(&coord_value).unwrap();

        // Should contain type tag and data content
        assert!(yaml.contains("type: Coordinates"));
        assert!(yaml.contains("data:"));
        assert!(yaml.contains("x: 10"));
        assert!(yaml.contains("y: 20"));
        assert!(yaml.contains("z: 30"));

        let deserialized: EnvironmentValue = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(coord_value, deserialized);
    }

    #[test]
    fn test_navigation_map_processing() {
        let map = process_navigation_map_yaml().unwrap();

        assert_eq!(map.name, "Laboratory Floor Map");
        assert_eq!(map.dimensions, (50, 30));
        assert_eq!(map.waypoints.len(), 3);
        assert_eq!(map.obstacles.len(), 3);
        assert_eq!(map.safe_zones.len(), 2);

        // Test waypoint types
        assert_eq!(map.waypoints[0].waypoint_type, WaypointType::Checkpoint);
        assert_eq!(map.waypoints[1].waypoint_type, WaypointType::DataCollection);
        assert_eq!(map.waypoints[2].waypoint_type, WaypointType::Charging);
    }

    #[test]
    fn test_untagged_obstacles() {
        let map = process_navigation_map_yaml().unwrap();

        // Test rectangle obstacle
        if let Obstacle::Rectangle { x, y, width, height } = &map.obstacles[0] {
            assert_eq!(*x, 10.0);
            assert_eq!(*y, 10.0);
            assert_eq!(*width, 5.0);
            assert_eq!(*height, 3.0);
        } else {
            panic!("Expected Rectangle obstacle");
        }

        // Test circle obstacle
        if let Obstacle::Circle { center, radius } = &map.obstacles[1] {
            assert_eq!(*center, (30.0, 20.0));
            assert_eq!(*radius, 2.5);
        } else {
            panic!("Expected Circle obstacle");
        }

        // Test polygon obstacle
        if let Obstacle::Polygon { vertices } = &map.obstacles[2] {
            assert_eq!(vertices.len(), 4);
            assert_eq!(vertices[0], (35.0, 10.0));
        } else {
            panic!("Expected Polygon obstacle");
        }
    }

    #[test]
    fn test_waypoint_default_type() {
        let yaml_data = r#"
id: "DEFAULT_TEST"
position: [5.0, 5.0]
        "#;

        let waypoint: Waypoint = serde_yaml::from_str(yaml_data).unwrap();
        assert_eq!(waypoint.waypoint_type, WaypointType::Checkpoint); // Default
    }

    #[test]
    fn test_robot_task_enum() {
        let tasks = vec![
            RobotTask::Move { destination: (1.0, 2.0), speed: 1.5 },
            RobotTask::Scan { area: "zone_1".to_string(), resolution: 0.5 },
            RobotTask::Collect { item_type: "mineral".to_string(), quantity: 5 },
            RobotTask::Transmit {
                message: "Hello".to_string(),
                target: None,
            },
        ];

        for task in tasks {
            let yaml = serde_yaml::to_string(&task).unwrap();
            let deserialized: RobotTask = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(task, deserialized);
        }
    }

    #[test]
    fn test_tagged_robot_task_serialization() {
        let move_task = RobotTask::Move { destination: (10.0, 20.0), speed: 3.0 };
        let yaml = serde_yaml::to_string(&move_task).unwrap();

        // Should use renamed tag
        assert!(yaml.contains("kind: movement"));
        assert!(yaml.contains("destination:"));
        assert!(yaml.contains("speed: 3"));
    }

    #[test]
    fn test_complex_mission_structure() {
        let mission = create_complex_robot_mission();

        let yaml = serde_yaml::to_string(&mission).unwrap();
        let deserialized: RobotMission = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(mission.id, deserialized.id);
        assert_eq!(mission.tasks.len(), deserialized.tasks.len());
        assert_eq!(mission.priority, deserialized.priority);
        assert_eq!(mission.environment_config.len(), deserialized.environment_config.len());
    }

    #[test]
    fn test_environment_config_complexity() {
        let mission = create_complex_robot_mission();

        // Test different EnvironmentValue types
        if let Some(EnvironmentValue::Text(unit)) = mission.environment_config.get("temperature_unit") {
            assert_eq!(unit, "celsius");
        } else {
            panic!("Expected text value");
        }

        if let Some(EnvironmentValue::Number(speed)) = mission.environment_config.get("max_speed") {
            assert_eq!(*speed, 5.0);
        } else {
            panic!("Expected number value");
        }

        if let Some(EnvironmentValue::Coordinates { x, y, z }) = mission.environment_config.get("home_position") {
            assert_eq!(*x, 0.0);
            assert_eq!(*y, 0.0);
            assert_eq!(*z, 0.0);
        } else {
            panic!("Expected coordinates value");
        }
    }

    #[test]
    fn test_sensor_type_rename_all() {
        let sensor_types = vec![
            SensorType::OpticalCamera,
            SensorType::LidarScanner,
            SensorType::ThermalImaging,
            SensorType::UltrasonicRange,
            SensorType::MotionDetector,
        ];

        for sensor_type in sensor_types {
            let yaml = serde_yaml::to_string(&sensor_type).unwrap();

            // Should use snake_case
            match sensor_type {
                SensorType::OpticalCamera => assert!(yaml.contains("optical_camera")),
                SensorType::LidarScanner => assert!(yaml.contains("lidar_scanner")),
                SensorType::ThermalImaging => assert!(yaml.contains("thermal_imaging")),
                SensorType::UltrasonicRange => assert!(yaml.contains("ultrasonic_range")),
                SensorType::MotionDetector => assert!(yaml.contains("motion_detector")),
            }
        }
    }

    #[test]
    fn test_sensor_settings_untagged() {
        let configs = create_sensor_configurations();

        for config in configs {
            let yaml = serde_yaml::to_string(&config).unwrap();
            let deserialized: SensorConfiguration = serde_yaml::from_str(&yaml).unwrap();

            // Verify the settings type matches
            match (&config.settings, &deserialized.settings) {
                (SensorSettings::Camera { .. }, SensorSettings::Camera { .. }) => {},
                (SensorSettings::Lidar { .. }, SensorSettings::Lidar { .. }) => {},
                (SensorSettings::Range { .. }, SensorSettings::Range { .. }) => {},
                _ => panic!("Settings type mismatch"),
            }
        }
    }

    #[test]
    fn test_camera_settings() {
        let camera_config = &create_sensor_configurations()[0];

        if let SensorSettings::Camera { resolution, fps, auto_focus } = &camera_config.settings {
            assert_eq!(resolution, "1920x1080");
            assert_eq!(*fps, 30);
            assert!(*auto_focus);
        } else {
            panic!("Expected Camera settings");
        }
    }

    #[test]
    fn test_lidar_settings() {
        let lidar_config = &create_sensor_configurations()[1];

        if let SensorSettings::Lidar { range_meters, scan_rate_hz, point_density } = &lidar_config.settings {
            assert_eq!(*range_meters, 100.0);
            assert_eq!(*scan_rate_hz, 10.0);
            assert_eq!(*point_density, 1000);
        } else {
            panic!("Expected Lidar settings");
        }
    }

    #[test]
    fn test_mission_priority_serialization() {
        let priorities = vec![
            MissionPriority::Low,
            MissionPriority::Medium,
            MissionPriority::High,
            MissionPriority::Critical,
        ];

        for priority in priorities {
            let yaml = serde_yaml::to_string(&priority).unwrap();
            let deserialized: MissionPriority = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(priority, deserialized);
        }
    }

    #[test]
    fn test_zone_bounds_structure() {
        let map = process_navigation_map_yaml().unwrap();

        let entry_zone = &map.safe_zones[0];
        assert_eq!(entry_zone.name, "Entry Zone");
        assert_eq!(entry_zone.bounds.min, (0.0, 0.0));
        assert_eq!(entry_zone.bounds.max, (5.0, 5.0));

        let staging_area = &map.safe_zones[1];
        assert_eq!(staging_area.name, "Staging Area");
        assert_eq!(staging_area.bounds.min, (40.0, 25.0));
        assert_eq!(staging_area.bounds.max, (50.0, 30.0));
    }

    #[test]
    fn test_complex_round_trip() {
        let original_mission = create_complex_robot_mission();

        // Serialize to YAML
        let yaml = serde_yaml::to_string(&original_mission).unwrap();

        // Deserialize back
        let deserialized: RobotMission = serde_yaml::from_str(&yaml).unwrap();

        // Should be equal
        assert_eq!(original_mission.id, deserialized.id);
        assert_eq!(original_mission.name, deserialized.name);
        assert_eq!(original_mission.tasks.len(), deserialized.tasks.len());
        assert_eq!(original_mission.priority, deserialized.priority);
    }

    #[test]
    fn test_polymorphic_obstacle_serialization() {
        let obstacles = vec![
            Obstacle::Rectangle { x: 1.0, y: 2.0, width: 3.0, height: 4.0 },
            Obstacle::Circle { center: (5.0, 6.0), radius: 7.0 },
            Obstacle::Polygon { vertices: vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)] },
        ];

        for obstacle in obstacles {
            let yaml = serde_yaml::to_string(&obstacle).unwrap();
            let deserialized: Obstacle = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(obstacle, deserialized);
        }
    }

    #[test]
    fn test_optional_task_fields() {
        let transmit_without_target = RobotTask::Transmit {
            message: "Broadcast message".to_string(),
            target: None,
        };

        let yaml = serde_yaml::to_string(&transmit_without_target).unwrap();
        let deserialized: RobotTask = serde_yaml::from_str(&yaml).unwrap();

        if let RobotTask::Transmit { message, target } = deserialized {
            assert_eq!(message, "Broadcast message");
            assert_eq!(target, None);
        } else {
            panic!("Expected Transmit task");
        }
    }

    #[test]
    fn test_mixed_environment_values() {
        let mut env_map = std::collections::HashMap::new();
        env_map.insert("string_val".to_string(), EnvironmentValue::Text("test".to_string()));
        env_map.insert("number_val".to_string(), EnvironmentValue::Number(123.45));
        env_map.insert("bool_val".to_string(), EnvironmentValue::Boolean(false));
        env_map.insert("list_val".to_string(), EnvironmentValue::List(vec!["a".to_string(), "b".to_string()]));
        env_map.insert("coord_val".to_string(), EnvironmentValue::Coordinates { x: 1.0, y: 2.0, z: 3.0 });

        let yaml = serde_yaml::to_string(&env_map).unwrap();
        let deserialized: std::collections::HashMap<String, EnvironmentValue> =
            serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(env_map.len(), deserialized.len());

        for (key, value) in env_map {
            assert_eq!(value, deserialized[&key]);
        }
    }
}

// Example usage and reference implementation
fn main() {
    println!("Level 15 Task 4: Handle Enums and Complex Data Types");
    println!("Run with: cargo test level15_task4");
}