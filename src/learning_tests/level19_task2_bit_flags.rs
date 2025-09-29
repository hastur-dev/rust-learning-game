// Level 19 Task 2: Bit Flag Manipulation and Masking
// Learn advanced bit flag techniques for robot system management

use std::fmt;

/// Robot system permissions and capabilities
/// Uses bitwise flags to efficiently store multiple boolean states
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RobotPermissions(pub u32);

impl RobotPermissions {
    // Movement permissions (bits 0-7)
    pub const MOVE_FORWARD: u32     = 1 << 0;  // 0x00000001
    pub const MOVE_BACKWARD: u32    = 1 << 1;  // 0x00000002
    pub const TURN_LEFT: u32        = 1 << 2;  // 0x00000004
    pub const TURN_RIGHT: u32       = 1 << 3;  // 0x00000008
    pub const STRAFE_LEFT: u32      = 1 << 4;  // 0x00000010
    pub const STRAFE_RIGHT: u32     = 1 << 5;  // 0x00000020
    pub const FLY_UP: u32           = 1 << 6;  // 0x00000040
    pub const FLY_DOWN: u32         = 1 << 7;  // 0x00000080

    // Sensor permissions (bits 8-15)
    pub const CAMERA_ACCESS: u32    = 1 << 8;  // 0x00000100
    pub const LIDAR_ACCESS: u32     = 1 << 9;  // 0x00000200
    pub const GPS_ACCESS: u32       = 1 << 10; // 0x00000400
    pub const SONAR_ACCESS: u32     = 1 << 11; // 0x00000800
    pub const INFRARED_ACCESS: u32  = 1 << 12; // 0x00001000
    pub const RADAR_ACCESS: u32     = 1 << 13; // 0x00002000
    pub const MICROPHONE_ACCESS: u32= 1 << 14; // 0x00004000
    pub const TEMPERATURE_ACCESS: u32= 1 << 15; // 0x00008000

    // Action permissions (bits 16-23)
    pub const PICK_UP_ITEMS: u32    = 1 << 16; // 0x00010000
    pub const DROP_ITEMS: u32       = 1 << 17; // 0x00020000
    pub const OPEN_DOORS: u32       = 1 << 18; // 0x00040000
    pub const ACTIVATE_SWITCHES: u32= 1 << 19; // 0x00080000
    pub const COMMUNICATE: u32      = 1 << 20; // 0x00100000
    pub const RECORD_DATA: u32      = 1 << 21; // 0x00200000
    pub const TRANSMIT_DATA: u32    = 1 << 22; // 0x00400000
    pub const DELETE_DATA: u32      = 1 << 23; // 0x00800000

    // System permissions (bits 24-31)
    pub const ADMIN_ACCESS: u32     = 1 << 24; // 0x01000000
    pub const MODIFY_CONFIG: u32    = 1 << 25; // 0x02000000
    pub const INSTALL_SOFTWARE: u32 = 1 << 26; // 0x04000000
    pub const NETWORK_ACCESS: u32   = 1 << 27; // 0x08000000
    pub const SHUTDOWN_SYSTEM: u32  = 1 << 28; // 0x10000000
    pub const FACTORY_RESET: u32    = 1 << 29; // 0x20000000
    pub const DEBUG_MODE: u32       = 1 << 30; // 0x40000000
    pub const EMERGENCY_STOP: u32   = 1 << 31; // 0x80000000

    // Permission groups using masks
    pub const ALL_MOVEMENT: u32 = 0x000000FF;     // Bits 0-7
    pub const ALL_SENSORS: u32 = 0x0000FF00;      // Bits 8-15
    pub const ALL_ACTIONS: u32 = 0x00FF0000;      // Bits 16-23
    pub const ALL_SYSTEM: u32 = 0xFF000000;       // Bits 24-31

    // Predefined permission sets
    pub const BASIC_ROBOT: u32 = Self::MOVE_FORWARD | Self::MOVE_BACKWARD |
                                 Self::TURN_LEFT | Self::TURN_RIGHT |
                                 Self::CAMERA_ACCESS | Self::GPS_ACCESS;

