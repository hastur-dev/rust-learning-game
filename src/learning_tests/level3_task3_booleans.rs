// Level 3 Task 3 Test: Boolean Logic
// Tests that user understands boolean values and logical operations

#[cfg(test)]
mod level3_task3_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_boolean_variables() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_bool = analyzer.code.contains("bool") ||
                      analyzer.code.contains("true") ||
                      analyzer.code.contains("false");
        assert!(
            has_bool,
            "❌ You should declare boolean variables with true/false values"
        );
    }

    #[test]
    fn test_has_boolean_type_annotation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_bool_type = analyzer.code.contains(": bool");
        assert!(
            has_bool_type,
            "❌ You should use explicit boolean type annotation (: bool)"
        );
    }

    #[test]
    fn test_has_logical_and_operator() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("&&"),
            "❌ You should use the logical AND operator (&&)"
        );
    }

    #[test]
    fn test_has_logical_or_operator() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("||"),
            "❌ You should use the logical OR operator (||)"
        );
    }

    #[test]
    fn test_has_logical_not_operator() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("!"),
            "❌ You should use the logical NOT operator (!)"
        );
    }

    #[test]
    fn test_has_comparison_operations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_comparison = analyzer.code.contains("==") ||
                           analyzer.code.contains("!=") ||
                           analyzer.code.contains(">") ||
                           analyzer.code.contains("<");
        assert!(
            has_comparison,
            "❌ You should demonstrate comparison operations (==, >, <, etc.)"
        );
    }

    #[test]
    fn test_prints_boolean_values() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ Your code should print boolean values using println!"
        );
    }

    #[test]
    fn test_demonstrates_boolean_logic() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let logical_ops_count = analyzer.code.matches("&&").count() +
                               analyzer.code.matches("||").count() +
                               analyzer.code.matches("!").count();
        assert!(
            logical_ops_count >= 3,
            "❌ You should demonstrate multiple logical operations (AND, OR, NOT)"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_boolean_output = result.stdout.contains("true") ||
                               result.stdout.contains("false");
        assert!(
            has_boolean_output,
            "❌ Your program should output boolean values (true/false)"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let is_rust_awesome: bool = true;
    let is_difficult: bool = false;

    let both_true = is_rust_awesome && is_difficult; // AND
    let either_true = is_rust_awesome || is_difficult; // OR
    let not_difficult = !is_difficult; // NOT

    println!("Rust is awesome: {}", is_rust_awesome);
    println!("Rust is difficult: {}", is_difficult);
    println!("Both true: {}", both_true);
    println!("Either true: {}", either_true);
    println!("Not difficult: {}", not_difficult);

    let x = 10;
    let y = 20;
    let is_greater = x > y;
    let is_equal = x == y;

    println!("{} > {}: {}", x, y, is_greater);
    println!("{} == {}: {}", x, y, is_equal);
}