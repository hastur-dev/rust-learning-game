use macroquad::prelude::*;
use crate::gamestate::Game;
use crate::item::Pos;
use crate::level::EnemyDirection;
use crate::font_scaling::*;

const TILE: f32 = 42.0;

// Helper function to wrap text with dynamic font sizing and scrolling support
fn calculate_wrapped_text_dimensions(text: &str, initial_font_size: f32, max_width: f32, max_height: f32) -> (Vec<String>, f32, f32, f32) {
    let mut font_size = initial_font_size;
    let min_font_size = 10.0; // Don't go below readable size
    
    loop {
        let mut wrapped_lines = Vec::new();
        let mut max_line_width = 0.0f32;
        
        // Calculate line height for current font size
        let line_height = font_size * 1.2; // Standard line spacing
        
        for line in text.lines() {
            if line.trim().is_empty() {
                wrapped_lines.push("".to_string());
                continue;
            }
            
            let line_width = measure_scaled_text(line, font_size).width;
            if line_width <= max_width {
                wrapped_lines.push(line.to_string());
                max_line_width = max_line_width.max(line_width);
            } else {
                // Need to wrap this line
                let words: Vec<&str> = line.split_whitespace().collect();
                let mut current_line = String::new();
                let mut current_width = 0.0;
                
                for word in words {
                    let word_width = measure_scaled_text(&format!("{} ", word), font_size).width;
                    
                    if current_line.is_empty() {
                        // First word on line
                        current_line = word.to_string();
                        current_width = word_width;
                    } else if current_width + word_width <= max_width {
                        // Add word to current line
                        current_line.push(' ');
                        current_line.push_str(word);
                        current_width += word_width;
                    } else {
                        // Start new line
                        max_line_width = max_line_width.max(current_width);
                        wrapped_lines.push(current_line);
                        current_line = word.to_string();
                        current_width = word_width;
                    }
                }
                
                if !current_line.is_empty() {
                    max_line_width = max_line_width.max(current_width);
                    wrapped_lines.push(current_line);
                }
            }
        }
        
        let total_height = wrapped_lines.len() as f32 * line_height;
        
        // If it fits within max height or we're at minimum font size, use this
        if total_height <= max_height || font_size <= min_font_size {
            return (wrapped_lines, max_line_width, total_height, font_size);
        }
        
        // Reduce font size and try again
        font_size -= 1.0;
    }
}

