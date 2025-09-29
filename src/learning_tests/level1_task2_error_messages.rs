// Level 1 Task 2 Test: Error Messages with eprintln!
// Tests if the user code outputs error messages using eprintln! macro

#[cfg(test)]
mod level1_task2_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_code_contains_eprintln() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_eprintln(),
            "❌ Your code should contain an eprintln! statement"
        );
    }

    #[test]
    fn test_eprintln_says_error_message() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_eprintln_with_text("This is an error message!"),
            "❌ Your eprintln! statement should output exactly: 'This is an error message!'"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");
        assert!(
            result.stderr.contains("This is an error message!"),
            "❌ Your program should output 'This is an error message!' to stderr"
        );
    }

    #[test]
    fn test_has_main_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_function("main"),
            "❌ Your code should have a main() function"
        );
    }

    #[test]
    fn test_error_vs_normal_output() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        // Should use stderr for error messages, not stdout
        assert!(
            !result.stdout.contains("This is an error message!"),
            "❌ Error messages should go to stderr (eprintln!), not stdout (println!)"
        );
    }
}

// Reference implementation for comparison
fn main() {
    eprintln!("This is an error message!");
}