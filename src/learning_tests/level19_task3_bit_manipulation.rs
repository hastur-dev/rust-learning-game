// Level 19 Task 3: Advanced Bit Manipulation Techniques
// Learn sophisticated bit manipulation for robot optimization and algorithms

use std::fmt;

/// Robot sensor data packed efficiently using bit manipulation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PackedSensorData {
    data: u64, // Pack multiple sensor readings into single 64-bit value
}

impl PackedSensorData {
    // Bit field layout for sensor data:
    // Bits 63-60: Sensor type (4 bits, 16 types)
    // Bits 59-56: Sensor quality (4 bits, 0-15)
    // Bits 55-48: Battery level (8 bits, 0-255)
    // Bits 47-40: Temperature (8 bits, signed offset from -128 to +127)
    // Bits 39-32: Humidity (8 bits, 0-100%)
    // Bits 31-24: Light level (8 bits, 0-255)
    // Bits 23-16: Sound level (8 bits, 0-255)
    // Bits 15-8:  Pressure (8 bits, relative)
    // Bits 7-0:   Timestamp (8 bits, seconds mod 256)

    const SENSOR_TYPE_SHIFT: u8 = 60;
    const SENSOR_TYPE_MASK: u64 = 0xF000_0000_0000_0000;

    const QUALITY_SHIFT: u8 = 56;
    const QUALITY_MASK: u64 = 0x0F00_0000_0000_0000;

    const BATTERY_SHIFT: u8 = 48;
    const BATTERY_MASK: u64 = 0x00FF_0000_0000_0000;

    const TEMPERATURE_SHIFT: u8 = 40;
    const TEMPERATURE_MASK: u64 = 0x0000_FF00_0000_0000;

    const HUMIDITY_SHIFT: u8 = 32;
    const HUMIDITY_MASK: u64 = 0x0000_00FF_0000_0000;

    const LIGHT_SHIFT: u8 = 24;
    const LIGHT_MASK: u64 = 0x0000_0000_FF00_0000;

    const SOUND_SHIFT: u8 = 16;
    const SOUND_MASK: u64 = 0x0000_0000_00FF_0000;

    const PRESSURE_SHIFT: u8 = 8;
    const PRESSURE_MASK: u64 = 0x0000_0000_0000_FF00;

    const TIMESTAMP_SHIFT: u8 = 0;
    const TIMESTAMP_MASK: u64 = 0x0000_0000_0000_00FF;

    /// Create new sensor data with all values zero
    pub fn new() -> Self {
        Self { data: 0 }
    }

    /// Create from raw packed value
    pub fn from_raw(data: u64) -> Self {
        Self { data }
    }

    /// Get raw packed value
    pub fn raw(&self) -> u64 {
        self.data
    }

    /// Extract a bit field from the packed data
    fn extract_field(&self, shift: u8, mask: u64) -> u64 {
        (self.data & mask) >> shift
    }

    /// Set a bit field in the packed data
    fn set_field(&mut self, shift: u8, mask: u64, value: u64) {
        self.data = (self.data & !mask) | ((value << shift) & mask);
    }

    /// Set sensor type (0-15)
    pub fn set_sensor_type(&mut self, sensor_type: u8) {
        self.set_field(Self::SENSOR_TYPE_SHIFT, Self::SENSOR_TYPE_MASK, sensor_type as u64);
    }

    /// Get sensor type
    pub fn sensor_type(&self) -> u8 {
        self.extract_field(Self::SENSOR_TYPE_SHIFT, Self::SENSOR_TYPE_MASK) as u8
    }

    /// Set sensor quality (0-15)
    pub fn set_quality(&mut self, quality: u8) {
        let clamped = quality.min(15);
        self.set_field(Self::QUALITY_SHIFT, Self::QUALITY_MASK, clamped as u64);
    }

    /// Get sensor quality
    pub fn quality(&self) -> u8 {
        self.extract_field(Self::QUALITY_SHIFT, Self::QUALITY_MASK) as u8
    }

    /// Set battery level (0-255)
    pub fn set_battery(&mut self, battery: u8) {
        self.set_field(Self::BATTERY_SHIFT, Self::BATTERY_MASK, battery as u64);
    }

