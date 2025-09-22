use macroquad::prelude::*;
use crate::font_scaling::*;
use crate::progressive_loader::{LoadingProgress, LoadingStage};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
pub enum MenuState {
    MainMenu,
    Settings,
    LevelSelect,
    HotkeySettings,
    InGame,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenuAction {
    None,
    StartGame,
    OpenSettings,
    OpenLevelSelect,
    OpenCommunityLevels,
    SelectLevel(usize),
    BackToMain,
    BackToGame,  // New action for returning to game from settings
    Exit,
    IncreaseResolution,
    DecreaseResolution,
    ToggleFullscreen,
    IncreaseSfxVolume,
    DecreaseSfxVolume,
    IncreaseMusicVolume,
    DecreaseMusicVolume,
    IncreaseFontSize,
    DecreaseFontSize,
    ToggleAutocomplete,
    ToggleVSCodeIntegration,
    OpenHotkeySettings,
    BackToSettings,
}

#[derive(Clone, Debug)]
pub struct MenuButton {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub action: MenuAction,
    pub enabled: bool,
}

impl MenuButton {
    pub fn new(text: String, x: f32, y: f32, width: f32, height: f32, action: MenuAction) -> Self {
        Self {
            text,
            x,
            y,
            width,
            height,
            action,
            enabled: true,
        }
    }

    pub fn is_clicked(&self, mouse_x: f32, mouse_y: f32) -> bool {
        self.enabled && 
        mouse_x >= self.x && 
        mouse_x <= self.x + self.width &&
        mouse_y >= self.y && 
        mouse_y <= self.y + self.height
    }

    pub fn draw(&self) {
        let bg_color = if self.enabled {
            Color::new(0.2, 0.3, 0.5, 0.9)
        } else {
            Color::new(0.1, 0.1, 0.1, 0.5)
        };
        
        let text_color = if self.enabled { WHITE } else { GRAY };

        // Draw button background
        draw_rectangle(self.x, self.y, self.width, self.height, bg_color);
        draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0, WHITE);

        // Center text in button - use default multiplier for menu buttons
        let text_size = 24.0;
        let scaled_text_size = scale_font_size(text_size);
        let text_dimensions = measure_text(&self.text, None, scaled_text_size as u16, 1.0);
        let text_x = self.x + (self.width - text_dimensions.width) / 2.0;
        let text_y = self.y + (self.height + text_dimensions.height) / 2.0;

        draw_scaled_text(&self.text, text_x, text_y, text_size, text_color);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerProgress {
    pub max_level_unlocked: usize, // Highest level the player has reached
    pub completed_levels: Vec<bool>, // Track which levels have been completed
}

impl Default for PlayerProgress {
    fn default() -> Self {
        Self {
            max_level_unlocked: 0, // Start with only level 0 unlocked
            completed_levels: Vec::new(),
        }
    }
}

impl PlayerProgress {
    const SAVE_FILE: &'static str = "player_progress.json";
    
    pub fn load_or_default() -> Self {
        if Path::new(Self::SAVE_FILE).exists() {
            match fs::read_to_string(Self::SAVE_FILE) {
                Ok(contents) => {
                    match serde_json::from_str::<PlayerProgress>(&contents) {
                        Ok(progress) => progress,
                        Err(_) => {
                            // If file is corrupted, create new progress and save it
                            let default = Self::default();
                            let _ = default.save();
                            default
                        }
                    }
                }
                Err(_) => Self::default(),
            }
        } else {
            // Create new save file
            let default = Self::default();
            let _ = default.save();
            default
        }
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(Self::SAVE_FILE, json)?;
        Ok(())
    }
    
    pub fn unlock_level(&mut self, level: usize) {
        if level > self.max_level_unlocked {
            self.max_level_unlocked = level;
            let _ = self.save();
        }
    }
    
    pub fn mark_level_completed(&mut self, level: usize) {
        // Ensure the completed_levels vec is large enough
        while self.completed_levels.len() <= level {
            self.completed_levels.push(false);
        }
        self.completed_levels[level] = true;
        let _ = self.save();
    }
    
    pub fn is_level_unlocked(&self, level: usize) -> bool {
        level <= self.max_level_unlocked
    }
    
