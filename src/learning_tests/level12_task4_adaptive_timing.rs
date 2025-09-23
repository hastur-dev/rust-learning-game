//! Level 12 Task 4: Adaptive Timing System with Smol
//!
//! This module demonstrates how to create systems that adapt their timing behavior
//! based on environmental conditions and dynamic requirements using Smol runtime.

use std::time::Duration;
use futures_lite::future;

/// Robot operating modes that affect timing
#[derive(Debug, Clone, PartialEq)]
pub enum OperatingMode {
    Efficient,    // Slow, power-saving
    Normal,       // Balanced performance
    Alert,        // Fast response to threats
    Emergency,    // Maximum speed
}

impl OperatingMode {
    /// Get the reaction time for this mode
    pub fn reaction_time(&self) -> Duration {
        match self {
            OperatingMode::Efficient => Duration::from_millis(500),
            OperatingMode::Normal => Duration::from_millis(200),
            OperatingMode::Alert => Duration::from_millis(100),
            OperatingMode::Emergency => Duration::from_millis(50),
        }
    }

    /// Get the scan interval for this mode
    pub fn scan_interval(&self) -> Duration {
        match self {
            OperatingMode::Efficient => Duration::from_millis(800),
            OperatingMode::Normal => Duration::from_millis(400),
            OperatingMode::Alert => Duration::from_millis(200),
            OperatingMode::Emergency => Duration::from_millis(100),
        }
    }
}

/// Environmental conditions that affect robot behavior
#[derive(Debug, Clone)]
pub struct EnvironmentState {
    pub enemies_nearby: bool,
    pub doors_present: bool,
    pub items_available: bool,
    pub energy_level: u32,
    pub threat_level: u32,
}

impl EnvironmentState {
    pub fn new() -> Self {
        Self {
            enemies_nearby: false,
            doors_present: false,
            items_available: false,
            energy_level: 100,
            threat_level: 0,
        }
    }

    /// Determine the appropriate operating mode based on environment
    pub fn determine_mode(&self) -> OperatingMode {
        if self.threat_level >= 80 || (self.enemies_nearby && self.energy_level > 20) {
            OperatingMode::Emergency
        } else if self.enemies_nearby || self.threat_level >= 40 {
            OperatingMode::Alert
        } else if self.energy_level < 30 {
            OperatingMode::Efficient
        } else {
            OperatingMode::Normal
        }
    }
}

/// Adaptive robot that changes behavior based on conditions
#[derive(Debug, Clone)]
pub struct AdaptiveRobot {
    pub position: (i32, i32),
    pub mode: OperatingMode,
    pub environment: EnvironmentState,
    pub doors_opened: u32,
    pub items_collected: Vec<String>,
    pub scan_count: u32,
    pub mode_changes: Vec<(OperatingMode, Duration)>,
}