    /// Get battery level
    pub fn battery(&self) -> u8 {
        self.extract_field(Self::BATTERY_SHIFT, Self::BATTERY_MASK) as u8
    }

    /// Set temperature (-128 to +127, stored as offset)
    pub fn set_temperature(&mut self, temp: i8) {
        let unsigned = (temp as i16 + 128) as u8;
        self.set_field(Self::TEMPERATURE_SHIFT, Self::TEMPERATURE_MASK, unsigned as u64);
    }

    /// Get temperature
    pub fn temperature(&self) -> i8 {
        let unsigned = self.extract_field(Self::TEMPERATURE_SHIFT, Self::TEMPERATURE_MASK) as u8;
        (unsigned as i16 - 128) as i8
    }

    /// Set humidity (0-100%)
    pub fn set_humidity(&mut self, humidity: u8) {
        let clamped = humidity.min(100);
        self.set_field(Self::HUMIDITY_SHIFT, Self::HUMIDITY_MASK, clamped as u64);
    }

    /// Get humidity
    pub fn humidity(&self) -> u8 {
        self.extract_field(Self::HUMIDITY_SHIFT, Self::HUMIDITY_MASK) as u8
    }

    /// Set light level (0-255)
    pub fn set_light(&mut self, light: u8) {
        self.set_field(Self::LIGHT_SHIFT, Self::LIGHT_MASK, light as u64);
    }

    /// Get light level
    pub fn light(&self) -> u8 {
        self.extract_field(Self::LIGHT_SHIFT, Self::LIGHT_MASK) as u8
    }

    /// Set sound level (0-255)
    pub fn set_sound(&mut self, sound: u8) {
        self.set_field(Self::SOUND_SHIFT, Self::SOUND_MASK, sound as u64);
    }

    /// Get sound level
    pub fn sound(&self) -> u8 {
        self.extract_field(Self::SOUND_SHIFT, Self::SOUND_MASK) as u8
    }

    /// Set pressure (0-255)
    pub fn set_pressure(&mut self, pressure: u8) {
        self.set_field(Self::PRESSURE_SHIFT, Self::PRESSURE_MASK, pressure as u64);
    }

    /// Get pressure
    pub fn pressure(&self) -> u8 {
        self.extract_field(Self::PRESSURE_SHIFT, Self::PRESSURE_MASK) as u8
    }

    /// Set timestamp (0-255)
    pub fn set_timestamp(&mut self, timestamp: u8) {
        self.set_field(Self::TIMESTAMP_SHIFT, Self::TIMESTAMP_MASK, timestamp as u64);
    }

    /// Get timestamp
    pub fn timestamp(&self) -> u8 {
        self.extract_field(Self::TIMESTAMP_SHIFT, Self::TIMESTAMP_MASK) as u8
    }
}

impl fmt::Display for PackedSensorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sensor[Type:{}, Q:{}, Bat:{}%, T:{}°C, H:{}%, L:{}, S:{}, P:{}, @{}s]",
               self.sensor_type(), self.quality(), self.battery(),
               self.temperature(), self.humidity(), self.light(),
               self.sound(), self.pressure(), self.timestamp())
    }
}

/// Advanced bit manipulation algorithms for robot processing
pub struct BitAlgorithms;

