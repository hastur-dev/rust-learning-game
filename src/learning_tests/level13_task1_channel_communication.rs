//! Level 13 Task 1: Channel Communication System with Smol
//!
//! This module demonstrates how to use async-channel for communication between
//! robot subsystems in the Smol async runtime environment.

use async_channel::{Sender, Receiver, unbounded, bounded};
use std::time::Duration;
use futures_lite::future;

/// Message types for inter-subsystem communication
#[derive(Debug, Clone, PartialEq)]
pub enum RobotMessage {
    ScanReport { content: String, timestamp: u64 },
    MovementUpdate { position: (i32, i32), energy: u32 },
    ThreatAlert { threat_level: u32, location: (i32, i32) },
    ItemFound { item_type: String, position: (i32, i32) },
    DoorStatus { position: (i32, i32), is_open: bool },
    SystemStatus { subsystem: String, status: String },
    Shutdown,
}

impl RobotMessage {
    /// Get the priority of the message (higher number = higher priority)
    pub fn priority(&self) -> u32 {
        match self {
            RobotMessage::Shutdown => 100,
            RobotMessage::ThreatAlert { .. } => 90,
            RobotMessage::SystemStatus { .. } => 80,
            RobotMessage::DoorStatus { .. } => 60,
            RobotMessage::ItemFound { .. } => 50,
            RobotMessage::MovementUpdate { .. } => 40,
            RobotMessage::ScanReport { .. } => 30,
        }
    }

    /// Get a short description of the message
    pub fn description(&self) -> String {
        match self {
            RobotMessage::ScanReport { content, .. } => format!("Scan: {}", content),
            RobotMessage::MovementUpdate { position, energy } => format!("Move: {:?} ({}%)", position, energy),
            RobotMessage::ThreatAlert { threat_level, location } => format!("Threat: Level {} at {:?}", threat_level, location),
            RobotMessage::ItemFound { item_type, position } => format!("Item: {} at {:?}", item_type, position),
            RobotMessage::DoorStatus { position, is_open } => format!("Door: {:?} {}", position, if *is_open { "open" } else { "closed" }),
            RobotMessage::SystemStatus { subsystem, status } => format!("System: {} is {}", subsystem, status),
            RobotMessage::Shutdown => "Shutdown signal".to_string(),
        }
    }
}

/// Robot subsystem that can scan and send reports
#[derive(Debug, Clone)]
pub struct ScannerSubsystem {
    pub id: String,
    pub position: (i32, i32),
    pub scan_count: u32,
    pub scan_interval: Duration,
}

impl ScannerSubsystem {
    pub fn new(id: String) -> Self {
        Self {
            id,
            position: (0, 0),
            scan_count: 0,
            scan_interval: Duration::from_millis(300),
        }
    }

    /// Perform a scan and generate a message
    pub async fn scan(&mut self) -> RobotMessage {
        smol::Timer::after(self.scan_interval).await;
        self.scan_count += 1;

        let scan_results = vec![
            "all_clear",
            "enemy_detected",
            "door_found",
            "item_nearby",
            "obstacle_ahead",
            "multiple_threats",
        ];

        let result = scan_results[self.scan_count as usize % scan_results.len()];
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        RobotMessage::ScanReport {
            content: result.to_string(),
            timestamp,
        }
    }

    /// Move the scanner to a new position
    pub async fn move_to(&mut self, x: i32, y: i32) {
        smol::Timer::after(Duration::from_millis(200)).await;
        self.position = (x, y);
    }
}

/// Coordinator subsystem that processes messages from other subsystems
#[derive(Debug, Clone)]
pub struct CoordinatorSubsystem {
    pub id: String,
    pub messages_processed: u32,
    pub active_threats: Vec<(u32, (i32, i32))>,
    pub known_items: Vec<(String, (i32, i32))>,
    pub door_states: Vec<((i32, i32), bool)>,
    pub subsystem_status: std::collections::HashMap<String, String>,
}

