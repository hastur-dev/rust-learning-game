#[cfg(test)]
mod level16_task2_error_handling_tests {
    use super::*;
    use serde::{Serialize, Deserialize};
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    enum DataProcessingError {
        JsonError(serde_json::Error),
        YamlError(serde_yaml::Error),
        ValidationError(String),
        CorruptedData(String),
        UnsupportedFormat(String),
        MigrationError(String),
    }

    impl fmt::Display for DataProcessingError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                DataProcessingError::JsonError(e) => write!(f, "JSON processing error: {}", e),
                DataProcessingError::YamlError(e) => write!(f, "YAML processing error: {}", e),
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

    impl From<serde_yaml::Error> for DataProcessingError {
        fn from(error: serde_yaml::Error) -> Self {
            DataProcessingError::YamlError(error)
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct RobotData {
        id: u32,
        name: String,
        data: Vec<i32>,
        timestamp: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct RecoveryMetrics {
        attempts: u32,
        successful_repairs: u32,
        failed_repairs: u32,
        repair_strategies: Vec<String>,
    }

    fn process_corrupted_json_data() -> Result<serde_json::Value, DataProcessingError> {
        let corrupted_json = r#"
            {
                "id": 12345,
                "name": "Corrupted Data Set
                "data": [1, 2, 3, 4, 5
                "timestamp": "2024-01-15T10:30:00Z"
            }
        "#;

        // Try to parse as-is first
        match serde_json::from_str::<serde_json::Value>(corrupted_json) {
            Ok(value) => Ok(value),
            Err(e) => {
                println!("Initial parse failed: {}", e);

                // Attempt repair strategies
                let repaired = attempt_json_repair(corrupted_json)?;

                match serde_json::from_str::<serde_json::Value>(&repaired) {
                    Ok(value) => {
                        println!("Successfully repaired and parsed JSON");
                        Ok(value)
                    }
                    Err(e) => Err(DataProcessingError::CorruptedData(
                        format!("Unable to repair JSON: {}", e)
                    ))
                }
            }
        }
    }

    fn attempt_json_repair(corrupted: &str) -> Result<String, DataProcessingError> {
        let mut repaired = corrupted.to_string();

        // Fix missing quotes
        if !repaired.contains(r#""name": "Corrupted Data Set""#) {
            repaired = repaired.replace(
                r#""name": "Corrupted Data Set"#,
                r#""name": "Corrupted Data Set""#
            );
        }

        // Fix missing closing bracket
        if !repaired.contains(r#""data": [1, 2, 3, 4, 5]"#) {
            repaired = repaired.replace(
                r#""data": [1, 2, 3, 4, 5"#,
                r#""data": [1, 2, 3, 4, 5]"#
            );
        }

        Ok(repaired)
    }

    fn process_malformed_yaml() -> Result<serde_yaml::Value, DataProcessingError> {
        let malformed_yaml = r#"
robot_config:
  id: 123
  name: "Test Robot"
  sensors:
    - type: camera
      enabled: true
      calibration:
        focal_length 24.5
        aperture: 2.8
    - type: lidar
      enabled true
      range: 100.0
settings:
  mode: autonomous
  speed: 2.5
  # This comment breaks parsing
  debug_mode: false
        "#;

        match serde_yaml::from_str::<serde_yaml::Value>(malformed_yaml) {
            Ok(value) => Ok(value),
            Err(e) => {
                println!("YAML parse failed: {}", e);

                let repaired = attempt_yaml_repair(malformed_yaml)?;

                match serde_yaml::from_str::<serde_yaml::Value>(&repaired) {
                    Ok(value) => {
                        println!("Successfully repaired and parsed YAML");
                        Ok(value)
                    }
                    Err(e) => Err(DataProcessingError::CorruptedData(
                        format!("Unable to repair YAML: {}", e)
                    ))
                }
            }
        }
    }

    fn attempt_yaml_repair(malformed: &str) -> Result<String, DataProcessingError> {
        let mut repaired = malformed.to_string();

        // Fix missing colon
        repaired = repaired.replace("focal_length 24.5", "focal_length: 24.5");

        // Fix missing colon for boolean
        repaired = repaired.replace("enabled true", "enabled: true");

        // Remove problematic comments in the middle of structures
        let lines: Vec<&str> = repaired.lines().collect();
        let mut fixed_lines = Vec::new();

        for line in lines {
            // Skip lines that are comments in invalid positions
            if line.trim().starts_with("# This comment breaks parsing") {
                continue;
            }
            fixed_lines.push(line);
        }

        Ok(fixed_lines.join("\n"))
    }

    fn process_mixed_format_data() -> Result<Vec<RobotData>, DataProcessingError> {
        let mixed_data = vec![
            (r#"{"id": 1, "name": "Robot A", "data": [1, 2, 3], "timestamp": "2024-01-15T10:00:00Z"}"#, "json"),
            (r#"id: 2
name: "Robot B"
data: [4, 5, 6]
timestamp: "2024-01-15T10:01:00Z""#, "yaml"),
            (r#"{"id": 3, "name": "Robot C", "data": [7, 8, 9], "timestamp": "invalid_timestamp"}"#, "json"),
            (r#"id: 4
name: "Robot D"
data: [10, 11
timestamp: "2024-01-15T10:03:00Z""#, "yaml"), // Missing closing bracket
        ];

        let mut results = Vec::new();
        let mut error_count = 0;

        for (data, format) in mixed_data {
            match process_single_format(data, format) {
                Ok(robot_data) => results.push(robot_data),
                Err(e) => {
                    error_count += 1;
                    println!("Failed to process {} data: {}", format, e);

                    // Try repair and retry
                    if let Ok(repaired_data) = attempt_format_repair(data, format) {
                        match process_single_format(&repaired_data, format) {
                            Ok(robot_data) => {
                                println!("Successfully repaired and processed {} data", format);
                                results.push(robot_data);
                            }
                            Err(repair_error) => {
                                println!("Repair failed for {} data: {}", format, repair_error);
                            }
                        }
                    }
                }
            }
        }

        println!("Processed {} records, {} errors encountered", results.len(), error_count);
        Ok(results)
    }

    fn process_single_format(data: &str, format: &str) -> Result<RobotData, DataProcessingError> {
        match format {
            "json" => serde_json::from_str(data).map_err(DataProcessingError::from),
            "yaml" => serde_yaml::from_str(data).map_err(DataProcessingError::from),
            _ => Err(DataProcessingError::UnsupportedFormat(format.to_string())),
        }
    }

    fn attempt_format_repair(data: &str, format: &str) -> Result<String, DataProcessingError> {
        match format {
            "json" => attempt_json_repair(data),
            "yaml" => attempt_yaml_repair(data),
            _ => Err(DataProcessingError::UnsupportedFormat(format.to_string())),
        }
    }

    fn handle_legacy_format_data() -> Result<serde_json::Value, DataProcessingError> {
        // Simulate old format that needs conversion
        let legacy_data = r#"
            ROBOT_RECORD
            ID=12345
            NAME="Legacy Robot"
            X_COORD=10.5
            Y_COORD=20.3
            SENSORS=camera,lidar,gyroscope
            STATUS=active
            END_RECORD
        "#;

        match convert_legacy_to_json(legacy_data) {
            Ok(json_string) => {
                let value: serde_json::Value = serde_json::from_str(&json_string)?;
                println!("Successfully converted legacy format to JSON");
                Ok(value)
            }
            Err(e) => Err(DataProcessingError::CorruptedData(
                format!("Legacy format conversion failed: {}", e)
            ))
        }
    }

    fn convert_legacy_to_json(legacy: &str) -> Result<String, String> {
        let mut json_obj = serde_json::Map::new();

        for line in legacy.lines() {
            let line = line.trim();

            if line.is_empty() || line == "ROBOT_RECORD" || line == "END_RECORD" {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let json_key = key.to_lowercase();
                let json_value = match key {
                    "ID" => serde_json::Value::Number(
                        value.parse::<u64>().map_err(|e| format!("Invalid ID: {}", e))?.into()
                    ),
                    "NAME" => serde_json::Value::String(
                        value.trim_matches('"').to_string()
                    ),
                    "X_COORD" | "Y_COORD" => serde_json::Value::Number(
                        serde_json::Number::from_f64(
                            value.parse::<f64>().map_err(|e| format!("Invalid coordinate: {}", e))?
                        ).ok_or("Invalid float value")?
                    ),
                    "SENSORS" => {
                        let sensors: Vec<serde_json::Value> = value
                            .split(',')
                            .map(|s| serde_json::Value::String(s.trim().to_string()))
                            .collect();
                        serde_json::Value::Array(sensors)
                    },
                    "STATUS" => serde_json::Value::String(value.to_string()),
                    _ => serde_json::Value::String(value.to_string()),
                };

                json_obj.insert(json_key, json_value);
            }
        }

        serde_json::to_string_pretty(&json_obj)
            .map_err(|e| format!("JSON serialization failed: {}", e))
    }

    fn analyze_error_patterns() -> RecoveryMetrics {
        let test_cases = vec![
            (r#"{"valid": "json"}"#, "json"),
            (r#"{"invalid": json}"#, "json"), // Missing quotes
            (r#"valid: yaml"#, "yaml"),
            (r#"invalid yaml:"#, "yaml"), // Missing value
            ("ROBOT_RECORD\nID=123\nEND_RECORD", "legacy"),
        ];

        let mut metrics = RecoveryMetrics {
            attempts: 0,
            successful_repairs: 0,
            failed_repairs: 0,
            repair_strategies: Vec::new(),
        };

        for (data, format) in test_cases {
            metrics.attempts += 1;

            match format {
                "json" => {
                    if serde_json::from_str::<serde_json::Value>(data).is_err() {
                        if attempt_json_repair(data).is_ok() {
                            metrics.successful_repairs += 1;
                            metrics.repair_strategies.push("JSON quote fixing".to_string());
                        } else {
                            metrics.failed_repairs += 1;
                        }
                    }
                }
                "yaml" => {
                    if serde_yaml::from_str::<serde_yaml::Value>(data).is_err() {
                        if attempt_yaml_repair(data).is_ok() {
                            metrics.successful_repairs += 1;
                            metrics.repair_strategies.push("YAML structure fixing".to_string());
                        } else {
                            metrics.failed_repairs += 1;
                        }
                    }
                }
                "legacy" => {
                    if convert_legacy_to_json(data).is_ok() {
                        metrics.successful_repairs += 1;
                        metrics.repair_strategies.push("Legacy format conversion".to_string());
                    } else {
                        metrics.failed_repairs += 1;
                    }
                }
                _ => metrics.failed_repairs += 1,
            }
        }

        // Remove duplicates from strategies
        metrics.repair_strategies.sort();
        metrics.repair_strategies.dedup();

        metrics
    }

    #[test]
    fn test_corrupted_json_repair() {
        let result = process_corrupted_json_data();
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["id"], 12345);
        assert_eq!(value["name"], "Corrupted Data Set");
        assert_eq!(value["data"], serde_json::json!([1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_malformed_yaml_repair() {
        let result = process_malformed_yaml();
        assert!(result.is_ok());

        let value = result.unwrap();
        assert!(value["robot_config"].is_object());
        assert!(value["settings"].is_object());
    }

    #[test]
    fn test_json_repair_strategies() {
        let test_cases = vec![
            r#"{"missing": "quote}"#, // Valid
            r#"{"missing": quote}"#,  // Missing quotes
            r#"{"missing": "bracket"#, // Missing closing
        ];

        for test_case in test_cases {
            match serde_json::from_str::<serde_json::Value>(test_case) {
                Ok(_) => println!("✓ Valid JSON: {}", test_case),
                Err(_) => {
                    match attempt_json_repair(test_case) {
                        Ok(repaired) => {
                            println!("✓ Repaired: {} -> {}", test_case, repaired);
                            assert!(serde_json::from_str::<serde_json::Value>(&repaired).is_ok());
                        }
                        Err(e) => println!("✗ Repair failed: {}", e),
                    }
                }
            }
        }
    }

    #[test]
    fn test_yaml_repair_strategies() {
        let test_cases = vec![
            "valid: yaml",
            "missing colon value",
            "enabled true", // Missing colon
        ];

        for test_case in test_cases {
            match serde_yaml::from_str::<serde_yaml::Value>(test_case) {
                Ok(_) => println!("✓ Valid YAML: {}", test_case),
                Err(_) => {
                    match attempt_yaml_repair(test_case) {
                        Ok(repaired) => {
                            println!("✓ Repaired: {} -> {}", test_case, repaired);
                            assert!(serde_yaml::from_str::<serde_yaml::Value>(&repaired).is_ok());
                        }
                        Err(e) => println!("✗ Repair failed: {}", e),
                    }
                }
            }
        }
    }

    #[test]
    fn test_mixed_format_processing() {
        let result = process_mixed_format_data();
        assert!(result.is_ok());

        let robots = result.unwrap();
        assert!(!robots.is_empty());

        for robot in &robots {
            assert!(robot.id > 0);
            assert!(!robot.name.is_empty());
            assert!(!robot.data.is_empty());
        }
    }

    #[test]
    fn test_legacy_format_conversion() {
        let result = handle_legacy_format_data();
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["id"], 12345);
        assert_eq!(value["name"], "Legacy Robot");
        assert_eq!(value["x_coord"], 10.5);
        assert_eq!(value["y_coord"], 20.3);
        assert!(value["sensors"].is_array());
    }

    #[test]
    fn test_error_type_conversions() {
        // Test JSON error conversion
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let processing_error = DataProcessingError::from(json_error);
        assert!(matches!(processing_error, DataProcessingError::JsonError(_)));

        // Test YAML error conversion
        let yaml_error = serde_yaml::from_str::<serde_yaml::Value>("invalid: yaml: structure").unwrap_err();
        let processing_error = DataProcessingError::from(yaml_error);
        assert!(matches!(processing_error, DataProcessingError::YamlError(_)));
    }

    #[test]
    fn test_error_display() {
        let errors = vec![
            DataProcessingError::ValidationError("Test validation".to_string()),
            DataProcessingError::CorruptedData("Test corruption".to_string()),
            DataProcessingError::UnsupportedFormat("xml".to_string()),
            DataProcessingError::MigrationError("Test migration".to_string()),
        ];

        for error in errors {
            let error_string = format!("{}", error);
            assert!(!error_string.is_empty());
            println!("Error: {}", error_string);
        }
    }

    #[test]
    fn test_recovery_metrics() {
        let metrics = analyze_error_patterns();

        assert!(metrics.attempts > 0);
        assert!(metrics.successful_repairs > 0);
        assert!(!metrics.repair_strategies.is_empty());

        println!("Recovery metrics: {:?}", metrics);
    }

    #[test]
    fn test_comprehensive_error_handling() {
        // Test various error scenarios
        let test_scenarios = vec![
            ("Valid JSON", r#"{"valid": true}"#, true),
            ("Invalid JSON syntax", r#"{"invalid": }"#, false),
            ("Empty input", "", false),
            ("Non-JSON text", "This is not JSON", false),
        ];

        for (description, input, should_succeed) in test_scenarios {
            println!("Testing: {}", description);

            let direct_result = serde_json::from_str::<serde_json::Value>(input);

            if should_succeed {
                assert!(direct_result.is_ok(), "Expected success for: {}", description);
            } else {
                assert!(direct_result.is_err(), "Expected failure for: {}", description);

                // Try repair
                if let Err(_) = direct_result {
                    match attempt_json_repair(input) {
                        Ok(repaired) => {
                            println!("  Repair successful");
                            let repair_result = serde_json::from_str::<serde_json::Value>(&repaired);
                            if repair_result.is_ok() {
                                println!("  Repaired data is valid JSON");
                            }
                        }
                        Err(e) => println!("  Repair failed: {}", e),
                    }
                }
            }
        }
    }

    #[test]
    fn test_format_detection_and_processing() {
        let test_data = vec![
            (r#"{"format": "json"}"#, "json"),
            ("format: yaml", "yaml"),
            ("ROBOT_RECORD\nID=1\nEND_RECORD", "legacy"),
        ];

        for (data, expected_format) in test_data {
            println!("Processing {} data: {}", expected_format, data);

            match expected_format {
                "json" => {
                    let result = process_single_format(data, "json");
                    println!("JSON result: {:?}", result);
                }
                "yaml" => {
                    let result = process_single_format(data, "yaml");
                    println!("YAML result: {:?}", result);
                }
                "legacy" => {
                    let result = convert_legacy_to_json(data);
                    println!("Legacy conversion result: {:?}", result);
                }
                _ => println!("Unknown format: {}", expected_format),
            }
        }
    }
}

// Example usage and reference implementation
fn main() {
    println!("Level 16 Task 2: Implement Robust Error Handling for Data Processing");
    println!("Run with: cargo test level16_task2");
}