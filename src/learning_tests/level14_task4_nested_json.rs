#[cfg(test)]
mod level14_task4_nested_json_tests {
    use super::*;
    use serde::{Serialize, Deserialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct Mission {
        id: String,
        name: String,
        objectives: Vec<Objective>,
        estimated_duration: u32,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct Objective {
        description: String,
        priority: String,
        completed: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct RobotConfig {
        id: u32,
        name: String,
        max_speed: f64,
        sensors_enabled: bool,
        position: (i32, i32),
    }

    impl RobotConfig {
        fn new(id: u32, name: String, max_speed: f64) -> Self {
            RobotConfig {
                id,
                name,
                max_speed,
                sensors_enabled: true,
                position: (0, 0),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct ComplexRobotData {
        robots: Vec<RobotConfig>,
        missions: HashMap<String, Mission>,
        global_settings: GlobalSettings,
        metadata: Metadata,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct GlobalSettings {
        max_concurrent_missions: u32,
        default_timeout: u32,
        logging_enabled: bool,
        backup_frequency: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Metadata {
        version: String,
        created_by: String,
        created_at: String,
        tags: Vec<String>,
    }

    fn process_mission_params_item() -> Result<Mission, serde_json::Error> {
        let mission_json = r#"
            {
                "id": "MISSION_001",
                "name": "Data Collection Protocol",
                "objectives": [
                    {
                        "description": "Collect all JSON data items",
                        "priority": "high",
                        "completed": false
                    },
                    {
                        "description": "Process sensor readings",
                        "priority": "medium",
                        "completed": false
                    },
                    {
                        "description": "Generate status report",
                        "priority": "low",
                        "completed": false
                    }
                ],
                "estimated_duration": 300
            }
        "#;

        let mission: Mission = serde_json::from_str(mission_json)?;

        println!("Mission: {}", mission.name);
        for (i, objective) in mission.objectives.iter().enumerate() {
            println!("  Objective {}: {} ({})",
                     i + 1, objective.description, objective.priority);
        }

        Ok(mission)
    }

    fn process_complex_nested_data() -> Result<ComplexRobotData, serde_json::Error> {
        let complex_json = r#"
            {
                "robots": [
                    {
                        "id": 1,
                        "name": "Scout Alpha",
                        "max_speed": 2.5,
                        "sensors_enabled": true,
                        "position": [0, 0]
                    },
                    {
                        "id": 2,
                        "name": "Heavy Beta",
                        "max_speed": 1.8,
                        "sensors_enabled": true,
                        "position": [5, 3]
                    }
                ],
                "missions": {
                    "exploration": {
                        "id": "EXPLORE_001",
                        "name": "Area Exploration",
                        "objectives": [
                            {
                                "description": "Map unknown territory",
                                "priority": "high",
                                "completed": false
                            }
                        ],
                        "estimated_duration": 600
                    },
                    "rescue": {
                        "id": "RESCUE_001",
                        "name": "Emergency Rescue",
                        "objectives": [
                            {
                                "description": "Locate survivors",
                                "priority": "critical",
                                "completed": false
                            },
                            {
                                "description": "Establish communication",
                                "priority": "high",
                                "completed": false
                            }
                        ],
                        "estimated_duration": 1200
                    }
                },
                "global_settings": {
                    "max_concurrent_missions": 3,
                    "default_timeout": 30,
                    "logging_enabled": true,
                    "backup_frequency": "hourly"
                },
                "metadata": {
                    "version": "2.1.0",
                    "created_by": "System Administrator",
                    "created_at": "2024-01-15T10:30:00Z",
                    "tags": ["production", "autonomous", "multi-robot"]
                }
            }
        "#;

        let data: ComplexRobotData = serde_json::from_str(complex_json)?;

        println!("Loaded {} robots", data.robots.len());
        println!("Loaded {} missions", data.missions.len());
        println!("Version: {}", data.metadata.version);

        Ok(data)
    }

    fn process_dynamic_json() -> Result<(), serde_json::Error> {
        let dynamic_json = r#"
            {
                "sensor_readings": {
                    "temperature": 23.5,
                    "humidity": 45.2,
                    "pressure": 1013.25,
                    "air_quality": "good"
                },
                "diagnostics": {
                    "battery_level": 87,
                    "motor_status": "operational",
                    "memory_usage": 0.45,
                    "cpu_temperature": 42.3
                },
                "navigation": {
                    "current_position": [3, 7],
                    "destination": [9, 9],
                    "path_length": 12,
                    "obstacles_detected": ["wall", "debris", "enemy"]
                }
            }
        "#;

        let data: serde_json::Value = serde_json::from_str(dynamic_json)?;

        // Access nested values dynamically
        if let Some(sensor_data) = data.get("sensor_readings") {
            println!("Temperature: {}°C", sensor_data["temperature"]);
            println!("Humidity: {}%", sensor_data["humidity"]);
            println!("Air Quality: {}", sensor_data["air_quality"]);
        }

        if let Some(diagnostics) = data.get("diagnostics") {
            println!("Battery: {}%", diagnostics["battery_level"]);
            println!("CPU Temp: {}°C", diagnostics["cpu_temperature"]);
        }

        if let Some(nav_data) = data.get("navigation") {
            if let Some(obstacles) = nav_data.get("obstacles_detected") {
                if let Some(obstacles_array) = obstacles.as_array() {
                    println!("Obstacles: {:?}", obstacles_array);
                }
            }
        }

        Ok(())
    }

    fn create_nested_mission_structure() -> Result<String, serde_json::Error> {
        let objectives = vec![
            Objective {
                description: "Collect all JSON data items".to_string(),
                priority: "high".to_string(),
                completed: false,
            },
            Objective {
                description: "Process sensor readings".to_string(),
                priority: "medium".to_string(),
                completed: true,
            },
            Objective {
                description: "Generate status report".to_string(),
                priority: "low".to_string(),
                completed: false,
            },
        ];

        let mission = Mission {
            id: "MISSION_NESTED_001".to_string(),
            name: "Nested Data Processing".to_string(),
            objectives,
            estimated_duration: 450,
        };

        let json_string = serde_json::to_string_pretty(&mission)?;
        println!("Created mission JSON:\n{}", json_string);
        Ok(json_string)
    }

    #[test]
    fn test_mission_deserialization() {
        let result = process_mission_params_item();
        assert!(result.is_ok());

        let mission = result.unwrap();
        assert_eq!(mission.id, "MISSION_001");
        assert_eq!(mission.name, "Data Collection Protocol");
        assert_eq!(mission.objectives.len(), 3);
        assert_eq!(mission.estimated_duration, 300);

        assert_eq!(mission.objectives[0].description, "Collect all JSON data items");
        assert_eq!(mission.objectives[0].priority, "high");
        assert!(!mission.objectives[0].completed);
    }

    #[test]
    fn test_complex_nested_data() {
        let result = process_complex_nested_data();
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.robots.len(), 2);
        assert_eq!(data.missions.len(), 2);

        assert_eq!(data.robots[0].name, "Scout Alpha");
        assert_eq!(data.robots[1].name, "Heavy Beta");

        assert!(data.missions.contains_key("exploration"));
        assert!(data.missions.contains_key("rescue"));

        assert_eq!(data.global_settings.max_concurrent_missions, 3);
        assert!(data.global_settings.logging_enabled);

        assert_eq!(data.metadata.version, "2.1.0");
        assert_eq!(data.metadata.tags.len(), 3);
    }

    #[test]
    fn test_dynamic_json_access() {
        let result = process_dynamic_json();
        assert!(result.is_ok());
    }

    #[test]
    fn test_objective_array_operations() {
        let objectives_json = r#"
            [
                {
                    "description": "Scan area A",
                    "priority": "high",
                    "completed": true
                },
                {
                    "description": "Scan area B",
                    "priority": "medium",
                    "completed": false
                },
                {
                    "description": "Scan area C",
                    "priority": "low",
                    "completed": false
                }
            ]
        "#;

        let objectives: Vec<Objective> = serde_json::from_str(objectives_json).unwrap();
        assert_eq!(objectives.len(), 3);

        let completed_count = objectives.iter().filter(|obj| obj.completed).count();
        assert_eq!(completed_count, 1);

        let high_priority_count = objectives.iter()
            .filter(|obj| obj.priority == "high")
            .count();
        assert_eq!(high_priority_count, 1);
    }

    #[test]
    fn test_nested_mission_creation() {
        let result = create_nested_mission_structure();
        assert!(result.is_ok());

        let json_string = result.unwrap();
        assert!(json_string.contains("MISSION_NESTED_001"));
        assert!(json_string.contains("Nested Data Processing"));

        // Test round-trip serialization
        let mission: Mission = serde_json::from_str(&json_string).unwrap();
        assert_eq!(mission.id, "MISSION_NESTED_001");
        assert_eq!(mission.objectives.len(), 3);
        assert!(mission.objectives[1].completed); // Second objective should be completed
    }

    #[test]
    fn test_mission_objective_filtering() {
        let mission = process_mission_params_item().unwrap();

        let high_priority_objectives: Vec<&Objective> = mission.objectives
            .iter()
            .filter(|obj| obj.priority == "high")
            .collect();

        assert_eq!(high_priority_objectives.len(), 1);
        assert_eq!(high_priority_objectives[0].description, "Collect all JSON data items");
    }

    #[test]
    fn test_mission_completion_status() {
        let mut mission = process_mission_params_item().unwrap();

        // Initially no objectives completed
        let completed_count = mission.objectives.iter().filter(|obj| obj.completed).count();
        assert_eq!(completed_count, 0);

        // Mark first objective as completed
        mission.objectives[0].completed = true;

        let completed_count = mission.objectives.iter().filter(|obj| obj.completed).count();
        assert_eq!(completed_count, 1);

        // Test serialization with updated state
        let json = serde_json::to_string(&mission).unwrap();
        let deserialized: Mission = serde_json::from_str(&json).unwrap();
        assert!(deserialized.objectives[0].completed);
    }

    #[test]
    fn test_hashmap_missions() {
        let data = process_complex_nested_data().unwrap();

        assert!(data.missions.contains_key("exploration"));
        assert!(data.missions.contains_key("rescue"));

        let exploration = &data.missions["exploration"];
        assert_eq!(exploration.name, "Area Exploration");
        assert_eq!(exploration.objectives.len(), 1);

        let rescue = &data.missions["rescue"];
        assert_eq!(rescue.name, "Emergency Rescue");
        assert_eq!(rescue.objectives.len(), 2);
        assert_eq!(rescue.objectives[0].priority, "critical");
    }

    #[test]
    fn test_metadata_tags() {
        let data = process_complex_nested_data().unwrap();

        assert_eq!(data.metadata.tags.len(), 3);
        assert!(data.metadata.tags.contains(&"production".to_string()));
        assert!(data.metadata.tags.contains(&"autonomous".to_string()));
        assert!(data.metadata.tags.contains(&"multi-robot".to_string()));
    }

    #[test]
    fn test_robot_array_access() {
        let data = process_complex_nested_data().unwrap();

        let scout = &data.robots[0];
        assert_eq!(scout.id, 1);
        assert_eq!(scout.name, "Scout Alpha");
        assert_eq!(scout.max_speed, 2.5);
        assert_eq!(scout.position, (0, 0));

        let heavy = &data.robots[1];
        assert_eq!(heavy.id, 2);
        assert_eq!(heavy.name, "Heavy Beta");
        assert_eq!(heavy.max_speed, 1.8);
        assert_eq!(heavy.position, (5, 3));
    }

    #[test]
    fn test_global_settings() {
        let data = process_complex_nested_data().unwrap();

        assert_eq!(data.global_settings.max_concurrent_missions, 3);
        assert_eq!(data.global_settings.default_timeout, 30);
        assert!(data.global_settings.logging_enabled);
        assert_eq!(data.global_settings.backup_frequency, "hourly");
    }

    #[test]
    fn test_json_value_manipulation() {
        let json_str = r#"{"temperature": 25.5, "status": "active", "count": 42}"#;
        let mut value: serde_json::Value = serde_json::from_str(json_str).unwrap();

        // Read values
        assert_eq!(value["temperature"], 25.5);
        assert_eq!(value["status"], "active");
        assert_eq!(value["count"], 42);

        // Modify values
        value["temperature"] = serde_json::Value::Number(serde_json::Number::from_f64(30.0).unwrap());
        value["status"] = serde_json::Value::String("modified".to_string());

        assert_eq!(value["temperature"], 30.0);
        assert_eq!(value["status"], "modified");
    }

    #[test]
    fn test_array_of_objects_serialization() {
        let objectives = vec![
            Objective {
                description: "First task".to_string(),
                priority: "high".to_string(),
                completed: false,
            },
            Objective {
                description: "Second task".to_string(),
                priority: "medium".to_string(),
                completed: true,
            },
        ];

        let json = serde_json::to_string(&objectives).unwrap();
        assert!(json.contains("First task"));
        assert!(json.contains("Second task"));

        let deserialized: Vec<Objective> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.len(), 2);
        assert_eq!(deserialized[0].description, "First task");
        assert!(!deserialized[0].completed);
        assert!(deserialized[1].completed);
    }

    #[test]
    fn test_optional_fields() {
        #[derive(Serialize, Deserialize, Debug)]
        struct OptionalConfig {
            id: u32,
            name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(default = "default_timeout")]
            timeout: u32,
        }

        fn default_timeout() -> u32 { 30 }

        let json_with_description = r#"{"id": 1, "name": "Test", "description": "A test config"}"#;
        let config1: OptionalConfig = serde_json::from_str(json_with_description).unwrap();
        assert_eq!(config1.description, Some("A test config".to_string()));
        assert_eq!(config1.timeout, 30); // Default value

        let json_without_description = r#"{"id": 2, "name": "Test2", "timeout": 60}"#;
        let config2: OptionalConfig = serde_json::from_str(json_without_description).unwrap();
        assert_eq!(config2.description, None);
        assert_eq!(config2.timeout, 60);
    }
}

// Example usage and reference implementation
fn main() {
    println!("Level 14 Task 4: Handle JSON Arrays and Nested Objects");
    println!("Run with: cargo test level14_task4");
}