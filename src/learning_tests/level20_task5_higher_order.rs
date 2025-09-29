// Learning Tests for Level 20, Task 5: Higher-Order Functions and Functional Patterns
// Advanced functional programming concepts and higher-order function patterns

use std::collections::HashMap;
use std::fmt::Debug;

// Robot data structures for higher-order function demonstrations
#[derive(Debug, Clone, PartialEq)]
pub struct Robot {
    pub id: u32,
    pub name: String,
    pub position: (f64, f64),
    pub energy: f64,
    pub capabilities: Vec<String>,
    pub active: bool,
}

impl Robot {
    pub fn new(id: u32, name: String) -> Self {
        Robot {
            id,
            name,
            position: (0.0, 0.0),
            energy: 100.0,
            capabilities: Vec::new(),
            active: true,
        }
    }

    pub fn with_capability(mut self, capability: String) -> Self {
        self.capabilities.push(capability);
        self
    }

    pub fn at_position(mut self, x: f64, y: f64) -> Self {
        self.position = (x, y);
        self
    }

    pub fn with_energy(mut self, energy: f64) -> Self {
        self.energy = energy;
        self
    }
}

// Higher-order function utilities
pub struct HigherOrder;

impl HigherOrder {
    // Map function for any collection
    pub fn map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
    where
        F: Fn(T) -> U,
    {
        items.into_iter().map(f).collect()
    }

    // Filter function
    pub fn filter<T, F>(items: Vec<T>, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        items.into_iter().filter(|item| predicate(item)).collect()
    }

    // Reduce/fold function
    pub fn reduce<T, U, F>(items: Vec<T>, initial: U, reducer: F) -> U
    where
        F: Fn(U, T) -> U,
    {
        items.into_iter().fold(initial, reducer)
    }

    // Find function
    pub fn find<T, F>(items: &[T], predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        items.iter().find(|&item| predicate(item))
    }

    // Partition function
    pub fn partition<T, F>(items: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)
    where
        F: Fn(&T) -> bool,
    {
        items.into_iter().partition(|item| predicate(item))
    }

    // Group by function
    pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> HashMap<K, Vec<T>>
    where
        K: Eq + std::hash::Hash,
        F: Fn(&T) -> K,
    {
        let mut groups = HashMap::new();
        for item in items {
            let key = key_fn(&item);
            groups.entry(key).or_insert_with(Vec::new).push(item);
        }
        groups
    }
}

// Functional programming patterns
pub struct FunctionalPatterns;

impl FunctionalPatterns {
    // Maybe/Option monad operations
    pub fn maybe_map<T, U, F>(option: Option<T>, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        option.map(f)
    }

    pub fn maybe_and_then<T, U, F>(option: Option<T>, f: F) -> Option<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        option.and_then(f)
    }

    // Result monad operations
    pub fn result_map<T, U, E, F>(result: Result<T, E>, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
    {
        result.map(f)
    }

    pub fn result_and_then<T, U, E, F>(result: Result<T, E>, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        result.and_then(f)
    }

    // Sequence operations (convert Vec<Option<T>> to Option<Vec<T>>)
    pub fn sequence_options<T>(options: Vec<Option<T>>) -> Option<Vec<T>> {
        let mut results = Vec::new();
        for option in options {
            match option {
                Some(value) => results.push(value),
                None => return None,
            }
        }
        Some(results)
    }

    // Traverse operation
    pub fn traverse<T, U, F>(items: Vec<T>, f: F) -> Option<Vec<U>>
    where
        F: Fn(T) -> Option<U>,
    {
        Self::sequence_options(items.into_iter().map(f).collect())
    }
}

// Lazy evaluation and iterators
pub struct LazyEvaluation;

impl LazyEvaluation {
    // Create a lazy range
    pub fn range(start: i32, end: i32) -> impl Iterator<Item = i32> {
        start..end
    }

    // Infinite sequence generator
    pub fn fibonacci() -> impl Iterator<Item = u64> {
        let mut a = 0;
        let mut b = 1;
        std::iter::from_fn(move || {
            let result = a;
            let next = a + b;
            a = b;
            b = next;
            Some(result)
        })
    }

    // Take while predicate is true
    pub fn take_while<T, I, F>(iter: I, predicate: F) -> impl Iterator<Item = T>
    where
        I: Iterator<Item = T>,
        F: Fn(&T) -> bool,
    {
        iter.take_while(predicate)
    }

