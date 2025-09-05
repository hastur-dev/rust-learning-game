use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::level::LevelSpec;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GameCache {
    pub compiled_levels: HashMap<String, CachedLevel>,
    pub cache_version: u32,
    pub font_metrics: HashMap<String, FontMetrics>,
    pub precompiled_assets: HashMap<String, CachedAsset>,
    pub game_settings: Option<CachedGameSettings>,
    pub startup_data: Option<StartupData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedLevel {
    pub spec: LevelSpec,
    pub checksum: String,
    pub compiled_at: u64, // Unix timestamp
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FontMetrics {
    pub font_size: f32,
    pub char_width: f32,
    pub line_height: f32,
    pub cached_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedAsset {
    pub asset_type: String,
    pub data: Vec<u8>,
    pub checksum: String,
    pub cached_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedGameSettings {
    pub window_width: i32,
    pub window_height: i32,
    pub fullscreen: bool,
    pub font_size_multiplier: f32,
    pub maximized: bool,
    pub cached_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StartupData {
    pub last_played_level: usize,
    pub total_levels_count: usize,
    pub embedded_levels_checksum: String,
    pub startup_time_ms: u64,
    pub cached_at: u64,
}

const CACHE_VERSION: u32 = 2;
const CACHE_FILE: &str = "rust_game_cache.json";

impl GameCache {
    pub fn load() -> Self {
        if let Ok(cache_data) = fs::read_to_string(CACHE_FILE) {
            if let Ok(mut cache) = serde_json::from_str::<GameCache>(&cache_data) {
                // Check cache version compatibility
                if cache.cache_version == CACHE_VERSION {
                    log::info!("Loaded game cache with {} compiled levels", cache.compiled_levels.len());
                    return cache;
                } else {
                    log::warn!("Cache version mismatch, clearing cache");
                    cache.clear();
                }
            } else {
                log::warn!("Failed to parse cache file, starting fresh");
            }
        } else {
            log::info!("No cache file found, starting fresh");
        }
        
        Self {
            cache_version: CACHE_VERSION,
            ..Default::default()
        }
    }
    
    pub fn save(&self) {
        if let Ok(cache_data) = serde_json::to_string_pretty(self) {
            if let Err(e) = fs::write(CACHE_FILE, cache_data) {
                log::error!("Failed to save cache: {}", e);
            } else {
                log::debug!("Cache saved successfully");
            }
        } else {
            log::error!("Failed to serialize cache");
        }
    }
    
    pub fn clear(&mut self) {
        self.compiled_levels.clear();
        self.font_metrics.clear();
        self.precompiled_assets.clear();
        self.game_settings = None;
        self.startup_data = None;
        log::info!("Cache cleared");
    }
    
    pub fn get_cached_level(&self, key: &str) -> Option<&CachedLevel> {
        self.compiled_levels.get(key)
    }
    
    pub fn cache_level(&mut self, key: String, level_spec: LevelSpec, checksum: String) {
        let cached_level = CachedLevel {
            spec: level_spec,
            checksum,
            compiled_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        log::debug!("Cached level: {}", key);
        self.compiled_levels.insert(key, cached_level);
    }
    
    pub fn cache_font_metrics(&mut self, key: String, metrics: FontMetrics) {
        log::debug!("Cached font metrics: {}", key);
        self.font_metrics.insert(key, metrics);
    }
    
    pub fn get_font_metrics(&self, key: &str) -> Option<&FontMetrics> {
        self.font_metrics.get(key)
    }
    
    // Calculate a simple checksum for level content
    pub fn calculate_checksum(content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    // Check if cache entry is still valid (not too old)
    pub fn is_cache_fresh(&self, cached_level: &CachedLevel, max_age_seconds: u64) -> bool {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        current_time - cached_level.compiled_at < max_age_seconds
    }

    // Asset caching methods
    pub fn cache_asset(&mut self, key: String, asset_type: String, data: Vec<u8>) {
        let checksum = Self::calculate_checksum(&String::from_utf8_lossy(&data));
        let cached_asset = CachedAsset {
            asset_type,
            data,
            checksum,
            cached_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        log::debug!("Cached asset: {}", key);
        self.precompiled_assets.insert(key, cached_asset);
    }

    pub fn get_cached_asset(&self, key: &str) -> Option<&CachedAsset> {
        self.precompiled_assets.get(key)
    }

    // Game settings caching
    pub fn cache_game_settings(&mut self, settings: CachedGameSettings) {
        log::debug!("Cached game settings");
        self.game_settings = Some(settings);
    }

    pub fn get_cached_game_settings(&self) -> Option<&CachedGameSettings> {
        self.game_settings.as_ref()
    }

    // Startup data caching
    pub fn cache_startup_data(&mut self, data: StartupData) {
        log::debug!("Cached startup data");
        self.startup_data = Some(data);
    }

    pub fn get_startup_data(&self) -> Option<&StartupData> {
        self.startup_data.as_ref()
    }

    // Generate checksum for embedded levels to detect changes
    pub fn generate_embedded_levels_checksum() -> String {
        let embedded_levels = crate::embedded_levels::get_embedded_learning_levels();
        let serialized = serde_json::to_string(&embedded_levels).unwrap_or_default();
        Self::calculate_checksum(&serialized)
    }

    // Check if cached data is still valid
    pub fn is_startup_data_fresh(&self, max_age_seconds: u64) -> bool {
        if let Some(data) = &self.startup_data {
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            current_time - data.cached_at < max_age_seconds
        } else {
            false
        }
    }

    // Validate that cached embedded levels are still current
    pub fn is_embedded_levels_cache_valid(&self) -> bool {
        if let Some(data) = &self.startup_data {
            let current_checksum = Self::generate_embedded_levels_checksum();
            data.embedded_levels_checksum == current_checksum
        } else {
            false
        }
    }

    // Pre-cache common assets to speed up initial load
    pub fn precache_common_assets(&mut self) {
        // Cache commonly used text patterns for faster parsing
        let common_patterns = [
            ("rust_keywords", "fn main use let mut if else for while loop match"),
            ("game_commands", "move_bot grab scan laser::direction laser::tile open_door"),
            ("print_patterns", "println! eprintln! panic! format!"),
        ];

        for (key, pattern) in common_patterns.iter() {
            let pattern_data = pattern.as_bytes().to_vec();
            self.cache_asset(key.to_string(), "text_pattern".to_string(), pattern_data);
        }
    }
}