    pub fn is_level_completed(&self, level: usize) -> bool {
        level < self.completed_levels.len() && self.completed_levels[level]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub window_width: i32,
    pub window_height: i32,
    pub fullscreen: bool,
    pub maximized: bool,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub font_size_multiplier: f32,
    pub autocomplete_enabled: bool,
    pub vscode_integration_enabled: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            window_width: 1920,
            window_height: 1080,
            fullscreen: false,
            maximized: false,
            sfx_volume: 0.7,
            music_volume: 0.5,
            font_size_multiplier: 1.0,
            autocomplete_enabled: true,
            vscode_integration_enabled: true,
        }
    }
}

impl GameSettings {
    const SAVE_FILE: &'static str = "game_settings.json";
    
    pub fn load_or_default() -> Self {
        if Path::new(Self::SAVE_FILE).exists() {
            match fs::read_to_string(Self::SAVE_FILE) {
                Ok(contents) => {
                    match serde_json::from_str::<GameSettings>(&contents) {
                        Ok(settings) => settings,
                        Err(_) => {
                            // If file is corrupted, create new settings and save them
                            let default = Self::default();
                            let _ = default.save();
                            default
                        }
                    }
                }
                Err(_) => Self::default(),
            }
        } else {
            // Create new save file
            let default = Self::default();
            let _ = default.save();
            default
        }
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(Self::SAVE_FILE, json)?;
        Ok(())
    }
    
    pub fn update_from_actual_screen(&mut self) {
        // Update settings to match actual screen size
        self.window_width = screen_width() as i32;
        self.window_height = screen_height() as i32;
    }
}

#[derive(Clone, Debug)]
pub struct Menu {
    pub state: MenuState,
    pub buttons: Vec<MenuButton>,
    pub settings: GameSettings,
    pub progress: PlayerProgress,
    pub scroll_offset: f32,
    pub opened_from_game: bool,  // Track if settings were opened from in-game
    pub last_screen_width: f32,
    pub last_screen_height: f32,
    pub total_levels: usize, // Total number of levels available
}

impl Menu {
    pub fn new() -> Self {
        let mut menu = Self {
            state: MenuState::MainMenu,
            buttons: Vec::new(),
            settings: GameSettings::load_or_default(),
            progress: PlayerProgress::load_or_default(),
            scroll_offset: 0.0,
            opened_from_game: false,
            last_screen_width: screen_width(),
            last_screen_height: screen_height(),
            total_levels: 0, // Will be set when game starts
        };
        menu.setup_main_menu();
        menu
    }

    fn get_available_resolutions() -> Vec<(i32, i32)> {
        vec![
            (1280, 720),
            (1366, 768),
            (1600, 900),
            (1920, 1080),
            (2560, 1440),
            (3840, 2160),
        ]
    }

    fn current_resolution_index(&self) -> usize {
        let resolutions = Self::get_available_resolutions();
        resolutions.iter()
            .position(|(w, h)| *w == self.settings.window_width && *h == self.settings.window_height)
            .unwrap_or(3) // Default to 1920x1080 if not found
    }

    pub fn setup_main_menu(&mut self) {
        self.buttons.clear();
        
        let screen_center_x = screen_width() / 2.0;
        let button_width = scale_size(300.0);
        let button_height = scale_size(60.0);
        let button_spacing = scale_size(80.0);
        let start_y = screen_height() / 2.0;

        self.buttons.push(MenuButton::new(
            "Start Learning".to_string(),
            screen_center_x - button_width / 2.0,
            start_y - button_spacing * 1.0,
            button_width,
            button_height,
            MenuAction::StartGame,
        ));

        self.buttons.push(MenuButton::new(
            "Load Level".to_string(),
            screen_center_x - button_width / 2.0,
            start_y,
            button_width,
            button_height,
            MenuAction::OpenLevelSelect,
        ));

        self.buttons.push(MenuButton::new(
            "Community Levels".to_string(),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 1.0,
            button_width,
            button_height,
            MenuAction::OpenCommunityLevels,
        ));

        self.buttons.push(MenuButton::new(
            "Settings".to_string(),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 2.0,
            button_width,
            button_height,
            MenuAction::OpenSettings,
        ));

        self.buttons.push(MenuButton::new(
            "Exit".to_string(),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 3.0,
            button_width,
            button_height,
            MenuAction::Exit,
        ));
    }

