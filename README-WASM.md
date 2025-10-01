# 🎮 Robo Grid Explorer GUI - WebAssembly Edition

A Rust-based robot programming game that runs both as a desktop application and in web browsers via WebAssembly (WASM). Write actual Rust code to control your robot!

## 🌐 Web Version (WASM)

### Prerequisites

1. **Rust toolchain** - Install from [rustup.rs](https://rustup.rs/)
2. **wasm-pack** - Install with:
   ```bash
   cargo install wasm-pack
   ```
   Or download from: https://rustwasm.github.io/wasm-pack/installer/

### Building for Web

#### Option 1: Using the build script (Windows)
```bash
.\build-wasm.bat
```

#### Option 2: Using the build script (Linux/Mac)
```bash
./build-wasm.sh
```

#### Option 3: Manual build
```bash
# Build the WASM package
wasm-pack build --target web --out-dir pkg --dev

# Create distribution directory
mkdir dist
cp index.html dist/
cp -r pkg dist/
```

### Running the Web Version

1. **Start a local web server** in the `dist` directory:
   ```bash
   cd dist
   python -m http.server 8000
   ```
   
2. **Open your browser** and navigate to:
   ```
   http://localhost:8000
   ```

3. **Enjoy the game!** Write Rust code in the integrated editor to control your robot's movement and actions.

### Alternative Web Servers

If you don't have Python, you can use other web servers:

- **Node.js**: `npx serve dist`
- **PHP**: `php -S localhost:8000 -t dist`
- **Live Server** (VS Code extension)
- Any other local web server

## 🖥️ Desktop Version

To run the desktop version:

```bash
cargo run
```

## 🎯 Game Features

### Core Gameplay
- **Programming-based control**: Write actual Rust code to control your robot
- **Grid-based exploration** with fog of war mechanics
- **Visual code execution**: Watch your code run step-by-step
- **Item collection** system with various power-ups
- **Enemy avoidance** mechanics with multiple AI patterns
- **Multiple levels** with YAML configuration
- **Shop system** for upgrades and enhancements
- **Time slow mechanic** for precise control

### Movement Patterns
The game supports various enemy movement patterns:

- **Built-in patterns**: `horizontal`, `vertical`, `random`, `diagonal`, `circular`
- **Custom patterns**: Reference files in `movement_patterns/` directory

### YAML Level Configuration
Create custom levels using YAML files in the `levels/` directory:

```yaml
name: "My Custom Level"
grid_size: "20x15"
obstacles: 8
start_position: [1, 1]
fog_of_war: true
max_turns: 0
income_per_square: 1

enemies:
  - start_location: [18, 13]
    movement_pattern: "random"
  - start_location: [5, 10]
    movement_pattern: "file:movement_patterns/spiral_movement.rs"

items: []
```

## 🎮 Controls

| Key | Action |
|-----|--------|
| **Click Code Editor** | Edit robot code |
| **SHIFT+CTRL+ENTER** | Execute robot code |
| **SHIFT+CTRL+E** | Open code in external IDE (desktop only) |
| **SHIFT+CTRL+B** | Open upgrade shop |
| **SHIFT+CTRL+N** | Next level (when completed) |
| **SHIFT+CTRL+L** | Reload current level |
| **SHIFT+CTRL+M** | Return to main menu |
| **SHIFT+CTRL+R** | Reset code to default |

## 🚀 Technical Details

### Architecture
- **Engine**: Macroquad (Rust game framework)
- **WASM**: wasm-bindgen for web integration
- **Build**: Conditional compilation for desktop vs web
- **Levels**: YAML configuration system
- **Movement**: Trait-based movement patterns

### File Structure
```
src/
├── lib.rs              # WASM entry point
├── main.rs             # Desktop entry point
├── game_state.rs       # Game logic and state management
├── grid.rs             # Grid and enemy management
├── robot.rs            # Robot state and capabilities
├── level.rs            # Level loading and parsing
├── item.rs             # Item system and inventory
├── menu.rs             # Menu system and UI
├── movement_patterns.rs # Enemy AI patterns
├── popup.rs            # Popup message system
└── ...

levels/                 # YAML level definitions
movement_patterns/      # Custom movement patterns
pkg/                    # Generated WASM output
dist/                   # Web distribution files
```

### Platform Differences

| Feature | Desktop | Web |
|---------|---------|-----|
| File I/O | ✅ Full filesystem access | ❌ Embedded levels only |
| Hot reload | ✅ File watching for robot_code.rs | ❌ Not available |
| External IDE | ✅ Open code in system editor | ❌ Browser security restrictions |
| Custom levels | ✅ Load from levels/ directory | ⚠️ Must be embedded at build time |
| Performance | ✅ Native speed | ⚠️ Slight overhead |
| Distribution | 📦 Single executable | 🌐 Web-ready bundle |

## 🔧 Development

### Adding New Levels
1. Create a `.yaml` file in the `levels/` directory
2. For web builds, add the level to the `get_embedded_levels()` function in `lib.rs`
3. Update the `levels/order.txt` file to specify level ordering

### Custom Movement Patterns
1. Create a `.rs` file in `movement_patterns/`
2. Add the pattern marker comment: `// MOVEMENT_PATTERN: pattern_name`
3. Reference it in YAML: `movement_pattern: "file:movement_patterns/your_pattern.rs"`

### Building for Production
For optimized production builds:
```bash
wasm-pack build --target web --out-dir pkg --release
```

## 🐛 Troubleshooting

### Common Issues

1. **WASM build fails**
   - Ensure `wasm-pack` is installed
   - Check Rust toolchain is up to date
   - Try `cargo clean` and rebuild

2. **Game doesn't load in browser**
   - Check browser console for errors
   - Ensure you're using a local web server (not `file://`)
   - Verify WebAssembly support in your browser

3. **Controls don't work**
   - Click on the game canvas to focus it
   - Check that JavaScript is enabled

### Browser Compatibility
- **Chrome/Chromium**: Full support ✅
- **Firefox**: Full support ✅
- **Safari**: Full support (recent versions) ✅
- **Edge**: Full support ✅
- **Mobile browsers**: Limited support ⚠️ (no keyboard input)

## 🎓 Educational Value

The web version is perfect for:
- **Learning Rust** in an interactive environment
- **Teaching programming** concepts in classrooms
- **Code sharing** - easily share your solutions via URL
- **Cross-platform accessibility** - no installation required

## 📜 License

This project is open source. See the main README for license details.

---

**Learn Rust while playing in your browser! 🎮🦀**