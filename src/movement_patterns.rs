use crate::item::Pos;
use crate::grid::Grid;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Trait for custom enemy movement patterns
pub trait MovementPattern: Send + Sync + std::fmt::Debug {
    /// Calculate the next position for an enemy
    /// Returns None if the enemy should not move this turn
    fn next_move(&self, current_pos: Pos, grid: &Grid, enemy_data: &mut HashMap<String, serde_yaml::Value>) -> Option<Pos>;
    
    /// Initialize any data needed for this movement pattern
    /// This is called once when the enemy is created
    fn initialize(&self) -> HashMap<String, serde_yaml::Value> {
        HashMap::new()
    }
    
    /// Get a description of this movement pattern
    fn description(&self) -> &'static str {
        "Custom movement pattern"
    }
}

/// Built-in horizontal movement pattern
#[derive(Debug)]
pub struct HorizontalMovement {
    pub moving_positive: bool,
}

impl MovementPattern for HorizontalMovement {
    fn next_move(&self, current_pos: Pos, grid: &Grid, enemy_data: &mut HashMap<String, serde_yaml::Value>) -> Option<Pos> {
        let direction = enemy_data.get("moving_positive")
            .and_then(|v| v.as_bool())
            .unwrap_or(self.moving_positive);
        
        let dx = if direction { 1 } else { -1 };
        let next = Pos { x: current_pos.x + dx, y: current_pos.y };
        
        if grid.in_bounds(next) && !grid.is_blocked(next) && !grid.enemies.iter().any(|e| e.pos == next) {
            Some(next)
        } else {
            // Reverse direction
            enemy_data.insert("moving_positive".to_string(), serde_yaml::Value::Bool(!direction));
            let dx = if !direction { 1 } else { -1 };
            let next = Pos { x: current_pos.x + dx, y: current_pos.y };
            
            if grid.in_bounds(next) && !grid.is_blocked(next) && !grid.enemies.iter().any(|e| e.pos == next) {
                Some(next)
            } else {
                None
            }
        }
    }
    
    fn description(&self) -> &'static str {
        "Moves horizontally, reversing direction when blocked"
    }
}

/// Built-in vertical movement pattern
#[derive(Debug)]
pub struct VerticalMovement {
    pub moving_positive: bool,
}

impl MovementPattern for VerticalMovement {
    fn next_move(&self, current_pos: Pos, grid: &Grid, enemy_data: &mut HashMap<String, serde_yaml::Value>) -> Option<Pos> {
        let direction = enemy_data.get("moving_positive")
            .and_then(|v| v.as_bool())
            .unwrap_or(self.moving_positive);
        
        let dy = if direction { 1 } else { -1 };
        let next = Pos { x: current_pos.x, y: current_pos.y + dy };
        
        if grid.in_bounds(next) && !grid.is_blocked(next) && !grid.enemies.iter().any(|e| e.pos == next) {
            Some(next)
        } else {
            // Reverse direction
            enemy_data.insert("moving_positive".to_string(), serde_yaml::Value::Bool(!direction));
            let dy = if !direction { 1 } else { -1 };
            let next = Pos { x: current_pos.x, y: current_pos.y + dy };
            
            if grid.in_bounds(next) && !grid.is_blocked(next) && !grid.enemies.iter().any(|e| e.pos == next) {
                Some(next)
            } else {
                None
            }
        }
    }
    
    fn description(&self) -> &'static str {
        "Moves vertically, reversing direction when blocked"
    }
}

/// Registry for movement patterns
#[derive(Debug)]
pub struct MovementPatternRegistry {
    patterns: HashMap<String, Box<dyn MovementPattern>>,
}

impl Clone for MovementPatternRegistry {
    fn clone(&self) -> Self {
        // Create a new registry with the same built-in patterns
        // Note: Custom patterns loaded from files will need to be reloaded
        MovementPatternRegistry::new()
    }
}

