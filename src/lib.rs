#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;

mod level;
mod item;
mod grid;
mod robot;
mod game_state;
mod menu;
mod movement_patterns;
mod popup;
mod embedded_levels;
mod learning_tests;

use level::*;
use game_state::*;
use macroquad::prelude::*;

// Use `wee_alloc` as the global allocator for smaller WASM binary size
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `extern` block in C.
#[wasm_bindgen]
extern "C" {
    // Bind `console.log` from the web's global scope
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to make it easier to call console.log
macro_rules! console_log {
    ($($t:tt)*) => (unsafe { log(&format_args!($($t)*).to_string()) })
}

// Called when the WASM module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    // Set panic hook for better error messages in the browser
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    console_log!("Rust and WebAssembly game starting!");
}

// Main game entry point for WASM
#[wasm_bindgen]
pub async fn start_game() {
    console_log!("Starting Rust Steam Game in browser!");
    
    // Run the main game loop
    run_game().await;
}

// WASM-specific Game methods
impl Game {
    pub fn move_robot_wasm(&mut self, dx: i32, dy: i32) -> bool {
        let current_pos = self.robot.get_position();
        let new_x = current_pos.0 + dx;
        let new_y = current_pos.1 + dy;
        let new_pos = item::Pos { x: new_x, y: new_y };
        
        if self.grid.in_bounds(new_pos) && !self.grid.is_blocked(new_pos) {
            self.robot.set_position((new_x, new_y));
            
            // Check for items at new position
            if let Some(item) = self.item_manager.collect_item(new_pos) {
                self.show_item_collected(&item.name);
            }
            
            // Reveal adjacent squares
            let revealed = self.grid.reveal_adjacent((new_x, new_y));
            self.discovered_this_level += revealed;
            self.credits += (revealed as u32) * self.grid.income_per_square;
            
            true
        } else {
            false
        }
    }
}

// The main game function adapted for WASM
async fn run_game() {
    use macroquad::prelude::*;
    use ::rand::{rngs::StdRng, SeedableRng};

    let rng = StdRng::from_entropy();

    // Load embedded levels for WASM
    let levels = embedded_levels::get_embedded_level_specs();
    let mut game = Game::new(levels, rng);
    
    let mut current_level = 0;
    
    loop {
        clear_background(BLACK);
        
        // Handle popup input first - if popup is showing, consume input
        let popup_action = game.handle_popup_input();
        let popup_handled_input = popup_action != popup::PopupAction::None;
        
        // Update popup system with delta time
        game.update_popup_system(get_frame_time());

        // Only process game input if popup didn't handle it
        if !popup_handled_input {
            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            if is_key_pressed(KeyCode::R) && is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) {
                game.load_level(current_level);
                continue;
            }
            
            if is_key_pressed(KeyCode::C) && is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) {
                game.show_completion_instructions();
            }
        }

        if game.finished {
            // Game finished screen
            let screen_width = screen_width();
            let screen_height = screen_height();
            
            let text = if game.max_turns > 0 && game.turns >= game.max_turns {
                "Time's up! Press R to restart or ESC to quit"
            } else {
                "Level complete! Press SPACE for next level or R to restart"
            };
            
            let text_width = measure_text(text, None, 30, 1.0).width;
            draw_text(text, (screen_width - text_width) / 2.0, screen_height / 2.0, 30.0, GREEN);
            
            // Only handle level progression input if popup didn't handle it
            if !popup_handled_input && is_key_pressed(KeyCode::Space) && is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) && current_level + 1 < game.levels.len() {
                current_level += 1;
                game.load_level(current_level);
            }
            
            next_frame().await;
            continue;
        }

        // Handle input only if no popup is showing
        let mut moved = false;
        if !popup_handled_input {
            if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
                moved = game.move_robot_wasm(0, -1);
            } else if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
                moved = game.move_robot_wasm(0, 1);
            } else if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
                moved = game.move_robot_wasm(-1, 0);
            } else if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
                moved = game.move_robot_wasm(1, 0);
            }
        }

        if moved {
            game.update_laser_effects();
            game.grid.move_enemies(Some(game.robot.get_position()), &game.stunned_enemies);
            game.turns += 1;
            
            // Check for enemy collision
            let robot_pos = game.robot.get_position();
            if game.grid.check_enemy_collision(robot_pos) {
                console_log!("Hit by enemy! Restarting level...");
                game.load_level(current_level);
                continue;
            }
            
            // Check win condition (simple: visited enough squares)
            if game.discovered_this_level >= (game.grid.width * game.grid.height) as usize / 3 {
                game.finished = true;
                game.show_level_complete();
            }
            
            // Check turn limit
            if game.max_turns > 0 && game.turns >= game.max_turns {
                game.finished = true;
            }
        }

        // Simple drawing
        draw_game_wasm(&game);
        
        // Draw popups last so they appear on top
        game.draw_popups();
        
        next_frame().await;
    }
}

