// Level 2 Task 2 Test: Add loops to scan each tile
// Tests that user adds nested loops inside scan_level function

#[cfg(test)]
mod level2_task2_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_scan_level_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_function("scan_level"),
            "❌ You need to keep the scan_level function from Task 1"
        );
    }

    #[test]
    fn test_has_nested_for_loops() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.has_nested_for_loops(),
            "❌ You need nested for loops: 'for y in 0..6' and 'for x in 0..6'"
        );
    }

    #[test]
    fn test_has_y_loop() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("for y in 0..6"),
            "❌ You need a loop: 'for y in 0..6' for the grid height"
        );
    }

    #[test]
    fn test_has_x_loop() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("for x in 0..6"),
            "❌ You need a loop: 'for x in 0..6' for the grid width"
        );
    }

    #[test]
    fn test_loops_print_positions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        // Look for some kind of position printing
        let has_position_print = analyzer.code.contains("x, y") || 
                                analyzer.code.contains("{}, {}") ||
                                analyzer.code.contains("position");
        assert!(
            has_position_print,
            "❌ Your loops should print the x, y positions being scanned"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");
        
        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");
        
        // Should contain the beginning message from Task 1
        assert!(
            result.stdout.contains("Beginning level scan..."),
            "❌ Your program should still output 'Beginning level scan...' from Task 1"
        );
        
        // Should contain evidence of loop execution (multiple lines of output)
        let output_lines: Vec<&str> = result.stdout.lines().collect();
        assert!(
            output_lines.len() > 1,
            "❌ Your nested loops should produce multiple lines of output"
        );
    }

    #[test]
    fn test_processes_all_grid_positions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");
        
        // Check if output suggests all 36 positions are processed (6x6 = 36)
        // This is a heuristic - looking for reasonable amount of output
        let output_lines: Vec<&str> = result.stdout.lines().collect();
        assert!(
            output_lines.len() >= 10, // At least some substantial output
            "❌ Your nested loops should process multiple grid positions (6x6 = 36 total)"
        );
    }

    #[test]
    fn test_loop_range_is_correct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        // Make sure they use 0..6 (not 1..6 or 0..5 etc.)
        assert!(
            analyzer.code.contains("0..6"),
            "❌ Your loops should use the range 0..6 for a 6x6 grid"
        );
        
        // Make sure they have at least 2 occurrences of 0..6 (for both x and y)
        assert!(
            analyzer.count_pattern("0..6") >= 2,
            "❌ You need two loops, both using 0..6 range"
        );
    }
}

// Reference implementation for comparison
fn scan_level() {
    println!("Beginning level scan...");
    
    for y in 0..6 {
        for x in 0..6 {
            println!("Scanning position ({}, {})", x, y);
        }
    }
}

fn main() {
    scan_level();
}