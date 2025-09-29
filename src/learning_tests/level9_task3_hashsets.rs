// Level 9 Task 3 Test: HashSet and Unique Collections
// Tests if the user code uses HashSet<T> for unique data management

#[cfg(test)]
mod level9_task3_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_imports_hashset() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("use std::collections::HashSet") ||
            analyzer.code.contains("HashSet"),
            "❌ Your code should import and use HashSet"
        );
    }

    #[test]
    fn test_creates_hashset() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("HashSet::new()") ||
            analyzer.code.contains("HashSet<"),
            "❌ Your code should create HashSet collections"
        );
    }

    #[test]
    fn test_uses_insert_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".insert("),
            "❌ Your code should use .insert() to add elements to HashSet"
        );
    }

    #[test]
    fn test_visited_positions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_visited = analyzer.code.contains("visited_positions") ||
                         analyzer.code.contains("visited");
        assert!(
            has_visited,
            "❌ Your code should track visited positions using HashSet"
        );
    }

    #[test]
    fn test_duplicate_detection() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let handles_duplicates = analyzer.code.contains("Already") ||
                                analyzer.code.contains("duplicate") ||
                                analyzer.code.contains("skipping");
        assert!(
            handles_duplicates,
            "❌ Your code should detect and handle duplicate entries"
        );
    }

    #[test]
    fn test_position_tuples() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_tuple_positions = analyzer.code.contains("(i32, i32)") &&
                                 analyzer.code.contains("HashSet");
        assert!(
            has_tuple_positions,
            "❌ Your code should use position tuples in HashSet"
        );
    }

    #[test]
    fn test_item_types_collection() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_item_types = analyzer.code.contains("item_types") ||
                           analyzer.code.contains("collected");
        assert!(
            has_item_types,
            "❌ Your code should collect unique item types using HashSet"
        );
    }

    #[test]
    fn test_enemy_tracking() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let tracks_enemies = analyzer.code.contains("enemy") ||
                            analyzer.code.contains("threat") ||
                            analyzer.code.contains("alert");
        assert!(
            tracks_enemies,
            "❌ Your code should track unique enemies using HashSet"
        );
    }

    #[test]
    fn test_set_operations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_set_ops = analyzer.code.contains(".difference(") ||
                         analyzer.code.contains(".union(") ||
                         analyzer.code.contains(".intersection(");
        assert!(
            has_set_ops,
            "❌ Your code should use HashSet operations like difference, union, or intersection"
        );
    }

    #[test]
    fn test_into_iter_collect() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_into_iter = analyzer.code.contains(".into_iter().collect()") ||
                           analyzer.code.contains("into_iter");
        assert!(
            uses_into_iter,
            "❌ Your code should use .into_iter().collect() to create HashSet from Vec"
        );
    }

    #[test]
    fn test_is_subset_or_superset() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_subset_ops = analyzer.code.contains(".is_subset(") ||
                           analyzer.code.contains(".is_superset(");
        assert!(
            has_subset_ops,
            "❌ Your code should use .is_subset() or .is_superset() for set comparisons"
        );
    }

    #[test]
    fn test_mission_completion_check() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let checks_completion = analyzer.code.contains("mission") ||
                              analyzer.code.contains("complete") ||
                              analyzer.code.contains("required");
        assert!(
            checks_completion,
            "❌ Your code should check mission completion using HashSet operations"
        );
    }

    #[test]
    fn test_len_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".len()"),
            "❌ Your code should use .len() to get HashSet size"
        );
    }

    #[test]
    fn test_door_analysis() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let analyzes_doors = analyzer.code.contains("door") &&
                           analyzer.code.contains("accessible");
        assert!(
            analyzes_doors,
            "❌ Your code should analyze door accessibility using HashSet operations"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_hashset_output = result.stdout.contains("unique") ||
                                result.stdout.contains("visited") ||
                                result.stdout.contains("Already") ||
                                result.stdout.contains("duplicate") ||
                                result.stdout.contains("locked");

        assert!(
            has_hashset_output,
            "❌ Your program should output information about HashSet operations and unique collections"
        );
    }
}

