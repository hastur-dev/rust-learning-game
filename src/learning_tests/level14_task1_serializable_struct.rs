// Level 14 Task 1 Test: Define Serializable Robot Configuration Struct
// Tests that user creates structs with Serde derive macros

#[cfg(test)]
mod level14_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_serde_imports() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_serde = analyzer.code.contains("use serde::") ||
                       analyzer.code.contains("serde::{");
        assert!(
            has_serde,
            "❌ You need to import serde (use serde::{{Serialize, Deserialize}})"
        );
    }

    #[test]
    fn test_has_robot_config_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct RobotConfig"),
            "❌ You need to define a RobotConfig struct"
        );
    }

    #[test]
    fn test_has_serde_derives() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_derives = analyzer.code.contains("#[derive(") &&
                         analyzer.code.contains("Serialize") &&
                         analyzer.code.contains("Deserialize");
        assert!(
            has_derives,
            "❌ Your RobotConfig struct needs #[derive(Serialize, Deserialize)] attributes"
        );
    }

    #[test]
    fn test_has_required_fields() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_id = analyzer.code.contains("id:") && analyzer.code.contains("u32");
        let has_name = analyzer.code.contains("name:") && analyzer.code.contains("String");
        let has_speed = analyzer.code.contains("max_speed:") && analyzer.code.contains("f64");
        let has_sensors = analyzer.code.contains("sensors_enabled:") && analyzer.code.contains("bool");
        let has_position = analyzer.code.contains("position:");

        assert!(has_id, "❌ RobotConfig should have an 'id: u32' field");
        assert!(has_name, "❌ RobotConfig should have a 'name: String' field");
        assert!(has_speed, "❌ RobotConfig should have a 'max_speed: f64' field");
        assert!(has_sensors, "❌ RobotConfig should have a 'sensors_enabled: bool' field");
        assert!(has_position, "❌ RobotConfig should have a 'position' field");
    }

    #[test]
    fn test_has_constructor_impl() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_impl = analyzer.code.contains("impl RobotConfig") &&
                      analyzer.code.contains("fn new(");
        assert!(
            has_impl,
            "❌ RobotConfig should have an impl block with a 'new' constructor function"
        );
    }

    #[test]
    fn test_constructor_parameters() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let new_fn_section = analyzer.code.lines()
            .skip_while(|line| !line.contains("fn new("))
            .take_while(|line| !line.contains('}'))
            .collect::<Vec<_>>()
            .join("\n");

        let has_params = new_fn_section.contains("id: u32") &&
                        new_fn_section.contains("name: String") &&
                        new_fn_section.contains("max_speed: f64");

        assert!(
            has_params,
            "❌ The 'new' function should take parameters: id: u32, name: String, max_speed: f64"
        );
    }

    #[test]
    fn test_has_debug_derive() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_debug = analyzer.code.contains("Debug");
        assert!(
            has_debug,
            "❌ Your RobotConfig struct should also derive Debug for printing"
        );
    }

    #[test]
    fn test_struct_instantiation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_instantiation = analyzer.code.contains("RobotConfig::new(") ||
                              analyzer.code.contains("RobotConfig {");
        assert!(
            has_instantiation,
            "❌ You should demonstrate creating an instance of RobotConfig"
        );
    }

    #[test]
    fn test_proper_field_types() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        // Check for tuple type for position
        let has_tuple_position = analyzer.code.contains("(i32, i32)") ||
                               analyzer.code.contains("(f64, f64)");
        assert!(
            has_tuple_position,
            "❌ The position field should be a tuple type like (i32, i32) or (f64, f64)"
        );
    }
}

// Reference implementation
fn main() {
    println!("Level 14 Task 1: Serializable Struct");
    // Reference pattern for RobotConfig with Serde derives
}

// Reference struct pattern
// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct RobotConfig {
//     id: u32,
//     name: String,
//     max_speed: f64,
//     sensors_enabled: bool,
//     position: (i32, i32),
// }
//
// impl RobotConfig {
//     fn new(id: u32, name: String, max_speed: f64) -> Self {
//         RobotConfig {
//             id,
//             name,
//             max_speed,
//             sensors_enabled: true,
//             position: (0, 0),
//         }
//     }
// }