pub fn grid_origin(g: &Game) -> (f32, f32) {
    let gw = g.grid.width as f32 * TILE;
    let gh = g.grid.height as f32 * TILE;
    
    // Calculate available width dynamically based on single sidebar layout
    // We now have only the tabbed sidebar (Commands/Logs/Tasks/Editor)
    let sidebar_width = crate::crash_protection::safe_screen_width() * 0.25; // Tabbed sidebar on the right
    let padding = scale_size(10.0);
    
    // Available width is screen minus sidebar with padding
    let available_width = crate::crash_protection::safe_screen_width() - sidebar_width - (padding * 2.0);
    
    // Center the grid in the available space (left side of screen)
    let ox = (available_width - gw) * 0.5 + padding;
    
    // Center the grid vertically with some space for header
    let header_height = scale_size(100.0); // Space for game info at top
    let available_height = crate::crash_protection::safe_screen_height() - header_height - padding;
    let oy = header_height + (available_height - gh) * 0.5; // Center vertically
    
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

            // Draw enemies (including special robots for learning levels)
            let should_show_enemies = known || game.should_show_special_robots_at(p);

            if should_show_enemies {
                for enemy in &game.grid.enemies {
                    if enemy.pos == p {
                        // Special rendering based on learning level and robot type
                        let (txt, font_size) = game.get_robot_symbol_for_level(enemy);

                        let scaled_font_size = scale_font_size(font_size);
                        let dim = measure_text(txt, None, scaled_font_size as u16, 1.0);

                        // Determine enemy color based on level and robot type
                        let enemy_color = game.get_robot_color_for_level(enemy);

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
            
            // Calculate available space (avoid interfering with the grid)
            let (grid_x, _) = grid_origin(game);
            let max_task_box_width = (grid_x - scale.padding * 3.0).max(scale_size(300.0)); // At least 300px wide, but don't go into grid
            
            // Calculate max height (stop at bottom of screen with some padding)
            let tutorial_start_y = scale.padding + scale_size(70.0);
            let max_task_box_height = (crate::crash_protection::safe_screen_height() - tutorial_start_y - scale_size(50.0)).max(scale_size(200.0)); // At least 200px tall
            
            // Calculate wrapped text dimensions with dynamic font sizing and height constraints
            let initial_font_size = 14.0;
            let (wrapped_lines, actual_width, text_height, final_font_size) = calculate_wrapped_text_dimensions(
                &tutorial_message, 
                initial_font_size, 
                max_task_box_width, 
                max_task_box_height - scale_size(20.0) // Account for padding
            );
            
            // Position and size the task box
            let tutorial_x = scale.padding;
            let tutorial_y = tutorial_start_y;
            let box_width = (actual_width + scale_size(20.0)).max(scale_size(250.0)); // Add padding, ensure minimum width
            let actual_box_height = (text_height + scale_size(20.0)).min(max_task_box_height); // Don't exceed max height
            let needs_scrolling = text_height + scale_size(20.0) > max_task_box_height;
            
            // Draw background (expands downward only)
            draw_rectangle(tutorial_x - scale_size(10.0), tutorial_y - scale_size(10.0), box_width, actual_box_height, Color::new(0.0, 0.2, 0.4, 0.9));
            draw_rectangle_lines(tutorial_x - scale_size(10.0), tutorial_y - scale_size(10.0), box_width, actual_box_height, scale_size(2.0), SKYBLUE);
            
            // Draw scrolling indicator if needed
            if needs_scrolling {
                draw_scaled_text("↑↓ Arrow Keys | PgUp/PgDn", tutorial_x + box_width - scale_size(150.0), tutorial_y - scale_size(5.0), 9.0, GRAY);
            }
            
            // Draw wrapped text with scrolling support
            let line_height = final_font_size * 1.2; // Use calculated line height
            let visible_lines = ((actual_box_height - scale_size(20.0)) / line_height) as usize;
            
            // Advanced scrolling: use tutorial_scroll_offset to show appropriate lines
            let start_line = game.tutorial_scroll_offset.min(wrapped_lines.len());
            let end_line = (start_line + visible_lines).min(wrapped_lines.len());
            let lines_to_show = end_line - start_line;

            for i in 0..lines_to_show {
                let line_index = start_line + i;
                if line_index < wrapped_lines.len() {
                    let line = &wrapped_lines[line_index];
                    let color = if line.starts_with("Task") || line.starts_with("📋") { YELLOW } else { WHITE };
                    draw_scaled_text(line, tutorial_x, tutorial_y + (i as f32 * line_height), final_font_size, color);
                }
            }
            
            // Show scroll indicators
            if needs_scrolling {
                let indicator_y = tutorial_y + actual_box_height - scale_size(30.0);
                if start_line > 0 && end_line < wrapped_lines.len() {
                    draw_scaled_text("↑ More above ↓ More below", tutorial_x, indicator_y, final_font_size * 0.8, GRAY);
                } else if start_line > 0 {
                    draw_scaled_text("↑ More above", tutorial_x, indicator_y, final_font_size * 0.8, GRAY);
                } else if end_line < wrapped_lines.len() {
                    draw_scaled_text("↓ More below", tutorial_x, indicator_y, final_font_size * 0.8, GRAY);
                }
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
        draw_rectangle(crate::crash_protection::safe_screen_width() - scale_size(200.0), scale.padding, rect_width, rect_height, Color::new(0.0, 0.0, 0.5, 0.8));
        draw_rectangle_lines(crate::crash_protection::safe_screen_width() - scale_size(200.0), scale.padding, rect_width, rect_height, scale_size(2.0), YELLOW);
        draw_scaled_text("TIME SLOW ACTIVE", crate::crash_protection::safe_screen_width() - scale_size(190.0), scale.padding + scale_size(20.0), 16.0, YELLOW);
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
            (crate::crash_protection::safe_screen_width()-dim.width-rect_padding)*0.5, (crate::crash_protection::safe_screen_height()-rect_height)*0.5, dim.width+rect_padding, rect_height,
            Color::new(0.0,0.0,0.0,0.6)
        );
        draw_scaled_text(msg, (crate::crash_protection::safe_screen_width()-dim.width)*0.5, (crate::crash_protection::safe_screen_height()+scale_size(10.0))*0.5, font_size, YELLOW);
    }
}