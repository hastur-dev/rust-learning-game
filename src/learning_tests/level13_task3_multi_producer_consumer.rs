//! Level 13 Task 3: Multi-Producer, Single-Consumer System with Smol
//!
//! This module demonstrates how to set up multiple tasks sending data to a single
//! coordinator using async channels, implementing fan-in communication patterns.

use async_channel::{Sender, Receiver, unbounded, bounded};
use std::time::Duration;
use futures_lite::future;
use std::collections::HashMap;

/// Area exploration data sent by scanners
#[derive(Debug, Clone, PartialEq)]
pub struct AreaScanData {
    pub area_name: String,
    pub position: (i32, i32),
    pub scan_result: String,
    pub timestamp: u64,
    pub scanner_id: String,
    pub energy_used: u32,
    pub threats_detected: u32,
    pub items_found: Vec<String>,
}

impl AreaScanData {
    pub fn new(area_name: String, position: (i32, i32), scanner_id: String) -> Self {
        Self {
            area_name,
            position,
            scan_result: "unknown".to_string(),
            timestamp: 0,
            scanner_id,
            energy_used: 0,
            threats_detected: 0,
            items_found: Vec::new(),
        }
    }

    /// Calculate the priority score for this scan data
    pub fn priority_score(&self) -> u32 {
        let mut score = 0;
        score += self.threats_detected * 100; // Threats are highest priority
        score += self.items_found.len() as u32 * 50; // Items are valuable
        score += if self.scan_result.contains("clear") { 10 } else { 0 };
        score += (100 - self.energy_used); // Prefer energy-efficient scans
        score
    }

    /// Get a summary description
    pub fn summary(&self) -> String {
        format!("{} at {:?}: {} (Threats: {}, Items: {})",
                self.area_name, self.position, self.scan_result,
                self.threats_detected, self.items_found.len())
    }
}

/// Exploration statistics tracking
#[derive(Debug, Clone, Default)]
pub struct ExplorationStats {
    pub total_scans: u32,
    pub areas_explored: HashMap<String, u32>,
    pub total_threats: u32,
    pub total_items: u32,
    pub total_energy_used: u32,
    pub scanners_active: u32,
}

impl ExplorationStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update stats with new scan data
    pub fn update(&mut self, data: &AreaScanData) {
        self.total_scans += 1;
        *self.areas_explored.entry(data.area_name.clone()).or_insert(0) += 1;
        self.total_threats += data.threats_detected;
        self.total_items += data.items_found.len() as u32;
        self.total_energy_used += data.energy_used;
    }

    /// Calculate overall efficiency
    pub fn efficiency_rating(&self) -> f32 {
        if self.total_energy_used == 0 {
            return 100.0;
        }
        let value_score = (self.total_items * 20 + self.total_threats * 10) as f32;
        let efficiency = value_score / self.total_energy_used as f32;
        efficiency * 100.0
    }
}

/// Task 3: Area scanner that explores specific positions
pub async fn area_scanner(
    sender: Sender<AreaScanData>,
    area_name: String,
    positions: Vec<(i32, i32)>,
    scanner_id: String,
) -> u32 {
    let mut scans_completed = 0;

    for (i, pos) in positions.iter().enumerate() {
        // Move to position
        smol::Timer::after(Duration::from_millis(100)).await;

        // Perform scan
        let scan_delay = Duration::from_millis(200 + (i as u64 * 50));
        smol::Timer::after(scan_delay).await;

        let mut scan_data = AreaScanData::new(area_name.clone(), *pos, scanner_id.clone());

        // Simulate scan results based on position and area
        let (x, y) = pos;
        scan_data.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        // Generate varied scan results
        match (x + y + i as i32) % 6 {
            0 => {
                scan_data.scan_result = "clear_area".to_string();
                scan_data.energy_used = 5;
            }
            1 => {
                scan_data.scan_result = "obstacles_detected".to_string();
                scan_data.energy_used = 8;
            }
            2 => {
                scan_data.scan_result = "items_found".to_string();
                scan_data.items_found = vec![format!("item_{}_{}", area_name, i)];
                scan_data.energy_used = 10;
            }
            3 => {
                scan_data.scan_result = "threat_detected".to_string();
                scan_data.threats_detected = 1;
                scan_data.energy_used = 12;
            }
            4 => {
                scan_data.scan_result = "multiple_items".to_string();
                scan_data.items_found = vec![
                    format!("rare_{}_{}", area_name, i),
                    format!("common_{}_{}", area_name, i),
                ];
                scan_data.energy_used = 15;
            }
            _ => {
                scan_data.scan_result = "high_threat_zone".to_string();
                scan_data.threats_detected = 3;
                scan_data.energy_used = 20;
            }
        }

        // Send scan data
        if sender.send(scan_data.clone()).await.is_err() {
            println!("Scanner {}: Channel closed, stopping", scanner_id);
            break;
        }

        scans_completed += 1;
        println!("Scanner {}: Scanned {} at {:?} - {}",
                 scanner_id, area_name, pos, scan_data.scan_result);

        // Small delay between scans
        smol::Timer::after(Duration::from_millis(50)).await;
    }

    println!("Scanner {} completed {} scans in {}", scanner_id, scans_completed, area_name);
    scans_completed
}

