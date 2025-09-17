use macroquad::prelude::*;
use crate::font_scaling::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PopupMessage {
    pub title: String,
    pub content: String,
    pub popup_type: PopupType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PopupType {
    Info,
    Warning,
    Success,
    Tutorial,
    Stdout,  // For println! output
    Stderr,  // For eprintln! and error output
    Panic,   // For panic! output
    Congratulations, // For level completion
    FunctionResults, // For robot function execution results
}

#[derive(Clone, Debug, PartialEq)]
pub enum PopupAction {
    None,
    Dismissed,
    NextLevel,
    StayOnLevel,
}

#[derive(Clone, Debug)]
pub struct PopupSystem {
    pub current_popup: Option<PopupMessage>,
    pub show_popup: bool,
    pub popup_timer: f32,
    pub auto_close_duration: Option<f32>, // None = manual close only
}

impl PopupSystem {
    pub fn new() -> Self {
        Self {
            current_popup: None,
            show_popup: false,
            popup_timer: 0.0,
            auto_close_duration: None,
        }
    }
    
    pub fn show_message(&mut self, title: String, content: String, popup_type: PopupType, auto_close_seconds: Option<f32>) {
        self.current_popup = Some(PopupMessage {
            title,
            content,
            popup_type,
        });
        self.show_popup = true;
        self.popup_timer = 0.0;
        self.auto_close_duration = auto_close_seconds;
    }
    
    pub fn show_level_message(&mut self, content: String) {
        self.show_message(
            "Level Information".to_string(),
            content,
            PopupType::Info,
            None // Manual close only for level messages
        );
    }
    
    pub fn show_tutorial(&mut self, content: String) {
        self.show_message(
            "Tutorial".to_string(),
            content,
            PopupType::Tutorial,
            None
        );
    }
    
    pub fn show_item_collected(&mut self, item_name: String) {
        self.show_message(
            "Item Collected!".to_string(),
            format!("You found: {}", item_name),
            PopupType::Success,
            Some(3.0) // Auto-close after 3 seconds
        );
    }
    
    pub fn show_level_complete(&mut self) {
        self.show_message(
            "Level Complete!".to_string(),
            "Great job! Press SPACE to continue to the next level.".to_string(),
            PopupType::Success,
            None
        );
    }
    
    pub fn show_congratulations(&mut self, level_name: String, achievement: String, next_level_hint: Option<String>) {
        let content = if let Some(hint) = next_level_hint {
            format!(
                "🎉 Congratulations! 🎉\n\nYou completed: {}\n\nAchievement: {}\n\nNext up: {}\n\nPress CTRL+SHIFT+N to continue to the next level or ESC to stay here.",
                level_name, achievement, hint
            )
        } else {
            format!(
                "🎉 Congratulations! 🎉\n\nYou completed: {}\n\nAchievement: {}\n\nPress CTRL+SHIFT+N to continue to the next level or ESC to stay here.",
                level_name, achievement
            )
        };
        
        self.show_message(
            "🏆 Level Complete!".to_string(),
            content,
            PopupType::Congratulations,
            None // Manual close only for congratulations
        );
    }
    
    pub fn show_completion_instructions(&mut self, level_name: String, instructions: String) {
        let content = format!(
            "🎯 How to Complete This Level\n\nLevel: {}\n\n📝 Instructions:\n{}\n\n💡 Tip: Press Ctrl+Shift+C anytime to see these instructions again!\n\nPress any key to close this help.",
            level_name, instructions
        );
        
        self.show_message(
            "🚀 Level Instructions".to_string(),
            content,
            PopupType::Info,
            None // Manual close only
        );
    }
    
    pub fn show_println_output(&mut self, message: String) {
        // Check if we already have a stdout popup and stack the messages
        if let Some(ref mut current) = self.current_popup {
            if matches!(current.popup_type, PopupType::Stdout) {
                // Stack the new message with the existing one
                current.content = format!("{}\n{}", current.content, message);
                self.popup_timer = 0.0; // Reset timer for new message
                return;
            }
        }
        
        // Create new stdout popup
        self.show_message(
            "📝 Program Output".to_string(),
            message,
            PopupType::Stdout,
            None // Consider this for auto close by putting in Some(#.#) that will set a timer. Right now it's not needed.
        );
    }
    
    pub fn show_eprintln_output(&mut self, message: String) {
        // Check if we already have a stderr popup and stack the messages
        if let Some(ref mut current) = self.current_popup {
            if matches!(current.popup_type, PopupType::Stderr) {
                // Stack the new message with the existing one
                current.content = format!("{}\n{}", current.content, message);
                self.popup_timer = 0.0; // Reset timer for new message
                return;
            }
        }
        
        // Create new stderr popup
        self.show_message(
            "🔴 Error Output".to_string(),
            message,
            PopupType::Stderr,
            None // Consider this for auto close by putting in Some(#.#) that will set a timer. Right now it's not needed.
        );
    }
    
    pub fn show_panic_output(&mut self, message: String) {
        // Check if we already have a panic popup and stack the messages
        if let Some(ref mut current) = self.current_popup {
            if matches!(current.popup_type, PopupType::Panic) {
                // Stack the new panic message with the existing one
                let formatted_message = format!("Program terminated: {}", message);
                current.content = format!("{}\n{}", current.content, formatted_message);
                self.popup_timer = 0.0; // Reset timer for new message
                return;
            }
        }
        
        // Create new panic popup
        self.show_message(
            "💥 PANIC".to_string(),
            format!("Program terminated: {}", message),
            PopupType::Panic,
            None // Manual close for panics
        );
    }
    
    pub fn show_function_results(&mut self, results: Vec<String>) {
        if results.is_empty() {
            return;
        }
        
        // Filter out empty or generic results
        let meaningful_results: Vec<String> = results
            .into_iter()
            .filter(|r| !r.is_empty() && 
                        !r.contains("executed") && 
                        !r.contains("Print functions handled separately") &&
                        r != "No valid function calls found")
            .collect();
        
        if meaningful_results.is_empty() {
            return;
        }
        
        // Check if we already have a function results popup and stack the messages
        if let Some(ref mut current) = self.current_popup {
            if matches!(current.popup_type, PopupType::FunctionResults) {
                // Stack the new results with the existing ones
                let new_content = meaningful_results.join("\n");
                current.content = format!("{}\n{}", current.content, new_content);
                self.popup_timer = 0.0; // Reset timer for new message
                return;
            }
        }
        
        // Create new function results popup
        let content = meaningful_results.join("\n");
        self.show_message(
            "🤖 Robot Action Results".to_string(),
            content,
            PopupType::FunctionResults,
            Some(4.0) // Auto-close after 4 seconds for function results
        );
    }
    
    pub fn update(&mut self, delta_time: f32) {
        if self.show_popup {
            self.popup_timer += delta_time;
            
            // Auto-close if duration is set
            if let Some(duration) = self.auto_close_duration {
                if self.popup_timer >= duration {
                    self.close();
                }
            }
        }
    }
    
    pub fn handle_input(&mut self) -> PopupAction {
        if self.show_popup {
            if let Some(ref popup) = self.current_popup {
                match popup.popup_type {
                    PopupType::Congratulations => {
                        // Special handling for congratulations popup
                        if is_key_pressed(KeyCode::Space) {
                            self.close();
                            return PopupAction::NextLevel;
                        } else if is_key_pressed(KeyCode::Escape) {
                            self.close();
                            return PopupAction::StayOnLevel;
                        }
                    },
                    _ => {
                        // Normal popup handling
                        if is_key_pressed(KeyCode::Space) || 
                           is_key_pressed(KeyCode::Enter) || 
                           is_key_pressed(KeyCode::Escape) {
                            self.close();
                            return PopupAction::Dismissed;
                        }
                    }
                }
                
                // Check for mouse click to dismiss
                if is_mouse_button_pressed(MouseButton::Left) {
                    let screen_width = screen_width();
                    let screen_height = screen_height();
                    let (popup_width, popup_height) = calculate_popup_dimensions(
                        &popup.title, 
                        &popup.content, 
                        screen_width, 
                        screen_height
                    );
                    let popup_x = (screen_width - popup_width) / 2.0;
                    let popup_y = (screen_height - popup_height) / 2.0;
                    
                    let (mouse_x, mouse_y) = mouse_position();
                    
                    // Check if click is outside popup area
                    if mouse_x < popup_x || mouse_x > popup_x + popup_width ||
                       mouse_y < popup_y || mouse_y > popup_y + popup_height {
                        self.close();
                        return PopupAction::Dismissed;
                    } else {
                        // Click is inside popup area - dismiss the popup
                        self.close();
                        return PopupAction::Dismissed;
                    }
                }
            } // End of if let Some(ref popup) = self.current_popup
            
            return PopupAction::None; // Popup is showing, consume all input
        }
        
        PopupAction::None // No popup, don't consume input
    }
    
    pub fn close(&mut self) {
        self.show_popup = false;
        self.current_popup = None;
        self.popup_timer = 0.0;
        self.auto_close_duration = None;
    }
    
    pub fn is_showing(&self) -> bool {
        self.show_popup
    }
    
    pub fn draw(&self) {
        if !self.show_popup {
            return;
        }
        
        let Some(ref popup) = self.current_popup else {
            return;
        };
        
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        // Semi-transparent overlay
        draw_rectangle(0.0, 0.0, screen_width, screen_height, Color::new(0.0, 0.0, 0.0, 0.5));
        
        // Calculate dynamic popup dimensions based on content
        let (popup_width, popup_height) = calculate_popup_dimensions(
            &popup.title, 
            &popup.content, 
            screen_width, 
            screen_height
        );
        let popup_x = (screen_width - popup_width) / 2.0;
        let popup_y = (screen_height - popup_height) / 2.0;
        
        // Get colors based on popup type
        let (bg_color, border_color, title_color) = match popup.popup_type {
            PopupType::Info => (Color::new(0.2, 0.2, 0.3, 0.95), LIGHTGRAY, BLUE),
            PopupType::Warning => (Color::new(0.3, 0.2, 0.1, 0.95), ORANGE, YELLOW),
            PopupType::Success => (Color::new(0.1, 0.3, 0.1, 0.95), LIGHTGRAY, GREEN),
            PopupType::Stdout => (Color::new(0.1, 0.2, 0.3, 0.95), SKYBLUE, WHITE),
            PopupType::Stderr => (Color::new(0.3, 0.1, 0.1, 0.95), RED, YELLOW),
            PopupType::Panic => (Color::new(0.4, 0.1, 0.1, 0.95), RED, ORANGE),
            PopupType::Tutorial => (Color::new(0.25, 0.15, 0.3, 0.95), PURPLE, PINK),
            PopupType::Congratulations => (Color::new(0.1, 0.3, 0.1, 0.95), GOLD, YELLOW),
            PopupType::FunctionResults => (Color::new(0.15, 0.25, 0.15, 0.95), GREEN, LIME),
        };
        
        let scale = ScaledMeasurements::new();
        
        // Draw popup background
        draw_rectangle(popup_x, popup_y, popup_width, popup_height, bg_color);
        
        // Draw border
        draw_rectangle_lines(popup_x, popup_y, popup_width, popup_height, scale_size(3.0), border_color);
        
        // Draw title
        let title_size = 28.0;
        let scaled_title_size = scale_font_size(title_size);
        let title_metrics = measure_text(&popup.title, None, scaled_title_size as u16, 1.0);
        let title_x = popup_x + (popup_width - title_metrics.width) / 2.0;
        let title_y = popup_y + scale_size(40.0);
        draw_scaled_text(&popup.title, title_x, title_y, title_size, title_color);
        
        // Draw content
        let content_size = 20.0;
        let content_margin = scale_size(20.0);
        let content_x = popup_x + content_margin;
        let content_y = title_y + scale_size(50.0);
        let content_width = popup_width - (content_margin * 2.0);
        
        // Word wrap the content
        let wrapped_lines = wrap_text(&popup.content, content_width, scale_font_size(content_size));
        let line_height = scale_font_size(content_size) + scale_size(5.0);
        
        for (i, line) in wrapped_lines.iter().enumerate() {
            let line_y = content_y + (i as f32 * line_height);
            draw_scaled_text(line, content_x, line_y, content_size, WHITE);
        }
        
        // Draw instructions at bottom
        let instruction_text = if self.auto_close_duration.is_some() {
            format!("Auto-closing in {:.1}s | Press any key to dismiss", 
                   self.auto_close_duration.unwrap() - self.popup_timer)
        } else {
            "Press SPACE, ENTER, ESC, or click outside to dismiss".to_string()
        };
        
        let instruction_size = 16.0;
        let scaled_instruction_size = scale_font_size(instruction_size);
        let instruction_metrics = measure_text(&instruction_text, None, scaled_instruction_size as u16, 1.0);
        let instruction_x = popup_x + (popup_width - instruction_metrics.width) / 2.0;
        let instruction_y = popup_y + popup_height - scale_size(25.0);
        draw_scaled_text(&instruction_text, instruction_x, instruction_y, instruction_size, LIGHTGRAY);
    }
}

// Helper function to wrap text, respecting explicit newlines
fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let mut lines = Vec::new();
    
    // First split by explicit newlines
    for paragraph in text.split('\n') {
        if paragraph.trim().is_empty() {
            lines.push(String::new()); // Preserve empty lines
            continue;
        }
        
        // Then wrap each paragraph
        let words: Vec<&str> = paragraph.split_whitespace().collect();
        let mut current_line = String::new();
        
        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };
            
            let test_width = measure_text(&test_line, None, font_size as u16, 1.0).width;
            
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
    }
    
    lines
}

