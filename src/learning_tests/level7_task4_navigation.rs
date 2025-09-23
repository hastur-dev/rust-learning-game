// Level 7 Task 4 Test: Navigation and Pathfinding Structs
// Tests if the user code creates navigation system with 2D grids and pathfinding

#[cfg(test)]
mod level7_task4_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_cell_type_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("enum CellType"),
            "❌ Your code should define a CellType enum"
        );
    }

    #[test]
    fn test_has_game_map_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct GameMap"),
            "❌ Your code should define a GameMap struct"
        );
    }

    #[test]
    fn test_has_position_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct Position"),
            "❌ Your code should define or use a Position struct"
        );
    }

    #[test]
    fn test_game_map_has_2d_grid() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_2d_vec = analyzer.code.contains("Vec<Vec<") ||
                        analyzer.code.contains("cells: Vec<Vec");
        assert!(
            has_2d_vec,
            "❌ Your GameMap should have a 2D vector grid for storing cell types"
        );
    }

    #[test]
    fn test_cell_type_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_empty = analyzer.code.contains("Empty");
        let has_obstacle = analyzer.code.contains("Obstacle");
        let has_enemy = analyzer.code.contains("Enemy");
        assert!(
            has_empty && has_obstacle && has_enemy,
            "❌ Your CellType enum should have Empty, Obstacle, and Enemy variants"
        );
    }

    #[test]
    fn test_has_game_map_impl() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("impl GameMap"),
            "❌ Your code should have an impl block for GameMap"
        );
    }

    #[test]
    fn test_has_set_cell_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn set_cell"),
            "❌ Your GameMap should have a set_cell() method"
        );
    }

    #[test]
    fn test_has_get_cell_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn get_cell"),
            "❌ Your GameMap should have a get_cell() method"
        );
    }

    #[test]
    fn test_has_can_move_to_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn can_move_to"),
            "❌ Your GameMap should have a can_move_to() method"
        );
    }

    #[test]
    fn test_has_find_path_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn find_path") ||
            analyzer.code.contains("find_path_to"),
            "❌ Your GameMap should have a pathfinding method (find_path or find_path_to)"
        );
    }

    #[test]
    fn test_has_scan_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn scan") ||
            analyzer.code.contains("scan_around"),
            "❌ Your GameMap should have a scanning method"
        );
    }

    #[test]
    fn test_uses_bounds_checking() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_bounds_check = analyzer.code.contains("is_valid_position") ||
                              (analyzer.code.contains("width") && analyzer.code.contains("height"));
        assert!(
            has_bounds_check,
            "❌ Your code should check boundaries when accessing the grid"
        );
    }

    #[test]
    fn test_path_returns_vector() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let returns_vec_position = analyzer.code.contains("Vec<Position>") ||
                                  analyzer.code.contains("-> Vec<Position>");
        assert!(
            returns_vec_position,
            "❌ Your pathfinding method should return Vec<Position>"
        );
    }

    #[test]
    fn test_sets_up_level_obstacles() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let sets_enemies = analyzer.code.contains("set_cell") &&
                          (analyzer.code.contains("Enemy") || analyzer.code.contains("CellType::Enemy"));
        assert!(
            sets_enemies,
            "❌ Your code should set up obstacles/enemies using set_cell()"
        );
    }

    #[test]
    fn test_robot_position_tracking() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let tracks_robot = analyzer.code.contains("robot_position") ||
                          analyzer.code.contains("move_robot");
        assert!(
            tracks_robot,
            "❌ Your GameMap should track the robot's position"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output navigation information
        let has_nav_output = result.stdout.contains("path") ||
                            result.stdout.contains("scan") ||
                            result.stdout.contains("move") ||
                            result.stdout.contains("robot") ||
                            result.stdout.contains("position");

        assert!(
            has_nav_output,
            "❌ Your program should output information about navigation and pathfinding"
        );
    }
}

// Reference implementation for comparison
#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    Obstacle,
    Enemy,
    Door,
    Item,
    Goal,
}

#[derive(Debug)]
struct GameMap {
    width: i32,
    height: i32,
    cells: Vec<Vec<CellType>>,
    robot_position: Position,
}

impl GameMap {
    fn new(width: i32, height: i32) -> Self {
        let cells = vec![vec![CellType::Empty; height as usize]; width as usize];
        GameMap {
            width,
            height,
            cells,
            robot_position: Position::new(0, 0),
        }
    }