    pub fn open_settings_from_game(&mut self) {
        self.opened_from_game = true;
        self.state = MenuState::Settings;
        self.setup_settings_menu();
    }

    pub fn setup_settings_menu(&mut self) {
        self.buttons.clear();
        
        let screen_center_x = screen_width() / 2.0;
        let button_width = scale_size(400.0);
        let button_height = scale_size(50.0);
        let button_spacing = scale_size(70.0);
        let start_y = screen_height() / 2.0 - scale_size(100.0);

        // Resolution buttons
        self.buttons.push(MenuButton::new(
            format!("Resolution: {}x{} (Click: Next, Right-Click: Previous)", 
                   self.settings.window_width, self.settings.window_height),
            screen_center_x - button_width / 2.0,
            start_y,
            button_width,
            button_height,
            MenuAction::IncreaseResolution,
        ));

        // Fullscreen toggle
        self.buttons.push(MenuButton::new(
            format!("Fullscreen: {} (Click to Toggle)", 
                   if self.settings.fullscreen { "On" } else { "Off" }),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing,
            button_width,
            button_height,
            MenuAction::ToggleFullscreen,
        ));

        // Autocomplete toggle
        self.buttons.push(MenuButton::new(
            format!("Autocomplete: {} (Click to Toggle)",
                   if true { "On" } else { "Off" }), // TODO: Get actual autocomplete state
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 2.0,
            button_width,
            button_height,
            MenuAction::ToggleAutocomplete,
        ));

        // Font size control
        self.buttons.push(MenuButton::new(
            format!("Font Size: {:.0}% (Click: +10%, Right-Click: -10%)",
                   self.settings.font_size_multiplier * 100.0),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 3.0,
            button_width,
            button_height,
            MenuAction::IncreaseFontSize,
        ));

        // Hotkey settings button
        self.buttons.push(MenuButton::new(
            "Hotkey Settings".to_string(),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 4.0,
            button_width,
            button_height,
            MenuAction::OpenHotkeySettings,
        ));

        // Back button - context-aware
        let (back_text, back_action) = if self.opened_from_game {
            ("Back to Game".to_string(), MenuAction::BackToGame)
        } else {
            ("Back to Main".to_string(), MenuAction::BackToMain)
        };

        self.buttons.push(MenuButton::new(
            back_text,
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 5.0,
            button_width,
            button_height,
            back_action,
        ));
    }

    pub fn setup_hotkey_settings_menu(&mut self) {
        self.buttons.clear();

        let screen_center_x = screen_width() / 2.0;
        let button_width = scale_size(500.0);
        let button_height = scale_size(50.0);
        let button_spacing = scale_size(70.0);
        let start_y = screen_height() / 2.0 - scale_size(200.0);

        // Title info
        self.buttons.push(MenuButton::new(
            "HOTKEY SETTINGS".to_string(),
            screen_center_x - button_width / 2.0,
            start_y - button_spacing,
            button_width,
            button_height,
            MenuAction::None,
        ));

        // Display current key bindings (first few important ones)
        let key_bindings = vec![
            ("Tab", "Accept Autocomplete / Indent"),
            ("Ctrl+S", "Save File"),
            ("Ctrl+Shift+Enter", "Run Code"),
            ("Ctrl+`", "Toggle Editor"),
            ("Ctrl+Z", "Undo"),
            ("Ctrl+Y", "Redo"),
        ];

        for (i, (key, action)) in key_bindings.iter().enumerate() {
            self.buttons.push(MenuButton::new(
                format!("{}: {}", key, action),
                screen_center_x - button_width / 2.0,
                start_y + (i as f32 * button_spacing * 0.8),
                button_width,
                button_height * 0.8,
                MenuAction::None,
            ));
        }

        // Import buttons
        let import_y = start_y + (key_bindings.len() as f32 * button_spacing * 0.8) + button_spacing;

        self.buttons.push(MenuButton::new(
            "Import VSCode Keybindings".to_string(),
            screen_center_x - button_width / 2.0,
            import_y,
            button_width,
            button_height,
            MenuAction::None, // TODO: Add import actions
        ));

        self.buttons.push(MenuButton::new(
            "Import Vim Settings".to_string(),
            screen_center_x - button_width / 2.0,
            import_y + button_spacing,
            button_width,
            button_height,
            MenuAction::None, // TODO: Add import actions
        ));

        self.buttons.push(MenuButton::new(
            "Import Emacs Settings".to_string(),
            screen_center_x - button_width / 2.0,
            import_y + button_spacing * 2.0,
            button_width,
            button_height,
            MenuAction::None, // TODO: Add import actions
        ));

        // Reset and back buttons
        self.buttons.push(MenuButton::new(
            "Reset to Defaults".to_string(),
            screen_center_x - button_width / 2.0,
            import_y + button_spacing * 3.5,
            button_width,
            button_height,
            MenuAction::None, // TODO: Add reset action
        ));

        self.buttons.push(MenuButton::new(
            "Back to Settings".to_string(),
            screen_center_x - button_width / 2.0,
            import_y + button_spacing * 4.5,
            button_width,
            button_height,
            MenuAction::BackToSettings,
        ));
    }

