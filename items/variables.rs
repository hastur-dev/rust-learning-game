// Variables in Rust - Interactive Tutorial
//
// Learn about variables through robot actions!

// In the game, you can't declare actual variables, but you can
// see how variable concepts apply to robot programming:

println!("Learning about Rust variables!");

// Immutable concept: Robot position changes only through move commands
println!("Robot position is like an immutable variable");
move(right);
println!("Position changed through explicit function call");

// Mutable concept: Scanner and grabber can be used repeatedly
println!("Scanner range acts like a mutable variable");
scan(up);
println!("Scanner used - this is like modifying a mutable value");

// Error handling concept:
println!("Attempting invalid operation...");
// This would show how Rust prevents errors:
eprintln!("In real Rust: cannot assign to immutable variable");
println!("But the robot continues safely!");

// Variable scope concept through robot actions:
println!("Robot functions have their own 'scope'");
grab();
println!("grab() completed - like leaving a variable scope");

println!("Variables in Rust are immutable by default for safety!");