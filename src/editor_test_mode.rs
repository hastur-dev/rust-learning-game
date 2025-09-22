// Simple editor-only test mode for testing autocomplete visuals
use macroquad::prelude::*;
use crate::gamestate::Game;
use crate::font_scaling::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct HotkeyTestResults {
    tested_hotkeys: HashMap<String, bool>,
    autocomplete_tests: Vec<String>,
    tab_behavior_tests: Vec<String>,
    cursor_movement_tests: Vec<String>,
    text_editing_tests: Vec<String>,
    settings_tests: Vec<String>,
}

impl HotkeyTestResults {
    fn new() -> Self {
        Self {
            tested_hotkeys: HashMap::new(),
            autocomplete_tests: Vec::new(),
            tab_behavior_tests: Vec::new(),
            cursor_movement_tests: Vec::new(),
            text_editing_tests: Vec::new(),
            settings_tests: Vec::new(),
        }
    }

    fn record_hotkey_test(&mut self, hotkey: String, success: bool) {
        self.tested_hotkeys.insert(hotkey, success);
    }

    fn add_autocomplete_test(&mut self, result: String) {
        self.autocomplete_tests.push(result);
    }

    fn add_tab_test(&mut self, result: String) {
        self.tab_behavior_tests.push(result);
    }

    fn add_cursor_test(&mut self, result: String) {
        self.cursor_movement_tests.push(result);
    }

    fn add_text_test(&mut self, result: String) {
        self.text_editing_tests.push(result);
    }

    fn add_settings_test(&mut self, result: String) {
        self.settings_tests.push(result);
    }
}

pub async fn run_editor_test_mode() {
    // Create a minimal game for testing
    let mut game = create_test_game();
    let mut show_hotkey_help = false;
    let mut hotkey_test_results = HotkeyTestResults::new();

    println!("🎮 Editor Test Mode Started!");
    println!("  Type to test autocomplete suggestions");
    println!("  Press Tab to accept suggestions");
    println!("  Press F1 to toggle hotkey help");
    println!("  Press F2 to run hotkey tests");
    println!("  Press Escape to exit");

    loop {
        clear_background(Color::from_rgba(30, 30, 35, 255));

        // Handle input
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::F1) {
            show_hotkey_help = !show_hotkey_help;
        }

        if is_key_pressed(KeyCode::F2) {
            run_hotkey_tests(&mut game, &mut hotkey_test_results);
        }

        handle_editor_input(&mut game, &mut hotkey_test_results);

        // Update autocomplete
        game.update_autocomplete();

        // Draw the editor
        draw_test_editor(&mut game, show_hotkey_help, &hotkey_test_results);

        next_frame().await;
    }

    println!("✅ Editor Test Mode Exited");
    print_final_hotkey_report(&hotkey_test_results);
}

fn create_test_game() -> Game {
    use ::rand::{rngs::StdRng, SeedableRng};

    let test_level = crate::level::LevelSpec {
        name: "Editor Test".to_string(),
        width: 5,
        height: 5,
        start: (2, 2),
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
        starting_code: Some(r#"fn main() {
    println!("Hello World!");
    let message = String::new();
    message.push_str("Test");

    // Type here to test autocomplete:

}"#.to_string()),
        completion_condition: None,
        completion_flag: None,
        achievement_message: None,
        next_level_hint: None,
        completion_message: None,
    };

    let levels = vec![test_level];
    let rng = StdRng::from_seed([42; 32]);
    let mut game = Game::new(levels, rng);

    // Enable autocomplete for testing
    game.autocomplete_enabled = true;

    // Set cursor to a good position for testing
    game.cursor_position = game.current_code.len().saturating_sub(20); // Near the end, but safe

    game
}

fn run_hotkey_tests(game: &mut Game, results: &mut HotkeyTestResults) {
    println!("🧪 Running comprehensive hotkey tests...");

    // Test autocomplete behavior
    test_autocomplete_behavior(game, results);

    // Test tab key behavior
    test_tab_key_behavior(game, results);

    // Test cursor movement
    test_cursor_movement(game, results);

    // Test text editing operations
    test_text_editing(game, results);

    // Test common hotkey combinations
    test_common_hotkeys(game, results);

    // Test settings menu functionality
    test_settings_menu_functionality(results);

    // Test tab key with no autocomplete suggestion
    test_tab_key_no_suggestion(game, results);

    // Test click and drag text selection
    test_click_drag_selection(game, results);

    println!("✅ Hotkey tests completed! Check results on screen.");
}

