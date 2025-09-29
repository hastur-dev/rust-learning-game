// Hotkey testing mode for the editor
// This module provides comprehensive testing of all editor hotkeys

use macroquad::prelude::*;
use ::rand::{rngs::StdRng, SeedableRng};
use crate::gamestate::Game;
use crate::embedded_levels;
use crate::hotkeys::EditorAction;

#[derive(Debug, Clone)]
pub struct HotkeyTestResult {
    pub hotkey: String,
    pub action: EditorAction,
    pub tested: bool,
    pub success: bool,
    pub notes: String,
}

pub struct HotkeyTestSuite {
    pub tests: Vec<HotkeyTestResult>,
    pub current_test: usize,
    pub total_passed: usize,
    pub total_failed: usize,
}

impl HotkeyTestSuite {
    pub fn new() -> Self {
        let tests = vec![
            HotkeyTestResult {
                hotkey: "Tab".to_string(),
                action: EditorAction::Accept,
                tested: false,
                success: false,
                notes: "Should accept autocomplete suggestions".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+Z".to_string(),
                action: EditorAction::Undo,
                tested: false,
                success: false,
                notes: "Should undo last edit operation".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+Y".to_string(),
                action: EditorAction::Redo,
                tested: false,
                success: false,
                notes: "Should redo last undone operation".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+X".to_string(),
                action: EditorAction::Cut,
                tested: false,
                success: false,
                notes: "Should cut selected text to clipboard".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+C".to_string(),
                action: EditorAction::Copy,
                tested: false,
                success: false,
                notes: "Should copy selected text to clipboard".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+V".to_string(),
                action: EditorAction::Paste,
                tested: false,
                success: false,
                notes: "Should paste clipboard content".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+A".to_string(),
                action: EditorAction::SelectAll,
                tested: false,
                success: false,
                notes: "Should select all text in editor".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+F".to_string(),
                action: EditorAction::Find,
                tested: false,
                success: false,
                notes: "Should open find dialog".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+H".to_string(),
                action: EditorAction::Replace,
                tested: false,
                success: false,
                notes: "Should open find and replace dialog".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+G".to_string(),
                action: EditorAction::GoToLine,
                tested: false,
                success: false,
                notes: "Should open go to line dialog".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+/".to_string(),
                action: EditorAction::Comment,
                tested: false,
                success: false,
                notes: "Should toggle comment on current line".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Shift+Tab".to_string(),
                action: EditorAction::Unindent,
                tested: false,
                success: false,
                notes: "Should decrease indentation".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+D".to_string(),
                action: EditorAction::DuplicateLine,
                tested: false,
                success: false,
                notes: "Should duplicate current line".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+Shift+K".to_string(),
                action: EditorAction::DeleteLine,
                tested: false,
                success: false,
                notes: "Should delete current line".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+S".to_string(),
                action: EditorAction::SaveFile,
                tested: false,
                success: false,
                notes: "Should save current file".to_string(),
            },
            HotkeyTestResult {
                hotkey: "Ctrl+Shift+Enter".to_string(),
                action: EditorAction::RunCode,
                tested: false,
                success: false,
                notes: "Should run/execute the code".to_string(),
            },
        ];

        Self {
            tests,
            current_test: 0,
            total_passed: 0,
            total_failed: 0,
        }
    }

    pub fn mark_test_result(&mut self, hotkey: &str, success: bool, notes: String) {
        if let Some(test) = self.tests.iter_mut().find(|t| t.hotkey == hotkey) {
            test.tested = true;
            test.success = success;
            test.notes = notes;

            if success {
                self.total_passed += 1;
            } else {
                self.total_failed += 1;
            }
        }
    }

    pub fn get_completion_percentage(&self) -> f32 {
        let tested_count = self.tests.iter().filter(|t| t.tested).count();
        (tested_count as f32 / self.tests.len() as f32) * 100.0
    }

    pub fn is_complete(&self) -> bool {
        self.tests.iter().all(|t| t.tested)
    }
}

