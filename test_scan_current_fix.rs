// Test the fixed scan("current") functionality
fn main() {
    println!("Testing scan(\"current\") fix...");

    // First scan - should scan the current position + 1 tile in each direction
    let result1 = scan("current");
    println!("First scan result: {}", result1);

    // Second scan - should expand by 1 more tile in each direction
    let result2 = scan("current");
    println!("Second scan result: {}", result2);

    // Third scan - should expand by 1 more tile in each direction
    let result3 = scan("current");
    println!("Third scan result: {}", result3);

    println!("Scan test complete!");
}