// Level 4 Task 5 Test: Constants and Naming Conventions
// Tests that user understands constants vs variables and naming conventions

#[cfg(test)]
mod level4_task5_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_global_constants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_global_const = analyzer.code.contains("const MAX_ENERGY") ||
                             analyzer.code.contains("const ROBOT_NAME") ||
                             analyzer.code.contains("const PI");
        assert!(
            has_global_const,
            "❌ You should declare global constants outside functions (const MAX_ENERGY, etc.)"
        );
    }

    #[test]
    fn test_uses_screaming_snake_case() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_screaming = analyzer.code.contains("MAX_ENERGY") ||
                          analyzer.code.contains("ROBOT_NAME") ||
                          analyzer.code.matches("const ").any(|_| true);
        assert!(
            has_screaming,
            "❌ Constants should use SCREAMING_SNAKE_CASE naming convention"
        );
    }

    #[test]
    fn test_has_explicit_constant_types() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_types = analyzer.code.contains(": i32") ||
                       analyzer.code.contains(": &str") ||
                       analyzer.code.contains(": f64");
        assert!(
            has_types,
            "❌ Constants must have explicit type annotations (: i32, : &str, : f64)"
        );
    }

    #[test]
    fn test_uses_constants_in_calculations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_consts = analyzer.code.contains("MAX_ENERGY / 2") ||
                         analyzer.code.contains("PI * ") ||
                         (analyzer.code.contains("MAX_ENERGY") && analyzer.code.contains("half"));
        assert!(
            uses_consts,
            "❌ You should use constants in calculations (like MAX_ENERGY / 2)"
        );
    }

    #[test]
    fn test_demonstrates_snake_case_variables() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_snake_case = analyzer.code.contains("snake_case_variable") ||
                           analyzer.code.contains("another_example") ||
                           analyzer.code.contains("_");
        assert!(
            has_snake_case,
            "❌ You should demonstrate snake_case naming for variables"
        );
    }

    #[test]
    fn test_shows_const_vs_let_difference() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_comparison = analyzer.code.contains("immutable_var") &&
                           analyzer.code.contains("COMPILE_TIME") &&
                           analyzer.code.contains("const") &&
                           analyzer.code.contains("let");
        assert!(
            has_comparison,
            "❌ You should demonstrate the difference between const and let variables"
        );
    }

    #[test]
    fn test_has_compile_time_expression() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_compile_time = analyzer.code.contains("50 + 50") ||
                             analyzer.code.contains("const ") && analyzer.code.contains(" + ") ||
                             analyzer.code.contains("const ") && analyzer.code.contains(" * ");
        assert!(
            has_compile_time,
            "❌ You should show const with compile-time expressions (like 50 + 50)"
        );
    }

    #[test]
    fn test_demonstrates_block_constants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_block_const = analyzer.code.contains("BLOCK_CONSTANT") ||
                            (analyzer.code.contains("const") && analyzer.code.contains("{"));
        assert!(
            has_block_const,
            "❌ You should demonstrate constants within block scope"
        );
    }

    #[test]
    fn test_combines_multiple_constants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_combination = analyzer.code.contains("MAX_ENERGY + ") ||
                            analyzer.code.contains("COMPILE_TIME + ") ||
                            analyzer.code.contains("calculation");
        assert!(
            has_combination,
            "❌ You should combine multiple constants in calculations"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let output_lines: Vec<&str> = result.stdout.lines().collect();
        let shows_constants = output_lines.iter().any(|line| line.contains("Maximum energy")) &&
                            output_lines.iter().any(|line| line.contains("Robot name")) &&
                            output_lines.iter().any(|line| line.contains("1000") || line.contains("Ferris"));
        assert!(
            shows_constants,
            "❌ Your program should output information about the constants"
        );
    }
}

// Reference implementation for comparison
const MAX_ENERGY: i32 = 1000;
const ROBOT_NAME: &str = "Ferris";
const PI: f64 = 3.141592653589793;

fn main() {
    println!("Maximum energy: {}", MAX_ENERGY);
    println!("Robot name: {}", ROBOT_NAME);
    println!("Pi value: {}", PI);

    let half_max_energy = MAX_ENERGY / 2;
    let circle_area = PI * 5.0 * 5.0;

    println!("Half max energy: {}", half_max_energy);
    println!("Circle area: {}", circle_area);

    let snake_case_variable = "variables use snake_case";
    let another_example = 42;

    println!("Variable: {}", snake_case_variable);
    println!("Another: {}", another_example);

    let immutable_var = 100;
    const COMPILE_TIME: i32 = 50 + 50;
    println!("Compile-time constant: {}", COMPILE_TIME);

    {
        const BLOCK_CONSTANT: i32 = 999;
        println!("Block constant: {}", BLOCK_CONSTANT);

        let calculation = MAX_ENERGY + COMPILE_TIME + BLOCK_CONSTANT;
        println!("Combined calculation: {}", calculation);
    }
}