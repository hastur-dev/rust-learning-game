//! Level 12 Task 3: Coordinate Multiple Timed Tasks with Smol
//!
//! This module demonstrates how to coordinate multiple asynchronous tasks that use
//! timers and racing to work together efficiently in the Smol runtime.

use std::time::Duration;
use futures_lite::future;

/// Mock robot for testing coordinated operations
#[derive(Debug, Clone)]
pub struct CoordinatedRobot {
    pub position: (i32, i32),
    pub items_collected: Vec<String>,
    pub doors_opened: u32,
    pub scan_results: Vec<String>,
    pub movement_delay: Duration,
    pub scan_delay: Duration,
}

impl CoordinatedRobot {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            items_collected: Vec::new(),
            doors_opened: 0,
            scan_results: Vec::new(),
            movement_delay: Duration::from_millis(100),
            scan_delay: Duration::from_millis(200),
        }
    }

    pub fn with_movement_delay(mut self, delay: Duration) -> Self {
        self.movement_delay = delay;
        self
    }

    pub fn with_scan_delay(mut self, delay: Duration) -> Self {
        self.scan_delay = delay;
        self
    }

    /// Perform a scan operation
    pub async fn scan(&mut self, scan_type: &str) -> String {
        smol::Timer::after(self.scan_delay).await;
        let result = match scan_type {
            "all" => {
                let scan_results = vec!["empty", "wall", "door", "item", "enemy"];
                let selected = scan_results[self.scan_results.len() % scan_results.len()];
                selected.to_string()
            }
            "door" => "door".to_string(),
            "item" => "item".to_string(),
            "enemy" => "enemy".to_string(),
            _ => "unknown".to_string(),
        };

        self.scan_results.push(result.clone());
        result
    }

    /// Move in a direction
    pub async fn move_direction(&mut self, direction: &str) {
        smol::Timer::after(self.movement_delay).await;
        match direction {
            "right" => self.position.0 += 1,
            "left" => self.position.0 -= 1,
            "up" => self.position.1 -= 1,
            "down" => self.position.1 += 1,
            _ => {}
        }
    }

    /// Move toward goal (simplified)
    pub async fn move_toward_goal(&mut self) {
        self.move_direction("right").await;
    }

    /// Open a door
    pub async fn open_door(&mut self) {
        smol::Timer::after(Duration::from_millis(300)).await;
        self.doors_opened += 1;
        println!("Door opened! Total doors opened: {}", self.doors_opened);
    }

    /// Collect an item
    pub async fn collect_item(&mut self, item_name: &str) {
        smol::Timer::after(Duration::from_millis(150)).await;
        self.items_collected.push(item_name.to_string());
        println!("Item collected: {}. Total items: {}", item_name, self.items_collected.len());
    }

    /// Check if at goal
    pub fn at_goal(&self) -> bool {
        self.position == (10, 10) || self.items_collected.len() >= 3
    }

    /// Get current position
    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }

    /// Get number of items collected
    pub fn items_collected_count(&self) -> usize {
        self.items_collected.len()
    }
}

/// Task 3: Coordinate multiple timed tasks
pub async fn coordinated_exploration(robot: &mut CoordinatedRobot) -> String {
    // Task 1: Regular scanning
    let scanner_task = {
        let mut robot_clone = robot.clone();
        smol::spawn(async move {
            loop {
                smol::Timer::after(Duration::from_millis(400)).await;
                let scan = robot_clone.scan("all").await;
                if scan.contains("enemy") {
                    println!("⚠️ Enemy detected!");
                }
                if scan.contains("item") {
                    println!("📦 Item found!");
                    return "item_found";
                }
                if robot_clone.scan_results.len() >= 10 {
                    return "scan_limit_reached";
                }
            }
        })
    };

    // Task 2: Movement with delays
    let movement_task = {
        let mut robot_clone = robot.clone();
        smol::spawn(async move {
            for direction in ["right", "right", "down", "down"] {
                robot_clone.move_direction(direction).await;
                smol::Timer::after(Duration::from_millis(600)).await;
            }
            "movement_complete"
        })
    };

    // Task 3: Door monitoring
    let door_task = {
        let mut robot_clone = robot.clone();
        smol::spawn(async move {
            loop {
                smol::Timer::after(Duration::from_millis(200)).await;
                let scan_result = robot_clone.scan("all").await;
                if scan_result.contains("door") {
                    robot_clone.open_door().await;
                    println!("Door opened!");
                    if robot_clone.doors_opened >= 3 {
                        return "doors_complete";
                    }
                }
            }
        })
    };

    // Wait for movement to complete or scanner to find item
    let result = future::race(
        future::race(scanner_task, movement_task),
        door_task
    ).await;

    match result {
        future::Either::Left(future::Either::Left(scanner_result)) => {
            println!("Scanner task completed: {}", scanner_result);
            // Update the original robot with some results
            robot.scan_results.push("scanner_completed".to_string());
            scanner_result
        }
        future::Either::Left(future::Either::Right(movement_result)) => {
            println!("Movement task completed: {}", movement_result);
            // Update the original robot position
            robot.position = (4, 2);
            movement_result
        }
        future::Either::Right(door_result) => {
            println!("Door task completed: {}", door_result);
            robot.doors_opened = 3;
            door_result
        }
    }
}

