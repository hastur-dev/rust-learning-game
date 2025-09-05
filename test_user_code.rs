// Write your robot control code here
// Available functions:
// robot.move_up(), robot.move_down(), robot.move_left(), robot.move_right()
// robot.scan() -> returns what's at the robot's current position
// println!() for debugging output

struct GridInfo {
    x: i32,
    y: i32,
    content: String,
}

fn scan_level() {
    let mut item_locations = Vec::new();
    let mut scan_result = scan("current");
    for x in 0..6 {
        for y in 0..6 {
            if scan_result != "empty" && scan_result != "wall" {
                item_locations.push((x, y, scan_result.clone()));
            }
        }
    }
}

fn main() {
    println!("Robot starting...");
    scan_level();
    // Your code here
}