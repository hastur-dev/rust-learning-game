//! Level 13 Task 2: Shared State Management with Smol
//!
//! This module demonstrates how to create shared state that multiple tasks can
//! safely access and modify using Arc<Mutex<T>> and other synchronization primitives.

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use std::time::Duration;
use futures_lite::future;

/// Robot state that can be shared between multiple tasks
#[derive(Clone, Debug, PartialEq)]
pub struct RobotState {
    pub position: (i32, i32),
    pub energy: u32,
    pub items_collected: Vec<String>,
    pub doors_opened: u32,
    pub scan_count: u32,
    pub operational_mode: OperationalMode,
    pub last_update: u64,
}

impl RobotState {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            energy: 100,
            items_collected: Vec::new(),
            doors_opened: 0,
            scan_count: 0,
            operational_mode: OperationalMode::Normal,
            last_update: 0,
        }
    }

    /// Calculate the robot's current efficiency score
    pub fn efficiency_score(&self) -> f32 {
        let position_score = (self.position.0 + self.position.1) as f32 * 5.0;
        let energy_score = self.energy as f32 * 0.5;
        let items_score = self.items_collected.len() as f32 * 20.0;
        let doors_score = self.doors_opened as f32 * 15.0;
        let scan_efficiency = if self.scan_count > 0 {
            100.0 / self.scan_count as f32
        } else {
            0.0
        };

        position_score + energy_score + items_score + doors_score + scan_efficiency
    }
}

/// Operational modes for the robot
#[derive(Clone, Debug, PartialEq)]
pub enum OperationalMode {
    Normal,
    PowerSave,
    HighPerformance,
    Emergency,
    Maintenance,
}

impl OperationalMode {
    /// Get the energy consumption rate for this mode
    pub fn energy_consumption_rate(&self) -> u32 {
        match self {
            OperationalMode::PowerSave => 1,
            OperationalMode::Normal => 2,
            OperationalMode::HighPerformance => 4,
            OperationalMode::Emergency => 6,
            OperationalMode::Maintenance => 0,
        }
    }
}

/// Type alias for shared robot state
pub type SharedState = Arc<Mutex<RobotState>>;

/// Type alias for read-write shared state
pub type SharedStateRW = Arc<RwLock<RobotState>>;

/// System metrics that can be shared between tasks
#[derive(Clone, Debug, Default)]
pub struct SystemMetrics {
    pub task_executions: HashMap<String, u32>,
    pub total_operations: u32,
    pub error_count: u32,
    pub uptime_seconds: u64,
    pub performance_metrics: HashMap<String, f32>,
}

impl SystemMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a task execution
    pub fn record_task(&mut self, task_name: &str) {
        *self.task_executions.entry(task_name.to_string()).or_insert(0) += 1;
        self.total_operations += 1;
    }

    /// Record an error
    pub fn record_error(&mut self) {
        self.error_count += 1;
    }

    /// Set a performance metric
    pub fn set_metric(&mut self, name: &str, value: f32) {
        self.performance_metrics.insert(name.to_string(), value);
    }

    /// Get error rate
    pub fn error_rate(&self) -> f32 {
        if self.total_operations > 0 {
            self.error_count as f32 / self.total_operations as f32
        } else {
            0.0
        }
    }
}

/// Type alias for shared metrics
pub type SharedMetrics = Arc<Mutex<SystemMetrics>>;

/// Task 2: Position updater that modifies shared state
pub async fn position_updater(state: SharedState) -> u32 {
    let mut updates = 0;

    for i in 0..8 {
        smol::Timer::after(Duration::from_millis(200)).await;

        // Simulate getting new position (simple movement pattern)
        let new_pos = (i % 4, i / 2);

        {
            let mut robot_state = state.lock().unwrap();
            robot_state.position = new_pos;
            robot_state.energy = robot_state.energy.saturating_sub(
                robot_state.operational_mode.energy_consumption_rate()
            );
            robot_state.last_update = i as u64;
            updates += 1;
        }

        println!("Position updater: Moved to {:?}", new_pos);

        // Check if robot reached goal or energy depleted
        let should_stop = {
            let robot_state = state.lock().unwrap();
            robot_state.position.0 >= 3 && robot_state.position.1 >= 1
        };

        if should_stop {
            println!("Position updater: Goal reached, stopping");
            break;
        }
    }

    println!("Position updater completed {} updates", updates);
    updates
}

