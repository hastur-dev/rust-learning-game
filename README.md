# 🤖 Robo Grid Explorer GUI

A Rust-based grid exploration and robot programming game where you write Rust code to control a robot's movement and actions. Navigate through levels, collect items, avoid enemies, and learn programming concepts in an interactive environment.

![Version](https://img.shields.io/badge/version-0.2.0-blue)
![Rust](https://img.shields.io/badge/rust-2024-orange)
![License](https://img.shields.io/badge/license-open--source-green)

## 🎮 Game Overview

**Robo Grid Explorer** is an educational programming game where you control a robot by writing actual Rust code. The robot explores grid-based levels, collects items, avoids moving enemies, and completes objectives through your programming commands.

### 🌟 Key Features

- **Visual Programming Interface**: Write Rust code in an integrated editor with syntax highlighting
- **Real-time Execution**: Watch your code execute step-by-step on the game grid
- **Progressive Difficulty**: 22+ levels introducing new mechanics gradually
- **YAML Level System**: Create custom levels with flexible configuration
- **Enemy AI**: Multiple movement patterns including custom scriptable behaviors
- **Item Collection**: Scanners, grabbers, speed boosts, and special upgrades
- **Shop System**: Spend credits to upgrade your robot's capabilities
- **Menu System**: Professional start menu with settings and level selection

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (2021 edition or later)
- Windows, macOS, or Linux

### Installation & Running

```bash
# Clone the repository
git clone <repository-url>
cd rust-steam-game

# Run the game
cargo run --release
```

The game will automatically load custom YAML levels from the `levels/` directory, or fall back to built-in levels if none are found.

## 🕹️ How to Play

### Controls

| Key | Action |
|-----|--------|
| **Click Code Editor** | Edit robot code |
| **SHIFT+CTRL+ENTER** | Execute robot code |
| **SHIFT+CTRL+E** | Open code in external IDE |
| **SHIFT+CTRL+B** | Open upgrade shop |
| **SHIFT+CTRL+N** | Next level (when completed) |
| **SHIFT+CTRL+L** | Reload current level |
| **SHIFT+CTRL+M** | Return to main menu |
| **SHIFT+CTRL+R** | Reset code to default |

### Programming Your Robot

Write Rust code using these available functions:

```rust
// Movement (available from Level 1)
move(up);      // Move robot up
move(down);    // Move robot down
move(left);    // Move robot left
move(right);   // Move robot right

// Item collection (available from Level 2)
grab();        // Collect items and reveal tiles in grabber range

// Scanning (available from Level 3)
scan(up);      // Scan in direction to reveal distant tiles
scan(down);
scan(left);
scan(right);

// Advanced functions
search_all();         // Automated lawnmower pattern exploration
set_auto_grab(true);  // Automatically grab items when moving
set_auto_grab(false); // Disable auto-grab
```

### Example Robot Programs

**Basic Movement:**
```rust
// Move in a square pattern
move(right);
move(down);
move(left);
move(up);
```

**Complete Exploration:**
```rust
// Automatically explore all reachable areas
search_all();
grab(); // Collect any remaining items
```

**Strategic Scanning:**
```rust
// Use scanner to reveal areas safely
scan(right);
scan(down);
move(right);
grab();
```

## 🎯 Game Mechanics

### Levels & Progression

- **Progressive Learning**: Start with simple movement, advance to complex enemy avoidance
- **22+ Built-in Levels**: Carefully designed progression from tutorial to advanced challenges
- **Custom Levels**: Create your own levels using YAML configuration
- **Level Messages**: Helpful popups explain new mechanics and objectives

### Robot Upgrades

Spend credits earned by exploring to upgrade your robot:

- **Grabber Range**: Increase the area you can collect items from
- **Scanner Length**: Extend how far you can scan ahead
- **Special Items**: Find speed boosts, time slows, and other power-ups

### Enemy System

Starting from Level 4, enemies patrol the grid:

- **Movement Patterns**: Horizontal, vertical, diagonal, circular, random, and custom patterns
- **Collision Detection**: Getting caught resets the level with a new random layout
- **Strategic Planning**: Study enemy behavior to plan safe routes

### Item Collection

- **Credits**: Primary currency for upgrades
- **Scanners**: Reveal distant tiles without moving
- **Grabber Upgrades**: Increase collection range
- **Time Slow**: Slow down game execution for precise control
- **Special Gems**: High-value collectibles

## 🛠️ Custom Content Creation

### YAML Level Configuration

Create custom levels by adding `.yaml` files to the `levels/` directory:

```yaml
name: "Custom Adventure"
grid_size: "20x15"           # Width x Height
obstacles: 8                 # Random obstacle count
start_position: [1, 1]       # Robot starting position
max_turns: 200              # Turn limit (0 = unlimited)
income_per_square: 2        # Credits per revealed tile
message: "Welcome to your custom level! Explore carefully."

enemies:
  - start_location: [18, 10]
    movement_pattern: "horizontal"
    moving_positive: true    # true = right/down, false = left/up
  - start_location: [8, 5]
    movement_pattern: "file:movement_patterns/spiral_movement.rs"

items:
  - name: "scanner"
    item_file: "items/scanner.rs"
    location: [15, 12]
  - name: "speed_boost"
    item_file: "items/speed_boost.rs"
    spawn_randomly: true
```

### Custom Movement Patterns

Create custom enemy AI by adding `.rs` files to `movement_patterns/`:

```rust
// MOVEMENT_PATTERN: spiral_outward

pub fn update_position(current_pos: (i32, i32), state: &mut MovementState) -> (i32, i32) {
    // Custom movement logic here
    // Return new position
}
```

### Custom Items

Define item capabilities in `items/` directory:

```rust
// CAPABILITY: credits_value = 50
// CAPABILITY: grabber_boost = 2
// CAPABILITY: time_slow_duration = 2000

pub fn special_ability() {
    // Item-specific functionality
}
```

## 🎮 Menu System

### Main Menu

- **Normal Start**: Begin with automatic level detection
- **Settings**: Configure resolution, audio, and display options
- **Player Levels**: Browse and select custom YAML levels
- **Exit**: Close the game

### Settings Menu

Interactive configuration with click controls:
- **Resolution**: 720p to 4K presets
- **Fullscreen**: Toggle fullscreen mode
- **Audio**: Separate volume controls for SFX and music

### Player Levels Menu

- **Auto-detection**: Automatically finds all YAML levels in `levels/`
- **Scrollable List**: Navigate through many custom levels
- **Quick Play**: Click any level to start immediately
- **Refresh**: Reload level list after adding new files

## 🏗️ Technical Architecture

### Dependencies

- **macroquad**: Cross-platform game framework
- **serde/serde_yaml**: YAML level configuration
- **rand**: Random generation for level layouts
- **notify**: File watching for hot-reload (desktop only)

### Project Structure

```
src/
├── main.rs              # Desktop entry point and game loop
├── lib.rs               # WASM entry point
├── game_state.rs        # Core game state management
├── grid.rs              # Grid system and enemy management
├── robot.rs             # Robot state and capabilities
├── level.rs             # YAML level loading and parsing
├── item.rs              # Item system and inventory
├── menu.rs              # Menu system and UI
├── movement_patterns.rs # Enemy AI patterns
└── popup.rs             # Popup message system

levels/                  # YAML level definitions
├── 01_explore_grid.yaml
├── 02_find_scanner.yaml
├── 03_blockers.yaml
└── ...

movement_patterns/       # Custom enemy AI
├── spiral_movement.rs
├── chase_player.rs
└── guard_area.rs

items/                   # Item capability definitions
├── scanner.rs
├── grabber_upgrade.rs
├── speed_boost.rs
└── ...
```

### Platform Support

- **Desktop**: Full-featured version with file I/O and hot-reload
- **Web (WASM)**: ⚠️ Currently not working - will be fixed in a future update

## 🎓 Educational Value

This game teaches:

- **Rust Programming**: Syntax, functions, and control structures
- **Algorithmic Thinking**: Path planning and optimization
- **Game Development**: State management and event handling
- **Problem Solving**: Strategic thinking under constraints
- **Code Organization**: Modular programming concepts

Perfect for:
- Rust beginners learning the language
- Programming students practicing algorithms
- Game development enthusiasts
- Anyone interested in interactive coding

## 🔧 Development

### Building from Source

```bash
# Debug build
cargo build

# Release build (recommended for gameplay)
cargo build --release

# Run with full logging
cargo run --release -- --all-logs
```

### Adding New Features

1. **New Robot Functions**: Add to `RustFunction` enum and parsing logic
2. **Custom Items**: Create capability files in `items/` directory
3. **Enemy Patterns**: Add movement scripts to `movement_patterns/`
4. **Level Mechanics**: Extend YAML configuration options

### File Watching

The desktop version supports hot-reload of `robot_code.rs` - edit the file externally and changes are automatically detected.

## 🐛 Troubleshooting

### Common Issues

**Game won't start:**
- Ensure Rust toolchain is installed and up-to-date
- Try `cargo clean && cargo build --release`
- Check that graphics drivers support OpenGL

**Levels not loading:**
- Verify YAML syntax in custom level files
- Check that referenced item/movement files exist
- Look for error messages in the console

**Code editor not responding:**
- Click in the code editor area to focus it
- Ensure you're using supported function names
- Check for syntax errors in your robot code

**Performance issues:**
- Use `cargo run --release` for better performance
- Close other graphics-intensive applications
- Lower resolution in settings menu

## 🤝 Contributing

Contributions welcome! Areas for improvement:

- Additional built-in levels
- New enemy movement patterns
- More item types and capabilities
- UI/UX enhancements
- WASM version fixes
- Documentation improvements

## 📄 License

This project is open source. Feel free to use, modify, and distribute according to the license terms.

---

**Start your robot programming adventure today! 🤖✨**

*Learn Rust while having fun exploring grids, avoiding enemies, and solving programming puzzles.*