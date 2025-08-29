use macroquad::prelude::*;
use ::rand::{rngs::StdRng, SeedableRng};
use std::collections::HashSet;

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

use level::*;
use item::*;
use gamestate::*;
use menu::{MenuAction, MenuState};
use popup::PopupAction;
use drawing::*;
use rust_checker::format_errors_for_display;

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
        
        // Look for println! statements
        if let Some(start) = trimmed.find("println!(") {
            let after_paren = &trimmed[start + 9..];
            if let Some(end) = after_paren.find(')') {
                let param = after_paren[..end].trim();
                let message = if param.starts_with('"') && param.ends_with('"') {
                    param[1..param.len()-1].to_string()
                } else {
                    param.replace("{}", "[value]").trim_matches('"').to_string()
                };
                print_outputs.push(format!("stdout: {}", message));
            }
        }
        
        // Look for eprintln! statements
        if let Some(start) = trimmed.find("eprintln!(") {
            let after_paren = &trimmed[start + 10..];
            if let Some(end) = after_paren.find(')') {
                let param = after_paren[..end].trim();
                let message = if param.starts_with('"') && param.ends_with('"') {
                    param[1..param.len()-1].to_string()
                } else {
                    param.replace("{}", "[value]").trim_matches('"').to_string()
                };
                print_outputs.push(format!("stderr: {}", message));
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
    // For tutorial level (level 0), provide detailed scan information
    if game.level_idx == 0 {
        let positions = game.robot.get_scanner_positions(dir, game.grid.width, game.grid.height);
        let mut obstacles = 0;
        let mut items = 0;
        let mut enemies = 0;
        let tiles_scanned = positions.len();
        
        for pos in &positions {
            if game.grid.is_blocked(*pos) {
                obstacles += 1;
            }
            if game.item_manager.get_item_at_position(*pos).is_some() {
                items += 1;
            }
            for enemy in &game.grid.enemies {
                if enemy.pos == *pos {
                    enemies += 1;
                    break;
                }
            }
            if game.grid.reveal(*pos) {
                game.discovered_this_level += 1;
            }
        }
        
        return format!("Scanned {} tiles, found {} obstacles, {} items, {} enemies", 
                      tiles_scanned, obstacles, items, enemies);
    }
    
    // Original scan function for other levels
    if !game.robot.has_scanner() {
        return "No scanner owned.".to_string();
    }
    
    let positions = game.robot.get_scanner_positions(dir, game.grid.width, game.grid.height);
    let mut revealed_any = false;
    
    for pos in positions {
        if game.grid.is_blocked(pos) {
            return "Unknown Object Blocking Function".to_string();
        }
        if game.grid.reveal(pos) {
            revealed_any = true;
            game.discovered_this_level += 1;
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

    if revealed_any { "Scan complete.".to_string() } else { "Scan found nothing.".to_string() }
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
        
        // Parse move() calls
        if let Some(start) = trimmed.find("move(") {
            let after_paren = &trimmed[start + 5..];
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
                try_scan(game, dir).to_string()
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
    for output in print_outputs {
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
    if calls.is_empty() {
        return "No valid function calls found".to_string();
    }
    
    let mut results = Vec::new();
    for call in calls {
        let result = execute_function(game, call);
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
    
    // Check tutorial progress after execution
    game.check_tutorial_progress();
    
    results.join("; ")
}


#[cfg(not(target_arch = "wasm32"))]
fn load_yaml_levels() -> Vec<LevelSpec> {
    let mut levels = Vec::new();
    let mut rng = StdRng::seed_from_u64(0xC0FFEE);
    
    // Try to load YAML levels from levels directory first
    let yaml_configs = load_yaml_levels_from_directory("levels");
    
    if !yaml_configs.is_empty() {
        // Use external YAML files if available
        for config in yaml_configs {
            if let Ok(level_spec) = config.to_level_spec(&mut rng) {
                levels.push(level_spec);
            }
        }
    } else {
        // Fallback to embedded levels if no external files found
        levels = embedded_levels::get_embedded_level_specs();
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

fn draw_main_game_view(game: &Game) {
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
    Conf {
        window_title: "Rust Robot Programming Game".to_owned(),
        window_width: 1920,
        window_height: 1080,
        window_resizable: true,
        fullscreen: false,
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

// Desktop-specific main logic
#[cfg(not(target_arch = "wasm32"))]
async fn desktop_main() {
    let rng = StdRng::seed_from_u64(0xC0FFEE);
    
    // Load levels - first try YAML, then fallback to built-in
    let levels = load_yaml_levels();
    
    let mut game = Game::new(levels, rng);
    
    // Initialize robot code
    game.load_robot_code();
    game.file_watcher_receiver = setup_file_watcher(&game.robot_code_path);
    
    let mut shop_open = false;

    loop {
        // Handle menu input and updates
        let menu_action = game.menu.handle_input();
        game.menu.update(menu_action.clone());

        // Handle menu actions
        match menu_action {
            MenuAction::StartGame => {
                game.load_level(0);
            },
            MenuAction::Exit => break,
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

                draw_main_game_view(&game);

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
                    
                    if is_mouse_button_pressed(MouseButton::Left) {
                        // Function definitions area
                        let def_x = screen_width() * 0.5 + 16.0; // Match PADDING constant
                        let def_y = 16.0 + 100.0; // Match PADDING constant
                        let def_width = screen_width() * 0.25;
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
                        
                        if mouse_x >= editor_x - 10.0 && mouse_x <= editor_x + editor_width + 10.0 &&
                           mouse_y >= editor_y - 10.0 && mouse_y <= editor_y + editor_height + 10.0 {
                            game.code_editor_active = true;
                            
                            // Position cursor at click location
                            let editor_bounds = (editor_x, editor_y, editor_width, editor_height);
                            game.position_cursor_at_click(mouse_x, mouse_y, editor_bounds);
                        } else if mouse_x > screen_width() / 2.0 {
                            game.code_editor_active = false;
                        }
                    }
                    
                    // Code editor input
                    if game.code_editor_active {
                        let mut code_modified = false;
                        
                        while let Some(character) = get_char_pressed() {
                            if character.is_ascii() && !character.is_control() {
                                game.current_code.insert(game.cursor_position, character);
                                game.cursor_position += 1;
                                code_modified = true;
                            }
                        }
                        
                        if is_key_pressed(KeyCode::Enter) {
                            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                                game.execution_result = execute_rust_code(&mut game).await;
                            } else {
                                game.current_code.insert(game.cursor_position, '\n');
                                game.cursor_position += 1;
                                code_modified = true;
                            }
                        }
                        
                        if is_key_pressed(KeyCode::Backspace) {
                            if game.cursor_position > 0 {
                                game.cursor_position -= 1;
                                game.current_code.remove(game.cursor_position);
                                code_modified = true;
                            }
                        }
                        
                        // Arrow key navigation
                        if is_key_pressed(KeyCode::Up) {
                            game.move_cursor_up();
                        }
                        if is_key_pressed(KeyCode::Down) {
                            game.move_cursor_down();
                        }
                        if is_key_pressed(KeyCode::Left) {
                            game.move_cursor_left();
                        }
                        if is_key_pressed(KeyCode::Right) {
                            game.move_cursor_right();
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
                } else {
                    if is_key_pressed(KeyCode::Escape) { shop_open = false; }
                }

                game.check_end_condition();
            },
            _ => {
                // Draw menu
                game.menu.draw();
            }
        }

        next_frame().await;
    }
}