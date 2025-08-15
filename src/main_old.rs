use macroquad::prelude::*;
use ::rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::process::Command;
use std::path::Path;
use std::fs;
use notify::{Watcher, RecursiveMode, Event};
use crossbeam_channel::{Receiver, Sender, unbounded};


// --------- Data Types ---------
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LevelSpec {
    name: String,
    width: usize,
    height: usize,
    start: (usize, usize),
    scanner_at: Option<(usize, usize)>,
    blockers: Vec<(usize, usize)>,
    fog_of_war: bool,
    max_turns: usize,
}

#[derive(Clone, Debug, Default)]
struct Upgrades {
    grabber_level: u32, // manhattan range
    scanner_level: u32, // contiguous scan length; 0 = not owned
    time_slow_available: bool, // unlocked after Level 4
    attack_range: u32 // continguous attack length; 0 = not owned
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum EnemyDirection {
    Horizontal, // left/right movement
    Vertical,   // up/down movement
}

#[derive(Clone, Debug)]
struct Enemy {
    pos: Pos,
    direction: EnemyDirection,
    moving_positive: bool, // true = right/down, false = left/up
}

#[derive(Clone, Debug)]
struct Grid {
    w: i32,
    h: i32,
    known: HashSet<Pos>,
    visited: HashSet<Pos>,
    blockers: HashSet<Pos>,
    scanner_pickup: Option<Pos>,
    enemies: Vec<Enemy>,
    #[allow(dead_code)]
    fog_of_war: bool,
}

#[derive(Clone, Debug)]
struct Robot {
    pos: Pos,
    upgrades: Upgrades,
    inventory: HashSet<&'static str>, // {"scanner"} if owned
    auto_grab_enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum RustFunction {
    Move,
    Grab,
    Scan,
    SearchAll,
    AutoGrab,
}

#[derive(Clone, Debug)]
struct FunctionCall {
    function: RustFunction,
    direction: Option<(i32, i32)>, // for move and scan
    boolean_param: Option<bool>, // for auto_grab
}

#[derive(Clone, Debug)]
struct Game {
    level_idx: usize,
    levels: Vec<LevelSpec>,
    grid: Grid,
    robot: Robot,
    rng: StdRng,
    credits: u32,
    turns: usize,
    max_turns: usize,
    discovered_this_level: usize,
    finished: bool,
    scan_armed: bool,
    code_input: String,
    cursor_position: usize,
    execution_result: String,
    code_editor_active: bool,
    selected_function_to_view: Option<RustFunction>,
    external_file_mode: bool,
    external_file_path: String,
    file_watcher_receiver: Option<Receiver<notify::Result<Event>>>,
    external_file_modified: bool,
    enemy_step_paused: bool,
}

// --------- Helpers ---------
impl Grid {
    fn in_bounds(&self, p: Pos) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.w && p.y < self.h
    }
}
fn manhattan(a: Pos, b: Pos) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn reveal(game: &mut Game, p: Pos) {
    if game.grid.in_bounds(p) && !game.grid.known.contains(&p) {
        game.grid.known.insert(p);
        game.discovered_this_level += 1;
    }
}
fn reveal_adjacent(game: &mut Game, p: Pos) {
    reveal(game, p);
    for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        reveal(game, Pos { x: p.x + d.0, y: p.y + d.1 });
    }
}
/// Very simple parser to collect crate names declared with `use <crate>::...` or `extern crate <crate>;`
/// Skips std/core/alloc/crate/self/super and a small ignorelist for your own crates.
fn extract_crates_from_code(code: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    let ignore: HashSet<&'static str> = [
        "std","core","alloc","crate","self","super",
        // add your known local crates / already-in-deps here if you like:
        "macroquad","serde","serde_json","rand","notify","crossbeam_channel",
    ].into_iter().collect();

    for raw in code.lines() {
        let line = raw.trim();
        if line.starts_with("use ") {
            // use foo::bar::baz;
            let rest = &line[4..];
            // cut off 'as ...' and trailing ';'
            let rest = rest.split(" as ").next().unwrap_or(rest);
            let rest = rest.trim_end_matches(';').trim();
            // first segment up to '::' or to whitespace/comma
            let first = rest.split(&[':', ' ', ',', '{'][..]).next().unwrap_or("").trim();
            if !first.is_empty() && !ignore.contains(first) && first.chars().all(|c| c.is_ascii_alphanumeric() || c=='_') {
                out.insert(first.to_string());
            }
        } else if line.starts_with("extern crate ") {
            // extern crate foo;
            let name = line["extern crate ".len()..].trim().trim_end_matches(';').trim();
            if !name.is_empty() && !ignore.contains(name) {
                out.insert(name.to_string());
            }
        }
    }
    out
}

/// Naively parse Cargo.toml `[dependencies]` keys (no TOML crate needed).
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

        // key = "...", key = { ... }, key = {path="..."} etc.
        if let Some((key,_rest)) = l.split_once('=') {
            let k = key.trim();
            if !k.is_empty() && !k.starts_with('#') {
                deps.insert(k.to_string());
            }
        } else {
            // support bare table form: foo = { ... } handled above; skip anything else
        }
    }
    deps
}

/// Check if `cargo add` is available (cargo-edit). Returns true if it seems to work.
fn cargo_add_available() -> bool {
    Command::new("cargo")
        .arg("add")
        .arg("--help")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Run `cargo add <crate>` for each name in `new_crates`. Returns a status string for the UI.
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

    // Optionally trigger a quick check to resolve and lock (comment in if desired)
    // let _ = Command::new("cargo").arg("check").status();

    if !failed.is_empty() {
        format!("Added: {:?}. Failed: {:?}", added, failed)
    } else {
        format!("Added: {:?}", added)
    }
}

/// High-level: read robot_code.rs, diff against Cargo.toml, run cargo add for new crates.
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


// --------- Enemy Movement ---------
fn move_enemies(game: &mut Game) {
    let mut new_enemies = game.grid.enemies.clone();
    for i in 0..new_enemies.len() {
        let enemy = &mut new_enemies[i];
        let step = |_pos: Pos, dir: EnemyDirection, pos_dir: bool| -> (i32, i32) {
            match dir {
                EnemyDirection::Horizontal => if pos_dir { (1, 0) } else { (-1, 0) },
                EnemyDirection::Vertical   => if pos_dir { (0, 1) } else { (0, -1) },
            }
        };

        // First attempt in current direction
        let (dx, dy) = step(enemy.pos, enemy.direction, enemy.moving_positive);
        let mut next = Pos { x: enemy.pos.x + dx, y: enemy.pos.y + dy };

        let mut can_move = game.grid.in_bounds(next)
            && !game.grid.blockers.contains(&next)
            && !game.grid.enemies.iter().any(|other| other.pos == next);

        if !can_move {
            // Reverse and try once more this tick
            enemy.moving_positive = !enemy.moving_positive;
            let (dx2, dy2) = step(enemy.pos, enemy.direction, enemy.moving_positive);
            next = Pos { x: enemy.pos.x + dx2, y: enemy.pos.y + dy2 };

            can_move = game.grid.in_bounds(next)
                && !game.grid.blockers.contains(&next)
                && !game.grid.enemies.iter().any(|other| other.pos == next);

            if !can_move {
                continue; // stuck this turn
            }
        }

        enemy.pos = next;
    }
    game.grid.enemies = new_enemies;
}