    pub fn check_screen_resize(&mut self) {
        let current_width = screen_width();
        let current_height = screen_height();
        
        // Check if screen size has changed
        if (current_width - self.last_screen_width).abs() > 1.0 || 
           (current_height - self.last_screen_height).abs() > 1.0 {
            
            // Update tracked screen size
            self.last_screen_width = current_width;
            self.last_screen_height = current_height;
            
            // Refresh the appropriate menu
            match self.state {
                MenuState::MainMenu => self.setup_main_menu(),
                MenuState::Settings => self.setup_settings_menu(),
                MenuState::LevelSelect => self.setup_level_select_menu(),
                MenuState::HotkeySettings => self.setup_hotkey_settings_menu(),
                MenuState::InGame => {}, // No menu to refresh
            }
        }
    }

    pub fn setup_level_select_menu(&mut self) {
        self.buttons.clear();
        
        let screen_center_x = screen_width() / 2.0;
        let button_width = scale_size(300.0);
        let button_height = scale_size(40.0);
        let button_spacing = scale_size(45.0);
        let buttons_per_row = 3;
        let row_spacing = scale_size(70.0);
        
        let start_y = screen_height() * 0.2;
        
        // Create level buttons for unlocked levels
        let mut row = 0;
        let mut col = 0;
        
        for level in 0..=self.progress.max_level_unlocked {
            if level >= self.total_levels {
                break; // Don't show levels that don't exist
            }
            
            let x_offset = (col as f32 - (buttons_per_row as f32 - 1.0) / 2.0) * (button_width + scale_size(20.0));
            let x = screen_center_x + x_offset;
            let y = start_y + row as f32 * row_spacing;
            
            let level_name = if level < self.total_levels && level < 50 { // Reasonable upper limit
                format!("Level {} {}", level + 1, if self.progress.is_level_completed(level) { "✓" } else { "" })
            } else {
                format!("Level {}", level + 1)
            };
            
            self.buttons.push(MenuButton::new(
                level_name,
                x - button_width / 2.0,
                y,
                button_width,
                button_height,
                MenuAction::SelectLevel(level),
            ));
            
            col += 1;
            if col >= buttons_per_row {
                col = 0;
                row += 1;
            }
        }
        
        // Add back button at the bottom
        let back_y = start_y + (row + 2) as f32 * row_spacing;
        self.buttons.push(MenuButton::new(
            "Back to Main Menu".to_string(),
            screen_center_x - button_width / 2.0,
            back_y,
            button_width,
            button_height,
            MenuAction::BackToMain,
        ));
    }
    
    pub fn set_total_levels(&mut self, count: usize) {
        self.total_levels = count;
    }

