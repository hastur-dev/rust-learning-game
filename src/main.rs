use macroquad::prelude::*;
use ::rand::{rngs::StdRng, SeedableRng};
use std::collections::HashSet;
use log::{info, warn, error, debug, trace};
use std::env;

// Desktop-only imports
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use notify::{Watcher, RecursiveMode, Event};
#[cfg(not(target_arch = "wasm32"))]
use crossbeam_channel::{Receiver, Sender, unbounded};
#[cfg(not(target_arch = "wasm32"))]
use std::process::Command;

mod level;
mod item;
mod grid;
mod robot;
mod gamestate;
mod menu;
mod movement_patterns;
mod popup;
mod embedded_levels;
mod drawing;
mod rust_checker;
mod font_scaling;
mod cache;
mod progressive_loader;
mod coordinate_system;

use level::*;
use item::*;
use gamestate::*;
use menu::{MenuAction, MenuState};
use popup::PopupAction;
use drawing::*;
use progressive_loader::{ProgressiveLoader, LoadingProgress, LoadingStage};

// Reset robot_code.rs to default content
fn reset_robot_code(game: &mut Game) {
    let default_code = r#"// Write your robot control code here
// Available functions:
// robot.move_up(), robot.move_down(), robot.move_left(), robot.move_right()
// robot.scan() -> returns what's at the robot's current position
// println!() for debugging output

fn main() {
    println!("Robot starting...");
    
    // Your code here
    
}
"#;
    
    game.current_code = default_code.to_string();
    game.cursor_position = default_code.len();
    
    // Save the default code to robot_code.rs file
    println!("Robot code reset to default");
    game.save_robot_code();
}
use rust_checker::format_errors_for_display;

// Helper function to parse println/eprintln messages with variable substitution
fn parse_println_message(param: &str, code: &str) -> String {
    // Simple string literal case
    if param.starts_with('"') && param.ends_with('"') {
        return param[1..param.len()-1].to_string();
    }
    
    // Format string with variables case (e.g., "Hello {}", name)
    if let Some(comma_pos) = param.find(',') {
        let format_part = param[..comma_pos].trim();
        let vars_part = param[comma_pos + 1..].trim();
        
        if format_part.starts_with('"') && format_part.ends_with('"') {
            let mut message = format_part[1..format_part.len()-1].to_string();
            
            // Extract variable names (simple parsing - works for basic cases)
            let variables: Vec<&str> = vars_part.split(',').map(|v| v.trim()).collect();
            
            // Try to resolve variable values from the code
            for var_name in variables {
                if let Some(value) = extract_variable_value(var_name, code) {
                    // Replace the first {} with the variable value
                    if let Some(pos) = message.find("{}") {
                        message.replace_range(pos..pos+2, &value);
                    }
                }
            }
            
            return message;
        }
    }
    
    // Fallback for complex cases
    param.replace("{}", "[value]").trim_matches('"').to_string()
}

// Helper function to extract variable values from code
fn extract_variable_value(var_name: &str, code: &str) -> Option<String> {
    for line in code.lines() {
        let trimmed = line.trim();
        
        // Look for variable assignments like: let var_name = "value";
        if let Some(let_pos) = trimmed.find("let") {
            let after_let = &trimmed[let_pos + 3..].trim();
            
            if after_let.starts_with(&format!("{} =", var_name)) ||
               after_let.starts_with(&format!("mut {} =", var_name)) {
                
                if let Some(eq_pos) = after_let.find('=') {
                    let value_part = after_let[eq_pos + 1..].trim();
                    
                    // Handle string literals
                    if value_part.starts_with('"') {
                        if let Some(end_quote) = value_part[1..].find('"') {
                            return Some(value_part[1..end_quote+1].to_string());
                        }
                    }
                    
                    // Handle simple values (numbers, etc.)
                    if let Some(semicolon) = value_part.find(';') {
                        return Some(value_part[..semicolon].trim().to_string());
                    } else {
                        return Some(value_part.to_string());
                    }
                }
            }
        }
    }
    
    None
}

// Desktop-only functions
#[cfg(not(target_arch = "wasm32"))]
fn extract_print_statements_from_rust_code(code: &str) -> Vec<String> {
    let mut print_outputs = Vec::new();
    
    // Create a temporary Rust file with the user code wrapped in a main function
    let _rust_code = format!(
        r#"
fn main() {{
    // User robot code wrapped to capture println! output
    {}
}}
"#, code
    );
    
    // For now, let's simulate the output by parsing println! statements
    // In the future, this could be enhanced to actually compile and run the code
    let lines: Vec<&str> = code.lines().collect();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        
        // Look for eprintln! statements first (before println! to avoid substring matching)
        if let Some(start) = trimmed.find("eprintln!(") {
            let after_paren = &trimmed[start + 10..];
            if let Some(end) = after_paren.find(')') {
                let param = after_paren[..end].trim();
                let message = parse_println_message(param, &code);
                debug!("Parsed eprintln message: '{}' from param: '{}'", message, param);
                print_outputs.push(format!("stderr: {}", message));
            }
        }
        // Look for println! statements (after eprintln! to avoid substring matching)
        else if let Some(start) = trimmed.find("println!(") {
            let after_paren = &trimmed[start + 9..];
            if let Some(end) = after_paren.find(')') {
                let param = after_paren[..end].trim();
                let message = parse_println_message(param, &code);
                debug!("Parsed println message: '{}' from param: '{}'", message, param);
                print_outputs.push(format!("stdout: {}", message));
            }
        }
        
        // Look for panic! statements  
        if let Some(start) = trimmed.find("panic!(") {
            let after_paren = &trimmed[start + 7..];
            if let Some(end) = after_paren.find(')') {
                let param = after_paren[..end].trim();
                let message = if param.starts_with('"') && param.ends_with('"') {
                    param[1..param.len()-1].to_string()
                } else {
                    param.replace("{}", "[value]").trim_matches('"').to_string()
                };
                print_outputs.push(format!("panic: {}", message));
            }
        }
    }
    
    print_outputs
}

fn old_extract_crates_from_code(code: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    let ignore: HashSet<&'static str> = [
        "std","core","alloc","crate","self","super",
        "macroquad","serde","serde_json","serde_yaml","rand","notify","crossbeam_channel",
    ].into_iter().collect();

    for raw in code.lines() {
        let line = raw.trim();
        if line.starts_with("use ") {
            let rest = &line[4..];
            let rest = rest.split(" as ").next().unwrap_or(rest);
            let rest = rest.trim_end_matches(';').trim();
            let first = rest.split(&[':', ' ', ',', '{'][..]).next().unwrap_or("").trim();
            if !first.is_empty() && !ignore.contains(first) && first.chars().all(|c| c.is_ascii_alphanumeric() || c=='_') {
                out.insert(first.to_string());
            }
        } else if line.starts_with("extern crate ") {
            let name = line["extern crate ".len()..].trim().trim_end_matches(';').trim();
            if !name.is_empty() && !ignore.contains(name) {
                out.insert(name.to_string());
            }
        }
    }
    out
}

#[cfg(not(target_arch = "wasm32"))]
fn existing_deps_from_cargo_toml(cargo_toml_path: &str) -> HashSet<String> {
    let mut deps = HashSet::new();
    let Ok(toml) = fs::read_to_string(cargo_toml_path) else { return deps; };

    let mut in_deps = false;
    for line in toml.lines() {
        let l = line.trim();
        if l.starts_with('[') {
            in_deps = l == "[dependencies]";
            continue;
        }
        if !in_deps || l.is_empty() || l.starts_with('#') { continue; }

        if let Some((key,_rest)) = l.split_once('=') {
            let k = key.trim();
            if !k.is_empty() && !k.starts_with('#') {
                deps.insert(k.to_string());
            }
        }
    }
    deps
}

#[cfg(not(target_arch = "wasm32"))]
fn cargo_add_available() -> bool {
    Command::new("cargo")
        .arg("add")
        .arg("--help")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(not(target_arch = "wasm32"))]
