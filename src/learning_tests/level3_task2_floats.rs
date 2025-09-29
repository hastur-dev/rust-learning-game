// Level 3 Task 2 Test: Floating Point Numbers
// Tests that user understands f32, f64, and floating point arithmetic

#[cfg(test)]
mod level3_task2_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_f64_variable() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_f64 = analyzer.code.contains("f64") ||
                     analyzer.code.contains(": f64");
        assert!(
            has_f64,
            "❌ You should declare an f64 variable (like pi: f64)"
        );
    }

    #[test]
    fn test_has_f32_variable() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_f32 = analyzer.code.contains("f32") ||
                     analyzer.code.contains(": f32");
        assert!(
            has_f32,
            "❌ You should declare an f32 variable for single precision"
        );
    }

    #[test]
    fn test_has_decimal_literals() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_decimals = analyzer.code.contains("3.14") ||
                          analyzer.code.contains("2.71") ||
                          analyzer.code.chars().any(|c| c == '.' && analyzer.code.chars().any(|d| d.is_numeric()));
        assert!(
            has_decimals,
            "❌ You should use decimal number literals (like 3.14)"
        );
    }

    #[test]
    fn test_prints_float_values() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ Your code should print the floating point values using println!"
        );
    }

    #[test]
    fn test_demonstrates_precision_difference() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_both_types = analyzer.code.contains("f64") && analyzer.code.contains("f32");
        assert!(
            has_both_types,
            "❌ You should demonstrate both f64 and f32 to show precision differences"
        );
    }

    #[test]
    fn test_has_scientific_notation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_scientific = analyzer.code.contains("e") &&
                           (analyzer.code.contains("1.23e6") ||
                            analyzer.code.chars().filter(|&c| c == 'e').count() > 0);
        assert!(
            has_scientific,
            "❌ You should use scientific notation (like 1.23e6)"
        );
    }

    #[test]
    fn test_floating_point_arithmetic() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_arithmetic = analyzer.code.contains("+") ||
                           analyzer.code.contains("-") ||
                           analyzer.code.contains("*") ||
                           analyzer.code.contains("/");
        assert!(
            has_arithmetic,
            "❌ You should demonstrate floating point arithmetic operations"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Check for decimal numbers in output
        let has_decimal_output = result.stdout.contains('.') &&
                               result.stdout.chars().any(|c| c.is_numeric());
        assert!(
            has_decimal_output,
            "❌ Your program should output floating point numbers"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let pi: f64 = 3.141592653589793;
    let e = 2.71828; // Type inferred as f64
    let pi_f32: f32 = 3.14159;
    let large_num: f64 = 1.23e6; // 1,230,000

    println!("Pi (f64): {}", pi);
    println!("E (inferred): {}", e);
    println!("Pi (f32): {}", pi_f32);
    println!("Large number: {}", large_num);

    let sum = pi + e;
    println!("Pi + E = {}", sum);
}