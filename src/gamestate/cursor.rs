use super::types::Game;
use macroquad::prelude::*;

impl Game {
    // Cursor and scrolling helper methods
    pub fn position_cursor_at_click(&mut self, click_x: f32, click_y: f32, editor_bounds: (f32, f32, f32, f32)) {
        let (editor_x, editor_y, _editor_width, _editor_height) = editor_bounds;
        let input_y = editor_y + 55.0;
        let line_height = 14.0;
        let line_number_width = 35.0;
        let text_x = editor_x + line_number_width + 5.0;
        
        // Check if click is within text area
        if click_x >= text_x && click_y >= input_y {
            let clicked_line_offset = ((click_y - input_y - 12.0) / line_height) as usize;
            
            let lines: Vec<&str> = self.current_code.lines().collect();
            let target_line = (self.code_scroll_offset + clicked_line_offset).min(lines.len());
            
            // Calculate the absolute position in the string
            let mut position = 0;
            
            // Add all characters from lines before the target line
            for i in 0..target_line.min(lines.len()) {
                position += lines[i].len();
                if i < lines.len() - 1 {
                    position += 1; // Add newline character for all but last line
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
                    // Use macroquad's measure_text to get accurate text width for 12pt font
                    let text_dimensions = measure_text(text_before, None, 12, 1.0);
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
}