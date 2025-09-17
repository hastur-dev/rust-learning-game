use macroquad::prelude::*;
use crate::gamestate::{Game, RustFunction};
use crate::gamestate::types::EditorTab;
use crate::font_scaling::*;


pub fn draw_game_info(game: &Game) {
    let scale = ScaledMeasurements::new();
    let padding = scale.padding;
    
    let spec = &game.levels[game.level_idx];
    draw_scaled_text(
        &format!("{}  (Level {}/{})", spec.name, game.level_idx + 1, game.levels.len()),
        padding, padding + 0.0, 26.0, WHITE,
    );
    draw_scaled_text(
        &format!("Credits: {}   Turns: {}{}", game.credits, game.turns, if game.max_turns>0 { format!("/{}", game.max_turns) } else { "".into() }),
        padding, padding + scale.line_height, 22.0, WHITE,
    );
    let time_slow_status = if game.time_slow_active {
        format!(" | Time Slow: {}ms", game.time_slow_duration_ms)
    } else {
        "".to_string()
    };
    
    draw_scaled_text(
        &format!("Upgrades  Grabber range={}  |  Scanner len={}{}{}", 
                game.robot.upgrades.grabber_level, 
                game.robot.upgrades.scanner_level, 
                if game.robot.has_scanner() { " (owned)" } else { "" },
                time_slow_status),
        padding, padding + scale.line_height * 2.0, 20.0, WHITE,
    );
}

pub fn draw_controls_text() {
    let scale = ScaledMeasurements::new();
    let controls_text = "Controls: Click code editor to edit robot_code.rs | ENTER execute | Ctrl+Shift+C completion help | Ctrl+Shift+E IDE hint | Ctrl+Shift+B docs | Ctrl+Shift+S settings | Ctrl+Shift+N finish | Ctrl+Shift+L reload | Ctrl+Shift+M menu";
    draw_scaled_text(controls_text, scale.padding, screen_height() - scale_size(18.0), 18.0, GRAY);
}

fn get_function_definition(func: RustFunction) -> &'static str {
    match func {
        RustFunction::Move => r#"fn move_robot(direction: Direction) -> Result<String, String> {
    // Move robot in the specified direction
    // Returns Ok with status message or Err if blocked
}"#,
        RustFunction::Grab => r#"fn grab_items() -> String {
    // Grab all items and unknown tiles within grabber range
    // Returns status message with number of items grabbed
}"#,
        RustFunction::Scan => r#"fn scan_direction(direction: Direction) -> Result<String, String> {
    // Scan in a direction to reveal tiles (2-tile range)
    // Always available in the new design
}"#,
        RustFunction::LaserDirection => r#"fn laser_direction(direction: Direction) -> String {
    // Fire laser in specified direction until it hits something
    // Stuns enemies for 5 turns, destroys obstacles for 2 turns
}"#,
        RustFunction::LaserTile => r#"fn laser_tile(x: i32, y: i32) -> String {
    // Fire laser at specific coordinates
    // Stuns enemies for 5 turns, destroys obstacles for 2 turns
}"#,
        RustFunction::SkipLevel => r#"fn skip_this_level_because_i_say_so() -> String {
    // Skip to the next level
    // Secret command for testing and exploration
}"#,
        RustFunction::GotoLevel => r#"fn goto_this_level_because_i_say_so(level: usize) -> String {
    // Jump to a specific level number
    // Secret command for testing and exploration
}"#,
        RustFunction::OpenDoor => r#"fn open_door(open: bool) -> String {
    // Open or close a door at the robot's current position
    // Pass true to open, false to close
    // Teaches about boolean literals in Rust
}"#,
        // Print functions are available as standard Rust macros
        RustFunction::Println | RustFunction::Eprintln | RustFunction::Panic => {
            "Print functions are built-in Rust macros - use println!(), eprintln!(), panic!()"
        },
    }
}

