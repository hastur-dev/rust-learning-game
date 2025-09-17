// Level 22 Task 1 Test: Unified Error Recovery System
// Tests that user implements anyhow for unified error handling across all robot systems

#[cfg(test)]
mod level22_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_anyhow_import() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_anyhow = analyzer.code.contains("anyhow") ||
                        analyzer.code.contains("use anyhow");
        assert!(
            has_anyhow,
            "❌ You need to import anyhow for unified error handling"
        );
    }

    #[test]
    fn test_uses_anyhow_result() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_anyhow_result = analyzer.code.contains("anyhow::Result") ||
                               analyzer.code.contains("Result<") && analyzer.code.contains("anyhow");
        assert!(
            has_anyhow_result,
            "❌ You should use anyhow::Result<T> for unified error handling"
        );
    }

    #[test]
    fn test_handles_multiple_error_types() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_multiple_errors = analyzer.code.contains("std::io::Error") ||
                                 analyzer.code.contains("ParseFloatError") ||
                                 analyzer.code.contains("sensor") ||
                                 analyzer.code.contains("navigation");
        assert!(
            has_multiple_errors,
            "❌ You should handle multiple different error types (io::Error, ParseFloatError, etc.)"
        );
    }

    #[test]
    fn test_uses_context_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_context = analyzer.code.contains(".context(") ||
                         analyzer.code.contains("with_context");
        assert!(
            has_context,
            "❌ You should use .context() method to add meaningful error descriptions"
        );
    }

    #[test]
    fn test_has_error_conversion() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_conversion = analyzer.code.contains(".into()") ||
                           analyzer.code.contains("?") ||
                           analyzer.code.contains("anyhow::Error::from");
        assert!(
            has_conversion,
            "❌ You should convert different error types to anyhow::Error"
        );
    }

    #[test]
    fn test_has_recovery_functions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_recovery = analyzer.code.contains("fn ") &&
                          (analyzer.code.contains("recover") ||
                           analyzer.code.contains("handle") ||
                           analyzer.code.contains("repair"));
        assert!(
            has_recovery,
            "❌ You should create recovery functions that handle system failures"
        );
    }

    #[test]
    fn test_handles_sensor_errors() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_sensor_handling = analyzer.code.contains("sensor") &&
                                 (analyzer.code.contains("timeout") ||
                                  analyzer.code.contains("connection") ||
                                  analyzer.code.contains("malfunction"));
        assert!(
            has_sensor_handling,
            "❌ You should handle sensor array connection timeout errors"
        );
    }

    #[test]
    fn test_handles_navigation_errors() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_nav_handling = analyzer.code.contains("navigation") ||
                              analyzer.code.contains("GPS") ||
                              analyzer.code.contains("coordinate");
        assert!(
            has_nav_handling,
            "❌ You should handle navigation and GPS coordinate errors"
        );
    }

    #[test]
    fn test_demonstrates_unified_handling() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ You should demonstrate unified error handling with output"
        );
    }

    #[test]
    fn test_emergency_recovery_message() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_recovery_msg = analyzer.code.contains("emergency") ||
                              analyzer.code.contains("system") ||
                              analyzer.code.contains("recovery");
        assert!(
            has_recovery_msg,
            "❌ You should include emergency recovery or system repair messages"
        );
    }
}

// Reference implementation
fn main() {
    println!("Level 22 Task 1: Unified Error Recovery System");
    // Reference pattern for anyhow unified error handling
}

// Reference unified error handling pattern
// use anyhow::{Result, Context};
//
// fn emergency_system_recovery() -> Result<()> {
//     // Handle sensor errors
//     check_sensor_array()
//         .context("Failed to verify sensor array during emergency recovery")?;
//
//     // Handle navigation errors
//     verify_gps_coordinates()
//         .context("GPS system malfunction during recovery procedure")?;
//
//     // Handle power errors
//     check_battery_systems()
//         .context("Critical power system failure during emergency protocol")?;
//
//     println!("✅ Emergency recovery complete - all systems operational!");
//     Ok(())
// }
//
// fn check_sensor_array() -> Result<()> {
//     // Simulate sensor timeout error
//     Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Connection timeout after 30 seconds"))
//         .context("Sensor array communication failure")
// }
//
// fn verify_gps_coordinates() -> Result<()> {
//     // Simulate GPS parsing error
//     "invalid_latitude".parse::<f64>()
//         .context("Invalid GPS coordinates: latitude out of range")?;
//     Ok(())
// }