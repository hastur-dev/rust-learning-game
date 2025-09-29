// Level 11 Task 5 Test: Create Door Opening with Async
// Tests if the user code creates async functions for door detection and opening

#[cfg(test)]
mod level11_task5_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_defines_open_doors_async() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_open_doors = analyzer.code.contains("open_doors_async") ||
                           analyzer.code.contains("async fn open_doors");
        assert!(
            has_open_doors,
            "❌ Your code should define an open_doors_async function"
        );
    }

    #[test]
    fn test_scans_for_doors() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let scans_for_doors = analyzer.code.contains("robot.scan(\"all\")") ||
                            analyzer.code.contains("robot.scan") &&
                            analyzer.code.contains("all");
        assert!(
            scans_for_doors,
            "❌ Your function should scan all areas to detect doors"
        );
    }

    #[test]
    fn test_checks_for_door_in_scan() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let checks_door = analyzer.code.contains("scan_result.contains(\"door\")") ||
                         analyzer.code.contains("contains(\"door\")");
        assert!(
            checks_door,
            "❌ Your code should check if scan results contain 'door'"
        );
    }

    #[test]
    fn test_prints_door_detected() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let prints_detected = analyzer.code.contains("Door detected") ||
                             analyzer.code.contains("door") &&
                             analyzer.code.contains("detected");
        assert!(
            prints_detected,
            "❌ Your code should print a message when a door is detected"
        );
    }

    #[test]
    fn test_uses_timer_for_door_delay() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_timer = analyzer.code.contains("Timer::after") &&
                       analyzer.code.contains("100");
        assert!(
            has_timer,
            "❌ Your code should use Timer::after with ~100ms delay for door opening"
        );
    }

    #[test]
    fn test_calls_robot_open_door() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let calls_open_door = analyzer.code.contains("robot.open_door()") ||
                            analyzer.code.contains("robot.open_door");
        assert!(
            calls_open_door,
            "❌ Your code should call robot.open_door() to open the door"
        );
    }

    #[test]
    fn test_prints_door_opened() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let prints_opened = analyzer.code.contains("Door opened successfully") ||
                          analyzer.code.contains("door") &&
                          analyzer.code.contains("opened");
        assert!(
            prints_opened,
            "❌ Your code should print a success message when door is opened"
        );
    }

    #[test]
    fn test_async_function_structure() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_structure = analyzer.code.contains("async fn") &&
                          analyzer.code.contains("scan_result") &&
                          analyzer.code.contains("if") &&
                          analyzer.code.contains("await");
        assert!(
            has_structure,
            "❌ Your function should have proper async structure with scan, if condition, and await"
        );
    }

    #[test]
    fn test_conditional_door_opening() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_conditional = analyzer.code.contains("if ") &&
                            analyzer.code.contains("contains(\"door\")") &&
                            analyzer.code.contains("open_door");
        assert!(
            has_conditional,
            "❌ Your code should conditionally open doors only when detected"
        );
    }

    #[test]
    fn test_proper_awaiting() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let awaits_timer = analyzer.code.contains("Timer::after") &&
                          analyzer.code.contains(".await");
        assert!(
            awaits_timer,
            "❌ Your code should await the timer delay before opening door"
        );
    }

    #[test]
    fn test_uses_opening_message() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_opening_msg = analyzer.code.contains("Opening") ||
                            analyzer.code.contains("opening");
        assert!(
            has_opening_msg,
            "❌ Your code should print an 'Opening...' message during the process"
        );
    }

    #[test]
    fn test_door_detection_logic() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_detection_logic = analyzer.code.contains("scan_result") &&
                                analyzer.code.contains("if") &&
                                analyzer.code.contains("door");
        assert!(
            has_detection_logic,
            "❌ Your code should implement proper door detection logic"
        );
    }

    #[test]
    fn test_sequential_door_operations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        // Check that detection comes before opening
        let scan_pos = analyzer.code.find("scan_result");
        let open_pos = analyzer.code.find("open_door");

        if let (Some(scan), Some(open)) = (scan_pos, open_pos) {
            assert!(
                scan < open,
                "❌ Door scanning should come before door opening"
            );
        } else {
            assert!(false, "❌ Missing scan_result or open_door calls");
        }
    }

    #[test]
    fn test_delay_between_detect_and_open() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_delay_sequence = analyzer.code.contains("Door detected") &&
                               analyzer.code.contains("Timer::after") &&
                               analyzer.code.contains("open_door");
        assert!(
            has_delay_sequence,
            "❌ Your code should have a delay between detecting and opening doors"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_door_output = result.stdout.contains("Door") ||
                             result.stdout.contains("door") ||
                             result.stdout.contains("Opening") ||
                             result.stdout.contains("scan");

        assert!(
            has_door_output,
            "❌ Your program should output information about door detection and opening"
        );
    }
}

// Reference implementation
fn main() {
    use std::time::Duration;

    // Simulate robot operations
    fn robot_scan(_area: &str) -> String {
        "Scan results: door detected at position (2,2), wall at (3,3)".to_string()
    }

    fn robot_open_door() {
        println!("*MECHANICAL WHIR* Door mechanism activated");
    }

    async fn open_doors_async() {
        println!("=== Level 11 Task 5: Async Door Opening ===");

        let scan_result = robot_scan("all");
        println!("Scanning area...");
        println!("Scan result: {}", scan_result);

        if scan_result.contains("door") {
            println!("Door detected! Opening...");
            smol::Timer::after(Duration::from_millis(100)).await;
            robot_open_door();
            println!("Door opened successfully!");
        } else {
            println!("No doors detected in the area");
        }
    }

    async fn door_sequence_demo() {
        println!("Starting door detection and opening sequence...");

        // Test with door present
        open_doors_async().await;

        println!("\nWaiting before next scan...");
        smol::Timer::after(Duration::from_millis(300)).await;

        // Test multiple door operations
        for i in 1..=3 {
            println!("\n--- Door Check {} ---", i);
            open_doors_async().await;
            smol::Timer::after(Duration::from_millis(200)).await;
        }

        println!("\nDoor sequence completed!");
    }

    smol::block_on(door_sequence_demo());
}