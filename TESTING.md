# 🧪 Testing Documentation

**Robo Grid Explorer GUI** testing guide for developers and contributors. This document covers how to run tests, what they validate, and how to add new test cases.

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [Test Commands](#test-commands)
- [Testing Strategies](#testing-strategies)
- [Manual Testing](#manual-testing)
- [Automated Testing](#automated-testing)
- [Performance Testing](#performance-testing)
- [Contributing Tests](#contributing-tests)

## 🚀 Quick Start

### Running All Tests
```bash
# Run all tests (unit, integration, doc tests)
cargo test

# Run tests in release mode (faster execution)
cargo test --release

# Run tests with output displayed
cargo test -- --nocapture

# Run specific test by name
cargo test test_name_here
```

### Running the Game with Test Flags
```bash
# Run with test learning levels
cargo run --release --bin robo_grid_explorer_gui -- --test-learning-levels

# Run with verbose logging for debugging
cargo run --release -- --all-logs

# Run with specific test level
cargo run --release
```

## 🎯 Test Commands

### Unit Tests
```bash
# Run only library unit tests
cargo test --lib

# Run only binary unit tests
cargo test --bin robo_grid_explorer_gui

# Run tests for specific module
cargo test grid::
cargo test robot::
cargo test level::
```

### Integration Tests
```bash
# Run integration tests (if any exist in tests/ directory)
cargo test --test '*'
```

### Documentation Tests
```bash
# Run documentation examples as tests
cargo test --doc
```

### Benchmarks
```bash
# Run performance benchmarks (requires nightly Rust)
cargo +nightly bench
```

## 🔍 Testing Strategies

### 1. **Level Loading Tests**
Validates that YAML levels load correctly:
```bash
# Test loading all YAML levels
cargo run --release -- --test-learning-levels
```

**What it tests:**
- YAML syntax validation
- Level configuration parsing
- Item file references
- Movement pattern file references
- Grid size boundaries
- Enemy spawn positions

### 2. **Robot Code Execution Tests**
Tests the robot programming interface:

**Test scenarios:**
- Basic movement commands (`move(up)`, `move(down)`, etc.)
- Item collection (`grab()`)
- Scanning functionality (`scan(direction)`)
- Advanced functions (`search_all()`, `set_auto_grab()`)
- Code syntax validation
- Runtime error handling

### 3. **Game State Tests**
Validates game state transitions:

**Areas to test:**
- Menu navigation
- Level progression
- Score calculation
- Shop transactions
- Save/load functionality (if implemented)

### 4. **Enemy Behavior Tests**
Tests enemy movement patterns:

**Movement patterns to validate:**
- Horizontal movement
- Vertical movement
- Diagonal movement
- Circular patterns
- Random movement
- Custom movement scripts

### 5. **Collision Detection Tests**
Ensures proper collision handling:

**Test cases:**
- Robot-enemy collisions
- Robot-obstacle collisions
- Robot-item interactions
- Grid boundary checks

## 🎮 Manual Testing

### Level Progression Testing
1. Start the game normally
2. Complete each level in sequence
3. Verify:
   - Level transitions work correctly
   - Score carries over
   - Upgrades persist
   - No crashes between levels

### UI/UX Testing
1. Test all menu options:
   - Normal Start
   - Settings (resolution, audio)
   - Player Levels
   - Exit
2. Test all keyboard shortcuts:
   - SHIFT+CTRL+ENTER (execute code)
   - SHIFT+CTRL+B (open shop)
   - SHIFT+CTRL+N (next level)
   - SHIFT+CTRL+M (menu)
   - SHIFT+CTRL+R (reset code)
   - SHIFT+CTRL+L (reload level)

### Code Editor Testing
1. Write various robot programs
2. Test syntax highlighting
3. Verify error messages
4. Test code execution
5. Check external editor integration (SHIFT+CTRL+E)

### Platform Testing
```bash
# Desktop Windows
cargo run --release

# Desktop Linux/Mac
cargo run --release

# WebAssembly
./build-wasm.sh
python -m http.server 8000 --directory dist
```

## 🤖 Automated Testing

### Creating Unit Tests
Add tests to relevant source files:

```rust
// In src/grid.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
    }

    #[test]
    fn test_enemy_movement() {
        let mut grid = Grid::new(10, 10);
        let enemy = Enemy::new(5, 5, MovementPattern::Horizontal);
        grid.add_enemy(enemy);
        grid.update_enemies();
        // Assert enemy moved correctly
    }
}
```

### Creating Integration Tests
Add files to `tests/` directory:

```rust
// tests/level_loading.rs
use robo_grid_explorer_gui::*;

#[test]
fn test_yaml_level_loading() {
    let levels = load_yaml_levels();
    assert!(!levels.is_empty());

    for level in levels {
        assert!(!level.name.is_empty());
        assert!(level.grid_size.0 > 0);
        assert!(level.grid_size.1 > 0);
    }
}
```

### Property-Based Testing
Using `quickcheck` or `proptest` for randomized testing:

```rust
#[cfg(test)]
use quickcheck::quickcheck;

#[test]
fn test_grid_bounds() {
    fn prop(x: i32, y: i32, width: u32, height: u32) -> bool {
        let grid = Grid::new(width.max(1), height.max(1));
        grid.is_valid_position(x, y) ==
            (x >= 0 && x < width as i32 && y >= 0 && y < height as i32)
    }
    quickcheck(prop as fn(i32, i32, u32, u32) -> bool);
}
```

## 📊 Performance Testing

### Frame Rate Testing
```bash
# Run with FPS counter
cargo run --release -- --show-fps

# Profile with release optimizations
cargo build --release
perf record --call-graph=dwarf target/release/robo_grid_explorer_gui
perf report
```

### Memory Testing
```bash
# Check for memory leaks (Linux)
valgrind --leak-check=full target/release/robo_grid_explorer_gui

# Memory profiling (with heaptrack)
heaptrack target/release/robo_grid_explorer_gui
```

### WASM Performance
```javascript
// In browser console
performance.mark('start');
// Run game actions
performance.mark('end');
performance.measure('game-loop', 'start', 'end');
console.log(performance.getEntriesByType('measure'));
```

## 🧩 Test Coverage

### Generating Coverage Reports
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# With specific test types
cargo tarpaulin --lib --out Html
```

### Coverage Goals
- **Core Logic**: 80%+ coverage
- **Game State**: 70%+ coverage
- **UI Code**: 50%+ coverage
- **Utility Functions**: 90%+ coverage

## 🔧 Debugging Tests

### Verbose Output
```bash
# Run with full backtrace
RUST_BACKTRACE=full cargo test

# Run single test with println! output
cargo test test_name -- --nocapture

# Debug specific module
cargo test --lib grid:: -- --nocapture
```

### Using GDB/LLDB
```bash
# Compile with debug symbols
cargo build --tests

# Debug with gdb
gdb target/debug/deps/robo_grid_explorer_gui-*
(gdb) break test_function_name
(gdb) run
```

## 📝 Writing New Tests

### Test Naming Conventions
- Unit tests: `test_<module>_<functionality>`
- Integration tests: `integration_<feature>`
- Property tests: `prop_<property_name>`

### Test Organization
```
tests/
├── integration/
│   ├── level_loading.rs
│   ├── game_progression.rs
│   └── robot_execution.rs
├── performance/
│   ├── frame_rate.rs
│   └── memory_usage.rs
└── common/
    └── mod.rs  # Shared test utilities
```

### Test Data
Place test assets in `tests/data/`:
```
tests/data/
├── test_levels/
│   ├── valid_level.yaml
│   ├── invalid_level.yaml
│   └── edge_cases.yaml
├── test_code/
│   ├── valid_robot_code.rs
│   └── invalid_syntax.rs
└── expected_outputs/
    └── level_completions.json
```

## 🚦 Continuous Integration

### GitHub Actions Example
```yaml
name: Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --all-features
      - run: cargo test --release
```

## 🐛 Common Test Issues

### Issue: Tests fail on Windows but pass on Linux
**Solution**: Check file path separators and line endings

### Issue: WASM tests don't run
**Solution**: Use `wasm-pack test` for WASM-specific tests

### Issue: Flaky tests with timing
**Solution**: Use deterministic test conditions, avoid sleep()

### Issue: Tests interfere with each other
**Solution**: Ensure proper test isolation and cleanup

## 🤝 Contributing Tests

### Guidelines for Contributors
1. **Write tests for new features** - Every PR should include tests
2. **Fix broken tests** - Don't disable failing tests
3. **Document test purpose** - Add comments explaining what's being tested
4. **Keep tests fast** - Aim for < 1 second per unit test
5. **Make tests deterministic** - Avoid random behavior without seeds

### Test Review Checklist
- [ ] Tests compile and pass
- [ ] Tests cover the main functionality
- [ ] Tests include edge cases
- [ ] Tests have descriptive names
- [ ] Tests are independent
- [ ] Tests clean up after themselves

## 📚 Additional Resources

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Property-Based Testing in Rust](https://proptest-rs.github.io/proptest/)
- [Test Organization Best Practices](https://matklad.github.io/2021/02/27/test-organization.html)

---

**Remember**: Good tests make development faster, not slower. They catch bugs early, document expected behavior, and give confidence when refactoring!