pub fn draw_function_definitions(game: &mut Game) {
    let scale = ScaledMeasurements::new();
    let def_width = screen_width() * 0.25; // 1/4 of screen width
    let def_height = screen_height() * 0.6; // Take up more vertical space
    let def_x = screen_width() * 0.5 + scale.padding; // Position on left side of right half
    let def_y = scale.padding + scale_size(100.0);
    
    draw_rectangle(def_x - scale.padding, def_y - scale.padding, def_width + scale.padding * 2.0, def_height + scale.padding * 2.0, Color::new(0.0, 0.0, 0.0, 0.8));
    draw_rectangle_lines(def_x - scale.padding, def_y - scale.padding, def_width + scale.padding * 2.0, def_height + scale.padding * 2.0, scale_size(2.0), WHITE);

    // Always draw editor content (no tabs)
    draw_editor_content(game, def_x, def_y, def_width, def_height, &scale);
}


fn draw_commands_content(game: &Game, def_x: f32, def_y: f32, def_width: f32, def_height: f32, scale: &ScaledMeasurements) {
    draw_scaled_text("FUNCTION DEFINITIONS", def_x, def_y, 20.0, YELLOW);
    draw_scaled_text("Click a function name to view its implementation", def_x, def_y + scale.line_height, 12.0, GRAY);
    
    let available_functions = game.get_gui_functions();
    let mut y_offset = scale_size(50.0);
    
    for func in &available_functions {
        let button_y = def_y + y_offset;
        let button_color = if game.selected_function_to_view == Some(*func) { DARKBLUE } else { DARKGRAY };
        let text_color = if game.selected_function_to_view == Some(*func) { YELLOW } else { WHITE };
        
        let button_width = def_width - scale.padding * 2.0; // Use available width minus padding
        draw_rectangle(def_x, button_y, button_width, scale.button_height, button_color);
        draw_rectangle_lines(def_x, button_y, button_width, scale.button_height, scale_size(1.0), WHITE);
        
        let func_name = match func {
            RustFunction::Move => "move_bot(\"direction\")",
            RustFunction::Grab => "grab()",
            RustFunction::Scan => "scan(direction)",
            RustFunction::LaserDirection => "laser::direction(dir)",
            RustFunction::LaserTile => "laser::tile(x,y)",
            RustFunction::OpenDoor => "open_door(true/false)",
            _ => continue, // Skip hidden functions
        };
        
        draw_scaled_text(func_name, def_x + scale.padding, button_y + scale_size(17.0), 16.0, text_color);
        y_offset += scale_size(30.0);
    }
    
    if let Some(func) = game.selected_function_to_view {
        let code_y = def_y + y_offset + scale.padding;
        let code_area_height = def_height - y_offset - scale.padding * 2.0;
        
        draw_rectangle(def_x, code_y, def_width, code_area_height, Color::new(0.05, 0.05, 0.1, 0.9));
        draw_rectangle_lines(def_x, code_y, def_width, code_area_height, scale_size(1.0), LIGHTGRAY);
        
        let definition = get_function_definition(func);
        let lines: Vec<&str> = definition.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            let line_y = code_y + scale.line_height + (i as f32 * scale_size(14.0));
            if line_y < code_y + code_area_height - scale.padding {
                let color = if line.trim().starts_with("//") {
                    Color::new(0.5, 0.7, 0.5, 1.0)
                } else if line.contains("fn ") || line.contains("let ") || line.contains("if ") || line.contains("for ") {
                    Color::new(0.8, 0.6, 1.0, 1.0)
                } else if line.contains('"') {
                    Color::new(1.0, 0.8, 0.6, 1.0)
                } else {
                    WHITE
                };
                
                draw_scaled_text(line, def_x + scale.padding, line_y, 12.0, color);
            }
        }
    } else {
        draw_scaled_text("Select a function above to view its implementation", def_x, def_y + y_offset + scale_size(30.0), 16.0, GRAY);
    }
}

