// Write your robot control code here
// Available functions:
// robot.move_up(), robot.move_down(), robot.move_left(), robot.move_right()
// robot.scan() -> returns what's at the robot's current position
// println!() for debugging output
fn scan_level() {
    for y in 0..6 {
        for x in 0..6 {
            let scan_result = scan("current");
            println!("scanned {} {} {}", x, y, scan_result());
        }
     
}

fn main() {
    println!("Robot starting...");
    scan_level();    
    // Your code here
    
}
