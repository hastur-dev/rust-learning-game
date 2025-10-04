// Level 9 Task 4 Test: Iterator Methods and Data Processing
// Tests if the user code uses iterator methods for data processing

#[cfg(test)]
mod level9_task4_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_uses_filter_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".filter("),
            "❌ Your code should use .filter() to process data"
        );
    }

    #[test]
    fn test_uses_map_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".map("),
            "❌ Your code should use .map() to transform data"
        );
    }

    #[test]
    fn test_uses_collect_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".collect()"),
            "❌ Your code should use .collect() to build collections from iterators"
        );
    }

    #[test]
    fn test_scan_data_processing() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let processes_scan_data = analyzer.code.contains("scan_data") ||
                                 analyzer.code.contains("scan");
        assert!(
            processes_scan_data,
            "❌ Your code should process scan data using iterators"
        );
    }

    #[test]
    fn test_enemy_filtering() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let filters_enemies = analyzer.code.contains("Enemy") &&
                             analyzer.code.contains(".filter(");
        assert!(
            filters_enemies,
            "❌ Your code should filter enemies from scan data"
        );
    }

    #[test]
    fn test_sort_operation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".sort_by(") ||
            analyzer.code.contains(".sort()"),
            "❌ Your code should sort data collections"
        );
    }

    #[test]
    fn test_distance_calculation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let calculates_distance = analyzer.code.contains("distance") &&
                                 analyzer.code.contains("abs");
        assert!(
            calculates_distance,
            "❌ Your code should calculate distances in data processing"
        );
    }

    #[test]
    fn test_min_max_operations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_min_or_max = analyzer.code.contains(".min_by(") ||
                           analyzer.code.contains(".max_by(") ||
                           analyzer.code.contains(".min()") ||
                           analyzer.code.contains(".max()");
        assert!(
            has_min_or_max,
            "❌ Your code should use min/max operations to find optimal values"
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
    fn test_energy_analysis() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let analyzes_energy = analyzer.code.contains("energy") ||
                            analyzer.code.contains("cost");
        assert!(
            analyzes_energy,
            "❌ Your code should analyze energy consumption using iterators"
        );
    }

    #[test]
    fn test_windows_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".windows("),
            "❌ Your code should use .windows() for path segment processing"
        );
    }

    #[test]
    fn test_enumerate_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".enumerate()"),
            "❌ Your code should use .enumerate() for indexed iteration"
        );
    }

    #[test]
    fn test_take_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".take("),
            "❌ Your code should use .take() to limit output"
        );
    }

    #[test]
    fn test_chain_operations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_chained_ops = analyzer.code.matches(".").count() >= 3;
        assert!(
            has_chained_ops,
            "❌ Your code should chain multiple iterator operations together"
        );
    }

    #[test]
    fn test_path_optimization() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let optimizes_path = analyzer.code.contains("path") &&
                           analyzer.code.contains("waypoint");
        assert!(
            optimizes_path,
            "❌ Your code should optimize paths using iterator methods"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_iterator_output = result.stdout.contains("Priority") ||
                                 result.stdout.contains("Distance") ||
                                 result.stdout.contains("energy") ||
                                 result.stdout.contains("Nearest") ||
                                 result.stdout.contains("Total");

        assert!(
            has_iterator_output,
            "❌ Your program should output results from iterator processing"
        );
    }
}

