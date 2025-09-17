// Level 3 Task 4 Test: Character Type and Unicode
// Tests that user understands char type and Unicode support

#[cfg(test)]
mod level3_task4_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_char_type_annotation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_char_type = analyzer.code.contains(": char");
        assert!(
            has_char_type,
            "❌ You should use explicit char type annotation (: char)"
        );
    }

    #[test]
    fn test_has_single_quotes() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let single_quote_count = analyzer.code.matches('\'').count();
        assert!(
            single_quote_count >= 4, // At least 2 character literals
            "❌ You should use single quotes for character literals ('A', '7', etc.)"
        );
    }

    #[test]
    fn test_has_ascii_characters() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_ascii = analyzer.code.contains("'A'") ||
                       analyzer.code.contains("'7'") ||
                       analyzer.code.contains("'$'") ||
                       (analyzer.code.contains('\'') && analyzer.code.chars().any(|c| c.is_ascii_alphanumeric()));
        assert!(
            has_ascii,
            "❌ You should demonstrate basic ASCII characters (letters, digits, symbols)"
        );
    }

    #[test]
    fn test_has_unicode_characters() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_unicode = analyzer.code.contains("'♥'") ||
                         analyzer.code.contains("'λ'") ||
                         analyzer.code.chars().any(|c| !c.is_ascii() && c != ' ' && c != '\n' && c != '\r' && c != '\t');
        assert!(
            has_unicode,
            "❌ You should demonstrate Unicode characters (♥, λ, etc.)"
        );
    }

    #[test]
    fn test_has_emoji_characters() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_emoji = analyzer.code.contains("🦀") ||
                       analyzer.code.contains("🤖") ||
                       analyzer.code.chars().any(|c| c as u32 > 0x1F000);
        assert!(
            has_emoji,
            "❌ You should demonstrate emoji characters (🦀, 🤖, etc.)"
        );
    }

    #[test]
    fn test_prints_character_values() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ Your code should print character values using println!"
        );
    }

    #[test]
    fn test_demonstrates_char_size() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_size_demo = analyzer.code.contains("size_of") ||
                           analyzer.code.contains("4 bytes") ||
                           analyzer.code.contains("mem::");
        assert!(
            has_size_demo,
            "❌ You should demonstrate that char is 4 bytes (using std::mem::size_of)"
        );
    }

    #[test]
    fn test_multiple_character_types() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let char_count = analyzer.code.matches(": char").count();
        assert!(
            char_count >= 3,
            "❌ You should declare multiple character variables to show variety"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Check for character output (letters, symbols, unicode)
        let has_char_output = result.stdout.chars().any(|c| c.is_alphabetic()) &&
                            result.stdout.chars().any(|c| !c.is_ascii_alphanumeric() && !c.is_whitespace());
        assert!(
            has_char_output,
            "❌ Your program should output various character types"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let letter: char = 'A';
    let digit: char = '7';
    let symbol: char = '$';

    let heart: char = '♥';
    let lambda: char = 'λ';

    let crab: char = '🦀';  // Rust's mascot
    let robot: char = '🤖';

    println!("Letter: {}", letter);
    println!("Digit: {}", digit);
    println!("Symbol: {}", symbol);
    println!("Heart: {}", heart);
    println!("Lambda: {}", lambda);
    println!("Crab (Rust): {}", crab);
    println!("Robot: {}", robot);

    println!("Size of char: {} bytes", std::mem::size_of::<char>());
}