    // Chain multiple iterators
    pub fn chain_iterators<T, I1, I2>(iter1: I1, iter2: I2) -> impl Iterator<Item = T>
    where
        I1: Iterator<Item = T>,
        I2: Iterator<Item = T>,
    {
        iter1.chain(iter2)
    }

    // Cycle iterator infinitely
    pub fn cycle<T, I>(iter: I) -> impl Iterator<Item = T>
    where
        I: Iterator<Item = T> + Clone,
        T: Clone,
    {
        iter.cycle()
    }
}

// Function factories and builders
pub struct FunctionFactory;

impl FunctionFactory {
    // Create a predicate function
    pub fn create_predicate<T, F>(condition: F) -> impl Fn(&T) -> bool
    where
        F: Fn(&T) -> bool,
    {
        condition
    }

    // Create a transformer function
    pub fn create_transformer<T, U, F>(transform: F) -> impl Fn(T) -> U
    where
        F: Fn(T) -> U,
    {
        transform
    }

    // Create a validator with custom error message
    pub fn create_validator<T, F>(
        validation: F,
        error_msg: String,
    ) -> impl Fn(&T) -> Result<(), String>
    where
        F: Fn(&T) -> bool,
    {
        move |item| {
            if validation(item) {
                Ok(())
            } else {
                Err(error_msg.clone())
            }
        }
    }

    // Create a comparator function
    pub fn create_comparator<T, F>(compare: F) -> impl Fn(&T, &T) -> std::cmp::Ordering
    where
        F: Fn(&T, &T) -> std::cmp::Ordering,
    {
        compare
    }

    // Create a memoized function
    pub fn create_memoized<T, U, F>(mut func: F) -> impl FnMut(&T) -> U
    where
        T: Clone + Eq + std::hash::Hash,
        U: Clone,
        F: FnMut(&T) -> U,
    {
        let mut cache: HashMap<T, U> = HashMap::new();
        move |input| {
            if let Some(cached) = cache.get(input) {
                cached.clone()
            } else {
                let result = func(input);
                cache.insert(input.clone(), result.clone());
                result
            }
        }
    }
}

// Robot fleet management using higher-order functions
pub struct RobotFleetManager {
    robots: Vec<Robot>,
}

impl RobotFleetManager {
    pub fn new(robots: Vec<Robot>) -> Self {
        RobotFleetManager { robots }
    }

    // Transform all robots
    pub fn transform_robots<F>(mut self, transformer: F) -> Self
    where
        F: Fn(Robot) -> Robot,
    {
        self.robots = self.robots.into_iter().map(transformer).collect();
        self
    }

    // Filter robots by condition
    pub fn filter_robots<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&Robot) -> bool,
    {
        self.robots = self.robots.into_iter().filter(|r| predicate(r)).collect();
        self
    }

    // Find robot by condition
    pub fn find_robot<F>(&self, predicate: F) -> Option<&Robot>
    where
        F: Fn(&Robot) -> bool,
    {
        self.robots.iter().find(|&robot| predicate(robot))
    }

    // Group robots by some criterion
    pub fn group_by<K, F>(&self, key_fn: F) -> HashMap<K, Vec<&Robot>>
    where
        K: Eq + std::hash::Hash,
        F: Fn(&Robot) -> K,
    {
        let mut groups = HashMap::new();
        for robot in &self.robots {
            let key = key_fn(robot);
            groups.entry(key).or_insert_with(Vec::new).push(robot);
        }
        groups
    }

    // Apply operation to each robot
    pub fn for_each<F>(&self, operation: F)
    where
        F: Fn(&Robot),
    {
        self.robots.iter().for_each(operation);
    }

    // Reduce robots to a single value
    pub fn reduce<U, F>(&self, initial: U, reducer: F) -> U
    where
        F: Fn(U, &Robot) -> U,
    {
        self.robots.iter().fold(initial, reducer)
    }

    // Count robots matching condition
    pub fn count_where<F>(&self, predicate: F) -> usize
    where
        F: Fn(&Robot) -> bool,
    {
        self.robots.iter().filter(|&robot| predicate(robot)).count()
    }

    // Check if any robot matches condition
    pub fn any<F>(&self, predicate: F) -> bool
    where
        F: Fn(&Robot) -> bool,
    {
        self.robots.iter().any(|robot| predicate(robot))
    }

    // Check if all robots match condition
    pub fn all<F>(&self, predicate: F) -> bool
    where
        F: Fn(&Robot) -> bool,
    {
        self.robots.iter().all(|robot| predicate(robot))
    }

    // Sort robots by some key
    pub fn sort_by<F, K>(mut self, key_fn: F) -> Self
    where
        F: Fn(&Robot) -> K,
        K: Ord,
    {
        self.robots.sort_by_key(|robot| key_fn(robot));
        self
    }

    // Take first n robots
    pub fn take(mut self, n: usize) -> Self {
        self.robots.truncate(n);
        self
    }

    // Skip first n robots
    pub fn skip(mut self, n: usize) -> Self {
        if n < self.robots.len() {
            self.robots = self.robots.into_iter().skip(n).collect();
        } else {
            self.robots.clear();
        }
        self
    }

    pub fn robots(&self) -> &[Robot] {
        &self.robots
    }
}

