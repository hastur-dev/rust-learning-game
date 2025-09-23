// Level 1 Task 5 Test: Data Types and Movement
// Tests if the user code uses u32 integer type for movement loops

#[cfg(test)]
mod level1_task5_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_code_contains_u32_type() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("u32"),
            "❌ Your code should contain the u32 data type"
        );
    }

    #[test]
    fn test_code_contains_for_loop() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("for "),
            "❌ Your code should contain a for loop"
        );
    }

    #[test]
    fn test_steps_variable_with_type() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("steps: u32") || analyzer.code.contains("steps:u32"),
            "❌ Your code should declare a variable 'steps' with type u32"
        );
    }

    #[test]
    fn test_loop_range_with_steps() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("0..steps") || analyzer.code.contains("0..3"),
            "❌ Your for loop should use a range like '0..steps'"
        );
    }

    #[test]
    fn test_move_bot_function_call() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("move_bot("),
            "❌ Your code should call the move_bot function"
        );
    }

    #[test]
    fn test_move_bot_with_direction() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_direction = analyzer.code.contains("move_bot(\"right\")") ||
                          analyzer.code.contains("move_bot(\"left\")") ||
                          analyzer.code.contains("move_bot(\"up\")") ||
                          analyzer.code.contains("move_bot(\"down\")");

        assert!(
            has_direction,
            "❌ Your move_bot function should take a direction parameter like move_bot(\"right\")"
        );
    }

    #[test]
    fn test_proper_loop_structure() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Check for proper loop variable (often _i or similar)
        let has_loop_var = analyzer.code.contains("for _i in") ||
                          analyzer.code.contains("for i in") ||
                          analyzer.code.contains("for _ in");

        assert!(
            has_loop_var,
            "❌ Your for loop should have a proper structure: for _i in 0..steps"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");
    }

    #[test]
    fn test_complete_pattern() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Check for the complete expected pattern
        let has_pattern = analyzer.code.contains("let steps: u32 = 3;") &&
                         analyzer.code.contains("for _i in 0..steps") &&
                         analyzer.code.contains("move_bot(");

        assert!(
            has_pattern,
            "❌ Your code should follow the pattern:\n\
            let steps: u32 = 3;\n\
            for _i in 0..steps {\n\
                move_bot(\"right\");\n\
            }"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let steps: u32 = 3;
    for _i in 0..steps {
        move_bot("right");
    }
}

// Mock move_bot function for testing (would be provided by game)
fn move_bot(direction: &str) {
    println!("Moving {}", direction);
}