fn advance_enemies_and_check_collision(game: &mut Game) {
    if game.level_idx >= 3 {
        move_enemies(game);
        if game.grid.enemies.iter().any(|e| e.pos == game.robot.pos) {
            let idx = game.level_idx;
            load_level(game, idx);
            game.execution_result = "ENEMY COLLISION! Level reset and randomized.".to_string();
        }
    }
}



fn check_enemy_collision(game: &Game) -> bool {
    game.grid.enemies.iter().any(|enemy| enemy.pos == game.robot.pos)
}

// --------- Core Mechanics ---------

fn try_move(game: &mut Game, dx: i32, dy: i32) {
    if game.finished { return; }
    let next = Pos { x: game.robot.pos.x + dx, y: game.robot.pos.y + dy };
    if !game.grid.in_bounds(next) { return; }
    if game.grid.blockers.contains(&next) {
        reveal_adjacent(game, game.robot.pos);
        return;
    }
    // Move robot
    game.robot.pos = next;
    game.grid.visited.insert(next);
    reveal_adjacent(game, next);

    // Immediate collision: robot stepped onto an enemy
    if game.level_idx >= 3 && game.grid.enemies.iter().any(|e| e.pos == game.robot.pos) {
        let idx = game.level_idx;
        load_level(game, idx);
        game.execution_result = "ENEMY COLLISION! Level reset and randomized.".to_string();
        return;
    }

    // Enemies move after any player action, unless paused (e.g., inside search_all)
    if game.level_idx >= 3 && !game.enemy_step_paused {
        move_enemies(game);
        // Immediate collision: enemy stepped onto robot
        if game.grid.enemies.iter().any(|e| e.pos == game.robot.pos) {
            let idx = game.level_idx;
            load_level(game, idx);
            game.execution_result = "ENEMY COLLISION! Level reset and randomized.".to_string();
            return;
        }
    }

    // Auto-grab behavior
    if game.robot.auto_grab_enabled {
        let range = game.robot.upgrades.grabber_level as i32;
        let mut has_grabbable = false;
        for y in (game.robot.pos.y - range).max(0)..=(game.robot.pos.y + range).min(game.grid.h - 1) {
            for x in (game.robot.pos.x - range).max(0)..=(game.robot.pos.x + range).min(game.grid.w - 1) {
                let pos = Pos { x, y };
                if manhattan(pos, game.robot.pos) <= range && game.grid.in_bounds(pos) && !game.grid.known.contains(&pos) {
                    has_grabbable = true;
                    break;
                }
            }
            if has_grabbable { break; }
        }
        if has_grabbable { try_grab(game); }
    }
}



fn try_grab(game: &mut Game) -> &'static str {
    let range = game.robot.upgrades.grabber_level as i32;
    let mut grabbed = 0u32;
    let mut scanner_found = false;
    
    // Check for scanner pickup within grab range
    if let Some(scanner_pos) = game.grid.scanner_pickup {
        if manhattan(scanner_pos, game.robot.pos) <= range {
            // Grab the scanner!
            game.robot.inventory.insert("scanner");
            if game.robot.upgrades.scanner_level == 0 {
                game.robot.upgrades.scanner_level = 1;
            }
            game.grid.scanner_pickup = None;
            scanner_found = true;
        }
    }
    
    // Grab unknown tiles for credits
    for y in max(0, game.robot.pos.y - range)..=min(game.grid.h - 1, game.robot.pos.y + range) {
        for x in max(0, game.robot.pos.x - range)..=min(game.grid.w - 1, game.robot.pos.x + range) {
            let p = Pos { x, y };
            if manhattan(p, game.robot.pos) <= range
                && game.grid.in_bounds(p) && !game.grid.known.contains(&p) {
                    reveal(game, p);
                    grabbed += 1;
                }
        }
    }
    
    game.credits += grabbed;

    // Enemies advance on any action
    advance_enemies_and_check_collision(game);

    // Return appropriate message
    match (scanner_found, grabbed > 0) {
        (true, true) => "Grabbed scanner and unknown tiles for credits! scan() function now available.",
        (true, false) => "Grabbed scanner! scan() function now available.",
        (false, true) => "Grabbed unknown tiles for credits.",
        (false, false) => "Nothing to grab.",
    }
}

fn try_scan(game: &mut Game, dir: (i32, i32)) -> &'static str {
    if !game.robot.inventory.contains("scanner") || game.robot.upgrades.scanner_level == 0 {
        return "No scanner owned.";
    }
    let len = game.robot.upgrades.scanner_level as i32;
    let mut p = game.robot.pos;
    let mut revealed_any = false;
    for _ in 0..len {
        let next = Pos { x: p.x + dir.0, y: p.y + dir.1 };
        if !game.grid.in_bounds(next) { break; }
        if game.grid.blockers.contains(&next) {
            return "Unknown Object Blocking Function";
        }
        reveal(game, next);
        revealed_any = true;
        p = next;
    }
    // Enemies advance on any action
    advance_enemies_and_check_collision(game);

    if revealed_any { "Scan complete." } else { "Scan found nothing." }
}

fn get_available_functions(game: &Game) -> Vec<RustFunction> {
    let mut functions = vec![];
    
    // Level 1: move, search_all, and auto_grab
    functions.push(RustFunction::Move);
    functions.push(RustFunction::SearchAll);
    functions.push(RustFunction::AutoGrab);
    
    // Level 2+: grab becomes available
    if game.level_idx >= 1 {
        functions.push(RustFunction::Grab);
    }
    
    // Scan only available if player has actually grabbed the scanner
    if game.robot.inventory.contains("scanner") && game.robot.upgrades.scanner_level > 0 {
        functions.push(RustFunction::Scan);
    }
    
    functions
}

