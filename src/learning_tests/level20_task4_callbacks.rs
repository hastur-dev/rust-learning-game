// Learning Tests for Level 20, Task 4: Callback Systems and Event Handlers
// Building event-driven systems using callbacks and function traits

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

// Robot event system data structures
#[derive(Debug, Clone, PartialEq)]
pub struct Robot {
    pub id: u32,
    pub name: String,
    pub position: (f64, f64),
    pub energy: f64,
    pub status: RobotStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RobotStatus {
    Idle,
    Moving,
    Working,
    Charging,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct RobotEvent {
    pub robot_id: u32,
    pub event_type: EventType,
    pub timestamp: u64,
    pub data: EventData,
}

#[derive(Debug, Clone)]
pub enum EventType {
    Movement,
    EnergyChange,
    StatusChange,
    SensorUpdate,
    Error,
}

#[derive(Debug, Clone)]
pub enum EventData {
    Position(f64, f64),
    Energy(f64),
    Status(RobotStatus),
    Sensor(String, f64),
    ErrorMessage(String),
}

impl Robot {
    pub fn new(id: u32, name: String) -> Self {
        Robot {
            id,
            name,
            position: (0.0, 0.0),
            energy: 100.0,
            status: RobotStatus::Idle,
        }
    }
}

// Basic callback system
pub struct CallbackRegistry<T> {
    callbacks: Vec<Box<dyn Fn(&T)>>,
}

impl<T> CallbackRegistry<T> {
    pub fn new() -> Self {
        CallbackRegistry {
            callbacks: Vec::new(),
        }
    }

    pub fn register<F>(&mut self, callback: F)
    where
        F: Fn(&T) + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }

    pub fn trigger(&self, event: &T) {
        for callback in &self.callbacks {
            callback(event);
        }
    }

    pub fn callback_count(&self) -> usize {
        self.callbacks.len()
    }
}

// Typed event system
pub struct EventSystem {
    movement_handlers: CallbackRegistry<RobotEvent>,
    energy_handlers: CallbackRegistry<RobotEvent>,
    status_handlers: CallbackRegistry<RobotEvent>,
    sensor_handlers: CallbackRegistry<RobotEvent>,
    error_handlers: CallbackRegistry<RobotEvent>,
}

impl EventSystem {
    pub fn new() -> Self {
        EventSystem {
            movement_handlers: CallbackRegistry::new(),
            energy_handlers: CallbackRegistry::new(),
            status_handlers: CallbackRegistry::new(),
            sensor_handlers: CallbackRegistry::new(),
            error_handlers: CallbackRegistry::new(),
        }
    }

    pub fn on_movement<F>(&mut self, handler: F)
    where
        F: Fn(&RobotEvent) + 'static,
    {
        self.movement_handlers.register(handler);
    }

    pub fn on_energy_change<F>(&mut self, handler: F)
    where
        F: Fn(&RobotEvent) + 'static,
    {
        self.energy_handlers.register(handler);
    }

    pub fn on_status_change<F>(&mut self, handler: F)
    where
        F: Fn(&RobotEvent) + 'static,
    {
        self.status_handlers.register(handler);
    }

    pub fn on_sensor_update<F>(&mut self, handler: F)
    where
        F: Fn(&RobotEvent) + 'static,
    {
        self.sensor_handlers.register(handler);
    }

    pub fn on_error<F>(&mut self, handler: F)
    where
        F: Fn(&RobotEvent) + 'static,
    {
        self.error_handlers.register(handler);
    }

    pub fn emit(&self, event: RobotEvent) {
        match event.event_type {
            EventType::Movement => self.movement_handlers.trigger(&event),
            EventType::EnergyChange => self.energy_handlers.trigger(&event),
            EventType::StatusChange => self.status_handlers.trigger(&event),
            EventType::SensorUpdate => self.sensor_handlers.trigger(&event),
            EventType::Error => self.error_handlers.trigger(&event),
        }
    }

    pub fn handler_counts(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.movement_handlers.callback_count(),
            self.energy_handlers.callback_count(),
            self.status_handlers.callback_count(),
            self.sensor_handlers.callback_count(),
            self.error_handlers.callback_count(),
        )
    }
}

// Stateful callback handlers
pub struct StatefulHandlers {
    log: Rc<RefCell<Vec<String>>>,
    robot_positions: Rc<RefCell<HashMap<u32, (f64, f64)>>>,
    energy_alerts: Rc<RefCell<Vec<u32>>>,
}

