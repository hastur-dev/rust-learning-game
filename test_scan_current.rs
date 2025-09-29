// Test scan("current") functionality for Level 2
fn scan_level() {
    println!("Beginning level scan...");
    
    let mut item_locations = Vec::new();
    
    // Task 2: Nested loops for grid scanning
    for y in 0..6 {
        for x in 0..6 {
            let scan_result = scan("current");
            println!("Scanned ({}, {}): {}", x, y, scan_result);
            
            if scan_result != "empty" && scan_result != "wall" {
                item_locations.push((x, y, scan_result.clone()));
            }
            
            // Move to next position (if not at edge)
            if x < 5 {
                move_bot("right");
            }
        }
        
        // Move to next row (if not at bottom)
        if y < 5 {
            move_bot("down");
            // Move back to left side
            for _ in 0..5 {
                move_bot("left");
            }
        }
    }
    
    println!("Scanning complete! Found {} items.", item_locations.len());
}

fn main() {
    println!("Starting Level 2");
    scan_level();
    println!("Level 2 complete!");
}