use macroquad::prelude::*;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
pub enum MenuState {
    MainMenu,
    Settings,
    PlayerLevels,
    InGame,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenuAction {
    None,
    StartGame,
    OpenSettings,
    OpenPlayerLevels,
    BackToMain,
    Exit,
    LoadPlayerLevel(String),
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

        // Center text in button
        let text_size = 24.0;
        let text_dimensions = measure_text(&self.text, None, text_size as u16, 1.0);
        let text_x = self.x + (self.width - text_dimensions.width) / 2.0;
        let text_y = self.y + (self.height + text_dimensions.height) / 2.0;

        draw_text(&self.text, text_x, text_y, text_size, text_color);
    }
}

#[derive(Clone, Debug)]
pub struct GameSettings {
    pub window_width: i32,
    pub window_height: i32,
    pub fullscreen: bool,
    pub sfx_volume: f32,
    pub music_volume: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            window_width: 1920,
            window_height: 1080,
            fullscreen: false,
            sfx_volume: 0.7,
            music_volume: 0.5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Menu {
    pub state: MenuState,
    pub buttons: Vec<MenuButton>,
    pub settings: GameSettings,
    pub player_levels: Vec<String>,
    pub scroll_offset: f32,
}

impl Menu {
    pub fn new() -> Self {
        let mut menu = Self {
            state: MenuState::MainMenu,
            buttons: Vec::new(),
            settings: GameSettings::default(),
            player_levels: Vec::new(),
            scroll_offset: 0.0,
        };
        menu.load_player_levels();
        menu.setup_main_menu();
        menu
    }

