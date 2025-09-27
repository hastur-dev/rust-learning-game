// Level 9: Level 9: Collections - Automated Test Solutions
// Updated to match actual learning tests

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_9_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 9: Collections",
        level_index: 8,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Vector Basics",
                solution_code: r#"fn main() {
    let mut items = Vec::new();

    items.push("Battery".to_string());
    items.push("Key".to_string());
    items.push("Tool".to_string());

    println!("Inventory has {} items", items.len());

    for (index, item) in items.iter().enumerate() {
        println!("Item {}: {}", index, item);
    }

    if let Some(first_item) = items.first() {
        println!("First item: {}", first_item);
    }

    items.remove(1);
    println!("After removing item at index 1: {} items remaining", items.len());
}"#,
                completion_indicators: vec![
                    "Inventory has 3 items", "Item 0: Battery", "Item 1: Key", "Item 2: Tool", "First item: Battery", "After removing item at index 1: 2 items remaining"
                ],
            },
            TaskTest {
                task_number: 2,
                task_name: "HashMap Storage",
                solution_code: r#"use std::collections::HashMap;

fn main() {
    let mut robot_stats = HashMap::new();

    robot_stats.insert("health".to_string(), 100);
    robot_stats.insert("energy".to_string(), 85);
    robot_stats.insert("ammo".to_string(), 50);
    robot_stats.insert("shields".to_string(), 75);

    println!("Robot stats initialized with {} parameters", robot_stats.len());

    if let Some(health) = robot_stats.get("health") {
        println!("Current health: {}", health);
    }

    robot_stats.insert("health".to_string(), 90);
    println!("Health updated to: {}", robot_stats["health"]);

    for (stat, value) in &robot_stats {
        println!("{}: {}", stat, value);
    }

    if robot_stats.contains_key("energy") {
        println!("Energy system online");
    }
}"#,
                completion_indicators: vec![
                    "Robot stats initialized with 4 parameters", "Current health: 100", "Health updated to: 90", "Energy system online"
                ],
            },
            TaskTest {
                task_number: 3,
                task_name: "HashSet Operations",
                solution_code: r#"use std::collections::HashSet;

fn main() {
    let mut visited_positions = HashSet::new();
    let mut collectible_items = HashSet::new();

    visited_positions.insert("0,0".to_string());
    visited_positions.insert("1,0".to_string());
    visited_positions.insert("1,1".to_string());
    visited_positions.insert("0,1".to_string());

    collectible_items.insert("key".to_string());
    collectible_items.insert("battery".to_string());
    collectible_items.insert("tool".to_string());

    println!("Visited {} unique positions", visited_positions.len());
    println!("Found {} types of collectible items", collectible_items.len());

    if visited_positions.contains("1,1") {
        println!("Position (1,1) has been explored");
    }

    let new_items: HashSet<String> = ["key", "gem", "coin"].iter().map(|s| s.to_string()).collect();
    let common_items: HashSet<_> = collectible_items.intersection(&new_items).collect();
    println!("Common items found: {}", common_items.len());

    collectible_items.extend(new_items);
    println!("Total unique item types: {}", collectible_items.len());
}"#,
                completion_indicators: vec![
                    "Visited 4 unique positions", "Found 3 types of collectible items", "Position (1,1) has been explored", "Common items found: 1", "Total unique item types: 5"
                ],
            },
            TaskTest {
                task_number: 4,
                task_name: "Iterator Chains",
                solution_code: r#"fn main() {
    let sensor_readings = vec![15, 23, 8, 42, 16, 31, 7, 29, 18, 35];

    println!("Processing {} sensor readings", sensor_readings.len());

    let high_readings: Vec<i32> = sensor_readings
        .iter()
        .filter(|&&x| x > 20)
        .map(|&x| x)
        .collect();

    println!("High readings (>20): {:?}", high_readings);

    let sum: i32 = sensor_readings.iter().sum();
    let average = sum as f64 / sensor_readings.len() as f64;
    println!("Average reading: {:.2}", average);

    let max_reading = sensor_readings.iter().max();
    if let Some(max) = max_reading {
        println!("Maximum reading: {}", max);
    }

    let processed_readings: Vec<String> = sensor_readings
        .iter()
        .enumerate()
        .map(|(i, &value)| format!("Sensor {}: {}", i + 1, value))
        .collect();

    println!("Processed {} readings", processed_readings.len());

    for reading in processed_readings.iter().take(3) {
        println!("{}", reading);
    }
}"#,
                completion_indicators: vec![
                    "Processing 10 sensor readings", "High readings (>20):", "Average reading:", "Maximum reading: 42", "Processed 10 readings", "Sensor 1: 15", "Sensor 2: 23", "Sensor 3: 8"
                ],
            },
            TaskTest {
                task_number: 5,
                task_name: "Advanced Collections",
                solution_code: r#"use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct GameState {
    player_inventory: Vec<String>,
    world_map: HashMap<String, Vec<String>>,
    discovered_locations: HashSet<String>,
    quest_log: Vec<(String, bool)>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            player_inventory: Vec::new(),
            world_map: HashMap::new(),
            discovered_locations: HashSet::new(),
            quest_log: Vec::new(),
        }
    }

    fn add_location(&mut self, location: String, items: Vec<String>) {
        self.world_map.insert(location.clone(), items);
        self.discovered_locations.insert(location);
    }

    fn collect_item(&mut self, item: String) {
        self.player_inventory.push(item);
    }

    fn add_quest(&mut self, quest: String) {
        self.quest_log.push((quest, false));
    }

    fn complete_quest(&mut self, quest_name: &str) -> bool {
        for (quest, completed) in &mut self.quest_log {
            if quest == quest_name {
                *completed = true;
                return true;
            }
        }
        false
    }

    fn get_summary(&self) -> String {
        let completed_quests = self.quest_log.iter().filter(|(_, completed)| *completed).count();
        format!("Inventory: {} items, Locations: {}, Quests: {}/{}",
                self.player_inventory.len(),
                self.discovered_locations.len(),
                completed_quests,
                self.quest_log.len())
    }
}

fn main() {
    let mut game = GameState::new();

    game.add_location("Forest".to_string(), vec!["Berry".to_string(), "Stick".to_string()]);
    game.add_location("Cave".to_string(), vec!["Crystal".to_string(), "Gold".to_string()]);
    game.add_location("Village".to_string(), vec!["Bread".to_string()]);

    game.collect_item("Sword".to_string());
    game.collect_item("Shield".to_string());
    game.collect_item("Potion".to_string());

    game.add_quest("Find the Crystal".to_string());
    game.add_quest("Help the Village".to_string());
    game.add_quest("Explore the Forest".to_string());

    game.complete_quest("Find the Crystal");

    println!("Game initialized successfully");
    println!("{}", game.get_summary());

    let total_world_items: usize = game.world_map.values().map(|items| items.len()).sum();
    println!("Total items in world: {}", total_world_items);

    println!("Discovered locations: {:?}", game.discovered_locations);
}"#,
                completion_indicators: vec![
                    "Game initialized successfully", "Inventory: 3 items, Locations: 3, Quests: 1/3", "Total items in world: 5", "Discovered locations:"
                ],
            }
        ],
    }
}