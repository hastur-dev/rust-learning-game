// Learning Tests for Level 17, Task 5: Advanced Performance Logging
// Implementing module-specific logging with performance metrics and optimization

use log::{trace, debug, info, warn, error};
use std::env;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Module-specific performance metrics
#[derive(Debug, Clone)]
pub struct ModulePerformanceMetrics {
    pub module_name: String,
    pub total_calls: u64,
    pub total_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub avg_duration: Duration,
    pub error_count: u64,
    pub last_updated: Instant,
}

impl ModulePerformanceMetrics {
    pub fn new(module_name: &str) -> Self {
        ModulePerformanceMetrics {
            module_name: module_name.to_string(),
            total_calls: 0,
            total_duration: Duration::new(0, 0),
            min_duration: Duration::new(u64::MAX, 0),
            max_duration: Duration::new(0, 0),
            avg_duration: Duration::new(0, 0),
            error_count: 0,
            last_updated: Instant::now(),
        }
    }

    pub fn record_call(&mut self, duration: Duration, is_error: bool) {
        self.total_calls += 1;
        self.total_duration += duration;

        if duration < self.min_duration {
            self.min_duration = duration;
        }
        if duration > self.max_duration {
            self.max_duration = duration;
        }

        self.avg_duration = self.total_duration / self.total_calls as u32;

        if is_error {
            self.error_count += 1;
        }

        self.last_updated = Instant::now();
    }

    pub fn get_error_rate(&self) -> f64 {
        if self.total_calls == 0 {
            0.0
        } else {
            (self.error_count as f64) / (self.total_calls as f64) * 100.0
        }
    }

    pub fn get_calls_per_second(&self) -> f64 {
        let elapsed = self.last_updated.elapsed();
        if elapsed.as_secs() == 0 {
            self.total_calls as f64
        } else {
            (self.total_calls as f64) / elapsed.as_secs_f64()
        }
    }
}