fn ensure_crates_in_cargo(new_crates: &HashSet<String>) -> String {
    if new_crates.is_empty() {
        return "No new libraries detected in robot_code.rs".to_string();
    }

    if !cargo_add_available() {
        return "cargo-edit not found. Install with: `cargo install cargo-edit`".to_string();
    }

    let mut added = Vec::new();
    let mut failed = Vec::new();

    for name in new_crates {
        let out = Command::new("cargo")
            .arg("add")
            .arg(name)
            .arg("--quiet")
            .output();

        match out {
            Ok(o) if o.status.success() => added.push(name.clone()),
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr);
                failed.push(format!("{name} ({})", stderr.trim()));
            }
            Err(e) => failed.push(format!("{name} ({e})")),
        }
    }

    if !failed.is_empty() {
        format!("Added: {:?}. Failed: {:?}", added, failed)
    } else {
        format!("Added: {:?}", added)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn auto_add_crates_from_robot_code(robot_code_path: &str) -> String {
    let Ok(code) = fs::read_to_string(robot_code_path) else {
        return format!("Could not read {}", robot_code_path);
    };

    let mentioned = old_extract_crates_from_code(&code);
    if mentioned.is_empty() {
        return "No external libraries referenced in robot_code.rs".to_string();
    }

    let deps = existing_deps_from_cargo_toml("Cargo.toml");
    let new_ones: HashSet<String> = mentioned
        .into_iter()
        .filter(|c| !deps.contains(c))
        .collect();

    ensure_crates_in_cargo(&new_ones)
}

// Game mechanics
fn try_move(game: &mut Game, dx: i32, dy: i32) {
    if game.finished { return; }
    
    let current_pos = game.robot.get_position();
    let next = Pos { x: current_pos.0 + dx, y: current_pos.1 + dy };
    
    if !game.grid.in_bounds(next) { return; }
    
    if game.grid.is_blocked(next) {
        game.grid.reveal_adjacent(current_pos);
        return;
    }
    
    // Move robot
    game.robot.move_to(next);
    game.grid.visit(next);
    game.grid.reveal_adjacent((next.x, next.y));

    // Check for immediate collision
    if game.level_idx >= 3 && game.grid.check_enemy_collision((next.x, next.y)) {
        let idx = game.level_idx;
        game.load_level(idx);
        game.execution_result = "ENEMY COLLISION! Level reset and randomized.".to_string();
        return;
    }

    // Enemies move after player action
    if game.level_idx >= 3 && !game.enemy_step_paused {
        game.update_laser_effects();
        game.grid.move_enemies(Some(game.robot.get_position()), &game.stunned_enemies);
        if game.grid.check_enemy_collision(game.robot.get_position()) {
            let idx = game.level_idx;
            game.load_level(idx);
            game.execution_result = "ENEMY COLLISION! Level reset and randomized.".to_string();
            return;
        }
    }

    // Always auto-grab behavior since grabber is always enabled
    try_grab(game);
}

fn try_grab(game: &mut Game) -> &'static str {
    let range = game.robot.get_grabber_range();
    let robot_pos = game.robot.get_pos();
    let mut grabbed = 0u32;
    let mut items_found = Vec::new();
    
    // Check for items within grab range
    let grabbable_positions = game.robot.get_grabber_positions(game.grid.width, game.grid.height);
    for pos in grabbable_positions {
        if let Some(item) = game.item_manager.collect_item(pos) {
            items_found.push(item.name.clone());
            
            // Show popup for item collection
            game.show_item_collected(&item.name);
            
            // Apply item effects
            match item.name.as_str() {
                "scanner" => {
                    game.robot.set_scanner_level(1);
                },
                "time_slow" => {
                    game.time_slow_active = true;
                    if let Some(duration) = item.capabilities.time_slow_duration {
                        game.time_slow_duration_ms = duration;
                    }
                },
                _ => {
                    if let Some(credits) = item.capabilities.credits_value {
                        game.credits += credits;
                    }
                    if let Some(grabber_boost) = item.capabilities.grabber_boost {
                        for _ in 0..grabber_boost {
                            game.robot.upgrade_grabber();
                        }
                    }
                    if let Some(duration) = item.capabilities.time_slow_duration {
                        game.time_slow_active = true;
                        game.time_slow_duration_ms = duration;
                    }
                }
            }
        }
    }
    
    // Grab unknown tiles for credits
    for y in (robot_pos.y - range).max(0)..=(robot_pos.y + range).min(game.grid.height - 1) {
        for x in (robot_pos.x - range).max(0)..=(robot_pos.x + range).min(game.grid.width - 1) {
            let pos = Pos { x, y };
            if game.robot.distance_to(pos) <= range && 
               game.grid.in_bounds(pos) && 
               !game.grid.known.contains(&pos) {
                if game.grid.reveal(pos) {
                    grabbed += 1;
                    game.discovered_this_level += 1;
                }
            }
        }
    }
    
    game.credits += grabbed * game.grid.income_per_square;

    // Enemies advance on any action
    if game.level_idx >= 3 && !game.enemy_step_paused {
        game.update_laser_effects();
        game.grid.move_enemies(Some(game.robot.get_position()), &game.stunned_enemies);
        if game.grid.check_enemy_collision(game.robot.get_position()) {
            let idx = game.level_idx;
            game.load_level(idx);
            game.execution_result = "ENEMY COLLISION! Level reset and randomized.".to_string();
        }
    }

    // Return appropriate message
    match (items_found.len() > 0, grabbed > 0) {
        (true, true) => "Grabbed items and unknown tiles for credits!",
        (true, false) => "Grabbed items!",
        (false, true) => "Grabbed unknown tiles for credits.",
        (false, false) => "Nothing to grab.",
    }
}

fn try_scan(game: &mut Game, dir: (i32, i32)) -> String {
    // For tutorial level (level 0), use detailed scanning with same reveal logic
    if game.level_idx == 0 {
        let robot_pos = game.robot.get_position();
        let mut tiles_revealed = 0;
        let mut obstacles = 0;
        let mut items = 0;
        let mut enemies = 0;
        let target_reveals = 5;
        
        // Scan in the specified direction, looking for unrevealed tiles
        let mut distance = 1;
        loop {
            let scan_pos = crate::item::Pos {
                x: robot_pos.0 + (dir.0 * distance),
                y: robot_pos.1 + (dir.1 * distance)
            };
            
            // Check if position is within grid bounds
            if !game.grid.in_bounds(scan_pos) {
                break;
            }
            
            // Check for obstacle - stop scanning if we hit one
            if game.grid.is_blocked(scan_pos) {
                obstacles += 1;
                break; // Stop scanning when we hit an obstacle
            }
            
            // Count items at this position
            if game.item_manager.get_item_at_position(scan_pos).is_some() {
                items += 1;
            }
            
            // Count enemies at this position
            for enemy in &game.grid.enemies {
                if enemy.pos == scan_pos {
                    enemies += 1;
                    break;
                }
            }
            
            // Try to reveal the tile - only count if it was previously unrevealed
            if game.grid.reveal(scan_pos) {
                game.discovered_this_level += 1;
                tiles_revealed += 1;
                
                // Stop when we've revealed our target number of tiles
                if tiles_revealed >= target_reveals {
                    break;
                }
            }
            
            distance += 1;
            
            // Safety check to avoid infinite loops
            if distance > 100 {
                break;
            }
        }
        
        return format!("Scanned and revealed {} new tiles, found {} obstacles, {} items, {} enemies", 
                      tiles_revealed, obstacles, items, enemies);
    }
    
    // Enhanced scan function for other levels - reveal 5 unrevealed tiles in direction
    if !game.robot.has_scanner() {
        return "No scanner owned.".to_string();
    }
    
    let robot_pos = game.robot.get_position();
    let mut tiles_revealed = 0;
    let target_reveals = 5;
    
    // Scan in the specified direction, looking for unrevealed tiles
    // Continue until we've revealed 5 tiles or hit an obstacle or boundary
    let mut distance = 1;
    loop {
        let scan_pos = crate::item::Pos {
            x: robot_pos.0 + (dir.0 * distance),
            y: robot_pos.1 + (dir.1 * distance)
        };
        
        // Check if position is within grid bounds
        if !game.grid.in_bounds(scan_pos) {
            break;
        }
        
        // Check for obstacle - stop scanning if we hit one
        if game.grid.is_blocked(scan_pos) {
            break; // Stop scanning when we hit an obstacle
        }
        
        // Try to reveal the tile - only count if it was previously unrevealed
        if game.grid.reveal(scan_pos) {
            game.discovered_this_level += 1;
            tiles_revealed += 1;
            
            // Stop when we've revealed our target number of tiles
            if tiles_revealed >= target_reveals {
                break;
            }
        }
        
        distance += 1;
        
        // Safety check to avoid infinite loops (shouldn't be needed but good practice)
        if distance > 100 {
            break;
        }
    }
    
    // Enemies advance on any action
    if game.level_idx >= 3 && !game.enemy_step_paused {
        game.update_laser_effects();
        game.grid.move_enemies(Some(game.robot.get_position()), &game.stunned_enemies);
        if game.grid.check_enemy_collision(game.robot.get_position()) {
            let idx = game.level_idx;
            game.load_level(idx);
            game.execution_result = "ENEMY COLLISION! Level reset and randomized.".to_string();
        }
    }

    if tiles_revealed > 0 { 
        format!("Scan complete. Revealed {} new tiles in that direction.", tiles_revealed) 
    } else { 
        "Scan complete. No new tiles to reveal in that direction.".to_string() 
    }
}

