// Level 19 Task 1: Basic Bitwise Operations
// Learn fundamental bitwise operators through robot sensor flag manipulation

use std::fmt;

/// Robot sensor flags - each bit represents a different sensor or system status
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RobotSensorFlags(pub u8);

impl RobotSensorFlags {
    // Flag bit positions
    pub const BATTERY_LOW: u8 = 7;      // 0b10000000
    pub const MOTOR_ERROR: u8 = 6;      // 0b01000000
    pub const GPS_ENABLED: u8 = 5;      // 0b00100000
    pub const WIFI_CONNECTED: u8 = 4;   // 0b00010000
    pub const OBSTACLE_DETECTED: u8 = 3; // 0b00001000
    pub const CAMERA_ACTIVE: u8 = 2;    // 0b00000100
    pub const LIDAR_SCANNING: u8 = 1;   // 0b00000010
    pub const SYSTEM_READY: u8 = 0;     // 0b00000001

    /// Create new sensor flags with all systems off
    pub fn new() -> Self {
        Self(0)
    }

    /// Create sensor flags from raw value
    pub fn from_raw(value: u8) -> Self {
        Self(value)
    }

    /// Get raw flag value
    pub fn raw(&self) -> u8 {
        self.0
    }

    /// Check if a specific flag is set (AND operation)
    pub fn is_flag_set(&self, bit_position: u8) -> bool {
        (self.0 & (1 << bit_position)) != 0
    }

    /// Set a specific flag (OR operation)
    pub fn set_flag(&mut self, bit_position: u8) {
        self.0 |= 1 << bit_position;
    }

    /// Clear a specific flag (AND with NOT operation)
    pub fn clear_flag(&mut self, bit_position: u8) {
        self.0 &= !(1 << bit_position);
    }

    /// Toggle a specific flag (XOR operation)
    pub fn toggle_flag(&mut self, bit_position: u8) {
        self.0 ^= 1 << bit_position;
    }

    /// Get inverted flags (NOT operation)
    pub fn inverted(&self) -> Self {
        Self(!self.0)
    }

    /// Check if battery is low
    pub fn battery_low(&self) -> bool {
        self.is_flag_set(Self::BATTERY_LOW)
    }

    /// Check if any critical errors exist
    pub fn has_critical_errors(&self) -> bool {
        // Check battery low OR motor error
        (self.0 & 0b11000000) != 0
    }

    /// Get count of active sensors
    pub fn active_sensor_count(&self) -> u32 {
        self.0.count_ones()
    }
}

impl fmt::Display for RobotSensorFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SensorFlags(0b{:08b})", self.0)
    }
}

/// Demonstrate all basic bitwise operations
pub fn demonstrate_bitwise_operations() {
    println!("=== Robot Sensor Flag Management ===");

    // Start with some initial flags
    let mut flags = RobotSensorFlags::from_raw(0b10110100);
    println!("Initial flags: {}", flags);
    println!("  Binary representation: {:08b}", flags.raw());

    // AND operation - checking flags
    println!("\n--- AND Operation (Checking Flags) ---");
    println!("Battery low: {}", flags.battery_low());
    println!("GPS enabled: {}", flags.is_flag_set(RobotSensorFlags::GPS_ENABLED));
    println!("Camera active: {}", flags.is_flag_set(RobotSensorFlags::CAMERA_ACTIVE));

    // OR operation - setting flags
    println!("\n--- OR Operation (Setting Flags) ---");
    println!("Before setting WiFi: {}", flags);
    flags.set_flag(RobotSensorFlags::WIFI_CONNECTED);
    println!("After setting WiFi: {}", flags);

    // XOR operation - toggling flags
    println!("\n--- XOR Operation (Toggling Flags) ---");
    println!("Before toggling camera: {}", flags);
    flags.toggle_flag(RobotSensorFlags::CAMERA_ACTIVE);
    println!("After toggling camera: {}", flags);

    // NOT operation - inverting flags
    println!("\n--- NOT Operation (Inverting Flags) ---");
    let inverted = flags.inverted();
    println!("Original flags: {}", flags);
    println!("Inverted flags: {}", inverted);

    // Shift operations
    println!("\n--- Shift Operations ---");
    let shift_demo = 0b00000001u8;
    println!("Original: {:08b}", shift_demo);
    println!("Left shift 3: {:08b}", shift_demo << 3);
    println!("Right shift from 8: {:08b}", 0b10000000u8 >> 3);

    // Practical combinations
    println!("\n--- Practical Combinations ---");
    println!("Active sensors: {}", flags.active_sensor_count());
    println!("Critical errors: {}", flags.has_critical_errors());

    // Mask operations
    println!("\n--- Mask Operations ---");
    let sensor_mask = 0b00001110; // Camera, LiDAR, Obstacle sensors
    let active_sensors = flags.raw() & sensor_mask;
    println!("Sensor mask: {:08b}", sensor_mask);
    println!("Active sensors (masked): {:08b}", active_sensors);
}

