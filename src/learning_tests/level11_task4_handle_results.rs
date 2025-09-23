// Level 11 Task 4 Test: Handle Task Results
// Tests if the user code spawns tasks that return values and handles their results

#[cfg(test)]
mod level11_task4_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_defines_scan_area_async() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_scan_async = analyzer.code.contains("scan_area_async") ||
                           analyzer.code.contains("async fn scan_area");
        assert!(
            has_scan_async,
            "❌ Your code should define a scan_area_async function"
        );
    }

    #[test]
    fn test_scan_area_returns_string() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let returns_string = analyzer.code.contains("-> String") &&
                           analyzer.code.contains("scan_area_async");
        assert!(
            returns_string,
            "❌ Your scan_area_async function should return a String"
        );
    }

    #[test]
    fn test_scan_area_has_delay() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_delay = analyzer.code.contains("Timer::after") &&
                       analyzer.code.contains("200");
        assert!(
            has_delay,
            "❌ Your scan_area_async should have a ~200ms delay"
        );
    }

    #[test]
    fn test_defines_parallel_scanning() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_parallel = analyzer.code.contains("parallel_scanning") ||
                          analyzer.code.contains("async fn parallel_scanning");
        assert!(
            has_parallel,
            "❌ Your code should define a parallel_scanning function"
        );
    }

    #[test]
    fn test_spawns_directional_tasks() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_north = analyzer.code.contains("north_task");
        let has_south = analyzer.code.contains("south_task");
        let has_east = analyzer.code.contains("east_task");
        let has_west = analyzer.code.contains("west_task");

        assert!(
            has_north && has_south && has_east && has_west,
            "❌ Your code should spawn tasks for all four directions (north, south, east, west)"
        );
    }

    #[test]
    fn test_scans_directional_areas() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let scans_north = analyzer.code.contains("\"north\"");
        let scans_south = analyzer.code.contains("\"south\"");
        let scans_east = analyzer.code.contains("\"east\"");
        let scans_west = analyzer.code.contains("\"west\"");

        assert!(
            scans_north && scans_south && scans_east && scans_west,
            "❌ Your code should scan in all four directions"
        );
    }

    #[test]
    fn test_uses_join_all() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_join_all = analyzer.code.contains("join_all") ||
                           analyzer.code.contains("futures_lite::future::join_all");
        assert!(
            uses_join_all,
            "❌ Your code should use futures_lite::future::join_all to wait for all tasks"
        );
    }

    #[test]
    fn test_collects_task_results() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let collects_results = analyzer.code.contains("results") &&
                              analyzer.code.contains("await");
        assert!(
            collects_results,
            "❌ Your code should collect and await task results"
        );
    }

    #[test]
    fn test_iterates_over_results() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let iterates_results = analyzer.code.contains("for (i, result)") ||
                              analyzer.code.contains("enumerate()") ||
                              analyzer.code.contains("results.iter()");
        assert!(
            iterates_results,
            "❌ Your code should iterate over the results with enumerate or similar"
        );
    }

    #[test]
    fn test_prints_direction_results() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let prints_results = analyzer.code.contains("Direction") &&
                           analyzer.code.contains("result");
        assert!(
            prints_results,
            "❌ Your code should print results for each direction"
        );
    }

    #[test]
    fn test_passes_area_parameter() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_area_param = analyzer.code.contains("area: &str") ||
                           analyzer.code.contains("area:");
        assert!(
            has_area_param,
            "❌ Your scan_area_async function should accept an area parameter"
        );
    }

    #[test]
    fn test_uses_robot_scan_in_async() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_robot_scan = analyzer.code.contains("robot.scan(area)") ||
                            analyzer.code.contains("robot.scan");
        assert!(
            uses_robot_scan,
            "❌ Your scan_area_async should call robot.scan(area)"
        );
    }

    #[test]
    fn test_spawn_returns_values() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let spawns_with_return = analyzer.code.contains("spawn(scan_area_async") ||
                               analyzer.code.contains("spawn(async") &&
                               analyzer.code.contains("scan_area_async");
        assert!(
            spawns_with_return,
            "❌ Your spawned tasks should call scan_area_async to return values"
        );
    }

    #[test]
    fn test_creates_vec_of_tasks() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let creates_vec = analyzer.code.contains("vec![") &&
                         analyzer.code.contains("_task");
        assert!(
            creates_vec,
            "❌ Your code should create a vec! of spawned tasks for join_all"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_results_output = result.stdout.contains("Direction") ||
                                result.stdout.contains("scan") ||
                                result.stdout.contains("parallel") ||
                                result.stdout.contains("north") ||
                                result.stdout.contains("result");

        assert!(
            has_results_output,
            "❌ Your program should output information about scan results and directions"
        );
    }
}

// Reference implementation
fn main() {
    use std::time::Duration;

    // Simulate robot scanning
    fn robot_scan(area: &str) -> String {
        match area {
            "north" => "North: Clear path ahead".to_string(),
            "south" => "South: Wall detected".to_string(),
            "east" => "East: Enemy spotted".to_string(),
            "west" => "West: Item discovered".to_string(),
            _ => "Unknown area".to_string(),
        }
    }

    async fn scan_area_async(area: &str) -> String {
        smol::Timer::after(Duration::from_millis(200)).await;
        robot_scan(area)
    }

    async fn parallel_scanning() {
        println!("=== Level 11 Task 4: Parallel Scanning ===");

        let north_task = smol::spawn(scan_area_async("north"));
        let south_task = smol::spawn(scan_area_async("south"));
        let east_task = smol::spawn(scan_area_async("east"));
        let west_task = smol::spawn(scan_area_async("west"));

        println!("Spawned scanning tasks for all directions...");

        let results = futures_lite::future::join_all(vec![
            north_task, south_task, east_task, west_task
        ]).await;

        println!("All scanning tasks completed!");
        println!("Scan Results:");

        let directions = ["North", "South", "East", "West"];
        for (i, result) in results.iter().enumerate() {
            println!("Direction {}: {}", directions[i], result);
        }

        // Additional analysis
        let total_scans = results.len();
        let clear_areas = results.iter()
            .filter(|r| r.contains("Clear"))
            .count();

        println!("\nScan Summary:");
        println!("Total areas scanned: {}", total_scans);
        println!("Clear areas found: {}", clear_areas);
        println!("Threats detected: {}", total_scans - clear_areas);
    }

    smol::block_on(parallel_scanning());
}