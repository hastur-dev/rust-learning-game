// Level 8 Task 1 Test: Basic Enum Types for Robot Commands
// Tests if the user code creates basic enums for robot control

#[cfg(test)]
mod level8_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_direction_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("enum Direction"),
            "❌ Your code should define a Direction enum"
        );
    }

    #[test]
    fn test_direction_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_north = analyzer.code.contains("North");
        let has_south = analyzer.code.contains("South");
        let has_east = analyzer.code.contains("East");
        let has_west = analyzer.code.contains("West");
        assert!(
            has_north && has_south && has_east && has_west,
            "❌ Your Direction enum should have North, South, East, and West variants"
        );
    }

    #[test]
    fn test_has_robot_command_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("enum RobotCommand"),
            "❌ Your code should define a RobotCommand enum"
        );
    }

    #[test]
    fn test_robot_command_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_move = analyzer.code.contains("Move(");
        let has_scan = analyzer.code.contains("Scan");
        let has_grab = analyzer.code.contains("Grab");
        assert!(
            has_move && has_scan && has_grab,
            "❌ Your RobotCommand enum should have Move(Direction), Scan, and Grab variants"
        );
    }

    #[test]
    fn test_has_action_result_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("enum ActionResult") ||
            analyzer.code.contains("ActionResult"),
            "❌ Your code should define an ActionResult enum"
        );
    }

    #[test]
    fn test_action_result_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_success = analyzer.code.contains("Success");
        let has_failed = analyzer.code.contains("Failed");
        let has_blocked = analyzer.code.contains("Blocked");
        assert!(
            has_success && (has_failed || has_blocked),
            "❌ Your ActionResult enum should have Success, Failed, and/or Blocked variants"
        );
    }

    #[test]
    fn test_has_execute_command_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn execute_command"),
            "❌ Your code should have an execute_command function"
        );
    }

    #[test]
    fn test_uses_match_expressions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("match"),
            "❌ Your code should use match expressions to handle enum variants"
        );
    }

    #[test]
    fn test_enum_with_data() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_move_direction = analyzer.code.contains("Move(Direction)") ||
                                analyzer.code.contains("Move(direction)");
        assert!(
            has_move_direction,
            "❌ Your Move variant should carry Direction data"
        );
    }

    #[test]
    fn test_derives_debug() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_derive_debug = analyzer.code.contains("#[derive(Debug") ||
                              analyzer.code.contains("derive(Debug)");
        assert!(
            has_derive_debug,
            "❌ Your enums should derive Debug trait"
        );
    }

    #[test]
    fn test_creates_commands() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let creates_move_command = analyzer.code.contains("RobotCommand::Move(") ||
                                  analyzer.code.contains("Move(Direction::");
        assert!(
            creates_move_command,
            "❌ Your code should create RobotCommand instances with Direction"
        );
    }

    #[test]
    fn test_handles_command_results() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let handles_results = analyzer.code.contains("ActionResult::Success") ||
                             analyzer.code.contains("ActionResult::Failed");
        assert!(
            handles_results,
            "❌ Your code should handle ActionResult variants"
        );
    }

    #[test]
    fn test_direction_matching() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let matches_directions = analyzer.code.contains("Direction::North") ||
                                analyzer.code.contains("Direction::East");
        assert!(
            matches_directions,
            "❌ Your code should match on Direction variants"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output command execution information
        let has_command_output = result.stdout.contains("Moving") ||
                                result.stdout.contains("Command") ||
                                result.stdout.contains("Direction") ||
                                result.stdout.contains("Executing");

        assert!(
            has_command_output,
            "❌ Your program should output information about command execution"
        );
    }
}

// Reference implementation for comparison
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
enum RobotCommand {
    Move(Direction),
    Scan,
    Grab,
    OpenDoor,
    Recharge,
    Wait,
}

#[derive(Debug, PartialEq)]
enum ActionResult {
    Success,
    Failed(String),
    Blocked,
    EnergyRequired(u32),
}

fn execute_command(command: RobotCommand) -> ActionResult {
    match command {
        RobotCommand::Move(direction) => {
            match direction {
                Direction::North => {
                    println!("Moving north");
                    ActionResult::Success
                }
                Direction::South => {
                    println!("Moving south");
                    ActionResult::Success
                }
                Direction::East => {
                    println!("Moving east");
                    ActionResult::Success
                }
                Direction::West => {
                    println!("Moving west");
                    ActionResult::Success
                }
            }
        }
        RobotCommand::Scan => {
            println!("Scanning area...");
            ActionResult::Success
        }
        RobotCommand::Grab => {
            println!("Attempting to grab item");
            ActionResult::Success
        }
        RobotCommand::OpenDoor => {
            println!("Opening door");
            ActionResult::EnergyRequired(15)
        }
        RobotCommand::Recharge => {
            println!("Recharging battery");
            ActionResult::Success
        }
        RobotCommand::Wait => {
            println!("Waiting...");
            ActionResult::Success
        }
    }
}

fn main() {
    println!("Robot Command System Demo");

    // Create various robot commands
    let commands = vec![
        RobotCommand::Move(Direction::East),
        RobotCommand::Move(Direction::East),
        RobotCommand::Move(Direction::North),
        RobotCommand::Scan,
        RobotCommand::Grab,
        RobotCommand::OpenDoor,
        RobotCommand::Move(Direction::South),
        RobotCommand::Recharge,
    ];

    // Execute commands and handle results
    for (i, command) in commands.iter().enumerate() {
        println!("\n--- Command {} ---", i + 1);
        println!("Executing: {:?}", command);

        let result = execute_command(command.clone());
        println!("Result: {:?}", result);

        match result {
            ActionResult::Success => println!("✅ Command completed successfully"),
            ActionResult::Failed(reason) => println!("❌ Command failed: {}", reason),
            ActionResult::Blocked => println!("🚧 Path blocked, trying alternative"),
            ActionResult::EnergyRequired(amount) => {
                println!("⚡ Command requires {} energy", amount);
            }
        }
    }

    // Direction utilities
    let current_direction = Direction::North;
    let opposite = match current_direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    };

    println!("\nDirection: {:?}, Opposite: {:?}", current_direction, opposite);
}