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
            mouse_drag_start: None,
            is_dragging: false,
            code_scroll_offset: 0,
            code_lines_visible: 30, // Default number of lines visible
            tutorial_scroll_offset: 0,
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
            key_up_held_time: 0.0,
            key_down_held_time: 0.0,
            key_left_held_time: 0.0,
            key_right_held_time: 0.0,
            key_repeat_initial_delay: 0.5, // Wait 0.5 seconds before starting to repeat
            key_repeat_interval: 0.05,     // Repeat every 50ms after initial delay
            cached_font_size: 0.0,
            cached_char_width: 0.0,
            cached_line_height: 0.0,
            needs_font_refresh: true,      // Initially needs refresh
            editor_tab: EditorTab::Commands, // Default to Commands tab
            coordinate_transformer: crate::coordinate_system::CoordinateTransformer::new(), // Initialize coordinate transformer
            last_system_key_time: 0.0,    // Initialize system key timer
            enable_coordinate_logs: false, // Default to disabled, enabled via --all-logs command line flag
            last_window_update_time: 0.0, // Initialize timer
            last_mouse_click_time: 0.0,   // Initialize click timer
            autocomplete_engine: crate::autocomplete::AutocompleteEngine::new(),
            autocomplete_enabled: true,   // Enable autocomplete by default
            hotkey_system: crate::hotkeys::HotkeySystem::new(),
            // Initialize undo functionality
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
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

        // For Level 6, also reveal robot fleet positions for ownership demonstration
        if idx == 5 { // Level 6 (0-indexed)
            println!("🎮 Initializing Level 6 - revealing robot fleet positions (found {} enemies)", grid.enemies.len());
            for enemy in &grid.enemies {
                if let Some(ref pattern) = enemy.movement_pattern {
                    if pattern.contains("ownership_demo") ||
                       pattern.contains("borrowing_demo") ||
                       pattern.contains("clone_demo") {
                        println!("👁️ Revealing robot {} at position ({}, {})", pattern, enemy.pos.x, enemy.pos.y);
                        grid.known.insert(crate::item::Pos { x: enemy.pos.x, y: enemy.pos.y });
                    }
                } else {
                    println!("⚠️ Enemy at ({}, {}) has no movement pattern", enemy.pos.x, enemy.pos.y);
                }
            }
        }

        self.grid = grid;
        self.turns = 0;
        self.max_turns = spec.max_turns;
        self.discovered_this_level = 0;
        self.finished = false;
        self.scan_armed = false;
        self.enemy_step_paused = false;
        
        // Reset tutorial state and outputs for learning levels when starting fresh
        let should_reset_tutorial = if self.is_learning_level(idx) {
            // Reset if coming from a different level OR if current level tutorial is complete
            self.level_idx != idx || self.is_current_level_tutorial_complete()
        } else {
            false
        };
        
        if should_reset_tutorial {
            // Reset tutorial state for learning levels
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
        } else if !self.is_learning_level(idx) {
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

    // Autocomplete integration methods
    pub fn update_autocomplete(&mut self) {
        if self.autocomplete_enabled {
            self.autocomplete_engine.update_suggestions(&self.current_code, self.cursor_position);

            // Update user symbols occasionally for better completions (not every frame to avoid lag)
            if self.turns % 5 == 0 {  // Every 5 game turns to avoid lag
                self.autocomplete_engine.update_user_symbols(&self.current_code);
            }
        }
    }

    pub fn get_autocomplete_suggestion(&self) -> Option<&crate::autocomplete::AutocompleteSuggestion> {
        if self.autocomplete_enabled {
            self.autocomplete_engine.get_current_suggestion()
        } else {
            None
        }
    }

    pub fn accept_autocomplete(&mut self) -> bool {
        if self.autocomplete_enabled {
            if let Some(completion) = self.autocomplete_engine.accept_suggestion() {
                // Get the current word to replace
                let current_word = self.get_current_word_at_cursor();
                let start_pos = self.cursor_position - current_word.len();

                // Remove the current partial word
                for _ in 0..current_word.len() {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                        self.current_code.remove(self.cursor_position);
                    }
                }

                // Insert the completion
                for (i, ch) in completion.chars().enumerate() {
                    self.current_code.insert(self.cursor_position + i, ch);
                }
                self.cursor_position += completion.len();

                return true;
            }
        }
        false
    }

    pub fn toggle_autocomplete(&mut self) {
        self.autocomplete_enabled = !self.autocomplete_enabled;
        if !self.autocomplete_enabled {
            self.autocomplete_engine.clear_suggestion();
        }
    }

    fn get_current_word_at_cursor(&self) -> String {
        let chars: Vec<char> = self.current_code.chars().collect();
        let mut start = self.cursor_position;

        // Find start of current word
        while start > 0 {
            let prev_char = chars[start - 1];
            if prev_char.is_alphanumeric() || prev_char == '_' {
                start -= 1;
            } else {
                break;
            }
        }

        if start < self.cursor_position {
            chars[start..self.cursor_position].iter().collect()
        } else {
            String::new()
        }
    }

    // Hotkey system integration methods
    pub fn handle_hotkey(&mut self, key: macroquad::prelude::KeyCode, ctrl: bool, shift: bool, alt: bool) -> bool {
        if let Some(action) = self.hotkey_system.get_action_for_input(key, ctrl, shift, alt) {
            self.execute_hotkey_action(action)
        } else {
            false
        }
    }

    fn execute_hotkey_action(&mut self, action: crate::hotkeys::EditorAction) -> bool {
        match action {
            crate::hotkeys::EditorAction::Accept => {
                self.accept_autocomplete()
            },
            crate::hotkeys::EditorAction::ToggleEditor => {
                self.code_editor_active = !self.code_editor_active;
                true
            },
            crate::hotkeys::EditorAction::SaveFile => {
                self.save_robot_code();
                true
            },
            // Add more actions as needed
            _ => false,
        }
    }

    pub fn load_hotkey_config(&mut self) -> Result<(), String> {
        self.hotkey_system.load_config()
    }

    pub fn save_hotkey_config(&self) -> Result<(), String> {
        self.hotkey_system.save_config()
    }

    // Menu integration methods for autocomplete settings
    pub fn apply_menu_settings(&mut self, settings: &crate::menu::GameSettings) {
        self.autocomplete_enabled = settings.autocomplete_enabled;
        self.autocomplete_engine.set_enabled(settings.autocomplete_enabled);
        self.autocomplete_engine.set_vscode_enabled(settings.vscode_integration_enabled);
    }

    pub fn toggle_autocomplete_setting(&mut self) -> bool {
        self.autocomplete_enabled = !self.autocomplete_enabled;
        self.autocomplete_engine.set_enabled(self.autocomplete_enabled);
        self.autocomplete_enabled
    }

    pub fn toggle_vscode_integration_setting(&mut self) -> bool {
        let new_state = !self.autocomplete_engine.is_vscode_enabled();
        self.autocomplete_engine.set_vscode_enabled(new_state);
        new_state
    }

    pub fn is_vscode_available(&self) -> bool {
        self.autocomplete_engine.is_vscode_available()
    }

    // Clipboard and Undo/Redo functionality
    pub fn save_undo_state(&mut self) {
        let undo_state = UndoState {
            code: self.current_code.clone(),
            cursor_position: self.cursor_position,
            selection_start: self.selection_start,
            selection_end: self.selection_end,
        };

        self.undo_stack.push(undo_state);
        // Clear redo stack when new action is performed
        self.redo_stack.clear();

        // Limit undo stack size to prevent memory issues
        if self.undo_stack.len() > 100 {
            self.undo_stack.remove(0);
        }

        println!("📚 Undo state saved. Stack size: {}", self.undo_stack.len());
    }

    // Smart undo state saving for typing operations
    pub fn save_undo_state_if_needed(&mut self, force: bool) {
        // Always save if forced (e.g., before backspace, paste)
        if force {
            self.save_undo_state();
            return;
        }

        // For typing: only save if this is the start of a new typing session
        // or if significant time has passed since last action
        if self.undo_stack.is_empty() {
            // First action ever
            self.save_undo_state();
        } else if let Some(last_undo) = self.undo_stack.last() {
            // Save if the code has changed significantly or cursor jumped
            let code_length_diff = (self.current_code.len() as i32 - last_undo.code.len() as i32).abs();
            let cursor_jump = (self.cursor_position as i32 - last_undo.cursor_position as i32).abs();

            // Save undo state if:
            // 1. Code length changed significantly (> 10 chars)
            // 2. Cursor jumped significantly (> 5 positions without typing)
            // 3. It's been a while since last save (we could add timing later)
            if code_length_diff > 10 || cursor_jump > 5 {
                self.save_undo_state();
            }
        }
    }

    pub fn copy_to_clipboard(&mut self) -> bool {
        if let Some((start, end)) = self.get_selection_bounds() {
            let selected_text = self.current_code[start..end].to_string();

            // Use safe clipboard operation to prevent crashes on focus loss
            if crate::crash_protection::safe_clipboard_copy(&selected_text) {
                println!("📋 Copied to OS clipboard: '{}'", selected_text);
                true
            } else {
                println!("❌ Failed to copy to OS clipboard (window may not be focused)");
                false
            }
        } else {
            println!("📋 No text selected for copy");
            false
        }
    }

    pub fn cut_to_clipboard(&mut self) -> bool {
        if self.copy_to_clipboard() {
            self.save_undo_state();
            if let Some((start, end)) = self.get_selection_bounds() {
                self.current_code.drain(start..end);
                self.cursor_position = start;
                self.clear_selection();
                self.ensure_cursor_visible();
                println!("✂️ Cut text to clipboard");
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn paste_from_clipboard(&mut self) -> bool {
        // Use safe clipboard operation to prevent crashes on focus loss
        if let Some(clipboard_text) = crate::crash_protection::safe_clipboard_paste() {
            self.save_undo_state();

            // Delete selection if exists
            if self.delete_selection() {
                // Selection already deleted by delete_selection
            }

            // Insert clipboard content at cursor position
            self.current_code.insert_str(self.cursor_position, &clipboard_text);
            self.cursor_position += clipboard_text.len();
            self.clear_selection();
            self.ensure_cursor_visible();

            println!("📋 Pasted from OS clipboard: '{}'", clipboard_text);
            true
        } else {
            println!("❌ Failed to paste from OS clipboard (window may not be focused or clipboard empty)");
            false
        }
    }

    pub fn undo(&mut self) -> bool {
        if let Some(undo_state) = self.undo_stack.pop() {
            // Save current state to redo stack
            let redo_state = UndoState {
                code: self.current_code.clone(),
                cursor_position: self.cursor_position,
                selection_start: self.selection_start,
                selection_end: self.selection_end,
            };
            self.redo_stack.push(redo_state);

            // Restore previous state
            self.current_code = undo_state.code;
            self.cursor_position = undo_state.cursor_position;
            self.selection_start = undo_state.selection_start;
            self.selection_end = undo_state.selection_end;
            self.ensure_cursor_visible();
            println!("↶ Undo performed");
            true
        } else {
            println!("↶ Nothing to undo");
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(redo_state) = self.redo_stack.pop() {
            // Save current state to undo stack
            let undo_state = UndoState {
                code: self.current_code.clone(),
                cursor_position: self.cursor_position,
                selection_start: self.selection_start,
                selection_end: self.selection_end,
            };
            self.undo_stack.push(undo_state);

            // Restore redo state
            self.current_code = redo_state.code;
            self.cursor_position = redo_state.cursor_position;
            self.selection_start = redo_state.selection_start;
            self.selection_end = redo_state.selection_end;
            self.ensure_cursor_visible();
            println!("↷ Redo performed");
            true
        } else {
            println!("↷ Nothing to redo");
            false
        }
    }

    pub fn select_all(&mut self) {
        self.selection_start = Some(0);
        self.selection_end = Some(self.current_code.len());
        self.cursor_position = self.current_code.len();
        println!("🔲 Selected all text ({} characters)", self.current_code.len());
    }

    // Helper function to determine if special robots should be shown at a position
    pub fn should_show_special_robots_at(&self, pos: crate::item::Pos) -> bool {
        if !self.is_learning_level(self.level_idx) {
            return false;
        }

        self.grid.enemies.iter().any(|e| {
            e.pos == pos && self.is_special_robot_for_current_level(e)
        })
    }

    // Check if an enemy is a special robot for the current learning level
    fn is_special_robot_for_current_level(&self, enemy: &crate::grid::Enemy) -> bool {
        match self.level_idx {
            17 => self.is_ownership_robot(enemy),        // Level 18: Memory Management (ownership)
            1 | 7 | 8 | 9 | 10 => self.is_cloning_robot(enemy),  // Levels with cloning concepts
            13 | 14 | 15 => self.is_serde_robot(enemy),  // Serde levels
            _ => false
        }
    }

    // Check if enemy is an ownership robot (Robot Alpha)
    fn is_ownership_robot(&self, enemy: &crate::grid::Enemy) -> bool {
        enemy.movement_pattern.as_ref()
            .map_or(false, |pattern| pattern.contains("ownership"))
    }

    // Check if enemy is a cloning robot (Robot Gamma)
    fn is_cloning_robot(&self, enemy: &crate::grid::Enemy) -> bool {
        enemy.movement_pattern.as_ref()
            .map_or(false, |pattern| pattern.contains("clone"))
    }

    // Check if enemy is a serde robot (Scout Bot)
    fn is_serde_robot(&self, enemy: &crate::grid::Enemy) -> bool {
        enemy.movement_pattern.as_ref()
            .map_or(false, |pattern| pattern.contains("serde") || pattern.contains("scout"))
    }

    // Get robot symbol and font size for current level
    pub fn get_robot_symbol_for_level(&self, enemy: &crate::grid::Enemy) -> (&'static str, f32) {
        if !self.is_learning_level(self.level_idx) {
            return ("E", 28.0); // Standard enemy symbol for non-learning levels
        }

        match self.level_idx {
            17 if self.is_ownership_robot(enemy) => ("⚡", 24.0), // Robot Alpha - ownership
            1 | 7 | 8 | 9 | 10 if self.is_cloning_robot(enemy) => ("◆", 24.0), // Robot Gamma - cloning
            13 | 14 | 15 if self.is_serde_robot(enemy) => ("📡", 20.0), // Scout Bot - serde
            _ => {
                // Regular enemies in learning levels
                ("E", 28.0) // Standard enemy symbol
            }
        }
    }

    // Get robot color for current level
    pub fn get_robot_color_for_level(&self, enemy: &crate::grid::Enemy) -> macroquad::color::Color {
        use macroquad::color::*;
        use crate::level::EnemyDirection;

        if !self.is_learning_level(self.level_idx) {
            // Standard enemy colors for non-learning levels
            return if let Some(ref pattern) = enemy.movement_pattern {
                match pattern.as_str() {
                    "chase" => {
                        if let Some(is_chasing) = enemy.movement_data.get("is_chasing")
                            .and_then(|v| v.as_bool()) {
                            if is_chasing { ORANGE } else { BLUE }
                        } else { ORANGE }
                    }
                    "random" => MAGENTA,
                    "diagonal" => YELLOW,
                    "circular" => LIME,
                    "spiral" => PINK,
                    pattern if pattern.starts_with("file:") => PURPLE,
                    _ => RED
                }
            } else {
                match enemy.direction {
                    EnemyDirection::Horizontal => GREEN,
                    EnemyDirection::Vertical => DARKBLUE,
                }
            };
        }

        // Colors for learning level robots
        match self.level_idx {
            17 if self.is_ownership_robot(enemy) => SKYBLUE,    // Robot Alpha - ownership
            1 | 7 | 8 | 9 | 10 if self.is_cloning_robot(enemy) => YELLOW, // Robot Gamma - cloning
            13 | 14 | 15 if self.is_serde_robot(enemy) => LIME, // Scout Bot - serde (bright green)
            _ => {
                // Regular enemies in learning levels
                if let Some(ref pattern) = enemy.movement_pattern {
                    match pattern.as_str() {
                        "chase" => {
                            if let Some(is_chasing) = enemy.movement_data.get("is_chasing")
                                .and_then(|v| v.as_bool()) {
                                if is_chasing { ORANGE } else { BLUE }
                            } else { ORANGE }
                        }
                        "random" => MAGENTA,
                        "diagonal" => YELLOW,
                        "circular" => LIME,
                        "spiral" => PINK,
                        pattern if pattern.starts_with("file:") => PURPLE,
                        _ => RED
                    }
                } else {
                    match enemy.direction {
                        EnemyDirection::Horizontal => GREEN,
                        EnemyDirection::Vertical => DARKBLUE,
                    }
                }
            }
        }
    }
}