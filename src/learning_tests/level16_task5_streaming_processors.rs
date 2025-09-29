// Learning Tests for Level 16, Task 5: Performance-Optimized Streaming Data Processor
// Implementing efficient streaming data processing with custom deserializers

use serde::{Serialize, Deserialize};
use serde_json;
use std::fmt;
use std::error::Error;
use std::io::{BufRead, BufReader, Cursor};

// Data Processing Error types
#[derive(Debug)]
pub enum DataProcessingError {
    JsonError(serde_json::Error),
    ValidationError(String),
    CorruptedData(String),
    UnsupportedFormat(String),
    MigrationError(String),
    IoError(std::io::Error),
}

impl fmt::Display for DataProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataProcessingError::JsonError(e) => write!(f, "JSON processing error: {}", e),
            DataProcessingError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            DataProcessingError::CorruptedData(msg) => write!(f, "Data corruption detected: {}", msg),
            DataProcessingError::UnsupportedFormat(format) => write!(f, "Unsupported format: {}", format),
            DataProcessingError::MigrationError(msg) => write!(f, "Migration error: {}", msg),
            DataProcessingError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for DataProcessingError {}

impl From<serde_json::Error> for DataProcessingError {
    fn from(error: serde_json::Error) -> Self {
        DataProcessingError::JsonError(error)
    }
}

impl From<std::io::Error> for DataProcessingError {
    fn from(error: std::io::Error) -> Self {
        DataProcessingError::IoError(error)
    }
}

// Processed Record structure
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProcessedRecord {
    pub id: u32,
    pub name: String,
    pub position: (f64, f64),
}

