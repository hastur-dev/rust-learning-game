use crate::level::{LevelSpec, EnemyDirection};
use crate::item::Pos;
use rand::rngs::StdRng;
use rand::Rng;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Enemy {
    pub pos: Pos,
    pub direction: EnemyDirection,
    pub moving_positive: bool, // true = right/down, false = left/up
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub known: HashSet<Pos>,
    pub visited: HashSet<Pos>,
    pub blockers: HashSet<Pos>,
    pub enemies: Vec<Enemy>,
    pub fog_of_war: bool,
    pub income_per_square: u32,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            known: HashSet::new(),
            visited: HashSet::new(),
            blockers: HashSet::new(),
            enemies: Vec::new(),
            fog_of_war: true,
            income_per_square: 1,
        }
    }

    pub fn from_level_spec(spec: &LevelSpec, rng: &mut StdRng, _robot_carries_scanner: bool) -> Self {
        let mut grid = Self::new(spec.width as i32, spec.height as i32);
        grid.fog_of_war = spec.fog_of_war;
        grid.income_per_square = spec.income_per_square;

        // Add specified blockers
        for (x, y) in &spec.blockers {
            grid.blockers.insert(Pos { x: *x as i32, y: *y as i32 });
        }

        // Add enemies
        for enemy_spec in &spec.enemies {
            let enemy = Enemy {
                pos: Pos { x: enemy_spec.pos.0, y: enemy_spec.pos.1 },
                direction: enemy_spec.direction,
                moving_positive: enemy_spec.moving_positive,
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
                            grid.enemies.push(Enemy { pos, direction, moving_positive });
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

    pub fn move_enemies(&mut self) {
        let mut new_enemies = self.enemies.clone();
        for enemy in &mut new_enemies {
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