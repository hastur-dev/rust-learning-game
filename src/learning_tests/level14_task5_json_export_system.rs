#[cfg(test)]
mod level14_task5_json_export_system_tests {
    use super::*;
    use serde::{Serialize, Deserialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct RobotConfig {
        id: u32,
        name: String,
        max_speed: f64,
        sensors_enabled: bool,
        position: (i32, i32),
    }

    impl RobotConfig {
        fn new(id: u32, name: String, max_speed: f64) -> Self {
            RobotConfig {
                id,
                name,
                max_speed,
                sensors_enabled: true,
                position: (0, 0),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct Mission {
        id: String,
        name: String,
        objectives: Vec<Objective>,
        estimated_duration: u32,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct Objective {
        description: String,
        priority: String,
        completed: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct RobotReport {
        timestamp: String,
        robot_info: RobotConfig,
        mission_data: Mission,
        performance_metrics: HashMap<String, serde_json::Value>,
        final_position: (i32, i32),
        success_rate: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct DetailedReport {
        report_id: String,
        generated_at: String,
        summary: ReportSummary,
        robots: Vec<RobotPerformance>,
        missions_completed: Vec<MissionResult>,
        system_health: SystemHealth,
        recommendations: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct ReportSummary {
        total_robots: u32,
        total_missions: u32,
        success_rate: f64,
        total_runtime_minutes: u32,
        items_collected: u32,
        doors_opened: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct RobotPerformance {
        robot_id: u32,
        robot_name: String,
        missions_assigned: u32,
        missions_completed: u32,
        efficiency_score: f64,
        energy_usage: f64,
        errors_encountered: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct MissionResult {
        mission_id: String,
        mission_name: String,
        assigned_robot: u32,
        start_time: String,
        end_time: String,
        status: String,
        objectives_completed: u32,
        total_objectives: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct SystemHealth {
        overall_status: String,
        cpu_usage: f64,
        memory_usage: f64,
        network_latency: f64,
        error_rate: f64,
        uptime_hours: u32,
    }

    fn create_sample_mission() -> Mission {
        let objectives = vec![
            Objective {
                description: "Collect all JSON data items".to_string(),
                priority: "high".to_string(),
                completed: true,
            },
            Objective {
                description: "Process sensor readings".to_string(),
                priority: "medium".to_string(),
                completed: true,
            },
            Objective {
                description: "Generate status report".to_string(),
                priority: "low".to_string(),
                completed: true,
            },
        ];

        Mission {
            id: "MISSION_001".to_string(),
            name: "Data Collection Protocol".to_string(),
            objectives,
            estimated_duration: 300,
        }
    }

    fn generate_final_report() -> Result<RobotReport, Box<dyn std::error::Error>> {
        let mut metrics = HashMap::new();
        metrics.insert("items_collected".to_string(),
                       serde_json::Value::Number(serde_json::Number::from(5)));
        metrics.insert("doors_opened".to_string(),
                       serde_json::Value::Number(serde_json::Number::from(4)));
        metrics.insert("energy_efficiency".to_string(),
                       serde_json::Value::Number(serde_json::Number::from_f64(0.85).unwrap()));
        metrics.insert("scan_accuracy".to_string(),
                       serde_json::Value::Number(serde_json::Number::from_f64(0.92).unwrap()));
        metrics.insert("path_optimization".to_string(),
                       serde_json::Value::Number(serde_json::Number::from_f64(0.78).unwrap()));

        let report = RobotReport {
            timestamp: "2024-01-15T10:30:00Z".to_string(),
            robot_info: RobotConfig::new(1, "JSON Processor".to_string(), 2.8),
            mission_data: create_sample_mission(),
            performance_metrics: metrics,
            final_position: (9, 7),
            success_rate: 0.95,
        };

        Ok(report)
    }

    fn generate_detailed_system_report() -> Result<DetailedReport, Box<dyn std::error::Error>> {
        let summary = ReportSummary {
            total_robots: 3,
            total_missions: 5,
            success_rate: 0.92,
            total_runtime_minutes: 150,
            items_collected: 15,
            doors_opened: 12,
        };

        let robots = vec![
            RobotPerformance {
                robot_id: 1,
                robot_name: "Scout Alpha".to_string(),
                missions_assigned: 2,
                missions_completed: 2,
                efficiency_score: 0.88,
                energy_usage: 0.75,
                errors_encountered: 1,
            },
            RobotPerformance {
                robot_id: 2,
                robot_name: "Heavy Beta".to_string(),
                missions_assigned: 2,
                missions_completed: 1,
                efficiency_score: 0.65,
                energy_usage: 0.92,
                errors_encountered: 3,
            },
            RobotPerformance {
                robot_id: 3,
                robot_name: "Speedy Gamma".to_string(),
                missions_assigned: 1,
                missions_completed: 1,
                efficiency_score: 0.95,
                energy_usage: 0.45,
                errors_encountered: 0,
            },
        ];

        let missions = vec![
            MissionResult {
                mission_id: "MISSION_001".to_string(),
                mission_name: "Data Collection".to_string(),
                assigned_robot: 1,
                start_time: "2024-01-15T09:00:00Z".to_string(),
                end_time: "2024-01-15T09:45:00Z".to_string(),
                status: "completed".to_string(),
                objectives_completed: 3,
                total_objectives: 3,
            },
            MissionResult {
                mission_id: "MISSION_002".to_string(),
                mission_name: "Area Exploration".to_string(),
                assigned_robot: 2,
                start_time: "2024-01-15T10:00:00Z".to_string(),
                end_time: "2024-01-15T10:30:00Z".to_string(),
                status: "failed".to_string(),
                objectives_completed: 1,
                total_objectives: 3,
            },
        ];

        let system_health = SystemHealth {
            overall_status: "healthy".to_string(),
            cpu_usage: 0.45,
            memory_usage: 0.67,
            network_latency: 15.2,
            error_rate: 0.02,
            uptime_hours: 72,
        };

        let recommendations = vec![
            "Increase energy efficiency monitoring for Heavy Beta".to_string(),
            "Consider additional training for error handling".to_string(),
            "Optimize path planning algorithms".to_string(),
            "Schedule maintenance for robots with high error counts".to_string(),
        ];

        let report = DetailedReport {
            report_id: "REPORT_20240115_001".to_string(),
            generated_at: "2024-01-15T11:00:00Z".to_string(),
            summary,
            robots,
            missions_completed: missions,
            system_health,
            recommendations,
        };

        Ok(report)
    }

    fn export_report_formats(report: &RobotReport) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let mut formats = Vec::new();

        // Pretty JSON format
        let pretty_json = serde_json::to_string_pretty(report)?;
        formats.push(("pretty".to_string(), pretty_json));

        // Compact JSON format
        let compact_json = serde_json::to_string(report)?;
        formats.push(("compact".to_string(), compact_json));

        // Minified JSON (same as compact for demonstration)
        formats.push(("minified".to_string(), compact_json.clone()));

        Ok(formats)
    }

    fn create_export_manifest() -> Result<String, serde_json::Error> {
        let mut manifest = HashMap::new();
        manifest.insert("export_version", serde_json::Value::String("1.0".to_string()));
        manifest.insert("created_at", serde_json::Value::String("2024-01-15T11:00:00Z".to_string()));
        manifest.insert("total_reports", serde_json::Value::Number(serde_json::Number::from(1)));
        manifest.insert("formats", serde_json::Value::Array(vec![
            serde_json::Value::String("pretty".to_string()),
            serde_json::Value::String("compact".to_string()),
            serde_json::Value::String("minified".to_string()),
        ]));

        let metadata = vec![
            ("file_count", 3),
            ("total_size_kb", 45),
            ("compression_ratio", 75),
        ];

        let metadata_map: HashMap<String, serde_json::Value> = metadata
            .into_iter()
            .map(|(k, v)| (k.to_string(), serde_json::Value::Number(serde_json::Number::from(v))))
            .collect();

        manifest.insert("metadata", serde_json::Value::Object(
            metadata_map.into_iter().collect()
        ));

        serde_json::to_string_pretty(&manifest)
    }

    #[test]
    fn test_basic_report_generation() {
        let result = generate_final_report();
        assert!(result.is_ok());

        let report = result.unwrap();
        assert_eq!(report.robot_info.name, "JSON Processor");
        assert_eq!(report.final_position, (9, 7));
        assert_eq!(report.success_rate, 0.95);
        assert_eq!(report.performance_metrics.len(), 5);
    }

    #[test]
    fn test_detailed_system_report() {
        let result = generate_detailed_system_report();
        assert!(result.is_ok());

        let report = result.unwrap();
        assert_eq!(report.summary.total_robots, 3);
        assert_eq!(report.summary.total_missions, 5);
        assert_eq!(report.robots.len(), 3);
        assert_eq!(report.missions_completed.len(), 2);
        assert_eq!(report.recommendations.len(), 4);
    }

    #[test]
    fn test_performance_metrics_serialization() {
        let report = generate_final_report().unwrap();

        assert!(report.performance_metrics.contains_key("items_collected"));
        assert!(report.performance_metrics.contains_key("energy_efficiency"));

        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("items_collected"));
        assert!(json.contains("energy_efficiency"));

        // Test round-trip
        let deserialized: RobotReport = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.performance_metrics.len(), 5);
    }

    #[test]
    fn test_mission_data_integration() {
        let report = generate_final_report().unwrap();
        let mission = &report.mission_data;

        assert_eq!(mission.id, "MISSION_001");
        assert_eq!(mission.objectives.len(), 3);

        // All objectives should be completed in the sample
        let completed_count = mission.objectives.iter().filter(|obj| obj.completed).count();
        assert_eq!(completed_count, 3);
    }

    #[test]
    fn test_export_formats() {
        let report = generate_final_report().unwrap();
        let formats = export_report_formats(&report).unwrap();

        assert_eq!(formats.len(), 3);
        assert_eq!(formats[0].0, "pretty");
        assert_eq!(formats[1].0, "compact");
        assert_eq!(formats[2].0, "minified");

        // Pretty format should be larger than compact
        assert!(formats[0].1.len() > formats[1].1.len());

        // All formats should contain the robot name
        for (_, json) in &formats {
            assert!(json.contains("JSON Processor"));
        }
    }

    #[test]
    fn test_robot_performance_analysis() {
        let report = generate_detailed_system_report().unwrap();

        let scout = &report.robots[0];
        assert_eq!(scout.robot_name, "Scout Alpha");
        assert_eq!(scout.missions_completed, 2);
        assert_eq!(scout.efficiency_score, 0.88);

        let heavy = &report.robots[1];
        assert_eq!(heavy.robot_name, "Heavy Beta");
        assert_eq!(heavy.missions_completed, 1);
        assert_eq!(heavy.errors_encountered, 3);

        let speedy = &report.robots[2];
        assert_eq!(speedy.robot_name, "Speedy Gamma");
        assert_eq!(speedy.errors_encountered, 0);
        assert_eq!(speedy.efficiency_score, 0.95);
    }

    #[test]
    fn test_mission_results_tracking() {
        let report = generate_detailed_system_report().unwrap();

        let completed_mission = &report.missions_completed[0];
        assert_eq!(completed_mission.status, "completed");
        assert_eq!(completed_mission.objectives_completed, 3);
        assert_eq!(completed_mission.total_objectives, 3);

        let failed_mission = &report.missions_completed[1];
        assert_eq!(failed_mission.status, "failed");
        assert_eq!(failed_mission.objectives_completed, 1);
        assert_eq!(failed_mission.total_objectives, 3);
    }

    #[test]
    fn test_system_health_monitoring() {
        let report = generate_detailed_system_report().unwrap();
        let health = &report.system_health;

        assert_eq!(health.overall_status, "healthy");
        assert_eq!(health.uptime_hours, 72);
        assert!(health.cpu_usage < 1.0);
        assert!(health.memory_usage < 1.0);
        assert!(health.error_rate < 0.1);
    }

    #[test]
    fn test_recommendations_generation() {
        let report = generate_detailed_system_report().unwrap();

        assert_eq!(report.recommendations.len(), 4);
        assert!(report.recommendations[0].contains("Heavy Beta"));
        assert!(report.recommendations[1].contains("error handling"));
        assert!(report.recommendations[3].contains("maintenance"));
    }

    #[test]
    fn test_export_manifest_creation() {
        let manifest = create_export_manifest().unwrap();

        assert!(manifest.contains("export_version"));
        assert!(manifest.contains("1.0"));
        assert!(manifest.contains("total_reports"));
        assert!(manifest.contains("pretty"));
        assert!(manifest.contains("compact"));

        // Parse back to verify structure
        let parsed: serde_json::Value = serde_json::from_str(&manifest).unwrap();
        assert_eq!(parsed["export_version"], "1.0");
        assert_eq!(parsed["total_reports"], 1);
    }

    #[test]
    fn test_comprehensive_json_export() {
        // Generate all report types
        let basic_report = generate_final_report().unwrap();
        let detailed_report = generate_detailed_system_report().unwrap();

        // Export in all formats
        let basic_formats = export_report_formats(&basic_report).unwrap();
        let detailed_json = serde_json::to_string_pretty(&detailed_report).unwrap();

        // Verify all exports work
        assert_eq!(basic_formats.len(), 3);
        assert!(detailed_json.len() > 0);

        // Test that all formats are valid JSON
        for (_, json) in &basic_formats {
            let parsed: serde_json::Value = serde_json::from_str(json).unwrap();
            assert!(parsed.is_object());
        }

        let parsed_detailed: serde_json::Value = serde_json::from_str(&detailed_json).unwrap();
        assert!(parsed_detailed.is_object());
    }

    #[test]
    fn test_json_size_optimization() {
        let report = generate_final_report().unwrap();
        let formats = export_report_formats(&report).unwrap();

        let pretty_size = formats[0].1.len();
        let compact_size = formats[1].1.len();

        // Compact should be significantly smaller
        assert!(compact_size < pretty_size);

        // Pretty format should have indentation
        assert!(formats[0].1.contains("  ")); // Contains spaces for indentation

        // Compact format should not have extra whitespace
        assert!(!formats[1].1.contains("  "));
    }

    #[test]
    fn test_timestamp_consistency() {
        let basic_report = generate_final_report().unwrap();
        let detailed_report = generate_detailed_system_report().unwrap();

        assert!(basic_report.timestamp.contains("2024-01-15"));
        assert!(detailed_report.generated_at.contains("2024-01-15"));

        // Test timestamp format (ISO 8601)
        assert!(basic_report.timestamp.contains("T"));
        assert!(basic_report.timestamp.contains("Z"));
    }

    #[test]
    fn test_data_consistency_across_reports() {
        let basic_report = generate_final_report().unwrap();
        let detailed_report = generate_detailed_system_report().unwrap();

        // Basic report should have data that aligns with detailed summary
        assert_eq!(basic_report.robot_info.id, 1);

        // Find robot 1 in detailed report
        let robot_1 = detailed_report.robots.iter()
            .find(|r| r.robot_id == 1)
            .unwrap();

        // Both should reference the same robot
        assert!(robot_1.robot_name.contains("Alpha")); // Different naming scheme is OK
    }

    #[test]
    fn test_error_handling_in_export() {
        // Test with invalid data that should still serialize
        let mut metrics = HashMap::new();
        metrics.insert("valid_metric".to_string(),
                       serde_json::Value::Number(serde_json::Number::from(42)));

        let report = RobotReport {
            timestamp: "invalid-timestamp".to_string(), // Still should serialize
            robot_info: RobotConfig::new(999, "Test Robot".to_string(), 1.0),
            mission_data: create_sample_mission(),
            performance_metrics: metrics,
            final_position: (-1, -1), // Negative positions should be allowed
            success_rate: 1.5, // Invalid rate should still serialize
        };

        let json = serde_json::to_string(&report);
        assert!(json.is_ok());

        // Should be able to round-trip even with unusual data
        let json_str = json.unwrap();
        let deserialized: Result<RobotReport, _> = serde_json::from_str(&json_str);
        assert!(deserialized.is_ok());
    }
}

// Example usage and reference implementation
fn main() {
    println!("Level 14 Task 5: Create JSON Export System");
    println!("Run with: cargo test level14_task5");
}