fn draw_tasks_content(game: &Game, def_x: f32, def_y: f32, def_width: f32, def_height: f32, scale: &ScaledMeasurements) {
    if let Some(level_spec) = game.levels.get(game.level_idx) {
        if !level_spec.tasks.is_empty() {
            draw_scaled_text("CURRENT TASKS", def_x, def_y, 20.0, YELLOW);
            draw_scaled_text("Complete tasks in order to progress", def_x, def_y + scale.line_height, 12.0, GRAY);
            
            let mut y_offset = scale_size(50.0);
            
            for (i, task) in level_spec.tasks.iter().enumerate() {
                let task_y = def_y + y_offset;
                let task_color = if task.completed {
                    Color::new(0.0, 0.3, 0.0, 0.8) // Dark green for completed
                } else if i == 0 || level_spec.tasks.get(i-1).map_or(true, |prev| prev.completed) {
                    Color::new(0.2, 0.2, 0.4, 0.8) // Active task
                } else {
                    Color::new(0.1, 0.1, 0.1, 0.6) // Locked task
                };
                
                let text_color = if task.completed {
                    GREEN
                } else if i == 0 || level_spec.tasks.get(i-1).map_or(true, |prev| prev.completed) {
                    WHITE
                } else {
                    GRAY
                };
                
                let button_width = def_width - scale.padding * 2.0;
                let task_height = scale_size(60.0);
                
                draw_rectangle(def_x, task_y, button_width, task_height, task_color);
                draw_rectangle_lines(def_x, task_y, button_width, task_height, scale_size(1.0), 
                                   if task.completed { GREEN } else { WHITE });
                
                // Task status icon
                let status_icon = if task.completed { "✓" } else { "○" };
                draw_scaled_text(status_icon, def_x + scale.padding, task_y + scale_size(15.0), 16.0, 
                               if task.completed { GREEN } else { text_color });
                
                // Task name
                let task_title = format!("{}. {}", i + 1, task.name);
                draw_scaled_text(&task_title, def_x + scale.padding + scale_size(25.0), task_y + scale_size(15.0), 
                               14.0, text_color);
                
                // Task description preview
                if let Some(message) = &task.task_message {
                    let preview = message.lines().next().unwrap_or("").chars().take(50).collect::<String>();
                    let preview = if message.len() > 50 { format!("{}...", preview) } else { preview };
                    draw_scaled_text(&preview, def_x + scale.padding + scale_size(25.0), task_y + scale_size(35.0), 
                                   10.0, GRAY);
                }
                
                y_offset += task_height + scale_size(10.0);
                
                // Don't draw beyond the visible area
                if task_y + task_height > def_y + def_height {
                    break;
                }
            }
        } else {
            draw_scaled_text("NO TASKS", def_x, def_y, 20.0, YELLOW);
            draw_scaled_text("This level doesn't have specific tasks", def_x, def_y + scale.line_height, 12.0, GRAY);
        }
    } else {
        draw_scaled_text("LOADING...", def_x, def_y, 20.0, YELLOW);
    }
}

