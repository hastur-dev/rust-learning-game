// Learning Tests for Level 18, Task 3: Unsafe Transmutation
// Type transmutation, reinterpretation, and bit-level data manipulation

use std::mem;

// Robot data structures for transmutation exercises
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RobotState {
    pub id: u32,
    pub flags: u32,
    pub position: (f32, f32),
    pub energy: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RobotStateBytes {
    pub bytes: [u8; 20], // Same size as RobotState
}

// Bit flags for robot status
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RobotFlag {
    Active = 0b00000001,
    Armed = 0b00000010,
    Shielded = 0b00000100,
    Damaged = 0b00001000,
    LowEnergy = 0b00010000,
    Emergency = 0b00100000,
    Maintenance = 0b01000000,
    Offline = 0b10000000,
}

// Union for type punning
#[repr(C)]
pub union RobotData {
    pub as_u64: u64,
    pub as_f64: f64,
    pub as_bytes: [u8; 8],
    pub as_u32_pair: [u32; 2],
}

// Transmutation utilities
pub struct TransmutationSystem;

impl TransmutationSystem {
    // Safe wrapper around transmute for demonstration
    pub unsafe fn transmute_robot_state(state: RobotState) -> RobotStateBytes {
        mem::transmute(state)
    }

    pub unsafe fn transmute_bytes_to_state(bytes: RobotStateBytes) -> RobotState {
        mem::transmute(bytes)
    }

    // Float to bits conversion
    pub fn float_to_bits(f: f32) -> u32 {
        f.to_bits()
    }

    pub fn bits_to_float(bits: u32) -> f32 {
        f32::from_bits(bits)
    }

    // Double to bits conversion
    pub fn double_to_bits(f: f64) -> u64 {
        f.to_bits()
    }

    pub fn bits_to_double(bits: u64) -> f64 {
        f64::from_bits(bits)
    }

    // Type punning through pointers
    pub unsafe fn pun_u32_to_f32(value: &u32) -> f32 {
        *(value as *const u32 as *const f32)
    }

    pub unsafe fn pun_f32_to_u32(value: &f32) -> u32 {
        *(value as *const f32 as *const u32)
    }

    // Array transmutation
    pub unsafe fn transmute_array<T, U>(arr: &[T]) -> &[U] {
        let ptr = arr.as_ptr() as *const U;
        let len = arr.len() * mem::size_of::<T>() / mem::size_of::<U>();
        std::slice::from_raw_parts(ptr, len)
    }

    // Bit manipulation
    pub fn set_bit(value: u32, bit: u8) -> u32 {
        value | (1 << bit)
    }

    pub fn clear_bit(value: u32, bit: u8) -> u32 {
        value & !(1 << bit)
    }

    pub fn toggle_bit(value: u32, bit: u8) -> u32 {
        value ^ (1 << bit)
    }

    pub fn check_bit(value: u32, bit: u8) -> bool {
        (value & (1 << bit)) != 0
    }

    // Extract bit field
    pub fn extract_bits(value: u32, start: u8, length: u8) -> u32 {
        let mask = (1u32 << length) - 1;
        (value >> start) & mask
    }

    // Insert bit field
    pub fn insert_bits(value: u32, bits: u32, start: u8, length: u8) -> u32 {
        let mask = (1u32 << length) - 1;
        let cleared = value & !(mask << start);
        cleared | ((bits & mask) << start)
    }
}

// Advanced transmutation patterns
pub struct AdvancedTransmutation;

impl AdvancedTransmutation {
    // Slice casting
    pub unsafe fn cast_slice<T, U>(slice: &[T]) -> Result<&[U], String> {
        let ptr = slice.as_ptr();
        let len_bytes = slice.len() * mem::size_of::<T>();

        if len_bytes % mem::size_of::<U>() != 0 {
            return Err("Size mismatch for slice casting".to_string());
        }

        let new_len = len_bytes / mem::size_of::<U>();
        Ok(std::slice::from_raw_parts(ptr as *const U, new_len))
    }

    // Endianness conversion
    pub fn swap_bytes_u32(value: u32) -> u32 {
        value.swap_bytes()
    }

    pub fn to_big_endian(value: u32) -> [u8; 4] {
        value.to_be_bytes()
    }

    pub fn to_little_endian(value: u32) -> [u8; 4] {
        value.to_le_bytes()
    }

    pub fn from_bytes_be(bytes: [u8; 4]) -> u32 {
        u32::from_be_bytes(bytes)
    }

    pub fn from_bytes_le(bytes: [u8; 4]) -> u32 {
        u32::from_le_bytes(bytes)
    }

    // Zero-copy conversion
    pub unsafe fn zero_copy_convert<T: Copy>(data: &T) -> &[u8] {
        std::slice::from_raw_parts(
            data as *const T as *const u8,
            mem::size_of::<T>()
        )
    }

    // Struct to bytes
    pub unsafe fn struct_to_bytes<T>(s: &T) -> Vec<u8> {
        let ptr = s as *const T as *const u8;
        let slice = std::slice::from_raw_parts(ptr, mem::size_of::<T>());
        slice.to_vec()
    }

    // Bytes to struct
    pub unsafe fn bytes_to_struct<T>(bytes: &[u8]) -> Result<T, String> {
        if bytes.len() != mem::size_of::<T>() {
            return Err(format!("Expected {} bytes, got {}", mem::size_of::<T>(), bytes.len()));
        }
        Ok(ptr::read(bytes.as_ptr() as *const T))
    }
}

// Bit field manipulation
#[repr(C)]
pub struct BitFieldRobot {
    data: u32,
}

impl BitFieldRobot {
    pub fn new() -> Self {
        BitFieldRobot { data: 0 }
    }

    // ID: bits 0-7 (8 bits)
    pub fn get_id(&self) -> u8 {
        (self.data & 0xFF) as u8
    }

    pub fn set_id(&mut self, id: u8) {
        self.data = (self.data & !0xFF) | (id as u32);
    }

    // Status: bits 8-11 (4 bits)
    pub fn get_status(&self) -> u8 {
        ((self.data >> 8) & 0xF) as u8
    }

    pub fn set_status(&mut self, status: u8) {
        self.data = (self.data & !(0xF << 8)) | ((status as u32 & 0xF) << 8);
    }

    // Energy: bits 12-19 (8 bits)
    pub fn get_energy(&self) -> u8 {
        ((self.data >> 12) & 0xFF) as u8
    }

    pub fn set_energy(&mut self, energy: u8) {
        self.data = (self.data & !(0xFF << 12)) | ((energy as u32) << 12);
    }

    // Flags: bits 20-31 (12 bits)
    pub fn get_flags(&self) -> u16 {
        ((self.data >> 20) & 0xFFF) as u16
    }

    pub fn set_flags(&mut self, flags: u16) {
        self.data = (self.data & !(0xFFF << 20)) | ((flags as u32 & 0xFFF) << 20);
    }
}

use std::ptr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transmutation() {
        unsafe {
            let state = RobotState {
                id: 42,
                flags: 0b10101010,
                position: (10.5, 20.5),
                energy: 100.0,
            };

            let bytes = TransmutationSystem::transmute_robot_state(state);
            let recovered = TransmutationSystem::transmute_bytes_to_state(bytes);

            assert_eq!(state.id, recovered.id);
            assert_eq!(state.flags, recovered.flags);
            assert_eq!(state.position, recovered.position);
            assert_eq!(state.energy, recovered.energy);
        }
    }

    #[test]
    fn test_bit_manipulation() {
        let mut value = 0u32;

        value = TransmutationSystem::set_bit(value, 5);
        assert!(TransmutationSystem::check_bit(value, 5));

        value = TransmutationSystem::clear_bit(value, 5);
        assert!(!TransmutationSystem::check_bit(value, 5));

        value = TransmutationSystem::toggle_bit(value, 3);
        assert!(TransmutationSystem::check_bit(value, 3));
    }

    #[test]
    fn test_bit_fields() {
        let mut robot = BitFieldRobot::new();

        robot.set_id(123);
        robot.set_status(7);
        robot.set_energy(200);
        robot.set_flags(0xABC);

        assert_eq!(robot.get_id(), 123);
        assert_eq!(robot.get_status(), 7);
        assert_eq!(robot.get_energy(), 200);
        assert_eq!(robot.get_flags(), 0xABC);
    }

    #[test]
    fn test_endianness() {
        let value = 0x12345678u32;
        let swapped = AdvancedTransmutation::swap_bytes_u32(value);
        assert_eq!(swapped, 0x78563412);

        let be_bytes = AdvancedTransmutation::to_big_endian(value);
        assert_eq!(be_bytes, [0x12, 0x34, 0x56, 0x78]);

        let le_bytes = AdvancedTransmutation::to_little_endian(value);
        assert_eq!(le_bytes, [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_union_type_punning() {
        let data = RobotData { as_u64: 0xDEADBEEF_CAFEBABE };

        unsafe {
            assert_eq!(data.as_u64, 0xDEADBEEF_CAFEBABE);

            let bytes = data.as_bytes;
            assert_eq!(bytes[0], 0xBE);
            assert_eq!(bytes[1], 0xBA);

            let pairs = data.as_u32_pair;
            assert_eq!(pairs[0], 0xCAFEBABE);
            assert_eq!(pairs[1], 0xDEADBEEF);
        }
    }
}

// Student exercises
pub mod exercises {
    use super::*;

    // Exercise 1: Implement safe wrapper for type punning
    pub struct TypePunner;

    impl TypePunner {
        pub unsafe fn pun<T, U>(value: &T) -> &U {
            // TODO: Implement type punning from T to U
            // Ensure size compatibility
            unimplemented!("Implement safe type punning")
        }

        pub fn safe_transmute<T: Copy, U: Copy>(value: T) -> Result<U, String> {
            // TODO: Implement safe transmutation with size check
            unimplemented!("Implement safe transmutation")
        }
    }

    // Exercise 2: Implement packed bit struct
    #[repr(packed)]
    pub struct PackedRobot {
        // TODO: Define packed structure
        // Should contain id (16 bits), x (16 bits), y (16 bits), flags (16 bits)
    }

    impl PackedRobot {
        pub fn new(id: u16, x: i16, y: i16, flags: u16) -> Self {
            // TODO: Create packed robot
            unimplemented!("Create packed robot")
        }

        pub unsafe fn as_bytes(&self) -> &[u8; 8] {
            // TODO: Get bytes representation
            unimplemented!("Get bytes representation")
        }

        pub unsafe fn from_bytes(bytes: &[u8; 8]) -> Self {
            // TODO: Create from bytes
            unimplemented!("Create from bytes")
        }
    }

    // Exercise 3: Implement custom float representation
    pub struct CustomFloat {
        bits: u32,
    }

    impl CustomFloat {
        pub fn from_f32(f: f32) -> Self {
            // TODO: Convert f32 to custom representation
            unimplemented!("Convert from f32")
        }

        pub fn to_f32(&self) -> f32 {
            // TODO: Convert to f32
            unimplemented!("Convert to f32")
        }

        pub fn get_sign(&self) -> bool {
            // TODO: Extract sign bit (bit 31)
            unimplemented!("Get sign bit")
        }

        pub fn get_exponent(&self) -> u8 {
            // TODO: Extract exponent (bits 23-30)
            unimplemented!("Get exponent")
        }

        pub fn get_mantissa(&self) -> u32 {
            // TODO: Extract mantissa (bits 0-22)
            unimplemented!("Get mantissa")
        }
    }
}