fn try_area_scan(game: &mut Game) -> String {
    // Area scan for Level 2 - scans current position + 8 surrounding tiles
    let robot_pos = game.robot.get_position();
    let mut items_found = Vec::new();
    let mut obstacles_found = Vec::new();
    let mut empty_count = 0;
    let mut walls_found = 0;
    let mut out_of_bounds = 0;
    
    // Define 3x3 area around robot (including center)
    let scan_offsets = [
        (-1, -1), (0, -1), (1, -1),  // Top row
        (-1,  0), (0,  0), (1,  0),  // Middle row (including current)
        (-1,  1), (0,  1), (1,  1),  // Bottom row
    ];
    
    for (dx, dy) in scan_offsets.iter() {
        let scan_pos = crate::item::Pos {
            x: robot_pos.0 + dx,
            y: robot_pos.1 + dy
        };
        
        // Check if position is within grid bounds
        if !game.grid.in_bounds(scan_pos) {
            out_of_bounds += 1;
            continue;
        }
        
        // Reveal this tile
        game.grid.reveal(scan_pos);
        
        // Check what's at this position
        if game.grid.is_blocked(scan_pos) {
            obstacles_found.push(format!("({}, {})", scan_pos.x, scan_pos.y));
            walls_found += 1;
        } else if let Some(item) = game.item_manager.get_item_at_position(scan_pos) {
            items_found.push(format!("{} at ({}, {})", item.name, scan_pos.x, scan_pos.y));
        } else {
            empty_count += 1;
        }
    }
    
    // Build result message based on what was found
    if !items_found.is_empty() {
        format!("Found items: {}. Empty tiles: {}. Walls: {}.", 
                items_found.join(", "), empty_count, walls_found)
    } else if walls_found > 0 {
        format!("Empty tiles: {}. Found {} walls/obstacles.", empty_count, walls_found)
    } else {
        format!("All {} accessible tiles are empty.", empty_count)
    }
}


// Code parsing and execution
fn parse_rust_code(code: &str) -> Vec<FunctionCall> {
    let mut calls = Vec::new();
    
    let lines: Vec<&str> = code.lines().collect();
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        
        // Parse move_bot() calls (also support legacy move() for backward compatibility)
        if let Some(start) = trimmed.find("move_bot(").or_else(|| trimmed.find("move(")) {
            let paren_offset = if trimmed[start..].starts_with("move_bot(") { 9 } else { 5 };
            let after_paren = &trimmed[start + paren_offset..];
            if let Some(end) = after_paren.find(')') {
                let param = after_paren[..end].trim();
                let dir = match param {
                    "up" | "Up" | "\"up\"" | "\"Up\"" => Some((0, -1)),
                    "down" | "Down" | "\"down\"" | "\"Down\"" => Some((0, 1)),
                    "left" | "Left" | "\"left\"" | "\"Left\"" => Some((-1, 0)),
                    "right" | "Right" | "\"right\"" | "\"Right\"" => Some((1, 0)),
                    _ => None,
                };
                if let Some(d) = dir {
                    calls.push(FunctionCall {
                        function: RustFunction::Move,
                        direction: Some(d),
                        coordinates: None,
                        level_number: None,
                        boolean_param: None,
                        message: None,
                    });
                }
            }
        }
        // Parse grab() calls
        else if trimmed.contains("grab()") {
            calls.push(FunctionCall {
                function: RustFunction::Grab,
                direction: None,
                coordinates: None,
                level_number: None,
                boolean_param: None,
                message: None,
            });
        }
        // Parse skip_this_level_because_i_say_so() calls
        else if trimmed.contains("skip_this_level_because_i_say_so()") {
            calls.push(FunctionCall {
                function: RustFunction::SkipLevel,
                direction: None,
                coordinates: None,
                level_number: None,
                boolean_param: None,
                message: None,
            });
        }
        // Parse scan() calls
        else if let Some(start) = trimmed.find("scan(") {
            let after_paren = &trimmed[start + 5..];
            if let Some(end) = after_paren.find(')') {
                let param = after_paren[..end].trim();
                
                let dir = match param {
                    "up" | "Up" | "\"up\"" | "\"Up\"" => Some((0, -1)),
                    "down" | "Down" | "\"down\"" | "\"Down\"" => Some((0, 1)),
                    "left" | "Left" | "\"left\"" | "\"Left\"" => Some((-1, 0)),
                    "right" | "Right" | "\"right\"" | "\"Right\"" => Some((1, 0)),
                    "current" | "Current" | "\"current\"" | "\"Current\"" => Some((0, 0)), // Special case for area scan
                    _ => None,
                };
                if let Some(d) = dir {
                    calls.push(FunctionCall {
                        function: RustFunction::Scan,
                        direction: Some(d),
                        coordinates: None,
                        level_number: None,
                        boolean_param: None,
                        message: None,
                    });
                }
            }
        }
        // Parse laser::direction() calls
        else if let Some(start) = trimmed.find("laser::direction(") {
            let after_paren = &trimmed[start + 16..];
            if let Some(end) = after_paren.find(')') {
                let param = after_paren[..end].trim();
                let dir = match param {
                    "up" | "Up" | "\"up\"" | "\"Up\"" => Some((0, -1)),
                    "down" | "Down" | "\"down\"" | "\"Down\"" => Some((0, 1)),
                    "left" | "Left" | "\"left\"" | "\"Left\"" => Some((-1, 0)),
                    "right" | "Right" | "\"right\"" | "\"Right\"" => Some((1, 0)),
                    _ => None,
                };
                if let Some(d) = dir {
                    calls.push(FunctionCall {
                        function: RustFunction::LaserDirection,
                        direction: Some(d),
                        coordinates: None,
                        level_number: None,
                        boolean_param: None,
                        message: None,
                    });
                }
            }
        }
        // Parse laser::tile() calls
        else if let Some(start) = trimmed.find("laser::tile(") {
            let after_paren = &trimmed[start + 12..];
            if let Some(end) = after_paren.find(')') {
                let param = after_paren[..end].trim();
                // Parse coordinates like (x,y) or x,y
                let coords_str = param.trim_matches(|c| c == '(' || c == ')');
                let parts: Vec<&str> = coords_str.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].trim().parse::<i32>(), parts[1].trim().parse::<i32>()) {
                        calls.push(FunctionCall {
                            function: RustFunction::LaserTile,
                            direction: None,
                            coordinates: Some((x, y)),
                            level_number: None,
                            boolean_param: None,
                            message: None,
                        });
                    }
                }
            }
        }
        // Parse goto_this_level_because_i_say_so() calls
        else if let Some(start) = trimmed.find("goto_this_level_because_i_say_so(") {
            let after_paren = &trimmed[start + 33..];
            if let Some(end) = after_paren.find(')') {
                let param = after_paren[..end].trim();
                if let Ok(level_num) = param.parse::<usize>() {
                    calls.push(FunctionCall {
                        function: RustFunction::GotoLevel,
                        direction: None,
                        coordinates: None,
                        level_number: Some(level_num),
                        boolean_param: None,
                        message: None,
                    });
                }
            }
        }
        // Parse open_door() calls
        else if let Some(start) = trimmed.find("open_door(") {
            let after_paren = &trimmed[start + 10..];
            if let Some(end) = after_paren.find(')') {
                let param = after_paren[..end].trim();
                let bool_param = match param {
                    "true" | "True" => Some(true),
                    "false" | "False" => Some(false),
                    _ => None,
                };
                if let Some(open_val) = bool_param {
                    calls.push(FunctionCall {
                        function: RustFunction::OpenDoor,
                        direction: None,
                        coordinates: None,
                        level_number: None,
                        boolean_param: Some(open_val),
                        message: None,
                    });
                }
            }
        }
    }
    
    calls
}