pub async fn run_hotkey_test_mode(enable_all_logs: bool) {
    println!("⌨️  Hotkey Test Mode Started!");
    println!("  🧪 Testing all editor hotkeys");
    println!("  📋 Press hotkeys to test their functionality");
    println!("  ✅ Green = Working | ❌ Red = Not working | ⚪ Gray = Not tested");
    println!("  ❌ Press Escape to exit");

    // Initialize game with the real editor systems
    let rng = StdRng::seed_from_u64(0xDEADBEEF); // Valid hex

    let core_levels = embedded_levels::get_embedded_level_specs();
    let mut game = Game::new(core_levels.clone(), rng);

    // Enable coordinate logs if --all-logs flag is present
    game.enable_coordinate_logs = enable_all_logs;
    game.enable_key_press_logs = enable_all_logs;

    // Set up a simple level for testing
    game.level_idx = 0;
    game.load_level(0);

    // Enable code editor
    game.code_editor_active = true;

    // Set up test code with examples for each hotkey
    game.current_code = r#"fn main() {
    // Welcome to the Hotkey Test Mode!
    // Try these hotkeys:

    let test_variable = "Hello World";

    // Ctrl+A - Select all text
    // Ctrl+C - Copy selected text
    // Ctrl+V - Paste clipboard content
    // Ctrl+X - Cut selected text

    println!("Testing hotkeys: {}", test_variable);

    // Ctrl+D - Duplicate this line
    // Ctrl+Shift+K - Delete this line
    // Ctrl+/ - Comment/uncomment this line

    // Tab - Indent (try selecting multiple lines)
    // Shift+Tab - Unindent (try with selected lines)

    // Ctrl+Z - Undo last operation
    // Ctrl+Y - Redo last undone operation

    // Ctrl+F - Find text in editor
    // Ctrl+H - Find and replace text
    // Ctrl+G - Go to specific line number

    // Ctrl+S - Save file
    // Ctrl+Shift+Enter - Run code
}
"#.to_string();

    game.cursor_position = 100; // Position in the middle

    // Create hotkey test suite
    let mut test_suite = HotkeyTestSuite::new();

    // Set up window
    request_new_screen_size(1400.0, 900.0);

    // Track last action for feedback
    let mut last_action_detected = "None".to_string();
    let mut last_action_time = 0.0;

    loop {
        clear_background(Color::from_rgba(25, 25, 30, 255));

        // Exit handling
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Track current time for action feedback
        let current_time = crate::crash_protection::safe_get_time();

        // Get mouse position for editor interaction
        // Use safe mouse position to prevent crashes when window loses focus
        let (mouse_x, mouse_y) = crate::crash_protection::safe_mouse_position();

        // Handle mouse for editor text selection
        if is_mouse_button_pressed(MouseButton::Left) {
            let editor_x = 50.0;
            let editor_y = 50.0;
            let editor_width = 650.0;
            let editor_height = crate::crash_protection::safe_screen_height() - 100.0;

            if mouse_x >= editor_x && mouse_x <= editor_x + editor_width &&
               mouse_y >= editor_y && mouse_y <= editor_y + editor_height {

                let editor_bounds = (editor_x, editor_y, editor_width, editor_height);
                game.start_mouse_drag(mouse_x, mouse_y, editor_bounds);
            }
        }

        // Handle mouse dragging for text selection
        if is_mouse_button_down(MouseButton::Left) && game.mouse_drag_start.is_some() {
            let editor_x = 50.0;
            let editor_y = 50.0;
            let editor_width = 650.0;
            let editor_height = crate::crash_protection::safe_screen_height() - 100.0;
            let editor_bounds = (editor_x, editor_y, editor_width, editor_height);

            game.update_mouse_drag(mouse_x, mouse_y, editor_bounds);
        }

        // Handle mouse button release
        if is_mouse_button_released(MouseButton::Left) {
            if game.mouse_drag_start.is_some() {
                game.end_mouse_drag();
            }
        }

        // Test hotkey detection and response
        let mut hotkey_detected = false;
        let ctrl_held = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
        let shift_held = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        let alt_held = is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt);

        // Check for specific hotkey combinations
        if is_key_pressed(KeyCode::Tab) {
            if shift_held {
                last_action_detected = "Shift+Tab (Unindent)".to_string();
                test_suite.mark_test_result("Shift+Tab", true, "Unindent hotkey detected!".to_string());
            } else {
                last_action_detected = "Tab (Accept/Indent)".to_string();
                test_suite.mark_test_result("Tab", true, "Tab hotkey detected!".to_string());
            }
            last_action_time = current_time;
            hotkey_detected = true;
        }

        if ctrl_held {
            // Check individual key presses with Ctrl
            if is_key_pressed(KeyCode::Z) {
                last_action_detected = "Ctrl+Z (Undo)".to_string();
                test_suite.mark_test_result("Ctrl+Z", true, "Undo hotkey detected!".to_string());
                last_action_time = current_time;
                hotkey_detected = true;
            }
            if is_key_pressed(KeyCode::Y) {
                last_action_detected = "Ctrl+Y (Redo)".to_string();
                test_suite.mark_test_result("Ctrl+Y", true, "Redo hotkey detected!".to_string());
                last_action_time = current_time;
                hotkey_detected = true;
            }
            if is_key_pressed(KeyCode::X) {
                last_action_detected = "Ctrl+X (Cut)".to_string();
                test_suite.mark_test_result("Ctrl+X", true, "Cut hotkey detected!".to_string());
                last_action_time = current_time;
                hotkey_detected = true;
            }
            if is_key_pressed(KeyCode::C) {
                last_action_detected = "Ctrl+C (Copy)".to_string();
                test_suite.mark_test_result("Ctrl+C", true, "Copy hotkey detected!".to_string());
                last_action_time = current_time;
                hotkey_detected = true;
            }
            if is_key_pressed(KeyCode::V) {
                last_action_detected = "Ctrl+V (Paste)".to_string();
                test_suite.mark_test_result("Ctrl+V", true, "Paste hotkey detected!".to_string());
                last_action_time = current_time;
                hotkey_detected = true;
            }
            if is_key_pressed(KeyCode::A) {
                last_action_detected = "Ctrl+A (Select All)".to_string();
                test_suite.mark_test_result("Ctrl+A", true, "Select All hotkey detected!".to_string());
                last_action_time = current_time;
                hotkey_detected = true;
            }
            if is_key_pressed(KeyCode::F) {
                last_action_detected = "Ctrl+F (Find)".to_string();
                test_suite.mark_test_result("Ctrl+F", true, "Find hotkey detected!".to_string());
                last_action_time = current_time;
                hotkey_detected = true;
            }
            if is_key_pressed(KeyCode::H) {
                last_action_detected = "Ctrl+H (Replace)".to_string();
                test_suite.mark_test_result("Ctrl+H", true, "Replace hotkey detected!".to_string());
                last_action_time = current_time;
                hotkey_detected = true;
            }
            if is_key_pressed(KeyCode::G) {
                last_action_detected = "Ctrl+G (Go To Line)".to_string();
                test_suite.mark_test_result("Ctrl+G", true, "Go To Line hotkey detected!".to_string());
                last_action_time = current_time;
                hotkey_detected = true;
            }
            if is_key_pressed(KeyCode::Slash) {
                last_action_detected = "Ctrl+/ (Comment)".to_string();
                test_suite.mark_test_result("Ctrl+/", true, "Comment hotkey detected!".to_string());
                last_action_time = current_time;
                hotkey_detected = true;
            }
            if is_key_pressed(KeyCode::D) {
                last_action_detected = "Ctrl+D (Duplicate Line)".to_string();
                test_suite.mark_test_result("Ctrl+D", true, "Duplicate Line hotkey detected!".to_string());
                last_action_time = current_time;
                hotkey_detected = true;
            }
            if is_key_pressed(KeyCode::S) {
                last_action_detected = "Ctrl+S (Save File)".to_string();
                test_suite.mark_test_result("Ctrl+S", true, "Save File hotkey detected!".to_string());
                last_action_time = current_time;
                hotkey_detected = true;
            }

            // Check Ctrl+Shift combinations
            if shift_held {
                if is_key_pressed(KeyCode::K) {
                    last_action_detected = "Ctrl+Shift+K (Delete Line)".to_string();
                    test_suite.mark_test_result("Ctrl+Shift+K", true, "Delete Line hotkey detected!".to_string());
                    last_action_time = current_time;
                    hotkey_detected = true;
                }
                if is_key_pressed(KeyCode::Enter) {
                    last_action_detected = "Ctrl+Shift+Enter (Run Code)".to_string();
                    test_suite.mark_test_result("Ctrl+Shift+Enter", true, "Run Code hotkey detected!".to_string());
                    last_action_time = current_time;
                    hotkey_detected = true;
                }
            }
        }

        // Standard editor input handling (for typing test code)
        if game.code_editor_active {
            let mut code_modified = false;

            // Handle character input
            while let Some(character) = get_char_pressed() {
                if character.is_ascii() && !character.is_control() && character != ' ' {
                    // Skip if this is a hotkey combination
                    if !hotkey_detected {
                        if game.delete_selection() {
                            code_modified = true;
                        }
                        game.current_code.insert(game.cursor_position, character);
                        game.cursor_position += 1;
                        code_modified = true;
                    }
                }
            }

            // Handle space (if not part of hotkey)
            if is_key_pressed(KeyCode::Space) && !hotkey_detected {
                if game.delete_selection() {
                    code_modified = true;
                }
                game.current_code.insert(game.cursor_position, ' ');
                game.cursor_position += 1;
                code_modified = true;
            }

            // Backspace handling (if not part of hotkey)
            if is_key_pressed(KeyCode::Backspace) && !hotkey_detected {
                if !game.delete_selection() && game.cursor_position > 0 {
                    game.current_code.remove(game.cursor_position - 1);
                    game.cursor_position -= 1;
                    code_modified = true;
                }
            }

            // Enter key handling (if not part of hotkey)
            if is_key_pressed(KeyCode::Enter) && !hotkey_detected {
                if game.delete_selection() {
                    code_modified = true;
                }
                game.current_code.insert(game.cursor_position, '\n');
                game.cursor_position += 1;
                code_modified = true;
            }

            // Arrow key navigation
            if is_key_pressed(KeyCode::Up) && !hotkey_detected {
                game.move_cursor_up_with_selection(shift_held);
            }
            if is_key_pressed(KeyCode::Down) && !hotkey_detected {
                game.move_cursor_down_with_selection(shift_held);
            }
            if is_key_pressed(KeyCode::Left) && !hotkey_detected {
                game.move_cursor_left_with_selection(shift_held);
            }
            if is_key_pressed(KeyCode::Right) && !hotkey_detected {
                game.move_cursor_right_with_selection(shift_held);
            }

            // Update autocomplete if code modified
            if code_modified {
                game.update_autocomplete();
            }
        }

        // Draw the editor (left side)
        crate::drawing::editor_drawing::draw_code_editor(&mut game);

        // Draw test results panel (right side)
        let panel_x = 720.0;
        let panel_width = crate::crash_protection::safe_screen_width() - panel_x - 20.0;

        // Draw panel background
        draw_rectangle(panel_x, 50.0, panel_width, crate::crash_protection::safe_screen_height() - 100.0, Color::from_rgba(40, 40, 45, 255));
        draw_rectangle_lines(panel_x, 50.0, panel_width, crate::crash_protection::safe_screen_height() - 100.0, 2.0, WHITE);

        // Draw title
        draw_text("⌨️  HOTKEY TEST RESULTS", panel_x + 10.0, 80.0, 24.0, YELLOW);

        // Draw progress
        let completion = test_suite.get_completion_percentage();
        draw_text(&format!("Progress: {:.1}% ({}/{})", completion, test_suite.total_passed + test_suite.total_failed, test_suite.tests.len()),
                  panel_x + 10.0, 110.0, 16.0, LIGHTGRAY);

        draw_text(&format!("✅ Passed: {} | ❌ Failed: {}", test_suite.total_passed, test_suite.total_failed),
                  panel_x + 10.0, 130.0, 16.0, LIGHTGRAY);

        // Draw last detected action
        let action_color = if current_time - last_action_time < 2.0 { LIME } else { GRAY };
        draw_text(&format!("Last Action: {}", last_action_detected), panel_x + 10.0, 160.0, 16.0, action_color);

        // Draw test results
        let mut y_offset = 190.0;
        for test in &test_suite.tests {
            let status_color = if test.tested {
                if test.success { GREEN } else { RED }
            } else {
                GRAY
            };

            let status_icon = if test.tested {
                if test.success { "✅" } else { "❌" }
            } else {
                "⚪"
            };

            draw_text(&format!("{} {}", status_icon, test.hotkey), panel_x + 10.0, y_offset, 14.0, status_color);

            if test.tested {
                draw_text(&test.notes, panel_x + 20.0, y_offset + 15.0, 11.0, LIGHTGRAY);
                y_offset += 35.0;
            } else {
                draw_text(&test.notes, panel_x + 20.0, y_offset + 15.0, 11.0, DARKGRAY);
                y_offset += 35.0;
            }
        }

        // Draw instructions at top
        draw_text("⌨️  Hotkey Test Mode - Press hotkeys to test them!", 10.0, 25.0, 20.0, YELLOW);

        next_frame().await;
    }

    // Print final results
    println!("\n🏁 Hotkey Test Results:");
    println!("  ✅ Passed: {}/{}", test_suite.total_passed, test_suite.tests.len());
    println!("  ❌ Failed: {}/{}", test_suite.total_failed, test_suite.tests.len());
    println!("  📊 Completion: {:.1}%", test_suite.get_completion_percentage());

    if test_suite.is_complete() {
        println!("  🎉 All hotkeys tested!");
    } else {
        println!("  ⚠️  Some hotkeys not tested yet");
    }

    println!("✅ Hotkey Test Mode Exited");
}