use macroquad::prelude::*;
use crate::gamestate::{Game, RustFunction};

const PADDING: f32 = 16.0;

pub fn draw_game_info(game: &Game) {
    let spec = &game.levels[game.level_idx];
    draw_text(
        &format!("{}  (Level {}/{})", spec.name, game.level_idx + 1, game.levels.len()),
        PADDING, PADDING + 0.0, 26.0, WHITE,
    );
    draw_text(
        &format!("Credits: {}   Turns: {}{}", game.credits, game.turns, if game.max_turns>0 { format!("/{}", game.max_turns) } else { "".into() }),
        PADDING, PADDING + 24.0, 22.0, WHITE,
    );
    let time_slow_status = if game.time_slow_active {
        format!(" | Time Slow: {}ms", game.time_slow_duration_ms)
    } else {
        "".to_string()
    };
    
    draw_text(
        &format!("Upgrades  Grabber range={}  |  Scanner len={}{}{}", 
                game.robot.upgrades.grabber_level, 
                game.robot.upgrades.scanner_level, 
                if game.robot.has_scanner() { " (owned)" } else { "" },
                time_slow_status),
        PADDING, PADDING + 46.0, 20.0, WHITE,
    );
}

pub fn draw_controls_text() {
    let controls_text = "Controls: Click code editor to edit robot_code.rs | ENTER execute | Ctrl+Shift+C completion help | Ctrl+Shift+E IDE hint | Ctrl+Shift+B docs | Ctrl+Shift+N finish | Ctrl+Shift+L reload | Ctrl+Shift+M menu";
    draw_text(controls_text, PADDING, screen_height() - 18.0, 18.0, GRAY);
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
    let def_width = screen_width() * 0.25; // 1/4 of screen width
    let def_height = screen_height() * 0.6; // Take up more vertical space
    let def_x = screen_width() * 0.5 + PADDING; // Position on left side of right half
    let def_y = PADDING + 100.0;
    
    draw_rectangle(def_x - 10.0, def_y - 10.0, def_width + 20.0, def_height + 20.0, Color::new(0.0, 0.0, 0.0, 0.8));
    draw_rectangle_lines(def_x - 10.0, def_y - 10.0, def_width + 20.0, def_height + 20.0, 2.0, WHITE);
    
    draw_text("FUNCTION DEFINITIONS", def_x, def_y, 20.0, YELLOW);
    draw_text("Click a function name to view its implementation", def_x, def_y + 20.0, 12.0, GRAY);
    
    let available_functions = game.get_gui_functions();
    let mut y_offset = 50.0;
    
    for func in &available_functions {
        let button_y = def_y + y_offset;
        let button_color = if game.selected_function_to_view == Some(*func) { DARKBLUE } else { DARKGRAY };
        let text_color = if game.selected_function_to_view == Some(*func) { YELLOW } else { WHITE };
        
        let button_width = def_width - 20.0; // Use available width minus padding
        draw_rectangle(def_x, button_y, button_width, 25.0, button_color);
        draw_rectangle_lines(def_x, button_y, button_width, 25.0, 1.0, WHITE);
        
        let func_name = match func {
            RustFunction::Move => "move(direction)",
            RustFunction::Grab => "grab()",
            RustFunction::Scan => "scan(direction)",
            RustFunction::LaserDirection => "laser::direction(dir)",
            RustFunction::LaserTile => "laser::tile(x,y)",
            RustFunction::OpenDoor => "open_door(true/false)",
            _ => continue, // Skip hidden functions
        };
        
        draw_text(func_name, def_x + 10.0, button_y + 17.0, 16.0, text_color);
        y_offset += 30.0;
    }
    
    if let Some(func) = game.selected_function_to_view {
        let code_y = def_y + y_offset + 10.0;
        let code_area_height = def_height - y_offset - 20.0;
        
        draw_rectangle(def_x, code_y, def_width, code_area_height, Color::new(0.05, 0.05, 0.1, 0.9));
        draw_rectangle_lines(def_x, code_y, def_width, code_area_height, 1.0, LIGHTGRAY);
        
        let definition = get_function_definition(func);
        let lines: Vec<&str> = definition.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            let line_y = code_y + 20.0 + (i as f32 * 14.0);
            if line_y < code_y + code_area_height - 10.0 {
                let color = if line.trim().starts_with("//") {
                    Color::new(0.5, 0.7, 0.5, 1.0)
                } else if line.contains("fn ") || line.contains("let ") || line.contains("if ") || line.contains("for ") {
                    Color::new(0.8, 0.6, 1.0, 1.0)
                } else if line.contains('"') {
                    Color::new(1.0, 0.8, 0.6, 1.0)
                } else {
                    WHITE
                };
                
                draw_text(line, def_x + 10.0, line_y, 12.0, color);
            }
        }
    } else {
        draw_text("Select a function above to view its implementation", def_x, def_y + y_offset + 30.0, 16.0, GRAY);
    }
}

pub fn handle_shop(_game: &mut Game) {
    // Shop functionality can be implemented here if needed in the future
    // Currently using this for potential future expansion
}