impl AdaptiveRobot {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            mode: OperatingMode::Normal,
            environment: EnvironmentState::new(),
            doors_opened: 0,
            items_collected: Vec::new(),
            scan_count: 0,
            mode_changes: Vec::new(),
        }
    }

    /// Perform an adaptive scan that changes behavior based on results
    pub async fn adaptive_scan(&mut self) -> String {
        let scan_interval = self.mode.scan_interval();
        smol::Timer::after(scan_interval).await;

        self.scan_count += 1;

        // Simulate scan results based on scan count to create variety
        let scan_result = match self.scan_count % 6 {
            1 => "empty_area".to_string(),
            2 => "door_detected".to_string(),
            3 => "item_nearby".to_string(),
            4 => "enemy_contact".to_string(),
            5 => "multiple_threats".to_string(),
            _ => "all_clear".to_string(),
        };

        // Update environment based on scan
        self.update_environment_from_scan(&scan_result);

        scan_result
    }

    /// Update environment state based on scan results
    fn update_environment_from_scan(&mut self, scan_result: &str) {
        if scan_result.contains("enemy") || scan_result.contains("threat") {
            self.environment.enemies_nearby = true;
            if scan_result.contains("multiple") {
                self.environment.threat_level = 90;
            } else {
                self.environment.threat_level = 60;
            }
        } else {
            self.environment.enemies_nearby = false;
            if self.environment.threat_level > 0 {
                self.environment.threat_level = self.environment.threat_level.saturating_sub(20);
            }
        }

        if scan_result.contains("door") {
            self.environment.doors_present = true;
        }

        if scan_result.contains("item") {
            self.environment.items_available = true;
        }

        // Energy decreases with each scan
        self.environment.energy_level = self.environment.energy_level.saturating_sub(5);
    }

    /// Adapt the operating mode based on current environment
    pub fn adapt_mode(&mut self) -> bool {
        let new_mode = self.environment.determine_mode();
        if new_mode != self.mode {
            println!("Mode changed from {:?} to {:?}", self.mode, new_mode);
            self.mode_changes.push((new_mode.clone(), self.mode.reaction_time()));
            self.mode = new_mode;
            true
        } else {
            false
        }
    }

    /// React to environment with adaptive timing
    pub async fn adaptive_react(&mut self) {
        let reaction_time = self.mode.reaction_time();
        smol::Timer::after(reaction_time).await;

        if self.environment.doors_present {
            self.open_door().await;
        } else if self.environment.items_available {
            self.collect_item().await;
        } else if self.environment.enemies_nearby {
            self.evasive_action().await;
        } else {
            self.move_toward_goal().await;
        }
    }

    /// Open a door
    pub async fn open_door(&mut self) {
        smol::Timer::after(Duration::from_millis(300)).await;
        self.doors_opened += 1;
        self.environment.doors_present = false;
        println!("Door opened! Total: {}", self.doors_opened);
    }

    /// Collect an item
    pub async fn collect_item(&mut self) {
        smol::Timer::after(Duration::from_millis(200)).await;
        let item_name = format!("item_{}", self.items_collected.len() + 1);
        self.items_collected.push(item_name.clone());
        self.environment.items_available = false;
        self.environment.energy_level = std::cmp::min(100, self.environment.energy_level + 10);
        println!("Collected {}! Energy restored to {}", item_name, self.environment.energy_level);
    }

    /// Take evasive action
    pub async fn evasive_action(&mut self) {
        let evasive_time = match self.mode {
            OperatingMode::Emergency => Duration::from_millis(50),
            OperatingMode::Alert => Duration::from_millis(100),
            _ => Duration::from_millis(200),
        };
        smol::Timer::after(evasive_time).await;
        self.position.0 += 1; // Move away
        println!("Evasive action taken! New position: {:?}", self.position);
    }

    /// Move toward goal
    pub async fn move_toward_goal(&mut self) {
        let move_time = self.mode.reaction_time();
        smol::Timer::after(move_time).await;
        self.position.0 += 1;
        self.position.1 += 1;
    }

    /// Check if at goal
    pub fn at_goal(&self) -> bool {
        self.position.0 >= 10 && self.position.1 >= 10
    }
}

/// Task 4: Implement adaptive robot behavior
pub async fn adaptive_robot_behavior(robot: &mut AdaptiveRobot) -> Vec<String> {
    let mut behavior_log = Vec::new();

    loop {
        // Scan environment
        let scan_result = robot.adaptive_scan().await;
        behavior_log.push(format!("SCAN: {}", scan_result));

        // Adapt mode based on scan results
        let mode_changed = robot.adapt_mode();
        if mode_changed {
            behavior_log.push(format!("MODE: Changed to {:?}", robot.mode));
        }

        // React with adaptive timing
        robot.adaptive_react().await;
        behavior_log.push(format!("ACTION: Reacted in {:?} mode", robot.mode));

        // Break conditions
        if robot.at_goal() {
            behavior_log.push("COMPLETE: Reached goal".to_string());
            break;
        }

        if robot.scan_count >= 15 {
            behavior_log.push("COMPLETE: Scan limit reached".to_string());
            break;
        }

        if robot.environment.energy_level == 0 {
            behavior_log.push("COMPLETE: Energy depleted".to_string());
            break;
        }
    }

    behavior_log
}

