// Level 7 Task 1 Test: Create Robot Position Struct
// Tests if the user code creates a Position struct with methods

#[cfg(test)]
mod level7_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_position_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct Position"),
            "❌ Your code should define a Position struct"
        );
    }

    #[test]
    fn test_has_struct_fields() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_x_field = analyzer.code.contains("x: i32") || analyzer.code.contains("x:i32");
        let has_y_field = analyzer.code.contains("y: i32") || analyzer.code.contains("y:i32");
        assert!(
            has_x_field && has_y_field,
            "❌ Your Position struct should have x and y fields of type i32"
        );
    }

    #[test]
    fn test_has_derive_debug() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("#[derive(Debug") || analyzer.code.contains("#[derive(Debug)]"),
            "❌ Your Position struct should derive Debug trait"
        );
    }

    #[test]
    fn test_has_impl_block() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("impl Position"),
            "❌ Your code should have an impl block for Position"
        );
    }

    #[test]
    fn test_has_new_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn new("),
            "❌ Your Position struct should have a new() constructor method"
        );
    }

    #[test]
    fn test_has_distance_to_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn distance_to"),
            "❌ Your Position struct should have a distance_to() method"
        );
    }

    #[test]
    fn test_has_move_by_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn move_by"),
            "❌ Your Position struct should have a move_by() method"
        );
    }

    #[test]
    fn test_has_is_valid_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn is_valid"),
            "❌ Your Position struct should have an is_valid() method"
        );
    }

    #[test]
    fn test_uses_self_parameters() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_self_ref = analyzer.code.contains("&self");
        let has_mut_self = analyzer.code.contains("&mut self");
        assert!(
            has_self_ref || has_mut_self,
            "❌ Your methods should use &self or &mut self parameters"
        );
    }

    #[test]
    fn test_position_creation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("Position::new("),
            "❌ Your code should create Position instances using Position::new()"
        );
    }

    #[test]
    fn test_method_calls() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_method_calls = analyzer.code.contains(".move_by(") ||
                              analyzer.code.contains(".distance_to(") ||
                              analyzer.code.contains(".is_valid(");
        assert!(
            has_method_calls,
            "❌ Your code should call methods on Position instances"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output position information
        let has_position_output = result.stdout.contains("Robot") ||
                                 result.stdout.contains("Position") ||
                                 result.stdout.contains("moved") ||
                                 result.stdout.contains("distance");

        assert!(
            has_position_output,
            "❌ Your program should output information about robot position and movement"
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
    // Constructor method
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    // Calculate distance to another position
    fn distance_to(&self, other: &Position) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    // Move in a direction
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    // Check if position is within bounds
    fn is_valid(&self, width: i32, height: i32) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }
}

fn main() {
    // Create robot starting position
    let mut robot_pos = Position::new(0, 0);
    println!("Robot starts at: {:?}", robot_pos);

    // Move robot to collect struct_blueprint at (2, 1)
    robot_pos.move_by(2, 1);
    println!("Robot moved to: {:?}", robot_pos);

    // Check if move is valid
    if robot_pos.is_valid(12, 10) {
        println!("Position is valid within 12x10 grid");
    }

    // Calculate distance to goal
    let goal = Position::new(11, 9);
    let distance = robot_pos.distance_to(&goal);
    println!("Distance to goal: {:.2} units", distance);

    // Navigate to collect items using position struct
    let items = vec![Position::new(2, 1), Position::new(10, 3), Position::new(1, 6)];
    for item_pos in items {
        println!("Item at {:?}, distance: {:.2}", item_pos, robot_pos.distance_to(&item_pos));
    }
}