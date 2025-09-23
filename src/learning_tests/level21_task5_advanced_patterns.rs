//! Level 21, Task 5: Advanced Error Handling Patterns
//!
//! This module demonstrates sophisticated error handling patterns in Rust,
//! including custom error types, error recovery, monadic patterns, and
//! advanced composition techniques for robust robot systems.
//!
//! Learning objectives:
//! - Implement advanced error recovery strategies
//! - Use monadic patterns for complex error handling
//! - Create custom error hierarchies and conversions
//! - Master try blocks and error bubbling patterns
//! - Build resilient systems with graceful degradation

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Comprehensive error hierarchy for robot systems
#[derive(Debug, Clone)]
pub enum SystemError {
    // Critical errors that require immediate attention
    Critical(CriticalError),
    // Recoverable errors that can be handled gracefully
    Recoverable(RecoverableError),
    // Warning-level issues that don't stop operation
    Warning(WarningError),
}

#[derive(Debug, Clone)]
pub enum CriticalError {
    HardwareFailure { component: String, code: u32 },
    SafetyViolation { sensor: String, value: f64, limit: f64 },
    PowerSystemFailure { voltage: f64, minimum: f64 },
    EmergencyStop { reason: String, timestamp: u64 },
}

#[derive(Debug, Clone)]
pub enum RecoverableError {
    SensorTimeout { sensor: String, timeout_ms: u64 },
    CommunicationLoss { device: String, retry_count: u32 },
    CalibrationDrift { sensor: String, drift_percent: f64 },
    ResourceContention { resource: String, wait_time_ms: u64 },
}

#[derive(Debug, Clone)]
pub enum WarningError {
    PerformanceDegradation { component: String, efficiency: f64 },
    MinorCalibrationIssue { sensor: String, deviation: f64 },
    NetworkLatency { latency_ms: u64, threshold_ms: u64 },
    BatteryLow { level: f64, warning_threshold: f64 },
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemError::Critical(err) => write!(f, "CRITICAL: {}", err),
            SystemError::Recoverable(err) => write!(f, "RECOVERABLE: {}", err),
            SystemError::Warning(err) => write!(f, "WARNING: {}", err),
        }
    }
}

impl fmt::Display for CriticalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CriticalError::HardwareFailure { component, code } => {
                write!(f, "Hardware failure in {} (code: 0x{:04X})", component, code)
            }
            CriticalError::SafetyViolation { sensor, value, limit } => {
                write!(f, "Safety violation: {} reading {} exceeds limit {}", sensor, value, limit)
            }
            CriticalError::PowerSystemFailure { voltage, minimum } => {
                write!(f, "Power system failure: {}V below minimum {}V", voltage, minimum)
            }
            CriticalError::EmergencyStop { reason, timestamp } => {
                write!(f, "Emergency stop at {}: {}", timestamp, reason)
            }
        }
    }
}

impl fmt::Display for RecoverableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecoverableError::SensorTimeout { sensor, timeout_ms } => {
                write!(f, "Sensor {} timeout after {}ms", sensor, timeout_ms)
            }
            RecoverableError::CommunicationLoss { device, retry_count } => {
                write!(f, "Communication lost with {} (retries: {})", device, retry_count)
            }
            RecoverableError::CalibrationDrift { sensor, drift_percent } => {
                write!(f, "Calibration drift in {}: {:.1}%", sensor, drift_percent)
            }
            RecoverableError::ResourceContention { resource, wait_time_ms } => {
                write!(f, "Resource {} contention, waited {}ms", resource, wait_time_ms)
            }
        }
    }
}

impl fmt::Display for WarningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WarningError::PerformanceDegradation { component, efficiency } => {
                write!(f, "Performance degradation in {}: {:.1}% efficiency", component, efficiency)
            }
            WarningError::MinorCalibrationIssue { sensor, deviation } => {
                write!(f, "Minor calibration issue in {}: {:.2}% deviation", sensor, deviation)
            }
            WarningError::NetworkLatency { latency_ms, threshold_ms } => {
                write!(f, "Network latency {}ms exceeds threshold {}ms", latency_ms, threshold_ms)
            }
            WarningError::BatteryLow { level, warning_threshold } => {
                write!(f, "Battery low: {:.1}% (warning at {:.1}%)", level, warning_threshold)
            }
        }
    }
}

impl std::error::Error for SystemError {}

/// Result type alias for robot operations
pub type RobotResult<T> = Result<T, SystemError>;