fn draw_editor_content(game: &mut Game, editor_x: f32, editor_y: f32, editor_width: f32, editor_height: f32, scale: &ScaledMeasurements) {
    // Draw editor title and info
    draw_scaled_text("ROBOT CODE EDITOR", editor_x, editor_y, 18.0, YELLOW);
    draw_scaled_text(&format!("File: {}", game.robot_code_path), editor_x, editor_y + scale.line_height, 11.0, LIGHTGRAY);
    
    if game.robot_code_modified {
        draw_scaled_text("File modified externally! Changes loaded.", editor_x, editor_y + scale_size(32.0), 11.0, YELLOW);
    } else {
        draw_scaled_text("Click to position cursor | Arrow keys navigate | Ctrl+Shift+Enter to run", editor_x, editor_y + scale_size(32.0), 10.0, GRAY);
    }
    
    // Text area setup - adjusted for sidebar size
    let input_y = editor_y + scale_size(50.0);
    let line_height = game.get_cached_line_height();
    let available_height = editor_height - scale_size(50.0);
    let max_visible_lines = ((available_height / line_height) as usize).max(10); // At least 10 lines
    let text_area_height = max_visible_lines as f32 * line_height;
    
    draw_rectangle(editor_x, input_y, editor_width, text_area_height, Color::new(0.05, 0.05, 0.05, 0.9));
    draw_rectangle_lines(editor_x, input_y, editor_width, text_area_height, scale_size(1.0), 
                        if game.code_editor_active { YELLOW } else { WHITE });
    
    // Show current code from game state
    let code_to_display = if game.current_code.is_empty() {
        "// Start typing your Rust code here...\n".to_string()
    } else {
        game.current_code.clone()
    };
    
    let lines: Vec<&str> = code_to_display.lines().collect();
    let start_line = game.code_scroll_offset;
    
    // Draw line numbers
    let line_number_width = scale_size(28.0); // Slightly smaller for sidebar
    draw_rectangle(editor_x, input_y, line_number_width, text_area_height, Color::new(0.15, 0.15, 0.2, 1.0));
    draw_line(editor_x + line_number_width, input_y, editor_x + line_number_width, input_y + text_area_height, scale_size(1.0), DARKGRAY);
    
    for i in 0..max_visible_lines {
        let line_num = start_line + i + 1;
        let y = input_y + scale_size(10.0) + (i as f32 * line_height);
        let color = if line_num <= lines.len() { DARKGRAY } else { Color::new(0.3, 0.3, 0.3, 1.0) };
        draw_scaled_text(&format!("{:2}", line_num), editor_x + scale_size(2.0), y, 10.0, color);
    }
    
    // Grid-based character rendering
    let text_x = editor_x + line_number_width + scale_size(4.0);
    let char_width = game.get_cached_char_width();
    let char_height = line_height;
    
    // Calculate grid dimensions for sidebar width
    let max_cols = ((editor_width - line_number_width - scale_size(15.0)) / char_width) as usize;
    let grid_start_x = text_x;
    let grid_start_y = input_y + scale_size(10.0);
    
    // Draw character grid
    for row in 0..max_visible_lines {
        let line_index = start_line + row;
        let grid_y = grid_start_y + (row as f32 * char_height);
        
        if line_index < lines.len() {
            let line = lines[line_index];
            let chars: Vec<char> = line.chars().collect();
            
            // Highlight current line if cursor is on it
            let cursor_line = get_cursor_line(game);
            if game.code_editor_active && line_index == cursor_line {
                draw_rectangle(grid_start_x, grid_y - scale_size(8.0), editor_width - line_number_width - scale_size(8.0), char_height, Color::new(0.2, 0.2, 0.3, 0.3));
            }
            
            // Draw characters
            for col in 0..max_cols {
                if col < chars.len() {
                    let grid_x = grid_start_x + (col as f32 * char_width);
                    let ch = chars[col];
                    let color = get_syntax_color(ch, col, line);
                    draw_scaled_text(&ch.to_string(), grid_x, grid_y, 11.0, color);
                }
            }
        }
    }
    
    // Show cursor if it's in the visible area and editor is active
    if game.code_editor_active {
        let cursor_line = get_cursor_line(game);
        let cursor_col = get_cursor_col(game);
        
        if cursor_line >= start_line && cursor_line < start_line + max_visible_lines {
            let visible_row = cursor_line - start_line;
            let cursor_x = grid_start_x + (cursor_col as f32 * char_width);
            let cursor_y = grid_start_y + (visible_row as f32 * char_height);
            
            // Draw blinking cursor
            let time = get_time() as f32;
            if (time * 2.0) % 2.0 < 1.0 {
                draw_line(cursor_x, cursor_y - scale_size(8.0), cursor_x, cursor_y + scale_size(4.0), scale_size(2.0), YELLOW);
            }
        }
    }
}