fn parse_rust_code(code: &str) -> Vec<FunctionCall> {
    let mut calls = Vec::new();
    
    // Simple parser for basic Rust function calls
    let lines: Vec<&str> = code.lines().collect();
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        
        // Simple string-based parsing (more efficient than regex)
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

fn get_function_definition(func: RustFunction) -> &'static str {
    match func {
        RustFunction::Move => r#"fn move_robot(direction: Direction) -> Result<String, String> {
    let (dx, dy) = match direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    };
    
    let old_pos = robot.position;
    let new_pos = Position {
        x: old_pos.x + dx,
        y: old_pos.y + dy,
    };
    
    if !grid.in_bounds(new_pos) {
        return Err("Move blocked - out of bounds".to_string());
    }
    
    if grid.blockers.contains(&new_pos) {
        return Err("Unknown Object Blocking Function".to_string());
    }
    
    robot.position = new_pos;
    grid.visited.insert(new_pos);
    reveal_adjacent(&mut grid, new_pos);
    
    Ok("Move executed".to_string())
}"#,
        RustFunction::Grab => r#"fn grab_items() -> String {
    let range = robot.upgrades.grabber_level as i32;
    let mut grabbed = 0;
    
    for y in max(0, robot.pos.y - range)..=min(grid.height - 1, robot.pos.y + range) {
        for x in max(0, robot.pos.x - range)..=min(grid.width - 1, robot.pos.x + range) {
            let pos = Position { x, y };
            let distance = manhattan_distance(pos, robot.pos);
            
            if distance <= range && grid.in_bounds(pos) && !grid.known.contains(&pos) {
                reveal(&mut grid, pos);
                grabbed += 1;
            }
        }
    }
    
    credits += grabbed;
    
    if grabbed > 0 {
        format!("Grabbed {} unknown tiles for credits", grabbed)
    } else {
        "Nothing to grab".to_string()
    }
}"#,
        RustFunction::Scan => r#"fn scan_direction(direction: Direction) -> Result<String, String> {
    if !robot.inventory.contains("scanner") || robot.upgrades.scanner_level == 0 {
        return Err("No scanner owned".to_string());
    }
    
    let (dx, dy) = match direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    };
    
    let scan_length = robot.upgrades.scanner_level as i32;
    let mut current_pos = robot.position;
    let mut revealed_any = false;
    
    for _ in 0..scan_length {
        let next_pos = Position {
            x: current_pos.x + dx,
            y: current_pos.y + dy,
        };
        
        if !grid.in_bounds(next_pos) {
            break;
        }
        
        if grid.blockers.contains(&next_pos) {
            return Err("Unknown Object Blocking Function".to_string());
        }
        
        reveal(&mut grid, next_pos);
        revealed_any = true;
        current_pos = next_pos;
    }
    
    if revealed_any {
        Ok("Scan complete".to_string())
    } else {
        Ok("Scan found nothing".to_string())
    }
}"#,
        RustFunction::SearchAll => r#"fn search_all() -> String {
    let mut discovered = 0;
    let mut moves_made = 0;
    let mut going_right = true;
    
    // Simple lawnmower pattern search - GETS BLOCKED BY OBSTACLES!
    // This basic algorithm forces you to design better solutions in Level 3
    
    // Step 1: Try to reach top-left corner
    while robot.position.y > 0 {
        let next = Position { x: robot.position.x, y: robot.position.y - 1 };
        if grid.blockers.contains(&next) {
            return "Search blocked - cannot reach starting position!".to_string();
        }
        move_robot(Direction::Up)?;
        moves_made += 1;
        if moves_made > 100 { break; } // Safety limit
    }
    
    while robot.position.x > 0 {
        let next = Position { x: robot.position.x - 1, y: robot.position.y };
        if grid.blockers.contains(&next) {
            return "Search blocked - cannot reach starting position!".to_string();
        }
        move_robot(Direction::Left)?;
        moves_made += 1;
        if moves_made > 100 { break; }
    }
    
    // Step 2: Lawnmower pattern (will get BLOCKED by obstacles!)
    while moves_made < 200 {
        if !grid.known.contains(&robot.position) {
            discovered += 1;
        }
        
        if going_right {
            let next = Position { x: robot.position.x + 1, y: robot.position.y };
            if grid.in_bounds(next) && !grid.blockers.contains(&next) {
                move_robot(Direction::Right)?;
            } else {
                // Hit obstacle or boundary - try going down
                let down = Position { x: robot.position.x, y: robot.position.y + 1 };
                if grid.in_bounds(down) && !grid.blockers.contains(&down) {
                    move_robot(Direction::Down)?;
                    going_right = false;
                } else {
                    // BLOCKED! This is where the magic happens in Level 3
                    return format!("❌ LAWNMOWER SEARCH BLOCKED BY OBSTACLE!
                    
Only discovered {} squares. The simple search_all() algorithm 
cannot navigate around obstacles.

🔧 TIME TO BUILD YOUR OWN SOLUTION! Use:
- move(direction) to navigate carefully
- scan(direction) to detect obstacles ahead  
- grab() to collect items along the way
- set_auto_grab(true) for automatic collection

Design a smarter algorithm that can work around the '?' blockers!", discovered);
                }
            }
        } else {
            // Same logic for moving left
            let next = Position { x: robot.position.x - 1, y: robot.position.y };
            if grid.in_bounds(next) && !grid.blockers.contains(&next) {
                move_robot(Direction::Left)?;
            } else {
                let down = Position { x: robot.position.x, y: robot.position.y + 1 };
                if grid.in_bounds(down) && !grid.blockers.contains(&down) {
                    move_robot(Direction::Down)?;
                    going_right = true;
                } else {
                    return format!("❌ BLOCKED! Design your own navigation algorithm!");
                }
            }
        }
        moves_made += 1;
    }
    
    format!("Lawnmower search discovered {} squares", discovered)
}"#,
        RustFunction::AutoGrab => r#"fn set_auto_grab(enabled: bool) -> String {
    robot.auto_grab_enabled = enabled;
    
    if enabled {
        // When auto-grab is enabled, the robot will automatically
        // grab any items when it moves onto squares containing them
        "Auto-grab enabled - robot will grab items automatically when moving"
    } else {
        "Auto-grab disabled - robot will not grab items automatically"
    }.to_string()
    
    // Note: Auto-grab works by checking if the robot's current position
    // has any grabbable items within the grabber range, and if so,
    // automatically calls the grab() function.
}"#,
    }
}

