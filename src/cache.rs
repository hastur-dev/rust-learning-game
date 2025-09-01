use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::level::LevelSpec;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GameCache {
    pub compiled_levels: HashMap<String, CachedLevel>,
    pub cache_version: u32,
    pub font_metrics: HashMap<String, FontMetrics>,
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

const CACHE_VERSION: u32 = 1;
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
}