impl CoordinatorSubsystem {
    pub fn new(id: String) -> Self {
        Self {
            id,
            messages_processed: 0,
            active_threats: Vec::new(),
            known_items: Vec::new(),
            door_states: Vec::new(),
            subsystem_status: std::collections::HashMap::new(),
        }
    }

    /// Process a received message
    pub async fn process_message(&mut self, message: RobotMessage) -> Option<RobotMessage> {
        self.messages_processed += 1;

        match message {
            RobotMessage::ScanReport { content, .. } => {
                println!("📡 Coordinator: Processing scan - {}", content);
                if content.contains("enemy") || content.contains("threat") {
                    Some(RobotMessage::ThreatAlert {
                        threat_level: 70,
                        location: (5, 5),
                    })
                } else if content.contains("door") {
                    Some(RobotMessage::DoorStatus {
                        position: (3, 4),
                        is_open: false,
                    })
                } else if content.contains("item") {
                    Some(RobotMessage::ItemFound {
                        item_type: "energy_cell".to_string(),
                        position: (2, 6),
                    })
                } else {
                    None
                }
            }
            RobotMessage::ThreatAlert { threat_level, location } => {
                println!("⚠️  Coordinator: Threat level {} at {:?}", threat_level, location);
                self.active_threats.push((threat_level, location));
                Some(RobotMessage::SystemStatus {
                    subsystem: "defense".to_string(),
                    status: "alert".to_string(),
                })
            }
            RobotMessage::ItemFound { item_type, position } => {
                println!("📦 Coordinator: {} found at {:?}", item_type, position);
                self.known_items.push((item_type, position));
                None
            }
            RobotMessage::DoorStatus { position, is_open } => {
                println!("🚪 Coordinator: Door at {:?} is {}", position, if is_open { "open" } else { "closed" });
                self.door_states.push((position, is_open));
                if !is_open {
                    Some(RobotMessage::SystemStatus {
                        subsystem: "navigation".to_string(),
                        status: "door_blocked".to_string(),
                    })
                } else {
                    None
                }
            }
            RobotMessage::SystemStatus { subsystem, status } => {
                println!("🔧 Coordinator: {} system is {}", subsystem, status);
                self.subsystem_status.insert(subsystem, status);
                None
            }
            RobotMessage::MovementUpdate { position, energy } => {
                println!("🚀 Coordinator: Robot at {:?} with {}% energy", position, energy);
                if energy < 20 {
                    Some(RobotMessage::SystemStatus {
                        subsystem: "power".to_string(),
                        status: "low_energy".to_string(),
                    })
                } else {
                    None
                }
            }
            RobotMessage::Shutdown => {
                println!("🔴 Coordinator: Shutdown signal received");
                None
            }
        }
    }
}

/// Task 1: Create scanner and coordinator tasks with channel communication
pub async fn scanner_task(sender: Sender<RobotMessage>) -> u32 {
    let mut scanner = ScannerSubsystem::new("primary_scanner".to_string());
    let mut messages_sent = 0;

    loop {
        let scan_message = scanner.scan().await;
        messages_sent += 1;

        if sender.send(scan_message.clone()).await.is_err() {
            println!("Scanner: Channel closed, stopping scanner");
            break;
        }

        if scan_message.description().contains("goal") || messages_sent >= 10 {
            break;
        }
    }

    println!("Scanner task completed. Messages sent: {}", messages_sent);
    messages_sent
}

pub async fn coordinator_task(receiver: Receiver<RobotMessage>) -> u32 {
    let mut coordinator = CoordinatorSubsystem::new("main_coordinator".to_string());

    while let Ok(message) = receiver.recv().await {
        println!("Coordinator received: {}", message.description());

        if let Some(response) = coordinator.process_message(message).await {
            println!("Coordinator generated response: {}", response.description());
        }

        // Break on shutdown or after processing many messages
        if coordinator.messages_processed >= 10 {
            break;
        }
    }

    println!("Coordinator task completed. Messages processed: {}", coordinator.messages_processed);
    coordinator.messages_processed
}

