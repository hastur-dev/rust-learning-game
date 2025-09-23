//! Level 12 Task 2: Timeout Operations with Smol
//!
//! This module demonstrates how to implement timeout behavior using racing with timers
//! in the Smol async runtime. Learn to handle operations that may take too long.

use std::time::Duration;
use futures_lite::future;

/// Mock robot for testing timeout operations
#[derive(Debug, Clone)]
pub struct TimeoutRobot {
    pub position: (i32, i32),
    pub scan_delay: Duration,
    pub timeout_threshold: Duration,
}

impl TimeoutRobot {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            scan_delay: Duration::from_millis(300),
            timeout_threshold: Duration::from_secs(1),
        }
    }

    pub fn with_scan_delay(mut self, delay: Duration) -> Self {
        self.scan_delay = delay;
        self
    }

    pub fn with_timeout_threshold(mut self, threshold: Duration) -> Self {
        self.timeout_threshold = threshold;
        self
    }

    /// Simulate a scan operation that takes time
    pub async fn scan(&self, scan_type: &str) -> String {
        smol::Timer::after(self.scan_delay).await;
        match scan_type {
            "quick" => "basic_scan_result".to_string(),
            "deep_scan" => "detailed_scan_with_coordinates_and_energy_readings".to_string(),
            "full_analysis" => "comprehensive_environmental_analysis_complete".to_string(),
            _ => "unknown_scan_type".to_string(),
        }
    }

    /// Move to a new position
    pub async fn move_to(&mut self, x: i32, y: i32) {
        smol::Timer::after(Duration::from_millis(100)).await;
        self.position = (x, y);
    }

    /// Get current position
    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }
}

/// Task 2: Implement timeout operations using racing
pub async fn scan_with_timeout(robot: &TimeoutRobot) -> Result<String, &'static str> {
    let scan_future = async {
        robot.scan("deep_scan").await
    };

    let timeout_future = async {
        smol::Timer::after(robot.timeout_threshold).await;
        "timeout"
    };

    // Race between scan and timeout
    match future::race(scan_future, timeout_future).await {
        future::Either::Left(scan_result) => {
            println!("Scan completed: {}", scan_result);
            Ok(scan_result)
        }
        future::Either::Right(_) => {
            println!("Scan timeout - using quick scan");
            Err("timeout")
        }
    }
}

/// Advanced timeout with fallback strategy
pub async fn scan_with_fallback(robot: &TimeoutRobot) -> String {
    let primary_scan = async {
        robot.scan("full_analysis").await
    };

    let timeout_with_fallback = async {
        smol::Timer::after(Duration::from_millis(500)).await;
        robot.scan("quick").await
    };

    match future::race(primary_scan, timeout_with_fallback).await {
        future::Either::Left(detailed_result) => {
            println!("Primary scan completed: {}", detailed_result);
            detailed_result
        }
        future::Either::Right(quick_result) => {
            println!("Primary scan timed out, used fallback: {}", quick_result);
            quick_result
        }
    }
}

/// Multiple operations with individual timeouts
pub async fn multi_operation_with_timeouts(robot: &mut TimeoutRobot) -> Vec<Result<String, &'static str>> {
    let operations = vec![
        ("scan", "deep_scan"),
        ("scan", "full_analysis"),
        ("scan", "quick"),
    ];

    let mut results = Vec::new();

    for (op_type, param) in operations {
        let result = match op_type {
            "scan" => {
                let scan_future = robot.scan(param);
                let timeout_future = async {
                    smol::Timer::after(Duration::from_millis(400)).await;
                    "timeout"
                };

                match future::race(scan_future, timeout_future).await {
                    future::Either::Left(scan_result) => Ok(scan_result),
                    future::Either::Right(_) => Err("timeout"),
                }
            }
            _ => Err("unknown_operation"),
        };

        results.push(result);
        smol::Timer::after(Duration::from_millis(100)).await; // Small delay between operations
    }

    results
}

