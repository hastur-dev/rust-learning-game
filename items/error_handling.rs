// Error Handling in Rust - Game Tutorial
//
// Learn how Rust handles errors through interactive robot programming

println!("Demonstrating Rust error handling concepts");

// Normal operation
println!("Normal robot operation:");
move(right);
grab();
println!("✅ Operations completed successfully");

// Simulating different types of messages:
println!("📋 Status: Robot systems online");
eprintln!("⚠️ Warning: Low battery detected");  
println!("🔧 Debug: Sensor calibration complete");

// Error recovery concept:
println!("Attempting potentially risky operation...");
laser::direction(up);
println!("✅ Laser fired safely - no panic needed");

// Demonstrate panic for critical errors:
println!("For critical system failures, Rust uses panic:");
// Uncomment next line to see panic in action:
// panic!("Critical system failure - robot must halt");

println!("Error handling makes Rust programs more reliable");
println!("Use println! for normal messages");  
println!("Use eprintln! for errors and warnings");
println!("Use panic! only for unrecoverable failures");

// Practical robot error handling:
println!("Robot completing mission with error awareness");
scan(left);
println!("✅ Scan completed - no obstacles detected");
move(left);
println!("✅ Movement successful");

eprintln!("Tutorial complete - you've learned Rust error handling!");