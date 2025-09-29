// Learning Tests for Level 18, Task 2: Unsafe Memory Management
// Managing manual memory allocation, deallocation, and lifetime tracking

use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::mem;

#[derive(Debug, Clone)]
pub struct RobotMemoryPool {
    pub layout: Layout,
    pub ptr: *mut u8,
    pub size: usize,
    pub used: usize,
    pub allocations: Vec<(usize, usize)>, // (offset, size)
}

impl RobotMemoryPool {
    pub fn new(size: usize) -> Result<Self, String> {
        if size == 0 || size > 1024 * 1024 {
            return Err("Invalid pool size".to_string());
        }

        let layout = Layout::from_size_align(size, 8)
            .map_err(|_| "Failed to create layout".to_string())?;

        let ptr = unsafe { alloc(layout) };

        if ptr.is_null() {
            return Err("Failed to allocate memory".to_string());
        }

        Ok(RobotMemoryPool {
            layout,
            ptr,
            size,
            used: 0,
            allocations: Vec::new(),
        })
    }

    pub unsafe fn allocate(&mut self, size: usize) -> Result<*mut u8, String> {
        if size == 0 {
            return Err("Cannot allocate zero bytes".to_string());
        }

        let aligned_size = (size + 7) & !7; // Align to 8 bytes

        if self.used + aligned_size > self.size {
            return Err("Not enough memory in pool".to_string());
        }

        let offset = self.used;
        self.used += aligned_size;
        self.allocations.push((offset, size));

        Ok(self.ptr.add(offset))
    }

    pub unsafe fn deallocate(&mut self, ptr: *mut u8) -> Result<(), String> {
        let offset = ptr.offset_from(self.ptr) as usize;

        // Find and remove allocation
        let mut found = false;
        self.allocations.retain(|(alloc_offset, _)| {
            if *alloc_offset == offset {
                found = true;
                false
            } else {
                true
            }
        });

        if !found {
            return Err("Invalid pointer for deallocation".to_string());
        }

        // Simple deallocation - just mark as free (no compaction)
        Ok(())
    }

    pub unsafe fn write<T>(&self, ptr: *mut u8, value: T) -> Result<(), String> {
        if ptr.is_null() {
            return Err("Cannot write to null pointer".to_string());
        }

        // Verify pointer is within our pool
        let offset = ptr.offset_from(self.ptr) as usize;
        if offset >= self.size {
            return Err("Pointer outside memory pool".to_string());
        }

        ptr::write(ptr as *mut T, value);
        Ok(())
    }

    pub unsafe fn read<T>(&self, ptr: *const u8) -> Result<T, String> {
        if ptr.is_null() {
            return Err("Cannot read from null pointer".to_string());
        }

        // Verify pointer is within our pool
        let offset = ptr.offset_from(self.ptr) as usize;
        if offset >= self.size {
            return Err("Pointer outside memory pool".to_string());
        }

        Ok(ptr::read(ptr as *const T))
    }

    pub fn get_statistics(&self) -> (usize, usize, usize) {
        (self.used, self.size - self.used, self.allocations.len())
    }
}

impl Drop for RobotMemoryPool {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}

// Custom allocator for robot systems
pub struct RobotAllocator {
    pools: Vec<RobotMemoryPool>,
}

impl RobotAllocator {
    pub fn new() -> Self {
        RobotAllocator {
            pools: Vec::new(),
        }
    }

    pub fn create_pool(&mut self, size: usize) -> Result<usize, String> {
        let pool = RobotMemoryPool::new(size)?;
        self.pools.push(pool);
        Ok(self.pools.len() - 1)
    }

    pub unsafe fn allocate_in_pool(&mut self, pool_id: usize, size: usize) -> Result<*mut u8, String> {
        self.pools.get_mut(pool_id)
            .ok_or_else(|| "Invalid pool ID".to_string())?
            .allocate(size)
    }

    pub unsafe fn deallocate_in_pool(&mut self, pool_id: usize, ptr: *mut u8) -> Result<(), String> {
        self.pools.get_mut(pool_id)
            .ok_or_else(|| "Invalid pool ID".to_string())?
            .deallocate(ptr)
    }

    pub fn get_pool_stats(&self, pool_id: usize) -> Result<(usize, usize, usize), String> {
        self.pools.get(pool_id)
            .map(|p| p.get_statistics())
            .ok_or_else(|| "Invalid pool ID".to_string())
    }
}

// Memory-mapped robot registers
#[repr(C)]
pub struct RobotRegisters {
    pub status: u32,
    pub position_x: i32,
    pub position_y: i32,
    pub energy: u32,
    pub mission_id: u64,
}

