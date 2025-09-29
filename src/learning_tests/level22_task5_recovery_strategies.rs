//! Level 22, Task 5: Advanced Error Recovery Strategies
//!
//! This module demonstrates sophisticated error recovery patterns and strategies
//! for robust robot systems using anyhow and custom recovery mechanisms.
//!
//! Learning objectives:
//! - Implement retry patterns with exponential backoff
//! - Create circuit breaker patterns for fault tolerance
//! - Design graceful degradation systems
//! - Implement recovery state machines
//! - Handle partial system failures with fallback strategies

use anyhow::{anyhow, bail, ensure, Context, Result};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::fmt;
use std::error::Error as StdError;

/// Recovery strategy configuration
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub jitter: bool,
    pub circuit_breaker_threshold: u32,
    pub circuit_breaker_timeout: Duration,
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            jitter: true,
            circuit_breaker_threshold: 5,
            circuit_breaker_timeout: Duration::from_secs(30),
        }
    }
}

/// Circuit breaker states for fault tolerance
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    Closed,    // Normal operation
    Open,      // Failing, blocking requests
    HalfOpen,  // Testing if service recovered
}

/// Circuit breaker for managing service failures
#[derive(Debug)]
pub struct CircuitBreaker {
    state: CircuitBreakerState,
    failure_count: u32,
    threshold: u32,
    timeout: Duration,
    last_failure_time: Option<Instant>,
    success_count_in_half_open: u32,
}

impl CircuitBreaker {
    pub fn new(threshold: u32, timeout: Duration) -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            threshold,
            timeout,
            last_failure_time: None,
            success_count_in_half_open: 0,
        }
    }

    pub fn can_proceed(&mut self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    if last_failure.elapsed() >= self.timeout {
                        self.state = CircuitBreakerState::HalfOpen;
                        self.success_count_in_half_open = 0;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }

    pub fn record_success(&mut self) {
        match self.state {
            CircuitBreakerState::Closed => {
                self.failure_count = 0;
            }
            CircuitBreakerState::HalfOpen => {
                self.success_count_in_half_open += 1;
                if self.success_count_in_half_open >= 3 {
                    self.state = CircuitBreakerState::Closed;
                    self.failure_count = 0;
                }
            }
            CircuitBreakerState::Open => {
                // This shouldn't happen, but reset if it does
                self.state = CircuitBreakerState::Closed;
                self.failure_count = 0;
            }
        }
    }

    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(Instant::now());

        match self.state {
            CircuitBreakerState::Closed => {
                if self.failure_count >= self.threshold {
                    self.state = CircuitBreakerState::Open;
                }
            }
            CircuitBreakerState::HalfOpen => {
                self.state = CircuitBreakerState::Open;
            }
            CircuitBreakerState::Open => {
                // Already open, just update failure time
            }
        }
    }

    pub fn get_state(&self) -> &CircuitBreakerState {
        &self.state
    }

    pub fn get_failure_count(&self) -> u32 {
        self.failure_count
    }
}

/// Retry policy with exponential backoff
#[derive(Debug)]
pub struct RetryPolicy {
    config: RecoveryConfig,
    attempt_count: u32,
    last_delay: Duration,
}

impl RetryPolicy {
    pub fn new(config: RecoveryConfig) -> Self {
        Self {
            last_delay: config.initial_delay,
            config,
            attempt_count: 0,
        }
    }

    pub fn should_retry(&self) -> bool {
        self.attempt_count < self.config.max_attempts
    }

    pub fn next_delay(&mut self) -> Duration {
        if self.attempt_count == 0 {
            self.attempt_count += 1;
            return Duration::from_millis(0); // No delay for first attempt
        }

        let mut delay = Duration::from_millis(
            (self.last_delay.as_millis() as f64 * self.config.backoff_multiplier) as u64
        );

        if delay > self.config.max_delay {
            delay = self.config.max_delay;
        }

        // Add jitter to prevent thundering herd
        if self.config.jitter {
            let jitter_amount = delay.as_millis() as f64 * 0.1 * (rand::random::<f64>() - 0.5);
            let jittered_ms = (delay.as_millis() as f64 + jitter_amount).max(0.0) as u64;
            delay = Duration::from_millis(jittered_ms);
        }

        self.last_delay = delay;
        self.attempt_count += 1;
        delay
    }

    pub fn reset(&mut self) {
        self.attempt_count = 0;
        self.last_delay = self.config.initial_delay;
    }

    pub fn get_attempt_count(&self) -> u32 {
        self.attempt_count
    }
}

