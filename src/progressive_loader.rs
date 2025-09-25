use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use crate::level::LevelSpec;
use crate::cache::GameCache;
use rand::SeedableRng;

#[derive(Debug, Clone)]
pub enum LoadingStage {
    Initialization,
    CoreAssets,
    LearningLevels,
    CommunityLevels,
    FontCache,
    Complete,
}

#[derive(Debug, Clone)]
pub struct LoadingProgress {
    pub stage: LoadingStage,
    pub current_item: String,
    pub progress: f32, // 0.0 to 1.0
    pub total_items: usize,
    pub completed_items: usize,
}

pub struct ProgressiveLoader {
    pub cache: GameCache,
    pub progress_receiver: mpsc::Receiver<LoadingProgress>,
    pub levels_receiver: mpsc::Receiver<Vec<LevelSpec>>,
    progress_sender: mpsc::Sender<LoadingProgress>,
    levels_sender: mpsc::Sender<Vec<LevelSpec>>,
    is_loading: bool,
}

impl ProgressiveLoader {
    pub fn new() -> Self {
        let (progress_sender, progress_receiver) = mpsc::channel();
        let (levels_sender, levels_receiver) = mpsc::channel();
        
        Self {
            cache: GameCache::load(),
            progress_receiver,
            levels_receiver,
            progress_sender,
            levels_sender,
            is_loading: false,
        }
    }
    
    pub fn start_loading(&mut self) {
        if self.is_loading {
            log::warn!("Loading already in progress");
            return;
        }
        
        self.is_loading = true;
        let progress_sender = self.progress_sender.clone();
        let levels_sender = self.levels_sender.clone();
        let mut cache = self.cache.clone();
        
        thread::spawn(move || {
            Self::load_game_async(progress_sender, levels_sender, &mut cache);
        });
    }
    
    fn load_game_async(
        progress_sender: mpsc::Sender<LoadingProgress>, 
        levels_sender: mpsc::Sender<Vec<LevelSpec>>,
        cache: &mut GameCache
    ) {
        let start_time = Instant::now();
        
        // Stage 1: Initialization
        let _ = progress_sender.send(LoadingProgress {
            stage: LoadingStage::Initialization,
            current_item: "Initializing game systems...".to_string(),
            progress: 0.0,
            total_items: 6, // Increased for new caching stages
            completed_items: 0,
        });
        
        // Check if we can use cached startup data for ultra-fast loading
        if cache.is_startup_data_fresh(300) && cache.is_embedded_levels_cache_valid() {
            log::info!("Using cached startup data for ultra-fast loading");
            Self::load_from_cache_fast(progress_sender, levels_sender, cache, start_time);
            return;
        }
        
        // Pre-cache common assets early for faster parsing later
        cache.precache_common_assets();
        
        // Small delay to prevent instantaneous loading feeling
        thread::sleep(Duration::from_millis(30));
        
        // Stage 2: Load core assets
        let _ = progress_sender.send(LoadingProgress {
            stage: LoadingStage::CoreAssets,
            current_item: "Loading core game assets...".to_string(),
            progress: 0.16,
            total_items: 6,
            completed_items: 1,
        });
        
        // Load basic game constants and configurations
        thread::sleep(Duration::from_millis(80));
        
        // Stage 3: Load learning levels (embedded)
        let _ = progress_sender.send(LoadingProgress {
            stage: LoadingStage::LearningLevels,
            current_item: "Loading learning levels...".to_string(),
            progress: 0.33,
            total_items: 6,
            completed_items: 2,
        });
        
        // Load embedded learning levels first (highest priority)
        let learning_levels = crate::embedded_levels::get_embedded_level_specs();
        let mut all_levels = learning_levels;
        log::info!("Loaded {} embedded learning levels", all_levels.len());
        
        // Send learning levels immediately so game can start
        let _ = levels_sender.send(all_levels.clone());
        
        // Stage 4: Load community levels (external files)
        let _ = progress_sender.send(LoadingProgress {
            stage: LoadingStage::CommunityLevels,
            current_item: "Loading community levels...".to_string(),
            progress: 0.5,
            total_items: 6,
            completed_items: 3,
        });
        
        // Load external community levels with caching
        let community_levels = Self::load_community_levels_cached(cache);
        all_levels.extend(community_levels);
        log::info!("Total levels loaded: {}", all_levels.len());
        
        // Stage 5: Font cache optimization
        let _ = progress_sender.send(LoadingProgress {
            stage: LoadingStage::FontCache,
            current_item: "Optimizing font rendering...".to_string(),
            progress: 0.66,
            total_items: 6,
            completed_items: 4,
        });
        
        // Pre-cache common font sizes for better performance
        Self::precache_font_metrics(cache);
        
        // Stage 6: Cache startup data for next time
        let _ = progress_sender.send(LoadingProgress {
            stage: LoadingStage::Complete,
            current_item: "Saving startup cache...".to_string(),
            progress: 0.83,
            total_items: 6,
            completed_items: 5,
        });
        
        // Cache startup data for ultra-fast loading next time
        Self::cache_startup_data(cache, &all_levels, start_time.elapsed().as_millis() as u64);
        
        // Stage 7: Complete
        let _ = progress_sender.send(LoadingProgress {
            stage: LoadingStage::Complete,
            current_item: "Loading complete!".to_string(),
            progress: 1.0,
            total_items: 6,
            completed_items: 6,
        });
        
        // Brief delay to show completion message before hiding
        thread::sleep(Duration::from_millis(1500));
        
        // Save updated cache
        cache.save();
        
        let load_time = start_time.elapsed();
        log::info!("Progressive loading completed in {:?}", load_time);
    }
    