    pub const SECURITY_ROBOT: u32 = Self::ALL_MOVEMENT | Self::ALL_SENSORS |
                                    Self::COMMUNICATE | Self::RECORD_DATA;

    pub const MAINTENANCE_ROBOT: u32 = Self::ALL_MOVEMENT | Self::ALL_SENSORS |
                                      Self::ALL_ACTIONS | Self::ADMIN_ACCESS |
                                      Self::MODIFY_CONFIG;

    pub const EMERGENCY_ROBOT: u32 = Self::ALL_MOVEMENT | Self::ALL_SENSORS |
                                     Self::COMMUNICATE | Self::EMERGENCY_STOP;

    /// Create new permissions with no access
    pub fn new() -> Self {
        Self(0)
    }

    /// Create permissions from raw value
    pub fn from_raw(value: u32) -> Self {
        Self(value)
    }

    /// Get raw permission value
    pub fn raw(&self) -> u32 {
        self.0
    }

    /// Check if a specific permission is granted
    pub fn has_permission(&self, permission: u32) -> bool {
        (self.0 & permission) != 0
    }

    /// Grant a specific permission
    pub fn grant(&mut self, permission: u32) {
        self.0 |= permission;
    }

    /// Revoke a specific permission
    pub fn revoke(&mut self, permission: u32) {
        self.0 &= !permission;
    }

    /// Toggle a specific permission
    pub fn toggle(&mut self, permission: u32) {
        self.0 ^= permission;
    }

    /// Check if all permissions in a mask are granted
    pub fn has_all_permissions(&self, mask: u32) -> bool {
        (self.0 & mask) == mask
    }

    /// Check if any permissions in a mask are granted
    pub fn has_any_permissions(&self, mask: u32) -> bool {
        (self.0 & mask) != 0
    }

    /// Grant all permissions in a mask
    pub fn grant_mask(&mut self, mask: u32) {
        self.0 |= mask;
    }

    /// Revoke all permissions in a mask
    pub fn revoke_mask(&mut self, mask: u32) {
        self.0 &= !mask;
    }

    /// Get only the permissions that match a specific mask
    pub fn filter_by_mask(&self, mask: u32) -> Self {
        Self(self.0 & mask)
    }

    /// Combine with another permission set (union)
    pub fn union(&self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Get intersection with another permission set
    pub fn intersection(&self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Get difference from another permission set
    pub fn difference(&self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Check if this is a subset of another permission set
    pub fn is_subset_of(&self, other: Self) -> bool {
        (self.0 & other.0) == self.0
    }

    /// Count total number of granted permissions
    pub fn permission_count(&self) -> u32 {
        self.0.count_ones()
    }

    /// Get permissions by category
    pub fn movement_permissions(&self) -> Self {
        self.filter_by_mask(Self::ALL_MOVEMENT)
    }

    pub fn sensor_permissions(&self) -> Self {
        self.filter_by_mask(Self::ALL_SENSORS)
    }

    pub fn action_permissions(&self) -> Self {
        self.filter_by_mask(Self::ALL_ACTIONS)
    }

    pub fn system_permissions(&self) -> Self {
        self.filter_by_mask(Self::ALL_SYSTEM)
    }

    /// Security level assessment
    pub fn security_level(&self) -> SecurityLevel {
        if self.has_permission(Self::FACTORY_RESET) {
            SecurityLevel::Maximum
        } else if self.has_permission(Self::ADMIN_ACCESS) {
            SecurityLevel::High
        } else if self.has_any_permissions(Self::ALL_SYSTEM) {
            SecurityLevel::Medium
        } else {
            SecurityLevel::Low
        }
    }
}

impl fmt::Display for RobotPermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Permissions(0x{:08X}, {} active)", self.0, self.permission_count())
    }
}

#[derive(Debug, PartialEq)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Maximum,
}

/// Advanced bit manipulation utilities
pub struct BitUtils;

