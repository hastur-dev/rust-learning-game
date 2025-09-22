// Level 11 Task 2 Test: Use block_on to Run Async Code
// Tests that user uses smol::block_on to execute async functions from main

#[cfg(test)]
mod level11_task2_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_block_on() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("smol::block_on") || analyzer.code.contains("block_on"),
            "❌ You need to use smol::block_on to run async code from main"
        );
    }

    #[test]
    fn test_main_function_not_async() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            !analyzer.code.contains("async fn main"),
            "❌ Your main function should NOT be async when using smol::block_on"
        );
    }

    #[test]
    fn test_has_async_block() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("async {") || analyzer.code.contains("async move {"),
            "❌ You should pass an async block to smol::block_on"
        );
    }

    #[test]
    fn test_uses_await_in_block() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(".await"),
            "❌ Your async block should use .await to call async functions"
        );
    }

    #[test]
    fn test_has_startup_message() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_startup = analyzer.code.contains("Starting async") ||
                         analyzer.code.contains("async robot") ||
                         analyzer.code.contains("Beginning") ||
                         analyzer.contains_println_with_text("Starting async robot control...");
        assert!(
            has_startup,
            "❌ Your async block should print a startup message like 'Starting async robot control...'"
        );
    }

    #[test]
    fn test_calls_async_movement() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_movement_call = analyzer.code.contains("move_robot_async") ||
                               analyzer.code.contains("robot") ||
                               analyzer.code.contains("movement");
        assert!(
            has_movement_call,
            "❌ Your async block should call your async movement function"
        );
    }

    #[test]
    fn test_multiple_awaits() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let await_count = analyzer.code.matches(".await").count();
        assert!(
            await_count >= 2,
            "❌ You should call multiple async functions with .await (at least 2)"
        );
    }

    #[test]
    fn test_proper_block_on_usage() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_structure = analyzer.code.contains("smol::block_on") &&
                           analyzer.code.contains("async {") &&
                           analyzer.code.contains(".await");

        assert!(
            has_structure,
            "❌ Your code should have proper structure: smol::block_on(async {{ ... }}) with .await usage"
        );
    }
}

// Reference implementation
fn main() {
    println!("Level 11 Task 2: Block On");
    // This would use: smol::block_on(async { ... });
}

// Reference pattern for smol::block_on usage
// fn main() {
//     smol::block_on(async {
//         println!("Starting async robot control...");
//         move_robot_async("right").await;
//         move_robot_async("down").await;
//         println!("Async robot operations complete!");
//     })
// }
//
// async fn move_robot_async(direction: &str) {
//     smol::Timer::after(std::time::Duration::from_millis(100)).await;
//     println!("Moving {} asynchronously", direction);
// }