// Level 2 Task 3 Test: Create struct to track grid information
// Tests that user creates GridInfo struct and uses it to track items

#[cfg(test)]
mod level2_task3_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_gridinfo_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_struct("GridInfo"),
            "❌ You need to define a struct named 'GridInfo'"
        );
    }

    #[test]
    fn test_gridinfo_has_required_fields() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Check for the required fields
        assert!(
            analyzer.code.contains("x: i32") || analyzer.code.contains("x:i32"),
            "❌ GridInfo struct should have an 'x: i32' field"
        );
        assert!(
            analyzer.code.contains("y: i32") || analyzer.code.contains("y:i32"),
            "❌ GridInfo struct should have a 'y: i32' field"
        );
        assert!(
            analyzer.code.contains("content: String") || analyzer.code.contains("content:String"),
            "❌ GridInfo struct should have a 'content: String' field"
        );
    }

    #[test]
    fn test_still_has_scan_level_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_function("scan_level"),
            "❌ You need to keep the scan_level function from previous tasks"
        );
    }

    #[test]
    fn test_still_has_nested_loops() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.has_nested_for_loops(),
            "❌ You need to keep the nested for loops from Task 2"
        );
    }

    #[test]
    fn test_creates_gridinfo_instances() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Look for GridInfo instance creation patterns
        let creates_instance = analyzer.code.contains("GridInfo {") ||
                              analyzer.code.contains("GridInfo{");
        assert!(
            creates_instance,
            "❌ You should create GridInfo instances inside your loops"
        );
    }

    #[test]
    fn test_uses_vector_for_collection() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Look for Vec usage
        assert!(
            analyzer.code.contains("Vec::new()") || analyzer.code.contains("vec!"),
            "❌ You should use a Vec to collect item locations or grid data"
        );
    }

    #[test]
    fn test_struct_defined_before_functions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        let struct_pos = analyzer.code.find("struct GridInfo").unwrap_or(usize::MAX);
        let fn_pos = analyzer.code.find("fn ").unwrap_or(0);
        
        assert!(
            struct_pos < fn_pos,
            "❌ The GridInfo struct should be defined before any functions"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");
        
        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");
        
        // Should still contain the beginning message
        assert!(
            result.stdout.contains("Beginning level scan..."),
            "❌ Your program should still output 'Beginning level scan...' from Task 1"
        );
    }

    #[test]
    fn test_prints_item_information() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");
        
        // Look for evidence that the code is tracking and reporting on items/locations
        let mentions_items_or_locations = result.stdout.contains("Found") ||
                                        result.stdout.contains("locations") ||
                                        result.stdout.contains("items") ||
                                        result.stdout.contains("Item");
        
        assert!(
            mentions_items_or_locations,
            "❌ Your program should print information about item locations found during the scan"
        );
    }

    #[test]
    fn test_uses_struct_fields_correctly() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        
        // Look for field access patterns
        let accesses_fields = analyzer.code.contains("x: x") ||
                             analyzer.code.contains("y: y") ||
                             analyzer.code.contains("content:");
        
        assert!(
            accesses_fields,
            "❌ You should populate the GridInfo struct fields with appropriate values"
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
            let content = format!("tile_{}_{}", x, y);
            
            let grid_info = GridInfo {
                x: x,
                y: y,
                content: content.clone(),
            };
            
            if content.contains("item") {
                item_locations.push((x, y, content));
            }
        }
    }
    
    println!("Found {} locations", item_locations.len());
}

fn main() {
    scan_level();
}