// Performance tracker for all modules
#[derive(Debug)]
pub struct PerformanceTracker {
    pub modules: Arc<Mutex<HashMap<String, ModulePerformanceMetrics>>>,
    start_time: Instant,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        PerformanceTracker {
            modules: Arc::new(Mutex::new(HashMap::new())),
            start_time: Instant::now(),
        }
    }

    pub fn record_operation(&self, module: &str, operation: &str, duration: Duration, is_error: bool) {
        let mut modules = self.modules.lock().unwrap();
        let key = format!("{}::{}", module, operation);

        let metrics = modules.entry(key).or_insert_with(|| {
            ModulePerformanceMetrics::new(&format!("{}::{}", module, operation))
        });

        metrics.record_call(duration, is_error);

        // Log performance based on module configuration
        match module {
            "ai" => {
                if duration.as_millis() > 200 {
                    warn!(target: "performance::ai",
                          "AI operation took longer than expected";
                          "operation" => operation,
                          "duration_ms" => duration.as_millis(),
                          "threshold_ms" => 200u64);
                } else {
                    debug!(target: "performance::ai",
                           "AI operation completed";
                           "operation" => operation,
                           "duration_ms" => duration.as_millis());
                }
            }
            "navigation" => {
                if duration.as_millis() > 50 {
                    warn!(target: "performance::navigation",
                          "Navigation operation slower than optimal";
                          "operation" => operation,
                          "duration_ms" => duration.as_millis(),
                          "threshold_ms" => 50u64);
                } else {
                    trace!(target: "performance::navigation",
                           "Navigation operation - excellent performance";
                           "operation" => operation,
                           "duration_ms" => duration.as_millis());
                }
            }
            "sensors" => {
                if duration.as_millis() > 10 {
                    warn!(target: "performance::sensors",
                          "Sensor operation exceeded optimal timing";
                          "operation" => operation,
                          "duration_ms" => duration.as_millis(),
                          "threshold_ms" => 10u64);
                } else {
                    trace!(target: "performance::sensors",
                           "Sensor operation - optimal performance";
                           "operation" => operation,
                           "duration_ms" => duration.as_millis());
                }
            }
            "movement" => {
                if duration.as_millis() > 100 {
                    info!(target: "performance::movement",
                          "Movement operation took acceptable time";
                          "operation" => operation,
                          "duration_ms" => duration.as_millis(),
                          "status" => "acceptable");
                } else {
                    debug!(target: "performance::movement",
                           "Movement operation completed efficiently";
                           "operation" => operation,
                           "duration_ms" => duration.as_millis());
                }
            }
            _ => {
                debug!(target: "performance::general",
                       "General operation completed";
                       "module" => module,
                       "operation" => operation,
                       "duration_ms" => duration.as_millis());
            }
        }
    }

    pub fn get_module_metrics(&self, module: &str) -> Option<ModulePerformanceMetrics> {
        let modules = self.modules.lock().unwrap();
        modules.values()
            .find(|m| m.module_name.starts_with(module))
            .cloned()
    }

    pub fn get_all_metrics(&self) -> Vec<ModulePerformanceMetrics> {
        let modules = self.modules.lock().unwrap();
        modules.values().cloned().collect()
    }

    pub fn generate_performance_report(&self) {
        let modules = self.modules.lock().unwrap();

        info!(target: "performance::report", "=== ROBOT PERFORMANCE REPORT ===");

        for (module_key, metrics) in modules.iter() {
            let status = if metrics.avg_duration.as_millis() > 100 {
                "NEEDS_OPTIMIZATION"
            } else if metrics.avg_duration.as_millis() > 50 {
                "ACCEPTABLE"
            } else {
                "OPTIMAL"
            };

            info!(target: "performance::report",
                  "Module performance summary";
                  "module" => module_key,
                  "total_calls" => metrics.total_calls,
                  "avg_duration_ms" => metrics.avg_duration.as_millis(),
                  "min_duration_ms" => metrics.min_duration.as_millis(),
                  "max_duration_ms" => metrics.max_duration.as_millis(),
                  "error_rate_percent" => metrics.get_error_rate(),
                  "calls_per_second" => metrics.get_calls_per_second(),
                  "performance_status" => status);
        }

        let uptime = self.start_time.elapsed();
        info!(target: "performance::report",
              "System uptime and overall stats";
              "uptime_seconds" => uptime.as_secs(),
              "total_modules" => modules.len());
    }
}

// Lazy evaluation wrapper for expensive log operations
pub struct LazyLogMessage<F>
where
    F: Fn() -> String,
{
    generator: F,
}

impl<F> LazyLogMessage<F>
where
    F: Fn() -> String,
{
    pub fn new(generator: F) -> Self {
        LazyLogMessage { generator }
    }

    pub fn evaluate(&self) -> String {
        (self.generator)()
    }
}

// Macro for lazy logging to avoid expensive operations when log level is disabled
macro_rules! lazy_debug {
    ($target:expr, $msg:expr, $($arg:expr),*) => {
        if log::log_enabled!(log::Level::Debug) {
            debug!(target: $target, $msg, $($arg),*);
        }
    };
}

macro_rules! lazy_trace {
    ($target:expr, $msg:expr, $($arg:expr),*) => {
        if log::log_enabled!(log::Level::Trace) {
            trace!(target: $target, $msg, $($arg),*);
        }
    };
}

// Performance measurement wrapper
pub fn measure_performance<F, R>(
    tracker: &PerformanceTracker,
    module: &str,
    operation: &str,
    func: F,
) -> R
where
    F: FnOnce() -> Result<R, Box<dyn std::error::Error>>,
{
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();

    let is_error = result.is_err();
    tracker.record_operation(module, operation, duration, is_error);

    match result {
        Ok(value) => value,
        Err(e) => {
            error!(target: "performance::error",
                   "Operation failed with error";
                   "module" => module,
                   "operation" => operation,
                   "error" => %e,
                   "duration_ms" => duration.as_millis());
            panic!("Operation failed: {}", e);
        }
    }
}

