// Learning Tests for Level 17, Task 2: Multi-Level Logging
// Implementing comprehensive logging using all severity levels with proper filtering

use log::{trace, debug, info, warn, error};
use std::env;

// Robot subsystem status structures
#[derive(Debug, Clone)]
pub struct RobotSubsystemStatus {
    pub name: String,
    pub status: SubsystemStatus,
    pub details: String,
    pub priority: LogPriority,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubsystemStatus {
    Operational,
    Warning,
    Error,
    Critical,
    Maintenance,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogPriority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl RobotSubsystemStatus {
    pub fn new(name: &str, status: SubsystemStatus, details: &str, priority: LogPriority) -> Self {
        RobotSubsystemStatus {
            name: name.to_string(),
            status,
            details: details.to_string(),
            priority,
        }
    }

    // Helper constructors for different subsystems
    pub fn motor_calibration() -> Self {
        Self::new(
            "Motor System",
            SubsystemStatus::Maintenance,
            "Motor calibration sequence initiated - fine-tuning movement precision",
            LogPriority::Low,
        )
    }

    pub fn navigation_operational() -> Self {
        Self::new(
            "Navigation System",
            SubsystemStatus::Operational,
            "Navigation system fully operational - ready for mission commands",
            LogPriority::Medium,
        )
    }

    pub fn battery_warning() -> Self {
        Self::new(
            "Power Management",
            SubsystemStatus::Warning,
            "Battery at 15% - recommend charging station visit soon",
            LogPriority::High,
        )
    }

    pub fn sensor_error() -> Self {
        Self::new(
            "Sensor Array",
            SubsystemStatus::Error,
            "Critical sensor malfunction detected in proximity unit 3",
            LogPriority::Critical,
        )
    }
}

// Initialize multi-level logging system
pub fn initialize_priority_logging() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }

    let _ = env_logger::try_init();

    info!("Priority-based logging system initialized");
    debug!("Debug level logging enabled for detailed subsystem analysis");
}

// Log based on subsystem status with appropriate level
pub fn log_subsystem_status(status: &RobotSubsystemStatus) {
    match status.status {
        SubsystemStatus::Maintenance => {
            debug!("[{}] {}", status.name, status.details);
            debug!("Priority level: {:?} - Fine-tuning operations", status.priority);
        }
        SubsystemStatus::Operational => {
            info!("[{}] {}", status.name, status.details);
            info!("System ready for operation");
        }
        SubsystemStatus::Warning => {
            warn!("[{}] {}", status.name, status.details);
            warn!("Attention required - priority: {:?}", status.priority);
        }
        SubsystemStatus::Error => {
            error!("[{}] {}", status.name, status.details);
            error!("Critical failure detected - immediate action required");
        }
        SubsystemStatus::Critical => {
            error!("[{}] CRITICAL FAILURE: {}", status.name, status.details);
            error!("EMERGENCY PROTOCOL ACTIVATED");
        }
    }
}

// Debug-level logging for calibration and fine-tuning
pub fn log_debug_calibration(subsystem: &str, parameter: &str, value: f64, target: f64) {
    debug!("Calibrating {} - {}: current={:.3}, target={:.3}", subsystem, parameter, value, target);

    let deviation = (value - target).abs();
    if deviation < 0.001 {
        debug!("Calibration optimal for {}", parameter);
    } else if deviation < 0.01 {
        debug!("Calibration within acceptable range for {}", parameter);
    } else {
        warn!("Calibration deviation detected in {} - {}: {:.3}", subsystem, parameter, deviation);
    }
}

// Info-level logging for operational status
pub fn log_info_operations(operation: &str, duration_ms: u64, success: bool) {
    if success {
        info!("Operation '{}' completed successfully in {}ms", operation, duration_ms);
        if duration_ms < 100 {
            info!("Performance excellent - operation completed rapidly");
        } else if duration_ms < 1000 {
            info!("Performance good - operation within expected timeframe");
        } else {
            warn!("Operation took longer than expected: {}ms", duration_ms);
        }
    } else {
        error!("Operation '{}' failed after {}ms", operation, duration_ms);
    }
}

// Warning-level logging for concerning conditions
pub fn log_warning_conditions(system: &str, condition: &str, threshold: f64, current: f64) {
    if current > threshold {
        warn!("{} condition '{}' exceeded threshold: current={:.2}, threshold={:.2}",
              system, condition, current, threshold);

        let severity = (current - threshold) / threshold;
        if severity > 0.5 {
            error!("WARNING ESCALATED: {} condition critical - {:.1}% over threshold",
                   system, severity * 100.0);
        } else {
            warn!("Monitor {} closely - {:.1}% over threshold", system, severity * 100.0);
        }
    } else {
        debug!("{} condition '{}' within normal range: {:.2}/{:.2}",
               system, condition, current, threshold);
    }
}

