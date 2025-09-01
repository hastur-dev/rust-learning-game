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
            total_items: 5,
            completed_items: 0,
        });
        
        // Small delay to prevent instantaneous loading feeling
        thread::sleep(Duration::from_millis(50));
        
        // Stage 2: Load core assets
        let _ = progress_sender.send(LoadingProgress {
            stage: LoadingStage::CoreAssets,
            current_item: "Loading core game assets...".to_string(),
            progress: 0.2,
            total_items: 5,
            completed_items: 1,
        });
        
        // Load basic game constants and configurations
        thread::sleep(Duration::from_millis(100));
        
        // Stage 3: Load learning levels (embedded)
        let _ = progress_sender.send(LoadingProgress {
            stage: LoadingStage::LearningLevels,
            current_item: "Loading learning levels...".to_string(),
            progress: 0.4,
            total_items: 5,
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
            progress: 0.6,
            total_items: 5,
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
            progress: 0.8,
            total_items: 5,
            completed_items: 4,
        });
        
        // Pre-cache common font sizes for better performance
        Self::precache_font_metrics(cache);
        
        // Stage 6: Complete
        let _ = progress_sender.send(LoadingProgress {
            stage: LoadingStage::Complete,
            current_item: "Loading complete!".to_string(),
            progress: 1.0,
            total_items: 5,
            completed_items: 5,
        });
        
        // Brief delay to show completion message before hiding
        thread::sleep(Duration::from_millis(1500));
        
        // Save updated cache
        cache.save();
        
        let load_time = start_time.elapsed();
        log::info!("Progressive loading completed in {:?}", load_time);
    }
    
    fn load_community_levels_cached(cache: &mut GameCache) -> Vec<LevelSpec> {
        let mut community_levels = Vec::new();
        
        // Try to load from community_levels directory
        if let Ok(entries) = std::fs::read_dir("community_levels") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") ||
                   path.extension().and_then(|s| s.to_str()) == Some("yml") {
                    
                    let file_path = path.to_string_lossy();
                    
                    // Try to load from cache first
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        let checksum = GameCache::calculate_checksum(&content);
                        let cache_key = file_path.to_string();
                        
                        // Check if we have a valid cached version
                        if let Some(cached) = cache.get_cached_level(&cache_key) {
                            if cached.checksum == checksum && cache.is_cache_fresh(cached, 3600) { // 1 hour cache
                                community_levels.push(cached.spec.clone());
                                log::debug!("Using cached level: {}", file_path);
                                continue;
                            }
                        }
                        
                        // Load and parse the level
                        if let Ok(yaml_config) = serde_yaml::from_str::<crate::level::YamlLevelConfig>(&content) {
                            let mut rng = ::rand::rngs::StdRng::seed_from_u64(0xDEADBEEF);
                            if let Ok(level_spec) = yaml_config.to_level_spec(&mut rng) {
                                // Cache the parsed level
                                cache.cache_level(cache_key, level_spec.clone(), checksum);
                                community_levels.push(level_spec);
                                log::debug!("Loaded and cached level: {}", file_path);
                            }
                        }
                    }
                }
            }
        }
        
        community_levels
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