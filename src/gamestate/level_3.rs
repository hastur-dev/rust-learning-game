use super::types::Game;

impl Game {
    // Level 3 specific tutorial system methods
    pub fn get_level_3_task_message(&self) -> String {
        if self.level_idx != 2 {
            return String::new(); // Only for level 3
        }

        match self.tutorial_state.current_task {
            0 => "📋 **TASK 1/5: Work with Integer Types**\n\nLearn about Rust's integer types - signed and unsigned:\n\n```rust\n// Signed integers (can be negative)\nlet signed: i32 = -42;\nlet large_signed: i64 = -1_000_000;\n\n// Unsigned integers (only positive)\nlet unsigned: u32 = 255;\nlet small_unsigned: u8 = 200;\n\nprintln!(\"Signed i32: {}\", signed);\nprintln!(\"Unsigned u32: {}\", unsigned);\n```\n\n• `i32` = signed 32-bit (-2 billion to +2 billion)\n• `u32` = unsigned 32-bit (0 to 4 billion)\n• `i64`/`u8` = different sizes for different needs".to_string(),

            1 => "📋 **TASK 2/5: Floating Point Numbers**\n\nWork with decimal numbers using f64 and f32:\n\n```rust\n// f64 is the default (double precision)\nlet pi: f64 = 3.141592653589793;\nlet e = 2.71828; // Type inferred as f64\n\n// f32 is single precision (less precise)\nlet pi_f32: f32 = 3.14159;\n\n// Scientific notation\nlet large_num: f64 = 1.23e6; // 1,230,000\n\nprintln!(\"Pi (f64): {}\", pi);\nprintln!(\"Large number: {}\", large_num);\n```\n\n• Use f64 for most calculations (more precise)\n• Use f32 when memory/performance is critical".to_string(),

            2 => "📋 **TASK 3/5: Boolean Values and Logic**\n\nMaster boolean logic with true/false and logical operators:\n\n```rust\n// Basic boolean values\nlet is_rust_awesome: bool = true;\nlet is_difficult: bool = false;\n\n// Boolean operations\nlet both_true = is_rust_awesome && is_difficult; // AND\nlet either_true = is_rust_awesome || is_difficult; // OR\nlet not_difficult = !is_difficult; // NOT\n\nprintln!(\"Both true: {}\", both_true);\nprintln!(\"Either true: {}\", either_true);\n\n// Comparison operations\nlet x = 10;\nlet y = 20;\nlet is_greater = x > y;\nprintln!(\"{} > {}: {}\", x, y, is_greater);\n```\n\n• `&&` = AND, `||` = OR, `!` = NOT\n• Comparisons return booleans".to_string(),

            3 => "📋 **TASK 4/5: Character Type and Unicode**\n\nWork with single characters including Unicode and emoji:\n\n```rust\n// Basic ASCII characters\nlet letter: char = 'A';\nlet digit: char = '7';\nlet symbol: char = '$';\n\n// Unicode characters\nlet heart: char = '♥';\nlet lambda: char = 'λ';\n\n// Emoji (also Unicode!)\nlet crab: char = '🦀';  // Rust's mascot\nlet robot: char = '🤖';\n\nprintln!(\"Letter: {}\", letter);\nprintln!(\"Heart: {}\", heart);\nprintln!(\"Crab (Rust): {}\", crab);\n\n// Characters are 4 bytes (full Unicode support)\nprintln!(\"Size of char: {} bytes\", std::mem::size_of::<char>());\n```\n\n• Use single quotes for `char`\n• Full Unicode support including emoji!\n• Each char is exactly 4 bytes".to_string(),

            4 => "📋 **TASK 5/5: Type Inference and Annotations**\n\nUnderstand how Rust figures out types automatically vs explicit annotations:\n\n```rust\n// Type inference - Rust figures out the types\nlet inferred_int = 42;        // i32 by default\nlet inferred_float = 3.14;    // f64 by default\nlet inferred_bool = true;     // bool\nlet inferred_char = 'R';      // char\n\n// Explicit type annotations\nlet explicit_u64: u64 = 1000;\nlet explicit_f32: f32 = 2.5;\nlet explicit_i8: i8 = -128;\n\n// Suffix notation (alternative)\nlet suffix_u32 = 100u32;\nlet suffix_f32 = 3.14f32;\n\nprintln!(\"Inferred integer: {}\", inferred_int);\nprintln!(\"Explicit u64: {}\", explicit_u64);\n```\n\n• Rust infers types when possible\n• Use annotations when ambiguous\n• Suffix notation: `42u32`, `3.14f32`".to_string(),

            _ => "🎉 **Level 3 Complete!**\n\nCongratulations! You've mastered Rust's fundamental data types:\n• Integer types (i32, u32, i64, u8) for whole numbers\n• Floating point types (f64, f32) for decimals\n• Boolean type (bool) for true/false logic\n• Character type (char) for Unicode text\n• Type inference vs explicit annotations\n\nYou now understand Rust's type system - the foundation for memory safety and performance!\n\n🚀 Ready for Level 4: Variable Bindings and Mutability!".to_string(),
        }
    }

