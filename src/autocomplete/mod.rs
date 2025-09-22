use std::collections::HashSet;
use regex::Regex;

pub mod vscode_integration;
pub mod rust_intellisense;

#[derive(Debug, Clone)]
pub struct AutocompleteSuggestion {
    pub text: String,
    pub kind: SymbolKind,
    pub priority: u8, // 0 = highest priority
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Function,
    Struct,
    Enum,
    Variable,
    Keyword,
    Type,
}

#[derive(Debug, Clone)]
pub struct CodeSymbol {
    pub name: String,
    pub kind: SymbolKind,
    pub location: usize, // Position in code where defined
}

#[derive(Debug)]
pub struct CodeAnalyzer {
    symbols: Vec<CodeSymbol>,
    keywords: HashSet<String>,
    built_in_functions: HashSet<String>,
}

impl CodeAnalyzer {
    pub fn new() -> Self {
        let mut keywords = HashSet::new();
        keywords.insert("fn".to_string());
        keywords.insert("let".to_string());
        keywords.insert("mut".to_string());
        keywords.insert("if".to_string());
        keywords.insert("else".to_string());
        keywords.insert("for".to_string());
        keywords.insert("while".to_string());
        keywords.insert("loop".to_string());
        keywords.insert("match".to_string());
        keywords.insert("struct".to_string());
        keywords.insert("enum".to_string());
        keywords.insert("impl".to_string());
        keywords.insert("pub".to_string());
        keywords.insert("mod".to_string());
        keywords.insert("use".to_string());
        keywords.insert("return".to_string());
        keywords.insert("break".to_string());
        keywords.insert("continue".to_string());
        keywords.insert("true".to_string());
        keywords.insert("false".to_string());

        let mut built_in_functions = HashSet::new();
        built_in_functions.insert("println".to_string());
        built_in_functions.insert("eprintln".to_string());
        built_in_functions.insert("print".to_string());
        built_in_functions.insert("eprint".to_string());
        built_in_functions.insert("panic".to_string());
        built_in_functions.insert("scan".to_string());
        built_in_functions.insert("move_bot".to_string());
        built_in_functions.insert("grab".to_string());

        Self {
            symbols: Vec::new(),
            keywords,
            built_in_functions,
        }
    }

    pub fn analyze_code(&mut self, code: &str) {
        self.symbols.clear();

        // Extract functions
        self.extract_functions(code);

        // Extract structs
        self.extract_structs(code);

        // Extract enums
        self.extract_enums(code);

        // Extract variables
        self.extract_variables(code);
    }

    fn extract_functions(&mut self, code: &str) {
        let function_regex = Regex::new(r"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();

        for caps in function_regex.captures_iter(code) {
            if let Some(name_match) = caps.get(1) {
                let name = name_match.as_str().to_string();
                let location = name_match.start();

                self.symbols.push(CodeSymbol {
                    name,
                    kind: SymbolKind::Function,
                    location,
                });
            }
        }
    }

    fn extract_structs(&mut self, code: &str) {
        let struct_regex = Regex::new(r"struct\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\{").unwrap();

        for caps in struct_regex.captures_iter(code) {
            if let Some(name_match) = caps.get(1) {
                let name = name_match.as_str().to_string();
                let location = name_match.start();

                self.symbols.push(CodeSymbol {
                    name,
                    kind: SymbolKind::Struct,
                    location,
                });
            }
        }
    }

    fn extract_enums(&mut self, code: &str) {
        let enum_regex = Regex::new(r"enum\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\{").unwrap();

        for caps in enum_regex.captures_iter(code) {
            if let Some(name_match) = caps.get(1) {
                let name = name_match.as_str().to_string();
                let location = name_match.start();

                self.symbols.push(CodeSymbol {
                    name,
                    kind: SymbolKind::Enum,
                    location,
                });
            }
        }
    }

    fn extract_variables(&mut self, code: &str) {
        // Extract let bindings
        let let_regex = Regex::new(r"let\s+(?:mut\s+)?([a-zA-Z_][a-zA-Z0-9_]*)\s*[=:]").unwrap();

        for caps in let_regex.captures_iter(code) {
            if let Some(name_match) = caps.get(1) {
                let name = name_match.as_str().to_string();
                let location = name_match.start();

                self.symbols.push(CodeSymbol {
                    name,
                    kind: SymbolKind::Variable,
                    location,
                });
            }
        }

        // Extract function parameters
        let param_regex = Regex::new(r"fn\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\([^)]*([a-zA-Z_][a-zA-Z0-9_]*)\s*:").unwrap();

        for caps in param_regex.captures_iter(code) {
            if let Some(name_match) = caps.get(1) {
                let name = name_match.as_str().to_string();
                let location = name_match.start();

                self.symbols.push(CodeSymbol {
                    name,
                    kind: SymbolKind::Variable,
                    location,
                });
            }
        }
    }

    pub fn get_symbols(&self) -> &[CodeSymbol] {
        &self.symbols
    }

    pub fn get_keywords(&self) -> &HashSet<String> {
        &self.keywords
    }

    pub fn get_built_in_functions(&self) -> &HashSet<String> {
        &self.built_in_functions
    }
}

#[derive(Debug)]
pub struct AutocompleteEngine {
    analyzer: CodeAnalyzer,
    current_suggestion: Option<AutocompleteSuggestion>,
    enabled: bool,
    use_vscode: bool,
    vscode_integration: Option<vscode_integration::VSCodeIntegration>,
    intellisense: rust_intellisense::RustIntellisense,
}

impl AutocompleteEngine {
    pub fn new() -> Self {
        Self {
            analyzer: CodeAnalyzer::new(),
            current_suggestion: None,
            enabled: true,
            use_vscode: false, // Disable VSCode integration
            vscode_integration: None, // Remove VSCode integration
            intellisense: rust_intellisense::RustIntellisense::new(),
        }
    }