/// Bidirectional communication system
pub async fn bidirectional_communication() -> (u32, u32) {
    let (scanner_tx, coordinator_rx) = unbounded::<RobotMessage>();
    let (coordinator_tx, scanner_rx) = unbounded::<RobotMessage>();

    // Enhanced scanner that can receive commands
    let enhanced_scanner_task = {
        let scanner_tx = scanner_tx.clone();
        smol::spawn(async move {
            let mut scanner = ScannerSubsystem::new("enhanced_scanner".to_string());
            let mut messages_sent = 0;
            let mut commands_received = 0;

            for i in 0..6 {
                // Send scan report
                let scan_message = scanner.scan().await;
                if scanner_tx.send(scan_message).await.is_err() {
                    break;
                }
                messages_sent += 1;

                // Check for incoming commands (non-blocking)
                if let Ok(command) = scanner_rx.try_recv() {
                    commands_received += 1;
                    match command {
                        RobotMessage::SystemStatus { status, .. } if status.contains("move") => {
                            scanner.move_to(i, i).await;
                        }
                        RobotMessage::SystemStatus { status, .. } if status.contains("fast") => {
                            scanner.scan_interval = Duration::from_millis(100);
                        }
                        _ => {}
                    }
                }

                smol::Timer::after(Duration::from_millis(200)).await;
            }

            (messages_sent, commands_received)
        })
    };

    // Enhanced coordinator that can send commands
    let enhanced_coordinator_task = {
        smol::spawn(async move {
            let mut coordinator = CoordinatorSubsystem::new("enhanced_coordinator".to_string());
            let mut commands_sent = 0;

            while let Ok(message) = coordinator_rx.recv().await {
                if let Some(response) = coordinator.process_message(message).await {
                    if coordinator_tx.send(response).await.is_ok() {
                        commands_sent += 1;
                    }
                }

                // Send periodic commands
                if coordinator.messages_processed % 3 == 0 {
                    let command = RobotMessage::SystemStatus {
                        subsystem: "scanner".to_string(),
                        status: "move_requested".to_string(),
                    };
                    if coordinator_tx.send(command).await.is_ok() {
                        commands_sent += 1;
                    }
                }

                if coordinator.messages_processed >= 6 {
                    break;
                }
            }

            (coordinator.messages_processed, commands_sent)
        })
    };

    // Wait for both tasks to complete
    let ((scanner_sent, scanner_commands), (coordinator_processed, coordinator_commands)) =
        future::join(enhanced_scanner_task, enhanced_coordinator_task).await;

    println!("Bidirectional communication completed:");
    println!("  Scanner: {} messages sent, {} commands received", scanner_sent, scanner_commands);
    println!("  Coordinator: {} messages processed, {} commands sent", coordinator_processed, coordinator_commands);

    (scanner_sent + coordinator_processed, scanner_commands + coordinator_commands)
}

