use crate::item::Pos;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Upgrades {
    pub grabber_level: u32, // manhattan range
    pub scanner_level: u32, // contiguous scan length; 0 = not owned
    pub time_slow_available: bool, // unlocked after Level 4
    pub attack_range: u32 // contiguous attack length; 0 = not owned
}

#[derive(Clone, Debug)]
pub struct Robot {
    pos: Pos,
    pub upgrades: Upgrades,
    pub inventory: HashSet<String>, // item names
    pub auto_grab_enabled: bool,
}

impl Robot {
    pub fn new(start_pos: (i32, i32)) -> Self {
        Self {
            pos: Pos { x: start_pos.0, y: start_pos.1 },
            upgrades: Upgrades { 
                grabber_level: 1, 
                scanner_level: 0, 
                attack_range: 0, 
                time_slow_available: false 
            },
            inventory: HashSet::new(),
            auto_grab_enabled: false,
        }
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.pos.x, self.pos.y)
    }

    pub fn get_pos(&self) -> Pos {
        self.pos
    }

    pub fn set_position(&mut self, new_pos: (i32, i32)) {
        self.pos = Pos { x: new_pos.0, y: new_pos.1 };
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.pos.x += dx;
        self.pos.y += dy;
    }

    pub fn move_to(&mut self, target: Pos) {
        self.pos = target;
    }

    pub fn add_to_inventory(&mut self, item_name: String) {
        self.inventory.insert(item_name);
    }

    pub fn has_item(&self, item_name: &str) -> bool {
        self.inventory.contains(item_name)
    }

    pub fn remove_from_inventory(&mut self, item_name: &str) -> bool {
        self.inventory.remove(item_name)
    }

    pub fn get_grabber_range(&self) -> i32 {
        self.upgrades.grabber_level as i32
    }

    pub fn get_scanner_range(&self) -> i32 {
        2 // Always 2-tile range for the new design
    }

    pub fn upgrade_grabber(&mut self) {
        self.upgrades.grabber_level += 1;
    }

    pub fn upgrade_scanner(&mut self) {
        self.upgrades.scanner_level += 1;
    }

    pub fn set_scanner_level(&mut self, level: u32) {
        self.upgrades.scanner_level = level;
        if level > 0 {
            self.inventory.insert("scanner".to_string());
        }
    }

    pub fn get_inventory_items(&self) -> Vec<String> {
        self.inventory.iter().cloned().collect()
    }

    pub fn clear_inventory(&mut self) {
        self.inventory.clear();
    }

    // Calculate Manhattan distance from robot to target
    pub fn distance_to(&self, target: Pos) -> i32 {
        (self.pos.x - target.x).abs() + (self.pos.y - target.y).abs()
    }

    // Check if target is within grabber range
    pub fn can_grab_at(&self, target: Pos) -> bool {
        self.distance_to(target) <= self.get_grabber_range()
    }

    // Get all positions within grabber range
    pub fn get_grabber_positions(&self, grid_width: i32, grid_height: i32) -> Vec<Pos> {
        let mut positions = Vec::new();
        let range = self.get_grabber_range();
        
        for y in (self.pos.y - range).max(0)..=(self.pos.y + range).min(grid_height - 1) {
            for x in (self.pos.x - range).max(0)..=(self.pos.x + range).min(grid_width - 1) {
                let pos = Pos { x, y };
                if self.distance_to(pos) <= range {
                    positions.push(pos);
                }
            }
        }
        
        positions
    }

    // Get positions in scanner direction
    pub fn get_scanner_positions(&self, direction: (i32, i32), grid_width: i32, grid_height: i32) -> Vec<Pos> {
        let mut positions = Vec::new();
        let range = self.get_scanner_range();
        
        if range == 0 {
            return positions;
        }
        
        let mut current = self.pos;
        for _ in 0..range {
            let next = Pos { 
                x: current.x + direction.0, 
                y: current.y + direction.1 
            };
            
            if next.x >= 0 && next.y >= 0 && next.x < grid_width && next.y < grid_height {
                positions.push(next);
                current = next;
            } else {
                break;
            }
        }
        
        positions
    }

    // Toggle auto-grab
    pub fn toggle_auto_grab(&mut self) {
        self.auto_grab_enabled = !self.auto_grab_enabled;
    }

    pub fn set_auto_grab(&mut self, enabled: bool) {
        self.auto_grab_enabled = enabled;
    }

    // Check if robot has scanner capability
    pub fn has_scanner(&self) -> bool {
        true // Scanner is always available in the new design
    }
}