// Level 2 Task 4 Test: Create function with conditional logic
// Tests that user creates grab_if_item function with if statement

#[cfg(test)]
mod level2_task4_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_grab_if_item_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_function("grab_if_item"),
            "❌ You need to create a function named 'grab_if_item'"
        );
    }

    #[test]
    fn test_still_has_all_previous_requirements() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Task 1: scan_level function
        assert!(
            analyzer.contains_function("scan_level"),
            "❌ You need to keep the scan_level function from Task 1"
        );
        
        // Task 2: nested loops
        assert!(
            analyzer.has_nested_for_loops(),
            "❌ You need to keep the nested for loops from Task 2"
        );
        
        // Task 3: GridInfo struct
        assert!(
            analyzer.contains_struct("GridInfo"),
            "❌ You need to keep the GridInfo struct from Task 3"
        );
    }

    #[test]
    fn test_grab_if_item_has_parameter() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Look for function signature with string parameter
        let has_str_param = analyzer.code.contains("grab_if_item(&str") ||
                           analyzer.code.contains("grab_if_item(scan_result: &str") ||
                           analyzer.code.contains("grab_if_item(result: &str");
        
        assert!(
            has_str_param,
            "❌ grab_if_item function should take a string parameter like 'scan_result: &str'"
        );
    }

    #[test]
    fn test_grab_if_item_has_conditional_logic() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Look for if statements
        assert!(
            analyzer.code.contains("if "),
            "❌ grab_if_item function should use 'if' statements for conditional logic"
        );
    }

    #[test]
    fn test_grab_if_item_checks_for_empty_and_wall() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Should check against "empty" and "wall"
        assert!(
            analyzer.code.contains("\"empty\""),
            "❌ grab_if_item should check if scan_result is not 'empty'"
        );
        assert!(
            analyzer.code.contains("\"wall\""),
            "❌ grab_if_item should check if scan_result is not 'wall'"
        );
    }

    #[test]
    fn test_uses_boolean_operators() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Look for boolean operators
        let has_boolean_ops = analyzer.code.contains("&&") || analyzer.code.contains("!=");
        assert!(
            has_boolean_ops,
            "❌ You should use boolean operators like '!=' and '&&' in your conditional logic"
        );
    }

    #[test]
    fn test_calls_grab_if_item_from_scan_level() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Look for function call
        assert!(
            analyzer.code.contains("grab_if_item("),
            "❌ You should call grab_if_item() from inside your scan_level loops"
        );
    }

    #[test]
    fn test_grab_if_item_prints_when_grabbing() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Should have println in grab_if_item function area
        // This is a heuristic - we assume if there's println and grab_if_item, they're related
        assert!(
            analyzer.contains_println(),
            "❌ grab_if_item should print a message when it grabs an item"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");
        
        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");
        
        // Should still contain the beginning message from Task 1
        assert!(
            result.stdout.contains("Beginning level scan..."),
            "❌ Your program should still output 'Beginning level scan...' from Task 1"
        );
    }

    #[test]
    fn test_demonstrates_conditional_behavior() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");
        
        // Look for evidence that conditional logic is working
        // Should show different behavior for different scan results
        let shows_grabbed_items = result.stdout.contains("Grabbed:") ||
                                 result.stdout.contains("grabbed") ||
                                 result.stdout.contains("Grabbed");
        
        let shows_counting_or_tracking = result.stdout.contains("Found") ||
                                       result.stdout.contains("total") ||
                                       result.stdout.contains("items");
        
        assert!(
            shows_grabbed_items || shows_counting_or_tracking,
            "❌ Your program should demonstrate the conditional logic working (showing grabbed items or counting)"
        );
    }

    #[test]
    fn test_proper_function_organization() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Check function order: struct first, then functions, then main last
        let struct_pos = analyzer.code.find("struct GridInfo").unwrap_or(usize::MAX);
        let scan_level_pos = analyzer.code.find("fn scan_level").unwrap_or(usize::MAX);
        let grab_if_item_pos = analyzer.code.find("fn grab_if_item").unwrap_or(usize::MAX);
        let main_pos = analyzer.code.find("fn main").unwrap_or(usize::MAX);
        
        assert!(
            struct_pos < scan_level_pos && struct_pos < main_pos,
            "❌ GridInfo struct should be defined before functions"
        );
        
        assert!(
            scan_level_pos < main_pos && grab_if_item_pos < main_pos,
            "❌ All helper functions should be defined before main()"
        );
    }
}

// Reference implementation for comparison
struct GridInfo {
    x: i32,
    y: i32,
    content: String,
}

fn scan_level() {
    println!("Beginning level scan...");
    
    let mut item_locations = Vec::new();
    
    for y in 0..6 {
        for x in 0..6 {
            let scan_result = format!("scan_{}_{}", x, y);
            
            if scan_result != "empty" && scan_result != "wall" {
                item_locations.push((x, y, scan_result.clone()));
            }
            
            grab_if_item(&scan_result);
        }
    }
    
    println!("Found {} items total", item_locations.len());
}

fn grab_if_item(scan_result: &str) {
    if scan_result != "empty" && scan_result != "wall" && scan_result != "goal" {
        println!("Grabbed: {}", scan_result);
    }
}

fn main() {
    scan_level();
}