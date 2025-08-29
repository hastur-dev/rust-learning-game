use crate::level::YamlLevelConfig;

// Embedded level data - these levels will always be available in the executable
pub const EMBEDDED_LEVELS: &[&str] = &[
    include_str!("../levels/01_explore_grid.yaml"),
    include_str!("../levels/02_find_scanner.yaml"),
    include_str!("../levels/03_blockers.yaml"),
    include_str!("../levels/04_moving_enemies.yaml"),
    include_str!("../levels/time_slow_demo.yaml"),
    include_str!("../levels/treasure_hunt.yaml"),
];

// Level names in order (matching order.txt)
pub const EMBEDDED_LEVEL_ORDER: &[&str] = &[
    "01_explore_grid",
    "02_find_scanner", 
    "03_blockers",
    "04_moving_enemies",
    "time_slow_demo",
    "treasure_hunt",
];

// Embedded item files
pub const EMBEDDED_SCANNER_RS: &str = include_str!("../items/scanner.rs");
pub const EMBEDDED_CREDIT_GEM_RS: &str = include_str!("../items/credit_gem.rs");
pub const EMBEDDED_GOLDEN_GEM_RS: &str = include_str!("../items/golden_gem.rs");
pub const EMBEDDED_SILVER_COIN_RS: &str = include_str!("../items/silver_coin.rs");
pub const EMBEDDED_SPEED_BOOST_RS: &str = include_str!("../items/speed_boost.rs");
pub const EMBEDDED_TIME_SLOW_RS: &str = include_str!("../items/time_slow.rs");
pub const EMBEDDED_GRABBER_UPGRADE_RS: &str = include_str!("../items/grabber_upgrade.rs");

/// Load embedded levels as YamlLevelConfig structs
pub fn load_embedded_levels() -> Result<Vec<YamlLevelConfig>, Box<dyn std::error::Error>> {
    let mut levels = Vec::new();
    
    for level_str in EMBEDDED_LEVELS {
        let config: YamlLevelConfig = serde_yaml::from_str(level_str)?;
        levels.push(config);
    }
    
    Ok(levels)
}

/// Get embedded item content by file path
pub fn get_embedded_item_content(file_path: &str) -> Option<&'static str> {
    match file_path {
        "items/scanner.rs" => Some(EMBEDDED_SCANNER_RS),
        "items/credit_gem.rs" => Some(EMBEDDED_CREDIT_GEM_RS),
        "items/golden_gem.rs" => Some(EMBEDDED_GOLDEN_GEM_RS),
        "items/silver_coin.rs" => Some(EMBEDDED_SILVER_COIN_RS),
        "items/speed_boost.rs" => Some(EMBEDDED_SPEED_BOOST_RS),
        "items/time_slow.rs" => Some(EMBEDDED_TIME_SLOW_RS),
        "items/grabber_upgrade.rs" => Some(EMBEDDED_GRABBER_UPGRADE_RS),
        _ => None,
    }
}