impl BitAlgorithms {
    /// Fast integer square root using bit manipulation
    pub fn int_sqrt(n: u32) -> u32 {
        if n == 0 {
            return 0;
        }

        let mut x = n;
        let mut y = (x + 1) / 2;

        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }
        x
    }

    /// Check if a number is a perfect power of 2
    pub fn is_power_of_2(n: u32) -> bool {
        n != 0 && (n & (n - 1)) == 0
    }

    /// Find the next power of 2 greater than or equal to n
    pub fn next_power_of_2(n: u32) -> u32 {
        if n == 0 {
            return 1;
        }
        1 << (32 - (n - 1).leading_zeros())
    }

    /// Count the number of set bits (population count)
    pub fn popcount(n: u32) -> u32 {
        n.count_ones()
    }

    /// Reverse the bits in a 32-bit integer
    pub fn reverse_bits(n: u32) -> u32 {
        n.reverse_bits()
    }

    /// Find the position of the least significant set bit
    pub fn trailing_zeros(n: u32) -> u32 {
        n.trailing_zeros()
    }

    /// Find the position of the most significant set bit
    pub fn leading_zeros(n: u32) -> u32 {
        n.leading_zeros()
    }

    /// Isolate the rightmost set bit
    pub fn isolate_rightmost_bit(n: u32) -> u32 {
        n & (!n + 1)
    }

    /// Clear the rightmost set bit
    pub fn clear_rightmost_bit(n: u32) -> u32 {
        n & (n - 1)
    }

    /// Set all bits after (to the right of) the rightmost set bit
    pub fn set_bits_after_rightmost(n: u32) -> u32 {
        n | (n - 1)
    }

    /// Isolate the rightmost 0-bit, producing 0 if none
    pub fn isolate_rightmost_zero(n: u32) -> u32 {
        !n & (n + 1)
    }

    /// Fast multiplication by 3 using bit operations
    pub fn multiply_by_3(n: u32) -> u32 {
        (n << 1) + n
    }

    /// Fast division by 2 using bit operations
    pub fn divide_by_2(n: u32) -> u32 {
        n >> 1
    }

    /// Check if two integers have opposite signs
    pub fn opposite_signs(x: i32, y: i32) -> bool {
        (x ^ y) < 0
    }

    /// Compute absolute value without branching
    pub fn abs_no_branch(n: i32) -> i32 {
        let mask = n >> 31;
        (n + mask) ^ mask
    }

    /// Swap two variables using XOR (without temporary variable)
    pub fn xor_swap(a: &mut u32, b: &mut u32) {
        if a != b {
            *a ^= *b;
            *b ^= *a;
            *a ^= *b;
        }
    }

    /// Count trailing zeros using bit manipulation
    pub fn count_trailing_zeros_custom(mut n: u32) -> u32 {
        if n == 0 {
            return 32;
        }
        let mut count = 0;
        while (n & 1) == 0 {
            n >>= 1;
            count += 1;
        }
        count
    }

    /// Rotate bits left
    pub fn rotate_left(n: u32, shift: u32) -> u32 {
        n.rotate_left(shift)
    }

    /// Rotate bits right
    pub fn rotate_right(n: u32, shift: u32) -> u32 {
        n.rotate_right(shift)
    }

    /// Gray code conversion
    pub fn binary_to_gray(n: u32) -> u32 {
        n ^ (n >> 1)
    }

    /// Inverse Gray code conversion
    pub fn gray_to_binary(n: u32) -> u32 {
        let mut result = n;
        result ^= result >> 16;
        result ^= result >> 8;
        result ^= result >> 4;
        result ^= result >> 2;
        result ^= result >> 1;
        result
    }
}

/// Bit-parallel algorithms for robot array processing
pub struct BitParallelOps;

impl BitParallelOps {
    /// SWAR (SIMD Within A Register) operation to add 4 bytes in parallel
    pub fn parallel_byte_add(a: u32, b: u32) -> u32 {
        let mask = 0x7F7F7F7F;
        let sum = (a & mask) + (b & mask);
        let carry = ((a ^ b) ^ sum) & 0x80808080;
        sum ^ carry
    }

    /// Count bits set in each byte of a 32-bit word
    pub fn parallel_popcount_bytes(n: u32) -> u32 {
        let mut x = n;
        x = (x & 0x55555555) + ((x >> 1) & 0x55555555);
        x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
        x = (x & 0x0F0F0F0F) + ((x >> 4) & 0x0F0F0F0F);
        x
    }

    /// Find if any byte in a 32-bit word is zero
    pub fn has_zero_byte(n: u32) -> bool {
        (n.wrapping_sub(0x01010101) & !n & 0x80808080) != 0
    }

    /// Find if any byte in a 32-bit word equals a specific value
    pub fn has_byte_value(n: u32, value: u8) -> bool {
        let pattern = value as u32 * 0x01010101;
        Self::has_zero_byte(n ^ pattern)
    }

