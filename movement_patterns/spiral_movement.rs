// MOVEMENT_PATTERN: spiral
// This is an example custom movement pattern that makes enemies move in a spiral pattern
// 
// To use this pattern in your YAML file, reference it like this:
// enemies:
//   - start_position: [10, 10]
//     movement_pattern: "file:movement_patterns/spiral_movement.rs"
//
// Custom movement patterns can maintain state using the movement_data HashMap
// that is passed to the next_move function. This allows for complex behaviors
// that depend on the enemy's history or current state.
//
// The actual spiral movement implementation is built into the game engine.
// When you reference this file in YAML, the system will automatically
// create a spiral movement pattern for the enemy.
//
// Spiral movement behavior:
// - Enemy starts moving right
// - After a certain number of steps, turns clockwise (right -> down -> left -> up)
// - Each time it completes a full rotation, the spiral expands (more steps in each direction)
// - If blocked, tries to continue the spiral in the next direction
//
// This creates an expanding spiral pattern around the enemy's starting position.