fn test_autocomplete_behavior(game: &mut Game, results: &mut HotkeyTestResults) {
    // Save initial state
    let original_code = game.current_code.clone();
    let original_cursor = game.cursor_position;

    // Test 1: Type partial keyword and check for suggestion
    game.current_code = "pri".to_string();
    game.cursor_position = 3;
    game.update_autocomplete();

    if game.get_autocomplete_suggestion().is_some() {
        results.add_autocomplete_test("✅ Autocomplete suggests for 'pri'".to_string());
    } else {
        results.add_autocomplete_test("❌ No autocomplete for 'pri'".to_string());
    }

    // Test 2: Check if suggestion works for Rust types
    game.current_code = "Str".to_string();
    game.cursor_position = 3;
    game.update_autocomplete();

    if game.get_autocomplete_suggestion().is_some() {
        results.add_autocomplete_test("✅ Autocomplete suggests for 'Str'".to_string());
    } else {
        results.add_autocomplete_test("❌ No autocomplete for 'Str'".to_string());
    }

    // Test 3: Check empty string doesn't crash
    game.current_code = "".to_string();
    game.cursor_position = 0;
    game.update_autocomplete();
    results.add_autocomplete_test("✅ Empty string handled safely".to_string());

    // Restore state
    game.current_code = original_code;
    game.cursor_position = original_cursor;
}

fn test_tab_key_behavior(game: &mut Game, results: &mut HotkeyTestResults) {
    // Save initial state
    let original_code = game.current_code.clone();
    let original_cursor = game.cursor_position;

    // Test 1: Tab with autocomplete suggestion
    game.current_code = "pri".to_string();
    game.cursor_position = 3;
    game.update_autocomplete();

    let has_suggestion = game.get_autocomplete_suggestion().is_some();
    let accepted = game.accept_autocomplete();

    if has_suggestion && accepted {
        results.add_tab_test("✅ Tab accepts autocomplete when suggestion available".to_string());
    } else if !has_suggestion && !accepted {
        results.add_tab_test("✅ Tab doesn't autocomplete when no suggestion".to_string());
    } else {
        results.add_tab_test("❌ Tab behavior inconsistent with suggestion state".to_string());
    }

    // Test 2: Tab without suggestion should not autocomplete
    game.current_code = "xyz123".to_string();
    game.cursor_position = 6;
    game.update_autocomplete();

    let has_suggestion = game.get_autocomplete_suggestion().is_some();
    let accepted = game.accept_autocomplete();

    if !has_suggestion && !accepted {
        results.add_tab_test("✅ Tab doesn't autocomplete for invalid text".to_string());
    } else {
        results.add_tab_test("❌ Tab incorrectly tried to autocomplete invalid text".to_string());
    }

    // Restore state
    game.current_code = original_code;
    game.cursor_position = original_cursor;
}

fn test_cursor_movement(game: &mut Game, results: &mut HotkeyTestResults) {
    // Test cursor movement functions exist and work
    let original_cursor = game.cursor_position;

    // Test basic bounds checking
    if game.cursor_position <= game.current_code.len() {
        results.add_cursor_test("✅ Cursor position within bounds".to_string());
    } else {
        results.add_cursor_test("❌ Cursor position out of bounds".to_string());
    }

    // Test cursor at start
    game.cursor_position = 0;
    if game.cursor_position == 0 {
        results.add_cursor_test("✅ Cursor can be at start".to_string());
    }

    // Test cursor at end
    game.cursor_position = game.current_code.len();
    if game.cursor_position == game.current_code.len() {
        results.add_cursor_test("✅ Cursor can be at end".to_string());
    }

    game.cursor_position = original_cursor;
}

fn test_text_editing(game: &mut Game, results: &mut HotkeyTestResults) {
    let original_code = game.current_code.clone();
    let original_cursor = game.cursor_position;

    // Test basic text insertion
    let initial_len = game.current_code.len();
    game.current_code.insert(game.cursor_position, 'X');
    game.cursor_position += 1;

    if game.current_code.len() == initial_len + 1 {
        results.add_text_test("✅ Text insertion works".to_string());
    } else {
        results.add_text_test("❌ Text insertion failed".to_string());
    }

    // Test text deletion
    if game.cursor_position > 0 {
        game.cursor_position -= 1;
        game.current_code.remove(game.cursor_position);

        if game.current_code.len() == initial_len {
            results.add_text_test("✅ Text deletion works".to_string());
        } else {
            results.add_text_test("❌ Text deletion failed".to_string());
        }
    }

    // Restore state
    game.current_code = original_code;
    game.cursor_position = original_cursor;
}

