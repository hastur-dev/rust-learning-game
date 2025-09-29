// Learning Tests for Level 17, Task 4: Custom Log Formatting and Output Redirection
// Creating custom log formatters and redirecting output to files with rotation

use log::{trace, debug, info, warn, error};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufWriter};
use std::path::Path;
use std::sync::Mutex;

// Custom log entry structure for parsing
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub robot_id: String,
    pub operation: String,
    pub details: String,
    pub duration: Option<f64>,
    pub status: String,
}

impl LogEntry {
    pub fn new(robot_id: &str, operation: &str, details: &str, status: &str) -> Self {
        LogEntry {
            timestamp: format_timestamp(),
            level: "INFO".to_string(),
            robot_id: robot_id.to_string(),
            operation: operation.to_string(),
            details: details.to_string(),
            duration: None,
            status: status.to_string(),
        }
    }

    pub fn with_level(mut self, level: &str) -> Self {
        self.level = level.to_string();
        self
    }

    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    // Format log entry in custom format
    pub fn format_custom(&self) -> String {
        if let Some(duration) = self.duration {
            format!("[{}] {} {} {} -> {} {} {:.1}s",
                    self.timestamp,
                    self.robot_id,
                    self.operation,
                    self.details,
                    self.status,
                    self.level,
                    duration)
        } else {
            format!("[{}] {} {} {} -> {}",
                    self.timestamp,
                    self.robot_id,
                    self.operation,
                    self.details,
                    self.status)
        }
    }

    // Parse from custom format string
    pub fn parse_custom(log_line: &str) -> Result<Self, String> {
        // Parse format: [timestamp] ROBOT_ID OPERATION details -> STATUS LEVEL duration
        let parts: Vec<&str> = log_line.split(" -> ").collect();
        if parts.len() != 2 {
            return Err("Invalid log format: missing ' -> ' separator".to_string());
        }

        let left_part = parts[0];
        let right_part = parts[1];

        // Extract timestamp
        if !left_part.starts_with('[') {
            return Err("Invalid log format: missing timestamp brackets".to_string());
        }

        let timestamp_end = left_part.find(']').ok_or("Invalid timestamp format")?;
        let timestamp = left_part[1..timestamp_end].to_string();

        // Parse the rest of left part
        let remaining = &left_part[timestamp_end + 2..]; // Skip "] "
        let parts: Vec<&str> = remaining.split_whitespace().collect();

        if parts.len() < 3 {
            return Err("Invalid log format: insufficient parts".to_string());
        }

        let robot_id = parts[0].to_string();
        let operation = parts[1].to_string();
        let details = parts[2..].join(" ");

        // Parse right part (status and optional level/duration)
        let right_parts: Vec<&str> = right_part.split_whitespace().collect();
        let status = right_parts[0].to_string();

        let mut level = "INFO".to_string();
        let mut duration = None;

        // Parse additional parts
        for part in &right_parts[1..] {
            if part.ends_with('s') && part.len() > 1 {
                // Try to parse as duration
                let duration_str = &part[..part.len() - 1];
                if let Ok(dur) = duration_str.parse::<f64>() {
                    duration = Some(dur);
                }
            } else if ["DEBUG", "INFO", "WARN", "ERROR", "TRACE"].contains(part) {
                level = part.to_string();
            }
        }

        Ok(LogEntry {
            timestamp,
            level,
            robot_id,
            operation,
            details,
            duration,
            status,
        })
    }
}

// File-based logger with rotation
pub struct FileLogger {
    current_file: Mutex<Option<BufWriter<File>>>,
    base_filename: String,
    max_file_size: u64,
    current_size: Mutex<u64>,
    file_counter: Mutex<u32>,
}

impl FileLogger {
    pub fn new(base_filename: &str, max_file_size: u64) -> Self {
        FileLogger {
            current_file: Mutex::new(None),
            base_filename: base_filename.to_string(),
            max_file_size,
            current_size: Mutex::new(0),
            file_counter: Mutex::new(0),
        }
    }