    // Ultra-fast loading path using cached data
    fn load_from_cache_fast(
        progress_sender: mpsc::Sender<LoadingProgress>, 
        levels_sender: mpsc::Sender<Vec<LevelSpec>>,
        cache: &GameCache,
        start_time: Instant
    ) {
        let _ = progress_sender.send(LoadingProgress {
            stage: LoadingStage::CoreAssets,
            current_item: "Loading from cache...".to_string(),
            progress: 0.5,
            total_items: 2,
            completed_items: 1,
        });
        
        // Load embedded levels (these are always available)
        let learning_levels = crate::embedded_levels::get_embedded_level_specs();
        let mut all_levels = learning_levels;
        
        // Add any cached community levels
        for cached_level in cache.compiled_levels.values() {
            all_levels.push(cached_level.spec.clone());
        }
        
        // Send levels immediately
        let _ = levels_sender.send(all_levels);
        
        let _ = progress_sender.send(LoadingProgress {
            stage: LoadingStage::Complete,
            current_item: "Cache loading complete!".to_string(),
            progress: 1.0,
            total_items: 2,
            completed_items: 2,
        });
        
        let load_time = start_time.elapsed();
        log::info!("Ultra-fast cache loading completed in {:?}", load_time);
        
        // Brief delay to show completion
        thread::sleep(Duration::from_millis(200));
    }
    
    // Cache startup data for next session
    fn cache_startup_data(cache: &mut GameCache, levels: &[LevelSpec], load_time_ms: u64) {
        use crate::cache::StartupData;
        
        let startup_data = StartupData {
            last_played_level: 0, // Default to first level
            total_levels_count: levels.len(),
            embedded_levels_checksum: GameCache::generate_embedded_levels_checksum(),
            startup_time_ms: load_time_ms,
            cached_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        cache.cache_startup_data(startup_data);
    }
    
    fn load_community_levels_cached(_cache: &mut GameCache) -> Vec<LevelSpec> {
        // Community levels removed - only use learning levels now
        Vec::new()
    }
    
    fn precache_font_metrics(cache: &mut GameCache) {
        use crate::cache::FontMetrics;
        
        // Pre-cache common font sizes used in the game
        let common_sizes = [12.0, 14.0, 16.0, 18.0, 20.0, 22.0, 24.0, 26.0, 28.0];
        
        for &size in &common_sizes {
            let cache_key = format!("font_{}", size as u32);
            
            // Check if already cached and fresh
            if let Some(cached) = cache.get_font_metrics(&cache_key) {
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                    
                if current_time - cached.cached_at < 86400 { // 24 hour cache
                    continue; // Skip if fresh
                }
            }
            
            // Pre-calculate metrics (simplified version)
            let metrics = FontMetrics {
                font_size: size,
                char_width: size * 0.6, // Approximate monospace width
                line_height: size * 1.4, // Approximate line height
                cached_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            };
            
            cache.cache_font_metrics(cache_key, metrics);
        }
    }
    
    pub fn is_complete(&self) -> bool {
        !self.is_loading && self.levels_receiver.try_recv().is_err()
    }
    
    pub fn get_latest_progress(&self) -> Option<LoadingProgress> {
        self.progress_receiver.try_recv().ok()
    }
    
    pub fn get_loaded_levels(&self) -> Option<Vec<LevelSpec>> {
        self.levels_receiver.try_recv().ok()
    }
}