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
mod game_state;
mod menu;
mod movement_patterns;
mod popup;

use level::*;
use item::*;
use game_state::*;
use menu::{MenuAction, MenuState};

// Desktop-only functions
#[cfg(not(target_arch = "wasm32"))]
fn extract_crates_from_code(code: &str) -> HashSet<String> {
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

    let mentioned = extract_crates_from_code(&code);
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
        game.grid.move_enemies();
        if game.grid.check_enemy_collision(game.robot.get_position()) {
            let idx = game.level_idx;
            game.load_level(idx);
            game.execution_result = "ENEMY COLLISION! Level reset and randomized.".to_string();
            return;
        }
    }

    // Auto-grab behavior
    if game.robot.auto_grab_enabled {
        let range = game.robot.get_grabber_range();
        let robot_pos = game.robot.get_pos();
        let mut has_grabbable = false;
        
        for y in (robot_pos.y - range).max(0)..=(robot_pos.y + range).min(game.grid.height - 1) {
            for x in (robot_pos.x - range).max(0)..=(robot_pos.x + range).min(game.grid.width - 1) {
                let pos = Pos { x, y };
                if game.robot.distance_to(pos) <= range && 
                   game.grid.in_bounds(pos) && 
                   !game.grid.known.contains(&pos) {
                    has_grabbable = true;
                    break;
                }
            }
            if has_grabbable { break; }
        }
        
        if has_grabbable { 
            try_grab(game); 
        }
    }
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
        game.grid.move_enemies();
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

fn try_scan(game: &mut Game, dir: (i32, i32)) -> &'static str {
    if !game.robot.has_scanner() {
        return "No scanner owned.";
    }
    
    let positions = game.robot.get_scanner_positions(dir, game.grid.width, game.grid.height);
    let mut revealed_any = false;
    
    for pos in positions {
        if game.grid.is_blocked(pos) {
            return "Unknown Object Blocking Function";
        }
        if game.grid.reveal(pos) {
            revealed_any = true;
            game.discovered_this_level += 1;
        }
    }
    
    // Enemies advance on any action
    if game.level_idx >= 3 && !game.enemy_step_paused {
        game.grid.move_enemies();
        if game.grid.check_enemy_collision(game.robot.get_position()) {
            let idx = game.level_idx;
            game.load_level(idx);
            game.execution_result = "ENEMY COLLISION! Level reset and randomized.".to_string();
        }
    }

    if revealed_any { "Scan complete." } else { "Scan found nothing." }
}

