// Level 9 Task 1 Test: Vector Basics and Dynamic Arrays
// Tests if the user code uses Vec<T> for robot data management

#[cfg(test)]
mod level9_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_creates_vector() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("Vec::new()") ||
            analyzer.code.contains("vec![]") ||
            analyzer.code.contains("Vec<"),
            "❌ Your code should create Vec collections"
        );
    }

    #[test]
    fn test_uses_push_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".push("),
            "❌ Your code should use .push() to add elements to Vec"
        );
    }

    #[test]
    fn test_uses_len_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".len()"),
            "❌ Your code should use .len() to get vector length"
        );
    }

    #[test]
    fn test_iterates_over_vector() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".iter()") ||
            analyzer.code.contains("for "),
            "❌ Your code should iterate over vectors"
        );
    }

    #[test]
    fn test_uses_enumerate() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".enumerate()"),
            "❌ Your code should use .enumerate() for indexed iteration"
        );
    }

    #[test]
    fn test_vector_indexing() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("[") && analyzer.code.contains("]"),
            "❌ Your code should use vector indexing with []"
        );
    }

    #[test]
    fn test_position_tuples() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_position_tuple = analyzer.code.contains("(i32, i32)") ||
                                analyzer.code.contains("(0, 0)");
        assert!(
            has_position_tuple,
            "❌ Your code should use position tuples (i32, i32)"
        );
    }

    #[test]
    fn test_path_planning() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_path = analyzer.code.contains("path") ||
                      analyzer.code.contains("waypoint");
        assert!(
            has_path,
            "❌ Your code should implement path planning with vectors"
        );
    }

    #[test]
    fn test_inventory_management() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("inventory"),
            "❌ Your code should manage inventory using vectors"
        );
    }

    #[test]
    fn test_string_vectors() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("Vec<String>") ||
            analyzer.code.contains("vec!["),
            "❌ Your code should use Vec<String> for item collections"
        );
    }

    #[test]
    fn test_contains_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".contains("),
            "❌ Your code should use .contains() to check for items"
        );
    }

    #[test]
    fn test_pop_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".pop()"),
            "❌ Your code should use .pop() to remove items"
        );
    }

    #[test]
    fn test_filter_collect() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_filter_collect = analyzer.code.contains(".filter(") &&
                               analyzer.code.contains(".collect()");
        assert!(
            has_filter_collect,
            "❌ Your code should use .filter() and .collect() for data processing"
        );
    }

    #[test]
    fn test_distance_calculation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let calculates_distance = analyzer.code.contains("distance") &&
                                 analyzer.code.contains("abs");
        assert!(
            calculates_distance,
            "❌ Your code should calculate distances between positions"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_vector_output = result.stdout.contains("path") ||
                               result.stdout.contains("waypoint") ||
                               result.stdout.contains("inventory") ||
                               result.stdout.contains("distance");

        assert!(
            has_vector_output,
            "❌ Your program should output information about vector operations"
        );
    }
}

// Reference implementation
fn main() {
    println!("=== Robot Path Planning with Vectors ===");

    // Create vector of positions for robot path
    let mut path: Vec<(i32, i32)> = Vec::new();

    // Add waypoints to the path
    path.push((0, 0)); // Start
    path.push((3, 1)); // First item
    path.push((13, 3)); // Second item
    path.push((2, 8)); // Third item
    path.push((14, 9)); // Goal

    println!("Planned path has {} waypoints", path.len());

    // Display the path
    for (index, (x, y)) in path.iter().enumerate() {
        println!("Waypoint {}: ({}, {})", index + 1, x, y);
    }

    // Calculate path distances
    let mut total_distance = 0;
    for i in 1..path.len() {
        let (prev_x, prev_y) = path[i - 1];
        let (curr_x, curr_y) = path[i];
        let distance = (curr_x - prev_x).abs() + (curr_y - prev_y).abs();
        total_distance += distance;
        println!("Segment {} distance: {}", i, distance);
    }

    println!("Total path distance: {}", total_distance);

    // Robot inventory using Vec
    let mut inventory: Vec<String> = vec![];

    // Simulate collecting items
    let items = vec!["vector_core", "hashmap_engine", "iterator_module", "collection_analyzer"];

    for item in items.iter() {
        inventory.push(item.to_string());
        println!("Collected: {}", item);
        println!("Inventory size: {}", inventory.len());
    }

    // Check inventory contents
    println!("Final inventory: {:?}", inventory);

    // Remove and use items
    if let Some(item) = inventory.pop() {
        println!("Used item: {}", item);
    }

    // Check if specific item exists
    if inventory.contains(&"vector_core".to_string()) {
        println!("Vector core still available!");
    }

    // Create scan results vector
    let scan_results: Vec<String> = vec![
        "Enemy at (5, 2)".to_string(),
        "Door at (8, 1)".to_string(),
        "Item at (3, 1)".to_string(),
        "Wall at (7, 3)".to_string(),
    ];

    println!("Scan detected {} objects:", scan_results.len());
    for result in &scan_results {
        println!("  - {}", result);
    }

    // Filter scan results
    let enemies: Vec<_> = scan_results.iter()
        .filter(|s| s.contains("Enemy"))
        .collect();

    println!("Enemies detected: {}", enemies.len());
}