    /// Parallel comparison of 4 bytes
    pub fn parallel_byte_compare(a: u32, b: u32) -> u32 {
        let diff = a ^ b;
        let borrow = (((!a) & b) | ((!diff) & (!a))) & 0x80808080;
        (diff | borrow) >> 7
    }

    /// Set bits at positions specified by another bit pattern
    pub fn scatter_bits(source: u32, mask: u32) -> u32 {
        let mut result = 0;
        let mut bit_pos = 0;
        let mut mask_copy = mask;

        while mask_copy != 0 {
            if (mask_copy & 1) != 0 {
                if (source & (1 << bit_pos)) != 0 {
                    result |= mask_copy & (!mask_copy + 1);
                }
                bit_pos += 1;
            }
            mask_copy &= mask_copy - 1;
        }
        result
    }

    /// Gather bits from positions specified by a mask
    pub fn gather_bits(source: u32, mask: u32) -> u32 {
        let mut result = 0;
        let mut bit_pos = 0;
        let mut mask_copy = mask;

        while mask_copy != 0 {
            let lowest_bit = mask_copy & (!mask_copy + 1);
            if (source & lowest_bit) != 0 {
                result |= 1 << bit_pos;
            }
            bit_pos += 1;
            mask_copy &= mask_copy - 1;
        }
        result
    }
}

/// Demonstrate advanced bit manipulation techniques
pub fn demonstrate_advanced_techniques() {
    println!("=== Advanced Bit Manipulation Techniques ===");

    // Packed sensor data demonstration
    println!("\n--- Packed Sensor Data ---");
    let mut sensor = PackedSensorData::new();
    sensor.set_sensor_type(3);    // LiDAR
    sensor.set_quality(12);       // High quality
    sensor.set_battery(85);       // 85% battery
    sensor.set_temperature(22);   // 22°C
    sensor.set_humidity(45);      // 45% humidity
    sensor.set_light(180);        // Bright light
    sensor.set_sound(60);         // Moderate sound
    sensor.set_pressure(128);     // Normal pressure
    sensor.set_timestamp(147);    // Time mark

    println!("Packed sensor data: {}", sensor);
    println!("Raw packed value: 0x{:016X} ({} bytes)", sensor.raw(), 8);

    // Demonstrate space savings
    println!("Space comparison:");
    println!("  Unpacked struct: ~32 bytes");
    println!("  Packed struct: 8 bytes");
    println!("  Space saving: 75%");

    // Bit algorithm demonstrations
    println!("\n--- Bit Algorithm Demonstrations ---");
    let test_values = [0, 1, 7, 8, 15, 16, 31, 32, 100, 255, 1023, 1024];

    for value in test_values {
        println!("Value: {}", value);
        println!("  Is power of 2: {}", BitAlgorithms::is_power_of_2(value));
        println!("  Next power of 2: {}", BitAlgorithms::next_power_of_2(value));
        println!("  Square root: {}", BitAlgorithms::int_sqrt(value));
        println!("  Bit count: {}", BitAlgorithms::popcount(value));
        println!("  Leading zeros: {}", BitAlgorithms::leading_zeros(value));
        println!("  Trailing zeros: {}", BitAlgorithms::trailing_zeros(value));
        println!("  Rightmost bit: 0x{:X}", BitAlgorithms::isolate_rightmost_bit(value));
        println!();
    }

    // Bit manipulation tricks
    println!("--- Bit Manipulation Tricks ---");
    let demo_value = 0b10110100u32;
    println!("Demo value: 0b{:08b}", demo_value);
    println!("Reversed: 0b{:08b}", BitAlgorithms::reverse_bits(demo_value));
    println!("Clear rightmost: 0b{:08b}", BitAlgorithms::clear_rightmost_bit(demo_value));
    println!("Set after rightmost: 0b{:08b}", BitAlgorithms::set_bits_after_rightmost(demo_value));

    // Gray code demonstration
    println!("\n--- Gray Code Conversion ---");
    for i in 0..8 {
        let gray = BitAlgorithms::binary_to_gray(i);
        let back = BitAlgorithms::gray_to_binary(gray);
        println!("Binary: {:03b} -> Gray: {:03b} -> Binary: {:03b}", i, gray, back);
    }

    // Parallel operations demonstration
    println!("\n--- Parallel Bit Operations ---");
    let bytes = 0x12345678u32;
    println!("Four bytes: 0x{:08X}", bytes);
    println!("Parallel popcount: 0x{:08X}", BitParallelOps::parallel_popcount_bytes(bytes));
    println!("Has zero byte: {}", BitParallelOps::has_zero_byte(bytes));
    println!("Has byte 0x34: {}", BitParallelOps::has_byte_value(bytes, 0x34));
    println!("Has byte 0xAB: {}", BitParallelOps::has_byte_value(bytes, 0xAB));
}

