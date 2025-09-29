// Learning Tests for Level 20, Task 2: Closure Capturing and Moving
// Understanding closure capture modes, lifetime management, and move semantics

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

// Robot struct for closure demonstrations
#[derive(Debug, Clone)]
pub struct Robot {
    pub id: u32,
    pub name: String,
    pub position: (f64, f64),
    pub energy: f64,
    pub inventory: Vec<String>,
}

impl Robot {
    pub fn new(id: u32, name: String) -> Self {
        Robot {
            id,
            name,
            position: (0.0, 0.0),
            energy: 100.0,
            inventory: Vec::new(),
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.position = (x, y);
        self.energy -= 1.0;
    }

    pub fn add_item(&mut self, item: String) {
        self.inventory.push(item);
    }

    pub fn get_info(&self) -> String {
        format!("Robot {} at {:?} with {:.1} energy", self.name, self.position, self.energy)
    }
}

// Demonstration of different capture modes
pub struct ClosureCaptureDemo;

impl ClosureCaptureDemo {
    // Capturing by reference (immutable)
    pub fn create_battery_monitor(threshold: f64) -> impl Fn(&Robot) -> bool {
        // Captures threshold by value (Copy type)
        move |robot| robot.energy > threshold
    }

    // Capturing by reference with environment access
    pub fn create_name_filter(robots: &[Robot]) -> impl Fn(&str) -> bool + '_ {
        // Captures robots by reference with lifetime
        |name| robots.iter().any(|r| r.name == name)
    }

    // Capturing mutable reference
    pub fn create_counter() -> impl FnMut() -> usize {
        let mut count = 0;
        move || {
            count += 1;
            count
        }
    }

    // Capturing owned data with move
    pub fn create_ownership_demo(data: Vec<String>) -> impl Fn() -> String {
        move || {
            format!("Captured {} items", data.len())
        }
    }
}

// Environment and scope demonstrations
pub struct EnvironmentCapture {
    pub base_energy: f64,
    pub multiplier: f64,
}

impl EnvironmentCapture {
    pub fn new(base: f64, mult: f64) -> Self {
        EnvironmentCapture {
            base_energy: base,
            multiplier: mult,
        }
    }

    // Method that creates closure capturing self
    pub fn create_energy_calculator(&self) -> impl Fn(f64) -> f64 + '_ {
        |distance| self.base_energy - (distance * self.multiplier)
    }

    // Method that moves self into closure
    pub fn create_owned_calculator(self) -> impl Fn(f64) -> f64 {
        move |distance| self.base_energy - (distance * self.multiplier)
    }

    // Creating closure that captures multiple environment variables
    pub fn create_complex_capture(robots: Vec<Robot>, config: HashMap<String, f64>) -> impl Fn(usize) -> Option<String> {
        move |index| {
            robots.get(index).map(|robot| {
                let bonus = config.get("bonus").unwrap_or(&0.0);
                format!("{} (energy: {:.1} + {:.1})", robot.name, robot.energy, bonus)
            })
        }
    }
}

// Closure lifetime management
pub struct LifetimeManagement;

impl LifetimeManagement {
    // Function that returns closure with explicit lifetime
    pub fn create_validator<'a>(valid_names: &'a [String]) -> impl Fn(&str) -> bool + 'a {
        move |name| valid_names.iter().any(|valid| valid == name)
    }

    // Function demonstrating closure that outlives its creator
    pub fn create_stateful_processor(initial_state: i32) -> Box<dyn FnMut(i32) -> i32> {
        let mut state = initial_state;
        Box::new(move |input| {
            state += input;
            state
        })
    }

    // Closure with shared ownership using Rc
    pub fn create_shared_closure(data: Rc<RefCell<Vec<String>>>) -> impl Fn(String) {
        move |item| {
            data.borrow_mut().push(item);
        }
    }
}

// Move semantics and ownership transfer
pub struct MoveSemantics;