/// Scan counter that tracks scanning activity in shared state
pub async fn scan_counter(state: SharedState) -> u32 {
    let mut scans_performed = 0;

    for i in 0..10 {
        smol::Timer::after(Duration::from_millis(500)).await;

        // Simulate scan operation
        let scan_result = match i % 4 {
            0 => "clear",
            1 => "obstacle",
            2 => "item",
            _ => "enemy",
        };

        {
            let mut robot_state = state.lock().unwrap();
            robot_state.scan_count += 1;
            scans_performed += 1;

            // React to scan results
            match scan_result {
                "item" => {
                    robot_state.items_collected.push(format!("item_{}", i));
                    robot_state.energy = std::cmp::min(100, robot_state.energy + 5);
                }
                "enemy" => {
                    robot_state.operational_mode = OperationalMode::Emergency;
                }
                "obstacle" => {
                    robot_state.operational_mode = OperationalMode::Normal;
                }
                _ => {}
            }

            println!("Scan counter: Scan {} - {} (Total scans: {})",
                     i + 1, scan_result, robot_state.scan_count);
        }

        // Check for completion condition
        let should_stop = {
            let robot_state = state.lock().unwrap();
            robot_state.energy == 0 || robot_state.items_collected.len() >= 3
        };

        if should_stop {
            println!("Scan counter: Stopping condition met");
            break;
        }
    }

    println!("Scan counter completed {} scans", scans_performed);
    scans_performed
}

/// Energy manager that monitors and adjusts energy levels
pub async fn energy_manager(state: SharedState) -> u32 {
    let mut energy_adjustments = 0;

    for _ in 0..12 {
        smol::Timer::after(Duration::from_millis(300)).await;

        let current_energy = {
            let robot_state = state.lock().unwrap();
            robot_state.energy
        };

        // Make energy management decisions
        if current_energy < 20 {
            let mut robot_state = state.lock().unwrap();
            robot_state.operational_mode = OperationalMode::PowerSave;
            robot_state.energy = std::cmp::min(100, robot_state.energy + 10);
            energy_adjustments += 1;
            println!("Energy manager: Low energy, switching to power save mode");
        } else if current_energy > 80 {
            let mut robot_state = state.lock().unwrap();
            robot_state.operational_mode = OperationalMode::HighPerformance;
            energy_adjustments += 1;
            println!("Energy manager: High energy, switching to high performance mode");
        }

        // Check termination condition
        let should_stop = {
            let robot_state = state.lock().unwrap();
            robot_state.position.0 >= 3 && robot_state.position.1 >= 1
        };

        if should_stop {
            break;
        }
    }

    println!("Energy manager completed {} adjustments", energy_adjustments);
    energy_adjustments
}

/// Door operator that manages door operations
pub async fn door_operator(state: SharedState) -> u32 {
    let mut doors_opened = 0;

    for i in 0..6 {
        smol::Timer::after(Duration::from_millis(400)).await;

        // Simulate door detection and opening
        let door_detected = i % 3 == 0; // Every third iteration

        if door_detected {
            smol::Timer::after(Duration::from_millis(300)).await; // Door opening time

            {
                let mut robot_state = state.lock().unwrap();
                robot_state.doors_opened += 1;
                robot_state.energy = robot_state.energy.saturating_sub(3);
                doors_opened += 1;
            }

            println!("Door operator: Opened door {} (Total: {})", doors_opened, doors_opened);
        }

        // Check energy condition
        let should_stop = {
            let robot_state = state.lock().unwrap();
            robot_state.energy < 10
        };

        if should_stop {
            println!("Door operator: Low energy, stopping");
            break;
        }
    }

    println!("Door operator completed, opened {} doors", doors_opened);
    doors_opened
}

