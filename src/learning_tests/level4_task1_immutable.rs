// Level 4 Task 1 Test: Immutable Variable Bindings
// Tests that user understands immutable variables and their benefits

#[cfg(test)]
mod level4_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_immutable_variables() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        // Count lines with 'let' but without 'mut'
        let immutable_count = analyzer.code.lines()
            .filter(|line| line.contains("let ") && !line.contains("mut"))
            .count();
        assert!(
            immutable_count >= 3,
            "❌ You should have at least 3 immutable variables (let without mut)"
        );
    }

    #[test]
    fn test_has_string_literal_binding() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_string = analyzer.code.contains("robot_name") ||
                        analyzer.code.contains("\"") ||
                        analyzer.code.contains("Ferris");
        assert!(
            has_string,
            "❌ You should bind a string literal to a variable (like robot_name)"
        );
    }

    #[test]
    fn test_has_numeric_bindings() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_numbers = analyzer.code.contains("robot_id") ||
                         analyzer.code.contains("energy_level") ||
                         analyzer.code.chars().any(|c| c.is_numeric());
        assert!(
            has_numbers,
            "❌ You should bind numeric values to variables (like robot_id, energy_level)"
        );
    }

    #[test]
    fn test_prints_immutable_values() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ Your code should print the immutable variable values"
        );
    }

    #[test]
    fn test_demonstrates_calculated_value() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_calculation = analyzer.code.contains("*") ||
                            analyzer.code.contains("+") ||
                            analyzer.code.contains("-") ||
                            analyzer.code.contains("/");
        assert!(
            has_calculation,
            "❌ You should demonstrate using immutable values in calculations"
        );
    }

    #[test]
    fn test_has_conditional_logic() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_condition = analyzer.code.contains("if") &&
                          (analyzer.code.contains("==") || analyzer.code.contains("!="));
        assert!(
            has_condition,
            "❌ You should demonstrate using immutable values in conditional logic"
        );
    }

    #[test]
    fn test_no_mutable_keywords() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let mut_count = analyzer.code.matches("mut").count();
        assert!(
            mut_count == 0,
            "❌ This task should only use immutable variables (no 'mut' keyword)"
        );
    }

    #[test]
    fn test_demonstrates_immutability_benefit() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_benefit_demo = analyzer.code.contains("calculated") ||
                             analyzer.code.contains("rely") ||
                             analyzer.code.contains("safe") ||
                             analyzer.code.contains("100");
        assert!(
            has_benefit_demo,
            "❌ You should demonstrate the benefits of immutable variables"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_variable_output = result.stdout.contains("Robot") ||
                                result.stdout.contains("name") ||
                                result.stdout.contains("energy") ||
                                result.stdout.chars().any(|c| c.is_numeric());
        assert!(
            has_variable_output,
            "❌ Your program should output information about the robot variables"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let robot_name = "Ferris";
    let robot_id = 12345;
    let energy_level = 100;

    println!("Robot name: {}", robot_name);
    println!("Robot ID: {}", robot_id);
    println!("Energy level: {}", energy_level);

    let calculated_value = robot_id * 2;
    println!("Calculated value: {}", calculated_value);

    if energy_level == 100 {
        println!("Robot is fully charged!");
    }
}