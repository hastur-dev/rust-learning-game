// Test Level 2 Task 4: Function with conditional logic
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
            }
            
            // Task 4: Call the grab function
            grab_if_item(&scan_result);
        }
    }
    
    println!("Found {} items total", item_locations.len());
}

// Task 4: Create grab function with conditional logic
fn grab_if_item(scan_result: &str) {
    if scan_result != "empty" && scan_result != "wall" && scan_result != "goal" {
        grab();
        println!("Grabbed: {}", scan_result);
    }
}

fn main() {
    println!("Starting Level 2 - Task 4 Test");
    scan_level();
    println!("Level 2 Task 4 complete!");
}