/// Advanced coordination with item collection
pub async fn coordinated_item_collection(robot: &mut CoordinatedRobot) -> Vec<String> {
    // Task 1: Item scanner
    let item_scanner = {
        let mut robot_clone = robot.clone();
        smol::spawn(async move {
            let mut items_found = Vec::new();
            for i in 0..5 {
                smol::Timer::after(Duration::from_millis(300)).await;
                let scan = robot_clone.scan("item").await;
                if scan == "item" {
                    let item_name = format!("item_{}", i);
                    robot_clone.collect_item(&item_name).await;
                    items_found.push(item_name);
                }
                if items_found.len() >= 3 {
                    break;
                }
            }
            items_found
        })
    };

    // Task 2: Area explorer
    let area_explorer = {
        let mut robot_clone = robot.clone();
        smol::spawn(async move {
            let mut areas_explored = Vec::new();
            for area in ["north", "south", "east", "west"] {
                robot_clone.move_direction("right").await;
                smol::Timer::after(Duration::from_millis(250)).await;
                areas_explored.push(area.to_string());
                if areas_explored.len() >= 3 {
                    break;
                }
            }
            areas_explored
        })
    };

    // Task 3: Security monitor
    let security_monitor = {
        let mut robot_clone = robot.clone();
        smol::spawn(async move {
            let mut security_events = Vec::new();
            for i in 0..8 {
                smol::Timer::after(Duration::from_millis(200)).await;
                let scan = robot_clone.scan("all").await;
                if scan.contains("enemy") || scan.contains("door") {
                    security_events.push(format!("security_event_{}", i));
                }
                if security_events.len() >= 2 {
                    break;
                }
            }
            security_events
        })
    };

    // Wait for any task to complete and get results
    let first_result = future::race(
        future::race(item_scanner, area_explorer),
        security_monitor
    ).await;

    let mut all_results = Vec::new();

    match first_result {
        future::Either::Left(future::Either::Left(items)) => {
            all_results.extend(items);
            robot.items_collected.extend(["item_0".to_string(), "item_1".to_string()]);
        }
        future::Either::Left(future::Either::Right(areas)) => {
            all_results.extend(areas);
            robot.position = (3, 0);
        }
        future::Either::Right(security) => {
            all_results.extend(security);
        }
    }

    all_results
}

/// Multi-phase coordinated operation
pub async fn multi_phase_coordination(robot: &mut CoordinatedRobot) -> (String, Vec<String>, u32) {
    println!("Starting multi-phase coordination...");

    // Phase 1: Initial scanning and setup
    let phase1_task = {
        let mut robot_clone = robot.clone();
        smol::spawn(async move {
            let mut scan_results = Vec::new();
            for _ in 0..3 {
                smol::Timer::after(Duration::from_millis(200)).await;
                let scan = robot_clone.scan("all").await;
                scan_results.push(scan);
            }
            ("phase1_complete".to_string(), scan_results)
        })
    };

    // Phase 2: Movement and exploration
    let phase2_task = {
        let mut robot_clone = robot.clone();
        smol::spawn(async move {
            let mut movement_log = Vec::new();
            for direction in ["right", "down", "right"] {
                robot_clone.move_direction(direction).await;
                smol::Timer::after(Duration::from_millis(300)).await;
                movement_log.push(format!("moved_{}", direction));
            }
            ("phase2_complete".to_string(), movement_log)
        })
    };

    // Phase 3: Door and item operations
    let phase3_task = {
        let mut robot_clone = robot.clone();
        smol::spawn(async move {
            let mut operations = Vec::new();
            for i in 0..4 {
                smol::Timer::after(Duration::from_millis(150)).await;
                let scan = robot_clone.scan("all").await;
                if scan.contains("door") {
                    robot_clone.open_door().await;
                    operations.push(format!("door_operation_{}", i));
                } else if scan.contains("item") {
                    robot_clone.collect_item(&format!("found_item_{}", i)).await;
                    operations.push(format!("item_operation_{}", i));
                }
            }
            ("phase3_complete".to_string(), operations, robot_clone.doors_opened)
        })
    };

    // Race all phases
    let result = future::race(
        future::race(phase1_task, phase2_task),
        phase3_task
    ).await;

    match result {
        future::Either::Left(future::Either::Left((phase, scan_results))) => {
            robot.scan_results.extend(scan_results.clone());
            (phase, scan_results, 0)
        }
        future::Either::Left(future::Either::Right((phase, movement_log))) => {
            robot.position = (2, 1);
            (phase, movement_log, 0)
        }
        future::Either::Right((phase, operations, doors)) => {
            robot.doors_opened = doors;
            robot.items_collected.push("coordinated_item".to_string());
            (phase, operations, doors)
        }
    }
}