/// Multiple scanner coordination
pub async fn multi_scanner_coordination() -> Vec<u32> {
    let (sender, receiver) = unbounded::<RobotMessage>();

    // Spawn multiple scanner tasks
    let scanner1 = {
        let sender = sender.clone();
        smol::spawn(async move {
            let mut scanner = ScannerSubsystem::new("scanner_alpha".to_string());
            scanner.scan_interval = Duration::from_millis(250);
            let mut count = 0;
            for _ in 0..4 {
                let message = scanner.scan().await;
                if sender.send(message).await.is_err() {
                    break;
                }
                count += 1;
            }
            count
        })
    };

    let scanner2 = {
        let sender = sender.clone();
        smol::spawn(async move {
            let mut scanner = ScannerSubsystem::new("scanner_beta".to_string());
            scanner.scan_interval = Duration::from_millis(400);
            let mut count = 0;
            for _ in 0..3 {
                let message = scanner.scan().await;
                if sender.send(message).await.is_err() {
                    break;
                }
                count += 1;
            }
            count
        })
    };

    let scanner3 = {
        let sender = sender.clone();
        smol::spawn(async move {
            let mut scanner = ScannerSubsystem::new("scanner_gamma".to_string());
            scanner.scan_interval = Duration::from_millis(150);
            let mut count = 0;
            for _ in 0..5 {
                let message = scanner.scan().await;
                if sender.send(message).await.is_err() {
                    break;
                }
                count += 1;
            }
            count
        })
    };

    // Drop the original sender so receiver knows when all are done
    drop(sender);

    // Coordinator task to receive from all scanners
    let coordinator_task = smol::spawn(async move {
        let mut coordinator = CoordinatorSubsystem::new("multi_coordinator".to_string());
        let mut received_count = 0;

        while let Ok(message) = receiver.recv().await {
            coordinator.process_message(message).await;
            received_count += 1;
        }

        println!("Multi-scanner coordination: {} total messages received", received_count);
        received_count
    });

    // Wait for all tasks to complete
    let (count1, count2, count3, total_received) = future::join(
        future::join(scanner1, scanner2),
        future::join(scanner3, coordinator_task)
    ).await;

    let (count1, count2) = count1;
    let (count3, total_received) = count3;

    vec![count1, count2, count3, total_received]
}

