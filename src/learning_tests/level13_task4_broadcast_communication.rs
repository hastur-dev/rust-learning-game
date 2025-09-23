//! Level 13 Task 4: Broadcast Communication System with Smol
//!
//! This module demonstrates how to create a system where one task broadcasts
//! to multiple listeners, implementing fan-out communication patterns.

use async_channel::{Sender, Receiver, unbounded, bounded};
use std::time::Duration;
use futures_lite::future;
use std::collections::HashMap;

/// Command types that can be broadcast to subsystems
#[derive(Debug, Clone, PartialEq)]
pub enum RobotCommand {
    ScanArea { area_id: String, scan_type: String },
    OpenDoors { door_positions: Vec<(i32, i32)> },
    CollectItems { item_types: Vec<String> },
    MoveToGoal { target_position: (i32, i32) },
    ChangeMode { new_mode: String },
    EmergencyStop,
    SystemCheck { subsystem: String },
    UpdateSettings { key: String, value: String },
}

impl RobotCommand {
    /// Get the priority level of the command
    pub fn priority(&self) -> u32 {
        match self {
            RobotCommand::EmergencyStop => 100,
            RobotCommand::SystemCheck { .. } => 90,
            RobotCommand::ChangeMode { .. } => 80,
            RobotCommand::ScanArea { .. } => 70,
            RobotCommand::OpenDoors { .. } => 60,
            RobotCommand::CollectItems { .. } => 50,
            RobotCommand::MoveToGoal { .. } => 40,
            RobotCommand::UpdateSettings { .. } => 30,
        }
    }

    /// Get a description of the command
    pub fn description(&self) -> String {
        match self {
            RobotCommand::ScanArea { area_id, scan_type } => format!("Scan {} with {}", area_id, scan_type),
            RobotCommand::OpenDoors { door_positions } => format!("Open {} doors", door_positions.len()),
            RobotCommand::CollectItems { item_types } => format!("Collect {} types of items", item_types.len()),
            RobotCommand::MoveToGoal { target_position } => format!("Move to {:?}", target_position),
            RobotCommand::ChangeMode { new_mode } => format!("Change mode to {}", new_mode),
            RobotCommand::EmergencyStop => "EMERGENCY STOP".to_string(),
            RobotCommand::SystemCheck { subsystem } => format!("Check {} subsystem", subsystem),
            RobotCommand::UpdateSettings { key, value } => format!("Update {}: {}", key, value),
        }
    }
}

/// Result of command execution by a subsystem
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub subsystem_name: String,
    pub command_id: String,
    pub success: bool,
    pub execution_time_ms: u64,
    pub result_data: String,
    pub energy_consumed: u32,
}

impl CommandResult {
    pub fn new(subsystem_name: String, command_id: String) -> Self {
        Self {
            subsystem_name,
            command_id,
            success: false,
            execution_time_ms: 0,
            result_data: String::new(),
            energy_consumed: 0,
        }
    }

    pub fn success(mut self, result: String, time_ms: u64, energy: u32) -> Self {
        self.success = true;
        self.result_data = result;
        self.execution_time_ms = time_ms;
        self.energy_consumed = energy;
        self
    }

    pub fn failure(mut self, error: String, time_ms: u64) -> Self {
        self.success = false;
        self.result_data = error;
        self.execution_time_ms = time_ms;
        self
    }
}

/// Task 4: Command broadcaster that sends commands to multiple subsystems
pub async fn command_broadcaster(sender: Sender<RobotCommand>) -> u32 {
    let commands = vec![
        RobotCommand::SystemCheck { subsystem: "all".to_string() },
        RobotCommand::ScanArea { area_id: "sector_1".to_string(), scan_type: "full".to_string() },
        RobotCommand::OpenDoors { door_positions: vec![(4, 1), (5, 1)] },
        RobotCommand::CollectItems { item_types: vec!["energy_cell".to_string(), "key_card".to_string()] },
        RobotCommand::MoveToGoal { target_position: (9, 7) },
        RobotCommand::ChangeMode { new_mode: "stealth".to_string() },
        RobotCommand::UpdateSettings { key: "scan_interval".to_string(), value: "200ms".to_string() },
        RobotCommand::EmergencyStop,
    ];

    let mut commands_sent = 0;

    for (i, command) in commands.into_iter().enumerate() {
        println!("📡 Broadcasting command {}: {}", i + 1, command.description());

        if sender.send(command).await.is_err() {
            println!("Broadcaster: Channel closed, stopping");
            break;
        }

        commands_sent += 1;
        smol::Timer::after(Duration::from_secs(1)).await;
    }

    println!("Command broadcaster completed. Sent {} commands", commands_sent);
    commands_sent
}

