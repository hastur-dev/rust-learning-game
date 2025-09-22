use macroquad::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key: String,
    pub modifiers: Vec<String>, // "ctrl", "shift", "alt"
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub bindings: Vec<KeyBinding>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum EditorAction {
    Accept,                    // Tab for autocomplete
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,
    Find,
    Replace,
    GoToLine,
    Comment,
    Uncomment,
    Indent,
    Unindent,
    DuplicateLine,
    DeleteLine,
    MoveCursorUp,
    MoveCursorDown,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorWordLeft,
    MoveCursorWordRight,
    MoveCursorLineStart,
    MoveCursorLineEnd,
    MoveCursorDocStart,
    MoveCursorDocEnd,
    SelectUp,
    SelectDown,
    SelectLeft,
    SelectRight,
    SelectWordLeft,
    SelectWordRight,
    SelectLineStart,
    SelectLineEnd,
    SelectAll_,
    RunCode,
    SaveFile,
    ToggleEditor,
}

#[derive(Debug)]
pub struct HotkeySystem {
    bindings: HashMap<String, EditorAction>,
    default_bindings: HashMap<String, EditorAction>,
    config_path: String,
}

impl HotkeySystem {
    pub fn new() -> Self {
        let mut default_bindings = HashMap::new();

        // Default VS Code-like bindings
        default_bindings.insert("Tab".to_string(), EditorAction::Accept);
        default_bindings.insert("Ctrl+Z".to_string(), EditorAction::Undo);
        default_bindings.insert("Ctrl+Y".to_string(), EditorAction::Redo);
        default_bindings.insert("Ctrl+X".to_string(), EditorAction::Cut);
        default_bindings.insert("Ctrl+C".to_string(), EditorAction::Copy);
        default_bindings.insert("Ctrl+V".to_string(), EditorAction::Paste);
        default_bindings.insert("Ctrl+A".to_string(), EditorAction::SelectAll);
        default_bindings.insert("Ctrl+F".to_string(), EditorAction::Find);
        default_bindings.insert("Ctrl+H".to_string(), EditorAction::Replace);
        default_bindings.insert("Ctrl+G".to_string(), EditorAction::GoToLine);
        default_bindings.insert("Ctrl+/".to_string(), EditorAction::Comment);
        default_bindings.insert("Tab".to_string(), EditorAction::Indent);
        default_bindings.insert("Shift+Tab".to_string(), EditorAction::Unindent);
        default_bindings.insert("Ctrl+D".to_string(), EditorAction::DuplicateLine);
        default_bindings.insert("Ctrl+Shift+K".to_string(), EditorAction::DeleteLine);
        default_bindings.insert("Ctrl+S".to_string(), EditorAction::SaveFile);
        default_bindings.insert("Ctrl+Shift+Enter".to_string(), EditorAction::RunCode);
        default_bindings.insert("Ctrl+`".to_string(), EditorAction::ToggleEditor);

        let config_path = "hotkeys_config.json".to_string();
        let bindings = default_bindings.clone();

        Self {
            bindings,
            default_bindings,
            config_path,
        }
    }

    pub fn load_config(&mut self) -> Result<(), String> {
        if !Path::new(&self.config_path).exists() {
            return self.save_config(); // Create default config
        }

        let content = fs::read_to_string(&self.config_path)
            .map_err(|e| format!("Failed to read hotkey config: {}", e))?;

        let config: HotkeyConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse hotkey config: {}", e))?;

        self.bindings.clear();

        for binding in config.bindings {
            let key_combo = self.format_key_combination(&binding.key, &binding.modifiers);
            if let Some(action) = self.parse_action(&binding.action) {
                self.bindings.insert(key_combo, action);
            }
        }

        Ok(())
    }

    pub fn save_config(&self) -> Result<(), String> {
        let mut bindings = Vec::new();

        for (key_combo, action) in &self.bindings {
            let (key, modifiers) = self.parse_key_combination(key_combo);
            bindings.push(KeyBinding {
                key,
                modifiers,
                action: self.action_to_string(action),
            });
        }

        let config = HotkeyConfig {
            bindings,
            description: "Custom hotkey configuration for Rust Steam Game".to_string(),
        };

        let content = serde_json::to_string_pretty(&config)
            .map_err(|e| format!("Failed to serialize hotkey config: {}", e))?;

        fs::write(&self.config_path, content)
            .map_err(|e| format!("Failed to write hotkey config: {}", e))?;

        Ok(())
    }

    pub fn get_action_for_input(&self, key: KeyCode, ctrl: bool, shift: bool, alt: bool) -> Option<EditorAction> {
        let key_combo = self.format_input_combination(key, ctrl, shift, alt);
        self.bindings.get(&key_combo).cloned()
    }

    pub fn set_binding(&mut self, key_combo: String, action: EditorAction) {
        self.bindings.insert(key_combo, action);
    }

    pub fn remove_binding(&mut self, key_combo: &str) {
        self.bindings.remove(key_combo);
    }

    pub fn reset_to_defaults(&mut self) {
        self.bindings = self.default_bindings.clone();
    }