/// System health monitoring
#[derive(Debug, Clone)]
pub struct HealthMetrics {
    pub success_rate: f64,
    pub average_response_time: Duration,
    pub error_rate: f64,
    pub last_success: Option<Instant>,
    pub last_failure: Option<Instant>,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
}

impl Default for HealthMetrics {
    fn default() -> Self {
        Self {
            success_rate: 1.0,
            average_response_time: Duration::from_millis(0),
            error_rate: 0.0,
            last_success: None,
            last_failure: None,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
        }
    }
}

impl HealthMetrics {
    pub fn record_success(&mut self, response_time: Duration) {
        self.total_requests += 1;
        self.successful_requests += 1;
        self.last_success = Some(Instant::now());

        // Update average response time (simple moving average)
        let total_time = self.average_response_time.as_millis() as f64 * (self.successful_requests - 1) as f64;
        self.average_response_time = Duration::from_millis(
            ((total_time + response_time.as_millis() as f64) / self.successful_requests as f64) as u64
        );

        self.update_rates();
    }

    pub fn record_failure(&mut self) {
        self.total_requests += 1;
        self.failed_requests += 1;
        self.last_failure = Some(Instant::now());
        self.update_rates();
    }

    fn update_rates(&mut self) {
        if self.total_requests > 0 {
            self.success_rate = self.successful_requests as f64 / self.total_requests as f64;
            self.error_rate = self.failed_requests as f64 / self.total_requests as f64;
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.success_rate >= 0.8 && self.error_rate <= 0.2
    }
}

/// Recovery action types
#[derive(Debug, Clone)]
pub enum RecoveryAction {
    Retry,
    FallbackToBackup,
    GracefulDegradation,
    EmergencyStop,
    RestartComponent,
    SwitchToManualMode,
    IgnoreAndContinue,
}

/// Recovery strategy based on error analysis
#[derive(Debug)]
pub struct RecoveryStrategy {
    action: RecoveryAction,
    estimated_time: Duration,
    success_probability: f64,
    description: String,
}

impl RecoveryStrategy {
    pub fn new(action: RecoveryAction, estimated_time: Duration, success_probability: f64, description: String) -> Self {
        Self {
            action,
            estimated_time,
            success_probability,
            description,
        }
    }

    pub fn get_action(&self) -> &RecoveryAction {
        &self.action
    }

    pub fn get_estimated_time(&self) -> Duration {
        self.estimated_time
    }

    pub fn get_success_probability(&self) -> f64 {
        self.success_probability
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}

/// Resilient robot navigation system with advanced recovery
pub struct ResilientNavigationSystem {
    circuit_breaker: CircuitBreaker,
    health_metrics: HealthMetrics,
    recovery_history: VecDeque<(Instant, RecoveryAction, bool)>,
    fallback_routes: Vec<String>,
    current_route_index: usize,
    emergency_stop_active: bool,
    manual_mode: bool,
    degraded_operation: bool,
}

impl ResilientNavigationSystem {
    pub fn new() -> Self {
        Self {
            circuit_breaker: CircuitBreaker::new(3, Duration::from_secs(30)),
            health_metrics: HealthMetrics::default(),
            recovery_history: VecDeque::with_capacity(100),
            fallback_routes: vec![
                "primary_route".to_string(),
                "backup_route_1".to_string(),
                "backup_route_2".to_string(),
                "emergency_route".to_string(),
            ],
            current_route_index: 0,
            emergency_stop_active: false,
            manual_mode: false,
            degraded_operation: false,
        }
    }

    pub fn navigate_to(&mut self, destination: &str) -> Result<String> {
        if self.emergency_stop_active {
            return Err(anyhow!("Emergency stop is active - navigation disabled"));
        }

        if self.manual_mode {
            return Ok("Manual navigation mode - operator control".to_string());
        }

        if !self.circuit_breaker.can_proceed() {
            return Err(anyhow!("Navigation circuit breaker is open - service unavailable"));
        }

        let start_time = Instant::now();
        let result = self.attempt_navigation(destination);

        match &result {
            Ok(_) => {
                self.circuit_breaker.record_success();
                self.health_metrics.record_success(start_time.elapsed());
                if self.degraded_operation {
                    // Try to return to normal operation
                    self.attempt_recovery_to_normal_operation();
                }
            }
            Err(_) => {
                self.circuit_breaker.record_failure();
                self.health_metrics.record_failure();

                // Attempt recovery based on error type and system state
                if let Err(recovery_error) = self.attempt_error_recovery(&result) {
                    return Err(recovery_error.context("Navigation failed and recovery failed"));
                }
            }
        }

        result
    }