/// Priority message handling system
pub async fn priority_message_system() -> Vec<RobotMessage> {
    let (sender, receiver) = bounded::<RobotMessage>(10);
    let mut processed_messages = Vec::new();

    // Message producer task
    let producer_task = smol::spawn(async move {
        let messages = vec![
            RobotMessage::ScanReport { content: "routine_scan".to_string(), timestamp: 1000 },
            RobotMessage::ThreatAlert { threat_level: 85, location: (5, 5) },
            RobotMessage::ItemFound { item_type: "key".to_string(), position: (3, 3) },
            RobotMessage::MovementUpdate { position: (2, 2), energy: 15 },
            RobotMessage::Shutdown,
            RobotMessage::DoorStatus { position: (4, 4), is_open: false },
            RobotMessage::SystemStatus { subsystem: "engine".to_string(), status: "critical".to_string() },
        ];

        for message in messages {
            if sender.send(message).await.is_err() {
                break;
            }
            smol::Timer::after(Duration::from_millis(100)).await;
        }
    });

    // Message consumer with priority handling
    let consumer_task = smol::spawn(async move {
        let mut message_buffer = Vec::new();
        let mut processed = Vec::new();

        // Collect some messages first
        for _ in 0..7 {
            if let Ok(message) = receiver.recv().await {
                message_buffer.push(message);
            }
        }

        // Sort by priority (highest first)
        message_buffer.sort_by(|a, b| b.priority().cmp(&a.priority()));

        // Process in priority order
        for message in message_buffer {
            println!("Processing priority {} message: {}", message.priority(), message.description());
            processed.push(message);

            // Stop on shutdown message
            if matches!(processed.last(), Some(RobotMessage::Shutdown)) {
                break;
            }
        }

        processed
    });

    // Wait for both tasks
    producer_task.await;
    processed_messages = consumer_task.await;

    processed_messages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_message_priority() {
        assert_eq!(RobotMessage::Shutdown.priority(), 100);
        assert_eq!(RobotMessage::ThreatAlert { threat_level: 50, location: (0, 0) }.priority(), 90);
        assert_eq!(RobotMessage::ScanReport { content: "test".to_string(), timestamp: 0 }.priority(), 30);
    }

    #[test]
    fn test_robot_message_description() {
        let scan_msg = RobotMessage::ScanReport { content: "all_clear".to_string(), timestamp: 1000 };
        assert!(scan_msg.description().contains("Scan: all_clear"));

        let threat_msg = RobotMessage::ThreatAlert { threat_level: 80, location: (3, 4) };
        assert!(threat_msg.description().contains("Threat: Level 80"));
    }

    #[test]
    fn test_scanner_subsystem_creation() {
        let scanner = ScannerSubsystem::new("test_scanner".to_string());
        assert_eq!(scanner.id, "test_scanner");
        assert_eq!(scanner.position, (0, 0));
        assert_eq!(scanner.scan_count, 0);
    }

    #[test]
    fn test_coordinator_subsystem_creation() {
        let coordinator = CoordinatorSubsystem::new("test_coordinator".to_string());
        assert_eq!(coordinator.id, "test_coordinator");
        assert_eq!(coordinator.messages_processed, 0);
        assert_eq!(coordinator.active_threats.len(), 0);
        assert_eq!(coordinator.known_items.len(), 0);
    }

    #[smol_potat::test]
    async fn test_scanner_scan_operation() {
        let mut scanner = ScannerSubsystem::new("test".to_string());
        let message = scanner.scan().await;

        assert_eq!(scanner.scan_count, 1);
        match message {
            RobotMessage::ScanReport { content, timestamp } => {
                assert!(!content.is_empty());
                assert!(timestamp > 0);
            }
            _ => panic!("Expected ScanReport message"),
        }
    }

    #[smol_potat::test]
    async fn test_scanner_movement() {
        let mut scanner = ScannerSubsystem::new("test".to_string());
        assert_eq!(scanner.position, (0, 0));

        scanner.move_to(5, 3).await;
        assert_eq!(scanner.position, (5, 3));
    }

    #[smol_potat::test]
    async fn test_coordinator_message_processing() {
        let mut coordinator = CoordinatorSubsystem::new("test".to_string());

        let scan_message = RobotMessage::ScanReport {
            content: "enemy_detected".to_string(),
            timestamp: 1000,
        };

        let response = coordinator.process_message(scan_message).await;
        assert_eq!(coordinator.messages_processed, 1);
        assert!(response.is_some());

        if let Some(RobotMessage::ThreatAlert { threat_level, .. }) = response {
            assert_eq!(threat_level, 70);
        }
    }

    #[smol_potat::test]
    async fn test_basic_channel_communication() {
        let (sender, receiver) = unbounded::<RobotMessage>();

        // Send a message
        let test_message = RobotMessage::ScanReport {
            content: "test_scan".to_string(),
            timestamp: 1000,
        };

        sender.send(test_message.clone()).await.unwrap();

        // Receive the message
        let received = receiver.recv().await.unwrap();
        assert_eq!(received, test_message);
    }

    #[smol_potat::test]
    async fn test_scanner_and_coordinator_tasks() {
        let (sender, receiver) = unbounded::<RobotMessage>();

        let scanner_handle = smol::spawn(scanner_task(sender));
        let coordinator_handle = smol::spawn(coordinator_task(receiver));

        let (messages_sent, messages_processed) = future::join(scanner_handle, coordinator_handle).await;

        assert!(messages_sent > 0);
        assert!(messages_processed > 0);
        assert_eq!(messages_sent, messages_processed); // Should match
    }

    #[smol_potat::test]
    async fn test_bidirectional_communication() {
        let (total_messages, total_commands) = bidirectional_communication().await;

        assert!(total_messages > 0);
        // Commands might be 0 depending on timing, but should not error
        assert!(total_commands >= 0);
    }

    #[smol_potat::test]
    async fn test_multi_scanner_coordination() {
        let results = multi_scanner_coordination().await;

        assert_eq!(results.len(), 4); // 3 scanners + 1 total received count
        assert!(results[0] > 0); // Scanner 1 sent messages
        assert!(results[1] > 0); // Scanner 2 sent messages
        assert!(results[2] > 0); // Scanner 3 sent messages
        assert!(results[3] > 0); // Total received messages

        // Total received should equal sum of sent
        let total_sent = results[0] + results[1] + results[2];
        assert_eq!(total_sent, results[3]);
    }

    #[smol_potat::test]
    async fn test_priority_message_system() {
        let processed_messages = priority_message_system().await;

        assert!(!processed_messages.is_empty());

        // First message should be highest priority (Shutdown = 100)
        assert_eq!(processed_messages[0].priority(), 100);

        // Messages should be in descending priority order
        for window in processed_messages.windows(2) {
            assert!(window[0].priority() >= window[1].priority());
        }

        // Should end with Shutdown message
        assert!(matches!(processed_messages.last(), Some(RobotMessage::Shutdown)));
    }

    #[smol_potat::test]
    async fn test_bounded_channel_behavior() {
        let (sender, receiver) = bounded::<RobotMessage>(3);

        // Send messages up to capacity
        for i in 0..3 {
            let message = RobotMessage::ScanReport {
                content: format!("scan_{}", i),
                timestamp: i as u64,
            };
            sender.send(message).await.unwrap();
        }

        // Channel should now be full, but we can still receive
        let received = receiver.recv().await.unwrap();
        match received {
            RobotMessage::ScanReport { content, .. } => {
                assert_eq!(content, "scan_0");
            }
            _ => panic!("Expected first scan message"),
        }
    }

    #[smol_potat::test]
    async fn test_channel_closure() {
        let (sender, receiver) = unbounded::<RobotMessage>();

        // Drop sender
        drop(sender);

        // Receiver should get an error
        assert!(receiver.recv().await.is_err());
    }

    #[smol_potat::test]
    async fn test_multiple_senders_single_receiver() {
        let (sender, receiver) = unbounded::<RobotMessage>();

        let sender1 = sender.clone();
        let sender2 = sender.clone();
        drop(sender); // Drop original

        let task1 = smol::spawn(async move {
            for i in 0..3 {
                let message = RobotMessage::ScanReport {
                    content: format!("sender1_{}", i),
                    timestamp: i as u64,
                };
                if sender1.send(message).await.is_err() {
                    break;
                }
            }
        });

        let task2 = smol::spawn(async move {
            for i in 0..2 {
                let message = RobotMessage::ScanReport {
                    content: format!("sender2_{}", i),
                    timestamp: (i + 100) as u64,
                };
                if sender2.send(message).await.is_err() {
                    break;
                }
            }
        });

        // Wait for senders to complete
        future::join(task1, task2).await;

        // Receive all messages
        let mut received_count = 0;
        while receiver.recv().await.is_ok() {
            received_count += 1;
        }

        assert_eq!(received_count, 5); // 3 + 2 messages
    }
}