/// Coordinator that receives and processes data from multiple scanners
pub async fn exploration_coordinator(receiver: Receiver<AreaScanData>) -> ExplorationStats {
    let mut stats = ExplorationStats::new();
    let mut scan_data = Vec::new();
    let mut active_scanners = std::collections::HashSet::new();

    println!("Exploration coordinator starting...");

    while let Ok(data) = receiver.recv().await {
        active_scanners.insert(data.scanner_id.clone());
        stats.update(&data);

        println!("Coordinator received: {}", data.summary());

        // Process based on scan results
        if data.threats_detected > 0 {
            println!("⚠️  Coordinator: {} threats detected at {:?} by {}",
                     data.threats_detected, data.position, data.scanner_id);
        }

        if !data.items_found.is_empty() {
            println!("📦 Coordinator: {} items found at {:?}: {:?}",
                     data.items_found.len(), data.position, data.items_found);
        }

        if data.scan_result.contains("clear") {
            println!("✅ Coordinator: Clear area confirmed at {:?}", data.position);
        }

        scan_data.push(data);

        // Break after collecting significant data
        if scan_data.len() >= 20 {
            println!("Coordinator: Data collection limit reached");
            break;
        }
    }

    stats.scanners_active = active_scanners.len() as u32;

    // Analyze collected data
    println!("\n=== Exploration Analysis ===");
    println!("Total scans received: {}", stats.total_scans);
    println!("Areas explored: {:?}", stats.areas_explored);
    println!("Total threats found: {}", stats.total_threats);
    println!("Total items found: {}", stats.total_items);
    println!("Total energy used: {}", stats.total_energy_used);
    println!("Active scanners: {}", stats.scanners_active);
    println!("Efficiency rating: {:.2}", stats.efficiency_rating());

    // Find highest priority scans
    scan_data.sort_by(|a, b| b.priority_score().cmp(&a.priority_score()));
    println!("\nTop 3 priority scans:");
    for (i, data) in scan_data.iter().take(3).enumerate() {
        println!("  {}: {} (Priority: {})", i + 1, data.summary(), data.priority_score());
    }

    stats
}

