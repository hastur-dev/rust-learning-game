// Level 5 Task 2 Test: Safe Conversions with From and Into
// Tests that user understands From/Into traits for safe conversions

#[cfg(test)]
mod level5_task2_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_uses_into_for_safe_conversion() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_into = analyzer.code.contains(".into()");
        assert!(
            has_into,
            "❌ You should use .into() for safe type conversions"
        );
    }

    #[test]
    fn test_converts_smaller_to_larger_integers() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_safe_int_conv = (analyzer.code.contains("i32") && analyzer.code.contains("i64")) &&
                              analyzer.code.contains(".into()");
        assert!(
            has_safe_int_conv,
            "❌ You should convert smaller integers to larger ones (i32 to i64)"
        );
    }

    #[test]
    fn test_demonstrates_string_conversions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_string_conv = analyzer.code.contains(".to_string()") ||
                            analyzer.code.contains("String::from") ||
                            analyzer.code.contains("format!");
        assert!(
            has_string_conv,
            "❌ You should demonstrate string conversions (to_string, String::from)"
        );
    }

    #[test]
    fn test_converts_char_to_string() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_char_conv = analyzer.code.contains("char") &&
                          (analyzer.code.contains(".into()") || analyzer.code.contains("String"));
        assert!(
            has_char_conv,
            "❌ You should convert character to string"
        );
    }

    #[test]
    fn test_converts_array_to_vec() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_array_vec = analyzer.code.contains("[") && analyzer.code.contains("]") &&
                          analyzer.code.contains("Vec") && analyzer.code.contains(".into()");
        assert!(
            has_array_vec,
            "❌ You should convert array to Vec using into()"
        );
    }

    #[test]
    fn test_uses_from_explicitly() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_from = analyzer.code.contains("String::from") ||
                      analyzer.code.contains("::from(");
        assert!(
            has_from,
            "❌ You should use From trait explicitly (String::from, i64::from)"
        );
    }

    #[test]
    fn test_demonstrates_chained_conversions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_chain = analyzer.code.contains("original") && analyzer.code.contains("converted") &&
                       analyzer.code.contains("back_to_string");
        assert!(
            has_chain,
            "❌ You should demonstrate chained conversions (original -> converted -> string)"
        );
    }

    #[test]
    fn test_shows_both_into_and_from() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_both = analyzer.code.contains(".into()") &&
                      (analyzer.code.contains("::from") || analyzer.code.contains("From::from"));
        assert!(
            has_both,
            "❌ You should demonstrate both .into() and From::from() methods"
        );
    }

    #[test]
    fn test_prints_conversion_results() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let println_count = analyzer.code.matches("println!").count();
        assert!(
            println_count >= 6,
            "❌ You should print the results of various conversions"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let output_lines: Vec<&str> = result.stdout.lines().collect();
        let shows_conversions = output_lines.iter().any(|line| line.contains("Small") && line.contains("i32")) &&
                              output_lines.iter().any(|line| line.contains("Large") && line.contains("i64")) &&
                              output_lines.iter().any(|line| line.contains("string") || line.contains("String"));
        assert!(
            shows_conversions,
            "❌ Your program should show safe conversions between different types"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let small: i32 = 100;
    let large: i64 = small.into();

    println!("Small (i32): {}", small);
    println!("Large (i64): {}", large);

    let number: i32 = 42;
    let number_string: String = number.to_string();
    let formatted: String = format!("Number: {}", number);

    println!("Original number: {}", number);
    println!("As string: {}", number_string);
    println!("Formatted: {}", formatted);

    let ch: char = 'R';
    let ch_string: String = ch.into();

    println!("Character: {}", ch);
    println!("As string: {}", ch_string);

    let array: [i32; 3] = [1, 2, 3];
    let vector: Vec<i32> = array.into();

    println!("Array: {:?}", array);
    println!("Vector: {:?}", vector);

    let from_example: String = String::from("Hello, Rust!");
    let into_example: String = "Hello, Into!".into();

    println!("From example: {}", from_example);
    println!("Into example: {}", into_example);

    let original: u16 = 500;
    let converted: u64 = original.into();
    let back_to_string = converted.to_string();

    println!("Chain: {} -> {} -> {}", original, converted, back_to_string);
}