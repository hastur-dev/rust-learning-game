// Level 11 Task 3 Test: Spawn Concurrent Tasks
// Tests that user creates concurrent tasks using smol::spawn

#[cfg(test)]
mod level11_task3_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_uses_smol_spawn() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("smol::spawn"),
            "❌ You should use smol::spawn to create concurrent tasks"
        );
    }

    #[test]
    fn test_spawns_multiple_tasks() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let spawn_count = analyzer.code.matches("smol::spawn").count();
        assert!(
            spawn_count >= 2,
            "❌ You should spawn at least 2 concurrent tasks"
        );
    }

    #[test]
    fn test_has_background_task() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_background = analyzer.code.contains("scan_task") ||
                            analyzer.code.contains("background") ||
                            analyzer.code.contains("loop {");
        assert!(
            has_background,
            "❌ You should create a background scanning task that runs in a loop"
        );
    }

    #[test]
    fn test_has_movement_task() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_movement = analyzer.code.contains("move_task") ||
                          analyzer.code.contains("movement") ||
                          analyzer.code.contains("move_robot");
        assert!(
            has_movement,
            "❌ You should create a movement task"
        );
    }

    #[test]
    fn test_awaits_task_completion() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_task_await = analyzer.code.contains("task.await") ||
                            analyzer.code.contains("move_task.await") ||
                            (analyzer.code.contains("await") && analyzer.code.contains("task"));
        assert!(
            has_task_await,
            "❌ You should await at least one task to wait for its completion"
        );
    }

    #[test]
    fn test_uses_async_closures() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("async {") || analyzer.code.contains("async move {"),
            "❌ Use async closures {} in smol::spawn"
        );
    }

    #[test]
    fn test_has_timer_in_background_task() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_timer = analyzer.code.contains("Timer::after") ||
                       analyzer.code.contains("smol::Timer") ||
                       analyzer.code.contains("Duration");
        assert!(
            has_timer,
            "❌ Your background task should use timers for periodic operations"
        );
    }

    #[test]
    fn test_scanning_in_background() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_scan = analyzer.code.contains("scan(") ||
                      analyzer.code.contains("robot.scan") ||
                      analyzer.code.contains("Background scan");
        assert!(
            has_scan,
            "❌ Your background task should perform scanning operations"
        );
    }

    #[test]
    fn test_main_async_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_main_async = analyzer.code.contains("main_async") ||
                            analyzer.code.contains("async fn main_async");
        assert!(
            has_main_async,
            "❌ You should create a main_async function to organize your async logic"
        );
    }
}

// Reference implementation
fn main() {
    println!("Level 11 Task 3: Spawn Tasks");
    // This would use: smol::block_on(main_async());
}

// Reference pattern for task spawning
// async fn main_async() {
//     // Spawn a background scanning task
//     let scan_task = smol::spawn(async {
//         loop {
//             println!("Background scan: all clear");
//             smol::Timer::after(Duration::from_millis(500)).await;
//         }
//     });
//
//     // Spawn movement task
//     let move_task = smol::spawn(async {
//         move_robot_async("right").await;
//         move_robot_async("down").await;
//     });
//
//     // Wait for movement to complete
//     move_task.await;
// }
//
// async fn move_robot_async(direction: &str) {
//     smol::Timer::after(Duration::from_millis(100)).await;
//     println!("Moving {} asynchronously", direction);
// }