fn execute_function(game: &mut Game, call: FunctionCall) -> String {
    let available = game.get_available_functions();
    if !available.contains(&call.function) {
        return "Function not available".to_string();
    }
    
    match call.function {
        RustFunction::Move => {
            if let Some((dx, dy)) = call.direction {
                let old_pos = game.robot.get_position();
                try_move(game, dx, dy);
                game.turns += 1;
                if game.robot.get_position() != old_pos {
                    "Move executed".to_string()
                } else {
                    let target_pos = Pos { x: old_pos.0 + dx, y: old_pos.1 + dy };
                    if game.grid.is_blocked(target_pos) {
                        "Unknown Object Blocking Function".to_string()
                    } else {
                        "Move blocked".to_string()
                    }
                }
            } else {
                "Direction required for move".to_string()
            }
        },
        RustFunction::Grab => {
            try_grab(game).to_string()
        },
        RustFunction::Scan => {
            if let Some(dir) = call.direction {
                if dir == (0, 0) {
                    // Special case: scan("current") - scan 3x3 area around robot
                    try_area_scan(game)
                } else {
                    // Normal directional scan
                    try_scan(game, dir).to_string()
                }
            } else {
                "Direction required for scan".to_string()
            }
        },
        RustFunction::LaserDirection => {
            if let Some(dir) = call.direction {
                let result = game.fire_laser_direction(dir);
                game.turns += 1;
                // Move enemies after laser
                if game.level_idx >= 3 && !game.enemy_step_paused {
                    game.update_laser_effects();
                    game.grid.move_enemies(Some(game.robot.get_position()), &game.stunned_enemies);
                    if game.grid.check_enemy_collision(game.robot.get_position()) {
                        let idx = game.level_idx;
                        game.load_level(idx);
                        return "ENEMY COLLISION! Level reset and randomized.".to_string();
                    }
                }
                result
            } else {
                "Direction required for laser".to_string()
            }
        },
        RustFunction::LaserTile => {
            if let Some(coords) = call.coordinates {
                let result = game.fire_laser_tile(coords);
                game.turns += 1;
                // Move enemies after laser
                if game.level_idx >= 3 && !game.enemy_step_paused {
                    game.update_laser_effects();
                    game.grid.move_enemies(Some(game.robot.get_position()), &game.stunned_enemies);
                    if game.grid.check_enemy_collision(game.robot.get_position()) {
                        let idx = game.level_idx;
                        game.load_level(idx);
                        return "ENEMY COLLISION! Level reset and randomized.".to_string();
                    }
                }
                result
            } else {
                "Coordinates required for laser tile".to_string()
            }
        },
        RustFunction::SkipLevel => {
            game.skip_level()
        },
        RustFunction::GotoLevel => {
            if let Some(level) = call.level_number {
                game.goto_level(level)
            } else {
                "Level number required for goto_level".to_string()
            }
        },
        RustFunction::OpenDoor => {
            if let Some(open) = call.boolean_param {
                let result = game.open_door(open);
                game.turns += 1;
                // Move enemies after door action
                if game.level_idx >= 3 && !game.enemy_step_paused {
                    game.update_laser_effects();
                    game.grid.move_enemies(Some(game.robot.get_position()), &game.stunned_enemies);
                    if game.grid.check_enemy_collision(game.robot.get_position()) {
                        let idx = game.level_idx;
                        game.load_level(idx);
                        return "ENEMY COLLISION! Level reset and randomized.".to_string();
                    }
                }
                result
            } else {
                "Boolean parameter required for open_door (true or false)".to_string()
            }
        },
        // Print functions are handled separately in execute_rust_code
        RustFunction::Println | RustFunction::Eprintln | RustFunction::Panic => {
            "Print functions handled separately".to_string()
        },
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_external_code(file_path: &str) -> Result<String, String> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}

fn get_default_robot_code() -> &'static str {
    r#"// Welcome to the Rust Programming Tutorial!
// Follow the tasks shown above to learn Rust step by step.

// Task 1: Try your first print statement!
// Uncomment the line below and run your code:
// println!("Hello, Rust!");

"#
}