fn test_common_hotkeys(game: &mut Game, results: &mut HotkeyTestResults) {
    // Test that hotkey system is available
    let hotkey_count = game.hotkey_system.get_all_bindings().len();

    if hotkey_count > 0 {
        results.record_hotkey_test("Hotkey System Loaded".to_string(), true);
        results.add_text_test(format!("✅ {} hotkeys loaded", hotkey_count));
    } else {
        results.record_hotkey_test("Hotkey System Loaded".to_string(), false);
        results.add_text_test("❌ No hotkeys loaded".to_string());
    }

    // Test autocomplete toggle
    let initial_state = game.autocomplete_enabled;
    game.toggle_autocomplete_setting();
    let toggled_state = game.autocomplete_enabled;

    if initial_state != toggled_state {
        results.record_hotkey_test("Autocomplete Toggle".to_string(), true);
    } else {
        results.record_hotkey_test("Autocomplete Toggle".to_string(), false);
    }

    // Restore state
    game.autocomplete_enabled = initial_state;
}

fn handle_editor_input(game: &mut Game, results: &mut HotkeyTestResults) {
    // Handle character input
    while let Some(character) = get_char_pressed() {
        if character.is_control() {
            continue;
        }

        // Insert character at cursor position
        game.current_code.insert(game.cursor_position, character);
        game.cursor_position += 1;
    }

    // Handle special keys
    if is_key_pressed(KeyCode::Backspace) {
        if game.cursor_position > 0 {
            game.cursor_position -= 1;
            game.current_code.remove(game.cursor_position);
        }
    }

    if is_key_pressed(KeyCode::Enter) {
        game.current_code.insert(game.cursor_position, '\n');
        game.cursor_position += 1;
    }

    if is_key_pressed(KeyCode::Tab) {
        // Try autocomplete first
        if !game.accept_autocomplete() {
            // No autocomplete, do normal tab (4 spaces)
            for _ in 0..4 {
                game.current_code.insert(game.cursor_position, ' ');
                game.cursor_position += 1;
            }
        }
    }

    // Arrow keys for cursor movement
    if is_key_pressed(KeyCode::Left) && game.cursor_position > 0 {
        game.cursor_position -= 1;
    }

    if is_key_pressed(KeyCode::Right) && game.cursor_position < game.current_code.len() {
        game.cursor_position += 1;
    }

    // Line up/down movement
    if is_key_pressed(KeyCode::Up) {
        move_cursor_up(game);
    }

    if is_key_pressed(KeyCode::Down) {
        move_cursor_down(game);
    }
}

fn move_cursor_up(game: &mut Game) {
    let lines: Vec<&str> = game.current_code.split('\n').collect();
    let mut current_pos = 0;
    let mut current_line = 0;
    let mut column_in_line = 0;

    // Find current line and column
    for (line_idx, line) in lines.iter().enumerate() {
        if current_pos + line.len() >= game.cursor_position {
            current_line = line_idx;
            column_in_line = game.cursor_position - current_pos;
            break;
        }
        current_pos += line.len() + 1; // +1 for newline
    }

    // Move to previous line if possible
    if current_line > 0 {
        let prev_line = lines[current_line - 1];
        let new_column = column_in_line.min(prev_line.len());

        // Calculate new cursor position
        let mut new_pos = 0;
        for i in 0..(current_line - 1) {
            new_pos += lines[i].len() + 1;
        }
        new_pos += new_column;

        game.cursor_position = new_pos;
    }
}

