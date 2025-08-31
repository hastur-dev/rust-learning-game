use super::types::Game;
use macroquad::prelude::*;
use crate::font_scaling::*;

impl Game {
    // Cursor and scrolling helper methods
    pub fn position_cursor_at_click(&mut self, click_x: f32, click_y: f32, editor_bounds: (f32, f32, f32, f32)) {
        let (editor_x, editor_y, _editor_width, _editor_height) = editor_bounds;
        let input_y = editor_y + scale_size(55.0);
        let line_height = self.get_cached_line_height(); // Use cached line height
        let line_number_width = scale_size(35.0);
        let text_x = editor_x + line_number_width + scale_size(5.0);
        
        // Check if click is within text area
        if click_x >= text_x && click_y >= input_y {
            // Get cached font measurements first to avoid borrowing issues
            let cached_font_size = self.get_cached_font_size();
            
            // Calculate which line was clicked - need to account for the text offset within the line
            let click_offset_from_input = click_y - input_y;
            let text_start_offset = scale_size(12.0); // This matches the y offset used in drawing
            
            // Calculate the line index more accurately
            let clicked_line_offset = if click_offset_from_input >= text_start_offset {
                ((click_offset_from_input - text_start_offset) / line_height) as usize
            } else {
                0
            };
            
            let lines: Vec<&str> = self.current_code.lines().collect();
            let target_line = (self.code_scroll_offset + clicked_line_offset).min(lines.len());
            
            // Calculate the absolute position in the string
            let mut position = 0;
            
            // Add all characters from lines before the target line
            for i in 0..target_line.min(lines.len()) {
                position += lines[i].len();
                // Add newline character except for the last line (unless the code ends with a newline)
                if i < lines.len() - 1 || (i == lines.len() - 1 && self.current_code.ends_with('\n')) {
                    position += 1;
                }
            }
            
            // For the target line, find the closest character position
            if target_line < lines.len() {
                let line = lines[target_line];
                let click_offset = click_x - text_x;
                
                // Find the character position within the line by measuring text width
                let mut best_col = 0;
                let mut best_distance = f32::INFINITY;
                
                for col in 0..=line.len() {
                    let text_before = &line[..col];
                    // Use cached font size for consistent measurements
                    let text_dimensions = measure_text(text_before, None, cached_font_size as u16, 1.0);
                    let text_width = text_dimensions.width;
                    let distance = (text_width - click_offset).abs();
                    
                    if distance < best_distance {
                        best_distance = distance;
                        best_col = col;
                    }
                }
                
                position += best_col;
            }
            
            // Handle the case where there's a trailing newline
            if target_line == lines.len() && self.current_code.ends_with('\n') {
                position += 1;
            }
            
            self.cursor_position = position.min(self.current_code.len());
            self.clear_selection(); // Clear any existing selection when clicking
            self.ensure_cursor_visible();
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
        
        // Scroll up if cursor is above visible area
        if cursor_line < self.code_scroll_offset {
            self.code_scroll_offset = cursor_line;
        }
        
        // Scroll down if cursor is below visible area
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
    
    // Handle continuous key press timing
    pub fn update_key_press_timers(&mut self, delta_time: f32) {
        // Update backspace hold time
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
    
    pub fn should_repeat_backspace(&self) -> bool {
        self.key_backspace_held_time > self.key_repeat_initial_delay &&
        ((self.key_backspace_held_time - self.key_repeat_initial_delay) % self.key_repeat_interval) < self.key_repeat_interval / 2.0
    }
    
    pub fn should_repeat_space(&self) -> bool {
        self.key_space_held_time > self.key_repeat_initial_delay &&
        ((self.key_space_held_time - self.key_repeat_initial_delay) % self.key_repeat_interval) < self.key_repeat_interval / 2.0
    }
    
    // Selection management functions
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
    
    // Movement functions with selection support
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
    
    // Font measurement caching functions
    pub fn refresh_font_measurements(&mut self) {
        let font_size = scale_font_size(12.0);
        let char_width = measure_text("M", None, font_size as u16, 1.0).width;
        let line_height = scale_size(14.0);
        
        self.cached_font_size = font_size;
        self.cached_char_width = char_width;
        self.cached_line_height = line_height;
        self.needs_font_refresh = false;
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