impl MoveSemantics {
    // Demonstrates when move is required
    pub fn create_thread_safe_closure(robots: Vec<Robot>) -> impl Fn() -> usize + Send + 'static {
        move || robots.len()
    }

    // Partial moves and reborrowing
    pub fn partial_move_demo() -> (impl Fn() -> String, impl Fn() -> usize) {
        let data = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let len = data.len();

        // This moves data into the first closure
        let string_closure = move || data.join(",");

        // This captures len by value (Copy)
        let len_closure = move || len;

        (string_closure, len_closure)
    }

    // Clone before move pattern
    pub fn clone_before_move(robot: Robot) -> (impl Fn() -> String, Robot) {
        let robot_clone = robot.clone();
        let closure = move || robot_clone.get_info();
        (closure, robot)
    }
}

// Advanced capture patterns
pub struct AdvancedCapture;

impl AdvancedCapture {
    // Capturing by different modes in same closure
    pub fn mixed_capture_demo(
        immutable_data: &str,
        mut mutable_data: Vec<String>,
        owned_data: String,
    ) -> impl FnMut() -> String {
        let captured_ref = immutable_data.to_string(); // Convert to owned to avoid lifetime issues
        move || {
            mutable_data.push(owned_data.clone()); // Both moved into closure
            format!("{}: {:?}", captured_ref, mutable_data)
        }
    }

    // Closure that captures and modifies external state
    pub fn create_state_modifier() -> (impl FnMut() -> i32, Rc<RefCell<i32>>) {
        let state = Rc::new(RefCell::new(0));
        let state_clone = state.clone();

        let modifier = move || {
            let mut value = state_clone.borrow_mut();
            *value += 1;
            *value
        };

        (modifier, state)
    }

    // Closure composition with different capture modes
    pub fn compose_closures<F, G>(f: F, g: G) -> impl Fn(i32) -> String
    where
        F: Fn(i32) -> i32,
        G: Fn(i32) -> String,
    {
        move |x| g(f(x))
    }
}

// Practical closure patterns for robot systems
pub struct RobotClosurePatterns;

impl RobotClosurePatterns {
    // Command pattern with closures
    pub fn create_command_queue() -> Vec<Box<dyn FnOnce(&mut Robot)>> {
        let mut commands: Vec<Box<dyn FnOnce(&mut Robot)>> = Vec::new();

        // Move command
        commands.push(Box::new(|robot| robot.move_to(10.0, 20.0)));

        // Add item command with captured data
        let item = "power_cell".to_string();
        commands.push(Box::new(move |robot| robot.add_item(item)));

        commands
    }

    // Factory pattern with configuration closure
    pub fn create_robot_factory(config: HashMap<String, f64>) -> impl Fn(u32, String) -> Robot {
        move |id, name| {
            let mut robot = Robot::new(id, name);
            if let Some(energy) = config.get("initial_energy") {
                robot.energy = *energy;
            }
            robot
        }
    }

    // Observer pattern with closures
    pub fn create_observers() -> Vec<Box<dyn Fn(&Robot)>> {
        let mut observers: Vec<Box<dyn Fn(&Robot)>> = Vec::new();

        // Energy monitor
        let low_energy_threshold = 20.0;
        observers.push(Box::new(move |robot| {
            if robot.energy < low_energy_threshold {
                println!("Warning: {} has low energy!", robot.name);
            }
        }));

        // Position monitor
        let max_distance = 100.0;
        observers.push(Box::new(move |robot| {
            let distance = (robot.position.0.powi(2) + robot.position.1.powi(2)).sqrt();
            if distance > max_distance {
                println!("Warning: {} is too far from base!", robot.name);
            }
        }));

        observers
    }

