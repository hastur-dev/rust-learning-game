//! Level 21, Task 2: Option Type Handling
//!
//! This module demonstrates comprehensive usage of Rust's Option type for handling
//! nullable values in robot systems, showing how to work with optional data safely.
//!
//! Learning objectives:
//! - Master Option<T> for representing optional values
//! - Learn Option methods for transformation and control flow
//! - Understand the difference between Option and Result
//! - Handle missing data gracefully in robot operations
//! - Combine Option with other types effectively

use std::collections::HashMap;
use std::fmt;

/// Robot identification and metadata
#[derive(Debug, Clone, PartialEq)]
pub struct RobotId {
    pub id: u32,
    pub name: Option<String>,
    pub model: String,
    pub serial_number: Option<String>,
}

impl RobotId {
    pub fn new(id: u32, model: String) -> Self {
        Self {
            id,
            name: None,
            model,
            serial_number: None,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_serial(mut self, serial: String) -> Self {
        self.serial_number = Some(serial);
        self
    }

    pub fn display_name(&self) -> String {
        self.name.as_ref()
            .map(|name| format!("{} ({})", name, self.model))
            .unwrap_or_else(|| format!("Robot-{} ({})", self.id, self.model))
    }

    pub fn has_complete_info(&self) -> bool {
        self.name.is_some() && self.serial_number.is_some()
    }
}

/// GPS coordinates with optional altitude
#[derive(Debug, Clone, PartialEq)]
pub struct GpsCoordinate {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub accuracy: Option<f64>,
    pub timestamp: Option<u64>,
}

impl GpsCoordinate {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self {
            latitude: lat,
            longitude: lon,
            altitude: None,
            accuracy: None,
            timestamp: None,
        }
    }

    pub fn with_altitude(mut self, alt: f64) -> Self {
        self.altitude = Some(alt);
        self
    }

    pub fn with_accuracy(mut self, acc: f64) -> Self {
        self.accuracy = Some(acc);
        self
    }

    pub fn with_timestamp(mut self, ts: u64) -> Self {
        self.timestamp = Some(ts);
        self
    }

    pub fn is_3d(&self) -> bool {
        self.altitude.is_some()
    }

    pub fn distance_to(&self, other: &GpsCoordinate) -> f64 {
        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let delta_lat = (other.latitude - self.latitude).to_radians();
        let delta_lon = (other.longitude - self.longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2) +
                lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        let earth_radius = 6371.0; // Earth's radius in kilometers
        earth_radius * c
    }

    pub fn altitude_difference(&self, other: &GpsCoordinate) -> Option<f64> {
        match (self.altitude, other.altitude) {
            (Some(alt1), Some(alt2)) => Some((alt1 - alt2).abs()),
            _ => None,
        }
    }
}

/// Sensor data with optional calibration information
#[derive(Debug, Clone)]
pub struct SensorData {
    pub sensor_id: String,
    pub value: f64,
    pub unit: String,
    pub calibration_offset: Option<f64>,
    pub calibration_scale: Option<f64>,
    pub last_calibration: Option<u64>,
    pub confidence: Option<f64>,
}

impl SensorData {
    pub fn new(sensor_id: String, value: f64, unit: String) -> Self {
        Self {
            sensor_id,
            value,
            unit,
            calibration_offset: None,
            calibration_scale: None,
            last_calibration: None,
            confidence: None,
        }
    }

    pub fn with_calibration(mut self, offset: f64, scale: f64, timestamp: u64) -> Self {
        self.calibration_offset = Some(offset);
        self.calibration_scale = Some(scale);
        self.last_calibration = Some(timestamp);
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = Some(confidence);
        self
    }

    /// Get calibrated value if calibration data is available
    pub fn calibrated_value(&self) -> Option<f64> {
        match (self.calibration_offset, self.calibration_scale) {
            (Some(offset), Some(scale)) => Some((self.value + offset) * scale),
            _ => None,
        }
    }

    /// Get the best available value (calibrated if possible, raw otherwise)
    pub fn best_value(&self) -> f64 {
        self.calibrated_value().unwrap_or(self.value)
    }

    /// Check if calibration is recent (within specified seconds)
    pub fn is_calibration_recent(&self, current_time: u64, max_age_seconds: u64) -> Option<bool> {
        self.last_calibration.map(|cal_time| {
            current_time.saturating_sub(cal_time) <= max_age_seconds
        })
    }