/// Advanced bitwise techniques demonstration
pub fn demonstrate_advanced_techniques() {
    println!("\n=== Advanced Bitwise Techniques ===");

    // Bit manipulation tricks
    println!("\n--- Bit Manipulation Tricks ---");

    // Check if power of 2
    let test_values = [1, 2, 3, 4, 8, 15, 16];
    for value in test_values {
        let is_power_of_2 = value > 0 && (value & (value - 1)) == 0;
        println!("{} is power of 2: {}", value, is_power_of_2);
    }

    // Count trailing zeros
    println!("\n--- Counting Operations ---");
    let test_byte = 0b10110000u8;
    println!("Value: {:08b}", test_byte);
    println!("Leading zeros: {}", test_byte.leading_zeros());
    println!("Trailing zeros: {}", test_byte.trailing_zeros());
    println!("Count ones: {}", test_byte.count_ones());

    // Bit reversal
    println!("\n--- Bit Reversal ---");
    let original = 0b10110100u8;
    let reversed = original.reverse_bits();
    println!("Original: {:08b}", original);
    println!("Reversed: {:08b}", reversed);

    // Isolation of rightmost set bit
    println!("\n--- Bit Isolation ---");
    let value = 0b10110100u8;
    let rightmost_bit = value & (!value + 1);
    println!("Value: {:08b}", value);
    println!("Rightmost set bit: {:08b}", rightmost_bit);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::learning_tests::test_utils::*;

    #[test]
    fn test_robot_sensor_flags_creation() {
        let flags = RobotSensorFlags::new();
        assert_eq!(flags.raw(), 0);

        let flags = RobotSensorFlags::from_raw(0b10110100);
        assert_eq!(flags.raw(), 0b10110100);
    }

    #[test]
    fn test_flag_checking_and_operation() {
        let flags = RobotSensorFlags::from_raw(0b10110100);

        // Test AND operation for checking flags
        assert!(flags.is_flag_set(RobotSensorFlags::BATTERY_LOW));
        assert!(flags.is_flag_set(RobotSensorFlags::GPS_ENABLED));
        assert!(flags.is_flag_set(RobotSensorFlags::CAMERA_ACTIVE));
        assert!(!flags.is_flag_set(RobotSensorFlags::WIFI_CONNECTED));
        assert!(!flags.is_flag_set(RobotSensorFlags::SYSTEM_READY));
    }

    #[test]
    fn test_flag_setting_or_operation() {
        let mut flags = RobotSensorFlags::new();

        // Test OR operation for setting flags
        flags.set_flag(RobotSensorFlags::SYSTEM_READY);
        assert!(flags.is_flag_set(RobotSensorFlags::SYSTEM_READY));
        assert_eq!(flags.raw(), 0b00000001);

        flags.set_flag(RobotSensorFlags::GPS_ENABLED);
        assert!(flags.is_flag_set(RobotSensorFlags::GPS_ENABLED));
        assert_eq!(flags.raw(), 0b00100001);
    }

    #[test]
    fn test_flag_clearing_and_not_operation() {
        let mut flags = RobotSensorFlags::from_raw(0b11111111);

        // Test AND with NOT operation for clearing flags
        flags.clear_flag(RobotSensorFlags::BATTERY_LOW);
        assert!(!flags.is_flag_set(RobotSensorFlags::BATTERY_LOW));
        assert_eq!(flags.raw(), 0b01111111);

        flags.clear_flag(RobotSensorFlags::SYSTEM_READY);
        assert!(!flags.is_flag_set(RobotSensorFlags::SYSTEM_READY));
        assert_eq!(flags.raw(), 0b01111110);
    }

    #[test]
    fn test_flag_toggling_xor_operation() {
        let mut flags = RobotSensorFlags::from_raw(0b10110100);

        // Test XOR operation for toggling flags
        let original_camera = flags.is_flag_set(RobotSensorFlags::CAMERA_ACTIVE);
        flags.toggle_flag(RobotSensorFlags::CAMERA_ACTIVE);
        assert_eq!(flags.is_flag_set(RobotSensorFlags::CAMERA_ACTIVE), !original_camera);

        // Toggle again should restore original state
        flags.toggle_flag(RobotSensorFlags::CAMERA_ACTIVE);
        assert_eq!(flags.is_flag_set(RobotSensorFlags::CAMERA_ACTIVE), original_camera);
    }

    #[test]
    fn test_flag_inversion_not_operation() {
        let flags = RobotSensorFlags::from_raw(0b10110100);
        let inverted = flags.inverted();

        // Test NOT operation
        assert_eq!(inverted.raw(), !0b10110100);
        assert_eq!(inverted.raw(), 0b01001011);

        // Each bit should be flipped
        for bit in 0..8 {
            assert_eq!(flags.is_flag_set(bit), !inverted.is_flag_set(bit));
        }
    }

    #[test]
    fn test_shift_operations() {
        // Test left shift
        assert_eq!(1u8 << 3, 0b00001000);
        assert_eq!(0b00000001u8 << 7, 0b10000000);

        // Test right shift
        assert_eq!(0b10000000u8 >> 3, 0b00010000);
        assert_eq!(0b11110000u8 >> 4, 0b00001111);
    }

    #[test]
    fn test_practical_flag_operations() {
        let flags = RobotSensorFlags::from_raw(0b10110100);

        // Test critical error detection
        assert!(flags.has_critical_errors());

        let safe_flags = RobotSensorFlags::from_raw(0b00110100);
        assert!(!safe_flags.has_critical_errors());

        // Test sensor counting
        assert_eq!(flags.active_sensor_count(), 5);
        assert_eq!(safe_flags.active_sensor_count(), 4);
    }

    #[test]
    fn test_mask_operations() {
        let flags = RobotSensorFlags::from_raw(0b10110100);

        // Test sensor mask (bits 1-3: LiDAR, Camera, Obstacle)
        let sensor_mask = 0b00001110;
        let active_sensors = flags.raw() & sensor_mask;
        assert_eq!(active_sensors, 0b00000100); // Only camera is active

        // Test system mask (bits 4-7: WiFi, GPS, Motor, Battery)
        let system_mask = 0b11110000;
        let system_status = flags.raw() & system_mask;
        assert_eq!(system_status, 0b10110000);
    }

    #[test]
    fn test_power_of_two_detection() {
        // Test power of 2 detection using bit trick: n & (n-1) == 0
        assert!(1 > 0 && (1 & (1 - 1)) == 0); // 1 is 2^0
        assert!(2 > 0 && (2 & (2 - 1)) == 0); // 2 is 2^1
        assert!(4 > 0 && (4 & (4 - 1)) == 0); // 4 is 2^2
        assert!(8 > 0 && (8 & (8 - 1)) == 0); // 8 is 2^3

        assert!(!(3 > 0 && (3 & (3 - 1)) == 0)); // 3 is not power of 2
        assert!(!(5 > 0 && (5 & (5 - 1)) == 0)); // 5 is not power of 2
        assert!(!(6 > 0 && (6 & (6 - 1)) == 0)); // 6 is not power of 2
    }

    #[test]
    fn test_bit_counting_operations() {
        let value = 0b10110100u8;

        assert_eq!(value.count_ones(), 5);
        assert_eq!(value.count_zeros(), 3);
        assert_eq!(value.leading_zeros(), 0); // First bit is 1
        assert_eq!(value.trailing_zeros(), 2); // Two zeros at the end
    }

    #[test]
    fn test_user_code_implementation() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Check for bitwise operators
        assert!(
            analyzer.code.contains(" & "),
            "❌ You need to use AND (&) operator for checking flags"
        );
        assert!(
            analyzer.code.contains(" | "),
            "❌ You need to use OR (|) operator for setting flags"
        );
        assert!(
            analyzer.code.contains(" ^ "),
            "❌ You need to use XOR (^) operator for toggling flags"
        );
        assert!(
            analyzer.code.contains("!") || analyzer.code.contains("~"),
            "❌ You need to use NOT operator for flag inversion"
        );

        // Check for shift operations
        assert!(
            analyzer.code.contains("<<") || analyzer.code.contains(">>"),
            "❌ You should use shift operators (<<, >>) for bit positioning"
        );

        // Check for flag manipulation functions
        assert!(
            analyzer.contains_function("check") ||
            analyzer.contains_function("set") ||
            analyzer.contains_function("toggle") ||
            analyzer.code.contains("flag"),
            "❌ You should implement flag manipulation functions"
        );

        // Check for output demonstration
        assert!(
            analyzer.contains_println(),
            "❌ You should demonstrate bitwise operations with output"
        );
    }
}

