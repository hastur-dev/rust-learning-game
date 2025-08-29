use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use rand::Rng;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YamlLevelConfig {
    pub name: String,
    pub grid_size: String, // Format: "WxH" like "16x10"
    pub obstacles: Option<u32>, // Number of random obstacles to place
    pub enemies: Option<Vec<EnemyConfig>>,
    pub items: Option<Vec<ItemConfig>>,
    pub income_per_square: Option<u32>,
    pub start_position: Option<(u32, u32)>,
    pub max_turns: Option<u32>,
    pub fog_of_war: Option<bool>,
    pub message: Option<String>, // Popup message shown at level start
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemyConfig {
    pub start_location: (u32, u32),
    pub movement_pattern: String, // "horizontal", "vertical", or "file:path/to/pattern.rs"
    pub moving_positive: Option<bool>, // true = right/down, false = left/up
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemConfig {
    pub name: String,
    pub item_file: String, // Path to rust file with item capabilities
    pub spawn_randomly: Option<bool>, // If true, spawned randomly; if false, placed at specific location
    pub location: Option<(u32, u32)>, // Specific location if spawn_randomly is false
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LevelSpec {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub start: (usize, usize),
    pub scanner_at: Option<(usize, usize)>,
    pub blockers: Vec<(usize, usize)>,
    pub enemies: Vec<EnemySpec>,
    pub items: Vec<ItemSpec>,
    pub fog_of_war: bool,
    pub max_turns: usize,
    pub income_per_square: u32,
    pub message: Option<String>, // Popup message shown at level start
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemySpec {
    pub pos: (i32, i32),
    pub direction: EnemyDirection,
    pub moving_positive: bool,
    pub movement_pattern: Option<String>, // For custom movement patterns
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnemyDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemSpec {
    pub name: String,
    pub pos: Option<(i32, i32)>,
    pub capabilities: HashMap<String, serde_yaml::Value>,
}

impl YamlLevelConfig {
    pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: YamlLevelConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn to_level_spec<R: Rng>(&self, rng: &mut R) -> Result<LevelSpec, Box<dyn std::error::Error>> {
        // Parse grid size
        let parts: Vec<&str> = self.grid_size.split('x').collect();
        if parts.len() != 2 {
            return Err("Grid size must be in format 'WxH' (e.g., '16x10')".into());
        }
        
        let width: usize = parts[0].parse()?;
        let height: usize = parts[1].parse()?;
        
        // Set default start position or use specified one
        let start = self.start_position
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap_or((1, 1));
        
        // Generate random obstacles if specified
        let mut blockers = Vec::new();
        if let Some(obstacle_count) = self.obstacles {
            for _ in 0..obstacle_count {
                loop {
                    let x = rng.gen_range(0..width);
                    let y = rng.gen_range(0..height);
                    let pos = (x, y);
                    
                    // Don't place obstacles on start position or existing obstacles
                    if pos != start && !blockers.contains(&pos) {
                        blockers.push(pos);
                        break;
                    }
                }
            }
        }
        
        // Convert enemies
        let enemies = self.enemies.as_ref()
            .map(|enemies| {
                enemies.iter().map(|enemy| {
                    let (direction, movement_pattern) = if enemy.movement_pattern.starts_with("file:") {
                        // Custom movement pattern from file
                        (EnemyDirection::Horizontal, Some(enemy.movement_pattern.clone()))
                    } else {
                        // Built-in movement pattern
                        let dir = match enemy.movement_pattern.as_str() {
                            "horizontal" => EnemyDirection::Horizontal,
                            "vertical" => EnemyDirection::Vertical,
                            _ => EnemyDirection::Horizontal, // Default
                        };
                        (dir, None)
                    };
                    
                    EnemySpec {
                        pos: (enemy.start_location.0 as i32, enemy.start_location.1 as i32),
                        direction,
                        moving_positive: enemy.moving_positive.unwrap_or(true),
                        movement_pattern,
                    }
                }).collect()
            })
            .unwrap_or_else(Vec::new);
        
        // Convert items
        let items = self.items.as_ref()
            .map(|items| {
                items.iter().map(|item| {
                    let pos = if item.spawn_randomly.unwrap_or(false) {
                        // Generate random position
                        loop {
                            let x = rng.gen_range(0..width as i32);
                            let y = rng.gen_range(0..height as i32);
                            let pos = Some((x, y));
                            
                            // Don't place items on start position
                            if pos != Some((start.0 as i32, start.1 as i32)) {
                                break pos;
                            }
                        }
                    } else {
                        item.location.map(|(x, y)| (x as i32, y as i32))
                    };
                    
                    // Load item capabilities from file or embedded data
                    let capabilities = if Path::new(&item.item_file).exists() {
                        // External file exists - use it
                        let mut caps = HashMap::new();
                        caps.insert("file_path".to_string(), serde_yaml::Value::String(item.item_file.clone()));
                        caps
                    } else if let Some(embedded_content) = crate::embedded_levels::get_embedded_item_content(&item.item_file) {
                        // Use embedded item data as fallback
                        let mut caps = HashMap::new();
                        caps.insert("file_path".to_string(), serde_yaml::Value::String(item.item_file.clone()));
                        // Parse embedded content for capabilities
                        for line in embedded_content.lines() {
                            if line.starts_with("// CAPABILITY: ") {
                                let capability_def = &line[15..]; // Skip "// CAPABILITY: "
                                if let Some((key, value)) = capability_def.split_once(" = ") {
                                    // Try to parse as number first, then string
                                    if let Ok(num_val) = value.parse::<i64>() {
                                        caps.insert(key.to_string(), serde_yaml::Value::Number(serde_yaml::Number::from(num_val)));
                                    } else {
                                        caps.insert(key.to_string(), serde_yaml::Value::String(value.to_string()));
                                    }
                                }
                            }
                        }
                        caps
                    } else {
                        HashMap::new()
                    };
                    
                    ItemSpec {
                        name: item.name.clone(),
                        pos,
                        capabilities,
                    }
                }).collect()
            })
            .unwrap_or_else(Vec::new);
        
        // Handle scanner placement - if there's an item named "scanner", use it
        let scanner_at = items.iter()
            .find(|item| item.name.to_lowercase() == "scanner")
            .and_then(|scanner| scanner.pos)
            .map(|(x, y)| (x as usize, y as usize));
        
        Ok(LevelSpec {
            name: self.name.clone(),
            width,
            height,
            start,
            scanner_at,
            blockers,
            enemies,
            items,
            fog_of_war: self.fog_of_war.unwrap_or(true),
            max_turns: self.max_turns.unwrap_or(0) as usize,
            income_per_square: self.income_per_square.unwrap_or(1),
            message: self.message.clone(),
        })
    }
}

pub fn load_yaml_levels_from_directory<P: AsRef<Path>>(dir: P) -> Vec<YamlLevelConfig> {
    let dir_path = dir.as_ref();
    let order_file = dir_path.join("order.txt");
    
    // Try to load ordered list first
    if let Ok(order_content) = fs::read_to_string(&order_file) {
        let mut levels = Vec::new();
        
        for line in order_content.lines() {
            let line = line.trim();
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Try to load the specified file
            let yaml_path = dir_path.join(format!("{}.yaml", line));
            if let Ok(level) = YamlLevelConfig::from_yaml_file(yaml_path) {
                levels.push(level);
            }
        }
        
        // If we found ordered levels, return them
        if !levels.is_empty() {
            return levels;
        }
    }
    
    // Fallback: load all yaml files in directory order (alphabetical)
    let mut levels = Vec::new();
    let mut paths = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "yaml" || ext == "yml" {
                    paths.push(path);
                }
            }
        }
    }
    
    // Sort paths alphabetically
    paths.sort();
    
    for path in paths {
        if let Ok(level) = YamlLevelConfig::from_yaml_file(path) {
            levels.push(level);
        }
    }
    
    levels
}