fn draw_logs_content(game: &Game, def_x: f32, def_y: f32, def_width: f32, def_height: f32, scale: &ScaledMeasurements) {
    draw_scaled_text("PROGRAM OUTPUT LOGS", def_x, def_y, 20.0, YELLOW);
    draw_scaled_text("Output from your Rust program execution", def_x, def_y + scale.line_height, 12.0, GRAY);
    
    let content_y = def_y + scale_size(50.0);
    let content_height = def_height - scale_size(70.0);
    let line_height = scale_size(16.0);
    
    // Draw log area background
    draw_rectangle(def_x, content_y, def_width, content_height, Color::new(0.05, 0.05, 0.05, 0.9));
    draw_rectangle_lines(def_x, content_y, def_width, content_height, scale_size(1.0), LIGHTGRAY);
    
    if game.println_outputs.is_empty() && game.error_outputs.is_empty() {
        draw_scaled_text("No program output yet.", def_x + scale.padding, content_y + scale_size(30.0), 14.0, GRAY);
        draw_scaled_text("Run your code to see println!() and", def_x + scale.padding, content_y + scale_size(50.0), 12.0, LIGHTGRAY);
        draw_scaled_text("eprintln!() output here.", def_x + scale.padding, content_y + scale_size(66.0), 12.0, LIGHTGRAY);
        return;
    }
    
    let mut y_position = content_y + scale.padding;
    let max_lines = ((content_height - scale.padding * 2.0) / line_height) as usize;
    let mut line_count = 0;
    
    // Show println outputs (regular output)
    if !game.println_outputs.is_empty() {
        draw_scaled_text("📝 Standard Output:", def_x + scale.padding, y_position, 14.0, GREEN);
        y_position += line_height;
        line_count += 1;
        
        for (i, output) in game.println_outputs.iter().enumerate() {
            if line_count >= max_lines { break; }
            
            let text = format!("{}. {}", i + 1, output);
            let wrapped_lines = wrap_log_text(&text, def_width - scale.padding * 2.0, 12.0);
            
            for wrapped_line in wrapped_lines {
                if line_count >= max_lines { break; }
                draw_scaled_text(&wrapped_line, def_x + scale.padding * 2.0, y_position, 12.0, WHITE);
                y_position += line_height;
                line_count += 1;
            }
        }
    }
    
    // Show error outputs (error messages)
    if !game.error_outputs.is_empty() {
        if line_count < max_lines {
            if !game.println_outputs.is_empty() {
                y_position += line_height / 2.0; // Add some spacing
                line_count += 1;
            }
            
            if line_count < max_lines {
                draw_scaled_text("🔴 Error Output:", def_x + scale.padding, y_position, 14.0, RED);
                y_position += line_height;
                line_count += 1;
                
                for (i, error) in game.error_outputs.iter().enumerate() {
                    if line_count >= max_lines { break; }
                    
                    let text = format!("{}. {}", i + 1, error);
                    let wrapped_lines = wrap_log_text(&text, def_width - scale.padding * 2.0, 12.0);
                    
                    for wrapped_line in wrapped_lines {
                        if line_count >= max_lines { break; }
                        draw_scaled_text(&wrapped_line, def_x + scale.padding * 2.0, y_position, 12.0, Color::new(1.0, 0.8, 0.8, 1.0));
                        y_position += line_height;
                        line_count += 1;
                    }
                }
            }
        }
    }
    
    // Show if there are more logs than can be displayed
    if line_count >= max_lines {
        let remaining_println = game.println_outputs.len().saturating_sub(max_lines / 2);
        let remaining_error = game.error_outputs.len().saturating_sub(max_lines / 2);
        if remaining_println > 0 || remaining_error > 0 {
            draw_scaled_text("... (more logs available)", def_x + scale.padding, content_y + content_height - scale_size(30.0), 11.0, YELLOW);
        }
    }
}

