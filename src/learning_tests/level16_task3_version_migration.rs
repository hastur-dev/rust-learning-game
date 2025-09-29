#[cfg(test)]
mod level16_task3_version_migration_tests {
    use super::*;
    use serde::{Serialize, Deserialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(tag = "version")]
    enum VersionedConfig {
        #[serde(rename = "1.0")]
        V1(ConfigV1),
        #[serde(rename = "2.0")]
        V2(ConfigV2),
        #[serde(rename = "3.0")]
        V3(ConfigV3),
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct ConfigV1 {
        name: String,
        speed: f64,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct ConfigV2 {
        name: String,
        max_speed: f64,
        acceleration: f64,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct ConfigV3 {
        metadata: ConfigMetadata,
        performance: PerformanceConfig,
        features: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct ConfigMetadata {
        name: String,
        version: String,
        migrated_from: Option<String>,
        created_at: String,
        updated_at: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct PerformanceConfig {
        max_speed: f64,
        acceleration: f64,
        efficiency_mode: bool,
        advanced_settings: HashMap<String, f64>,
    }

    #[derive(Debug)]
    enum MigrationError {
        UnsupportedVersion(String),
        DataCorruption(String),
        MigrationFailed(String),
    }

    impl std::fmt::Display for MigrationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MigrationError::UnsupportedVersion(v) => write!(f, "Unsupported version: {}", v),
                MigrationError::DataCorruption(msg) => write!(f, "Data corruption: {}", msg),
                MigrationError::MigrationFailed(msg) => write!(f, "Migration failed: {}", msg),
            }
        }
    }

    impl std::error::Error for MigrationError {}

    impl VersionedConfig {
        fn migrate_to_latest(self) -> Result<ConfigV3, MigrationError> {
            match self {
                VersionedConfig::V1(v1) => {
                    println!("Migrating from v1.0 to v3.0");

                    // Validate V1 data
                    if v1.name.is_empty() {
                        return Err(MigrationError::DataCorruption("Empty name in V1 config".to_string()));
                    }
                    if v1.speed < 0.0 {
                        return Err(MigrationError::DataCorruption("Negative speed in V1 config".to_string()));
                    }

                    let mut advanced_settings = HashMap::new();
                    advanced_settings.insert("legacy_compatibility".to_string(), 1.0);
                    advanced_settings.insert("migration_source".to_string(), 1.0);

                    Ok(ConfigV3 {
                        metadata: ConfigMetadata {
                            name: v1.name,
                            version: "3.0".to_string(),
                            migrated_from: Some("1.0".to_string()),
                            created_at: "1970-01-01T00:00:00Z".to_string(), // Unknown original date
                            updated_at: "2024-01-15T10:00:00Z".to_string(),
                        },
                        performance: PerformanceConfig {
                            max_speed: v1.speed,
                            acceleration: 1.0, // Default value for legacy configs
                            efficiency_mode: false,
                            advanced_settings,
                        },
                        features: vec!["basic_movement".to_string(), "legacy_support".to_string()],
                    })
                }
                VersionedConfig::V2(v2) => {
                    println!("Migrating from v2.0 to v3.0");

                    // Validate V2 data
                    if v2.name.is_empty() {
                        return Err(MigrationError::DataCorruption("Empty name in V2 config".to_string()));
                    }
                    if v2.max_speed < 0.0 || v2.acceleration < 0.0 {
                        return Err(MigrationError::DataCorruption("Negative values in V2 config".to_string()));
                    }

                    let mut advanced_settings = HashMap::new();
                    advanced_settings.insert("v2_compatibility".to_string(), 1.0);
                    advanced_settings.insert("acceleration_factor".to_string(), v2.acceleration / v2.max_speed);

                    Ok(ConfigV3 {
                        metadata: ConfigMetadata {
                            name: v2.name,
                            version: "3.0".to_string(),
                            migrated_from: Some("2.0".to_string()),
                            created_at: "2023-01-01T00:00:00Z".to_string(), // Estimated date
                            updated_at: "2024-01-15T10:00:00Z".to_string(),
                        },
                        performance: PerformanceConfig {
                            max_speed: v2.max_speed,
                            acceleration: v2.acceleration,
                            efficiency_mode: false, // Not available in V2
                            advanced_settings,
                        },
                        features: vec![
                            "basic_movement".to_string(),
                            "acceleration_control".to_string(),
                            "v2_migration".to_string(),
                        ],
                    })
                }
                VersionedConfig::V3(v3) => {
                    println!("Already at latest version (3.0)");
                    Ok(v3)
                }
            }
        }

        fn get_version(&self) -> &str {
            match self {
                VersionedConfig::V1(_) => "1.0",
                VersionedConfig::V2(_) => "2.0",
                VersionedConfig::V3(_) => "3.0",
            }
        }

        fn validate_data(&self) -> Result<(), MigrationError> {
            match self {
                VersionedConfig::V1(v1) => {
                    if v1.name.len() > 100 {
                        return Err(MigrationError::DataCorruption("Name too long in V1".to_string()));
                    }
                    if v1.speed > 1000.0 {
                        return Err(MigrationError::DataCorruption("Speed too high in V1".to_string()));
                    }
                }
                VersionedConfig::V2(v2) => {
                    if v2.name.len() > 100 {
                        return Err(MigrationError::DataCorruption("Name too long in V2".to_string()));
                    }
                    if v2.max_speed > 1000.0 || v2.acceleration > 100.0 {
                        return Err(MigrationError::DataCorruption("Performance values too high in V2".to_string()));
                    }
                }
                VersionedConfig::V3(v3) => {
                    if v3.metadata.name.len() > 100 {
                        return Err(MigrationError::DataCorruption("Name too long in V3".to_string()));
                    }
                    if v3.performance.max_speed > 1000.0 {
                        return Err(MigrationError::DataCorruption("Max speed too high in V3".to_string()));
                    }
                }
            }
            Ok(())
        }
    }

    fn process_versioned_config() -> Result<ConfigV3, Box<dyn std::error::Error>> {
        let config_data = r#"
            {
                "version": "1.0",
                "name": "Legacy Robot",
                "speed": 2.5
            }
        "#;

        let versioned: VersionedConfig = serde_json::from_str(config_data)?;

        // Validate before migration
        versioned.validate_data()?;

        let latest = versioned.migrate_to_latest()?;

        println!("Migration successful: {:?}", latest);
        Ok(latest)
    }

    fn process_multiple_versions() -> Result<Vec<ConfigV3>, Box<dyn std::error::Error>> {
        let configs = vec![
            r#"{"version": "1.0", "name": "Old Robot", "speed": 1.5}"#,
            r#"{"version": "2.0", "name": "Modern Robot", "max_speed": 3.0, "acceleration": 2.0}"#,
            r#"{"version": "3.0", "metadata": {"name": "Latest Robot", "version": "3.0", "migrated_from": null, "created_at": "2024-01-15T10:00:00Z", "updated_at": "2024-01-15T10:00:00Z"}, "performance": {"max_speed": 4.0, "acceleration": 2.5, "efficiency_mode": true, "advanced_settings": {}}, "features": ["basic_movement", "advanced_ai"]}"#,
        ];

        let mut results = Vec::new();

        for (i, config_str) in configs.iter().enumerate() {
            println!("Processing config {}: ", i + 1);

            match serde_json::from_str::<VersionedConfig>(config_str) {
                Ok(versioned) => {
                    println!("  Detected version: {}", versioned.get_version());

                    match versioned.validate_data() {
                        Ok(_) => {
                            match versioned.migrate_to_latest() {
                                Ok(latest) => {
                                    println!("  ✓ Successfully migrated to v3.0");
                                    results.push(latest);
                                }
                                Err(e) => {
                                    println!("  ✗ Migration failed: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("  ✗ Validation failed: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("  ✗ Parse failed: {}", e);
                }
            }
        }

        Ok(results)
    }

    fn handle_migration_data() -> Result<Vec<ConfigV3>, Box<dyn std::error::Error>> {
        // Simulate a batch of mixed version configs
        let migration_batch = vec![
            ConfigV1 { name: "Robot Alpha".to_string(), speed: 2.0 },
            ConfigV1 { name: "Robot Beta".to_string(), speed: 1.8 },
        ];

        let mut migrated_configs = Vec::new();

        for (i, old_config) in migration_batch.into_iter().enumerate() {
            println!("Migrating robot {} from V1 format", i + 1);

            let versioned = VersionedConfig::V1(old_config);

            match versioned.migrate_to_latest() {
                Ok(v3_config) => {
                    println!("  ✓ Migration successful");
                    migrated_configs.push(v3_config);
                }
                Err(e) => {
                    println!("  ✗ Migration failed: {}", e);
                }
            }
        }

        // Also migrate some V2 configs
        let v2_batch = vec![
            ConfigV2 {
                name: "Robot Gamma".to_string(),
                max_speed: 3.5,
                acceleration: 2.2,
            },
            ConfigV2 {
                name: "Robot Delta".to_string(),
                max_speed: 4.0,
                acceleration: 3.0,
            },
        ];

        for (i, v2_config) in v2_batch.into_iter().enumerate() {
            println!("Migrating robot {} from V2 format", i + 3);

            let versioned = VersionedConfig::V2(v2_config);

            match versioned.migrate_to_latest() {
                Ok(v3_config) => {
                    println!("  ✓ Migration successful");
                    migrated_configs.push(v3_config);
                }
                Err(e) => {
                    println!("  ✗ Migration failed: {}", e);
                }
            }
        }

        Ok(migrated_configs)
    }

    fn test_backward_compatibility() -> Result<(), Box<dyn std::error::Error>> {
        // Test that we can still read old formats
        let v1_json = r#"{"version": "1.0", "name": "Test Robot", "speed": 2.5}"#;
        let v2_json = r#"{"version": "2.0", "name": "Test Robot", "max_speed": 3.0, "acceleration": 1.5}"#;

        // Parse V1
        let v1_config: VersionedConfig = serde_json::from_str(v1_json)?;
        assert_eq!(v1_config.get_version(), "1.0");

        // Parse V2
        let v2_config: VersionedConfig = serde_json::from_str(v2_json)?;
        assert_eq!(v2_config.get_version(), "2.0");

        // Migrate both to V3
        let v1_migrated = v1_config.migrate_to_latest()?;
        let v2_migrated = v2_config.migrate_to_latest()?;

        assert_eq!(v1_migrated.metadata.version, "3.0");
        assert_eq!(v2_migrated.metadata.version, "3.0");

        println!("Backward compatibility test passed");
        Ok(())
    }

    #[test]
    fn test_version_detection() {
        let configs = vec![
            (r#"{"version": "1.0", "name": "Test", "speed": 1.0}"#, "1.0"),
            (r#"{"version": "2.0", "name": "Test", "max_speed": 1.0, "acceleration": 0.5}"#, "2.0"),
        ];

        for (config_str, expected_version) in configs {
            let versioned: VersionedConfig = serde_json::from_str(config_str).unwrap();
            assert_eq!(versioned.get_version(), expected_version);
        }
    }

    #[test]
    fn test_v1_to_v3_migration() {
        let v1 = ConfigV1 {
            name: "Test Robot".to_string(),
            speed: 2.5,
        };

        let versioned = VersionedConfig::V1(v1);
        let v3 = versioned.migrate_to_latest().unwrap();

        assert_eq!(v3.metadata.name, "Test Robot");
        assert_eq!(v3.metadata.version, "3.0");
        assert_eq!(v3.metadata.migrated_from, Some("1.0".to_string()));
        assert_eq!(v3.performance.max_speed, 2.5);
        assert_eq!(v3.performance.acceleration, 1.0); // Default
        assert!(v3.features.contains(&"basic_movement".to_string()));
        assert!(v3.features.contains(&"legacy_support".to_string()));
    }

    #[test]
    fn test_v2_to_v3_migration() {
        let v2 = ConfigV2 {
            name: "Advanced Robot".to_string(),
            max_speed: 4.0,
            acceleration: 2.0,
        };

        let versioned = VersionedConfig::V2(v2);
        let v3 = versioned.migrate_to_latest().unwrap();

        assert_eq!(v3.metadata.name, "Advanced Robot");
        assert_eq!(v3.metadata.version, "3.0");
        assert_eq!(v3.metadata.migrated_from, Some("2.0".to_string()));
        assert_eq!(v3.performance.max_speed, 4.0);
        assert_eq!(v3.performance.acceleration, 2.0);
        assert!(v3.features.contains(&"acceleration_control".to_string()));
    }

    #[test]
    fn test_v3_no_migration() {
        let v3 = ConfigV3 {
            metadata: ConfigMetadata {
                name: "Current Robot".to_string(),
                version: "3.0".to_string(),
                migrated_from: None,
                created_at: "2024-01-15T10:00:00Z".to_string(),
                updated_at: "2024-01-15T10:00:00Z".to_string(),
            },
            performance: PerformanceConfig {
                max_speed: 5.0,
                acceleration: 3.0,
                efficiency_mode: true,
                advanced_settings: HashMap::new(),
            },
            features: vec!["advanced_ai".to_string()],
        };

        let original_v3 = v3.clone();
        let versioned = VersionedConfig::V3(v3);
        let result_v3 = versioned.migrate_to_latest().unwrap();

        assert_eq!(result_v3, original_v3);
    }

    #[test]
    fn test_data_validation() {
        // Test valid V1
        let valid_v1 = VersionedConfig::V1(ConfigV1 {
            name: "Valid Robot".to_string(),
            speed: 2.5,
        });
        assert!(valid_v1.validate_data().is_ok());

        // Test invalid V1 - negative speed
        let invalid_v1 = VersionedConfig::V1(ConfigV1 {
            name: "Invalid Robot".to_string(),
            speed: -1.0,
        });
        assert!(invalid_v1.validate_data().is_err());

        // Test invalid V1 - empty name
        let invalid_v1_empty = VersionedConfig::V1(ConfigV1 {
            name: "".to_string(),
            speed: 2.5,
        });
        match invalid_v1_empty.migrate_to_latest() {
            Err(MigrationError::DataCorruption(_)) => {}, // Expected
            _ => panic!("Expected data corruption error"),
        }
    }

    #[test]
    fn test_migration_errors() {
        // Test corrupted V1 data
        let corrupted_v1 = ConfigV1 {
            name: "".to_string(), // Empty name
            speed: 2.5,
        };

        let versioned = VersionedConfig::V1(corrupted_v1);
        let result = versioned.migrate_to_latest();
        assert!(result.is_err());

        if let Err(MigrationError::DataCorruption(msg)) = result {
            assert!(msg.contains("Empty name"));
        }

        // Test corrupted V2 data
        let corrupted_v2 = ConfigV2 {
            name: "Test".to_string(),
            max_speed: -1.0, // Negative speed
            acceleration: 2.0,
        };

        let versioned = VersionedConfig::V2(corrupted_v2);
        let result = versioned.migrate_to_latest();
        assert!(result.is_err());
    }

    #[test]
    fn test_round_trip_serialization() {
        let original_configs = vec![
            VersionedConfig::V1(ConfigV1 {
                name: "Robot 1".to_string(),
                speed: 1.5,
            }),
            VersionedConfig::V2(ConfigV2 {
                name: "Robot 2".to_string(),
                max_speed: 3.0,
                acceleration: 2.0,
            }),
        ];

        for config in original_configs {
            // Serialize to JSON
            let json = serde_json::to_string(&config).unwrap();

            // Deserialize back
            let deserialized: VersionedConfig = serde_json::from_str(&json).unwrap();

            // Should be the same version
            assert_eq!(config.get_version(), deserialized.get_version());
        }
    }

    #[test]
    fn test_batch_migration() {
        let result = handle_migration_data();
        assert!(result.is_ok());

        let migrated = result.unwrap();
        assert_eq!(migrated.len(), 4); // 2 V1 + 2 V2 configs

        for config in &migrated {
            assert_eq!(config.metadata.version, "3.0");
            assert!(config.metadata.migrated_from.is_some());
        }
    }

    #[test]
    fn test_advanced_settings_migration() {
        let v1 = ConfigV1 {
            name: "Settings Test".to_string(),
            speed: 3.0,
        };

        let versioned = VersionedConfig::V1(v1);
        let v3 = versioned.migrate_to_latest().unwrap();

        // Check that advanced settings were populated
        assert!(!v3.performance.advanced_settings.is_empty());
        assert!(v3.performance.advanced_settings.contains_key("legacy_compatibility"));
        assert!(v3.performance.advanced_settings.contains_key("migration_source"));
    }

    #[test]
    fn test_backward_compatibility_processing() {
        let result = test_backward_compatibility();
        assert!(result.is_ok());
    }

    #[test]
    fn test_version_specific_features() {
        // V1 migration should add legacy features
        let v1 = VersionedConfig::V1(ConfigV1 {
            name: "V1 Robot".to_string(),
            speed: 2.0,
        });
        let v3_from_v1 = v1.migrate_to_latest().unwrap();
        assert!(v3_from_v1.features.contains(&"legacy_support".to_string()));

        // V2 migration should add acceleration features
        let v2 = VersionedConfig::V2(ConfigV2 {
            name: "V2 Robot".to_string(),
            max_speed: 3.0,
            acceleration: 1.5,
        });
        let v3_from_v2 = v2.migrate_to_latest().unwrap();
        assert!(v3_from_v2.features.contains(&"acceleration_control".to_string()));
        assert!(v3_from_v2.features.contains(&"v2_migration".to_string()));
    }

    #[test]
    fn test_multiple_version_processing() {
        let result = process_multiple_versions();
        assert!(result.is_ok());

        let configs = result.unwrap();
        assert_eq!(configs.len(), 3); // All three should migrate successfully

        for config in &configs {
            assert_eq!(config.metadata.version, "3.0");
        }
    }
}

// Example usage and reference implementation
fn main() {
    println!("Level 16 Task 3: Handle Version Migration and Backward Compatibility");
    println!("Run with: cargo test level16_task3");
}