    pub fn check_level_3_progress(&mut self) {
        if self.level_idx != 2 || self.tutorial_state.current_task >= 5 {
            return; // Only for level 3 and if not completed
        }

        match self.tutorial_state.current_task {
            0 => {
                // Task 1: Integer types - look for integer variable declarations
                if self.println_outputs.iter().any(|output|
                    output.contains("Signed i32:") ||
                    output.contains("signed") ||
                    output.contains("unsigned") ||
                    output.contains("-42") ||
                    output.contains("255")
                ) {
                    self.tutorial_state.task_completed[0] = true;
                    self.tutorial_state.current_task = 1;
                    println!("✅ Task 1 completed: Integer types!");
                }
            },
            1 => {
                // Task 2: Floating point numbers
                if self.println_outputs.iter().any(|output|
                    output.contains("Pi") ||
                    output.contains("3.141") ||
                    output.contains("f64") ||
                    output.contains("1.23e6") ||
                    output.contains("large_num")
                ) {
                    self.tutorial_state.task_completed[1] = true;
                    self.tutorial_state.current_task = 2;
                    println!("✅ Task 2 completed: Floating point numbers!");
                }
            },
            2 => {
                // Task 3: Boolean values and logic
                if self.println_outputs.iter().any(|output|
                    output.contains("Both true") ||
                    output.contains("Either true") ||
                    output.contains("true") ||
                    output.contains("false") ||
                    output.contains("&&") ||
                    output.contains("||")
                ) {
                    self.tutorial_state.task_completed[2] = true;
                    self.tutorial_state.current_task = 3;
                    println!("✅ Task 3 completed: Boolean logic!");
                }
            },
            3 => {
                // Task 4: Character type and Unicode
                if self.println_outputs.iter().any(|output|
                    output.contains("Heart") ||
                    output.contains("Crab") ||
                    output.contains("♥") ||
                    output.contains("🦀") ||
                    output.contains("char") ||
                    output.contains("Size of char")
                ) {
                    self.tutorial_state.task_completed[3] = true;
                    self.tutorial_state.current_task = 4;
                    println!("✅ Task 4 completed: Character types and Unicode!");
                }
            },
            4 => {
                // Task 5: Type inference and annotations
                if self.println_outputs.iter().any(|output|
                    output.contains("Inferred integer") ||
                    output.contains("Explicit u64") ||
                    output.contains("inferred") ||
                    output.contains("explicit") ||
                    output.contains("u64") ||
                    output.contains("suffix")
                ) {
                    self.tutorial_state.task_completed[4] = true;
                    self.tutorial_state.current_task = 5;
                    println!("✅ Task 5 completed: Type inference and annotations!");
                }
            },
            _ => {}
        }
    }
}