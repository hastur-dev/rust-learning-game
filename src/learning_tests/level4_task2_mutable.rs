// Level 4 Task 2 Test: Mutable Variable Bindings
// Tests that user understands mutable variables and the mut keyword

#[cfg(test)]
mod level4_task2_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_mutable_variables() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let mut_count = analyzer.code.matches("let mut").count();
        assert!(
            mut_count >= 3,
            "❌ You should have at least 3 mutable variables (let mut)"
        );
    }

    #[test]
    fn test_has_mutable_numeric_variables() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_mutable_nums = analyzer.code.contains("mut robot_position") ||
                              analyzer.code.contains("mut energy_level") ||
                              analyzer.code.contains("mut position") ||
                              analyzer.code.contains("mut energy");
        assert!(
            has_mutable_nums,
            "❌ You should have mutable numeric variables (robot_position, energy_level)"
        );
    }

    #[test]
    fn test_has_mutable_boolean() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_mutable_bool = analyzer.code.contains("mut is_active") ||
                              (analyzer.code.contains("mut") &&
                               (analyzer.code.contains("true") || analyzer.code.contains("false")));
        assert!(
            has_mutable_bool,
            "❌ You should have a mutable boolean variable (is_active)"
        );
    }

    #[test]
    fn test_modifies_mutable_variables() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_modifications = analyzer.code.contains("+=") ||
                               analyzer.code.contains("-=") ||
                               analyzer.code.contains("=") && !analyzer.code.contains("==");
        assert!(
            has_modifications,
            "❌ You should modify the mutable variables (use +=, -=, or = operators)"
        );
    }

    #[test]
    fn test_demonstrates_multiple_modifications() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let modification_count = analyzer.code.matches("+=").count() +
                               analyzer.code.matches("-=").count() +
                               analyzer.code.lines().filter(|line|
                                   line.contains("=") &&
                                   !line.contains("==") &&
                                   !line.contains("let") &&
                                   !line.contains("!=")).count();
        assert!(
            modification_count >= 3,
            "❌ You should demonstrate multiple variable modifications"
        );
    }

    #[test]
    fn test_has_loop_with_mutations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_loop = analyzer.code.contains("for") &&
                      (analyzer.code.contains("+=") || analyzer.code.contains("-="));
        assert!(
            has_loop,
            "❌ You should use a loop to demonstrate multiple modifications"
        );
    }

    #[test]
    fn test_prints_before_and_after() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let println_count = analyzer.code.matches("println!").count();
        assert!(
            println_count >= 4,
            "❌ You should print values before and after modifications"
        );
    }

    #[test]
    fn test_demonstrates_type_safety() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        // Should not try to assign wrong types (this would be a compile error)
        let no_type_errors = !analyzer.code.contains("robot_position = \"invalid\"") &&
                           !analyzer.code.contains("energy_level = \"text\"");
        assert!(
            no_type_errors,
            "❌ Your code should maintain type safety even with mutable variables"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should show progression of values
        let lines: Vec<&str> = result.stdout.lines().collect();
        let has_progression = lines.len() >= 4 &&
                            lines.iter().any(|line| line.contains("Initial") || line.contains("position")) &&
                            lines.iter().any(|line| line.contains("New") || line.contains("Step"));
        assert!(
            has_progression,
            "❌ Your program should show the progression of mutable variable values"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let mut robot_position = 0;
    let mut energy_level = 100;
    let mut is_active = true;

    println!("Initial position: {}", robot_position);
    println!("Initial energy: {}", energy_level);
    println!("Initially active: {}", is_active);

    robot_position += 5;
    energy_level -= 10;
    is_active = false;

    println!("New position: {}", robot_position);
    println!("New energy: {}", energy_level);
    println!("Currently active: {}", is_active);

    for i in 1..=3 {
        robot_position += i;
        energy_level -= 5;
        println!("Step {}: position = {}, energy = {}", i, robot_position, energy_level);
    }
}