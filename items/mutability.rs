// Mutability in Rust
//
// Understanding when and why to use mutable variables

fn main() {
    // Immutable by default prevents accidental changes
    let robot_name = "Ferris";
    println!("Robot name: {}", robot_name);
    
    // When you need to change data, explicitly use 'mut'
    let mut robot_energy = 100;
    println!("Starting energy: {}", robot_energy);
    
    // Simulate robot using energy
    robot_energy -= 25;
    println!("Energy after scan: {}", robot_energy);
    
    robot_energy -= 15;
    println!("Energy after move: {}", robot_energy);
    
    // Constants are always immutable and must have type annotations
    const MAX_ENERGY: u32 = 100;
    println!("Maximum possible energy: {}", MAX_ENERGY);
    
    // Mutability helps with performance and safety
    // - Prevents accidental modifications
    // - Makes data flow explicit
    // - Enables compiler optimizations
}

// Rule of thumb: Start with immutable, add 'mut' only when needed!