    pub fn handle_input(&mut self) -> MenuAction {
        // Only handle input when we're actually showing a menu, not when in-game
        if self.state == MenuState::InGame {
            return MenuAction::None;
        }

        let (mouse_x, mouse_y) = mouse_position();

        // Handle left mouse button
        if is_mouse_button_pressed(MouseButton::Left) {
            for button in &self.buttons {
                if button.is_clicked(mouse_x, mouse_y) {
                    return button.action.clone();
                }
            }
        }

        // Handle right mouse button (for settings decrease actions)
        if is_mouse_button_pressed(MouseButton::Right) {
            for button in &self.buttons {
                if button.is_clicked(mouse_x, mouse_y) {
                    // Convert increase actions to decrease actions
                    return match button.action {
                        MenuAction::IncreaseResolution => MenuAction::DecreaseResolution,
                        MenuAction::IncreaseSfxVolume => MenuAction::DecreaseSfxVolume,
                        MenuAction::IncreaseMusicVolume => MenuAction::DecreaseMusicVolume,
                        MenuAction::IncreaseFontSize => MenuAction::DecreaseFontSize,
                        MenuAction::ToggleFullscreen => MenuAction::ToggleFullscreen,
                        _ => button.action.clone(),
                    };
                }
            }
        }

        // Handle keyboard shortcuts
        if is_key_pressed(KeyCode::Escape) {
            match self.state {
                MenuState::MainMenu => return MenuAction::Exit,
                MenuState::Settings => {
                    if self.opened_from_game {
                        return MenuAction::BackToGame;
                    } else {
                        return MenuAction::BackToMain;
                    }
                },
                MenuState::HotkeySettings => return MenuAction::BackToSettings,
                _ => return MenuAction::BackToMain,
            }
        }

        MenuAction::None
    }

    pub fn update(&mut self, action: MenuAction) {
        match action {
            MenuAction::StartGame => {
                self.state = MenuState::InGame;
            },
            MenuAction::OpenSettings => {
                self.state = MenuState::Settings;
                self.opened_from_game = false;  // Ensure flag is false when opened from main menu
                self.setup_settings_menu();
            },
            MenuAction::OpenLevelSelect => {
                self.state = MenuState::LevelSelect;
                self.setup_level_select_menu();
            },
            MenuAction::SelectLevel(_) => {
                // Level selection is handled by the main game loop
                self.state = MenuState::InGame;
            },
            MenuAction::BackToMain => {
                self.state = MenuState::MainMenu;
                self.opened_from_game = false;  // Reset context flag
                self.setup_main_menu();
            },
            MenuAction::BackToGame => {
                self.state = MenuState::InGame;
                self.opened_from_game = false;  // Reset context flag
            },
            MenuAction::IncreaseResolution => {
                let resolutions = Self::get_available_resolutions();
                let current_index = self.current_resolution_index();
                let next_index = (current_index + 1) % resolutions.len();
                let (width, height) = resolutions[next_index];
                self.settings.window_width = width;
                self.settings.window_height = height;
                let _ = self.settings.save(); // Save settings when changed
                // Menu will be refreshed at end of update method
            },
            MenuAction::DecreaseResolution => {
                let resolutions = Self::get_available_resolutions();
                let current_index = self.current_resolution_index();
                let prev_index = if current_index == 0 { 
                    resolutions.len() - 1 
                } else { 
                    current_index - 1 
                };
                let (width, height) = resolutions[prev_index];
                self.settings.window_width = width;
                self.settings.window_height = height;
                let _ = self.settings.save(); // Save settings when changed
                // Menu will be refreshed at end of update method
            },
            MenuAction::ToggleFullscreen => {
                self.settings.fullscreen = !self.settings.fullscreen;
                let _ = self.settings.save(); // Save settings when changed
                // Menu will be refreshed at end of update method
            },
            MenuAction::IncreaseSfxVolume => {
                self.settings.sfx_volume = (self.settings.sfx_volume + 0.1).min(1.0);
                let _ = self.settings.save(); // Save settings when changed
                // Menu will be refreshed at end of update method
            },
            MenuAction::DecreaseSfxVolume => {
                self.settings.sfx_volume = (self.settings.sfx_volume - 0.1).max(0.0);
                let _ = self.settings.save(); // Save settings when changed
                // Menu will be refreshed at end of update method
            },
            MenuAction::IncreaseMusicVolume => {
                self.settings.music_volume = (self.settings.music_volume + 0.1).min(1.0);
                let _ = self.settings.save(); // Save settings when changed
                // Menu will be refreshed at end of update method
            },
            MenuAction::DecreaseMusicVolume => {
                self.settings.music_volume = (self.settings.music_volume - 0.1).max(0.0);
                let _ = self.settings.save(); // Save settings when changed
                // Menu will be refreshed at end of update method
            },
            MenuAction::IncreaseFontSize => {
                self.settings.font_size_multiplier = (self.settings.font_size_multiplier + 0.1).min(2.0);
                let _ = self.settings.save(); // Save settings when changed
                // Menu will be refreshed at end of update method
            },
            MenuAction::DecreaseFontSize => {
                self.settings.font_size_multiplier = (self.settings.font_size_multiplier - 0.1).max(0.5);
                let _ = self.settings.save(); // Save settings when changed
                // Menu will be refreshed at end of update method
            },
            MenuAction::ToggleAutocomplete => {
                self.settings.autocomplete_enabled = !self.settings.autocomplete_enabled;
                let _ = self.settings.save(); // Save settings when changed
                // Menu will be refreshed at end of update method
            },
            MenuAction::OpenHotkeySettings => {
                self.state = MenuState::HotkeySettings;
                self.setup_hotkey_settings_menu();
            },
            MenuAction::BackToSettings => {
                self.state = MenuState::Settings;
                self.setup_settings_menu();
            },
            _ => {}
        }
        
        // Refresh menu if we're in Settings or HotkeySettings to ensure buttons stay visible
        match self.state {
            MenuState::Settings => self.setup_settings_menu(),
            MenuState::HotkeySettings => self.setup_hotkey_settings_menu(),
            _ => {}
        }
    }

