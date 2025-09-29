#[cfg(test)]
mod level13_task5_master_coordination_tests {
    use super::*;
    use async_channel::{Sender, Receiver, unbounded};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use futures_lite::future;

    #[derive(Clone, Debug)]
    struct RobotState {
        position: (i32, i32),
        energy: u32,
        items_collected: Vec<String>,
        doors_opened: u32,
        scan_count: u32,
    }

    type SharedState = Arc<Mutex<RobotState>>;

    async fn scanner_task(sender: Sender<String>) -> Result<(), Box<dyn std::error::Error>> {
        let scan_data = vec![
            "scan: empty area",
            "scan: enemy detected at (5,3)",
            "scan: door found at (4,1)",
            "scan: item detected at (7,2)",
            "scan: goal visible at (11,9)",
        ];

        for (i, scan) in scan_data.iter().enumerate() {
            smol::Timer::after(Duration::from_millis(100)).await;
            let message = format!("SCAN_{}: {}", i + 1, scan);

            if sender.send(message).await.is_err() {
                println!("Scanner: Channel closed");
                break;
            }

            if scan.contains("goal") {
                break;
            }
        }
        Ok(())
    }

    async fn coordinator_task(receiver: Receiver<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut message_count = 0;
        while let Ok(message) = receiver.recv().await {
            println!("Coordinator received: {}", message);
            message_count += 1;

            if message.contains("enemy") {
                println!("⚠️ Coordinator: Enemy detected, changing strategy");
                // Simulate evasive action
                smol::Timer::after(Duration::from_millis(50)).await;
            } else if message.contains("door") {
                println!("🚪 Coordinator: Door found, opening");
                // Simulate door opening
                smol::Timer::after(Duration::from_millis(50)).await;
            } else if message.contains("item") {
                println!("📦 Coordinator: Item detected, collecting");
                // Simulate item collection
                smol::Timer::after(Duration::from_millis(50)).await;
            } else if message.contains("goal") {
                println!("🎯 Coordinator: Goal detected, mission complete!");
                break;
            }

            if message_count >= 10 {
                break;
            }
        }
        Ok(())
    }