impl RobotRegisters {
    pub unsafe fn from_raw_ptr(ptr: *mut u8) -> &'static mut Self {
        &mut *(ptr as *mut RobotRegisters)
    }

    pub unsafe fn write_to_ptr(&self, ptr: *mut u8) {
        ptr::write(ptr as *mut RobotRegisters, *self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool_creation() {
        let pool = RobotMemoryPool::new(1024).unwrap();
        let (used, free, allocs) = pool.get_statistics();
        assert_eq!(used, 0);
        assert_eq!(free, 1024);
        assert_eq!(allocs, 0);
    }

    #[test]
    fn test_allocation_and_deallocation() {
        unsafe {
            let mut pool = RobotMemoryPool::new(256).unwrap();

            // Allocate memory
            let ptr1 = pool.allocate(32).unwrap();
            let ptr2 = pool.allocate(64).unwrap();

            // Write and read
            pool.write(ptr1, 42u32).unwrap();
            let value: u32 = pool.read(ptr1).unwrap();
            assert_eq!(value, 42);

            // Deallocate
            pool.deallocate(ptr1).unwrap();
            pool.deallocate(ptr2).unwrap();
        }
    }

    #[test]
    fn test_robot_allocator() {
        unsafe {
            let mut allocator = RobotAllocator::new();
            let pool_id = allocator.create_pool(512).unwrap();

            let ptr = allocator.allocate_in_pool(pool_id, 128).unwrap();
            assert!(!ptr.is_null());

            allocator.deallocate_in_pool(pool_id, ptr).unwrap();

            let stats = allocator.get_pool_stats(pool_id).unwrap();
            assert_eq!(stats.2, 0); // No active allocations
        }
    }

    #[test]
    fn test_robot_registers() {
        unsafe {
            let mut buffer = [0u8; mem::size_of::<RobotRegisters>()];
            let ptr = buffer.as_mut_ptr();

            let registers = RobotRegisters {
                status: 0x01,
                position_x: 100,
                position_y: 200,
                energy: 5000,
                mission_id: 0xDEADBEEF,
            };

            registers.write_to_ptr(ptr);

            let read_registers = RobotRegisters::from_raw_ptr(ptr);
            assert_eq!(read_registers.status, 0x01);
            assert_eq!(read_registers.position_x, 100);
            assert_eq!(read_registers.position_y, 200);
            assert_eq!(read_registers.energy, 5000);
            assert_eq!(read_registers.mission_id, 0xDEADBEEF);
        }
    }
}

// Student exercises
pub mod exercises {
    use super::*;

    // Exercise 1: Implement a stack allocator
    pub struct StackAllocator {
        buffer: Vec<u8>,
        top: usize,
    }

    impl StackAllocator {
        pub fn new(size: usize) -> Self {
            StackAllocator {
                buffer: vec![0; size],
                top: 0,
            }
        }

        pub unsafe fn allocate(&mut self, size: usize) -> Result<*mut u8, String> {
            // TODO: Implement stack allocation
            // Return pointer to allocated memory
            // Update top pointer
            unimplemented!("Implement stack allocation")
        }

        pub unsafe fn deallocate(&mut self, size: usize) -> Result<(), String> {
            // TODO: Implement stack deallocation
            // Only can deallocate from top
            unimplemented!("Implement stack deallocation")
        }
    }

    // Exercise 2: Implement a buddy allocator
    pub struct BuddyAllocator {
        memory: Vec<u8>,
        free_lists: Vec<Vec<usize>>, // Free lists for each size class
        min_size: usize,
    }

    impl BuddyAllocator {
        pub fn new(size: usize) -> Self {
            // TODO: Initialize buddy allocator with power-of-2 size
            unimplemented!("Initialize buddy allocator")
        }

        pub unsafe fn allocate(&mut self, size: usize) -> Result<*mut u8, String> {
            // TODO: Find smallest power-of-2 block that fits
            // Split larger blocks if necessary
            unimplemented!("Implement buddy allocation")
        }

        pub unsafe fn deallocate(&mut self, ptr: *mut u8, size: usize) -> Result<(), String> {
            // TODO: Free block and merge with buddy if both are free
            unimplemented!("Implement buddy deallocation")
        }
    }

    // Exercise 3: Implement a memory-mapped I/O interface
    pub struct MemoryMappedIO {
        base_address: *mut u8,
        size: usize,
    }

    impl MemoryMappedIO {
        pub unsafe fn new(base: *mut u8, size: usize) -> Self {
            MemoryMappedIO {
                base_address: base,
                size,
            }
        }

        pub unsafe fn read_register(&self, offset: usize) -> Result<u32, String> {
            // TODO: Read 32-bit value from base_address + offset
            unimplemented!("Read from memory-mapped register")
        }

        pub unsafe fn write_register(&mut self, offset: usize, value: u32) -> Result<(), String> {
            // TODO: Write 32-bit value to base_address + offset
            unimplemented!("Write to memory-mapped register")
        }
    }
}