    // Strategy pattern with closures
    pub fn create_movement_strategies() -> HashMap<String, Box<dyn Fn(&Robot, f64, f64) -> (f64, f64)>> {
        let mut strategies = HashMap::new();

        // Direct movement
        strategies.insert("direct".to_string(), Box::new(|_robot: &Robot, x: f64, y: f64| (x, y)) as Box<dyn Fn(&Robot, f64, f64) -> (f64, f64)>);

        // Relative movement
        strategies.insert("relative".to_string(), Box::new(|robot: &Robot, dx: f64, dy: f64| {
            (robot.position.0 + dx, robot.position.1 + dy)
        }) as Box<dyn Fn(&Robot, f64, f64) -> (f64, f64)>);

        // Energy-efficient movement (closer moves)
        let efficiency_factor = 0.5;
        strategies.insert("efficient".to_string(), Box::new(move |robot: &Robot, x: f64, y: f64| {
            let current = robot.position;
            let target_x = current.0 + (x - current.0) * efficiency_factor;
            let target_y = current.1 + (y - current.1) * efficiency_factor;
            (target_x, target_y)
        }) as Box<dyn Fn(&Robot, f64, f64) -> (f64, f64)>);

        strategies
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_modes() {
        let threshold = 50.0;
        let monitor = ClosureCaptureDemo::create_battery_monitor(threshold);

        let robot1 = Robot::new(1, "Test1".to_string());
        let mut robot2 = Robot::new(2, "Test2".to_string());
        robot2.energy = 30.0;

        assert!(monitor(&robot1));
        assert!(!monitor(&robot2));
    }

    #[test]
    fn test_mutable_capture() {
        let mut counter = ClosureCaptureDemo::create_counter();

        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_ownership_transfer() {
        let data = vec!["item1".to_string(), "item2".to_string()];
        let closure = ClosureCaptureDemo::create_ownership_demo(data);

        // data is moved, so it's no longer accessible here
        let result = closure();
        assert!(result.contains("2 items"));
    }

    #[test]
    fn test_environment_capture() {
        let env = EnvironmentCapture::new(100.0, 2.0);
        let calculator = env.create_energy_calculator();

        assert_eq!(calculator(10.0), 80.0);
        assert_eq!(calculator(5.0), 90.0);
    }

    #[test]
    fn test_lifetime_management() {
        let valid_names = vec!["Robot1".to_string(), "Robot2".to_string()];
        let validator = LifetimeManagement::create_validator(&valid_names);

        assert!(validator("Robot1"));
        assert!(!validator("Robot3"));
    }

    #[test]
    fn test_shared_ownership() {
        let data = Rc::new(RefCell::new(Vec::new()));
        let closure = LifetimeManagement::create_shared_closure(data.clone());

        closure("test".to_string());
        assert_eq!(data.borrow().len(), 1);
        assert_eq!(data.borrow()[0], "test");
    }

    #[test]
    fn test_move_semantics() {
        let robots = vec![
            Robot::new(1, "Bot1".to_string()),
            Robot::new(2, "Bot2".to_string()),
        ];

        let closure = MoveSemantics::create_thread_safe_closure(robots);
        assert_eq!(closure(), 2);
    }

    #[test]
    fn test_partial_moves() {
        let (string_closure, len_closure) = MoveSemantics::partial_move_demo();

        assert_eq!(string_closure(), "a,b,c");
        assert_eq!(len_closure(), 3);
    }

    #[test]
    fn test_clone_before_move() {
        let robot = Robot::new(1, "TestBot".to_string());
        let (closure, original_robot) = MoveSemantics::clone_before_move(robot);

        let info = closure();
        assert!(info.contains("TestBot"));
        assert_eq!(original_robot.name, "TestBot");
    }

    #[test]
    fn test_state_modifier() {
        let (mut modifier, state) = AdvancedCapture::create_state_modifier();

        assert_eq!(modifier(), 1);
        assert_eq!(modifier(), 2);
        assert_eq!(*state.borrow(), 2);
    }

    #[test]
    fn test_command_queue() {
        let mut robot = Robot::new(1, "TestBot".to_string());
        let commands = RobotClosurePatterns::create_command_queue();

        for command in commands {
            command(&mut robot);
        }

        assert_eq!(robot.position, (10.0, 20.0));
        assert!(robot.inventory.contains(&"power_cell".to_string()));
    }

    #[test]
    fn test_robot_factory() {
        let mut config = HashMap::new();
        config.insert("initial_energy".to_string(), 75.0);

        let factory = RobotClosurePatterns::create_robot_factory(config);
        let robot = factory(1, "FactoryBot".to_string());

        assert_eq!(robot.energy, 75.0);
        assert_eq!(robot.name, "FactoryBot");
    }

    #[test]
    fn test_movement_strategies() {
        let strategies = RobotClosurePatterns::create_movement_strategies();
        let robot = Robot::new(1, "TestBot".to_string());

        let direct = strategies.get("direct").unwrap();
        assert_eq!(direct(&robot, 10.0, 20.0), (10.0, 20.0));

        let relative = strategies.get("relative").unwrap();
        assert_eq!(relative(&robot, 5.0, 5.0), (5.0, 5.0));
    }
}

// Student exercises
pub mod exercises {
    use super::*;

    // Exercise 1: Implement a closure-based event system
    pub struct EventSystem {
        // TODO: Store event handlers as closures
    }

    impl EventSystem {
        pub fn new() -> Self {
            // TODO: Initialize event system
            unimplemented!("Initialize event system")
        }

        pub fn subscribe<F>(&mut self, event_type: String, handler: F)
        where
            F: Fn(&str) + 'static,
        {
            // TODO: Subscribe to events with closure handlers
            unimplemented!("Subscribe to event")
        }

        pub fn emit(&self, event_type: &str, data: &str) {
            // TODO: Emit events to all subscribed handlers
            unimplemented!("Emit event")
        }
    }

    // Exercise 2: Create a closure-based middleware system
    pub struct MiddlewareChain {
        // TODO: Store middleware functions as closures
    }

    impl MiddlewareChain {
        pub fn new() -> Self {
            // TODO: Initialize middleware chain
            unimplemented!("Initialize middleware")
        }

        pub fn add<F>(&mut self, middleware: F)
        where
            F: Fn(&str) -> String + 'static,
        {
            // TODO: Add middleware to the chain
            unimplemented!("Add middleware")
        }

        pub fn process(&self, input: &str) -> String {
            // TODO: Process input through all middleware
            unimplemented!("Process through middleware")
        }
    }

    // Exercise 3: Implement a closure-based caching system
    pub struct Cache<T> {
        // TODO: Store cached data and cache strategies
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T: Clone> Cache<T> {
        pub fn new() -> Self {
            // TODO: Initialize cache
            unimplemented!("Initialize cache")
        }

        pub fn get_or_compute<F>(&mut self, key: &str, compute: F) -> T
        where
            F: FnOnce() -> T,
        {
            // TODO: Get from cache or compute and store
            unimplemented!("Get or compute")
        }

        pub fn set_eviction_policy<F>(&mut self, policy: F)
        where
            F: Fn(&str, &T) -> bool + 'static,
        {
            // TODO: Set eviction policy using closure
            unimplemented!("Set eviction policy")
        }
    }

    // Exercise 4: Create a functional pipeline with closures
    pub struct Pipeline<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Pipeline<T> {
        pub fn new(initial: T) -> Self {
            // TODO: Initialize pipeline with starting value
            unimplemented!("Initialize pipeline")
        }

        pub fn map<U, F>(self, func: F) -> Pipeline<U>
        where
            F: FnOnce(T) -> U,
        {
            // TODO: Transform pipeline value
            unimplemented!("Map pipeline")
        }

        pub fn filter<F>(self, predicate: F) -> Option<Pipeline<T>>
        where
            F: FnOnce(&T) -> bool,
        {
            // TODO: Filter pipeline value
            unimplemented!("Filter pipeline")
        }

        pub fn execute(self) -> T {
            // TODO: Execute the pipeline and return result
            unimplemented!("Execute pipeline")
        }
    }

    // Exercise 5: Implement a closure-based state machine
    pub struct StateMachine<S> {
        // TODO: Store state and transition functions
        _phantom: std::marker::PhantomData<S>,
    }

    impl<S> StateMachine<S> {
        pub fn new(initial_state: S) -> Self {
            // TODO: Initialize state machine
            unimplemented!("Initialize state machine")
        }

        pub fn add_transition<F>(&mut self, from: S, to: S, condition: F)
        where
            S: Clone + PartialEq,
            F: Fn(&S) -> bool + 'static,
        {
            // TODO: Add state transition with condition closure
            unimplemented!("Add transition")
        }

        pub fn update(&mut self) {
            // TODO: Update state machine based on transitions
            unimplemented!("Update state machine")
        }

        pub fn current_state(&self) -> &S {
            // TODO: Get current state
            unimplemented!("Get current state")
        }
    }
}