// Learning Tests for Level 20, Task 3: Function Composition and Chaining
// Building complex behaviors through function composition patterns

use std::collections::HashMap;

// Robot data structures for composition demonstrations
#[derive(Debug, Clone, PartialEq)]
pub struct Robot {
    pub id: u32,
    pub name: String,
    pub position: (f64, f64),
    pub energy: f64,
    pub speed: f64,
    pub sensors: Vec<String>,
}

impl Robot {
    pub fn new(id: u32, name: String) -> Self {
        Robot {
            id,
            name,
            position: (0.0, 0.0),
            energy: 100.0,
            speed: 1.0,
            sensors: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SensorReading {
    pub sensor_type: String,
    pub value: f64,
    pub timestamp: u64,
}

// Basic function composition
pub struct FunctionComposition;

impl FunctionComposition {
    // Simple function composition: f(g(x))
    pub fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(B) -> C,
        G: Fn(A) -> B,
    {
        move |x| f(g(x))
    }

    // Three-function composition: f(g(h(x)))
    pub fn compose3<F, G, H, A, B, C, D>(f: F, g: G, h: H) -> impl Fn(A) -> D
    where
        F: Fn(C) -> D,
        G: Fn(B) -> C,
        H: Fn(A) -> B,
    {
        move |x| f(g(h(x)))
    }

    // Chain multiple functions together
    pub fn chain<T>(functions: Vec<Box<dyn Fn(T) -> T>>) -> impl Fn(T) -> T
    where
        T: Clone,
    {
        move |input| {
            functions.iter().fold(input, |acc, func| func(acc))
        }
    }

    // Demonstration with robot transformations
    pub fn create_robot_pipeline() -> impl Fn(Robot) -> String {
        let energy_boost = |mut robot: Robot| {
            robot.energy += 20.0;
            robot
        };

        let speed_upgrade = |mut robot: Robot| {
            robot.speed *= 1.5;
            robot
        };

        let status_report = |robot: Robot| {
            format!("Robot {} at {:?} - Energy: {:.1}, Speed: {:.1}",
                    robot.name, robot.position, robot.energy, robot.speed)
        };

        Self::compose3(status_report, speed_upgrade, energy_boost)
    }
}

// Functional pipeline patterns
pub struct Pipeline<T> {
    value: T,
}

impl<T> Pipeline<T> {
    pub fn new(value: T) -> Self {
        Pipeline { value }
    }

    pub fn map<U, F>(self, f: F) -> Pipeline<U>
    where
        F: FnOnce(T) -> U,
    {
        Pipeline::new(f(self.value))
    }

    pub fn and_then<U, F>(self, f: F) -> Pipeline<U>
    where
        F: FnOnce(T) -> Pipeline<U>,
    {
        f(self.value)
    }

    pub fn filter<F>(self, predicate: F) -> Option<Pipeline<T>>
    where
        F: FnOnce(&T) -> bool,
    {
        if predicate(&self.value) {
            Some(self)
        } else {
            None
        }
    }

    pub fn execute(self) -> T {
        self.value
    }

    pub fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(&T),
        T: Clone,
    {
        f(&self.value);
        self
    }
}

// Result-based composition for error handling
pub struct ResultComposition;

impl ResultComposition {
    // Compose functions that return Results
    pub fn compose_results<F, G, A, B, C, E>(
        f: F,
        g: G,
    ) -> impl Fn(A) -> Result<C, E>
    where
        F: Fn(B) -> Result<C, E>,
        G: Fn(A) -> Result<B, E>,
    {
        move |x| g(x).and_then(|y| f(y))
    }

    // Robot validation pipeline
    pub fn create_validation_pipeline() -> impl Fn(Robot) -> Result<Robot, String> {
        let validate_energy = |robot: Robot| -> Result<Robot, String> {
            if robot.energy > 0.0 {
                Ok(robot)
            } else {
                Err("Robot has no energy".to_string())
            }
        };

        let validate_position = |robot: Robot| -> Result<Robot, String> {
            let distance = (robot.position.0.powi(2) + robot.position.1.powi(2)).sqrt();
            if distance <= 1000.0 {
                Ok(robot)
            } else {
                Err("Robot is too far from base".to_string())
            }
        };

        let validate_sensors = |robot: Robot| -> Result<Robot, String> {
            if !robot.sensors.is_empty() {
                Ok(robot)
            } else {
                Err("Robot has no sensors".to_string())
            }
        };

        move |robot| {
            validate_energy(robot)
                .and_then(validate_position)
                .and_then(validate_sensors)
        }
    }
}

// Combinator patterns
pub struct Combinators;

impl Combinators {
    // Identity combinator
    pub fn identity<T>() -> impl Fn(T) -> T {
        |x| x
    }

    // Constant combinator
    pub fn constant<T: Clone>(value: T) -> impl Fn(T) -> T {
        move |_| value.clone()
    }

    // Conditional combinator
    pub fn conditional<T, F, G>(predicate: F, then_branch: G, else_branch: G) -> impl Fn(T) -> T
    where
        F: Fn(&T) -> bool,
        G: Fn(T) -> T,
        T: Clone,
    {
        move |input| {
            if predicate(&input) {
                then_branch(input)
            } else {
                else_branch(input)
            }
        }
    }

    // Retry combinator
    pub fn retry<T, F, E>(mut operation: F, max_attempts: usize) -> impl FnMut(T) -> Result<T, E>
    where
        F: FnMut(T) -> Result<T, E>,
        T: Clone,
    {
        move |input| {
            let mut current_input = input;
            for _ in 0..max_attempts {
                match operation(current_input.clone()) {
                    Ok(result) => return Ok(result),
                    Err(_) if max_attempts > 1 => continue,
                    Err(e) => return Err(e),
                }
            }
            operation(current_input)
        }
    }
}

// Monadic composition patterns
pub trait Monad<T> {
    type Output<U>;

    fn unit(value: T) -> Self::Output<T>;
    fn bind<U, F>(self, f: F) -> Self::Output<U>
    where
        F: FnOnce(T) -> Self::Output<U>;
}

pub struct Maybe<T>(Option<T>);

impl<T> Maybe<T> {
    pub fn some(value: T) -> Self {
        Maybe(Some(value))
    }

    pub fn none() -> Self {
        Maybe(None)
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn unwrap(self) -> T {
        self.0.unwrap()
    }
}

impl<T> Monad<T> for Maybe<T> {
    type Output<U> = Maybe<U>;

    fn unit(value: T) -> Self::Output<T> {
        Maybe::some(value)
    }

    fn bind<U, F>(self, f: F) -> Self::Output<U>
    where
        F: FnOnce(T) -> Self::Output<U>,
    {
        match self.0 {
            Some(value) => f(value),
            None => Maybe::none(),
        }
    }
}

// Applicative patterns
pub struct Applicative;

impl Applicative {
    // Apply a function wrapped in a context to a value in a context
    pub fn apply<T, U, F>(func_maybe: Maybe<F>, value_maybe: Maybe<T>) -> Maybe<U>
    where
        F: FnOnce(T) -> U,
    {
        match (func_maybe.0, value_maybe.0) {
            (Some(f), Some(v)) => Maybe::some(f(v)),
            _ => Maybe::none(),
        }
    }

    // Lift a binary function to work with Maybe values
    pub fn lift2<T, U, V, F>(f: F) -> impl Fn(Maybe<T>, Maybe<U>) -> Maybe<V>
    where
        F: Fn(T, U) -> V,
    {
        move |maybe_t, maybe_u| {
            match (maybe_t.0, maybe_u.0) {
                (Some(t), Some(u)) => Maybe::some(f(t, u)),
                _ => Maybe::none(),
            }
        }
    }
}

// Robot behavior composition
pub struct RobotBehaviorComposer;

impl RobotBehaviorComposer {
    // Compose movement behaviors
    pub fn compose_movement() -> impl Fn(Robot, (f64, f64)) -> Robot {
        let calculate_energy_cost = |(x, y): (f64, f64)| {
            (x.powi(2) + y.powi(2)).sqrt() * 0.1
        };

        let move_robot = |mut robot: Robot, position: (f64, f64)| {
            robot.position = position;
            robot
        };

        let consume_energy = |mut robot: Robot, cost: f64| {
            robot.energy = (robot.energy - cost).max(0.0);
            robot
        };

        move |robot, target| {
            let cost = calculate_energy_cost(target);
            let robot_with_new_pos = move_robot(robot, target);
            consume_energy(robot_with_new_pos, cost)
        }
    }

    // Sensor data processing pipeline
    pub fn create_sensor_pipeline() -> impl Fn(Vec<SensorReading>) -> HashMap<String, f64> {
        let filter_recent = |readings: Vec<SensorReading>| {
            readings.into_iter()
                .filter(|r| r.timestamp > 1000)
                .collect::<Vec<_>>()
        };

        let group_by_type = |readings: Vec<SensorReading>| {
            let mut grouped: HashMap<String, Vec<f64>> = HashMap::new();
            for reading in readings {
                grouped.entry(reading.sensor_type)
                    .or_insert_with(Vec::new)
                    .push(reading.value);
            }
            grouped
        };

        let calculate_averages = |grouped: HashMap<String, Vec<f64>>| {
            grouped.into_iter()
                .map(|(sensor_type, values)| {
                    let avg = values.iter().sum::<f64>() / values.len() as f64;
                    (sensor_type, avg)
                })
                .collect()
        };

        FunctionComposition::compose3(calculate_averages, group_by_type, filter_recent)
    }

    // Decision-making pipeline
    pub fn create_decision_pipeline() -> impl Fn(Robot, Vec<SensorReading>) -> String {
        let analyze_sensors = Self::create_sensor_pipeline();

        let make_decision = |robot: Robot, sensor_data: HashMap<String, f64>| {
            if robot.energy < 20.0 {
                "return_to_base".to_string()
            } else if sensor_data.get("obstacle").unwrap_or(&0.0) > &0.5 {
                "avoid_obstacle".to_string()
            } else if sensor_data.get("target").unwrap_or(&0.0) > &0.8 {
                "approach_target".to_string()
            } else {
                "patrol".to_string()
            }
        };

        move |robot, readings| {
            let sensor_summary = analyze_sensors(readings);
            make_decision(robot, sensor_summary)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_composition() {
        let add_one = |x: i32| x + 1;
        let multiply_two = |x: i32| x * 2;

        let composed = FunctionComposition::compose(multiply_two, add_one);
        assert_eq!(composed(5), 12); // (5 + 1) * 2
    }

    #[test]
    fn test_three_function_composition() {
        let add_one = |x: i32| x + 1;
        let multiply_two = |x: i32| x * 2;
        let subtract_three = |x: i32| x - 3;

        let composed = FunctionComposition::compose3(subtract_three, multiply_two, add_one);
        assert_eq!(composed(5), 9); // ((5 + 1) * 2) - 3
    }

    #[test]
    fn test_robot_pipeline() {
        let pipeline = FunctionComposition::create_robot_pipeline();
        let robot = Robot::new(1, "TestBot".to_string());

        let result = pipeline(robot);
        assert!(result.contains("Energy: 120.0"));
        assert!(result.contains("Speed: 1.5"));
    }

    #[test]
    fn test_pipeline_pattern() {
        let result = Pipeline::new(5)
            .map(|x| x * 2)
            .map(|x| x + 3)
            .filter(|&x| x > 10)
            .unwrap()
            .execute();

        assert_eq!(result, 13);
    }

    #[test]
    fn test_result_composition() {
        let mut robot = Robot::new(1, "TestBot".to_string());
        robot.sensors.push("camera".to_string());

        let validation = ResultComposition::create_validation_pipeline();
        let result = validation(robot);

        assert!(result.is_ok());
    }

    #[test]
    fn test_result_composition_failure() {
        let robot = Robot::new(1, "TestBot".to_string());
        // No sensors added

        let validation = ResultComposition::create_validation_pipeline();
        let result = validation(robot);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("no sensors"));
    }

    #[test]
    fn test_maybe_monad() {
        let add_one = |x: i32| Maybe::some(x + 1);
        let multiply_two = |x: i32| Maybe::some(x * 2);

        let result = Maybe::some(5)
            .bind(add_one)
            .bind(multiply_two);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), 12);
    }

    #[test]
    fn test_maybe_monad_none() {
        let add_one = |x: i32| Maybe::some(x + 1);

        let result: Maybe<i32> = Maybe::none().bind(add_one);
        assert!(!result.is_some());
    }

    #[test]
    fn test_applicative_lift() {
        let add = |x: i32, y: i32| x + y;
        let lifted_add = Applicative::lift2(add);

        let result = lifted_add(Maybe::some(5), Maybe::some(3));
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 8);

        let result_none = lifted_add(Maybe::some(5), Maybe::none());
        assert!(!result_none.is_some());
    }

    #[test]
    fn test_movement_composition() {
        let movement = RobotBehaviorComposer::compose_movement();
        let robot = Robot::new(1, "TestBot".to_string());

        let moved_robot = movement(robot, (10.0, 0.0));
        assert_eq!(moved_robot.position, (10.0, 0.0));
        assert!(moved_robot.energy < 100.0);
    }

    #[test]
    fn test_sensor_pipeline() {
        let pipeline = RobotBehaviorComposer::create_sensor_pipeline();

        let readings = vec![
            SensorReading {
                sensor_type: "temperature".to_string(),
                value: 25.0,
                timestamp: 1500,
            },
            SensorReading {
                sensor_type: "temperature".to_string(),
                value: 27.0,
                timestamp: 1600,
            },
            SensorReading {
                sensor_type: "humidity".to_string(),
                value: 60.0,
                timestamp: 1700,
            },
        ];

        let result = pipeline(readings);
        assert!(result.contains_key("temperature"));
        assert!(result.contains_key("humidity"));
        assert_eq!(result["temperature"], 26.0);
        assert_eq!(result["humidity"], 60.0);
    }

    #[test]
    fn test_decision_pipeline() {
        let pipeline = RobotBehaviorComposer::create_decision_pipeline();
        let mut robot = Robot::new(1, "TestBot".to_string());
        robot.energy = 10.0; // Low energy

        let readings = vec![
            SensorReading {
                sensor_type: "obstacle".to_string(),
                value: 0.3,
                timestamp: 1500,
            },
        ];

        let decision = pipeline(robot, readings);
        assert_eq!(decision, "return_to_base");
    }
}

// Student exercises
pub mod exercises {
    use super::*;

    // Exercise 1: Implement function currying
    pub struct Currying;

    impl Currying {
        pub fn curry2<A, B, C, F>(f: F) -> Box<dyn Fn(A) -> Box<dyn Fn(B) -> C>>
        where
            F: Fn(A, B) -> C + 'static,
            A: 'static,
            B: 'static,
            C: 'static,
        {
            // TODO: Implement currying for 2-parameter functions
            unimplemented!("Implement curry2")
        }

        pub fn curry3<A, B, C, D, F>(f: F) -> Box<dyn Fn(A) -> Box<dyn Fn(B) -> Box<dyn Fn(C) -> D>>>
        where
            F: Fn(A, B, C) -> D + 'static,
            A: 'static,
            B: 'static,
            C: 'static,
            D: 'static,
        {
            // TODO: Implement currying for 3-parameter functions
            unimplemented!("Implement curry3")
        }
    }

    // Exercise 2: Implement partial application
    pub struct PartialApplication;

    impl PartialApplication {
        pub fn partial1<A, B, C, F>(f: F, a: A) -> impl Fn(B) -> C
        where
            F: Fn(A, B) -> C,
            A: Clone,
        {
            move |b| f(a.clone(), b)
        }

        pub fn partial2<A, B, C, F>(f: F, b: B) -> impl Fn(A) -> C
        where
            F: Fn(A, B) -> C,
            B: Clone,
        {
            move |a| f(a, b.clone())
        }
    }

    // Exercise 3: Implement function memoization
    pub struct Memoization<T> {
        cache: std::collections::HashMap<String, T>,
    }

    impl<T> Memoization<T> {
        pub fn new() -> Self {
            Memoization {
                cache: std::collections::HashMap::new(),
            }
        }

        pub fn memoize<'a, F>(&'a mut self, f: F) -> impl FnMut(&str) -> T + 'a
        where
            F: Fn(&str) -> T + 'a,
            T: Clone,
        {
            move |input: &str| {
                if let Some(result) = self.cache.get(input) {
                    result.clone()
                } else {
                    let result = f(input);
                    self.cache.insert(input.to_string(), result.clone());
                    result
                }
            }
        }
    }

    // Exercise 4: Implement compose_all for multiple functions
    pub struct ComposeAll;

    impl ComposeAll {
        pub fn compose_all<T>(functions: Vec<Box<dyn Fn(T) -> T>>) -> impl Fn(T) -> T
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

        pub fn compose_all_results<T, E>(
            functions: Vec<Box<dyn Fn(T) -> Result<T, E>>>,
        ) -> impl Fn(T) -> Result<T, E>
        where
            T: Clone,
        {
            move |mut input| {
                for func in &functions {
                    match func(input) {
                        Ok(result) => input = result,
                        Err(e) => return Err(e),
                    }
                }
                Ok(input)
            }
        }
    }

    // Exercise 5: Implement lens pattern for functional updates
    pub struct Lens<S, A> {
        // TODO: Store getter and setter functions
        _phantom: std::marker::PhantomData<(S, A)>,
    }

    impl<S, A> Lens<S, A> {
        pub fn new<G, St>(getter: G, setter: St) -> Self
        where
            G: Fn(&S) -> A,
            St: Fn(S, A) -> S,
        {
            // TODO: Create lens with getter and setter
            unimplemented!("Create lens")
        }

        pub fn view(&self, source: &S) -> A
        where
            A: Clone,
        {
            // TODO: Get value using lens
            unimplemented!("View through lens")
        }

        pub fn set(&self, source: S, value: A) -> S {
            // TODO: Set value using lens
            unimplemented!("Set through lens")
        }

        pub fn over<F>(&self, source: S, f: F) -> S
        where
            F: FnOnce(A) -> A,
        {
            // TODO: Modify value using lens and function
            unimplemented!("Modify through lens")
        }
    }
}