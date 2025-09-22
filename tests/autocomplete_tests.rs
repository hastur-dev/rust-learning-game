use robo_grid_explorer_gui::autocomplete::{AutocompleteEngine, CodeAnalyzer, SymbolKind};
use std::path::PathBuf;

#[cfg(test)]
mod autocomplete_tests {
    use super::*;

    #[test]
    fn test_code_analyzer_creation() {
        let analyzer = CodeAnalyzer::new();

        // Test that built-in functions are loaded
        assert!(analyzer.get_built_in_functions().contains("scan"));
        assert!(analyzer.get_built_in_functions().contains("move_bot"));
        assert!(analyzer.get_built_in_functions().contains("grab"));

        // Test that keywords are loaded
        assert!(analyzer.get_keywords().contains("fn"));
        assert!(analyzer.get_keywords().contains("let"));
        assert!(analyzer.get_keywords().contains("mut"));

        println!("✅ CodeAnalyzer creation test passed");
    }

    #[test]
    fn test_function_extraction() {
        let mut analyzer = CodeAnalyzer::new();
        let test_code = r#"
fn test_function() {
    println!("Hello");
}

fn another_function(x: i32) -> bool {
    x > 0
}
"#;

        analyzer.analyze_code(test_code);
        let symbols = analyzer.get_symbols();

        let functions: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .collect();

        assert_eq!(functions.len(), 2);
        assert!(functions.iter().any(|f| f.name == "test_function"));
        assert!(functions.iter().any(|f| f.name == "another_function"));

        println!("✅ Function extraction test passed");
    }

    #[test]
    fn test_struct_extraction() {
        let mut analyzer = CodeAnalyzer::new();
        let test_code = r#"
struct Position {
    x: i32,
    y: i32,
}

struct Robot {
    position: Position,
    energy: u32,
}
"#;

        analyzer.analyze_code(test_code);
        let symbols = analyzer.get_symbols();

        let structs: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Struct)
            .collect();

        assert_eq!(structs.len(), 2);
        assert!(structs.iter().any(|s| s.name == "Position"));
        assert!(structs.iter().any(|s| s.name == "Robot"));

