// CAPABILITY: time_slow_duration = 500
// CAPABILITY: credits_value = 25

// Time Slow item - slows down robot execution for better visualization
// Duration is in milliseconds between each action

pub fn activate_time_slow() {
    // This function would be called to activate time slow mode
    // The actual implementation is handled by the game engine
}

pub fn get_slow_duration_ms() -> u32 {
    500 // Default 500ms delay between actions
}

pub fn is_time_modifier() -> bool {
    true
}

// Different time slow variations can be created by adjusting the duration
pub fn create_ultra_slow() -> u32 {
    1000 // 1 second delay for very slow execution
}

pub fn create_medium_slow() -> u32 {
    300 // 300ms delay for moderate slowdown
}

pub fn create_fast_slow() -> u32 {
    100 // 100ms delay for slight slowdown
}