    pub fn draw(&self) {
        self.draw_with_loading_progress(None);
    }
    
    pub fn draw_with_loading_progress(&self, loading_progress: Option<&LoadingProgress>) {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        match self.state {
            MenuState::MainMenu => {
                self.draw_main_menu();
                if let Some(progress) = loading_progress {
                    // Only show loading progress if not complete
                    if !matches!(progress.stage, LoadingStage::Complete) || progress.progress < 1.0 {
                        self.draw_loading_progress(progress);
                    }
                }
            },
            MenuState::Settings => self.draw_settings_menu(),
            MenuState::LevelSelect => self.draw_level_select_menu(),
            MenuState::HotkeySettings => self.draw_hotkey_settings_menu(),
            MenuState::InGame => {}, // Game drawing handled elsewhere
        }
    }

    fn draw_main_menu(&self) {
        // Draw background pattern
        self.draw_background();

        // Draw banner
        let banner_text = "Welcome to Robo Wars Crab Edition";
        let banner_size = 48.0;
        let scaled_banner_size = scale_font_size(banner_size);
        let banner_dimensions = measure_text(banner_text, None, scaled_banner_size as u16, 1.0);
        let banner_x = (screen_width() - banner_dimensions.width) / 2.0;
        let banner_y = screen_height() / 3.0;

        // Banner shadow
        draw_scaled_text(banner_text, banner_x + scale_size(2.0), banner_y + scale_size(2.0), banner_size, Color::new(0.0, 0.0, 0.0, 0.5));
        // Banner text
        draw_scaled_text(banner_text, banner_x, banner_y, banner_size, GOLD);

        // Draw subtitle
        let subtitle = "Program your robot to explore and conquer!";
        let subtitle_size = 20.0;
        let scaled_subtitle_size = scale_font_size(subtitle_size);
        let subtitle_dimensions = measure_text(subtitle, None, scaled_subtitle_size as u16, 1.0);
        let subtitle_x = (screen_width() - subtitle_dimensions.width) / 2.0;
        draw_scaled_text(subtitle, subtitle_x, banner_y + scale_size(60.0), subtitle_size, LIGHTGRAY);

        // Draw buttons
        for button in &self.buttons {
            button.draw();
        }

        // Draw version info
        draw_scaled_text("Version 2.0 - YAML Edition", scale_size(10.0), screen_height() - scale_size(10.0), 16.0, DARKGRAY);
    }

