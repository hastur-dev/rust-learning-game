//! Level 12 Task 5: Parallel Task Racing with Smol
//!
//! This module demonstrates how to create multiple parallel tasks and use racing
//! to coordinate them efficiently, allowing the first successful strategy to win.

use std::time::Duration;
use futures_lite::future;

/// Exploration strategy types
#[derive(Debug, Clone, PartialEq)]
pub enum StrategyType {
    DirectPath,
    ScanAndMove,
    ItemCollection,
    DoorSearch,
    EnergyConservation,
}

impl StrategyType {
    /// Get a description of the strategy
    pub fn description(&self) -> &'static str {
        match self {
            StrategyType::DirectPath => "Move directly to goal",
            StrategyType::ScanAndMove => "Scan thoroughly then move",
            StrategyType::ItemCollection => "Collect items first",
            StrategyType::DoorSearch => "Find and open doors",
            StrategyType::EnergyConservation => "Conserve energy while progressing",
        }
    }
}

/// Result of a strategy execution
#[derive(Debug, Clone)]
pub struct StrategyResult {
    pub strategy: StrategyType,
    pub success: bool,
    pub score: u32,
    pub time_taken: Duration,
    pub items_collected: Vec<String>,
    pub doors_opened: u32,
    pub final_position: (i32, i32),
}

impl StrategyResult {
    pub fn new(strategy: StrategyType) -> Self {
        Self {
            strategy,
            success: false,
            score: 0,
            time_taken: Duration::from_millis(0),
            items_collected: Vec::new(),
            doors_opened: 0,
            final_position: (0, 0),
        }
    }
}

/// Racing robot for parallel strategy testing
#[derive(Debug, Clone)]
pub struct RacingRobot {
    pub position: (i32, i32),
    pub energy: u32,
    pub items_collected: Vec<String>,
    pub doors_opened: u32,
    pub scan_count: u32,
    pub movement_count: u32,
    pub strategy: StrategyType,
}

impl RacingRobot {
    pub fn new(strategy: StrategyType) -> Self {
        Self {
            position: (0, 0),
            energy: 100,
            items_collected: Vec::new(),
            doors_opened: 0,
            scan_count: 0,
            movement_count: 0,
            strategy,
        }
    }

    /// Move in a specific direction
    pub async fn move_direction(&mut self, direction: &str) {
        smol::Timer::after(Duration::from_millis(200)).await;
        self.movement_count += 1;
        self.energy = self.energy.saturating_sub(2);

        match direction {
            "right" => self.position.0 += 1,
            "left" => self.position.0 -= 1,
            "up" => self.position.1 -= 1,
            "down" => self.position.1 += 1,
            _ => {}
        }
    }

    /// Move toward goal position
    pub async fn move_toward_goal(&mut self) {
        if self.position.0 < 9 {
            self.move_direction("right").await;
        } else if self.position.1 < 7 {
            self.move_direction("down").await;
        }
    }

    /// Perform a scan
    pub async fn scan(&mut self, scan_type: &str) -> String {
        let scan_time = match scan_type {
            "quick" => Duration::from_millis(100),
            "thorough" => Duration::from_millis(300),
            "deep" => Duration::from_millis(500),
            _ => Duration::from_millis(200),
        };

        smol::Timer::after(scan_time).await;
        self.scan_count += 1;
        self.energy = self.energy.saturating_sub(1);

        // Simulate scan results based on position and scan count
        let scan_result = match (self.position.0 + self.position.1 + self.scan_count as i32) % 5 {
            0 => "clear_path".to_string(),
            1 => "door_found".to_string(),
            2 => "item_detected".to_string(),
            3 => "enemy_nearby".to_string(),
            _ => "obstacle_ahead".to_string(),
        };

        scan_result
    }

    /// Collect an item
    pub async fn collect_item(&mut self, item_name: &str) {
        smol::Timer::after(Duration::from_millis(150)).await;
        self.items_collected.push(item_name.to_string());
        self.energy = std::cmp::min(100, self.energy + 5);
    }

