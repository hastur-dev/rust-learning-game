// Level 5 Task 3 Test: String Parsing and Error Handling
// Tests that user understands string parsing with Result and error handling

#[cfg(test)]
mod level5_task3_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_uses_parse_with_expect() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_parse_expect = analyzer.code.contains(".parse()") &&
                             analyzer.code.contains(".expect(");
        assert!(
            has_parse_expect,
            "❌ You should use .parse().expect() for basic string parsing"
        );
    }

    #[test]
    fn test_demonstrates_parse_with_match() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_match_parse = analyzer.code.contains("match") &&
                            analyzer.code.contains(".parse") &&
                            analyzer.code.contains("Ok(") &&
                            analyzer.code.contains("Err(");
        assert!(
            has_match_parse,
            "❌ You should use match to handle parse results (Ok/Err)"
        );
    }

    #[test]
    fn test_parses_multiple_strings() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_array_parsing = analyzer.code.contains("strings") ||
                              analyzer.code.contains("[") && analyzer.code.contains("]") &&
                              analyzer.code.contains("for") && analyzer.code.contains("parse");
        assert!(
            has_array_parsing,
            "❌ You should parse multiple strings in a loop or array"
        );
    }

    #[test]
    fn test_handles_both_success_and_failure() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let handles_both = analyzer.code.contains("success") ||
                         (analyzer.code.contains("Ok(") && analyzer.code.contains("Err("));
        assert!(
            handles_both,
            "❌ You should handle both successful and failed parsing cases"
        );
    }

    #[test]
    fn test_parses_float_values() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_float_parse = analyzer.code.contains("f64") ||
                            analyzer.code.contains("3.14") ||
                            analyzer.code.contains("float");
        assert!(
            has_float_parse,
            "❌ You should demonstrate parsing string to float (f64)"
        );
    }

    #[test]
    fn test_uses_unwrap_or_for_defaults() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_unwrap_or = analyzer.code.contains(".unwrap_or(");
        assert!(
            has_unwrap_or,
            "❌ You should use .unwrap_or() to provide default values for failed parsing"
        );
    }

    #[test]
    fn test_demonstrates_type_inference_in_parsing() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_inference = analyzer.code.contains(".parse::<") ||
                          analyzer.code.contains("Result<") ||
                          analyzer.code.contains(": i32 = ");
        assert!(
            has_inference,
            "❌ You should show type inference with parsing (parse::<i32> or type annotations)"
        );
    }

    #[test]
    fn test_handles_invalid_input() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_invalid = analyzer.code.contains("not_a_number") ||
                        analyzer.code.contains("invalid") ||
                        analyzer.code.contains("Error");
        assert!(
            has_invalid,
            "❌ You should include strings that can't be parsed to demonstrate error handling"
        );
    }

    #[test]
    fn test_prints_parse_results() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let println_count = analyzer.code.matches("println!").count();
        assert!(
            println_count >= 5,
            "❌ You should print the results of parsing operations"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let output_lines: Vec<&str> = result.stdout.lines().collect();
        let shows_parsing = output_lines.iter().any(|line| line.contains("Parsed") || line.contains("success")) &&
                          output_lines.iter().any(|line| line.contains("Error") || line.contains("Invalid")) &&
                          output_lines.iter().any(|line| line.contains("42") || line.contains("123"));
        assert!(
            shows_parsing,
            "❌ Your program should show both successful parsing and error cases"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let valid_number = "42";
    let parsed: i32 = valid_number.parse().expect("Failed to parse number");

    println!("Valid string: '{}'", valid_number);
    println!("Parsed number: {}", parsed);

    let strings = ["123", "45.67", "not_a_number", "0"];

    for string_val in strings.iter() {
        match string_val.parse::<i32>() {
            Ok(number) => println!("'{}' -> {} (success)", string_val, number),
            Err(error) => println!("'{}' -> Error: {}", string_val, error),
        }
    }

    let float_strings = ["3.14", "2.718", "invalid", "42.0"];

    for float_str in float_strings.iter() {
        match float_str.parse::<f64>() {
            Ok(float_val) => println!("'{}' -> {} (float)", float_str, float_val),
            Err(_) => println!("'{}' -> Invalid float", float_str),
        }
    }

    let inputs = ["100", "invalid", "200"];

    for input in inputs.iter() {
        let number: i32 = input.parse().unwrap_or(0);
        println!("'{}' -> {} (with default)", input, number);
    }

    let inferred_parse = "999".parse::<i32>().unwrap_or(-1);
    let explicit_type: Result<i32, _> = "888".parse();

    println!("Inferred parse: {}", inferred_parse);
    match explicit_type {
        Ok(val) => println!("Explicit parse: {}", val),
        Err(_) => println!("Explicit parse failed"),
    }
}