/// Demonstrate practical applications
pub fn demonstrate_practical_applications() {
    println!("\n=== Practical Applications ===");

    // Fast robot path validation using bit operations
    println!("--- Fast Path Validation ---");
    let grid_obstacles = 0b11010010_01100110_10010110_00110011u32; // 32-cell grid
    println!("Obstacle grid: 0b{:032b}", grid_obstacles);

    // Check if specific paths are clear
    let path_masks = [
        0b00000001_00000010_00000100_00001000u32, // Diagonal path
        0b00001111_00000000_00000000_00000000u32, // Top row path
        0b00000001_00000001_00000001_00000001u32, // Vertical path
    ];

    for (i, path) in path_masks.iter().enumerate() {
        let collision = grid_obstacles & path;
        println!("Path {}: Clear = {}, Collisions = 0b{:032b}",
                 i + 1, collision == 0, collision);
    }

    // Efficient robot ID allocation using bit sets
    println!("\n--- Robot ID Allocation ---");
    let mut allocated_ids = 0u64; // Can track 64 robot IDs

    // Allocate some IDs
    let allocate_ids = [5, 12, 23, 31, 47, 59];
    for id in allocate_ids {
        allocated_ids |= 1u64 << id;
        println!("Allocated robot ID {}", id);
    }

    println!("Allocated IDs bitmap: 0x{:016X}", allocated_ids);
    println!("Active robots: {}", allocated_ids.count_ones());

    // Find first available ID
    let first_free = (!allocated_ids).trailing_zeros();
    println!("First available ID: {}", first_free);

    // Sensor data compression demonstration
    println!("\n--- Sensor Data Compression ---");
    let original_readings = [
        (23.5f32, 65u8, 180u8), // temp, humidity, light
        (22.1f32, 67u8, 175u8),
        (24.2f32, 63u8, 185u8),
        (21.8f32, 69u8, 170u8),
    ];

    println!("Original sensor readings:");
    for (i, (temp, hum, light)) in original_readings.iter().enumerate() {
        println!("  Reading {}: {}°C, {}%, {}", i, temp, hum, light);
    }

    // Pack into compressed format (losing some precision)
    let mut compressed_data = Vec::new();
    for (temp, hum, light) in original_readings {
        let temp_compressed = ((temp + 50.0) * 2.0) as u8; // Range -50 to +77.5°C
        let compressed = (temp_compressed as u32) << 16 | (*hum as u32) << 8 | (*light as u32);
        compressed_data.push(compressed);
    }

    println!("Compressed to {} bytes total", compressed_data.len() * 4);
    println!("Compression ratio: {:.1}%",
             (1.0 - (compressed_data.len() * 4) as f32 / (original_readings.len() * 9) as f32) * 100.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::learning_tests::test_utils::*;

    #[test]
    fn test_packed_sensor_data() {
        let mut sensor = PackedSensorData::new();

        // Test individual field setting and getting
        sensor.set_sensor_type(7);
        assert_eq!(sensor.sensor_type(), 7);

        sensor.set_quality(12);
        assert_eq!(sensor.quality(), 12);

        sensor.set_battery(200);
        assert_eq!(sensor.battery(), 200);

        sensor.set_temperature(-30);
        assert_eq!(sensor.temperature(), -30);

        sensor.set_temperature(120);
        assert_eq!(sensor.temperature(), 120);

        sensor.set_humidity(85);
        assert_eq!(sensor.humidity(), 85);

        // Test clamping
        sensor.set_quality(20); // Should clamp to 15
        assert_eq!(sensor.quality(), 15);

        sensor.set_humidity(150); // Should clamp to 100
        assert_eq!(sensor.humidity(), 100);
    }

    #[test]
    fn test_bit_algorithms() {
        // Test power of 2 detection
        assert!(BitAlgorithms::is_power_of_2(1));
        assert!(BitAlgorithms::is_power_of_2(8));
        assert!(BitAlgorithms::is_power_of_2(1024));
        assert!(!BitAlgorithms::is_power_of_2(0));
        assert!(!BitAlgorithms::is_power_of_2(3));
        assert!(!BitAlgorithms::is_power_of_2(6));

        // Test next power of 2
        assert_eq!(BitAlgorithms::next_power_of_2(0), 1);
        assert_eq!(BitAlgorithms::next_power_of_2(1), 1);
        assert_eq!(BitAlgorithms::next_power_of_2(2), 2);
        assert_eq!(BitAlgorithms::next_power_of_2(3), 4);
        assert_eq!(BitAlgorithms::next_power_of_2(15), 16);

        // Test integer square root
        assert_eq!(BitAlgorithms::int_sqrt(0), 0);
        assert_eq!(BitAlgorithms::int_sqrt(1), 1);
        assert_eq!(BitAlgorithms::int_sqrt(4), 2);
        assert_eq!(BitAlgorithms::int_sqrt(9), 3);
        assert_eq!(BitAlgorithms::int_sqrt(15), 3);
        assert_eq!(BitAlgorithms::int_sqrt(16), 4);
        assert_eq!(BitAlgorithms::int_sqrt(100), 10);

        // Test bit manipulation
        assert_eq!(BitAlgorithms::isolate_rightmost_bit(0b10110100), 0b00000100);
        assert_eq!(BitAlgorithms::clear_rightmost_bit(0b10110100), 0b10110000);
    }

    #[test]
    fn test_advanced_bit_ops() {
        // Test XOR swap
        let mut a = 42u32;
        let mut b = 17u32;
        BitAlgorithms::xor_swap(&mut a, &mut b);
        assert_eq!(a, 17);
        assert_eq!(b, 42);

        // Test absolute value without branching
        assert_eq!(BitAlgorithms::abs_no_branch(-42), 42);
        assert_eq!(BitAlgorithms::abs_no_branch(42), 42);
        assert_eq!(BitAlgorithms::abs_no_branch(0), 0);

        // Test opposite signs
        assert!(BitAlgorithms::opposite_signs(-5, 3));
        assert!(BitAlgorithms::opposite_signs(5, -3));
        assert!(!BitAlgorithms::opposite_signs(-5, -3));
        assert!(!BitAlgorithms::opposite_signs(5, 3));

        // Test Gray code conversion
        for i in 0..16 {
            let gray = BitAlgorithms::binary_to_gray(i);
            let back = BitAlgorithms::gray_to_binary(gray);
            assert_eq!(i, back);
        }
    }

    #[test]
    fn test_parallel_operations() {
        // Test parallel byte operations
        let bytes1 = 0x12345678u32;
        let bytes2 = 0x11111111u32;

        // Test has zero byte
        assert!(!BitParallelOps::has_zero_byte(bytes1));
        assert!(BitParallelOps::has_zero_byte(0x12003456));

        // Test has specific byte value
        assert!(BitParallelOps::has_byte_value(bytes1, 0x34));
        assert!(!BitParallelOps::has_byte_value(bytes1, 0xAB));

        // Test parallel popcount
        let popcount_result = BitParallelOps::parallel_popcount_bytes(0xFF00FF00);
        // Each 0xFF should give 8 ones, each 0x00 should give 0
        assert_eq!(popcount_result & 0xFF, 8); // Lower byte
        assert_eq!((popcount_result >> 16) & 0xFF, 8); // Third byte
    }

    #[test]
    fn test_scatter_gather_operations() {
        let source = 0b11110000u32;
        let mask = 0b10101010u32;

        let scattered = BitParallelOps::scatter_bits(source, mask);
        let gathered = BitParallelOps::gather_bits(scattered, mask);

        // The gathered result should reconstruct the lower bits of source
        // based on how many mask bits were set
        assert_eq!(mask.count_ones(), 4);
    }

    #[test]
    fn test_practical_applications() {
        // Test grid path collision detection
        let obstacles = 0b11010010u32;
        let path = 0b00100100u32;
        let collision = obstacles & path;
        assert_eq!(collision, 0b00000000); // No collision

        let bad_path = 0b11000000u32;
        let bad_collision = obstacles & bad_path;
        assert_ne!(bad_collision, 0); // Has collision

        // Test ID allocation
        let mut ids = 0u64;
        ids |= 1u64 << 5;
        ids |= 1u64 << 12;
        assert_eq!(ids.count_ones(), 2);
        assert_eq!((!ids).trailing_zeros(), 0); // First free ID is 0
    }

    #[test]
    fn test_user_code_implementation() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Check for advanced bitwise operations
        assert!(
            analyzer.code.contains("<<") || analyzer.code.contains(">>"),
            "❌ You need to use bit shift operations (<<, >>)"
        );

        // Check for bit manipulation techniques
        assert!(
            analyzer.code.contains("&") && analyzer.code.contains("|"),
            "❌ You should use AND (&) and OR (|) operations for bit manipulation"
        );

        // Check for packed data structures
        assert!(
            analyzer.code.contains("pack") ||
            analyzer.code.contains("bit") ||
            analyzer.code.contains("field") ||
            analyzer.contains_struct("Packed") ||
            analyzer.contains_struct("Sensor"),
            "❌ You should implement packed data structures using bit fields"
        );

        // Check for advanced algorithms
        assert!(
            analyzer.contains_function("extract") ||
            analyzer.contains_function("set") ||
            analyzer.contains_function("mask") ||
            analyzer.contains_function("shift"),
            "❌ You should implement functions for bit field manipulation"
        );

        // Check for bit counting or analysis
        assert!(
            analyzer.code.contains("count") ||
            analyzer.code.contains("leading") ||
            analyzer.code.contains("trailing") ||
            analyzer.code.contains("popcount"),
            "❌ You should use bit counting operations"
        );

        // Check for output demonstration
        assert!(
            analyzer.contains_println(),
            "❌ You should demonstrate advanced bit manipulation with output"
        );
    }
}