impl MovementPatternRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            patterns: HashMap::new(),
        };
        
        // Register built-in patterns
        registry.register("horizontal", Box::new(HorizontalMovement { moving_positive: true }));
        registry.register("vertical", Box::new(VerticalMovement { moving_positive: true }));
        
        registry
    }
    
    pub fn register(&mut self, name: &str, pattern: Box<dyn MovementPattern>) {
        self.patterns.insert(name.to_string(), pattern);
    }
    
    pub fn get(&self, name: &str) -> Option<&Box<dyn MovementPattern>> {
        self.patterns.get(name)
    }
    
    pub fn load_from_file<P: AsRef<Path>>(&mut self, pattern_name: &str, file_path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = file_path.as_ref();
        if !path.exists() {
            return Err(format!("Movement pattern file not found: {}", path.display()).into());
        }
        
        // For now, we'll parse the file for special comments that define the movement logic
        // In a real implementation, you might use a scripting language or compile the Rust code
        let content = fs::read_to_string(path)?;
        
        if content.contains("// MOVEMENT_PATTERN: random") {
            self.register(pattern_name, Box::new(RandomMovement));
        } else if content.contains("// MOVEMENT_PATTERN: diagonal") {
            self.register(pattern_name, Box::new(DiagonalMovement { moving_positive: true }));
        } else if content.contains("// MOVEMENT_PATTERN: circular") {
            self.register(pattern_name, Box::new(CircularMovement::new()));
        } else if content.contains("// MOVEMENT_PATTERN: spiral") {
            self.register(pattern_name, Box::new(SpiralMovement));
        } else if content.contains("// MOVEMENT_PATTERN: chase") {
            self.register(pattern_name, Box::new(ChaseMovement));
        } else if content.contains("// MOVEMENT_PATTERN: guard") {
            self.register(pattern_name, Box::new(GuardMovement));
        }
        // Add more pattern types as needed
        
        Ok(())
    }
}

/// Example: Random movement pattern
#[derive(Debug)]
pub struct RandomMovement;

impl MovementPattern for RandomMovement {
    fn next_move(&self, current_pos: Pos, grid: &Grid, _enemy_data: &mut HashMap<String, serde_yaml::Value>) -> Option<Pos> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut attempts = 0;
        
        while attempts < 10 {
            let (dx, dy) = directions[rng.gen_range(0..directions.len())];
            let next = Pos { x: current_pos.x + dx, y: current_pos.y + dy };
            
            if grid.in_bounds(next) && !grid.is_blocked(next) && !grid.enemies.iter().any(|e| e.pos == next) {
                return Some(next);
            }
            attempts += 1;
        }
        
        None
    }
    
    fn description(&self) -> &'static str {
        "Moves randomly in any direction"
    }
}

/// Example: Diagonal movement pattern
#[derive(Debug)]
pub struct DiagonalMovement {
    pub moving_positive: bool,
}

impl MovementPattern for DiagonalMovement {
    fn next_move(&self, current_pos: Pos, grid: &Grid, enemy_data: &mut HashMap<String, serde_yaml::Value>) -> Option<Pos> {
        let direction = enemy_data.get("moving_positive")
            .and_then(|v| v.as_bool())
            .unwrap_or(self.moving_positive);
        
        let (dx, dy) = if direction { (1, 1) } else { (-1, -1) };
        let next = Pos { x: current_pos.x + dx, y: current_pos.y + dy };
        
        if grid.in_bounds(next) && !grid.is_blocked(next) && !grid.enemies.iter().any(|e| e.pos == next) {
            Some(next)
        } else {
            // Reverse direction
            enemy_data.insert("moving_positive".to_string(), serde_yaml::Value::Bool(!direction));
            let (dx, dy) = if !direction { (1, 1) } else { (-1, -1) };
            let next = Pos { x: current_pos.x + dx, y: current_pos.y + dy };
            
            if grid.in_bounds(next) && !grid.is_blocked(next) && !grid.enemies.iter().any(|e| e.pos == next) {
                Some(next)
            } else {
                None
            }
        }
    }
    
    fn description(&self) -> &'static str {
        "Moves diagonally, reversing direction when blocked"
    }
}

/// Example: Circular movement pattern
#[derive(Debug)]
pub struct CircularMovement {
    directions: Vec<(i32, i32)>,
}

impl CircularMovement {
    pub fn new() -> Self {
        Self {
            directions: vec![(1, 0), (0, 1), (-1, 0), (0, -1)], // Right, Down, Left, Up
        }
    }
}

