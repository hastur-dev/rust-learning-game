// Level 3 Task 1 Test: Work with Integer Types
// Tests that user understands different integer types and their usage

#[cfg(test)]
mod level3_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_signed_integers() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_signed = analyzer.code.contains("i32") ||
                        analyzer.code.contains("i64") ||
                        analyzer.code.contains(": i");
        assert!(
            has_signed,
            "❌ You should declare signed integer variables (i32, i64, etc.)"
        );
    }

    #[test]
    fn test_has_unsigned_integers() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_unsigned = analyzer.code.contains("u32") ||
                          analyzer.code.contains("u64") ||
                          analyzer.code.contains("u8") ||
                          analyzer.code.contains(": u");
        assert!(
            has_unsigned,
            "❌ You should declare unsigned integer variables (u32, u64, u8, etc.)"
        );
    }

    #[test]
    fn test_has_negative_number() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("-") && analyzer.code.chars().any(|c| c.is_numeric()),
            "❌ You should use a negative number to demonstrate signed integers"
        );
    }

    #[test]
    fn test_prints_integer_values() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ Your code should print the integer values using println!"
        );
    }

    #[test]
    fn test_has_type_annotations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_annotations = analyzer.code.contains(": i32") ||
                             analyzer.code.contains(": i64") ||
                             analyzer.code.contains(": u32") ||
                             analyzer.code.contains(": u64") ||
                             analyzer.code.contains(": u8");
        assert!(
            has_annotations,
            "❌ You should use explicit type annotations like 'let x: i32 = 42;'"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_numeric_output = result.stdout.chars().any(|c| c.is_numeric());
        assert!(
            has_numeric_output,
            "❌ Your program should output some integer values"
        );
    }

    #[test]
    fn test_demonstrates_range_differences() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_multiple_sizes = (analyzer.code.contains("i32") && analyzer.code.contains("i64")) ||
                                (analyzer.code.contains("u8") && analyzer.code.contains("u32")) ||
                                (analyzer.count_pattern(": i") + analyzer.count_pattern(": u") >= 2);
        assert!(
            has_multiple_sizes,
            "❌ You should demonstrate different integer sizes (e.g., i32 and i64, or u8 and u32)"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let signed: i32 = -42;
    let large_signed: i64 = -1_000_000;
    let unsigned: u32 = 255;
    let small_unsigned: u8 = 200;

    println!("Signed i32: {}", signed);
    println!("Large i64: {}", large_signed);
    println!("Unsigned u32: {}", unsigned);
    println!("Small u8: {}", small_unsigned);
}