/// Subsystem listener that receives and executes broadcast commands
pub async fn subsystem_listener(
    mut receiver: Receiver<RobotCommand>,
    subsystem_name: String,
    result_sender: Option<Sender<CommandResult>>,
) -> u32 {
    let mut commands_processed = 0;

    while let Ok(command) = receiver.recv().await {
        let start_time = std::time::Instant::now();
        commands_processed += 1;

        println!("{} received command: {}", subsystem_name, command.description());

        // Execute command based on subsystem capabilities
        let result = match (&command, subsystem_name.as_str()) {
            (RobotCommand::ScanArea { area_id, scan_type }, "scanner") => {
                smol::Timer::after(Duration::from_millis(300)).await;
                CommandResult::new(subsystem_name.clone(), format!("scan_{}", commands_processed))
                    .success(format!("Scanned {} with {}", area_id, scan_type), 300, 15)
            }
            (RobotCommand::OpenDoors { door_positions }, "door_controller") => {
                smol::Timer::after(Duration::from_millis(500)).await;
                CommandResult::new(subsystem_name.clone(), format!("doors_{}", commands_processed))
                    .success(format!("Opened {} doors", door_positions.len()), 500, 25)
            }
            (RobotCommand::CollectItems { item_types }, "item_collector") => {
                smol::Timer::after(Duration::from_millis(400)).await;
                CommandResult::new(subsystem_name.clone(), format!("items_{}", commands_processed))
                    .success(format!("Collected {} types", item_types.len()), 400, 20)
            }
            (RobotCommand::MoveToGoal { target_position }, "navigator") => {
                smol::Timer::after(Duration::from_millis(600)).await;
                CommandResult::new(subsystem_name.clone(), format!("move_{}", commands_processed))
                    .success(format!("Moved to {:?}", target_position), 600, 30)
            }
            (RobotCommand::ChangeMode { new_mode }, "system_controller") => {
                smol::Timer::after(Duration::from_millis(200)).await;
                CommandResult::new(subsystem_name.clone(), format!("mode_{}", commands_processed))
                    .success(format!("Mode changed to {}", new_mode), 200, 5)
            }
            (RobotCommand::SystemCheck { subsystem }, _) if subsystem == "all" || subsystem == &subsystem_name => {
                smol::Timer::after(Duration::from_millis(150)).await;
                CommandResult::new(subsystem_name.clone(), format!("check_{}", commands_processed))
                    .success("System OK".to_string(), 150, 3)
            }
            (RobotCommand::UpdateSettings { key, value }, _) => {
                smol::Timer::after(Duration::from_millis(100)).await;
                CommandResult::new(subsystem_name.clone(), format!("update_{}", commands_processed))
                    .success(format!("Updated {}: {}", key, value), 100, 2)
            }
            (RobotCommand::EmergencyStop, _) => {
                smol::Timer::after(Duration::from_millis(50)).await;
                CommandResult::new(subsystem_name.clone(), format!("stop_{}", commands_processed))
                    .success("Emergency stop executed".to_string(), 50, 0)
            }
            _ => {
                CommandResult::new(subsystem_name.clone(), format!("unsupported_{}", commands_processed))
                    .failure("Command not supported by this subsystem".to_string(), 10)
            }
        };

        let execution_time = start_time.elapsed().as_millis() as u64;
        println!("{} executed command in {}ms: {}",
                 subsystem_name, execution_time,
                 if result.success { "SUCCESS" } else { "FAILED" });

        // Send result if result channel is available
        if let Some(ref sender) = result_sender {
            sender.send(result).await.ok();
        }

        // Stop on emergency stop
        if matches!(command, RobotCommand::EmergencyStop) {
            println!("{} stopping due to emergency command", subsystem_name);
            break;
        }
    }

    println!("{} processed {} commands", subsystem_name, commands_processed);
    commands_processed
}

