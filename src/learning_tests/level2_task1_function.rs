//Level 2 Task 1 Test: Function with print statement
// Tests that user creates a function named scan_level with a print statement

#[cfg(test)]
mod level2_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_scan_level_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_function("scan_level"),
            "❌ You need to create a function named 'scan_level'"
        );
    }

    #[test]
    fn test_scan_level_contains_println() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        // Check if the scan_level function area contains println
        assert!(
            analyzer.contains_println(),
            "❌ Your scan_level function should contain a println! statement"
        );
    }

    #[test]
    fn test_scan_level_prints_beginning_message() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println_with_text("Beginning level scan..."),
            "❌ Your scan_level function should print exactly: 'Beginning level scan...'"
        );
    }

    #[test]
    fn test_main_calls_scan_level() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("scan_level();"),
            "❌ Your main() function should call scan_level();"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");
        
        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");
        
        // Check that output contains the expected message
        assert!(
            result.stdout.contains("Beginning level scan..."),
            "❌ Your program should output 'Beginning level scan...' when run"
        );
    }

    #[test]
    fn test_has_proper_function_structure() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Check that scan_level function is defined before main
        let scan_level_pos = analyzer.code.find("fn scan_level").unwrap_or(usize::MAX);
        let main_pos = analyzer.code.find("fn main").unwrap_or(0);
        
        assert!(
            scan_level_pos < main_pos,
            "❌ The scan_level function should be defined before the main function"
        );
    }
}

// Reference implementation for comparison
fn scan_level() {
    println!("Beginning level scan...");
}

fn main() {
    scan_level();
}