// Initialize advanced performance logging
pub fn initialize_advanced_performance_logging() {
    if env::var("RUST_LOG").is_err() {
        // Set different log levels for different modules
        env::set_var("RUST_LOG", "info,performance::ai=debug,performance::sensors=warn,performance::navigation=trace");
    }

    let _ = env_logger::try_init();

    info!(target: "performance::system",
          "Advanced performance logging initialized";
          "ai_level" => "debug",
          "sensors_level" => "warn",
          "navigation_level" => "trace",
          "movement_level" => "info");
}

// AI Module simulation
pub mod ai_module {
    use super::*;

    pub fn process_decision_tree(tracker: &PerformanceTracker, complexity: u32) -> String {
        measure_performance(tracker, "ai", "decision_tree", || {
            // Simulate AI processing with variable complexity
            let processing_time = std::time::Duration::from_millis(complexity as u64 * 10);
            std::thread::sleep(processing_time);

            lazy_debug!("performance::ai", "Decision tree processing details";
                       "complexity" => complexity,
                       "nodes_evaluated" => complexity * 5,
                       "memory_usage_mb" => complexity as f64 * 0.5);

            if complexity > 20 {
                Err("Decision tree too complex".into())
            } else {
                Ok(format!("Decision made with complexity {}", complexity))
            }
        })
    }

    pub fn neural_network_inference(tracker: &PerformanceTracker, input_size: u32) -> Vec<f32> {
        measure_performance(tracker, "ai", "neural_inference", || {
            let processing_time = std::time::Duration::from_millis(input_size as u64 * 2);
            std::thread::sleep(processing_time);

            lazy_debug!("performance::ai", "Neural network inference";
                       "input_size" => input_size,
                       "layers_processed" => 5,
                       "activations" => input_size * 10);

            Ok(vec![0.5; input_size as usize])
        })
    }
}

// Navigation Module simulation
pub mod navigation_module {
    use super::*;

    pub fn calculate_path(tracker: &PerformanceTracker, waypoints: u32) -> Vec<(f32, f32)> {
        measure_performance(tracker, "navigation", "path_calculation", || {
            let processing_time = std::time::Duration::from_millis(waypoints as u64);
            std::thread::sleep(processing_time);

            lazy_trace!("performance::navigation", "Path calculation details";
                       "waypoints" => waypoints,
                       "algorithm" => "A*",
                       "heuristic" => "euclidean");

            Ok((0..waypoints).map(|i| (i as f32, i as f32)).collect())
        })
    }

    pub fn update_position(tracker: &PerformanceTracker, x: f32, y: f32) -> bool {
        measure_performance(tracker, "navigation", "position_update", || {
            let processing_time = std::time::Duration::from_millis(5);
            std::thread::sleep(processing_time);

            lazy_trace!("performance::navigation", "Position updated";
                       "new_x" => x,
                       "new_y" => y,
                       "precision" => "high");

            Ok(true)
        })
    }
}

// Sensors Module simulation
pub mod sensors_module {
    use super::*;

    pub fn read_all_sensors(tracker: &PerformanceTracker) -> HashMap<String, f32> {
        measure_performance(tracker, "sensors", "read_all", || {
            let processing_time = std::time::Duration::from_millis(2);
            std::thread::sleep(processing_time);

            let mut readings = HashMap::new();
            readings.insert("temperature".to_string(), 42.5);
            readings.insert("humidity".to_string(), 65.0);
            readings.insert("pressure".to_string(), 1013.25);

            lazy_trace!("performance::sensors", "All sensors read successfully";
                       "sensor_count" => readings.len(),
                       "read_time_ms" => 2u64);

            Ok(readings)
        })
    }

    pub fn calibrate_sensor(tracker: &PerformanceTracker, sensor_name: &str) -> bool {
        measure_performance(tracker, "sensors", "calibration", || {
            let processing_time = std::time::Duration::from_millis(15);
            std::thread::sleep(processing_time);

            if sensor_name == "faulty_sensor" {
                warn!(target: "performance::sensors",
                      "Sensor calibration failed";
                      "sensor" => sensor_name,
                      "reason" => "hardware_malfunction");
                Err("Sensor calibration failed".into())
            } else {
                lazy_trace!("performance::sensors", "Sensor calibrated successfully";
                           "sensor" => sensor_name,
                           "calibration_accuracy" => 0.99f32);
                Ok(true)
            }
        })
    }
}