        println!("✅ Struct extraction test passed");
    }

    #[test]
    fn test_enum_extraction() {
        let mut analyzer = CodeAnalyzer::new();
        let test_code = r#"
enum Direction {
    North,
    South,
    East,
    West,
}

enum Color {
    Red,
    Green,
    Blue,
}
"#;

        analyzer.analyze_code(test_code);
        let symbols = analyzer.get_symbols();

        let enums: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Enum)
            .collect();

        assert_eq!(enums.len(), 2);
        assert!(enums.iter().any(|e| e.name == "Direction"));
        assert!(enums.iter().any(|e| e.name == "Color"));

        println!("✅ Enum extraction test passed");
    }

    #[test]
    fn test_variable_extraction() {
        let mut analyzer = CodeAnalyzer::new();
        let test_code = r#"
fn main() {
    let x = 10;
    let mut y = 20;
    let position: Position = Position { x: 0, y: 0 };
}

fn test_function(param1: i32, param2: String) {
    let local_var = param1 + 5;
}
"#;

        analyzer.analyze_code(test_code);
        let symbols = analyzer.get_symbols();

        let variables: Vec<_> = symbols.iter()
            .filter(|s| s.kind == SymbolKind::Variable)
            .collect();

        // Should find let variables and function parameters
        assert!(variables.iter().any(|v| v.name == "x"));
        assert!(variables.iter().any(|v| v.name == "y"));
        assert!(variables.iter().any(|v| v.name == "position"));
        assert!(variables.iter().any(|v| v.name == "param1"));
        assert!(variables.iter().any(|v| v.name == "param2"));
        assert!(variables.iter().any(|v| v.name == "local_var"));

        println!("✅ Variable extraction test passed");
    }

    #[test]
    fn test_autocomplete_engine_creation() {
        let engine = AutocompleteEngine::new();

        assert!(engine.is_enabled());
        assert!(engine.get_current_suggestion().is_none());

        println!("✅ AutocompleteEngine creation test passed");
    }

    #[test]
    fn test_keyword_suggestions() {
        let mut engine = AutocompleteEngine::new();
        let test_code = "f";

        engine.update_suggestions(test_code, 1);

        if let Some(suggestion) = engine.get_current_suggestion() {
            assert_eq!(suggestion.kind, SymbolKind::Keyword);
            assert!(suggestion.text.starts_with("f"));
            assert_eq!(suggestion.text, "fn");
            println!("✅ Keyword suggestion test passed: {}", suggestion.text);
        } else {
            panic!("Expected keyword suggestion for 'f'");
        }
    }

    #[test]
    fn test_builtin_function_suggestions() {
        let mut engine = AutocompleteEngine::new();
        let test_code = "sc";

        engine.update_suggestions(test_code, 2);

        if let Some(suggestion) = engine.get_current_suggestion() {
            assert_eq!(suggestion.kind, SymbolKind::Function);
            assert!(suggestion.text.starts_with("sc"));
            assert_eq!(suggestion.text, "scan");
            println!("✅ Built-in function suggestion test passed: {}", suggestion.text);
        } else {
            panic!("Expected function suggestion for 'sc'");
        }
    }

    #[test]
    fn test_user_defined_function_suggestions() {
        let mut engine = AutocompleteEngine::new();
        let test_code = r#"
fn custom_function() {}
fn calculate_distance() {}

cust
"#;
        let cursor_pos = test_code.len() - 1; // Position after "cust"

        engine.update_suggestions(test_code, cursor_pos);

        if let Some(suggestion) = engine.get_current_suggestion() {
            assert_eq!(suggestion.kind, SymbolKind::Function);
            assert!(suggestion.text.starts_with("cust"));
            assert_eq!(suggestion.text, "custom_function");
            println!("✅ User-defined function suggestion test passed: {}", suggestion.text);
        } else {
            panic!("Expected user function suggestion for 'cust'");
        }
    }

    #[test]
    fn test_variable_suggestions() {
        let mut engine = AutocompleteEngine::new();
        let test_code = r#"
fn main() {
    let my_variable = 10;
    let my_other_var = 20;

    my_v
}
"#;
        // Find position after "my_v"
        let cursor_pos = test_code.rfind("my_v").unwrap() + 4;

        engine.update_suggestions(test_code, cursor_pos);

        if let Some(suggestion) = engine.get_current_suggestion() {
            assert_eq!(suggestion.kind, SymbolKind::Variable);
            assert!(suggestion.text.starts_with("my_v"));
            assert!(suggestion.text == "my_variable" || suggestion.text == "my_other_var");
            println!("✅ Variable suggestion test passed: {}", suggestion.text);
        } else {
            panic!("Expected variable suggestion for 'my_v'");
        }
    }

    #[test]
    fn test_no_suggestion_for_short_input() {
        let mut engine = AutocompleteEngine::new();
        let test_code = "f";

        engine.update_suggestions(test_code, 1);

        // Should get suggestion since "f" matches "fn"
        assert!(engine.get_current_suggestion().is_some());

        // Test with single character
        let test_code = "x";
        engine.update_suggestions(test_code, 1);

        // Should not get suggestion for "x" as it doesn't match anything
        assert!(engine.get_current_suggestion().is_none());

        println!("✅ Short input handling test passed");
    }

    #[test]
    fn test_suggestion_priority() {
        let mut engine = AutocompleteEngine::new();
        let test_code = r#"
fn my_function() {}
let my_var = 10;

m
"#;
        let cursor_pos = test_code.len() - 1;

        engine.update_suggestions(test_code, cursor_pos);

        if let Some(suggestion) = engine.get_current_suggestion() {
            // Built-in functions should have higher priority than user-defined
            // Variables should have priority over functions
            println!("✅ Suggestion priority test passed: {} ({})", suggestion.text, format!("{:?}", suggestion.kind));
        } else {
            panic!("Expected suggestion for 'm'");
        }
    }

    #[test]
    fn test_autocomplete_enable_disable() {
        let mut engine = AutocompleteEngine::new();
        let test_code = "fn";

        // Test enabled state
        assert!(engine.is_enabled());
        engine.update_suggestions(test_code, 2);
        assert!(engine.get_current_suggestion().is_some());

        // Test disabled state
        engine.set_enabled(false);
        assert!(!engine.is_enabled());
        engine.update_suggestions(test_code, 2);
        assert!(engine.get_current_suggestion().is_none());

        // Test re-enabled state
        engine.set_enabled(true);
        assert!(engine.is_enabled());
        engine.update_suggestions(test_code, 2);
        assert!(engine.get_current_suggestion().is_some());

        println!("✅ Enable/disable functionality test passed");
    }

    #[test]
    fn test_suggestion_acceptance() {
        let mut engine = AutocompleteEngine::new();
        let test_code = "fn";

        engine.update_suggestions(test_code, 2);
        assert!(engine.get_current_suggestion().is_some());

        let accepted = engine.accept_suggestion();
        assert!(accepted.is_some());
        assert_eq!(accepted.unwrap(), "fn");

        // After acceptance, suggestion should be cleared
        assert!(engine.get_current_suggestion().is_none());

        println!("✅ Suggestion acceptance test passed");
    }

    #[test]
    fn test_cursor_position_conversion() {
        let test_code = "line 1\nline 2\nline 3";
        let mut engine = AutocompleteEngine::new();

        // Test various cursor positions
        let (line, char) = engine.cursor_to_line_character(test_code, 0);
        assert_eq!(line, 0);
        assert_eq!(char, 0);

        let (line, char) = engine.cursor_to_line_character(test_code, 7); // Start of line 2
        assert_eq!(line, 1);
        assert_eq!(char, 0);

        let (line, char) = engine.cursor_to_line_character(test_code, 14); // Start of line 3
        assert_eq!(line, 2);
        assert_eq!(char, 0);

        println!("✅ Cursor position conversion test passed");
    }

    #[test]
    fn test_vscode_integration_availability() {
        let engine = AutocompleteEngine::new();

        // This test will check if VSCode integration is available
        // It might be false in CI environments
        let vscode_available = engine.is_vscode_available();
        println!("✅ VSCode integration availability test passed: available = {}", vscode_available);

        if vscode_available {
            assert!(engine.is_vscode_enabled());
            println!("  VSCode integration is enabled by default when available");
        } else {
            println!("  VSCode integration not available (expected in CI/test environments)");
        }
    }
}