impl MovementPattern for CircularMovement {
    fn next_move(&self, current_pos: Pos, grid: &Grid, enemy_data: &mut HashMap<String, serde_yaml::Value>) -> Option<Pos> {
        let current_dir = enemy_data.get("direction_index")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;
        
        let (dx, dy) = self.directions[current_dir % self.directions.len()];
        let next = Pos { x: current_pos.x + dx, y: current_pos.y + dy };
        
        if grid.in_bounds(next) && !grid.is_blocked(next) && !grid.enemies.iter().any(|e| e.pos == next) {
            Some(next)
        } else {
            // Try next direction in circle
            let next_dir = (current_dir + 1) % self.directions.len();
            enemy_data.insert("direction_index".to_string(), serde_yaml::Value::Number(next_dir.into()));
            
            let (dx, dy) = self.directions[next_dir];
            let next = Pos { x: current_pos.x + dx, y: current_pos.y + dy };
            
            if grid.in_bounds(next) && !grid.is_blocked(next) && !grid.enemies.iter().any(|e| e.pos == next) {
                Some(next)
            } else {
                None
            }
        }
    }
    
    fn initialize(&self) -> HashMap<String, serde_yaml::Value> {
        let mut data = HashMap::new();
        data.insert("direction_index".to_string(), serde_yaml::Value::Number(0.into()));
        data
    }
    
    fn description(&self) -> &'static str {
        "Moves in a circular pattern (right, down, left, up)"
    }
}

/// Spiral movement pattern
#[derive(Debug)]
pub struct SpiralMovement;

impl MovementPattern for SpiralMovement {
    fn next_move(&self, current_pos: Pos, grid: &Grid, enemy_data: &mut HashMap<String, serde_yaml::Value>) -> Option<Pos> {
        let direction_index = enemy_data.get("direction_index")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;
            
        let steps_in_direction = enemy_data.get("steps_in_direction")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as usize;
            
        let current_step = enemy_data.get("current_step")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;
        
        // Spiral directions: right, down, left, up
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let (dx, dy) = directions[direction_index % directions.len()];
        
        let next_pos = Pos {
            x: current_pos.x + dx,
            y: current_pos.y + dy,
        };
        
        if grid.in_bounds(next_pos) 
            && !grid.is_blocked(next_pos) 
            && !grid.enemies.iter().any(|e| e.pos == next_pos) {
            
            let new_current_step = current_step + 1;
            
            if new_current_step >= steps_in_direction {
                let new_direction_index = (direction_index + 1) % directions.len();
                let new_steps_in_direction = if new_direction_index % 2 == 0 {
                    steps_in_direction + 1
                } else {
                    steps_in_direction
                };
                
                enemy_data.insert("direction_index".to_string(), serde_yaml::Value::Number(new_direction_index.into()));
                enemy_data.insert("steps_in_direction".to_string(), serde_yaml::Value::Number(new_steps_in_direction.into()));
                enemy_data.insert("current_step".to_string(), serde_yaml::Value::Number(0.into()));
            } else {
                enemy_data.insert("current_step".to_string(), serde_yaml::Value::Number(new_current_step.into()));
            }
            
            Some(next_pos)
        } else {
            None
        }
    }
    
    fn initialize(&self) -> HashMap<String, serde_yaml::Value> {
        let mut data = HashMap::new();
        data.insert("direction_index".to_string(), serde_yaml::Value::Number(0.into()));
        data.insert("steps_in_direction".to_string(), serde_yaml::Value::Number(1.into()));
        data.insert("current_step".to_string(), serde_yaml::Value::Number(0.into()));
        data
    }
    
    fn description(&self) -> &'static str {
        "Moves in an expanding spiral pattern"
    }
}

/// Chase movement pattern
#[derive(Debug)]
pub struct ChaseMovement;