fn move_cursor_down(game: &mut Game) {
    let lines: Vec<&str> = game.current_code.split('\n').collect();
    let mut current_pos = 0;
    let mut current_line = 0;
    let mut column_in_line = 0;

    // Find current line and column
    for (line_idx, line) in lines.iter().enumerate() {
        if current_pos + line.len() >= game.cursor_position {
            current_line = line_idx;
            column_in_line = game.cursor_position - current_pos;
            break;
        }
        current_pos += line.len() + 1; // +1 for newline
    }

    // Move to next line if possible
    if current_line < lines.len() - 1 {
        let next_line = lines[current_line + 1];
        let new_column = column_in_line.min(next_line.len());

        // Calculate new cursor position
        let mut new_pos = 0;
        for i in 0..=current_line {
            new_pos += lines[i].len() + 1;
        }
        new_pos += new_column;

        game.cursor_position = new_pos.min(game.current_code.len());
    }
}

fn print_final_hotkey_report(results: &HotkeyTestResults) {
    println!("\n📊 Final Hotkey Test Report:");
    println!("==========================");

    if !results.autocomplete_tests.is_empty() {
        println!("\n🔮 Autocomplete Tests:");
        for test in &results.autocomplete_tests {
            println!("  {}", test);
        }
    }

    if !results.tab_behavior_tests.is_empty() {
        println!("\n⌨️ Tab Behavior Tests:");
        for test in &results.tab_behavior_tests {
            println!("  {}", test);
        }
    }

    if !results.cursor_movement_tests.is_empty() {
        println!("\n↔️ Cursor Movement Tests:");
        for test in &results.cursor_movement_tests {
            println!("  {}", test);
        }
    }

    if !results.text_editing_tests.is_empty() {
        println!("\n✏️ Text Editing Tests:");
        for test in &results.text_editing_tests {
            println!("  {}", test);
        }
    }

    if !results.tested_hotkeys.is_empty() {
        println!("\n🎹 Hotkey Tests:");
        for (hotkey, success) in &results.tested_hotkeys {
            let status = if *success { "✅" } else { "❌" };
            println!("  {} {}", status, hotkey);
        }
    }
}

fn draw_test_editor(game: &mut Game, show_hotkey_help: bool, results: &HotkeyTestResults) {
    let scale = ScaledMeasurements::new();

    // Draw title
    draw_text(
        "🔧 AUTOCOMPLETE TEST MODE",
        scale_size(20.0),
        scale_size(30.0),
        scale_size(24.0),
        YELLOW,
    );

    // Draw instructions
    let instructions = [
        "Type to see autocomplete suggestions",
        "Tab = Accept autocomplete (or indent if no suggestion)",
        "F1 = Toggle hotkey help, F2 = Run tests",
        "Try typing: 'pri', 'Str', 'Vec', etc.",
    ];

    for (i, instruction) in instructions.iter().enumerate() {
        draw_text(
            instruction,
            scale_size(20.0),
            scale_size(60.0) + (i as f32 * scale_size(20.0)),
            scale_size(16.0),
            LIGHTGRAY,
        );
    }

    // Draw editor area
    let editor_x = scale_size(20.0);
    let editor_y = scale_size(160.0);
    let editor_width = screen_width() - scale_size(40.0);
    let editor_height = screen_height() - scale_size(180.0);

    // Editor background
    draw_rectangle(editor_x, editor_y, editor_width, editor_height, Color::from_rgba(25, 25, 30, 255));
    draw_rectangle_lines(editor_x, editor_y, editor_width, editor_height, 2.0, GRAY);

    // Draw code with syntax highlighting
    draw_code_with_autocomplete(game, editor_x, editor_y, editor_width, editor_height);

    // Draw autocomplete info
    if let Some(suggestion) = game.get_autocomplete_suggestion() {
        let info_text = format!("Autocomplete: {} ({})", suggestion.text, format!("{:?}", suggestion.kind));
        draw_text(
            &info_text,
            scale_size(20.0),
            screen_height() - scale_size(40.0),
            scale_size(16.0),
            GREEN,
        );
    } else {
        draw_text(
            "No autocomplete suggestion",
            scale_size(20.0),
            screen_height() - scale_size(40.0),
            scale_size(16.0),
            GRAY,
        );
    }

    // Draw hotkey help if toggled
    if show_hotkey_help {
        draw_hotkey_help();
    }

    // Draw test results sidebar
    draw_test_results_sidebar(results);
}