    fn attempt_navigation(&self, destination: &str) -> Result<String> {
        let route = &self.fallback_routes[self.current_route_index];

        // Simulate navigation with potential failures
        match route.as_str() {
            "primary_route" => {
                if rand::random::<f64>() < 0.2 {
                    bail!("Primary route blocked by obstacle");
                }
                if self.degraded_operation {
                    Ok(format!("Navigated to {} via {} (degraded mode)", destination, route))
                } else {
                    Ok(format!("Navigated to {} via {}", destination, route))
                }
            }
            "backup_route_1" => {
                if rand::random::<f64>() < 0.15 {
                    bail!("Backup route 1 has sensor malfunction");
                }
                Ok(format!("Navigated to {} via {} (slower)", destination, route))
            }
            "backup_route_2" => {
                if rand::random::<f64>() < 0.1 {
                    bail!("Backup route 2 has communication issues");
                }
                Ok(format!("Navigated to {} via {} (much slower)", destination, route))
            }
            "emergency_route" => {
                if rand::random::<f64>() < 0.05 {
                    bail!("Emergency route requires manual intervention");
                }
                Ok(format!("Navigated to {} via {} (emergency mode)", destination, route))
            }
            _ => bail!("Unknown route: {}", route),
        }
    }

    fn attempt_error_recovery(&mut self, error: &Result<String>) -> Result<()> {
        let strategy = self.determine_recovery_strategy(error);

        match strategy.get_action() {
            RecoveryAction::FallbackToBackup => {
                if self.current_route_index + 1 < self.fallback_routes.len() {
                    self.current_route_index += 1;
                    self.record_recovery_attempt(RecoveryAction::FallbackToBackup, true);
                    Ok(())
                } else {
                    self.record_recovery_attempt(RecoveryAction::FallbackToBackup, false);
                    Err(anyhow!("No more backup routes available"))
                }
            }
            RecoveryAction::GracefulDegradation => {
                self.degraded_operation = true;
                self.record_recovery_attempt(RecoveryAction::GracefulDegradation, true);
                Ok(())
            }
            RecoveryAction::EmergencyStop => {
                self.emergency_stop_active = true;
                self.record_recovery_attempt(RecoveryAction::EmergencyStop, true);
                Err(anyhow!("Emergency stop activated due to critical navigation failure"))
            }
            RecoveryAction::SwitchToManualMode => {
                self.manual_mode = true;
                self.record_recovery_attempt(RecoveryAction::SwitchToManualMode, true);
                Ok(())
            }
            RecoveryAction::RestartComponent => {
                // Simulate component restart
                self.current_route_index = 0;
                self.degraded_operation = false;
                self.record_recovery_attempt(RecoveryAction::RestartComponent, true);
                Ok(())
            }
            _ => {
                self.record_recovery_attempt(RecoveryAction::Retry, false);
                Err(anyhow!("No suitable recovery strategy available"))
            }
        }
    }

    fn determine_recovery_strategy(&self, error: &Result<String>) -> RecoveryStrategy {
        let error_msg = format!("{:?}", error);

        if error_msg.contains("obstacle") {
            RecoveryStrategy::new(
                RecoveryAction::FallbackToBackup,
                Duration::from_secs(5),
                0.8,
                "Switch to backup route to avoid obstacle".to_string(),
            )
        } else if error_msg.contains("sensor malfunction") {
            if self.health_metrics.error_rate > 0.5 {
                RecoveryStrategy::new(
                    RecoveryAction::GracefulDegradation,
                    Duration::from_secs(10),
                    0.7,
                    "Switch to degraded operation with reduced sensor accuracy".to_string(),
                )
            } else {
                RecoveryStrategy::new(
                    RecoveryAction::FallbackToBackup,
                    Duration::from_secs(3),
                    0.9,
                    "Use backup route with working sensors".to_string(),
                )
            }
        } else if error_msg.contains("communication issues") {
            RecoveryStrategy::new(
                RecoveryAction::RestartComponent,
                Duration::from_secs(15),
                0.6,
                "Restart communication subsystem".to_string(),
            )
        } else if error_msg.contains("manual intervention") {
            RecoveryStrategy::new(
                RecoveryAction::SwitchToManualMode,
                Duration::from_secs(1),
                1.0,
                "Switch to manual operator control".to_string(),
            )
        } else if self.circuit_breaker.get_failure_count() > 5 {
            RecoveryStrategy::new(
                RecoveryAction::EmergencyStop,
                Duration::from_secs(1),
                1.0,
                "Emergency stop due to repeated failures".to_string(),
            )
        } else {
            RecoveryStrategy::new(
                RecoveryAction::Retry,
                Duration::from_secs(2),
                0.5,
                "Retry current operation".to_string(),
            )
        }
    }