/// Multi-subsystem broadcast demonstration
pub async fn multi_subsystem_broadcast() -> (u32, Vec<u32>) {
    let (command_sender, command_receiver) = unbounded::<RobotCommand>();
    let (result_sender, result_receiver) = unbounded::<CommandResult>();

    // Create multiple subsystem listeners
    let scanner_listener = smol::spawn(subsystem_listener(
        command_receiver.clone(),
        "scanner".to_string(),
        Some(result_sender.clone()),
    ));

    let door_controller_listener = smol::spawn(subsystem_listener(
        command_receiver.clone(),
        "door_controller".to_string(),
        Some(result_sender.clone()),
    ));

    let item_collector_listener = smol::spawn(subsystem_listener(
        command_receiver.clone(),
        "item_collector".to_string(),
        Some(result_sender.clone()),
    ));

    let navigator_listener = smol::spawn(subsystem_listener(
        command_receiver.clone(),
        "navigator".to_string(),
        Some(result_sender.clone()),
    ));

    let system_controller_listener = smol::spawn(subsystem_listener(
        command_receiver.clone(),
        "system_controller".to_string(),
        Some(result_sender.clone()),
    ));

    // Result collector
    let result_collector = smol::spawn(async move {
        let mut results = Vec::new();
        let mut subsystem_stats = HashMap::new();

        while let Ok(result) = result_receiver.recv().await {
            println!("📊 Result from {}: {} ({}ms, {}% energy)",
                     result.subsystem_name,
                     if result.success { "✅" } else { "❌" },
                     result.execution_time_ms,
                     result.energy_consumed);

            let stats = subsystem_stats.entry(result.subsystem_name.clone()).or_insert((0u32, 0u32, 0u64));
            if result.success {
                stats.0 += 1; // Success count
            } else {
                stats.1 += 1; // Failure count
            }
            stats.2 += result.execution_time_ms; // Total time

            results.push(result);

            // Stop collecting after enough results
            if results.len() >= 40 {
                break;
            }
        }

        println!("\n=== Subsystem Performance ===");
        for (subsystem, (success, failures, total_time)) in subsystem_stats {
            let total = success + failures;
            let success_rate = if total > 0 { (success as f32 / total as f32) * 100.0 } else { 0.0 };
            let avg_time = if total > 0 { total_time / total as u64 } else { 0 };
            println!("{}: {:.1}% success rate, avg {}ms execution time",
                     subsystem, success_rate, avg_time);
        }

        results.len()
    });

    // Start broadcaster
    let broadcaster_task = smol::spawn(command_broadcaster(command_sender));

    // Wait for broadcaster to complete
    let commands_sent = broadcaster_task.await;

    // Give subsystems time to process remaining commands
    smol::Timer::after(Duration::from_millis(2000)).await;

    // Stop subsystem listeners by dropping receivers
    drop(command_receiver);
    drop(result_sender);

    // Collect results
    let results_collected = result_collector.await;

    // Wait for all subsystem listeners to complete
    let subsystem_results = future::join(
        future::join(scanner_listener, door_controller_listener),
        future::join(future::join(item_collector_listener, navigator_listener), system_controller_listener)
    ).await;

    let ((scanner_count, door_count), ((item_count, nav_count), sys_count)) = subsystem_results;

    println!("\n=== Final Results ===");
    println!("Commands sent: {}", commands_sent);
    println!("Results collected: {}", results_collected);
    println!("Scanner processed: {}", scanner_count);
    println!("Door controller processed: {}", door_count);
    println!("Item collector processed: {}", item_count);
    println!("Navigator processed: {}", nav_count);
    println!("System controller processed: {}", sys_count);

    (commands_sent, vec![scanner_count, door_count, item_count, nav_count, sys_count])
}

