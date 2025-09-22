// Fast, native Rust intellisense - no external processes!
use std::collections::{HashMap, HashSet};
use once_cell::sync::Lazy;

// Pre-computed Rust standard library data for instant completions
static RUST_STD_ITEMS: Lazy<HashMap<&'static str, Vec<CompletionItem>>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Common types
    map.insert("", vec![
        CompletionItem::new("String", CompletionKind::Struct, "A UTF-8 encoded, growable string"),
        CompletionItem::new("Vec", CompletionKind::Struct, "A contiguous growable array type"),
        CompletionItem::new("HashMap", CompletionKind::Struct, "A hash map implementation"),
        CompletionItem::new("Option", CompletionKind::Enum, "Optional value: Some(T) or None"),
        CompletionItem::new("Result", CompletionKind::Enum, "Result type: Ok(T) or Err(E)"),
        CompletionItem::new("Box", CompletionKind::Struct, "A pointer type for heap allocation"),
        CompletionItem::new("Rc", CompletionKind::Struct, "Single-threaded reference-counting pointer"),
        CompletionItem::new("Arc", CompletionKind::Struct, "Thread-safe reference-counting pointer"),
        CompletionItem::new("RefCell", CompletionKind::Struct, "Mutable memory location with dynamically checked borrow rules"),
        CompletionItem::new("Mutex", CompletionKind::Struct, "Mutual exclusion primitive for protecting shared data"),
        CompletionItem::new("bool", CompletionKind::Primitive, "Boolean type"),
        CompletionItem::new("char", CompletionKind::Primitive, "Character type"),
        CompletionItem::new("i8", CompletionKind::Primitive, "8-bit signed integer"),
        CompletionItem::new("i16", CompletionKind::Primitive, "16-bit signed integer"),
        CompletionItem::new("i32", CompletionKind::Primitive, "32-bit signed integer"),
        CompletionItem::new("i64", CompletionKind::Primitive, "64-bit signed integer"),
        CompletionItem::new("i128", CompletionKind::Primitive, "128-bit signed integer"),
        CompletionItem::new("isize", CompletionKind::Primitive, "Pointer-sized signed integer"),
        CompletionItem::new("u8", CompletionKind::Primitive, "8-bit unsigned integer"),
        CompletionItem::new("u16", CompletionKind::Primitive, "16-bit unsigned integer"),
        CompletionItem::new("u32", CompletionKind::Primitive, "32-bit unsigned integer"),
        CompletionItem::new("u64", CompletionKind::Primitive, "64-bit unsigned integer"),
        CompletionItem::new("u128", CompletionKind::Primitive, "128-bit unsigned integer"),
        CompletionItem::new("usize", CompletionKind::Primitive, "Pointer-sized unsigned integer"),
        CompletionItem::new("f32", CompletionKind::Primitive, "32-bit floating point"),
        CompletionItem::new("f64", CompletionKind::Primitive, "64-bit floating point"),
    ]);

    // String methods
    map.insert("String.", vec![
        CompletionItem::new("new()", CompletionKind::Method, "Creates a new empty String"),
        CompletionItem::new("from()", CompletionKind::Method, "Creates a String from &str"),
        CompletionItem::new("push()", CompletionKind::Method, "Appends a char to the String"),
        CompletionItem::new("push_str()", CompletionKind::Method, "Appends a &str to the String"),
        CompletionItem::new("pop()", CompletionKind::Method, "Removes the last character"),
        CompletionItem::new("len()", CompletionKind::Method, "Returns the length in bytes"),
        CompletionItem::new("is_empty()", CompletionKind::Method, "Returns true if empty"),
        CompletionItem::new("clear()", CompletionKind::Method, "Truncates to zero length"),
        CompletionItem::new("chars()", CompletionKind::Method, "Returns an iterator over chars"),
        CompletionItem::new("bytes()", CompletionKind::Method, "Returns an iterator over bytes"),
        CompletionItem::new("contains()", CompletionKind::Method, "Checks if pattern is contained"),
        CompletionItem::new("starts_with()", CompletionKind::Method, "Checks if string starts with pattern"),
        CompletionItem::new("ends_with()", CompletionKind::Method, "Checks if string ends with pattern"),
        CompletionItem::new("find()", CompletionKind::Method, "Searches for pattern"),
        CompletionItem::new("replace()", CompletionKind::Method, "Replaces all matches of pattern"),
        CompletionItem::new("trim()", CompletionKind::Method, "Returns string with whitespace removed"),
        CompletionItem::new("to_lowercase()", CompletionKind::Method, "Returns lowercase version"),
        CompletionItem::new("to_uppercase()", CompletionKind::Method, "Returns uppercase version"),
        CompletionItem::new("parse()", CompletionKind::Method, "Parses string into another type"),
    ]);

    // Vec methods
    map.insert("Vec.", vec![
        CompletionItem::new("new()", CompletionKind::Method, "Creates a new empty Vec"),
        CompletionItem::new("with_capacity()", CompletionKind::Method, "Creates Vec with specified capacity"),
        CompletionItem::new("push()", CompletionKind::Method, "Appends element to the back"),
        CompletionItem::new("pop()", CompletionKind::Method, "Removes last element"),
        CompletionItem::new("insert()", CompletionKind::Method, "Inserts element at index"),
        CompletionItem::new("remove()", CompletionKind::Method, "Removes element at index"),
        CompletionItem::new("len()", CompletionKind::Method, "Returns number of elements"),
        CompletionItem::new("is_empty()", CompletionKind::Method, "Returns true if empty"),
        CompletionItem::new("clear()", CompletionKind::Method, "Removes all elements"),
        CompletionItem::new("get()", CompletionKind::Method, "Returns reference to element"),
        CompletionItem::new("get_mut()", CompletionKind::Method, "Returns mutable reference"),
        CompletionItem::new("first()", CompletionKind::Method, "Returns first element"),
        CompletionItem::new("last()", CompletionKind::Method, "Returns last element"),
        CompletionItem::new("iter()", CompletionKind::Method, "Returns iterator"),
        CompletionItem::new("iter_mut()", CompletionKind::Method, "Returns mutable iterator"),
        CompletionItem::new("sort()", CompletionKind::Method, "Sorts the Vec"),
        CompletionItem::new("reverse()", CompletionKind::Method, "Reverses the order"),
        CompletionItem::new("contains()", CompletionKind::Method, "Checks if value is contained"),
        CompletionItem::new("extend()", CompletionKind::Method, "Extends with iterator contents"),
    ]);

    // Iterator methods (common for many types)
    map.insert("iter.", vec![
        CompletionItem::new("map()", CompletionKind::Method, "Transforms each element"),
        CompletionItem::new("filter()", CompletionKind::Method, "Filters elements by predicate"),
        CompletionItem::new("fold()", CompletionKind::Method, "Reduces to single value"),
        CompletionItem::new("collect()", CompletionKind::Method, "Collects into collection"),
        CompletionItem::new("count()", CompletionKind::Method, "Counts elements"),
        CompletionItem::new("sum()", CompletionKind::Method, "Sums elements"),
        CompletionItem::new("product()", CompletionKind::Method, "Multiplies elements"),
        CompletionItem::new("min()", CompletionKind::Method, "Returns minimum element"),
        CompletionItem::new("max()", CompletionKind::Method, "Returns maximum element"),
        CompletionItem::new("find()", CompletionKind::Method, "Finds first matching element"),
        CompletionItem::new("position()", CompletionKind::Method, "Finds position of element"),
        CompletionItem::new("any()", CompletionKind::Method, "Tests if any element matches"),
        CompletionItem::new("all()", CompletionKind::Method, "Tests if all elements match"),
        CompletionItem::new("take()", CompletionKind::Method, "Takes first n elements"),
        CompletionItem::new("skip()", CompletionKind::Method, "Skips first n elements"),
        CompletionItem::new("chain()", CompletionKind::Method, "Chains with another iterator"),
        CompletionItem::new("zip()", CompletionKind::Method, "Zips with another iterator"),
        CompletionItem::new("enumerate()", CompletionKind::Method, "Yields (index, element) pairs"),
        CompletionItem::new("rev()", CompletionKind::Method, "Reverses the iterator"),
        CompletionItem::new("cloned()", CompletionKind::Method, "Clones all elements"),
    ]);

    // Option methods
    map.insert("Option.", vec![
        CompletionItem::new("Some()", CompletionKind::Constructor, "Wraps value in Some variant"),
        CompletionItem::new("None", CompletionKind::Constructor, "The None variant"),
        CompletionItem::new("is_some()", CompletionKind::Method, "Returns true if Some"),
        CompletionItem::new("is_none()", CompletionKind::Method, "Returns true if None"),
        CompletionItem::new("unwrap()", CompletionKind::Method, "Unwraps value or panics"),
        CompletionItem::new("unwrap_or()", CompletionKind::Method, "Unwraps or returns default"),
        CompletionItem::new("unwrap_or_else()", CompletionKind::Method, "Unwraps or computes default"),
        CompletionItem::new("unwrap_or_default()", CompletionKind::Method, "Unwraps or returns Default::default()"),
        CompletionItem::new("expect()", CompletionKind::Method, "Unwraps with custom panic message"),
        CompletionItem::new("map()", CompletionKind::Method, "Maps Some value"),
        CompletionItem::new("map_or()", CompletionKind::Method, "Maps or returns default"),
        CompletionItem::new("and_then()", CompletionKind::Method, "Chains operations on Some"),
        CompletionItem::new("or()", CompletionKind::Method, "Returns self or alternative"),
        CompletionItem::new("or_else()", CompletionKind::Method, "Returns self or computes alternative"),
        CompletionItem::new("take()", CompletionKind::Method, "Takes value leaving None"),
        CompletionItem::new("replace()", CompletionKind::Method, "Replaces value"),
        CompletionItem::new("as_ref()", CompletionKind::Method, "Converts to Option<&T>"),
        CompletionItem::new("as_mut()", CompletionKind::Method, "Converts to Option<&mut T>"),
    ]);

    // Result methods
    map.insert("Result.", vec![
        CompletionItem::new("Ok()", CompletionKind::Constructor, "Success variant"),
        CompletionItem::new("Err()", CompletionKind::Constructor, "Error variant"),
        CompletionItem::new("is_ok()", CompletionKind::Method, "Returns true if Ok"),
        CompletionItem::new("is_err()", CompletionKind::Method, "Returns true if Err"),
        CompletionItem::new("unwrap()", CompletionKind::Method, "Unwraps Ok value or panics"),
        CompletionItem::new("unwrap_err()", CompletionKind::Method, "Unwraps Err value or panics"),
        CompletionItem::new("unwrap_or()", CompletionKind::Method, "Unwraps Ok or returns default"),
        CompletionItem::new("unwrap_or_else()", CompletionKind::Method, "Unwraps Ok or computes default"),
        CompletionItem::new("expect()", CompletionKind::Method, "Unwraps Ok with custom panic"),
        CompletionItem::new("expect_err()", CompletionKind::Method, "Unwraps Err with custom panic"),
        CompletionItem::new("map()", CompletionKind::Method, "Maps Ok value"),
        CompletionItem::new("map_err()", CompletionKind::Method, "Maps Err value"),
        CompletionItem::new("and_then()", CompletionKind::Method, "Chains operations on Ok"),
        CompletionItem::new("or()", CompletionKind::Method, "Returns self or alternative if Err"),
        CompletionItem::new("or_else()", CompletionKind::Method, "Chains operations on Err"),
        CompletionItem::new("ok()", CompletionKind::Method, "Converts to Option<T>"),
        CompletionItem::new("err()", CompletionKind::Method, "Converts to Option<E>"),
        CompletionItem::new("as_ref()", CompletionKind::Method, "Converts to Result<&T, &E>"),
        CompletionItem::new("as_mut()", CompletionKind::Method, "Converts to Result<&mut T, &mut E>"),
    ]);

    map
});