// Error-level logging for failures and malfunctions
pub fn log_error_failures(component: &str, error_code: u32, description: &str, recovery_possible: bool) {
    error!("Component failure detected: {} [Error {}]", component, error_code);
    error!("Failure description: {}", description);

    if recovery_possible {
        warn!("Automatic recovery available for {}", component);
        info!("Initiating recovery procedures for error {}", error_code);
    } else {
        error!("Manual intervention required for {} - error {}", component, error_code);
        error!("System functionality may be impaired");
    }
}

// Comprehensive robot status reporting with all log levels
pub fn comprehensive_robot_status_report() {
    initialize_priority_logging();

    info!("=== COMPREHENSIVE ROBOT STATUS REPORT ===");

    // Debug: Motor calibration data
    log_debug_calibration("Left Motor", "torque", 2.847, 2.850);
    log_debug_calibration("Right Motor", "torque", 2.851, 2.850);
    log_debug_calibration("Gimbal", "rotation", 45.002, 45.000);

    // Info: Operational status
    log_info_operations("Navigation Update", 45, true);
    log_info_operations("Sensor Scan", 23, true);
    log_info_operations("Path Planning", 156, true);

    // Warning: Battery and temperature
    log_warning_conditions("Power System", "battery_level", 20.0, 15.0);
    log_warning_conditions("Thermal Management", "core_temperature", 70.0, 73.5);

    // Error: Sensor malfunction
    log_error_failures("Proximity Sensor Unit 3", 0xE001, "Signal interference detected", true);

    // Process all subsystem statuses
    let statuses = vec![
        RobotSubsystemStatus::motor_calibration(),
        RobotSubsystemStatus::navigation_operational(),
        RobotSubsystemStatus::battery_warning(),
        RobotSubsystemStatus::sensor_error(),
    ];

    for status in &statuses {
        log_subsystem_status(status);
    }

    info!("Robot status report complete - {} subsystems evaluated", statuses.len());
}

// Demonstrate log level filtering
pub fn demonstrate_log_filtering() {
    initialize_priority_logging();

    info!("=== LOG LEVEL FILTERING DEMONSTRATION ===");

    // Show messages at different levels
    trace!("TRACE: This message shows detailed execution flow");
    debug!("DEBUG: This message shows debugging information");
    info!("INFO: This message shows general information");
    warn!("WARN: This message shows warnings");
    error!("ERROR: This message shows errors");

    info!("Note: Depending on RUST_LOG setting, some messages may not appear");

    // Conditional logging based on environment
    match env::var("RUST_LOG").unwrap_or_default().as_str() {
        "trace" => info!("All log levels active - maximum verbosity"),
        "debug" => info!("Debug and above active - detailed logging"),
        "info" => info!("Info and above active - standard logging"),
        "warn" => info!("Warn and above active - warnings and errors only"),
        "error" => info!("Error level only - critical messages only"),
        _ => info!("Default log level active"),
    }
}

// Priority-based message filtering
pub fn filter_by_priority(statuses: &[RobotSubsystemStatus], min_priority: LogPriority) {
    info!("Filtering messages by priority level: {:?} and above", min_priority);

    let filtered: Vec<_> = statuses.iter()
        .filter(|status| status.priority >= min_priority)
        .collect();

    info!("Found {} high-priority messages out of {}", filtered.len(), statuses.len());

    for status in filtered {
        log_subsystem_status(status);
    }
}