    pub fn log(&self, entry: &LogEntry) -> Result<(), std::io::Error> {
        let formatted = entry.format_custom();
        let formatted_with_newline = format!("{}\n", formatted);

        let mut current_file = self.current_file.lock().unwrap();
        let mut current_size = self.current_size.lock().unwrap();

        // Check if we need to rotate the file
        if current_file.is_none() || *current_size + formatted_with_newline.len() as u64 > self.max_file_size {
            self.rotate_file(&mut current_file)?;
            *current_size = 0;
        }

        if let Some(ref mut writer) = *current_file {
            writer.write_all(formatted_with_newline.as_bytes())?;
            writer.flush()?;
            *current_size += formatted_with_newline.len() as u64;
        }

        Ok(())
    }

    fn rotate_file(&self, current_file: &mut Option<BufWriter<File>>) -> Result<(), std::io::Error> {
        // Close current file
        if let Some(writer) = current_file.take() {
            let _ = writer.into_inner()?.sync_all();
        }

        let mut counter = self.file_counter.lock().unwrap();
        let filename = if *counter == 0 {
            format!("{}.log", self.base_filename)
        } else {
            format!("{}.{}.log", self.base_filename, counter)
        };

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filename)?;

        *current_file = Some(BufWriter::new(file));
        *counter += 1;

        println!("Log rotated to: {}", filename);
        Ok(())
    }

    pub fn close(&self) -> Result<(), std::io::Error> {
        let mut current_file = self.current_file.lock().unwrap();
        if let Some(writer) = current_file.take() {
            writer.into_inner()?.sync_all()?;
        }
        Ok(())
    }
}

// Custom timestamp formatting
fn format_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Convert to a readable format (simplified)
    format!("2023-10-18 14:{:02}:{:02}", (timestamp % 3600) / 60, timestamp % 60)
}

// Initialize custom logging system
pub fn initialize_custom_logging() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    let _ = env_logger::try_init();

    info!("Custom logging system with file output initialized");
    debug!("Log format: [timestamp] ROBOT_ID OPERATION details -> STATUS LEVEL duration");
}

// Log robot movement with custom format
pub fn log_robot_movement(robot_id: &str, from_pos: (f64, f64), to_pos: (f64, f64), duration: f64, logger: &FileLogger) {
    let details = format!("({:.1},{:.1}) -> ({:.1},{:.1})", from_pos.0, from_pos.1, to_pos.0, to_pos.1);
    let status = if duration < 2.0 { "SUCCESS" } else { "SLOW" };

    let entry = LogEntry::new(robot_id, "MOVE", &details, status)
        .with_duration(duration);

    logger.log(&entry).unwrap_or_else(|e| {
        error!("Failed to write to log file: {}", e);
    });

    // Also log to standard logger
    info!("Movement logged: {}", entry.format_custom());
}

// Log robot scanning operation
pub fn log_robot_scan(robot_id: &str, area: &str, items_found: u32, energy_used: f32, logger: &FileLogger) {
    let details = format!("area={} items={} energy_used={:.1}%", area, items_found, energy_used);
    let status = if items_found > 0 { "ITEMS_FOUND" } else { "EMPTY" };

    let entry = LogEntry::new(robot_id, "SCAN", &details, status);

    logger.log(&entry).unwrap_or_else(|e| {
        error!("Failed to write to log file: {}", e);
    });

    info!("Scan logged: {}", entry.format_custom());
}

// Log robot error with recovery attempt
pub fn log_robot_error(robot_id: &str, error_type: &str, position: (f64, f64), recovery_attempted: bool, logger: &FileLogger) {
    let details = format!("{} pos=({:.1},{:.1})", error_type, position.0, position.1);
    let status = if recovery_attempted { "RETRYING" } else { "FAILED" };

    let entry = LogEntry::new(robot_id, "ERROR", &details, status)
        .with_level("ERROR");

    logger.log(&entry).unwrap_or_else(|e| {
        error!("Failed to write to log file: {}", e);
    });

    error!("Error logged: {}", entry.format_custom());
}

// Parse and analyze log files
pub fn parse_log_file(filename: &str) -> Result<Vec<LogEntry>, String> {
    use std::fs;
    use std::io::{BufRead, BufReader};

    if !Path::new(filename).exists() {
        return Err(format!("Log file does not exist: {}", filename));
    }

    let file = File::open(filename).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);

    let mut entries = Vec::new();

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(|e| format!("Failed to read line {}: {}", line_num + 1, e))?;

        if line.trim().is_empty() {
            continue;
        }

        match LogEntry::parse_custom(&line) {
            Ok(entry) => entries.push(entry),
            Err(e) => {
                eprintln!("Failed to parse line {}: {} - Error: {}", line_num + 1, line, e);
            }
        }
    }

    Ok(entries)
}