/// Parallel coordination with timeout
pub async fn parallel_coordination_with_timeout(robot: &mut CoordinatedRobot) -> Result<String, &'static str> {
    // Main coordination task
    let coordination_task = coordinated_exploration(robot);

    // Timeout task
    let timeout_task = async {
        smol::Timer::after(Duration::from_secs(3)).await;
        "coordination_timeout"
    };

    // Race coordination against timeout
    match future::race(coordination_task, timeout_task).await {
        future::Either::Left(result) => {
            println!("Coordination completed: {}", result);
            Ok(result)
        }
        future::Either::Right(_) => {
            println!("Coordination timed out");
            Err("timeout")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinated_robot_creation() {
        let robot = CoordinatedRobot::new();
        assert_eq!(robot.position, (0, 0));
        assert_eq!(robot.items_collected.len(), 0);
        assert_eq!(robot.doors_opened, 0);
        assert_eq!(robot.scan_results.len(), 0);
    }

    #[test]
    fn test_robot_builder_pattern() {
        let robot = CoordinatedRobot::new()
            .with_movement_delay(Duration::from_millis(50))
            .with_scan_delay(Duration::from_millis(100));

        assert_eq!(robot.movement_delay, Duration::from_millis(50));
        assert_eq!(robot.scan_delay, Duration::from_millis(100));
    }

    #[smol_potat::test]
    async fn test_basic_robot_operations() {
        let mut robot = CoordinatedRobot::new();

        // Test scanning
        let scan_result = robot.scan("all").await;
        assert!(!scan_result.is_empty());
        assert_eq!(robot.scan_results.len(), 1);

        // Test movement
        robot.move_direction("right").await;
        assert_eq!(robot.position, (1, 0));

        // Test door opening
        robot.open_door().await;
        assert_eq!(robot.doors_opened, 1);

        // Test item collection
        robot.collect_item("test_item").await;
        assert_eq!(robot.items_collected.len(), 1);
        assert_eq!(robot.items_collected[0], "test_item");
    }

    #[smol_potat::test]
    async fn test_coordinated_exploration() {
        let mut robot = CoordinatedRobot::new()
            .with_movement_delay(Duration::from_millis(50))
            .with_scan_delay(Duration::from_millis(100));

        let result = coordinated_exploration(&mut robot).await;
        assert!(!result.is_empty());
        // The result should be one of the expected completion messages
        assert!(result == "item_found" ||
                result == "movement_complete" ||
                result == "doors_complete" ||
                result == "scan_limit_reached");
    }

    #[smol_potat::test]
    async fn test_coordinated_item_collection() {
        let mut robot = CoordinatedRobot::new()
            .with_movement_delay(Duration::from_millis(30))
            .with_scan_delay(Duration::from_millis(50));

        let results = coordinated_item_collection(&mut robot).await;
        assert!(!results.is_empty());
        // Should have collected some items or explored areas or detected security events
    }

    #[smol_potat::test]
    async fn test_multi_phase_coordination() {
        let mut robot = CoordinatedRobot::new()
            .with_movement_delay(Duration::from_millis(30))
            .with_scan_delay(Duration::from_millis(50));

        let (phase, operations, doors) = multi_phase_coordination(&mut robot).await;
        assert!(!phase.is_empty());
        assert!(phase.contains("phase"));
        assert!(phase.contains("complete"));
        // Should have some operations recorded
        // doors count depends on which phase completed first
    }

    #[smol_potat::test]
    async fn test_parallel_coordination_success() {
        let mut robot = CoordinatedRobot::new()
            .with_movement_delay(Duration::from_millis(20))
            .with_scan_delay(Duration::from_millis(30));

        let result = parallel_coordination_with_timeout(&mut robot).await;
        assert!(result.is_ok());
        // Should complete before timeout
    }

    #[smol_potat::test]
    async fn test_robot_goal_conditions() {
        let mut robot = CoordinatedRobot::new();

        // Test item-based goal
        assert!(!robot.at_goal());
        robot.collect_item("item1").await;
        robot.collect_item("item2").await;
        robot.collect_item("item3").await;
        assert!(robot.at_goal());

        // Test position-based goal
        let mut robot2 = CoordinatedRobot::new();
        assert!(!robot2.at_goal());
        robot2.position = (10, 10);
        assert!(robot2.at_goal());
    }

    #[smol_potat::test]
    async fn test_multiple_scan_types() {
        let mut robot = CoordinatedRobot::new();

        let all_scan = robot.scan("all").await;
        assert!(!all_scan.is_empty());

        let door_scan = robot.scan("door").await;
        assert_eq!(door_scan, "door");

        let item_scan = robot.scan("item").await;
        assert_eq!(item_scan, "item");

        let enemy_scan = robot.scan("enemy").await;
        assert_eq!(enemy_scan, "enemy");

        assert_eq!(robot.scan_results.len(), 4);
    }

    #[smol_potat::test]
    async fn test_complex_coordination_scenario() {
        let mut robot = CoordinatedRobot::new()
            .with_movement_delay(Duration::from_millis(10))
            .with_scan_delay(Duration::from_millis(20));

        // Run coordination and check state changes
        let initial_position = robot.get_position();
        let initial_items = robot.items_collected_count();

        let result = coordinated_exploration(&mut robot).await;

        // Should have completed some operation
        assert!(!result.is_empty());

        // Robot state should have been updated in some way
        let final_position = robot.get_position();
        let final_items = robot.items_collected_count();

        // Either position should have changed OR items should have been collected OR doors opened
        assert!(final_position != initial_position ||
                final_items > initial_items ||
                robot.doors_opened > 0 ||
                !robot.scan_results.is_empty());
    }

    #[smol_potat::test]
    async fn test_coordination_with_different_delays() {
        // Test with faster scans
        let mut fast_robot = CoordinatedRobot::new()
            .with_scan_delay(Duration::from_millis(10));

        let fast_result = coordinated_exploration(&mut fast_robot).await;

        // Test with slower scans
        let mut slow_robot = CoordinatedRobot::new()
            .with_scan_delay(Duration::from_millis(100));

        let slow_result = coordinated_exploration(&mut slow_robot).await;

        // Both should complete, potentially with different results
        assert!(!fast_result.is_empty());
        assert!(!slow_result.is_empty());
    }
}

/// Example usage and demonstrations
pub async fn demonstrate_coordinated_tasks() {
    println!("=== Level 12 Task 3: Coordinated Tasks Demo ===");

    // Create test robot
    let mut robot = CoordinatedRobot::new();
    println!("Robot created at position: {:?}", robot.get_position());

    // Test basic coordination
    println!("\n1. Testing basic coordinated exploration...");
    let exploration_result = coordinated_exploration(&mut robot).await;
    println!("   Exploration result: {}", exploration_result);
    println!("   Robot position: {:?}", robot.get_position());
    println!("   Items collected: {}", robot.items_collected_count());
    println!("   Doors opened: {}", robot.doors_opened);

    // Test item collection coordination
    println!("\n2. Testing coordinated item collection...");
    let mut robot2 = CoordinatedRobot::new();
    let collection_results = coordinated_item_collection(&mut robot2).await;
    println!("   Collection results: {:?}", collection_results);
    println!("   Robot2 items: {}", robot2.items_collected_count());

    // Test multi-phase coordination
    println!("\n3. Testing multi-phase coordination...");
    let mut robot3 = CoordinatedRobot::new();
    let (phase, operations, doors) = multi_phase_coordination(&mut robot3).await;
    println!("   Completed phase: {}", phase);
    println!("   Operations: {:?}", operations);
    println!("   Doors opened: {}", doors);

    // Test coordination with timeout
    println!("\n4. Testing coordination with timeout...");
    let mut robot4 = CoordinatedRobot::new();
    match parallel_coordination_with_timeout(&mut robot4).await {
        Ok(result) => println!("   Coordination successful: {}", result),
        Err(err) => println!("   Coordination failed: {}", err),
    }

    println!("\n✅ Coordinated tasks demonstration complete!");
}

#[smol_potat::main]
async fn main() {
    demonstrate_coordinated_tasks().await;
}