/// Multi-area exploration with multiple scanners
pub async fn multi_area_exploration() -> ExplorationStats {
    let (sender, receiver) = unbounded::<AreaScanData>();

    // Define exploration areas and positions
    let north_area = vec![(1, 1), (2, 1), (3, 1), (4, 1)];
    let south_area = vec![(1, 8), (2, 8), (3, 8)];
    let east_area = vec![(10, 3), (10, 4), (10, 5), (10, 6)];
    let west_area = vec![(0, 3), (0, 4), (0, 5)];
    let central_area = vec![(5, 4), (5, 5), (6, 4), (6, 5)];

    // Spawn multiple area scanners
    let north_scanner = smol::spawn(area_scanner(
        sender.clone(),
        "North".to_string(),
        north_area,
        "scanner_alpha".to_string(),
    ));

    let south_scanner = smol::spawn(area_scanner(
        sender.clone(),
        "South".to_string(),
        south_area,
        "scanner_beta".to_string(),
    ));

    let east_scanner = smol::spawn(area_scanner(
        sender.clone(),
        "East".to_string(),
        east_area,
        "scanner_gamma".to_string(),
    ));

    let west_scanner = smol::spawn(area_scanner(
        sender.clone(),
        "West".to_string(),
        west_area,
        "scanner_delta".to_string(),
    ));

    let central_scanner = smol::spawn(area_scanner(
        sender.clone(),
        "Central".to_string(),
        central_area,
        "scanner_epsilon".to_string(),
    ));

    // Drop the original sender so receiver knows when all are done
    drop(sender);

    // Start coordinator
    let coordinator_task = smol::spawn(exploration_coordinator(receiver));

    // Wait for all scanners to complete
    let scanner_results = future::join(
        future::join(north_scanner, south_scanner),
        future::join(future::join(east_scanner, west_scanner), central_scanner)
    ).await;

    let ((north_scans, south_scans), ((east_scans, west_scans), central_scans)) = scanner_results;

    println!("\n=== Scanner Results ===");
    println!("North scanner: {} scans", north_scans);
    println!("South scanner: {} scans", south_scans);
    println!("East scanner: {} scans", east_scans);
    println!("West scanner: {} scans", west_scans);
    println!("Central scanner: {} scans", central_scans);

    // Get coordinator results
    let stats = coordinator_task.await;
    let total_scans = north_scans + south_scans + east_scans + west_scans + central_scans;
    println!("Total scans by scanners: {}", total_scans);

    stats
}

/// High-frequency data collection system
pub async fn high_frequency_data_collection() -> (u32, u32, f32) {
    let (sender, receiver) = bounded::<AreaScanData>(50); // Buffered channel

    // Fast data producer
    let producer1 = {
        let sender = sender.clone();
        smol::spawn(async move {
            let mut count = 0;
            for i in 0..15 {
                let data = AreaScanData {
                    area_name: "HighFreq1".to_string(),
                    position: (i % 5, i / 5),
                    scan_result: format!("rapid_scan_{}", i),
                    timestamp: i as u64,
                    scanner_id: "rapid_scanner_1".to_string(),
                    energy_used: 3,
                    threats_detected: if i % 7 == 0 { 1 } else { 0 },
                    items_found: if i % 4 == 0 { vec![format!("fast_item_{}", i)] } else { Vec::new() },
                };

                if sender.send(data).await.is_err() {
                    break;
                }
                count += 1;
                smol::Timer::after(Duration::from_millis(50)).await;
            }
            count
        })
    };

    // Another fast producer
    let producer2 = {
        let sender = sender.clone();
        smol::spawn(async move {
            let mut count = 0;
            for i in 0..12 {
                let data = AreaScanData {
                    area_name: "HighFreq2".to_string(),
                    position: ((i + 5) % 6, (i + 2) / 3),
                    scan_result: format!("burst_scan_{}", i),
                    timestamp: (i + 100) as u64,
                    scanner_id: "rapid_scanner_2".to_string(),
                    energy_used: 4,
                    threats_detected: if i % 5 == 0 { 2 } else { 0 },
                    items_found: if i % 3 == 0 { vec![format!("burst_item_{}", i)] } else { Vec::new() },
                };

                if sender.send(data).await.is_err() {
                    break;
                }
                count += 1;
                smol::Timer::after(Duration::from_millis(80)).await;
            }
            count
        })
    };

    drop(sender);

    // High-speed consumer
    let consumer = smol::spawn(async move {
        let mut received_count = 0;
        let mut total_energy = 0;
        let start_time = std::time::Instant::now();

        while let Ok(data) = receiver.recv().await {
            received_count += 1;
            total_energy += data.energy_used;

            if received_count % 5 == 0 {
                println!("High-freq consumer: Processed {} messages", received_count);
            }
        }

        let processing_time = start_time.elapsed().as_millis() as f32;
        let throughput = received_count as f32 / (processing_time / 1000.0);

        println!("High-frequency processing complete:");
        println!("  Messages processed: {}", received_count);
        println!("  Total energy tracked: {}", total_energy);
        println!("  Processing time: {:.2}ms", processing_time);
        println!("  Throughput: {:.2} messages/second", throughput);

        (received_count, total_energy, throughput)
    });

    // Wait for producers and consumer
    let ((count1, count2), (received, energy, throughput)) = future::join(
        future::join(producer1, producer2),
        consumer
    ).await;

    println!("High-frequency collection: {} + {} sent, {} received",
             count1, count2, received);

    (received, energy, throughput)
}