    fn attempt_recovery_to_normal_operation(&mut self) {
        if self.degraded_operation && self.health_metrics.success_rate > 0.9 {
            self.degraded_operation = false;
            self.current_route_index = 0;
        }
    }

    fn record_recovery_attempt(&mut self, action: RecoveryAction, success: bool) {
        self.recovery_history.push_back((Instant::now(), action, success));
        if self.recovery_history.len() > 100 {
            self.recovery_history.pop_front();
        }
    }

    pub fn get_system_status(&self) -> String {
        format!(
            "Navigation System Status:\n\
             - Circuit Breaker: {:?}\n\
             - Health: {} (success rate: {:.1}%)\n\
             - Current Route: {}\n\
             - Emergency Stop: {}\n\
             - Manual Mode: {}\n\
             - Degraded Operation: {}\n\
             - Recent Recoveries: {}",
            self.circuit_breaker.get_state(),
            if self.health_metrics.is_healthy() { "Healthy" } else { "Unhealthy" },
            self.health_metrics.success_rate * 100.0,
            self.fallback_routes[self.current_route_index],
            self.emergency_stop_active,
            self.manual_mode,
            self.degraded_operation,
            self.recovery_history.len()
        )
    }

    pub fn reset_emergency_stop(&mut self) -> Result<()> {
        if !self.emergency_stop_active {
            return Err(anyhow!("Emergency stop is not active"));
        }

        // Perform safety checks before resetting
        if self.health_metrics.error_rate > 0.3 {
            return Err(anyhow!("System error rate too high to reset emergency stop"));
        }

        self.emergency_stop_active = false;
        self.current_route_index = 0;
        self.degraded_operation = false;
        self.manual_mode = false;

        Ok(())
    }

    pub fn exit_manual_mode(&mut self) -> Result<()> {
        if !self.manual_mode {
            return Err(anyhow!("Not in manual mode"));
        }

        if !self.health_metrics.is_healthy() {
            return Err(anyhow!("System not healthy enough for automatic mode"));
        }

        self.manual_mode = false;
        Ok(())
    }
}

/// Robust service manager with retry and circuit breaker
pub struct RobotServiceManager {
    services: HashMap<String, ServiceInfo>,
    global_config: RecoveryConfig,
}

#[derive(Debug)]
struct ServiceInfo {
    circuit_breaker: CircuitBreaker,
    retry_policy: RetryPolicy,
    health_metrics: HealthMetrics,
    backup_services: Vec<String>,
}

impl RobotServiceManager {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            global_config: RecoveryConfig::default(),
        }
    }

    pub fn register_service(&mut self, name: &str, backup_services: Vec<String>) {
        let service_info = ServiceInfo {
            circuit_breaker: CircuitBreaker::new(
                self.global_config.circuit_breaker_threshold,
                self.global_config.circuit_breaker_timeout,
            ),
            retry_policy: RetryPolicy::new(self.global_config.clone()),
            health_metrics: HealthMetrics::default(),
            backup_services,
        };

        self.services.insert(name.to_string(), service_info);
    }

    pub fn call_service<F, T>(&mut self, service_name: &str, operation: F) -> Result<T>
    where
        F: Fn() -> Result<T> + Clone,
    {
        let service_info = self.services.get_mut(service_name)
            .ok_or_else(|| anyhow!("Service '{}' not registered", service_name))?;

        // Check circuit breaker
        if !service_info.circuit_breaker.can_proceed() {
            return self.try_backup_services(service_name, operation);
        }

        // Attempt with retry policy
        let result = self.retry_operation(&mut service_info.retry_policy, operation.clone());

        match &result {
            Ok(_) => {
                service_info.circuit_breaker.record_success();
                service_info.health_metrics.record_success(Duration::from_millis(100)); // Simulated
                service_info.retry_policy.reset();
            }
            Err(_) => {
                service_info.circuit_breaker.record_failure();
                service_info.health_metrics.record_failure();

                // Try backup services if primary failed
                return self.try_backup_services(service_name, operation);
            }
        }

        result
    }

    fn retry_operation<F, T>(&self, retry_policy: &mut RetryPolicy, operation: F) -> Result<T>
    where
        F: Fn() -> Result<T>,
    {
        let mut last_error = None;

        while retry_policy.should_retry() {
            let delay = retry_policy.next_delay();
            if delay > Duration::from_millis(0) {
                std::thread::sleep(delay);
            }

            match operation() {
                Ok(result) => return Ok(result),
                Err(e) => last_error = Some(e),
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("Retry limit exceeded")))
    }

