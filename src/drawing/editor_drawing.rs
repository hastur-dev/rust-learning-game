use macroquad::prelude::*;
use crate::gamestate::Game;
use crate::font_scaling::*;

pub fn draw_code_editor(game: &mut Game) {
    let scale = ScaledMeasurements::new();
    let editor_width = screen_width() * 0.25; // Keep same width
    let editor_height = screen_height() * 0.6; // Keep same height  
    let editor_x = screen_width() - editor_width - scale.padding;
    let editor_y = scale.padding + scale_size(100.0);
    
    let bg_color = if game.code_editor_active { 
        Color::new(0.1, 0.1, 0.2, 0.9) 
    } else { 
        Color::new(0.0, 0.0, 0.0, 0.8) 
    };
    
    draw_rectangle(editor_x - scale.padding, editor_y - scale.padding, editor_width + scale.padding * 2.0, editor_height + scale.padding * 2.0, bg_color);
    draw_rectangle_lines(editor_x - scale.padding, editor_y - scale.padding, editor_width + scale.padding * 2.0, editor_height + scale.padding * 2.0, scale_size(2.0), 
                        if game.code_editor_active { YELLOW } else { WHITE });
    
    let title = "ROBOT CODE EDITOR";
    draw_scaled_text(title, editor_x, editor_y, 20.0, YELLOW);
    
    draw_scaled_text(&format!("File: {}", game.robot_code_path), editor_x, editor_y + scale.line_height, 12.0, LIGHTGRAY);
    if game.robot_code_modified {
        draw_scaled_text("File modified externally! Changes loaded.", editor_x, editor_y + scale_size(35.0), 12.0, YELLOW);
    } else {
        draw_scaled_text("Click to position cursor | Arrow keys navigate | Ctrl+Shift+Enter to run", editor_x, editor_y + scale_size(35.0), 11.0, GRAY);
    }
    
    // Text area setup - designed for 30 visible lines
    let input_y = editor_y + scale_size(55.0);
    let line_height = game.get_cached_line_height(); // Use cached line height
    let text_area_height = line_height * 30.0; // Exactly 30 lines
    // Use cached character width for consistent measurements
    let font_size = game.get_cached_font_size();
    let sample_char_width = game.get_cached_char_width();
    
    draw_rectangle(editor_x, input_y, editor_width, text_area_height, Color::new(0.05, 0.05, 0.05, 0.9));
    draw_rectangle_lines(editor_x, input_y, editor_width, text_area_height, scale_size(1.0), WHITE);
    
    // Show current code from game state
    let code_to_display = if game.current_code.is_empty() {
        "// Start typing your Rust code here...\n".to_string()
    } else {
        game.current_code.clone()
    };
    
    let lines: Vec<&str> = code_to_display.lines().collect();
    let max_visible_lines = 30; // Always show 30 lines
    let chars_per_line = ((editor_width - scale_size(60.0)) / sample_char_width) as usize; // Account for line numbers
    
    // Calculate which lines to show based on scroll offset
    let start_line = game.code_scroll_offset;
    let _end_line = (start_line + max_visible_lines).min(lines.len().max(max_visible_lines));
    
    // Draw line numbers
    let line_number_width = scale_size(35.0);
    draw_rectangle(editor_x, input_y, line_number_width, text_area_height, Color::new(0.15, 0.15, 0.2, 1.0));
    draw_line(editor_x + line_number_width, input_y, editor_x + line_number_width, input_y + text_area_height, scale_size(1.0), DARKGRAY);
    
    for i in 0..max_visible_lines {
        let line_num = start_line + i + 1;
        let y = input_y + scale_size(12.0) + (i as f32 * line_height);
        let color = if line_num <= lines.len() { DARKGRAY } else { Color::new(0.3, 0.3, 0.3, 1.0) };
        draw_scaled_text(&format!("{:2}", line_num), editor_x + scale_size(3.0), y, 11.0, color);
    }
    
    // Draw code text with selection highlighting
    let text_x = editor_x + line_number_width + scale_size(5.0);
    let font_size = game.get_cached_font_size();
    
    for i in 0..max_visible_lines {
        let line_index = start_line + i;
        let y = input_y + scale_size(12.0) + (i as f32 * line_height);
        
        if line_index < lines.len() {
            let line = lines[line_index];
            let display_line = if line.len() > chars_per_line {
                format!("{}...", &line[..chars_per_line.saturating_sub(3)])
            } else {
                line.to_string()
            };
            
            let text_color = if game.code_editor_active { WHITE } else { LIGHTGRAY };
            
            // Draw selection highlighting if there's a selection
            if let Some((sel_start, sel_end)) = game.get_selection_bounds() {
                // Calculate the absolute position of the start of this line
                let mut line_start_pos = 0;
                let code_lines: Vec<&str> = game.current_code.lines().collect();
                for i in 0..line_index.min(code_lines.len()) {
                    line_start_pos += code_lines[i].len();
                    if i < code_lines.len() - 1 || game.current_code.ends_with('\n') {
                        line_start_pos += 1; // Add newline character
                    }
                }
                
                let line_end_pos = line_start_pos + line.len();
                
                // Check if this line has any selection
                if sel_start < line_end_pos && sel_end > line_start_pos {
                    let line_sel_start = sel_start.saturating_sub(line_start_pos).min(line.len());
                    let line_sel_end = sel_end.saturating_sub(line_start_pos).min(line.len());
                    
                    if line_sel_start < line_sel_end {
                        // Draw text before selection
                        if line_sel_start > 0 {
                            let before_text = &line[..line_sel_start];
                            draw_scaled_text(before_text, text_x, y, 12.0, text_color);
                        }
                        
                        // Draw selection background
                        let before_width = if line_sel_start > 0 {
                            measure_text(&line[..line_sel_start], None, font_size as u16, 1.0).width
                        } else {
                            0.0
                        };
                        
                        let selected_text = &line[line_sel_start..line_sel_end];
                        let selection_width = measure_text(selected_text, None, font_size as u16, 1.0).width;
                        
                        // Draw selection background
                        draw_rectangle(
                            text_x + before_width,
                            y - scale_size(10.0),
                            selection_width,
                            line_height,
                            Color::new(0.2, 0.4, 0.8, 0.6) // Blue selection background
                        );
                        
                        // Draw selected text
                        draw_scaled_text(selected_text, text_x + before_width, y, 12.0, WHITE);
                        
                        // Draw text after selection
                        if line_sel_end < line.len() {
                            let after_text = &line[line_sel_end..];
                            let after_x = text_x + before_width + selection_width;
                            draw_scaled_text(after_text, after_x, y, 12.0, text_color);
                        }
                    } else {
                        // No selection on this line, draw normally
                        draw_scaled_text(&display_line, text_x, y, 12.0, text_color);
                    }
                } else {
                    // No selection on this line, draw normally
                    draw_scaled_text(&display_line, text_x, y, 12.0, text_color);
                }
            } else {
                // No selection at all, draw normally
                draw_scaled_text(&display_line, text_x, y, 12.0, text_color);
            }
        }
    }
    
    // Draw cursor when active
    if game.code_editor_active {
        let cursor_line = game.current_code[..game.cursor_position].matches('\n').count();
        let line_start = game.current_code[..game.cursor_position].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let cursor_col = game.cursor_position - line_start;
        
        // Show cursor if it's in the visible area
        if cursor_line >= start_line && cursor_line < start_line + max_visible_lines {
            // Use the new precise position calculation method
            let editor_bounds = (editor_x, editor_y, editor_width, editor_height);
            let (cursor_x, cursor_y) = game.get_text_position(cursor_line, cursor_col, editor_bounds);
            
            // Draw blinking cursor
            let time = get_time() as f32;
            if (time * 2.0) % 2.0 < 1.0 { // Blink every 0.5 seconds
                draw_line(cursor_x, cursor_y - scale_size(10.0), cursor_x, cursor_y + scale_size(2.0), scale_size(2.0), YELLOW);
            }
        }
    }
    
    // Draw scroll indicator if there are more lines than visible
    if lines.len() > max_visible_lines {
        let scroll_bar_x = editor_x + editor_width - 12.0;
        let scroll_bar_height = text_area_height - 4.0;
        let thumb_height = (max_visible_lines as f32 / lines.len() as f32 * scroll_bar_height).max(10.0);
        let thumb_y = input_y + 2.0 + (start_line as f32 / lines.len() as f32 * scroll_bar_height);
        
        // Scroll bar background
        draw_rectangle(scroll_bar_x, input_y + 2.0, 8.0, scroll_bar_height, Color::new(0.2, 0.2, 0.2, 0.8));
        // Scroll thumb
        draw_rectangle(scroll_bar_x, thumb_y, 8.0, thumb_height, Color::new(0.6, 0.6, 0.6, 0.9));
        draw_rectangle_lines(scroll_bar_x, thumb_y, 8.0, thumb_height, 1.0, GRAY);
    }
    
    // Draw buttons at the bottom
    draw_editor_buttons(editor_x, input_y + text_area_height + 10.0);
    
    // Draw execution results if any
    draw_execution_results(game, editor_x, input_y + text_area_height + 50.0);
}