impl BitUtils {
    /// Extract a range of bits from a value
    pub fn extract_bits(value: u32, start: u8, length: u8) -> u32 {
        let mask = (1u32 << length) - 1;
        (value >> start) & mask
    }

    /// Set a range of bits in a value
    pub fn set_bits(value: u32, start: u8, length: u8, new_bits: u32) -> u32 {
        let mask = ((1u32 << length) - 1) << start;
        (value & !mask) | ((new_bits << start) & mask)
    }

    /// Create a mask with specific bits set
    pub fn create_mask(bit_positions: &[u8]) -> u32 {
        bit_positions.iter().fold(0, |mask, &bit| mask | (1 << bit))
    }

    /// Find the highest set bit position
    pub fn highest_set_bit(value: u32) -> Option<u8> {
        if value == 0 {
            None
        } else {
            Some(31 - value.leading_zeros() as u8)
        }
    }

    /// Find the lowest set bit position
    pub fn lowest_set_bit(value: u32) -> Option<u8> {
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros() as u8)
        }
    }

    /// Check if exactly one bit is set
    pub fn is_single_bit(value: u32) -> bool {
        value != 0 && (value & (value - 1)) == 0
    }

    /// Get next power of 2 greater than or equal to value
    pub fn next_power_of_2(value: u32) -> u32 {
        if value == 0 {
            1
        } else {
            1 << (32 - (value - 1).leading_zeros())
        }
    }
}

/// Demonstrate advanced bit flag manipulation
pub fn demonstrate_flag_operations() {
    println!("=== Advanced Bit Flag Manipulation ===");

    // Create different robot types with predefined permissions
    let mut basic_robot = RobotPermissions::from_raw(RobotPermissions::BASIC_ROBOT);
    let security_robot = RobotPermissions::from_raw(RobotPermissions::SECURITY_ROBOT);
    let maintenance_robot = RobotPermissions::from_raw(RobotPermissions::MAINTENANCE_ROBOT);

    println!("Basic Robot: {}", basic_robot);
    println!("Security Robot: {}", security_robot);
    println!("Maintenance Robot: {}", maintenance_robot);

    // Demonstrate permission checking
    println!("\n--- Permission Checking ---");
    println!("Basic robot can move forward: {}",
             basic_robot.has_permission(RobotPermissions::MOVE_FORWARD));
    println!("Basic robot can access LIDAR: {}",
             basic_robot.has_permission(RobotPermissions::LIDAR_ACCESS));
    println!("Security robot has all sensors: {}",
             security_robot.has_all_permissions(RobotPermissions::ALL_SENSORS));

    // Demonstrate permission granting and revoking
    println!("\n--- Permission Modification ---");
    println!("Before LIDAR grant: {}", basic_robot);
    basic_robot.grant(RobotPermissions::LIDAR_ACCESS);
    println!("After LIDAR grant: {}", basic_robot);

    basic_robot.revoke(RobotPermissions::CAMERA_ACCESS);
    println!("After camera revoke: {}", basic_robot);

    // Demonstrate mask operations
    println!("\n--- Mask Operations ---");
    let movement_only = security_robot.filter_by_mask(RobotPermissions::ALL_MOVEMENT);
    println!("Security robot movement permissions: {}", movement_only);

    let sensor_only = security_robot.filter_by_mask(RobotPermissions::ALL_SENSORS);
    println!("Security robot sensor permissions: {}", sensor_only);

    // Demonstrate set operations
    println!("\n--- Set Operations ---");
    let combined = basic_robot.union(security_robot);
    println!("Basic ∪ Security: {}", combined);

    let common = basic_robot.intersection(security_robot);
    println!("Basic ∩ Security: {}", common);

    let difference = security_robot.difference(basic_robot);
    println!("Security - Basic: {}", difference);

    // Security level assessment
    println!("\n--- Security Assessment ---");
    println!("Basic robot security: {:?}", basic_robot.security_level());
    println!("Security robot security: {:?}", security_robot.security_level());
    println!("Maintenance robot security: {:?}", maintenance_robot.security_level());
}