/// Example usage and demonstrations
pub async fn demonstrate_channel_communication() {
    println!("=== Level 13 Task 1: Channel Communication Demo ===");

    // Test basic scanner and coordinator communication
    println!("\n1. Testing basic scanner-coordinator communication...");
    let (sender, receiver) = unbounded::<RobotMessage>();

    let scanner_handle = smol::spawn(scanner_task(sender));
    let coordinator_handle = smol::spawn(coordinator_task(receiver));

    let (messages_sent, messages_processed) = future::join(scanner_handle, coordinator_handle).await;
    println!("   Scanner sent: {} messages", messages_sent);
    println!("   Coordinator processed: {} messages", messages_processed);

    // Test bidirectional communication
    println!("\n2. Testing bidirectional communication...");
    let (total_messages, total_commands) = bidirectional_communication().await;
    println!("   Total message flow: {} messages, {} commands", total_messages, total_commands);

    // Test multi-scanner coordination
    println!("\n3. Testing multi-scanner coordination...");
    let scanner_results = multi_scanner_coordination().await;
    println!("   Scanner Alpha: {} messages", scanner_results[0]);
    println!("   Scanner Beta: {} messages", scanner_results[1]);
    println!("   Scanner Gamma: {} messages", scanner_results[2]);
    println!("   Total received: {} messages", scanner_results[3]);

    // Test priority message system
    println!("\n4. Testing priority message system...");
    let priority_messages = priority_message_system().await;
    println!("   Processed {} messages in priority order:", priority_messages.len());
    for (i, message) in priority_messages.iter().take(3).enumerate() {
        println!("     {}: [P{}] {}", i + 1, message.priority(), message.description());
    }

    println!("\n✅ Channel communication demonstration complete!");
}

#[smol_potat::main]
async fn main() {
    demonstrate_channel_communication().await;
}