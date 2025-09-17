// Level 19 Task 1 Test: Basic Bitwise Operations
// Tests that user implements fundamental bitwise operators for flag manipulation

#[cfg(test)]
mod level19_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_uses_and_operator() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(" & "),
            "❌ You need to use AND (&) operator for bit testing"
        );
    }

    #[test]
    fn test_uses_or_operator() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(" | "),
            "❌ You need to use OR (|) operator for bit setting"
        );
    }

    #[test]
    fn test_uses_xor_operator() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains(" ^ "),
            "❌ You need to use XOR (^) operator for bit toggling"
        );
    }

    #[test]
    fn test_uses_not_operator() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("~") || analyzer.code.contains("!"),
            "❌ You need to use NOT (~) operator for bit inversion"
        );
    }

    #[test]
    fn test_has_sensor_flags() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_flags = analyzer.code.contains("0b10110100") ||
                       analyzer.code.contains("sensor") ||
                       analyzer.code.contains("flags");
        assert!(
            has_flags,
            "❌ You should work with the sensor status flags data"
        );
    }

    #[test]
    fn test_checks_battery_flag() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_battery_check = analyzer.code.contains("battery") &&
                              (analyzer.code.contains("bit 7") ||
                               analyzer.code.contains("0x80") ||
                               analyzer.code.contains("0b10000000"));
        assert!(
            has_battery_check,
            "❌ You should check the battery flag (bit 7) using AND operator"
        );
    }

    #[test]
    fn test_enables_gps() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_gps = analyzer.code.contains("GPS") &&
                     (analyzer.code.contains("bit 5") ||
                      analyzer.code.contains("0x20") ||
                      analyzer.code.contains("0b00100000"));
        assert!(
            has_gps,
            "❌ You should enable GPS (bit 5) using OR operator"
        );
    }

    #[test]
    fn test_toggles_camera() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_camera = analyzer.code.contains("camera") &&
                        (analyzer.code.contains("bit 2") ||
                         analyzer.code.contains("0x04") ||
                         analyzer.code.contains("0b00000100"));
        assert!(
            has_camera,
            "❌ You should toggle camera (bit 2) using XOR operator"
        );
    }

    #[test]
    fn test_implements_flag_functions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_functions = analyzer.code.contains("fn ") &&
                          (analyzer.code.contains("check") ||
                           analyzer.code.contains("set") ||
                           analyzer.code.contains("toggle"));
        assert!(
            has_functions,
            "❌ You should implement functions for flag manipulation"
        );
    }

    #[test]
    fn test_demonstrates_all_operations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ You should demonstrate all bitwise operations with output"
        );
    }
}

// Reference implementation
fn main() {
    println!("Level 19 Task 1: Basic Bitwise Operations");
    // Reference pattern for bitwise flag manipulation
}

// Reference bitwise operations pattern
// fn manipulate_sensor_flags() {
//     let mut flags = 0b10110100u8;  // Initial sensor status
//
//     // Check battery flag (bit 7) using AND
//     let battery_ok = (flags & 0x80) != 0;
//     println!("Battery OK: {}", battery_ok);
//
//     // Enable GPS (bit 5) using OR
//     flags = flags | 0x20;
//     println!("GPS enabled: {:08b}", flags);
//
//     // Toggle camera (bit 2) using XOR
//     flags = flags ^ 0x04;
//     println!("Camera toggled: {:08b}", flags);
//
//     // Invert all flags using NOT
//     let inverted = !flags;
//     println!("Inverted flags: {:08b}", inverted);
// }