/// Metrics collector that tracks system performance
pub async fn metrics_collector(state: SharedState, metrics: SharedMetrics) -> u32 {
    let mut collections = 0;

    for i in 0..8 {
        smol::Timer::after(Duration::from_millis(600)).await;

        // Collect current state metrics
        let (efficiency, energy, items, position) = {
            let robot_state = state.lock().unwrap();
            (
                robot_state.efficiency_score(),
                robot_state.energy,
                robot_state.items_collected.len(),
                robot_state.position,
            )
        };

        // Update metrics
        {
            let mut sys_metrics = metrics.lock().unwrap();
            sys_metrics.record_task("metrics_collection");
            sys_metrics.set_metric("efficiency", efficiency);
            sys_metrics.set_metric("energy_level", energy as f32);
            sys_metrics.set_metric("items_collected", items as f32);
            sys_metrics.uptime_seconds = i as u64;
            collections += 1;
        }

        println!("Metrics collector: Efficiency {:.1}, Energy {}%, Items {}, Position {:?}",
                 efficiency, energy, items, position);

        // Random error simulation
        if i % 5 == 4 {
            let mut sys_metrics = metrics.lock().unwrap();
            sys_metrics.record_error();
            println!("Metrics collector: Simulated error recorded");
        }
    }

    println!("Metrics collector completed {} collections", collections);
    collections
}

/// Comprehensive shared state coordination test
pub async fn comprehensive_shared_state_test() -> (u32, SystemMetrics) {
    let shared_state = Arc::new(Mutex::new(RobotState::new()));
    let shared_metrics = Arc::new(Mutex::new(SystemMetrics::new()));

    println!("Starting comprehensive shared state test...");

    // Launch all tasks concurrently
    let position_task = smol::spawn(position_updater(shared_state.clone()));
    let scan_task = smol::spawn(scan_counter(shared_state.clone()));
    let energy_task = smol::spawn(energy_manager(shared_state.clone()));
    let door_task = smol::spawn(door_operator(shared_state.clone()));
    let metrics_task = smol::spawn(metrics_collector(shared_state.clone(), shared_metrics.clone()));

    // Wait for all tasks to complete
    let (position_updates, scans, energy_adjustments, doors, collections) = future::join(
        future::join(position_task, scan_task),
        future::join(future::join(energy_task, door_task), metrics_task)
    ).await;

    let (position_updates, scans) = position_updates;
    let ((energy_adjustments, doors), collections) = energy_adjustments;

    // Get final state
    let final_state = {
        let state = shared_state.lock().unwrap();
        state.clone()
    };

    let final_metrics = {
        let metrics = shared_metrics.lock().unwrap();
        metrics.clone()
    };

    println!("\n=== Final Results ===");
    println!("Position updates: {}", position_updates);
    println!("Scans performed: {}", scans);
    println!("Energy adjustments: {}", energy_adjustments);
    println!("Doors opened: {}", doors);
    println!("Metric collections: {}", collections);
    println!("Final state: {:?}", final_state);
    println!("Final efficiency: {:.2}", final_state.efficiency_score());
    println!("Total operations: {}", final_metrics.total_operations);
    println!("Error rate: {:.2}%", final_metrics.error_rate() * 100.0);

    let total_operations = position_updates + scans + energy_adjustments + doors + collections;
    (total_operations, final_metrics)
}

