fn main() {
    // Task 1: println! output
    println!("Hello, Rust robot!");
    
    // Task 2: eprintln! output
    eprintln!("This is an error message for debugging");
    
    // Task 3: Variable used in print statement
    let my_message = "Variables are powerful!";
    println!("{}", my_message);
    
    // Task 4: Mutable variable with scan function
    let mut scan_result = scan("right");
    println!("Scan found: {}", scan_result);
    
    // Task 5: u32 integer used for movement
    let steps: u32 = 3;
    for _i in 0..steps {
        move_bot("right");
    }
    
    println!("Level 1 complete!");
}