// Extended record with more fields
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExtendedRecord {
    pub id: u32,
    pub name: String,
    pub position: (f64, f64),
    pub timestamp: Option<String>,
    pub status: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl From<ProcessedRecord> for ExtendedRecord {
    fn from(record: ProcessedRecord) -> Self {
        ExtendedRecord {
            id: record.id,
            name: record.name,
            position: record.position,
            timestamp: None,
            status: None,
            metadata: None,
        }
    }
}

// Streaming Data Processor
#[derive(Debug)]
pub struct StreamingDataProcessor {
    pub processed_count: usize,
    pub error_count: usize,
    pub last_error: Option<String>,
    pub successful_formats: Vec<String>,
    pub failed_formats: Vec<String>,
}

impl StreamingDataProcessor {
    pub fn new() -> Self {
        StreamingDataProcessor {
            processed_count: 0,
            error_count: 0,
            last_error: None,
            successful_formats: Vec::new(),
            failed_formats: Vec::new(),
        }
    }

    pub fn process_data_stream(&mut self, stream_data: &str) -> Result<Vec<ProcessedRecord>, DataProcessingError> {
        let mut results = Vec::new();

        for (line_num, line) in stream_data.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            match self.process_single_record(trimmed) {
                Ok(record) => {
                    results.push(record);
                    self.processed_count += 1;
                }
                Err(e) => {
                    self.error_count += 1;
                    self.last_error = Some(format!("Line {}: {}", line_num + 1, e));
                    println!("Skipping invalid record at line {}: {}", line_num + 1, e);
                }
            }
        }

        println!("Streaming complete: {} processed, {} errors",
                 self.processed_count, self.error_count);

        Ok(results)
    }

    pub fn process_single_record(&mut self, line: &str) -> Result<ProcessedRecord, DataProcessingError> {
        // Try JSON format first
        if let Ok(record) = serde_json::from_str::<ProcessedRecord>(line) {
            self.successful_formats.push("JSON".to_string());
            return Ok(record);
        }

        // Try extended JSON format and convert
        if let Ok(extended) = serde_json::from_str::<ExtendedRecord>(line) {
            self.successful_formats.push("Extended JSON".to_string());
            return Ok(ProcessedRecord {
                id: extended.id,
                name: extended.name,
                position: extended.position,
            });
        }

        // Try custom pipe-separated format
        if let Ok(record) = self.parse_custom_format(line) {
            self.successful_formats.push("Custom".to_string());
            return Ok(record);
        }

        // Try CSV format
        if let Ok(record) = self.parse_csv_format(line) {
            self.successful_formats.push("CSV".to_string());
            return Ok(record);
        }

        // Try key-value format
        if let Ok(record) = self.parse_keyvalue_format(line) {
            self.successful_formats.push("Key-Value".to_string());
            return Ok(record);
        }

        self.failed_formats.push(line.chars().take(20).collect::<String>());
        Err(DataProcessingError::UnsupportedFormat(
            format!("Unable to parse: {}", line)
        ))
    }

    fn parse_custom_format(&self, line: &str) -> Result<ProcessedRecord, DataProcessingError> {
        // Custom format: "ID:123|NAME:Robot|X:10.5|Y:20.3"
        let mut id = None;
        let mut name = None;
        let mut x = None;
        let mut y = None;

        for part in line.split('|') {
            if let Some((key, value)) = part.split_once(':') {
                match key.trim() {
                    "ID" => id = value.trim().parse().ok(),
                    "NAME" => name = Some(value.trim().to_string()),
                    "X" => x = value.trim().parse().ok(),
                    "Y" => y = value.trim().parse().ok(),
                    _ => {}
                }
            }
        }

        if let (Some(id), Some(name), Some(x), Some(y)) = (id, name, x, y) {
            Ok(ProcessedRecord { id, name, position: (x, y) })
        } else {
            Err(DataProcessingError::CorruptedData(
                "Missing required fields in custom format".to_string()
            ))
        }
    }

    fn parse_csv_format(&self, line: &str) -> Result<ProcessedRecord, DataProcessingError> {
        // CSV format: "123,Robot Name,10.5,20.3"
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 4 {
            return Err(DataProcessingError::CorruptedData(
                "CSV format requires exactly 4 fields".to_string()
            ));
        }

        let id = parts[0].trim().parse()
            .map_err(|_| DataProcessingError::CorruptedData("Invalid ID in CSV".to_string()))?;
        let name = parts[1].trim().to_string();
        let x = parts[2].trim().parse()
            .map_err(|_| DataProcessingError::CorruptedData("Invalid X coordinate in CSV".to_string()))?;
        let y = parts[3].trim().parse()
            .map_err(|_| DataProcessingError::CorruptedData("Invalid Y coordinate in CSV".to_string()))?;

        Ok(ProcessedRecord { id, name, position: (x, y) })
    }

    fn parse_keyvalue_format(&self, line: &str) -> Result<ProcessedRecord, DataProcessingError> {
        // Key-value format: "id=123 name="Robot Name" x=10.5 y=20.3"
        let mut id = None;
        let mut name = None;
        let mut x = None;
        let mut y = None;

        let mut current_key = String::new();
        let mut current_value = String::new();
        let mut in_quotes = false;
        let mut expecting_value = false;
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '=' if !in_quotes => {
                    if expecting_value {
                        return Err(DataProcessingError::CorruptedData(
                            "Unexpected '=' in key-value format".to_string()
                        ));
                    }
                    expecting_value = true;
                }
                '"' => {
                    in_quotes = !in_quotes;
                }
                ' ' if !in_quotes && expecting_value => {
                    // End of value
                    self.assign_keyvalue_field(&current_key.trim(), &current_value.trim(),
                                             &mut id, &mut name, &mut x, &mut y)?;
                    current_key.clear();
                    current_value.clear();
                    expecting_value = false;
                }
                ' ' if !in_quotes && !expecting_value => {
                    // Skip whitespace between key-value pairs
                }
                _ => {
                    if expecting_value {
                        current_value.push(ch);
                    } else {
                        current_key.push(ch);
                    }
                }
            }
        }

        // Handle the last key-value pair
        if expecting_value {
            self.assign_keyvalue_field(&current_key.trim(), &current_value.trim(),
                                     &mut id, &mut name, &mut x, &mut y)?;
        }

        if let (Some(id), Some(name), Some(x), Some(y)) = (id, name, x, y) {
            Ok(ProcessedRecord { id, name, position: (x, y) })
        } else {
            Err(DataProcessingError::CorruptedData(
                "Missing required fields in key-value format".to_string()
            ))
        }
    }

    fn assign_keyvalue_field(&self, key: &str, value: &str, id: &mut Option<u32>,
                           name: &mut Option<String>, x: &mut Option<f64>, y: &mut Option<f64>)
                           -> Result<(), DataProcessingError> {
        match key {
            "id" => *id = value.parse().ok(),
            "name" => *name = Some(value.trim_matches('"').to_string()),
            "x" => *x = value.parse().ok(),
            "y" => *y = value.parse().ok(),
            _ => {} // Ignore unknown keys
        }
        Ok(())
    }

    pub fn process_buffered_stream<R: BufRead>(&mut self, reader: R) -> Result<Vec<ProcessedRecord>, DataProcessingError> {
        let mut results = Vec::new();

        for (line_num, line_result) in reader.lines().enumerate() {
            let line = line_result?;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            match self.process_single_record(trimmed) {
                Ok(record) => {
                    results.push(record);
                    self.processed_count += 1;
                }
                Err(e) => {
                    self.error_count += 1;
                    self.last_error = Some(format!("Line {}: {}", line_num + 1, e));
                    eprintln!("Skipping invalid record at line {}: {}", line_num + 1, e);
                }
            }
        }

        Ok(results)
    }

    pub fn get_statistics(&self) -> ProcessingStatistics {
        ProcessingStatistics {
            total_processed: self.processed_count,
            total_errors: self.error_count,
            success_rate: if self.processed_count + self.error_count > 0 {
                (self.processed_count as f64) / ((self.processed_count + self.error_count) as f64) * 100.0
            } else {
                0.0
            },
            format_distribution: self.get_format_distribution(),
        }
    }

    fn get_format_distribution(&self) -> std::collections::HashMap<String, usize> {
        let mut distribution = std::collections::HashMap::new();
        for format in &self.successful_formats {
            *distribution.entry(format.clone()).or_insert(0) += 1;
        }
        distribution
    }

    pub fn reset(&mut self) {
        self.processed_count = 0;
        self.error_count = 0;
        self.last_error = None;
        self.successful_formats.clear();
        self.failed_formats.clear();
    }
}

