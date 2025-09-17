// Level 11 Task 1 Test: Create Async Function for Robot Movement
// Tests that user creates an async function with proper timer/await patterns using Smol

#[cfg(test)]
mod level11_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_async_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("async fn"),
            "❌ You need to create an async function using 'async fn'"
        );
    }

    #[test]
    fn test_has_move_robot_async_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("move_robot_async") || analyzer.code.contains("async fn move_robot"),
            "❌ You need to create an async function for robot movement (move_robot_async or similar)"
        );
    }

    #[test]
    fn test_uses_smol_timer() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_timer = analyzer.code.contains("smol::Timer") ||
                       analyzer.code.contains("Timer::after") ||
                       analyzer.code.contains("Duration");
        assert!(
            has_timer,
            "❌ Your async function should use smol::Timer::after with Duration"
        );
    }

    #[test]
    fn test_uses_await_keyword() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".await"),
            "❌ Your async function should use .await to wait for the timer operation"
        );
    }

    #[test]
    fn test_has_match_or_if_for_directions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_direction_logic = analyzer.code.contains("match direction") ||
                                 analyzer.code.contains("if direction") ||
                                 analyzer.code.contains("right") ||
                                 analyzer.code.contains("down") ||
                                 analyzer.code.contains("left") ||
                                 analyzer.code.contains("up");
        assert!(
            has_direction_logic,
            "❌ Your function should handle different movement directions (right, down, left, up)"
        );
    }

    #[test]
    fn test_has_smol_imports() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_imports = analyzer.code.contains("use smol") ||
                         analyzer.code.contains("smol::");
        assert!(
            has_imports,
            "❌ You need to import smol functionality (use std::time::Duration and smol::Timer)"
        );
    }

    #[test]
    fn test_function_signature_correct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_string_param = analyzer.code.contains("&str") ||
                              analyzer.code.contains("String") ||
                              analyzer.code.contains("direction");
        assert!(
            has_string_param,
            "❌ Your async function should take a direction parameter (&str or String)"
        );
    }

    #[test]
    fn test_uses_block_on() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("smol::block_on") || analyzer.code.contains("block_on"),
            "❌ You should use smol::block_on to run your async code in main"
        );
    }
}

// Reference implementation for comparison
fn main() {
    println!("Level 11 Task 1: Async Function");
    // This would use: smol::block_on(async { move_robot_async("right").await; });
}

// Reference async function pattern
// async fn move_robot_async(direction: &str) {
//     smol::Timer::after(std::time::Duration::from_millis(100)).await;
//     match direction {
//         "right" => println!("Moving right"),
//         "down" => println!("Moving down"),
//         "left" => println!("Moving left"),
//         "up" => println!("Moving up"),
//         _ => println!("Invalid direction"),
//     }
// }