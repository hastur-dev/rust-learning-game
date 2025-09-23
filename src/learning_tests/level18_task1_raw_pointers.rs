// Learning Tests for Level 18, Task 1: Raw Pointer Operations
// Mastering raw pointer creation, dereferencing, and arithmetic in unsafe blocks

use std::ptr;

// Memory bank structure for robot identity recovery
#[derive(Debug, Clone, Copy)]
pub struct MemoryBank {
    pub address: usize,
    pub data: [u8; 16],
    pub size: usize,
}

impl MemoryBank {
    pub fn new(address: usize, data: &[u8]) -> Self {
        let mut bank_data = [0u8; 16];
        let copy_len = data.len().min(16);
        bank_data[..copy_len].copy_from_slice(&data[..copy_len]);

        MemoryBank {
            address,
            data: bank_data,
            size: copy_len,
        }
    }

    // Create a memory bank with corrupted robot data
    pub fn corrupted_robot_data() -> Self {
        // Robot ID: 1145259586 (as bytes)
        let robot_id_bytes = [0x42, 0x43, 0x44, 0x45, 0x00, 0x00, 0x00, 0x00];
        MemoryBank::new(0x1000, &robot_id_bytes)
    }

    pub fn corrupted_mission_data() -> Self {
        // Mission code: "Hello!"
        let mission_bytes = [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x21, 0x00, 0x00];
        MemoryBank::new(0x1004, &mission_bytes)
    }
}

// Raw pointer operations for memory recovery
pub struct MemoryRecoverySystem {
    memory_banks: Vec<MemoryBank>,
}

impl MemoryRecoverySystem {
    pub fn new() -> Self {
        MemoryRecoverySystem {
            memory_banks: Vec::new(),
        }
    }

    // Add corrupted memory banks
    pub fn add_corrupted_data(&mut self) {
        self.memory_banks.push(MemoryBank::corrupted_robot_data());
        self.memory_banks.push(MemoryBank::corrupted_mission_data());

        // Additional corrupted data
        let extra_data = [0x52, 0x4F, 0x42, 0x4F, 0x54, 0x00, 0x00, 0x00]; // "ROBOT"
        self.memory_banks.push(MemoryBank::new(0x1008, &extra_data));
    }

    // Create raw const pointer to memory bank data
    pub fn create_const_pointer(&self, bank_index: usize) -> *const u8 {
        if bank_index < self.memory_banks.len() {
            self.memory_banks[bank_index].data.as_ptr()
        } else {
            ptr::null()
        }
    }

    // Create raw mutable pointer to memory bank data
    pub fn create_mut_pointer(&mut self, bank_index: usize) -> *mut u8 {
        if bank_index < self.memory_banks.len() {
            self.memory_banks[bank_index].data.as_mut_ptr()
        } else {
            ptr::null_mut()
        }
    }

    // Safely dereference pointer to read single byte
    pub unsafe fn read_byte_at_pointer(ptr: *const u8, offset: usize) -> Option<u8> {
        if ptr.is_null() {
            return None;
        }

        // Use pointer arithmetic to get byte at offset
        let target_ptr = ptr.add(offset);
        Some(*target_ptr)
    }

    // Extract robot ID from memory using raw pointers
    pub fn extract_robot_id(&self) -> Option<u32> {
        if self.memory_banks.is_empty() {
            return None;
        }

        let ptr = self.create_const_pointer(0);
        unsafe {
            if ptr.is_null() {
                return None;
            }

            // Read 4 bytes as u32 (little endian)
            let b0 = *ptr;
            let b1 = *ptr.add(1);
            let b2 = *ptr.add(2);
            let b3 = *ptr.add(3);

            let robot_id = u32::from_le_bytes([b0, b1, b2, b3]);
            Some(robot_id)
        }
    }

    // Extract mission string from memory using raw pointers
    pub fn extract_mission_code(&self) -> Option<String> {
        if self.memory_banks.len() < 2 {
            return None;
        }

        let ptr = self.create_const_pointer(1);
        unsafe {
            if ptr.is_null() {
                return None;
            }

            // Read bytes until null terminator or max length
            let mut bytes = Vec::new();
            for i in 0..16 {
                let byte_ptr = ptr.add(i);
                let byte = *byte_ptr;
                if byte == 0 {
                    break;
                }
                bytes.push(byte);
            }

            String::from_utf8(bytes).ok()
        }
    }
}

