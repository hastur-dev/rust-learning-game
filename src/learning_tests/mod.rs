// Learning tests module for validating user code against task requirements

pub mod test_utils;

// Level 1 tests
pub mod level1_task1_hello;

// Level 2 tests
pub mod level2_task1_function;
pub mod level2_task2_loops;
pub mod level2_task3_struct;
pub mod level2_task4_conditional;

// Level 3 tests
pub mod level3_task1_integers;
pub mod level3_task2_floats;
pub mod level3_task3_booleans;
pub mod level3_task4_characters;
pub mod level3_task5_type_inference;

// Level 4 tests
pub mod level4_task1_immutable;
pub mod level4_task2_mutable;
pub mod level4_task3_shadowing;
pub mod level4_task4_scope;
pub mod level4_task5_constants;

// Level 5 tests
pub mod level5_task1_casting;
pub mod level5_task2_from_into;
pub mod level5_task3_parsing;
pub mod level5_task4_custom_conversion;
pub mod level5_task5_inference;

// Level 11 tests (Smol Async Basics)
pub mod level11_task1_async_function;
pub mod level11_task2_block_on;
pub mod level11_task3_spawn_tasks;

// Level 14 tests (Serde JSON Basics)
pub mod level14_task1_serializable_struct;
pub mod level14_task2_deserialize_json;
pub mod level14_task3_serialize_state;

// Level 15 tests (Serde YAML Advanced)
pub mod level15_task1_yaml_config;

// Level 16 tests (Serde Custom Serialization)
pub mod level16_task1_custom_serialize;

// Level 17 tests (Log Library Basics)
pub mod level17_task1_basic_logging;

// Level 18 tests (Direct Memory Management)
pub mod level18_task1_raw_pointers;

// Level 19 tests (Bitwise Operations)
pub mod level19_task1_basic_bitwise;
pub mod level19_task2_bit_flags;
pub mod level19_task3_bit_manipulation;
pub mod level19_task4_binary_protocols;
pub mod level19_task5_compression;

// Level 20 tests (Callable Traits)
pub mod level20_task1_fn_traits;
pub mod level20_task2_closures;
pub mod level20_task3_function_composition;
pub mod level20_task4_callbacks;
pub mod level20_task5_higher_order;

// Level 21 tests (Operator Traits)
pub mod level21_task1_arithmetic_traits;

// Level 22 tests (Anyhow Error Recovery)
pub mod level22_task1_unified_errors;