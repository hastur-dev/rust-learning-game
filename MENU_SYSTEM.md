# Start Menu System

The game now features a comprehensive start menu system with the banner "Welcome to Robo Wars Crab Edition"!

## Main Menu Features

### 4 Main Buttons:

1. **Normal Start** - Start the game with built-in levels or automatically detected YAML levels
2. **Settings** - Configure game settings (currently displays current settings)
3. **Player Levels** - Browse and load custom YAML levels from the `levels/` directory
4. **Exit** - Exit the game

## Settings Menu

The settings menu displays current configuration:
- Window resolution
- Fullscreen mode status
- SFX and Music volume levels
- Back to main menu option

*Note: Settings are currently read-only for display. Future versions may include interactive settings modification.*

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
- Press **M** to return to the main menu at any time
- All other controls remain the same (B for shop, N for next level, etc.)

## Visual Features

- **Animated background** with grid pattern and decorative crab elements
- **Professional UI** with proper button highlighting and text centering
- **Theme consistency** with the existing game aesthetic
- **Responsive design** that adapts to different screen sizes

## Technical Details

The menu system is built with:
- Modular design in `src/menu.rs`
- State management integrated with the game state
- Real-time file detection for custom levels
- Mouse and keyboard input handling
- Smooth transitions between menu states

## Usage Tips

1. **Custom Levels**: Place your `.yaml` level files in the `levels/` directory before starting the game
2. **Level Testing**: Use the Player Levels menu to quickly test different custom levels
3. **Quick Access**: The M key provides instant access back to the menu during gameplay
4. **File Changes**: The Refresh button in Player Levels will detect newly added level files

The menu system provides a polished, user-friendly way to access all game features while maintaining the technical depth of the robot programming gameplay!