/// Demonstrate advanced bit manipulation utilities
pub fn demonstrate_bit_utilities() {
    println!("\n=== Advanced Bit Utilities ===");

    let test_value = 0b11010110101011110000111100001111u32;
    println!("Test value: 0x{:08X} (0b{:032b})", test_value, test_value);

    // Extract bit ranges
    println!("\n--- Bit Range Extraction ---");
    let low_nibble = BitUtils::extract_bits(test_value, 0, 4);
    let high_nibble = BitUtils::extract_bits(test_value, 4, 4);
    let middle_byte = BitUtils::extract_bits(test_value, 8, 8);

    println!("Low nibble (bits 0-3): 0x{:X}", low_nibble);
    println!("High nibble (bits 4-7): 0x{:X}", high_nibble);
    println!("Middle byte (bits 8-15): 0x{:02X}", middle_byte);

    // Set bit ranges
    println!("\n--- Bit Range Setting ---");
    let mut modified = test_value;
    modified = BitUtils::set_bits(modified, 0, 4, 0xA);
    println!("After setting low nibble to 0xA: 0x{:08X}", modified);

    modified = BitUtils::set_bits(modified, 16, 8, 0xFF);
    println!("After setting bits 16-23 to 0xFF: 0x{:08X}", modified);

    // Create custom masks
    println!("\n--- Custom Mask Creation ---");
    let custom_mask = BitUtils::create_mask(&[0, 2, 4, 6, 8, 10]);
    println!("Mask for bits [0,2,4,6,8,10]: 0x{:08X}", custom_mask);

    // Find bit positions
    println!("\n--- Bit Position Finding ---");
    if let Some(highest) = BitUtils::highest_set_bit(test_value) {
        println!("Highest set bit: {}", highest);
    }
    if let Some(lowest) = BitUtils::lowest_set_bit(test_value) {
        println!("Lowest set bit: {}", lowest);
    }

    // Power of 2 operations
    println!("\n--- Power of 2 Operations ---");
    let test_values = [0, 1, 7, 8, 15, 16, 31, 32];
    for value in test_values {
        println!("Value: {}, Is single bit: {}, Next power of 2: {}",
                 value, BitUtils::is_single_bit(value), BitUtils::next_power_of_2(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::learning_tests::test_utils::*;

    #[test]
    fn test_permission_basic_operations() {
        let mut perms = RobotPermissions::new();

        // Test granting permissions
        perms.grant(RobotPermissions::MOVE_FORWARD);
        assert!(perms.has_permission(RobotPermissions::MOVE_FORWARD));

        perms.grant(RobotPermissions::CAMERA_ACCESS);
        assert!(perms.has_permission(RobotPermissions::CAMERA_ACCESS));

        // Test revoking permissions
        perms.revoke(RobotPermissions::MOVE_FORWARD);
        assert!(!perms.has_permission(RobotPermissions::MOVE_FORWARD));

        // Test toggling permissions
        perms.toggle(RobotPermissions::GPS_ACCESS);
        assert!(perms.has_permission(RobotPermissions::GPS_ACCESS));
        perms.toggle(RobotPermissions::GPS_ACCESS);
        assert!(!perms.has_permission(RobotPermissions::GPS_ACCESS));
    }

    #[test]
    fn test_permission_mask_operations() {
        let mut perms = RobotPermissions::new();

        // Test granting mask
        perms.grant_mask(RobotPermissions::ALL_MOVEMENT);
        assert!(perms.has_all_permissions(RobotPermissions::ALL_MOVEMENT));

        // Test partial mask checking
        perms.revoke(RobotPermissions::FLY_UP);
        assert!(!perms.has_all_permissions(RobotPermissions::ALL_MOVEMENT));
        assert!(perms.has_any_permissions(RobotPermissions::ALL_MOVEMENT));

        // Test mask filtering
        let movement_only = perms.filter_by_mask(RobotPermissions::ALL_MOVEMENT);
        assert!(!movement_only.has_permission(RobotPermissions::FLY_UP));
        assert!(movement_only.has_permission(RobotPermissions::MOVE_FORWARD));
    }

    #[test]
    fn test_permission_set_operations() {
        let perms1 = RobotPermissions::from_raw(0x0000000F); // First 4 bits
        let perms2 = RobotPermissions::from_raw(0x000000F0); // Next 4 bits
        let perms3 = RobotPermissions::from_raw(0x0000000C); // Bits 2,3

        // Test union
        let union = perms1.union(perms2);
        assert_eq!(union.raw(), 0x000000FF);

        // Test intersection
        let intersection = perms1.intersection(perms3);
        assert_eq!(intersection.raw(), 0x0000000C);

        // Test difference
        let difference = perms1.difference(perms3);
        assert_eq!(difference.raw(), 0x00000003);

        // Test subset
        assert!(perms3.is_subset_of(perms1));
        assert!(!perms1.is_subset_of(perms3));
    }

    #[test]
    fn test_bit_utils_extract_and_set() {
        let value = 0xABCD1234u32;

        // Test bit extraction
        assert_eq!(BitUtils::extract_bits(value, 0, 4), 0x4);
        assert_eq!(BitUtils::extract_bits(value, 4, 4), 0x3);
        assert_eq!(BitUtils::extract_bits(value, 8, 8), 0x12);
        assert_eq!(BitUtils::extract_bits(value, 16, 16), 0xABCD);

        // Test bit setting
        let modified = BitUtils::set_bits(value, 0, 4, 0xF);
        assert_eq!(modified & 0xF, 0xF);

        let modified = BitUtils::set_bits(value, 16, 8, 0x00);
        assert_eq!((modified >> 16) & 0xFF, 0x00);
    }

    #[test]
    fn test_bit_utils_position_finding() {
        assert_eq!(BitUtils::highest_set_bit(0x80000000), Some(31));
        assert_eq!(BitUtils::highest_set_bit(0x00000001), Some(0));
        assert_eq!(BitUtils::highest_set_bit(0x00000000), None);

        assert_eq!(BitUtils::lowest_set_bit(0x80000000), Some(31));
        assert_eq!(BitUtils::lowest_set_bit(0x00000008), Some(3));
        assert_eq!(BitUtils::lowest_set_bit(0x00000000), None);
    }

    #[test]
    fn test_bit_utils_power_operations() {
        // Test single bit detection
        assert!(BitUtils::is_single_bit(1));
        assert!(BitUtils::is_single_bit(8));
        assert!(BitUtils::is_single_bit(1024));
        assert!(!BitUtils::is_single_bit(3));
        assert!(!BitUtils::is_single_bit(0));

        // Test next power of 2
        assert_eq!(BitUtils::next_power_of_2(0), 1);
        assert_eq!(BitUtils::next_power_of_2(1), 1);
        assert_eq!(BitUtils::next_power_of_2(2), 2);
        assert_eq!(BitUtils::next_power_of_2(3), 4);
        assert_eq!(BitUtils::next_power_of_2(15), 16);
    }

    #[test]
    fn test_predefined_permission_sets() {
        let basic = RobotPermissions::from_raw(RobotPermissions::BASIC_ROBOT);
        let security = RobotPermissions::from_raw(RobotPermissions::SECURITY_ROBOT);
        let maintenance = RobotPermissions::from_raw(RobotPermissions::MAINTENANCE_ROBOT);

        // Test basic robot permissions
        assert!(basic.has_permission(RobotPermissions::MOVE_FORWARD));
        assert!(basic.has_permission(RobotPermissions::CAMERA_ACCESS));
        assert!(!basic.has_permission(RobotPermissions::ADMIN_ACCESS));

        // Test security robot permissions
        assert!(security.has_all_permissions(RobotPermissions::ALL_MOVEMENT));
        assert!(security.has_all_permissions(RobotPermissions::ALL_SENSORS));
        assert!(!security.has_permission(RobotPermissions::ADMIN_ACCESS));

        // Test maintenance robot permissions
        assert!(maintenance.has_permission(RobotPermissions::ADMIN_ACCESS));
        assert!(maintenance.has_all_permissions(RobotPermissions::ALL_ACTIONS));
    }

    #[test]
    fn test_security_level_assessment() {
        let basic = RobotPermissions::from_raw(RobotPermissions::BASIC_ROBOT);
        let with_system = RobotPermissions::from_raw(RobotPermissions::NETWORK_ACCESS);
        let with_admin = RobotPermissions::from_raw(RobotPermissions::ADMIN_ACCESS);
        let with_factory = RobotPermissions::from_raw(RobotPermissions::FACTORY_RESET);

        assert_eq!(basic.security_level(), SecurityLevel::Low);
        assert_eq!(with_system.security_level(), SecurityLevel::Medium);
        assert_eq!(with_admin.security_level(), SecurityLevel::High);
        assert_eq!(with_factory.security_level(), SecurityLevel::Maximum);
    }

    #[test]
    fn test_user_code_implementation() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Check for bit flag operations
        assert!(
            analyzer.code.contains("&") && analyzer.code.contains("|"),
            "❌ You need to use bitwise AND (&) and OR (|) for flag operations"
        );

        // Check for permission checking patterns
        assert!(
            analyzer.code.contains("permission") ||
            analyzer.code.contains("flag") ||
            analyzer.code.contains("mask"),
            "❌ You should implement permission/flag checking functionality"
        );

        // Check for mask operations
        assert!(
            analyzer.code.contains("mask") ||
            analyzer.code.contains("0x") ||
            analyzer.code.contains("0b"),
            "❌ You should use bit masks in hexadecimal or binary format"
        );

        // Check for struct or enum definitions
        assert!(
            analyzer.contains_struct("Permission") ||
            analyzer.contains_struct("Flag") ||
            analyzer.contains_struct("Robot") ||
            analyzer.code.contains("enum"),
            "❌ You should define structures for organizing bit flags"
        );

        // Check for functions
        assert!(
            analyzer.contains_function("has") ||
            analyzer.contains_function("set") ||
            analyzer.contains_function("grant") ||
            analyzer.contains_function("check"),
            "❌ You should implement functions for flag manipulation"
        );

        // Check for output demonstration
        assert!(
            analyzer.contains_println(),
            "❌ You should demonstrate bit flag operations with output"
        );
    }
}

/// Student exercises for practicing bit flag manipulation
pub mod exercises {
    use super::*;

    /// Exercise 1: Create a robot capability system
    pub fn exercise_capability_system() {
        println!("Exercise 1: Robot Capability System");

        // TODO: Create a RobotCapabilities struct using a u16 for storage
        // Define capabilities like: Navigation, Communication, Manipulation,
        // Sensing, Processing, Learning, Security, Maintenance

        // Implement methods to:
        // - Add/remove capabilities
        // - Check if robot can perform a task requiring multiple capabilities
        // - Calculate capability score (number of active capabilities)

        println!("Create a capability system with the following robots:");
        println!("- Exploration Robot: Navigation + Sensing + Communication");
        println!("- Security Robot: All capabilities except Learning");
        println!("- Maintenance Robot: Manipulation + Processing + Security");

        // Test capability combinations and requirements
    }

    /// Exercise 2: Implement a mission status tracker
    pub fn exercise_mission_status() {
        println!("\nExercise 2: Mission Status Tracker");

        // TODO: Create a MissionStatus struct that tracks:
        // - Objective completion (8 different objectives)
        // - Resource status (4 different resources: battery, fuel, ammo, data)
        // - Alert conditions (4 different alert levels)
        // - System health (8 different systems)

        // Use bit fields to pack all this information efficiently
        // Implement methods to update status and check mission completion

        let test_scenarios = [
            "Objectives 1,3,5 complete, low battery alert",
            "All objectives complete, all systems healthy",
            "Objectives 2,4,6,8 complete, critical fuel alert",
            "No objectives complete, multiple system failures",
        ];

        for scenario in test_scenarios {
            println!("Scenario: {}", scenario);
            println!("  Mission Status: [YOUR IMPLEMENTATION HERE]");
        }
    }

    /// Exercise 3: Create a permission inheritance system
    pub fn exercise_permission_inheritance() {
        println!("\nExercise 3: Permission Inheritance System");

        // TODO: Create a permission system with inheritance:
        // - Base permissions that all robots have
        // - Role-specific permissions (Worker, Security, Admin)
        // - Temporary permissions that can be granted/revoked
        // - Emergency permissions that override normal restrictions

        // Implement permission checking that considers all inheritance levels
        // Create functions to promote/demote robot roles

        println!("Robot hierarchy:");
        println!("- Guest Robot (minimal permissions)");
        println!("- Worker Robot (Guest + work permissions)");
        println!("- Security Robot (Worker + security permissions)");
        println!("- Admin Robot (Security + admin permissions)");
        println!("- Emergency Override (Admin + emergency permissions)");

        // Test permission inheritance and role changes
    }

    /// Exercise 4: Implement efficient bit field encoding
    pub fn exercise_bit_field_encoding() {
        println!("\nExercise 4: Bit Field Encoding");

        // TODO: Pack complex robot state into minimal bits:
        // - Position coordinates (limited range, 4 bits each for x,y)
        // - Direction facing (3 bits for 8 directions)
        // - Speed level (3 bits for 8 speed levels)
        // - Current action (4 bits for 16 different actions)
        // - Health level (4 bits for 16 health levels)
        // - Battery level (4 bits for 16 battery levels)
        // - Carried items (8 bits for 8 different item types)

        // All this should fit in a single u32!
        // Implement encode/decode functions with bit manipulation

        println!("Encode these robot states into a single u32:");
        let states = [
            "Position(5,3), North, Speed3, Walking, Health15, Battery12, Items[Key,Tool]",
            "Position(12,7), Southeast, Speed7, Fighting, Health8, Battery3, Items[Weapon,Ammo]",
            "Position(0,15), West, Speed1, Scanning, Health15, Battery15, Items[Scanner,Data]",
        ];

        for state in states {
            println!("State: {}", state);
            println!("  Encoded: [YOUR IMPLEMENTATION HERE]");
        }
    }

    /// Exercise 5: Create a network protocol using bit flags
    pub fn exercise_network_protocol() {
        println!("\nExercise 5: Network Protocol with Bit Flags");

        // TODO: Design a communication protocol using bit flags:
        // - Message type (4 bits: status, command, data, emergency, etc.)
        // - Priority level (2 bits: low, normal, high, critical)
        // - Sender ID (6 bits: up to 64 robots)
        // - Recipient ID (6 bits: up to 64 robots, 0 = broadcast)
        // - Flags (8 bits: ack_required, encrypted, compressed, etc.)
        // - Sequence number (6 bits: for message ordering)

        // Implement message encoding/decoding and flag processing
        // Create functions to generate different message types

        println!("Protocol message format (32 bits total):");
        println!("  [31-28] Message Type");
        println!("  [27-26] Priority");
        println!("  [25-20] Sender ID");
        println!("  [19-14] Recipient ID");
        println!("  [13-6]  Flags");
        println!("  [5-0]   Sequence Number");

        // Test with different message scenarios
        let messages = [
            "Emergency broadcast from Robot 15, critical priority, ack required",
            "Status update from Robot 3 to Robot 7, normal priority, encrypted",
            "Command from Robot 1 to Robot 23, high priority, compressed",
        ];

        for message in messages {
            println!("Message: {}", message);
            println!("  Encoded: [YOUR IMPLEMENTATION HERE]");
        }
    }
}

fn main() {
    demonstrate_flag_operations();
    demonstrate_bit_utilities();

    println!("\n{}", "=".repeat(50));
    println!("STUDENT EXERCISES");
    println!("{}", "=".repeat(50));

    exercises::exercise_capability_system();
    exercises::exercise_mission_status();
    exercises::exercise_permission_inheritance();
    exercises::exercise_bit_field_encoding();
    exercises::exercise_network_protocol();
}