/// Priority-based multi-producer system
pub async fn priority_based_collection() -> Vec<AreaScanData> {
    let (sender, receiver) = unbounded::<AreaScanData>();

    // Critical area scanner (high priority data)
    let critical_scanner = {
        let sender = sender.clone();
        smol::spawn(async move {
            for i in 0..4 {
                let data = AreaScanData {
                    area_name: "Critical".to_string(),
                    position: (i, 0),
                    scan_result: "security_scan".to_string(),
                    timestamp: i as u64,
                    scanner_id: "critical_scanner".to_string(),
                    energy_used: 8,
                    threats_detected: 2 + i as u32,
                    items_found: vec![format!("critical_item_{}", i), format!("secure_data_{}", i)],
                };

                sender.send(data).await.ok();
                smol::Timer::after(Duration::from_millis(300)).await;
            }
        })
    };

    // Routine area scanner (normal priority data)
    let routine_scanner = {
        let sender = sender.clone();
        smol::spawn(async move {
            for i in 0..6 {
                let data = AreaScanData {
                    area_name: "Routine".to_string(),
                    position: (i % 3, i / 3),
                    scan_result: "standard_scan".to_string(),
                    timestamp: (i + 50) as u64,
                    scanner_id: "routine_scanner".to_string(),
                    energy_used: 5,
                    threats_detected: if i % 4 == 0 { 1 } else { 0 },
                    items_found: if i % 3 == 0 { vec![format!("routine_item_{}", i)] } else { Vec::new() },
                };

                sender.send(data).await.ok();
                smol::Timer::after(Duration::from_millis(200)).await;
            }
        })
    };

    // Surveillance scanner (low priority, background data)
    let surveillance_scanner = {
        let sender = sender.clone();
        smol::spawn(async move {
            for i in 0..8 {
                let data = AreaScanData {
                    area_name: "Surveillance".to_string(),
                    position: (i % 4, 10),
                    scan_result: "background_scan".to_string(),
                    timestamp: (i + 200) as u64,
                    scanner_id: "surveillance_scanner".to_string(),
                    energy_used: 2,
                    threats_detected: 0,
                    items_found: Vec::new(),
                };

                sender.send(data).await.ok();
                smol::Timer::after(Duration::from_millis(150)).await;
            }
        })
    };

    drop(sender);

    // Priority-aware consumer
    let consumer = smol::spawn(async move {
        let mut all_data = Vec::new();

        while let Ok(data) = receiver.recv().await {
            all_data.push(data);
        }

        // Sort by priority score (highest first)
        all_data.sort_by(|a, b| b.priority_score().cmp(&a.priority_score()));

        println!("Priority-based collection complete. Sorted by priority:");
        for (i, data) in all_data.iter().take(5).enumerate() {
            println!("  {}: {} [Priority: {}]", i + 1, data.summary(), data.priority_score());
        }

        all_data
    });

    // Wait for all tasks
    future::join(
        future::join(critical_scanner, routine_scanner),
        surveillance_scanner
    ).await;

    consumer.await
}