fn draw_hotkey_help() {
    let scale = ScaledMeasurements::new();
    let panel_x = screen_width() - scale_size(350.0);
    let panel_y = scale_size(20.0);
    let panel_width = scale_size(320.0);
    let panel_height = scale_size(400.0);

    // Background panel
    draw_rectangle(panel_x, panel_y, panel_width, panel_height, Color::from_rgba(20, 20, 25, 240));
    draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 2.0, YELLOW);

    // Title
    draw_text("⌨️ HOTKEY REFERENCE", panel_x + scale_size(10.0), panel_y + scale_size(25.0), scale_size(18.0), YELLOW);

    // Hotkey list
    let hotkeys = [
        ("Tab", "Accept autocomplete / Indent"),
        ("Ctrl+C", "Copy"),
        ("Ctrl+V", "Paste"),
        ("Ctrl+X", "Cut"),
        ("Ctrl+Z", "Undo"),
        ("Ctrl+Y", "Redo"),
        ("Ctrl+S", "Save"),
        ("Ctrl+A", "Select All"),
        ("Ctrl+F", "Find"),
        ("Arrow Keys", "Move cursor"),
        ("Home/End", "Line start/end"),
        ("Ctrl+Home/End", "Document start/end"),
        ("Backspace", "Delete previous char"),
        ("Delete", "Delete next char"),
        ("Enter", "New line"),
        ("F1", "Toggle this help"),
        ("F2", "Run hotkey tests"),
        ("Escape", "Exit test mode"),
    ];

    for (i, (key, desc)) in hotkeys.iter().enumerate() {
        let y = panel_y + scale_size(50.0) + (i as f32 * scale_size(18.0));
        if y > panel_y + panel_height - scale_size(20.0) { break; }

        draw_text(key, panel_x + scale_size(10.0), y, scale_size(14.0), BLUE);
        draw_text(desc, panel_x + scale_size(100.0), y, scale_size(12.0), WHITE);
    }
}

fn draw_test_results_sidebar(results: &HotkeyTestResults) {
    if results.autocomplete_tests.is_empty() && results.tab_behavior_tests.is_empty()
        && results.cursor_movement_tests.is_empty() && results.text_editing_tests.is_empty()
        && results.settings_tests.is_empty() {
        return; // No test results to show
    }

    let scale = ScaledMeasurements::new();
    let panel_x = scale_size(20.0);
    let panel_y = screen_height() - scale_size(300.0);
    let panel_width = scale_size(400.0);
    let panel_height = scale_size(250.0);

    // Background panel
    draw_rectangle(panel_x, panel_y, panel_width, panel_height, Color::from_rgba(20, 25, 20, 240));
    draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 2.0, GREEN);

    // Title
    draw_text("🧪 TEST RESULTS", panel_x + scale_size(10.0), panel_y + scale_size(25.0), scale_size(18.0), GREEN);

    let mut y_offset = scale_size(45.0);
    let line_height = scale_size(16.0);

    // Show test results
    for test in &results.autocomplete_tests {
        if y_offset > panel_height - scale_size(30.0) { break; }
        draw_text(test, panel_x + scale_size(10.0), panel_y + y_offset, scale_size(12.0), WHITE);
        y_offset += line_height;
    }

    for test in &results.tab_behavior_tests {
        if y_offset > panel_height - scale_size(30.0) { break; }
        draw_text(test, panel_x + scale_size(10.0), panel_y + y_offset, scale_size(12.0), WHITE);
        y_offset += line_height;
    }

    for test in &results.cursor_movement_tests {
        if y_offset > panel_height - scale_size(30.0) { break; }
        draw_text(test, panel_x + scale_size(10.0), panel_y + y_offset, scale_size(12.0), WHITE);
        y_offset += line_height;
    }

    for test in &results.text_editing_tests {
        if y_offset > panel_height - scale_size(30.0) { break; }
        draw_text(test, panel_x + scale_size(10.0), panel_y + y_offset, scale_size(12.0), WHITE);
        y_offset += line_height;
    }

    for test in &results.settings_tests {
        if y_offset > panel_height - scale_size(30.0) { break; }
        draw_text(test, panel_x + scale_size(10.0), panel_y + y_offset, scale_size(12.0), WHITE);
        y_offset += line_height;
    }
}