/// Timeout with retry mechanism
pub async fn scan_with_retry(robot: &TimeoutRobot, max_retries: u32) -> Result<String, String> {
    let mut attempts = 0;

    while attempts <= max_retries {
        let scan_future = robot.scan("deep_scan");
        let timeout_future = async {
            smol::Timer::after(Duration::from_millis(600)).await;
            "timeout"
        };

        match future::race(scan_future, timeout_future).await {
            future::Either::Left(scan_result) => {
                println!("Scan succeeded on attempt {}: {}", attempts + 1, scan_result);
                return Ok(scan_result);
            }
            future::Either::Right(_) => {
                attempts += 1;
                if attempts <= max_retries {
                    println!("Scan timeout on attempt {}, retrying...", attempts);
                    smol::Timer::after(Duration::from_millis(200)).await; // Wait before retry
                } else {
                    println!("All attempts exhausted, scan failed");
                    return Err("max_retries_exceeded".to_string());
                }
            }
        }
    }

    Err("unexpected_exit".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeout_robot_creation() {
        let robot = TimeoutRobot::new();
        assert_eq!(robot.position, (0, 0));
        assert_eq!(robot.scan_delay, Duration::from_millis(300));
        assert_eq!(robot.timeout_threshold, Duration::from_secs(1));
    }

    #[test]
    fn test_timeout_robot_builder() {
        let robot = TimeoutRobot::new()
            .with_scan_delay(Duration::from_millis(500))
            .with_timeout_threshold(Duration::from_millis(800));

        assert_eq!(robot.scan_delay, Duration::from_millis(500));
        assert_eq!(robot.timeout_threshold, Duration::from_millis(800));
    }

    #[smol_potat::test]
    async fn test_basic_scan_operation() {
        let robot = TimeoutRobot::new();
        let result = robot.scan("quick").await;
        assert_eq!(result, "basic_scan_result");
    }

    #[smol_potat::test]
    async fn test_scan_with_successful_timeout() {
        let robot = TimeoutRobot::new()
            .with_scan_delay(Duration::from_millis(200))
            .with_timeout_threshold(Duration::from_secs(1));

        let result = scan_with_timeout(&robot).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "detailed_scan_with_coordinates_and_energy_readings");
    }

    #[smol_potat::test]
    async fn test_scan_with_timeout_failure() {
        let robot = TimeoutRobot::new()
            .with_scan_delay(Duration::from_millis(1200))
            .with_timeout_threshold(Duration::from_millis(500));

        let result = scan_with_timeout(&robot).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "timeout");
    }

    #[smol_potat::test]
    async fn test_scan_with_fallback_primary_success() {
        let robot = TimeoutRobot::new()
            .with_scan_delay(Duration::from_millis(200));

        let result = scan_with_fallback(&robot).await;
        assert_eq!(result, "comprehensive_environmental_analysis_complete");
    }

    #[smol_potat::test]
    async fn test_scan_with_fallback_timeout() {
        let robot = TimeoutRobot::new()
            .with_scan_delay(Duration::from_millis(800));

        let result = scan_with_fallback(&robot).await;
        assert_eq!(result, "basic_scan_result");
    }

    #[smol_potat::test]
    async fn test_multi_operation_timeouts() {
        let mut robot = TimeoutRobot::new()
            .with_scan_delay(Duration::from_millis(300));

        let results = multi_operation_with_timeouts(&mut robot).await;
        assert_eq!(results.len(), 3);

        // First operation should succeed (300ms scan < 400ms timeout)
        assert!(results[0].is_ok());
        // Second operation should succeed (300ms scan < 400ms timeout)
        assert!(results[1].is_ok());
        // Third operation should succeed (300ms scan < 400ms timeout)
        assert!(results[2].is_ok());
    }

    #[smol_potat::test]
    async fn test_multi_operation_with_some_timeouts() {
        let mut robot = TimeoutRobot::new()
            .with_scan_delay(Duration::from_millis(500));

        let results = multi_operation_with_timeouts(&mut robot).await;
        assert_eq!(results.len(), 3);

        // All operations should timeout (500ms scan > 400ms timeout)
        for result in results {
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "timeout");
        }
    }

    #[smol_potat::test]
    async fn test_scan_with_retry_success_first_attempt() {
        let robot = TimeoutRobot::new()
            .with_scan_delay(Duration::from_millis(400));

        let result = scan_with_retry(&robot, 3).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "detailed_scan_with_coordinates_and_energy_readings");
    }

    #[smol_potat::test]
    async fn test_scan_with_retry_eventual_success() {
        // This test is conceptual - in practice, the scan delay is consistent
        // In a real scenario, you might have variable conditions
        let robot = TimeoutRobot::new()
            .with_scan_delay(Duration::from_millis(400));

        let result = scan_with_retry(&robot, 3).await;
        assert!(result.is_ok());
    }

    #[smol_potat::test]
    async fn test_scan_with_retry_max_retries_exceeded() {
        let robot = TimeoutRobot::new()
            .with_scan_delay(Duration::from_millis(800));

        let result = scan_with_retry(&robot, 2).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "max_retries_exceeded");
    }

    #[smol_potat::test]
    async fn test_robot_movement() {
        let mut robot = TimeoutRobot::new();
        assert_eq!(robot.get_position(), (0, 0));

        robot.move_to(5, 3).await;
        assert_eq!(robot.get_position(), (5, 3));
    }

    #[smol_potat::test]
    async fn test_different_scan_types() {
        let robot = TimeoutRobot::new();

        let quick_scan = robot.scan("quick").await;
        assert_eq!(quick_scan, "basic_scan_result");

        let deep_scan = robot.scan("deep_scan").await;
        assert_eq!(deep_scan, "detailed_scan_with_coordinates_and_energy_readings");

        let full_scan = robot.scan("full_analysis").await;
        assert_eq!(full_scan, "comprehensive_environmental_analysis_complete");

        let unknown_scan = robot.scan("unknown").await;
        assert_eq!(unknown_scan, "unknown_scan_type");
    }

    #[smol_potat::test]
    async fn test_complex_timeout_scenario() {
        // Test a complex scenario with multiple timeout operations
        let robot = TimeoutRobot::new()
            .with_scan_delay(Duration::from_millis(350))
            .with_timeout_threshold(Duration::from_millis(400));

        // First scan should succeed
        let result1 = scan_with_timeout(&robot).await;
        assert!(result1.is_ok());

        // Change robot to have longer scan delay
        let slow_robot = robot.with_scan_delay(Duration::from_millis(500));

        // Second scan should timeout
        let result2 = scan_with_timeout(&slow_robot).await;
        assert!(result2.is_err());

        // But fallback should work
        let fallback_result = scan_with_fallback(&slow_robot).await;
        assert_eq!(fallback_result, "basic_scan_result");
    }
}