    /// Get confidence level or default if not available
    pub fn confidence_level(&self) -> f64 {
        self.confidence.unwrap_or(0.5) // Default to 50% confidence
    }
}

/// Robot mission with optional parameters
#[derive(Debug, Clone)]
pub struct Mission {
    pub id: u32,
    pub name: String,
    pub target_location: GpsCoordinate,
    pub backup_location: Option<GpsCoordinate>,
    pub max_duration: Option<u64>, // seconds
    pub priority: Option<u8>,      // 1-10, higher is more important
    pub required_equipment: Vec<String>,
    pub optional_equipment: Vec<String>,
    pub assigned_robot: Option<u32>,
    pub estimated_completion: Option<u64>,
}

impl Mission {
    pub fn new(id: u32, name: String, target: GpsCoordinate) -> Self {
        Self {
            id,
            name,
            target_location: target,
            backup_location: None,
            max_duration: None,
            priority: None,
            required_equipment: Vec::new(),
            optional_equipment: Vec::new(),
            assigned_robot: None,
            estimated_completion: None,
        }
    }

    pub fn with_backup_location(mut self, backup: GpsCoordinate) -> Self {
        self.backup_location = Some(backup);
        self
    }

    pub fn with_duration(mut self, duration: u64) -> Self {
        self.max_duration = Some(duration);
        self
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority.min(10).max(1));
        self
    }

    pub fn assign_to_robot(mut self, robot_id: u32) -> Self {
        self.assigned_robot = Some(robot_id);
        self
    }

    pub fn get_priority(&self) -> u8 {
        self.priority.unwrap_or(5) // Default to medium priority
    }

    pub fn is_assigned(&self) -> bool {
        self.assigned_robot.is_some()
    }

    pub fn has_backup_plan(&self) -> bool {
        self.backup_location.is_some()
    }

    pub fn is_time_critical(&self) -> bool {
        self.max_duration.map_or(false, |duration| duration < 3600) // Less than 1 hour
    }

    /// Get effective target (primary or backup based on conditions)
    pub fn get_effective_target(&self, use_backup: bool) -> &GpsCoordinate {
        if use_backup {
            self.backup_location.as_ref().unwrap_or(&self.target_location)
        } else {
            &self.target_location
        }
    }
}

/// Robot registry for managing multiple robots
#[derive(Debug)]
pub struct RobotRegistry {
    robots: HashMap<u32, RobotInfo>,
    next_id: u32,
}