impl MovementPattern for ChaseMovement {
    fn next_move(&self, current_pos: Pos, grid: &Grid, enemy_data: &mut HashMap<String, serde_yaml::Value>) -> Option<Pos> {
        // Try to get player position from enemy data, fallback to (1,1) if not available
        let player_pos = if let Some(player_x) = enemy_data.get("player_x").and_then(|v| v.as_i64()) {
            if let Some(player_y) = enemy_data.get("player_y").and_then(|v| v.as_i64()) {
                Pos { x: player_x as i32, y: player_y as i32 }
            } else {
                Pos { x: 1, y: 1 } // fallback
            }
        } else {
            Pos { x: 1, y: 1 } // fallback
        };
        
        // Calculate direction to player
        let dx = if player_pos.x > current_pos.x {
            1
        } else if player_pos.x < current_pos.x {
            -1
        } else {
            0
        };
        
        let dy = if player_pos.y > current_pos.y {
            1
        } else if player_pos.y < current_pos.y {
            -1
        } else {
            0
        };
        
        // Try to move toward player
        let preferred_pos = Pos {
            x: current_pos.x + dx,
            y: current_pos.y + dy,
        };
        
        // Check if preferred move is valid
        if grid.in_bounds(preferred_pos) && !grid.is_blocked(preferred_pos) {
            // Mark as actively chasing (moving toward player)
            enemy_data.insert("is_chasing".to_string(), serde_yaml::Value::Bool(true));
            return Some(preferred_pos);
        }
        
        // If direct path blocked, try alternative moves
        let alternative_moves = vec![
            (dx, 0),  // Horizontal only
            (0, dy),  // Vertical only
            (1, 0),   // Right
            (-1, 0),  // Left
            (0, 1),   // Down
            (0, -1),  // Up
        ];
        
        for (alt_dx, alt_dy) in alternative_moves {
            if alt_dx == 0 && alt_dy == 0 { continue; }
            
            let alt_pos = Pos {
                x: current_pos.x + alt_dx,
                y: current_pos.y + alt_dy,
            };
            
            if grid.in_bounds(alt_pos) && !grid.is_blocked(alt_pos) {
                // Mark as searching/not directly chasing
                enemy_data.insert("is_chasing".to_string(), serde_yaml::Value::Bool(false));
                return Some(alt_pos);
            }
        }
        
        // If all moves blocked, don't move - mark as not chasing
        enemy_data.insert("is_chasing".to_string(), serde_yaml::Value::Bool(false));
        None
    }
    
    fn description(&self) -> &'static str {
        "Chases the player using simple pathfinding"
    }
}

/// Guard movement pattern
#[derive(Debug)]
pub struct GuardMovement;

impl MovementPattern for GuardMovement {
    fn next_move(&self, current_pos: Pos, grid: &Grid, enemy_data: &mut HashMap<String, serde_yaml::Value>) -> Option<Pos> {
        let center_x = enemy_data.get("center_x")
            .and_then(|v| v.as_i64())
            .unwrap_or(current_pos.x as i64) as i32;
            
        let center_y = enemy_data.get("center_y")
            .and_then(|v| v.as_i64())
            .unwrap_or(current_pos.y as i64) as i32;
        
        if !enemy_data.contains_key("center_x") {
            enemy_data.insert("center_x".to_string(), serde_yaml::Value::Number(center_x.into()));
            enemy_data.insert("center_y".to_string(), serde_yaml::Value::Number(center_y.into()));
        }
        
        let direction_index = enemy_data.get("direction_index")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;
        
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let (dx, dy) = directions[direction_index % directions.len()];
        
        let next_pos = Pos {
            x: current_pos.x + dx,
            y: current_pos.y + dy,
        };
        
        let center = Pos { x: center_x, y: center_y };
        let distance_from_center = (next_pos.x - center.x).abs() + (next_pos.y - center.y).abs();
        
        if distance_from_center <= 3
            && grid.in_bounds(next_pos) 
            && !grid.is_blocked(next_pos) 
            && !grid.enemies.iter().any(|e| e.pos == next_pos) {
            Some(next_pos)
        } else {
            let new_direction_index = (direction_index + 1) % directions.len();
            enemy_data.insert("direction_index".to_string(), serde_yaml::Value::Number(new_direction_index.into()));
            None
        }
    }
    
    fn initialize(&self) -> HashMap<String, serde_yaml::Value> {
        let mut data = HashMap::new();
        data.insert("direction_index".to_string(), serde_yaml::Value::Number(0.into()));
        data
    }
    
    fn description(&self) -> &'static str {
        "Guards a small area around the starting position"
    }
}