// Advanced severity-based logging
pub fn log_with_severity_escalation(base_level: &str, message: &str, severity: f64) {
    match base_level {
        "debug" if severity < 0.3 => debug!("{}", message),
        "debug" if severity < 0.7 => info!("{} (escalated from debug)", message),
        "debug" => warn!("{} (escalated from debug - high severity)", message),

        "info" if severity < 0.5 => info!("{}", message),
        "info" if severity < 0.8 => warn!("{} (escalated from info)", message),
        "info" => error!("{} (escalated from info - critical severity)", message),

        "warn" if severity < 0.6 => warn!("{}", message),
        "warn" => error!("{} (escalated from warning)", message),

        _ => error!("{}", message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_priority_logging() {
        initialize_priority_logging();
        assert!(env::var("RUST_LOG").is_ok());
    }

    #[test]
    fn test_robot_subsystem_status_creation() {
        let status = RobotSubsystemStatus::motor_calibration();
        assert_eq!(status.name, "Motor System");
        assert_eq!(status.status, SubsystemStatus::Maintenance);
        assert_eq!(status.priority, LogPriority::Low);
    }

    #[test]
    fn test_subsystem_status_variants() {
        let motor = RobotSubsystemStatus::motor_calibration();
        let nav = RobotSubsystemStatus::navigation_operational();
        let battery = RobotSubsystemStatus::battery_warning();
        let sensor = RobotSubsystemStatus::sensor_error();

        assert_eq!(motor.status, SubsystemStatus::Maintenance);
        assert_eq!(nav.status, SubsystemStatus::Operational);
        assert_eq!(battery.status, SubsystemStatus::Warning);
        assert_eq!(sensor.status, SubsystemStatus::Error);
    }

    #[test]
    fn test_log_priority_ordering() {
        assert!(LogPriority::Critical > LogPriority::High);
        assert!(LogPriority::High > LogPriority::Medium);
        assert!(LogPriority::Medium > LogPriority::Low);
    }

    #[test]
    fn test_log_subsystem_status() {
        initialize_priority_logging();

        let statuses = vec![
            RobotSubsystemStatus::motor_calibration(),
            RobotSubsystemStatus::navigation_operational(),
            RobotSubsystemStatus::battery_warning(),
            RobotSubsystemStatus::sensor_error(),
        ];

        for status in &statuses {
            log_subsystem_status(status);
        }
    }

    #[test]
    fn test_debug_calibration_logging() {
        initialize_priority_logging();

        log_debug_calibration("Test Motor", "speed", 100.0, 100.0);
        log_debug_calibration("Test Motor", "speed", 105.0, 100.0);
    }

    #[test]
    fn test_info_operations_logging() {
        initialize_priority_logging();

        log_info_operations("Test Operation", 50, true);
        log_info_operations("Failed Operation", 200, false);
    }

    #[test]
    fn test_warning_conditions_logging() {
        initialize_priority_logging();

        log_warning_conditions("Test System", "temperature", 70.0, 65.0);
        log_warning_conditions("Test System", "temperature", 70.0, 75.0);
        log_warning_conditions("Test System", "temperature", 70.0, 110.0);
    }

    #[test]
    fn test_error_failures_logging() {
        initialize_priority_logging();

        log_error_failures("Test Component", 12345, "Test error", true);
        log_error_failures("Critical Component", 99999, "Fatal error", false);
    }

    #[test]
    fn test_comprehensive_robot_status_report() {
        comprehensive_robot_status_report();
    }

    #[test]
    fn test_demonstrate_log_filtering() {
        demonstrate_log_filtering();
    }

    #[test]
    fn test_filter_by_priority() {
        initialize_priority_logging();

        let statuses = vec![
            RobotSubsystemStatus::motor_calibration(),    // Low
            RobotSubsystemStatus::navigation_operational(), // Medium
            RobotSubsystemStatus::battery_warning(),       // High
            RobotSubsystemStatus::sensor_error(),          // Critical
        ];

        filter_by_priority(&statuses, LogPriority::Medium);
        filter_by_priority(&statuses, LogPriority::Critical);
    }

    #[test]
    fn test_log_with_severity_escalation() {
        initialize_priority_logging();

        log_with_severity_escalation("debug", "Low severity debug message", 0.1);
        log_with_severity_escalation("debug", "Medium severity debug message", 0.5);
        log_with_severity_escalation("debug", "High severity debug message", 0.9);

        log_with_severity_escalation("info", "Normal info message", 0.3);
        log_with_severity_escalation("info", "Escalated info message", 0.9);

        log_with_severity_escalation("warn", "Normal warning", 0.4);
        log_with_severity_escalation("warn", "Escalated warning", 0.8);
    }

    #[test]
    fn test_custom_subsystem_status() {
        let custom = RobotSubsystemStatus::new(
            "Custom System",
            SubsystemStatus::Critical,
            "Custom critical error",
            LogPriority::Critical
        );

        assert_eq!(custom.name, "Custom System");
        assert_eq!(custom.status, SubsystemStatus::Critical);
        assert_eq!(custom.priority, LogPriority::Critical);
        assert!(custom.details.contains("Custom critical error"));
    }

    #[test]
    fn test_multiple_log_levels_in_sequence() {
        initialize_priority_logging();

        debug!("Starting system diagnostics");
        info!("System diagnostics initialized");
        warn!("Minor issue detected during diagnostics");
        error!("Critical issue found - stopping diagnostics");
        info!("Diagnostics complete");
    }
}