// Rust keywords for instant completion
static RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
    "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "Self", "self", "static", "struct", "super",
    "trait", "true", "type", "unsafe", "use", "where", "while", "yield",
];

// Common macros
static RUST_MACROS: &[&str] = &[
    "println!", "print!", "eprintln!", "eprint!", "format!", "write!", "writeln!",
    "panic!", "assert!", "assert_eq!", "assert_ne!", "debug_assert!", "debug_assert_eq!",
    "debug_assert_ne!", "vec!", "include!", "include_str!", "include_bytes!", "concat!",
    "env!", "option_env!", "cfg!", "line!", "column!", "file!", "module_path!",
    "stringify!", "todo!", "unimplemented!", "unreachable!", "compile_error!",
];

#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: String,
    pub insert_text: Option<String>,
    pub snippet: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompletionKind {
    Keyword,
    Function,
    Method,
    Struct,
    Enum,
    Trait,
    Module,
    Variable,
    Constant,
    Macro,
    Primitive,
    Constructor,
    Field,
    Parameter,
}

impl CompletionItem {
    pub fn new(label: &str, kind: CompletionKind, detail: &str) -> Self {
        Self {
            label: label.to_string(),
            kind,
            detail: detail.to_string(),
            insert_text: None,
            snippet: None,
        }
    }