    async fn position_updater(state: SharedState) -> Result<(), Box<dyn std::error::Error>> {
        let positions = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)];

        for pos in positions {
            smol::Timer::after(Duration::from_millis(200)).await;

            {
                let mut robot_state = state.lock().unwrap();
                robot_state.position = pos;
                robot_state.energy = robot_state.energy.saturating_sub(5);
                println!("Position updated to {:?}, energy: {}", pos, robot_state.energy);
            }

            // Check if we should stop
            let should_stop = {
                let robot_state = state.lock().unwrap();
                robot_state.position == (4, 4)
            };

            if should_stop {
                break;
            }
        }
        Ok(())
    }

    async fn scan_counter(state: SharedState) -> Result<(), Box<dyn std::error::Error>> {
        for i in 0..5 {
            smol::Timer::after(Duration::from_millis(300)).await;

            {
                let mut robot_state = state.lock().unwrap();
                robot_state.scan_count += 1;
                println!("Scans performed: {}", robot_state.scan_count);
            }

            // Check if we should stop
            let should_stop = {
                let robot_state = state.lock().unwrap();
                robot_state.scan_count >= 5
            };

            if should_stop {
                break;
            }
        }
        Ok(())
    }

    async fn door_manager_task(
        door_rx: Receiver<(i32, i32)>,
        state: SharedState,
        status_tx: Sender<String>
    ) -> Result<(), Box<dyn std::error::Error>> {
        while let Ok(door_pos) = door_rx.recv().await {
            // Simulate opening door
            smol::Timer::after(Duration::from_millis(100)).await;
            println!("Opening door at {:?}", door_pos);

            {
                let mut robot_state = state.lock().unwrap();
                robot_state.doors_opened += 1;
            }

            let message = format!("Door opened at {:?}", door_pos);
            if status_tx.send(message).await.is_err() {
                break;
            }
        }
        Ok(())
    }

    async fn status_monitor_task(status_rx: Receiver<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut status_count = 0;
        while let Ok(status) = status_rx.recv().await {
            println!("STATUS: {}", status);
            status_count += 1;

            if status_count >= 10 {
                break;
            }
        }
        Ok(())
    }

    async fn master_robot_system() -> Result<RobotState, Box<dyn std::error::Error>> {
        // Shared state
        let shared_state = Arc::new(Mutex::new(RobotState {
            position: (0, 0),
            energy: 100,
            items_collected: Vec::new(),
            doors_opened: 0,
            scan_count: 0,
        }));

        // Communication channels
        let (scan_tx, scan_rx) = unbounded::<String>();
        let (door_tx, door_rx) = unbounded::<(i32, i32)>();
        let (status_tx, status_rx) = unbounded::<String>();

        // Spawn coordinated tasks
        let scanner = smol::spawn(scanner_task(scan_tx));
        let coordinator = smol::spawn(coordinator_task(scan_rx));
        let position_tracker = smol::spawn(position_updater(shared_state.clone()));
        let scan_tracker = smol::spawn(scan_counter(shared_state.clone()));

        // Door management system
        let door_manager = smol::spawn(door_manager_task(
            door_rx,
            shared_state.clone(),
            status_tx.clone()
        ));

        // Status monitor
        let status_monitor = smol::spawn(status_monitor_task(status_rx));

        // Send door positions to be opened
        let door_positions = vec![(4, 1), (5, 1), (4, 3), (5, 3)];
        let door_sender = smol::spawn(async move {
            for door_pos in door_positions {
                if door_tx.send(door_pos).await.is_err() {
                    break;
                }
                smol::Timer::after(Duration::from_millis(200)).await;
            }
        });

        // Wait for tasks to complete with timeout
        let timeout_future = smol::Timer::after(Duration::from_secs(5));

        future::race(
            timeout_future,
            future::race(
                future::race(scanner, coordinator),
                future::race(
                    future::race(door_manager, status_monitor),
                    future::race(position_tracker, scan_tracker)
                )
            )
        ).await;

        // Ensure door sender completes
        let _ = door_sender.await;

        // Return final state
        let final_state = shared_state.lock().unwrap().clone();
        println!("Mission complete! Final state: {:?}", final_state);
        Ok(final_state)
    }

    #[smol_potat::test]
    async fn test_basic_channel_communication() {
        let (tx, rx) = unbounded::<String>();

        let sender_task = smol::spawn(async move {
            tx.send("Hello from scanner".to_string()).await.unwrap();
            tx.send("Enemy detected".to_string()).await.unwrap();
            tx.send("Mission complete".to_string()).await.unwrap();
        });

        let receiver_task = smol::spawn(async move {
            let mut messages = Vec::new();
            while let Ok(msg) = rx.recv().await {
                messages.push(msg.clone());
                if msg.contains("complete") {
                    break;
                }
            }
            messages
        });

        sender_task.await.unwrap();
        let messages = receiver_task.await.unwrap();

        assert_eq!(messages.len(), 3);
        assert!(messages[0].contains("Hello"));
        assert!(messages[1].contains("Enemy"));
        assert!(messages[2].contains("complete"));
    }

    #[smol_potat::test]
    async fn test_shared_state_updates() {
        let state = Arc::new(Mutex::new(RobotState {
            position: (0, 0),
            energy: 100,
            items_collected: Vec::new(),
            doors_opened: 0,
            scan_count: 0,
        }));

        let state1 = state.clone();
        let state2 = state.clone();

        let updater1 = smol::spawn(async move {
            for i in 0..3 {
                {
                    let mut robot_state = state1.lock().unwrap();
                    robot_state.position = (i, i);
                    robot_state.energy -= 10;
                }
                smol::Timer::after(Duration::from_millis(10)).await;
            }
        });

        let updater2 = smol::spawn(async move {
            for i in 0..3 {
                {
                    let mut robot_state = state2.lock().unwrap();
                    robot_state.scan_count += 1;
                    robot_state.doors_opened += 1;
                }
                smol::Timer::after(Duration::from_millis(15)).await;
            }
        });

        updater1.await.unwrap();
        updater2.await.unwrap();

        let final_state = state.lock().unwrap();
        assert_eq!(final_state.position, (2, 2));
        assert_eq!(final_state.energy, 70);
        assert_eq!(final_state.scan_count, 3);
        assert_eq!(final_state.doors_opened, 3);
    }

    #[smol_potat::test]
    async fn test_multi_producer_single_consumer() {
        let (tx, rx) = unbounded::<(String, i32)>();

        let producer1 = smol::spawn({
            let tx = tx.clone();
            async move {
                for i in 0..3 {
                    tx.send((format!("Producer1-{}", i), i)).await.unwrap();
                    smol::Timer::after(Duration::from_millis(10)).await;
                }
            }
        });

        let producer2 = smol::spawn({
            let tx = tx.clone();
            async move {
                for i in 0..3 {
                    tx.send((format!("Producer2-{}", i), i + 10)).await.unwrap();
                    smol::Timer::after(Duration::from_millis(15)).await;
                }
            }
        });

        // Drop original sender
        drop(tx);

        let consumer = smol::spawn(async move {
            let mut messages = Vec::new();
            while let Ok(msg) = rx.recv().await {
                messages.push(msg);
            }
            messages
        });

        producer1.await.unwrap();
        producer2.await.unwrap();
        let messages = consumer.await.unwrap();

        assert_eq!(messages.len(), 6);

        let producer1_count = messages.iter().filter(|(msg, _)| msg.contains("Producer1")).count();
        let producer2_count = messages.iter().filter(|(msg, _)| msg.contains("Producer2")).count();

        assert_eq!(producer1_count, 3);
        assert_eq!(producer2_count, 3);
    }

    #[smol_potat::test]
    async fn test_broadcast_communication() {
        let (tx, rx) = unbounded::<String>();

        // Create multiple receivers
        let rx1 = rx.clone();
        let rx2 = rx.clone();
        let rx3 = rx.clone();

        let broadcaster = smol::spawn(async move {
            let commands = vec!["SCAN", "MOVE", "COLLECT"];
            for cmd in commands {
                tx.send(cmd.to_string()).await.unwrap();
                smol::Timer::after(Duration::from_millis(10)).await;
            }
        });

        let listener1 = smol::spawn(async move {
            let mut commands = Vec::new();
            while let Ok(cmd) = rx1.recv().await {
                commands.push(format!("System1: {}", cmd));
                if commands.len() >= 3 { break; }
            }
            commands
        });

        let listener2 = smol::spawn(async move {
            let mut commands = Vec::new();
            while let Ok(cmd) = rx2.recv().await {
                commands.push(format!("System2: {}", cmd));
                if commands.len() >= 3 { break; }
            }
            commands
        });

        let listener3 = smol::spawn(async move {
            let mut commands = Vec::new();
            while let Ok(cmd) = rx3.recv().await {
                commands.push(format!("System3: {}", cmd));
                if commands.len() >= 3 { break; }
            }
            commands
        });

        broadcaster.await.unwrap();
        let results1 = listener1.await.unwrap();
        let results2 = listener2.await.unwrap();
        let results3 = listener3.await.unwrap();

        // All systems should receive all commands
        assert_eq!(results1.len(), 3);
        assert_eq!(results2.len(), 3);
        assert_eq!(results3.len(), 3);
    }

    #[smol_potat::test]
    async fn test_coordinated_door_management() {
        let (door_tx, door_rx) = unbounded::<(i32, i32)>();
        let (status_tx, status_rx) = unbounded::<String>();

        let state = Arc::new(Mutex::new(RobotState {
            position: (0, 0),
            energy: 100,
            items_collected: Vec::new(),
            doors_opened: 0,
            scan_count: 0,
        }));

        let door_manager = smol::spawn(door_manager_task(
            door_rx,
            state.clone(),
            status_tx
        ));

        let status_collector = smol::spawn(async move {
            let mut statuses = Vec::new();
            while let Ok(status) = status_rx.recv().await {
                statuses.push(status);
                if statuses.len() >= 3 { break; }
            }
            statuses
        });

        // Send door positions
        let door_sender = smol::spawn(async move {
            let doors = vec![(4, 1), (5, 1), (4, 3)];
            for door in doors {
                door_tx.send(door).await.unwrap();
                smol::Timer::after(Duration::from_millis(50)).await;
            }
        });

        door_sender.await.unwrap();
        let statuses = status_collector.await.unwrap();
        door_manager.await.unwrap();

        let final_state = state.lock().unwrap();
        assert_eq!(final_state.doors_opened, 3);
        assert_eq!(statuses.len(), 3);
        assert!(statuses[0].contains("(4, 1)"));
    }

    #[smol_potat::test]
    async fn test_scanner_coordinator_integration() {
        let (tx, rx) = unbounded::<String>();

        let scanner = smol::spawn(scanner_task(tx));
        let coordinator = smol::spawn(coordinator_task(rx));

        // Wait for both tasks to complete
        let (scanner_result, coordinator_result) = future::join(scanner, coordinator).await;

        assert!(scanner_result.is_ok());
        assert!(coordinator_result.is_ok());
    }

    #[smol_potat::test]
    async fn test_position_tracking() {
        let state = Arc::new(Mutex::new(RobotState {
            position: (0, 0),
            energy: 100,
            items_collected: Vec::new(),
            doors_opened: 0,
            scan_count: 0,
        }));

        let tracker = smol::spawn(position_updater(state.clone()));
        tracker.await.unwrap();

        let final_state = state.lock().unwrap();
        assert_eq!(final_state.position, (4, 4));
        assert!(final_state.energy < 100);
    }

    #[smol_potat::test]
    async fn test_scan_counting() {
        let state = Arc::new(Mutex::new(RobotState {
            position: (0, 0),
            energy: 100,
            items_collected: Vec::new(),
            doors_opened: 0,
            scan_count: 0,
        }));

        let counter = smol::spawn(scan_counter(state.clone()));
        counter.await.unwrap();

        let final_state = state.lock().unwrap();
        assert_eq!(final_state.scan_count, 5);
    }

    #[smol_potat::test]
    async fn test_master_coordination_system() {
        let result = master_robot_system().await;
        assert!(result.is_ok());

        let final_state = result.unwrap();
        assert!(final_state.scan_count > 0);
        assert!(final_state.doors_opened > 0);
        assert_ne!(final_state.position, (0, 0));
        assert!(final_state.energy < 100);
    }

    #[smol_potat::test]
    async fn test_channel_error_handling() {
        let (tx, rx) = unbounded::<String>();

        // Drop sender immediately
        drop(tx);

        // Receiver should handle closed channel gracefully
        let result = rx.recv().await;
        assert!(result.is_err());
    }

    #[smol_potat::test]
    async fn test_state_consistency() {
        let state = Arc::new(Mutex::new(RobotState {
            position: (0, 0),
            energy: 100,
            items_collected: Vec::new(),
            doors_opened: 0,
            scan_count: 0,
        }));

        // Multiple concurrent updates
        let tasks: Vec<_> = (0..5).map(|i| {
            let state = state.clone();
            smol::spawn(async move {
                for j in 0..10 {
                    {
                        let mut robot_state = state.lock().unwrap();
                        robot_state.scan_count += 1;
                        robot_state.energy = robot_state.energy.saturating_sub(1);
                    }
                    smol::Timer::after(Duration::from_millis(1)).await;
                }
            })
        }).collect();

        // Wait for all tasks
        for task in tasks {
            task.await.unwrap();
        }

        let final_state = state.lock().unwrap();
        assert_eq!(final_state.scan_count, 50); // 5 tasks * 10 increments
        assert_eq!(final_state.energy, 50); // 100 - 50
    }

    #[smol_potat::test]
    async fn test_complex_coordination_workflow() {
        let shared_state = Arc::new(Mutex::new(RobotState {
            position: (0, 0),
            energy: 100,
            items_collected: Vec::new(),
            doors_opened: 0,
            scan_count: 0,
        }));

        let (command_tx, command_rx) = unbounded::<String>();
        let (result_tx, result_rx) = unbounded::<String>();

        // Command processor
        let processor = smol::spawn({
            let state = shared_state.clone();
            let result_tx = result_tx.clone();
            async move {
                while let Ok(command) = command_rx.recv().await {
                    match command.as_str() {
                        "SCAN" => {
                            let mut robot_state = state.lock().unwrap();
                            robot_state.scan_count += 1;
                            result_tx.send(format!("Scan {} completed", robot_state.scan_count)).await.ok();
                        }
                        "MOVE" => {
                            let mut robot_state = state.lock().unwrap();
                            robot_state.position.0 += 1;
                            robot_state.energy -= 5;
                            result_tx.send(format!("Moved to {:?}", robot_state.position)).await.ok();
                        }
                        "COLLECT" => {
                            let mut robot_state = state.lock().unwrap();
                            robot_state.items_collected.push("item".to_string());
                            result_tx.send(format!("Collected item #{}", robot_state.items_collected.len())).await.ok();
                        }
                        "STOP" => break,
                        _ => {}
                    }
                }
            }
        });

        // Command sender
        let sender = smol::spawn(async move {
            let commands = vec!["SCAN", "MOVE", "COLLECT", "SCAN", "MOVE", "STOP"];
            for cmd in commands {
                command_tx.send(cmd.to_string()).await.unwrap();
                smol::Timer::after(Duration::from_millis(10)).await;
            }
        });

        // Result collector
        let collector = smol::spawn(async move {
            let mut results = Vec::new();
            while let Ok(result) = result_rx.recv().await {
                results.push(result);
                if results.len() >= 5 { break; }
            }
            results
        });

        sender.await.unwrap();
        let results = collector.await.unwrap();
        processor.await.unwrap();

        let final_state = shared_state.lock().unwrap();
        assert_eq!(final_state.scan_count, 2);
        assert_eq!(final_state.position.0, 2);
        assert_eq!(final_state.items_collected.len(), 1);
        assert_eq!(final_state.energy, 90);
        assert_eq!(results.len(), 5);
    }
}

// Example usage and reference implementation
fn main() {
    println!("Level 13 Task 5: Master Coordination System");
    println!("Run with: cargo test level13_task5");
}