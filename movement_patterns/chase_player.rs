// MOVEMENT_PATTERN: chase
// This movement pattern makes enemies chase the player using simple pathfinding
// 
// To use this pattern in your YAML file:
// enemies:
//   - start_position: [15, 8]
//     movement_pattern: "file:movement_patterns/chase_player.rs"
//
// The actual chase movement implementation is built into the game engine.
// When you reference this file in YAML, the system will automatically
// create a chase movement pattern for the enemy.
//
// Chase movement behavior:
// - Enemy calculates the direction toward the player's current position
// - Tries to move directly toward the player (prefers horizontal movement)
// - If the direct path is blocked, tries alternative routes
// - If all direct paths are blocked, moves randomly to avoid getting stuck
//
// This creates a simple but effective enemy AI that will pursue the player
// around the game grid, making the game more challenging and dynamic.