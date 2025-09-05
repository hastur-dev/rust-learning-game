// Test file for the improved messaging system
// This file demonstrates:
// 1. Multiple println! statements should stack into one popup
// 2. Mixed print types should create separate popups
// 3. Robot function calls should show in function results popup

fn main() {
    // Multiple println! statements - should stack in one popup
    println!("Hello, World!");
    println!("This is the second message");
    println!("And a third message");
    
    // Robot movement - should show function results
    move_bot("right");
    move_bot("down");
    
    // Error output - should create separate popup
    eprintln!("This is an error message");
    eprintln!("Another error message");
    
    // More robot actions
    grab();
    
    // More println after robot actions - should stack with previous println
    println!("Final message");
}