// Calculate optimal popup dimensions based on content
fn calculate_popup_dimensions(title: &str, content: &str, screen_width: f32, screen_height: f32) -> (f32, f32) {
    let min_width = scale_size(400.0);
    let max_width = screen_width * 0.85; // Maximum 85% of screen width
    let min_height = scale_size(200.0);
    let max_height = screen_height * 0.85; // Maximum 85% of screen height
    
    // Calculate title dimensions
    let title_size = 28.0;
    let scaled_title_size = scale_font_size(title_size);
    let title_metrics = measure_text(title, None, scaled_title_size as u16, 1.0);
    let title_width = title_metrics.width;
    
    // Calculate content dimensions with progressive width testing
    let content_size = 20.0;
    let content_margin = scale_size(20.0);
    let line_height = scale_font_size(content_size) + scale_size(5.0);
    
    // Start with a reasonable width and expand if needed
    let mut test_width = (min_width).max(title_width + scale_size(80.0)); // Title + padding
    let mut final_width = test_width;
    let mut final_height = min_height;
    
    // Test different widths to find optimal layout
    for width_factor in [0.6, 0.7, 0.8, 0.85] {
        test_width = (screen_width * width_factor).min(max_width);
        let content_width = test_width - (content_margin * 2.0);
        
        if content_width > scale_size(300.0) { // Minimum reasonable content width
            let wrapped_lines = wrap_text(content, content_width, scale_font_size(content_size));
            
            // Calculate required height
            let title_area_height = scale_size(90.0); // Title + spacing
            let content_area_height = wrapped_lines.len() as f32 * line_height;
            let instruction_area_height = scale_size(60.0); // Instructions + spacing
            
            let required_height = title_area_height + content_area_height + instruction_area_height;
            
            // If this layout fits within screen bounds, use it
            if required_height <= max_height {
                final_width = test_width;
                final_height = required_height.max(min_height);
                break;
            }
        }
    }
    
    // Ensure minimum dimensions
    final_width = final_width.max(min_width);
    final_height = final_height.max(min_height);
    
    // Ensure maximum dimensions
    final_width = final_width.min(max_width);
    final_height = final_height.min(max_height);
    
    (final_width, final_height)
}

impl Default for PopupSystem {
    fn default() -> Self {
        Self::new()
    }
}