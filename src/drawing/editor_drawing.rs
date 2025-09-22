use macroquad::prelude::*;
use crate::gamestate::Game;
use crate::font_scaling::*;

/// Helper function to convert line/column position to absolute character position
fn get_absolute_position(line_index: usize, col: usize, lines: &[&str]) -> usize {
    let mut pos = 0;
    for i in 0..line_index.min(lines.len()) {
        pos += lines[i].len();
        // Only add newline if there's actually a next line (not at the end of file)
        if i < lines.len() - 1 {
            pos += 1; // Add newline character
        }
    }
    pos + col
}

/// Convert mouse coordinates to grid position (row, col)
pub fn mouse_to_grid_position(
    mouse_x: f32, 
    mouse_y: f32, 
    editor_bounds: (f32, f32, f32, f32),
    char_width: f32,
    line_height: f32,
    scroll_offset: usize
) -> Option<(usize, usize)> {
    use crate::font_scaling::scale_size;

    let (editor_x, editor_y, editor_width, _editor_height) = editor_bounds;
    let line_number_width = scale_size(28.0); // Match the actual line number width from drawing code
    let grid_start_x = editor_x + line_number_width + scale_size(4.0); // Match text_x calculation
    let grid_start_y = editor_y + scale_size(50.0) + scale_size(10.0); // Match exact drawing coordinates
    
    // Check if mouse is within the text area
    if mouse_x < grid_start_x || mouse_x > editor_x + editor_width - 20.0 {
        return None;
    }
    if mouse_y < grid_start_y || mouse_y > grid_start_y + (30.0 * line_height) {
        return None;
    }
    
    // Calculate grid position
    let col = ((mouse_x - grid_start_x) / char_width) as usize;
    let row = ((mouse_y - grid_start_y) / line_height) as usize;
    let actual_line = row + scroll_offset;

    // Debug logging to find the off-by-one issue
    debug!("mouse_to_grid_position: mouse=({:.2}, {:.2}), grid_start=({:.2}, {:.2}), char_width={:.2}, line_height={:.2}",
           mouse_x, mouse_y, grid_start_x, grid_start_y, char_width, line_height);
    debug!("Calculated: raw_row={}, raw_col={}, actual_line={}, scroll_offset={}",
           row, col, actual_line, scroll_offset);

    Some((actual_line, col))
}

/// Convert grid position to absolute cursor position in text
pub fn grid_to_cursor_position(row: usize, col: usize, lines: &[&str]) -> usize {
    if row >= lines.len() {
        // Position at end of last line
        let mut pos = 0;
        for line in lines {
            pos += line.len() + 1; // +1 for newline
        }
        return pos.saturating_sub(1); // Remove extra newline
    }
    
    let line = lines[row];
    let clamped_col = col.min(line.len());
    get_absolute_position(row, clamped_col, lines)
}

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
    
    // Grid-based character rendering - each character in its own cell
    let text_x = editor_x + line_number_width + scale_size(5.0);
    let char_width = game.get_cached_char_width();
    let char_height = line_height;
    
    // Calculate grid dimensions
    let max_cols = ((editor_width - line_number_width - scale_size(20.0)) / char_width) as usize;
    let grid_start_x = text_x;
    let grid_start_y = input_y + scale_size(12.0);
    
    // Draw character grid
    for row in 0..max_visible_lines {
        let line_index = start_line + row;
        let grid_y = grid_start_y + (row as f32 * char_height);
        
        if line_index < lines.len() {
            let line = lines[line_index];
            let chars: Vec<char> = line.chars().collect();
            
            // Draw each character in its own grid cell
            for col in 0..max_cols {
                let grid_x = grid_start_x + (col as f32 * char_width);
                let char_rect = macroquad::prelude::Rect {
                    x: grid_x,
                    y: grid_y - scale_size(10.0),
                    w: char_width,
                    h: char_height,
                };
                
                // Check if this position is selected
                let absolute_pos = get_absolute_position(line_index, col, &lines);

                // Enhanced selection detection - check raw selection values too
                let is_selected = if let Some((sel_start, sel_end)) = game.get_selection_bounds() {
                    let selected = absolute_pos >= sel_start && absolute_pos < sel_end;
                    if selected && line_index == 0 && col < 5 { // Debug first few chars
                        println!("🎨 Char at pos {} (line {}, col {}) is SELECTED (range: {}-{})",
                                absolute_pos, line_index, col, sel_start, sel_end);
                    }
                    selected
                } else {
                    // Also check raw selection state for debugging (less spam)
                    if line_index == 0 && col == 0 && game.selection_start.is_some() || game.selection_end.is_some() {
                        println!("🎨 No selection bounds but has raw values: start={:?}, end={:?}",
                                game.selection_start, game.selection_end);
                    }

                    // Fallback: check raw selection values for immediate visual feedback
                    if let (Some(start), Some(end)) = (game.selection_start, game.selection_end) {
                        let selected = absolute_pos >= start.min(end) && absolute_pos < start.max(end);
                        if selected && line_index == 0 && col < 5 {
                            println!("🎨 Raw selection: pos {} selected in range {}-{}", absolute_pos, start.min(end), start.max(end));
                        }
                        selected
                    } else {
                        false
                    }
                };
                
                // Draw selection background - make it more visible
                if is_selected {
                    draw_rectangle(char_rect.x, char_rect.y, char_rect.w, char_rect.h,
                                 Color::new(0.3, 0.5, 1.0, 0.8)); // Brighter blue with higher opacity
                }
                
                // Draw character if it exists
                if col < chars.len() {
                    let ch = chars[col];
                    let char_str = ch.to_string();
                    let text_color = if game.code_editor_active {
                        if is_selected { WHITE } else { WHITE }
                    } else {
                        LIGHTGRAY
                    };
                    
                    // Center the character in its cell
                    let char_x = char_rect.x + (char_rect.w - char_width) * 0.5;
                    let char_y = char_rect.y + char_rect.h - scale_size(3.0);
                    
                    draw_scaled_text(&char_str, char_x, char_y, 12.0, text_color);
                }
            }
        }
    }

    // Calculate cursor position for both cursor drawing and autocomplete
    let cursor_line = game.current_code[..game.cursor_position].matches('\n').count();
    let line_start = game.current_code[..game.cursor_position].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let cursor_col = game.cursor_position - line_start;

    // Draw cursor when active - now grid-based
    if game.code_editor_active {
        // Show cursor if it's in the visible area
        if cursor_line >= start_line && cursor_line < start_line + max_visible_lines {
            let visible_row = cursor_line - start_line;
            let cursor_x = grid_start_x + (cursor_col as f32 * char_width);
            let cursor_y = grid_start_y + (visible_row as f32 * char_height);

            // Draw blinking cursor as a vertical line in the grid cell
            let time = get_time() as f32;
            if (time * 2.0) % 2.0 < 1.0 { // Blink every 0.5 seconds
                draw_line(
                    cursor_x,
                    cursor_y - scale_size(10.0),
                    cursor_x,
                    cursor_y + char_height - scale_size(10.0),
                    scale_size(2.0),
                    YELLOW
                );
            }
        }
    }

    // Draw autocomplete suggestion
    draw_autocomplete_suggestion(game, cursor_line, cursor_col, start_line, max_visible_lines,
                                grid_start_x, grid_start_y, char_width, char_height);

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

