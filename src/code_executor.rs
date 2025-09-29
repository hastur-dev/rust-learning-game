use std::process::{Command, Stdio};
use std::fs;
use std::path::PathBuf;
use std::io::Write;

pub struct CodeExecutor {
    temp_dir: PathBuf,
}

impl CodeExecutor {
    pub fn new() -> Result<Self, String> {
        // Create a temporary directory for code execution
        let temp_dir = std::env::temp_dir().join("rust_game_executor");
        fs::create_dir_all(&temp_dir)
            .map_err(|e| format!("Failed to create temp directory: {}", e))?;

        Ok(Self { temp_dir })
    }

    /// Execute user's Rust code and capture output
    pub fn execute_code(&self, user_code: &str) -> Result<ExecutionResult, String> {
        // Use the same wrapper system as the syntax checker to provide game function stubs
        let code = self.wrap_user_code_for_execution(user_code);

        // Write code to a temporary .rs file
        let source_path = self.temp_dir.join("user_code.rs");
        fs::write(&source_path, &code)
            .map_err(|e| format!("Failed to write source file: {}", e))?;

        // Compile the code
        let exe_path = self.temp_dir.join("user_code.exe");
        let compile_output = Command::new("rustc")
            .arg(&source_path)
            .arg("-o")
            .arg(&exe_path)
            .arg("--edition=2021")
            .output()
            .map_err(|e| format!("Failed to run rustc: {}. Make sure Rust is installed.", e))?;

        if !compile_output.status.success() {
            // Compilation failed - return compiler errors
            let stderr = String::from_utf8_lossy(&compile_output.stderr);
            return Ok(ExecutionResult {
                stdout: String::new(),
                stderr: stderr.to_string(),
                success: false,
                is_compilation_error: true,
            });
        }

        // Run the compiled executable and capture output
        let run_output = Command::new(&exe_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to run executable: {}", e))?;

        let stdout = String::from_utf8_lossy(&run_output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&run_output.stderr).to_string();

        Ok(ExecutionResult {
            stdout,
            stderr,
            success: run_output.status.success(),
            is_compilation_error: false,
        })
    }

    /// Wrap user code with game function stubs for execution
    fn wrap_user_code_for_execution(&self, user_code: &str) -> String {
        // Check if the user code already contains fn main()
        let has_main = user_code.contains("fn main()") || user_code.contains("fn main (");

        if has_main {
            // If user code already has fn main(), just add the necessary stubs
            format!(r#"#![allow(unused_variables, dead_code, unused_imports, unused_mut, unused_parens)]
#![allow(unused_assignments, unused_must_use, unreachable_code, path_statements)]

// Game function stubs that return empty strings (for execution)
fn scan() -> String {{ String::new() }}
fn grab() -> String {{ String::new() }}
fn search() -> String {{ String::new() }}
fn move_bot(direction: &str) -> String {{ String::new() }}

// User code with its own main function
{}
"#, user_code)
        } else {
            // If no main function, wrap it
            format!(r#"#![allow(unused_variables, dead_code, unused_imports, unused_mut, unused_parens)]
#![allow(unused_assignments, unused_must_use, unreachable_code, path_statements)]

// Game function stubs that return empty strings (for execution)
fn scan() -> String {{ String::new() }}
fn grab() -> String {{ String::new() }}
fn search() -> String {{ String::new() }}
fn move_bot(direction: &str) -> String {{ String::new() }}

fn main() {{
    {}
}}
"#, user_code)
        }
    }

    /// Clean up temporary files
    pub fn cleanup(&self) -> Result<(), String> {
        // Best effort cleanup - don't fail if it doesn't work
        let _ = fs::remove_file(self.temp_dir.join("user_code.rs"));
        let _ = fs::remove_file(self.temp_dir.join("user_code.exe"));
        let _ = fs::remove_file(self.temp_dir.join("user_code.pdb")); // Windows debug symbols
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub is_compilation_error: bool,
}

impl Drop for CodeExecutor {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}