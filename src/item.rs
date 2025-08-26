use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    pub pos: Pos,
    pub capabilities: ItemCapabilities,
    pub collected: bool,
}

#[derive(Clone, Debug)]
pub struct ItemCapabilities {
    pub scanner_range: Option<u32>,
    pub grabber_boost: Option<u32>,
    pub credits_value: Option<u32>,
    pub time_slow_duration: Option<u32>, // Milliseconds between actions
    pub special_functions: Vec<String>,
    pub rust_code: Option<String>, // Raw Rust code for advanced items
}

impl Default for ItemCapabilities {
    fn default() -> Self {
        Self {
            scanner_range: None,
            grabber_boost: None,
            credits_value: Some(1), // Default credit value
            time_slow_duration: None,
            special_functions: Vec::new(),
            rust_code: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ItemManager {
    pub items: Vec<Item>,
    pub collected_items: HashSet<String>,
}

impl ItemManager {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            collected_items: HashSet::new(),
        }
    }

    pub fn add_item(&mut self, name: String, pos: Pos, item_file_path: Option<String>) {
        let capabilities = if let Some(file_path) = item_file_path {
            Self::load_item_capabilities(&file_path).unwrap_or_default()
        } else {
            ItemCapabilities::default()
        };

        let item = Item {
            name: name.clone(),
            pos,
            capabilities,
            collected: false,
        };

        self.items.push(item);
    }

    pub fn collect_item(&mut self, pos: Pos) -> Option<Item> {
        if let Some(index) = self.items.iter().position(|item| item.pos == pos && !item.collected) {
            let mut item = self.items[index].clone();
            item.collected = true;
            self.items[index].collected = true;
            self.collected_items.insert(item.name.clone());
            Some(item)
        } else {
            None
        }
    }

    pub fn get_item_at_position(&self, pos: Pos) -> Option<&Item> {
        self.items.iter().find(|item| item.pos == pos && !item.collected)
    }

    pub fn has_collected(&self, item_name: &str) -> bool {
        self.collected_items.contains(item_name)
    }

    pub fn get_collected_items(&self) -> Vec<&Item> {
        self.items.iter().filter(|item| item.collected).collect()
    }

    pub fn get_active_items(&self) -> Vec<&Item> {
        self.items.iter().filter(|item| !item.collected).collect()
    }
    
    pub fn add_dummy_item(&mut self, name: &str) {
        // Add a dummy collected item for tracking purposes (like tutorial shown flags)
        self.collected_items.insert(name.to_string());
    }

    fn load_item_capabilities(file_path: &str) -> Result<ItemCapabilities, Box<dyn std::error::Error>> {
        if !Path::new(file_path).exists() {
            return Ok(ItemCapabilities::default());
        }

        let content = fs::read_to_string(file_path)?;
        
        // Parse the Rust file for specific patterns
        // This is a simple parser - in a real implementation you might use syn crate
        let mut capabilities = ItemCapabilities::default();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Look for capability comments
            if line.starts_with("// CAPABILITY:") {
                let capability_str = line.strip_prefix("// CAPABILITY:").unwrap().trim();
                Self::parse_capability_line(capability_str, &mut capabilities);
            }
            
            // Look for function definitions
            if line.starts_with("pub fn ") || line.starts_with("fn ") {
                if let Some(func_name) = Self::extract_function_name(line) {
                    capabilities.special_functions.push(func_name);
                }
            }
        }
        
        capabilities.rust_code = Some(content);
        Ok(capabilities)
    }

    fn parse_capability_line(line: &str, capabilities: &mut ItemCapabilities) {
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            match parts[0].to_lowercase().as_str() {
                "scanner_range" => {
                    if let Ok(range) = parts[1].parse::<u32>() {
                        capabilities.scanner_range = Some(range);
                    }
                }
                "grabber_boost" => {
                    if let Ok(boost) = parts[1].parse::<u32>() {
                        capabilities.grabber_boost = Some(boost);
                    }
                }
                "credits_value" => {
                    if let Ok(value) = parts[1].parse::<u32>() {
                        capabilities.credits_value = Some(value);
                    }
                }
                "time_slow_duration" => {
                    if let Ok(duration) = parts[1].parse::<u32>() {
                        capabilities.time_slow_duration = Some(duration);
                    }
                }
                _ => {}
            }
        }
    }

    fn extract_function_name(line: &str) -> Option<String> {
        // Extract function name from "fn name(" or "pub fn name("
        let line = line.trim_start_matches("pub ");
        if let Some(fn_start) = line.find("fn ") {
            let after_fn = &line[fn_start + 3..];
            if let Some(paren_pos) = after_fn.find('(') {
                let func_name = after_fn[..paren_pos].trim();
                return Some(func_name.to_string());
            }
        }
        None
    }
}

// Standard item types
pub fn create_scanner_item(pos: Pos) -> Item {
    Item {
        name: "scanner".to_string(),
        pos,
        capabilities: ItemCapabilities {
            scanner_range: Some(1),
            grabber_boost: None,
            credits_value: Some(5),
            time_slow_duration: None,
            special_functions: vec!["scan".to_string()],
            rust_code: None,
        },
        collected: false,
    }
}

pub fn create_grabber_upgrade(pos: Pos) -> Item {
    Item {
        name: "grabber_upgrade".to_string(),
        pos,
        capabilities: ItemCapabilities {
            scanner_range: None,
            grabber_boost: Some(1),
            credits_value: Some(3),
            time_slow_duration: None,
            special_functions: Vec::new(),
            rust_code: None,
        },
        collected: false,
    }
}

pub fn create_credit_gem(pos: Pos, value: u32) -> Item {
    Item {
        name: format!("credit_gem_{}", value),
        pos,
        capabilities: ItemCapabilities {
            scanner_range: None,
            grabber_boost: None,
            credits_value: Some(value),
            time_slow_duration: None,
            special_functions: Vec::new(),
            rust_code: None,
        },
        collected: false,
    }
}

pub fn create_time_slow_item(pos: Pos, duration_ms: u32) -> Item {
    Item {
        name: "time_slow".to_string(),
        pos,
        capabilities: ItemCapabilities {
            scanner_range: None,
            grabber_boost: None,
            credits_value: Some(25),
            time_slow_duration: Some(duration_ms),
            special_functions: vec!["time_slow".to_string()],
            rust_code: None,
        },
        collected: false,
    }
}