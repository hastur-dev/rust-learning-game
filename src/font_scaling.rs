use macroquad::prelude::*;
use std::sync::{Mutex, OnceLock};

/// Font scaling system for responsive text based on display size
/// This ensures text appears at consistent physical sizes across different displays

// Base reference display size (1920x1080 as standard)
const BASE_DISPLAY_WIDTH: f32 = 1920.0;
const BASE_DISPLAY_HEIGHT: f32 = 1080.0;

// Minimum and maximum scaling factors for safety
const MIN_SCALE_FACTOR: f32 = 0.5;
const MAX_SCALE_FACTOR: f32 = 3.0;

// Global font size multiplier set from user settings
static USER_FONT_MULTIPLIER: OnceLock<Mutex<f32>> = OnceLock::new();

/// Set the global user font size multiplier
pub fn set_user_font_multiplier(multiplier: f32) {
    let mutex = USER_FONT_MULTIPLIER.get_or_init(|| Mutex::new(1.0));
    if let Ok(mut value) = mutex.lock() {
        *value = multiplier.clamp(0.5, 2.0);
    }
}

/// Get the current user font size multiplier
pub fn get_user_font_multiplier() -> f32 {
    let mutex = USER_FONT_MULTIPLIER.get_or_init(|| Mutex::new(1.0));
    match mutex.lock() {
        Ok(value) => *value,
        Err(_) => 1.0, // Default value on error
    }
}

/// Calculate display scaling factor based on current screen size
pub fn get_display_scale_factor() -> f32 {
    get_display_scale_factor_with_multiplier(1.0)
}

/// Calculate display scaling factor with user font size multiplier
pub fn get_display_scale_factor_with_multiplier(font_size_multiplier: f32) -> f32 {
    let current_width = screen_width();
    let current_height = screen_height();
    
    // Calculate scale based on both width and height, take the smaller for better fit
    let width_scale = current_width / BASE_DISPLAY_WIDTH;
    let height_scale = current_height / BASE_DISPLAY_HEIGHT;
    
    // Use the average of width and height scaling for balanced scaling
    let base_scale_factor = (width_scale + height_scale) / 2.0;
    
    // Apply user font size multiplier
    let final_scale_factor = base_scale_factor * font_size_multiplier;
    
    // Clamp to safe bounds
    final_scale_factor.clamp(MIN_SCALE_FACTOR, MAX_SCALE_FACTOR)
}

/// Scale a font size based on display size (uses global user font multiplier)
pub fn scale_font_size(base_font_size: f32) -> f32 {
    let user_multiplier = get_user_font_multiplier();
    scale_font_size_with_multiplier(base_font_size, user_multiplier)
}

/// Scale a font size with user font size multiplier
pub fn scale_font_size_with_multiplier(base_font_size: f32, font_size_multiplier: f32) -> f32 {
    let scale_factor = get_display_scale_factor_with_multiplier(font_size_multiplier);
    let scaled_size = base_font_size * scale_factor;
    
    // Ensure minimum readable font size
    scaled_size.max(8.0)
}

/// Convenience function for drawing scaled text (uses global user font multiplier)
pub fn draw_scaled_text(text: &str, x: f32, y: f32, base_font_size: f32, color: Color) {
    let user_multiplier = get_user_font_multiplier();
    draw_scaled_text_with_multiplier(text, x, y, base_font_size, color, user_multiplier);
}

/// Convenience function for drawing scaled text with user font size multiplier
pub fn draw_scaled_text_with_multiplier(text: &str, x: f32, y: f32, base_font_size: f32, color: Color, font_size_multiplier: f32) {
    let scaled_font_size = scale_font_size_with_multiplier(base_font_size, font_size_multiplier);
    draw_text(text, x, y, scaled_font_size, color);
}

/// Scale a position/size value proportionally to the display (uses global user font multiplier)
pub fn scale_size(base_size: f32) -> f32 {
    let user_multiplier = get_user_font_multiplier();
    scale_size_with_multiplier(base_size, user_multiplier)
}

/// Scale a position/size value with user font size multiplier
pub fn scale_size_with_multiplier(base_size: f32, font_size_multiplier: f32) -> f32 {
    base_size * get_display_scale_factor_with_multiplier(font_size_multiplier)
}

/// Get scaled measurements for UI layout
pub struct ScaledMeasurements {
    pub small_font: f32,    // 12.0 base
    pub medium_font: f32,   // 16.0 base  
    pub large_font: f32,    // 20.0 base
    pub title_font: f32,    // 24.0 base
    pub padding: f32,       // 10.0 base
    pub line_height: f32,   // 20.0 base
    pub button_height: f32, // 25.0 base
}

impl ScaledMeasurements {
    pub fn new() -> Self {
        let user_multiplier = get_user_font_multiplier();
        Self::new_with_multiplier(user_multiplier)
    }
    
    pub fn new_with_multiplier(font_size_multiplier: f32) -> Self {
        Self {
            small_font: scale_font_size_with_multiplier(12.0, font_size_multiplier),
            medium_font: scale_font_size_with_multiplier(16.0, font_size_multiplier),
            large_font: scale_font_size_with_multiplier(20.0, font_size_multiplier),
            title_font: scale_font_size_with_multiplier(24.0, font_size_multiplier),
            padding: scale_size_with_multiplier(10.0, font_size_multiplier),
            line_height: scale_size_with_multiplier(20.0, font_size_multiplier),
            button_height: scale_size_with_multiplier(25.0, font_size_multiplier),
        }
    }
}

/// Debug function to display current scaling info
#[allow(dead_code)]
pub fn debug_display_scaling() -> String {
    let scale = get_display_scale_factor();
    let measurements = ScaledMeasurements::new();
    
    format!(
        "Display: {}x{}, Scale: {:.2}, Fonts: S{:.1} M{:.1} L{:.1} T{:.1}",
        screen_width(),
        screen_height(),
        scale,
        measurements.small_font,
        measurements.medium_font,
        measurements.large_font,
        measurements.title_font
    )
}