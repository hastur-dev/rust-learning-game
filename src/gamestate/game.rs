use super::types::*;
use crate::level::LevelSpec;
use crate::grid::Grid;
use crate::robot::Robot;
use crate::item::ItemManager;
use crate::menu::Menu;
use crate::popup::{PopupSystem, PopupAction};
use rand::rngs::StdRng;

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
            #[cfg(not(target_arch = "wasm32"))]
            file_watcher_receiver: None,
            robot_code_modified: false,
            current_code: String::new(),
            cursor_position: 0,
            selection_start: None,
            selection_end: None,
            code_scroll_offset: 0,
            code_lines_visible: 30, // Default number of lines visible
            enemy_step_paused: false,
            time_slow_active: false,
            time_slow_duration_ms: 500, // Default 500ms
            menu: Menu::new(),
            popup_system: PopupSystem::new(),
            stunned_enemies: std::collections::HashMap::new(),
            temporary_removed_obstacles: std::collections::HashMap::new(),
            println_outputs: Vec::new(),
            error_outputs: Vec::new(),
            panic_occurred: false,
            tutorial_state: TutorialState {
                task_completed: [false; 5],
                current_task: 0,
                variables_used: Vec::new(),
                scan_output_stored: false,
                u32_move_used: false,
            },
            #[cfg(not(target_arch = "wasm32"))]
            rust_checker: crate::rust_checker::RustChecker::new().ok(),
            key_backspace_held_time: 0.0,
            key_space_held_time: 0.0,
            key_char_held_time: 0.0,
            last_char_pressed: None,
            key_repeat_initial_delay: 0.5, // Wait 0.5 seconds before starting to repeat
            key_repeat_interval: 0.05,     // Repeat every 50ms after initial delay
            cached_font_size: 0.0,
            cached_char_width: 0.0,
            cached_line_height: 0.0,
            needs_font_refresh: true,      // Initially needs refresh
            commands_logs_tab: CommandsLogsTab::Commands, // Default to Commands tab
            coordinate_transformer: crate::coordinate_system::CoordinateTransformer::new(), // Initialize coordinate transformer
            enable_coordinate_logs: false, // Default to disabled, enabled via --all-logs command line flag
            last_window_update_time: 0.0, // Initialize timer
            last_mouse_click_time: 0.0,   // Initialize click timer
        }
    }

    pub fn get_available_functions(&self) -> Vec<RustFunction> {
        vec![
            RustFunction::Move,
            RustFunction::Scan, 
            RustFunction::Grab,
            RustFunction::LaserDirection,
            RustFunction::LaserTile,
            RustFunction::OpenDoor,
            RustFunction::SkipLevel,
            RustFunction::GotoLevel,
        ]
    }
    
    // Functions displayed in GUI (excludes skip/goto commands and print functions)
    pub fn get_gui_functions(&self) -> Vec<RustFunction> {
        vec![
            RustFunction::Move,
            RustFunction::Scan, 
            RustFunction::Grab,
            RustFunction::LaserDirection,
            RustFunction::LaserTile,
            RustFunction::OpenDoor,
        ]
    }

    pub fn finish_level(&mut self) {
        self.finished = true;
        let reward = self.discovered_this_level as u32;
        self.credits += reward;
        
        // Mark current level as completed and unlock next level
        self.menu.progress.mark_level_completed(self.level_idx);
        if self.level_idx + 1 < self.levels.len() {
            self.menu.progress.unlock_level(self.level_idx + 1);
        }
    }

    pub fn next_level(&mut self) {
        if self.level_idx + 1 < self.levels.len() {
            self.level_idx += 1;
            self.load_level(self.level_idx);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_robot_code(&mut self) {
        if let Ok(code) = crate::read_robot_code(&self.robot_code_path) {
            self.current_code = code;
            self.cursor_position = self.cursor_position.min(self.current_code.len());
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load_robot_code(&mut self) {
        // WASM version - no file I/O
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_robot_code(&mut self) {
        if let Err(e) = crate::write_robot_code(&self.robot_code_path, &self.current_code) {
            self.execution_result = format!("Save error: {}", e);
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save_robot_code(&mut self) {
        // WASM version - no file I/O
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
        
        // Reset tutorial state and outputs for level 0 only when starting fresh
        if idx == 0 && (self.level_idx != 0 || self.tutorial_state.current_task >= 5) {
            // Only reset when coming from a different level or tutorial is complete
            self.tutorial_state = TutorialState {
                task_completed: [false; 5],
                current_task: 0,
                variables_used: Vec::new(),
                scan_output_stored: false,
                u32_move_used: false,
            };
            self.println_outputs.clear();
            self.error_outputs.clear();
            self.panic_occurred = false;
        } else if idx != 0 {
            // Clear outputs for non-tutorial levels
            self.println_outputs.clear();
            self.error_outputs.clear();
            self.panic_occurred = false;
        }
        
        // Load starting code if available, otherwise ensure current_code has content
        if let Some(ref starting_code) = spec.starting_code {
            self.current_code = starting_code.clone();
            self.cursor_position = starting_code.len();
        } else {
            // Ensure current_code is not empty if no starting code is provided
            if self.current_code.is_empty() {
                self.current_code = "// Start typing your Rust code here...\n".to_string();
            }
            self.cursor_position = self.cursor_position.min(self.current_code.len());
        }

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

        // Show completion message first (instructions on how to complete)
        if let Some(ref completion_message) = spec.completion_message {
            self.popup_system.show_completion_instructions(
                spec.name.clone(),
                completion_message.clone()
            );
        }
        
        // Then show base level message if it exists (initial information/hints)
        if let Some(ref message) = spec.message {
            self.popup_system.show_level_message(message.clone());
        }
    }

    pub fn show_item_collected(&mut self, item_name: &str) {
        self.popup_system.show_item_collected(item_name.to_string());
    }

    pub fn show_level_complete(&mut self) {
        self.popup_system.show_level_complete();
    }

    pub fn show_completion_instructions(&mut self) {
        let current_level = &self.levels[self.level_idx];
        if let Some(ref instructions) = current_level.completion_message {
            self.popup_system.show_completion_instructions(
                current_level.name.clone(),
                instructions.clone()
            );
        } else {
            // Fallback message if no completion instructions are defined
            let fallback_message = match &current_level.completion_flag {
                Some(flag) => {
                    if flag.contains(':') {
                        let parts: Vec<&str> = flag.splitn(2, ':').collect();
                        if parts.len() == 2 {
                            let flag_type = parts[0];
                            let expected_value = parts[1];
                            match flag_type {
                                "println" => format!("Make your code print exactly: '{}'", expected_value),
                                "println_exact" => format!("Make your code print exactly: '{}'", expected_value),
                                "eprintln" => format!("Make your code output this error message: '{}'", expected_value),
                                "error_exact" => format!("Make your code output exactly this error: '{}'", expected_value),
                                "items_collected" => format!("Collect {} item(s) to complete this level", expected_value),
                                "moves_made" => format!("Make at least {} move(s) to complete this level", expected_value),
                                _ => "Follow the level's requirements to complete it.".to_string()
                            }
                        } else {
                            "Follow the level's requirements to complete it.".to_string()
                        }
                    } else {
                        match flag.as_str() {
                            "println" => "Use println!() to display output".to_string(),
                            "error" | "eprintln" => "Use eprintln!() to display an error message".to_string(),
                            "panic" => "Trigger a panic in your code".to_string(),
                            "items_collected" => "Collect all items on the level".to_string(),
                            _ => "Follow the level's requirements to complete it.".to_string()
                        }
                    }
                },
                _ => "Collect all items and reach the goal to complete this level.".to_string()
            };
            
            self.popup_system.show_completion_instructions(
                current_level.name.clone(),
                fallback_message
            );
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_rust_docs(&self) -> String {
        let current_level = &self.levels[self.level_idx];
        if let Some(ref url) = current_level.rust_docs_url {
            if let Err(e) = std::process::Command::new("cmd")
                .args(["/C", "start", url])
                .spawn() {
                format!("Failed to open browser: {}. Manual URL: {}", e, url)
            } else {
                format!("Opening Rust docs: {}", url)
            }
        } else {
            "No documentation URL available for this level.".to_string()
        }
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn open_rust_docs(&self) -> String {
        let current_level = &self.levels[self.level_idx];
        if let Some(ref url) = current_level.rust_docs_url {
            // For WASM, we'll use JavaScript to open the URL
            unsafe {
                let js_code = format!("window.open('{}', '_blank');", url);
                js_sys::eval(&js_code).ok();
            }
            format!("Opening Rust docs: {}", url)
        } else {
            "No documentation URL available for this level.".to_string()
        }
    }

    pub fn update_popup_system(&mut self, delta_time: f32) {
        self.popup_system.update(delta_time);
    }

    pub fn handle_popup_input(&mut self) -> PopupAction {
        let action = self.popup_system.handle_input();
        
        // Handle popup actions
        match action {
            PopupAction::NextLevel => {
                if self.level_idx + 1 < self.levels.len() {
                    self.load_level(self.level_idx + 1);
                } else {
                    // Last level completed
                    self.popup_system.show_message(
                        "🏆 Game Complete!".to_string(),
                        "Congratulations! You've completed all levels and mastered the basics of Rust programming!".to_string(),
                        crate::popup::PopupType::Success,
                        None
                    );
                }
            },
            PopupAction::StayOnLevel => {
                // Player chose to stay on current level, just clear the finished flag
                self.finished = false;
            },
            _ => {}
        }
        
        action
    }

    pub fn draw_popups(&self) {
        self.popup_system.draw();
    }

    // Laser system methods
    pub fn fire_laser_direction(&mut self, direction: (i32, i32)) -> String {
        let robot_pos = self.robot.get_position();
        let mut current_pos = (robot_pos.0 + direction.0, robot_pos.1 + direction.1);
        
        // Trace laser path until it hits something
        loop {
            let pos = crate::item::Pos { x: current_pos.0, y: current_pos.1 };
            
            // Check bounds
            if !self.grid.in_bounds(pos) {
                return "Laser fired but hit the edge of the grid.".to_string();
            }
            
            // Check for enemy hit
            for (i, enemy) in self.grid.enemies.iter().enumerate() {
                if enemy.pos == pos {
                    self.stunned_enemies.insert(i, 5); // Stun for 5 turns
                    return format!("Laser hit enemy at ({}, {})! Enemy stunned for 5 turns.", current_pos.0, current_pos.1);
                }
            }
            
            // Check for obstacle hit
            if self.grid.is_blocked(pos) {
                self.hit_obstacle_with_laser(current_pos);
                return format!("Laser hit obstacle at ({}, {})! Obstacle destroyed for 2 turns.", current_pos.0, current_pos.1);
            }
            
            // Continue laser path
            current_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
        }
    }

    pub fn fire_laser_tile(&mut self, target: (i32, i32)) -> String {
        let pos = crate::item::Pos { x: target.0, y: target.1 };
        
        // Check bounds
        if !self.grid.in_bounds(pos) {
            return "Target coordinates are outside the grid.".to_string();
        }
        
        // Check for enemy at target
        for (i, enemy) in self.grid.enemies.iter().enumerate() {
            if enemy.pos == pos {
                self.stunned_enemies.insert(i, 5); // Stun for 5 turns
                return format!("Laser hit enemy at ({}, {})! Enemy stunned for 5 turns.", target.0, target.1);
            }
        }
        
        // Check for obstacle at target
        if self.grid.is_blocked(pos) {
            self.hit_obstacle_with_laser(target);
            return format!("Laser hit obstacle at ({}, {})! Obstacle destroyed for 2 turns.", target.0, target.1);
        }
        
        "Laser fired but hit nothing at target location.".to_string()
    }

    pub fn skip_level(&mut self) -> String {
        if self.level_idx + 1 < self.levels.len() {
            self.level_idx += 1;
            self.load_level(self.level_idx);
            format!("Skipped to level {}!", self.level_idx + 1)
        } else {
            "Already at the last level!".to_string()
        }
    }

    pub fn goto_level(&mut self, target_level: usize) -> String {
        if target_level > 0 && target_level <= self.levels.len() {
            self.level_idx = target_level - 1; // Convert to 0-based index
            self.load_level(self.level_idx);
            format!("Jumped to level {}!", target_level)
        } else {
            format!("Invalid level number {}. Valid range: 1-{}", target_level, self.levels.len())
        }
    }
    
    pub fn open_door(&mut self, open: bool) -> String {
        let robot_pos = self.robot.get_position();
        let robot_item_pos = crate::item::Pos { x: robot_pos.0, y: robot_pos.1 };
        
        // Check if robot is standing on a door
        if self.grid.is_door(robot_item_pos) {
            if open {
                if self.grid.is_door_open(robot_item_pos) {
                    "Door is already open.".to_string()
                } else {
                    self.grid.open_door(robot_item_pos);
                    "Door opened successfully!".to_string()
                }
            } else {
                if !self.grid.is_door_open(robot_item_pos) {
                    "Door is already closed.".to_string()
                } else {
                    self.grid.close_door(robot_item_pos);
                    "Door closed successfully!".to_string()
                }
            }
        } else {
            "Robot must be standing on a door to open/close it.".to_string()
        }
    }

    pub fn update_laser_effects(&mut self) {
        // Update stunned enemies
        self.stunned_enemies.retain(|_, turns| {
            *turns -= 1;
            *turns > 0
        });
        
        // Update temporary removed obstacles
        self.temporary_removed_obstacles.retain(|_, turns| {
            *turns -= 1;
            *turns > 0
        });
    }

    fn hit_obstacle_with_laser(&mut self, pos: (i32, i32)) {
        // Temporarily remove obstacle for 2 turns
        self.temporary_removed_obstacles.insert(pos, 2);
    }

    fn check_completion_flag(&self, completion_flag: &str) -> bool {
        // Parse completion_flag format: "type:expected_value" or just "type"
        if completion_flag.contains(':') {
            let parts: Vec<&str> = completion_flag.splitn(2, ':').collect();
            if parts.len() != 2 {
                return false;
            }
            
            let flag_type = parts[0];
            let expected_value = parts[1];
            
            match flag_type {
                "println" | "println_exact" => {
                    // Check if any println output matches expected value
                    self.println_outputs.iter().any(|output| output == expected_value)
                },
                "eprintln" | "error_exact" => {
                    // Check if any error output matches expected value
                    self.error_outputs.iter().any(|output| output == expected_value)
                },
                "items_collected" => {
                    // Check if expected number of items collected
                    if let Ok(expected_count) = expected_value.parse::<usize>() {
                        self.robot.get_inventory_items().len() >= expected_count
                    } else {
                        false
                    }
                },
                "moves_made" => {
                    // Check if minimum number of moves made
                    if let Ok(expected_moves) = expected_value.parse::<usize>() {
                        self.turns >= expected_moves
                    } else {
                        false
                    }
                },
                _ => false
            }
        } else {
            // Simple flag types
            match completion_flag {
                "println" => !self.println_outputs.is_empty(),
                "error" | "eprintln" => !self.error_outputs.is_empty(),
                "panic" => self.panic_occurred,
                "items_collected" => !self.robot.get_inventory_items().is_empty(),
                _ => false
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
        
        // Check special completion conditions first
        let current_level = &self.levels[self.level_idx];
        
        // Check for detailed completion_flag first (more specific)
        if let Some(ref completion_flag) = current_level.completion_flag {
            if self.check_completion_flag(completion_flag) {
                let achievement = current_level.achievement_message.clone()
                    .unwrap_or_else(|| "Level completed!".to_string());
                let level_name = current_level.name.clone();
                let next_hint = current_level.next_level_hint.clone();
                self.popup_system.show_congratulations(level_name, achievement, next_hint);
                self.finish_level();
                return;
            }
        }
        
        // Fallback to basic completion condition (all items collected)
        if self.item_manager.items.is_empty() {
            self.show_level_complete();
            self.finish_level();
        }
    }
}