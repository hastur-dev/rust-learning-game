use super::types::Game;
use macroquad::prelude::*;
use crate::font_scaling::*;
use log::{debug, warn, error};

impl Game {
    // Update window position for coordinate transformations (throttled to 1x per second, more during rapid clicking)
    pub fn update_window_coordinates(&mut self) {
        // Check window activity but don't skip entirely - just be more cautious
        let window_active = crate::coordinate_system::CoordinateTransformer::is_game_window_active(self.enable_coordinate_logs);
        if !window_active {
            if self.enable_coordinate_logs {
                debug!("Window not fully active - proceeding with cautious coordinate update");
            }
            // Continue with update but maybe with reduced frequency
        }
        
        let current_time = macroquad::prelude::get_time();
        
        // Adaptive throttling: if there's been recent clicking, be more conservative
        let time_since_last_click = current_time - self.last_mouse_click_time;
        let update_interval = if time_since_last_click < 1.0 {
            5.0 // If recent clicking, wait 5 seconds between window updates
        } else {
            1.0 // Normal interval of 1 second
        };
        
        // Only update window coordinates at the determined interval to reduce overhead
        if current_time - self.last_window_update_time >= update_interval {
            // Wrap window update in safety catch to prevent crashes
            let update_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.coordinate_transformer.update_window_info();
            }));

            match update_result {
                Ok(_) => {
                    self.last_window_update_time = current_time;
                    if self.enable_coordinate_logs {
                        if let Some(window_info) = self.coordinate_transformer.get_window_info() {
                            debug!("Updated window info: {:?}", window_info);
                        } else {
                            debug!("Window info update failed");
                        }
                    }
                }
                Err(_) => {
                    warn!("Window coordinate update panicked - skipping this update");
                    // Don't update the timer so we can try again later
                }
            }
        }
    }
    
    // some cursor and scrolling helpers, but the scroll doesn't work
    pub fn position_cursor_at_click(&mut self, click_x: f32, click_y: f32, editor_bounds: (f32, f32, f32, f32)) {
        let current_time = macroquad::prelude::get_time();
        
        // Rate limit clicks to prevent rapid-fire clicking from causing issues
        let click_delay = 0.05; // Minimum 50ms between clicks
        if current_time - self.last_mouse_click_time < click_delay {
            debug!("Click rate limited - too soon after last click");
            return;
        }
        
        self.last_mouse_click_time = current_time;
        debug!("position_cursor_at_click called: coordinates=({:.2}, {:.2}), bounds={:?}", click_x, click_y, editor_bounds);
        
        // Use the new grid-based positioning system
        self.position_cursor_at_click_grid_based(click_x, click_y, editor_bounds);
    }
    
    /// Grid-based cursor positioning - simpler and more accurate
    fn position_cursor_at_click_grid_based(&mut self, click_x: f32, click_y: f32, editor_bounds: (f32, f32, f32, f32)) {
        use crate::drawing::editor_drawing::{mouse_to_grid_position, grid_to_cursor_position};
        
        // Make sure code isn't empty
        if self.current_code.is_empty() {
            self.current_code = "// Start typing your Rust code here...\n".to_string();
        }
        
        let char_width = self.get_cached_char_width();
        let line_height = self.get_cached_line_height();
        
        if let Some((row, col)) = mouse_to_grid_position(
            click_x, 
            click_y, 
            editor_bounds, 
            char_width, 
            line_height, 
            self.code_scroll_offset
        ) {
            let lines: Vec<&str> = self.current_code.lines().collect();
            let new_cursor_pos = grid_to_cursor_position(row, col, &lines);
            
            // Clamp to valid range
            self.cursor_position = new_cursor_pos.min(self.current_code.len());
            
            debug!("Grid position: ({}, {}), cursor position: {}", row, col, self.cursor_position);
        } else {
            debug!("Mouse click outside editor text area");
        }
    }
    
    fn position_cursor_at_click_internal(&mut self, click_x: f32, click_y: f32, editor_bounds: (f32, f32, f32, f32)) {
        debug!("position_cursor_at_click_internal called: click=({:.2}, {:.2}), bounds={:?}", click_x, click_y, editor_bounds);
        
        // make sure it isn't empty
        if self.current_code.is_empty() {
            warn!("current_code was empty, initializing with default content");
            self.current_code = "// Start typing your Rust code here...\n".to_string();
        }
        
        debug!("Current code length: {}, cursor_position: {}", self.current_code.len(), self.cursor_position);
        
        // please god work
        if let Some((line_index, col_index)) = self.get_character_at_position(click_x, click_y, editor_bounds) {
            debug!("Character position found: line={}, col={}", line_index, col_index);
            // change the line/column to character position
            let lines: Vec<&str> = self.current_code.lines().collect();
            let mut position = 0;
            
            // add all things from lines to the target line
            for i in 0..line_index.min(lines.len()) {
                position += lines[i].len();
                // add newline character except for the last line (unless the code ends with a newline)
                if i < lines.len() - 1 || (i == lines.len() - 1 && self.current_code.ends_with('\n')) {
                    position += 1;
                }
            }
            
            // column offset
            if line_index < lines.len() {
                position += col_index.min(lines[line_index].len());
            }
            
            // trailing new lines were a sin. So we gotta deal with them here hopefully
            if line_index == lines.len() && self.current_code.ends_with('\n') {
                position += 1;
            }
            
            self.cursor_position = position.min(self.current_code.len());
            debug!("New cursor position: {}", self.cursor_position);
            self.clear_selection(); // delete selected thingy
            self.ensure_cursor_visible();
        } else {
            warn!("get_character_at_position returned None for click at ({:.2}, {:.2})", click_x, click_y);
        }
    }
    
    pub fn move_cursor_up(&mut self) {
        let lines: Vec<&str> = self.current_code.lines().collect();
        let cursor_line = self.current_code[..self.cursor_position].matches('\n').count();
        
        if cursor_line > 0 {
            let line_start = self.current_code[..self.cursor_position].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let cursor_col = self.cursor_position - line_start;
            
            let prev_line_start = if cursor_line > 1 {
                self.current_code[..line_start.saturating_sub(1)].rfind('\n').map(|i| i + 1).unwrap_or(0)
            } else {
                0
            };
            
            let prev_line_len = if cursor_line > 0 {
                lines[cursor_line - 1].len()
            } else {
                0
            };
            
            let new_col = cursor_col.min(prev_line_len);
            self.cursor_position = prev_line_start + new_col;
            self.ensure_cursor_visible();
        }
    }
    
    pub fn move_cursor_down(&mut self) {
        let lines: Vec<&str> = self.current_code.lines().collect();
        let cursor_line = self.current_code[..self.cursor_position].matches('\n').count();
        
        if cursor_line < lines.len().saturating_sub(1) {
            let line_start = self.current_code[..self.cursor_position].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let cursor_col = self.cursor_position - line_start;
            
            let next_line_start = line_start + lines[cursor_line].len() + 1; // +1 for newline
            let next_line_len = if cursor_line + 1 < lines.len() {
                lines[cursor_line + 1].len()
            } else {
                0
            };
            
            let new_col = cursor_col.min(next_line_len);
            self.cursor_position = next_line_start + new_col;
            self.ensure_cursor_visible();
        }
    }
    
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.ensure_cursor_visible();
        }
    }
    
    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.current_code.len() {
            self.cursor_position += 1;
            self.ensure_cursor_visible();
        }
    }
    
    pub fn ensure_cursor_visible(&mut self) {
        let cursor_line = self.current_code[..self.cursor_position].matches('\n').count();
        let max_visible_lines = 30;
        
        // scroll up if cursor is above visible area if the scroll works
        if cursor_line < self.code_scroll_offset {
            self.code_scroll_offset = cursor_line;
        }
        
        // scroll down if cursor is below visible area if the scroll works
        if cursor_line >= self.code_scroll_offset + max_visible_lines {
            self.code_scroll_offset = cursor_line.saturating_sub(max_visible_lines - 1);
        }
    }
    
    pub fn scroll_up(&mut self) {
        if self.code_scroll_offset > 0 {
            self.code_scroll_offset -= 1;
        }
    }
    
    pub fn scroll_down(&mut self) {
        let lines: Vec<&str> = self.current_code.lines().collect();
        let max_visible_lines = 30;
        if self.code_scroll_offset + max_visible_lines < lines.len() {
            self.code_scroll_offset += 1;
        }
    }
    
    // holding down a key should keep doing the thing
    pub fn update_key_press_timers(&mut self, delta_time: f32) {
        // update backspace hold time. There's a better way to do this and I'm sure I'll get to it later
        if is_key_down(KeyCode::Backspace) {
            self.key_backspace_held_time += delta_time;
        } else {
            self.key_backspace_held_time = 0.0;
        }
        
        // Update space hold time
        if is_key_down(KeyCode::Space) {
            self.key_space_held_time += delta_time;
        } else {
            self.key_space_held_time = 0.0;
        }
        
        // Update arrow key hold times
        if is_key_down(KeyCode::Up) {
            self.key_up_held_time += delta_time;
        } else {
            self.key_up_held_time = 0.0;
        }
        
        if is_key_down(KeyCode::Down) {
            self.key_down_held_time += delta_time;
        } else {
            self.key_down_held_time = 0.0;
        }
        
        if is_key_down(KeyCode::Left) {
            self.key_left_held_time += delta_time;
        } else {
            self.key_left_held_time = 0.0;
        }
        
        if is_key_down(KeyCode::Right) {
            self.key_right_held_time += delta_time;
        } else {
            self.key_right_held_time = 0.0;
        }
    }
    
    // update character timing if we hold down teh button
    pub fn update_char_key_timing(&mut self, char_pressed: Option<char>, delta_time: f32) {
        match (char_pressed, self.last_char_pressed) {
            (Some(current_char), Some(last_char)) if current_char == last_char => {
                self.key_char_held_time += delta_time;
            },
            (Some(current_char), _) => {
                // reset time when ya press a new button
                self.last_char_pressed = Some(current_char);
                self.key_char_held_time = 0.0;
            },
            (None, _) => {
                // you didn't do shit so go back to monkey
                self.last_char_pressed = None;
                self.key_char_held_time = 0.0;
            }
        }
    }
    
    pub fn should_repeat_backspace(&self) -> bool {
        self.key_backspace_held_time > self.key_repeat_initial_delay &&
        ((self.key_backspace_held_time - self.key_repeat_initial_delay) % self.key_repeat_interval) < self.key_repeat_interval / 2.0
    }
    
    pub fn should_repeat_space(&self) -> bool {
        self.key_space_held_time > self.key_repeat_initial_delay &&
        ((self.key_space_held_time - self.key_repeat_initial_delay) % self.key_repeat_interval) < self.key_repeat_interval / 2.0
    }
    
    pub fn should_repeat_char(&self) -> bool {
        self.key_char_held_time > self.key_repeat_initial_delay &&
        ((self.key_char_held_time - self.key_repeat_initial_delay) % self.key_repeat_interval) < self.key_repeat_interval / 2.0
    }
    
    // Arrow key repeat functions
    pub fn should_repeat_up(&self) -> bool {
        self.key_up_held_time > self.key_repeat_initial_delay &&
        ((self.key_up_held_time - self.key_repeat_initial_delay) % self.key_repeat_interval) < self.key_repeat_interval / 2.0
    }
    
    pub fn should_repeat_down(&self) -> bool {
        self.key_down_held_time > self.key_repeat_initial_delay &&
        ((self.key_down_held_time - self.key_repeat_initial_delay) % self.key_repeat_interval) < self.key_repeat_interval / 2.0
    }
    
    pub fn should_repeat_left(&self) -> bool {
        self.key_left_held_time > self.key_repeat_initial_delay &&
        ((self.key_left_held_time - self.key_repeat_initial_delay) % self.key_repeat_interval) < self.key_repeat_interval / 2.0
    }
    
    pub fn should_repeat_right(&self) -> bool {
        self.key_right_held_time > self.key_repeat_initial_delay &&
        ((self.key_right_held_time - self.key_repeat_initial_delay) % self.key_repeat_interval) < self.key_repeat_interval / 2.0
    }
    
    // selection management stuff
    pub fn start_selection(&mut self) {
        if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_position);
            self.selection_end = Some(self.cursor_position);
        }
    }
    
    pub fn update_selection(&mut self, new_cursor_pos: usize) {
        if self.selection_start.is_some() {
            self.selection_end = Some(new_cursor_pos);
        }
    }
    
    pub fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }
    
    pub fn has_selection(&self) -> bool {
        self.selection_start.is_some() && self.selection_end.is_some() &&
        self.selection_start != self.selection_end
    }
    
    pub fn get_selection_bounds(&self) -> Option<(usize, usize)> {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            if start != end {
                Some((start.min(end), start.max(end)))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub fn delete_selection(&mut self) -> bool {
        if let Some((start, end)) = self.get_selection_bounds() {
            self.current_code.drain(start..end);
            self.cursor_position = start;
            self.clear_selection();
            self.ensure_cursor_visible();
            true
        } else {
            false
        }
    }
    
    // I wanted something that would let me hold shift and arrow keys to select multiple stuff here. I find out if it works soon
    pub fn move_cursor_up_with_selection(&mut self, extend_selection: bool) {
        if extend_selection {
            self.start_selection();
        } else {
            self.clear_selection();
        }
        
        self.move_cursor_up();
        
        if extend_selection {
            self.update_selection(self.cursor_position);
        }
    }
    
    pub fn move_cursor_down_with_selection(&mut self, extend_selection: bool) {
        if extend_selection {
            self.start_selection();
        } else {
            self.clear_selection();
        }
        
        self.move_cursor_down();
        
        if extend_selection {
            self.update_selection(self.cursor_position);
        }
    }
    
    pub fn move_cursor_left_with_selection(&mut self, extend_selection: bool) {
        if extend_selection {
            self.start_selection();
        } else {
            self.clear_selection();
        }
        
        self.move_cursor_left();
        
        if extend_selection {
            self.update_selection(self.cursor_position);
        }
    }
    
    pub fn move_cursor_right_with_selection(&mut self, extend_selection: bool) {
        if extend_selection {
            self.start_selection();
        } else {
            self.clear_selection();
        }
        
        self.move_cursor_right();
        
        if extend_selection {
            self.update_selection(self.cursor_position);
        }
    }
    // it worked so I can forget how I did this
    // font measure caching thing
    pub fn refresh_font_measurements(&mut self) {
        let font_size = scale_font_size(12.0);
        let char_width = measure_text("M", None, font_size as u16, 1.0).width;
        let line_height = scale_size(14.0);
        
        self.cached_font_size = font_size;
        self.cached_char_width = char_width;
        self.cached_line_height = line_height;
        self.needs_font_refresh = false;
    }
    
    // figure out the position of stuff
    pub fn get_text_position(&mut self, line_index: usize, col_index: usize, editor_bounds: (f32, f32, f32, f32)) -> (f32, f32) {
        let (editor_x, editor_y, _editor_width, _editor_height) = editor_bounds;
        let input_y = editor_y + scale_size(55.0);
        let line_height = self.get_cached_line_height();
        let cached_font_size = self.get_cached_font_size(); // get cached values first
        let line_number_width = scale_size(35.0);
        let text_x = editor_x + line_number_width + scale_size(5.0);
        let text_start_offset = scale_size(12.0);
        
        // do the y position
        let display_line = line_index.saturating_sub(self.code_scroll_offset);
        let y = input_y + text_start_offset + (display_line as f32 * line_height);
        
        // do the x position by measuring exact character width
        let lines: Vec<&str> = self.current_code.lines().collect();
        let mut x = text_x;
        
        if line_index < lines.len() && col_index <= lines[line_index].len() {
            let line = lines[line_index];
            let text_before = &line[..col_index];
            let text_dimensions = measure_text(text_before, None, cached_font_size as u16, 1.0);
            x += text_dimensions.width;
        }
        
        (x, y)
    }
    
    pub fn get_character_at_position(&mut self, click_x: f32, click_y: f32, editor_bounds: (f32, f32, f32, f32)) -> Option<(usize, usize)> {
        // make sure current_code is not empty
        if self.current_code.is_empty() {
            return Some((0, 0));
        }
        
        let (editor_x, editor_y, _editor_width, _editor_height) = editor_bounds;
        let input_y = editor_y + scale_size(55.0);
        let line_height = self.get_cached_line_height();
        let cached_font_size = self.get_cached_font_size(); // cache first
        let line_number_width = scale_size(35.0);
        let text_x = editor_x + line_number_width + scale_size(5.0);
        let text_start_offset = scale_size(12.0);
        
        // check if you clicked within the thing
        if click_x < text_x || click_y < input_y + text_start_offset {
            return None;
        }
        
        // figure out which line was clicked and probably get it wrong
        let click_offset_from_text_start = click_y - (input_y + text_start_offset);
        let clicked_line_offset = (click_offset_from_text_start / line_height).floor() as usize;
        let target_line = self.code_scroll_offset + clicked_line_offset;
        
        let lines: Vec<&str> = self.current_code.lines().collect();
        if target_line >= lines.len() {
            // If you went to far then have it go to last line
            if let Some(last_line) = lines.last() {
                return Some((lines.len() - 1, last_line.len()));
            }
            return Some((0, 0));
        }
        
        // find character position within the line using precise measurements
        let line = lines[target_line];
        let click_offset = click_x - text_x;
        
        // Found some code online that chatgpt and claude explained to me. This looks like a good idea for precision
        let mut best_col = 0;
        let mut best_distance = f32::INFINITY;
        
        for col in 0..=line.len() {
            let text_before = &line[..col];
            let text_dimensions = measure_text(text_before, None, cached_font_size as u16, 1.0);
            let distance = (text_dimensions.width - click_offset).abs();
            
            if distance < best_distance {
                best_distance = distance;
                best_col = col;
            }
        }
        
        Some((target_line, best_col))
    }
    
    pub fn get_cached_font_size(&mut self) -> f32 {
        if self.needs_font_refresh {
            self.refresh_font_measurements();
        }
        self.cached_font_size
    }

    pub fn get_cached_char_width(&mut self) -> f32 {
        if self.needs_font_refresh {
            self.refresh_font_measurements();
        }
        self.cached_char_width
    }

    // Mouse drag selection methods
    pub fn start_mouse_drag(&mut self, mouse_x: f32, mouse_y: f32, editor_bounds: (f32, f32, f32, f32)) {
        println!("🖱️  Starting mouse drag at ({:.1}, {:.1})", mouse_x, mouse_y);
        self.mouse_drag_start = Some((mouse_x, mouse_y));
        self.is_dragging = false; // Will become true when mouse moves

        // Position cursor at click location and save this as our selection start
        let initial_cursor = self.cursor_position;
        self.position_cursor_at_click(mouse_x, mouse_y, editor_bounds);
        println!("🖱️  Cursor moved from {} to {}", initial_cursor, self.cursor_position);

        // Clear any existing selection and set the drag start position for selection
        self.clear_selection();
        // Store the click position as the potential selection start
        self.selection_start = Some(self.cursor_position);
    }

    pub fn update_mouse_drag(&mut self, mouse_x: f32, mouse_y: f32, editor_bounds: (f32, f32, f32, f32)) {
        if let Some((start_x, start_y)) = self.mouse_drag_start {
            // Check if mouse has moved enough to start dragging
            let drag_threshold = 3.0; // pixels
            let moved_distance = ((mouse_x - start_x).powi(2) + (mouse_y - start_y).powi(2)).sqrt();

            println!("🖱️  Mouse at ({:.1}, {:.1}), moved {:.1}px from start", mouse_x, mouse_y, moved_distance);

            if moved_distance > drag_threshold {
                if !self.is_dragging {
                    // Start dragging - selection start was already set in start_mouse_drag
                    println!("🖱️  Starting text selection (threshold exceeded)");
                    self.is_dragging = true;
                    println!("🖱️  Selection started at cursor position {}", self.selection_start.unwrap_or(0));
                }

                // Update selection end to current mouse position
                let old_cursor = self.cursor_position;
                self.position_cursor_at_click(mouse_x, mouse_y, editor_bounds);
                let new_cursor = self.cursor_position;

                // Update selection end
                self.selection_end = Some(new_cursor);
                println!("🖱️  Selection updated: {} to {}",
                    self.selection_start.unwrap_or(0), self.selection_end.unwrap_or(0));

                // Restore cursor to selection end
                self.cursor_position = new_cursor;
            }
        }
    }

    pub fn end_mouse_drag(&mut self) {
        self.mouse_drag_start = None;
        self.is_dragging = false;

        // If we have a selection but start equals end, clear it
        if let Some((start, end)) = self.get_selection_bounds() {
            if start == end {
                self.clear_selection();
            }
        }
    }

    pub fn select_left(&mut self) {
        if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_position);
        }
        self.move_cursor_left();
        self.selection_end = Some(self.cursor_position);
        println!("⬅️  Selection: {:?} to {:?}", self.selection_start, self.selection_end);
    }

    pub fn select_right(&mut self) {
        if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_position);
        }
        self.move_cursor_right();
        self.selection_end = Some(self.cursor_position);
        println!("➡️  Selection: {:?} to {:?}", self.selection_start, self.selection_end);
    }

    pub fn select_up(&mut self) {
        if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_position);
        }
        self.move_cursor_up();
        self.selection_end = Some(self.cursor_position);
        println!("⬆️  Selection: {:?} to {:?}", self.selection_start, self.selection_end);
    }

    pub fn select_down(&mut self) {
        if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_position);
        }
        self.move_cursor_down();
        self.selection_end = Some(self.cursor_position);
        println!("⬇️  Selection: {:?} to {:?}", self.selection_start, self.selection_end);
    }
    
    pub fn get_cached_line_height(&mut self) -> f32 {
        if self.needs_font_refresh {
            self.refresh_font_measurements();
        }
        self.cached_line_height
    }
    
    pub fn invalidate_font_cache(&mut self) {
        self.needs_font_refresh = true;
    }
}