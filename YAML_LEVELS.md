# YAML Level Configuration System

This game now supports YAML-based level configuration! You can create custom levels with specific grid sizes, obstacles, enemies, items, and more.

## YAML Level Format

Create `.yaml` or `.yml` files in the `levels/` directory with the following structure:

```yaml
name: "Your Level Name"
grid_size: "WxH"                    # e.g., "16x10" for 16 wide by 10 high
obstacles: 5                        # Number of random obstacles (optional)
start_position: [x, y]             # Starting position (optional, defaults to [1, 1])
fog_of_war: true                    # Enable fog of war (optional, defaults to true)
max_turns: 100                      # Maximum turns allowed (optional, 0 = unlimited)
income_per_square: 2                # Credits earned per square revealed (optional, defaults to 1)

enemies:                            # Optional list of enemies
  - start_location: [x, y]         # Enemy starting position
    movement_pattern: "horizontal"  # "horizontal" or "vertical"
    moving_positive: true           # true = right/down, false = left/up

items:                              # Optional list of items
  - name: "scanner"                 # Item name
    item_file: "items/scanner.rs"   # Path to item capability file
    spawn_randomly: false           # If true, spawns at random location
    location: [x, y]                # Specific location (if spawn_randomly is false)
```

## Example Levels

See the `levels/` directory for example YAML files:

- `basic_exploration.yaml` - Simple level with random obstacles and items
- `enemy_encounter.yaml` - Level with moving enemies and strategic item placement
- `treasure_hunt.yaml` - Large level with valuable treasures

## Item System

Items can reference Rust files in the `items/` directory that define their capabilities:

### Item Capability Format

Create `.rs` files in the `items/` directory with capability comments:

```rust
// CAPABILITY: scanner_range = 1
// CAPABILITY: grabber_boost = 2
// CAPABILITY: credits_value = 10

// Your item's Rust code here
pub fn item_function() {
    // Item-specific functionality
}
```

### Supported Capabilities

- `scanner_range` - Range of scanner functionality
- `grabber_boost` - Boost to grabber range
- `credits_value` - Credit value when collected

## Usage

1. Create your YAML level files in the `levels/` directory
2. Create corresponding item files in the `items/` directory (if using custom items)
3. Run the game - it will automatically load YAML levels if found, otherwise fall back to built-in levels

## Level Loading Priority

The game loads levels in this order:
1. YAML files from `levels/` directory
2. If no YAML levels found, loads built-in levels

This allows for easy development and distribution of custom level packs!