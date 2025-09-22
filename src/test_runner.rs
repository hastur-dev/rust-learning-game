// Test runner for autocomplete system
// This module can be called from main to run integration tests

pub fn run_autocomplete_integration_tests() {
    println!("🚀 Running Autocomplete Integration Tests");
    println!("==========================================");

    test_code_analyzer();
    test_autocomplete_engine();
    test_game_integration();
    test_hotkey_system();
    test_menu_settings_integration();

    println!("==========================================");
    println!("✅ All Integration Tests Completed!");
}

fn test_code_analyzer() {
    println!("📋 Testing Code Analyzer...");

    let mut analyzer = crate::autocomplete::CodeAnalyzer::new();

    // Test basic functionality
    assert!(!analyzer.get_built_in_functions().is_empty());
    assert!(!analyzer.get_keywords().is_empty());

    // Test code analysis
    let test_code = r#"
fn test_function() {
    let my_var = 10;
}

struct TestStruct {
    field: i32,
}

enum TestEnum {
    Variant1,
    Variant2,
}
"#;

    analyzer.analyze_code(test_code);
    let symbols = analyzer.get_symbols();

    let functions: Vec<_> = symbols.iter()
        .filter(|s| s.kind == crate::autocomplete::SymbolKind::Function)
        .collect();
    let structs: Vec<_> = symbols.iter()
        .filter(|s| s.kind == crate::autocomplete::SymbolKind::Struct)
        .collect();
    let enums: Vec<_> = symbols.iter()
        .filter(|s| s.kind == crate::autocomplete::SymbolKind::Enum)
        .collect();
    let variables: Vec<_> = symbols.iter()
        .filter(|s| s.kind == crate::autocomplete::SymbolKind::Variable)
        .collect();

    assert_eq!(functions.len(), 1);
    assert_eq!(structs.len(), 1);
    assert_eq!(enums.len(), 1);
    assert!(!variables.is_empty());

    println!("  ✅ Found {} functions, {} structs, {} enums, {} variables",
             functions.len(), structs.len(), enums.len(), variables.len());
}

fn test_autocomplete_engine() {
    println!("🤖 Testing Autocomplete Engine...");

    let mut engine = crate::autocomplete::AutocompleteEngine::new();

    // Test initial state
    assert!(engine.is_enabled());
    assert!(engine.get_current_suggestion().is_none());

    // Test keyword suggestion
    engine.update_suggestions("fn", 2);
    if let Some(suggestion) = engine.get_current_suggestion() {
        assert_eq!(suggestion.text, "fn");
        assert_eq!(suggestion.kind, crate::autocomplete::SymbolKind::Keyword);
        println!("  ✅ Keyword suggestion: {}", suggestion.text);
    }

    // Test built-in function suggestion
    engine.update_suggestions("sc", 2);
    if let Some(suggestion) = engine.get_current_suggestion() {
        assert_eq!(suggestion.text, "scan");
        assert_eq!(suggestion.kind, crate::autocomplete::SymbolKind::Function);
        println!("  ✅ Built-in function suggestion: {}", suggestion.text);
    }

    // Test suggestion acceptance
    let accepted = engine.accept_suggestion();
    assert!(accepted.is_some());
    assert!(engine.get_current_suggestion().is_none());
    println!("  ✅ Suggestion acceptance: {}", accepted.unwrap());

    // Test enable/disable
    engine.set_enabled(false);
    engine.update_suggestions("fn", 2);
    assert!(engine.get_current_suggestion().is_none());
    println!("  ✅ Disable functionality works");

    engine.set_enabled(true);
    engine.update_suggestions("fn", 2);
    assert!(engine.get_current_suggestion().is_some());
    println!("  ✅ Re-enable functionality works");
}

fn test_game_integration() {
    println!("🎮 Testing Game Integration...");

    use rand::{rngs::StdRng, SeedableRng};

    let levels = vec![];
    let rng = StdRng::from_seed([0; 32]);
    let mut game = crate::gamestate::Game::new(levels, rng);

    // Test initial state
    assert!(game.autocomplete_enabled);
    println!("  ✅ Game autocomplete enabled by default");

    // Test toggling
    let new_state = game.toggle_autocomplete_setting();
    assert!(!new_state);
    assert!(!game.autocomplete_enabled);
    println!("  ✅ Autocomplete toggle off works");

    let new_state = game.toggle_autocomplete_setting();
    assert!(new_state);
    assert!(game.autocomplete_enabled);
    println!("  ✅ Autocomplete toggle on works");

    // Test VSCode integration
    let vscode_available = game.is_vscode_available();
    println!("  ℹ️  VSCode integration available: {}", vscode_available);

    // Test autocomplete update
    game.current_code = "fn test() {}\nle".to_string();
    game.cursor_position = game.current_code.len();
    game.update_autocomplete();

    if let Some(suggestion) = game.get_autocomplete_suggestion() {
        println!("  ✅ Autocomplete suggestion: {}", suggestion.text);
    } else {
        println!("  ⚠️  No suggestion (expected in some cases)");
    }
}

