// Welcome to Rust Robot Programming Tutorial!
// This file is automatically saved as you type.
// You can also edit this file externally with any text editor.

// Display messages in the game:
println!("Starting robot program!");

// Always available functions:
move(right);
grab();
scan(left);

// Display educational messages:
// println!("Hello from the robot!");
// println!("Learning Rust is fun!");

// Door system (teaches boolean literals):
// open_door(true);   // Opens door at robot position
// open_door(false);  // Closes door at robot position

// Laser system (stuns enemies, destroys obstacles):
// laser::direction(up);
// laser::tile(5, 3);

// Example: Move in a pattern with messages
// println!("Moving in a square pattern");
// move(right);
// move(down);
// move(left);
// move(up);
// println!("Square pattern complete!");

// Example: Scan and grab with feedback
// println!("Scanning area and grabbing items");
// scan(up);
// grab();
// move(right);
// grab();
// println!("Items collected!");