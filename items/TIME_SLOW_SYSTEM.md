# Time Slow System

The game now includes a **Time Slow** item that introduces delays between robot actions, allowing you to watch your code execution in slow motion!

## How It Works

### **Time Slow Item**
- **Item Name**: `time_slow`
- **Default Duration**: 500ms delay between each action
- **Credit Value**: 25 credits when collected
- **Visual Indicator**: Golden "!" symbol on the grid

### **Activation**
When you collect a time_slow item:
1. **Time Slow Mode** activates automatically
2. **Visual feedback** appears in the UI showing the active state
3. **All subsequent code execution** will have delays between actions

### **Visual Feedback**
- **Status Display**: Shows "Time Slow: XXXms" in the upgrades line
- **Active Indicator**: Yellow "TIME SLOW ACTIVE" box in the top-right corner
- **Real-time Updates**: UI updates immediately when the item is collected

## Item Configuration

### **YAML Level Setup**
Add time_slow items to your custom levels:

```yaml
items:
  - name: "time_slow"
    item_file: "items/time_slow.rs"
    spawn_randomly: false
    location: [5, 4]
```

### **Custom Time Slow Items**
Create different time slow variations by modifying `items/time_slow.rs`:

```rust
// CAPABILITY: time_slow_duration = 500    # 500ms delay (default)
// CAPABILITY: time_slow_duration = 1000   # 1 second delay (very slow)
// CAPABILITY: time_slow_duration = 100    # 100ms delay (fast slow)
// CAPABILITY: credits_value = 25
```

## Benefits

### **Code Visualization**
- **Watch robot movement** step by step
- **Debug complex algorithms** by seeing each action
- **Educational value** for understanding program flow
- **Better appreciation** of robot behavior

### **Strategic Gameplay**
- **High-value item** (25 credits)
- **Permanent effect** once collected
- **Helps with precision** in complex levels
- **Makes enemy movement** easier to track

## Technical Implementation

### **Async Execution**
- Uses Rust's `async/await` for non-blocking delays
- Frame-based timing for smooth gameplay
- Maintains 60 FPS while adding delays between actions

### **Code Integration**
The time slow affects all robot functions:
- `move()` commands
- `grab()` actions  
- `scan()` operations
- `search_all()` execution

### **Performance**
- **Zero overhead** when not active
- **Minimal impact** on game performance
- **Smooth delays** without blocking the UI

## Example Levels

### **Time Slow Demo Level**
- **File**: `levels/time_slow_demo.yaml`
- **Purpose**: Showcase the time slow feature
- **Contains**: time_slow item at position [3,3]

### **Updated Levels**
- `basic_exploration.yaml` - includes time_slow at [5,4]
- `treasure_hunt.yaml` - includes time_slow at [15,8]

## Usage Tips

1. **Collect Early**: Time slow is most useful when collected before writing complex code
2. **Debug Tool**: Use it to debug algorithms that aren't working as expected
3. **Learning Aid**: Great for understanding how `search_all()` works
4. **Enemy Levels**: Especially helpful in levels with moving enemies

The time slow system transforms the instant robot execution into a visual, step-by-step experience that's both educational and strategically valuable!