#[derive(Debug, Clone)]
struct RobotInfo {
    id: RobotId,
    current_location: Option<GpsCoordinate>,
    battery_level: Option<f64>,
    status: RobotStatus,
    capabilities: Vec<String>,
    current_mission: Option<u32>,
    last_contact: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
enum RobotStatus {
    Active,
    Idle,
    Charging,
    Maintenance,
    Offline,
    Unknown,
}

impl RobotRegistry {
    pub fn new() -> Self {
        Self {
            robots: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn register_robot(&mut self, model: String, name: Option<String>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let robot_id = if let Some(name) = name {
            RobotId::new(id, model).with_name(name)
        } else {
            RobotId::new(id, model)
        };

        let robot_info = RobotInfo {
            id: robot_id,
            current_location: None,
            battery_level: None,
            status: RobotStatus::Unknown,
            capabilities: Vec::new(),
            current_mission: None,
            last_contact: None,
        };

        self.robots.insert(id, robot_info);
        id
    }

    pub fn get_robot(&self, id: u32) -> Option<&RobotInfo> {
        self.robots.get(&id)
    }

    pub fn update_location(&mut self, id: u32, location: GpsCoordinate) -> Option<GpsCoordinate> {
        self.robots.get_mut(&id).map(|robot| {
            let old_location = robot.current_location.clone();
            robot.current_location = Some(location);
            old_location
        }).flatten()
    }

    pub fn update_battery(&mut self, id: u32, level: f64) -> bool {
        self.robots.get_mut(&id).map(|robot| {
            robot.battery_level = Some(level.max(0.0).min(100.0));
            true
        }).unwrap_or(false)
    }

    pub fn set_status(&mut self, id: u32, status: RobotStatus) -> Option<RobotStatus> {
        self.robots.get_mut(&id).map(|robot| {
            let old_status = robot.status.clone();
            robot.status = status;
            old_status
        })
    }

    pub fn assign_mission(&mut self, robot_id: u32, mission_id: u32) -> bool {
        self.robots.get_mut(&robot_id).map(|robot| {
            robot.current_mission = Some(mission_id);
            true
        }).unwrap_or(false)
    }

    pub fn clear_mission(&mut self, robot_id: u32) -> Option<u32> {
        self.robots.get_mut(&robot_id).and_then(|robot| {
            robot.current_mission.take()
        })
    }

    pub fn record_contact(&mut self, robot_id: u32, timestamp: u64) -> bool {
        self.robots.get_mut(&robot_id).map(|robot| {
            robot.last_contact = Some(timestamp);
            true
        }).unwrap_or(false)
    }

    /// Find robots by various criteria
    pub fn find_available_robots(&self) -> Vec<u32> {
        self.robots.iter()
            .filter(|(_, robot)| {
                matches!(robot.status, RobotStatus::Idle | RobotStatus::Active) &&
                robot.current_mission.is_none()
            })
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn find_robots_near(&self, location: &GpsCoordinate, max_distance: f64) -> Vec<u32> {
        self.robots.iter()
            .filter_map(|(id, robot)| {
                robot.current_location.as_ref().map(|pos| {
                    if pos.distance_to(location) <= max_distance {
                        Some(*id)
                    } else {
                        None
                    }
                }).flatten()
            })
            .collect()
    }

    pub fn find_robots_with_capability(&self, capability: &str) -> Vec<u32> {
        self.robots.iter()
            .filter(|(_, robot)| robot.capabilities.contains(&capability.to_string()))
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn find_low_battery_robots(&self, threshold: f64) -> Vec<u32> {
        self.robots.iter()
            .filter_map(|(id, robot)| {
                robot.battery_level.and_then(|level| {
                    if level <= threshold {
                        Some(*id)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn get_robot_summary(&self, id: u32) -> Option<String> {
        self.robots.get(&id).map(|robot| {
            let location_str = robot.current_location.as_ref()
                .map(|loc| format!("({:.4}, {:.4})", loc.latitude, loc.longitude))
                .unwrap_or_else(|| "Unknown".to_string());

            let battery_str = robot.battery_level
                .map(|level| format!("{:.1}%", level))
                .unwrap_or_else(|| "Unknown".to_string());

            let mission_str = robot.current_mission
                .map(|id| format!("Mission {}", id))
                .unwrap_or_else(|| "None".to_string());

            let contact_str = robot.last_contact
                .map(|ts| format!("Timestamp {}", ts))
                .unwrap_or_else(|| "Never".to_string());

            format!(
                "{}\nStatus: {:?}\nLocation: {}\nBattery: {}\nMission: {}\nLast Contact: {}",
                robot.id.display_name(),
                robot.status,
                location_str,
                battery_str,
                mission_str,
                contact_str
            )
        })
    }

    pub fn get_fleet_statistics(&self) -> FleetStatistics {
        let total_robots = self.robots.len();
        let active_robots = self.robots.values()
            .filter(|r| matches!(r.status, RobotStatus::Active))
            .count();

        let robots_with_location = self.robots.values()
            .filter(|r| r.current_location.is_some())
            .count();

        let average_battery = if total_robots > 0 {
            let battery_sum: f64 = self.robots.values()
                .filter_map(|r| r.battery_level)
                .sum();
            let battery_count = self.robots.values()
                .filter(|r| r.battery_level.is_some())
                .count();

            if battery_count > 0 {
                Some(battery_sum / battery_count as f64)
            } else {
                None
            }
        } else {
            None
        };

        let missions_assigned = self.robots.values()
            .filter(|r| r.current_mission.is_some())
            .count();

        FleetStatistics {
            total_robots,
            active_robots,
            robots_with_location,
            average_battery,
            missions_assigned,
        }
    }
}

#[derive(Debug)]
pub struct FleetStatistics {
    pub total_robots: usize,
    pub active_robots: usize,
    pub robots_with_location: usize,
    pub average_battery: Option<f64>,
    pub missions_assigned: usize,
}

impl fmt::Display for FleetStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let battery_str = self.average_battery
            .map(|avg| format!("{:.1}%", avg))
            .unwrap_or_else(|| "N/A".to_string());

        write!(f,
            "Fleet Statistics:\n\
             - Total Robots: {}\n\
             - Active Robots: {}\n\
             - Robots with Location: {}\n\
             - Average Battery: {}\n\
             - Missions Assigned: {}",
            self.total_robots,
            self.active_robots,
            self.robots_with_location,
            battery_str,
            self.missions_assigned
        )
    }
}

/// Utility functions for working with Options
pub mod option_utils {
    use super::*;

    /// Safely get the first element of a vector
    pub fn first<T>(vec: &[T]) -> Option<&T> {
        vec.first()
    }

    /// Safely get the last element of a vector
    pub fn last<T>(vec: &[T]) -> Option<&T> {
        vec.last()
    }

    /// Find element in a vector by predicate
    pub fn find<T, F>(vec: &[T], predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        vec.iter().find(|item| predicate(item))
    }

    /// Get maximum value from iterator of Options
    pub fn max_option<T: Ord>(iter: impl Iterator<Item = Option<T>>) -> Option<T> {
        iter.filter_map(|x| x).max()
    }

    /// Get minimum value from iterator of Options
    pub fn min_option<T: Ord>(iter: impl Iterator<Item = Option<T>>) -> Option<T> {
        iter.filter_map(|x| x).min()
    }

    /// Combine two Options with a function
    pub fn combine_options<T, U, R, F>(opt1: Option<T>, opt2: Option<U>, f: F) -> Option<R>
    where
        F: FnOnce(T, U) -> R,
    {
        opt1.and_then(|a| opt2.map(|b| f(a, b)))
    }

    /// Chain multiple Options together
    pub fn chain_options<T>(options: Vec<Option<T>>) -> Option<Vec<T>> {
        let mut results = Vec::new();
        for opt in options {
            match opt {
                Some(value) => results.push(value),
                None => return None,
            }
        }
        Some(results)
    }

    /// Get first Some value from a list of Options
    pub fn first_some<T>(options: Vec<Option<T>>) -> Option<T> {
        for opt in options {
            if opt.is_some() {
                return opt;
            }
        }
        None
    }

    /// Count Some values in an iterator of Options
    pub fn count_some<T>(iter: impl Iterator<Item = Option<T>>) -> usize {
        iter.filter(|opt| opt.is_some()).count()
    }

    /// Convert Option to Result with a default error
    pub fn option_to_result<T, E>(opt: Option<T>, error: E) -> Result<T, E> {
        opt.ok_or(error)
    }

    /// Safely parse string to number
    pub fn safe_parse<T: std::str::FromStr>(s: &str) -> Option<T> {
        s.parse().ok()
    }

    /// Safely access nested Options
    pub fn nested_option<T>(outer: Option<Option<T>>) -> Option<T> {
        outer.flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::option_utils::*;

    #[test]
    fn test_robot_id() {
        let robot = RobotId::new(1, "ModelX".to_string());
        assert_eq!(robot.display_name(), "Robot-1 (ModelX)");
        assert!(!robot.has_complete_info());

        let robot_with_name = robot.with_name("Alpha".to_string());
        assert_eq!(robot_with_name.display_name(), "Alpha (ModelX)");
        assert!(!robot_with_name.has_complete_info());

        let complete_robot = robot_with_name.with_serial("SN12345".to_string());
        assert!(complete_robot.has_complete_info());
    }

    #[test]
    fn test_gps_coordinate() {
        let coord1 = GpsCoordinate::new(40.7128, -74.0060); // New York
        let coord2 = GpsCoordinate::new(34.0522, -118.2437); // Los Angeles

        assert!(!coord1.is_3d());

        let coord1_3d = coord1.with_altitude(10.0);
        assert!(coord1_3d.is_3d());

        let distance = coord1.distance_to(&coord2);
        assert!(distance > 3900.0 && distance < 4000.0); // Approximately 3944 km

        // Test altitude difference
        let coord2_3d = coord2.with_altitude(100.0);
        let alt_diff = coord1_3d.altitude_difference(&coord2_3d);
        assert_eq!(alt_diff, Some(90.0));

        let alt_diff_none = coord1.altitude_difference(&coord2);
        assert_eq!(alt_diff_none, None);
    }

    #[test]
    fn test_sensor_data() {
        let sensor = SensorData::new("temp1".to_string(), 25.0, "°C".to_string());

        assert_eq!(sensor.best_value(), 25.0);
        assert_eq!(sensor.calibrated_value(), None);
        assert_eq!(sensor.confidence_level(), 0.5);

        let calibrated_sensor = sensor.with_calibration(2.0, 1.1, 1000)
                                     .with_confidence(0.9);

        assert_eq!(calibrated_sensor.calibrated_value(), Some(29.7)); // (25 + 2) * 1.1
        assert_eq!(calibrated_sensor.best_value(), 29.7);
        assert_eq!(calibrated_sensor.confidence_level(), 0.9);

        // Test calibration age
        assert_eq!(calibrated_sensor.is_calibration_recent(1500, 600), Some(true));
        assert_eq!(calibrated_sensor.is_calibration_recent(2000, 600), Some(false));
    }

    #[test]
    fn test_mission() {
        let target = GpsCoordinate::new(40.7128, -74.0060);
        let mission = Mission::new(1, "Patrol Area A".to_string(), target);

        assert_eq!(mission.get_priority(), 5); // Default priority
        assert!(!mission.is_assigned());
        assert!(!mission.has_backup_plan());
        assert!(!mission.is_time_critical());

        let mission = mission.with_priority(8)
                            .with_duration(1800) // 30 minutes
                            .assign_to_robot(123);

        assert_eq!(mission.get_priority(), 8);
        assert!(mission.is_assigned());
        assert!(mission.is_time_critical());

        let backup = GpsCoordinate::new(40.7589, -73.9851);
        let mission_with_backup = mission.with_backup_location(backup);
        assert!(mission_with_backup.has_backup_plan());

        // Test effective target
        let effective = mission_with_backup.get_effective_target(false);
        assert_eq!(effective.latitude, 40.7128);

        let effective_backup = mission_with_backup.get_effective_target(true);
        assert_eq!(effective_backup.latitude, 40.7589);
    }

    #[test]
    fn test_robot_registry() {
        let mut registry = RobotRegistry::new();

        // Register robots
        let robot1 = registry.register_robot("ModelA".to_string(), Some("Alpha".to_string()));
        let robot2 = registry.register_robot("ModelB".to_string(), None);

        assert_eq!(robot1, 1);
        assert_eq!(robot2, 2);

        // Update robot information
        let location = GpsCoordinate::new(40.7128, -74.0060);
        let old_location = registry.update_location(robot1, location.clone());
        assert_eq!(old_location, None);

        assert!(registry.update_battery(robot1, 85.0));
        assert!(registry.set_status(robot1, RobotStatus::Active).is_some());
        assert!(registry.record_contact(robot1, 1000));

        // Test robot retrieval
        let robot_info = registry.get_robot(robot1);
        assert!(robot_info.is_some());

        let robot_info = registry.get_robot(999);
        assert!(robot_info.is_none());
    }

    #[test]
    fn test_robot_registry_searches() {
        let mut registry = RobotRegistry::new();

        let robot1 = registry.register_robot("ModelA".to_string(), Some("Alpha".to_string()));
        let robot2 = registry.register_robot("ModelB".to_string(), Some("Beta".to_string()));

        // Set up robot states
        registry.set_status(robot1, RobotStatus::Idle);
        registry.set_status(robot2, RobotStatus::Active);

        registry.update_battery(robot1, 90.0);
        registry.update_battery(robot2, 15.0);

        let location1 = GpsCoordinate::new(40.7128, -74.0060);
        let location2 = GpsCoordinate::new(40.7589, -73.9851);
        registry.update_location(robot1, location1);
        registry.update_location(robot2, location2);

        // Test available robots
        let available = registry.find_available_robots();
        assert_eq!(available, vec![robot1, robot2]); // Both idle/active and no mission

        // Test robots near location
        let search_location = GpsCoordinate::new(40.7300, -74.0000);
        let nearby = registry.find_robots_near(&search_location, 10.0); // 10 km radius
        assert_eq!(nearby.len(), 2); // Both should be within 10km

        // Test low battery robots
        let low_battery = registry.find_low_battery_robots(20.0);
        assert_eq!(low_battery, vec![robot2]);
    }

    #[test]
    fn test_fleet_statistics() {
        let mut registry = RobotRegistry::new();

        let robot1 = registry.register_robot("ModelA".to_string(), Some("Alpha".to_string()));
        let robot2 = registry.register_robot("ModelB".to_string(), Some("Beta".to_string()));

        registry.set_status(robot1, RobotStatus::Active);
        registry.set_status(robot2, RobotStatus::Idle);

        registry.update_battery(robot1, 80.0);
        registry.update_battery(robot2, 60.0);

        let location = GpsCoordinate::new(40.7128, -74.0060);
        registry.update_location(robot1, location);

        registry.assign_mission(robot1, 101);

        let stats = registry.get_fleet_statistics();
        assert_eq!(stats.total_robots, 2);
        assert_eq!(stats.active_robots, 1);
        assert_eq!(stats.robots_with_location, 1);
        assert_eq!(stats.missions_assigned, 1);
        assert!(stats.average_battery.is_some());
        assert_eq!(stats.average_battery.unwrap(), 70.0);
    }

    #[test]
    fn test_option_utils() {
        // Test first and last
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(first(&vec), Some(&1));
        assert_eq!(last(&vec), Some(&5));

        let empty_vec: Vec<i32> = vec![];
        assert_eq!(first(&empty_vec), None);
        assert_eq!(last(&empty_vec), None);

        // Test find
        let found = find(&vec, |&x| x > 3);
        assert_eq!(found, Some(&4));

        let not_found = find(&vec, |&x| x > 10);
        assert_eq!(not_found, None);

        // Test max_option and min_option
        let options = vec![Some(5), None, Some(2), Some(8), None];
        assert_eq!(max_option(options.iter().cloned()), Some(8));
        assert_eq!(min_option(options.iter().cloned()), Some(2));

        // Test combine_options
        let combined = combine_options(Some(5), Some(3), |a, b| a + b);
        assert_eq!(combined, Some(8));

        let combined_none = combine_options(Some(5), None, |a, b| a + b);
        assert_eq!(combined_none, None);

        // Test chain_options
        let all_some = vec![Some(1), Some(2), Some(3)];
        let chained = chain_options(all_some);
        assert_eq!(chained, Some(vec![1, 2, 3]));

        let with_none = vec![Some(1), None, Some(3)];
        let chained_none = chain_options(with_none);
        assert_eq!(chained_none, None);

        // Test first_some
        let options = vec![None, None, Some(42), Some(100)];
        assert_eq!(first_some(options), Some(42));

        let all_none: Vec<Option<i32>> = vec![None, None, None];
        assert_eq!(first_some(all_none), None);

        // Test count_some
        let options = vec![Some(1), None, Some(2), None, Some(3)];
        assert_eq!(count_some(options.iter().cloned()), 3);

        // Test safe_parse
        assert_eq!(safe_parse::<i32>("42"), Some(42));
        assert_eq!(safe_parse::<i32>("abc"), None);
        assert_eq!(safe_parse::<f64>("3.14"), Some(3.14));

        // Test nested_option
        let nested = Some(Some(42));
        assert_eq!(nested_option(nested), Some(42));

        let nested_none = Some(None);
        assert_eq!(nested_option(nested_none), None);

        let outer_none: Option<Option<i32>> = None;
        assert_eq!(nested_option(outer_none), None);
    }

    #[test]
    fn test_option_chaining() {
        let mut registry = RobotRegistry::new();
        let robot_id = registry.register_robot("ModelA".to_string(), Some("Test".to_string()));

        // Test chaining Option operations
        let battery_info = registry.get_robot(robot_id)
            .and_then(|robot| robot.battery_level)
            .map(|level| if level < 20.0 { "Low" } else { "OK" });

        assert_eq!(battery_info, None); // No battery level set yet

        registry.update_battery(robot_id, 15.0);

        let battery_info = registry.get_robot(robot_id)
            .and_then(|robot| robot.battery_level)
            .map(|level| if level < 20.0 { "Low" } else { "OK" });

        assert_eq!(battery_info, Some("Low"));
    }

    #[test]
    fn test_option_or_patterns() {
        let sensor = SensorData::new("test".to_string(), 25.0, "°C".to_string());

        // Test or_else patterns
        let confidence = sensor.confidence.or(Some(0.8));
        assert_eq!(confidence, Some(0.8));

        let value = sensor.calibrated_value().or(Some(sensor.value));
        assert_eq!(value, Some(25.0));

        // Test unwrap_or patterns
        let default_confidence = sensor.confidence.unwrap_or(0.5);
        assert_eq!(default_confidence, 0.5);
    }

    #[test]
    fn test_option_filter() {
        let sensor = SensorData::new("test".to_string(), 95.0, "°C".to_string())
                                .with_confidence(0.3);

        // Filter based on confidence
        let high_confidence_value = sensor.confidence
            .filter(|&conf| conf > 0.5)
            .map(|_| sensor.value);

        assert_eq!(high_confidence_value, None);

        let sensor_high_conf = sensor.with_confidence(0.9);
        let high_confidence_value = sensor_high_conf.confidence
            .filter(|&conf| conf > 0.5)
            .map(|_| sensor_high_conf.value);

        assert_eq!(high_confidence_value, Some(95.0));
    }
}

/// Public module for student exercises
pub mod exercises {
    use super::*;

    /// Exercise 1: Implement a robot patrol route optimizer
    ///
    /// Create a system that optimizes patrol routes for robots:
    /// - Routes have optional waypoints and time constraints
    /// - Some robots may not have GPS capabilities (optional coordinates)
    /// - Optimize based on available robot capabilities and current locations
    /// - Handle missing data gracefully with reasonable defaults
    ///
    /// Requirements:
    /// - Use Option types for optional route parameters
    /// - Handle robots with missing location or capability data
    /// - Implement fallback strategies when optimal solutions aren't available
    pub fn exercise_1_patrol_optimizer() {
        // TODO: Implement PatrolRoute struct with optional waypoints
        // TODO: Create RouteOptimizer that handles missing robot data
        // TODO: Add optimization algorithms that work with partial information
        println!("Exercise 1: Implement patrol route optimizer with Option handling");
    }

    /// Exercise 2: Robot capability matching system
    ///
    /// Create a system that matches missions to robots based on capabilities:
    /// - Missions have required and optional capabilities
    /// - Robots have varying levels of capability information
    /// - Some capability data may be missing or outdated
    /// - Implement scoring system for capability matching
    ///
    /// Requirements:
    /// - Use Option types for optional capabilities and metadata
    /// - Handle missing capability information gracefully
    /// - Implement partial matching when full requirements can't be met
    pub fn exercise_2_capability_matching() {
        // TODO: Implement CapabilityMatcher struct
        // TODO: Add scoring system for partial capability matches
        // TODO: Handle missing or incomplete capability data
        println!("Exercise 2: Implement capability matching with Option handling");
    }

    /// Exercise 3: Sensor data fusion with missing readings
    ///
    /// Create a sensor fusion system that handles missing sensor data:
    /// - Multiple sensors may provide overlapping measurements
    /// - Some sensors may be offline or providing invalid data
    /// - Implement confidence-weighted fusion algorithms
    /// - Provide fallback readings when primary sensors fail
    ///
    /// Requirements:
    /// - Use Option types for sensor availability and confidence
    /// - Implement fusion algorithms that work with partial data
    /// - Handle temporal aspects of missing data
    pub fn exercise_3_sensor_fusion_with_missing_data() {
        // TODO: Implement SensorFusion struct with Option handling
        // TODO: Add confidence-weighted averaging algorithms
        // TODO: Create fallback strategies for missing sensors
        println!("Exercise 3: Implement sensor fusion with missing data handling");
    }

    /// Exercise 4: Dynamic mission scheduling
    ///
    /// Create a mission scheduler that handles optional mission parameters:
    /// - Missions may have optional deadlines, priorities, and resources
    /// - Robot availability may change dynamically
    /// - Handle rescheduling when robots become unavailable
    /// - Implement partial mission execution when resources are limited
    ///
    /// Requirements:
    /// - Use Option types for optional mission parameters
    /// - Handle dynamic changes in robot availability
    /// - Implement graceful degradation for partial mission execution
    pub fn exercise_4_dynamic_mission_scheduling() {
        // TODO: Implement DynamicScheduler struct
        // TODO: Add algorithms for handling optional mission parameters
        // TODO: Create rescheduling logic for dynamic availability changes
        println!("Exercise 4: Implement dynamic mission scheduling with Option handling");
    }

    /// Exercise 5: Robot fleet health monitoring
    ///
    /// Create a comprehensive fleet health monitoring system:
    /// - Robots may have various levels of health data available
    /// - Some health metrics may be missing or outdated
    /// - Implement predictive health analysis with incomplete data
    /// - Generate health reports with confidence indicators
    ///
    /// Requirements:
    /// - Use Option types for optional health metrics and timestamps
    /// - Implement health prediction with missing data
    /// - Create comprehensive reporting with data availability indicators
    pub fn exercise_5_fleet_health_monitoring() {
        // TODO: Implement FleetHealthMonitor struct
        // TODO: Add predictive health analysis with Option handling
        // TODO: Create comprehensive health reporting system
        println!("Exercise 5: Implement fleet health monitoring with Option handling");
    }
}