    /// Open a door
    pub async fn open_door(&mut self) {
        smol::Timer::after(Duration::from_millis(300)).await;
        self.doors_opened += 1;
        self.energy = self.energy.saturating_sub(3);
    }

    /// Check if goal is reached
    pub fn at_goal(&self) -> bool {
        self.position.0 >= 9 && self.position.1 >= 7
    }

    /// Calculate current score
    pub fn calculate_score(&self) -> u32 {
        let mut score = 0;
        score += (self.position.0 + self.position.1) as u32 * 10; // Distance progress
        score += self.items_collected.len() as u32 * 50; // Items bonus
        score += self.doors_opened * 30; // Doors bonus
        score += self.energy / 2; // Energy efficiency bonus
        score
    }
}

/// Strategy 1: Direct path approach
pub async fn direct_path_strategy() -> StrategyResult {
    let start_time = std::time::Instant::now();
    let mut robot = RacingRobot::new(StrategyType::DirectPath);
    let mut result = StrategyResult::new(StrategyType::DirectPath);

    // Direct movement to goal
    for _ in 0..12 {
        robot.move_toward_goal().await;
        if robot.at_goal() {
            result.success = true;
            break;
        }
        if robot.energy == 0 {
            break;
        }
    }

    result.time_taken = start_time.elapsed();
    result.score = robot.calculate_score();
    result.final_position = robot.position;
    result.items_collected = robot.items_collected;
    result.doors_opened = robot.doors_opened;

    result
}

/// Strategy 2: Scan and move approach
pub async fn scan_and_move_strategy() -> StrategyResult {
    let start_time = std::time::Instant::now();
    let mut robot = RacingRobot::new(StrategyType::ScanAndMove);
    let mut result = StrategyResult::new(StrategyType::ScanAndMove);

    // Scan before each move
    for _ in 0..8 {
        let scan_result = robot.scan("thorough").await;

        if scan_result.contains("clear_path") {
            robot.move_toward_goal().await;
        } else if scan_result.contains("item") {
            robot.collect_item(&format!("scanned_item_{}", robot.items_collected.len())).await;
        } else if scan_result.contains("door") {
            robot.open_door().await;
        } else {
            robot.move_toward_goal().await; // Move anyway
        }

        if robot.at_goal() {
            result.success = true;
            break;
        }
        if robot.energy == 0 {
            break;
        }
    }

    result.time_taken = start_time.elapsed();
    result.score = robot.calculate_score();
    result.final_position = robot.position;
    result.items_collected = robot.items_collected;
    result.doors_opened = robot.doors_opened;

    result
}

/// Strategy 3: Item collection focus
pub async fn item_collection_strategy() -> StrategyResult {
    let start_time = std::time::Instant::now();
    let mut robot = RacingRobot::new(StrategyType::ItemCollection);
    let mut result = StrategyResult::new(StrategyType::ItemCollection);

    // Focus on collecting items first
    for i in 0..10 {
        let scan_result = robot.scan("quick").await;

        if scan_result.contains("item") || i % 3 == 0 {
            robot.collect_item(&format!("priority_item_{}", robot.items_collected.len())).await;
        }

        robot.move_toward_goal().await;

        if robot.items_collected.len() >= 3 || robot.at_goal() {
            result.success = true;
            break;
        }
        if robot.energy == 0 {
            break;
        }
    }

    result.time_taken = start_time.elapsed();
    result.score = robot.calculate_score();
    result.final_position = robot.position;
    result.items_collected = robot.items_collected;
    result.doors_opened = robot.doors_opened;

    result
}