    pub fn with_insert_text(mut self, text: &str) -> Self {
        self.insert_text = Some(text.to_string());
        self
    }

    pub fn with_snippet(mut self, snippet: &str) -> Self {
        self.snippet = Some(snippet.to_string());
        self
    }
}

#[derive(Debug)]
pub struct RustIntellisense {
    // Cache for parsed user code
    user_symbols: HashMap<String, CompletionKind>,
    // Cache for recent completions
    recent_completions: Vec<CompletionItem>,
    // Current context
    current_context: String,
}

impl RustIntellisense {
    pub fn new() -> Self {
        Self {
            user_symbols: HashMap::new(),
            recent_completions: Vec::new(),
            current_context: String::new(),
        }
    }

    /// Fast, non-blocking completion suggestions
    pub fn get_completions(&mut self, code: &str, cursor_pos: usize) -> Vec<CompletionItem> {
        // Don't suggest anything for completely empty code or invalid cursor position
        if code.is_empty() || cursor_pos == 0 {
            return Vec::new();
        }

        // Extract context around cursor
        let context = self.extract_context(code, cursor_pos);

        // Don't suggest anything if context is empty or just whitespace
        if context.trim().is_empty() {
            return Vec::new();
        }

        // If context hasn't changed, return cached results
        if context == self.current_context && !self.recent_completions.is_empty() {
            return self.recent_completions.clone();
        }

        self.current_context = context.clone();
        let mut completions = Vec::new();

        // Check for method completion (e.g., "string.")
        if let Some(method_context) = self.extract_method_context(&context) {
            if let Some(methods) = RUST_STD_ITEMS.get(method_context) {
                completions.extend_from_slice(methods);
            }
        }

        // Get the partial word being typed
        let partial = self.extract_partial_word(&context);

        // Only show suggestions if there's actually meaningful text being typed
        // Require at least 1 character to start suggesting
        if partial.is_empty() || partial.trim().is_empty() {
            // Don't show any suggestions for empty input
            // This prevents unwanted autocomplete when there's nothing to complete
            return Vec::new();
        } else {
            // Filter suggestions based on partial
            // Keywords
            for keyword in RUST_KEYWORDS {
                if keyword.starts_with(&partial) {
                    completions.push(CompletionItem::new(
                        keyword,
                        CompletionKind::Keyword,
                        "Rust keyword"
                    ));
                }
            }

            // Macros
            for macro_name in RUST_MACROS {
                if macro_name.starts_with(&partial) {
                    completions.push(CompletionItem::new(
                        macro_name,
                        CompletionKind::Macro,
                        "Rust macro"
                    ));
                }
            }

            // Standard library types
            if let Some(items) = RUST_STD_ITEMS.get("") {
                for item in items {
                    if item.label.to_lowercase().starts_with(&partial.to_lowercase()) {
                        completions.push(item.clone());
                    }
                }
            }

            // User-defined symbols
            for (symbol, kind) in &self.user_symbols {
                if symbol.to_lowercase().starts_with(&partial.to_lowercase()) {
                    completions.push(CompletionItem::new(
                        symbol,
                        kind.clone(),
                        "User-defined"
                    ));
                }
            }
        }

        // Limit results for performance
        completions.truncate(20);

        // Cache the results
        self.recent_completions = completions.clone();

        completions
    }

