// MOVEMENT_PATTERN: vertical
// This movement pattern makes enemies move up and down in a vertical line
//
// To use this pattern in your YAML file:
// enemies:
//   - start_position: [5, 3]
//     movement_pattern: "file:movement_patterns/vertical_patrol.rs"
//
// The actual vertical movement implementation is built into the game engine.
// When you reference this file in YAML, the system will automatically
// create a vertical movement pattern for the enemy.
//
// Vertical movement behavior:
// - Enemy moves up until hitting a wall or boundary
// - When blocked upward, switches to moving down
// - When blocked downward, switches to moving up
// - Creates a predictable up-down patrol pattern
// - If blocked on both sides, tries to move horizontally to find new vertical path
//
// This creates enemies that patrol vertically, making them predictable
// but still challenging to navigate around in async learning scenarios.