// Test utilities for analyzing user code and executing tests
use std::process::{Command, Stdio};
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

pub struct UserCodeAnalyzer {
    pub code: String,
}

impl UserCodeAnalyzer {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
        }
    }
    
    /// Check if user code contains a specific function definition
    pub fn contains_function(&self, function_name: &str) -> bool {
        let pattern = format!("fn {}(", function_name);
        self.code.contains(&pattern)
    }
    
    /// Check if user code contains a specific struct definition
    pub fn contains_struct(&self, struct_name: &str) -> bool {
        let pattern = format!("struct {} {{", struct_name);
        self.code.contains(&pattern) || self.code.contains(&format!("struct {}{}", struct_name, " {"))
    }
    
    /// Count occurrences of a pattern in the code
    pub fn count_pattern(&self, pattern: &str) -> usize {
        self.code.matches(pattern).count()
    }
    
    /// Check if code contains nested for loops
    pub fn has_nested_for_loops(&self) -> bool {
        // Simple heuristic: look for "for" appearing multiple times with proper nesting
        let for_count = self.count_pattern("for ");
        for_count >= 2 && self.code.contains("for y in") && self.code.contains("for x in")
    }
    
    /// Check if code contains a specific print statement
    pub fn contains_println_with_text(&self, text: &str) -> bool {
        let pattern = format!("println!(\"{}\")", text);
        self.code.contains(&pattern)
    }
    
    /// Check if code contains any println! statements
    pub fn contains_println(&self) -> bool {
        self.code.contains("println!")
    }
    
    /// Execute user code and capture output
    pub fn execute_and_capture_output(&self) -> Result<ExecutionResult, String> {
        // Create a temporary file with the user's code
        let mut temp_file = NamedTempFile::new()
            .map_err(|e| format!("Failed to create temp file: {}", e))?;
        
        writeln!(temp_file, "{}", self.code)
            .map_err(|e| format!("Failed to write to temp file: {}", e))?;
        
        let temp_path = temp_file.path();
        
        // Compile the code
        let compile_output = Command::new("rustc")
            .arg(temp_path)
            .arg("-o")
            .arg(temp_path.with_extension("exe"))
            .output()
            .map_err(|e| format!("Failed to run rustc: {}", e))?;
        
        if !compile_output.status.success() {
            let stderr = String::from_utf8_lossy(&compile_output.stderr);
            return Err(format!("Compilation failed: {}", stderr));
        }
        
        // Execute the compiled code
        let exe_path = temp_path.with_extension("exe");
        let run_output = Command::new(&exe_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to execute: {}", e))?;
        
        Ok(ExecutionResult {
            stdout: String::from_utf8_lossy(&run_output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&run_output.stderr).to_string(),
            exit_code: run_output.status.code().unwrap_or(-1),
        })
    }
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

/// Load user code from robot_code.rs file
pub fn load_user_code() -> Result<String, String> {
    fs::read_to_string("robot_code.rs")
        .map_err(|e| format!("Failed to read user code: {}", e))
}

/// Create a UserCodeAnalyzer from the user's robot_code.rs
pub fn create_analyzer() -> Result<UserCodeAnalyzer, String> {
    let code = load_user_code()?;
    Ok(UserCodeAnalyzer::new(&code))
}