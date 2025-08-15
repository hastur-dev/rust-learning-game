use macroquad::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MenuState {
    MainMenu,
    Settings,
    InGame,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenuAction {
    None,
    StartGame,
    OpenSettings,
    BackToMain,
    Exit,
    IncreaseResolution,
    DecreaseResolution,
    ToggleFullscreen,
    IncreaseSfxVolume,
    DecreaseSfxVolume,
    IncreaseMusicVolume,
    DecreaseMusicVolume,
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
    pub scroll_offset: f32,
}

impl Menu {
    pub fn new() -> Self {
        let mut menu = Self {
            state: MenuState::MainMenu,
            buttons: Vec::new(),
            settings: GameSettings::default(),
            scroll_offset: 0.0,
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
            "Exit".to_string(),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 2.0,
            button_width,
            button_height,
            MenuAction::Exit,
        ));
    }

    pub fn setup_settings_menu(&mut self) {
        self.buttons.clear();
        
        let screen_center_x = screen_width() / 2.0;
        let button_width = 400.0;
        let button_height = 50.0;
        let button_spacing = 70.0;
        let start_y = screen_height() / 2.0 - 100.0;

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

        // Volume controls
        self.buttons.push(MenuButton::new(
            format!("SFX Volume: {:.0}% (Click: +10%, Right-Click: -10%)", 
                   self.settings.sfx_volume * 100.0),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 2.0,
            button_width,
            button_height,
            MenuAction::IncreaseSfxVolume,
        ));

        self.buttons.push(MenuButton::new(
            format!("Music Volume: {:.0}% (Click: +10%, Right-Click: -10%)", 
                   self.settings.music_volume * 100.0),
            screen_center_x - button_width / 2.0,
            start_y + button_spacing * 3.0,
            button_width,
            button_height,
            MenuAction::IncreaseMusicVolume,
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


    pub fn handle_input(&mut self) -> MenuAction {
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
            MenuAction::BackToMain => {
                self.state = MenuState::MainMenu;
                self.setup_main_menu();
            },
            MenuAction::IncreaseResolution => {
                let resolutions = Self::get_available_resolutions();
                let current_index = self.current_resolution_index();
                let next_index = (current_index + 1) % resolutions.len();
                let (width, height) = resolutions[next_index];
                self.settings.window_width = width;
                self.settings.window_height = height;
                self.setup_settings_menu(); // Refresh the menu
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
                self.setup_settings_menu(); // Refresh the menu
            },
            MenuAction::ToggleFullscreen => {
                self.settings.fullscreen = !self.settings.fullscreen;
                self.setup_settings_menu(); // Refresh the menu
            },
            MenuAction::IncreaseSfxVolume => {
                self.settings.sfx_volume = (self.settings.sfx_volume + 0.1).min(1.0);
                self.setup_settings_menu(); // Refresh the menu
            },
            MenuAction::DecreaseSfxVolume => {
                self.settings.sfx_volume = (self.settings.sfx_volume - 0.1).max(0.0);
                self.setup_settings_menu(); // Refresh the menu
            },
            MenuAction::IncreaseMusicVolume => {
                self.settings.music_volume = (self.settings.music_volume + 0.1).min(1.0);
                self.setup_settings_menu(); // Refresh the menu
            },
            MenuAction::DecreaseMusicVolume => {
                self.settings.music_volume = (self.settings.music_volume - 0.1).max(0.0);
                self.setup_settings_menu(); // Refresh the menu
            },
            _ => {}
        }
    }

    pub fn draw(&self) {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        match self.state {
            MenuState::MainMenu => self.draw_main_menu(),
            MenuState::Settings => self.draw_settings_menu(),
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
        draw_text(title, title_x, 100.0, title_size, WHITE);

        // Draw instructions
        let instructions = "Left Click: Increase/Next | Right Click: Decrease/Previous";
        let inst_size = 18.0;
        let inst_dimensions = measure_text(instructions, None, inst_size as u16, 1.0);
        let inst_x = (screen_width() - inst_dimensions.width) / 2.0;
        draw_text(instructions, inst_x, 140.0, inst_size, YELLOW);

        // Draw buttons
        for button in &self.buttons {
            button.draw();
        }

        // Draw footer notes
        draw_text("Note: Window resolution changes require restart to take effect", 50.0, screen_height() - 70.0, 14.0, GRAY);
        draw_text("Volume and fullscreen changes apply immediately", 50.0, screen_height() - 50.0, 14.0, GRAY);
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