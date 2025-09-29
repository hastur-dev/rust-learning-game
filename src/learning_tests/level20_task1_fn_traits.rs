// Learning Tests for Level 20, Task 1: Fn Traits Fundamentals
// Understanding Fn, FnMut, and FnOnce traits for robot control systems

use std::collections::HashMap;

// Robot state for function trait demonstrations
#[derive(Debug, Clone, PartialEq)]
pub struct Robot {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub battery: f64,
    pub status: String,
}

impl Robot {
    pub fn new(id: u32, x: f64, y: f64) -> Self {
        Robot {
            id,
            x,
            y,
            battery: 100.0,
            status: "active".to_string(),
        }
    }

    pub fn move_to(&mut self, new_x: f64, new_y: f64) {
        let distance = ((new_x - self.x).powi(2) + (new_y - self.y).powi(2)).sqrt();
        self.x = new_x;
        self.y = new_y;
        self.battery -= distance * 0.1;
    }

    pub fn get_position(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn consume_energy(&mut self, amount: f64) {
        self.battery = (self.battery - amount).max(0.0);
    }
}

// Fn trait - immutable borrowing, can be called multiple times
pub struct RobotAnalyzer;

impl RobotAnalyzer {
    // Function that takes Fn - can be called multiple times
    pub fn analyze_robot<F>(robot: &Robot, analyzer: F) -> String
    where
        F: Fn(&Robot) -> String,
    {
        analyzer(robot)
    }

    // Multiple analysis functions
    pub fn analyze_multiple<F>(robots: &[Robot], analyzer: F) -> Vec<String>
    where
        F: Fn(&Robot) -> String,
    {
        robots.iter().map(|robot| analyzer(robot)).collect()
    }

    // Function that stores and reuses Fn
    pub fn create_battery_checker() -> impl Fn(&Robot) -> bool {
        |robot| robot.battery > 20.0
    }

    pub fn create_position_checker(max_distance: f64) -> impl Fn(&Robot) -> bool {
        move |robot| {
            let distance = (robot.x.powi(2) + robot.y.powi(2)).sqrt();
            distance <= max_distance
        }
    }
}

// FnMut trait - mutable borrowing, can be called multiple times but mutates state
pub struct RobotController {
    pub command_count: u32,
    pub last_command: String,
}

impl RobotController {
    pub fn new() -> Self {
        RobotController {
            command_count: 0,
            last_command: String::new(),
        }
    }

    // Function that takes FnMut - can modify captured state
    pub fn execute_command<F>(&self, robot: &mut Robot, mut command: F) -> String
    where
        F: FnMut(&mut Robot) -> String,
    {
        command(robot)
    }

    // Function that tracks commands
    pub fn execute_tracked_command<F>(&mut self, robot: &mut Robot, mut command: F) -> String
    where
        F: FnMut(&mut Robot) -> String,
    {
        self.command_count += 1;
        let result = command(robot);
        self.last_command = result.clone();
        result
    }

    // Process multiple robots with stateful operation
    pub fn process_robots<F>(&self, robots: &mut [Robot], mut processor: F)
    where
        F: FnMut(&mut Robot, usize),
    {
        for (index, robot) in robots.iter_mut().enumerate() {
            processor(robot, index);
        }
    }
}

// FnOnce trait - consumes captured variables, can only be called once
pub struct RobotFactory {
    pub next_id: u32,
}

impl RobotFactory {
    pub fn new() -> Self {
        RobotFactory { next_id: 1 }
    }

    // Function that takes FnOnce - consumes the closure
    pub fn create_robot<F>(mut self, initializer: F) -> Robot
    where
        F: FnOnce(u32) -> Robot,
    {
        let robot = initializer(self.next_id);
        self.next_id += 1;
        robot
    }

    // Function that takes owned data
    pub fn create_with_config<F>(config: HashMap<String, f64>, builder: F) -> Robot
    where
        F: FnOnce(HashMap<String, f64>) -> Robot,
    {
        builder(config)
    }

    // Batch creation with consuming closure
    pub fn create_batch<F>(count: u32, mut factory: F) -> Vec<Robot>
    where
        F: FnMut() -> Robot,
    {
        (0..count).map(|_| factory()).collect()
    }
}

// Trait bounds and complex function types
pub struct RobotCommandSystem;

impl RobotCommandSystem {
    // Function that accepts different callable types
    pub fn execute_any_command<F>(robot: &mut Robot, command: F) -> String
    where
        F: FnOnce(&mut Robot) -> String,
    {
        command(robot)
    }

    // Function with multiple trait bounds
    pub fn safe_execute<F>(robot: &mut Robot, command: F) -> Result<String, String>
    where
        F: Fn(&mut Robot) -> String + Send + Sync,
    {
        if robot.battery > 0.0 {
            Ok(command(robot))
        } else {
            Err("Robot has no battery".to_string())
        }
    }

