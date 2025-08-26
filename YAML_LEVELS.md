# YAML Level Configuration System

This game supports YAML-based level configuration! You can create custom levels with specific grid sizes, obstacles, enemies, items, and more.

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
message: "Welcome to the level!"    # Popup message shown when level starts (optional)

enemies:                            # Optional list of enemies
  - start_location: [x, y]         # Enemy starting position
    movement_pattern: "horizontal"  # Built-in: "horizontal", "vertical", "random", "diagonal", "circular"
                                    # Custom: "file:movement_patterns/pattern_name.rs"
    moving_positive: true           # true = right/down, false = left/up (for horizontal/vertical)

items:                              # Optional list of items
  - name: "scanner"                 # Item name
    item_file: "items/scanner.rs"   # Path to item capability file
    spawn_randomly: false           # If true, spawns at random location
    location: [x, y]                # Specific location (if spawn_randomly is false)
```

### Complete Example with Message

```yaml
name: "Enemy Patrol Tutorial"
grid_size: "18x12"
obstacles: 6
start_position: [1, 1]
max_turns: 150
message: "New mechanic: Enemies! The red squares are patrolling enemies. If they catch you, the level resets. Watch their movement patterns and plan your path carefully."

enemies:
  - start_location: [15, 8]
    movement_pattern: "horizontal"
    moving_positive: true
  - start_location: [8, 3]
    movement_pattern: "vertical" 
    moving_positive: false
  - start_location: [12, 9]
    movement_pattern: "random"

items:
  - name: "scanner"
    item_file: "items/scanner.rs"
    spawn_randomly: false
    location: [16, 2]
```

## Popup Message System

You can add informative popup messages that appear when players start a level using the `message` field:

```yaml
name: "Tutorial Level"
grid_size: "12x8"
message: "Welcome to your first level! Use WASD or arrow keys to move around and explore the grid. Find all hidden areas to complete the level."
```

### Message Guidelines

- **Keep it concise**: Aim for 1-3 sentences that fit comfortably on screen
- **Be informative**: Explain level objectives, new mechanics, or provide helpful hints
- **Use proper punctuation**: Messages are displayed exactly as written
- **Consider difficulty**: Provide more guidance for complex levels

### Example Messages

```yaml
# Tutorial level
message: "Welcome! Move with WASD keys and explore all the hidden tiles to win."

# Enemy introduction level  
message: "Danger ahead! Red enemies patrol this area. If they catch you, the level will reset. Plan your moves carefully."

# Advanced mechanics level
message: "This level features special movement patterns. Study the enemy behavior - some move randomly, others in patterns. Use the scanner to reveal distant areas."

# Boss level
message: "Final Challenge! Multiple enemies with different behaviors guard valuable treasures. You'll need all your skills and upgrades to succeed."

# Puzzle level
message: "Strategic thinking required! The solution isn't obvious - sometimes you need to let enemies chase you to create safe paths."
```

### Popup Behavior

- **Manual Dismiss**: Level start messages require manual dismissal (press SPACE, ENTER, ESC, or click outside)
- **Auto-close**: Other popups like item collection auto-close after a few seconds
- **Input Blocking**: While a popup is showing, all game input is blocked to prevent accidents
- **Visual Design**: Popups use color coding (blue for info, green for success, purple for tutorials)

### Other Automatic Popups

Beyond custom level messages, the game automatically shows popups for:

- **Item Collection**: "You found: [item name]" (auto-closes in 3 seconds)
- **Level Complete**: "Great job! Press SPACE to continue to the next level."
- **Shop Tutorial**: Detailed tutorial shown when first accessing the shop
- **Game Events**: Various contextual messages during gameplay

## Enemy Color System

Enemies are color-coded based on their movement patterns to help players quickly identify different threats:

### Movement Pattern Colors

- **🟢 GREEN**: Horizontal movement enemies (built-in pattern)
- **🔵 DARKBLUE**: Vertical movement enemies (built-in pattern) 
- **🟠 ORANGE**: Chase enemies actively pursuing the player
- **🔵 BLUE**: Chase enemies that are stuck/not moving toward player
- **🟣 MAGENTA**: Random movement enemies
- **🟡 YELLOW**: Diagonal movement enemies
- **🟢 LIME**: Circular movement enemies
- **🩷 PINK**: Spiral movement enemies
- **🟣 PURPLE**: Custom movement patterns loaded from files
- **🔴 RED**: Unknown or default movement patterns

### Visual Enemy Identification

```yaml
enemies:
  - start_location: [10, 5]
    movement_pattern: "horizontal"  # Will appear GREEN
  - start_location: [15, 8] 
    movement_pattern: "chase"       # Will appear ORANGE (chasing) or BLUE (stuck)
  - start_location: [8, 12]
    movement_pattern: "random"      # Will appear MAGENTA
```

This color system makes it easy to understand enemy behavior at a glance and plan your strategy accordingly!

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

## Custom Movement Patterns

You can create custom enemy movement patterns by writing Rust files in the `movement_patterns/` directory:

### Creating a Custom Movement Pattern

1. Create a `.rs` file in `movement_patterns/`
2. Add a comment at the top: `// MOVEMENT_PATTERN: pattern_name`
3. Implement your movement logic (see examples in `movement_patterns/`)

### Built-in Movement Patterns

- `"horizontal"` - Moves left/right, reversing at obstacles
- `"vertical"` - Moves up/down, reversing at obstacles  
- `"random"` - Moves randomly in any direction
- `"diagonal"` - Moves diagonally, reversing at obstacles
- `"circular"` - Moves in a circular pattern (right, down, left, up)

### Example Custom Movement Patterns

See the `movement_patterns/` directory for examples:
- `spiral_movement.rs` - Creates an expanding spiral pattern
- `chase_player.rs` - Enemy chases the player
- `guard_area.rs` - Enemy patrols around starting area

### Using Custom Patterns in YAML

```yaml
enemies:
  - start_location: [10, 5]
    movement_pattern: "file:movement_patterns/spiral_movement.rs"
  - start_location: [15, 8]
    movement_pattern: "file:movement_patterns/guard_area.rs"
  - start_location: [5, 5]
    movement_pattern: "random"  # Built-in pattern
```

## Usage

1. Create your YAML level files in the `levels/` directory
2. Create corresponding item files in the `items/` directory (if using custom items)
3. Create custom movement patterns in the `movement_patterns/` directory (optional)
4. Run the game - it will automatically load YAML levels if found, otherwise fall back to built-in levels

## Level Loading Priority

The game loads levels in this order:
1. YAML files from `levels/` directory
2. If no YAML levels found, loads built-in levels

This allows for easy development and distribution of custom level packs!