use macroquad::prelude::*;
use crate::gamestate::Game;
use crate::item::Pos;
use crate::level::EnemyDirection;

const TILE: f32 = 42.0;
const PADDING: f32 = 16.0;

pub fn grid_origin(g: &Game) -> (f32, f32) {
    let gw = g.grid.width as f32 * TILE;
    let gh = g.grid.height as f32 * TILE;
    // Position grid to take up roughly half the screen (center-left area)
    let available_width = screen_width() * 0.5; // Half the screen width for grid
    let ox = (available_width - gw) * 0.5;
    let oy = (screen_height() - gh) * 0.5;
    (ox, oy)
}

fn tile_rect(ox: f32, oy: f32, p: Pos) -> Rect {
    Rect { x: ox + p.x as f32 * TILE, y: oy + p.y as f32 * TILE, w: TILE - 1.0, h: TILE - 1.0 }
}

pub fn draw_game(game: &Game) {
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
                // Check if it's a door
                if game.grid.is_door(p) {
                    let (txt, color) = if game.grid.is_door_open(p) {
                        ("|", GREEN)  // Open door - green vertical line
                    } else {
                        ("█", BROWN)  // Closed door - brown block
                    };
                    let dim = measure_text(txt, None, 28, 1.0);
                    draw_text(
                        txt,
                        r.x + (r.w - dim.width) * 0.5,
                        r.y + (r.h + dim.height) * 0.5 - 6.0,
                        28.0,
                        color,
                    );
                } else {
                    // Regular obstacle
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
                                EnemyDirection::Horizontal => GREEN,  // Horizontal = green
                                EnemyDirection::Vertical => DARKBLUE, // Vertical = dark blue
                            }
                        };
                        
                        draw_text(
                            txt,
                            r.x + (r.w - dim.width) * 0.5,
                            r.y + (r.h + dim.height) * 0.5 - 6.0,
                            28.0,
                            enemy_color,
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
}

pub fn draw_tutorial_overlay(game: &Game) {
    // Draw tutorial task for level 1
    if game.level_idx == 0 && game.tutorial_state.current_task < 5 {
        let tutorial_message = game.get_tutorial_task_message();
        if !tutorial_message.is_empty() {
            // Draw tutorial background
            let tutorial_y = PADDING + 70.0;
            let lines: Vec<&str> = tutorial_message.lines().collect();
            let height = (lines.len() as f32 * 16.0) + 20.0;
            
            draw_rectangle(PADDING - 10.0, tutorial_y - 10.0, 500.0, height, Color::new(0.0, 0.2, 0.4, 0.9));
            draw_rectangle_lines(PADDING - 10.0, tutorial_y - 10.0, 500.0, height, 2.0, SKYBLUE);
            
            // Draw tutorial text
            for (i, line) in lines.iter().enumerate() {
                let color = if line.starts_with("Task") { YELLOW } else { WHITE };
                draw_text(line, PADDING, tutorial_y + (i as f32 * 16.0), 14.0, color);
            }
        }
    }
}

pub fn draw_time_slow_indicator(game: &Game) {
    // Draw time slow indicator
    if game.time_slow_active {
        draw_rectangle(screen_width() - 200.0, PADDING, 180.0, 30.0, Color::new(0.0, 0.0, 0.5, 0.8));
        draw_rectangle_lines(screen_width() - 200.0, PADDING, 180.0, 30.0, 2.0, YELLOW);
        draw_text("TIME SLOW ACTIVE", screen_width() - 190.0, PADDING + 20.0, 16.0, YELLOW);
    }
}

pub fn draw_level_complete_overlay(game: &Game) {
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