#[cfg(not(target_arch = "wasm32"))]
fn create_default_robot_code(file_path: &str) -> Result<(), String> {
    match fs::write(file_path, get_default_robot_code()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create robot_code.rs: {}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_robot_code(file_path: &str) -> Result<String, String> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(_) => {
            // File doesn't exist, create it with default content
            create_default_robot_code(file_path)?;
            Ok(get_default_robot_code().to_string())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn write_robot_code(file_path: &str, content: &str) -> Result<(), String> {
    match fs::write(file_path, content) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to save robot_code.rs: {}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn setup_file_watcher(file_path: &str) -> Option<Receiver<notify::Result<Event>>> {
    let (tx, rx): (Sender<notify::Result<Event>>, Receiver<notify::Result<Event>>) = unbounded();
    
    let mut watcher = match notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    }) {
        Ok(w) => w,
        Err(_) => return None,
    };
    
    if let Err(_) = watcher.watch(Path::new(file_path), RecursiveMode::NonRecursive) {
        return None;
    }
    
    std::mem::forget(watcher);
    Some(rx)
}

async fn execute_rust_code(game: &mut Game) -> String {
    let code_to_execute = if game.current_code.is_empty() {
        // Fallback to reading from file if current_code is empty
        match crate::read_robot_code(&game.robot_code_path) {
            Ok(code) => {
                game.current_code = code.clone();
                code
            },
            Err(e) => return e,
        }
    } else {
        game.current_code.clone()
    };
    
    // First, check syntax with Cargo (desktop only)
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(ref mut checker) = game.rust_checker {
            match checker.check_syntax(&code_to_execute) {
                Ok(errors) => {
                    // Format and display syntax check results
                    let syntax_result = format_errors_for_display(&errors);
                    
                    // If there are errors, show them and don't execute
                    let has_errors = errors.iter().any(|e| e.severity == rust_checker::ErrorSeverity::Error);
                    if has_errors {
                        return format!("🔍 SYNTAX CHECK:\n{}\n\nFix these errors before running your code!", syntax_result);
                    } else if !errors.is_empty() {
                        // Show warnings but continue execution
                        game.execution_result = format!("🔍 SYNTAX CHECK:\n{}\n\nCode executing with warnings...", syntax_result);
                    } else {
                        // Clean compilation, show success briefly
                        game.execution_result = "🔍 SYNTAX CHECK: ✅ Code compiled successfully!".to_string();
                    }
                },
                Err(e) => {
                    // If syntax checking fails, fall back to basic parsing but show the error
                    game.execution_result = format!("⚠️ Syntax checker unavailable: {}\nFalling back to basic code parsing...", e);
                }
            }
        }
    }
    
    // Extract and display print statements
    let print_outputs = extract_print_statements_from_rust_code(&code_to_execute);
    
    // Debug: Show extracted print outputs (commented out)
    // if game.level_idx == 0 {
    //     game.execution_result = format!("DEBUG: Extracted {} print outputs: {:?}", print_outputs.len(), print_outputs);
    // }
    
    for output in &print_outputs {
        if output.starts_with("stdout:") {
            let message = output.strip_prefix("stdout: ").unwrap_or("").to_string();
            game.popup_system.show_println_output(message.clone());
            game.println_outputs.push(message);
        } else if output.starts_with("stderr:") {
            let message = output.strip_prefix("stderr: ").unwrap_or("").to_string();
            game.popup_system.show_eprintln_output(message.clone());
            game.error_outputs.push(message);
        } else if output.starts_with("panic:") {
            let message = output.strip_prefix("panic: ").unwrap_or("").to_string();
            game.popup_system.show_panic_output(message.clone());
            game.panic_occurred = true;
            game.error_outputs.push(format!("panic: {}", message));
        }
    }
    
    let calls = parse_rust_code(&code_to_execute);
    if calls.is_empty() && print_outputs.is_empty() {
        return "No valid function calls found".to_string();
    }
    
    let mut results = Vec::new();
    
    // Handle robot function calls if any
    for call in &calls {
        let result = execute_function(game, call.clone());
        results.push(result.clone());
        
        // Add delay if time slow is active
        if game.time_slow_active {
            let frames_to_wait = (game.time_slow_duration_ms as f32 / 16.67).round() as i32; // Assuming ~60 FPS
            for _ in 0..frames_to_wait {
                next_frame().await;
            }
        }
        
        // Halt execution on blocking conditions or panic
        if result.contains("Unknown Object Blocking Function") || 
           result.contains("blocked by obstacle") || 
           result.contains("Search blocked") {
            results.push("EXECUTION HALTED! Rewrite your program to avoid obstacles.".to_string());
            break;
        } else if result.contains("💥 PANIC:") {
            // Panic halts all further execution
            results.push("EXECUTION HALTED! Program panicked.".to_string());
            break;
        }
    }
    
    // If we only had print statements (no robot function calls), provide feedback
    if calls.is_empty() && !print_outputs.is_empty() {
        results.push("Print statements executed successfully!".to_string());
    }
    
    // Show function results in popup if we have meaningful robot function calls
    if !calls.is_empty() {
        game.popup_system.show_function_results(results.clone());
    }
    
    // Check tutorial progress after execution
    game.check_tutorial_progress();
    
    // Check for level completion after execution
    game.check_end_condition();
    
    results.join("; ")
}


#[cfg(not(target_arch = "wasm32"))]
fn load_yaml_levels() -> Vec<LevelSpec> {
    let mut levels = Vec::new();
    let mut rng = StdRng::seed_from_u64(0xC0FFEE);
    
    // Always load embedded learning levels first
    let learning_configs = embedded_levels::get_embedded_learning_levels();
    for config in learning_configs {
        if let Ok(level_spec) = config.to_level_spec(&mut rng) {
            levels.push(level_spec);
        }
    }
    
    // Then try to load community levels from community_levels directory
    let community_configs = load_yaml_levels_from_directory("community_levels");
    for config in community_configs {
        if let Ok(level_spec) = config.to_level_spec(&mut rng) {
            levels.push(level_spec);
        }
    }
    
    levels
}

// Shop system
struct ShopItem {
    name: &'static str,
    cost: u32,
    apply: fn(&mut Game),
}

fn shop_items(game: &Game) -> Vec<ShopItem> {
    let mut v = vec![
        ShopItem {
            name: "Grabber +1 range",
            cost: 5 + game.robot.upgrades.grabber_level * 3,
            apply: |g| g.robot.upgrade_grabber(),
        }
    ];
    
    if game.level_idx < 3 {
        if !game.robot.has_scanner() {
            v.push(ShopItem {
                name: "Scanner (len 1)",
                cost: 8,
                apply: |g| { g.robot.set_scanner_level(1); },
            });
        } else {
            v.push(ShopItem {
                name: "Scanner +1 length",
                cost: 7 + game.robot.upgrades.scanner_level * 4,
                apply: |g| g.robot.upgrade_scanner(),
            });
        }
    }
    v
}

fn draw_main_game_view(game: &mut Game) {
    clear_background(Color::from_rgba(18, 18, 18, 255));
    draw_game(game);
    draw_game_info(game);
    draw_tutorial_overlay(game);
    draw_time_slow_indicator(game);
    draw_controls_text();
    draw_function_definitions(game);
    draw_code_editor(game);
    draw_level_complete_overlay(game);
}

#[cfg(not(target_arch = "wasm32"))]
fn window_conf() -> Conf {
    use crate::menu::GameSettings;
    
    let settings = GameSettings::load_or_default();
    
    Conf {
        window_title: "Rust Robot Programming Game".to_owned(),
        window_width: settings.window_width,
        window_height: settings.window_height,
        window_resizable: true,
        fullscreen: settings.fullscreen,
        ..Default::default()
    }
}

// Main function for desktop
#[cfg(not(target_arch = "wasm32"))]
#[macroquad::main(window_conf)]
async fn main() {
    desktop_main().await;
}

// Main function for WASM
#[cfg(target_arch = "wasm32")]
fn main() {
    // For WASM, macroquad handles the main function differently
    // The actual game logic is in lib.rs
}

// Helper function to cache game state when exiting
#[cfg(not(target_arch = "wasm32"))]
fn cache_game_state_on_exit(cache: &mut cache::GameCache, game: &Game) {
    use cache::{CachedGameSettings, StartupData};
    
    info!("Caching game state before exit...");
    
    // Cache current game settings
    let settings = CachedGameSettings {
        window_width: game.menu.settings.window_width,
        window_height: game.menu.settings.window_height,
        fullscreen: game.menu.settings.fullscreen,
        font_size_multiplier: game.menu.settings.font_size_multiplier,
        maximized: game.menu.settings.maximized,
        cached_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    };
    cache.cache_game_settings(settings);
    
    // Update startup data with current level
    let current_checksum = cache::GameCache::generate_embedded_levels_checksum();
    let startup_data = StartupData {
        last_played_level: game.level_idx,
        total_levels_count: game.levels.len(),
        embedded_levels_checksum: current_checksum,
        startup_time_ms: 0, // Will be updated on next startup
        cached_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    };
    cache.cache_startup_data(startup_data);
    
    // Save the cache
    cache.save();
    info!("Game state cached successfully");
}

// Helper function to update cached settings during runtime
#[cfg(not(target_arch = "wasm32"))]
fn update_cached_settings(cache: &mut cache::GameCache, settings: &menu::GameSettings) {
    use cache::CachedGameSettings;
    
    let cached_settings = CachedGameSettings {
        window_width: settings.window_width,
        window_height: settings.window_height,
        fullscreen: settings.fullscreen,
        font_size_multiplier: settings.font_size_multiplier,
        maximized: settings.maximized,
        cached_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    };
    cache.cache_game_settings(cached_settings);
    // Note: We don't save here to avoid frequent disk I/O, it will be saved on exit
}

// Test mode function for headless code execution
#[cfg(not(target_arch = "wasm32"))]
async fn run_test_mode(test_file: String, enable_all_logs: bool) {
    println!("=== RUST ROBOT PROGRAMMING GAME - TEST MODE ===");
    println!("Testing code from file: {}", test_file);
    
    // Read the test code from file
    let test_code = match std::fs::read_to_string(&test_file) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error reading test file '{}': {}", test_file, e);
            return;
        }
    };
    
    println!("\n--- Test Code ---");
    println!("{}", test_code);
    println!("--- End Test Code ---\n");
    
    // Initialize minimal game state for testing
    let rng = StdRng::seed_from_u64(TEST_SEED);
    let core_levels = embedded_levels::get_embedded_level_specs();
    let mut game = Game::new(core_levels, rng);
    game.enable_coordinate_logs = enable_all_logs;
    game.current_code = test_code.clone();
    
    // Load level 0 for testing
    game.load_level(0);
    
    println!("=== Executing Test Code ===");
    
    // Create a custom test execution function
    let execution_result = execute_test_code(&mut game, &test_code).await;
    
    println!("\n=== Execution Results ===");
    println!("Result: {}", execution_result);
    
    // Print any accumulated outputs
    if !game.println_outputs.is_empty() {
        println!("\n--- Print Outputs (println!) ---");
        for output in &game.println_outputs {
            println!("stdout: {}", output);
        }
    }
    
    if !game.error_outputs.is_empty() {
        println!("\n--- Error Outputs (eprintln!/panic!) ---");
        for output in &game.error_outputs {
            println!("stderr: {}", output);
        }
    }
    
    // Show what popups would have appeared
    println!("\n--- Message Popup Simulation ---");
    if let Some(popup) = &game.popup_system.current_popup {
        println!("Message Popup: {} - {}", popup.title, popup.content);
    } else {
        println!("No popups would be displayed");
    }
    
    // Show robot final position
    let final_pos = game.robot.get_position();
    println!("\n--- Robot Final State ---");
    println!("Position: ({}, {})", final_pos.0, final_pos.1);
    println!("Credits: {}", game.credits);
    println!("Turns taken: {}", game.turns);
    
    if game.finished {
        println!("Level completed!");
    } else {
        println!("Level not completed");
    }
    
    println!("\n=== Test Mode Complete ===");
}

// Custom test execution that simulates the popup system output
#[cfg(not(target_arch = "wasm32"))]
async fn execute_test_code(game: &mut Game, code: &str) -> String {
    // Extract and display print statements
    let print_outputs = extract_print_statements_from_rust_code(code);
    
    for output in &print_outputs {
        if output.starts_with("stdout:") {
            let message = output.strip_prefix("stdout: ").unwrap_or("").to_string();
            println!("Message Popup: 📝 Program Output - {}", message);
            game.println_outputs.push(message);
        } else if output.starts_with("stderr:") {
            let message = output.strip_prefix("stderr: ").unwrap_or("").to_string();
            println!("Message Popup: 🔴 Error Output - {}", message);
            game.error_outputs.push(message);
        } else if output.starts_with("panic:") {
            let message = output.strip_prefix("panic: ").unwrap_or("").to_string();
            println!("Message Popup: 💥 PANIC - Program terminated: {}", message);
            game.panic_occurred = true;
            game.error_outputs.push(format!("panic: {}", message));
        }
    }
    
    let calls = parse_rust_code(code);
    if calls.is_empty() && print_outputs.is_empty() {
        return "No valid function calls found".to_string();
    }
    
    let mut results = Vec::new();
    
    // Handle robot function calls
    for call in &calls {
        let result = execute_function(game, call.clone());
        results.push(result.clone());
        
        println!("Robot Action: {:?} -> {}", call.function, result);
        
        // Halt execution on blocking conditions or panic
        if result.contains("Unknown Object Blocking Function") || 
           result.contains("blocked by obstacle") || 
           result.contains("Search blocked") {
            results.push("EXECUTION HALTED! Rewrite your program to avoid obstacles.".to_string());
            break;
        } else if result.contains("💥 PANIC:") {
            results.push("EXECUTION HALTED! Program panicked.".to_string());
            break;
        }
    }
    
    // Show what function results popup would contain
    if !calls.is_empty() {
        let meaningful_results: Vec<String> = results
            .iter()
            .filter(|r| !r.is_empty() && 
                        !r.contains("executed") && 
                        !r.contains("Print functions handled separately") &&
                        r.as_str() != "No valid function calls found")
            .cloned()
            .collect();
        
        if !meaningful_results.is_empty() {
            println!("Message Popup: 🤖 Robot Action Results - {}", meaningful_results.join("\n"));
        }
    }
    
    // If we only had print statements (no robot function calls), provide feedback
    if calls.is_empty() && !print_outputs.is_empty() {
        results.push("Print statements executed successfully!".to_string());
    }
    
    // Check tutorial progress and level completion
    game.check_tutorial_progress();
    game.check_end_condition();
    
    results.join("; ")
}