    fn set_cell(&mut self, x: i32, y: i32, cell_type: CellType) {
        if self.is_valid_position(x, y) {
            self.cells[x as usize][y as usize] = cell_type;
        }
    }

    fn get_cell(&self, x: i32, y: i32) -> CellType {
        if self.is_valid_position(x, y) {
            self.cells[x as usize][y as usize]
        } else {
            CellType::Obstacle
        }
    }

    fn is_valid_position(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn can_move_to(&self, x: i32, y: i32) -> bool {
        match self.get_cell(x, y) {
            CellType::Empty | CellType::Item | CellType::Goal => true,
            _ => false,
        }
    }

    fn find_path_to(&self, target: Position) -> Vec<Position> {
        let mut path = vec![];
        let current = self.robot_position;

        // Simple pathfinding: move towards target
        let dx = if target.x > current.x { 1 } else if target.x < current.x { -1 } else { 0 };
        let dy = if target.y > current.y { 1 } else if target.y < current.y { -1 } else { 0 };

        let mut next = current;

        while next != target {
            // Try to move horizontally first
            if dx != 0 && self.can_move_to(next.x + dx, next.y) {
                next.x += dx;
            }
            // Then try vertically
            else if dy != 0 && self.can_move_to(next.x, next.y + dy) {
                next.y += dy;
            }
            // If blocked, try alternative routes
            else {
                // Try moving around obstacles
                let alternatives = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                let mut moved = false;

                for (alt_dx, alt_dy) in alternatives {
                    let alt_x = next.x + alt_dx;
                    let alt_y = next.y + alt_dy;

                    if self.can_move_to(alt_x, alt_y) {
                        next.x = alt_x;
                        next.y = alt_y;
                        moved = true;
                        break;
                    }
                }

                if !moved {
                    println!("Path blocked, cannot reach target");
                    break;
                }
            }

            path.push(next);

            // Prevent infinite loops
            if path.len() > 50 {
                break;
            }
        }

        path
    }

    fn scan_around_robot(&self, range: i32) -> Vec<(Position, CellType)> {
        let mut scan_results = vec![];

        for dx in -range..=range {
            for dy in -range..=range {
                if dx == 0 && dy == 0 { continue; } // Skip robot position

                let scan_x = self.robot_position.x + dx;
                let scan_y = self.robot_position.y + dy;

                if self.is_valid_position(scan_x, scan_y) {
                    let cell_type = self.get_cell(scan_x, scan_y);
                    scan_results.push((Position::new(scan_x, scan_y), cell_type));
                }
            }
        }

        scan_results
    }

    fn move_robot(&mut self, new_position: Position) -> bool {
        if self.can_move_to(new_position.x, new_position.y) {
            self.robot_position = new_position;
            println!("Robot moved to {:?}", new_position);
            true
        } else {
            println!("Cannot move to {:?} - blocked", new_position);
            false
        }
    }
}

fn main() {
    // Create map matching level layout (12x10)
    let mut game_map = GameMap::new(12, 10);

    // Set up obstacles (from level configuration)
    game_map.set_cell(3, 2, CellType::Enemy);
    game_map.set_cell(8, 5, CellType::Enemy);
    game_map.set_cell(5, 8, CellType::Enemy);
    game_map.set_cell(6, 3, CellType::Door);
    game_map.set_cell(4, 7, CellType::Door);

    // Set up items from level
    game_map.set_cell(2, 1, CellType::Item);
    game_map.set_cell(10, 3, CellType::Item);
    game_map.set_cell(1, 6, CellType::Item);
    game_map.set_cell(11, 9, CellType::Goal);

    println!("Initial robot position: {:?}", game_map.robot_position);

    // Scan around robot
    let scan_results = game_map.scan_around_robot(2);
    println!("Scan results: {} cells detected", scan_results.len());

    for (pos, cell_type) in scan_results.iter().take(5) {
        println!("  {:?}: {:?}", pos, cell_type);
    }

    // Plan path to first item
    let item_position = Position::new(2, 1);
    let path = game_map.find_path_to(item_position);

    println!("Path to item at {:?}: {} steps", item_position, path.len());

    // Execute path
    for (i, step) in path.iter().enumerate().take(5) {
        if game_map.move_robot(*step) {
            println!("Step {}: Moved to {:?}", i + 1, step);
        } else {
            println!("Step {}: Move blocked", i + 1);
        }
    }

    // Check if we can reach the goal
    let goal_position = Position::new(11, 9);
    let goal_path = game_map.find_path_to(goal_position);
    println!("Path to goal: {} steps planned", goal_path.len());
}