fn try_search_all(game: &mut Game) -> String {
    let mut discovered = 0;
    let mut moves_made = 0;
    let mut going_right = true;
    
    game.enemy_step_paused = true;

    // Simple lawnmower pattern: go right across each row, then move down
    // This will get blocked by obstacles and force users to create better algorithms
    
    // Try to move to top-left corner first
    while game.robot.pos.y > 0 {
        let next = Pos { x: game.robot.pos.x, y: game.robot.pos.y - 1 };
        if game.grid.blockers.contains(&next) {
            game.enemy_step_paused = false;
            advance_enemies_and_check_collision(game);
            return "Search blocked by obstacle - cannot reach starting position".to_string();
        }
        try_move(game, 0, -1);
        moves_made += 1;
        if moves_made > 100 { break; } // Safety limit
    }
    
    while game.robot.pos.x > 0 {
        let next = Pos { x: game.robot.pos.x - 1, y: game.robot.pos.y };
        if game.grid.blockers.contains(&next) {
            game.enemy_step_paused = false;
            advance_enemies_and_check_collision(game);
            return "Search blocked by obstacle - cannot reach starting position".to_string();
        }
        try_move(game, -1, 0);
        moves_made += 1;
        if moves_made > 100 { break; } // Safety limit
    }
    
    // Now do lawnmower pattern
    let max_moves = 200; // Safety limit to prevent infinite loops
    
    while moves_made < max_moves {
        // Count discovered squares in current position
        if !game.grid.known.contains(&game.robot.pos) {
            discovered += 1;
        }
        
        if going_right {
            // Try to move right
            let next = Pos { x: game.robot.pos.x + 1, y: game.robot.pos.y };
            if game.grid.in_bounds(next) && !game.grid.blockers.contains(&next) {
                try_move(game, 1, 0);
                moves_made += 1;
            } else {
        game.enemy_step_paused = false;
        advance_enemies_and_check_collision(game);
                // Hit boundary or obstacle, try to go down and switch direction
                let down = Pos { x: game.robot.pos.x, y: game.robot.pos.y + 1 };
                if game.grid.in_bounds(down) && !game.grid.blockers.contains(&down) {
                    try_move(game, 0, 1);
                    moves_made += 1;
                    going_right = false;
                } else {
        game.enemy_step_paused = false;
        advance_enemies_and_check_collision(game);
                    // Can't go down either - we're blocked
                    game.enemy_step_paused = false;
                    advance_enemies_and_check_collision(game);
                    return format!("Lawnmower search blocked by obstacle! Discovered {} squares. Try using move(), scan(), and grab() to navigate around obstacles.", discovered);
}
            }
        } else {
        game.enemy_step_paused = false;
        advance_enemies_and_check_collision(game);
            // Try to move left
            let next = Pos { x: game.robot.pos.x - 1, y: game.robot.pos.y };
            if game.grid.in_bounds(next) && !game.grid.blockers.contains(&next) {
                try_move(game, -1, 0);
                moves_made += 1;
            } else {
        game.enemy_step_paused = false;
        advance_enemies_and_check_collision(game);
                // Hit boundary or obstacle, try to go down and switch direction
                let down = Pos { x: game.robot.pos.x, y: game.robot.pos.y + 1 };
                if game.grid.in_bounds(down) && !game.grid.blockers.contains(&down) {
                    try_move(game, 0, 1);
                    moves_made += 1;
                    going_right = true;
                } else {
        game.enemy_step_paused = false;
        advance_enemies_and_check_collision(game);
                    // Can't go down either - we're blocked
                    game.enemy_step_paused = false;
                    advance_enemies_and_check_collision(game);
                    return format!("Lawnmower search blocked by obstacle! Discovered {} squares. Try using move(), scan(), and grab() to navigate around obstacles.", discovered);
}
            }
        }
        
        // Check if we've reached the bottom and completed the pattern
        if game.robot.pos.y >= game.grid.h - 1 {
            // Check if we're at the rightmost or leftmost edge
            if (going_right && game.robot.pos.x >= game.grid.w - 1) || 
               (!going_right && game.robot.pos.x <= 0) {
                break; // Completed the lawnmower pattern
            }
        }
    }
    
    if moves_made >= max_moves {
        game.enemy_step_paused = false;
        advance_enemies_and_check_collision(game);
        format!("Lawnmower search incomplete - too many moves! Discovered {} squares. Consider using scan() to plan better routes.", discovered)
    } else {
        game.enemy_step_paused = false;
        advance_enemies_and_check_collision(game);
        format!("Lawnmower search complete! Discovered {} squares.", discovered)
    }
}

fn execute_function(game: &mut Game, call: FunctionCall) -> String {
    let available = get_available_functions(game);
    if !available.contains(&call.function) {
        return "Function not available".to_string();
    }
    
    match call.function {
        RustFunction::Move => {
            if let Some((dx, dy)) = call.direction {
                let old_pos = game.robot.pos;
                try_move(game, dx, dy);
                game.turns += 1;
                if game.robot.pos != old_pos {
                    "Move executed".to_string()
                } else {
                    if game.grid.blockers.contains(&Pos { x: old_pos.x + dx, y: old_pos.y + dy }) {
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
                game.robot.auto_grab_enabled = enabled;
                if enabled {
                    "Auto-grab enabled - will grab items when moving onto squares with items".to_string()
                } else {
                    "Auto-grab disabled".to_string()
                }
            } else {
                // Toggle if no parameter provided
                game.robot.auto_grab_enabled = !game.robot.auto_grab_enabled;
                if game.robot.auto_grab_enabled {
                    "Auto-grab enabled".to_string()
                } else {
                    "Auto-grab disabled".to_string()
                }
            }
        },
    }
}

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
    
    // Keep watcher alive by leaking it (simple approach for game)
    std::mem::forget(watcher);
    
    Some(rx)
}

fn load_external_code(file_path: &str) -> Result<String, String> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}

fn create_sample_external_file(file_path: &str) -> Result<(), String> {
    let sample_code = r#"// Rust Robot Programming - External File Mode
// Save this file and the game will automatically detect changes!
// Use your favorite IDE/editor to write code here.

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
"#;
    
    match fs::write(file_path, sample_code) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create sample file: {}", e)),
    }
}

fn execute_rust_code(game: &mut Game) -> String {
    let code_to_execute = if game.external_file_mode {
        match load_external_code(&game.external_file_path) {
            Ok(code) => code,
            Err(e) => return e,
        }
    } else {
        game.code_input.clone()
    };
    
    let calls = parse_rust_code(&code_to_execute);
    if calls.is_empty() {
        return "No valid function calls found".to_string();
    }
    
    let mut results = Vec::new();
    for call in calls {
        let result = execute_function(game, call);
        results.push(result.clone());
        
        // HALT EXECUTION if we hit a blocker - forces user to rewrite their program
        if result.contains("Unknown Object Blocking Function") || 
           result.contains("blocked by obstacle") || 
           result.contains("Search blocked") {
            results.push("EXECUTION HALTED! Rewrite your program to avoid obstacles.".to_string());
            break;
        }
    }
    
    results.join("; ")
}

// --------- Shop ---------
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
            apply: |g| g.robot.upgrades.grabber_level += 1,
        }
    ];
    // Scanner is not for sale on Level 4 (index 3)
    if game.level_idx < 3 {
        if !game.robot.inventory.contains("scanner") {
            v.push(ShopItem {
                name: "Scanner (len 1)",
                cost: 8,
                apply: |g| { g.robot.inventory.insert("scanner"); if g.robot.upgrades.scanner_level == 0 { g.robot.upgrades.scanner_level = 1; } },
            });
        } else {
            v.push(ShopItem {
                name: "Scanner +1 length",
                cost: 7 + game.robot.upgrades.scanner_level * 4,
                apply: |g| g.robot.upgrades.scanner_level += 1,
            });
        }
    }
    v
}