/// Selective broadcast based on subsystem capabilities
pub async fn selective_broadcast() -> HashMap<String, u32> {
    let (scanner_tx, scanner_rx) = unbounded::<RobotCommand>();
    let (navigator_tx, navigator_rx) = unbounded::<RobotCommand>();
    let (controller_tx, controller_rx) = unbounded::<RobotCommand>();

    // Selective broadcaster that sends different commands to different subsystems
    let broadcaster = smol::spawn(async move {
        let scan_commands = vec![
            RobotCommand::ScanArea { area_id: "north".to_string(), scan_type: "thermal".to_string() },
            RobotCommand::ScanArea { area_id: "south".to_string(), scan_type: "visual".to_string() },
            RobotCommand::SystemCheck { subsystem: "scanner".to_string() },
        ];

        let navigation_commands = vec![
            RobotCommand::MoveToGoal { target_position: (5, 5) },
            RobotCommand::MoveToGoal { target_position: (8, 3) },
            RobotCommand::OpenDoors { door_positions: vec![(4, 4)] },
        ];

        let control_commands = vec![
            RobotCommand::ChangeMode { new_mode: "silent".to_string() },
            RobotCommand::UpdateSettings { key: "power_level".to_string(), value: "high".to_string() },
            RobotCommand::SystemCheck { subsystem: "all".to_string() },
        ];

        // Send commands to appropriate subsystems
        for cmd in scan_commands {
            scanner_tx.send(cmd).await.ok();
            smol::Timer::after(Duration::from_millis(500)).await;
        }

        for cmd in navigation_commands {
            navigator_tx.send(cmd).await.ok();
            smol::Timer::after(Duration::from_millis(400)).await;
        }

        for cmd in control_commands {
            controller_tx.send(cmd).await.ok();
            smol::Timer::after(Duration::from_millis(300)).await;
        }

        println!("Selective broadcaster completed");
    });

    // Specialized subsystem listeners
    let scanner_task = smol::spawn(subsystem_listener(
        scanner_rx,
        "specialized_scanner".to_string(),
        None,
    ));

    let navigator_task = smol::spawn(subsystem_listener(
        navigator_rx,
        "specialized_navigator".to_string(),
        None,
    ));

    let controller_task = smol::spawn(subsystem_listener(
        controller_rx,
        "specialized_controller".to_string(),
        None,
    ));

    // Wait for all tasks
    broadcaster.await;

    // Close channels
    drop(scanner_tx);
    drop(navigator_tx);
    drop(controller_tx);

    let (scanner_processed, navigator_processed, controller_processed) =
        future::join(scanner_task, future::join(navigator_task, controller_task)).await;
    let (navigator_processed, controller_processed) = navigator_processed;

    let mut results = HashMap::new();
    results.insert("specialized_scanner".to_string(), scanner_processed);
    results.insert("specialized_navigator".to_string(), navigator_processed);
    results.insert("specialized_controller".to_string(), controller_processed);

    println!("Selective broadcast results: {:?}", results);
    results
}

/// Priority-based broadcast system
pub async fn priority_broadcast_system() -> Vec<CommandResult> {
    let (command_tx, command_rx) = unbounded::<RobotCommand>();
    let (result_tx, result_rx) = unbounded::<CommandResult>();

    // Priority broadcaster
    let broadcaster = smol::spawn(async move {
        let mut commands = vec![
            RobotCommand::UpdateSettings { key: "timeout".to_string(), value: "5s".to_string() },
            RobotCommand::CollectItems { item_types: vec!["data".to_string()] },
            RobotCommand::MoveToGoal { target_position: (7, 7) },
            RobotCommand::OpenDoors { door_positions: vec![(3, 3), (6, 6)] },
            RobotCommand::ScanArea { area_id: "priority_area".to_string(), scan_type: "deep".to_string() },
            RobotCommand::ChangeMode { new_mode: "emergency".to_string() },
            RobotCommand::SystemCheck { subsystem: "critical".to_string() },
            RobotCommand::EmergencyStop,
        ];

        // Sort by priority (highest first)
        commands.sort_by(|a, b| b.priority().cmp(&a.priority()));

        println!("Broadcasting commands in priority order:");
        for (i, command) in commands.iter().enumerate() {
            println!("  {}: [P{}] {}", i + 1, command.priority(), command.description());
            command_tx.send(command.clone()).await.ok();
            smol::Timer::after(Duration::from_millis(200)).await;
        }
    });

    // Priority-aware listener
    let listener = smol::spawn(subsystem_listener(
        command_rx,
        "priority_handler".to_string(),
        Some(result_tx),
    ));

    // Result collector
    let collector = smol::spawn(async move {
        let mut results = Vec::new();
        while let Ok(result) = result_rx.recv().await {
            results.push(result);
        }
        results
    });

    // Execute all tasks
    broadcaster.await;
    drop(command_tx);

    listener.await;
    drop(result_tx);

    let results = collector.await;

    println!("Priority broadcast completed. {} results collected", results.len());
    for (i, result) in results.iter().enumerate() {
        println!("  {}: {} - {} ({}ms)",
                 i + 1, result.subsystem_name,
                 if result.success { "✅" } else { "❌" },
                 result.execution_time_ms);
    }

    results
}

