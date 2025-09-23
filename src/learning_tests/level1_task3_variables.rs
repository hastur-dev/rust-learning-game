// Level 1 Task 3 Test: Variables in Print Statements
// Tests if the user code creates variables and uses them in print statements

#[cfg(test)]
mod level1_task3_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_code_contains_let_statement() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("let "),
            "❌ Your code should contain a variable declaration with 'let'"
        );
    }

    #[test]
    fn test_code_contains_println() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ Your code should contain a println! statement"
        );
    }

    #[test]
    fn test_variable_used_in_println() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Check for variable interpolation patterns
        let has_interpolation = analyzer.code.contains("println!(\"{}\", ") ||
                               analyzer.code.contains("println!(\"{:?}\", ") ||
                               analyzer.code.contains("println!(\"{} ") ||
                               analyzer.code.contains("println!(\"{}\"");

        assert!(
            has_interpolation,
            "❌ Your println! should use string interpolation with {} to display a variable"
        );
    }

    #[test]
    fn test_outputs_variables_are_powerful() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");
        assert!(
            result.stdout.contains("Variables are powerful!"),
            "❌ Your program should output 'Variables are powerful!' when run"
        );
    }

    #[test]
    fn test_variable_assignment_pattern() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Look for variable assignment with string
        let has_assignment = analyzer.code.contains("my_message") ||
                           analyzer.code.contains("message") ||
                           analyzer.code.contains("text") ||
                           analyzer.code.contains("\"Variables are powerful!\"");

        assert!(
            has_assignment,
            "❌ Your code should assign a string to a variable (like: let my_message = \"Variables are powerful!\";)"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");
    }
}

// Reference implementation for comparison
fn main() {
    let my_message = "Variables are powerful!";
    println!("{}", my_message);
}