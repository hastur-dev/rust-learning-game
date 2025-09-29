// Level 5 Task 1 Test: Explicit Type Casting with 'as'
// Tests that user understands explicit casting and potential data loss

#[cfg(test)]
mod level5_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_basic_integer_casting() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_int_cast = analyzer.code.contains("as i32") ||
                          analyzer.code.contains("as i64") ||
                          analyzer.code.contains(" as ");
        assert!(
            has_int_cast,
            "❌ You should demonstrate basic integer casting with 'as' keyword"
        );
    }

    #[test]
    fn test_demonstrates_precision_loss() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_precision_loss = analyzer.code.contains("f64") && analyzer.code.contains("f32") &&
                               analyzer.code.contains("as f32");
        assert!(
            has_precision_loss,
            "❌ You should demonstrate precision loss when casting f64 to f32"
        );
    }

    #[test]
    fn test_has_float_to_integer_cast() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_float_to_int = (analyzer.code.contains("f64") || analyzer.code.contains("f32")) &&
                             analyzer.code.contains("as i32");
        assert!(
            has_float_to_int,
            "❌ You should demonstrate casting float to integer (loses decimal part)"
        );
    }

    #[test]
    fn test_demonstrates_dangerous_casting() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_dangerous = analyzer.code.contains("as i8") ||
                          (analyzer.code.contains("1000") && analyzer.code.contains("as"));
        assert!(
            has_dangerous,
            "❌ You should demonstrate potentially dangerous casting (like i32 to i8)"
        );
    }

    #[test]
    fn test_shows_unsigned_to_signed() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_unsigned_signed = analyzer.code.contains("u32") && analyzer.code.contains("i32") &&
                                analyzer.code.contains("as i32");
        assert!(
            has_unsigned_signed,
            "❌ You should demonstrate unsigned to signed casting"
        );
    }

    #[test]
    fn test_explains_data_loss() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let explains_loss = analyzer.code.contains("wrapped") ||
                          analyzer.code.contains("overflow") ||
                          analyzer.code.contains("lost") ||
                          analyzer.code.contains("truncat");
        assert!(
            explains_loss,
            "❌ You should comment about data loss, wrapping, or truncation"
        );
    }

    #[test]
    fn test_prints_before_and_after_cast() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let println_count = analyzer.code.matches("println!").count();
        assert!(
            println_count >= 6,
            "❌ You should print values before and after each casting operation"
        );
    }

    #[test]
    fn test_demonstrates_multiple_cast_types() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let cast_count = analyzer.code.matches(" as ").count();
        assert!(
            cast_count >= 4,
            "❌ You should demonstrate multiple different casting operations"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let output_lines: Vec<&str> = result.stdout.lines().collect();
        let shows_casting = output_lines.iter().any(|line| line.contains("Large") || line.contains("i64")) &&
                          output_lines.iter().any(|line| line.contains("Small") || line.contains("i32")) &&
                          output_lines.iter().any(|line| line.contains("Pi"));
        assert!(
            shows_casting,
            "❌ Your program should show the results of various casting operations"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let large_number: i64 = 1000;
    let small_number: i32 = large_number as i32;

    println!("Large (i64): {}", large_number);
    println!("Small (i32): {}", small_number);

    let precise_float: f64 = 3.14159265359;
    let less_precise: f32 = precise_float as f32;

    println!("Precise (f64): {}", precise_float);
    println!("Less precise (f32): {}", less_precise);

    let pi: f64 = 3.14159;
    let pi_int: i32 = pi as i32;

    println!("Pi as float: {}", pi);
    println!("Pi as integer: {} (decimal part lost)", pi_int);

    let big_value: i32 = 1000;
    let small_type: i8 = big_value as i8;

    println!("Big value (i32): {}", big_value);
    println!("As i8: {} (wrapped around)", small_type);

    let unsigned: u32 = 4294967295;
    let signed: i32 = unsigned as i32;

    println!("Unsigned: {}", unsigned);
    println!("As signed: {} (overflow)", signed);
}