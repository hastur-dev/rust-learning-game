// Level 1 Task 4 Test: Mutable Variables and Scan Function
// Tests if the user code uses mutable variables with the scan function

#[cfg(test)]
mod level1_task4_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_code_contains_mut_keyword() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("let mut "),
            "❌ Your code should contain a mutable variable declaration with 'let mut'"
        );
    }

    #[test]
    fn test_code_contains_scan_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("scan("),
            "❌ Your code should call the scan function"
        );
    }

    #[test]
    fn test_scan_result_variable() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("scan_result") || analyzer.code.contains("result"),
            "❌ Your code should store the scan result in a variable (like 'scan_result')"
        );
    }

    #[test]
    fn test_println_with_scan_result() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ Your code should contain a println! statement to display the scan result"
        );

        // Check for pattern that shows scan result
        let has_scan_output = analyzer.code.contains("Scan found:") ||
                             analyzer.code.contains("scan_result") ||
                             analyzer.code.contains("scan(");

        assert!(
            has_scan_output,
            "❌ Your println! should display the scan result (hint: 'Scan found: {}')"
        );
    }

    #[test]
    fn test_proper_scan_usage() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Look for proper scan function call pattern
        let has_scan_direction = analyzer.code.contains("scan(\"") ||
                                analyzer.code.contains("scan(\'");

        assert!(
            has_scan_direction,
            "❌ Your scan function should take a direction parameter like scan(\"right\")"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should contain evidence of scan result output
        assert!(
            result.stdout.contains("Scan found:") || result.stdout.contains("result"),
            "❌ Your program should output scan results when run"
        );
    }

    #[test]
    fn test_mutable_pattern() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Check for proper mutable scan pattern
        let has_proper_pattern = analyzer.code.contains("let mut scan_result = scan(") ||
                                analyzer.code.contains("let mut result = scan(");

        assert!(
            has_proper_pattern,
            "❌ Your code should follow the pattern: let mut scan_result = scan(\"direction\");"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let mut scan_result = scan("right");
    println!("Scan found: {}", scan_result);
}

// Mock scan function for testing (would be provided by game)
fn scan(direction: &str) -> String {
    format!("scanned_{}", direction)
}