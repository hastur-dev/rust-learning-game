use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Write, BufRead, BufReader, Read};
use serde::{Serialize, Deserialize};
use serde_json::Value;

// Windows-specific imports for running processes in background
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeCompletionItem {
    pub label: String,
    pub kind: u32,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
    pub filter_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeCompletionList {
    pub is_incomplete: bool,
    pub items: Vec<VSCodeCompletionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LSPRequest {
    pub jsonrpc: String,
    pub id: u32,
    pub method: String,
    pub params: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LSPResponse {
    pub jsonrpc: String,
    pub id: Option<u32>,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

#[derive(Debug)]
pub struct VSCodeIntegration {
    vscode_path: Option<PathBuf>,
    workspace_path: PathBuf,
    rust_analyzer_available: bool,
    request_id: u32,
}

impl VSCodeIntegration {
    pub fn new(workspace_path: PathBuf) -> Self {
        // We DON'T actually need VSCode installed - we just need rust-analyzer
        // This prevents VSCode from ever opening
        let rust_analyzer_available = Self::check_rust_analyzer_availability(&None);

        Self {
            vscode_path: None, // Deliberately set to None to prevent VSCode from opening
            workspace_path,
            rust_analyzer_available,
            request_id: 0,
        }
    }

    pub fn is_available(&self) -> bool {
        // Only check if rust-analyzer is available, not VSCode
        self.rust_analyzer_available
    }

    pub fn get_completions(&mut self, file_content: &str, line: u32, character: u32) -> Option<Vec<super::AutocompleteSuggestion>> {
        if !self.is_available() {
            return None;
        }

        // Create a temporary Rust file
        let temp_file = self.workspace_path.join("temp_completion.rs");
        if let Err(_) = fs::write(&temp_file, file_content) {
            return None;
        }

        // Try to get completions using rust-analyzer
        let completions = self.get_rust_analyzer_completions(&temp_file, line, character);

        // Clean up temporary file
        let _ = fs::remove_file(&temp_file);

        completions
    }

    fn find_vscode_installation() -> Option<PathBuf> {
        // Check common VSCode installation paths
        let potential_paths = vec![
            // Windows
            PathBuf::from(r"C:\Users").join(std::env::var("USERNAME").unwrap_or_default()).join(r"AppData\Local\Programs\Microsoft VS Code\Code.exe"),
            PathBuf::from(r"C:\Program Files\Microsoft VS Code\Code.exe"),
            PathBuf::from(r"C:\Program Files (x86)\Microsoft VS Code\Code.exe"),

            // Add to PATH
            PathBuf::from("code"),
        ];

        for path in potential_paths {
            if path.exists() || Self::command_exists(&path) {
                return Some(path);
            }
        }

        None
    }

    fn command_exists(command: &Path) -> bool {
        Command::new(command)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
    }

    fn check_rust_analyzer_availability(vscode_path: &Option<PathBuf>) -> bool {
        if let Some(vscode) = vscode_path {
            // Check if rust-analyzer extension is installed
            let output = Command::new(vscode)
                .args(&["--list-extensions"])
                .output();

            if let Ok(output) = output {
                let extensions = String::from_utf8_lossy(&output.stdout);
                return extensions.contains("rust-lang.rust-analyzer") ||
                       extensions.contains("matklad.rust-analyzer");
            }
        }

        // Also check if rust-analyzer is available as a standalone binary
        Command::new("rust-analyzer")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
    }

    fn get_rust_analyzer_completions(&mut self, file_path: &Path, line: u32, character: u32) -> Option<Vec<super::AutocompleteSuggestion>> {
        // Only use rust-analyzer directly - NEVER start VSCode
        self.try_rust_analyzer_direct(file_path, line, character)
    }

    fn try_rust_analyzer_direct(&mut self, file_path: &Path, line: u32, character: u32) -> Option<Vec<super::AutocompleteSuggestion>> {
        // Create a simple LSP client for rust-analyzer
        let mut command = Command::new("rust-analyzer");
        command.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        // On Windows, ensure the process runs completely hidden in the background
        #[cfg(windows)]
        {
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            command.creation_flags(CREATE_NO_WINDOW);
        }

        let mut child = command.spawn().ok()?;

        let stdin = child.stdin.as_mut()?;
        let stdout = child.stdout.as_mut()?;
        let mut reader = BufReader::new(stdout);

        // Send initialize request
        self.request_id += 1;
        let initialize_request = LSPRequest {
            jsonrpc: "2.0".to_string(),
            id: self.request_id,
            method: "initialize".to_string(),
            params: serde_json::json!({
                "processId": std::process::id(),
                "rootUri": format!("file://{}", self.workspace_path.display()),
                "capabilities": {
                    "textDocument": {
                        "completion": {
                            "completionItem": {
                                "snippetSupport": true,
                                "resolveSupport": {
                                    "properties": ["documentation", "detail"]
                                }
                            }
                        }
                    }
                }
            }),
        };

        if let Err(_) = self.send_lsp_request(stdin, &initialize_request) {
            let _ = child.kill();
            return None;
        }

        // Wait for initialize response
        if let Err(_) = self.read_lsp_response(&mut reader) {
            let _ = child.kill();
            return None;
        }

        // Send initialized notification
        let initialized_notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "initialized",
            "params": {}
        });

        if let Err(_) = self.send_lsp_notification(stdin, &initialized_notification) {
            let _ = child.kill();
            return None;
        }

        // Open document
        let file_uri = format!("file://{}", file_path.display());
        let file_content = fs::read_to_string(file_path).ok()?;

        let did_open_notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": file_uri,
                    "languageId": "rust",
                    "version": 1,
                    "text": file_content
                }
            }
        });

        if let Err(_) = self.send_lsp_notification(stdin, &did_open_notification) {
            let _ = child.kill();
            return None;
        }

        // Request completions
        self.request_id += 1;
        let completion_request = LSPRequest {
            jsonrpc: "2.0".to_string(),
            id: self.request_id,
            method: "textDocument/completion".to_string(),
            params: serde_json::json!({
                "textDocument": {
                    "uri": file_uri
                },
                "position": {
                    "line": line,
                    "character": character
                }
            }),
        };

        if let Err(_) = self.send_lsp_request(stdin, &completion_request) {
            let _ = child.kill();
            return None;
        }

        // Read completion response
        let response = self.read_lsp_response(&mut reader).ok()?;
        let _ = child.kill();

        self.parse_completion_response(response)
    }


    fn get_rust_analyzer_background_completions(&mut self, workspace_path: &Path, file_path: &Path, line: u32, character: u32) -> Option<Vec<super::AutocompleteSuggestion>> {
        // Run rust-analyzer in the background without any UI
        // This provides the same completion quality as VSCode's rust-analyzer extension

        // Change to the workspace directory for proper Rust project context
        let original_dir = std::env::current_dir().ok()?;
        let _ = std::env::set_current_dir(workspace_path);

        // Initialize cargo project if needed
        let mut cargo_command = Command::new("cargo");
        cargo_command
            .arg("check")
            .arg("--quiet")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        // On Windows, ensure the cargo check also runs hidden
        #[cfg(windows)]
        {
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cargo_command.creation_flags(CREATE_NO_WINDOW);
        }

        let _ = cargo_command
            .spawn()
            .and_then(|mut child| child.wait());

        // Run rust-analyzer in background mode
        let mut command = Command::new("rust-analyzer");
        command.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        // On Windows, ensure the process runs completely hidden in the background
        #[cfg(windows)]
        {
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            command.creation_flags(CREATE_NO_WINDOW);
        }

        let mut child = command.spawn().ok()?;

        let result = self.communicate_with_rust_analyzer(&mut child, file_path, line, character);

        // Restore original directory
        let _ = std::env::set_current_dir(original_dir);

        // Terminate the background rust-analyzer process
        let _ = child.kill();
        let _ = child.wait();

        result
    }

    fn communicate_with_rust_analyzer(&mut self, child: &mut std::process::Child, file_path: &Path, line: u32, character: u32) -> Option<Vec<super::AutocompleteSuggestion>> {
        // Communicate with rust-analyzer using LSP protocol in the background
        let stdin = child.stdin.as_mut()?;
        let stdout = child.stdout.as_mut()?;
        let mut reader = BufReader::new(stdout);

        // Send initialization and completion requests
        let workspace_uri = format!("file://{}", file_path.parent()?.display());
        let file_uri = format!("file://{}", file_path.display());

        // Initialize rust-analyzer
        self.request_id += 1;
        let initialize_request = LSPRequest {
            jsonrpc: "2.0".to_string(),
            id: self.request_id,
            method: "initialize".to_string(),
            params: serde_json::json!({
                "processId": std::process::id(),
                "rootUri": workspace_uri,
                "capabilities": {
                    "textDocument": {
                        "completion": {
                            "completionItem": { "snippetSupport": true }
                        }
                    }
                }
            }),
        };

        // Send requests in background without blocking the main game thread
        if self.send_lsp_request(stdin, &initialize_request).is_err() {
            return None;
        }

        // Read file content for didOpen notification
        let file_content = fs::read_to_string(file_path).ok()?;

        // Send didOpen notification
        let did_open = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": file_uri,
                    "languageId": "rust",
                    "version": 1,
                    "text": file_content
                }
            }
        });

        let _ = self.send_lsp_notification(stdin, &did_open);

        // Request completions
        self.request_id += 1;
        let completion_request = LSPRequest {
            jsonrpc: "2.0".to_string(),
            id: self.request_id,
            method: "textDocument/completion".to_string(),
            params: serde_json::json!({
                "textDocument": { "uri": file_uri },
                "position": { "line": line, "character": character }
            }),
        };

        if self.send_lsp_request(stdin, &completion_request).is_err() {
            return None;
        }

        // Read completion response in background
        self.read_completion_response(&mut reader)
    }

    fn send_lsp_notification(&self, stdin: &mut std::process::ChildStdin, notification: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(notification)?;
        let message = format!("Content-Length: {}\r\n\r\n{}", json.len(), json);
        stdin.write_all(message.as_bytes())?;
        stdin.flush()?;
        Ok(())
    }

    fn parse_content_length(&self, header: &str) -> Option<usize> {
        header
            .strip_prefix("Content-Length: ")
            .and_then(|s| s.trim().parse().ok())
    }

    fn read_completion_response(&mut self, reader: &mut BufReader<&mut std::process::ChildStdout>) -> Option<Vec<super::AutocompleteSuggestion>> {
        // Read LSP responses in background with timeout to avoid blocking
        let mut buffer = String::new();

        // Simple timeout mechanism - don't wait too long for responses
        for _ in 0..10 {
            buffer.clear();
            if reader.read_line(&mut buffer).is_ok() && buffer.contains("Content-Length:") {
                if let Some(length) = self.parse_content_length(&buffer) {
                    let mut content = vec![0u8; length];
                    let _ = reader.read_line(&mut String::new()); // Skip empty line
                    if reader.read_exact(&mut content).is_ok() {
                        if let Ok(json_str) = String::from_utf8(content) {
                            if let Ok(response) = serde_json::from_str::<LSPResponse>(&json_str) {
                                if response.id == Some(self.request_id) {
                                    return self.parse_completion_response(response);
                                }
                            }
                        }
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(50)); // Small delay
        }

        None
    }

    fn send_lsp_request(&self, stdin: &mut std::process::ChildStdin, request: &LSPRequest) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(request)?;
        let message = format!("Content-Length: {}\r\n\r\n{}", json.len(), json);
        stdin.write_all(message.as_bytes())?;
        stdin.flush()?;
        Ok(())
    }


    fn read_lsp_response(&self, reader: &mut BufReader<&mut std::process::ChildStdout>) -> Result<LSPResponse, Box<dyn std::error::Error>> {
        // Read Content-Length header
        let mut line = String::new();
        reader.read_line(&mut line)?;

        let content_length: usize = line
            .strip_prefix("Content-Length: ")
            .and_then(|s| s.trim().parse().ok())
            .ok_or("Invalid Content-Length header")?;

        // Skip empty line
        line.clear();
        reader.read_line(&mut line)?;

        // Read JSON content
        let mut content = vec![0u8; content_length];
        std::io::Read::read_exact(reader, &mut content)?;

        let response: LSPResponse = serde_json::from_slice(&content)?;
        Ok(response)
    }

    fn parse_completion_response(&self, response: LSPResponse) -> Option<Vec<super::AutocompleteSuggestion>> {
        let result = response.result?;

        // Handle both CompletionList and CompletionItem[] formats
        let items = if let Some(list) = result.get("items") {
            list.as_array()?
        } else if result.is_array() {
            result.as_array()?
        } else {
            return None;
        };

        let mut suggestions = Vec::new();

        for item in items {
            if let Some(label) = item.get("label").and_then(|l| l.as_str()) {
                let kind = self.lsp_completion_kind_to_symbol_kind(
                    item.get("kind").and_then(|k| k.as_u64()).unwrap_or(1) as u32
                );

                let priority = match kind {
                    super::SymbolKind::Keyword => 0,
                    super::SymbolKind::Function => 1,
                    super::SymbolKind::Variable => 2,
                    super::SymbolKind::Struct => 3,
                    super::SymbolKind::Enum => 3,
                    super::SymbolKind::Type => 4,
                };

                suggestions.push(super::AutocompleteSuggestion {
                    text: label.to_string(),
                    kind,
                    priority,
                });
            }
        }

        Some(suggestions)
    }

    fn lsp_completion_kind_to_symbol_kind(&self, lsp_kind: u32) -> super::SymbolKind {
        match lsp_kind {
            1 => super::SymbolKind::Type,     // Text
            2 => super::SymbolKind::Function, // Method
            3 => super::SymbolKind::Function, // Function
            4 => super::SymbolKind::Function, // Constructor
            5 => super::SymbolKind::Variable, // Field
            6 => super::SymbolKind::Variable, // Variable
            7 => super::SymbolKind::Struct,   // Class
            8 => super::SymbolKind::Type,     // Interface
            9 => super::SymbolKind::Type,     // Module
            10 => super::SymbolKind::Variable, // Property
            11 => super::SymbolKind::Type,    // Unit
            12 => super::SymbolKind::Type,    // Value
            13 => super::SymbolKind::Enum,    // Enum
            14 => super::SymbolKind::Keyword, // Keyword
            15 => super::SymbolKind::Type,    // Snippet
            16 => super::SymbolKind::Type,    // Color
            17 => super::SymbolKind::Type,    // File
            18 => super::SymbolKind::Type,    // Reference
            19 => super::SymbolKind::Type,    // Folder
            20 => super::SymbolKind::Variable, // EnumMember
            21 => super::SymbolKind::Variable, // Constant
            22 => super::SymbolKind::Struct,  // Struct
            23 => super::SymbolKind::Type,    // Event
            24 => super::SymbolKind::Type,    // Operator
            25 => super::SymbolKind::Type,    // TypeParameter
            _ => super::SymbolKind::Type,
        }
    }

    pub fn setup_workspace(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create Cargo.toml if it doesn't exist
        let cargo_toml = self.workspace_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            fs::write(&cargo_toml, r#"
[package]
name = "rust_steam_game_code"
version = "0.1.0"
edition = "2021"

[dependencies]
# Add any dependencies your game code might need
"#)?;
        }

        // Create src directory
        let src_dir = self.workspace_path.join("src");
        fs::create_dir_all(&src_dir)?;

        // Create lib.rs with game function stubs
        let lib_rs = src_dir.join("lib.rs");
        if !lib_rs.exists() {
            fs::write(&lib_rs, r#"
// Rust Steam Game - Function stubs for autocompletion

/// Move the robot in a direction
pub fn move_bot(direction: &str) -> bool {
    unimplemented!()
}

/// Scan the area around the robot
pub fn scan(direction: &str) -> String {
    unimplemented!()
}

/// Grab an item at the robot's current position
pub fn grab() -> bool {
    unimplemented!()
}

/// Laser in a specific direction
pub fn laser_direction(direction: &str) -> bool {
    unimplemented!()
}

/// Laser at specific coordinates
pub fn laser_tile(x: i32, y: i32) -> bool {
    unimplemented!()
}

/// Open a door at the current position
pub fn open_door() -> bool {
    unimplemented!()
}
"#)?;
        }

        Ok(())
    }
}