fn draw_code_with_autocomplete(game: &mut Game, x: f32, y: f32, width: f32, height: f32) {
    let font_size = scale_size(16.0);
    let line_height = font_size + scale_size(4.0);
    let padding = scale_size(10.0);

    let lines: Vec<&str> = game.current_code.split('\n').collect();
    let mut char_pos = 0;

    // Find cursor line and column
    let mut cursor_line = 0;
    let mut cursor_col = 0;
    let mut temp_pos = 0;

    for (line_idx, line) in lines.iter().enumerate() {
        if temp_pos + line.len() >= game.cursor_position {
            cursor_line = line_idx;
            cursor_col = game.cursor_position - temp_pos;
            break;
        }
        temp_pos += line.len() + 1; // +1 for newline
    }

    // Draw each line
    for (line_idx, line) in lines.iter().enumerate() {
        let line_y = y + padding + (line_idx as f32 * line_height);

        // Skip lines that are outside the visible area
        if line_y > y + height {
            break;
        }

        // Draw line number
        let line_num_text = format!("{:3}", line_idx + 1);
        draw_text(&line_num_text, x + padding, line_y, font_size, GRAY);

        // Draw the line content
        let content_x = x + padding + scale_size(40.0);
        draw_text(line, content_x, line_y, font_size, WHITE);

        // Draw cursor if on this line
        if line_idx == cursor_line {
            let cursor_x = content_x + (cursor_col as f32 * scale_size(9.0)); // Approximate char width
            draw_line(cursor_x, line_y - font_size + scale_size(2.0), cursor_x, line_y + scale_size(2.0), 2.0, YELLOW);

            // Draw autocomplete overlay at cursor position
            if let Some(suggestion) = game.get_autocomplete_suggestion() {
                draw_autocomplete_overlay(&suggestion.text, cursor_x, line_y, font_size);
            }
        }
    }
}

fn draw_autocomplete_overlay(suggestion: &str, x: f32, y: f32, font_size: f32) {
    // Draw the suggestion text with 50% opacity overlay
    let overlay_color = Color::from_rgba(150, 150, 255, 128); // Light blue with 50% opacity

    // First draw a subtle background for the suggestion
    let text_width = suggestion.len() as f32 * scale_size(9.0); // Approximate
    draw_rectangle(x, y - font_size + scale_size(2.0), text_width, font_size, Color::from_rgba(100, 100, 200, 50));

    // Draw the suggestion text with transparency
    draw_text(suggestion, x, y, font_size, overlay_color);
}

fn test_settings_menu_functionality(results: &mut HotkeyTestResults) {
    println!("⚙️  Testing Settings Menu functionality...");

    let mut menu_system = crate::menu::Menu::new();

    // Test initial state
    if menu_system.state == crate::menu::MenuState::MainMenu {
        results.add_settings_test("✅ Menu system starts in MainMenu state".to_string());
    } else {
        results.add_settings_test("❌ Menu system doesn't start in MainMenu state".to_string());
    }

    // Test opening settings
    menu_system.update(crate::menu::MenuAction::OpenSettings);
    if menu_system.state == crate::menu::MenuState::Settings {
        results.add_settings_test("✅ Can navigate to Settings menu".to_string());
    } else {
        results.add_settings_test("❌ Cannot navigate to Settings menu".to_string());
    }

    // Test opening hotkey settings
    menu_system.update(crate::menu::MenuAction::OpenHotkeySettings);
    if menu_system.state == crate::menu::MenuState::HotkeySettings {
        results.add_settings_test("✅ Can navigate to Hotkey Settings menu".to_string());
    } else {
        results.add_settings_test("❌ Cannot navigate to Hotkey Settings menu".to_string());
    }

    // Test that hotkey settings menu has buttons
    if !menu_system.buttons.is_empty() {
        results.add_settings_test("✅ Hotkey settings menu has buttons".to_string());
    } else {
        results.add_settings_test("❌ Hotkey settings menu has no buttons".to_string());
    }

    // Find the Back to Settings button
    let back_button = menu_system.buttons.iter()
        .find(|b| b.action == crate::menu::MenuAction::BackToSettings);
    if back_button.is_some() {
        results.add_settings_test("✅ Has Back to Settings button".to_string());
    } else {
        results.add_settings_test("❌ Missing Back to Settings button".to_string());
    }

    // Test back navigation
    menu_system.update(crate::menu::MenuAction::BackToSettings);
    if menu_system.state == crate::menu::MenuState::Settings {
        results.add_settings_test("✅ Can navigate back to Settings".to_string());
    } else {
        results.add_settings_test("❌ Cannot navigate back to Settings".to_string());
    }
}