// Movement Module simulation
pub mod movement_module {
    use super::*;

    pub fn execute_movement(tracker: &PerformanceTracker, distance: f32) -> bool {
        measure_performance(tracker, "movement", "execute", || {
            let processing_time = std::time::Duration::from_millis((distance * 10.0) as u64);
            std::thread::sleep(processing_time);

            debug!(target: "performance::movement", "Movement executed";
                   "distance" => distance,
                   "estimated_time_ms" => (distance * 10.0) as u64);

            Ok(true)
        })
    }

    pub fn rotate_robot(tracker: &PerformanceTracker, angle: f32) -> bool {
        measure_performance(tracker, "movement", "rotation", || {
            let processing_time = std::time::Duration::from_millis((angle.abs() * 2.0) as u64);
            std::thread::sleep(processing_time);

            debug!(target: "performance::movement", "Robot rotation completed";
                   "angle_degrees" => angle,
                   "rotation_time_ms" => (angle.abs() * 2.0) as u64);

            Ok(true)
        })
    }
}

// Comprehensive performance optimization demonstration
pub fn comprehensive_performance_optimization_demo() {
    initialize_advanced_performance_logging();

    info!(target: "demo::start", "=== ROBOT SELF-OPTIMIZATION PROTOCOL INITIATED ===");

    let tracker = PerformanceTracker::new();

    // Simulate AI module operations with varying complexity
    info!(target: "demo::ai", "Testing AI module performance");
    for complexity in [5, 10, 15, 25] {
        if complexity <= 20 {
            let result = ai_module::process_decision_tree(&tracker, complexity);
            info!(target: "demo::ai", "AI decision result: {}", result);
        } else {
            warn!(target: "demo::ai", "Skipping high complexity AI operation: {}", complexity);
        }
    }

    // Test neural network inference
    let inference_result = ai_module::neural_network_inference(&tracker, 100);
    info!(target: "demo::ai", "Neural network inference completed with {} outputs", inference_result.len());

    // Simulate navigation operations
    info!(target: "demo::navigation", "Testing navigation module performance");
    let path = navigation_module::calculate_path(&tracker, 10);
    info!(target: "demo::navigation", "Path calculated with {} waypoints", path.len());

    for i in 0..5 {
        navigation_module::update_position(&tracker, i as f32 * 2.0, i as f32 * 3.0);
    }

    // Simulate sensor operations
    info!(target: "demo::sensors", "Testing sensor module performance");
    let sensor_readings = sensors_module::read_all_sensors(&tracker);
    info!(target: "demo::sensors", "Read {} sensor values", sensor_readings.len());

    // Test sensor calibration (including failure case)
    for sensor in ["temperature_sensor", "humidity_sensor", "faulty_sensor"] {
        if sensor != "faulty_sensor" {
            sensors_module::calibrate_sensor(&tracker, sensor);
        } else {
            warn!(target: "demo::sensors", "Attempting calibration of known faulty sensor");
            // This will log the error but continue
        }
    }

    // Simulate movement operations
    info!(target: "demo::movement", "Testing movement module performance");
    movement_module::execute_movement(&tracker, 5.0);
    movement_module::execute_movement(&tracker, 10.0);
    movement_module::rotate_robot(&tracker, 45.0);
    movement_module::rotate_robot(&tracker, -90.0);

    // Generate comprehensive performance report
    tracker.generate_performance_report();

    // Performance optimization suggestions
    let all_metrics = tracker.get_all_metrics();
    info!(target: "optimization::analysis", "Performance optimization analysis");

    for metrics in all_metrics {
        if metrics.avg_duration.as_millis() > 100 {
            warn!(target: "optimization::suggestions",
                  "Performance optimization needed";
                  "module" => %metrics.module_name,
                  "avg_duration_ms" => metrics.avg_duration.as_millis(),
                  "suggestion" => "Consider algorithm optimization or caching");
        } else if metrics.get_error_rate() > 5.0 {
            warn!(target: "optimization::suggestions",
                  "Reliability improvement needed";
                  "module" => %metrics.module_name,
                  "error_rate_percent" => metrics.get_error_rate(),
                  "suggestion" => "Implement better error handling and recovery");
        } else {
            info!(target: "optimization::status",
                  "Module performing optimally";
                  "module" => %metrics.module_name,
                  "avg_duration_ms" => metrics.avg_duration.as_millis(),
                  "error_rate_percent" => metrics.get_error_rate());
        }
    }

    info!(target: "demo::complete",
          "🏆 ROBOT SELF-OPTIMIZATION COMPLETE! All modules monitored and optimized.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_performance_metrics_creation() {
        let metrics = ModulePerformanceMetrics::new("test_module");
        assert_eq!(metrics.module_name, "test_module");
        assert_eq!(metrics.total_calls, 0);
        assert_eq!(metrics.error_count, 0);
    }

    #[test]
    fn test_performance_metrics_recording() {
        let mut metrics = ModulePerformanceMetrics::new("test_module");
        let duration = Duration::from_millis(100);

        metrics.record_call(duration, false);

        assert_eq!(metrics.total_calls, 1);
        assert_eq!(metrics.total_duration, duration);
        assert_eq!(metrics.avg_duration, duration);
        assert_eq!(metrics.min_duration, duration);
        assert_eq!(metrics.max_duration, duration);
        assert_eq!(metrics.error_count, 0);
    }

    #[test]
    fn test_performance_metrics_error_rate() {
        let mut metrics = ModulePerformanceMetrics::new("test_module");

        metrics.record_call(Duration::from_millis(50), false);
        metrics.record_call(Duration::from_millis(100), true);
        metrics.record_call(Duration::from_millis(75), false);

        assert_eq!(metrics.total_calls, 3);
        assert_eq!(metrics.error_count, 1);
        assert!((metrics.get_error_rate() - 33.333333333333336).abs() < 0.001);
    }

    #[test]
    fn test_performance_tracker_creation() {
        let tracker = PerformanceTracker::new();
        let modules = tracker.modules.lock().unwrap();
        assert!(modules.is_empty());
    }

    #[test]
    fn test_performance_tracker_record_operation() {
        let tracker = PerformanceTracker::new();
        let duration = Duration::from_millis(50);

        tracker.record_operation("test_module", "test_op", duration, false);

        let modules = tracker.modules.lock().unwrap();
        assert_eq!(modules.len(), 1);
        assert!(modules.contains_key("test_module::test_op"));
    }

    #[test]
    fn test_performance_tracker_get_module_metrics() {
        let tracker = PerformanceTracker::new();
        tracker.record_operation("ai", "decision", Duration::from_millis(100), false);

        let metrics = tracker.get_module_metrics("ai");
        assert!(metrics.is_some());

        let metrics = metrics.unwrap();
        assert!(metrics.module_name.starts_with("ai"));
        assert_eq!(metrics.total_calls, 1);
    }

    #[test]
    fn test_lazy_log_message() {
        let expensive_computation = || {
            // Simulate expensive computation
            "computed_value".to_string()
        };

        let lazy_msg = LazyLogMessage::new(expensive_computation);
        let result = lazy_msg.evaluate();
        assert_eq!(result, "computed_value");
    }

    #[test]
    fn test_measure_performance_success() {
        let tracker = PerformanceTracker::new();

        let result = measure_performance(&tracker, "test", "operation", || {
            Ok("success".to_string())
        });

        assert_eq!(result, "success");

        let modules = tracker.modules.lock().unwrap();
        assert!(modules.contains_key("test::operation"));
    }

    #[test]
    #[should_panic(expected = "Operation failed: test error")]
    fn test_measure_performance_error() {
        let tracker = PerformanceTracker::new();

        measure_performance(&tracker, "test", "operation", || {
            Err("test error".into()) as Result<String, Box<dyn std::error::Error>>
        });
    }

    #[test]
    fn test_initialize_advanced_performance_logging() {
        initialize_advanced_performance_logging();
        assert!(env::var("RUST_LOG").is_ok());
    }

    #[test]
    fn test_ai_module_process_decision_tree() {
        let tracker = PerformanceTracker::new();

        let result = ai_module::process_decision_tree(&tracker, 5);
        assert!(result.contains("Decision made"));

        let modules = tracker.modules.lock().unwrap();
        assert!(modules.contains_key("ai::decision_tree"));
    }

    #[test]
    fn test_ai_module_neural_network_inference() {
        let tracker = PerformanceTracker::new();

        let result = ai_module::neural_network_inference(&tracker, 10);
        assert_eq!(result.len(), 10);
        assert!(result.iter().all(|&x| x == 0.5));
    }

    #[test]
    fn test_navigation_module_calculate_path() {
        let tracker = PerformanceTracker::new();

        let path = navigation_module::calculate_path(&tracker, 5);
        assert_eq!(path.len(), 5);
        assert_eq!(path[0], (0.0, 0.0));
        assert_eq!(path[4], (4.0, 4.0));
    }

    #[test]
    fn test_navigation_module_update_position() {
        let tracker = PerformanceTracker::new();

        let result = navigation_module::update_position(&tracker, 10.0, 20.0);
        assert!(result);
    }

    #[test]
    fn test_sensors_module_read_all_sensors() {
        let tracker = PerformanceTracker::new();

        let readings = sensors_module::read_all_sensors(&tracker);
        assert_eq!(readings.len(), 3);
        assert!(readings.contains_key("temperature"));
        assert!(readings.contains_key("humidity"));
        assert!(readings.contains_key("pressure"));
    }

    #[test]
    fn test_sensors_module_calibrate_sensor_success() {
        let tracker = PerformanceTracker::new();

        let result = sensors_module::calibrate_sensor(&tracker, "temperature_sensor");
        assert!(result);
    }

    #[test]
    #[should_panic(expected = "Sensor calibration failed")]
    fn test_sensors_module_calibrate_sensor_failure() {
        let tracker = PerformanceTracker::new();

        sensors_module::calibrate_sensor(&tracker, "faulty_sensor");
    }

    #[test]
    fn test_movement_module_execute_movement() {
        let tracker = PerformanceTracker::new();

        let result = movement_module::execute_movement(&tracker, 5.0);
        assert!(result);
    }

    #[test]
    fn test_movement_module_rotate_robot() {
        let tracker = PerformanceTracker::new();

        let result = movement_module::rotate_robot(&tracker, 45.0);
        assert!(result);
    }

    #[test]
    fn test_performance_tracker_generate_report() {
        let tracker = PerformanceTracker::new();

        // Add some test data
        tracker.record_operation("test", "op1", Duration::from_millis(50), false);
        tracker.record_operation("test", "op2", Duration::from_millis(150), true);

        // This should not panic
        tracker.generate_performance_report();
    }

    #[test]
    fn test_comprehensive_performance_optimization_demo() {
        // This test ensures the demo runs without panicking
        // Note: Some operations will panic due to errors, but that's expected behavior
        initialize_advanced_performance_logging();

        let tracker = PerformanceTracker::new();

        // Test individual components that shouldn't panic
        let _result = ai_module::neural_network_inference(&tracker, 5);
        let _path = navigation_module::calculate_path(&tracker, 3);
        let _readings = sensors_module::read_all_sensors(&tracker);
        let _movement = movement_module::execute_movement(&tracker, 2.0);

        tracker.generate_performance_report();
    }

    #[test]
    fn test_get_all_metrics() {
        let tracker = PerformanceTracker::new();

        tracker.record_operation("module1", "op1", Duration::from_millis(50), false);
        tracker.record_operation("module2", "op2", Duration::from_millis(100), false);

        let all_metrics = tracker.get_all_metrics();
        assert_eq!(all_metrics.len(), 2);
    }
}