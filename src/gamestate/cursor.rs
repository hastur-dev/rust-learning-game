use super::types::Game;
use macroquad::prelude::*;
use crate::font_scaling::*;
use log::{debug, warn, error};

impl Game {
    // some cursor and scrolling helpers, but the scroll doesn't work
    pub fn position_cursor_at_click(&mut self, click_x: f32, click_y: f32, editor_bounds: (f32, f32, f32, f32)) {
        debug!("position_cursor_at_click called: click=({:.2}, {:.2}), bounds={:?}", click_x, click_y, editor_bounds);
        
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
    
    fn ensure_cursor_visible(&mut self) {
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