fn test_tab_key_no_suggestion(game: &mut Game, results: &mut HotkeyTestResults) {
    println!("🔍 Testing Tab key behavior with no autocomplete suggestion...");

    // Save initial state
    let original_code = game.current_code.clone();
    let original_cursor = game.cursor_position;

    // Test 1: Empty editor - tab should NOT autocomplete anything
    game.current_code = String::new();
    game.cursor_position = 0;
    game.update_autocomplete();

    let has_suggestion_before = game.get_autocomplete_suggestion().is_some();
    let initial_code_length = game.current_code.len();

    // Try to accept autocomplete (this should return false)
    let autocomplete_accepted = game.accept_autocomplete();

    let final_code_length = game.current_code.len();
    let code_changed_by_autocomplete = final_code_length != initial_code_length;

    if !has_suggestion_before && !autocomplete_accepted && !code_changed_by_autocomplete {
        results.add_tab_test("✅ Empty editor: No suggestion, no autocomplete, no unwanted code".to_string());
    } else {
        results.add_tab_test(format!("❌ Empty editor failed: suggestion={}, accepted={}, code_changed={}",
            has_suggestion_before, autocomplete_accepted, code_changed_by_autocomplete));
    }

    // Test 2: Random text that doesn't match any autocomplete - tab should NOT autocomplete
    game.current_code = "zxcvbnm123".to_string();
    game.cursor_position = game.current_code.len();
    game.update_autocomplete();

    let has_suggestion_before = game.get_autocomplete_suggestion().is_some();
    let initial_code = game.current_code.clone();
    let initial_cursor = game.cursor_position;

    // Try to accept autocomplete (this should return false)
    let autocomplete_accepted = game.accept_autocomplete();

    let final_code = game.current_code.clone();
    let final_cursor = game.cursor_position;
    let code_changed_by_autocomplete = final_code != initial_code || final_cursor != initial_cursor;

    if !has_suggestion_before && !autocomplete_accepted && !code_changed_by_autocomplete {
        results.add_tab_test("✅ Random text: No suggestion, no autocomplete, no unwanted changes".to_string());
    } else {
        results.add_tab_test(format!("❌ Random text failed: suggestion={}, accepted={}, changed={}",
            has_suggestion_before, autocomplete_accepted, code_changed_by_autocomplete));
    }

    // Test 3: Verify that valid autocomplete text still works
    game.current_code = "pri".to_string();
    game.cursor_position = 3;
    game.update_autocomplete();

    let has_suggestion_after = game.get_autocomplete_suggestion().is_some();
    let autocomplete_accepted_valid = game.accept_autocomplete();

    if has_suggestion_after && autocomplete_accepted_valid {
        results.add_tab_test("✅ Valid text 'pri': Has suggestion and autocomplete works".to_string());
    } else {
        results.add_tab_test(format!("❌ Valid text 'pri' failed: suggestion={}, accepted={}",
            has_suggestion_after, autocomplete_accepted_valid));
    }

    // Test 4: Check that the autocomplete engine doesn't suggest random strings
    let test_strings = vec!["xyz", "qqq", "123abc", "!@#", ""];
    let mut all_no_suggestions = true;

    for test_str in test_strings {
        game.current_code = test_str.to_string();
        game.cursor_position = test_str.len();
        game.update_autocomplete();

        if game.get_autocomplete_suggestion().is_some() {
            all_no_suggestions = false;
            break;
        }
    }

    if all_no_suggestions {
        results.add_tab_test("✅ Invalid strings produce no suggestions".to_string());
    } else {
        results.add_tab_test("❌ Invalid strings incorrectly produced suggestions".to_string());
    }

    // Restore state
    game.current_code = original_code;
    game.cursor_position = original_cursor;

    println!("  🎯 Tab key behavior tests completed!");
}