impl StatefulHandlers {
    pub fn new() -> Self {
        StatefulHandlers {
            log: Rc::new(RefCell::new(Vec::new())),
            robot_positions: Rc::new(RefCell::new(HashMap::new())),
            energy_alerts: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn create_logger(&self) -> impl Fn(&RobotEvent) {
        let log = self.log.clone();
        move |event| {
            let message = format!("Robot {} - {:?} at {}",
                                event.robot_id, event.event_type, event.timestamp);
            log.borrow_mut().push(message);
        }
    }

    pub fn create_position_tracker(&self) -> impl Fn(&RobotEvent) {
        let positions = self.robot_positions.clone();
        move |event| {
            if let EventData::Position(x, y) = event.data {
                positions.borrow_mut().insert(event.robot_id, (x, y));
            }
        }
    }

    pub fn create_energy_monitor(&self, threshold: f64) -> impl Fn(&RobotEvent) {
        let alerts = self.energy_alerts.clone();
        move |event| {
            if let EventData::Energy(energy) = event.data {
                if energy < threshold {
                    let mut alerts_mut = alerts.borrow_mut();
                    if !alerts_mut.contains(&event.robot_id) {
                        alerts_mut.push(event.robot_id);
                    }
                }
            }
        }
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.log.borrow().clone()
    }

    pub fn get_robot_position(&self, robot_id: u32) -> Option<(f64, f64)> {
        self.robot_positions.borrow().get(&robot_id).copied()
    }

    pub fn get_low_energy_robots(&self) -> Vec<u32> {
        self.energy_alerts.borrow().clone()
    }
}

// Conditional callbacks
pub struct ConditionalCallbacks;

impl ConditionalCallbacks {
    // Create callback that only triggers under certain conditions
    pub fn when<F, P>(predicate: P, callback: F) -> impl Fn(&RobotEvent)
    where
        F: Fn(&RobotEvent),
        P: Fn(&RobotEvent) -> bool,
    {
        move |event| {
            if predicate(event) {
                callback(event);
            }
        }
    }

    // Create callback that triggers only once
    pub fn once<F>(callback: F) -> impl FnMut(&RobotEvent)
    where
        F: FnOnce(&RobotEvent),
    {
        let mut called = false;
        let mut callback = Some(callback);

        move |event| {
            if !called {
                called = true;
                if let Some(cb) = callback.take() {
                    cb(event);
                }
            }
        }
    }

    // Create callback with timeout
    pub fn with_timeout<F>(callback: F, max_time: u64) -> impl Fn(&RobotEvent)
    where
        F: Fn(&RobotEvent),
    {
        move |event| {
            if event.timestamp <= max_time {
                callback(event);
            }
        }
    }

    // Create throttled callback (limits frequency)
    pub fn throttled<F>(callback: F, min_interval: u64) -> impl FnMut(&RobotEvent)
    where
        F: Fn(&RobotEvent),
    {
        let mut last_call = 0u64;
        move |event| {
            if event.timestamp >= last_call + min_interval {
                last_call = event.timestamp;
                callback(event);
            }
        }
    }

    // Create debounced callback (waits for quiet period)
    pub fn debounced<F>(callback: F, delay: u64) -> impl FnMut(&RobotEvent)
    where
        F: Fn(&RobotEvent),
    {
        let mut last_event_time = 0u64;
        move |event| {
            last_event_time = event.timestamp;
            // In real implementation, would use a timer
            // For demo, we just check if enough time has passed
            if event.timestamp >= last_event_time + delay {
                callback(event);
            }
        }
    }
}

// Callback chains and composition
pub struct CallbackChain<T> {
    callbacks: Vec<Box<dyn Fn(&T) -> bool>>, // Return false to stop chain
}

impl<T> CallbackChain<T> {
    pub fn new() -> Self {
        CallbackChain {
            callbacks: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, callback: F)
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }

    pub fn execute(&self, event: &T) -> bool {
        for callback in &self.callbacks {
            if !callback(event) {
                return false; // Chain stopped
            }
        }
        true // All callbacks executed
    }
}

// Observer pattern with callbacks
pub struct Subject<T> {
    observers: Vec<Box<dyn Fn(&T)>>,
}

impl<T> Subject<T> {
    pub fn new() -> Self {
        Subject {
            observers: Vec::new(),
        }
    }

    pub fn attach<F>(&mut self, observer: F)
    where
        F: Fn(&T) + 'static,
    {
        self.observers.push(Box::new(observer));
    }

    pub fn notify(&self, data: &T) {
        for observer in &self.observers {
            observer(data);
        }
    }

    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }
}

// Command pattern with callbacks
pub struct Command {
    execute: Box<dyn Fn()>,
    undo: Option<Box<dyn Fn()>>,
}

impl Command {
    pub fn new<E, U>(execute: E, undo: Option<U>) -> Self
    where
        E: Fn() + 'static,
        U: Fn() + 'static,
    {
        Command {
            execute: Box::new(execute),
            undo: undo.map(|u| Box::new(u) as Box<dyn Fn()>),
        }
    }

