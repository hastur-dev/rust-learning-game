use crate::level::{YamlLevelConfig, LevelSpec, ItemConfig};
use rand::{rngs::StdRng, SeedableRng};
use serde_yaml;

// Embedded learning levels - these are core levels included in the executable
pub fn get_embedded_learning_levels() -> Vec<YamlLevelConfig> {
    vec![
        // Level 1: Hello Rust!
        YamlLevelConfig {
            name: "Level 1 - Hello Rust!".to_string(),
            grid_size: "12x8".to_string(),
            obstacles: Some(3),
            doors: None,
            enemies: None,
            items: Some(vec![
                ItemConfig {
                    name: "hello_world_tip".to_string(),
                    item_file: "items/hello_world.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((10, 6)),
                },
                ItemConfig {
                    name: "goal_item".to_string(),
                    item_file: "items/level_complete.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((8, 2)),
                }
            ]),
            income_per_square: Some(1),
            start_position: Some((1, 1)),
            max_turns: Some(0),
            fog_of_war: Some(true),
            message: Some("Welcome to Rust Robot Programming! 🦀 Your goal: Navigate to collect all items and reach the goal. Use basic movement commands (move, grab, scan) to explore. This level introduces Rust basics and the println! macro for output. TO WIN: Collect the 'hello_world_tip' item first, then grab the 'goal_item' to complete the level.".to_string()),
            hint_message: Some("HOW TO PLAY: Type commands like 'move_bot(\"right\");' then press ENTER to execute. Use 'grab();' to collect items. The exclamation mark in println! means it's a macro, not a function - this is Rust's way of code generation at compile time.".to_string()),
            rust_docs_url: Some("https://doc.rust-lang.org/rust-by-example/hello.html".to_string()),
            starting_code: Some(r#"// Welcome to Rust! Let's start with the classic Hello World program.
// Your task: Make this program print "rust robo wars" to complete the level.

fn main() {
    // TODO: Add your println! statement here
    println!("Hello, World!");
}
"#.to_string()),
            next_level_hint: Some("Next: Learn about functions, loops, and organizing code!".to_string()),
            achievement_message: Some("🎉 Congratulations! You've completed your first Rust program!".to_string()),
        },
        
        // Level 2: Functions and Loops
        YamlLevelConfig {
            name: "Level 2: Functions and Loops".to_string(),
            grid_size: "6x6".to_string(),
            obstacles: Some(0),
            doors: None, 
            enemies: None,
            items: Some(vec![
                ItemConfig {
                    name: "key".to_string(),
                    item_file: "items/key.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((3, 0)),
                },
                ItemConfig {
                    name: "goal_item".to_string(),
                    item_file: "items/level_complete.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((5, 5)),
                }
            ]),
            income_per_square: Some(1),
            start_position: Some((0, 0)),
            max_turns: Some(150),
            fog_of_war: Some(false),
            message: Some("🎯 **LEVEL 2: Functions, Loops, and Structs** - Learn to organize your code effectively and process data systematically!".to_string()),
            hint_message: Some("Create functions to organize your code, use loops to repeat actions, and structs to organize data. All code must be in functions!".to_string()),
            rust_docs_url: Some("https://doc.rust-lang.org/book/ch03-03-how-functions-work.html".to_string()),
            starting_code: Some(r#"// Level 2: Functions, Loops, and Structs
// All code must be organized into functions!

// TODO: Task 3 - Define your GridInfo struct here
// struct GridInfo {
//     x: i32,
//     y: i32, 
//     content: String,
// }

// TODO: Task 1 - Create scan_level function with print statement
// fn scan_level() {
//     println!("Beginning level scan...");
//     
//     // TODO: Task 2 - Add nested loops here
//     // for y in 0..6 {
//     //     for x in 0..6 {
//     //         // Movement and scanning code
//     //     }
//     // }
// }

// TODO: Task 4 - Create grab_if_item function
// fn grab_if_item(scan_result: &str) {
//     // Add if statement to check for items and grab them
// }

fn main() {
    println!("Level 2: Functions, Loops, and Structs");
    println!("Remember: All code must be in functions!");
    
    // TODO: Call your scan_level() function here
    // scan_level();
    
    // Navigate to goal when done
    println!("All tasks complete! Moving to goal...");
    // You'll need movement code to reach (5,5)
}
"#.to_string()),
            next_level_hint: Some("Next level: Error handling and advanced movement patterns".to_string()),
            achievement_message: Some("Perfect! You've mastered function organization, loops, structs, and conditional logic!".to_string()),
        },
    ]
}

// Additional embedded level functions (kept for backward compatibility)
pub fn get_embedded_educational_levels() -> Vec<YamlLevelConfig> {
        YamlLevelConfig {
            name: "Variables and Movement".to_string(),
            grid_size: "12x10".to_string(),
            obstacles: Some(3),
            doors: None,
            enemies: None,
            items: Some(vec![
                ItemConfig {
                    name: "goal_item".to_string(),
                    item_file: "items/goal.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((10, 8)),
                }
            ]),
            income_per_square: Some(1),
            start_position: Some((1, 1)),
            max_turns: Some(0),
            fog_of_war: Some(true),
            message: Some("📚 This level explores Rust variables. Remember: Ctrl+Shift+C shows completion instructions, and Ctrl+Shift+B opens documentation. Variables are fundamental to all programming!".to_string()),
            hint_message: Some("Declare variables with 'let' keyword. Use them to store direction strings like \"right\" or \"down\".".to_string()),
            rust_docs_url: Some("https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html".to_string()),
            starting_code: Some(r#"// Learn about variables in Rust!
// Variables let you store values and reuse them.

// TODO: Complete the variable declarations below
// let direction1 = \"right\";
// let direction2 = \"down\";

println!(\"Starting robot movement with variables!\");

// TODO: Use the variables in move commands
// move(direction1);
// move(direction1);
// move(direction2);
// move(direction2);

// Always end by grabbing the goal
grab();
"#.to_string()),
            completion_condition: None,
            completion_flag: Some("items_collected:1".to_string()),
            achievement_message: Some("💫 Great work! You've learned about variables - the building blocks of data storage in Rust. Variables let you reuse values throughout your code!".to_string()),
            next_level_hint: Some("Next, you'll learn about error handling - how to deal with problems in your code gracefully.".to_string()),
            completion_message: Some("🔍 TO COMPLETE THIS LEVEL:\n\n1. Uncomment the variable declarations:\n   let direction1 = \"right\";\n   let direction2 = \"down\";\n\n2. Uncomment the movement commands that use variables:\n   move(direction1); // Use the variable, not \"right\"\n   move(direction1);\n   move(direction2);\n   move(direction2);\n\n3. Navigate to collect the goal item\n\n🔑 Key concept: Variables store values with 'let' and can be reused multiple times.".to_string()),
        },
        
        // Level 3: Error Handling - Learn about errors
        YamlLevelConfig {
            name: "Understanding Errors".to_string(),
            grid_size: "8x6".to_string(),
            obstacles: Some(1),
            doors: None,
            enemies: None,
            items: None,
            income_per_square: Some(1),
            start_position: Some((1, 1)),
            max_turns: Some(0),
            fog_of_war: Some(true),
            message: Some("🚨 Error handling is a core Rust concept! As always, use Ctrl+Shift+C for instructions and Ctrl+Shift+B for docs. Learning to handle errors makes your code robust and reliable.".to_string()),
            hint_message: Some("Use eprintln!(\"This is an error message\"); to print to standard error. This completes the level!".to_string()),
            rust_docs_url: Some("https://doc.rust-lang.org/book/ch09-00-error-handling.html".to_string()),
            starting_code: Some(r#"// Learning about error messages in Rust
// Sometimes programs need to report errors or warnings
// eprintln! prints to \"standard error\" instead of standard output

println!(\"This is a normal message\");

// TODO: Uncomment the line below to create an error message
// eprintln!(\"This is an error message!\");

// The level completes when you generate an error message!
"#.to_string()),
            completion_condition: Some("error".to_string()),
            completion_flag: Some("eprintln:This is an error message!".to_string()),
            achievement_message: Some("🚫 Perfect! You've learned about eprintln! - Rust's way of printing error messages. This is crucial for debugging and user feedback!".to_string()),
            next_level_hint: Some("Next, you'll learn about functions - reusable blocks of code that make programming more organized.".to_string()),
            completion_message: Some("🔍 TO COMPLETE THIS LEVEL:\n\n1. Find the commented line: // eprintln!(\"This is an error message!\");\n2. Uncomment it by removing the //\n3. Press ENTER to run your code\n4. When your code outputs the error message exactly, the level will complete!\n\n📝 The final line should be: eprintln!(\"This is an error message!\");\n\n🔑 Key concept: eprintln! prints to 'standard error' - different from normal output, used for errors and warnings.".to_string()),
        },
        
        // Level 4: Functions and Movement
        YamlLevelConfig {
            name: "Functions and Robot Movement".to_string(),
            grid_size: "12x10".to_string(),
            obstacles: Some(5),
            doors: None,
            enemies: None,
            items: Some(vec![
                ItemConfig {
                    name: "scanner".to_string(),
                    item_file: "items/scanner.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((6, 5)),
                },
                ItemConfig {
                    name: "goal_item".to_string(),
                    item_file: "items/goal.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((10, 8)),
                }
            ]),
            income_per_square: Some(1),
            start_position: Some((1, 1)),
            max_turns: Some(0),
            fog_of_war: Some(true),
            message: Some("⚡ Functions make code modular and reusable! Remember your helpful shortcuts: Ctrl+Shift+C for instructions, Ctrl+Shift+B for documentation. Functions are the building blocks of larger programs.".to_string()),
            hint_message: Some("Define a function with 'fn function_name() { }'. Call it by writing the function name followed by parentheses.".to_string()),
            rust_docs_url: Some("https://doc.rust-lang.org/book/ch03-03-how-functions-work.html".to_string()),
            starting_code: Some(r#"// Learning about functions in Rust!
// Functions help organize and reuse code

// TODO: Complete this function to move right 3 times
fn move_right_three_times() {
    // Fill in the function body here
    // move(\"right\");
    // move(\"right\");
    // move(\"right\");
}

// TODO: Complete this function to move down and grab
fn move_down_and_grab() {
    // Fill in the function body here
    // move(\"down\");
    // grab();
}

println!(\"Starting function-based movement!\");

// TODO: Call your functions here to complete the level
// move_right_three_times();
// move_down_and_grab();
// move_down_and_grab();
"#.to_string()),
            completion_condition: None,
            completion_flag: Some("items_collected:2".to_string()),
            achievement_message: Some("🔧 Outstanding! You've mastered functions - the foundation of organized programming. Functions help you break complex tasks into manageable pieces!".to_string()),
            next_level_hint: Some("Next, you'll learn about control flow - making decisions in your code with if statements.".to_string()),
            completion_message: Some("🔍 TO COMPLETE THIS LEVEL:\n\n1. Complete the move_right_three_times() function:\n   - Add: move(\"right\"); three times\n\n2. Complete the move_down_and_grab() function:\n   - Add: move(\"down\");\n   - Add: grab();\n\n3. Call your functions in the correct order:\n   - move_right_three_times();\n   - move_down_and_grab();\n   - move_down_and_grab();\n\n4. Collect both items (scanner and goal) to complete!\n\n🔑 Key concept: Functions organize code into reusable blocks with fn name() { }.".to_string()),
        },
        
        // Level 5: Control Flow with if statements
        YamlLevelConfig {
            name: "Control Flow and Decision Making".to_string(),
            grid_size: "10x8".to_string(),
            obstacles: Some(3),
            doors: None,
            enemies: None,
            items: Some(vec![
                ItemConfig {
                    name: "goal_item".to_string(),
                    item_file: "items/goal.rs".to_string(),
                    spawn_randomly: Some(false),
                    location: Some((8, 6)),
                }
            ]),
            income_per_square: Some(1),
            start_position: Some((1, 1)),
            max_turns: Some(0),
            fog_of_war: Some(true),
            message: Some("Learn about conditional logic! Use if statements to make decisions in your code. Complete the conditional movement logic.".to_string()),
            hint_message: Some("Use 'if condition { }' for conditional execution. Boolean variables can be true or false.".to_string()),
            rust_docs_url: Some("https://doc.rust-lang.org/book/ch03-05-control-flow.html".to_string()),
            starting_code: Some(r#"// Learning about control flow with if statements
// Conditional logic lets your program make decisions

let should_move_right = true;
let should_move_down = true;
let should_grab = true;

println!(\"Making decisions with if statements!\");

// TODO: Complete the if statements below
if should_move_right {
    // Move right 3 times
    // move(\"right\");
    // move(\"right\");
    // move(\"right\");
}

if should_move_down {
    // Move down 2 times
    // move(\"down\");
    // move(\"down\");
}

if should_grab {
    // Grab items
    // grab();
}

println!(\"Conditional movement complete!\");
"#.to_string()),
            completion_condition: None,
            completion_flag: Some("println:Conditional movement complete!".to_string()),
            achievement_message: Some("🤔 Brilliant! You've learned conditional logic - the power to make your programs smart and responsive. If statements are the key to decision-making in code!".to_string()),
            next_level_hint: Some("You've mastered the fundamentals! Next levels will introduce more advanced Rust concepts like ownership and borrowing.".to_string()),
            completion_message: Some("🔍 TO COMPLETE THIS LEVEL:\n\n1. Fill in the if statements by uncommenting the move commands:\n\nif should_move_right {\n    move(\"right\");\n    move(\"right\");\n    move(\"right\");\n}\n\nif should_move_down {\n    move(\"down\");\n    move(\"down\");\n}\n\nif should_grab {\n    grab();\n}\n\n2. Make sure the println!(\"Conditional movement complete!\"); line runs\n\n3. When it prints exactly \"Conditional movement complete!\", you win!\n\n🔑 Key concept: if statements execute code only when conditions are true.".to_string()),
        },
    ]
}

// Embedded YAML level data - this ensures levels are packaged in both desktop and WASM builds
pub fn get_embedded_yaml_levels() -> Vec<YamlLevelConfig> {
    let level_data = vec![
        ("01_hello_rust", include_str!("../levels/01_hello_rust.yaml")),
        ("02_variables_mutability", include_str!("../levels/02_variables_mutability.yaml")),
        ("03_data_types", include_str!("../levels/03_data_types.yaml")),
        ("04_functions", include_str!("../levels/04_functions.yaml")),
        ("05_control_flow", include_str!("../levels/05_control_flow.yaml")),
        ("06_ownership_basics", include_str!("../levels/06_ownership_basics.yaml")),
        ("07_boolean_doors", include_str!("../levels/07_boolean_doors.yaml")),
        ("07_references_borrowing", include_str!("../levels/07_references_borrowing.yaml")),
        ("08_strings", include_str!("../levels/08_strings.yaml")),
        ("09_arrays_tuples", include_str!("../levels/09_arrays_tuples.yaml")),
        ("10_structs", include_str!("../levels/10_structs.yaml")),
        ("11_vectors", include_str!("../levels/11_vectors.yaml")),
        ("12_hashmaps", include_str!("../levels/12_hashmaps.yaml")),
        ("13_iterators", include_str!("../levels/13_iterators.yaml")),
        ("14_closures", include_str!("../levels/14_closures.yaml")),
        ("15_enums", include_str!("../levels/15_enums.yaml")),
        ("16_pattern_matching_advanced", include_str!("../levels/16_pattern_matching_advanced.yaml")),
        ("17_method_syntax", include_str!("../levels/17_method_syntax.yaml")),
        ("18_associated_functions", include_str!("../levels/18_associated_functions.yaml")),
        ("19_modules_crates", include_str!("../levels/19_modules_crates.yaml")),
        ("20_error_handling_intro", include_str!("../levels/20_error_handling_intro.yaml")),
        ("21_lifetimes", include_str!("../levels/21_lifetimes.yaml")),
        ("22_lifetime_annotations", include_str!("../levels/22_lifetime_annotations.yaml")),
        ("23_references_in_structs", include_str!("../levels/23_references_in_structs.yaml")),
        ("24_smart_pointers_box", include_str!("../levels/24_smart_pointers_box.yaml")),
        ("25_smart_pointers_rc", include_str!("../levels/25_smart_pointers_rc.yaml")),
        ("26_smart_pointers_refcell", include_str!("../levels/26_smart_pointers_refcell.yaml")),
        ("27_memory_leaks_cycles", include_str!("../levels/27_memory_leaks_cycles.yaml")),
        ("28_weak_references", include_str!("../levels/28_weak_references.yaml")),
        ("29_drop_trait", include_str!("../levels/29_drop_trait.yaml")),
        ("30_raii_pattern", include_str!("../levels/30_raii_pattern.yaml")),
        ("31_result_deep_dive", include_str!("../levels/31_result_deep_dive.yaml")),
        ("32_option_advanced", include_str!("../levels/32_option_advanced.yaml")),
        ("33_panic_unwinding", include_str!("../levels/33_panic_unwinding.yaml")),
        ("34_custom_error_types", include_str!("../levels/34_custom_error_types.yaml")),
        ("35_error_propagation", include_str!("../levels/35_error_propagation.yaml")),
        ("36_file_io", include_str!("../levels/36_file_io.yaml")),
        ("37_command_line_args", include_str!("../levels/37_command_line_args.yaml")),
        ("38_environment_variables", include_str!("../levels/38_environment_variables.yaml")),
        ("39_standard_input_output", include_str!("../levels/39_standard_input_output.yaml")),
        ("40_serialization", include_str!("../levels/40_serialization.yaml")),
        ("41_traits_basics", include_str!("../levels/41_traits_basics.yaml")),
        ("42_trait_objects", include_str!("../levels/42_trait_objects.yaml")),
        ("43_generics", include_str!("../levels/43_generics.yaml")),
        ("44_associated_types", include_str!("../levels/44_associated_types.yaml")),
        ("45_operator_overloading", include_str!("../levels/45_operator_overloading.yaml")),
        ("46_concurrency_threads", include_str!("../levels/46_concurrency_threads.yaml")),
        ("47_concurrency_channels", include_str!("../levels/47_concurrency_channels.yaml")),
        ("48_async_await_basics", include_str!("../levels/48_async_await_basics.yaml")),
        ("49_testing", include_str!("../levels/49_testing.yaml")),
        ("50_final_project", include_str!("../levels/50_final_project.yaml")),
        ("custom_movement_demo", include_str!("../levels/custom_movement_demo.yaml")),
    ];

    let mut levels = Vec::new();
    
    for (_name, yaml_content) in level_data {
        match serde_yaml::from_str::<YamlLevelConfig>(yaml_content) {
            Ok(config) => levels.push(config),
            Err(e) => eprintln!("Failed to parse embedded level: {}", e),
        }
    }
    
    levels
}

pub fn get_embedded_level_specs() -> Vec<LevelSpec> {
    let mut levels = Vec::new();
    let mut rng = StdRng::seed_from_u64(0xC0FFEE);
    
    // Use the new embedded learning levels
    let learning_configs = get_embedded_learning_levels();
    for config in learning_configs {
        if let Ok(level_spec) = config.to_level_spec(&mut rng) {
            levels.push(level_spec);
        }
    }
    
    // Then add the YAML levels from files
    let yaml_configs = get_embedded_yaml_levels();
    for config in yaml_configs {
        if let Ok(level_spec) = config.to_level_spec(&mut rng) {
            levels.push(level_spec);
        }
    }
    
    levels
}