fn draw_autocomplete_suggestion(
    game: &Game,
    cursor_line: usize,
    cursor_col: usize,
    start_line: usize,
    max_visible_lines: usize,
    grid_start_x: f32,
    grid_start_y: f32,
    char_width: f32,
    char_height: f32,
) {
    // Only draw if autocomplete is enabled and there's a suggestion
    if !game.autocomplete_enabled {
        return;
    }

    if let Some(suggestion) = game.get_autocomplete_suggestion() {
        // Check if cursor is in visible area
        if cursor_line >= start_line && cursor_line < start_line + max_visible_lines {
            let visible_row = cursor_line - start_line;

            // Get the current word at cursor to know what to replace
            let current_word = get_current_word_at_cursor_position(&game.current_code, game.cursor_position);

            // Calculate where to draw the completion (after current partial word)
            let completion_text = if suggestion.text.len() > current_word.len() {
                &suggestion.text[current_word.len()..]
            } else {
                ""
            };

            if !completion_text.is_empty() {
                let suggestion_x = grid_start_x + (cursor_col as f32 * char_width);
                let suggestion_y = grid_start_y + (visible_row as f32 * char_height);

                // Draw subtle background for the suggestion text (50% opacity)
                let text_width = completion_text.len() as f32 * char_width;
                draw_rectangle(
                    suggestion_x,
                    suggestion_y - char_height + scale_size(2.0),
                    text_width,
                    char_height,
                    Color::from_rgba(100, 100, 200, 50) // Light blue background with 50% opacity
                );

                // Draw each character of the completion with 50% opacity overlay
                let overlay_color = Color::from_rgba(150, 150, 255, 128); // Light blue with 50% opacity

                for (i, ch) in completion_text.chars().enumerate() {
                    let char_x = suggestion_x + (i as f32 * char_width);
                    let char_y = suggestion_y;

                    draw_scaled_text(&ch.to_string(), char_x, char_y, 12.0, overlay_color);
                }

                // Draw a small indicator showing the suggestion type
                let type_indicator = match suggestion.kind {
                    crate::autocomplete::SymbolKind::Function => "fn",
                    crate::autocomplete::SymbolKind::Struct => "struct",
                    crate::autocomplete::SymbolKind::Enum => "enum",
                    crate::autocomplete::SymbolKind::Variable => "var",
                    crate::autocomplete::SymbolKind::Keyword => "key",
                    crate::autocomplete::SymbolKind::Type => "type",
                };

                let indicator_x = suggestion_x + (completion_text.len() as f32 * char_width) + scale_size(5.0);
                let indicator_y = suggestion_y - scale_size(3.0);

                // Draw type indicator with 50% opacity to match the overlay theme
                draw_scaled_text(type_indicator, indicator_x, indicator_y, 8.0, Color::from_rgba(120, 120, 180, 128));
            }
        }
    }
}

fn get_current_word_at_cursor_position(code: &str, cursor_position: usize) -> String {
    let chars: Vec<char> = code.chars().collect();
    let mut start = cursor_position;

    // Find start of current word
    while start > 0 {
        let prev_char = chars[start - 1];
        if prev_char.is_alphanumeric() || prev_char == '_' {
            start -= 1;
        } else {
            break;
        }
    }

    if start < cursor_position {
        chars[start..cursor_position].iter().collect()
    } else {
        String::new()
    }
}


