use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Help,
}

#[derive(Debug)]
pub struct RustChecker {
    temp_dir: PathBuf,
    project_initialized: bool,
}

impl RustChecker {
    pub fn new() -> Result<Self, String> {
        // Create a temporary directory for our Rust project
        let temp_dir = std::env::temp_dir().join("rust_game_checker");
        
        let checker = RustChecker {
            temp_dir,
            project_initialized: false,
        };
        
        Ok(checker)
    }
    
    fn ensure_project(&mut self) -> Result<(), String> {
        if self.project_initialized {
            return Ok(());
        }
        
        // Create temp directory if it doesn't exist
        if !self.temp_dir.exists() {
            fs::create_dir_all(&self.temp_dir)
                .map_err(|e| format!("Failed to create temp directory: {}", e))?;
        }
        
        // Create Cargo.toml
        let cargo_toml_path = self.temp_dir.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            let cargo_toml_content = r#"[package]
name = "rust_game_checker"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;
            fs::write(&cargo_toml_path, cargo_toml_content)
                .map_err(|e| format!("Failed to create Cargo.toml: {}", e))?;
        }
        
        // Create src directory
        let src_dir = self.temp_dir.join("src");
        if !src_dir.exists() {
            fs::create_dir_all(&src_dir)
                .map_err(|e| format!("Failed to create src directory: {}", e))?;
        }
        
        self.project_initialized = true;
        Ok(())
    }
    
    pub fn check_syntax(&mut self, user_code: &str) -> Result<Vec<CompilerError>, String> {
        self.ensure_project()?;
        
        // Create a main.rs file with the user's code wrapped in proper structure
        let wrapped_code = self.wrap_user_code(user_code);
        let main_rs_path = self.temp_dir.join("src").join("main.rs");
        
        fs::write(&main_rs_path, wrapped_code)
            .map_err(|e| format!("Failed to write main.rs: {}", e))?;
        
        // Run cargo check to get compiler output
        let output = Command::new("cargo")
            .args(&["check", "--message-format=json"])
            .current_dir(&self.temp_dir)
            .output()
            .map_err(|e| format!("Failed to run cargo check: {}. Make sure cargo is installed.", e))?;
        
        // Parse the JSON output from cargo
        self.parse_cargo_output(&output.stdout)
    }
    
    fn wrap_user_code(&self, user_code: &str) -> String {
        // Comprehensive wrapper supporting full Rust language
        format!(r#"// Comprehensive Rust syntax checker
#![allow(unused_variables, dead_code, unused_imports, unused_mut, unused_parens)]
#![allow(unused_assignments, unused_must_use, unreachable_code, path_statements)]

// Standard library prelude for full language support
use std::{{
    collections::HashMap,
    fmt::Display,
    ops::Range,
}};

// Game function stubs
fn move_bot(direction: &str) -> String {{ String::new() }}
fn r#move(direction: &str) -> String {{ String::new() }}
fn scan(direction: &str) -> String {{ String::new() }}
fn grab() -> String {{ String::new() }}
fn open_door(open: bool) -> String {{ String::new() }}

mod laser {{
    pub fn direction(dir: &str) -> String {{ String::new() }}
    pub fn tile(x: i32, y: i32) -> String {{ String::new() }}
}}

// Direction constants commonly used in game code
const UP: &str = "up";
const DOWN: &str = "down"; 
const LEFT: &str = "left";
const RIGHT: &str = "right";

fn main() {{
    // Execute user code in a block to isolate it
    {{
        {}
    }};
    // Explicit unit return to avoid expression issues
}}
"#, user_code)
    }
    
    fn parse_cargo_output(&self, output: &[u8]) -> Result<Vec<CompilerError>, String> {
        let output_str = String::from_utf8_lossy(output);
        let mut errors = Vec::new();
        
        for line in output_str.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            // Try to parse each line as JSON
            if let Ok(json) = serde_json::from_str::<Value>(line) {
                if let Some(message) = json.get("message") {
                    if let Some(compiler_error) = self.parse_compiler_message(message) {
                        errors.push(compiler_error);
                    }
                }
            }
        }
        
        Ok(errors)
    }
    
    fn parse_compiler_message(&self, message: &Value) -> Option<CompilerError> {
        let message_text = message.get("message")?.as_str()?.to_string();
        let level = message.get("level")?.as_str()?;
        
        let severity = match level {
            "error" => ErrorSeverity::Error,
            "warning" => ErrorSeverity::Warning,
            "help" => ErrorSeverity::Help,
            _ => return None,
        };
        
        // Get span information (line and column)
        let spans = message.get("spans")?.as_array()?;
        if spans.is_empty() {
            return None;
        }
        
        let span = &spans[0];
        let line_start = span.get("line_start")?.as_u64()? as usize;
        let column_start = span.get("column_start")?.as_u64()? as usize;
        
        // Adjust line number to account for our wrapper code
        // The user's code starts around line 35 in our wrapper
        let adjusted_line = if line_start >= 35 {
            line_start - 34  // Adjust for wrapper overhead
        } else {
            1  // If error is in wrapper, show as line 1
        };
        
        Some(CompilerError {
            line: adjusted_line,
            column: column_start,
            message: message_text,
            severity,
        })
    }
    
    pub fn cleanup(&self) -> Result<(), String> {
        if self.temp_dir.exists() {
            fs::remove_dir_all(&self.temp_dir)
                .map_err(|e| format!("Failed to cleanup temp directory: {}", e))?;
        }
        Ok(())
    }
}

impl Drop for RustChecker {
    fn drop(&mut self) {
        let _ = self.cleanup(); // Best effort cleanup
    }
}

pub fn format_errors_for_display(errors: &[CompilerError]) -> String {
    if errors.is_empty() {
        return "✅ Code compiled successfully!".to_string();
    }
    
    let mut result = String::new();
    
    let error_count = errors.iter().filter(|e| e.severity == ErrorSeverity::Error).count();
    let warning_count = errors.iter().filter(|e| e.severity == ErrorSeverity::Warning).count();
    
    if error_count > 0 {
        result.push_str(&format!("❌ {} error(s)", error_count));
        if warning_count > 0 {
            result.push_str(&format!(", {} warning(s)", warning_count));
        }
        result.push_str(":\n\n");
    } else if warning_count > 0 {
        result.push_str(&format!("⚠️ {} warning(s):\n\n", warning_count));
    }
    
    for error in errors.iter().take(5) { // Limit to first 5 errors
        let icon = match error.severity {
            ErrorSeverity::Error => "❌",
            ErrorSeverity::Warning => "⚠️",
            ErrorSeverity::Help => "💡",
        };
        
        result.push_str(&format!("{} Line {}: {}\n", 
            icon, error.line, error.message));
    }
    
    if errors.len() > 5 {
        result.push_str(&format!("\n... and {} more issues", errors.len() - 5));
    }
    
    result
}