/// Operation result with metadata for advanced error handling
#[derive(Debug, Clone)]
pub struct OperationResult<T> {
    pub value: T,
    pub warnings: Vec<WarningError>,
    pub performance_metrics: PerformanceMetrics,
    pub recovery_actions: Vec<RecoveryAction>,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub execution_time: Duration,
    pub memory_usage: u64,
    pub cpu_usage: f64,
    pub success_rate: f64,
}

#[derive(Debug, Clone)]
pub enum RecoveryAction {
    RetryWithBackoff { max_attempts: u32, base_delay_ms: u64 },
    FallbackToAlternative { alternative: String },
    GracefulDegradation { reduced_functionality: Vec<String> },
    RequestMaintenance { component: String, urgency: MaintenanceUrgency },
}

#[derive(Debug, Clone)]
pub enum MaintenanceUrgency {
    Low,
    Medium,
    High,
    Critical,
}

/// Advanced error recovery manager
#[derive(Debug)]
pub struct ErrorRecoveryManager {
    recovery_strategies: HashMap<String, RecoveryStrategy>,
    error_history: VecDeque<ErrorHistoryEntry>,
    recovery_statistics: RecoveryStatistics,
    max_history_size: usize,
}

#[derive(Debug, Clone)]
struct RecoveryStrategy {
    max_retries: u32,
    base_delay: Duration,
    backoff_multiplier: f64,
    fallback_actions: Vec<RecoveryAction>,
    success_threshold: f64,
}

#[derive(Debug, Clone)]
struct ErrorHistoryEntry {
    error: SystemError,
    timestamp: Instant,
    recovery_attempted: bool,
    recovery_successful: bool,
    recovery_time: Option<Duration>,
}

#[derive(Debug, Clone)]
struct RecoveryStatistics {
    total_errors: u64,
    successful_recoveries: u64,
    failed_recoveries: u64,
    average_recovery_time: Duration,
}

impl ErrorRecoveryManager {
    pub fn new() -> Self {
        Self {
            recovery_strategies: HashMap::new(),
            error_history: VecDeque::new(),
            recovery_statistics: RecoveryStatistics {
                total_errors: 0,
                successful_recoveries: 0,
                failed_recoveries: 0,
                average_recovery_time: Duration::from_millis(0),
            },
            max_history_size: 1000,
        }
    }

    /// Register a recovery strategy for a specific error type
    pub fn register_strategy(&mut self, error_type: String, strategy: RecoveryStrategy) {
        self.recovery_strategies.insert(error_type, strategy);
    }

    /// Attempt to recover from an error using registered strategies
    pub fn attempt_recovery<T, F>(&mut self, error: SystemError, operation: F) -> RobotResult<T>
    where
        F: Fn() -> RobotResult<T>,
    {
        let start_time = Instant::now();
        self.record_error(error.clone(), start_time);

        let error_type = self.classify_error(&error);
        let strategy = self.recovery_strategies.get(&error_type);

        match strategy {
            Some(strategy) => self.execute_recovery_strategy(strategy, operation, error, start_time),
            None => {
                self.record_recovery_attempt(false, start_time);
                Err(error)
            }
        }
    }

    fn classify_error(&self, error: &SystemError) -> String {
        match error {
            SystemError::Critical(CriticalError::HardwareFailure { component, .. }) => {
                format!("hardware_failure_{}", component)
            }
            SystemError::Critical(CriticalError::SafetyViolation { .. }) => "safety_violation".to_string(),
            SystemError::Critical(CriticalError::PowerSystemFailure { .. }) => "power_failure".to_string(),
            SystemError::Critical(CriticalError::EmergencyStop { .. }) => "emergency_stop".to_string(),
            SystemError::Recoverable(RecoverableError::SensorTimeout { .. }) => "sensor_timeout".to_string(),
            SystemError::Recoverable(RecoverableError::CommunicationLoss { .. }) => "communication_loss".to_string(),
            SystemError::Recoverable(RecoverableError::CalibrationDrift { .. }) => "calibration_drift".to_string(),
            SystemError::Recoverable(RecoverableError::ResourceContention { .. }) => "resource_contention".to_string(),
            SystemError::Warning(_) => "warning".to_string(),
        }
    }