    /// Update user-defined symbols (call this periodically, not on every keystroke)
    pub fn update_user_symbols(&mut self, code: &str) {
        self.user_symbols.clear();

        // Quick regex-based extraction (much faster than full parsing)
        // Functions
        if let Ok(re) = regex::Regex::new(r"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)") {
            for cap in re.captures_iter(code) {
                if let Some(name) = cap.get(1) {
                    self.user_symbols.insert(name.as_str().to_string(), CompletionKind::Function);
                }
            }
        }

        // Structs
        if let Ok(re) = regex::Regex::new(r"struct\s+([a-zA-Z_][a-zA-Z0-9_]*)") {
            for cap in re.captures_iter(code) {
                if let Some(name) = cap.get(1) {
                    self.user_symbols.insert(name.as_str().to_string(), CompletionKind::Struct);
                }
            }
        }

        // Enums
        if let Ok(re) = regex::Regex::new(r"enum\s+([a-zA-Z_][a-zA-Z0-9_]*)") {
            for cap in re.captures_iter(code) {
                if let Some(name) = cap.get(1) {
                    self.user_symbols.insert(name.as_str().to_string(), CompletionKind::Enum);
                }
            }
        }

        // Variables (let bindings)
        if let Ok(re) = regex::Regex::new(r"let\s+(?:mut\s+)?([a-zA-Z_][a-zA-Z0-9_]*)") {
            for cap in re.captures_iter(code) {
                if let Some(name) = cap.get(1) {
                    self.user_symbols.insert(name.as_str().to_string(), CompletionKind::Variable);
                }
            }
        }
    }