/// Strategy 4: Door search strategy
pub async fn door_search_strategy() -> StrategyResult {
    let start_time = std::time::Instant::now();
    let mut robot = RacingRobot::new(StrategyType::DoorSearch);
    let mut result = StrategyResult::new(StrategyType::DoorSearch);

    // Search for and open doors
    for i in 0..8 {
        let scan_result = robot.scan("deep").await;

        if scan_result.contains("door") || i % 2 == 0 {
            robot.open_door().await;
        }

        // Move in a search pattern
        if i % 4 < 2 {
            robot.move_direction("right").await;
        } else {
            robot.move_direction("down").await;
        }

        if robot.doors_opened >= 3 || robot.at_goal() {
            result.success = true;
            break;
        }
        if robot.energy == 0 {
            break;
        }
    }

    result.time_taken = start_time.elapsed();
    result.score = robot.calculate_score();
    result.final_position = robot.position;
    result.items_collected = robot.items_collected;
    result.doors_opened = robot.doors_opened;

    result
}

/// Strategy 5: Energy conservation
pub async fn energy_conservation_strategy() -> StrategyResult {
    let start_time = std::time::Instant::now();
    let mut robot = RacingRobot::new(StrategyType::EnergyConservation);
    let mut result = StrategyResult::new(StrategyType::EnergyConservation);

    // Conservative approach with energy management
    for _ in 0..15 {
        if robot.energy > 80 {
            // High energy: scan and move
            robot.scan("thorough").await;
            robot.move_toward_goal().await;
        } else if robot.energy > 40 {
            // Medium energy: quick actions
            robot.scan("quick").await;
            robot.move_toward_goal().await;
        } else {
            // Low energy: collect items for energy
            robot.collect_item(&format!("energy_item_{}", robot.items_collected.len())).await;
        }

        if robot.at_goal() || robot.energy >= 95 {
            result.success = true;
            break;
        }
        if robot.energy == 0 {
            break;
        }
    }

    result.time_taken = start_time.elapsed();
    result.score = robot.calculate_score();
    result.final_position = robot.position;
    result.items_collected = robot.items_collected;
    result.doors_opened = robot.doors_opened;

    result
}

/// Task 5: Parallel operations with multiple racing strategies
pub async fn parallel_operations() -> StrategyResult {
    println!("Starting parallel racing strategies...");

    // Strategy A: Direct path
    let strategy_a = smol::spawn(direct_path_strategy());

    // Strategy B: Scan and move
    let strategy_b = smol::spawn(scan_and_move_strategy());

    // Strategy C: Item collection
    let strategy_c = smol::spawn(item_collection_strategy());

    // Strategy D: Door search
    let strategy_d = smol::spawn(door_search_strategy());

    // Strategy E: Energy conservation
    let strategy_e = smol::spawn(energy_conservation_strategy());

    // Race all strategies
    let winner = future::race(
        future::race(
            future::race(strategy_a, strategy_b),
            future::race(strategy_c, strategy_d)
        ),
        strategy_e
    ).await;

    let winning_result = match winner {
        future::Either::Left(future::Either::Left(future::Either::Left(result))) => {
            println!("Strategy A (Direct Path) won!");
            result
        }
        future::Either::Left(future::Either::Left(future::Either::Right(result))) => {
            println!("Strategy B (Scan and Move) won!");
            result
        }
        future::Either::Left(future::Either::Right(future::Either::Left(result))) => {
            println!("Strategy C (Item Collection) won!");
            result
        }
        future::Either::Left(future::Either::Right(future::Either::Right(result))) => {
            println!("Strategy D (Door Search) won!");
            result
        }
        future::Either::Right(result) => {
            println!("Strategy E (Energy Conservation) won!");
            result
        }
    };

    winning_result
}

/// Advanced racing with timeout and fallback
pub async fn racing_with_timeout() -> Result<StrategyResult, &'static str> {
    // Main racing operation
    let racing_task = parallel_operations();

    // Timeout task
    let timeout_task = async {
        smol::Timer::after(Duration::from_secs(5)).await;
        StrategyResult::new(StrategyType::DirectPath) // Fallback result
    };

    // Race with timeout
    match future::race(racing_task, timeout_task).await {
        future::Either::Left(result) => {
            println!("Racing completed successfully");
            Ok(result)
        }
        future::Either::Right(_) => {
            println!("Racing timed out");
            Err("timeout")
        }
    }
}