/// Advanced adaptive system with multiple adaptation strategies
pub async fn multi_strategy_adaptation(robot: &mut AdaptiveRobot) -> (Vec<String>, u32, usize) {
    let mut adaptation_log = Vec::new();
    let start_time = std::time::Instant::now();

    // Strategy 1: Energy management
    let energy_task = {
        let mut robot_clone = robot.clone();
        smol::spawn(async move {
            let mut energy_events = Vec::new();
            while robot_clone.environment.energy_level > 20 {
                smol::Timer::after(Duration::from_millis(300)).await;
                robot_clone.environment.energy_level = robot_clone.environment.energy_level.saturating_sub(10);
                energy_events.push(format!("Energy: {}", robot_clone.environment.energy_level));

                if robot_clone.environment.energy_level <= 30 {
                    robot_clone.mode = OperatingMode::Efficient;
                    energy_events.push("Switched to efficient mode".to_string());
                    break;
                }
            }
            energy_events
        })
    };

    // Strategy 2: Threat monitoring
    let threat_task = {
        let mut robot_clone = robot.clone();
        smol::spawn(async move {
            let mut threat_events = Vec::new();
            for i in 0..10 {
                smol::Timer::after(Duration::from_millis(200)).await;
                let scan = robot_clone.adaptive_scan().await;
                threat_events.push(format!("Threat scan {}: {}", i, scan));

                if scan.contains("enemy") {
                    robot_clone.mode = OperatingMode::Alert;
                    threat_events.push("Switched to alert mode".to_string());
                    break;
                }
            }
            threat_events
        })
    };

    // Strategy 3: Goal seeking
    let goal_task = {
        let mut robot_clone = robot.clone();
        smol::spawn(async move {
            let mut goal_events = Vec::new();
            while !robot_clone.at_goal() {
                smol::Timer::after(robot_clone.mode.reaction_time()).await;
                robot_clone.move_toward_goal().await;
                goal_events.push(format!("Goal progress: {:?}", robot_clone.position));

                if robot_clone.position.0 + robot_clone.position.1 >= 15 {
                    break;
                }
            }
            goal_events
        })
    };

    // Race all strategies
    let result = future::race(
        future::race(energy_task, threat_task),
        goal_task
    ).await;

    let completed_events = match result {
        future::Either::Left(future::Either::Left(energy_events)) => {
            adaptation_log.extend(energy_events);
            robot.mode = OperatingMode::Efficient;
            "energy_strategy"
        }
        future::Either::Left(future::Either::Right(threat_events)) => {
            adaptation_log.extend(threat_events);
            robot.mode = OperatingMode::Alert;
            "threat_strategy"
        }
        future::Either::Right(goal_events) => {
            adaptation_log.extend(goal_events);
            robot.position = (8, 7);
            "goal_strategy"
        }
    };

    let elapsed_ms = start_time.elapsed().as_millis() as u32;
    adaptation_log.push(format!("Winning strategy: {}", completed_events));

    (adaptation_log, elapsed_ms, robot.mode_changes.len())
}