/// Read-write lock demonstration
pub async fn read_write_lock_demo() -> (u32, u32) {
    let shared_state_rw = Arc::new(RwLock::new(RobotState::new()));
    let mut read_operations = 0;
    let mut write_operations = 0;

    // Multiple reader tasks
    let reader1 = {
        let state = shared_state_rw.clone();
        smol::spawn(async move {
            let mut reads = 0;
            for _ in 0..5 {
                smol::Timer::after(Duration::from_millis(100)).await;
                {
                    let robot_state = state.read().unwrap();
                    let _position = robot_state.position;
                    let _energy = robot_state.energy;
                    reads += 1;
                }
            }
            reads
        })
    };

    let reader2 = {
        let state = shared_state_rw.clone();
        smol::spawn(async move {
            let mut reads = 0;
            for _ in 0..4 {
                smol::Timer::after(Duration::from_millis(150)).await;
                {
                    let robot_state = state.read().unwrap();
                    let _items = robot_state.items_collected.len();
                    let _scans = robot_state.scan_count;
                    reads += 1;
                }
            }
            reads
        })
    };

    // Single writer task
    let writer = {
        let state = shared_state_rw.clone();
        smol::spawn(async move {
            let mut writes = 0;
            for i in 0..3 {
                smol::Timer::after(Duration::from_millis(300)).await;
                {
                    let mut robot_state = state.write().unwrap();
                    robot_state.position = (i, i);
                    robot_state.energy = robot_state.energy.saturating_sub(5);
                    robot_state.items_collected.push(format!("rw_item_{}", i));
                    writes += 1;
                }
                println!("Writer: Updated state (write {})", writes);
            }
            writes
        })
    };

    let (reads_result, writes_result) = future::join(
        future::join(reader1, reader2),
        writer
    ).await;

    read_operations = reads_result.0 + reads_result.1;
    write_operations = writes_result;

    println!("Read-write demo: {} reads, {} writes", read_operations, write_operations);
    (read_operations, write_operations)
}