#[derive(Debug)]
pub struct ProcessingStatistics {
    pub total_processed: usize,
    pub total_errors: usize,
    pub success_rate: f64,
    pub format_distribution: std::collections::HashMap<String, usize>,
}

// Main processing function
pub fn process_streaming_data_source() -> Result<(), DataProcessingError> {
    let stream_data = r#"
{"id": 1, "name": "Robot A", "position": [10.0, 20.0]}
{"id": 2, "name": "Robot B", "position": [15.0, 25.0]}
ID:3|NAME:Robot C|X:30.0|Y:40.0
{"id": 4, "name": "Robot D", "position": [50.0, 60.0]}
INVALID_LINE_HERE
ID:5|NAME:Robot E|X:70.0|Y:80.0
{"id": 6, "name": "Robot F", "position": [90.0, 100.0]}
7,Robot G,110.0,120.0
id=8 name="Robot H" x=130.0 y=140.0
{"id": 9, "name": "Robot I", "position": [150.0, 160.0], "timestamp": "2024-01-01T00:00:00Z", "status": "active"}
ID:10|NAME:Robot J with Special Characters!@#|X:170.0|Y:180.0
    "#;

    let mut processor = StreamingDataProcessor::new();
    let results = processor.process_data_stream(stream_data)?;

    println!("Successfully processed {} records:", results.len());
    for record in &results {
        println!("  {:?}", record);
    }

    if let Some(error) = &processor.last_error {
        println!("Last error encountered: {}", error);
    }

    let stats = processor.get_statistics();
    println!("\nProcessing Statistics:");
    println!("  Total processed: {}", stats.total_processed);
    println!("  Total errors: {}", stats.total_errors);
    println!("  Success rate: {:.2}%", stats.success_rate);
    println!("  Format distribution: {:?}", stats.format_distribution);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_format_processing() {
        let mut processor = StreamingDataProcessor::new();
        let json_line = r#"{"id": 123, "name": "Test Robot", "position": [10.0, 20.0]}"#;

        let result = processor.process_single_record(json_line).unwrap();
        assert_eq!(result.id, 123);
        assert_eq!(result.name, "Test Robot");
        assert_eq!(result.position, (10.0, 20.0));
    }

    #[test]
    fn test_custom_format_processing() {
        let mut processor = StreamingDataProcessor::new();
        let custom_line = "ID:456|NAME:Custom Robot|X:30.0|Y:40.0";

        let result = processor.process_single_record(custom_line).unwrap();
        assert_eq!(result.id, 456);
        assert_eq!(result.name, "Custom Robot");
        assert_eq!(result.position, (30.0, 40.0));
    }

    #[test]
    fn test_csv_format_processing() {
        let mut processor = StreamingDataProcessor::new();
        let csv_line = "789,CSV Robot,50.0,60.0";

        let result = processor.process_single_record(csv_line).unwrap();
        assert_eq!(result.id, 789);
        assert_eq!(result.name, "CSV Robot");
        assert_eq!(result.position, (50.0, 60.0));
    }

    #[test]
    fn test_keyvalue_format_processing() {
        let mut processor = StreamingDataProcessor::new();
        let kv_line = r#"id=101 name="Key Value Robot" x=70.0 y=80.0"#;

        let result = processor.process_single_record(kv_line).unwrap();
        assert_eq!(result.id, 101);
        assert_eq!(result.name, "Key Value Robot");
        assert_eq!(result.position, (70.0, 80.0));
    }

    #[test]
    fn test_extended_json_format() {
        let mut processor = StreamingDataProcessor::new();
        let extended_json = r#"{"id": 999, "name": "Extended Robot", "position": [1.0, 2.0], "timestamp": "2024-01-01", "status": "active", "metadata": {"type": "test"}}"#;

        let result = processor.process_single_record(extended_json).unwrap();
        assert_eq!(result.id, 999);
        assert_eq!(result.name, "Extended Robot");
        assert_eq!(result.position, (1.0, 2.0));
    }

    #[test]
    fn test_invalid_format_handling() {
        let mut processor = StreamingDataProcessor::new();
        let invalid_line = "This is not a valid format";

        let result = processor.process_single_record(invalid_line);
        assert!(result.is_err());
        match result.unwrap_err() {
            DataProcessingError::UnsupportedFormat(_) => {},
            _ => panic!("Expected UnsupportedFormat error"),
        }
    }

    #[test]
    fn test_stream_processing_with_mixed_formats() {
        let mut processor = StreamingDataProcessor::new();
        let mixed_data = r#"
{"id": 1, "name": "JSON Robot", "position": [1.0, 2.0]}
ID:2|NAME:Custom Robot|X:3.0|Y:4.0
3,CSV Robot,5.0,6.0
INVALID LINE
id=4 name="KV Robot" x=7.0 y=8.0
        "#;

        let results = processor.process_data_stream(mixed_data).unwrap();
        assert_eq!(results.len(), 4);
        assert_eq!(processor.processed_count, 4);
        assert_eq!(processor.error_count, 1);
    }

    #[test]
    fn test_custom_format_with_spaces_in_name() {
        let mut processor = StreamingDataProcessor::new();
        let custom_line = "ID:123|NAME:Robot With Spaces|X:10.0|Y:20.0";

        let result = processor.process_single_record(custom_line).unwrap();
        assert_eq!(result.name, "Robot With Spaces");
    }

    #[test]
    fn test_csv_format_invalid_field_count() {
        let mut processor = StreamingDataProcessor::new();
        let invalid_csv = "123,Robot Name,10.0"; // Missing Y coordinate

        let result = processor.process_single_record(invalid_csv);
        assert!(result.is_err());
    }

    #[test]
    fn test_keyvalue_format_with_quoted_name() {
        let mut processor = StreamingDataProcessor::new();
        let kv_line = r#"id=123 name="Robot with "quotes"" x=1.0 y=2.0"#;

        let result = processor.process_single_record(kv_line).unwrap();
        assert_eq!(result.name, r#"Robot with "quotes""#);
    }

    #[test]
    fn test_buffered_stream_processing() {
        let mut processor = StreamingDataProcessor::new();
        let data = "1,Robot A,1.0,2.0\n2,Robot B,3.0,4.0\n";
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);

        let results = processor.process_buffered_stream(reader).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "Robot A");
        assert_eq!(results[1].name, "Robot B");
    }

    #[test]
    fn test_processor_statistics() {
        let mut processor = StreamingDataProcessor::new();
        let data = r#"
{"id": 1, "name": "Robot 1", "position": [1.0, 2.0]}
ID:2|NAME:Robot 2|X:3.0|Y:4.0
INVALID
        "#;

        processor.process_data_stream(data).unwrap();
        let stats = processor.get_statistics();

        assert_eq!(stats.total_processed, 2);
        assert_eq!(stats.total_errors, 1);
        assert!((stats.success_rate - 66.67).abs() < 0.1);
    }

    #[test]
    fn test_processor_reset() {
        let mut processor = StreamingDataProcessor::new();
        processor.processed_count = 10;
        processor.error_count = 5;
        processor.last_error = Some("Test error".to_string());

        processor.reset();

        assert_eq!(processor.processed_count, 0);
        assert_eq!(processor.error_count, 0);
        assert!(processor.last_error.is_none());
        assert!(processor.successful_formats.is_empty());
    }

    #[test]
    fn test_process_streaming_data_source_function() {
        // This test runs the main processing function
        let result = process_streaming_data_source();
        assert!(result.is_ok());
    }

    #[test]
    fn test_custom_format_missing_fields() {
        let mut processor = StreamingDataProcessor::new();
        let incomplete_line = "ID:123|NAME:Robot"; // Missing X and Y

        let result = processor.process_single_record(incomplete_line);
        assert!(result.is_err());
        match result.unwrap_err() {
            DataProcessingError::CorruptedData(msg) => {
                assert!(msg.contains("Missing required fields"));
            }
            _ => panic!("Expected CorruptedData error"),
        }
    }

    #[test]
    fn test_csv_invalid_numeric_fields() {
        let mut processor = StreamingDataProcessor::new();
        let invalid_csv = "not_a_number,Robot,10.0,20.0";

        let result = processor.process_single_record(invalid_csv);
        assert!(result.is_err());
    }
}