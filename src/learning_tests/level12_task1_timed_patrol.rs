// Level 12 Task 1 Test: Create Timed Robot Patrol
// Tests if the user code uses Smol timers to create a patrol system

#[cfg(test)]
mod level12_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_imports_futures_lite() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let imports_futures = analyzer.code.contains("use futures_lite::future") ||
                             analyzer.code.contains("futures_lite");
        assert!(
            imports_futures,
            "❌ Your code should import futures_lite::future"
        );
    }

    #[test]
    fn test_defines_timed_patrol() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_patrol = analyzer.code.contains("timed_patrol") ||
                        analyzer.code.contains("async fn timed_patrol");
        assert!(
            has_patrol,
            "❌ Your code should define a timed_patrol function"
        );
    }

    #[test]
    fn test_creates_patrol_points() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_patrol_points = analyzer.code.contains("patrol_points") ||
                               analyzer.code.contains("vec![");
        assert!(
            has_patrol_points,
            "❌ Your code should create a vector of patrol points"
        );
    }

    #[test]
    fn test_patrol_points_coordinates() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_coords = analyzer.code.contains("(2, 2)") &&
                        analyzer.code.contains("(8, 2)") &&
                        analyzer.code.contains("(8, 6)") &&
                        analyzer.code.contains("(2, 6)");
        assert!(
            has_coords,
            "❌ Your patrol points should include coordinates (2,2), (8,2), (8,6), (2,6)"
        );
    }

    #[test]
    fn test_iterates_patrol_points() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let iterates_points = analyzer.code.contains("for point in patrol_points") ||
                            analyzer.code.contains("for ") &&
                            analyzer.code.contains("patrol_points");
        assert!(
            iterates_points,
            "❌ Your code should iterate through patrol points"
        );
    }

    #[test]
    fn test_prints_patrol_movement() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let prints_movement = analyzer.code.contains("Patrolling to point") ||
                            analyzer.code.contains("patrol") &&
                            analyzer.code.contains("point");
        assert!(
            prints_movement,
            "❌ Your code should print patrol movement messages"
        );
    }

    #[test]
    fn test_moves_to_points() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let moves_to_points = analyzer.code.contains("robot.move_to") ||
                            analyzer.code.contains("move_to(point");
        assert!(
            moves_to_points,
            "❌ Your code should move robot to patrol points"
        );
    }

    #[test]
    fn test_uses_timer_at_patrol_points() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_timer = analyzer.code.contains("Timer::after") &&
                        analyzer.code.contains("800");
        assert!(
            uses_timer,
            "❌ Your code should use Timer::after with ~800ms delay at patrol points"
        );
    }

    #[test]
    fn test_scans_at_patrol_points() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let scans_points = analyzer.code.contains("robot.scan") &&
                          analyzer.code.contains("all");
        assert!(
            scans_points,
            "❌ Your code should scan at each patrol point"
        );
    }

    #[test]
    fn test_prints_scan_results() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let prints_scan = analyzer.code.contains("Patrol scan") ||
                         analyzer.code.contains("scan_result");
        assert!(
            prints_scan,
            "❌ Your code should print patrol scan results"
        );
    }

    #[test]
    fn test_awaits_movement() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let awaits_movement = analyzer.code.contains("move_to") &&
                            analyzer.code.contains(".await");
        assert!(
            awaits_movement,
            "❌ Your code should await robot movement operations"
        );
    }

    #[test]
    fn test_awaits_timer() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let awaits_timer = analyzer.code.contains("Timer::after") &&
                          analyzer.code.contains(".await");
        assert!(
            awaits_timer,
            "❌ Your code should await timer delays"
        );
    }

    #[test]
    fn test_patrol_sequence() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        // Check proper sequence: move -> wait -> scan
        let has_sequence = analyzer.code.contains("move_to") &&
                          analyzer.code.contains("Timer::after") &&
                          analyzer.code.contains("scan");
        assert!(
            has_sequence,
            "❌ Your patrol should follow sequence: move -> wait -> scan"
        );
    }

    #[test]
    fn test_accesses_point_coordinates() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let accesses_coords = analyzer.code.contains("point.0") &&
                            analyzer.code.contains("point.1");
        assert!(
            accesses_coords,
            "❌ Your code should access point coordinates with point.0 and point.1"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_patrol_output = result.stdout.contains("Patrolling") ||
                               result.stdout.contains("patrol") ||
                               result.stdout.contains("point") ||
                               result.stdout.contains("scan");

        assert!(
            has_patrol_output,
            "❌ Your program should output information about patrol operations"
        );
    }
}

// Reference implementation
fn main() {
    use std::time::Duration;
    use futures_lite::future;

    // Simulate robot operations
    async fn robot_move_to(x: i32, y: i32) {
        println!("Moving robot to ({}, {})", x, y);
        smol::Timer::after(Duration::from_millis(100)).await;
    }

    fn robot_scan(_area: &str) -> String {
        "Area clear, no threats detected".to_string()
    }

    async fn timed_patrol() {
        println!("=== Level 12 Task 1: Timed Robot Patrol ===");

        let patrol_points = vec![(2, 2), (8, 2), (8, 6), (2, 6)];

        println!("Starting patrol sequence with {} points", patrol_points.len());

        for (i, point) in patrol_points.iter().enumerate() {
            println!("Patrolling to point {:?} (stop {}/{})", point, i + 1, patrol_points.len());
            robot_move_to(point.0, point.1).await;

            println!("Arrived at patrol point {:?}, waiting...", point);
            // Wait at patrol point
            smol::Timer::after(Duration::from_millis(800)).await;

            // Quick scan at each point
            let scan_result = robot_scan("all");
            println!("Patrol scan: {}", scan_result);

            if i < patrol_points.len() - 1 {
                println!("Moving to next patrol point...\n");
            }
        }

        println!("Patrol sequence completed!");
    }

    async fn patrol_demo() {
        println!("Initiating automated patrol system...");

        // Run multiple patrol cycles
        for cycle in 1..=2 {
            println!("\n--- Patrol Cycle {} ---", cycle);
            timed_patrol().await;

            if cycle < 2 {
                println!("Waiting before next patrol cycle...");
                smol::Timer::after(Duration::from_millis(500)).await;
            }
        }

        println!("\nPatrol system demonstration completed!");
    }

    smol::block_on(patrol_demo());
}