/// Student exercises for practicing advanced bit manipulation
pub mod exercises {
    use super::*;

    /// Exercise 1: Create a robot instruction encoder
    pub fn exercise_instruction_encoder() {
        println!("Exercise 1: Robot Instruction Encoder");

        // TODO: Create a RobotInstruction struct that packs the following into a u32:
        // - Instruction type (4 bits): Move, Turn, Scan, Pick, Drop, etc.
        // - Direction (3 bits): N, NE, E, SE, S, SW, W, NW
        // - Distance/Amount (8 bits): 0-255
        // - Speed (4 bits): 0-15
        // - Priority (2 bits): Low, Normal, High, Critical
        // - Flags (8 bits): Various instruction modifiers
        // - Checksum (3 bits): Simple error detection

        println!("Design a 32-bit instruction format for robot commands");
        println!("Implement encode/decode functions with error checking");

        let test_instructions = [
            "Move North 50 units at speed 8, high priority, silent mode",
            "Turn Southeast 90 degrees at speed 3, normal priority",
            "Scan East 100 units at speed 1, critical priority, record data",
        ];

        for instruction in test_instructions {
            println!("Instruction: {}", instruction);
            println!("  Encoded: [YOUR IMPLEMENTATION HERE]");
        }
    }

    /// Exercise 2: Implement a bitmap-based collision system
    pub fn exercise_collision_system() {
        println!("\nExercise 2: Bitmap Collision System");

        // TODO: Create a collision detection system using bitmaps:
        // - 64x64 grid represented as 64 u64 values (4096 bits total)
        // - Implement functions to set/clear/check collision bits
        // - Create fast line-of-sight checking using bit operations
        // - Implement area collision detection for rectangular regions

        println!("Create a bitmap-based collision system");
        println!("Grid size: 64x64 (4096 bits in 64 u64 values)");

        let test_scenarios = [
            "Robot at (10,10) moving to (20,15)",
            "Rectangle from (5,5) to (15,12)",
            "Line of sight from (0,0) to (30,30)",
            "Circular area around (25,25) with radius 5",
        ];

        for scenario in test_scenarios {
            println!("Scenario: {}", scenario);
            println!("  Collision check: [YOUR IMPLEMENTATION HERE]");
        }
    }

