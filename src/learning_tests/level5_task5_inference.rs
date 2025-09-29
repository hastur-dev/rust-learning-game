// Level 5 Task 5 Test: Type Inference with Conversions
// Tests that user understands type inference in conversion contexts

#[cfg(test)]
mod level5_task5_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_demonstrates_numeric_inference() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_numeric_inference = analyzer.code.contains("small") &&
                                  analyzer.code.contains("large") &&
                                  analyzer.code.contains(".into()");
        assert!(
            has_numeric_inference,
            "❌ You should demonstrate type inference with numeric conversions"
        );
    }

    #[test]
    fn test_shows_explicit_vs_inferred() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_both = analyzer.code.contains("explicit: i64") &&
                      analyzer.code.contains("inferred: i64");
        assert!(
            has_both,
            "❌ You should show both explicit type annotation and inferred types"
        );
    }

    #[test]
    fn test_demonstrates_collection_inference() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_collection = analyzer.code.contains("Vec<i64>") &&
                           analyzer.code.contains(".map(") &&
                           analyzer.code.contains(".collect()");
        assert!(
            has_collection,
            "❌ You should demonstrate type inference with collection conversions"
        );
    }

    #[test]
    fn test_shows_string_conversion_inference() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_string_inference = analyzer.code.contains(".to_string()");
        assert!(
            has_string_inference,
            "❌ You should demonstrate string conversion with clear inference"
        );
    }

    #[test]
    fn test_requires_parse_type_annotation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_parse_annotation = analyzer.code.contains(".parse()") &&
                                 (analyzer.code.contains(": i32") || analyzer.code.contains(": f64"));
        assert!(
            has_parse_annotation,
            "❌ You should show that parsing requires type annotations"
        );
    }

    #[test]
    fn test_demonstrates_function_parameter_inference() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_function_inference = analyzer.code.contains("fn process_number") ||
                                   analyzer.code.contains("process_number(") &&
                                   analyzer.code.contains(".into()");
        assert!(
            has_function_inference,
            "❌ You should show type inference from function parameters"
        );
    }

    #[test]
    fn test_uses_turbofish_syntax() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_turbofish = analyzer.code.contains(".parse::<") ||
                          analyzer.code.contains(": i32 = ") && analyzer.code.contains(".into()");
        assert!(
            has_turbofish,
            "❌ You should demonstrate turbofish syntax for parsing and explicit type annotation for into()"
        );
    }

    #[test]
    fn test_shows_when_inference_fails() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_explicit_collection = analyzer.code.contains("Vec<i32>") &&
                                    analyzer.code.contains(".collect()");
        assert!(
            has_explicit_collection,
            "❌ You should show cases where explicit type annotation is needed"
        );
    }

    #[test]
    fn test_demonstrates_multiple_inference_contexts() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let inference_contexts = analyzer.code.matches(".into()").count() +
                               analyzer.code.matches(".to_string()").count() +
                               analyzer.code.matches(".parse()").count();
        assert!(
            inference_contexts >= 4,
            "❌ You should demonstrate type inference in multiple contexts"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let output_lines: Vec<&str> = result.stdout.lines().collect();
        let shows_inference = output_lines.iter().any(|line| line.contains("Small") || line.contains("500")) &&
                            output_lines.iter().any(|line| line.contains("Large") || line.contains("converted")) &&
                            output_lines.iter().any(|line| line.contains("Turbofish") || line.contains("456"));
        assert!(
            shows_inference,
            "❌ Your program should demonstrate various type inference scenarios"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let small = 100_i32;
    let large: i64 = small.into();

    let explicit: i64 = small.into();
    let inferred: i64 = small.into();

    println!("Small: {}", small);
    println!("Large (inferred): {}", large);
    println!("Explicit: {}", explicit);
    println!("Inferred: {}", inferred);

    let numbers = vec![1, 2, 3];
    let converted: Vec<i64> = numbers.into_iter().map(|x| x.into()).collect();

    println!("Original: [1, 2, 3]");
    println!("Converted: {:?}", converted);

    let value = 42;
    let string_val = value.to_string();

    println!("Value: {}", value);
    println!("String: {}", string_val);

    let parse_target = "123";

    let as_i32: i32 = parse_target.parse().expect("Parse failed");
    let as_f64: f64 = parse_target.parse().expect("Parse failed");

    println!("Parsed as i32: {}", as_i32);
    println!("Parsed as f64: {}", as_f64);

    fn process_number(num: i64) {
        println!("Processing: {}", num);
    }

    let input = 500_i32;
    process_number(input.into());

    let parsed_with_turbofish = "456".parse::<i32>().expect("Parse failed");
    let converted_with_turbofish: i32 = 789_i16.into();

    println!("Turbofish parsed: {}", parsed_with_turbofish);
    println!("Turbofish converted: {}", converted_with_turbofish);

    let ambiguous_collection: Vec<i32> = vec![1, 2, 3]
        .into_iter()
        .map(|x| x * 2)
        .collect();

    println!("Explicit collection: {:?}", ambiguous_collection);
}