/// Multiple rounds of racing
pub async fn multi_round_racing(rounds: u32) -> Vec<StrategyResult> {
    let mut all_results = Vec::new();

    for round in 1..=rounds {
        println!("=== Round {} ===", round);

        // Add small delay between rounds
        smol::Timer::after(Duration::from_millis(100)).await;

        let result = parallel_operations().await;
        println!("Round {} winner: {:?} (Score: {})", round, result.strategy, result.score);
        all_results.push(result);
    }

    all_results
}

/// Tournament-style racing with elimination
pub async fn tournament_racing() -> StrategyResult {
    println!("Starting tournament-style racing...");

    // Round 1: All strategies
    let semifinal_strategies = vec![
        smol::spawn(direct_path_strategy()),
        smol::spawn(scan_and_move_strategy()),
        smol::spawn(item_collection_strategy()),
    ];

    // Wait for first to complete
    let semifinal_winner = future::race(
        future::race(semifinal_strategies[0], semifinal_strategies[1]),
        semifinal_strategies[2]
    ).await;

    let semifinal_result = match semifinal_winner {
        future::Either::Left(future::Either::Left(result)) => result,
        future::Either::Left(future::Either::Right(result)) => result,
        future::Either::Right(result) => result,
    };

    println!("Semifinal winner: {:?}", semifinal_result.strategy);

    // Round 2: Winner vs special strategies
    let final_strategies = vec![
        smol::spawn(async { semifinal_result }),
        smol::spawn(door_search_strategy()),
        smol::spawn(energy_conservation_strategy()),
    ];

    let final_winner = future::race(
        future::race(final_strategies[0], final_strategies[1]),
        final_strategies[2]
    ).await;

    let tournament_result = match final_winner {
        future::Either::Left(future::Either::Left(result)) => result,
        future::Either::Left(future::Either::Right(result)) => result,
        future::Either::Right(result) => result,
    };

    println!("Tournament winner: {:?}", tournament_result.strategy);
    tournament_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_types() {
        assert_eq!(StrategyType::DirectPath.description(), "Move directly to goal");
        assert_eq!(StrategyType::ScanAndMove.description(), "Scan thoroughly then move");
        assert_eq!(StrategyType::ItemCollection.description(), "Collect items first");
        assert_eq!(StrategyType::DoorSearch.description(), "Find and open doors");
        assert_eq!(StrategyType::EnergyConservation.description(), "Conserve energy while progressing");
    }

    #[test]
    fn test_strategy_result_creation() {
        let result = StrategyResult::new(StrategyType::DirectPath);
        assert_eq!(result.strategy, StrategyType::DirectPath);
        assert!(!result.success);
        assert_eq!(result.score, 0);
        assert_eq!(result.items_collected.len(), 0);
        assert_eq!(result.doors_opened, 0);
        assert_eq!(result.final_position, (0, 0));
    }

    #[test]
    fn test_racing_robot_creation() {
        let robot = RacingRobot::new(StrategyType::ScanAndMove);
        assert_eq!(robot.strategy, StrategyType::ScanAndMove);
        assert_eq!(robot.position, (0, 0));
        assert_eq!(robot.energy, 100);
        assert_eq!(robot.items_collected.len(), 0);
        assert_eq!(robot.doors_opened, 0);
        assert_eq!(robot.scan_count, 0);
        assert_eq!(robot.movement_count, 0);
    }

    #[smol_potat::test]
    async fn test_robot_movement() {
        let mut robot = RacingRobot::new(StrategyType::DirectPath);
        let initial_energy = robot.energy;

        robot.move_direction("right").await;
        assert_eq!(robot.position, (1, 0));
        assert_eq!(robot.movement_count, 1);
        assert!(robot.energy < initial_energy);

        robot.move_direction("down").await;
        assert_eq!(robot.position, (1, 1));
        assert_eq!(robot.movement_count, 2);
    }

    #[smol_potat::test]
    async fn test_robot_scanning() {
        let mut robot = RacingRobot::new(StrategyType::ScanAndMove);
        let initial_energy = robot.energy;

        let scan_result = robot.scan("quick").await;
        assert!(!scan_result.is_empty());
        assert_eq!(robot.scan_count, 1);
        assert!(robot.energy < initial_energy);

        let thorough_scan = robot.scan("thorough").await;
        assert!(!thorough_scan.is_empty());
        assert_eq!(robot.scan_count, 2);
    }

    #[smol_potat::test]
    async fn test_robot_item_collection() {
        let mut robot = RacingRobot::new(StrategyType::ItemCollection);
        let initial_energy = robot.energy;

        robot.collect_item("test_item").await;
        assert_eq!(robot.items_collected.len(), 1);
        assert_eq!(robot.items_collected[0], "test_item");
        assert!(robot.energy >= initial_energy); // Energy should increase
    }

    #[smol_potat::test]
    async fn test_robot_door_opening() {
        let mut robot = RacingRobot::new(StrategyType::DoorSearch);
        let initial_energy = robot.energy;

        robot.open_door().await;
        assert_eq!(robot.doors_opened, 1);
        assert!(robot.energy < initial_energy); // Energy should decrease
    }

    #[smol_potat::test]
    async fn test_robot_goal_detection() {
        let mut robot = RacingRobot::new(StrategyType::DirectPath);
        assert!(!robot.at_goal());

        robot.position = (9, 7);
        assert!(robot.at_goal());

        robot.position = (10, 8);
        assert!(robot.at_goal());

        robot.position = (5, 3);
        assert!(!robot.at_goal());
    }

    #[smol_potat::test]
    async fn test_robot_score_calculation() {
        let mut robot = RacingRobot::new(StrategyType::ItemCollection);
        let initial_score = robot.calculate_score();

        robot.position = (5, 3);
        robot.collect_item("item1").await;
        robot.open_door().await;

        let final_score = robot.calculate_score();
        assert!(final_score > initial_score);
    }

    #[smol_potat::test]
    async fn test_direct_path_strategy() {
        let result = direct_path_strategy().await;
        assert_eq!(result.strategy, StrategyType::DirectPath);
        assert!(result.score > 0);
        assert!(result.time_taken > Duration::from_millis(0));
    }

    #[smol_potat::test]
    async fn test_scan_and_move_strategy() {
        let result = scan_and_move_strategy().await;
        assert_eq!(result.strategy, StrategyType::ScanAndMove);
        assert!(result.score > 0);
        assert!(result.time_taken > Duration::from_millis(0));
    }

    #[smol_potat::test]
    async fn test_item_collection_strategy() {
        let result = item_collection_strategy().await;
        assert_eq!(result.strategy, StrategyType::ItemCollection);
        assert!(result.score > 0);
        assert!(result.items_collected.len() > 0);
    }

    #[smol_potat::test]
    async fn test_door_search_strategy() {
        let result = door_search_strategy().await;
        assert_eq!(result.strategy, StrategyType::DoorSearch);
        assert!(result.score > 0);
        // Should have opened some doors
    }

    #[smol_potat::test]
    async fn test_energy_conservation_strategy() {
        let result = energy_conservation_strategy().await;
        assert_eq!(result.strategy, StrategyType::EnergyConservation);
        assert!(result.score > 0);
        // Energy conservation should result in decent final position
    }

    #[smol_potat::test]
    async fn test_parallel_operations() {
        let result = parallel_operations().await;
        assert!(result.score > 0);
        assert!(result.time_taken > Duration::from_millis(0));
        // One of the strategies should have won
        assert!(matches!(result.strategy,
            StrategyType::DirectPath |
            StrategyType::ScanAndMove |
            StrategyType::ItemCollection |
            StrategyType::DoorSearch |
            StrategyType::EnergyConservation
        ));
    }

    #[smol_potat::test]
    async fn test_racing_with_timeout() {
        let result = racing_with_timeout().await;
        assert!(result.is_ok()); // Should complete before timeout
        let strategy_result = result.unwrap();
        assert!(strategy_result.score > 0);
    }

    #[smol_potat::test]
    async fn test_multi_round_racing() {
        let results = multi_round_racing(3).await;
        assert_eq!(results.len(), 3);

        for result in results {
            assert!(result.score > 0);
            assert!(result.time_taken > Duration::from_millis(0));
        }
    }

    #[smol_potat::test]
    async fn test_tournament_racing() {
        let result = tournament_racing().await;
        assert!(result.score > 0);
        assert!(result.time_taken > Duration::from_millis(0));
        // Tournament winner should be one of the valid strategies
        assert!(matches!(result.strategy,
            StrategyType::DirectPath |
            StrategyType::ScanAndMove |
            StrategyType::ItemCollection |
            StrategyType::DoorSearch |
            StrategyType::EnergyConservation
        ));
    }

    #[smol_potat::test]
    async fn test_move_toward_goal() {
        let mut robot = RacingRobot::new(StrategyType::DirectPath);
        let initial_pos = robot.position;

        robot.move_toward_goal().await;
        assert_ne!(robot.position, initial_pos);

        // Should prioritize rightward movement first
        assert!(robot.position.0 > initial_pos.0 || robot.position.1 > initial_pos.1);
    }
}