    pub fn execute(&self) {
        (self.execute)();
    }

    pub fn undo(&self) {
        if let Some(undo_fn) = &self.undo {
            undo_fn();
        }
    }

    pub fn has_undo(&self) -> bool {
        self.undo.is_some()
    }
}

pub struct CommandInvoker {
    history: Vec<Command>,
    current: usize,
}

impl CommandInvoker {
    pub fn new() -> Self {
        CommandInvoker {
            history: Vec::new(),
            current: 0,
        }
    }

    pub fn execute_command(&mut self, command: Command) {
        command.execute();

        // Remove any commands after current position
        self.history.truncate(self.current);
        self.history.push(command);
        self.current += 1;
    }

    pub fn undo(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
            if let Some(command) = self.history.get(self.current) {
                command.undo();
                return true;
            }
        }
        false
    }

    pub fn redo(&mut self) -> bool {
        if self.current < self.history.len() {
            if let Some(command) = self.history.get(self.current) {
                command.execute();
                self.current += 1;
                return true;
            }
        }
        false
    }
}

// Async-style callbacks (simplified)
pub struct AsyncCallbacks;

impl AsyncCallbacks {
    // Simulate async operation with callback
    pub fn fetch_robot_data<F, E>(robot_id: u32, on_success: F, on_error: E)
    where
        F: FnOnce(Robot),
        E: FnOnce(String),
    {
        // Simulate network delay and possible failure
        if robot_id == 0 {
            on_error("Invalid robot ID".to_string());
        } else {
            let robot = Robot::new(robot_id, format!("Robot_{}", robot_id));
            on_success(robot);
        }
    }

