// Level 1 Task 1 Test: Hello World with println!
// Tests if the user code outputs "Hello, Rust!" exactly

#[cfg(test)]
mod level1_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_code_contains_println() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(), 
            "❌ Your code should contain a println! statement"
        );
    }

    #[test]
    fn test_println_says_hello_rust() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println_with_text("Hello, Rust!"),
            "❌ Your println! statement should output exactly: 'Hello, Rust!'"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");
        
        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");
        assert_eq!(
            result.stdout.trim(), 
            "Hello, Rust!",
            "❌ Your program should output exactly 'Hello, Rust!'"
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
}

// Reference implementation for comparison
fn main() {
    println!("Hello, Rust!");
}