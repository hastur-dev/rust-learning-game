// MOVEMENT_PATTERN: horizontal
// This movement pattern makes enemies move left and right in a horizontal line
//
// To use this pattern in your YAML file:
// enemies:
//   - start_position: [3, 6]
//     movement_pattern: "file:movement_patterns/horizontal_patrol.rs"
//
// The actual horizontal movement implementation is built into the game engine.
// When you reference this file in YAML, the system will automatically
// create a horizontal movement pattern for the enemy.
//
// Horizontal movement behavior:
// - Enemy moves right until hitting a wall or boundary
// - When blocked rightward, switches to moving left
// - When blocked leftward, switches to moving right
// - Creates a predictable left-right patrol pattern
// - If blocked on both sides, tries to move vertically to find new horizontal path
//
// This creates enemies that patrol horizontally, perfect for creating
// timing challenges in async programming scenarios where the robot
// needs to coordinate movement with enemy patrol timing.