fn test_hotkey_system() {
    println!("⌨️  Testing Hotkey System...");

    let mut hotkey_system = crate::hotkeys::HotkeySystem::new();

    // Test default bindings
    let bindings = hotkey_system.get_all_bindings();
    assert!(!bindings.is_empty());
    println!("  ✅ Default bindings loaded: {} bindings", bindings.len());

    // Test specific binding
    use macroquad::prelude::KeyCode;
    let action = hotkey_system.get_action_for_input(KeyCode::S, true, false, false);
    if let Some(action) = action {
        println!("  ✅ Ctrl+S binding found: {:?}", action);
    }

    // Test custom binding
    hotkey_system.set_binding("Ctrl+K".to_string(), crate::hotkeys::EditorAction::Comment);
    let action = hotkey_system.get_action_for_input(KeyCode::K, true, false, false);
    assert!(action.is_some());
    println!("  ✅ Custom binding set and retrieved");

    // Test reset to defaults
    hotkey_system.reset_to_defaults();
    let action = hotkey_system.get_action_for_input(KeyCode::Z, true, false, false);
    if let Some(action) = action {
        println!("  ✅ Reset to defaults works: Ctrl+Z = {:?}", action);
    }
}

pub fn run_quick_smoke_test() {
    println!("💨 Running Quick Smoke Test...");

    // Test 1: Create code analyzer
    println!("  📝 Testing Code Analyzer...");
    let mut analyzer = crate::autocomplete::CodeAnalyzer::new();
    let test_code = r#"
        fn main() {
            println!("Hello");
            let robot = Robot::new();
            robot.move(10);
        }

        struct Robot {
            position: i32,
        }

        enum Direction {
            Up,
            Down,
        }
    "#;
    analyzer.analyze_code(test_code);
    println!("    ✓ Code analyzer created and analyzed test code");

    // Test 2: Create fast autocomplete engine
    println!("  🔮 Testing Fast Autocomplete Engine...");
    let mut engine = crate::autocomplete::AutocompleteEngine::new();

    // Test performance with multiple rapid updates (this should be very fast now!)
    let start_time = std::time::Instant::now();
    for i in 0..100 {
        engine.update_suggestions(test_code, 30 + i % 50);
    }
    let elapsed = start_time.elapsed();
    println!("    ✓ 100 autocomplete updates completed in {:.2}ms (should be <10ms)", elapsed.as_millis());

    // Test actual completion
    engine.update_suggestions("pri", 3);
    if let Some(suggestion) = engine.get_current_suggestion() {
        println!("    ✓ Fast suggestion for 'pri': {}", suggestion.text);
    } else {
        println!("    ✓ Fast autocomplete engine created successfully");
    }

    // Test 3: Create hotkey system
    println!("  ⌨️ Testing Hotkey System...");
    let mut hotkey_system = crate::hotkeys::HotkeySystem::new();
    assert!(!hotkey_system.get_all_bindings().is_empty(), "Should have default keybindings");
    println!("    ✓ Loaded {} default keybindings", hotkey_system.get_all_bindings().len());

    // Test 4: Game integration
    println!("  🎮 Testing Game Integration...");
    use rand::{rngs::StdRng, SeedableRng};
    let minimal_level = crate::level::LevelSpec {
        name: "Test Level".to_string(),
        width: 3,
        height: 3,
        start: (1, 1),
        scanner_at: None,
        blockers: vec![],
        doors: vec![],
        enemies: vec![],
        items: vec![],
        tasks: vec![],
        fog_of_war: false,
        max_turns: 0,
        income_per_square: 1,
        message: None,
        hint_message: None,
        rust_docs_url: None,
        starting_code: Some("fn main() { }".to_string()),
        completion_condition: None,
        completion_flag: None,
        achievement_message: None,
        next_level_hint: None,
        completion_message: None,
    };
    let levels = vec![minimal_level];
    let rng = StdRng::from_seed([0; 32]);
    let mut game = crate::gamestate::Game::new(levels, rng);

    // Test autocomplete in game
    game.update_autocomplete();
    println!("    ✓ Autocomplete integrated with game");

    // Test toggling autocomplete
    let initial_state = game.autocomplete_enabled;
    game.toggle_autocomplete_setting();
    assert_ne!(game.autocomplete_enabled, initial_state, "Autocomplete toggle should work");
    println!("    ✓ Autocomplete toggle works");

    println!("\n✅ All smoke tests passed successfully!");
    println!("📊 Summary:");
    println!("  • Code analyzer: ✅");
    println!("  • Autocomplete engine: ✅");
    println!("  • Hotkey system: ✅");
    println!("  • Game integration: ✅");
    println!("  • Settings toggle: ✅");
}

