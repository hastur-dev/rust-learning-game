// Test Level 2 Task 3: Struct usage
struct GridInfo {
    x: i32,
    y: i32,
    content: String,
}

fn scan_level() {
    println!("Beginning level scan...");
    
    let mut item_locations = Vec::new();
    
    // Task 2: Nested loops for grid scanning  
    for y in 0..6 {
        for x in 0..6 {
            let scan_result = scan("current");
            println!("Scanned ({}, {}): {}", x, y, scan_result);
            
            // Task 3: Using struct and collecting data
            if scan_result != "empty" && scan_result != "wall" {
                item_locations.push((x, y, scan_result.clone()));
                
                // Create GridInfo struct instance
                let grid_info = GridInfo {
                    x: x,
                    y: y, 
                    content: scan_result.clone(),
                };
                println!("Created GridInfo at ({}, {})", grid_info.x, grid_info.y);
            }
        }
    }
    
    println!("Found {} items total", item_locations.len());
}

fn main() {
    println!("Starting Level 2 - Task 3 Test");
    scan_level();
    println!("Level 2 Task 3 complete!");
}