fn try_search_all(game: &mut Game) -> String {
    let mut discovered = 0;
    let mut moves_made = 0;
    let mut going_right = true;
    
    game.enemy_step_paused = true;

    let _robot_pos = game.robot.get_position();
    
    // Try to move to top-left corner first
    while game.robot.get_position().1 > 0 {
        let current_pos = game.robot.get_position();
        let next = Pos { x: current_pos.0, y: current_pos.1 - 1 };
        if game.grid.is_blocked(next) {
            game.enemy_step_paused = false;
            return "Search blocked by obstacle - cannot reach starting position".to_string();
        }
        try_move(game, 0, -1);
        moves_made += 1;
        if moves_made > 100 { break; }
    }
    
    while game.robot.get_position().0 > 0 {
        let current_pos = game.robot.get_position();
        let next = Pos { x: current_pos.0 - 1, y: current_pos.1 };
        if game.grid.is_blocked(next) {
            game.enemy_step_paused = false;
            return "Search blocked by obstacle - cannot reach starting position".to_string();
        }
        try_move(game, -1, 0);
        moves_made += 1;
        if moves_made > 100 { break; }
    }
    
    // Now do lawnmower pattern
    let max_moves = 200;
    
    while moves_made < max_moves {
        let current_pos = game.robot.get_pos();
        if !game.grid.known.contains(&current_pos) {
            discovered += 1;
        }
        
        if going_right {
            let next = Pos { x: current_pos.x + 1, y: current_pos.y };
            if game.grid.in_bounds(next) && !game.grid.is_blocked(next) {
                try_move(game, 1, 0);
                moves_made += 1;
            } else {
                let down = Pos { x: current_pos.x, y: current_pos.y + 1 };
                if game.grid.in_bounds(down) && !game.grid.is_blocked(down) {
                    try_move(game, 0, 1);
                    moves_made += 1;
                    going_right = false;
                } else {
                    game.enemy_step_paused = false;
                    return format!("Lawnmower search blocked by obstacle! Discovered {} squares.", discovered);
                }
            }
        } else {
            let next = Pos { x: current_pos.x - 1, y: current_pos.y };
            if game.grid.in_bounds(next) && !game.grid.is_blocked(next) {
                try_move(game, -1, 0);
                moves_made += 1;
            } else {
                let down = Pos { x: current_pos.x, y: current_pos.y + 1 };
                if game.grid.in_bounds(down) && !game.grid.is_blocked(down) {
                    try_move(game, 0, 1);
                    moves_made += 1;
                    going_right = true;
                } else {
                    game.enemy_step_paused = false;
                    return format!("Lawnmower search blocked by obstacle! Discovered {} squares.", discovered);
                }
            }
        }
        
        let robot_pos = game.robot.get_position();
        if robot_pos.1 >= game.grid.height - 1 {
            if (going_right && robot_pos.0 >= game.grid.width - 1) || 
               (!going_right && robot_pos.0 <= 0) {
                break;
            }
        }
    }
    
    game.enemy_step_paused = false;
    
    if moves_made >= max_moves {
        format!("Lawnmower search incomplete - too many moves! Discovered {} squares.", discovered)
    } else {
        format!("Lawnmower search complete! Discovered {} squares.", discovered)
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
                        boolean_param: None,
                    });
                }
            }
        }
        // Parse grab() calls
        else if trimmed.contains("grab()") {
            calls.push(FunctionCall {
                function: RustFunction::Grab,
                direction: None,
                boolean_param: None,
            });
        }
        // Parse search_all() calls
        else if trimmed.contains("search_all()") {
            calls.push(FunctionCall {
                function: RustFunction::SearchAll,
                direction: None,
                boolean_param: None,
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
                        boolean_param: None,
                    });
                }
            }
        }
        // Parse set_auto_grab() calls
        else if let Some(start) = trimmed.find("set_auto_grab(") {
            let after_paren = &trimmed[start + 14..];
            if let Some(end) = after_paren.find(')') {
                let param = after_paren[..end].trim();
                let bool_param = match param {
                    "true" | "True" => Some(true),
                    "false" | "False" => Some(false),
                    _ => None,
                };
                if bool_param.is_some() {
                    calls.push(FunctionCall {
                        function: RustFunction::AutoGrab,
                        direction: None,
                        boolean_param: bool_param,
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
        RustFunction::SearchAll => {
            try_search_all(game)
        },
        RustFunction::AutoGrab => {
            if let Some(enabled) = call.boolean_param {
                game.robot.set_auto_grab(enabled);
                if enabled {
                    "Auto-grab enabled - will grab items when moving onto squares with items".to_string()
                } else {
                    "Auto-grab disabled".to_string()
                }
            } else {
                game.robot.toggle_auto_grab();
                if game.robot.auto_grab_enabled {
                    "Auto-grab enabled".to_string()
                } else {
                    "Auto-grab disabled".to_string()
                }
            }
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
    r#"// Welcome to Rust Robot Programming!
// This file is automatically saved as you type.
// You can also edit this file externally with any text editor.

// Try this function to search all reachable areas:
search_all();

// You can also use:
// move(right);
// move(up);
// grab();  // Available from Level 2+
// scan(left);  // Available from Level 3+

// Example: Move in a pattern
// move(right);
// move(down);
// move(left);
// move(up);

// Example: Grab everything nearby
// grab();

// Example: Advanced exploration
// search_all();
// grab();
// move(right);
// move(right);
// grab();
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
        
        if result.contains("Unknown Object Blocking Function") || 
           result.contains("blocked by obstacle") || 
           result.contains("Search blocked") {
            results.push("EXECUTION HALTED! Rewrite your program to avoid obstacles.".to_string());
            break;
        }
    }
    
    results.join("; ")
}


#[cfg(not(target_arch = "wasm32"))]
fn load_yaml_levels() -> Vec<LevelSpec> {
    let mut levels = Vec::new();
    let mut rng = StdRng::seed_from_u64(0xC0FFEE);
    
    // Load YAML levels from levels directory
    let yaml_configs = load_yaml_levels_from_directory("levels");
    
    for config in yaml_configs {
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

// Drawing functions - keeping the same as original for now
const TILE: f32 = 42.0;
const PADDING: f32 = 16.0;

fn grid_origin(g: &Game) -> (f32, f32) {
    let gw = g.grid.width as f32 * TILE;
    let gh = g.grid.height as f32 * TILE;
    let ox = (screen_width() - gw) * 0.5;
    let oy = (screen_height() - gh) * 0.5;
    (ox, oy)
}

fn tile_rect(ox: f32, oy: f32, p: Pos) -> Rect {
    Rect { x: ox + p.x as f32 * TILE, y: oy + p.y as f32 * TILE, w: TILE - 1.0, h: TILE - 1.0 }
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
    // Scan in a direction to reveal tiles
    // Requires scanner to be owned
}"#,
        RustFunction::SearchAll => r#"fn search_all() -> String {
    // Automated lawnmower pattern search
    // May get blocked by obstacles
}"#,
        RustFunction::AutoGrab => r#"fn set_auto_grab(enabled: bool) -> String {
    // Enable or disable automatic item grabbing
    // When enabled, automatically grabs items when moving
}"#,
    }
}

fn draw_function_definitions(game: &Game) {
    let def_width = 600.0;
    let def_height = 400.0;
    let def_x = PADDING;
    let def_y = PADDING + 100.0;
    
    draw_rectangle(def_x - 10.0, def_y - 10.0, def_width + 20.0, def_height + 20.0, Color::new(0.0, 0.0, 0.0, 0.8));
    draw_rectangle_lines(def_x - 10.0, def_y - 10.0, def_width + 20.0, def_height + 20.0, 2.0, WHITE);
    
    draw_text("FUNCTION DEFINITIONS", def_x, def_y, 20.0, YELLOW);
    draw_text("Click a function name to view its implementation", def_x, def_y + 20.0, 12.0, GRAY);
    
    let available_functions = game.get_available_functions();
    let mut y_offset = 50.0;
    
    for func in &available_functions {
        let button_y = def_y + y_offset;
        let button_color = if game.selected_function_to_view == Some(*func) { DARKBLUE } else { DARKGRAY };
        let text_color = if game.selected_function_to_view == Some(*func) { YELLOW } else { WHITE };
        
        draw_rectangle(def_x, button_y, 200.0, 25.0, button_color);
        draw_rectangle_lines(def_x, button_y, 200.0, 25.0, 1.0, WHITE);
        
        let func_name = match func {
            RustFunction::Move => "move(direction)",
            RustFunction::Grab => "grab()",
            RustFunction::Scan => "scan(direction)",
            RustFunction::SearchAll => "search_all()",
            RustFunction::AutoGrab => "set_auto_grab(bool)",
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

fn draw_code_editor(game: &Game) {
    let editor_width = 500.0;
    let editor_height = 400.0;
    let editor_x = screen_width() - editor_width - PADDING;
    let editor_y = PADDING + 100.0;
    
    let bg_color = if game.code_editor_active { 
        Color::new(0.1, 0.1, 0.2, 0.9) 
    } else { 
        Color::new(0.0, 0.0, 0.0, 0.8) 
    };
    
    draw_rectangle(editor_x - 10.0, editor_y - 10.0, editor_width + 20.0, editor_height + 20.0, bg_color);
    draw_rectangle_lines(editor_x - 10.0, editor_y - 10.0, editor_width + 20.0, editor_height + 20.0, 2.0, 
                        if game.code_editor_active { YELLOW } else { WHITE });
    
    let title = "ROBOT CODE EDITOR";
    draw_text(title, editor_x, editor_y, 20.0, YELLOW);
    
    draw_text(&format!("File: {}", game.robot_code_path), editor_x, editor_y + 20.0, 12.0, LIGHTGRAY);
    if game.robot_code_modified {
        draw_text("File modified externally! Changes loaded.", editor_x, editor_y + 35.0, 12.0, YELLOW);
    } else {
        draw_text("Click to edit, auto-saves on changes", editor_x, editor_y + 35.0, 12.0, GRAY);
    }
    
    let available_functions = game.get_available_functions();
    let mut help_text = "Available functions: ".to_string();
    for func in &available_functions {
        match func {
            RustFunction::Move => help_text.push_str("move(up/down/left/right) "),
            RustFunction::Grab => help_text.push_str("grab() "),
            RustFunction::Scan => help_text.push_str("scan(up/down/left/right) "),
            RustFunction::SearchAll => help_text.push_str("search_all() "),
            RustFunction::AutoGrab => help_text.push_str("set_auto_grab(true/false) "),
        }
    }
    
    draw_text(&help_text, editor_x, editor_y + 35.0, 12.0, LIGHTGRAY);
    
    let input_y = editor_y + 60.0;
    let input_height = 150.0;
    draw_rectangle(editor_x, input_y, editor_width, input_height, Color::new(0.05, 0.05, 0.05, 0.9));
    draw_rectangle_lines(editor_x, input_y, editor_width, input_height, 1.0, WHITE);
    
    // Show current code from game state
    let code_to_display = if game.current_code.is_empty() {
        "// Loading robot_code.rs...".to_string()
    } else {
        game.current_code.clone()
    };
    
    let lines: Vec<&str> = code_to_display.lines().collect();
    for (i, line) in lines.iter().take(8).enumerate() {
        let line_y = input_y + 20.0 + (i as f32 * 16.0);
        let display_line = if line.len() > 55 {
            format!("{}...", &line[..52])
        } else {
            line.to_string()
        };
        let color = if game.code_editor_active { WHITE } else { LIGHTGRAY };
        draw_text(&display_line, editor_x + 10.0, line_y, 12.0, color);
    }
    if lines.len() > 8 {
        draw_text(&format!("... and {} more lines", lines.len() - 8), editor_x + 10.0, input_y + 145.0, 12.0, GRAY);
    }
    
    // Show cursor when editing (simplified cursor position)
    if game.code_editor_active {
        let cursor_line = game.current_code[..game.cursor_position].matches('\n').count();
        let line_start = game.current_code[..game.cursor_position].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let cursor_col = game.cursor_position - line_start;
        
        if cursor_line < 8 { // Only show cursor if it's in visible area
            let cursor_x = editor_x + 10.0 + (cursor_col as f32 * 7.0); // Approximate char width
            let cursor_y = input_y + 20.0 + (cursor_line as f32 * 16.0);
            draw_line(cursor_x, cursor_y - 12.0, cursor_x, cursor_y + 2.0, 1.0, YELLOW);
        }
    }
    
    let button_y = input_y + input_height + 10.0;
    
    draw_rectangle(editor_x, button_y, 100.0, 30.0, DARKGREEN);
    draw_rectangle_lines(editor_x, button_y, 100.0, 30.0, 1.0, WHITE);
    draw_text("[ENTER] Run", editor_x + 10.0, button_y + 20.0, 16.0, WHITE);
    
    draw_rectangle(editor_x + 110.0, button_y, 120.0, 30.0, DARKBLUE);
    draw_rectangle_lines(editor_x + 110.0, button_y, 120.0, 30.0, 1.0, WHITE);
    draw_text("[E] Edit in IDE", editor_x + 120.0, button_y + 20.0, 14.0, WHITE);
    
    draw_rectangle(editor_x + 240.0, button_y, 100.0, 30.0, Color::new(0.5, 0.1, 0.1, 1.0));
    draw_rectangle_lines(editor_x + 240.0, button_y, 100.0, 30.0, 1.0, WHITE);
    draw_text("[R] Reset", editor_x + 250.0, button_y + 20.0, 16.0, WHITE);
    
    if !game.execution_result.is_empty() {
        let result_y = button_y + 40.0;
        draw_text("EXECUTION RESULT:", editor_x, result_y, 16.0, WHITE);
        
        let max_chars_per_line = 45;
        let words: Vec<&str> = game.execution_result.split_whitespace().collect();
        let mut current_line = String::new();
        let mut line_count = 0;
        
        for word in words {
            if current_line.len() + word.len() + 1 > max_chars_per_line {
                draw_text(&current_line, editor_x, result_y + 20.0 + (line_count as f32 * 16.0), 14.0, GREEN);
                current_line = word.to_string();
                line_count += 1;
            } else {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            }
        }
        
        if !current_line.is_empty() {
            draw_text(&current_line, editor_x, result_y + 20.0 + (line_count as f32 * 16.0), 14.0, GREEN);
        }
    }
}

fn draw_game(game: &Game) {
    let (ox, oy) = grid_origin(game);

    for y in 0..game.grid.height {
        for x in 0..game.grid.width {
            let p = Pos { x, y };
            let r = tile_rect(ox, oy, p);

            draw_rectangle(r.x, r.y, r.w, r.h, BLACK);

            let known = game.grid.known.contains(&p);
            if known {
                draw_rectangle(r.x+2.0, r.y+2.0, r.w-4.0, r.h-4.0, GREEN);
            }

            if game.grid.is_blocked(p) && known {
                let txt = "?";
                let dim = measure_text(txt, None, 28, 1.0);
                draw_text(
                    txt,
                    r.x + (r.w - dim.width) * 0.5,
                    r.y + (r.h + dim.height) * 0.5 - 6.0,
                    28.0,
                    WHITE,
                );
            }

            // Draw items
            if known {
                if let Some(_item) = game.item_manager.get_item_at_position(p) {
                    let txt = "!";
                    let dim = measure_text(txt, None, 28, 1.0);
                    draw_text(
                        txt,
                        r.x + (r.w - dim.width) * 0.5,
                        r.y + (r.h + dim.height) * 0.5 - 6.0,
                        28.0,
                        WHITE,
                    );
                }
            }

            // Draw enemies
            if known {
                for enemy in &game.grid.enemies {
                    if enemy.pos == p {
                        let txt = "E";
                        let dim = measure_text(txt, None, 28, 1.0);
                        draw_text(
                            txt,
                            r.x + (r.w - dim.width) * 0.5,
                            r.y + (r.h + dim.height) * 0.5 - 6.0,
                            28.0,
                            RED,
                        );
                        break;
                    }
                }
            }
        }
    }

    // Robot circle
    let robot_pos = game.robot.get_pos();
    let rr = tile_rect(ox, oy, robot_pos);
    let cx = rr.x + rr.w * 0.5;
    let cy = rr.y + rr.h * 0.5;
    draw_circle(cx, cy, (TILE * 0.35).min(16.0), SKYBLUE);

    // UI
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

    // Draw time slow indicator
    if game.time_slow_active {
        draw_rectangle(screen_width() - 200.0, PADDING, 180.0, 30.0, Color::new(0.0, 0.0, 0.5, 0.8));
        draw_rectangle_lines(screen_width() - 200.0, PADDING, 180.0, 30.0, 2.0, YELLOW);
        draw_text("TIME SLOW ACTIVE", screen_width() - 190.0, PADDING + 20.0, 16.0, YELLOW);
    }

    let controls_text = "Controls: Click code editor to edit robot_code.rs | ENTER execute | E IDE hint | B shop | N finish | L reload | M menu";
    draw_text(controls_text, PADDING, screen_height() - 18.0, 18.0, GRAY);

    draw_function_definitions(game);
    draw_code_editor(game);

    if game.finished {
        let msg = "Level complete! Press N for next level.";
        let dim = measure_text(msg, None, 28, 1.0);
        draw_rectangle(
            (screen_width()-dim.width-40.0)*0.5, (screen_height()-60.0)*0.5, dim.width+40.0, 60.0,
            Color::new(0.0,0.0,0.0,0.6)
        );
        draw_text(msg, (screen_width()-dim.width)*0.5, (screen_height()+10.0)*0.5, 28.0, YELLOW);
    }
}

fn handle_shop(game: &mut Game) {
    let items = shop_items(game);
    let mut y = PADDING + 80.0;
    draw_rectangle(PADDING-10.0, y-26.0, 520.0, (items.len() as f32)*26.0 + 56.0, Color::new(0.0,0.0,0.0,0.6));
    draw_text("SHOP — press number to buy, ESC to close", PADDING, y, 20.0, YELLOW);
    y += 22.0;

    for (i, it) in items.iter().enumerate() {
        draw_text(
            &format!("[{}] {} — {} credits", i+1, it.name, it.cost),
            PADDING, y + (i as f32)*22.0, 20.0, WHITE,
        );
    }
    draw_text(&format!("Credits: {}", game.credits), PADDING, y + (items.len() as f32)*22.0 + 22.0, 20.0, WHITE);

    for (i, it) in items.iter().enumerate() {
        let keycode = match i {
            0 => KeyCode::Key1,
            1 => KeyCode::Key2,
            2 => KeyCode::Key3,
            3 => KeyCode::Key4,
            4 => KeyCode::Key5,
            5 => KeyCode::Key6,
            6 => KeyCode::Key7,
            7 => KeyCode::Key8,
            8 => KeyCode::Key9,
            _ => continue,
        };
        if is_key_pressed(keycode) && game.credits >= it.cost {
            (it.apply)(game);
            game.credits -= it.cost;
        }
    }
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
                let popup_handled_input = game.handle_popup_input();
                
                // Update popup system with delta time
                game.update_popup_system(get_frame_time());

                clear_background(Color::from_rgba(18, 18, 18, 255));
                draw_game(&game);

                if shop_open { 
                    handle_shop(&mut game); 
                }
                
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
                        let def_x = PADDING;
                        let def_y = PADDING + 100.0;
                        let available_functions = game.get_available_functions();
                        
                        for (i, func) in available_functions.iter().enumerate() {
                            let button_y = def_y + 50.0 + (i as f32 * 30.0);
                            if mouse_x >= def_x && mouse_x <= def_x + 200.0 &&
                               mouse_y >= button_y && mouse_y <= button_y + 25.0 {
                                game.selected_function_to_view = Some(*func);
                            }
                        }
                        
                        // Editor mode toggle
                        let editor_x = screen_width() - 500.0 - PADDING;
                        let editor_y = PADDING + 100.0;
                        if mouse_x >= editor_x - 10.0 && mouse_x <= editor_x + 510.0 &&
                           mouse_y >= editor_y - 10.0 && mouse_y <= editor_y + 410.0 {
                            game.code_editor_active = true;
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
                        
                        if is_key_pressed(KeyCode::R) {
                            // Reset to default code
                            game.current_code = get_default_robot_code().to_string();
                            game.cursor_position = 0;
                            code_modified = true;
                        }
                        
                        // Auto-save on any modification
                        if code_modified {
                            game.save_robot_code();
                        }
                    }
                    
                    if is_key_pressed(KeyCode::Enter) && !game.code_editor_active {
                        game.execution_result = execute_rust_code(&mut game).await;
                        game.robot_code_modified = false;
                    }
                    
                    if is_key_pressed(KeyCode::E) && !game.code_editor_active {
                        // Open external editor hint
                        game.execution_result = format!("Edit {} with your preferred IDE/editor", game.robot_code_path);
                    }

                    if is_key_pressed(KeyCode::B) { 
                        // Show shop tutorial on first time opening
                        if !game.item_manager.has_collected("shop_tutorial_shown") {
                            game.show_shop_tutorial();
                            // Mark tutorial as shown using a dummy item
                            game.item_manager.add_dummy_item("shop_tutorial_shown");
                        }
                        shop_open = true; 
                    }
                    if is_key_pressed(KeyCode::N) {
                        if !game.finished { game.finish_level(); }
                        game.next_level();
                    }
                    if is_key_pressed(KeyCode::L) {
                        let idx = game.level_idx;
                        game.load_level(idx);
                        game.execution_result.clear();
                    }
                    if is_key_pressed(KeyCode::M) {
                        // Return to main menu
                        game.menu.state = MenuState::MainMenu;
                        game.menu.setup_main_menu();
                        shop_open = false;
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