// Reference implementation
fn main() {
    println!("=== Robot Data Processing with Iterators ===");

    // Robot scan data processing
    let scan_data = vec![
        ("Enemy", 5, 2, 3),    // (type, x, y, threat_level)
        ("Item", 3, 1, 0),
        ("Enemy", 10, 4, 5),
        ("Door", 8, 1, 0),
        ("Item", 13, 3, 0),
        ("Enemy", 3, 7, 2),
        ("Wall", 7, 3, 0),
        ("Door", 4, 5, 0),
    ];

    println!("Processing {} scan results...", scan_data.len());

    // Filter enemies and sort by threat level
    let mut enemies: Vec<_> = scan_data.iter()
        .filter(|(obj_type, _, _, _)| *obj_type == "Enemy")
        .collect();

    enemies.sort_by(|a, b| b.3.cmp(&a.3)); // Sort by threat level (descending)

    println!("\n=== Enemy Threat Analysis ===");
    for (i, (_, x, y, threat)) in enemies.iter().enumerate() {
        println!("Priority {}: Enemy at ({}, {}) - Threat: {}", i + 1, x, y, threat);
    }

    // Find all items and calculate distances from robot
    let robot_pos = (0, 0);
    let items_with_distance: Vec<_> = scan_data.iter()
        .filter(|(obj_type, _, _, _)| *obj_type == "Item")
        .map(|(_, x, y, _)| {
            let distance = (robot_pos.0 - *x as i32).abs() + (robot_pos.1 - *y as i32).abs();
            ((x, y), distance)
        })
        .collect();

    println!("\n=== Item Collection Route ===");
    for ((x, y), distance) in &items_with_distance {
        println!("Item at ({}, {}) - Distance: {}", x, y, distance);
    }

    // Find nearest item
    let nearest_item = items_with_distance.iter()
        .min_by(|a, b| a.1.cmp(&b.1));

    if let Some(((x, y), distance)) = nearest_item {
        println!("Nearest item: ({}, {}) at distance {}", x, y, distance);
    }

    // Energy consumption analysis
    let moves = vec![
        ("right", 5),   // (direction, energy_cost)
        ("up", 3),
        ("right", 8),
        ("down", 2),
        ("right", 12),
        ("up", 7),
    ];

    let total_energy: i32 = moves.iter().map(|(_, cost)| cost).sum();
    let expensive_moves: Vec<_> = moves.iter()
        .filter(|(_, cost)| *cost > 5)
        .collect();

    println!("\n=== Energy Analysis ===");
    println!("Total energy cost: {}", total_energy);
    println!("Expensive moves (>5 energy):");
    for (direction, cost) in expensive_moves {
        println!("  {} direction: {} energy", direction, cost);
    }

    // Path optimization - find efficient route
    let waypoints: Vec<(i32, i32)> = vec![(3, 1), (13, 3), (2, 8), (14, 9)];
    let path_segments: Vec<_> = waypoints.windows(2)
        .map(|segment| {
            let (x1, y1) = segment[0];
            let (x2, y2) = segment[1];
            let distance = (x2 - x1).abs() + (y2 - y1).abs();
            (segment[0], segment[1], distance)
        })
        .collect();

    println!("\n=== Path Optimization ===");
    for (start, end, distance) in &path_segments {
        println!("From {:?} to {:?}: {} units", start, end, distance);
    }

    let total_path_distance: i32 = path_segments.iter().map(|(_, _, d)| d).sum();
    println!("Total path distance: {} units", total_path_distance);

    // Resource management with iterators
    let resources = vec![
        ("Energy", 75),
        ("Health", 90),
        ("Ammo", 15),
        ("Fuel", 60),
    ];

    // Find critical resources (below 50)
    let critical_resources: Vec<_> = resources.iter()
        .filter(|(_, amount)| *amount < 50)
        .collect();

    println!("\n=== Resource Status ===");
    if critical_resources.is_empty() {
        println!("All resources above critical levels");
    } else {
        println!("Critical resources:");
        for (resource, amount) in critical_resources {
            println!("  {}: {}", resource, amount);
        }
    }

    // Inventory value calculation
    let inventory = vec![
        ("vector_core", 100, 1),      // (item, value, quantity)
        ("hashmap_engine", 150, 1),
        ("energy_cell", 25, 3),
        ("repair_kit", 50, 2),
    ];

    let total_value: i32 = inventory.iter()
        .map(|(_, value, qty)| value * qty)
        .sum();

    let most_valuable = inventory.iter()
        .max_by(|a, b| (a.1 * a.2).cmp(&(b.1 * b.2)));

    println!("\n=== Inventory Analysis ===");
    println!("Total inventory value: {} credits", total_value);
    if let Some((item, value, qty)) = most_valuable {
        println!("Most valuable: {} x{} = {} credits", item, qty, value * qty);
    }

    // Complex data transformation
    let mission_data: Vec<_> = scan_data.iter()
        .enumerate()
        .filter(|(_, (obj_type, _, _, _))| *obj_type != "Wall")
        .map(|(index, (obj_type, x, y, priority))| {
            format!("Scan #{}: {} at ({}, {}) priority:{}",
                   index + 1, obj_type, x, y, priority)
        })
        .collect();

    println!("\n=== Mission Report ===");
    for report in mission_data.iter().take(5) {
        println!("  {}", report);
    }
}