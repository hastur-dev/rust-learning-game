use macroquad::prelude::*;
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
                "🎉 Congratulations! 🎉\n\nYou completed: {}\n\nAchievement: {}\n\nNext up: {}\n\nPress SPACE to continue to the next level or ESC to stay here.",
                level_name, achievement, hint
            )
        } else {
            format!(
                "🎉 Congratulations! 🎉\n\nYou completed: {}\n\nAchievement: {}\n\nPress SPACE to continue to the next level or ESC to stay here.",
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
        self.show_message(
            "📝 Program Output".to_string(),
            message,
            PopupType::Stdout,
            Some(4.0) // Auto-close after 4 seconds
        );
    }
    
    pub fn show_eprintln_output(&mut self, message: String) {
        self.show_message(
            "🔴 Error Output".to_string(),
            message,
            PopupType::Stderr,
            Some(5.0) // Auto-close after 5 seconds for errors
        );
    }
    
    pub fn show_panic_output(&mut self, message: String) {
        self.show_message(
            "💥 PANIC".to_string(),
            format!("Program terminated: {}", message),
            PopupType::Panic,
            None // Manual close for panics
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
            }
            
            // Check for mouse click to dismiss
            if is_mouse_button_pressed(MouseButton::Left) {
                let screen_width = screen_width();
                let screen_height = screen_height();
                let popup_width = (screen_width * 0.6).min(600.0);
                let popup_height = (screen_height * 0.4).min(300.0);
                let popup_x = (screen_width - popup_width) / 2.0;
                let popup_y = (screen_height - popup_height) / 2.0;
                
                let (mouse_x, mouse_y) = mouse_position();
                
                // Check if click is outside popup area
                if mouse_x < popup_x || mouse_x > popup_x + popup_width ||
                   mouse_y < popup_y || mouse_y > popup_y + popup_height {
                    self.close();
                    return PopupAction::Dismissed;
                }
                // Always consume mouse click when popup is showing, regardless of where clicked
                return PopupAction::None;
            }
            
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
        
        // Popup dimensions
        let popup_width = (screen_width * 0.6).min(600.0);
        let popup_height = (screen_height * 0.4).min(300.0);
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
        };
        
        // Draw popup background
        draw_rectangle(popup_x, popup_y, popup_width, popup_height, bg_color);
        
        // Draw border
        draw_rectangle_lines(popup_x, popup_y, popup_width, popup_height, 3.0, border_color);
        
        // Draw title
        let title_size = 28.0;
        let title_metrics = measure_text(&popup.title, None, title_size as u16, 1.0);
        let title_x = popup_x + (popup_width - title_metrics.width) / 2.0;
        let title_y = popup_y + 40.0;
        draw_text(&popup.title, title_x, title_y, title_size, title_color);
        
        // Draw content
        let content_size = 20.0;
        let content_margin = 20.0;
        let content_x = popup_x + content_margin;
        let content_y = title_y + 50.0;
        let content_width = popup_width - (content_margin * 2.0);
        
        // Word wrap the content
        let wrapped_lines = wrap_text(&popup.content, content_width, content_size);
        let line_height = content_size + 5.0;
        
        for (i, line) in wrapped_lines.iter().enumerate() {
            let line_y = content_y + (i as f32 * line_height);
            draw_text(line, content_x, line_y, content_size, WHITE);
        }
        
        // Draw instructions at bottom
        let instruction_text = if self.auto_close_duration.is_some() {
            format!("Auto-closing in {:.1}s | Press any key to dismiss", 
                   self.auto_close_duration.unwrap() - self.popup_timer)
        } else {
            "Press SPACE, ENTER, ESC, or click outside to dismiss".to_string()
        };
        
        let instruction_size = 16.0;
        let instruction_metrics = measure_text(&instruction_text, None, instruction_size as u16, 1.0);
        let instruction_x = popup_x + (popup_width - instruction_metrics.width) / 2.0;
        let instruction_y = popup_y + popup_height - 25.0;
        draw_text(&instruction_text, instruction_x, instruction_y, instruction_size, LIGHTGRAY);
    }
}

// Helper function to wrap text
fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
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
    
    lines
}

impl Default for PopupSystem {
    fn default() -> Self {
        Self::new()
    }
}