    fn draw_settings_menu(&self) {
        // Draw background
        self.draw_background();

        // Draw title
        let title = "Settings";
        let title_size = 36.0;
        let scaled_title_size = scale_font_size(title_size);
        let title_dimensions = measure_text(title, None, scaled_title_size as u16, 1.0);
        let title_x = (screen_width() - title_dimensions.width) / 2.0;
        draw_scaled_text(title, title_x, scale_size(100.0), title_size, WHITE);

        // Draw instructions
        let instructions = "Left Click: Increase/Next | Right Click: Decrease/Previous";
        let inst_size = 18.0;
        let scaled_inst_size = scale_font_size(inst_size);
        let inst_dimensions = measure_text(instructions, None, scaled_inst_size as u16, 1.0);
        let inst_x = (screen_width() - inst_dimensions.width) / 2.0;
        draw_scaled_text(instructions, inst_x, scale_size(140.0), inst_size, YELLOW);

        // Draw buttons
        for button in &self.buttons {
            button.draw();
        }

        // Draw footer notes
        draw_scaled_text("Note: Window resolution changes require restart to take effect", scale_size(50.0), screen_height() - scale_size(70.0), 14.0, GRAY);
        draw_scaled_text("Volume and fullscreen changes apply immediately", scale_size(50.0), screen_height() - scale_size(50.0), 14.0, GRAY);
    }

    fn draw_level_select_menu(&self) {
        // Draw background
        self.draw_background();

        // Draw title
        let title = "Level Select";
        let title_size = 36.0;
        let scaled_title_size = scale_font_size(title_size);
        let title_dimensions = measure_text(title, None, scaled_title_size as u16, 1.0);
        let title_x = (screen_width() - title_dimensions.width) / 2.0;
        draw_scaled_text(title, title_x, scale_size(100.0), title_size, WHITE);

        // Draw progress info
        let progress_text = format!("Progress: {} levels unlocked", self.progress.max_level_unlocked + 1);
        let progress_size = 18.0;
        let scaled_progress_size = scale_font_size(progress_size);
        let progress_dimensions = measure_text(&progress_text, None, scaled_progress_size as u16, 1.0);
        let progress_x = (screen_width() - progress_dimensions.width) / 2.0;
        draw_scaled_text(&progress_text, progress_x, scale_size(140.0), progress_size, YELLOW);

        // Draw buttons
        for button in &self.buttons {
            button.draw();
        }

        // Draw instructions
        draw_scaled_text("Select a level to jump directly to it", scale_size(50.0), screen_height() - scale_size(50.0), 14.0, GRAY);
    }

    fn draw_background(&self) {
        // Draw a simple grid pattern
        let grid_size = 50.0;
        let grid_color = Color::new(0.1, 0.1, 0.2, 0.3);
        
        // Vertical lines
        let mut x = 0.0;
        while x < screen_width() {
            draw_line(x, 0.0, x, screen_height(), 1.0, grid_color);
            x += grid_size;
        }
        
        // Horizontal lines
        let mut y = 0.0;
        while y < screen_height() {
            draw_line(0.0, y, screen_width(), y, 1.0, grid_color);
            y += grid_size;
        }

        // Draw some decorative robots/crabs in corners
        self.draw_decorative_elements();
    }

    fn draw_decorative_elements(&self) {
        // Simple robot/crab-like symbols in corners
        let crab_color = Color::new(0.8, 0.4, 0.2, 0.3);
        
        // Top-left crab
        draw_circle(80.0, 80.0, 15.0, crab_color);
        draw_circle(65.0, 70.0, 5.0, crab_color); // Left eye
        draw_circle(95.0, 70.0, 5.0, crab_color); // Right eye
        
        // Bottom-right crab
        let br_x = screen_width() - 80.0;
        let br_y = screen_height() - 80.0;
        draw_circle(br_x, br_y, 15.0, crab_color);
        draw_circle(br_x - 15.0, br_y - 10.0, 5.0, crab_color); // Left eye
        draw_circle(br_x + 15.0, br_y - 10.0, 5.0, crab_color); // Right eye
    }
    