// Functional validation system
pub struct ValidationSystem;

impl ValidationSystem {
    // Combine multiple validators
    pub fn combine_validators<T, F1, F2>(
        validator1: F1,
        validator2: F2,
    ) -> impl Fn(&T) -> Result<(), Vec<String>>
    where
        F1: Fn(&T) -> Result<(), String>,
        F2: Fn(&T) -> Result<(), String>,
    {
        move |item| {
            let mut errors = Vec::new();

            if let Err(e1) = validator1(item) {
                errors.push(e1);
            }

            if let Err(e2) = validator2(item) {
                errors.push(e2);
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors)
            }
        }
    }

    // Create robot validators
    pub fn create_robot_validators() -> Vec<Box<dyn Fn(&Robot) -> Result<(), String>>> {
        vec![
            Box::new(|robot| {
                if robot.energy > 0.0 {
                    Ok(())
                } else {
                    Err("Robot has no energy".to_string())
                }
            }),
            Box::new(|robot| {
                if robot.active {
                    Ok(())
                } else {
                    Err("Robot is not active".to_string())
                }
            }),
            Box::new(|robot| {
                if !robot.name.is_empty() {
                    Ok(())
                } else {
                    Err("Robot has no name".to_string())
                }
            }),
        ]
    }

    // Validate robot with all validators
    pub fn validate_robot(robot: &Robot) -> Result<(), Vec<String>> {
        let validators = Self::create_robot_validators();
        let mut errors = Vec::new();

        for validator in validators {
            if let Err(error) = validator(robot) {
                errors.push(error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// Currying and partial application
pub struct Currying;

impl Currying {
    // Curry a 2-argument function
    pub fn curry2<A, B, C, F>(f: F) -> Box<dyn Fn(A) -> Box<dyn Fn(B) -> C>>
    where
        F: Fn(A, B) -> C + 'static,
        A: Clone + 'static,
        B: 'static,
        C: 'static,
    {
        use std::rc::Rc;
        let f = Rc::new(f);
        Box::new(move |a| {
            let a_clone = a.clone();
            let f_clone = f.clone();
            Box::new(move |b| f_clone(a_clone.clone(), b)) as Box<dyn Fn(B) -> C>
        })
    }

    // Partial application of first argument
    pub fn partial<A, B, C, F>(f: F, a: A) -> impl Fn(B) -> C
    where
        F: Fn(A, B) -> C,
        A: Clone,
    {
        move |b| f(a.clone(), b)
    }

    // Flip argument order
    pub fn flip<A, B, C, F>(f: F) -> impl Fn(B, A) -> C
    where
        F: Fn(A, B) -> C,
    {
        move |b, a| f(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_higher_order_functions() {
        let numbers = vec![1, 2, 3, 4, 5];

        let doubled = HigherOrder::map(numbers.clone(), |x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);

        let evens = HigherOrder::filter(numbers.clone(), |&x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);

        let sum = HigherOrder::reduce(numbers.clone(), 0, |acc, x| acc + x);
        assert_eq!(sum, 15);

        let found = HigherOrder::find(&numbers, |&&x| x == 3);
        assert_eq!(found, Some(&3));

        let (evens, odds) = HigherOrder::partition(numbers, |&x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_functional_patterns() {
        // Maybe operations
        let some_value = Some(5);
        let mapped = FunctionalPatterns::maybe_map(some_value, |x| x * 2);
        assert_eq!(mapped, Some(10));

        let chained = FunctionalPatterns::maybe_and_then(Some(5), |x| {
            if x > 0 { Some(x * 2) } else { None }
        });
        assert_eq!(chained, Some(10));

        // Result operations
        let ok_value: Result<i32, String> = Ok(5);
        let mapped_result = FunctionalPatterns::result_map(ok_value, |x| x * 2);
        assert_eq!(mapped_result, Ok(10));
    }

    #[test]
    fn test_lazy_evaluation() {
        let range: Vec<i32> = LazyEvaluation::range(1, 5).collect();
        assert_eq!(range, vec![1, 2, 3, 4]);

        let first_fibs: Vec<u64> = LazyEvaluation::fibonacci().take(5).collect();
        assert_eq!(first_fibs, vec![0, 1, 1, 2, 3]);

        let chained: Vec<i32> = LazyEvaluation::chain_iterators(1..3, 3..5).collect();
        assert_eq!(chained, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_robot_fleet_manager() {
        let robots = vec![
            Robot::new(1, "Bot1".to_string()).with_energy(50.0),
            Robot::new(2, "Bot2".to_string()).with_energy(80.0),
            Robot::new(3, "Bot3".to_string()).with_energy(20.0),
        ];

        let manager = RobotFleetManager::new(robots);

        // Test filtering
        let high_energy_count = manager.count_where(|robot| robot.energy > 30.0);
        assert_eq!(high_energy_count, 2);

        // Test finding
        let found = manager.find_robot(|robot| robot.energy == 80.0);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, 2);

        // Test any/all
        assert!(manager.any(|robot| robot.energy > 70.0));
        assert!(!manager.all(|robot| robot.energy > 70.0));

        // Test reduce
        let total_energy = manager.reduce(0.0, |acc, robot| acc + robot.energy);
        assert_eq!(total_energy, 150.0);
    }

    #[test]
    fn test_function_factory() {
        // Test validator creation
        let energy_validator = FunctionFactory::create_validator(
            |robot: &Robot| robot.energy > 50.0,
            "Robot energy too low".to_string()
        );

        let robot1 = Robot::new(1, "Bot1".to_string()).with_energy(60.0);
        let robot2 = Robot::new(2, "Bot2".to_string()).with_energy(30.0);

        assert!(energy_validator(&robot1).is_ok());
        assert!(energy_validator(&robot2).is_err());

        // Test predicate creation
        let high_energy_predicate = FunctionFactory::create_predicate(|robot: &Robot| robot.energy > 50.0);
        assert!(high_energy_predicate(&robot1));
        assert!(!high_energy_predicate(&robot2));
    }

    #[test]
    fn test_validation_system() {
        let robot1 = Robot::new(1, "Bot1".to_string()).with_energy(60.0);
        let mut robot2 = Robot::new(2, "Bot2".to_string()).with_energy(0.0);
        robot2.active = false;

        let result1 = ValidationSystem::validate_robot(&robot1);
        assert!(result1.is_ok());

        let result2 = ValidationSystem::validate_robot(&robot2);
        assert!(result2.is_err());
        let errors = result2.unwrap_err();
        assert_eq!(errors.len(), 2); // No energy and not active
    }

    #[test]
    fn test_currying() {
        let add = |a: i32, b: i32| a + b;
        let curried_add = Currying::curry2(add);

        let add_5 = curried_add(5);
        assert_eq!(add_5(3), 8);

        let multiply = |a: i32, b: i32| a * b;
        let multiply_by_3 = Currying::partial(multiply, 3);
        assert_eq!(multiply_by_3(4), 12);

        let subtract = |a: i32, b: i32| a - b;
        let flipped_subtract = Currying::flip(subtract);
        assert_eq!(flipped_subtract(3, 10), 7); // 10 - 3
    }

    #[test]
    fn test_chaining_operations() {
        let robots = vec![
            Robot::new(1, "Alpha".to_string()).with_energy(90.0).at_position(10.0, 20.0),
            Robot::new(2, "Beta".to_string()).with_energy(30.0).at_position(5.0, 15.0),
            Robot::new(3, "Gamma".to_string()).with_energy(70.0).at_position(0.0, 0.0),
            Robot::new(4, "Delta".to_string()).with_energy(40.0).at_position(8.0, 12.0),
        ];

        let result = RobotFleetManager::new(robots)
            .filter_robots(|robot| robot.energy > 40.0)
            .sort_by(|robot| (robot.energy as i32)) // Sort by energy
            .take(2)
            .robots()
            .to_vec();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "Gamma"); // 70 energy
        assert_eq!(result[1].name, "Alpha"); // 90 energy
    }

    #[test]
    fn test_group_by() {
        let robots = vec![
            Robot::new(1, "Bot1".to_string()).with_energy(90.0),
            Robot::new(2, "Bot2".to_string()).with_energy(30.0),
            Robot::new(3, "Bot3".to_string()).with_energy(70.0),
            Robot::new(4, "Bot4".to_string()).with_energy(20.0),
        ];

        let manager = RobotFleetManager::new(robots);

        let grouped = manager.group_by(|robot| {
            if robot.energy > 50.0 { "high" } else { "low" }
        });

        assert_eq!(grouped.get("high").unwrap().len(), 2);
        assert_eq!(grouped.get("low").unwrap().len(), 2);
    }
}

// Student exercises
pub mod exercises {
    use super::*;

    // Exercise 1: Implement a stream processing system
    pub struct StreamProcessor<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> StreamProcessor<T> {
        pub fn new() -> Self {
            // TODO: Initialize stream processor
            unimplemented!("Initialize stream processor")
        }

        pub fn map<U, F>(self, mapper: F) -> StreamProcessor<U>
        where
            F: Fn(T) -> U + 'static,
        {
            // TODO: Transform stream elements
            unimplemented!("Map stream")
        }

        pub fn filter<F>(self, predicate: F) -> StreamProcessor<T>
        where
            F: Fn(&T) -> bool + 'static,
        {
            // TODO: Filter stream elements
            unimplemented!("Filter stream")
        }

        pub fn fold<U, F>(self, initial: U, folder: F) -> U
        where
            F: Fn(U, T) -> U,
        {
            // TODO: Fold stream to single value
            unimplemented!("Fold stream")
        }
    }

    // Exercise 2: Implement a function composition operator
    pub struct Compose;

    impl Compose {
        pub fn compose_many<T>(functions: Vec<Box<dyn Fn(T) -> T>>) -> impl Fn(T) -> T
        where
            T: Clone,
        {
            move |mut input| {
                for func in &functions {
                    input = func(input);
                }
                input
            }
        }

        pub fn pipe<T, U, V, F, G>(f: F, g: G) -> impl Fn(T) -> V
        where
            F: Fn(T) -> U,
            G: Fn(U) -> V,
        {
            move |input| g(f(input))
        }
    }

    // Exercise 3: Implement a functional builder pattern
    pub struct RobotBuilder {
        // TODO: Store build functions
    }

    impl RobotBuilder {
        pub fn new() -> Self {
            // TODO: Initialize builder
            unimplemented!("Initialize robot builder")
        }

        pub fn with_modifier<F>(self, modifier: F) -> Self
        where
            F: Fn(Robot) -> Robot + 'static,
        {
            // TODO: Add modifier function to builder
            unimplemented!("Add modifier")
        }

        pub fn build(self, base_robot: Robot) -> Robot {
            // TODO: Apply all modifiers to create final robot
            unimplemented!("Build robot")
        }
    }

    // Exercise 4: Implement a functional state machine
    pub struct FunctionalStateMachine<S> {
        // TODO: Store state and transition functions
        _phantom: std::marker::PhantomData<S>,
    }

    impl<S> FunctionalStateMachine<S> {
        pub fn new(initial_state: S) -> Self {
            // TODO: Initialize state machine
            unimplemented!("Initialize state machine")
        }

        pub fn add_transition<F>(&mut self, transition: F)
        where
            F: Fn(S) -> S + 'static,
        {
            // TODO: Add transition function
            unimplemented!("Add transition")
        }

        pub fn step(self) -> Self {
            // TODO: Apply next transition
            unimplemented!("Step state machine")
        }

        pub fn get_state(&self) -> &S {
            // TODO: Get current state
            unimplemented!("Get state")
        }
    }

    // Exercise 5: Implement a monadic parser combinator
    pub struct Parser<T> {
        // TODO: Store parsing function
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Parser<T> {
        pub fn new<F>(parse_fn: F) -> Self
        where
            F: Fn(&str) -> Option<(T, &str)> + 'static,
        {
            // TODO: Create parser from function
            unimplemented!("Create parser")
        }

        pub fn map<U, F>(self, mapper: F) -> Parser<U>
        where
            F: Fn(T) -> U + 'static,
        {
            // TODO: Map parser result
            unimplemented!("Map parser")
        }

        pub fn and_then<U, F>(self, next: F) -> Parser<U>
        where
            F: Fn(T) -> Parser<U> + 'static,
        {
            // TODO: Chain parsers monadically
            unimplemented!("Chain parsers")
        }

        pub fn parse(&self, input: &str) -> Option<(T, &str)> {
            // TODO: Execute parser
            unimplemented!("Execute parser")
        }
    }

    // Helper functions for parser combinators
    impl Parser<char> {
        pub fn char(expected: char) -> Self {
            // TODO: Parser that matches a specific character
            unimplemented!("Char parser")
        }
    }

    impl Parser<String> {
        pub fn string(expected: &str) -> Self {
            // TODO: Parser that matches a specific string
            unimplemented!("String parser")
        }
    }
}