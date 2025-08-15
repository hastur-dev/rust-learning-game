use crate::level::LevelSpec;
use crate::grid::Grid;
use crate::robot::Robot;
use crate::item::ItemManager;
use crate::menu::Menu;
use rand::rngs::StdRng;
use crossbeam_channel::Receiver;
use notify::Event;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RustFunction {
    Move,
    Grab,
    Scan,
    SearchAll,
    AutoGrab,
}

#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub function: RustFunction,
    pub direction: Option<(i32, i32)>, // for move and scan
    pub boolean_param: Option<bool>, // for auto_grab
}

#[derive(Clone, Debug)]
pub struct Game {
    pub level_idx: usize,
    pub levels: Vec<LevelSpec>,
    pub grid: Grid,
    pub robot: Robot,
    pub item_manager: ItemManager,
    pub rng: StdRng,
    pub credits: u32,
    pub turns: usize,
    pub max_turns: usize,
    pub discovered_this_level: usize,
    pub finished: bool,
    pub scan_armed: bool,
    pub execution_result: String,
    pub code_editor_active: bool,
    pub selected_function_to_view: Option<RustFunction>,
    pub robot_code_path: String,
    pub file_watcher_receiver: Option<Receiver<notify::Result<Event>>>,
    pub robot_code_modified: bool,
    pub current_code: String,
    pub cursor_position: usize,
    pub enemy_step_paused: bool,
    pub time_slow_active: bool,
    pub time_slow_duration_ms: u32,
    pub menu: Menu,
}

impl Game {
    pub fn new(levels: Vec<LevelSpec>, mut rng: StdRng) -> Self {
        let first = levels.first().expect("no levels").clone();
        let grid = Grid::from_level_spec(&first, &mut rng, false);
        let robot = Robot::new((first.start.0 as i32, first.start.1 as i32));
        let item_manager = ItemManager::new();

        Self {
            level_idx: 0,
            levels,
            grid,
            robot,
            item_manager,
            rng,
            credits: 0,
            turns: 0,
            max_turns: first.max_turns,
            discovered_this_level: 0,
            finished: false,
            scan_armed: false,
            execution_result: String::new(),
            code_editor_active: false,
            selected_function_to_view: None,
            robot_code_path: "robot_code.rs".to_string(),
            file_watcher_receiver: None,
            robot_code_modified: false,
            current_code: String::new(),
            cursor_position: 0,
            enemy_step_paused: false,
            time_slow_active: false,
            time_slow_duration_ms: 500, // Default 500ms
            menu: Menu::new(),
        }
    }

    pub fn get_available_functions(&self) -> Vec<RustFunction> {
        let mut functions = vec![];
        
        // Level 1: move, search_all, and auto_grab
        functions.push(RustFunction::Move);
        functions.push(RustFunction::SearchAll);
        functions.push(RustFunction::AutoGrab);
        
        // Level 2+: grab becomes available
        if self.level_idx >= 1 {
            functions.push(RustFunction::Grab);
        }
        
        // Scan only available if player has actually grabbed the scanner
        if self.item_manager.has_collected("scanner") {
            functions.push(RustFunction::Scan);
        }
        
        functions
    }

    pub fn finish_level(&mut self) {
        self.finished = true;
        let reward = self.discovered_this_level as u32;
        self.credits += reward;
    }

    pub fn next_level(&mut self) {
        if self.level_idx + 1 < self.levels.len() {
            self.level_idx += 1;
            self.load_level(self.level_idx);
        }
    }

    pub fn load_robot_code(&mut self) {
        if let Ok(code) = crate::read_robot_code(&self.robot_code_path) {
            self.current_code = code;
            self.cursor_position = self.cursor_position.min(self.current_code.len());
        }
    }

    pub fn save_robot_code(&mut self) {
        if let Err(e) = crate::write_robot_code(&self.robot_code_path, &self.current_code) {
            self.execution_result = format!("Save error: {}", e);
        }
    }

    pub fn load_level(&mut self, idx: usize) {
        let spec = self.levels[idx].clone();
        let mut grid = Grid::from_level_spec(&spec, &mut self.rng, self.item_manager.has_collected("scanner"));
        let start = (spec.start.0 as i32, spec.start.1 as i32);
        self.robot.set_position(start);

        // Reveal starting tile + neighbors
        grid.reveal_adjacent(start);

        self.grid = grid;
        self.turns = 0;
        self.max_turns = spec.max_turns;
        self.discovered_this_level = 0;
        self.finished = false;
        self.scan_armed = false;
        self.enemy_step_paused = false;

        // Initialize item manager with level items
        self.item_manager.items.clear();
        for item_spec in &spec.items {
            if let Some(pos) = item_spec.pos {
                self.item_manager.add_item(
                    item_spec.name.clone(),
                    crate::item::Pos { x: pos.0, y: pos.1 },
                    item_spec.capabilities.get("file_path")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                );
            }
        }

        // Add scanner if specified in level
        if let Some((x, y)) = spec.scanner_at {
            if !self.item_manager.has_collected("scanner") {
                let scanner_item = crate::item::create_scanner_item(
                    crate::item::Pos { x: x as i32, y: y as i32 }
                );
                self.item_manager.items.push(scanner_item);
            }
        }
    }

    pub fn check_end_condition(&mut self) {
        if self.finished { 
            return; 
        }
        
        // Check for enemy collision (Level 4+)
        if self.level_idx >= 3 && self.grid.check_enemy_collision(self.robot.get_position()) {
            // Reset and randomize the level when enemy catches player
            let idx = self.level_idx;
            self.load_level(idx);
            self.execution_result = "ENEMY COLLISION! Level reset and randomized.".to_string();
            return;
        }
        
        // Check if all items have been collected
        let items_remaining = self.item_manager.get_active_items().len() > 0;
        if items_remaining {
            // Cannot complete level until all items are grabbed
            return;
        }
        
        // Original completion logic - all squares explored
        let total_cells = (self.grid.width * self.grid.height) as usize;
        let known_nonblockers = self.grid.known.iter()
            .filter(|pos| !self.grid.blockers.contains(pos))
            .count();
        let blockers_count = self.grid.blockers.len();
        
        if known_nonblockers + blockers_count == total_cells || 
           (self.max_turns > 0 && self.turns >= self.max_turns) {
            self.finish_level();
        }
    }
}