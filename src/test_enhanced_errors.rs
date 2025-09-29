// Test program for the enhanced error checking system
// This can be used to manually test error detection

use crate::rust_checker::RustChecker;
use crate::rust_checker::format_errors_for_display;

pub fn test_error_system() {
    println!("🧪 TESTING ENHANCED ERROR SYSTEM 🧪");
    println!("=======================================");

    let mut checker = match RustChecker::new() {
        Ok(checker) => checker,
        Err(e) => {
            println!("❌ Failed to create rust checker: {}", e);
            return;
        }
    };

    // Test case 1: User's problematic code
    let user_problematic_code = r#"fn m(direction: &str) { move(0); println!("{}"); 0); }

fn search() {
    for row_index in 0..9 {
        for _step_index in row in 0..8 {
            if row_index % 2 == 0 { m("left") } else { m("right") }
        }
        if row_index < 8 { m("down") }
    }
}

fn main() {
    println!("Robot starting...");
    s;
    // Your code here
}"#;

    println!("\n📝 Testing user's problematic code:");
    println!("-------------------------------------");

    match checker.check_syntax_enhanced(user_problematic_code) {
        Ok(errors) => {
            let formatted = format_errors_for_display(&errors);
            println!("{}", formatted);

            let error_count = errors.iter().filter(|e| e.severity == crate::rust_checker::ErrorSeverity::Error).count();
            if error_count > 0 {
                println!("\n✅ SUCCESS: Error system detected {} error(s)!", error_count);
            } else {
                println!("\n❌ PROBLEM: No errors detected in obviously broken code!");
            }
        },
        Err(e) => {
            println!("❌ Enhanced checking failed: {}", e);
        }
    }

    // Test case 2: Valid code
    let valid_code = r#"fn main() {
    println!("Hello, Robot!");
    move_bot("right");
    scan("forward");
}"#;

    println!("\n📝 Testing valid code:");
    println!("---------------------");

    match checker.check_syntax_enhanced(valid_code) {
        Ok(errors) => {
            let formatted = format_errors_for_display(&errors);
            println!("{}", formatted);

            if errors.is_empty() {
                println!("\n✅ SUCCESS: Valid code passed!");
            } else {
                println!("\n⚠️ Note: Valid code has warnings/issues");
            }
        },
        Err(e) => {
            println!("❌ Enhanced checking failed: {}", e);
        }
    }

    // Test case 3: Common syntax errors
    let syntax_error_code = r#"fn main() {
    let x = 5
    println!("Missing semicolon above");
    for i in 0..5 {
        println!("Number: {}", i)
    }
    let y = 10 +;
}"#;

    println!("\n📝 Testing common syntax errors:");
    println!("--------------------------------");

    match checker.check_syntax_enhanced(syntax_error_code) {
        Ok(errors) => {
            let formatted = format_errors_for_display(&errors);
            println!("{}", formatted);
        },
        Err(e) => {
            println!("❌ Enhanced checking failed: {}", e);
        }
    }

    println!("\n🎯 TEST COMPLETE!");
}

/// Function to manually check any code string
pub fn check_code_manually(code: &str) -> String {
    let mut checker = match RustChecker::new() {
        Ok(checker) => checker,
        Err(e) => return format!("❌ Failed to create rust checker: {}", e),
    };

    match checker.check_syntax_enhanced(code) {
        Ok(errors) => {
            format_errors_for_display(&errors)
        },
        Err(e) => {
            format!("❌ Enhanced syntax checking failed: {}", e)
        }
    }
}