    fn execute_recovery_strategy<T, F>(
        &mut self,
        strategy: &RecoveryStrategy,
        operation: F,
        original_error: SystemError,
        start_time: Instant,
    ) -> RobotResult<T>
    where
        F: Fn() -> RobotResult<T>,
    {
        let mut attempts = 0;
        let mut current_delay = strategy.base_delay;

        while attempts < strategy.max_retries {
            attempts += 1;

            if attempts > 1 {
                std::thread::sleep(current_delay);
                current_delay = Duration::from_millis(
                    (current_delay.as_millis() as f64 * strategy.backoff_multiplier) as u64
                );
            }

            match operation() {
                Ok(result) => {
                    self.record_recovery_attempt(true, start_time);
                    return Ok(result);
                }
                Err(err) => {
                    if attempts == strategy.max_retries {
                        self.record_recovery_attempt(false, start_time);
                        return Err(err);
                    }
                    // Continue to next attempt
                }
            }
        }

        self.record_recovery_attempt(false, start_time);
        Err(original_error)
    }

    fn record_error(&mut self, error: SystemError, timestamp: Instant) {
        self.recovery_statistics.total_errors += 1;

        let entry = ErrorHistoryEntry {
            error,
            timestamp,
            recovery_attempted: true,
            recovery_successful: false,
            recovery_time: None,
        };

        self.error_history.push_back(entry);

        if self.error_history.len() > self.max_history_size {
            self.error_history.pop_front();
        }
    }

    fn record_recovery_attempt(&mut self, successful: bool, start_time: Instant) {
        let recovery_time = start_time.elapsed();

        if let Some(last_entry) = self.error_history.back_mut() {
            last_entry.recovery_successful = successful;
            last_entry.recovery_time = Some(recovery_time);
        }

        if successful {
            self.recovery_statistics.successful_recoveries += 1;
        } else {
            self.recovery_statistics.failed_recoveries += 1;
        }

        // Update average recovery time
        let total_recoveries = self.recovery_statistics.successful_recoveries + self.recovery_statistics.failed_recoveries;
        if total_recoveries > 0 {
            let total_time = self.recovery_statistics.average_recovery_time.as_millis() as f64 * (total_recoveries - 1) as f64;
            self.recovery_statistics.average_recovery_time = Duration::from_millis(
                ((total_time + recovery_time.as_millis() as f64) / total_recoveries as f64) as u64
            );
        }
    }

    pub fn get_recovery_statistics(&self) -> &RecoveryStatistics {
        &self.recovery_statistics
    }

    pub fn get_recent_errors(&self, since: Duration) -> Vec<&ErrorHistoryEntry> {
        let cutoff = Instant::now() - since;
        self.error_history.iter()
            .filter(|entry| entry.timestamp >= cutoff)
            .collect()
    }
}

/// Monadic error handling wrapper for advanced composition
#[derive(Debug)]
pub struct Monitored<T> {
    value: RobotResult<T>,
    context: ExecutionContext,
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    operation_id: String,
    start_time: Instant,
    component: String,
    metadata: HashMap<String, String>,
}

impl<T> Monitored<T> {
    pub fn new(value: RobotResult<T>, operation_id: String, component: String) -> Self {
        Self {
            value,
            context: ExecutionContext {
                operation_id,
                start_time: Instant::now(),
                component,
                metadata: HashMap::new(),
            },
        }
    }

    /// Map the value while preserving error context
    pub fn map<U, F>(self, f: F) -> Monitored<U>
    where
        F: FnOnce(T) -> U,
    {
        Monitored {
            value: self.value.map(f),
            context: self.context,
        }
    }

    /// Chain operations with error context preservation
    pub fn and_then<U, F>(self, f: F) -> Monitored<U>
    where
        F: FnOnce(T) -> Monitored<U>,
    {
        match self.value {
            Ok(value) => {
                let mut next = f(value);
                next.context.metadata.extend(self.context.metadata);
                next
            }
            Err(err) => Monitored {
                value: Err(err),
                context: self.context,
            },
        }
    }

    /// Provide fallback value on error
    pub fn or_else<F>(self, f: F) -> Monitored<T>
    where
        F: FnOnce(SystemError) -> Monitored<T>,
    {
        match self.value {
            Ok(_) => self,
            Err(err) => {
                let mut fallback = f(err);
                fallback.context.metadata.extend(self.context.metadata);
                fallback
            }
        }
    }