    // Chain async operations
    pub fn chain_operations<F1, F2, F3>(
        operation1: F1,
        operation2: F2,
        final_callback: F3,
    ) where
        F1: FnOnce(Box<dyn FnOnce(i32)>),
        F2: FnOnce(i32, Box<dyn FnOnce(String)>) + 'static,
        F3: FnOnce(String) + 'static,
    {
        operation1(Box::new(move |result1| {
            operation2(result1, Box::new(final_callback));
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_callback_registry() {
        let mut registry = CallbackRegistry::new();
        let mut called = false;

        registry.register(move |_event: &RobotEvent| {
            // This won't work due to move, but demonstrates the pattern
        });

        // Use Rc<RefCell<bool>> for shared state in real tests
        let call_count = Rc::new(RefCell::new(0));
        let count_clone = call_count.clone();

        registry.register(move |_event| {
            *count_clone.borrow_mut() += 1;
        });

        let event = RobotEvent {
            robot_id: 1,
            event_type: EventType::Movement,
            timestamp: 1000,
            data: EventData::Position(10.0, 20.0),
        };

        registry.trigger(&event);
        assert_eq!(*call_count.borrow(), 1);
    }

    #[test]
    fn test_event_system() {
        let mut system = EventSystem::new();
        let call_count = Rc::new(RefCell::new(0));
        let count_clone = call_count.clone();

        system.on_movement(move |_event| {
            *count_clone.borrow_mut() += 1;
        });

        let event = RobotEvent {
            robot_id: 1,
            event_type: EventType::Movement,
            timestamp: 1000,
            data: EventData::Position(10.0, 20.0),
        };

        system.emit(event);
        assert_eq!(*call_count.borrow(), 1);

        let counts = system.handler_counts();
        assert_eq!(counts.0, 1); // movement handlers
    }

    #[test]
    fn test_stateful_handlers() {
        let handlers = StatefulHandlers::new();
        let logger = handlers.create_logger();
        let position_tracker = handlers.create_position_tracker();

        let movement_event = RobotEvent {
            robot_id: 1,
            event_type: EventType::Movement,
            timestamp: 1000,
            data: EventData::Position(10.0, 20.0),
        };

        logger(&movement_event);
        position_tracker(&movement_event);

        let logs = handlers.get_logs();
        assert_eq!(logs.len(), 1);
        assert!(logs[0].contains("Robot 1"));

        let position = handlers.get_robot_position(1);
        assert_eq!(position, Some((10.0, 20.0)));
    }

    #[test]
    fn test_conditional_callbacks() {
        let call_count = Rc::new(RefCell::new(0));
        let count_clone = call_count.clone();

        let conditional = ConditionalCallbacks::when(
            |event| event.robot_id == 1,
            move |_event| {
                *count_clone.borrow_mut() += 1;
            }
        );

        let event1 = RobotEvent {
            robot_id: 1,
            event_type: EventType::Movement,
            timestamp: 1000,
            data: EventData::Position(10.0, 20.0),
        };

        let event2 = RobotEvent {
            robot_id: 2,
            event_type: EventType::Movement,
            timestamp: 1001,
            data: EventData::Position(15.0, 25.0),
        };

        conditional(&event1); // Should trigger
        conditional(&event2); // Should not trigger

        assert_eq!(*call_count.borrow(), 1);
    }

    #[test]
    fn test_callback_chain() {
        let mut chain = CallbackChain::new();

        chain.add(|event: &RobotEvent| {
            event.robot_id == 1 // Continue only for robot 1
        });

        chain.add(|event: &RobotEvent| {
            event.timestamp > 500 // Continue only for recent events
        });

        let good_event = RobotEvent {
            robot_id: 1,
            event_type: EventType::Movement,
            timestamp: 1000,
            data: EventData::Position(10.0, 20.0),
        };

        let bad_event = RobotEvent {
            robot_id: 2,
            event_type: EventType::Movement,
            timestamp: 1000,
            data: EventData::Position(10.0, 20.0),
        };

        assert!(chain.execute(&good_event));
        assert!(!chain.execute(&bad_event));
    }

    #[test]
    fn test_observer_pattern() {
        let mut subject = Subject::new();
        let call_count = Rc::new(RefCell::new(0));
        let count_clone = call_count.clone();

        subject.attach(move |_data: &String| {
            *count_clone.borrow_mut() += 1;
        });

        subject.notify(&"test data".to_string());
        assert_eq!(*call_count.borrow(), 1);
        assert_eq!(subject.observer_count(), 1);
    }

    #[test]
    fn test_command_pattern() {
        let state = Rc::new(RefCell::new(0));
        let state_clone1 = state.clone();
        let state_clone2 = state.clone();

        let command = Command::new(
            move || { *state_clone1.borrow_mut() += 1; },
            Some(move || { *state_clone2.borrow_mut() -= 1; })
        );

        assert_eq!(*state.borrow(), 0);

        command.execute();
        assert_eq!(*state.borrow(), 1);

        command.undo();
        assert_eq!(*state.borrow(), 0);

        assert!(command.has_undo());
    }

    #[test]
    fn test_command_invoker() {
        let mut invoker = CommandInvoker::new();
        let state = Rc::new(RefCell::new(0));

        let state1 = state.clone();
        let state2 = state.clone();
        let command1 = Command::new(
            move || { *state1.borrow_mut() += 5; },
            Some(move || { *state2.borrow_mut() -= 5; })
        );

        let state3 = state.clone();
        let state4 = state.clone();
        let command2 = Command::new(
            move || { *state3.borrow_mut() *= 2; },
            Some(move || { *state4.borrow_mut() /= 2; })
        );

        invoker.execute_command(command1);
        assert_eq!(*state.borrow(), 5);

        invoker.execute_command(command2);
        assert_eq!(*state.borrow(), 10);

        assert!(invoker.undo());
        assert_eq!(*state.borrow(), 5);

        assert!(invoker.undo());
        assert_eq!(*state.borrow(), 0);

        assert!(invoker.redo());
        assert_eq!(*state.borrow(), 5);
    }

    #[test]
    fn test_async_callbacks() {
        let success_result = Rc::new(RefCell::new(None));
        let error_result = Rc::new(RefCell::new(None));

        let success_clone = success_result.clone();
        let error_clone = error_result.clone();

        // Test successful callback
        AsyncCallbacks::fetch_robot_data(
            1,
            move |robot| { *success_clone.borrow_mut() = Some(robot); },
            move |error| { *error_clone.borrow_mut() = Some(error); }
        );

        assert!(success_result.borrow().is_some());
        assert!(error_result.borrow().is_none());

        // Test error callback
        let error_result2 = Rc::new(RefCell::new(None));
        let error_clone2 = error_result2.clone();

        AsyncCallbacks::fetch_robot_data(
            0,
            |_robot| {},
            move |error| { *error_clone2.borrow_mut() = Some(error); }
        );

        assert!(error_result2.borrow().is_some());
    }
}

// Student exercises
pub mod exercises {
    use super::*;

    // Exercise 1: Implement priority-based event system
    pub struct PriorityEventSystem {
        // TODO: Store handlers with priorities
    }

    impl PriorityEventSystem {
        pub fn new() -> Self {
            // TODO: Initialize priority event system
            unimplemented!("Initialize priority event system")
        }

        pub fn register_handler<F>(&mut self, priority: u32, handler: F)
        where
            F: Fn(&RobotEvent) + 'static,
        {
            // TODO: Register handler with priority (higher number = higher priority)
            unimplemented!("Register priority handler")
        }

        pub fn emit(&self, event: RobotEvent) {
            // TODO: Execute handlers in priority order
            unimplemented!("Emit with priority")
        }
    }

    // Exercise 2: Implement middleware system with callbacks
    pub struct MiddlewareSystem {
        // TODO: Store middleware functions
    }

    impl MiddlewareSystem {
        pub fn new() -> Self {
            // TODO: Initialize middleware system
            unimplemented!("Initialize middleware")
        }

        pub fn use_middleware<F>(&mut self, middleware: F)
        where
            F: Fn(&mut RobotEvent, Box<dyn Fn(&mut RobotEvent)>) + 'static,
        {
            // TODO: Add middleware that can modify events and call next
            unimplemented!("Add middleware")
        }

        pub fn process(&self, mut event: RobotEvent) -> RobotEvent {
            // TODO: Process event through middleware chain
            unimplemented!("Process through middleware")
        }
    }

    // Exercise 3: Implement subscription management
    pub struct SubscriptionManager {
        // TODO: Store subscriptions with IDs for removal
    }

    pub struct SubscriptionId(usize);

    impl SubscriptionManager {
        pub fn new() -> Self {
            // TODO: Initialize subscription manager
            unimplemented!("Initialize subscription manager")
        }

        pub fn subscribe<F>(&mut self, handler: F) -> SubscriptionId
        where
            F: Fn(&RobotEvent) + 'static,
        {
            // TODO: Subscribe and return ID for later unsubscription
            unimplemented!("Subscribe with ID")
        }

        pub fn unsubscribe(&mut self, id: SubscriptionId) -> bool {
            // TODO: Remove subscription by ID
            unimplemented!("Unsubscribe by ID")
        }

        pub fn emit(&self, event: &RobotEvent) {
            // TODO: Emit to all active subscriptions
            unimplemented!("Emit to subscriptions")
        }
    }

    // Exercise 4: Implement callback aggregator
    pub struct CallbackAggregator<T> {
        // TODO: Collect results from multiple callbacks
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> CallbackAggregator<T> {
        pub fn new() -> Self {
            // TODO: Initialize aggregator
            unimplemented!("Initialize aggregator")
        }

        pub fn add_callback<F>(&mut self, callback: F)
        where
            F: Fn(&T) -> String + 'static,
        {
            // TODO: Add callback that returns a result
            unimplemented!("Add result callback")
        }

        pub fn execute(&self, input: &T) -> Vec<String> {
            // TODO: Execute all callbacks and collect results
            unimplemented!("Execute and collect")
        }
    }

    // Exercise 5: Implement event replay system
    pub struct EventReplay {
        // TODO: Store events for replay
    }

    impl EventReplay {
        pub fn new() -> Self {
            // TODO: Initialize replay system
            unimplemented!("Initialize replay")
        }

        pub fn record_event(&mut self, event: RobotEvent) {
            // TODO: Record event for later replay
            unimplemented!("Record event")
        }

        pub fn replay_events<F>(&self, handler: F)
        where
            F: Fn(&RobotEvent),
        {
            // TODO: Replay all recorded events
            unimplemented!("Replay events")
        }

        pub fn replay_filtered<F, P>(&self, predicate: P, handler: F)
        where
            P: Fn(&RobotEvent) -> bool,
            F: Fn(&RobotEvent),
        {
            // TODO: Replay events matching predicate
            unimplemented!("Replay filtered")
        }
    }
}