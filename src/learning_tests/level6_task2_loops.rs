// Level 6 Task 2 Test: Loops - loop, while, and for
// Tests if the user code uses different types of loops

#[cfg(test)]
mod level6_task2_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_infinite_loop() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("loop {") || analyzer.code.contains("loop{"),
            "❌ Your code should contain an infinite loop using 'loop'"
        );
    }

    #[test]
    fn test_has_break_statement() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("break"),
            "❌ Your code should use 'break' to exit the infinite loop"
        );
    }

    #[test]
    fn test_has_while_loop() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("while "),
            "❌ Your code should contain a while loop"
        );
    }

    #[test]
    fn test_has_for_loop() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("for "),
            "❌ Your code should contain a for loop"
        );
    }

    #[test]
    fn test_has_range_in_for_loop() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_range = analyzer.code.contains("..") || analyzer.code.contains("..=");
        assert!(
            has_range,
            "❌ Your for loop should use a range like '1..=5' or '0..3'"
        );
    }

    #[test]
    fn test_has_continue_statement() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("continue"),
            "❌ Your code should demonstrate the 'continue' statement"
        );
    }

    #[test]
    fn test_has_counter_variable() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_counter = analyzer.code.contains("counter") || analyzer.code.contains("mut ");
        assert!(
            has_counter,
            "❌ Your code should use a mutable counter variable in loops"
        );
    }

    #[test]
    fn test_has_collection_iteration() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_vec = analyzer.code.contains("vec!") || analyzer.code.contains("Vec::") || analyzer.code.contains(".iter()");
        assert!(
            has_vec,
            "❌ Your code should demonstrate iterating over a collection (vec, array, etc.)"
        );
    }

    #[test]
    fn test_has_nested_loops() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let for_count = analyzer.code.matches("for ").count();
        assert!(
            for_count >= 2,
            "❌ Your code should demonstrate nested loops (multiple for loops)"
        );
    }

    #[test]
    fn test_has_labeled_break() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_label = analyzer.code.contains("'") && analyzer.code.contains("break ");
        assert!(
            has_label,
            "❌ Your code should demonstrate labeled breaks (e.g., 'outer: for ... break 'outer;)"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output evidence of different loop types
        let has_loop_output = result.stdout.contains("Loop") ||
                             result.stdout.contains("iteration") ||
                             result.stdout.contains("step");

        assert!(
            has_loop_output,
            "❌ Your program should output information about loop iterations"
        );
    }
}

// Reference implementation for comparison
fn main() {
    println!("=== Infinite loop with break ===");
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Loop iteration: {}", counter);

        if counter >= 3 {
            println!("Breaking out of infinite loop");
            break;
        }
    }

    println!("\n=== While loop ===");
    let mut energy = 100;
    while energy > 0 {
        println!("Energy remaining: {}", energy);
        energy -= 25;

        if energy == 25 {
            println!("Low energy warning!");
            continue;  // Skip the rest of this iteration
        }
    }

    println!("\n=== For loop with range ===");
    for i in 1..=5 {
        println!("For loop step: {}", i);
    }

    println!("\n=== For loop with collection ===");
    let positions = vec![(0, 0), (1, 2), (3, 4), (5, 6)];
    for (index, (x, y)) in positions.iter().enumerate() {
        println!("Position {}: ({}, {})", index, x, y);
    }

    println!("\n=== Nested loops ===");
    for row in 0..3 {
        for col in 0..3 {
            if row == col {
                print!("X ");
            } else {
                print!(". ");
            }
        }
        println!();  // New line after each row
    }

    println!("\n=== Loop with labeled break ===");
    'outer: for x in 0..3 {
        for y in 0..3 {
            if x == 1 && y == 1 {
                println!("Breaking outer loop at ({}, {})", x, y);
                break 'outer;
            }
            print!("({},{}) ", x, y);
        }
    }
    println!();
}