/// Example usage and demonstrations
pub async fn demonstrate_timeout_operations() {
    println!("=== Level 12 Task 2: Timeout Operations Demo ===");

    // Create test robot
    let robot = TimeoutRobot::new();
    println!("Robot created at position: {:?}", robot.get_position());

    // Test basic timeout operation
    println!("\n1. Testing basic timeout operation...");
    match scan_with_timeout(&robot).await {
        Ok(result) => println!("   Scan successful: {}", result),
        Err(err) => println!("   Scan failed: {}", err),
    }

    // Test fallback mechanism
    println!("\n2. Testing fallback mechanism...");
    let fallback_result = scan_with_fallback(&robot).await;
    println!("   Fallback result: {}", fallback_result);

    // Test multiple operations
    println!("\n3. Testing multiple operations with timeouts...");
    let mut test_robot = robot.clone();
    let multi_results = multi_operation_with_timeouts(&mut test_robot).await;
    for (i, result) in multi_results.iter().enumerate() {
        match result {
            Ok(scan) => println!("   Operation {}: Success - {}", i + 1, scan),
            Err(err) => println!("   Operation {}: Failed - {}", i + 1, err),
        }
    }

    // Test retry mechanism
    println!("\n4. Testing retry mechanism...");
    match scan_with_retry(&robot, 3).await {
        Ok(result) => println!("   Retry successful: {}", result),
        Err(err) => println!("   Retry failed: {}", err),
    }

    println!("\n✅ Timeout operations demonstration complete!");
}

#[smol_potat::main]
async fn main() {
    demonstrate_timeout_operations().await;
}