/// Student exercises for practicing bitwise operations
pub mod exercises {
    use super::*;

    /// Exercise 1: Implement basic flag checker
    /// Create a function that checks if specific robot systems are online
    pub fn exercise_flag_checker() {
        println!("Exercise 1: Flag Checker");

        // TODO: Implement a function that takes sensor flags and returns
        // whether the robot is "mission ready" (GPS enabled AND camera active AND system ready)

        // Test with these flag values:
        let test_flags = [
            0b00100101, // GPS + Camera + Ready
            0b00100100, // GPS + Camera (no Ready)
            0b00000101, // Camera + Ready (no GPS)
            0b00100001, // GPS + Ready (no Camera)
        ];

        for flags in test_flags {
            let sensor_flags = RobotSensorFlags::from_raw(flags);
            println!("Flags: {} - Mission Ready: [YOUR IMPLEMENTATION HERE]", sensor_flags);
        }
    }

    /// Exercise 2: Implement status register manipulation
    /// Create functions to manage a robot status register
    pub fn exercise_status_register() {
        println!("\nExercise 2: Status Register Manipulation");

        // TODO: Implement functions to:
        // 1. Set emergency mode (set bit 7)
        // 2. Clear all sensor flags (clear bits 0-3)
        // 3. Toggle communication mode (toggle bit 4)
        // 4. Check if any navigation systems are active (bits 1, 2, 5)

        let mut status = 0b01010101u8;
        println!("Initial status: {:08b}", status);

        // Test your implementations here
        println!("After emergency mode: [YOUR IMPLEMENTATION HERE]");
        println!("After clearing sensors: [YOUR IMPLEMENTATION HERE]");
        println!("After toggling comm: [YOUR IMPLEMENTATION HERE]");
        println!("Navigation active: [YOUR IMPLEMENTATION HERE]");
    }