// Reference implementation
fn main() {
    use std::collections::HashSet;

    println!("=== Robot Exploration with HashSet ===");

    // Track visited positions to avoid revisiting
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();

    // Exploration path
    let exploration_moves = vec![
        (0, 0), (1, 0), (2, 0), (3, 1), // Move to first item
        (3, 2), (4, 2), (5, 2), (5, 2), // Repeated position
        (6, 2), (7, 2), (8, 2), (9, 2), (10, 2), // Move across
        (3, 1), // Revisit first item position
    ];

    println!("Processing {} movement commands...", exploration_moves.len());

    for (x, y) in exploration_moves {
        if visited_positions.insert((x, y)) {
            println!("New position visited: ({}, {})", x, y);
        } else {
            println!("Already visited ({}, {}) - skipping", x, y);
        }
    }

    println!("Total unique positions visited: {}", visited_positions.len());

    // Track collected item types to avoid duplicates
    let mut collected_item_types: HashSet<String> = HashSet::new();

    let items_found = vec![
        "vector_core", "hashmap_engine", "vector_core", // Duplicate
        "iterator_module", "collection_analyzer", "hashmap_engine", // Duplicate
    ];

    println!("\n=== Item Collection (No Duplicates) ===");
    for item in items_found {
        if collected_item_types.insert(item.to_string()) {
            println!("Collected new item type: {}", item);
        } else {
            println!("Already have {}, leaving duplicate", item);
        }
    }

    println!("Unique item types collected: {}", collected_item_types.len());

    // Enemy tracking - avoid duplicate alerts
    let mut alerted_enemies: HashSet<(i32, i32)> = HashSet::new();

    let enemy_sightings = vec![
        (5, 2), (10, 4), (5, 2), // Duplicate sighting
        (3, 7), (12, 8), (7, 6), (5, 2), // Another duplicate
    ];

    println!("\n=== Enemy Alert System ===");
    for enemy_pos in enemy_sightings {
        if alerted_enemies.insert(enemy_pos) {
            println!("🚨 NEW THREAT at ({}, {})", enemy_pos.0, enemy_pos.1);
        } else {
            println!("Already tracking enemy at ({}, {})", enemy_pos.0, enemy_pos.1);
        }
    }

    // Set operations
    let level_doors: HashSet<(i32, i32)> = vec![
        (8, 1), (4, 5), (11, 7), (6, 9)
    ].into_iter().collect();

    let accessible_doors: HashSet<(i32, i32)> = vec![
        (8, 1), (4, 5) // Have keys for these
    ].into_iter().collect();

    // Find doors we can't access
    let locked_doors: HashSet<_> = level_doors.difference(&accessible_doors).collect();
    println!("\n=== Door Analysis ===");
    println!("Total doors: {}", level_doors.len());
    println!("Accessible doors: {}", accessible_doors.len());
    println!("Locked doors: {}", locked_doors.len());

    for door_pos in locked_doors {
        println!("  Locked door at: {:?}", door_pos);
    }

    // Check if all doors are accessible
    if accessible_doors.is_superset(&level_doors) {
        println!("✅ All doors accessible!");
    } else {
        println!("❌ Some doors remain locked");
    }

    // Required vs collected items
    let required_items: HashSet<String> = vec![
        "vector_core".to_string(),
        "hashmap_engine".to_string(),
        "iterator_module".to_string(),
        "collection_analyzer".to_string(),
    ].into_iter().collect();

    let mission_complete = required_items.is_subset(&collected_item_types);
    println!("\n=== Mission Status ===");
    println!("Required items: {}", required_items.len());
    println!("Collected items: {}", collected_item_types.len());
    println!("Mission complete: {}", mission_complete);

    // Find missing items
    let missing_items: HashSet<_> = required_items.difference(&collected_item_types).collect();
    if !missing_items.is_empty() {
        println!("Still need to collect:");
        for item in missing_items {
            println!("  - {}", item);
        }
    }
}