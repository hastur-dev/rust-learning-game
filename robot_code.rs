// Rust Robot Programming - External File Mode
// Save this file and the game will automatically detect changes!
// Use your favorite IDE/editor to write code here.
// robo bobo stuffs
// Level 2 Strategy: Find and grab the scanner item
// Note: You must grab ALL items before you can complete the level!
search_all()
set_auto_grab(true)
// Enable auto-grab to automatically collect items when moving
// Try this function to search all reachable areas:
// search_all();
// Once you find the scanner item (marked with "!"), grab it:
// grab();  // This will unlock the scan() function!

// You can also use:
// move(right);
// move(up);
// grab();  // Available from Level 2+ - REQUIRED to pick up scanner!
// scan(left);  // Available ONLY after grabbing scanner with grab()

// IMPORTANT: Items and obstacles ("!" and "?") only appear on explored squares!

// Example: Manual exploration to find items
// set_auto_grab(true);
// move(right);
// move(down);
// move(left);
// move(up);

// Example: Manual grabbing
// set_auto_grab(false);
// grab();

// Example: Advanced exploration with auto-grab
// set_auto_grab(true);
// search_all();
// move(right);
// move(right);
// set_auto_grab(false);  // Disable for precise control