fn wrap_log_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let scaled_font_size = scale_font_size(font_size);
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        
        let test_width = measure_text(&test_line, None, scaled_font_size as u16, 1.0).width;
        
        if test_width <= max_width {
            current_line = test_line;
        } else {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = word.to_string();
            } else {
                // Word is too long, just add it anyway
                lines.push(word.to_string());
            }
        }
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    lines
}

pub fn draw_tabbed_sidebar(game: &mut Game) {
    let scale = ScaledMeasurements::new();

    // Define sidebar position and dimensions (same as old function definitions area)
    let sidebar_x = screen_width() * 0.5 + scale.padding;
    let sidebar_y = scale.padding + scale_size(100.0);
    let sidebar_width = screen_width() * 0.25;
    let sidebar_height = screen_height() * 0.6;

    // Draw the main sidebar background
    draw_rectangle(sidebar_x - scale.padding, sidebar_y - scale.padding,
                   sidebar_width + scale.padding * 2.0, sidebar_height + scale.padding * 2.0,
                   Color::new(0.0, 0.0, 0.0, 0.8));
    draw_rectangle_lines(sidebar_x - scale.padding, sidebar_y - scale.padding,
                        sidebar_width + scale.padding * 2.0, sidebar_height + scale.padding * 2.0,
                        scale_size(2.0), WHITE);

    // Always draw editor content (no tabs)
    draw_editor_content(game, sidebar_x, sidebar_y, sidebar_width, sidebar_height, &scale);
}

// Removed draw_code_editor_standalone - now integrated into tabbed interface as Editor tab

// Helper functions for the editor
fn get_cursor_line(game: &Game) -> usize {
    let lines: Vec<&str> = game.current_code.lines().collect();
    let mut current_pos = 0;
    for (line_idx, line) in lines.iter().enumerate() {
        let line_len = line.len() + 1; // +1 for newline
        if current_pos + line_len > game.cursor_position {
            return line_idx;
        }
        current_pos += line_len;
    }
    lines.len().max(1) - 1
}

fn get_cursor_col(game: &Game) -> usize {
    let lines: Vec<&str> = game.current_code.lines().collect();
    let mut current_pos = 0;
    for line in lines.iter() {
        let line_len = line.len() + 1; // +1 for newline
        if current_pos + line_len > game.cursor_position {
            return game.cursor_position - current_pos;
        }
        current_pos += line_len;
    }
    0
}

fn get_syntax_color(ch: char, col: usize, line: &str) -> Color {
    // Simple syntax highlighting
    if line.trim_start().starts_with("//") {
        Color::new(0.5, 0.7, 0.5, 1.0) // Green for comments
    } else if line.contains("fn ") || line.contains("let ") || line.contains("if ") || line.contains("for ") {
        if col < line.len() {
            let word_start = line[..col].rfind(|c: char| c.is_whitespace()).map(|i| i + 1).unwrap_or(0);
            let word_end = line[col..].find(|c: char| c.is_whitespace()).map(|i| col + i).unwrap_or(line.len());
            let word = &line[word_start..word_end];
            if matches!(word, "fn" | "let" | "if" | "for" | "while" | "match" | "struct" | "impl") {
                Color::new(0.8, 0.6, 1.0, 1.0) // Purple for keywords
            } else {
                WHITE
            }
        } else {
            WHITE
        }
    } else if ch == '"' || (line.contains('"') && col >= line.find('"').unwrap_or(usize::MAX)) {
        Color::new(1.0, 0.8, 0.6, 1.0) // Orange for strings
    } else {
        WHITE
    }
}

pub fn handle_shop(_game: &mut Game) {
    // Shop functionality can be implemented here if needed in the future
    // Currently using this for potential future expansion
}