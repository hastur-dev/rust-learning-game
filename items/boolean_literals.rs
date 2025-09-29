// Boolean Literals in Rust
//
// Rust has two boolean values: true and false
// These are keywords and must be lowercase

fn main() {
    let door_should_open: bool = true;
    let door_should_close: bool = false;
    
    // In this game:
    // open_door(true);  // Opens the door
    // open_door(false); // Closes the door
    
    // Boolean expressions:
    let is_door_open = door_should_open && !door_should_close;
    let can_proceed = is_door_open || has_alternate_path();
}

fn has_alternate_path() -> bool {
    false // This is also a boolean literal
}