/// Example usage and demonstrations
pub async fn demonstrate_parallel_racing() {
    println!("=== Level 12 Task 5: Parallel Racing Demo ===");

    // Test individual strategies
    println!("\n1. Testing individual strategies...");

    let direct_result = direct_path_strategy().await;
    println!("   Direct Path: Score {} in {:?}", direct_result.score, direct_result.time_taken);

    let scan_result = scan_and_move_strategy().await;
    println!("   Scan & Move: Score {} in {:?}", scan_result.score, scan_result.time_taken);

    let item_result = item_collection_strategy().await;
    println!("   Item Collection: Score {} in {:?} ({} items)",
             item_result.score, item_result.time_taken, item_result.items_collected.len());

    // Test parallel racing
    println!("\n2. Testing parallel racing...");
    let racing_result = parallel_operations().await;
    println!("   Winner: {:?}", racing_result.strategy);
    println!("   Score: {}", racing_result.score);
    println!("   Time: {:?}", racing_result.time_taken);
    println!("   Position: {:?}", racing_result.final_position);

    // Test racing with timeout
    println!("\n3. Testing racing with timeout...");
    match racing_with_timeout().await {
        Ok(result) => {
            println!("   Racing completed: {:?} with score {}", result.strategy, result.score);
        }
        Err(err) => {
            println!("   Racing failed: {}", err);
        }
    }

    // Test multi-round racing
    println!("\n4. Testing multi-round racing...");
    let multi_results = multi_round_racing(3).await;
    let mut strategy_wins = std::collections::HashMap::new();
    for result in &multi_results {
        *strategy_wins.entry(result.strategy.clone()).or_insert(0) += 1;
    }
    println!("   Strategy wins: {:?}", strategy_wins);

    // Test tournament racing
    println!("\n5. Testing tournament racing...");
    let tournament_result = tournament_racing().await;
    println!("   Tournament champion: {:?}", tournament_result.strategy);
    println!("   Champion score: {}", tournament_result.score);

    println!("\n✅ Parallel racing demonstration complete!");
}

#[smol_potat::main]
async fn main() {
    demonstrate_parallel_racing().await;
}