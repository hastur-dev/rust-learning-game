// Level 4 Task 4 Test: Variable Scope and Blocks
// Tests that user understands variable scope and lifetime

#[cfg(test)]
mod level4_task4_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_outer_scope_variable() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_outer = analyzer.code.contains("outer_variable") ||
                       analyzer.code.contains("outer scope");
        assert!(
            has_outer,
            "❌ You should declare a variable in the outer scope"
        );
    }

    #[test]
    fn test_has_inner_scope_block() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        // Look for nested curly braces that aren't function definitions
        let has_inner_block = analyzer.code.matches('{').count() > 1 &&
                            analyzer.code.contains("inner_variable");
        assert!(
            has_inner_block,
            "❌ You should create an inner scope with curly braces containing inner_variable"
        );
    }

    #[test]
    fn test_accesses_outer_from_inner() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_access = analyzer.code.contains("outer") &&
                        analyzer.code.contains("inner") &&
                        analyzer.code.contains("Accessing outer from inner");
        assert!(
            has_access,
            "❌ You should access outer scope variables from within the inner scope"
        );
    }

    #[test]
    fn test_demonstrates_inner_shadowing() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_inner_shadow = analyzer.code.contains("shadowing") &&
                             analyzer.code.contains("inner");
        assert!(
            has_inner_shadow,
            "❌ You should demonstrate shadowing within the inner scope"
        );
    }

    #[test]
    fn test_has_mutable_in_inner_scope() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_inner_mut = analyzer.code.contains("mut counter") ||
                          (analyzer.code.contains("mut") && analyzer.code.contains("for"));
        assert!(
            has_inner_mut,
            "❌ You should have a mutable variable in the inner scope (like counter)"
        );
    }

    #[test]
    fn test_demonstrates_loop_scope() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_loop_scope = analyzer.code.contains("for i in") &&
                           analyzer.code.contains("counter") &&
                           analyzer.code.contains("+=");
        assert!(
            has_loop_scope,
            "❌ You should demonstrate variable scope within a for loop"
        );
    }

    #[test]
    fn test_returns_to_outer_scope() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_return = analyzer.code.contains("Back to outer") ||
                        analyzer.code.contains("outer scope") &&
                        analyzer.code.contains('}') &&
                        analyzer.code.contains("println!");
        assert!(
            has_return,
            "❌ You should print from outer scope after the inner block ends"
        );
    }

    #[test]
    fn test_has_helper_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_function = analyzer.code.contains("fn calculate_something") ||
                         analyzer.code.contains("fn helper") ||
                         (analyzer.code.contains("fn ") && analyzer.code.contains("-> i32"));
        assert!(
            has_function,
            "❌ You should define a helper function to demonstrate function scope"
        );
    }

    #[test]
    fn test_calls_helper_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_call = analyzer.code.contains("calculate_something()") ||
                      analyzer.code.contains("result =");
        assert!(
            has_call,
            "❌ You should call the helper function and use its return value"
        );
    }

    #[test]
    fn test_demonstrates_variable_lifetime() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_lifetime_demo = analyzer.code.contains("goes out of scope") ||
                              analyzer.code.contains("local_value") ||
                              analyzer.code.contains("// ") && analyzer.code.contains("scope");
        assert!(
            has_lifetime_demo,
            "❌ You should comment about variables going out of scope"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let output_lines: Vec<&str> = result.stdout.lines().collect();
        let demonstrates_scope = output_lines.iter().any(|line| line.contains("outer")) &&
                               output_lines.iter().any(|line| line.contains("inner")) &&
                               output_lines.iter().any(|line| line.contains("Function result") || line.contains("result"));
        assert!(
            demonstrates_scope,
            "❌ Your program should demonstrate outer scope, inner scope, and function scope"
        );
    }
}

// Reference implementation for comparison
fn main() {
    let outer_variable = "I'm in the outer scope";
    println!("Outer scope: {}", outer_variable);

    {
        let inner_variable = "I'm in the inner scope";
        println!("Inner scope: {}", inner_variable);

        println!("Accessing outer from inner: {}", outer_variable);

        let outer_variable = "I'm shadowing the outer variable";
        println!("Shadowed in inner: {}", outer_variable);

        let mut counter = 0;
        for i in 1..=3 {
            counter += i;
            println!("Counter in loop: {}", counter);
        }
    }

    println!("Back to outer scope: {}", outer_variable);

    let result = calculate_something();
    println!("Function result: {}", result);
}

fn calculate_something() -> i32 {
    let local_value = 42;
    let calculation = local_value * 2;
    calculation
}