// Simplified drawing function for WASM
fn draw_game_wasm(game: &Game) {
    let tile_size = 30.0;
    let grid_start_x = 50.0;
    let grid_start_y = 80.0;
    
    // Draw title
    draw_text("Rust Steam Game - Web Edition", 10.0, 30.0, 24.0, WHITE);
    draw_text(&format!("Level: {} | Credits: {} | Turns: {}", 
                      game.level_idx + 1, game.credits, game.turns), 
              10.0, 60.0, 20.0, WHITE);
    
    // Draw grid
    for y in 0..game.grid.height {
        for x in 0..game.grid.width {
            let pos = item::Pos { x, y };
            let screen_x = grid_start_x + (x as f32) * tile_size;
            let screen_y = grid_start_y + (y as f32) * tile_size;
            
            let color = if game.grid.known.contains(&pos) {
                if game.grid.is_blocked(pos) {
                    if game.grid.is_door(pos) {
                        if game.grid.is_door_open(pos) {
                            GREEN  // Open door
                        } else {
                            BROWN  // Closed door
                        }
                    } else {
                        BROWN  // Regular obstacle
                    }
                } else if game.grid.visited.contains(&pos) {
                    LIGHTGRAY
                } else {
                    GRAY
                }
            } else {
                BLACK
            };
            
            draw_rectangle(screen_x, screen_y, tile_size - 2.0, tile_size - 2.0, color);
        }
    }
    
    // Draw robot
    let robot_pos = game.robot.get_position();
    let robot_screen_x = grid_start_x + (robot_pos.0 as f32) * tile_size;
    let robot_screen_y = grid_start_y + (robot_pos.1 as f32) * tile_size;
    draw_rectangle(robot_screen_x + 5.0, robot_screen_y + 5.0, tile_size - 10.0, tile_size - 10.0, SKYBLUE);
    
    // Draw enemies
    for enemy in &game.grid.enemies {
        let enemy_screen_x = grid_start_x + (enemy.pos.x as f32) * tile_size;
        let enemy_screen_y = grid_start_y + (enemy.pos.y as f32) * tile_size;
        
        // Determine enemy color based on movement type and state
        let enemy_color = if let Some(ref pattern) = enemy.movement_pattern {
            match pattern.as_str() {
                "chase" => {
                    // Check if actively chasing (orange) or not moving (blue)
                    if let Some(is_chasing) = enemy.movement_data.get("is_chasing")
                        .and_then(|v| v.as_bool()) {
                        if is_chasing {
                            ORANGE  // Actively chasing player
                        } else {
                            BLUE    // Not moving/searching
                        }
                    } else {
                        ORANGE  // Default to orange for chase enemies
                    }
                }
                "random" => MAGENTA,    // Random movement = magenta
                "diagonal" => YELLOW,   // Diagonal movement = yellow
                "circular" => LIME,     // Circular movement = lime green
                "spiral" => PINK,       // Spiral movement = pink
                pattern if pattern.starts_with("file:") => PURPLE, // Custom file patterns = purple
                _ => RED                 // Unknown patterns = red
            }
        } else {
            // Built-in horizontal/vertical enemies (no movement_pattern field)
            match enemy.direction {
                level::EnemyDirection::Horizontal => GREEN,  // Horizontal = green
                level::EnemyDirection::Vertical => DARKBLUE, // Vertical = dark blue
            }
        };
        
        draw_rectangle(enemy_screen_x + 5.0, enemy_screen_y + 5.0, tile_size - 10.0, tile_size - 10.0, enemy_color);
    }
    
    // Draw controls
    let controls_y = grid_start_y + (game.grid.height as f32 + 2.0) * tile_size;
    draw_text("Controls: WASD/Arrow Keys = Move, Ctrl+Shift+R = Restart, ESC = Quit", 
              10.0, controls_y, 16.0, WHITE);
}

// Levels are now loaded from embedded_levels module for consistency between desktop and WASM