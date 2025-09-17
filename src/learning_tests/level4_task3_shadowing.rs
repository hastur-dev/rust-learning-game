// Level 4 Task 3 Test: Variable Shadowing
// Tests that user understands variable shadowing and type transformations

#[cfg(test)]
mod level4_task3_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_demonstrates_basic_shadowing() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        // Look for multiple 'let' declarations with the same variable name
        let has_shadowing = analyzer.code.contains("robot_data") &&
                          analyzer.code.matches("let robot_data").count() >= 2;
        assert!(
            has_shadowing,
            "❌ You should demonstrate variable shadowing by redefining the same variable name"
        );
    }

    #[test]
    fn test_shadows_with_type_change() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_type_change = analyzer.code.contains("parse()") ||
                            (analyzer.code.contains("\"") && analyzer.code.contains(": i32"));
        assert!(
            has_type_change,
            "❌ You should shadow a variable while changing its type (string to number)"
        );
    }

    #[test]
    fn test_has_parse_operation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_parse = analyzer.code.contains(".parse()") &&
                       (analyzer.code.contains("expect") || analyzer.code.contains("unwrap"));
        assert!(
            has_parse,
            "❌ You should use .parse() to convert string to number with proper error handling"
        );
    }

    #[test]
    fn test_demonstrates_calculation_shadowing() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_calc_shadow = analyzer.code.contains("*") || analyzer.code.contains("+");
        assert!(
            has_calc_shadow,
            "❌ You should shadow a variable with a calculation (like value * 2 + 100)"
        );
    }

    #[test]
    fn test_shows_shadowing_vs_mutation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_comparison = analyzer.code.contains("value") &&
                           analyzer.code.matches("let value").count() >= 2 &&
                           !analyzer.code.contains("mut value");
        assert!(
            has_comparison,
            "❌ You should demonstrate shadowing vs mutation (multiple 'let value' without mut)"
        );
    }

    #[test]
    fn test_demonstrates_block_scope() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_block_scope = analyzer.code.contains("{") &&
                            analyzer.code.contains("}") &&
                            analyzer.code.contains("Inside block");
        assert!(
            has_block_scope,
            "❌ You should demonstrate shadowing with block scope using curly braces"
        );
    }

    #[test]
    fn test_shows_scope_restoration() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_restoration = analyzer.code.contains("Outside block") ||
                            analyzer.code.contains("restored") ||
                            analyzer.code.contains("original");
        assert!(
            has_restoration,
            "❌ You should show that original values are restored outside blocks"
        );
    }

    #[test]
    fn test_has_multiple_transformations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_format = analyzer.code.contains("format!") ||
                        analyzer.code.contains("String");
        assert!(
            has_format,
            "❌ You should demonstrate multiple transformations (number to string, etc.)"
        );
    }

    #[test]
    fn test_prints_each_transformation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let println_count = analyzer.code.matches("println!").count();
        assert!(
            println_count >= 5,
            "❌ You should print the value after each shadowing transformation"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let output_lines: Vec<&str> = result.stdout.lines().collect();
        let shows_transformation = output_lines.iter().any(|line| line.contains("string")) &&
                                 output_lines.iter().any(|line| line.contains("number")) &&
                                 output_lines.iter().any(|line| line.contains("calculated"));
        assert!(
            shows_transformation,
            "❌ Your program should show the transformation from string to number to calculation"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let robot_data = "12345";
    println!("Robot data as string: {}", robot_data);

    let robot_data: i32 = robot_data.parse().expect("Failed to parse");
    println!("Robot data as number: {}", robot_data);

    let robot_data = robot_data * 2 + 100;
    println!("Robot data calculated: {}", robot_data);

    let value = 10;
    println!("Original value: {}", value);

    let value = value + 5;
    println!("Shadowed value: {}", value);

    let value = format!("The answer is {}", value);
    println!("Final shadowed value: {}", value);

    {
        let value = "Inside block";
        println!("Block value: {}", value);
    }
    println!("Outside block value: {}", value);
}