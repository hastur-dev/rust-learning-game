// Learning Tests for Level 17, Task 1: Initialize Basic Logging
// Setting up env_logger and capturing diagnostic output with trace-level logging

use log::{trace, debug, info, warn, error};
use std::env;

// Mock diagnostic data structures
#[derive(Debug, Clone)]
pub struct SystemDiagnostics {
    pub battery_voltage: f32,
    pub core_temperature: f32,
    pub memory_fragmentation: f32,
    pub sensors_responding: u8,
    pub total_sensors: u8,
}

impl SystemDiagnostics {
    pub fn new() -> Self {
        SystemDiagnostics {
            battery_voltage: 11.2,
            core_temperature: 78.0,
            memory_fragmentation: 23.0,
            sensors_responding: 8,
            total_sensors: 12,
        }
    }

    pub fn emergency_data() -> Self {
        SystemDiagnostics {
            battery_voltage: 10.1, // Critical level
            core_temperature: 85.0, // Overheating
            memory_fragmentation: 45.0, // High fragmentation
            sensors_responding: 5,
            total_sensors: 12,
        }
    }
}

// Initialize logging with env_logger
pub fn initialize_emergency_logging() {
    // Check if logger is already initialized to avoid double initialization
    if env::var("RUST_LOG").is_err() {
        unsafe {
            env::set_var("RUST_LOG", "trace");
        }
    }

    // Initialize env_logger - this should only be called once
    let _ = env_logger::try_init();

    info!("Emergency logging system initialized");
    trace!("Log level set to TRACE for comprehensive diagnostic capture");
}

// Capture vital diagnostic traces
pub fn capture_emergency_diagnostics(diagnostics: &SystemDiagnostics) {
    trace!("=== EMERGENCY DIAGNOSTIC STREAM ===");
    trace!("Battery voltage: {:.1}V (dropping rapidly)", diagnostics.battery_voltage);
    trace!("Core temperature: {:.1}°C (rising)", diagnostics.core_temperature);
    trace!("Memory fragmentation: {:.1}%", diagnostics.memory_fragmentation);
    trace!("Sensor array: {}/{} sensors responding",
           diagnostics.sensors_responding, diagnostics.total_sensors);

    // Additional diagnostic traces for comprehensive monitoring
    trace!("System stability index: {:.2}", calculate_stability_index(diagnostics));
    trace!("Critical threshold breached: {}", is_critical_state(diagnostics));
    trace!("Recommended action: {}", recommend_action(diagnostics));
}

// Calculate system stability index
fn calculate_stability_index(diagnostics: &SystemDiagnostics) -> f32 {
    let battery_factor = (diagnostics.battery_voltage / 12.0).min(1.0);
    let temp_factor = (1.0 - (diagnostics.core_temperature - 50.0) / 50.0).max(0.0);
    let memory_factor = (1.0 - diagnostics.memory_fragmentation / 100.0).max(0.0);
    let sensor_factor = diagnostics.sensors_responding as f32 / diagnostics.total_sensors as f32;

    (battery_factor + temp_factor + memory_factor + sensor_factor) / 4.0
}

// Determine if system is in critical state
fn is_critical_state(diagnostics: &SystemDiagnostics) -> bool {
    diagnostics.battery_voltage < 10.5 ||
    diagnostics.core_temperature > 80.0 ||
    diagnostics.memory_fragmentation > 40.0 ||
    diagnostics.sensors_responding < 6
}

// Recommend action based on diagnostics
fn recommend_action(diagnostics: &SystemDiagnostics) -> &'static str {
    if diagnostics.battery_voltage < 10.0 {
        "IMMEDIATE CHARGING REQUIRED"
    } else if diagnostics.core_temperature > 85.0 {
        "EMERGENCY COOLING PROTOCOL"
    } else if diagnostics.memory_fragmentation > 50.0 {
        "MEMORY DEFRAGMENTATION NEEDED"
    } else if diagnostics.sensors_responding < 4 {
        "SENSOR ARRAY REPLACEMENT"
    } else if is_critical_state(diagnostics) {
        "SYSTEM MAINTENANCE REQUIRED"
    } else {
        "CONTINUE NORMAL OPERATION"
    }
}

// Emergency stabilization function
pub fn emergency_diagnostics_online() {
    initialize_emergency_logging();

    let diagnostics = SystemDiagnostics::emergency_data();
    capture_emergency_diagnostics(&diagnostics);

    if calculate_stability_index(&diagnostics) > 0.3 {
        info!("Emergency diagnostics online - robot stabilizing");
    } else {
        error!("Emergency diagnostics online - critical intervention required");
    }

    trace!("Emergency diagnostic system fully operational");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_emergency_logging() {
        initialize_emergency_logging();
        assert!(env::var("RUST_LOG").is_ok());
    }

    #[test]
    fn test_system_diagnostics_creation() {
        let diagnostics = SystemDiagnostics::new();
        assert_eq!(diagnostics.battery_voltage, 11.2);
        assert_eq!(diagnostics.core_temperature, 78.0);
        assert_eq!(diagnostics.memory_fragmentation, 23.0);
        assert_eq!(diagnostics.sensors_responding, 8);
        assert_eq!(diagnostics.total_sensors, 12);
    }

    #[test]
    fn test_emergency_diagnostics_data() {
        let emergency = SystemDiagnostics::emergency_data();
        assert_eq!(emergency.battery_voltage, 10.1);
        assert_eq!(emergency.core_temperature, 85.0);
        assert_eq!(emergency.memory_fragmentation, 45.0);
        assert_eq!(emergency.sensors_responding, 5);
    }

    #[test]
    fn test_stability_index_calculation() {
        let normal_diagnostics = SystemDiagnostics::new();
        let stability = calculate_stability_index(&normal_diagnostics);
        assert!(stability > 0.5);

        let emergency_diagnostics = SystemDiagnostics::emergency_data();
        let emergency_stability = calculate_stability_index(&emergency_diagnostics);
        assert!(emergency_stability < stability);
    }

    #[test]
    fn test_critical_state_detection() {
        let normal_diagnostics = SystemDiagnostics::new();
        assert!(!is_critical_state(&normal_diagnostics));

        let emergency_diagnostics = SystemDiagnostics::emergency_data();
        assert!(is_critical_state(&emergency_diagnostics));
    }

    #[test]
    fn test_action_recommendations() {
        let low_battery = SystemDiagnostics {
            battery_voltage: 9.5,
            ..SystemDiagnostics::new()
        };
        assert_eq!(recommend_action(&low_battery), "IMMEDIATE CHARGING REQUIRED");

        let high_temp = SystemDiagnostics {
            core_temperature: 90.0,
            ..SystemDiagnostics::new()
        };
        assert_eq!(recommend_action(&high_temp), "EMERGENCY COOLING PROTOCOL");
    }

    #[test]
    fn test_capture_emergency_diagnostics() {
        initialize_emergency_logging();
        let diagnostics = SystemDiagnostics::emergency_data();
        capture_emergency_diagnostics(&diagnostics);
    }

    #[test]
    fn test_emergency_diagnostics_online() {
        emergency_diagnostics_online();
    }
}