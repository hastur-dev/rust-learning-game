// Level 3 Task 5 Test: Type Inference and Annotations
// Tests that user understands type inference vs explicit annotations

#[cfg(test)]
mod level3_task5_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_inferred_variables() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        // Look for variables without explicit type annotations
        let has_inference = analyzer.code.contains("let inferred_int = ") ||
                          analyzer.code.contains("let inferred_float = ") ||
                          analyzer.code.contains("let inferred_bool = ") ||
                          analyzer.code.contains("let inferred_char = ");
        assert!(
            has_inference,
            "❌ You should demonstrate type inference with variables like 'let inferred_int = 42;'"
        );
    }

    #[test]
    fn test_has_explicit_annotations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_explicit = analyzer.code.contains(": u64") ||
                         analyzer.code.contains(": f32") ||
                         analyzer.code.contains(": i8");
        assert!(
            has_explicit,
            "❌ You should demonstrate explicit type annotations like 'let explicit: u64 = 1000;'"
        );
    }

    #[test]
    fn test_demonstrates_default_inference() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_default_demo = analyzer.code.contains("42") &&
                             analyzer.code.contains("3.14") &&
                             !analyzer.code.contains("42i8") &&
                             !analyzer.code.contains("3.14f32");
        assert!(
            has_default_demo,
            "❌ You should show default type inference (42 -> i32, 3.14 -> f64)"
        );
    }

    #[test]
    fn test_has_parse_with_annotation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_parse = analyzer.code.contains(".parse()") &&
                       (analyzer.code.contains(": i32") || analyzer.code.contains("<i32>"));
        assert!(
            has_parse,
            "❌ You should demonstrate parsing with type annotation (parse needs explicit types)"
        );
    }

    #[test]
    fn test_has_suffix_notation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_suffix = analyzer.code.contains("u32") ||
                        analyzer.code.contains("f32") ||
                        analyzer.code.contains("100u32") ||
                        analyzer.code.contains("3.14f32");
        assert!(
            has_suffix,
            "❌ You should demonstrate suffix notation (100u32, 3.14f32)"
        );
    }

    #[test]
    fn test_prints_type_information() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ Your code should print information about the types and values"
        );
    }

    #[test]
    fn test_demonstrates_inference_vs_explicit() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let inferred_count = analyzer.code.lines()
            .filter(|line| line.contains("let ") && !line.contains(": "))
            .count();
        let explicit_count = analyzer.code.lines()
            .filter(|line| line.contains("let ") && line.contains(": "))
            .count();

        assert!(
            inferred_count >= 2 && explicit_count >= 2,
            "❌ You should demonstrate both type inference and explicit annotations"
        );
    }

    #[test]
    fn test_shows_ambiguous_case() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_ambiguous = analyzer.code.contains(".parse()") ||
                          analyzer.code.contains("expect") ||
                          analyzer.code.contains("unwrap");
        assert!(
            has_ambiguous,
            "❌ You should show cases where type annotation is needed (like parsing)"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Check for various types in output
        let has_numeric_output = result.stdout.chars().any(|c| c.is_numeric());
        let has_type_info = result.stdout.contains("type") ||
                           result.stdout.contains("i32") ||
                           result.stdout.contains("f64");
        assert!(
            has_numeric_output,
            "❌ Your program should output values from different types"
        );
    }
}

// Reference implementation for comparison
fn main() {
    // Type inference - Rust figures out the types
    let inferred_int = 42;        // i32 by default
    let inferred_float = 3.14;    // f64 by default
    let inferred_bool = true;     // bool
    let inferred_char = 'R';      // char

    println!("Inferred integer: {} (type: i32)", inferred_int);
    println!("Inferred float: {} (type: f64)", inferred_float);
    println!("Inferred bool: {} (type: bool)", inferred_bool);
    println!("Inferred char: {} (type: char)", inferred_char);

    // Explicit type annotations
    let explicit_u64: u64 = 1000;
    let explicit_f32: f32 = 2.5;
    let explicit_i8: i8 = -128;

    println!("Explicit u64: {}", explicit_u64);
    println!("Explicit f32: {}", explicit_f32);
    println!("Explicit i8: {}", explicit_i8);

    // Type annotations needed for ambiguous cases
    let parsed_number: i32 = "42".parse().expect("Failed to parse");
    println!("Parsed number: {}", parsed_number);

    // Suffix notation (alternative to annotations)
    let suffix_u32 = 100u32;
    let suffix_f32 = 3.14f32;
    println!("Suffix u32: {}", suffix_u32);
    println!("Suffix f32: {}", suffix_f32);
}