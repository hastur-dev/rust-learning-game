use macroquad::prelude::*;
use crate::gamestate::Game;
use crate::item::Pos;
use crate::level::EnemyDirection;
use crate::font_scaling::*;

const TILE: f32 = 42.0;

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
                    let font_size = 28.0;
                    let scaled_font_size = scale_font_size(font_size);
                    let dim = measure_text(txt, None, scaled_font_size as u16, 1.0);
                    draw_scaled_text(
                        txt,
                        r.x + (r.w - dim.width) * 0.5,
                        r.y + (r.h + dim.height) * 0.5 - scale_size(6.0),
                        font_size,
                        color,
                    );
                } else {
                    // Regular obstacle
                    let txt = "?";
                    let font_size = 28.0;
                    let scaled_font_size = scale_font_size(font_size);
                    let dim = measure_text(txt, None, scaled_font_size as u16, 1.0);
                    draw_scaled_text(
                        txt,
                        r.x + (r.w - dim.width) * 0.5,
                        r.y + (r.h + dim.height) * 0.5 - scale_size(6.0),
                        font_size,
                        WHITE,
                    );
                }
            }

            // Draw items
            if known {
                if let Some(_item) = game.item_manager.get_item_at_position(p) {
                    let txt = "!";
                    let font_size = 28.0;
                    let scaled_font_size = scale_font_size(font_size);
                    let dim = measure_text(txt, None, scaled_font_size as u16, 1.0);
                    draw_scaled_text(
                        txt,
                        r.x + (r.w - dim.width) * 0.5,
                        r.y + (r.h + dim.height) * 0.5 - scale_size(6.0),
                        font_size,
                        WHITE,
                    );
                }
            }

            // Draw enemies
            if known {
                for enemy in &game.grid.enemies {
                    if enemy.pos == p {
                        let txt = "E";
                        let font_size = 28.0;
                        let scaled_font_size = scale_font_size(font_size);
                        let dim = measure_text(txt, None, scaled_font_size as u16, 1.0);
                        
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
                        
                        draw_scaled_text(
                            txt,
                            r.x + (r.w - dim.width) * 0.5,
                            r.y + (r.h + dim.height) * 0.5 - scale_size(6.0),
                            font_size,
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
    // Draw tutorial task for all learning levels
    let should_show_tutorial = if game.is_learning_level(game.level_idx) {
        if let Some(max_tasks) = game.get_max_tasks_for_level(game.level_idx) {
            game.tutorial_state.current_task < max_tasks
        } else {
            false
        }
    } else {
        false
    };
    
    if should_show_tutorial {
        let tutorial_message = game.get_tutorial_task_message();
        if !tutorial_message.is_empty() {
            let scale = ScaledMeasurements::new();
            // Draw tutorial background
            let tutorial_y = scale.padding + scale_size(70.0);
            let lines: Vec<&str> = tutorial_message.lines().collect();
            let line_height = scale_size(16.0);
            let height = (lines.len() as f32 * line_height) + scale_size(20.0);
            
            draw_rectangle(scale.padding - scale_size(10.0), tutorial_y - scale_size(10.0), scale_size(500.0), height, Color::new(0.0, 0.2, 0.4, 0.9));
            draw_rectangle_lines(scale.padding - scale_size(10.0), tutorial_y - scale_size(10.0), scale_size(500.0), height, scale_size(2.0), SKYBLUE);
            
            // Draw tutorial text
            for (i, line) in lines.iter().enumerate() {
                let color = if line.starts_with("Task") { YELLOW } else { WHITE };
                draw_scaled_text(line, scale.padding, tutorial_y + (i as f32 * line_height), 14.0, color);
            }
        }
    }
}

pub fn draw_time_slow_indicator(game: &Game) {
    // Draw time slow indicator
    if game.time_slow_active {
        let scale = ScaledMeasurements::new();
        let rect_width = scale_size(180.0);
        let rect_height = scale_size(30.0);
        draw_rectangle(screen_width() - scale_size(200.0), scale.padding, rect_width, rect_height, Color::new(0.0, 0.0, 0.5, 0.8));
        draw_rectangle_lines(screen_width() - scale_size(200.0), scale.padding, rect_width, rect_height, scale_size(2.0), YELLOW);
        draw_scaled_text("TIME SLOW ACTIVE", screen_width() - scale_size(190.0), scale.padding + scale_size(20.0), 16.0, YELLOW);
    }
}

pub fn draw_level_complete_overlay(game: &Game) {
    if game.finished {
        let msg = "Level complete! Press N for next level.";
        let font_size = 28.0;
        let scaled_font_size = scale_font_size(font_size);
        let dim = measure_text(msg, None, scaled_font_size as u16, 1.0);
        let rect_padding = scale_size(40.0);
        let rect_height = scale_size(60.0);
        draw_rectangle(
            (screen_width()-dim.width-rect_padding)*0.5, (screen_height()-rect_height)*0.5, dim.width+rect_padding, rect_height,
            Color::new(0.0,0.0,0.0,0.6)
        );
        draw_scaled_text(msg, (screen_width()-dim.width)*0.5, (screen_height()+scale_size(10.0))*0.5, font_size, YELLOW);
    }
}