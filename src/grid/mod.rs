use crate::level::{LevelSpec, EnemyDirection};
use crate::item::Pos;
use crate::movement_patterns::MovementPatternRegistry;
use rand::rngs::StdRng;
use rand::Rng;
use std::collections::{HashSet, HashMap};

#[derive(Clone, Debug)]
pub struct Enemy {
    pub pos: Pos,
    pub direction: EnemyDirection,
    pub moving_positive: bool, // true = right/down, false = left/up
    pub movement_pattern: Option<String>, // For custom movement patterns
    pub movement_data: HashMap<String, serde_yaml::Value>, // Data for custom movement patterns
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub known: HashSet<Pos>,
    pub visited: HashSet<Pos>,
    pub blockers: HashSet<Pos>,
    pub doors: HashSet<Pos>,  // Door positions
    pub open_doors: HashSet<Pos>,  // Currently open doors
    pub enemies: Vec<Enemy>,
    pub fog_of_war: bool,
    pub income_per_square: u32,
    pub movement_registry: MovementPatternRegistry,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            known: HashSet::new(),
            visited: HashSet::new(),
            blockers: HashSet::new(),
            doors: HashSet::new(),
            open_doors: HashSet::new(),
            enemies: Vec::new(),
            fog_of_war: true,
            income_per_square: 1,
            movement_registry: MovementPatternRegistry::new(),
        }
    }

    pub fn from_level_spec(spec: &LevelSpec, rng: &mut StdRng, _robot_carries_scanner: bool) -> Self {
        let mut grid = Self::new(spec.width as i32, spec.height as i32);
        grid.fog_of_war = spec.fog_of_war;
        grid.income_per_square = spec.income_per_square;
        
        // Register additional built-in patterns
        grid.movement_registry.register("random", Box::new(crate::movement_patterns::RandomMovement));
        grid.movement_registry.register("diagonal", Box::new(crate::movement_patterns::DiagonalMovement { moving_positive: true }));
        grid.movement_registry.register("circular", Box::new(crate::movement_patterns::CircularMovement::new()));

        // Add specified blockers
        for (x, y) in &spec.blockers {
            grid.blockers.insert(Pos { x: *x as i32, y: *y as i32 });
        }
        
        // Add specified doors
        for (x, y) in &spec.doors {
            grid.doors.insert(Pos { x: *x as i32, y: *y as i32 });
        }

        // Add enemies
        for enemy_spec in &spec.enemies {
            // Load custom movement pattern if specified
            if let Some(ref pattern_str) = enemy_spec.movement_pattern {
                if pattern_str.starts_with("file:") {
                    let file_path = &pattern_str[5..]; // Remove "file:" prefix
                    let pattern_name = format!("custom_{}", grid.enemies.len());
                    if let Err(e) = grid.movement_registry.load_from_file(&pattern_name, file_path) {
                        eprintln!("Failed to load movement pattern from {}: {}", file_path, e);
                    }
                }
            }
            
            // Initialize movement data
            let movement_data = if let Some(ref pattern_str) = enemy_spec.movement_pattern {
                if pattern_str.starts_with("file:") {
                    let pattern_name = format!("custom_{}", grid.enemies.len());
                    if let Some(pattern) = grid.movement_registry.get(&pattern_name) {
                        pattern.initialize()
                    } else {
                        HashMap::new()
                    }
                } else {
                    HashMap::new()
                }
            } else {
                HashMap::new()
            };
            
            let enemy = Enemy {
                pos: Pos { x: enemy_spec.pos.0, y: enemy_spec.pos.1 },
                direction: enemy_spec.direction,
                moving_positive: enemy_spec.moving_positive,
                movement_pattern: enemy_spec.movement_pattern.clone(),
                movement_data,
            };
            grid.enemies.push(enemy);
        }

        // Generate additional random obstacles for certain levels
        if spec.name.contains("Level 3") && spec.blockers.is_empty() {
            let n = (grid.width * grid.height) / 8;
            for _ in 0..n {
                let p = Pos { 
                    x: rng.gen_range(0..grid.width), 
                    y: rng.gen_range(0..grid.height) 
                };
                if p != (Pos { x: spec.start.0 as i32, y: spec.start.1 as i32 }) {
                    grid.blockers.insert(p);
                }
            }
        } else if spec.name.contains("Level 4") && spec.blockers.is_empty() {
            // Generate some obstacles for Level 4
            let obstacle_count = (grid.width * grid.height) / 12;
            for _ in 0..obstacle_count {
                let p = Pos { 
                    x: rng.gen_range(0..grid.width), 
                    y: rng.gen_range(0..grid.height) 
                };
                if p != (Pos { x: spec.start.0 as i32, y: spec.start.1 as i32 }) {
                    grid.blockers.insert(p);
                }
            }
            
            // Generate enemies for Level 4 if not specified
            if spec.enemies.is_empty() {
                let enemy_count = 3;
                for _ in 0..enemy_count {
                    loop {
                        let pos = Pos { 
                            x: rng.gen_range(2..grid.width-2), 
                            y: rng.gen_range(2..grid.height-2) 
                        };
                        let start_pos = Pos { x: spec.start.0 as i32, y: spec.start.1 as i32 };
                        if pos != start_pos && !grid.blockers.contains(&pos) && 
                           manhattan_distance(pos, start_pos) > 3 {
                            let direction = if rng.gen_bool(0.5) { 
                                EnemyDirection::Horizontal 
                            } else { 
                                EnemyDirection::Vertical 
                            };
                            let moving_positive = rng.gen_bool(0.5);
                            grid.enemies.push(Enemy { 
                                pos, 
                                direction, 
                                moving_positive,
                                movement_pattern: None,
                                movement_data: HashMap::new(),
                            });
                            break;
                        }
                    }
                }
            }
        }

        grid
    }

    pub fn in_bounds(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width && pos.y < self.height
    }

    pub fn reveal(&mut self, pos: Pos) -> bool {
        if self.in_bounds(pos) && !self.known.contains(&pos) {
            self.known.insert(pos);
            true
        } else {
            false
        }
    }

    pub fn reveal_adjacent(&mut self, center: (i32, i32)) -> usize {
        let center_pos = Pos { x: center.0, y: center.1 };
        let mut revealed = 0;
        
        if self.reveal(center_pos) {
            revealed += 1;
        }
        
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let adjacent_pos = Pos { x: center.0 + dx, y: center.1 + dy };
            if self.reveal(adjacent_pos) {
                revealed += 1;
            }
        }
        
        revealed
    }

    pub fn move_enemies(&mut self, player_pos: Option<(i32, i32)>, stunned_enemies: &std::collections::HashMap<usize, u8>) {
        let mut new_enemies = self.enemies.clone();
        
        for (i, enemy) in new_enemies.iter_mut().enumerate() {
            // Skip stunned enemies
            if stunned_enemies.contains_key(&i) {
                continue;
            }
            
            // Check if enemy uses a custom movement pattern
            if let Some(ref pattern_str) = enemy.movement_pattern {
                if pattern_str.starts_with("file:") {
                    let pattern_name = format!("custom_{}", i);
                    if let Some(pattern) = self.movement_registry.get(&pattern_name) {
                        if let Some(new_pos) = pattern.next_move(enemy.pos, self, &mut enemy.movement_data) {
                            enemy.pos = new_pos;
                        }
                        continue;
                    }
                } else if pattern_str == "random" {
                    if let Some(pattern) = self.movement_registry.get("random") {
                        if let Some(new_pos) = pattern.next_move(enemy.pos, self, &mut enemy.movement_data) {
                            enemy.pos = new_pos;
                        }
                        continue;
                    }
                } else if pattern_str == "diagonal" {
                    if let Some(pattern) = self.movement_registry.get("diagonal") {
                        if let Some(new_pos) = pattern.next_move(enemy.pos, self, &mut enemy.movement_data) {
                            enemy.pos = new_pos;
                        }
                        continue;
                    }
                } else if pattern_str == "circular" {
                    if let Some(pattern) = self.movement_registry.get("circular") {
                        if let Some(new_pos) = pattern.next_move(enemy.pos, self, &mut enemy.movement_data) {
                            enemy.pos = new_pos;
                        }
                        continue;
                    }
                } else if pattern_str == "chase" {
                    // Pass player position to chase enemies
                    if let Some((px, py)) = player_pos {
                        enemy.movement_data.insert("player_x".to_string(), serde_yaml::Value::Number(serde_yaml::Number::from(px)));
                        enemy.movement_data.insert("player_y".to_string(), serde_yaml::Value::Number(serde_yaml::Number::from(py)));
                    }
                    
                    if let Some(pattern) = self.movement_registry.get("chase") {
                        if let Some(new_pos) = pattern.next_move(enemy.pos, self, &mut enemy.movement_data) {
                            enemy.pos = new_pos;
                        }
                        continue;
                    }
                }
            }
            
            // Fall back to built-in movement patterns
            let step = |_pos: Pos, dir: EnemyDirection, pos_dir: bool| -> (i32, i32) {
                match dir {
                    EnemyDirection::Horizontal => if pos_dir { (1, 0) } else { (-1, 0) },
                    EnemyDirection::Vertical   => if pos_dir { (0, 1) } else { (0, -1) },
                }
            };

            // First attempt in current direction
            let (dx, dy) = step(enemy.pos, enemy.direction, enemy.moving_positive);
            let mut next = Pos { x: enemy.pos.x + dx, y: enemy.pos.y + dy };

            let mut can_move = self.in_bounds(next)
                && !self.blockers.contains(&next)
                && !self.enemies.iter().any(|other| other.pos == next);

            if !can_move {
                // Reverse and try once more this tick
                enemy.moving_positive = !enemy.moving_positive;
                let (dx2, dy2) = step(enemy.pos, enemy.direction, enemy.moving_positive);
                next = Pos { x: enemy.pos.x + dx2, y: enemy.pos.y + dy2 };

                can_move = self.in_bounds(next)
                    && !self.blockers.contains(&next)
                    && !self.enemies.iter().any(|other| other.pos == next);

                if !can_move {
                    continue; // stuck this turn
                }
            }

            enemy.pos = next;
        }
        self.enemies = new_enemies;
    }

    pub fn check_enemy_collision(&self, robot_pos: (i32, i32)) -> bool {
        let robot_pos = Pos { x: robot_pos.0, y: robot_pos.1 };
        self.enemies.iter().any(|enemy| enemy.pos == robot_pos)
    }

    pub fn is_blocked(&self, pos: Pos) -> bool {
        self.blockers.contains(&pos) || (self.doors.contains(&pos) && !self.open_doors.contains(&pos))
    }
    
    pub fn is_door(&self, pos: Pos) -> bool {
        self.doors.contains(&pos)
    }
    
    pub fn is_door_open(&self, pos: Pos) -> bool {
        self.doors.contains(&pos) && self.open_doors.contains(&pos)
    }
    
    pub fn open_door(&mut self, pos: Pos) -> bool {
        if self.doors.contains(&pos) {
            self.open_doors.insert(pos);
            true
        } else {
            false
        }
    }
    
    pub fn close_door(&mut self, pos: Pos) -> bool {
        if self.doors.contains(&pos) {
            self.open_doors.remove(&pos);
            true
        } else {
            false
        }
    }

    pub fn is_blocked_with_temp_removal(&self, pos: Pos, temp_removed: &std::collections::HashMap<(i32, i32), u8>) -> bool {
        // Check if temporarily removed
        if temp_removed.contains_key(&(pos.x, pos.y)) {
            return false;
        }
        self.blockers.contains(&pos)
    }

    pub fn visit(&mut self, pos: Pos) {
        if self.in_bounds(pos) {
            self.visited.insert(pos);
        }
    }

    pub fn get_enemies_at_position(&self, pos: Pos) -> Vec<&Enemy> {
        self.enemies.iter().filter(|enemy| enemy.pos == pos).collect()
    }
}

pub fn manhattan_distance(a: Pos, b: Pos) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}