// Comprehensive memory recovery demonstration
pub fn comprehensive_memory_recovery_demo() {
    println!("=== ROBOT MEMORY RECOVERY SYSTEM ===");

    let mut recovery_system = MemoryRecoverySystem::new();
    recovery_system.add_corrupted_data();

    println!("Memory banks loaded: {}", recovery_system.memory_banks.len());

    // Extract robot identity using raw pointers
    if let Some(robot_id) = recovery_system.extract_robot_id() {
        println!("✓ Robot ID recovered: {}", robot_id);
    } else {
        println!("✗ Failed to recover robot ID");
    }

    // Extract mission code
    if let Some(mission_code) = recovery_system.extract_mission_code() {
        println!("✓ Mission code recovered: '{}'", mission_code);
    } else {
        println!("✗ Failed to recover mission code");
    }

    println!("🏆 Robot identity restored! Memory management successful!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_bank_creation() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let bank = MemoryBank::new(0x1000, &data);

        assert_eq!(bank.address, 0x1000);
        assert_eq!(bank.size, 4);
        assert_eq!(&bank.data[..4], &data);
    }

    #[test]
    fn test_corrupted_data_creation() {
        let robot_data = MemoryBank::corrupted_robot_data();
        let mission_data = MemoryBank::corrupted_mission_data();

        assert_eq!(robot_data.address, 0x1000);
        assert_eq!(mission_data.address, 0x1004);
        assert!(robot_data.size > 0);
        assert!(mission_data.size > 0);
    }

    #[test]
    fn test_memory_recovery_system_creation() {
        let mut system = MemoryRecoverySystem::new();
        assert_eq!(system.memory_banks.len(), 0);

        system.add_corrupted_data();
        assert_eq!(system.memory_banks.len(), 3);
    }

    #[test]
    fn test_const_pointer_creation() {
        let mut system = MemoryRecoverySystem::new();
        system.add_corrupted_data();

        let ptr = system.create_const_pointer(0);
        assert!(!ptr.is_null());

        let null_ptr = system.create_const_pointer(999);
        assert!(null_ptr.is_null());
    }

    #[test]
    fn test_mut_pointer_creation() {
        let mut system = MemoryRecoverySystem::new();
        system.add_corrupted_data();

        let ptr = system.create_mut_pointer(0);
        assert!(!ptr.is_null());

        let null_ptr = system.create_mut_pointer(999);
        assert!(null_ptr.is_null());
    }

    #[test]
    fn test_unsafe_byte_reading() {
        let mut system = MemoryRecoverySystem::new();
        system.add_corrupted_data();

        let ptr = system.create_const_pointer(0);
        unsafe {
            let byte = MemoryRecoverySystem::read_byte_at_pointer(ptr, 0);
            assert!(byte.is_some());
            assert_eq!(byte.unwrap(), 0x42);

            let null_byte = MemoryRecoverySystem::read_byte_at_pointer(ptr::null(), 0);
            assert!(null_byte.is_none());
        }
    }

    #[test]
    fn test_robot_id_extraction() {
        let mut system = MemoryRecoverySystem::new();
        system.add_corrupted_data();

        let robot_id = system.extract_robot_id();
        assert!(robot_id.is_some());

        let empty_system = MemoryRecoverySystem::new();
        let no_id = empty_system.extract_robot_id();
        assert!(no_id.is_none());
    }

    #[test]
    fn test_mission_code_extraction() {
        let mut system = MemoryRecoverySystem::new();
        system.add_corrupted_data();

        let mission_code = system.extract_mission_code();
        assert!(mission_code.is_some());
        let code = mission_code.unwrap();
        assert!(code.contains("Hello"));

        let empty_system = MemoryRecoverySystem::new();
        let no_code = empty_system.extract_mission_code();
        assert!(no_code.is_none());
    }

    #[test]
    fn test_comprehensive_memory_recovery_demo() {
        // This test ensures the demo function runs without panicking
        comprehensive_memory_recovery_demo();
    }
}