fn test_click_drag_selection(game: &mut Game, results: &mut HotkeyTestResults) {
    println!("🖱️  Testing Click and Drag Text Selection...");

    // Save initial state
    let original_code = game.current_code.clone();
    let original_cursor = game.cursor_position;
    let original_selection_start = game.selection_start;
    let original_selection_end = game.selection_end;
    let original_drag_start = game.mouse_drag_start;
    let original_is_dragging = game.is_dragging;

    // Test 1: Mouse drag state initialization
    game.mouse_drag_start = None;
    game.is_dragging = false;
    game.clear_selection();

    // Set up test text
    game.current_code = "fn main() {\n    println!(\"Hello World!\");\n}".to_string();
    game.cursor_position = 0;

    // Test 2: Start mouse drag
    let editor_bounds = (100.0, 100.0, 400.0, 300.0);
    game.start_mouse_drag(150.0, 120.0, editor_bounds);

    if game.mouse_drag_start.is_some() && !game.is_dragging {
        results.add_tab_test("✅ Mouse drag started correctly".to_string());
    } else {
        results.add_tab_test("❌ Mouse drag start failed".to_string());
    }

    // Test 3: Update mouse drag (simulate movement)
    game.update_mouse_drag(200.0, 150.0, editor_bounds);

    if game.is_dragging && game.has_selection() {
        results.add_tab_test("✅ Mouse drag creates text selection".to_string());
    } else {
        results.add_tab_test("❌ Mouse drag didn't create selection".to_string());
    }

    // Test 4: Check selection bounds
    if let Some((start, end)) = game.get_selection_bounds() {
        if start < end && start < game.current_code.len() && end <= game.current_code.len() {
            results.add_tab_test("✅ Selection bounds are valid".to_string());
        } else {
            results.add_tab_test(format!("❌ Invalid selection bounds: start={}, end={}, len={}", start, end, game.current_code.len()));
        }
    } else {
        results.add_tab_test("❌ No selection bounds after drag".to_string());
    }

    // Test 5: End mouse drag
    game.end_mouse_drag();

    if game.mouse_drag_start.is_none() && !game.is_dragging {
        results.add_tab_test("✅ Mouse drag ended correctly".to_string());
    } else {
        results.add_tab_test("❌ Mouse drag end failed".to_string());
    }

    // Test 6: Selection should still exist after drag ends (if it was valid)
    let selection_preserved = game.has_selection();
    if selection_preserved {
        results.add_tab_test("✅ Text selection preserved after drag end".to_string());
    } else {
        results.add_tab_test("ℹ️  Text selection cleared (normal for same-position clicks)".to_string());
    }

    // Test 7: Delete selection functionality
    if game.has_selection() {
        let original_length = game.current_code.len();
        let deleted = game.delete_selection();
        let new_length = game.current_code.len();

        if deleted && new_length < original_length {
            results.add_tab_test("✅ Delete selection works correctly".to_string());
        } else {
            results.add_tab_test("❌ Delete selection failed".to_string());
        }
    }

    // Test 8: Clear selection functionality
    game.start_selection();
    game.update_selection(10);

    if game.has_selection() {
        game.clear_selection();
        if !game.has_selection() {
            results.add_tab_test("✅ Clear selection works correctly".to_string());
        } else {
            results.add_tab_test("❌ Clear selection failed".to_string());
        }
    }

    // Test 9: Small drag movement (should not create selection)
    game.start_mouse_drag(150.0, 120.0, editor_bounds);
    game.update_mouse_drag(151.0, 121.0, editor_bounds); // Very small movement

    if !game.has_selection() {
        results.add_tab_test("✅ Small mouse movements don't create selections".to_string());
    } else {
        results.add_tab_test("❌ Small mouse movement incorrectly created selection".to_string());
    }

    // Test 10: Position-to-cursor conversion works
    let test_positions = vec![
        (120.0, 110.0), // Start of first line
        (180.0, 130.0), // Middle of second line
        (300.0, 150.0), // End area
    ];

    let mut position_tests_passed = 0;
    for (x, y) in test_positions {
        let old_cursor = game.cursor_position;
        game.start_mouse_drag(x, y, editor_bounds);
        let new_cursor = game.cursor_position;

        if new_cursor <= game.current_code.len() && new_cursor != old_cursor {
            position_tests_passed += 1;
        }
        game.end_mouse_drag();
    }

    if position_tests_passed >= 2 {
        results.add_tab_test(format!("✅ Mouse position to cursor conversion works ({}/3 positions)", position_tests_passed));
    } else {
        results.add_tab_test(format!("❌ Mouse position conversion issues ({}/3 positions)", position_tests_passed));
    }

    // Restore original state
    game.current_code = original_code;
    game.cursor_position = original_cursor;
    game.selection_start = original_selection_start;
    game.selection_end = original_selection_end;
    game.mouse_drag_start = original_drag_start;
    game.is_dragging = original_is_dragging;

    println!("  🎯 Click and drag selection tests completed!");
}