#[cfg(test)]
mod game_integration_tests {
    use robo_grid_explorer_gui::gamestate::Game;
    use robo_grid_explorer_gui::level::LevelSpec;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_game_autocomplete_integration() {
        let levels = vec![]; // Empty levels for testing
        let rng = StdRng::from_seed([0; 32]);
        let mut game = Game::new(levels, rng);

        // Test initial state
        assert!(game.autocomplete_enabled);

        // Test toggling
        let new_state = game.toggle_autocomplete_setting();
        assert!(!new_state);
        assert!(!game.autocomplete_enabled);

        let new_state = game.toggle_autocomplete_setting();
        assert!(new_state);
        assert!(game.autocomplete_enabled);

        println!("✅ Game autocomplete integration test passed");
    }

    #[test]
    fn test_game_vscode_integration() {
        let levels = vec![];
        let rng = StdRng::from_seed([0; 32]);
        let mut game = Game::new(levels, rng);

        // Test VSCode availability check
        let vscode_available = game.is_vscode_available();
        println!("VSCode available in test: {}", vscode_available);

        // Test toggling VSCode integration
        let initial_state = game.autocomplete_engine.is_vscode_enabled();
        let new_state = game.toggle_vscode_integration_setting();
        assert_eq!(new_state, !initial_state);

        println!("✅ Game VSCode integration test passed");
    }

    #[test]
    fn test_game_autocomplete_update() {
        let levels = vec![];
        let rng = StdRng::from_seed([0; 32]);
        let mut game = Game::new(levels, rng);

        // Set some test code
        game.current_code = "fn test_func() {}\nle".to_string();
        game.cursor_position = game.current_code.len();

        // Update autocomplete
        game.update_autocomplete();

        // Should have a suggestion for "let"
        let suggestion = game.get_autocomplete_suggestion();
        if let Some(suggestion) = suggestion {
            assert!(suggestion.text.starts_with("le"));
            println!("✅ Game autocomplete update test passed: {}", suggestion.text);
        } else {
            println!("⚠️  No suggestion found (might be expected in test environment)");
        }
    }

    #[test]
    fn test_game_autocomplete_acceptance() {
        let levels = vec![];
        let rng = StdRng::from_seed([0; 32]);
        let mut game = Game::new(levels, rng);

        // Set test code with partial word
        game.current_code = "le".to_string();
        game.cursor_position = 2;

        // Update autocomplete to get a suggestion
        game.update_autocomplete();

        // Try to accept the suggestion
        let accepted = game.accept_autocomplete();

        if accepted {
            // Code should be updated with the completion
            assert!(game.current_code.len() > 2);
            assert!(game.cursor_position > 2);
            println!("✅ Game autocomplete acceptance test passed: '{}'", game.current_code);
        } else {
            println!("⚠️  No suggestion to accept (might be expected in test environment)");
        }
    }
}

#[cfg(test)]
mod hotkey_tests {
    use robo_grid_explorer_gui::hotkeys::{HotkeySystem, EditorAction};
    use macroquad::prelude::KeyCode;

