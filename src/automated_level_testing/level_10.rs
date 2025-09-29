// Level 10: Level 10: Error Handling - Automated Test Solutions
// Updated to match actual learning tests

use super::level_1::{LevelTestConfig, TaskTest};

pub fn get_level_10_tests() -> LevelTestConfig {
    LevelTestConfig {
        level_name: "Level 10: Error Handling",
        level_index: 9,
        tasks: vec![
            TaskTest {
                task_number: 1,
                task_name: "Result Basics",
                solution_code: r#"fn parse_sensor_data(input: &str) -> Result<i32, String> {
    match input.parse::<i32>() {
        Ok(value) => Ok(value),
        Err(_) => Err(format!("Failed to parse '{}' as number", input)),
    }
}

fn main() {
    let sensor_inputs = vec!["42", "invalid", "100", "not_a_number", "75"];

    println!("Processing {} sensor readings", sensor_inputs.len());

    for input in sensor_inputs {
        match parse_sensor_data(input) {
            Ok(value) => println!("Successfully parsed: {}", value),
            Err(error) => println!("Error: {}", error),
        }
    }

    let valid_reading = parse_sensor_data("42");
    if let Ok(value) = valid_reading {
        println!("Valid sensor reading: {}", value);
    }

    let invalid_reading = parse_sensor_data("bad_data");
    if let Err(error) = invalid_reading {
        println!("Sensor error occurred: {}", error);
    }
}"#,
                completion_indicators: vec![
                    "Processing 5 sensor readings", "Successfully parsed: 42", "Error: Failed to parse 'invalid' as number", "Successfully parsed: 100", "Valid sensor reading: 42", "Sensor error occurred:"
                ],
            },
            TaskTest {
                task_number: 2,
                task_name: "Custom Error Types",
                solution_code: r#"#[derive(Debug)]
enum RobotError {
    LowBattery,
    InvalidCommand,
    SensorFailure(String),
    NavigationError,
}

fn execute_robot_command(command: &str, battery_level: u32) -> Result<String, RobotError> {
    if battery_level < 10 {
        return Err(RobotError::LowBattery);
    }

    match command {
        "move_forward" => Ok("Robot moved forward".to_string()),
        "turn_left" => Ok("Robot turned left".to_string()),
        "turn_right" => Ok("Robot turned right".to_string()),
        "scan" => {
            if battery_level < 20 {
                Err(RobotError::SensorFailure("Insufficient power for scan".to_string()))
            } else {
                Ok("Scan complete: area clear".to_string())
            }
        },
        _ => Err(RobotError::InvalidCommand),
    }
}

fn main() {
    let commands = vec![
        ("move_forward", 50),
        ("scan", 15),
        ("invalid_cmd", 100),
        ("turn_left", 5),
        ("scan", 25),
    ];

    println!("Robot command execution system initialized");

    for (cmd, battery) in commands {
        println!("Executing '{}' with {}% battery", cmd, battery);

        match execute_robot_command(cmd, battery) {
            Ok(result) => println!("Success: {}", result),
            Err(RobotError::LowBattery) => println!("Error: Battery too low for operation"),
            Err(RobotError::InvalidCommand) => println!("Error: Unknown command '{}'", cmd),
            Err(RobotError::SensorFailure(msg)) => println!("Sensor Error: {}", msg),
            Err(RobotError::NavigationError) => println!("Error: Navigation system failure"),
        }
        println!();
    }
}"#,
                completion_indicators: vec![
                    "Robot command execution system initialized", "Executing 'move_forward' with 50% battery", "Success: Robot moved forward", "Executing 'scan' with 15% battery", "Sensor Error: Insufficient power for scan", "Error: Unknown command 'invalid_cmd'", "Error: Battery too low for operation"
                ],
            },
            TaskTest {
                task_number: 3,
                task_name: "Error Propagation",
                solution_code: r#"#[derive(Debug)]
enum SystemError {
    NetworkError(String),
    DatabaseError(String),
    AuthenticationError,
    ConfigurationError,
}

fn connect_to_network() -> Result<String, SystemError> {
    // Simulate network connection
    Ok("Network connected successfully".to_string())
}

fn authenticate_user(username: &str) -> Result<String, SystemError> {
    if username == "admin" {
        Ok(format!("User '{}' authenticated", username))
    } else {
        Err(SystemError::AuthenticationError)
    }
}

fn load_configuration() -> Result<String, SystemError> {
    Ok("Configuration loaded successfully".to_string())
}

fn initialize_robot_system(username: &str) -> Result<String, SystemError> {
    let _network = connect_to_network()?;
    let _auth = authenticate_user(username)?;
    let _config = load_configuration()?;

    Ok("Robot system fully initialized".to_string())
}

fn main() {
    println!("Starting robot system initialization");

    let test_users = vec!["admin", "guest", "operator"];

    for user in test_users {
        println!("Attempting initialization for user: {}", user);

        match initialize_robot_system(user) {
            Ok(message) => println!("✓ {}", message),
            Err(SystemError::NetworkError(msg)) => println!("✗ Network Error: {}", msg),
            Err(SystemError::DatabaseError(msg)) => println!("✗ Database Error: {}", msg),
            Err(SystemError::AuthenticationError) => println!("✗ Authentication failed for user '{}'", user),
            Err(SystemError::ConfigurationError) => println!("✗ Configuration Error"),
        }
        println!();
    }

    println!("System initialization attempts completed");
}"#,
                completion_indicators: vec![
                    "Starting robot system initialization", "Attempting initialization for user: admin", "✓ Robot system fully initialized", "Attempting initialization for user: guest", "✗ Authentication failed for user 'guest'", "System initialization attempts completed"
                ],
            },
            TaskTest {
                task_number: 4,
                task_name: "Error Recovery",
                solution_code: r#"use std::collections::HashMap;

#[derive(Debug)]
enum DataError {
    NotFound,
    CorruptedData,
    AccessDenied,
}

struct DataStore {
    data: HashMap<String, String>,
}

impl DataStore {
    fn new() -> Self {
        let mut data = HashMap::new();
        data.insert("sensor_1".to_string(), "temperature:22".to_string());
        data.insert("sensor_2".to_string(), "corrupted_data_here".to_string());
        data.insert("sensor_3".to_string(), "humidity:65".to_string());

        DataStore { data }
    }

    fn get_sensor_data(&self, sensor_id: &str) -> Result<String, DataError> {
        match self.data.get(sensor_id) {
            Some(data) => {
                if data.contains("corrupted") {
                    Err(DataError::CorruptedData)
                } else {
                    Ok(data.clone())
                }
            },
            None => Err(DataError::NotFound),
        }
    }
}

fn get_sensor_data_with_fallback(store: &DataStore, primary_sensor: &str, backup_sensor: &str) -> String {
    match store.get_sensor_data(primary_sensor) {
        Ok(data) => {
            println!("Primary sensor data retrieved: {}", data);
            data
        },
        Err(error) => {
            println!("Primary sensor error: {:?}", error);
            println!("Attempting backup sensor...");

            match store.get_sensor_data(backup_sensor) {
                Ok(backup_data) => {
                    println!("Backup sensor data retrieved: {}", backup_data);
                    backup_data
                },
                Err(backup_error) => {
                    println!("Backup sensor also failed: {:?}", backup_error);
                    "default:unknown".to_string()
                }
            }
        }
    }
}

fn main() {
    let data_store = DataStore::new();

    println!("Data recovery system initialized");
    println!("Available sensors: sensor_1, sensor_2, sensor_3");
    println!();

    let test_cases = vec![
        ("sensor_1", "sensor_3"),
        ("sensor_2", "sensor_1"),
        ("sensor_4", "sensor_2"),
        ("sensor_5", "sensor_6"),
    ];

    for (primary, backup) in test_cases {
        println!("Testing primary: {}, backup: {}", primary, backup);
        let result = get_sensor_data_with_fallback(&data_store, primary, backup);
        println!("Final result: {}", result);
        println!("---");
    }
}"#,
                completion_indicators: vec![
                    "Data recovery system initialized", "Available sensors: sensor_1, sensor_2, sensor_3", "Testing primary: sensor_1, backup: sensor_3", "Primary sensor data retrieved: temperature:22", "Testing primary: sensor_2, backup: sensor_1", "Primary sensor error: CorruptedData", "Attempting backup sensor...", "Backup sensor data retrieved: temperature:22"
                ],
            },
            TaskTest {
                task_number: 5,
                task_name: "Fault-Tolerant Systems",
                solution_code: r#"use std::collections::HashMap;

#[derive(Debug)]
enum SystemError {
    CriticalFailure(String),
    RecoverableError(String),
    WarningCondition(String),
}

struct HealthMonitor {
    component_status: HashMap<String, bool>,
    error_count: u32,
    recovery_attempts: u32,
}

impl HealthMonitor {
    fn new() -> Self {
        let mut status = HashMap::new();
        status.insert("navigation".to_string(), true);
        status.insert("sensors".to_string(), true);
        status.insert("communication".to_string(), true);
        status.insert("power".to_string(), true);

        HealthMonitor {
            component_status: status,
            error_count: 0,
            recovery_attempts: 0,
        }
    }

    fn simulate_component_failure(&mut self, component: &str) -> Result<(), SystemError> {
        if let Some(status) = self.component_status.get_mut(component) {
            *status = false;
            self.error_count += 1;

            match component {
                "power" => Err(SystemError::CriticalFailure(format!("Power system failure - immediate shutdown required"))),
                "navigation" => Err(SystemError::RecoverableError(format!("Navigation system offline - switching to manual mode"))),
                _ => Err(SystemError::WarningCondition(format!("Component '{}' experiencing issues", component))),
            }
        } else {
            Err(SystemError::WarningCondition("Unknown component".to_string()))
        }
    }

    fn attempt_recovery(&mut self, component: &str) -> Result<String, SystemError> {
        self.recovery_attempts += 1;

        if self.recovery_attempts > 3 {
            return Err(SystemError::CriticalFailure("Maximum recovery attempts exceeded".to_string()));
        }

        if let Some(status) = self.component_status.get_mut(component) {
            *status = true;
            Ok(format!("Component '{}' recovered successfully", component))
        } else {
            Err(SystemError::RecoverableError("Component not found".to_string()))
        }
    }

    fn system_status(&self) -> String {
        let active_components = self.component_status.values().filter(|&&status| status).count();
        let total_components = self.component_status.len();

        format!("System Status: {}/{} components operational, {} errors, {} recovery attempts",
                active_components, total_components, self.error_count, self.recovery_attempts)
    }
}

fn main() {
    let mut monitor = HealthMonitor::new();

    println!("Fault-tolerant robot system initialized");
    println!("{}", monitor.system_status());
    println!();

    let failure_scenarios = vec![
        "sensors",
        "navigation",
        "communication",
        "power",
    ];

    for component in failure_scenarios {
        println!("=== Simulating {} failure ===", component);

        match monitor.simulate_component_failure(component) {
            Ok(_) => println!("Component operating normally"),
            Err(SystemError::CriticalFailure(msg)) => {
                println!("CRITICAL: {}", msg);
                println!("System shutdown initiated");
                break;
            },
            Err(SystemError::RecoverableError(msg)) => {
                println!("RECOVERABLE: {}", msg);
                println!("Attempting recovery...");

                match monitor.attempt_recovery(component) {
                    Ok(recovery_msg) => println!("SUCCESS: {}", recovery_msg),
                    Err(recovery_error) => println!("RECOVERY FAILED: {:?}", recovery_error),
                }
            },
            Err(SystemError::WarningCondition(msg)) => {
                println!("WARNING: {}", msg);
            }
        }

        println!("{}", monitor.system_status());
        println!();
    }
}"#,
                completion_indicators: vec![
                    "Fault-tolerant robot system initialized", "System Status: 4/4 components operational, 0 errors, 0 recovery attempts", "=== Simulating sensors failure ===", "WARNING: Component 'sensors' experiencing issues", "=== Simulating navigation failure ===", "RECOVERABLE: Navigation system offline - switching to manual mode", "Attempting recovery...", "SUCCESS: Component 'navigation' recovered successfully", "=== Simulating power failure ===", "CRITICAL: Power system failure - immediate shutdown required", "System shutdown initiated"
                ],
            }
        ],
    }
}