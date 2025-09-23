// Level 6 Task 1 Test: If/Else Conditionals and Expressions
// Tests if the user code properly uses if/else statements and expressions

#[cfg(test)]
mod level6_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_if_statement() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("if "),
            "❌ Your code should contain an if statement"
        );
    }

    #[test]
    fn test_has_else_statement() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("else"),
            "❌ Your code should contain an else clause"
        );
    }

    #[test]
    fn test_energy_variable() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("energy") && analyzer.code.contains("let "),
            "❌ Your code should declare an energy variable"
        );
    }

    #[test]
    fn test_energy_comparison() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_comparison = analyzer.code.contains("energy > 50") ||
                           analyzer.code.contains("energy > 80") ||
                           analyzer.code.contains("energy < ");

        assert!(
            has_comparison,
            "❌ Your code should compare energy levels (e.g., energy > 50)"
        );
    }

    #[test]
    fn test_if_expression_assignment() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Look for if expression pattern (assignment without semicolon)
        let has_if_expression = analyzer.code.contains("let status = if") ||
                              analyzer.code.contains("= if ");

        assert!(
            has_if_expression,
            "❌ Your code should use an if expression to assign a value to a variable"
        );
    }

    #[test]
    fn test_multiple_else_if() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let else_if_count = analyzer.code.matches("else if").count();

        assert!(
            else_if_count >= 2,
            "❌ Your code should have multiple 'else if' clauses for different energy levels"
        );
    }

    #[test]
    fn test_logical_operators() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_logical = analyzer.code.contains(" && ") || analyzer.code.contains(" || ");

        assert!(
            has_logical,
            "❌ Your code should use logical operators (&& or ||) in conditions"
        );
    }

    #[test]
    fn test_position_tuple() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_position = analyzer.code.contains("position") &&
                          (analyzer.code.contains("(") || analyzer.code.contains("tuple"));

        assert!(
            has_position,
            "❌ Your code should work with position as a tuple or similar structure"
        );
    }

    #[test]
    fn test_nested_if_statements() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Count if statements to detect nesting
        let if_count = analyzer.code.matches("if ").count();

        assert!(
            if_count >= 3,
            "❌ Your code should include nested if statements"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output energy status information
        let has_energy_output = result.stdout.contains("energy") ||
                               result.stdout.contains("status") ||
                               result.stdout.contains("Robot");

        assert!(
            has_energy_output,
            "❌ Your program should output information about robot energy or status"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let energy = 75;
    let position = (5, 3);

    // Basic if/else statements
    if energy > 50 {
        println!("Robot has sufficient energy: {}", energy);
    } else {
        println!("Robot needs recharging: {}", energy);
    }

    // If/else expressions (return values)
    let status = if energy > 80 {
        "Excellent"
    } else if energy > 50 {
        "Good"
    } else if energy > 20 {
        "Low"
    } else {
        "Critical"
    };

    println!("Energy status: {}", status);

    // Complex conditions with logical operators
    let x = position.0;
    let y = position.1;

    if x > 0 && y > 0 {
        println!("Robot is in positive quadrant: ({}, {})", x, y);
    } else if x == 0 || y == 0 {
        println!("Robot is on an axis: ({}, {})", x, y);
    } else {
        println!("Robot position: ({}, {})", x, y);
    }

    // Nested if statements
    if energy > 30 {
        if x < 10 {
            println!("Can move to x={}", x + 1);
        } else {
            println!("At edge, cannot move further right");
        }
    }
}