/// Real-time adaptive coordination system
pub async fn realtime_adaptive_coordination(robot: &mut AdaptiveRobot) -> String {
    // Primary adaptive loop
    let primary_loop = async {
        let mut iterations = 0;
        loop {
            let scan = robot.adaptive_scan().await;
            robot.adapt_mode();
            robot.adaptive_react().await;

            iterations += 1;
            if iterations >= 8 || robot.at_goal() {
                return format!("primary_completed_{}_iterations", iterations);
            }
        }
    };

    // Emergency override system
    let emergency_override = async {
        smol::Timer::after(Duration::from_millis(1500)).await;
        robot.mode = OperatingMode::Emergency;
        robot.environment.threat_level = 95;
        "emergency_override_activated"
    };

    // Energy monitoring system
    let energy_monitor = async {
        loop {
            smol::Timer::after(Duration::from_millis(400)).await;
            robot.environment.energy_level = robot.environment.energy_level.saturating_sub(15);
            if robot.environment.energy_level <= 25 {
                robot.mode = OperatingMode::Efficient;
                return "energy_critical_mode_activated";
            }
        }
    };

    // Race all systems
    match future::race(
        future::race(primary_loop, emergency_override),
        energy_monitor
    ).await {
        future::Either::Left(future::Either::Left(primary_result)) => primary_result,
        future::Either::Left(future::Either::Right(emergency_result)) => emergency_result,
        future::Either::Right(energy_result) => energy_result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operating_mode_timings() {
        assert_eq!(OperatingMode::Emergency.reaction_time(), Duration::from_millis(50));
        assert_eq!(OperatingMode::Alert.reaction_time(), Duration::from_millis(100));
        assert_eq!(OperatingMode::Normal.reaction_time(), Duration::from_millis(200));
        assert_eq!(OperatingMode::Efficient.reaction_time(), Duration::from_millis(500));
    }

    #[test]
    fn test_environment_mode_determination() {
        let mut env = EnvironmentState::new();
        assert_eq!(env.determine_mode(), OperatingMode::Normal);

        env.enemies_nearby = true;
        assert_eq!(env.determine_mode(), OperatingMode::Alert);

        env.threat_level = 85;
        assert_eq!(env.determine_mode(), OperatingMode::Emergency);

        env.enemies_nearby = false;
        env.threat_level = 0;
        env.energy_level = 25;
        assert_eq!(env.determine_mode(), OperatingMode::Efficient);
    }

    #[test]
    fn test_adaptive_robot_creation() {
        let robot = AdaptiveRobot::new();
        assert_eq!(robot.position, (0, 0));
        assert_eq!(robot.mode, OperatingMode::Normal);
        assert_eq!(robot.doors_opened, 0);
        assert_eq!(robot.items_collected.len(), 0);
        assert_eq!(robot.scan_count, 0);
    }

    #[smol_potat::test]
    async fn test_adaptive_scan_updates_environment() {
        let mut robot = AdaptiveRobot::new();

        // First scan
        let scan1 = robot.adaptive_scan().await;
        assert!(!scan1.is_empty());
        assert_eq!(robot.scan_count, 1);
        assert_eq!(robot.environment.energy_level, 95); // Decreased by 5

        // Scan that should detect enemy
        while !robot.environment.enemies_nearby && robot.scan_count < 10 {
            robot.adaptive_scan().await;
        }

        // Should have detected some environment change
        assert!(robot.environment.threat_level > 0 || robot.environment.doors_present || robot.environment.items_available);
    }

    #[smol_potat::test]
    async fn test_mode_adaptation() {
        let mut robot = AdaptiveRobot::new();
        assert_eq!(robot.mode, OperatingMode::Normal);

        // Force enemy detection
        robot.environment.enemies_nearby = true;
        robot.environment.threat_level = 60;

        let changed = robot.adapt_mode();
        assert!(changed);
        assert_eq!(robot.mode, OperatingMode::Alert);

        // Force emergency mode
        robot.environment.threat_level = 85;
        let changed2 = robot.adapt_mode();
        assert!(changed2);
        assert_eq!(robot.mode, OperatingMode::Emergency);
    }

    #[smol_potat::test]
    async fn test_adaptive_reactions() {
        let mut robot = AdaptiveRobot::new();

        // Test door reaction
        robot.environment.doors_present = true;
        robot.adaptive_react().await;
        assert_eq!(robot.doors_opened, 1);
        assert!(!robot.environment.doors_present);

        // Test item collection
        robot.environment.items_available = true;
        robot.adaptive_react().await;
        assert_eq!(robot.items_collected.len(), 1);
        assert!(!robot.environment.items_available);

        // Test evasive action
        robot.environment.enemies_nearby = true;
        let initial_pos = robot.position;
        robot.adaptive_react().await;
        assert_ne!(robot.position, initial_pos);
    }

    #[smol_potat::test]
    async fn test_adaptive_robot_behavior() {
        let mut robot = AdaptiveRobot::new();
        let behavior_log = adaptive_robot_behavior(&mut robot).await;

        assert!(!behavior_log.is_empty());
        assert!(behavior_log.iter().any(|log| log.contains("SCAN")));
        assert!(behavior_log.iter().any(|log| log.contains("ACTION")));
        assert!(behavior_log.iter().any(|log| log.contains("COMPLETE")));
    }

    #[smol_potat::test]
    async fn test_multi_strategy_adaptation() {
        let mut robot = AdaptiveRobot::new();
        let (adaptation_log, elapsed_ms, mode_changes) = multi_strategy_adaptation(&mut robot).await;

        assert!(!adaptation_log.is_empty());
        assert!(elapsed_ms > 0);
        // Mode changes depend on which strategy won
        assert!(adaptation_log.iter().any(|log| log.contains("strategy")));
    }

    #[smol_potat::test]
    async fn test_realtime_adaptive_coordination() {
        let mut robot = AdaptiveRobot::new();
        let result = realtime_adaptive_coordination(&mut robot).await;

        assert!(!result.is_empty());
        assert!(result.contains("completed") ||
                result.contains("override") ||
                result.contains("critical"));
    }

    #[smol_potat::test]
    async fn test_energy_level_effects() {
        let mut robot = AdaptiveRobot::new();

        // Reduce energy to critical level
        robot.environment.energy_level = 25;
        robot.adapt_mode();
        assert_eq!(robot.mode, OperatingMode::Efficient);

        // Collect item should restore energy
        robot.environment.items_available = true;
        robot.adaptive_react().await;
        assert!(robot.environment.energy_level > 25);
    }

    #[smol_potat::test]
    async fn test_goal_reaching() {
        let mut robot = AdaptiveRobot::new();
        assert!(!robot.at_goal());

        robot.position = (10, 10);
        assert!(robot.at_goal());

        robot.position = (5, 8);
        assert!(!robot.at_goal());

        robot.position = (12, 15);
        assert!(robot.at_goal());
    }

    #[smol_potat::test]
    async fn test_mode_change_tracking() {
        let mut robot = AdaptiveRobot::new();
        assert_eq!(robot.mode_changes.len(), 0);

        robot.environment.enemies_nearby = true;
        robot.adapt_mode();
        assert_eq!(robot.mode_changes.len(), 1);

        robot.environment.threat_level = 90;
        robot.adapt_mode();
        assert_eq!(robot.mode_changes.len(), 2);

        // No change should not add to tracking
        robot.adapt_mode();
        assert_eq!(robot.mode_changes.len(), 2);
    }
}

/// Example usage and demonstrations
pub async fn demonstrate_adaptive_timing() {
    println!("=== Level 12 Task 4: Adaptive Timing Demo ===");

    // Create adaptive robot
    let mut robot = AdaptiveRobot::new();
    println!("Robot created in {:?} mode at {:?}", robot.mode, robot.position);

    // Test basic adaptive behavior
    println!("\n1. Testing basic adaptive behavior...");
    let behavior_log = adaptive_robot_behavior(&mut robot).await;
    for (i, log_entry) in behavior_log.iter().take(5).enumerate() {
        println!("   {}: {}", i + 1, log_entry);
    }
    if behavior_log.len() > 5 {
        println!("   ... and {} more entries", behavior_log.len() - 5);
    }
    println!("   Final mode: {:?}", robot.mode);
    println!("   Final position: {:?}", robot.position);

    // Test multi-strategy adaptation
    println!("\n2. Testing multi-strategy adaptation...");
    let mut robot2 = AdaptiveRobot::new();
    let (adaptation_log, elapsed_ms, mode_changes) = multi_strategy_adaptation(&mut robot2).await;
    println!("   Adaptation completed in {}ms", elapsed_ms);
    println!("   Mode changes: {}", mode_changes);
    for (i, log_entry) in adaptation_log.iter().take(3).enumerate() {
        println!("   {}: {}", i + 1, log_entry);
    }

    // Test realtime coordination
    println!("\n3. Testing realtime adaptive coordination...");
    let mut robot3 = AdaptiveRobot::new();
    let coordination_result = realtime_adaptive_coordination(&mut robot3).await;
    println!("   Coordination result: {}", coordination_result);
    println!("   Final mode: {:?}", robot3.mode);
    println!("   Energy level: {}", robot3.environment.energy_level);

    // Demonstrate mode changes
    println!("\n4. Demonstrating mode adaptation...");
    let mut robot4 = AdaptiveRobot::new();
    println!("   Initial mode: {:?}", robot4.mode);

    robot4.environment.enemies_nearby = true;
    robot4.adapt_mode();
    println!("   After enemy detection: {:?}", robot4.mode);

    robot4.environment.threat_level = 85;
    robot4.adapt_mode();
    println!("   After high threat: {:?}", robot4.mode);

    robot4.environment.enemies_nearby = false;
    robot4.environment.threat_level = 0;
    robot4.environment.energy_level = 20;
    robot4.adapt_mode();
    println!("   After low energy: {:?}", robot4.mode);

    println!("\n✅ Adaptive timing demonstration complete!");
}

#[smol_potat::main]
async fn main() {
    demonstrate_adaptive_timing().await;
}