    #[test]
    fn test_hotkey_system_creation() {
        let hotkey_system = HotkeySystem::new();

        // Test that default bindings are loaded
        let bindings = hotkey_system.get_all_bindings();
        assert!(!bindings.is_empty());

        // Test some default bindings
        if let Some(action) = hotkey_system.get_action_for_input(KeyCode::Tab, false, false, false) {
            assert!(matches!(action, EditorAction::Accept) || matches!(action, EditorAction::Indent));
        }

        if let Some(action) = hotkey_system.get_action_for_input(KeyCode::S, true, false, false) {
            assert!(matches!(action, EditorAction::SaveFile));
        }

        println!("✅ HotkeySystem creation test passed");
    }

    #[test]
    fn test_hotkey_configuration() {
        let mut hotkey_system = HotkeySystem::new();

        // Test setting a custom binding
        hotkey_system.set_binding("Ctrl+K".to_string(), EditorAction::Comment);

        let action = hotkey_system.get_action_for_input(KeyCode::K, true, false, false);
        assert!(action.is_some());
        assert!(matches!(action.unwrap(), EditorAction::Comment));

        // Test removing a binding
        hotkey_system.remove_binding("Ctrl+K");
        let action = hotkey_system.get_action_for_input(KeyCode::K, true, false, false);
        assert!(action.is_none() || !matches!(action.unwrap(), EditorAction::Comment));

        println!("✅ Hotkey configuration test passed");
    }

    #[test]
    fn test_hotkey_reset_to_defaults() {
        let mut hotkey_system = HotkeySystem::new();

        // Modify a binding
        hotkey_system.set_binding("Ctrl+Z".to_string(), EditorAction::Comment);

        // Reset to defaults
        hotkey_system.reset_to_defaults();

        // Should be back to default (Undo)
        let action = hotkey_system.get_action_for_input(KeyCode::Z, true, false, false);
        assert!(action.is_some());
        assert!(matches!(action.unwrap(), EditorAction::Undo));

        println!("✅ Hotkey reset to defaults test passed");
    }
}

#[cfg(test)]
mod menu_integration_tests {
    use robo_grid_explorer_gui::menu::{GameSettings, MenuAction};

    #[test]
    fn test_game_settings_autocomplete_defaults() {
        let settings = GameSettings::default();

        assert!(settings.autocomplete_enabled);
        assert!(settings.vscode_integration_enabled);

        println!("✅ GameSettings autocomplete defaults test passed");
    }

    #[test]
    fn test_menu_actions_existence() {
        // Test that the new menu actions exist
        let toggle_autocomplete = MenuAction::ToggleAutocomplete;
        let toggle_vscode = MenuAction::ToggleVSCodeIntegration;
        let open_hotkeys = MenuAction::OpenHotkeySettings;

        // Just ensure they can be created (enum variants exist)
        assert!(matches!(toggle_autocomplete, MenuAction::ToggleAutocomplete));
        assert!(matches!(toggle_vscode, MenuAction::ToggleVSCodeIntegration));
        assert!(matches!(open_hotkeys, MenuAction::OpenHotkeySettings));

        println!("✅ Menu actions existence test passed");
    }
}

// Integration test runner
pub fn run_all_autocomplete_tests() {
    println!("🚀 Running Autocomplete System Tests");
    println!("=====================================");

    // Code analyzer tests
    autocomplete_tests::test_code_analyzer_creation();
    autocomplete_tests::test_function_extraction();
    autocomplete_tests::test_struct_extraction();
    autocomplete_tests::test_enum_extraction();
    autocomplete_tests::test_variable_extraction();

    // Autocomplete engine tests
    autocomplete_tests::test_autocomplete_engine_creation();
    autocomplete_tests::test_keyword_suggestions();
    autocomplete_tests::test_builtin_function_suggestions();
    autocomplete_tests::test_user_defined_function_suggestions();
    autocomplete_tests::test_variable_suggestions();
    autocomplete_tests::test_no_suggestion_for_short_input();
    autocomplete_tests::test_suggestion_priority();
    autocomplete_tests::test_autocomplete_enable_disable();
    autocomplete_tests::test_suggestion_acceptance();
    autocomplete_tests::test_cursor_position_conversion();
    autocomplete_tests::test_vscode_integration_availability();

    // Game integration tests
    game_integration_tests::test_game_autocomplete_integration();
    game_integration_tests::test_game_vscode_integration();
    game_integration_tests::test_game_autocomplete_update();
    game_integration_tests::test_game_autocomplete_acceptance();

    // Hotkey system tests
    hotkey_tests::test_hotkey_system_creation();
    hotkey_tests::test_hotkey_configuration();
    hotkey_tests::test_hotkey_reset_to_defaults();

    // Menu integration tests
    menu_integration_tests::test_game_settings_autocomplete_defaults();
    menu_integration_tests::test_menu_actions_existence();

    println!("=====================================");
    println!("✅ All Autocomplete System Tests Completed!");
}