/// Broadcast with acknowledgment system
pub async fn broadcast_with_acknowledgment() -> (u32, u32) {
    let (command_tx, command_rx) = unbounded::<RobotCommand>();
    let (ack_tx, ack_rx) = unbounded::<String>();

    // Acknowledging subsystem
    let acknowledging_subsystem = smol::spawn(async move {
        let mut acks_sent = 0;
        while let Ok(command) = command_rx.recv().await {
            // Process command
            smol::Timer::after(Duration::from_millis(100)).await;

            // Send acknowledgment
            let ack_message = format!("ACK: {}", command.description());
            if ack_tx.send(ack_message).await.is_ok() {
                acks_sent += 1;
            }

            // Stop on emergency
            if matches!(command, RobotCommand::EmergencyStop) {
                break;
            }
        }
        acks_sent
    });

    // Broadcaster with acknowledgment tracking
    let broadcaster_with_ack = smol::spawn(async move {
        let commands = vec![
            RobotCommand::SystemCheck { subsystem: "ack_test".to_string() },
            RobotCommand::ScanArea { area_id: "ack_area".to_string(), scan_type: "basic".to_string() },
            RobotCommand::UpdateSettings { key: "ack_timeout".to_string(), value: "1s".to_string() },
            RobotCommand::EmergencyStop,
        ];

        let mut commands_sent = 0;
        for command in commands {
            println!("Sending command: {}", command.description());
            if command_tx.send(command).await.is_ok() {
                commands_sent += 1;
            }
            smol::Timer::after(Duration::from_millis(300)).await;
        }
        commands_sent
    });

    // Acknowledgment collector
    let ack_collector = smol::spawn(async move {
        let mut acks_received = 0;
        while let Ok(ack) = ack_rx.recv().await {
            println!("Received: {}", ack);
            acks_received += 1;
        }
        acks_received
    });

    // Execute all tasks
    let commands_sent = broadcaster_with_ack.await;
    drop(command_tx);

    let acks_sent = acknowledging_subsystem.await;
    drop(ack_tx);

    let acks_received = ack_collector.await;

    println!("Acknowledgment system: {} commands sent, {} acks received",
             commands_sent, acks_received);

    (commands_sent, acks_received)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_command_priority() {
        assert_eq!(RobotCommand::EmergencyStop.priority(), 100);
        assert_eq!(RobotCommand::SystemCheck { subsystem: "test".to_string() }.priority(), 90);
        assert_eq!(RobotCommand::UpdateSettings { key: "test".to_string(), value: "value".to_string() }.priority(), 30);
    }

    #[test]
    fn test_robot_command_description() {
        let scan_cmd = RobotCommand::ScanArea { area_id: "test_area".to_string(), scan_type: "thermal".to_string() };
        assert!(scan_cmd.description().contains("Scan test_area"));

        let stop_cmd = RobotCommand::EmergencyStop;
        assert_eq!(stop_cmd.description(), "EMERGENCY STOP");
    }

    #[test]
    fn test_command_result_creation() {
        let result = CommandResult::new("test_subsystem".to_string(), "cmd_1".to_string());
        assert_eq!(result.subsystem_name, "test_subsystem");
        assert_eq!(result.command_id, "cmd_1");
        assert!(!result.success);
        assert_eq!(result.execution_time_ms, 0);
    }

    #[test]
    fn test_command_result_success() {
        let result = CommandResult::new("test".to_string(), "cmd".to_string())
            .success("Test completed".to_string(), 150, 10);

        assert!(result.success);
        assert_eq!(result.result_data, "Test completed");
        assert_eq!(result.execution_time_ms, 150);
        assert_eq!(result.energy_consumed, 10);
    }

    #[test]
    fn test_command_result_failure() {
        let result = CommandResult::new("test".to_string(), "cmd".to_string())
            .failure("Test failed".to_string(), 50);

        assert!(!result.success);
        assert_eq!(result.result_data, "Test failed");
        assert_eq!(result.execution_time_ms, 50);
        assert_eq!(result.energy_consumed, 0);
    }

    #[smol_potat::test]
    async fn test_command_broadcaster() {
        let (sender, receiver) = unbounded::<RobotCommand>();

        let broadcaster_task = command_broadcaster(sender);
        let receiver_task = smol::spawn(async move {
            let mut received = Vec::new();
            while let Ok(command) = receiver.recv().await {
                received.push(command);
            }
            received
        });

        let commands_sent = broadcaster_task.await;
        let commands_received = receiver_task.await;

        assert!(commands_sent > 0);
        assert_eq!(commands_sent as usize, commands_received.len());

        // Should end with emergency stop
        assert!(matches!(commands_received.last(), Some(RobotCommand::EmergencyStop)));
    }

    #[smol_potat::test]
    async fn test_subsystem_listener() {
        let (sender, receiver) = unbounded::<RobotCommand>();

        let test_commands = vec![
            RobotCommand::ScanArea { area_id: "test".to_string(), scan_type: "basic".to_string() },
            RobotCommand::SystemCheck { subsystem: "all".to_string() },
            RobotCommand::EmergencyStop,
        ];

        let sender_task = smol::spawn(async move {
            for command in test_commands {
                sender.send(command).await.ok();
                smol::Timer::after(Duration::from_millis(100)).await;
            }
        });

        let listener_task = subsystem_listener(receiver, "test_subsystem".to_string(), None);

        sender_task.await;
        drop(sender);

        let commands_processed = listener_task.await;
        assert_eq!(commands_processed, 3);
    }

    #[smol_potat::test]
    async fn test_multi_subsystem_broadcast() {
        let (commands_sent, subsystem_counts) = multi_subsystem_broadcast().await;

        assert!(commands_sent > 0);
        assert_eq!(subsystem_counts.len(), 5); // 5 subsystems

        // All subsystems should have processed some commands
        for count in subsystem_counts {
            assert!(count > 0);
        }
    }

    #[smol_potat::test]
    async fn test_selective_broadcast() {
        let results = selective_broadcast().await;

        assert_eq!(results.len(), 3);
        assert!(results.contains_key("specialized_scanner"));
        assert!(results.contains_key("specialized_navigator"));
        assert!(results.contains_key("specialized_controller"));

        // Each subsystem should have processed appropriate commands
        for (_, count) in results {
            assert!(count > 0);
        }
    }

    #[smol_potat::test]
    async fn test_priority_broadcast_system() {
        let results = priority_broadcast_system().await;

        assert!(!results.is_empty());

        // First result should be from highest priority command (EmergencyStop)
        if let Some(first_result) = results.first() {
            assert!(first_result.result_data.contains("Emergency stop") ||
                    first_result.command_id.contains("stop"));
        }
    }

    #[smol_potat::test]
    async fn test_broadcast_with_acknowledgment() {
        let (commands_sent, acks_received) = broadcast_with_acknowledgment().await;

        assert!(commands_sent > 0);
        assert!(acks_received > 0);
        assert_eq!(commands_sent, acks_received); // Should receive ack for each command
    }

    #[smol_potat::test]
    async fn test_subsystem_command_filtering() {
        let (sender, receiver) = unbounded::<RobotCommand>();

        // Send commands to a scanner subsystem
        let sender_task = smol::spawn(async move {
            let commands = vec![
                RobotCommand::ScanArea { area_id: "test".to_string(), scan_type: "basic".to_string() },
                RobotCommand::OpenDoors { door_positions: vec![(1, 1)] }, // Should not be supported
                RobotCommand::SystemCheck { subsystem: "scanner".to_string() },
                RobotCommand::EmergencyStop,
            ];

            for command in commands {
                sender.send(command).await.ok();
            }
        });

        let (result_tx, result_rx) = unbounded::<CommandResult>();
        let listener_task = subsystem_listener(receiver, "scanner".to_string(), Some(result_tx));

        let result_collector = smol::spawn(async move {
            let mut results = Vec::new();
            while let Ok(result) = result_rx.recv().await {
                results.push(result);
            }
            results
        });

        sender_task.await;
        drop(sender);

        let commands_processed = listener_task.await;
        drop(result_tx);

        let results = result_collector.await;

        assert_eq!(commands_processed, 4);
        assert_eq!(results.len(), 4);

        // Check that scan command succeeded but door command failed
        let scan_result = results.iter().find(|r| r.result_data.contains("Scanned"));
        let door_result = results.iter().find(|r| r.result_data.contains("not supported"));

        assert!(scan_result.is_some());
        assert!(door_result.is_some());
        assert!(scan_result.unwrap().success);
        assert!(!door_result.unwrap().success);
    }

    #[smol_potat::test]
    async fn test_concurrent_broadcast_reception() {
        let (sender, receiver) = unbounded::<RobotCommand>();

        // Multiple subsystems receiving same broadcast
        let listener1 = smol::spawn(subsystem_listener(
            receiver.clone(),
            "concurrent_1".to_string(),
            None,
        ));

        let listener2 = smol::spawn(subsystem_listener(
            receiver.clone(),
            "concurrent_2".to_string(),
            None,
        ));

        let listener3 = smol::spawn(subsystem_listener(
            receiver.clone(),
            "concurrent_3".to_string(),
            None,
        ));

        // Send broadcast commands
        let broadcaster_task = smol::spawn(async move {
            let commands = vec![
                RobotCommand::SystemCheck { subsystem: "all".to_string() },
                RobotCommand::ChangeMode { new_mode: "test".to_string() },
                RobotCommand::EmergencyStop,
            ];

            for command in commands {
                sender.send(command).await.ok();
                smol::Timer::after(Duration::from_millis(100)).await;
            }
        });

        broadcaster_task.await;
        drop(sender);

        let (count1, count2, count3) = future::join(listener1, future::join(listener2, listener3)).await;
        let (count2, count3) = count2;

        // Each subsystem should process all commands
        assert_eq!(count1, 3);
        assert_eq!(count2, 3);
        assert_eq!(count3, 3);
    }

    #[smol_potat::test]
    async fn test_command_priority_ordering() {
        let mut commands = vec![
            RobotCommand::UpdateSettings { key: "test".to_string(), value: "value".to_string() },
            RobotCommand::EmergencyStop,
            RobotCommand::ScanArea { area_id: "test".to_string(), scan_type: "basic".to_string() },
            RobotCommand::SystemCheck { subsystem: "test".to_string() },
        ];

        // Sort by priority (highest first)
        commands.sort_by(|a, b| b.priority().cmp(&a.priority()));

        // Emergency stop should be first
        assert!(matches!(commands[0], RobotCommand::EmergencyStop));
        // System check should be second
        assert!(matches!(commands[1], RobotCommand::SystemCheck { .. }));
        // Scan should be third
        assert!(matches!(commands[2], RobotCommand::ScanArea { .. }));
        // Update settings should be last
        assert!(matches!(commands[3], RobotCommand::UpdateSettings { .. }));
    }
}