fn draw_editor_buttons(editor_x: f32, button_y: f32) {
    let button_width = scale_size(140.0);
    let button_height = scale_size(30.0);
    let button_spacing = scale_size(150.0);
    
    draw_rectangle(editor_x, button_y, button_width, button_height, DARKGREEN);
    draw_rectangle_lines(editor_x, button_y, button_width, button_height, scale_size(1.0), WHITE);
    draw_scaled_text("[Ctrl+Shift+Enter] Run", editor_x + scale_size(5.0), button_y + scale_size(20.0), 12.0, WHITE);
    
    draw_rectangle(editor_x + button_spacing, button_y, scale_size(110.0), button_height, DARKBLUE);
    draw_rectangle_lines(editor_x + button_spacing, button_y, scale_size(110.0), button_height, scale_size(1.0), WHITE);
    draw_scaled_text("[Ctrl+E] IDE", editor_x + scale_size(140.0), button_y + scale_size(20.0), 12.0, WHITE);
    
    draw_rectangle(editor_x + scale_size(250.0), button_y, scale_size(110.0), button_height, Color::new(0.5, 0.1, 0.1, 1.0));
    draw_rectangle_lines(editor_x + scale_size(250.0), button_y, scale_size(110.0), button_height, scale_size(1.0), WHITE);
    draw_scaled_text("[Ctrl+R] Reset", editor_x + scale_size(255.0), button_y + scale_size(20.0), 11.0, WHITE);
}

fn draw_execution_results(game: &Game, editor_x: f32, result_y: f32) {
    if !game.execution_result.is_empty() {
        draw_scaled_text("EXECUTION RESULT:", editor_x, result_y, 14.0, WHITE);
        
        let max_chars_per_line = 35;
        let words: Vec<&str> = game.execution_result.split_whitespace().collect();
        let mut current_line = String::new();
        let mut line_count = 0;
        let line_height = scale_size(14.0);
        
        for word in words {
            if current_line.len() + word.len() + 1 > max_chars_per_line && line_count < 3 {
                draw_scaled_text(&current_line, editor_x, result_y + scale_size(16.0) + (line_count as f32 * line_height), 12.0, GREEN);
                current_line = word.to_string();
                line_count += 1;
            } else {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            }
        }
        
        if !current_line.is_empty() && line_count < 3 {
            draw_scaled_text(&current_line, editor_x, result_y + scale_size(16.0) + (line_count as f32 * line_height), 12.0, GREEN);
        }
    }
}