    pub fn get_all_bindings(&self) -> &HashMap<String, EditorAction> {
        &self.bindings
    }

    pub fn import_vscode_keybindings(&mut self, vscode_path: &str) -> Result<(), String> {
        let content = fs::read_to_string(vscode_path)
            .map_err(|e| format!("Failed to read VSCode keybindings: {}", e))?;

        // Parse VSCode keybindings.json format
        let vscode_bindings: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse VSCode keybindings: {}", e))?;

        if let Some(bindings_array) = vscode_bindings.as_array() {
            for binding in bindings_array {
                if let (Some(key), Some(command)) = (
                    binding.get("key").and_then(|k| k.as_str()),
                    binding.get("command").and_then(|c| c.as_str())
                ) {
                    if let Some(action) = self.vscode_command_to_action(command) {
                        let key_combo = self.vscode_key_to_combo(key);
                        self.bindings.insert(key_combo, action);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn import_vim_config(&mut self, vim_path: &str) -> Result<(), String> {
        let content = fs::read_to_string(vim_path)
            .map_err(|e| format!("Failed to read Vim config: {}", e))?;

        // Basic vim key mapping parser (simplified)
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("map ") || line.starts_with("nmap ") {
                // Parse basic vim mappings like "map <C-s> :w<CR>"
                if let Some(action) = self.parse_vim_mapping(line) {
                    // Convert vim mapping to our format (simplified)
                    // This is a basic implementation - a full vim parser would be much more complex
                    continue;
                }
            }
        }

        Ok(())
    }

    pub fn import_emacs_config(&mut self, emacs_path: &str) -> Result<(), String> {
        let content = fs::read_to_string(emacs_path)
            .map_err(|e| format!("Failed to read Emacs config: {}", e))?;

        // Basic emacs key binding parser (simplified)
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("(global-set-key") {
                // Parse emacs key bindings like "(global-set-key (kbd \"C-s\") 'save-buffer)"
                if let Some(action) = self.parse_emacs_binding(line) {
                    // Convert emacs binding to our format (simplified)
                    continue;
                }
            }
        }

        Ok(())
    }

    // Helper methods
    fn format_key_combination(&self, key: &str, modifiers: &[String]) -> String {
        let mut parts = Vec::new();

        for modifier in modifiers {
            match modifier.to_lowercase().as_str() {
                "ctrl" | "control" => parts.push("Ctrl"),
                "shift" => parts.push("Shift"),
                "alt" => parts.push("Alt"),
                _ => {}
            }
        }

        parts.push(key);
        parts.join("+")
    }

    fn parse_key_combination(&self, key_combo: &str) -> (String, Vec<String>) {
        let parts: Vec<&str> = key_combo.split('+').collect();
        if parts.is_empty() {
            return ("".to_string(), Vec::new());
        }

        let key = parts.last().unwrap().to_string();
        let modifiers = parts[..parts.len()-1].iter().map(|s| s.to_string()).collect();

        (key, modifiers)
    }

    fn format_input_combination(&self, key: KeyCode, ctrl: bool, shift: bool, alt: bool) -> String {
        let mut parts = Vec::new();

        if ctrl { parts.push("Ctrl"); }
        if shift { parts.push("Shift"); }
        if alt { parts.push("Alt"); }

        let key_string = self.keycode_to_string(key);
        parts.push(&key_string);
        parts.join("+")
    }

    fn keycode_to_string(&self, key: KeyCode) -> String {
        match key {
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Escape => "Escape".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Space => "Space".to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PageUp".to_string(),
            KeyCode::PageDown => "PageDown".to_string(),
            KeyCode::F1 => "F1".to_string(),
            KeyCode::F2 => "F2".to_string(),
            KeyCode::F3 => "F3".to_string(),
            KeyCode::F4 => "F4".to_string(),
            KeyCode::F5 => "F5".to_string(),
            KeyCode::F6 => "F6".to_string(),
            KeyCode::F7 => "F7".to_string(),
            KeyCode::F8 => "F8".to_string(),
            KeyCode::F9 => "F9".to_string(),
            KeyCode::F10 => "F10".to_string(),
            KeyCode::F11 => "F11".to_string(),
            KeyCode::F12 => "F12".to_string(),
            KeyCode::A => "A".to_string(),
            KeyCode::B => "B".to_string(),
            KeyCode::C => "C".to_string(),
            KeyCode::D => "D".to_string(),
            KeyCode::E => "E".to_string(),
            KeyCode::F => "F".to_string(),
            KeyCode::G => "G".to_string(),
            KeyCode::H => "H".to_string(),
            KeyCode::I => "I".to_string(),
            KeyCode::J => "J".to_string(),
            KeyCode::K => "K".to_string(),
            KeyCode::L => "L".to_string(),
            KeyCode::M => "M".to_string(),
            KeyCode::N => "N".to_string(),
            KeyCode::O => "O".to_string(),
            KeyCode::P => "P".to_string(),
            KeyCode::Q => "Q".to_string(),
            KeyCode::R => "R".to_string(),
            KeyCode::S => "S".to_string(),
            KeyCode::T => "T".to_string(),
            KeyCode::U => "U".to_string(),
            KeyCode::V => "V".to_string(),
            KeyCode::W => "W".to_string(),
            KeyCode::X => "X".to_string(),
            KeyCode::Y => "Y".to_string(),
            KeyCode::Z => "Z".to_string(),
            KeyCode::Key0 => "0".to_string(),
            KeyCode::Key1 => "1".to_string(),
            KeyCode::Key2 => "2".to_string(),
            KeyCode::Key3 => "3".to_string(),
            KeyCode::Key4 => "4".to_string(),
            KeyCode::Key5 => "5".to_string(),
            KeyCode::Key6 => "6".to_string(),
            KeyCode::Key7 => "7".to_string(),
            KeyCode::Key8 => "8".to_string(),
            KeyCode::Key9 => "9".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    fn parse_action(&self, action_str: &str) -> Option<EditorAction> {
        match action_str.to_lowercase().as_str() {
            "accept" => Some(EditorAction::Accept),
            "undo" => Some(EditorAction::Undo),
            "redo" => Some(EditorAction::Redo),
            "cut" => Some(EditorAction::Cut),
            "copy" => Some(EditorAction::Copy),
            "paste" => Some(EditorAction::Paste),
            "selectall" => Some(EditorAction::SelectAll),
            "find" => Some(EditorAction::Find),
            "replace" => Some(EditorAction::Replace),
            "gotoline" => Some(EditorAction::GoToLine),
            "comment" => Some(EditorAction::Comment),
            "uncomment" => Some(EditorAction::Uncomment),
            "indent" => Some(EditorAction::Indent),
            "unindent" => Some(EditorAction::Unindent),
            "duplicateline" => Some(EditorAction::DuplicateLine),
            "deleteline" => Some(EditorAction::DeleteLine),
            "runcode" => Some(EditorAction::RunCode),
            "savefile" => Some(EditorAction::SaveFile),
            "toggleeditor" => Some(EditorAction::ToggleEditor),
            _ => None,
        }
    }

    fn action_to_string(&self, action: &EditorAction) -> String {
        match action {
            EditorAction::Accept => "accept".to_string(),
            EditorAction::Undo => "undo".to_string(),
            EditorAction::Redo => "redo".to_string(),
            EditorAction::Cut => "cut".to_string(),
            EditorAction::Copy => "copy".to_string(),
            EditorAction::Paste => "paste".to_string(),
            EditorAction::SelectAll => "selectall".to_string(),
            EditorAction::Find => "find".to_string(),
            EditorAction::Replace => "replace".to_string(),
            EditorAction::GoToLine => "gotoline".to_string(),
            EditorAction::Comment => "comment".to_string(),
            EditorAction::Uncomment => "uncomment".to_string(),
            EditorAction::Indent => "indent".to_string(),
            EditorAction::Unindent => "unindent".to_string(),
            EditorAction::DuplicateLine => "duplicateline".to_string(),
            EditorAction::DeleteLine => "deleteline".to_string(),
            EditorAction::RunCode => "runcode".to_string(),
            EditorAction::SaveFile => "savefile".to_string(),
            EditorAction::ToggleEditor => "toggleeditor".to_string(),
            _ => "unknown".to_string(),
        }
    }

    fn vscode_command_to_action(&self, command: &str) -> Option<EditorAction> {
        match command {
            "undo" => Some(EditorAction::Undo),
            "redo" => Some(EditorAction::Redo),
            "editor.action.clipboardCutAction" => Some(EditorAction::Cut),
            "editor.action.clipboardCopyAction" => Some(EditorAction::Copy),
            "editor.action.clipboardPasteAction" => Some(EditorAction::Paste),
            "editor.action.selectAll" => Some(EditorAction::SelectAll),
            "actions.find" => Some(EditorAction::Find),
            "editor.action.startFindReplaceAction" => Some(EditorAction::Replace),
            "workbench.action.gotoLine" => Some(EditorAction::GoToLine),
            "editor.action.commentLine" => Some(EditorAction::Comment),
            "editor.action.indentLines" => Some(EditorAction::Indent),
            "editor.action.outdentLines" => Some(EditorAction::Unindent),
            "editor.action.copyLinesDownAction" => Some(EditorAction::DuplicateLine),
            "editor.action.deleteLines" => Some(EditorAction::DeleteLine),
            "workbench.action.files.save" => Some(EditorAction::SaveFile),
            _ => None,
        }
    }

    fn vscode_key_to_combo(&self, vscode_key: &str) -> String {
        // Convert VSCode key format to our format
        // VSCode uses "ctrl+s", we use "Ctrl+S"
        vscode_key.replace("ctrl", "Ctrl")
                  .replace("shift", "Shift")
                  .replace("alt", "Alt")
                  .replace("cmd", "Ctrl") // Mac cmd -> Ctrl on Windows/Linux
    }

    fn parse_vim_mapping(&self, _line: &str) -> Option<EditorAction> {
        // Simplified vim mapping parser
        // A full implementation would be much more complex
        None
    }

    fn parse_emacs_binding(&self, _line: &str) -> Option<EditorAction> {
        // Simplified emacs binding parser
        // A full implementation would be much more complex
        None
    }
}