/// Load balancing demonstration
pub async fn load_balanced_scanning() -> HashMap<String, u32> {
    let (sender, receiver) = bounded::<AreaScanData>(20);

    // Light-load scanner
    let light_scanner = {
        let sender = sender.clone();
        smol::spawn(async move {
            let mut count = 0;
            for i in 0..3 {
                let data = AreaScanData::new("Light".to_string(), (i, 0), "light_scanner".to_string());
                if sender.send(data).await.is_ok() {
                    count += 1;
                }
                smol::Timer::after(Duration::from_millis(400)).await;
            }
            ("light_scanner".to_string(), count)
        })
    };

    // Medium-load scanner
    let medium_scanner = {
        let sender = sender.clone();
        smol::spawn(async move {
            let mut count = 0;
            for i in 0..5 {
                let data = AreaScanData::new("Medium".to_string(), (i, 1), "medium_scanner".to_string());
                if sender.send(data).await.is_ok() {
                    count += 1;
                }
                smol::Timer::after(Duration::from_millis(250)).await;
            }
            ("medium_scanner".to_string(), count)
        })
    };

    // Heavy-load scanner
    let heavy_scanner = {
        let sender = sender.clone();
        smol::spawn(async move {
            let mut count = 0;
            for i in 0..8 {
                let data = AreaScanData::new("Heavy".to_string(), (i, 2), "heavy_scanner".to_string());
                if sender.send(data).await.is_ok() {
                    count += 1;
                }
                smol::Timer::after(Duration::from_millis(150)).await;
            }
            ("heavy_scanner".to_string(), count)
        })
    };

    drop(sender);

    // Load tracking consumer
    let consumer = smol::spawn(async move {
        let mut scanner_loads = HashMap::new();

        while let Ok(data) = receiver.recv().await {
            *scanner_loads.entry(data.scanner_id.clone()).or_insert(0) += 1;
            println!("Load balancer: Received from {} (Total from this scanner: {})",
                     data.scanner_id, scanner_loads[&data.scanner_id]);
        }

        scanner_loads
    });

    // Wait for scanners
    let scanner_results = future::join(
        future::join(light_scanner, medium_scanner),
        heavy_scanner
    ).await;

    let ((light_result, medium_result), heavy_result) = scanner_results;

    println!("Load balancing results:");
    println!("  {}: {} messages sent", light_result.0, light_result.1);
    println!("  {}: {} messages sent", medium_result.0, medium_result.1);
    println!("  {}: {} messages sent", heavy_result.0, heavy_result.1);

    consumer.await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_scan_data_creation() {
        let data = AreaScanData::new("TestArea".to_string(), (5, 3), "test_scanner".to_string());
        assert_eq!(data.area_name, "TestArea");
        assert_eq!(data.position, (5, 3));
        assert_eq!(data.scanner_id, "test_scanner");
        assert_eq!(data.energy_used, 0);
        assert_eq!(data.threats_detected, 0);
        assert_eq!(data.items_found.len(), 0);
    }

    #[test]
    fn test_area_scan_data_priority() {
        let mut data1 = AreaScanData::new("Area1".to_string(), (0, 0), "scanner1".to_string());
        data1.threats_detected = 2;
        data1.items_found = vec!["item1".to_string()];
        data1.energy_used = 10;

        let mut data2 = AreaScanData::new("Area2".to_string(), (1, 1), "scanner2".to_string());
        data2.threats_detected = 1;
        data2.energy_used = 5;

        assert!(data1.priority_score() > data2.priority_score());
    }

    #[test]
    fn test_exploration_stats() {
        let mut stats = ExplorationStats::new();
        assert_eq!(stats.total_scans, 0);
        assert_eq!(stats.total_threats, 0);
        assert_eq!(stats.total_items, 0);

        let mut data = AreaScanData::new("TestArea".to_string(), (0, 0), "test".to_string());
        data.threats_detected = 2;
        data.items_found = vec!["item1".to_string(), "item2".to_string()];
        data.energy_used = 10;

        stats.update(&data);
        assert_eq!(stats.total_scans, 1);
        assert_eq!(stats.total_threats, 2);
        assert_eq!(stats.total_items, 2);
        assert_eq!(stats.total_energy_used, 10);
        assert_eq!(stats.areas_explored["TestArea"], 1);
    }

    #[test]
    fn test_exploration_stats_efficiency() {
        let mut stats = ExplorationStats::new();

        // High-value, low-energy scan
        let mut good_data = AreaScanData::new("Good".to_string(), (0, 0), "test".to_string());
        good_data.items_found = vec!["valuable_item".to_string()];
        good_data.threats_detected = 1;
        good_data.energy_used = 5;
        stats.update(&good_data);

        let efficiency = stats.efficiency_rating();
        assert!(efficiency > 0.0);

        // Zero energy case
        let zero_energy_stats = ExplorationStats::new();
        assert_eq!(zero_energy_stats.efficiency_rating(), 100.0);
    }

    #[smol_potat::test]
    async fn test_area_scanner() {
        let (sender, receiver) = unbounded::<AreaScanData>();
        let positions = vec![(1, 1), (2, 2), (3, 3)];

        let scanner_task = area_scanner(
            sender,
            "TestArea".to_string(),
            positions.clone(),
            "test_scanner".to_string(),
        );

        let receiver_task = smol::spawn(async move {
            let mut received = Vec::new();
            while let Ok(data) = receiver.recv().await {
                received.push(data);
            }
            received
        });

        let scans_completed = scanner_task.await;
        let received_data = receiver_task.await;

        assert_eq!(scans_completed, positions.len() as u32);
        assert_eq!(received_data.len(), positions.len());

        for (i, data) in received_data.iter().enumerate() {
            assert_eq!(data.area_name, "TestArea");
            assert_eq!(data.position, positions[i]);
            assert_eq!(data.scanner_id, "test_scanner");
        }
    }

    #[smol_potat::test]
    async fn test_exploration_coordinator() {
        let (sender, receiver) = unbounded::<AreaScanData>();

        // Send test data
        let coordinator_task = smol::spawn(exploration_coordinator(receiver));

        let sender_task = smol::spawn(async move {
            for i in 0..5 {
                let mut data = AreaScanData::new(
                    format!("Area{}", i),
                    (i, i),
                    format!("scanner_{}", i),
                );
                data.threats_detected = i as u32 % 2;
                data.items_found = if i % 2 == 0 { vec![format!("item_{}", i)] } else { Vec::new() };
                data.energy_used = 5 + i as u32;

                if sender.send(data).await.is_err() {
                    break;
                }
            }
        });

        sender_task.await;
        drop(sender);

        let stats = coordinator_task.await;
        assert_eq!(stats.total_scans, 5);
        assert!(stats.total_energy_used > 0);
        assert!(stats.areas_explored.len() > 0);
    }

    #[smol_potat::test]
    async fn test_multi_area_exploration() {
        let stats = multi_area_exploration().await;

        assert!(stats.total_scans > 0);
        assert!(stats.scanners_active > 0);
        assert!(stats.areas_explored.len() > 0);
        assert!(stats.total_energy_used > 0);

        // Should have multiple areas
        assert!(stats.areas_explored.contains_key("North"));
        assert!(stats.areas_explored.contains_key("South"));
        assert!(stats.areas_explored.contains_key("East"));
        assert!(stats.areas_explored.contains_key("West"));
        assert!(stats.areas_explored.contains_key("Central"));
    }

    #[smol_potat::test]
    async fn test_high_frequency_data_collection() {
        let (received, energy, throughput) = high_frequency_data_collection().await;

        assert!(received > 0);
        assert!(energy > 0);
        assert!(throughput > 0.0);
        // Should handle high-frequency data efficiently
        assert!(received >= 20); // Should receive most/all messages
    }

    #[smol_potat::test]
    async fn test_priority_based_collection() {
        let sorted_data = priority_based_collection().await;

        assert!(!sorted_data.is_empty());

        // Verify priority ordering (highest first)
        for window in sorted_data.windows(2) {
            assert!(window[0].priority_score() >= window[1].priority_score());
        }

        // Critical scanner data should have highest priority
        let first_item = &sorted_data[0];
        assert!(first_item.area_name == "Critical" || first_item.priority_score() > 100);
    }

    #[smol_potat::test]
    async fn test_load_balanced_scanning() {
        let scanner_loads = load_balanced_scanning().await;

        assert!(!scanner_loads.is_empty());
        assert!(scanner_loads.contains_key("light_scanner"));
        assert!(scanner_loads.contains_key("medium_scanner"));
        assert!(scanner_loads.contains_key("heavy_scanner"));

        // Heavy scanner should have sent the most messages
        let heavy_count = scanner_loads.get("heavy_scanner").unwrap_or(&0);
        let light_count = scanner_loads.get("light_scanner").unwrap_or(&0);
        assert!(heavy_count >= light_count);
    }

    #[smol_potat::test]
    async fn test_channel_capacity_handling() {
        let (sender, receiver) = bounded::<AreaScanData>(3);

        // Send more than capacity
        let sender_task = smol::spawn(async move {
            let mut sent = 0;
            for i in 0..6 {
                let data = AreaScanData::new(
                    "CapacityTest".to_string(),
                    (i, 0),
                    "capacity_scanner".to_string(),
                );

                if sender.send(data).await.is_ok() {
                    sent += 1;
                }
            }
            sent
        });

        // Receive with delay to test buffering
        let receiver_task = smol::spawn(async move {
            let mut received = 0;
            while let Ok(_) = receiver.recv().await {
                received += 1;
                smol::Timer::after(Duration::from_millis(100)).await;
            }
            received
        });

        let sent = sender_task.await;
        let received = receiver_task.await;

        assert!(sent > 0);
        assert_eq!(sent, received); // All sent messages should be received
    }

    #[smol_potat::test]
    async fn test_producer_consumer_synchronization() {
        let (sender, receiver) = unbounded::<AreaScanData>();

        // Multiple producers
        let producer1 = {
            let sender = sender.clone();
            smol::spawn(async move {
                for i in 0..3 {
                    let data = AreaScanData::new("Sync1".to_string(), (i, 0), "sync_scanner_1".to_string());
                    sender.send(data).await.ok();
                    smol::Timer::after(Duration::from_millis(100)).await;
                }
                3
            })
        };

        let producer2 = {
            let sender = sender.clone();
            smol::spawn(async move {
                for i in 0..2 {
                    let data = AreaScanData::new("Sync2".to_string(), (i, 1), "sync_scanner_2".to_string());
                    sender.send(data).await.ok();
                    smol::Timer::after(Duration::from_millis(150)).await;
                }
                2
            })
        };

        drop(sender);

        // Consumer
        let consumer = smol::spawn(async move {
            let mut count = 0;
            let mut area1_count = 0;
            let mut area2_count = 0;

            while let Ok(data) = receiver.recv().await {
                count += 1;
                if data.area_name == "Sync1" {
                    area1_count += 1;
                } else if data.area_name == "Sync2" {
                    area2_count += 1;
                }
            }

            (count, area1_count, area2_count)
        });

        let (sent1, sent2) = future::join(producer1, producer2).await;
        let (total_received, area1_received, area2_received) = consumer.await;

        assert_eq!(sent1 + sent2, total_received);
        assert_eq!(sent1, area1_received);
        assert_eq!(sent2, area2_received);
    }
}