    fn draw_loading_progress(&self, progress: &LoadingProgress) {
        let bar_width = scale_size(400.0);
        let bar_height = scale_size(20.0);
        let bar_x = (screen_width() - bar_width) / 2.0;
        let bar_y = screen_height() - scale_size(150.0);
        
        // Draw loading text
        let stage_text = match progress.stage {
            LoadingStage::Initialization => "⚙️ Initializing...",
            LoadingStage::CoreAssets => "📦 Loading core assets...",
            LoadingStage::LearningLevels => "🎓 Loading learning levels...",
            LoadingStage::CommunityLevels => "🌍 Loading community levels...",
            LoadingStage::FontCache => "🔤 Optimizing fonts...",
            LoadingStage::Complete => "✅ Complete!",
        };
        
        let text_size = 16.0;
        let scaled_text_size = scale_font_size(text_size);
        let text_dimensions = measure_text(stage_text, None, scaled_text_size as u16, 1.0);
        let text_x = (screen_width() - text_dimensions.width) / 2.0;
        
        draw_scaled_text(stage_text, text_x, bar_y - scale_size(30.0), text_size, YELLOW);
        
        // Draw current item being processed
        if !progress.current_item.is_empty() {
            let item_size = 12.0;
            let scaled_item_size = scale_font_size(item_size);
            let item_dimensions = measure_text(&progress.current_item, None, scaled_item_size as u16, 1.0);
            let item_x = (screen_width() - item_dimensions.width) / 2.0;
            
            draw_scaled_text(&progress.current_item, item_x, bar_y - scale_size(10.0), item_size, LIGHTGRAY);
        }
        
        // Draw progress bar background
        draw_rectangle(bar_x, bar_y, bar_width, bar_height, Color::new(0.2, 0.2, 0.2, 0.8));
        draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, scale_size(2.0), WHITE);
        
        // Draw progress bar fill
        let fill_width = bar_width * progress.progress;
        let fill_color = match progress.stage {
            LoadingStage::Complete => GREEN,
            _ => Color::new(0.2, 0.6, 1.0, 0.8), // Blue
        };
        
        if fill_width > 0.0 {
            draw_rectangle(bar_x, bar_y, fill_width, bar_height, fill_color);
        }
        
        // Draw progress percentage
        let percent_text = format!("{}%", (progress.progress * 100.0) as i32);
        let percent_size = 14.0;
        let scaled_percent_size = scale_font_size(percent_size);
        let percent_dimensions = measure_text(&percent_text, None, scaled_percent_size as u16, 1.0);
        let percent_x = bar_x + (bar_width - percent_dimensions.width) / 2.0;
        let percent_y = bar_y + (bar_height + percent_dimensions.height) / 2.0;
        
        draw_scaled_text(&percent_text, percent_x, percent_y, percent_size, WHITE);
        
        // Draw item count if available
        if progress.total_items > 0 {
            let count_text = format!("{}/{} items", progress.completed_items, progress.total_items);
            let count_size = 12.0;
            let scaled_count_size = scale_font_size(count_size);
            let count_dimensions = measure_text(&count_text, None, scaled_count_size as u16, 1.0);
            let count_x = (screen_width() - count_dimensions.width) / 2.0;
            
            draw_scaled_text(&count_text, count_x, bar_y + bar_height + scale_size(15.0), count_size, GRAY);
        }
    }

    fn draw_hotkey_settings_menu(&self) {
        // Draw background
        self.draw_background();

        // Draw title
        let title = "Hotkey Settings";
        let title_size = 36.0;
        let scaled_title_size = scale_font_size(title_size);
        let title_dimensions = measure_text(title, None, scaled_title_size as u16, 1.0);
        let title_x = (screen_width() - title_dimensions.width) / 2.0;
        draw_scaled_text(title, title_x, scale_size(100.0), title_size, WHITE);

        // Draw buttons
        for button in &self.buttons {
            button.draw();
        }

        // Draw instructions
        draw_scaled_text("Configure keyboard shortcuts and import from other editors", scale_size(50.0), screen_height() - scale_size(50.0), 14.0, GRAY);
    }
}