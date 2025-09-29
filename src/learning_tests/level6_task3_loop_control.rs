// Level 6 Task 3 Test: Loop Control - break, continue, and labels
// Tests if the user code uses advanced loop control mechanisms

#[cfg(test)]
mod level6_task3_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_continue_statement() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("continue"),
            "❌ Your code should use 'continue' to skip loop iterations"
        );
    }

    #[test]
    fn test_has_break_statement() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("break"),
            "❌ Your code should use 'break' to exit loops early"
        );
    }

    #[test]
    fn test_has_labeled_loop() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_label = analyzer.code.contains("'") &&
                       (analyzer.code.contains(": for") || analyzer.code.contains(": loop"));
        assert!(
            has_label,
            "❌ Your code should use labeled loops (e.g., 'outer: for)"
        );
    }

    #[test]
    fn test_has_labeled_break() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_labeled_break = analyzer.code.contains("break '") ||
                               analyzer.code.contains("break'");
        assert!(
            has_labeled_break,
            "❌ Your code should use labeled breaks (e.g., break 'outer)"
        );
    }

    #[test]
    fn test_has_nested_loops() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let loop_count = analyzer.code.matches("for ").count() +
                        analyzer.code.matches("while ").count() +
                        analyzer.code.matches("loop {").count();
        assert!(
            loop_count >= 2,
            "❌ Your code should have nested loops to demonstrate labeled breaks"
        );
    }

    #[test]
    fn test_has_conditional_logic() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("if "),
            "❌ Your code should use conditionals to control loop flow"
        );
    }

    #[test]
    fn test_demonstrates_prime_or_search() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_pattern = analyzer.code.contains("prime") ||
                         analyzer.code.contains("search") ||
                         analyzer.code.contains("found") ||
                         analyzer.code.contains("target");
        assert!(
            has_pattern,
            "❌ Your code should demonstrate a search pattern or prime finding algorithm"
        );
    }

    #[test]
    fn test_has_loop_returning_value() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_return_value = analyzer.code.contains("break ") &&
                              !analyzer.code.contains("break;") &&
                              analyzer.code.contains("=");
        assert!(
            has_return_value,
            "❌ Your code should demonstrate loops returning values with 'break value'"
        );
    }

    #[test]
    fn test_has_modulo_operator() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(" % "),
            "❌ Your code should use the modulo operator (%) for number patterns or divisibility"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output evidence of search or prime finding
        let has_control_output = result.stdout.contains("prime") ||
                               result.stdout.contains("found") ||
                               result.stdout.contains("target") ||
                               result.stdout.contains("search") ||
                               result.stdout.contains("break");

        assert!(
            has_control_output,
            "❌ Your program should output information about loop control operations"
        );
    }

    #[test]
    fn test_complex_control_flow() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_complex_flow = analyzer.code.contains("continue") &&
                              analyzer.code.contains("break") &&
                              analyzer.code.contains("'");
        assert!(
            has_complex_flow,
            "❌ Your code should demonstrate complex control flow with continue, break, and labels"
        );
    }
}

// Reference implementation for comparison
fn main() {
    println!("=== Finding prime numbers with continue ===");
    for num in 2..20 {
        let mut is_prime = true;

        for i in 2..num {
            if num % i == 0 {
                is_prime = false;
                break;  // No need to check further
            }
        }

        if !is_prime {
            continue;  // Skip non-prime numbers
        }

        println!("{} is prime", num);
    }

    println!("\n=== Grid search with labeled breaks ===");
    let target = (2, 3);
    let mut found = false;

    'search: for row in 0..5 {
        for col in 0..5 {
            println!("Checking ({}, {})", row, col);

            if (row, col) == target {
                println!("Found target at ({}, {})!", row, col);
                found = true;
                break 'search;  // Break out of both loops
            }

            // Skip certain positions
            if row == col {
                println!("Skipping diagonal position ({}, {})", row, col);
                continue;
            }
        }
    }

    if !found {
        println!("Target not found");
    }

    println!("\n=== Loop returning values ===");
    let result = loop {
        let mut input = 0;
        for i in 1..10 {
            input += i;
            if input > 20 {
                break input;  // Return value from loop
            }
        }
        break input;
    };

    println!("Loop returned value: {}", result);
}