/// Example usage and demonstrations
pub async fn demonstrate_broadcast_communication() {
    println!("=== Level 13 Task 4: Broadcast Communication Demo ===");

    // Test basic multi-subsystem broadcast
    println!("\n1. Testing multi-subsystem broadcast...");
    let (commands_sent, subsystem_counts) = multi_subsystem_broadcast().await;
    println!("   Commands broadcast: {}", commands_sent);
    let total_processed: u32 = subsystem_counts.iter().sum();
    println!("   Total commands processed by all subsystems: {}", total_processed);

    // Test selective broadcast
    println!("\n2. Testing selective broadcast...");
    let selective_results = selective_broadcast().await;
    println!("   Selective broadcast results:");
    for (subsystem, count) in selective_results {
        println!("     {}: {} commands", subsystem, count);
    }

    // Test priority broadcast
    println!("\n3. Testing priority broadcast system...");
    let priority_results = priority_broadcast_system().await;
    println!("   Priority broadcast completed with {} results", priority_results.len());
    let success_rate = priority_results.iter().filter(|r| r.success).count() as f32 / priority_results.len() as f32;
    println!("   Overall success rate: {:.1}%", success_rate * 100.0);

    // Test acknowledgment system
    println!("\n4. Testing broadcast with acknowledgment...");
    let (sent, acks) = broadcast_with_acknowledgment().await;
    println!("   Commands sent: {}, Acknowledgments received: {}", sent, acks);
    let ack_rate = if sent > 0 { (acks as f32 / sent as f32) * 100.0 } else { 0.0 };
    println!("   Acknowledgment rate: {:.1}%", ack_rate);

    println!("\n✅ Broadcast communication demonstration complete!");
}

#[smol_potat::main]
async fn main() {
    demonstrate_broadcast_communication().await;
}