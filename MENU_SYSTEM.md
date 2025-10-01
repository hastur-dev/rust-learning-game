# 🎮 Menu System Documentation

**Robo Grid Explorer GUI** features a comprehensive menu system with intuitive navigation and professional UI design for easy access to all game features.

## Main Menu Features

### 4 Main Buttons:

1. **Normal Start** - Start the game with built-in levels or automatically detected YAML levels
2. **Settings** - Configure game settings (currently displays current settings)
3. **Player Levels** - Browse and load custom YAML levels from the `levels/` directory
4. **Exit** - Exit the game

## Settings Menu

The settings menu provides **fully interactive** configuration options:

### **Interactive Controls:**
- **Resolution**: Click through 6 preset resolutions (720p to 4K)
  - **Left Click**: Next resolution
  - **Right Click**: Previous resolution
  - Available: 1280x720, 1366x768, 1600x900, 1920x1080, 2560x1440, 3840x2160

- **Fullscreen**: Toggle fullscreen mode
  - **Click**: Toggle on/off

- **SFX Volume**: Audio effects volume (0-100%)
  - **Left Click**: +10% volume
  - **Right Click**: -10% volume

- **Music Volume**: Background music volume (0-100%)
  - **Left Click**: +10% volume
  - **Right Click**: -10% volume

### **Clear Instructions:**
- On-screen prompts show "Left Click: Increase/Next | Right Click: Decrease/Previous"
- Each button displays current value and available actions
- Immediate visual feedback when settings change

## Player Levels Menu

Features:
- **Auto-detection** of all `.yaml` and `.yml` files in the `levels/` directory
- **Scrollable list** if you have many custom levels
- **Refresh button** to reload the level list
- **Click to play** - select any custom level to start playing immediately
- **Back to main** - return to the main menu

### Navigation:
- **Mouse wheel** - scroll through the level list
- **Click** - select a level to play
- **Escape** - return to main menu
- **Refresh** - reload level files from disk

## In-Game Menu Access

While playing the game, you can:
- Press **SHIFT+CTRL+M** to return to the main menu at any time
- Press **SHIFT+CTRL+B** to open the upgrade shop
- Press **SHIFT+CTRL+N** to proceed to the next level (when completed)
- Press **SHIFT+CTRL+L** to reload the current level
- Press **SHIFT+CTRL+R** to reset code to default

## Visual Features

- **Animated background** with grid pattern and robot/technology themed elements
- **Professional UI** with proper button highlighting and text centering
- **Theme consistency** with the robot programming aesthetic
- **Responsive design** that adapts to different screen sizes
- **Visual feedback** for all interactive elements
- **Color coding** for different menu states and options

## Technical Details

The menu system is built with:
- Modular design in `src/menu.rs`
- State management integrated with the game state
- Real-time file detection for custom levels
- Mouse and keyboard input handling
- Smooth transitions between menu states
- Settings persistence (planned feature)
- Resolution management for different displays

## Usage Tips

1. **Custom Levels**: Place your `.yaml` level files in the `levels/` directory before starting the game
2. **Level Testing**: Use the Player Levels menu to quickly test different custom levels
3. **Quick Access**: SHIFT+CTRL+M provides instant access back to the menu during gameplay
4. **File Changes**: The Refresh button in Player Levels will detect newly added level files
5. **Settings**: Adjust resolution and audio settings to match your preferences
6. **Level Order**: Customize level progression by editing `levels/order.txt`

## Integration with Game Features

### Shop System Access
- Access the upgrade shop from the menu or in-game with SHIFT+CTRL+B
- Purchase grabber range upgrades, scanner length extensions, and special items
- View your current credit balance and upgrade status

### Level Progression
- Track completed levels and unlock new challenges
- Quick restart for failed attempts
- Level selection for replay and practice

### Code Editor Integration
- Menu provides context for the current coding challenge
- Quick reset to default code if stuck
- External IDE support (desktop version)

The menu system provides a polished, user-friendly interface that seamlessly connects all game features while maintaining focus on the educational robot programming experience!