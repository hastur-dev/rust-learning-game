// Complete Level 2 test - all 4 tasks combined
// Task 3: Define struct above functions  
struct GridInfo {
    x: i32,
    y: i32,
    content: String,
}

// Task 1: Create function with print statement
fn scan_level() {
    println!("Beginning level scan...");
    
    // Task 3: Create vector for data collection
    let mut item_locations = Vec::new();
    
    // Task 2: Nested loops for grid scanning  
    for y in 0..6 {        // 6x6 grid height
        for x in 0..6 {    // 6x6 grid width
            // Movement and scanning code here
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
            
            // Task 4: Call the grab function
            grab_if_item(&scan_result);
        }
    }
    
    println!("Scanning complete! Found {} items.", item_locations.len());
}

// Task 4: Create grab function with conditional logic
fn grab_if_item(scan_result: &str) {
    if scan_result != "empty" && scan_result != "wall" && scan_result != "goal" {
        grab();
        println!("Grabbed: {}", scan_result);
    }
}

fn main() {
    println!("Starting Level 2 - Complete Test");
    // Task 1: Call scan_level function from main
    scan_level();
    println!("Level 2 complete test finished!");
}