    // Higher-order function returning closures
    pub fn create_move_command(dx: f64, dy: f64) -> impl FnOnce(&mut Robot) -> String {
        move |robot| {
            let old_pos = robot.get_position();
            robot.move_to(old_pos.0 + dx, old_pos.1 + dy);
            format!("Moved robot {} by ({}, {})", robot.id, dx, dy)
        }
    }

    // Function that takes Box<dyn Fn>
    pub fn execute_boxed_command(robot: &Robot, command: Box<dyn Fn(&Robot) -> String>) -> String {
        command(robot)
    }
}

// Function pointer vs closure demonstration
pub struct FunctionPointerDemo;

impl FunctionPointerDemo {
    // Function pointer
    pub fn apply_fn_ptr(value: i32, func: fn(i32) -> i32) -> i32 {
        func(value)
    }

    // Closure
    pub fn apply_closure<F>(value: i32, func: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        func(value)
    }

    // Demonstrate the difference
    pub fn compare_approaches() -> (i32, i32) {
        let multiplier = 5;

        // Function pointer - no captures allowed
        fn double(x: i32) -> i32 {
            x * 2
        }

        // Closure - can capture
        let multiply_by_captured = |x: i32| x * multiplier;

        let fp_result = Self::apply_fn_ptr(10, double);
        let closure_result = Self::apply_closure(10, multiply_by_captured);

        (fp_result, closure_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_trait() {
        let robot = Robot::new(1, 10.0, 20.0);

        // Fn can be called multiple times
        let battery_analyzer = |r: &Robot| format!("Battery: {:.1}%", r.battery);

        let result1 = RobotAnalyzer::analyze_robot(&robot, &battery_analyzer);
        let result2 = RobotAnalyzer::analyze_robot(&robot, &battery_analyzer);

        assert_eq!(result1, "Battery: 100.0%");
        assert_eq!(result2, "Battery: 100.0%");

        // Test multiple analysis
        let robots = vec![
            Robot::new(1, 0.0, 0.0),
            Robot::new(2, 10.0, 10.0),
        ];

        let results = RobotAnalyzer::analyze_multiple(&robots, |r| {
            format!("Robot {} at ({}, {})", r.id, r.x, r.y)
        });

        assert_eq!(results.len(), 2);
        assert!(results[0].contains("Robot 1"));
        assert!(results[1].contains("Robot 2"));
    }

    #[test]
    fn test_fnmut_trait() {
        let mut robot = Robot::new(1, 0.0, 0.0);
        let controller = RobotController::new();

        let mut move_count = 0;
        let move_command = |r: &mut Robot| {
            move_count += 1;
            r.move_to(move_count as f64, move_count as f64);
            format!("Move #{}", move_count)
        };

        // FnMut can be called multiple times and modify captured state
        let result1 = controller.execute_command(&mut robot, move_command);
        // Note: move_command was moved, so we can't use it again directly

        assert!(result1.contains("Move #1"));
        assert_eq!(robot.get_position(), (1.0, 1.0));
    }

    #[test]
    fn test_fnonce_trait() {
        let factory = RobotFactory::new();
        let initial_config = HashMap::new();

        // FnOnce consumes captured variables
        let config_copy = initial_config.clone();
        let robot = factory.create_robot(|id| {
            // This closure takes ownership of config_copy
            Robot::new(id, 0.0, 0.0)
        });

        assert_eq!(robot.id, 1);

        // Test with consuming config
        let mut config = HashMap::new();
        config.insert("x".to_string(), 5.0);
        config.insert("y".to_string(), 10.0);

        let robot = RobotFactory::create_with_config(config, |cfg| {
            let x = cfg.get("x").copied().unwrap_or(0.0);
            let y = cfg.get("y").copied().unwrap_or(0.0);
            Robot::new(100, x, y)
        });

        assert_eq!(robot.get_position(), (5.0, 10.0));
    }

    #[test]
    fn test_trait_bounds() {
        let mut robot = Robot::new(1, 0.0, 0.0);

        // Test safe execution
        let safe_command = |r: &mut Robot| {
            r.consume_energy(10.0);
            format!("Energy consumed, battery: {:.1}", r.battery)
        };

        let result = RobotCommandSystem::safe_execute(&mut robot, safe_command);
        assert!(result.is_ok());
        assert_eq!(robot.battery, 90.0);

        // Test with no battery
        robot.battery = 0.0;
        let result = RobotCommandSystem::safe_execute(&mut robot, |r| "Test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_created_closures() {
        let battery_checker = RobotAnalyzer::create_battery_checker();
        let position_checker = RobotAnalyzer::create_position_checker(15.0);

        let robot1 = Robot::new(1, 10.0, 10.0); // Distance ≈ 14.14
        let mut robot2 = Robot::new(2, 20.0, 20.0); // Distance ≈ 28.28
        robot2.battery = 10.0;

        assert!(battery_checker(&robot1));
        assert!(!battery_checker(&robot2));

        assert!(position_checker(&robot1));
        assert!(!position_checker(&robot2));
    }

    #[test]
    fn test_function_pointers_vs_closures() {
        let (fp_result, closure_result) = FunctionPointerDemo::compare_approaches();

        assert_eq!(fp_result, 20); // 10 * 2
        assert_eq!(closure_result, 50); // 10 * 5
    }

    #[test]
    fn test_boxed_closures() {
        let robot = Robot::new(1, 10.0, 20.0);

        let boxed_command: Box<dyn Fn(&Robot) -> String> = Box::new(|r| {
            format!("Robot {} status: {}", r.id, r.status)
        });

        let result = RobotCommandSystem::execute_boxed_command(&robot, boxed_command);
        assert!(result.contains("Robot 1"));
        assert!(result.contains("active"));
    }
}

// Student exercises
pub mod exercises {
    use super::*;

    // Exercise 1: Implement a command pattern using Fn traits
    pub struct Command {
        // TODO: Store a boxed function that can modify a robot
    }

    impl Command {
        pub fn new<F>(action: F) -> Self
        where
            F: FnOnce(&mut Robot) -> String + 'static,
        {
            // TODO: Create command that stores the action
            unimplemented!("Create command")
        }

        pub fn execute(self, robot: &mut Robot) -> String {
            // TODO: Execute the stored command
            unimplemented!("Execute command")
        }
    }

    // Exercise 2: Create a robot behavior system
    pub struct BehaviorSystem {
        behaviors: Vec<Box<dyn FnMut(&mut Robot)>>,
    }

    impl BehaviorSystem {
        pub fn new() -> Self {
            // TODO: Initialize behavior system
            unimplemented!("Initialize behavior system")
        }

        pub fn add_behavior<F>(&mut self, behavior: F)
        where
            F: FnMut(&mut Robot) + 'static,
        {
            // TODO: Add behavior to the system
            unimplemented!("Add behavior")
        }

        pub fn update(&mut self, robot: &mut Robot) {
            // TODO: Execute all behaviors on the robot
            unimplemented!("Update behaviors")
        }
    }

    // Exercise 3: Implement a filter system using different Fn traits
    pub struct RobotFilter;

    impl RobotFilter {
        pub fn filter_by<F>(robots: &[Robot], predicate: F) -> Vec<&Robot>
        where
            F: Fn(&Robot) -> bool,
        {
            // TODO: Filter robots using the predicate
            unimplemented!("Filter robots")
        }

        pub fn find_first<F>(robots: &[Robot], predicate: F) -> Option<&Robot>
        where
            F: Fn(&Robot) -> bool,
        {
            // TODO: Find first robot matching predicate
            unimplemented!("Find first robot")
        }

        pub fn transform<F, T>(robots: &[Robot], transformer: F) -> Vec<T>
        where
            F: Fn(&Robot) -> T,
        {
            // TODO: Transform robots using the function
            unimplemented!("Transform robots")
        }
    }

    // Exercise 4: Create a stateful robot processor
    pub struct StatefulProcessor {
        total_processed: usize,
        average_battery: f64,
    }

    impl StatefulProcessor {
        pub fn new() -> Self {
            // TODO: Initialize processor
            unimplemented!("Initialize processor")
        }

        pub fn process_with_state<F>(&mut self, robots: &mut [Robot], mut processor: F)
        where
            F: FnMut(&mut Robot, &mut Self),
        {
            // TODO: Process robots while updating internal state
            unimplemented!("Process with state")
        }

        pub fn get_stats(&self) -> (usize, f64) {
            // TODO: Return processing statistics
            unimplemented!("Get statistics")
        }
    }

    // Exercise 5: Implement closure composition
    pub struct ClosureComposer;

    impl ClosureComposer {
        pub fn compose<F, G, T, U, V>(f: F, g: G) -> impl Fn(T) -> V
        where
            F: Fn(T) -> U,
            G: Fn(U) -> V,
        {
            move |x| g(f(x))
        }

        pub fn chain_transforms<F1, F2, F3>(
            transform1: F1,
            transform2: F2,
            transform3: F3,
        ) -> impl Fn(&Robot) -> String
        where
            F1: Fn(&Robot) -> f64,
            F2: Fn(f64) -> i32,
            F3: Fn(i32) -> String,
        {
            move |robot| transform3(transform2(transform1(robot)))
        }
    }
}