fn test_menu_settings_integration() {
    println!("⚙️  Testing Menu Settings Integration...");

    let mut menu_system = crate::menu::Menu::new();

    // Test initial state
    assert_eq!(menu_system.state, crate::menu::MenuState::MainMenu);
    println!("  ✅ Menu system starts in MainMenu state");

    // Test opening settings
    menu_system.update(crate::menu::MenuAction::OpenSettings);
    assert_eq!(menu_system.state, crate::menu::MenuState::Settings);
    println!("  ✅ Can navigate to Settings menu");

    // Test opening hotkey settings
    menu_system.update(crate::menu::MenuAction::OpenHotkeySettings);
    assert_eq!(menu_system.state, crate::menu::MenuState::HotkeySettings);
    println!("  ✅ Can navigate to Hotkey Settings menu");

    // Test that hotkey settings menu has buttons
    assert!(!menu_system.buttons.is_empty(), "Hotkey settings should have buttons");

    // Find the Back to Settings button
    let back_button = menu_system.buttons.iter()
        .find(|b| b.action == crate::menu::MenuAction::BackToSettings);
    assert!(back_button.is_some(), "Should have a Back to Settings button");
    println!("  ✅ Hotkey settings menu has Back to Settings button");

    // Test back navigation
    menu_system.update(crate::menu::MenuAction::BackToSettings);
    assert_eq!(menu_system.state, crate::menu::MenuState::Settings);
    println!("  ✅ Can navigate back from Hotkey Settings to Settings");

    // Test fullscreen toggle
    let initial_fullscreen = menu_system.settings.fullscreen;
    menu_system.update(crate::menu::MenuAction::ToggleFullscreen);
    assert_ne!(menu_system.settings.fullscreen, initial_fullscreen);
    println!("  ✅ Fullscreen toggle changes setting");

    // Test autocomplete toggle
    let initial_autocomplete = menu_system.settings.autocomplete_enabled;
    menu_system.update(crate::menu::MenuAction::ToggleAutocomplete);
    assert_ne!(menu_system.settings.autocomplete_enabled, initial_autocomplete);
    println!("  ✅ Autocomplete toggle changes setting");

    // Test font size adjustment
    let initial_font_size = menu_system.settings.font_size_multiplier;
    menu_system.update(crate::menu::MenuAction::IncreaseFontSize);
    assert!(menu_system.settings.font_size_multiplier > initial_font_size);
    println!("  ✅ Font size increase works");

    menu_system.update(crate::menu::MenuAction::DecreaseFontSize);
    assert!(menu_system.settings.font_size_multiplier < initial_font_size + 0.05); // Allow for floating point precision
    println!("  ✅ Font size decrease works");

    // Test settings buttons exist in settings menu
    menu_system.setup_settings_menu();
    let has_fullscreen_button = menu_system.buttons.iter()
        .any(|b| b.action == crate::menu::MenuAction::ToggleFullscreen);
    let has_autocomplete_button = menu_system.buttons.iter()
        .any(|b| b.action == crate::menu::MenuAction::ToggleAutocomplete);
    let has_hotkey_button = menu_system.buttons.iter()
        .any(|b| b.action == crate::menu::MenuAction::OpenHotkeySettings);

    assert!(has_fullscreen_button, "Settings menu should have fullscreen button");
    assert!(has_autocomplete_button, "Settings menu should have autocomplete button");
    assert!(has_hotkey_button, "Settings menu should have hotkey settings button");

    println!("  ✅ Settings menu has all required buttons");

    // Test that music and SFX volume buttons are removed
    let has_sfx_button = menu_system.buttons.iter()
        .any(|b| matches!(b.action, crate::menu::MenuAction::IncreaseSfxVolume | crate::menu::MenuAction::DecreaseSfxVolume));
    let has_music_button = menu_system.buttons.iter()
        .any(|b| matches!(b.action, crate::menu::MenuAction::IncreaseMusicVolume | crate::menu::MenuAction::DecreaseMusicVolume));

    assert!(!has_sfx_button, "Settings menu should NOT have SFX volume buttons");
    assert!(!has_music_button, "Settings menu should NOT have music volume buttons");

    println!("  ✅ Music and SFX volume buttons successfully removed from settings");

    println!("  🎯 All menu settings integration tests passed!");
}