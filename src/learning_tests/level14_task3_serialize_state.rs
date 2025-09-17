// Level 14 Task 3 Test: Serialize Robot State to JSON
// Tests that user can serialize structs to JSON format

#[cfg(test)]
mod level14_task3_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_robot_state_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct RobotState"),
            "❌ You need to define a RobotState struct"
        );
    }

    #[test]
    fn test_robot_state_has_required_fields() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_config = analyzer.code.contains("config:");
        let has_position = analyzer.code.contains("current_position:");
        let has_energy = analyzer.code.contains("energy:");
        let has_items = analyzer.code.contains("items_collected:");
        let has_doors = analyzer.code.contains("doors_opened:");
        let has_mission = analyzer.code.contains("mission_completed:");

        assert!(has_config, "❌ RobotState should have a 'config' field");
        assert!(has_position, "❌ RobotState should have a 'current_position' field");
        assert!(has_energy, "❌ RobotState should have an 'energy' field");
        assert!(has_items, "❌ RobotState should have an 'items_collected' field");
        assert!(has_doors, "❌ RobotState should have a 'doors_opened' field");
        assert!(has_mission, "❌ RobotState should have a 'mission_completed' field");
    }

    #[test]
    fn test_has_serialize_robot_state_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("serialize_robot_state") ||
            analyzer.code.contains("fn serialize_robot_state"),
            "❌ You need a function to serialize robot state"
        );
    }

    #[test]
    fn test_uses_serde_json_to_string() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_to_string = analyzer.code.contains("serde_json::to_string") ||
                           analyzer.code.contains("to_string_pretty");
        assert!(
            has_to_string,
            "❌ You should use serde_json::to_string or to_string_pretty for serialization"
        );
    }

    #[test]
    fn test_creates_robot_state_instance() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_state_creation = analyzer.code.contains("RobotState {") ||
                               analyzer.code.contains("let state =");
        assert!(
            has_state_creation,
            "❌ You should create a RobotState instance to serialize"
        );
    }

    #[test]
    fn test_includes_vector_field() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_vec = analyzer.code.contains("Vec<") ||
                     analyzer.code.contains("vec!");
        assert!(
            has_vec,
            "❌ Your RobotState should include Vec fields like items_collected"
        );
    }

    #[test]
    fn test_returns_json_string() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let returns_string = analyzer.code.contains("Result<String,") ||
                           analyzer.code.contains("-> String");
        assert!(
            returns_string,
            "❌ Your serialize function should return a JSON string"
        );
    }

    #[test]
    fn test_pretty_prints_json() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_pretty = analyzer.code.contains("to_string_pretty");
        assert!(
            has_pretty,
            "❌ You should use to_string_pretty for nicely formatted JSON output"
        );
    }

    #[test]
    fn test_robot_state_serde_derives() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let struct_and_derives = analyzer.code.lines()
            .skip_while(|line| !line.contains("struct RobotState"))
            .take(5)
            .collect::<Vec<_>>()
            .join("\n");

        let has_serialize = struct_and_derives.contains("Serialize");
        let has_deserialize = struct_and_derives.contains("Deserialize");

        assert!(has_serialize, "❌ RobotState should derive Serialize");
        assert!(has_deserialize, "❌ RobotState should derive Deserialize");
    }

    #[test]
    fn test_prints_serialized_json() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ You should print the serialized JSON to demonstrate it worked"
        );
    }

    #[test]
    fn test_handles_nested_structs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_nested = analyzer.code.contains("config: RobotConfig") ||
                        analyzer.code.contains("config,");
        assert!(
            has_nested,
            "❌ Your RobotState should include a nested RobotConfig struct"
        );
    }
}

// Reference implementation
fn main() {
    println!("Level 14 Task 3: Serialize State");
    // Reference pattern for JSON serialization
}

// Reference serialization pattern
// #[derive(Serialize, Deserialize, Debug)]
// struct RobotState {
//     config: RobotConfig,
//     current_position: (i32, i32),
//     energy: u32,
//     items_collected: Vec<String>,
//     doors_opened: u32,
//     mission_completed: bool,
// }
//
// fn serialize_robot_state() -> Result<String, serde_json::Error> {
//     let config = RobotConfig::new(1, "Data Collector".to_string(), 3.0);
//     let state = RobotState { /* ... */ };
//     let json_string = serde_json::to_string_pretty(&state)?;
//     println!("Robot state JSON:\n{}", json_string);
//     Ok(json_string)
// }