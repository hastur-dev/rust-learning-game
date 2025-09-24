// Automated Level Testing Module
// Contains simplified test configurations for each learning level

pub mod level_1;
pub mod level_2;
pub mod level_3;
pub mod level_4;
pub mod level_5;
pub mod level_6;

// Re-export the main configuration types for easier access
pub use level_1::{LevelTestConfig, TaskTest};

// Convenience function to get all level tests
pub fn get_all_level_tests() -> Vec<LevelTestConfig> {
    vec![
        level_1::get_level_1_tests(),
        level_2::get_level_2_tests(),
        level_3::get_level_3_tests(),
        level_4::get_level_4_tests(),
        level_5::get_level_5_tests(),
        level_6::get_level_6_tests(),
    ]
}

// Get a specific level's tests by index
pub fn get_level_tests(level_index: usize) -> Option<LevelTestConfig> {
    match level_index {
        0 => Some(level_1::get_level_1_tests()),
        1 => Some(level_2::get_level_2_tests()),
        2 => Some(level_3::get_level_3_tests()),
        3 => Some(level_4::get_level_4_tests()),
        4 => Some(level_5::get_level_5_tests()),
        5 => Some(level_6::get_level_6_tests()),
        _ => None,
    }
}