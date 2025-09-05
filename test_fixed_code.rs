fn main() {
    println!("Robot starting...");
    
    // Move the robot and collect items
    move_bot("right");
    grab(); // Try to grab anything at current position
    
    move_bot("down");
    grab(); // Try to grab anything at current position
    
    // Try scanning in different directions
    scan("up");
    scan("right");
    
    println!("Exploration complete!");
}