/// Deadlock prevention demonstration
pub async fn deadlock_prevention_demo() -> bool {
    let state1 = Arc::new(Mutex::new(RobotState::new()));
    let state2 = Arc::new(Mutex::new(RobotState::new()));

    // Task 1: Always locks state1 then state2
    let task1 = {
        let s1 = state1.clone();
        let s2 = state2.clone();
        smol::spawn(async move {
            for i in 0..3 {
                smol::Timer::after(Duration::from_millis(100)).await;

                // Lock in consistent order to prevent deadlock
                let _lock1 = s1.lock().unwrap();
                smol::Timer::after(Duration::from_millis(50)).await;
                let _lock2 = s2.lock().unwrap();

                println!("Task 1: Acquired both locks (iteration {})", i + 1);
                smol::Timer::after(Duration::from_millis(50)).await;
            }
            true
        })
    };

    // Task 2: Also locks state1 then state2 (same order)
    let task2 = {
        let s1 = state1.clone();
        let s2 = state2.clone();
        smol::spawn(async move {
            for i in 0..3 {
                smol::Timer::after(Duration::from_millis(150)).await;

                // Lock in same order to prevent deadlock
                let _lock1 = s1.lock().unwrap();
                smol::Timer::after(Duration::from_millis(50)).await;
                let _lock2 = s2.lock().unwrap();

                println!("Task 2: Acquired both locks (iteration {})", i + 1);
                smol::Timer::after(Duration::from_millis(50)).await;
            }
            true
        })
    };

    let (result1, result2) = future::join(task1, task2).await;
    let success = result1 && result2;

    println!("Deadlock prevention demo: {}", if success { "SUCCESS" } else { "FAILED" });
    success
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_state_creation() {
        let state = RobotState::new();
        assert_eq!(state.position, (0, 0));
        assert_eq!(state.energy, 100);
        assert_eq!(state.items_collected.len(), 0);
        assert_eq!(state.doors_opened, 0);
        assert_eq!(state.scan_count, 0);
        assert_eq!(state.operational_mode, OperationalMode::Normal);
    }

    #[test]
    fn test_robot_state_efficiency() {
        let mut state = RobotState::new();
        let initial_efficiency = state.efficiency_score();

        state.position = (5, 3);
        state.items_collected.push("test_item".to_string());
        state.doors_opened = 2;

        let final_efficiency = state.efficiency_score();
        assert!(final_efficiency > initial_efficiency);
    }

    #[test]
    fn test_operational_mode_energy_consumption() {
        assert_eq!(OperationalMode::PowerSave.energy_consumption_rate(), 1);
        assert_eq!(OperationalMode::Normal.energy_consumption_rate(), 2);
        assert_eq!(OperationalMode::HighPerformance.energy_consumption_rate(), 4);
        assert_eq!(OperationalMode::Emergency.energy_consumption_rate(), 6);
        assert_eq!(OperationalMode::Maintenance.energy_consumption_rate(), 0);
    }

    #[test]
    fn test_system_metrics() {
        let mut metrics = SystemMetrics::new();
        assert_eq!(metrics.total_operations, 0);
        assert_eq!(metrics.error_count, 0);

        metrics.record_task("test_task");
        assert_eq!(metrics.total_operations, 1);
        assert_eq!(metrics.task_executions["test_task"], 1);

        metrics.record_error();
        assert_eq!(metrics.error_count, 1);
        assert_eq!(metrics.error_rate(), 0.5); // 1 error out of 2 total operations
    }

    #[smol_potat::test]
    async fn test_shared_state_basic_operations() {
        let shared_state = Arc::new(Mutex::new(RobotState::new()));

        // Test basic state modification
        {
            let mut state = shared_state.lock().unwrap();
            state.position = (3, 4);
            state.energy = 75;
            state.items_collected.push("test_item".to_string());
        }

        // Verify modifications
        {
            let state = shared_state.lock().unwrap();
            assert_eq!(state.position, (3, 4));
            assert_eq!(state.energy, 75);
            assert_eq!(state.items_collected.len(), 1);
        }
    }

    #[smol_potat::test]
    async fn test_position_updater() {
        let shared_state = Arc::new(Mutex::new(RobotState::new()));
        let updates = position_updater(shared_state.clone()).await;

        assert!(updates > 0);

        let final_state = shared_state.lock().unwrap();
        assert_ne!(final_state.position, (0, 0)); // Should have moved
        assert!(final_state.energy < 100); // Should have consumed energy
    }

    #[smol_potat::test]
    async fn test_scan_counter() {
        let shared_state = Arc::new(Mutex::new(RobotState::new()));
        let scans = scan_counter(shared_state.clone()).await;

        assert!(scans > 0);

        let final_state = shared_state.lock().unwrap();
        assert!(final_state.scan_count > 0);
        // May have collected items or changed mode based on scans
    }

    #[smol_potat::test]
    async fn test_energy_manager() {
        let shared_state = Arc::new(Mutex::new(RobotState::new()));

        // Set up initial low energy state
        {
            let mut state = shared_state.lock().unwrap();
            state.energy = 15;
        }

        let adjustments = energy_manager(shared_state.clone()).await;

        let final_state = shared_state.lock().unwrap();
        // Energy manager should have made adjustments and possibly changed mode
        assert!(adjustments > 0 || final_state.operational_mode == OperationalMode::PowerSave);
    }

    #[smol_potat::test]
    async fn test_door_operator() {
        let shared_state = Arc::new(Mutex::new(RobotState::new()));
        let doors_opened = door_operator(shared_state.clone()).await;

        let final_state = shared_state.lock().unwrap();
        assert_eq!(final_state.doors_opened, doors_opened);
        // Energy should have been consumed if doors were opened
        if doors_opened > 0 {
            assert!(final_state.energy < 100);
        }
    }

    #[smol_potat::test]
    async fn test_metrics_collector() {
        let shared_state = Arc::new(Mutex::new(RobotState::new()));
        let shared_metrics = Arc::new(Mutex::new(SystemMetrics::new()));

        let collections = metrics_collector(shared_state.clone(), shared_metrics.clone()).await;

        assert!(collections > 0);

        let final_metrics = shared_metrics.lock().unwrap();
        assert!(final_metrics.total_operations > 0);
        assert!(final_metrics.performance_metrics.contains_key("efficiency"));
    }

    #[smol_potat::test]
    async fn test_comprehensive_shared_state() {
        let (total_operations, final_metrics) = comprehensive_shared_state_test().await;

        assert!(total_operations > 0);
        assert!(final_metrics.total_operations > 0);
        assert!(final_metrics.performance_metrics.contains_key("efficiency"));
    }

    #[smol_potat::test]
    async fn test_read_write_lock() {
        let (reads, writes) = read_write_lock_demo().await;

        assert!(reads > 0);
        assert!(writes > 0);
        // Should have more reads than writes typically
        assert!(reads >= writes);
    }

    #[smol_potat::test]
    async fn test_deadlock_prevention() {
        let success = deadlock_prevention_demo().await;
        assert!(success);
    }

    #[smol_potat::test]
    async fn test_concurrent_state_access() {
        let shared_state = Arc::new(Mutex::new(RobotState::new()));

        // Multiple tasks accessing state concurrently
        let task1 = {
            let state = shared_state.clone();
            smol::spawn(async move {
                for i in 0..5 {
                    let mut s = state.lock().unwrap();
                    s.position.0 += 1;
                    s.scan_count += 1;
                    drop(s); // Explicit drop to release lock quickly
                    smol::Timer::after(Duration::from_millis(50)).await;
                }
                5
            })
        };

        let task2 = {
            let state = shared_state.clone();
            smol::spawn(async move {
                for i in 0..3 {
                    let mut s = state.lock().unwrap();
                    s.position.1 += 1;
                    s.energy = s.energy.saturating_sub(5);
                    drop(s);
                    smol::Timer::after(Duration::from_millis(80)).await;
                }
                3
            })
        };

        let (ops1, ops2) = future::join(task1, task2).await;
        assert_eq!(ops1, 5);
        assert_eq!(ops2, 3);

        let final_state = shared_state.lock().unwrap();
        assert_eq!(final_state.position.0, 5);
        assert_eq!(final_state.position.1, 3);
        assert_eq!(final_state.scan_count, 5);
        assert_eq!(final_state.energy, 85); // 100 - (3 * 5)
    }

    #[smol_potat::test]
    async fn test_state_consistency() {
        let shared_state = Arc::new(Mutex::new(RobotState::new()));

        // Task that maintains invariant: scan_count should equal items_collected length
        let consistency_task = {
            let state = shared_state.clone();
            smol::spawn(async move {
                for i in 0..4 {
                    let mut s = state.lock().unwrap();
                    s.scan_count += 1;
                    s.items_collected.push(format!("item_{}", i));
                    // Invariant: scan_count == items_collected.len()
                    assert_eq!(s.scan_count as usize, s.items_collected.len());
                    drop(s);
                    smol::Timer::after(Duration::from_millis(100)).await;
                }
            })
        };

        consistency_task.await;

        let final_state = shared_state.lock().unwrap();
        assert_eq!(final_state.scan_count as usize, final_state.items_collected.len());
    }
}

