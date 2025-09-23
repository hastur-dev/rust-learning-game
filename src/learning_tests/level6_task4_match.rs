// Level 6 Task 4 Test: Match Expressions and Pattern Matching
// Tests if the user code uses match expressions for pattern matching

#[cfg(test)]
mod level6_task4_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_match_expression() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("match "),
            "❌ Your code should contain a match expression"
        );
    }

    #[test]
    fn test_has_match_arms() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let arrow_count = analyzer.code.matches(" => ").count();
        assert!(
            arrow_count >= 3,
            "❌ Your match expression should have at least 3 arms (patterns)"
        );
    }

    #[test]
    fn test_has_catch_all_pattern() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("_ =>"),
            "❌ Your match should include a catch-all pattern using underscore (_)"
        );
    }

    #[test]
    fn test_has_range_pattern() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_range = analyzer.code.contains("..=") || analyzer.code.contains("..");
        assert!(
            has_range,
            "❌ Your code should use range patterns in match (e.g., 1..=10)"
        );
    }

    #[test]
    fn test_has_tuple_matching() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_tuple_match = analyzer.code.contains("(") &&
                             analyzer.code.contains(")") &&
                             analyzer.code.contains("match");
        assert!(
            has_tuple_match,
            "❌ Your code should demonstrate tuple pattern matching"
        );
    }

    #[test]
    fn test_has_option_matching() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_option = analyzer.code.contains("Some(") || analyzer.code.contains("None");
        assert!(
            has_option,
            "❌ Your code should demonstrate Option enum matching with Some/None"
        );
    }

    #[test]
    fn test_has_guard_conditions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(" if ") && analyzer.code.contains("match"),
            "❌ Your code should use match guards (conditions after patterns with 'if')"
        );
    }

    #[test]
    fn test_has_destructuring() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_destructuring = analyzer.code.contains("(") &&
                               analyzer.code.contains(",") &&
                               analyzer.code.contains("match");
        assert!(
            has_destructuring,
            "❌ Your code should demonstrate destructuring in match patterns"
        );
    }

    #[test]
    fn test_match_as_expression() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_assignment = analyzer.code.contains("= match") ||
                            analyzer.code.contains("let") && analyzer.code.contains("match");
        assert!(
            has_assignment,
            "❌ Your code should use match as an expression (assigning its result to a variable)"
        );
    }

    #[test]
    fn test_has_string_matching() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_string_match = analyzer.code.contains("\"") && analyzer.code.contains("match");
        assert!(
            has_string_match,
            "❌ Your code should demonstrate string pattern matching"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output evidence of pattern matching
        let has_match_output = result.stdout.contains("mode") ||
                              result.stdout.contains("energy") ||
                              result.stdout.contains("position") ||
                              result.stdout.contains("found") ||
                              result.stdout.contains("robot");

        assert!(
            has_match_output,
            "❌ Your program should output results from pattern matching operations"
        );
    }
}

// Reference implementation for comparison
fn main() {
    println!("=== Basic match with integers ===");
    let robot_mode = 2;

    let mode_name = match robot_mode {
        1 => "Exploration",
        2 => "Collection",
        3 => "Return Home",
        _ => "Unknown Mode",  // Catch-all pattern
    };

    println!("Robot mode {}: {}", robot_mode, mode_name);

    println!("\n=== Match with ranges ===");
    let energy_level = 45;

    match energy_level {
        81..=100 => println!("Energy: Excellent ({}%)", energy_level),
        61..=80 => println!("Energy: Good ({}%)", energy_level),
        41..=60 => println!("Energy: Moderate ({}%)", energy_level),
        21..=40 => println!("Energy: Low ({}%)", energy_level),
        1..=20 => println!("Energy: Critical ({}%)", energy_level),
        0 => println!("Energy: Depleted"),
        _ => println!("Energy: Invalid reading ({})", energy_level),
    }

    println!("\n=== Match with tuples ===");
    let position = (3, 4);

    match position {
        (0, 0) => println!("At origin"),
        (0, y) => println!("On Y-axis at y={}", y),
        (x, 0) => println!("On X-axis at x={}", x),
        (x, y) if x == y => println!("On diagonal at ({}, {})", x, y),
        (x, y) if x > y => println!("Above diagonal at ({}, {})", x, y),
        (x, y) => println!("Below diagonal at ({}, {})", x, y),
    }

    println!("\n=== Match with Option enum ===");
    let maybe_item: Option<&str> = Some("Energy Cell");

    match maybe_item {
        Some("Energy Cell") => println!("Found energy cell!"),
        Some("Key") => println!("Found key!"),
        Some(item) => println!("Found unknown item: {}", item),
        None => println!("No item found"),
    }

    println!("\n=== Match as expression ===");
    let command = "move_right";

    let (dx, dy) = match command {
        "move_right" => (1, 0),
        "move_left" => (-1, 0),
        "move_up" => (0, 1),
        "move_down" => (0, -1),
        _ => {
            println!("Unknown command: {}", command);
            (0, 0)
        }
    };

    println!("Command '{}' results in movement: ({}, {})", command, dx, dy);
}