const TEST_SEED: u64 = 0xDEADBEEF;

// Debug mode function to test all learning level solutions
#[cfg(not(target_arch = "wasm32"))]
async fn run_debug_all_levels(enable_all_logs: bool) {
    println!("=== RUST ROBOT PROGRAMMING GAME - DEBUG ALL LEVELS ===");
    
    let learning_configs = crate::gamestate::types::Game::get_learning_level_configs();
    
    println!("Found {} learning levels to test", learning_configs.len());
    println!();
    
    let mut total_tests = 0;
    let mut passed_tests = 0;
    
    for config in learning_configs {
        println!("🧪 Testing Level {}: {}", config.level_idx, config.name);
        println!("Expected {} tasks to complete", config.max_tasks);
        
        // Try to load and test the example solution for this level
        let level_file = format!("levels/{:02}_*.yaml", config.level_idx + 1);
        
        // For now, let's test with some basic solutions for the levels we know
        let test_results = match config.level_idx {
            0 => {
                // Level 1: Complete solution that satisfies all 5 tasks
                let solution = r#"
fn main() {
    // Task 1: println! output
    println!("Hello, Rust robot!");
    
    // Task 2: eprintln! output
    eprintln!("This is an error message for debugging");
    
    // Task 3: Variable used in print statement
    let my_message = "Variables are powerful!";
    println!("{}", my_message);
    
    // Task 4: Mutable variable with scan function
    let mut scan_result = scan("right");
    println!("Scan found: {}", scan_result);
    
    // Task 5: u32 integer used for movement
    let steps: u32 = 3;
    for _i in 0..steps {
        move_bot("right");
    }
    
    println!("Level 1 complete!");
}"#;
                test_level_solution(&config, solution, enable_all_logs).await
            },
            1 => {
                // Level 2: Complete solution that satisfies all 4 tasks
                let solution = r#"
// Task 3: Define struct above functions  
struct GridInfo {
    x: i32,
    y: i32,
    content: String,
}

// Task 1: Create function with print statement
fn scan_level() {
    println!("Beginning level scan...");
    
    // Task 3: Create vector for data collection
    let mut item_locations = Vec::new();
    
    // Task 2: Nested loops for grid scanning  
    for y in 0..6 {        // 6x6 grid height
        for x in 0..6 {    // 6x6 grid width
            // Movement and scanning code here
            let scan_result = scan("current");
            println!("Scanned ({}, {}): {}", x, y, scan_result);
            
            // Task 3: Using struct and collecting data
            if scan_result != "empty" && scan_result != "wall" {
                item_locations.push((x, y, scan_result.clone()));
                
                // Create GridInfo struct instance
                let grid_info = GridInfo {
                    x: x,
                    y: y, 
                    content: scan_result.clone(),
                };
            }
            
            // Task 4: Call the grab function
            grab_if_item(&scan_result);
        }
    }
    
    println!("Scanning complete! Found {} items.", item_locations.len());
}

// Task 4: Create grab function with conditional logic
fn grab_if_item(scan_result: &str) {
    if scan_result != "empty" && scan_result != "wall" && scan_result != "goal" {
        grab();
        println!("Grabbed: {}", scan_result);
    }
}

fn main() {
    println!("Starting Level 2 - Complete Test");
    // Task 1: Call scan_level function from main
    scan_level();
    println!("Level 2 complete test finished!");
}"#;
                test_level_solution(&config, solution, enable_all_logs).await
            },
            _ => {
                println!("  ⚠️  No test solution available for level {}", config.level_idx);
                (false, "No test solution available".to_string())
            }
        };
        
        total_tests += 1;
        if test_results.0 {
            passed_tests += 1;
            println!("  ✅ PASSED: Level {} completed successfully", config.level_idx);
        } else {
            println!("  ❌ FAILED: Level {} - {}", config.level_idx, test_results.1);
        }
        
        println!("  📊 Result: {}", test_results.1);
        println!();
    }
    
    println!("=== DEBUG TEST SUMMARY ===");
    println!("Total tests: {}", total_tests);
    println!("Passed: {}", passed_tests);
    println!("Failed: {}", total_tests - passed_tests);
    println!("Success rate: {:.1}%", (passed_tests as f32 / total_tests as f32) * 100.0);
    
    if passed_tests == total_tests {
        println!("🎉 All tests passed!");
    } else {
        println!("⚠️  Some tests failed - check output above for details");
    }
    
    println!("=== DEBUG ALL LEVELS COMPLETE ===");
}

// Test a solution against a specific learning level
#[cfg(not(target_arch = "wasm32"))]
async fn test_level_solution(config: &crate::gamestate::types::LearningLevelConfig, solution: &str, enable_all_logs: bool) -> (bool, String) {
    println!("  🔄 Testing solution for level {}...", config.level_idx);
    
    // Initialize game state for this level
    let rng = StdRng::seed_from_u64(TEST_SEED);
    let core_levels = embedded_levels::get_embedded_level_specs();
    
    if config.level_idx >= core_levels.len() {
        return (false, format!("Level {} not found in embedded levels", config.level_idx));
    }
    
    let mut game = Game::new(core_levels, rng);
    game.enable_coordinate_logs = enable_all_logs;
    game.current_code = solution.to_string();
    
    // Load the specific level
    game.load_level(config.level_idx);
    let initial_task_count = game.tutorial_state.task_completed.iter().filter(|&&x| x).count();
    
    // Execute the solution code
    println!("    ⚙️  Executing solution...");
    let execution_result = execute_test_code(&mut game, solution).await;
    
    // Manually trigger tutorial progress checking to ensure tasks are evaluated
    println!("    🔍 Checking tutorial progress...");
    for _ in 0..5 { // Check multiple times to allow all tasks to be evaluated
        game.check_tutorial_progress();
        if game.tutorial_state.current_task >= config.max_tasks {
            break;
        }
    }
    
    // Check if the level was completed
    let final_task_count = game.tutorial_state.task_completed.iter().filter(|&&x| x).count();
    let tasks_completed = final_task_count - initial_task_count;
    let level_complete = game.finished;
    
    println!("    📈 Tasks completed: {}/{}", tasks_completed, config.max_tasks);
    println!("    🏁 Level finished: {}", level_complete);
    
    // Determine if the test passed
    let passed = level_complete || tasks_completed >= config.max_tasks;
    let result_msg = if passed {
        format!("Completed {} tasks, level finished: {}", tasks_completed, level_complete)
    } else {
        format!("Only completed {} of {} tasks, level not finished", tasks_completed, config.max_tasks)
    };
    
    (passed, result_msg)
}

