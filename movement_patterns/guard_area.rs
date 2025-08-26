// MOVEMENT_PATTERN: guard
// This movement pattern makes enemies patrol a specific area around their starting position
// 
// To use this pattern in your YAML file:
// enemies:
//   - start_position: [10, 5]
//     movement_pattern: "file:movement_patterns/guard_area.rs"
//
// The actual guard movement implementation is built into the game engine.
// When you reference this file in YAML, the system will automatically
// create a guard movement pattern for the enemy.
//
// Guard movement behavior:
// - Enemy remembers its starting position as the center of its patrol area
// - Patrols in a rectangular pattern around the starting position
// - Has a limited patrol radius (typically 3 squares from center)
// - If it moves too far from center, it will return toward the center
// - Changes direction periodically to create varied patrol patterns
// - If blocked, tries alternative directions but stays within patrol area
//
// This creates enemies that act like guards, staying in a specific area
// rather than wandering the entire map. Good for protecting specific
// locations or creating predictable enemy patterns.