/// Example usage and demonstrations
pub async fn demonstrate_multi_producer_consumer() {
    println!("=== Level 13 Task 3: Multi-Producer Consumer Demo ===");

    // Test basic multi-area exploration
    println!("\n1. Testing multi-area exploration...");
    let exploration_stats = multi_area_exploration().await;
    println!("   Exploration efficiency: {:.2}%", exploration_stats.efficiency_rating());
    println!("   Total scanners active: {}", exploration_stats.scanners_active);

    // Test high-frequency data collection
    println!("\n2. Testing high-frequency data collection...");
    let (messages, energy, throughput) = high_frequency_data_collection().await;
    println!("   Messages processed: {}", messages);
    println!("   Total energy tracked: {}", energy);
    println!("   Throughput: {:.2} msg/sec", throughput);

    // Test priority-based collection
    println!("\n3. Testing priority-based data collection...");
    let priority_data = priority_based_collection().await;
    println!("   Total priority items: {}", priority_data.len());
    if let Some(highest) = priority_data.first() {
        println!("   Highest priority: {} (Score: {})", highest.summary(), highest.priority_score());
    }

    // Test load balancing
    println!("\n4. Testing load-balanced scanning...");
    let load_stats = load_balanced_scanning().await;
    println!("   Scanner loads: {:?}", load_stats);
    let total_load: u32 = load_stats.values().sum();
    println!("   Total messages processed: {}", total_load);

    println!("\n✅ Multi-producer consumer demonstration complete!");
}

#[smol_potat::main]
async fn main() {
    demonstrate_multi_producer_consumer().await;
}