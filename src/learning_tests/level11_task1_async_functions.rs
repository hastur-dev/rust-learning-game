// Level 11 Task 1 Test: Create Your First Async Function
// Tests if the user code creates basic async functions with smol

#[cfg(test)]
mod level11_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_creates_async_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_async_fn = analyzer.code.contains("async fn");
        assert!(
            has_async_fn,
            "❌ Your code should define async functions using 'async fn'"
        );
    }

    #[test]
    fn test_defines_move_robot_async() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_move_robot_async = analyzer.code.contains("move_robot_async") ||
                                  analyzer.code.contains("async fn move_robot");
        assert!(
            has_move_robot_async,
            "❌ Your code should define a move_robot_async function"
        );
    }

    #[test]
    fn test_uses_smol_timer() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_timer = analyzer.code.contains("smol::Timer::after") ||
                        analyzer.code.contains("Timer::after");
        assert!(
            uses_timer,
            "❌ Your code should use smol::Timer::after for delays"
        );
    }

    #[test]
    fn test_uses_await_keyword() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_await = analyzer.code.contains(".await");
        assert!(
            uses_await,
            "❌ Your code should use .await to wait for async operations"
        );
    }

    #[test]
    fn test_uses_duration() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_duration = analyzer.code.contains("Duration::from_millis") ||
                           analyzer.code.contains("std::time::Duration");
        assert!(
            uses_duration,
            "❌ Your code should use Duration for timing delays"
        );
    }

    #[test]
    fn test_handles_direction_parameter() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let handles_direction = analyzer.code.contains("direction: &str") ||
                               analyzer.code.contains("direction:");
        assert!(
            handles_direction,
            "❌ Your async function should accept a direction parameter"
        );
    }

    #[test]
    fn test_matches_direction_values() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_match = analyzer.code.contains("match direction") ||
                       analyzer.code.contains("match ");
        assert!(
            has_match,
            "❌ Your code should use match to handle different directions"
        );
    }

    #[test]
    fn test_handles_robot_movement() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let handles_movement = analyzer.code.contains("right") ||
                              analyzer.code.contains("down") ||
                              analyzer.code.contains("left") ||
                              analyzer.code.contains("up");
        assert!(
            handles_movement,
            "❌ Your code should handle different movement directions"
        );
    }

    #[test]
    fn test_proper_async_syntax() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_proper_syntax = analyzer.code.contains("async fn") &&
                               analyzer.code.contains(".await");
        assert!(
            has_proper_syntax,
            "❌ Your code should use proper async/await syntax"
        );
    }

    #[test]
    fn test_timing_configuration() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_timing = analyzer.code.contains("100") ||
                        analyzer.code.contains("from_millis");
        assert!(
            has_timing,
            "❌ Your code should configure timing delays (around 100ms)"
        );
    }

    #[test]
    fn test_imports_required_modules() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let imports_time = analyzer.code.contains("use std::time") ||
                          analyzer.code.contains("Duration");
        assert!(
            imports_time,
            "❌ Your code should import time-related modules"
        );
    }

    #[test]
    fn test_async_function_structure() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_async_structure = analyzer.code.contains("async fn") &&
                                 analyzer.code.contains("Timer::after") &&
                                 analyzer.code.contains("await");
        assert!(
            has_async_structure,
            "❌ Your async function should have the proper structure with Timer and await"
        );
    }

    #[test]
    fn test_handles_invalid_direction() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let handles_invalid = analyzer.code.contains("Invalid") ||
                             analyzer.code.contains("_") ||
                             analyzer.code.contains("default");
        assert!(
            handles_invalid,
            "❌ Your code should handle invalid directions"
        );
    }

    #[test]
    fn test_movement_delay_before_action() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        // Check that delay comes before movement logic
        let timer_pos = analyzer.code.find("Timer::after");
        let match_pos = analyzer.code.find("match");

        if let (Some(timer), Some(match_stmt)) = (timer_pos, match_pos) {
            assert!(
                timer < match_stmt,
                "❌ Timer delay should come before movement logic"
            );
        } else {
            assert!(false, "❌ Missing Timer or match statement");
        }
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_async_output = result.stdout.contains("async") ||
                              result.stdout.contains("Timer") ||
                              result.stdout.contains("movement") ||
                              result.stdout.contains("robot");

        assert!(
            has_async_output,
            "❌ Your program should output information about async operations"
        );
    }
}

// Reference implementation
fn main() {
    use std::time::Duration;

    async fn move_robot_async(direction: &str) {
        smol::Timer::after(Duration::from_millis(100)).await;
        match direction {
            "right" => {
                // robot.move_right();
                println!("Moving robot right");
            }
            "down" => {
                // robot.move_down();
                println!("Moving robot down");
            }
            "left" => {
                // robot.move_left();
                println!("Moving robot left");
            }
            "up" => {
                // robot.move_up();
                println!("Moving robot up");
            }
            _ => println!("Invalid direction"),
        }
    }

    println!("=== Level 11 Task 1: Async Function Creation ===");

    smol::block_on(async {
        println!("Creating first async function...");

        println!("Testing robot movement with async delays:");
        move_robot_async("right").await;
        move_robot_async("down").await;
        move_robot_async("left").await;
        move_robot_async("up").await;

        println!("Testing invalid direction:");
        move_robot_async("invalid").await;

        println!("Async function creation completed!");
    });
}