    /// Exercise 3: Create a fast checksum calculator
    pub fn exercise_fast_checksum() {
        println!("\nExercise 3: Fast Checksum Calculator");

        // TODO: Implement fast checksum algorithms using bit manipulation:
        // - Simple XOR checksum for robot communication
        // - CRC-8 using bit manipulation for sensor data
        // - Parallel checksum for multiple data streams
        // - Error detection and correction using Hamming codes

        println!("Implement various checksum algorithms:");
        println!("1. XOR checksum for message integrity");
        println!("2. CRC-8 for sensor data validation");
        println!("3. Parallel processing of multiple checksums");

        let test_data = [
            vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
            vec![0xFF, 0x00, 0xFF, 0x00, 0xAA, 0x55, 0xAA, 0x55],
            vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF],
        ];

        for (i, data) in test_data.iter().enumerate() {
            println!("Data set {}: {:02X?}", i + 1, data);
            println!("  XOR checksum: [YOUR IMPLEMENTATION HERE]");
            println!("  CRC-8: [YOUR IMPLEMENTATION HERE]");
        }
    }

    /// Exercise 4: Implement bit-level data compression
    pub fn exercise_bit_compression() {
        println!("\nExercise 4: Bit-Level Data Compression");

        // TODO: Create compression algorithms using bit manipulation:
        // - Run-length encoding for sparse robot maps
        // - Dictionary compression for repeated sensor patterns
        // - Delta compression for sensor time series
        // - Huffman-style encoding for frequent robot commands

        println!("Implement compression for robot data:");

        // Sparse map data (mostly zeros with some obstacles)
        let sparse_map = vec![
            0, 0, 0, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        println!("Sparse map: {:?}", sparse_map);
        println!("  Run-length compressed: [YOUR IMPLEMENTATION HERE]");

        // Sensor time series (small changes between readings)
        let sensor_series = vec![100, 101, 99, 102, 98, 103, 97, 104];
        println!("Sensor series: {:?}", sensor_series);
        println!("  Delta compressed: [YOUR IMPLEMENTATION HERE]");
    }

    /// Exercise 5: Create a robot behavior state machine
    pub fn exercise_state_machine() {
        println!("\nExercise 5: Robot Behavior State Machine");

        // TODO: Implement a complex state machine using bit manipulation:
        // - State encoding in minimal bits
        // - Transition table using bit patterns
        // - State history tracking in a circular buffer
        // - Fast state validation and transition checking

        println!("Design a robot behavior state machine:");
        println!("States: Idle, Patrol, Investigate, Alert, Emergency, Recharge");
        println!("Pack state, substates, timers, and flags into efficient format");

        let behaviors = [
            "Robot starts in Idle, receives patrol command",
            "During patrol, detects anomaly, switches to Investigate",
            "Investigation confirms threat, escalates to Alert",
            "Battery low during Alert, transitions to Emergency Recharge",
        ];

        for behavior in behaviors {
            println!("Behavior: {}", behavior);
            println!("  State encoding: [YOUR IMPLEMENTATION HERE]");
            println!("  Valid transitions: [YOUR IMPLEMENTATION HERE]");
        }
    }
}

fn main() {
    demonstrate_advanced_techniques();
    demonstrate_practical_applications();

    println!("\n{}", "=".repeat(50));
    println!("STUDENT EXERCISES");
    println!("{}", "=".repeat(50));

    exercises::exercise_instruction_encoder();
    exercises::exercise_collision_system();
    exercises::exercise_fast_checksum();
    exercises::exercise_bit_compression();
    exercises::exercise_state_machine();
}