// --------- Levels ---------
fn make_built_in_levels() -> Vec<LevelSpec> {
    vec![
        LevelSpec { name: "Level 1 - Explore the grid".into(), width: 12, height: 8, start: (1,1), scanner_at: None, blockers: vec![], fog_of_war: true, max_turns: 0 },
        LevelSpec { name: "Level 2 - Find the scanner".into(), width: 14, height: 9, start: (2,2), scanner_at: None, blockers: vec![], fog_of_war: true, max_turns: 0 },
        LevelSpec { name: "Level 3 - Blockers!".into(), width: 16, height: 10, start: (1,1), scanner_at: None, blockers: vec![], fog_of_war: true, max_turns: 0 },
        LevelSpec { name: "Level 4 - Moving Enemies".into(), width: 18, height: 12, start: (1,1), scanner_at: None, blockers: vec![], fog_of_war: true, max_turns: 0 },
    ]
}
fn generate_grid_from_level(rng: &mut StdRng, spec: &LevelSpec, robot_carries_scanner: bool) -> Grid {
    let mut blockers = HashSet::new();
    let mut scanner_pick = spec.scanner_at.map(|(x,y)| Pos { x: x as i32, y: y as i32 });
    let mut enemies = Vec::new();

    if spec.name.contains("Level 2") && scanner_pick.is_none() && !robot_carries_scanner {
        let (w, h) = (spec.width as i32, spec.height as i32);
        loop {
            let p = Pos { x: rng.gen_range(0..w), y: rng.gen_range(0..h) };
            if p != (Pos { x: spec.start.0 as i32, y: spec.start.1 as i32 }) {
                scanner_pick = Some(p);
                break;
            }
        }
    }
    if spec.name.contains("Level 3") && spec.blockers.is_empty() {
        let (w, h) = (spec.width as i32, spec.height as i32);
        let n = (w * h) / 8;
        for _ in 0..n {
            let p = Pos { x: rng.gen_range(0..w), y: rng.gen_range(0..h) };
            if p != (Pos { x: spec.start.0 as i32, y: spec.start.1 as i32 }) {
                blockers.insert(p);
            }
        }
    } else if spec.name.contains("Level 4") && spec.blockers.is_empty() {
        let (w, h) = (spec.width as i32, spec.height as i32);
        // Level 4 has NO items
        scanner_pick = None;
        
        // Generate some obstacles for Level 4
        let obstacle_count = (w * h) / 12;
        for _ in 0..obstacle_count {
            let p = Pos { x: rng.gen_range(0..w), y: rng.gen_range(0..h) };
            if p != (Pos { x: spec.start.0 as i32, y: spec.start.1 as i32 }) {
                blockers.insert(p);
            }
        }
        
        // Generate enemies for Level 4
        let enemy_count = 3;
        for _ in 0..enemy_count {
            loop {
                let pos = Pos { x: rng.gen_range(2..w-2), y: rng.gen_range(2..h-2) };
                let start_pos = Pos { x: spec.start.0 as i32, y: spec.start.1 as i32 };
                if pos != start_pos && !blockers.contains(&pos) && manhattan(pos, start_pos) > 3 {
                    let direction = if rng.gen_bool(0.5) { EnemyDirection::Horizontal } else { EnemyDirection::Vertical };
                    let moving_positive = rng.gen_bool(0.5);
                    enemies.push(Enemy { pos, direction, moving_positive });
                    break;
                }
            }
        }
    } else {
        for (x, y) in &spec.blockers {
            blockers.insert(Pos { x: *x as i32, y: *y as i32 });
        }
    }

    Grid {
        w: spec.width as i32,
        h: spec.height as i32,
        known: HashSet::new(),
        visited: HashSet::new(),
        blockers,
        scanner_pickup: scanner_pick,
        enemies,
        fog_of_war: spec.fog_of_war,
    }
}
fn finish_level(game: &mut Game) {
    game.finished = true;
    let reward = game.discovered_this_level as u32;
    game.credits += reward;
}
fn load_level(game: &mut Game, idx: usize) {
    let spec = game.levels[idx].clone();
    let mut grid = generate_grid_from_level(&mut game.rng, &spec, game.robot.inventory.contains("scanner"));
    let start = Pos { x: spec.start.0 as i32, y: spec.start.1 as i32 };
    game.robot.pos = start;

    // Reveal starting tile + neighbors
    let mut dummy = game.clone();
    dummy.grid = grid.clone();
    reveal_adjacent(&mut dummy, start);
    grid = dummy.grid;

    game.grid = grid;
    game.turns = 0;
    game.max_turns = spec.max_turns;
    game.discovered_this_level = 0;
    game.finished = false;
    game.scan_armed = false;
    game.enemy_step_paused = false;
}
fn next_level(game: &mut Game) {
    if game.level_idx + 1 < game.levels.len() {
        game.level_idx += 1;
        load_level(game, game.level_idx);
    }
}

fn load_custom_levels_from(dir: &str) -> Vec<LevelSpec> {
    let mut out = vec![];
    if !Path::new(dir).exists() { return out; }
    let entries = fs::read_dir(dir).ok();
    if let Some(entries) = entries {
        for e in entries.flatten() {
            let path = e.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(txt) = fs::read_to_string(&path) {
                    if let Ok(spec) = serde_json::from_str::<LevelSpec>(&txt) {
                        out.push(spec);
                    }
                }
            }
        }
    }
    out
}

// --------- Drawing ---------
const TILE: f32 = 42.0;
const PADDING: f32 = 16.0;

