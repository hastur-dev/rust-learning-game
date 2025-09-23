// Level 9 Task 2 Test: HashMap for Key-Value Data Storage
// Tests if the user code uses HashMap<K, V> for robot data management

#[cfg(test)]
mod level9_task2_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_imports_hashmap() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("use std::collections::HashMap") ||
            analyzer.code.contains("HashMap"),
            "❌ Your code should import and use HashMap"
        );
    }

    #[test]
    fn test_creates_hashmap() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("HashMap::new()") ||
            analyzer.code.contains("HashMap<"),
            "❌ Your code should create HashMap collections"
        );
    }

    #[test]
    fn test_uses_insert_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".insert("),
            "❌ Your code should use .insert() to add key-value pairs"
        );
    }

    #[test]
    fn test_uses_get_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".get("),
            "❌ Your code should use .get() to retrieve values from HashMap"
        );
    }

    #[test]
    fn test_robot_position_tracking() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_robot_positions = analyzer.code.contains("robot_positions") ||
                                 analyzer.code.contains("position");
        assert!(
            has_robot_positions,
            "❌ Your code should track robot positions using HashMap"
        );
    }

    #[test]
    fn test_string_keys() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("String") && analyzer.code.contains("HashMap"),
            "❌ Your code should use String keys in HashMap"
        );
    }

    #[test]
    fn test_tuple_values() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_tuple_values = analyzer.code.contains("(i32, i32)") &&
                              analyzer.code.contains("HashMap");
        assert!(
            has_tuple_values,
            "❌ Your code should use position tuples as HashMap values"
        );
    }

    #[test]
    fn test_iterates_over_hashmap() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("for (") && analyzer.code.contains("in &"),
            "❌ Your code should iterate over HashMap key-value pairs"
        );
    }

    #[test]
    fn test_item_database() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_item_db = analyzer.code.contains("item_database") ||
                         analyzer.code.contains("item") && analyzer.code.contains("value");
        assert!(
            has_item_db,
            "❌ Your code should create an item database using HashMap"
        );
    }

    #[test]
    fn test_values_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".values()"),
            "❌ Your code should use .values() to access HashMap values"
        );
    }

    #[test]
    fn test_threat_levels() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_threat = analyzer.code.contains("threat") ||
                        analyzer.code.contains("enemy");
        assert!(
            has_threat,
            "❌ Your code should track threat levels using HashMap"
        );
    }

    #[test]
    fn test_sum_operation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".sum()"),
            "❌ Your code should use .sum() to calculate totals"
        );
    }

    #[test]
    fn test_max_operation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".max()"),
            "❌ Your code should use .max() to find maximum values"
        );
    }

    #[test]
    fn test_get_mut_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".get_mut("),
            "❌ Your code should use .get_mut() for mutable access"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_hashmap_output = result.stdout.contains("robot") ||
                                result.stdout.contains("position") ||
                                result.stdout.contains("item") ||
                                result.stdout.contains("threat") ||
                                result.stdout.contains("credits");

        assert!(
            has_hashmap_output,
            "❌ Your program should output information about HashMap operations"
        );
    }
}

// Reference implementation
fn main() {
    use std::collections::HashMap;

    println!("=== Robot Database with HashMap ===");

    // Create robot position tracking system
    let mut robot_positions: HashMap<String, (i32, i32)> = HashMap::new();

    // Add robot positions
    robot_positions.insert("MainRobot".to_string(), (0, 0));
    robot_positions.insert("ScoutBot".to_string(), (5, 2));
    robot_positions.insert("CollectorBot".to_string(), (10, 4));

    println!("Tracking {} robots", robot_positions.len());

    // Access robot positions
    if let Some(pos) = robot_positions.get("MainRobot") {
        println!("MainRobot is at: {:?}", pos);
    }

    // Update positions
    robot_positions.insert("MainRobot".to_string(), (3, 1));
    println!("MainRobot moved to: {:?}", robot_positions["MainRobot"]);

    // Create item database
    let mut item_database: HashMap<String, u32> = HashMap::new();
    item_database.insert("vector_core".to_string(), 100);
    item_database.insert("hashmap_engine".to_string(), 150);
    item_database.insert("iterator_module".to_string(), 75);
    item_database.insert("collection_analyzer".to_string(), 200);

    println!("\n=== Item Values ===");
    for (item_name, value) in &item_database {
        println!("{}: {} credits", item_name, value);
    }

    // Calculate total inventory value
    let total_value: u32 = item_database.values().sum();
    println!("Total item value: {} credits", total_value);

    // Enemy threat level mapping
    let mut threat_levels: HashMap<(i32, i32), u32> = HashMap::new();
    threat_levels.insert((5, 2), 3); // Guard
    threat_levels.insert((10, 4), 5); // Chaser
    threat_levels.insert((3, 7), 2); // Spiral
    threat_levels.insert((12, 8), 4); // Patrol
    threat_levels.insert((7, 6), 3); // Vertical

    println!("\n=== Threat Assessment ===");
    for ((x, y), threat) in &threat_levels {
        println!("Position ({}, {}): Threat level {}", x, y, threat);
    }

    // Find highest threat
    let max_threat = threat_levels.values().max().unwrap_or(&0);
    println!("Maximum threat level: {}", max_threat);

    // Door key requirements
    let mut door_keys: HashMap<(i32, i32), String> = HashMap::new();
    door_keys.insert((8, 1), "Red Key".to_string());
    door_keys.insert((4, 5), "Blue Key".to_string());
    door_keys.insert((11, 7), "Green Key".to_string());
    door_keys.insert((6, 9), "Master Key".to_string());

    // Check if we can open doors
    let available_keys = vec!["Red Key", "Blue Key"];
    let mut accessible_doors = 0;

    for ((x, y), required_key) in &door_keys {
        if available_keys.contains(&required_key.as_str()) {
            println!("Can open door at ({}, {}) with {}", x, y, required_key);
            accessible_doors += 1;
        } else {
            println!("Need {} for door at ({}, {})", required_key, x, y);
        }
    }

    println!("Accessible doors: {}/{}", accessible_doors, door_keys.len());

    // Resource management
    let mut resources: HashMap<String, i32> = HashMap::new();
    resources.insert("Energy".to_string(), 100);
    resources.insert("Health".to_string(), 100);
    resources.insert("Scan Range".to_string(), 3);

    // Update resources
    *resources.get_mut("Energy").unwrap() -= 25;
    *resources.get_mut("Health").unwrap() -= 10;

    println!("\n=== Resource Status ===");
    for (resource, amount) in &resources {
        println!("{}: {}", resource, amount);
    }
}