    pub fn update_suggestions(&mut self, code: &str, cursor_position: usize) {
        if !self.enabled {
            self.current_suggestion = None;
            return;
        }

        // Use fast native intellisense - no external processes, no lag!
        let completions = self.intellisense.get_completions(code, cursor_position);

        // Get the best suggestion (first one)
        self.current_suggestion = completions.into_iter()
            .next()
            .map(|completion| AutocompleteSuggestion {
                text: completion.label,
                kind: match completion.kind {
                    rust_intellisense::CompletionKind::Function => SymbolKind::Function,
                    rust_intellisense::CompletionKind::Method => SymbolKind::Function,
                    rust_intellisense::CompletionKind::Struct => SymbolKind::Struct,
                    rust_intellisense::CompletionKind::Enum => SymbolKind::Enum,
                    rust_intellisense::CompletionKind::Variable => SymbolKind::Variable,
                    rust_intellisense::CompletionKind::Keyword => SymbolKind::Keyword,
                    rust_intellisense::CompletionKind::Macro => SymbolKind::Function,
                    rust_intellisense::CompletionKind::Primitive => SymbolKind::Type,
                    _ => SymbolKind::Type,
                },
                priority: 0,
            });
    }

    /// Update user symbols - call this occasionally, not on every keystroke
    pub fn update_user_symbols(&mut self, code: &str) {
        self.intellisense.update_user_symbols(code);
    }

    fn get_current_word(&self, code: &str, cursor_position: usize) -> String {
        if cursor_position > code.len() {
            return String::new();
        }

        let chars: Vec<char> = code.chars().collect();
        let mut start = cursor_position;
        let mut end = cursor_position;

        // Find start of current word
        while start > 0 {
            let prev_char = chars[start - 1];
            if prev_char.is_alphanumeric() || prev_char == '_' {
                start -= 1;
            } else {
                break;
            }
        }

        // Find end of current word
        while end < chars.len() {
            let current_char = chars[end];
            if current_char.is_alphanumeric() || current_char == '_' {
                end += 1;
            } else {
                break;
            }
        }

        if start < end {
            chars[start..end].iter().collect()
        } else {
            String::new()
        }
    }

    fn generate_suggestion(&self, partial_word: &str) -> Option<AutocompleteSuggestion> {
        let mut candidates = Vec::new();

        // Check built-in functions
        for func in self.analyzer.get_built_in_functions() {
            if func.starts_with(partial_word) && func != partial_word {
                candidates.push(AutocompleteSuggestion {
                    text: func.clone(),
                    kind: SymbolKind::Function,
                    priority: 0,
                });
            }
        }

        // Check keywords
        for keyword in self.analyzer.get_keywords() {
            if keyword.starts_with(partial_word) && keyword != partial_word {
                candidates.push(AutocompleteSuggestion {
                    text: keyword.clone(),
                    kind: SymbolKind::Keyword,
                    priority: 1,
                });
            }
        }

        // Check user-defined symbols
        for symbol in self.analyzer.get_symbols() {
            if symbol.name.starts_with(partial_word) && symbol.name != partial_word {
                let priority = match symbol.kind {
                    SymbolKind::Variable => 2,
                    SymbolKind::Function => 3,
                    SymbolKind::Struct => 4,
                    SymbolKind::Enum => 4,
                    _ => 5,
                };

                candidates.push(AutocompleteSuggestion {
                    text: symbol.name.clone(),
                    kind: symbol.kind.clone(),
                    priority,
                });
            }
        }

        // Sort by priority and return best match
        candidates.sort_by_key(|c| c.priority);
        candidates.into_iter().next()
    }

    pub fn get_current_suggestion(&self) -> Option<&AutocompleteSuggestion> {
        self.current_suggestion.as_ref()
    }

    pub fn accept_suggestion(&mut self) -> Option<String> {
        if let Some(suggestion) = &self.current_suggestion {
            let result = suggestion.text.clone();
            self.current_suggestion = None;
            Some(result)
        } else {
            None
        }
    }

    pub fn clear_suggestion(&mut self) {
        self.current_suggestion = None;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.current_suggestion = None;
        }
    }

    pub fn set_vscode_enabled(&mut self, enabled: bool) {
        self.use_vscode = enabled && self.vscode_integration.as_ref().map_or(false, |v| v.is_available());
    }

    pub fn is_vscode_enabled(&self) -> bool {
        self.use_vscode
    }

    pub fn is_vscode_available(&self) -> bool {
        self.vscode_integration.as_ref().map_or(false, |v| v.is_available())
    }

    fn get_vscode_suggestion(&mut self, code: &str, cursor_position: usize, current_word: &str) -> Option<AutocompleteSuggestion> {
        // Convert cursor position to line/character first
        let (line, character) = self.cursor_to_line_character(code, cursor_position);

        if let Some(ref mut vscode) = self.vscode_integration {
            if let Some(completions) = vscode.get_completions(code, line, character) {
                // Find the best completion that matches the current word
                for completion in completions {
                    if completion.text.starts_with(current_word) && completion.text != current_word {
                        return Some(completion);
                    }
                }
            }
        }
        None
    }

    pub fn cursor_to_line_character(&self, code: &str, cursor_position: usize) -> (u32, u32) {
        let mut line = 0;
        let mut character = 0;

        for (i, ch) in code.chars().enumerate() {
            if i >= cursor_position {
                break;
            }

            if ch == '\n' {
                line += 1;
                character = 0;
            } else {
                character += 1;
            }
        }

        (line, character)
    }
}