// Analyze log entries for patterns
pub fn analyze_log_entries(entries: &[LogEntry]) -> LogAnalysis {
    let mut analysis = LogAnalysis::new();

    for entry in entries {
        analysis.total_entries += 1;

        match entry.level.as_str() {
            "ERROR" => analysis.error_count += 1,
            "WARN" => analysis.warning_count += 1,
            "INFO" => analysis.info_count += 1,
            "DEBUG" => analysis.debug_count += 1,
            _ => analysis.other_count += 1,
        }

        match entry.operation.as_str() {
            "MOVE" => analysis.movement_operations += 1,
            "SCAN" => analysis.scan_operations += 1,
            "ERROR" => analysis.error_operations += 1,
            _ => analysis.other_operations += 1,
        }

        if let Some(duration) = entry.duration {
            analysis.total_duration += duration;
            if duration > analysis.max_duration {
                analysis.max_duration = duration;
                analysis.slowest_operation = Some(entry.clone());
            }
        }

        if !analysis.robots_seen.contains(&entry.robot_id) {
            analysis.robots_seen.push(entry.robot_id.clone());
        }
    }

    if analysis.movement_operations > 0 {
        analysis.avg_movement_duration = analysis.total_duration / analysis.movement_operations as f64;
    }

    analysis
}

#[derive(Debug, Clone)]
pub struct LogAnalysis {
    pub total_entries: u32,
    pub error_count: u32,
    pub warning_count: u32,
    pub info_count: u32,
    pub debug_count: u32,
    pub other_count: u32,
    pub movement_operations: u32,
    pub scan_operations: u32,
    pub error_operations: u32,
    pub other_operations: u32,
    pub total_duration: f64,
    pub max_duration: f64,
    pub avg_movement_duration: f64,
    pub slowest_operation: Option<LogEntry>,
    pub robots_seen: Vec<String>,
}

impl LogAnalysis {
    pub fn new() -> Self {
        LogAnalysis {
            total_entries: 0,
            error_count: 0,
            warning_count: 0,
            info_count: 0,
            debug_count: 0,
            other_count: 0,
            movement_operations: 0,
            scan_operations: 0,
            error_operations: 0,
            other_operations: 0,
            total_duration: 0.0,
            max_duration: 0.0,
            avg_movement_duration: 0.0,
            slowest_operation: None,
            robots_seen: Vec::new(),
        }
    }

    pub fn print_summary(&self) {
        println!("=== LOG ANALYSIS SUMMARY ===");
        println!("Total entries: {}", self.total_entries);
        println!("Log levels - ERROR: {}, WARN: {}, INFO: {}, DEBUG: {}, OTHER: {}",
                 self.error_count, self.warning_count, self.info_count, self.debug_count, self.other_count);
        println!("Operations - MOVE: {}, SCAN: {}, ERROR: {}, OTHER: {}",
                 self.movement_operations, self.scan_operations, self.error_operations, self.other_operations);
        println!("Performance - Max duration: {:.2}s, Avg movement duration: {:.2}s",
                 self.max_duration, self.avg_movement_duration);
        println!("Robots seen: {}", self.robots_seen.join(", "));

        if let Some(ref slowest) = self.slowest_operation {
            println!("Slowest operation: {} {} took {:.2}s",
                     slowest.robot_id, slowest.operation, slowest.duration.unwrap_or(0.0));
        }
    }
}