    pub fn setup_main_menu(&mut self) {
        self.buttons.clear();
        
        let screen_center_x = screen_width() / 2.0;
        let button_width = 300.0;
        let button_height = 60.0;
        let button_spacing = 80.0;
        let start_y = screen_height() / 2.0;

        self.buttons.push(MenuButton::new(
            "Normal Start".to_string(),
            screen_center_x - button_width / 2.0,
            start_y,
            button_width,
            button_height,
            MenuAction::StartGame,
        ));

        self.buttons.push(MenuButton::new(
            "Settings".to_string(),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing,
            button_width,
            button_height,
            MenuAction::OpenSettings,
        ));

        self.buttons.push(MenuButton::new(
            "Player Levels".to_string(),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 2.0,
            button_width,
            button_height,
            MenuAction::OpenPlayerLevels,
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

    pub fn setup_settings_menu(&mut self) {
        self.buttons.clear();
        
        let screen_center_x = screen_width() / 2.0;
        let button_width = 300.0;
        let button_height = 50.0;
        let button_spacing = 70.0;
        let start_y = screen_height() / 2.0 - 50.0;

        // Resolution buttons
        self.buttons.push(MenuButton::new(
            format!("Resolution: {}x{}", self.settings.window_width, self.settings.window_height),
            screen_center_x - button_width / 2.0,
            start_y,
            button_width,
            button_height,
            MenuAction::None,
        ));

        // Fullscreen toggle
        self.buttons.push(MenuButton::new(
            format!("Fullscreen: {}", if self.settings.fullscreen { "On" } else { "Off" }),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing,
            button_width,
            button_height,
            MenuAction::None,
        ));

        // Volume controls
        self.buttons.push(MenuButton::new(
            format!("SFX Volume: {:.0}%", self.settings.sfx_volume * 100.0),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 2.0,
            button_width,
            button_height,
            MenuAction::None,
        ));

        self.buttons.push(MenuButton::new(
            format!("Music Volume: {:.0}%", self.settings.music_volume * 100.0),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 3.0,
            button_width,
            button_height,
            MenuAction::None,
        ));

        // Back button
        self.buttons.push(MenuButton::new(
            "Back to Main".to_string(),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 4.5,
            button_width,
            button_height,
            MenuAction::BackToMain,
        ));
    }

    pub fn setup_player_levels_menu(&mut self) {
        self.buttons.clear();
        self.load_player_levels();
        
        let screen_center_x = screen_width() / 2.0;
        let button_width = 400.0;
        let button_height = 50.0;
        let button_spacing = 60.0;
        let start_y = 200.0;

        // Level buttons
        for (i, level_name) in self.player_levels.iter().enumerate() {
            let y_pos = start_y + (i as f32 * button_spacing) - self.scroll_offset;
            
            // Only create buttons that are visible on screen
            if y_pos > -button_height && y_pos < screen_height() {
                self.buttons.push(MenuButton::new(
                    level_name.clone(),
                    screen_center_x - button_width / 2.0,
                    y_pos,
                    button_width,
                    button_height,
                    MenuAction::LoadPlayerLevel(level_name.clone()),
                ));
            }
        }

        // Back button (always visible)
        self.buttons.push(MenuButton::new(
            "Back to Main".to_string(),
            screen_center_x - 150.0,
            screen_height() - 80.0,
            300.0,
            50.0,
            MenuAction::BackToMain,
        ));

        // Refresh button
        self.buttons.push(MenuButton::new(
            "Refresh".to_string(),
            50.0,
            screen_height() - 80.0,
            120.0,
            50.0,
            MenuAction::OpenPlayerLevels,
        ));
    }

    fn load_player_levels(&mut self) {
        self.player_levels.clear();
        
        if Path::new("levels").exists() {
            if let Ok(entries) = fs::read_dir("levels") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "yaml" || extension == "yml" {
                            if let Some(file_name) = path.file_stem() {
                                if let Some(name_str) = file_name.to_str() {
                                    self.player_levels.push(name_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        self.player_levels.sort();
    }

    pub fn handle_input(&mut self) -> MenuAction {
        let (mouse_x, mouse_y) = mouse_position();
        
        // Handle scrolling in player levels menu
        if self.state == MenuState::PlayerLevels {
            let scroll_speed = 30.0;
            if mouse_wheel().1 > 0.0 {
                self.scroll_offset = (self.scroll_offset - scroll_speed).max(0.0);
            } else if mouse_wheel().1 < 0.0 {
                let max_scroll = (self.player_levels.len() as f32 * 60.0).max(0.0);
                self.scroll_offset = (self.scroll_offset + scroll_speed).min(max_scroll);
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            for button in &self.buttons {
                if button.is_clicked(mouse_x, mouse_y) {
                    return button.action.clone();
                }
            }
        }

        // Handle keyboard shortcuts
        if is_key_pressed(KeyCode::Escape) {
            match self.state {
                MenuState::MainMenu => return MenuAction::Exit,
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
                self.setup_settings_menu();
            },
            MenuAction::OpenPlayerLevels => {
                self.state = MenuState::PlayerLevels;
                self.setup_player_levels_menu();
            },
            MenuAction::BackToMain => {
                self.state = MenuState::MainMenu;
                self.setup_main_menu();
            },
            MenuAction::LoadPlayerLevel(_level_name) => {
                // Level loading will be handled by the main game loop
                self.state = MenuState::InGame;
            },
            _ => {}
        }
    }

    pub fn draw(&self) {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        match self.state {
            MenuState::MainMenu => self.draw_main_menu(),
            MenuState::Settings => self.draw_settings_menu(),
            MenuState::PlayerLevels => self.draw_player_levels_menu(),
            MenuState::InGame => {}, // Game drawing handled elsewhere
        }
    }

    fn draw_main_menu(&self) {
        // Draw background pattern
        self.draw_background();

        // Draw banner
        let banner_text = "Welcome to Robo Wars Crab Edition";
        let banner_size = 48.0;
        let banner_dimensions = measure_text(banner_text, None, banner_size as u16, 1.0);
        let banner_x = (screen_width() - banner_dimensions.width) / 2.0;
        let banner_y = screen_height() / 3.0;

        // Banner shadow
        draw_text(banner_text, banner_x + 2.0, banner_y + 2.0, banner_size, Color::new(0.0, 0.0, 0.0, 0.5));
        // Banner text
        draw_text(banner_text, banner_x, banner_y, banner_size, GOLD);

        // Draw subtitle
        let subtitle = "Program your robot to explore and conquer!";
        let subtitle_size = 20.0;
        let subtitle_dimensions = measure_text(subtitle, None, subtitle_size as u16, 1.0);
        let subtitle_x = (screen_width() - subtitle_dimensions.width) / 2.0;
        draw_text(subtitle, subtitle_x, banner_y + 60.0, subtitle_size, LIGHTGRAY);

        // Draw buttons
        for button in &self.buttons {
            button.draw();
        }

        // Draw version info
        draw_text("Version 2.0 - YAML Edition", 10.0, screen_height() - 10.0, 16.0, DARKGRAY);
    }

    fn draw_settings_menu(&self) {
        // Draw background
        self.draw_background();

        // Draw title
        let title = "Settings";
        let title_size = 36.0;
        let title_dimensions = measure_text(title, None, title_size as u16, 1.0);
        let title_x = (screen_width() - title_dimensions.width) / 2.0;
        draw_text(title, title_x, 150.0, title_size, WHITE);

        // Draw buttons
        for button in &self.buttons {
            button.draw();
        }

        // Draw instructions
        draw_text("Note: Some settings require restart to take effect", 50.0, screen_height() - 50.0, 16.0, GRAY);
    }

    fn draw_player_levels_menu(&self) {
        // Draw background
        self.draw_background();

        // Draw title
        let title = "Player Levels";
        let title_size = 36.0;
        let title_dimensions = measure_text(title, None, title_size as u16, 1.0);
        let title_x = (screen_width() - title_dimensions.width) / 2.0;
        draw_text(title, title_x, 100.0, title_size, WHITE);

        // Draw subtitle
        let subtitle = format!("Found {} custom levels", self.player_levels.len());
        let subtitle_size = 18.0;
        let subtitle_dimensions = measure_text(&subtitle, None, subtitle_size as u16, 1.0);
        let subtitle_x = (screen_width() - subtitle_dimensions.width) / 2.0;
        draw_text(&subtitle, subtitle_x, 140.0, subtitle_size, LIGHTGRAY);

        if self.player_levels.is_empty() {
            let no_levels_text = "No custom levels found. Create .yaml files in the 'levels' directory.";
            let no_levels_size = 16.0;
            let no_levels_dimensions = measure_text(no_levels_text, None, no_levels_size as u16, 1.0);
            let no_levels_x = (screen_width() - no_levels_dimensions.width) / 2.0;
            draw_text(no_levels_text, no_levels_x, screen_height() / 2.0, no_levels_size, GRAY);
        }

        // Draw buttons
        for button in &self.buttons {
            button.draw();
        }

        // Draw scroll indicator if needed
        if self.player_levels.len() > 8 {
            let scroll_bar_x = screen_width() - 20.0;
            let scroll_bar_height = 300.0;
            let scroll_bar_y = 200.0;
            
            // Scroll bar background
            draw_rectangle(scroll_bar_x, scroll_bar_y, 10.0, scroll_bar_height, DARKGRAY);
            
            // Scroll indicator
            let max_scroll = (self.player_levels.len() as f32 * 60.0).max(1.0);
            let scroll_ratio = self.scroll_offset / max_scroll;
            let indicator_height = 20.0;
            let indicator_y = scroll_bar_y + (scroll_bar_height - indicator_height) * scroll_ratio;
            
            draw_rectangle(scroll_bar_x, indicator_y, 10.0, indicator_height, WHITE);
        }
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
}