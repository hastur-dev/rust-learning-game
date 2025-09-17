// Level 18 Task 1 Test: Raw Pointer Operations
// Tests that user implements raw pointer creation, dereferencing, and arithmetic

#[cfg(test)]
mod level18_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_raw_pointer_creation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_raw_ptr = analyzer.code.contains("*const") || analyzer.code.contains("*mut");
        assert!(
            has_raw_ptr,
            "❌ You need to create raw pointers using *const T or *mut T"
        );
    }

    #[test]
    fn test_uses_unsafe_blocks() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("unsafe"),
            "❌ You need to use unsafe blocks for raw pointer operations"
        );
    }

    #[test]
    fn test_dereferences_raw_pointers() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_deref = analyzer.code.contains("*ptr") ||
                       analyzer.code.contains("*raw_ptr") ||
                       analyzer.code.contains("ptr.read()");
        assert!(
            has_deref,
            "❌ You should dereference raw pointers to access memory"
        );
    }

    #[test]
    fn test_implements_pointer_arithmetic() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_arithmetic = analyzer.code.contains(".offset(") ||
                           analyzer.code.contains(".add(") ||
                           analyzer.code.contains("ptr +") ||
                           analyzer.code.contains("ptr -");
        assert!(
            has_arithmetic,
            "❌ You should use pointer arithmetic to traverse memory"
        );
    }

    #[test]
    fn test_handles_memory_addresses() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_addresses = analyzer.code.contains("0x1000") ||
                          analyzer.code.contains("as *const") ||
                          analyzer.code.contains("as *mut");
        assert!(
            has_addresses,
            "❌ You should work with memory addresses and pointer casting"
        );
    }

    #[test]
    fn test_converts_between_pointer_types() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_conversion = analyzer.code.contains("as_ptr") ||
                           analyzer.code.contains("as_mut_ptr") ||
                           analyzer.code.contains("&raw const") ||
                           analyzer.code.contains("&raw mut");
        assert!(
            has_conversion,
            "❌ You should convert between raw pointers and references safely"
        );
    }

    #[test]
    fn test_reads_memory_data() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_memory_read = analyzer.code.contains("0x42") ||
                            analyzer.code.contains("0x43") ||
                            analyzer.code.contains("Hell") ||
                            analyzer.code.contains("integer");
        assert!(
            has_memory_read,
            "❌ You should read the specific memory data from the buffer"
        );
    }

    #[test]
    fn test_handles_different_data_types() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_types = analyzer.code.contains("u8") &&
                       (analyzer.code.contains("i32") || analyzer.code.contains("u32")) &&
                       analyzer.code.contains("str");
        assert!(
            has_types,
            "❌ You should handle different data types (u8, i32, str) in memory"
        );
    }
}

// Reference implementation
fn main() {
    println!("Level 18 Task 1: Raw Pointer Operations");
    // Reference pattern for raw pointer manipulation
}

// Reference raw pointer pattern
// unsafe fn read_memory_buffer() {
//     // Create raw pointer to memory address
//     let ptr = 0x1000 as *const u8;
//
//     // Read integer from memory
//     let int_ptr = ptr as *const i32;
//     let integer_value = *int_ptr;  // 1145259586
//
//     // Read string data with pointer arithmetic
//     let str_ptr = ptr.add(4);
//     let string_data = std::slice::from_raw_parts(str_ptr, 6);
//
//     println!("Integer: {}", integer_value);
//     println!("String: {:?}", std::str::from_utf8(string_data));
// }