// Level 5 Task 4 Test: Custom Type Conversions
// Tests that user can create custom types and implement From trait

#[cfg(test)]
mod level5_task4_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_defines_position_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_position = analyzer.code.contains("struct Position") &&
                         analyzer.code.contains("x:") &&
                         analyzer.code.contains("y:");
        assert!(
            has_position,
            "❌ You should define a Position struct with x and y fields"
        );
    }

    #[test]
    fn test_defines_robot_state_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_robot_state = analyzer.code.contains("struct RobotState") &&
                            analyzer.code.contains("position:") &&
                            analyzer.code.contains("energy:");
        assert!(
            has_robot_state,
            "❌ You should define a RobotState struct with position and energy fields"
        );
    }

    #[test]
    fn test_implements_from_tuple_to_position() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_from_impl = analyzer.code.contains("impl From<(i32, i32)> for Position") &&
                          analyzer.code.contains("fn from(coord: (i32, i32))");
        assert!(
            has_from_impl,
            "❌ You should implement From<(i32, i32)> for Position"
        );
    }

    #[test]
    fn test_implements_from_position_to_tuple() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_reverse_impl = analyzer.code.contains("impl From<Position> for (i32, i32)") &&
                             analyzer.code.contains("(pos.x, pos.y)");
        assert!(
            has_reverse_impl,
            "❌ You should implement From<Position> for (i32, i32) for bidirectional conversion"
        );
    }

    #[test]
    fn test_uses_into_for_conversion() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_into_usage = analyzer.code.contains(".into()") &&
                           analyzer.code.contains("start_coords");
        assert!(
            has_into_usage,
            "❌ You should use .into() to convert tuple to Position"
        );
    }

    #[test]
    fn test_creates_robot_with_conversion() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_robot_creation = analyzer.code.contains("RobotState") &&
                               analyzer.code.contains("(0, 0).into()");
        assert!(
            has_robot_creation,
            "❌ You should create RobotState using tuple conversion for position"
        );
    }

    #[test]
    fn test_demonstrates_chained_conversions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_chain = analyzer.code.contains("movement") &&
                       analyzer.code.contains("new_position") &&
                       analyzer.code.contains("back_to_tuple");
        assert!(
            has_chain,
            "❌ You should demonstrate chained conversions (tuple -> Position -> tuple)"
        );
    }

    #[test]
    fn test_converts_collection_of_data() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_collection = analyzer.code.contains("coords") &&
                           analyzer.code.contains("positions") &&
                           analyzer.code.contains(".map(") &&
                           analyzer.code.contains(".collect()");
        assert!(
            has_collection,
            "❌ You should convert a collection of coordinates using map and collect"
        );
    }

    #[test]
    fn test_prints_conversion_results() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let println_count = analyzer.code.matches("println!").count();
        assert!(
            println_count >= 6,
            "❌ You should print the results of various custom conversions"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let output_lines: Vec<&str> = result.stdout.lines().collect();
        let shows_conversions = output_lines.iter().any(|line| line.contains("coordinates")) &&
                              output_lines.iter().any(|line| line.contains("Position")) &&
                              output_lines.iter().any(|line| line.contains("Robot"));
        assert!(
            shows_conversions,
            "❌ Your program should show custom type conversions working"
        );
    }
}

// Reference implementation for comparison
struct Position {
    x: i32,
    y: i32,
}

struct RobotState {
    position: Position,
    energy: u32,
}

impl From<(i32, i32)> for Position {
    fn from(coord: (i32, i32)) -> Self {
        Position {
            x: coord.0,
            y: coord.1,
        }
    }
}

impl From<Position> for (i32, i32) {
    fn from(pos: Position) -> Self {
        (pos.x, pos.y)
    }
}

fn main() {
    let start_coords = (5, 10);
    let start_position: Position = start_coords.into();

    println!("Start coordinates: {:?}", start_coords);
    println!("Position: x={}, y={}", start_position.x, start_position.y);

    let end_position = Position { x: 15, y: 25 };
    let end_coords: (i32, i32) = end_position.into();

    println!("End position: x=15, y=25");
    println!("End coordinates: {:?}", end_coords);

    let robot = RobotState {
        position: (0, 0).into(),
        energy: 100,
    };

    println!("Robot created at: x={}, y={}", robot.position.x, robot.position.y);
    println!("Robot energy: {}", robot.energy);

    let movement: (i32, i32) = (3, 4);
    let new_position: Position = movement.into();
    let back_to_tuple: (i32, i32) = new_position.into();

    println!("Movement chain: {:?} -> Position -> {:?}", movement, back_to_tuple);

    let coords = [(1, 2), (3, 4), (5, 6)];
    let positions: Vec<Position> = coords.iter().map(|&c| c.into()).collect();

    println!("Converted {} coordinates to positions", positions.len());
    for (i, pos) in positions.iter().enumerate() {
        println!("Position {}: ({}, {})", i, pos.x, pos.y);
    }
}