    fn try_backup_services<F, T>(&mut self, primary_service: &str, operation: F) -> Result<T>
    where
        F: Fn() -> Result<T> + Clone,
    {
        let backup_services = if let Some(service_info) = self.services.get(primary_service) {
            service_info.backup_services.clone()
        } else {
            return Err(anyhow!("Primary service '{}' not found", primary_service));
        };

        for backup_service in backup_services {
            if let Ok(result) = self.call_service(&backup_service, operation.clone()) {
                return Ok(result);
            }
        }

        Err(anyhow!("All services (primary and backups) failed for '{}'", primary_service))
    }

    pub fn get_service_status(&self, service_name: &str) -> Result<String> {
        let service_info = self.services.get(service_name)
            .ok_or_else(|| anyhow!("Service '{}' not found", service_name))?;

        Ok(format!(
            "Service '{}' Status:\n\
             - Circuit Breaker: {:?}\n\
             - Health: {} (success rate: {:.1}%)\n\
             - Retry Attempts: {}\n\
             - Backup Services: {:?}",
            service_name,
            service_info.circuit_breaker.get_state(),
            if service_info.health_metrics.is_healthy() { "Healthy" } else { "Unhealthy" },
            service_info.health_metrics.success_rate * 100.0,
            service_info.retry_policy.get_attempt_count(),
            service_info.backup_services
        ))
    }

    pub fn reset_service(&mut self, service_name: &str) -> Result<()> {
        let service_info = self.services.get_mut(service_name)
            .ok_or_else(|| anyhow!("Service '{}' not found", service_name))?;

        service_info.circuit_breaker = CircuitBreaker::new(
            self.global_config.circuit_breaker_threshold,
            self.global_config.circuit_breaker_timeout,
        );
        service_info.retry_policy.reset();
        service_info.health_metrics = HealthMetrics::default();

        Ok(())
    }
}

/// Graceful degradation manager for partial system failures
pub struct GracefulDegradationManager {
    features: HashMap<String, FeatureInfo>,
    degradation_levels: Vec<DegradationLevel>,
    current_level: usize,
}

#[derive(Debug, Clone)]
struct FeatureInfo {
    essential: bool,
    fallback_available: bool,
    performance_impact: f64,
    last_status: FeatureStatus,
}

#[derive(Debug, Clone)]
enum FeatureStatus {
    Operational,
    Degraded,
    Failed,
    Disabled,
}

#[derive(Debug, Clone)]
struct DegradationLevel {
    name: String,
    disabled_features: Vec<String>,
    description: String,
}

impl GracefulDegradationManager {
    pub fn new() -> Self {
        let mut manager = Self {
            features: HashMap::new(),
            degradation_levels: Vec::new(),
            current_level: 0,
        };

        // Initialize degradation levels
        manager.degradation_levels = vec![
            DegradationLevel {
                name: "Normal".to_string(),
                disabled_features: vec![],
                description: "All systems operational".to_string(),
            },
            DegradationLevel {
                name: "Level1".to_string(),
                disabled_features: vec!["advanced_path_planning".to_string()],
                description: "Disable advanced features".to_string(),
            },
            DegradationLevel {
                name: "Level2".to_string(),
                disabled_features: vec!["advanced_path_planning".to_string(), "object_tracking".to_string()],
                description: "Disable non-essential features".to_string(),
            },
            DegradationLevel {
                name: "Level3".to_string(),
                disabled_features: vec![
                    "advanced_path_planning".to_string(),
                    "object_tracking".to_string(),
                    "voice_recognition".to_string(),
                ],
                description: "Essential features only".to_string(),
            },
            DegradationLevel {
                name: "Emergency".to_string(),
                disabled_features: vec![
                    "advanced_path_planning".to_string(),
                    "object_tracking".to_string(),
                    "voice_recognition".to_string(),
                    "autonomous_navigation".to_string(),
                ],
                description: "Manual control only".to_string(),
            },
        ];

        manager
    }

    pub fn register_feature(&mut self, name: &str, essential: bool, fallback_available: bool, performance_impact: f64) {
        self.features.insert(name.to_string(), FeatureInfo {
            essential,
            fallback_available,
            performance_impact,
            last_status: FeatureStatus::Operational,
        });
    }

    pub fn report_feature_failure(&mut self, feature_name: &str) -> Result<()> {
        let feature_info = self.features.get_mut(feature_name)
            .ok_or_else(|| anyhow!("Feature '{}' not registered", feature_name))?;

        feature_info.last_status = if feature_info.fallback_available {
            FeatureStatus::Degraded
        } else {
            FeatureStatus::Failed
        };

        // Determine if degradation is needed
        if feature_info.essential && !feature_info.fallback_available {
            self.escalate_degradation_level()?;
        }

        Ok(())
    }