/// Example usage and demonstrations
pub async fn demonstrate_shared_state_management() {
    println!("=== Level 13 Task 2: Shared State Management Demo ===");

    // Test basic shared state operations
    println!("\n1. Testing basic shared state operations...");
    let shared_state = Arc::new(Mutex::new(RobotState::new()));

    println!("   Initial state: {:?}", shared_state.lock().unwrap().clone());

    let position_updates = position_updater(shared_state.clone()).await;
    println!("   Position updates completed: {}", position_updates);
    println!("   State after position updates: {:?}", shared_state.lock().unwrap().clone());

    // Test comprehensive coordination
    println!("\n2. Testing comprehensive shared state coordination...");
    let (total_ops, metrics) = comprehensive_shared_state_test().await;
    println!("   Total operations: {}", total_ops);
    println!("   System error rate: {:.2}%", metrics.error_rate() * 100.0);

    // Test read-write locks
    println!("\n3. Testing read-write lock performance...");
    let (reads, writes) = read_write_lock_demo().await;
    println!("   Read operations: {}", reads);
    println!("   Write operations: {}", writes);
    println!("   Read/Write ratio: {:.2}", reads as f32 / writes as f32);

    // Test deadlock prevention
    println!("\n4. Testing deadlock prevention...");
    let deadlock_success = deadlock_prevention_demo().await;
    println!("   Deadlock prevention: {}", if deadlock_success { "✅ Success" } else { "❌ Failed" });

    println!("\n✅ Shared state management demonstration complete!");
}

#[smol_potat::main]
async fn main() {
    demonstrate_shared_state_management().await;
}