    /// Exercise 3: Implement bit pattern matching
    /// Create a function that matches specific bit patterns
    pub fn exercise_pattern_matching() {
        println!("\nExercise 3: Bit Pattern Matching");

        // TODO: Implement a function that checks if a flag value matches
        // any of these error patterns:
        // - Critical error: bits 7,6 = 11 (battery low + motor error)
        // - Communication error: bits 5,4 = 01 (GPS on, WiFi off)
        // - Sensor error: bits 3,2,1 = 110 (obstacle + camera, no LiDAR)

        let test_patterns = [
            0b11000000, // Critical error
            0b00010000, // Communication error
            0b00001100, // Sensor error
            0b10010110, // Multiple errors
            0b00000001, // No errors
        ];

        for pattern in test_patterns {
            println!("Pattern {:08b}: [YOUR ERROR ANALYSIS HERE]", pattern);
        }
    }

    /// Exercise 4: Implement efficient flag operations
    /// Create optimized functions using bit manipulation tricks
    pub fn exercise_efficient_operations() {
        println!("\nExercise 4: Efficient Flag Operations");

        // TODO: Implement these efficient operations:
        // 1. Find the rightmost set bit using: value & (-value)
        // 2. Clear the rightmost set bit using: value & (value - 1)
        // 3. Check if value is power of 2 using: value & (value - 1) == 0
        // 4. Count set bits using bit manipulation loops

        let test_values = [0b10110100, 0b00001000, 0b11110000, 0b00000001];

        for value in test_values {
            println!("Value: {:08b}", value);
            println!("  Rightmost set bit: [YOUR IMPLEMENTATION HERE]");
            println!("  Clear rightmost: [YOUR IMPLEMENTATION HERE]");
            println!("  Is power of 2: [YOUR IMPLEMENTATION HERE]");
            println!("  Set bit count: [YOUR IMPLEMENTATION HERE]");
        }
    }

    /// Exercise 5: Create a robot configuration system
    /// Implement a complete flag-based configuration system
    pub fn exercise_configuration_system() {
        println!("\nExercise 5: Robot Configuration System");

        // TODO: Create a RobotConfig struct that uses bitwise operations to:
        // 1. Store configuration in a single u16 value
        // 2. Provide methods to get/set individual config options
        // 3. Validate configuration combinations
        // 4. Export/import configuration as bit patterns

        // Configuration bits:
        // 0-2: Movement speed (0-7)
        // 3-4: Sensor sensitivity (0-3)
        // 5: Auto-pilot enabled
        // 6: Stealth mode
        // 7: Debug mode
        // 8-10: Communication protocol (0-7)
        // 11-15: Reserved

        println!("Implement a complete configuration system using the bit layout above");
    }
}

fn main() {
    demonstrate_bitwise_operations();
    demonstrate_advanced_techniques();

    println!("\n" + "=".repeat(50).as_str());
    println!("STUDENT EXERCISES");
    println!("=".repeat(50));

    exercises::exercise_flag_checker();
    exercises::exercise_status_register();
    exercises::exercise_pattern_matching();
    exercises::exercise_efficient_operations();
    exercises::exercise_configuration_system();
}