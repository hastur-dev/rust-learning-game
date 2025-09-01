use macroquad::prelude::*;
use crate::gamestate::{Game, RustFunction};
use crate::gamestate::types::CommandsLogsTab;
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

pub fn draw_function_definitions(game: &Game) {
    let scale = ScaledMeasurements::new();
    let def_width = screen_width() * 0.25; // 1/4 of screen width
    let def_height = screen_height() * 0.6; // Take up more vertical space
    let def_x = screen_width() * 0.5 + scale.padding; // Position on left side of right half
    let def_y = scale.padding + scale_size(100.0);
    
    // Draw tabs before the main content
    let tab_height = scale_size(40.0);
    draw_commands_logs_tabs(game, def_x - scale.padding, def_y - scale.padding - tab_height, def_width + scale.padding * 2.0, tab_height);
    
    draw_rectangle(def_x - scale.padding, def_y - scale.padding, def_width + scale.padding * 2.0, def_height + scale.padding * 2.0, Color::new(0.0, 0.0, 0.0, 0.8));
    draw_rectangle_lines(def_x - scale.padding, def_y - scale.padding, def_width + scale.padding * 2.0, def_height + scale.padding * 2.0, scale_size(2.0), WHITE);
    
    match game.commands_logs_tab {
        CommandsLogsTab::Commands => {
            draw_commands_content(game, def_x, def_y, def_width, def_height, &scale);
        }
        CommandsLogsTab::Logs => {
            draw_logs_content(game, def_x, def_y, def_width, def_height, &scale);
        }
    }
}

fn draw_commands_logs_tabs(game: &Game, tab_x: f32, tab_y: f32, total_width: f32, tab_height: f32) {
    let tab_width = total_width / 2.0;
    
    // Commands Tab
    let commands_active = game.commands_logs_tab == CommandsLogsTab::Commands;
    let commands_color = if commands_active { Color::new(0.2, 0.2, 0.4, 0.9) } else { Color::new(0.1, 0.1, 0.1, 0.8) };
    let commands_text_color = if commands_active { YELLOW } else { LIGHTGRAY };
    
    draw_rectangle(tab_x, tab_y, tab_width, tab_height, commands_color);
    draw_rectangle_lines(tab_x, tab_y, tab_width, tab_height, scale_size(2.0), 
                        if commands_active { YELLOW } else { GRAY });
    draw_scaled_text("COMMANDS", tab_x + scale_size(10.0), tab_y + scale_size(25.0), 16.0, commands_text_color);
    
    // Logs Tab
    let logs_active = game.commands_logs_tab == CommandsLogsTab::Logs;
    let logs_color = if logs_active { Color::new(0.2, 0.2, 0.4, 0.9) } else { Color::new(0.1, 0.1, 0.1, 0.8) };
    let logs_text_color = if logs_active { YELLOW } else { LIGHTGRAY };
    
    draw_rectangle(tab_x + tab_width, tab_y, tab_width, tab_height, logs_color);
    draw_rectangle_lines(tab_x + tab_width, tab_y, tab_width, tab_height, scale_size(2.0), 
                        if logs_active { YELLOW } else { GRAY });
    
    // Show log count indicator
    let log_count = game.println_outputs.len() + game.error_outputs.len();
    let logs_text = if log_count > 0 {
        format!("LOGS ({})", log_count)
    } else {
        "LOGS".to_string()
    };
    draw_scaled_text(&logs_text, tab_x + tab_width + scale_size(10.0), tab_y + scale_size(25.0), 16.0, logs_text_color);
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

pub fn handle_shop(_game: &mut Game) {
    // Shop functionality can be implemented here if needed in the future
    // Currently using this for potential future expansion
}