// Level 20 Task 1 Test: Fn Trait Mastery
// Tests that user implements immutable closures using Fn trait for stateless operations

#[cfg(test)]
mod level20_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_closure_creation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_closure = analyzer.code.contains("|") &&
                         (analyzer.code.contains("||") || analyzer.code.contains("|x|"));
        assert!(
            has_closure,
            "❌ You need to create closures using closure syntax |args|"
        );
    }

    #[test]
    fn test_uses_fn_trait_bound() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_fn_trait = analyzer.code.contains("Fn(") ||
                          analyzer.code.contains("impl Fn") ||
                          analyzer.code.contains("<F: Fn");
        assert!(
            has_fn_trait,
            "❌ You should use Fn trait bound for function parameters"
        );
    }

    #[test]
    fn test_implements_filter_operation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_filter = analyzer.code.contains("filter") &&
                        (analyzer.code.contains("closure") ||
                         analyzer.code.contains("sensor") ||
                         analyzer.code.contains("valid"));
        assert!(
            has_filter,
            "❌ You should implement filtering operation with closures"
        );
    }

    #[test]
    fn test_implements_transform_operation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_transform = analyzer.code.contains("transform") ||
                          analyzer.code.contains("map") ||
                          analyzer.code.contains("coordinate");
        assert!(
            has_transform,
            "❌ You should implement transform/map operation for coordinate conversion"
        );
    }

    #[test]
    fn test_implements_validation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_validation = analyzer.code.contains("validate") ||
                           analyzer.code.contains("valid") ||
                           analyzer.code.contains("integrity");
        assert!(
            has_validation,
            "❌ You should implement validation with stateless closures"
        );
    }

    #[test]
    fn test_demonstrates_multiple_calls() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_multiple_calls = analyzer.code.contains("multiple") ||
                               analyzer.code.contains("reuse") ||
                               (analyzer.code.matches("()").count() >= 3);
        assert!(
            has_multiple_calls,
            "❌ You should demonstrate that Fn closures can be called multiple times"
        );
    }

    #[test]
    fn test_captures_immutable_references() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_capture = analyzer.code.contains("&") &&
                         (analyzer.code.contains("capture") ||
                          analyzer.code.contains("reference") ||
                          analyzer.code.contains("borrow"));
        assert!(
            has_capture,
            "❌ You should demonstrate capturing immutable references in closures"
        );
    }

    #[test]
    fn test_has_generic_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_generic = analyzer.code.contains("<") && analyzer.code.contains(">") &&
                         analyzer.code.contains("fn ") &&
                         (analyzer.code.contains("F") || analyzer.code.contains("T"));
        assert!(
            has_generic,
            "❌ You should create generic functions that accept Fn trait objects"
        );
    }

    #[test]
    fn test_demonstrates_stateless_behavior() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ You should demonstrate stateless closure behavior with output"
        );
    }
}

// Reference implementation
fn main() {
    println!("Level 20 Task 1: Fn Trait Implementation");
    // Reference pattern for Fn trait usage
}

// Reference Fn trait pattern
// fn process_data<F>(data: &[i32], processor: F) -> Vec<i32>
// where
//     F: Fn(&i32) -> i32,
// {
//     data.iter().map(processor).collect()
// }
//
// fn demonstrate_fn_trait() {
//     let threshold = 50;
//
//     // Immutable closure that captures by reference
//     let filter_valid = |&x: &i32| x > threshold;
//     let transform_coord = |&x: &i32| x * 2 + 1;
//
//     let sensor_data = vec![10, 60, 30, 80, 45];
//
//     // Can call multiple times
//     let filtered: Vec<_> = sensor_data.iter().filter(|&&x| filter_valid(&x)).collect();
//     let transformed = process_data(&sensor_data, transform_coord);
//
//     println!("Filtered: {:?}", filtered);
//     println!("Transformed: {:?}", transformed);
// }