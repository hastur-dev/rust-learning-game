// Level 14 Task 2 Test: Deserialize JSON Data from Items
// Tests that user can deserialize JSON strings into structs

#[cfg(test)]
mod level14_task2_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_serde_json_import() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_serde_json = analyzer.code.contains("serde_json");
        assert!(
            has_serde_json,
            "❌ You need to use serde_json for JSON processing"
        );
    }

    #[test]
    fn test_has_process_robot_config_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("process_robot_config_item") ||
            analyzer.code.contains("fn process_robot_config"),
            "❌ You need a function to process robot config JSON data"
        );
    }

    #[test]
    fn test_uses_serde_json_from_str() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("serde_json::from_str"),
            "❌ You should use serde_json::from_str to deserialize JSON strings"
        );
    }

    #[test]
    fn test_has_json_string_literal() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_json = analyzer.code.contains(r#"r#""#) ||
                      (analyzer.code.contains('"') && analyzer.code.contains("id") && analyzer.code.contains("name"));
        assert!(
            has_json,
            "❌ You should include JSON string literals to deserialize"
        );
    }

    #[test]
    fn test_returns_result_type() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_result = analyzer.code.contains("Result<") &&
                        analyzer.code.contains("serde_json::Error");
        assert!(
            has_result,
            "❌ Your function should return Result<RobotConfig, serde_json::Error>"
        );
    }

    #[test]
    fn test_has_sensor_data_processing() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("process_sensor_data") ||
            analyzer.code.contains("sensor_data"),
            "❌ You should also process sensor data JSON"
        );
    }

    #[test]
    fn test_uses_serde_json_value() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("serde_json::Value"),
            "❌ You should use serde_json::Value for dynamic JSON processing"
        );
    }

    #[test]
    fn test_accesses_json_fields() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_field_access = analyzer.code.contains('[') && analyzer.code.contains(']') &&
                              (analyzer.code.contains("temperature") || analyzer.code.contains("battery_level"));
        assert!(
            has_field_access,
            "❌ You should access JSON fields using bracket notation like sensor_data[\"temperature\"]"
        );
    }

    #[test]
    fn test_handles_json_arrays() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_array = analyzer.code.contains("obstacles_detected") ||
                       analyzer.code.contains("Vec<") ||
                       analyzer.code.contains("array");
        assert!(
            has_array,
            "❌ You should handle JSON arrays like \"obstacles_detected\""
        );
    }

    #[test]
    fn test_has_error_handling() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_error_handling = analyzer.code.contains('?') ||
                               analyzer.code.contains("unwrap") ||
                               analyzer.code.contains("expect");
        assert!(
            has_error_handling,
            "❌ You should handle potential JSON parsing errors"
        );
    }

    #[test]
    fn test_prints_deserialized_data() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ You should print the deserialized data to demonstrate it worked"
        );
    }
}

// Reference implementation
fn main() {
    println!("Level 14 Task 2: Deserialize JSON");
    // Reference pattern for JSON deserialization
}

// Reference deserialization pattern
// fn process_robot_config_item() -> Result<RobotConfig, serde_json::Error> {
//     let json_data = r#"
//         {
//             "id": 12345,
//             "name": "Explorer Bot",
//             "max_speed": 2.5,
//             "sensors_enabled": true,
//             "position": [3, 4]
//         }
//     "#;
//
//     let config: RobotConfig = serde_json::from_str(json_data)?;
//     println!("Loaded robot config: {:?}", config);
//     Ok(config)
// }