// Desktop-specific main logic
#[cfg(not(target_arch = "wasm32"))]
async fn desktop_main() {
    // Initialize fonts first
    font_scaling::initialize_fonts().await;
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let enable_all_logs = args.contains(&"--all-logs".to_string());
    let test_mode = args.iter().position(|arg| arg == "--test-code").map(|pos| {
        args.get(pos + 1).cloned()
    }).flatten();
    let debug_all_levels = args.contains(&"--debug".to_string());
    
    // Initialize logging with appropriate level based on command line args
    let log_level = if enable_all_logs {
        log::LevelFilter::Debug  // Show all debug logs when --all-logs is specified
    } else {
        log::LevelFilter::Info   // Only show info and higher by default
    };
    
    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .init();
    
    if enable_all_logs {
        info!("All logs enabled, including detailed coordinate tracking and debug messages");
    } else {
        info!("Normal logging mode - use --all-logs for detailed debug information");
    }
    
    // Check if we're in test mode
    if let Some(test_file) = test_mode {
        info!("Running in test mode with file: {}", test_file);
        run_test_mode(test_file, enable_all_logs).await;
        return;
    }
    
    // Check if we're in debug all levels mode
    if debug_all_levels {
        info!("Running debug mode - testing all learning levels");
        run_debug_all_levels(enable_all_logs).await;
        return;
    }
    
    info!("Starting Rust Steam Game...");
    
    let rng = StdRng::seed_from_u64(0xC0FFEE);
    
    // Initialize progressive loader
    let mut loader = ProgressiveLoader::new();
    
    // Check for cached startup data to potentially restore game state
    let cached_startup_data = loader.cache.get_startup_data();
    let cached_settings = loader.cache.get_cached_game_settings();
    
    // Start with minimal embedded levels for immediate play
    info!("Loading core embedded levels for immediate play...");
    let core_levels = embedded_levels::get_embedded_level_specs();
    info!("Loaded {} core levels", core_levels.len());
    
    let mut game = Game::new(core_levels.clone(), rng);
    
    // Enable coordinate logs if --all-logs flag is present
    game.enable_coordinate_logs = enable_all_logs;
    
    // Restore cached game settings if available
    if let Some(cached) = cached_settings {
        info!("Restoring cached game settings");
        game.menu.settings.window_width = cached.window_width;
        game.menu.settings.window_height = cached.window_height;
        game.menu.settings.fullscreen = cached.fullscreen;
        game.menu.settings.font_size_multiplier = cached.font_size_multiplier;
        game.menu.settings.maximized = cached.maximized;
        
        // Apply font scaling immediately
        font_scaling::set_user_font_multiplier(cached.font_size_multiplier);
    }
    
    info!("Game initialized successfully");
    
    // Set initial levels count in menu (use cached count if available)
    if let Some(startup_data) = cached_startup_data {
        game.menu.set_total_levels(startup_data.total_levels_count);
        info!("Using cached level count: {}", startup_data.total_levels_count);
    } else {
        game.menu.set_total_levels(core_levels.len());
    }
    
    // Start progressive loading in background
    loader.start_loading();
    
    // Initialize robot code
    game.load_robot_code();
    game.file_watcher_receiver = setup_file_watcher(&game.robot_code_path);
    
    // Apply saved maximize state on startup
    if game.menu.settings.maximized {
        info!("Restoring maximized window state");
        crate::coordinate_system::CoordinateTransformer::maximize_game_window();
    }
    
    let mut shop_open = false;
    let mut loading_progress: Option<LoadingProgress> = None;

    loop {
        // Check for progressive loading updates
        if let Some(progress) = loader.get_latest_progress() {
            // Clear loading progress once complete to stop checking
            if matches!(progress.stage, LoadingStage::Complete) && progress.progress >= 1.0 {
                loading_progress = None;
            } else {
                loading_progress = Some(progress.clone());
            }
            
            // Update game with newly loaded levels when available
            if let Some(new_levels) = loader.get_loaded_levels() {
                info!("Updating game with {} total levels", new_levels.len());
                game.levels = new_levels;
                game.menu.set_total_levels(game.levels.len());
            }
        }
        // Check for screen size changes and update menu layout if needed
        game.menu.check_screen_resize();
        
        // Check if user manually resized window and save the new size
        let current_width = screen_width() as i32;
        let current_height = screen_height() as i32;
        let current_maximized = crate::coordinate_system::CoordinateTransformer::is_game_window_maximized();
        
        // Track maximize state changes
        if current_maximized != game.menu.settings.maximized {
            game.menu.settings.maximized = current_maximized;
            let _ = game.menu.settings.save();
        }
        
        // Track window size changes (but don't save if maximized, as that's not the user's preferred size)
        if current_width != game.menu.settings.window_width || current_height != game.menu.settings.window_height {
            // Only save if this isn't a fullscreen change and window isn't maximized (to avoid saving weird dimensions)
            if !game.menu.settings.fullscreen && !current_maximized {
                game.menu.settings.window_width = current_width;
                game.menu.settings.window_height = current_height;
                let _ = game.menu.settings.save();
                
                // Also update the cache with new settings
                update_cached_settings(&mut loader.cache, &game.menu.settings);
            }
        }
        
        // Handle menu input and updates
        let menu_action = game.menu.handle_input();
        game.menu.update(menu_action.clone());
        
        // Update global font multiplier when settings change
        font_scaling::set_user_font_multiplier(game.menu.settings.font_size_multiplier);
        
        // Invalidate font cache to ensure cursor positioning updates
        game.invalidate_font_cache();

        // Handle menu actions
        match menu_action {
            MenuAction::StartGame => {
                println!("Starting new game...");
                // Reset to level 0 and clear robot code
                game.level_idx = 0;
                game.load_level(0);
                reset_robot_code(&mut game);
            },
            MenuAction::SelectLevel(level) => {
                println!("Loading level {}...", level);
                // Jump to selected level and reset robot code
                game.level_idx = level;
                game.load_level(level);
                reset_robot_code(&mut game);
            },
            MenuAction::Exit => {
                // Cache game settings and state before exit
                cache_game_state_on_exit(&mut loader.cache, &game);
                break;
            },
            _ => {}
        }

        // Draw based on current menu state
        match game.menu.state {
            MenuState::InGame => {
                // Handle popup input FIRST - before any other input processing
                let popup_action = game.handle_popup_input();
                let popup_handled_input = popup_action != PopupAction::None;
                
                // Update popup system with delta time
                game.update_popup_system(get_frame_time());

                draw_main_game_view(&mut game);

                // Shop functionality removed - replaced with Rust docs
                
                // Draw popups last so they appear on top
                game.draw_popups();

                // Game input handling
                if !shop_open && !popup_handled_input {
                    // Check for file changes
                    if let Some(ref receiver) = game.file_watcher_receiver {
                        if let Ok(_event) = receiver.try_recv() {
                            game.robot_code_modified = true;
                            game.load_robot_code(); // Reload file content
                        }
                    }
                    
                    // Mouse handling
                    let (mouse_x, mouse_y) = mouse_position();
                    trace!("Mouse position: ({:.2}, {:.2})", mouse_x, mouse_y);
                    
                    // Check for screenshot and system key combinations to prevent crashes
                    let system_key_combination = is_key_pressed(KeyCode::PrintScreen) || 
                        (is_key_down(KeyCode::LeftAlt) && is_key_pressed(KeyCode::PrintScreen)) ||
                        (is_key_down(KeyCode::LeftSuper) && is_key_down(KeyCode::LeftShift)) ||
                        (is_key_down(KeyCode::RightSuper) && is_key_down(KeyCode::LeftShift)) ||
                        (is_key_down(KeyCode::LeftSuper) && is_key_down(KeyCode::RightShift)) ||
                        (is_key_down(KeyCode::RightSuper) && is_key_down(KeyCode::RightShift)) ||
                        // Also check for Windows key + S combinations specifically
                        (is_key_down(KeyCode::LeftSuper) && is_key_pressed(KeyCode::S)) ||
                        (is_key_down(KeyCode::RightSuper) && is_key_pressed(KeyCode::S));
                    
                    // Update system key timing for extended safety period
                    let current_time = macroquad::prelude::get_time();
                    if system_key_combination {
                        game.last_system_key_time = current_time;
                        debug!("System key combination detected (screenshot/etc) - pausing coordinate updates");
                    }
                    
                    // Skip coordinate updates for 3 seconds after any system key combination
                    let time_since_system_keys = current_time - game.last_system_key_time;
                    let coordinate_safe_period = 3.0; // 3 second safety period
                    
                    // Update window coordinates for precise mouse tracking (skip during/after system key combinations)
                    if time_since_system_keys > coordinate_safe_period {
                        game.update_window_coordinates();
                    } else if system_key_combination {
                        debug!("Pausing coordinate updates for {:.1} seconds due to system key combination", coordinate_safe_period);
                    } else {
                        debug!("Still in system key safety period ({:.1}s remaining)", coordinate_safe_period - time_since_system_keys);
                    }
                    
                    if is_mouse_button_pressed(MouseButton::Left) {
                        debug!("Left mouse button pressed at ({:.2}, {:.2})", mouse_x, mouse_y);
                        
                        // Tab click handling (above function definitions area)
                        let def_x = screen_width() * 0.5 + 16.0; // Match PADDING constant
                        let def_y = 16.0 + 100.0; // Match PADDING constant
                        let def_width = screen_width() * 0.25;
                        let tab_height = 40.0;
                        let tab_y = def_y - 16.0 - tab_height; // Above the main area
                        let tab_width = (def_width + 32.0) / 2.0; // Split into two tabs
                        
                        // Commands tab click
                        if mouse_x >= def_x - 16.0 && mouse_x <= def_x - 16.0 + tab_width &&
                           mouse_y >= tab_y && mouse_y <= tab_y + tab_height {
                            game.commands_logs_tab = gamestate::types::CommandsLogsTab::Commands;
                            debug!("Switched to Commands tab");
                        }
                        // Logs tab click
                        else if mouse_x >= def_x - 16.0 + tab_width && mouse_x <= def_x - 16.0 + (def_width + 32.0) &&
                                mouse_y >= tab_y && mouse_y <= tab_y + tab_height {
                            game.commands_logs_tab = gamestate::types::CommandsLogsTab::Logs;
                            debug!("Switched to Logs tab");
                        }
                        
                        // Function definitions area
                        let available_functions = game.get_gui_functions();
                        
                        for (i, func) in available_functions.iter().enumerate() {
                            let button_y = def_y + 50.0 + (i as f32 * 30.0);
                            if mouse_x >= def_x && mouse_x <= def_x + def_width &&
                               mouse_y >= button_y && mouse_y <= button_y + 25.0 {
                                game.selected_function_to_view = Some(*func);
                            }
                        }
                        
                        // Editor click handling
                        let editor_x = screen_width() - (screen_width() * 0.25) - 16.0; // Match PADDING constant
                        let editor_y = 16.0 + 100.0; // Match PADDING constant
                        let editor_width = screen_width() * 0.25;
                        let editor_height = screen_height() * 0.6;
                        
                        debug!("Editor bounds: x={:.2}, y={:.2}, w={:.2}, h={:.2}", editor_x, editor_y, editor_width, editor_height);
                        
                        if mouse_x >= editor_x - 10.0 && mouse_x <= editor_x + editor_width + 10.0 &&
                           mouse_y >= editor_y - 10.0 && mouse_y <= editor_y + editor_height + 10.0 {
                            debug!("Click detected in editor area, activating editor");
                            game.code_editor_active = true;
                            
                            // Position cursor at click location
                            let editor_bounds = (editor_x, editor_y, editor_width, editor_height);
                            debug!("Calling position_cursor_at_click with bounds: {:?}", editor_bounds);
                            
                            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                game.position_cursor_at_click(mouse_x, mouse_y, editor_bounds);
                            })) {
                                Ok(_) => debug!("Cursor positioning completed successfully"),
                                Err(e) => {
                                    error!("Panic caught in position_cursor_at_click: {:?}", e);
                                    // Set safe defaults
                                    game.cursor_position = 0;
                                    if game.current_code.is_empty() {
                                        game.current_code = "// Start typing your Rust code here...\n".to_string();
                                    }
                                }
                            }
                        } else if mouse_x > screen_width() / 2.0 {
                            debug!("Click outside editor area, deactivating editor");
                            game.code_editor_active = false;
                        }
                    }
                    
                    // Code editor input
                    if game.code_editor_active {
                        let mut code_modified = false;
                        
                        // Update key press timers
                        game.update_key_press_timers(get_frame_time());
                        
                        // Handle character input - both initial press and continuous hold
                        let mut current_char_pressed = None;
                        while let Some(character) = get_char_pressed() {
                            if character.is_ascii() && !character.is_control() && character != ' ' {
                                current_char_pressed = Some(character);
                                
                                // Delete selection first if it exists
                                if game.delete_selection() {
                                    code_modified = true;
                                }
                                
                                game.current_code.insert(game.cursor_position, character);
                                game.cursor_position += 1;
                                code_modified = true;
                            }
                        }
                        
                        // Update character key timing
                        game.update_char_key_timing(current_char_pressed, get_frame_time());
                        
                        // Handle continuous character repeat
                        if game.should_repeat_char() {
                            if let Some(character) = game.last_char_pressed {
                                // Delete selection first if it exists
                                if game.delete_selection() {
                                    code_modified = true;
                                }
                                
                                game.current_code.insert(game.cursor_position, character);
                                game.cursor_position += 1;
                                code_modified = true;
                            }
                        }
                        
                        if is_key_pressed(KeyCode::Enter) {
                            if (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)) && 
                               (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) {
                                game.execution_result = execute_rust_code(&mut game).await;
                            } else {
                                // Delete selection first if it exists
                                if game.delete_selection() {
                                    code_modified = true;
                                }
                                
                                game.current_code.insert(game.cursor_position, '\n');
                                game.cursor_position += 1;
                                game.ensure_cursor_visible(); // Ensure the cursor scrolls into view after newline
                                code_modified = true;
                            }
                        }
                        
                        // Handle backspace - both initial press and continuous hold
                        if is_key_pressed(KeyCode::Backspace) || game.should_repeat_backspace() {
                            // Delete selection first if it exists, otherwise delete single character
                            if game.delete_selection() {
                                code_modified = true;
                            } else if game.cursor_position > 0 {
                                game.cursor_position -= 1;
                                game.current_code.remove(game.cursor_position);
                                code_modified = true;
                            }
                        }
                        
                        // Handle space - both initial press and continuous hold
                        if is_key_pressed(KeyCode::Space) || game.should_repeat_space() {
                            // Delete selection first if it exists
                            if game.delete_selection() {
                                code_modified = true;
                            }
                            
                            game.current_code.insert(game.cursor_position, ' ');
                            game.cursor_position += 1;
                            code_modified = true;
                        }
                        
                        // Handle tab key - insert 4 spaces for indentation
                        if is_key_pressed(KeyCode::Tab) {
                            // Delete selection first if it exists
                            if game.delete_selection() {
                                code_modified = true;
                            }
                            
                            // Insert 4 spaces for tab
                            let tab_spaces = "    "; // 4 spaces
                            for (i, space) in tab_spaces.chars().enumerate() {
                                game.current_code.insert(game.cursor_position + i, space);
                            }
                            game.cursor_position += tab_spaces.len();
                            code_modified = true;
                        }
                        
                        // Arrow key navigation with selection support
                        let shift_held = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
                        
                        if is_key_pressed(KeyCode::Up) || game.should_repeat_up() {
                            game.move_cursor_up_with_selection(shift_held);
                        }
                        if is_key_pressed(KeyCode::Down) || game.should_repeat_down() {
                            game.move_cursor_down_with_selection(shift_held);
                        }
                        if is_key_pressed(KeyCode::Left) || game.should_repeat_left() {
                            game.move_cursor_left_with_selection(shift_held);
                        }
                        if is_key_pressed(KeyCode::Right) || game.should_repeat_right() {
                            game.move_cursor_right_with_selection(shift_held);
                        }
                        
                        // Page Up/Down for scrolling
                        if is_key_pressed(KeyCode::PageUp) {
                            for _ in 0..10 {
                                game.scroll_up();
                            }
                        }
                        if is_key_pressed(KeyCode::PageDown) {
                            for _ in 0..10 {
                                game.scroll_down();
                            }
                        }
                        
                        if is_key_pressed(KeyCode::R) && is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) {
                            // Reset to default code
                            game.current_code = get_default_robot_code().to_string();
                            game.cursor_position = 0;
                            game.code_scroll_offset = 0;
                            code_modified = true;
                        }
                        
                        // Auto-save on any modification
                        if code_modified {
                            game.save_robot_code();
                        }
                    }
                    
                    if is_key_pressed(KeyCode::E) && is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) && !game.code_editor_active {
                        // Open external editor hint
                        game.execution_result = format!("Edit {} with your preferred IDE/editor", game.robot_code_path);
                    }

                    if is_key_pressed(KeyCode::B) && is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) { 
                        // Open Rust documentation for current level
                        game.execution_result = game.open_rust_docs();
                    }
                    if is_key_pressed(KeyCode::N) && is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) {
                        if !game.finished { game.finish_level(); }
                        game.next_level();
                        reset_robot_code(&mut game); // Reset robot code for next level
                    }
                    if is_key_pressed(KeyCode::L) && is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) {
                        let idx = game.level_idx;
                        game.load_level(idx);
                        game.execution_result.clear();
                    }
                    if is_key_pressed(KeyCode::M) && is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) {
                        // Return to main menu
                        game.menu.state = MenuState::MainMenu;
                        game.menu.setup_main_menu();
                        shop_open = false;
                    }
                    if is_key_pressed(KeyCode::C) && is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) {
                        // Show completion instructions
                        game.show_completion_instructions();
                    }
                    if is_key_pressed(KeyCode::S) && is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) {
                        // Open settings menu from in-game
                        game.menu.open_settings_from_game();
                    }
                } else {
                    if is_key_pressed(KeyCode::Escape) { shop_open = false; }
                }

                game.check_end_condition();
            },
            _ => {
                // Draw menu with loading progress
                game.menu.draw_with_loading_progress(loading_progress.as_ref());
            }
        }

        next_frame().await;
    }
}