fn grid_origin(g: &Game) -> (f32, f32) {
    // Center grid on screen
    let gw = g.grid.w as f32 * TILE;
    let gh = g.grid.h as f32 * TILE;
    let ox = (screen_width() - gw) * 0.5;
    let oy = (screen_height() - gh) * 0.5;
    (ox, oy)
}
fn tile_rect(ox: f32, oy: f32, p: Pos) -> Rect {
    Rect { x: ox + p.x as f32 * TILE, y: oy + p.y as f32 * TILE, w: TILE - 1.0, h: TILE - 1.0 }
}
fn draw_function_definitions(game: &Game) {
    let def_width = 600.0;
    let def_height = 400.0;
    let def_x = PADDING;
    let def_y = PADDING + 100.0;
    
    // Function definitions background
    draw_rectangle(def_x - 10.0, def_y - 10.0, def_width + 20.0, def_height + 20.0, Color::new(0.0, 0.0, 0.0, 0.8));
    draw_rectangle_lines(def_x - 10.0, def_y - 10.0, def_width + 20.0, def_height + 20.0, 2.0, WHITE);
    
    // Title
    draw_text("FUNCTION DEFINITIONS", def_x, def_y, 20.0, YELLOW);
    draw_text("Click a function name to view its implementation", def_x, def_y + 20.0, 12.0, GRAY);
    
    // Available functions as clickable buttons
    let available_functions = get_available_functions(game);
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
    
    // Show selected function definition
    if let Some(func) = game.selected_function_to_view {
        let code_y = def_y + y_offset + 10.0;
        let code_area_height = def_height - y_offset - 20.0;
        
        // Code background
        draw_rectangle(def_x, code_y, def_width, code_area_height, Color::new(0.05, 0.05, 0.1, 0.9));
        draw_rectangle_lines(def_x, code_y, def_width, code_area_height, 1.0, LIGHTGRAY);
        
        // Function code
        let definition = get_function_definition(func);
        let lines: Vec<&str> = definition.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            let line_y = code_y + 20.0 + (i as f32 * 14.0);
            if line_y < code_y + code_area_height - 10.0 {
                // Syntax highlighting - basic colors
                let color = if line.trim().starts_with("//") {
                    Color::new(0.5, 0.7, 0.5, 1.0) // Green for comments
                } else if line.contains("fn ") || line.contains("let ") || line.contains("if ") || line.contains("for ") {
                    Color::new(0.8, 0.6, 1.0, 1.0) // Purple for keywords
                } else if line.contains('"') {
                    Color::new(1.0, 0.8, 0.6, 1.0) // Orange for strings
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
    
    // Editor background
    let bg_color = if game.external_file_mode {
        Color::new(0.1, 0.2, 0.1, 0.9) // Green tint for external mode
    } else if game.code_editor_active { 
        Color::new(0.1, 0.1, 0.2, 0.9) 
    } else { 
        Color::new(0.0, 0.0, 0.0, 0.8) 
    };
    draw_rectangle(editor_x - 10.0, editor_y - 10.0, editor_width + 20.0, editor_height + 20.0, bg_color);
    draw_rectangle_lines(editor_x - 10.0, editor_y - 10.0, editor_width + 20.0, editor_height + 20.0, 2.0, if game.external_file_mode { GREEN } else if game.code_editor_active { YELLOW } else { WHITE });
    
    // Mode toggle buttons
    let toggle_y = editor_y - 35.0;
    
    // Internal editor button
    let internal_btn_color = if !game.external_file_mode { DARKBLUE } else { DARKGRAY };
    draw_rectangle(editor_x, toggle_y, 120.0, 25.0, internal_btn_color);
    draw_rectangle_lines(editor_x, toggle_y, 120.0, 25.0, 1.0, WHITE);
    draw_text("Internal Editor", editor_x + 5.0, toggle_y + 17.0, 14.0, WHITE);
    
    // External file button
    let external_btn_color = if game.external_file_mode { DARKGREEN } else { DARKGRAY };
    draw_rectangle(editor_x + 125.0, toggle_y, 120.0, 25.0, external_btn_color);
    draw_rectangle_lines(editor_x + 125.0, toggle_y, 120.0, 25.0, 1.0, WHITE);
    draw_text("External File", editor_x + 130.0, toggle_y + 17.0, 14.0, WHITE);
    
    // Title
    let title = if game.external_file_mode { "EXTERNAL FILE MODE" } else { "RUST CODE EDITOR" };
    draw_text(title, editor_x, editor_y, 20.0, if game.external_file_mode { GREEN } else { YELLOW });
    
    if game.external_file_mode {
        draw_text(&format!("File: {}", game.external_file_path), editor_x, editor_y + 20.0, 12.0, LIGHTGRAY);
        if game.external_file_modified {
            draw_text("File modified! Press ENTER to execute", editor_x, editor_y + 35.0, 12.0, YELLOW);
        } else {
            draw_text("Watching for file changes...", editor_x, editor_y + 35.0, 12.0, GREEN);
        }
    } else {
        draw_text("Click to activate, ESC to deactivate", editor_x, editor_y + 20.0, 12.0, GRAY);
    }
    
    // Available functions help
    let available_functions = get_available_functions(game);
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
    
    // Add conditional help for unavailable functions
    if !game.robot.inventory.contains("scanner") && game.level_idx >= 1 {
        help_text.push_str(" | Need scanner: grab() the scanner item first to unlock scan()!");
    }
    
    draw_text(&help_text, editor_x, editor_y + 35.0, 12.0, LIGHTGRAY);
    
    // Code display area
    let input_y = editor_y + 60.0;
    let input_height = 150.0;
    draw_rectangle(editor_x, input_y, editor_width, input_height, Color::new(0.05, 0.05, 0.05, 0.9));
    draw_rectangle_lines(editor_x, input_y, editor_width, input_height, 1.0, WHITE);
    
    if game.external_file_mode {
        // Show external file content (read-only preview)
        match load_external_code(&game.external_file_path) {
            Ok(external_code) => {
                let lines: Vec<&str> = external_code.lines().collect();
                for (i, line) in lines.iter().take(8).enumerate() { // Show first 8 lines
                    let line_y = input_y + 20.0 + (i as f32 * 16.0);
                    let display_line = if line.len() > 55 {
                        format!("{}...", &line[..52])
                    } else {
                        line.to_string()
                    };
                    draw_text(&display_line, editor_x + 10.0, line_y, 12.0, LIGHTGRAY);
                }
                if lines.len() > 8 {
                    draw_text(&format!("... and {} more lines", lines.len() - 8), editor_x + 10.0, input_y + 145.0, 12.0, GRAY);
                }
            },
            Err(e) => {
                draw_text(&e, editor_x + 10.0, input_y + 20.0, 14.0, RED);
            }
        }
    } else {
        // Show internal editor content
        let lines: Vec<&str> = game.code_input.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            let line_y = input_y + 20.0 + (i as f32 * 16.0);
            if line_y < input_y + input_height - 10.0 {
                draw_text(line, editor_x + 10.0, line_y, 14.0, WHITE);
            }
        }
        
        // Draw cursor if editor is active
        if game.code_editor_active {
            let cursor_line = game.code_input[..game.cursor_position].matches('\n').count();
            let line_start = game.code_input[..game.cursor_position].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let cursor_col = game.cursor_position - line_start;
            
            let cursor_x = editor_x + 10.0 + (cursor_col as f32 * 8.0);
            let cursor_y = input_y + 20.0 + (cursor_line as f32 * 16.0);
            
            if cursor_y < input_y + input_height - 10.0 {
                draw_line(cursor_x, cursor_y - 12.0, cursor_x, cursor_y + 2.0, 1.0, YELLOW);
            }
        }
    }
    
    // Control buttons
    let button_y = input_y + input_height + 10.0;
    
    if game.external_file_mode {
        // Create file button
        draw_rectangle(editor_x, button_y, 120.0, 30.0, DARKBLUE);
        draw_rectangle_lines(editor_x, button_y, 120.0, 30.0, 1.0, WHITE);
        draw_text("[F] Create File", editor_x + 5.0, button_y + 20.0, 14.0, WHITE);
        
        // Execute button
        draw_rectangle(editor_x + 130.0, button_y, 100.0, 30.0, DARKGREEN);
        draw_rectangle_lines(editor_x + 130.0, button_y, 100.0, 30.0, 1.0, WHITE);
        draw_text("[ENTER] Run", editor_x + 140.0, button_y + 20.0, 14.0, WHITE);
    } else {
        // Execute button
        draw_rectangle(editor_x, button_y, 100.0, 30.0, DARKGREEN);
        draw_rectangle_lines(editor_x, button_y, 100.0, 30.0, 1.0, WHITE);
        draw_text("[ENTER] Run", editor_x + 10.0, button_y + 20.0, 16.0, WHITE);
        
        // Clear button
        draw_rectangle(editor_x + 110.0, button_y, 100.0, 30.0, Color::new(0.5, 0.1, 0.1, 1.0));
        draw_rectangle_lines(editor_x + 110.0, button_y, 100.0, 30.0, 1.0, WHITE);
        draw_text("[C] Clear", editor_x + 120.0, button_y + 20.0, 16.0, WHITE);
    }
    
    // Execution result
    if !game.execution_result.is_empty() {
        let result_y = button_y + 40.0;
        draw_text("EXECUTION RESULT:", editor_x, result_y, 16.0, WHITE);
        
        // Word wrap the result text
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

    // Grid cells
    for y in 0..game.grid.h {
        for x in 0..game.grid.w {
            let p = Pos { x, y };
            let r = tile_rect(ox, oy, p);

            // Base tile: black square always
            draw_rectangle(r.x, r.y, r.w, r.h, BLACK);

            // Known/searched overlay: green
            let known = game.grid.known.contains(&p);
            if known {
                draw_rectangle(r.x+2.0, r.y+2.0, r.w-4.0, r.h-4.0, GREEN);
            }

            // Blocker overlay: show "?" text ONLY on known/explored squares
            if game.grid.blockers.contains(&p) && known {
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

            // Item overlay: "!" if scanner present and not picked up, ONLY on known/explored squares
            if game.grid.scanner_pickup == Some(p) && known {
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

            // Enemy overlay: "E" for enemies, ONLY on known/explored squares
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
    let rp = game.robot.pos;
    let rr = tile_rect(ox, oy, rp);
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
    draw_text(
        &format!("Upgrades  Grabber range={}  |  Scanner len={}{}", game.robot.upgrades.grabber_level, game.robot.upgrades.scanner_level, if game.robot.inventory.contains("scanner") { " (owned)" } else { "" }),
        PADDING, PADDING + 46.0, 20.0, WHITE,
    );

    let controls_text = if game.external_file_mode {
        "Controls: Edit robot_code.rs in your IDE | ENTER execute | F create file | B shop | N finish | L reload | Q quit"
    } else {
        "Controls: Write Rust code to control robot | ENTER execute | B shop | N finish | L reload | Q quit"
    };
    draw_text(controls_text, PADDING, screen_height() - 18.0, 18.0, GRAY);

    // Draw function definitions
    draw_function_definitions(game);
    
    // Draw code editor
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

// --------- Game Loop ---------
fn end_condition_check(game: &mut Game) {
    if game.finished { return; }
    
    // Check for enemy collision (Level 4+)
    if game.level_idx >= 3 && check_enemy_collision(game) {
        // Reset and randomize the level when enemy catches player
        let idx = game.level_idx;
        load_level(game, idx);
        game.execution_result = "ENEMY COLLISION! Level reset and randomized.".to_string();
        return;
    }
    
    // Check if all items have been collected
    let items_remaining = game.grid.scanner_pickup.is_some();
    if items_remaining {
        // Cannot complete level until all items are grabbed
        return;
    }
    
    // Original completion logic - all squares explored
    let total_cells = (game.grid.w * game.grid.h) as usize;
    let known_nonblockers = game.grid.known.iter().filter(|p| !game.grid.blockers.contains(p)).count();
    let blockers_count = game.grid.blockers.len();
    if known_nonblockers + blockers_count == total_cells || (game.max_turns>0 && game.turns>=game.max_turns) {
        finish_level(game);
    }
}

fn handle_shop(game: &mut Game) {
    // Minimal in-place shop overlay: number keys buy if affordable
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

    // Inputs for shop
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
        if is_key_pressed(keycode)
            && game.credits >= it.cost {
                (it.apply)(game);
                game.credits -= it.cost;
            }
    }
}

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

#[macroquad::main(window_conf)]
async fn main() {
    // Init RNG + levels
    let mut rng = StdRng::seed_from_u64(0xC0FFEE);
    let mut levels = make_built_in_levels();
    let mut user_levels = load_custom_levels_from("levels");
    levels.append(&mut user_levels);

    let first = levels.first().expect("no levels").clone();
    let start = Pos { x: first.start.0 as i32, y: first.start.1 as i32 };
    let grid = generate_grid_from_level(&mut rng, &first, false);
    let robot = Robot { 
        pos: start, 
        upgrades: Upgrades { grabber_level: 1, scanner_level: 0, attack_range: 0, time_slow_available: false }, 
        inventory: HashSet::new(),
        auto_grab_enabled: false,
    };

    let mut game = Game {
        level_idx: 0,
        levels,
        grid,
        robot,
        rng,
        credits: 0,
        turns: 0,
        max_turns: first.max_turns,
        discovered_this_level: 0,
        finished: false,
        scan_armed: false,
        code_input: r#"// Welcome to Rust Robot Programming!
// Try this function to search all reachable areas:
search_all();

// You can also use:
// move(right);
// move(up);
// grab();  // Available from Level 2+
// scan(left);  // Available from Level 3+"#.to_string(),
        cursor_position: 67, // Position after "search_all();"
        execution_result: String::new(),
        code_editor_active: false,
        selected_function_to_view: None,
        external_file_mode: false,
        external_file_path: "robot_code.rs".to_string(),
        file_watcher_receiver: None,
        external_file_modified: false,
        enemy_step_paused: false,
    };

    let mut shop_open = false;

    loop {
        clear_background(Color::from_rgba(18, 18, 18, 255));

        // Draw
        draw_game(&game);

        // Shop overlay
        if shop_open { handle_shop(&mut game); }

        // Input
        if !shop_open {
            if is_key_pressed(KeyCode::Q) { break; }

            // Check for file changes
            if game.external_file_mode {
                if let Some(ref receiver) = game.file_watcher_receiver {
                    if let Ok(_event) = receiver.try_recv() {
                        game.external_file_modified = true;
                    }
                }
            }
            
            // Mouse handling
            let (mouse_x, mouse_y) = mouse_position();
            
            if is_mouse_button_pressed(MouseButton::Left) {
                // Function definitions area
                let def_x = PADDING;
                let def_y = PADDING + 100.0;
                let available_functions = get_available_functions(&game);
                
                // Check if clicked on function buttons
                for (i, func) in available_functions.iter().enumerate() {
                    let button_y = def_y + 50.0 + (i as f32 * 30.0);
                    if mouse_x >= def_x && mouse_x <= def_x + 200.0 &&
                       mouse_y >= button_y && mouse_y <= button_y + 25.0 {
                        game.selected_function_to_view = Some(*func);
                    }
                }
                
                // Editor mode toggle buttons
                let editor_x = screen_width() - 500.0 - PADDING;
                let toggle_y = PADDING + 100.0 - 35.0;
                
                // Internal editor button
                if mouse_x >= editor_x && mouse_x <= editor_x + 120.0 &&
                   mouse_y >= toggle_y && mouse_y <= toggle_y + 25.0 {
                    game.external_file_mode = false;
                    game.code_editor_active = true;
                }
                
                // External file button
                if mouse_x >= editor_x + 125.0 && mouse_x <= editor_x + 245.0 &&
                   mouse_y >= toggle_y && mouse_y <= toggle_y + 25.0 {
                    game.external_file_mode = true;
                    game.code_editor_active = false;
                    
                    // Setup file watcher if not already done
                    if game.file_watcher_receiver.is_none() {
                        game.file_watcher_receiver = setup_file_watcher(&game.external_file_path);
                    }
                }
                
                // Code editor activation (only for internal mode)
                let editor_y = PADDING + 100.0;
                if !game.external_file_mode {
                    if mouse_x >= editor_x - 10.0 && mouse_x <= editor_x + 510.0 &&
                       mouse_y >= editor_y - 10.0 && mouse_y <= editor_y + 410.0 {
                        game.code_editor_active = true;
                    } else if mouse_x > screen_width() / 2.0 {
                        game.code_editor_active = false;
                    }
                }
            }
            
            if is_key_pressed(KeyCode::Escape) {
                game.code_editor_active = false;
            }
            
            // Code editor input (only for internal mode)
            if !game.external_file_mode && game.code_editor_active {
                // Handle text input
                while let Some(character) = get_char_pressed() {
                    if character.is_ascii() && !character.is_control() {
                        game.code_input.insert(game.cursor_position, character);
                        game.cursor_position += 1;
                    }
                }
                
                // Handle special keys
                if is_key_pressed(KeyCode::Enter) {
                    if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                        // Execute code
                        game.execution_result = execute_rust_code(&mut game);
                    } else {
                        // New line
                        game.code_input.insert(game.cursor_position, '\n');
                        game.cursor_position += 1;
                    }
                }
                
                if is_key_pressed(KeyCode::Backspace) {
                    if game.cursor_position > 0 {
                        game.cursor_position -= 1;
                        game.code_input.remove(game.cursor_position);
                    }
                }
                
                if is_key_pressed(KeyCode::Left) && game.cursor_position > 0 {
                    game.cursor_position -= 1;
                }
                
                if is_key_pressed(KeyCode::Right) && game.cursor_position < game.code_input.len() {
                    game.cursor_position += 1;
                }
                
                if is_key_pressed(KeyCode::Up) {
                    let current_line_start = game.code_input[..game.cursor_position].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    let current_col = game.cursor_position - current_line_start;
                    
                    if current_line_start > 0 {
                        let prev_line_start = game.code_input[..current_line_start - 1].rfind('\n').map(|i| i + 1).unwrap_or(0);
                        let prev_line_end = current_line_start - 1;
                        let prev_line_len = prev_line_end - prev_line_start;
                        let new_col = current_col.min(prev_line_len);
                        game.cursor_position = prev_line_start + new_col;
                    }
                }
                
                if is_key_pressed(KeyCode::Down) {
                    let current_line_start = game.code_input[..game.cursor_position].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    let current_col = game.cursor_position - current_line_start;
                    
                    if let Some(next_line_start) = game.code_input[game.cursor_position..].find('\n').map(|i| game.cursor_position + i + 1) {
                        if next_line_start < game.code_input.len() {
                            let next_line_end = game.code_input[next_line_start..].find('\n')
                                .map(|i| next_line_start + i)
                                .unwrap_or(game.code_input.len());
                            let next_line_len = next_line_end - next_line_start;
                            let new_col = current_col.min(next_line_len);
                            game.cursor_position = next_line_start + new_col;
                        }
                    }
                }
                
                if is_key_pressed(KeyCode::Home) {
                    let current_line_start = game.code_input[..game.cursor_position].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    game.cursor_position = current_line_start;
                }
                
                if is_key_pressed(KeyCode::End) {
                    let current_line_end = game.code_input[game.cursor_position..].find('\n')
                        .map(|i| game.cursor_position + i)
                        .unwrap_or(game.code_input.len());
                    game.cursor_position = current_line_end;
                }
                
                if is_key_pressed(KeyCode::C) {
                    game.code_input.clear();
                    game.cursor_position = 0;
                    game.execution_result.clear();
                }
            }
            
            // Global shortcuts
            if is_key_pressed(KeyCode::Enter) && !game.code_editor_active {
                game.execution_result = execute_rust_code(&mut game);
                if game.external_file_mode {
                    game.external_file_modified = false;
                }
            }
            
            // External file mode shortcuts
            if game.external_file_mode {
                if is_key_pressed(KeyCode::F) {
                    match create_sample_external_file(&game.external_file_path) {
                        Ok(_) => {
                            game.execution_result = format!("Created sample file: {}", game.external_file_path);
                        },
                        Err(e) => {
                            game.execution_result = e;
                        }
                    }
                }
            }

            if is_key_pressed(KeyCode::B) { shop_open = true; }
            if is_key_pressed(KeyCode::N) {
                if !game.finished { finish_level(&mut game); }
                next_level(&mut game);
            }
            if is_key_pressed(KeyCode::L) {
                // reload current level (or refresh random placements)
                let idx = game.level_idx;
                load_level(&mut game, idx);
                game.code_input.clear();
                game.cursor_position = 0;
                game.execution_result.clear();
            }
        } else {
            // Shop open inputs
            if is_key_pressed(KeyCode::Escape) { shop_open = false; }
        }

        // Check end condition
        end_condition_check(&mut game);

        next_frame().await;
    }
}