// Comprehensive custom logging demonstration
pub fn comprehensive_custom_logging_demo() {
    initialize_custom_logging();

    info!("=== CUSTOM LOGGING AND FILE OUTPUT DEMONSTRATION ===");

    // Create file logger with rotation
    let logger = FileLogger::new("robot_operations", 1024); // Small size for demo

    // Log various robot operations
    log_robot_movement("ROBOT_001", (10.0, 5.0), (12.0, 7.0), 1.2, &logger);
    log_robot_movement("ROBOT_001", (12.0, 7.0), (15.0, 10.0), 0.8, &logger);
    log_robot_scan("ROBOT_001", "zone_b", 3, 5.0, &logger);
    log_robot_error("ROBOT_001", "obstacle_detected", (13.0, 7.0), true, &logger);

    log_robot_movement("ROBOT_002", (0.0, 0.0), (5.0, 5.0), 2.5, &logger);
    log_robot_scan("ROBOT_002", "zone_a", 0, 3.2, &logger);

    // Create some entries manually for testing
    let manual_entries = vec![
        LogEntry::new("ROBOT_003", "MOVE", "(20,20) -> (25,25)", "SUCCESS").with_duration(0.9),
        LogEntry::new("ROBOT_003", "SCAN", "area=zone_c items=7", "ITEMS_FOUND"),
        LogEntry::new("ROBOT_003", "ERROR", "sensor_malfunction", "RETRYING").with_level("ERROR"),
    ];

    for entry in &manual_entries {
        logger.log(entry).unwrap_or_else(|e| {
            error!("Failed to log entry: {}", e);
        });
    }

    // Close the logger
    logger.close().unwrap_or_else(|e| {
        error!("Failed to close logger: {}", e);
    });

    info!("Custom logging demonstration complete - check robot_operations.log files");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry::new("TEST_ROBOT", "MOVE", "test move", "SUCCESS");
        assert_eq!(entry.robot_id, "TEST_ROBOT");
        assert_eq!(entry.operation, "MOVE");
        assert_eq!(entry.details, "test move");
        assert_eq!(entry.status, "SUCCESS");
        assert_eq!(entry.level, "INFO");
        assert!(entry.duration.is_none());
    }

    #[test]
    fn test_log_entry_with_level_and_duration() {
        let entry = LogEntry::new("TEST_ROBOT", "SCAN", "area scan", "COMPLETE")
            .with_level("DEBUG")
            .with_duration(1.5);

        assert_eq!(entry.level, "DEBUG");
        assert_eq!(entry.duration, Some(1.5));
    }

    #[test]
    fn test_log_entry_custom_formatting() {
        let entry = LogEntry::new("R2D2", "MOVE", "(10,5) -> (12,7)", "SUCCESS")
            .with_duration(1.2);

        let formatted = entry.format_custom();
        assert!(formatted.contains("R2D2"));
        assert!(formatted.contains("MOVE"));
        assert!(formatted.contains("(10,5) -> (12,7)"));
        assert!(formatted.contains("SUCCESS"));
        assert!(formatted.contains("1.2s"));
    }

    #[test]
    fn test_log_entry_parsing() {
        let log_line = "[2023-10-18 14:30:22] ROBOT_001 MOVE (10,5) -> (12,7) -> SUCCESS INFO 1.2s";
        let parsed = LogEntry::parse_custom(log_line).unwrap();

        assert_eq!(parsed.robot_id, "ROBOT_001");
        assert_eq!(parsed.operation, "MOVE");
        assert_eq!(parsed.details, "(10,5) -> (12,7)");
        assert_eq!(parsed.status, "SUCCESS");
        assert_eq!(parsed.level, "INFO");
        assert_eq!(parsed.duration, Some(1.2));
    }

    #[test]
    fn test_log_entry_parsing_without_duration() {
        let log_line = "[2023-10-18 14:30:25] ROBOT_001 SCAN area=zone_b items=3 -> ITEMS_FOUND";
        let parsed = LogEntry::parse_custom(log_line).unwrap();

        assert_eq!(parsed.robot_id, "ROBOT_001");
        assert_eq!(parsed.operation, "SCAN");
        assert!(parsed.details.contains("area=zone_b"));
        assert_eq!(parsed.status, "ITEMS_FOUND");
        assert_eq!(parsed.level, "INFO");
        assert!(parsed.duration.is_none());
    }

    #[test]
    fn test_log_entry_parsing_error_cases() {
        // Missing separator
        let invalid1 = "[2023-10-18 14:30:22] ROBOT_001 MOVE test";
        assert!(LogEntry::parse_custom(invalid1).is_err());

        // Missing timestamp brackets
        let invalid2 = "2023-10-18 14:30:22 ROBOT_001 MOVE test -> SUCCESS";
        assert!(LogEntry::parse_custom(invalid2).is_err());

        // Insufficient parts
        let invalid3 = "[2023-10-18 14:30:22] ROBOT_001 -> SUCCESS";
        assert!(LogEntry::parse_custom(invalid3).is_err());
    }

    #[test]
    fn test_file_logger_creation() {
        let logger = FileLogger::new("test_log", 1024);
        assert_eq!(logger.base_filename, "test_log");
        assert_eq!(logger.max_file_size, 1024);
    }

    #[test]
    fn test_file_logger_logging() {
        let logger = FileLogger::new("test_robot_log", 1024);
        let entry = LogEntry::new("TEST_ROBOT", "TEST_OP", "test details", "SUCCESS");

        // This test just ensures the method doesn't panic
        let result = logger.log(&entry);
        assert!(result.is_ok());

        // Clean up
        let _ = logger.close();
        let _ = std::fs::remove_file("test_robot_log.log");
    }

    #[test]
    fn test_timestamp_formatting() {
        let timestamp = format_timestamp();
        assert!(timestamp.starts_with("2023-10-18"));
        assert!(timestamp.contains("14:"));
    }

    #[test]
    fn test_initialize_custom_logging() {
        initialize_custom_logging();
        assert!(env::var("RUST_LOG").is_ok());
    }

    #[test]
    fn test_log_analysis_creation() {
        let analysis = LogAnalysis::new();
        assert_eq!(analysis.total_entries, 0);
        assert_eq!(analysis.error_count, 0);
        assert_eq!(analysis.max_duration, 0.0);
        assert!(analysis.robots_seen.is_empty());
    }

    #[test]
    fn test_analyze_log_entries() {
        let entries = vec![
            LogEntry::new("R2D2", "MOVE", "test move", "SUCCESS").with_duration(1.0),
            LogEntry::new("R2D2", "SCAN", "test scan", "COMPLETE"),
            LogEntry::new("C3PO", "ERROR", "test error", "FAILED").with_level("ERROR"),
            LogEntry::new("R2D2", "MOVE", "another move", "SUCCESS").with_duration(2.0),
        ];

        let analysis = analyze_log_entries(&entries);

        assert_eq!(analysis.total_entries, 4);
        assert_eq!(analysis.error_count, 1);
        assert_eq!(analysis.info_count, 3);
        assert_eq!(analysis.movement_operations, 2);
        assert_eq!(analysis.scan_operations, 1);
        assert_eq!(analysis.error_operations, 1);
        assert_eq!(analysis.max_duration, 2.0);
        assert_eq!(analysis.robots_seen.len(), 2);
        assert!(analysis.robots_seen.contains(&"R2D2".to_string()));
        assert!(analysis.robots_seen.contains(&"C3PO".to_string()));
    }

    #[test]
    fn test_comprehensive_custom_logging_demo() {
        // This test just ensures the demo function runs without panicking
        comprehensive_custom_logging_demo();

        // Clean up any created log files
        let _ = std::fs::remove_file("robot_operations.log");
        let _ = std::fs::remove_file("robot_operations.1.log");
        let _ = std::fs::remove_file("robot_operations.2.log");
    }

    #[test]
    fn test_parse_log_file_nonexistent() {
        let result = parse_log_file("nonexistent_file.log");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_log_analysis_print_summary() {
        let mut analysis = LogAnalysis::new();
        analysis.total_entries = 10;
        analysis.error_count = 2;
        analysis.movement_operations = 5;
        analysis.robots_seen.push("R2D2".to_string());

        // This should not panic
        analysis.print_summary();
    }

    #[test]
    fn test_round_trip_parse_format() {
        let original = LogEntry::new("TEST_ROBOT", "MOVE", "(0,0) -> (10,10)", "SUCCESS")
            .with_level("DEBUG")
            .with_duration(1.5);

        let formatted = original.format_custom();
        let parsed = LogEntry::parse_custom(&formatted).unwrap();

        assert_eq!(original.robot_id, parsed.robot_id);
        assert_eq!(original.operation, parsed.operation);
        assert_eq!(original.details, parsed.details);
        assert_eq!(original.status, parsed.status);
        assert_eq!(original.level, parsed.level);
        assert_eq!(original.duration, parsed.duration);
    }
}