    fn extract_context(&self, code: &str, cursor_pos: usize) -> String {
        // Get text around cursor (up to 100 chars before)
        let start = cursor_pos.saturating_sub(100);
        let context = &code[start..cursor_pos.min(code.len())];
        context.to_string()
    }

    fn extract_method_context(&self, context: &str) -> Option<&str> {
        // Check if we're after a dot (method call)
        if let Some(dot_pos) = context.rfind('.') {
            // Get the word before the dot
            let before_dot = &context[..dot_pos];
            if before_dot.ends_with("String") {
                return Some("String.");
            } else if before_dot.ends_with("Vec") {
                return Some("Vec.");
            } else if before_dot.ends_with("Option") {
                return Some("Option.");
            } else if before_dot.ends_with("Result") {
                return Some("Result.");
            } else if before_dot.contains("iter") || before_dot.contains("Iter") {
                return Some("iter.");
            }
        }
        None
    }

    fn extract_partial_word(&self, context: &str) -> String {
        // Get the partial word being typed
        let chars: Vec<char> = context.chars().collect();
        let mut end = chars.len();

        // Find the start of the current word
        while end > 0 {
            let ch = chars[end - 1];
            if ch.is_alphanumeric() || ch == '_' {
                end -= 1;
            } else {
                break;
            }
        }

        context[end..].to_string()
    }

    /// Clear caches when switching files or contexts
    pub fn clear_cache(&mut self) {
        self.recent_completions.clear();
        self.current_context.clear();
    }
}