// Test code for obstacle tracking - shows moves and tile reveals
fn main() {
    println!("=== Obstacle Testing Demo ===");
    println!("Testing robot movement and scanning with obstacles");
    
    // Try moving in different directions
    println!("Phase 1: Initial movement tests");
    move_bot("right");
    move_bot("right");
    
    // Scan the area to see what's around us
    println!("Phase 2: Area scanning");
    let scan_result = scan("current");
    println!("Current area scan: {}", scan_result);
    
    // Try more movement and scanning
    println!("Phase 3: Continued exploration");
    move_bot("down");
    let scan_result2 = scan("current");
    println!("New area scan: {}", scan_result2);
    
    // Directional scans to see obstacles
    println!("Phase 4: Directional obstacle detection");
    let up_scan = scan("up");
    println!("Up scan: {}", up_scan);
    
    let right_scan = scan("right");  
    println!("Right scan: {}", right_scan);
    
    let down_scan = scan("down");
    println!("Down scan: {}", down_scan);
    
    let left_scan = scan("left");
    println!("Left scan: {}", left_scan);
    
    // Try to move toward potential obstacles
    println!("Phase 5: Testing obstacle collision");
    move_bot("up");
    move_bot("up");
    move_bot("left");
    
    println!("=== Test Complete ===");
    println!("Check final position and move count!");
}