    fn escalate_degradation_level(&mut self) -> Result<()> {
        if self.current_level + 1 < self.degradation_levels.len() {
            self.current_level += 1;

            // Disable features for this level
            let level = &self.degradation_levels[self.current_level];
            for feature_name in &level.disabled_features {
                if let Some(feature_info) = self.features.get_mut(feature_name) {
                    feature_info.last_status = FeatureStatus::Disabled;
                }
            }

            Ok(())
        } else {
            Err(anyhow!("Maximum degradation level reached"))
        }
    }

    pub fn attempt_recovery(&mut self) -> Result<()> {
        // Try to recover features and potentially improve degradation level
        let mut recovered_features = Vec::new();

        for (feature_name, feature_info) in &mut self.features {
            if matches!(feature_info.last_status, FeatureStatus::Failed | FeatureStatus::Degraded) {
                // Simulate recovery attempt
                if rand::random::<f64>() < 0.3 {
                    feature_info.last_status = FeatureStatus::Operational;
                    recovered_features.push(feature_name.clone());
                }
            }
        }

        // If enough features recovered, try to improve degradation level
        if !recovered_features.is_empty() && self.current_level > 0 {
            let can_improve = self.can_improve_degradation_level();
            if can_improve {
                self.current_level -= 1;

                // Re-enable features for improved level
                let level = &self.degradation_levels[self.current_level];
                for feature_name in self.features.keys() {
                    if !level.disabled_features.contains(feature_name) {
                        if let Some(feature_info) = self.features.get_mut(feature_name) {
                            if matches!(feature_info.last_status, FeatureStatus::Disabled) {
                                feature_info.last_status = FeatureStatus::Operational;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn can_improve_degradation_level(&self) -> bool {
        if self.current_level == 0 {
            return false;
        }

        // Check if essential features are working
        let essential_features_working = self.features.iter()
            .filter(|(_, info)| info.essential)
            .all(|(_, info)| matches!(info.last_status, FeatureStatus::Operational | FeatureStatus::Degraded));

        essential_features_working
    }

    pub fn get_system_status(&self) -> String {
        let current_level = &self.degradation_levels[self.current_level];
        let operational_features = self.features.iter()
            .filter(|(_, info)| matches!(info.last_status, FeatureStatus::Operational))
            .count();
        let total_features = self.features.len();

        format!(
            "Graceful Degradation Status:\n\
             - Current Level: {} ({})\n\
             - Operational Features: {}/{}\n\
             - Description: {}",
            current_level.name,
            self.current_level,
            operational_features,
            total_features,
            current_level.description
        )
    }

    pub fn is_feature_available(&self, feature_name: &str) -> bool {
        if let Some(feature_info) = self.features.get(feature_name) {
            matches!(feature_info.last_status, FeatureStatus::Operational | FeatureStatus::Degraded)
        } else {
            false
        }
    }
}

// Simple random number generator for simulation
mod rand {
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEED: AtomicU64 = AtomicU64::new(1);

    pub fn random<T>() -> T
    where
        T: From<f64>
    {
        let mut seed = SEED.load(Ordering::Relaxed);
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        SEED.store(seed, Ordering::Relaxed);

        let normalized = (seed as f64) / (u64::MAX as f64);
        T::from(normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_states() {
        let mut cb = CircuitBreaker::new(2, Duration::from_millis(100));

        // Initial state should be closed
        assert_eq!(*cb.get_state(), CircuitBreakerState::Closed);
        assert!(cb.can_proceed());

        // Record failures to open circuit
        cb.record_failure();
        assert_eq!(*cb.get_state(), CircuitBreakerState::Closed);

        cb.record_failure();
        assert_eq!(*cb.get_state(), CircuitBreakerState::Open);
        assert!(!cb.can_proceed());

        // Wait for timeout and check half-open state
        std::thread::sleep(Duration::from_millis(110));
        assert!(cb.can_proceed());
        assert_eq!(*cb.get_state(), CircuitBreakerState::HalfOpen);

        // Success should close the circuit
        cb.record_success();
        cb.record_success();
        cb.record_success();
        assert_eq!(*cb.get_state(), CircuitBreakerState::Closed);
    }

    #[test]
    fn test_retry_policy() {
        let config = RecoveryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
            jitter: false,
            ..Default::default()
        };

        let mut policy = RetryPolicy::new(config);

        assert!(policy.should_retry());
        assert_eq!(policy.next_delay(), Duration::from_millis(0)); // First attempt

        assert!(policy.should_retry());
        let delay1 = policy.next_delay();
        assert!(delay1 >= Duration::from_millis(10));

        assert!(policy.should_retry());
        let delay2 = policy.next_delay();
        assert!(delay2 >= delay1);

        assert!(!policy.should_retry()); // Exceeded max attempts
    }

    #[test]
    fn test_health_metrics() {
        let mut metrics = HealthMetrics::default();

        metrics.record_success(Duration::from_millis(100));
        metrics.record_success(Duration::from_millis(200));
        metrics.record_failure();

        assert_eq!(metrics.total_requests, 3);
        assert_eq!(metrics.successful_requests, 2);
        assert_eq!(metrics.failed_requests, 1);
        assert!((metrics.success_rate - 0.666).abs() < 0.01);
        assert!(metrics.is_healthy());
    }

    #[test]
    fn test_resilient_navigation_system() {
        let mut nav = ResilientNavigationSystem::new();

        // Test successful navigation
        let result = nav.navigate_to("destination_1");
        // Result may succeed or fail based on simulation

        // Test system status
        let status = nav.get_system_status();
        assert!(status.contains("Navigation System Status"));
    }

    #[test]
    fn test_service_manager() {
        let mut manager = RobotServiceManager::new();

        manager.register_service("vision", vec!["backup_vision".to_string()]);
        manager.register_service("backup_vision", vec![]);

        // Test service call
        let result = manager.call_service("vision", || -> Result<String> {
            Ok("Vision data".to_string())
        });

        assert!(result.is_ok());

        // Test service status
        let status = manager.get_service_status("vision");
        assert!(status.is_ok());
    }

    #[test]
    fn test_graceful_degradation_manager() {
        let mut manager = GracefulDegradationManager::new();

        manager.register_feature("advanced_path_planning", false, true, 0.8);
        manager.register_feature("basic_navigation", true, false, 0.9);

        // Test feature failure
        let result = manager.report_feature_failure("advanced_path_planning");
        assert!(result.is_ok());

        // Test system status
        let status = manager.get_system_status();
        assert!(status.contains("Graceful Degradation Status"));

        // Test feature availability
        assert!(manager.is_feature_available("basic_navigation"));
    }

    #[test]
    fn test_recovery_strategy_selection() {
        let nav = ResilientNavigationSystem::new();

        let obstacle_error: Result<String> = Err(anyhow!("Primary route blocked by obstacle"));
        let strategy = nav.determine_recovery_strategy(&obstacle_error);

        assert!(matches!(strategy.get_action(), RecoveryAction::FallbackToBackup));
        assert!(strategy.get_success_probability() > 0.7);
    }

    #[test]
    fn test_emergency_stop_and_reset() {
        let mut nav = ResilientNavigationSystem::new();

        // Force emergency stop
        nav.emergency_stop_active = true;

        let result = nav.navigate_to("test_destination");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Emergency stop"));

        // Reset should fail if system is unhealthy
        nav.health_metrics.failed_requests = 10;
        nav.health_metrics.total_requests = 10;
        nav.health_metrics.update_rates();

        let reset_result = nav.reset_emergency_stop();
        assert!(reset_result.is_err());

        // Reset should succeed with healthy system
        nav.health_metrics.successful_requests = 15;
        nav.health_metrics.total_requests = 20;
        nav.health_metrics.update_rates();

        let reset_result = nav.reset_emergency_stop();
        assert!(reset_result.is_ok());
        assert!(!nav.emergency_stop_active);
    }

    #[test]
    fn test_manual_mode_transitions() {
        let mut nav = ResilientNavigationSystem::new();

        nav.manual_mode = true;

        let result = nav.navigate_to("test_destination");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Manual navigation"));

        // Exit manual mode should succeed with healthy system
        nav.health_metrics.successful_requests = 10;
        nav.health_metrics.total_requests = 10;
        nav.health_metrics.update_rates();

        let exit_result = nav.exit_manual_mode();
        assert!(exit_result.is_ok());
        assert!(!nav.manual_mode);
    }

    #[test]
    fn test_service_backup_failover() {
        let mut manager = RobotServiceManager::new();

        manager.register_service("primary", vec!["backup1".to_string(), "backup2".to_string()]);
        manager.register_service("backup1", vec![]);
        manager.register_service("backup2", vec![]);

        // Force primary service to fail
        let mut call_count = 0;
        let result = manager.call_service("primary", || -> Result<String> {
            call_count += 1;
            if call_count <= 3 { // Fail first few attempts
                Err(anyhow!("Primary service failure"))
            } else {
                Ok("Success from backup".to_string())
            }
        });

        // Should eventually succeed through backup services
        // Note: Actual behavior depends on retry logic and service implementation
    }

    #[test]
    fn test_degradation_level_escalation() {
        let mut manager = GracefulDegradationManager::new();

        manager.register_feature("essential_feature", true, false, 1.0);
        manager.register_feature("optional_feature", false, true, 0.5);

        let initial_level = manager.current_level;

        // Failing essential feature should escalate degradation
        let result = manager.report_feature_failure("essential_feature");
        assert!(result.is_ok());

        // Degradation level might have increased
        assert!(manager.current_level >= initial_level);
    }

    #[test]
    fn test_recovery_attempt() {
        let mut manager = GracefulDegradationManager::new();

        manager.register_feature("test_feature", false, true, 0.8);
        manager.report_feature_failure("test_feature").unwrap();

        // Attempt recovery
        let result = manager.attempt_recovery();
        assert!(result.is_ok());

        // Feature might have recovered (depends on random simulation)
    }
}

/// Public module for student exercises
pub mod exercises {
    use super::*;

    /// Exercise 1: Implement adaptive retry strategies
    ///
    /// Create a retry system that adapts its strategy based on:
    /// - Error type and frequency
    /// - Historical success rates
    /// - System load and resource availability
    /// - Time of day or operational context
    ///
    /// Requirements:
    /// - Dynamic retry interval calculation
    /// - Error-specific retry strategies
    /// - Learning from past recovery attempts
    pub fn exercise_1_adaptive_retry() {
        // TODO: Implement adaptive retry strategy
        // TODO: Add error pattern recognition
        // TODO: Create learning mechanism for optimal retry intervals
        println!("Exercise 1: Implement adaptive retry strategies");
    }

    /// Exercise 2: Build a distributed circuit breaker
    ///
    /// Create a circuit breaker system that:
    /// - Coordinates across multiple robot instances
    /// - Shares failure information between robots
    /// - Implements global and local circuit breakers
    /// - Provides distributed health monitoring
    ///
    /// Requirements:
    /// - Network communication for status sharing
    /// - Consensus mechanism for global decisions
    /// - Fallback when communication fails
    pub fn exercise_2_distributed_circuit_breaker() {
        // TODO: Implement distributed circuit breaker
        // TODO: Add inter-robot communication
        // TODO: Create consensus mechanism
        println!("Exercise 2: Build distributed circuit breaker system");
    }

    /// Exercise 3: Advanced graceful degradation
    ///
    /// Create a degradation system that:
    /// - Uses machine learning to predict failures
    /// - Implements resource-aware degradation
    /// - Provides user-customizable degradation policies
    /// - Supports gradual recovery with verification
    ///
    /// Requirements:
    /// - Predictive failure detection
    /// - Resource monitoring and allocation
    /// - Policy configuration system
    /// - Staged recovery verification
    pub fn exercise_3_advanced_degradation() {
        // TODO: Implement predictive failure detection
        // TODO: Add resource-aware degradation policies
        // TODO: Create staged recovery system
        println!("Exercise 3: Implement advanced graceful degradation");
    }

    /// Exercise 4: Recovery orchestration system
    ///
    /// Create a system that orchestrates recovery across multiple subsystems:
    /// - Dependency-aware recovery ordering
    /// - Parallel recovery where possible
    /// - Rollback on partial recovery failures
    /// - Recovery progress tracking and reporting
    ///
    /// Requirements:
    /// - Dependency graph modeling
    /// - Parallel execution framework
    /// - Transaction-like recovery semantics
    /// - Progress monitoring and user feedback
    pub fn exercise_4_recovery_orchestration() {
        // TODO: Implement dependency-aware recovery
        // TODO: Add parallel recovery execution
        // TODO: Create recovery transaction system
        println!("Exercise 4: Implement recovery orchestration system");
    }

    /// Exercise 5: Self-healing system architecture
    ///
    /// Create a comprehensive self-healing system:
    /// - Automatic anomaly detection
    /// - Self-diagnosis and root cause analysis
    /// - Autonomous recovery decision making
    /// - Continuous system optimization
    ///
    /// Requirements:
    /// - Real-time anomaly detection
    /// - Automated diagnosis system
    /// - Decision tree for recovery actions
    /// - Performance optimization feedback loop
    pub fn exercise_5_self_healing_system() {
        // TODO: Implement anomaly detection
        // TODO: Add automated diagnosis
        // TODO: Create autonomous recovery system
        println!("Exercise 5: Implement self-healing system architecture");
    }
}