// Level 17 Task 1 Test: Initialize Basic Logging
// Tests that user sets up env_logger and captures diagnostic output with trace-level logging

#[cfg(test)]
mod level17_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_env_logger_import() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("env_logger"),
            "❌ You need to import env_logger for logging initialization"
        );
    }

    #[test]
    fn test_has_log_import() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("log::") || analyzer.code.contains("use log"),
            "❌ You need to import log crate for logging macros"
        );
    }

    #[test]
    fn test_initializes_env_logger() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_init = analyzer.code.contains("env_logger::init") ||
                      analyzer.code.contains("env_logger::Builder");
        assert!(
            has_init,
            "❌ You need to initialize env_logger with env_logger::init()"
        );
    }

    #[test]
    fn test_uses_trace_macro() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("log::trace!") || analyzer.code.contains("trace!"),
            "❌ You should use log::trace!() macro to capture diagnostic messages"
        );
    }

    #[test]
    fn test_has_diagnostic_function() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_diagnostic = analyzer.code.contains("diagnostic") &&
                           (analyzer.code.contains("fn ") || analyzer.code.contains("capture"));
        assert!(
            has_diagnostic,
            "❌ You should create a diagnostic capture function"
        );
    }

    #[test]
    fn test_handles_rust_log_env() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_env_handling = analyzer.code.contains("RUST_LOG") ||
                             analyzer.code.contains("env::var") ||
                             analyzer.code.contains("filter");
        assert!(
            has_env_handling,
            "❌ You should handle RUST_LOG environment variable for log level control"
        );
    }

    #[test]
    fn test_prints_initialization_message() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_init_message = analyzer.code.contains("Diagnostic logging initialized") ||
                             analyzer.code.contains("logging initialized");
        assert!(
            has_init_message,
            "❌ You should print 'Diagnostic logging initialized' message"
        );
    }

    #[test]
    fn test_captures_system_diagnostics() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_diagnostics = analyzer.code.contains("Battery") ||
                            analyzer.code.contains("Sensors") ||
                            analyzer.code.contains("Position");
        assert!(
            has_diagnostics,
            "❌ You should capture system diagnostic information (Battery, Sensors, Position)"
        );
    }
}

// Reference implementation
fn main() {
    println!("Level 17 Task 1: Basic Logging Setup");
    // Reference pattern for basic logging initialization
}

// Reference logging pattern
// fn initialize_diagnostic_logging() {
//     env_logger::init();
//     log::trace!("Diagnostic logging initialized");
//
//     // Capture robot diagnostics
//     log::trace!("Battery: 85%");
//     log::trace!("Sensors: Online");
//     log::trace!("Position: Calibrated");
// }