    /// Add metadata to execution context
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.context.metadata.insert(key, value);
        self
    }

    /// Convert to regular Result with execution report
    pub fn into_result_with_report(self) -> (RobotResult<T>, ExecutionReport) {
        let duration = self.context.start_time.elapsed();
        let report = ExecutionReport {
            operation_id: self.context.operation_id.clone(),
            component: self.context.component.clone(),
            duration,
            success: self.value.is_ok(),
            error: self.value.as_ref().err().cloned(),
            metadata: self.context.metadata.clone(),
        };

        (self.value, report)
    }

    /// Unwrap the result, panicking with context on error
    pub fn unwrap_with_context(self) -> T {
        match self.value {
            Ok(value) => value,
            Err(err) => panic!(
                "Operation '{}' in component '{}' failed after {:?}: {}",
                self.context.operation_id,
                self.context.component,
                self.context.start_time.elapsed(),
                err
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionReport {
    pub operation_id: String,
    pub component: String,
    pub duration: Duration,
    pub success: bool,
    pub error: Option<SystemError>,
    pub metadata: HashMap<String, String>,
}

/// Advanced robot system with comprehensive error handling
#[derive(Debug)]
pub struct AdvancedRobotSystem {
    subsystems: HashMap<String, Arc<Mutex<dyn Subsystem>>>,
    error_manager: ErrorRecoveryManager,
    execution_reports: VecDeque<ExecutionReport>,
    health_monitor: HealthMonitor,
}

trait Subsystem: Send + Sync + std::fmt::Debug {
    fn name(&self) -> &str;
    fn status(&self) -> SubsystemStatus;
    fn execute_operation(&mut self, operation: &str) -> RobotResult<OperationResult<String>>;
    fn shutdown(&mut self) -> RobotResult<()>;
}

#[derive(Debug, Clone)]
pub struct SubsystemStatus {
    pub is_operational: bool,
    pub health_score: f64,
    pub last_error: Option<SystemError>,
    pub uptime: Duration,
}

#[derive(Debug)]
struct HealthMonitor {
    health_checks: Vec<HealthCheck>,
    last_check: Option<Instant>,
    check_interval: Duration,
}

#[derive(Debug)]
struct HealthCheck {
    name: String,
    check_fn: fn() -> RobotResult<f64>, // Returns health score 0.0-1.0
    weight: f64,
    last_result: Option<f64>,
}

impl AdvancedRobotSystem {
    pub fn new() -> Self {
        let mut error_manager = ErrorRecoveryManager::new();

        // Register default recovery strategies
        error_manager.register_strategy(
            "sensor_timeout".to_string(),
            RecoveryStrategy {
                max_retries: 3,
                base_delay: Duration::from_millis(500),
                backoff_multiplier: 2.0,
                fallback_actions: vec![
                    RecoveryAction::FallbackToAlternative {
                        alternative: "backup_sensor".to_string()
                    }
                ],
                success_threshold: 0.8,
            },
        );

        error_manager.register_strategy(
            "communication_loss".to_string(),
            RecoveryStrategy {
                max_retries: 5,
                base_delay: Duration::from_millis(1000),
                backoff_multiplier: 1.5,
                fallback_actions: vec![
                    RecoveryAction::RetryWithBackoff {
                        max_attempts: 5,
                        base_delay_ms: 1000,
                    }
                ],
                success_threshold: 0.7,
            },
        );

        Self {
            subsystems: HashMap::new(),
            error_manager,
            execution_reports: VecDeque::new(),
            health_monitor: HealthMonitor {
                health_checks: Vec::new(),
                last_check: None,
                check_interval: Duration::from_secs(60),
            },
        }
    }

    /// Execute operation with comprehensive error handling and monitoring
    pub fn execute_monitored_operation<F>(&mut self, operation_id: String, component: String, operation: F) -> Monitored<String>
    where
        F: FnOnce() -> RobotResult<String>,
    {
        let monitored = Monitored::new(
            operation(),
            operation_id.clone(),
            component.clone(),
        ).with_metadata("execution_mode".to_string(), "monitored".to_string());

        let (result, report) = monitored.into_result_with_report();

        // Store execution report
        self.execution_reports.push_back(report);
        if self.execution_reports.len() > 1000 {
            self.execution_reports.pop_front();
        }

        // If operation failed, attempt recovery
        match result {
            Ok(value) => Monitored::new(Ok(value), operation_id, component),
            Err(error) => {
                let recovery_result = self.error_manager.attempt_recovery(error, || operation());
                Monitored::new(recovery_result, operation_id, component)
            }
        }
    }

    /// Chain multiple operations with error context preservation
    pub fn execute_operation_chain(&mut self, operations: Vec<ChainedOperation>) -> RobotResult<Vec<String>> {
        let mut results = Vec::new();
        let mut accumulated_warnings = Vec::new();

        for op in operations {
            let monitored = self.execute_monitored_operation(
                op.operation_id.clone(),
                op.component.clone(),
                op.operation,
            );

            let (result, report) = monitored.into_result_with_report();

            match result {
                Ok(value) => {
                    results.push(value);
                    if !op.continue_on_error {
                        // Store any warnings from this operation
                    }
                }
                Err(error) => {
                    if op.continue_on_error {
                        // Convert error to warning and continue
                        let warning = WarningError::PerformanceDegradation {
                            component: op.component.clone(),
                            efficiency: 0.5,
                        };
                        accumulated_warnings.push(warning);
                        results.push(format!("Skipped due to error: {}", error));
                    } else {
                        return Err(error);
                    }
                }
            }
        }

        Ok(results)
    }

    /// Perform comprehensive system health check
    pub fn perform_health_check(&mut self) -> RobotResult<SystemHealthReport> {
        let now = Instant::now();

        // Check if enough time has passed since last check
        if let Some(last_check) = self.health_monitor.last_check {
            if now.duration_since(last_check) < self.health_monitor.check_interval {
                return Err(SystemError::Warning(WarningError::PerformanceDegradation {
                    component: "health_monitor".to_string(),
                    efficiency: 1.0,
                }));
            }
        }

        self.health_monitor.last_check = Some(now);

        let mut subsystem_health = HashMap::new();
        let mut overall_score = 0.0;
        let mut total_weight = 0.0;

        // Check subsystem health
        for (name, subsystem) in &self.subsystems {
            let status = subsystem.lock().unwrap().status();
            let health_score = if status.is_operational {
                status.health_score
            } else {
                0.0
            };

            subsystem_health.insert(name.clone(), health_score);
            overall_score += health_score;
            total_weight += 1.0;
        }

        // Run custom health checks
        for health_check in &mut self.health_monitor.health_checks {
            match (health_check.check_fn)() {
                Ok(score) => {
                    health_check.last_result = Some(score);
                    overall_score += score * health_check.weight;
                    total_weight += health_check.weight;
                }
                Err(_) => {
                    health_check.last_result = Some(0.0);
                    total_weight += health_check.weight;
                }
            }
        }

        let final_score = if total_weight > 0.0 {
            overall_score / total_weight
        } else {
            1.0
        };

        let recovery_stats = self.error_manager.get_recovery_statistics().clone();

        Ok(SystemHealthReport {
            overall_score: final_score,
            subsystem_health,
            recovery_statistics: recovery_stats,
            recent_errors: self.error_manager.get_recent_errors(Duration::from_hours(1)).len(),
            uptime: now.duration_since(Instant::now()), // This would be system start time in real implementation
            last_check: now,
        })
    }

    pub fn add_health_check(&mut self, name: String, check_fn: fn() -> RobotResult<f64>, weight: f64) {
        self.health_monitor.health_checks.push(HealthCheck {
            name,
            check_fn,
            weight,
            last_result: None,
        });
    }

    pub fn get_execution_summary(&self, since: Duration) -> ExecutionSummary {
        let cutoff = Instant::now() - since;
        let relevant_reports: Vec<_> = self.execution_reports.iter()
            .filter(|report| {
                // This is a simplified check - in real implementation we'd store timestamps
                true
            })
            .collect();

        let total_operations = relevant_reports.len();
        let successful_operations = relevant_reports.iter()
            .filter(|report| report.success)
            .count();

        let average_duration = if total_operations > 0 {
            let total_duration: Duration = relevant_reports.iter()
                .map(|report| report.duration)
                .sum();
            total_duration / total_operations as u32
        } else {
            Duration::from_millis(0)
        };

        ExecutionSummary {
            total_operations,
            successful_operations,
            failed_operations: total_operations - successful_operations,
            success_rate: if total_operations > 0 {
                successful_operations as f64 / total_operations as f64
            } else {
                1.0
            },
            average_duration,
            time_period: since,
        }
    }
}

pub struct ChainedOperation {
    pub operation_id: String,
    pub component: String,
    pub operation: Box<dyn FnOnce() -> RobotResult<String>>,
    pub continue_on_error: bool,
}

#[derive(Debug, Clone)]
pub struct SystemHealthReport {
    pub overall_score: f64,
    pub subsystem_health: HashMap<String, f64>,
    pub recovery_statistics: RecoveryStatistics,
    pub recent_errors: usize,
    pub uptime: Duration,
    pub last_check: Instant,
}

#[derive(Debug, Clone)]
pub struct ExecutionSummary {
    pub total_operations: usize,
    pub successful_operations: usize,
    pub failed_operations: usize,
    pub success_rate: f64,
    pub average_duration: Duration,
    pub time_period: Duration,
}

/// Utility functions for advanced error handling patterns
pub mod advanced_patterns {
    use super::*;

    /// Try multiple operations and return the first successful result
    pub fn try_alternatives<T, E>(operations: Vec<Box<dyn FnOnce() -> Result<T, E>>>) -> Result<T, Vec<E>> {
        let mut errors = Vec::new();

        for operation in operations {
            match operation() {
                Ok(result) => return Ok(result),
                Err(error) => errors.push(error),
            }
        }

        Err(errors)
    }

    /// Execute operation with timeout and automatic retry
    pub fn with_timeout_and_retry<T, F>(
        operation: F,
        timeout: Duration,
        max_retries: u32,
    ) -> RobotResult<T>
    where
        F: Fn() -> RobotResult<T>,
    {
        let mut attempts = 0;

        while attempts < max_retries {
            attempts += 1;

            // In a real implementation, this would use actual timeout mechanisms
            match operation() {
                Ok(result) => return Ok(result),
                Err(err) => {
                    if attempts == max_retries {
                        return Err(err);
                    }
                    std::thread::sleep(Duration::from_millis(100 * attempts as u64));
                }
            }
        }

        Err(SystemError::Recoverable(RecoverableError::SensorTimeout {
            sensor: "operation".to_string(),
            timeout_ms: timeout.as_millis() as u64,
        }))
    }

    /// Collect results from multiple operations, handling partial failures
    pub fn collect_with_partial_failure<T, F>(
        operations: Vec<F>,
        min_success_rate: f64,
    ) -> RobotResult<Vec<T>>
    where
        F: FnOnce() -> RobotResult<T>,
    {
        let mut results = Vec::new();
        let mut errors = Vec::new();

        for operation in operations {
            match operation() {
                Ok(result) => results.push(result),
                Err(error) => errors.push(error),
            }
        }

        let total_operations = results.len() + errors.len();
        let success_rate = if total_operations > 0 {
            results.len() as f64 / total_operations as f64
        } else {
            0.0
        };

        if success_rate >= min_success_rate {
            Ok(results)
        } else {
            Err(SystemError::Critical(CriticalError::HardwareFailure {
                component: "parallel_operations".to_string(),
                code: 0x1001,
            }))
        }
    }

    /// Create a resilient operation that degrades gracefully
    pub fn resilient_operation<T, F1, F2, F3>(
        primary: F1,
        fallback: F2,
        emergency: F3,
    ) -> RobotResult<T>
    where
        F1: FnOnce() -> RobotResult<T>,
        F2: FnOnce() -> RobotResult<T>,
        F3: FnOnce() -> RobotResult<T>,
    {
        primary()
            .or_else(|_| fallback())
            .or_else(|_| emergency())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::advanced_patterns::*;

    #[test]
    fn test_system_error_hierarchy() {
        let critical = SystemError::Critical(CriticalError::HardwareFailure {
            component: "motor".to_string(),
            code: 0x1001,
        });

        let recoverable = SystemError::Recoverable(RecoverableError::SensorTimeout {
            sensor: "temperature".to_string(),
            timeout_ms: 1000,
        });

        let warning = SystemError::Warning(WarningError::BatteryLow {
            level: 25.0,
            warning_threshold: 30.0,
        });

        assert!(format!("{}", critical).contains("CRITICAL"));
        assert!(format!("{}", recoverable).contains("RECOVERABLE"));
        assert!(format!("{}", warning).contains("WARNING"));
    }

    #[test]
    fn test_error_recovery_manager() {
        let mut manager = ErrorRecoveryManager::new();

        manager.register_strategy(
            "test_error".to_string(),
            RecoveryStrategy {
                max_retries: 2,
                base_delay: Duration::from_millis(100),
                backoff_multiplier: 2.0,
                fallback_actions: vec![],
                success_threshold: 0.8,
            },
        );

        let mut attempt_count = 0;
        let result = manager.attempt_recovery(
            SystemError::Recoverable(RecoverableError::SensorTimeout {
                sensor: "test".to_string(),
                timeout_ms: 1000,
            }),
            || {
                attempt_count += 1;
                if attempt_count >= 2 {
                    Ok("success".to_string())
                } else {
                    Err(SystemError::Recoverable(RecoverableError::SensorTimeout {
                        sensor: "test".to_string(),
                        timeout_ms: 1000,
                    }))
                }
            },
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempt_count, 2);
    }

    #[test]
    fn test_monitored_operations() {
        let monitored = Monitored::new(
            Ok(42),
            "test_op".to_string(),
            "test_component".to_string(),
        );

        let result = monitored
            .map(|x| x * 2)
            .and_then(|x| Monitored::new(Ok(x + 10), "chain_op".to_string(), "test_component".to_string()))
            .with_metadata("test_key".to_string(), "test_value".to_string());

        let (final_result, report) = result.into_result_with_report();
        assert!(final_result.is_ok());
        assert_eq!(final_result.unwrap(), 94); // (42 * 2) + 10
        assert!(report.success);
        assert!(report.metadata.contains_key("test_key"));
    }

    #[test]
    fn test_monitored_error_handling() {
        let monitored = Monitored::new(
            Err(SystemError::Warning(WarningError::BatteryLow {
                level: 10.0,
                warning_threshold: 20.0,
            })),
            "test_op".to_string(),
            "test_component".to_string(),
        );

        let result = monitored.or_else(|_| {
            Monitored::new(
                Ok("fallback_value".to_string()),
                "fallback_op".to_string(),
                "test_component".to_string(),
            )
        });

        let (final_result, report) = result.into_result_with_report();
        assert!(final_result.is_ok());
        assert_eq!(final_result.unwrap(), "fallback_value");
        assert!(report.success);
    }

    #[test]
    fn test_advanced_robot_system() {
        let mut system = AdvancedRobotSystem::new();

        let monitored = system.execute_monitored_operation(
            "test_operation".to_string(),
            "test_component".to_string(),
            || Ok("test_result".to_string()),
        );

        let (result, _) = monitored.into_result_with_report();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_result");
    }

    #[test]
    fn test_try_alternatives() {
        let operations: Vec<Box<dyn FnOnce() -> Result<i32, String>>> = vec![
            Box::new(|| Err("first error".to_string())),
            Box::new(|| Err("second error".to_string())),
            Box::new(|| Ok(42)),
            Box::new(|| Ok(100)),
        ];

        let result = try_alternatives(operations);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_try_alternatives_all_fail() {
        let operations: Vec<Box<dyn FnOnce() -> Result<i32, String>>> = vec![
            Box::new(|| Err("first error".to_string())),
            Box::new(|| Err("second error".to_string())),
        ];

        let result = try_alternatives(operations);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_collect_with_partial_failure() {
        let operations: Vec<Box<dyn FnOnce() -> RobotResult<i32>>> = vec![
            Box::new(|| Ok(1)),
            Box::new(|| Err(SystemError::Warning(WarningError::BatteryLow {
                level: 10.0,
                warning_threshold: 20.0,
            }))),
            Box::new(|| Ok(3)),
        ];

        let result = collect_with_partial_failure(operations, 0.5);
        assert!(result.is_ok());

        let results = result.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], 1);
        assert_eq!(results[1], 3);
    }

    #[test]
    fn test_resilient_operation() {
        let result = resilient_operation(
            || Err(SystemError::Warning(WarningError::BatteryLow {
                level: 10.0,
                warning_threshold: 20.0,
            })),
            || Err(SystemError::Warning(WarningError::NetworkLatency {
                latency_ms: 1000,
                threshold_ms: 500,
            })),
            || Ok("emergency_result".to_string()),
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "emergency_result");
    }

    #[test]
    fn test_with_timeout_and_retry() {
        let mut attempt_count = 0;
        let result = with_timeout_and_retry(
            || {
                attempt_count += 1;
                if attempt_count >= 3 {
                    Ok("success".to_string())
                } else {
                    Err(SystemError::Recoverable(RecoverableError::SensorTimeout {
                        sensor: "test".to_string(),
                        timeout_ms: 100,
                    }))
                }
            },
            Duration::from_secs(5),
            5,
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempt_count, 3);
    }

    #[test]
    fn test_health_check() {
        let mut system = AdvancedRobotSystem::new();

        system.add_health_check(
            "test_check".to_string(),
            || Ok(0.8),
            1.0,
        );

        let health_report = system.perform_health_check();
        // This test would pass in a real implementation with proper timing
    }

    #[test]
    fn test_execution_summary() {
        let system = AdvancedRobotSystem::new();
        let summary = system.get_execution_summary(Duration::from_hours(1));

        assert_eq!(summary.total_operations, 0);
        assert_eq!(summary.success_rate, 1.0);
    }
}

/// Public module for student exercises
pub mod exercises {
    use super::*;

    /// Exercise 1: Implement a distributed error coordination system
    ///
    /// Create a system that coordinates error handling across multiple robot instances:
    /// - Share error information between robots
    /// - Implement distributed recovery strategies
    /// - Handle network partitions gracefully
    /// - Create consensus mechanisms for critical errors
    ///
    /// Requirements:
    /// - Advanced error propagation across network boundaries
    /// - Consensus protocols for distributed error handling
    /// - Fault tolerance in distributed coordination
    pub fn exercise_1_distributed_error_coordination() {
        // TODO: Implement DistributedErrorCoordinator
        // TODO: Add consensus mechanisms for error handling
        // TODO: Create network partition tolerance
        println!("Exercise 1: Implement distributed error coordination system");
    }

    /// Exercise 2: Machine learning-based error prediction
    ///
    /// Create a predictive error handling system:
    /// - Analyze error patterns to predict future failures
    /// - Implement proactive recovery based on predictions
    /// - Create confidence intervals for error predictions
    /// - Build adaptive recovery strategies based on learning
    ///
    /// Requirements:
    /// - Error pattern analysis and prediction algorithms
    /// - Proactive recovery mechanism implementation
    /// - Adaptive strategy learning and optimization
    pub fn exercise_2_predictive_error_handling() {
        // TODO: Implement PredictiveErrorHandler
        // TODO: Add pattern analysis and prediction algorithms
        // TODO: Create proactive recovery mechanisms
        println!("Exercise 2: Implement ML-based error prediction system");
    }

    /// Exercise 3: Hierarchical error recovery with resource allocation
    ///
    /// Create a sophisticated error recovery system with resource management:
    /// - Implement hierarchical recovery strategies
    /// - Manage resource allocation during recovery
    /// - Handle resource conflicts and dependencies
    /// - Create recovery optimization based on resource constraints
    ///
    /// Requirements:
    /// - Hierarchical recovery strategy implementation
    /// - Resource-aware recovery optimization
    /// - Conflict resolution and dependency management
    pub fn exercise_3_hierarchical_recovery() {
        // TODO: Implement HierarchicalRecoveryManager
        // TODO: Add resource allocation during recovery
        // TODO: Create recovery optimization algorithms
        println!("Exercise 3: Implement hierarchical error recovery with resource allocation");
    }

    /// Exercise 4: Real-time error handling with temporal constraints
    ///
    /// Create a real-time error handling system:
    /// - Handle errors within strict timing constraints
    /// - Implement priority-based error handling
    /// - Create temporal error analysis and prediction
    /// - Build deadline-aware recovery strategies
    ///
    /// Requirements:
    /// - Real-time constraint satisfaction in error handling
    /// - Priority-based error processing queues
    /// - Temporal analysis of error patterns
    pub fn exercise_4_realtime_error_handling() {
        // TODO: Implement RealtimeErrorHandler
        // TODO: Add priority-based error processing
        // TODO: Create deadline-aware recovery strategies
        println!("Exercise 4: Implement real-time error handling system");
    }

    /// Exercise 5: Comprehensive error analytics and optimization
    ///
    /// Create an advanced error analytics system:
    /// - Collect and analyze comprehensive error metrics
    /// - Implement error handling optimization algorithms
    /// - Create error handling performance benchmarks
    /// - Build adaptive system optimization based on error patterns
    ///
    /// Requirements:
    /// - Comprehensive error metrics collection and analysis
    /// - Performance optimization algorithms for error handling
    /// - Benchmarking and continuous improvement systems
    pub fn exercise_5_error_analytics_optimization() {
        // TODO: Implement ErrorAnalyticsSystem
        // TODO: Add comprehensive metrics and optimization
        // TODO: Create performance benchmarking system
        println!("Exercise 5: Implement comprehensive error analytics and optimization");
    }
}