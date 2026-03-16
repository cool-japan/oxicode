//! Advanced file I/O encoding tests for OxiCode (set 8)

#![cfg(feature = "std")]
#![allow(
    clippy::approx_constant,
    clippy::useless_vec,
    clippy::len_zero,
    clippy::unnecessary_cast,
    clippy::redundant_closure,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_borrow,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms,
    clippy::inconsistent_digit_grouping,
    clippy::unit_cmp,
    clippy::assertions_on_constants,
    clippy::iter_on_single_items,
    clippy::expect_fun_call,
    clippy::redundant_pattern_matching,
    variant_size_differences,
    clippy::absurd_extreme_comparisons,
    clippy::nonminimal_bool,
    clippy::for_kv_map,
    clippy::needless_range_loop,
    clippy::single_match,
    clippy::collapsible_if,
    clippy::needless_return,
    clippy::redundant_clone,
    clippy::map_entry,
    clippy::match_single_binding,
    clippy::bool_comparison,
    clippy::derivable_impls,
    clippy::manual_range_contains,
    clippy::needless_borrows_for_generic_args,
    clippy::manual_map,
    clippy::vec_init_then_push,
    clippy::identity_op,
    clippy::manual_flatten,
    clippy::single_char_pattern,
    clippy::search_is_some,
    clippy::option_map_unit_fn,
    clippy::while_let_on_iterator,
    clippy::clone_on_copy,
    clippy::box_collection,
    clippy::redundant_field_names,
    clippy::ptr_arg,
    clippy::large_enum_variant,
    clippy::match_ref_pats,
    clippy::needless_pass_by_value,
    clippy::unused_unit,
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::manual_strip,
    clippy::match_like_matches_macro,
    clippy::from_over_into,
    clippy::wrong_self_convention,
    clippy::inherent_to_string,
    clippy::new_without_default,
    clippy::unnecessary_wraps,
    clippy::field_reassign_with_default,
    clippy::manual_find,
    clippy::unnecessary_lazy_evaluations,
    clippy::should_implement_trait,
    clippy::missing_safety_doc,
    clippy::unusual_byte_groupings,
    clippy::bool_assert_comparison,
    clippy::zero_prefixed_literal,
    clippy::await_holding_lock,
    clippy::manual_saturating_arithmetic,
    clippy::explicit_counter_loop,
    clippy::needless_lifetimes,
    clippy::single_component_path_imports,
    clippy::uninlined_format_args,
    clippy::iter_cloned_collect,
    clippy::manual_str_repeat,
    clippy::excessive_precision,
    clippy::precedence,
    clippy::unnecessary_literal_unwrap
)]
use oxicode::{Decode, Encode};
use std::env::temp_dir;

#[derive(Debug, PartialEq, Encode, Decode)]
struct Config {
    name: String,
    value: i64,
    enabled: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Category {
    Alpha,
    Beta(u32),
    Gamma { x: f64, y: f64 },
}

// Test 1: Config struct roundtrip via file
#[test]
fn test_adv8_01_config_struct_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_1.bin", std::process::id()));
    let cfg = Config {
        name: "my-config".to_string(),
        value: -42,
        enabled: true,
    };
    oxicode::encode_to_file(&cfg, &path).expect("encode Config to file");
    let decoded: Config = oxicode::decode_from_file(&path).expect("decode Config from file");
    assert_eq!(cfg, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 2: Category::Alpha roundtrip via file
#[test]
fn test_adv8_02_category_alpha_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_2.bin", std::process::id()));
    let val = Category::Alpha;
    oxicode::encode_to_file(&val, &path).expect("encode Category::Alpha to file");
    let decoded: Category =
        oxicode::decode_from_file(&path).expect("decode Category::Alpha from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 3: Category::Beta roundtrip via file
#[test]
fn test_adv8_03_category_beta_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_3.bin", std::process::id()));
    let val = Category::Beta(99_999);
    oxicode::encode_to_file(&val, &path).expect("encode Category::Beta to file");
    let decoded: Category =
        oxicode::decode_from_file(&path).expect("decode Category::Beta from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 4: Category::Gamma roundtrip via file
#[test]
fn test_adv8_04_category_gamma_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_4.bin", std::process::id()));
    let val = Category::Gamma {
        x: 1.23456789,
        y: -9.87654321,
    };
    oxicode::encode_to_file(&val, &path).expect("encode Category::Gamma to file");
    let decoded: Category =
        oxicode::decode_from_file(&path).expect("decode Category::Gamma from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 5: u32 to file roundtrip
#[test]
fn test_adv8_05_u32_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_5.bin", std::process::id()));
    let val: u32 = 3_141_592;
    oxicode::encode_to_file(&val, &path).expect("encode u32 to file");
    let decoded: u32 = oxicode::decode_from_file(&path).expect("decode u32 from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 6: String to file roundtrip
#[test]
fn test_adv8_06_string_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_6.bin", std::process::id()));
    let val = "OxiCode file I/O advanced test string".to_string();
    oxicode::encode_to_file(&val, &path).expect("encode String to file");
    let decoded: String = oxicode::decode_from_file(&path).expect("decode String from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 7: Vec<u8> to file roundtrip
#[test]
fn test_adv8_07_vec_u8_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_7.bin", std::process::id()));
    let val: Vec<u8> = (0u8..=255).collect();
    oxicode::encode_to_file(&val, &path).expect("encode Vec<u8> to file");
    let decoded: Vec<u8> = oxicode::decode_from_file(&path).expect("decode Vec<u8> from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 8: bool true/false to file
#[test]
fn test_adv8_08_bool_roundtrip() {
    let path_true = temp_dir().join(format!("oxicode_adv8_{}_8t.bin", std::process::id()));
    let path_false = temp_dir().join(format!("oxicode_adv8_{}_8f.bin", std::process::id()));

    oxicode::encode_to_file(&true, &path_true).expect("encode true to file");
    oxicode::encode_to_file(&false, &path_false).expect("encode false to file");

    let decoded_true: bool = oxicode::decode_from_file(&path_true).expect("decode true from file");
    let decoded_false: bool =
        oxicode::decode_from_file(&path_false).expect("decode false from file");

    assert!(decoded_true);
    assert!(!decoded_false);

    std::fs::remove_file(&path_true).ok();
    std::fs::remove_file(&path_false).ok();
}

// Test 9: f64 NaN to file (check bits)
#[test]
fn test_adv8_09_f64_nan_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_9.bin", std::process::id()));
    let val = f64::NAN;
    oxicode::encode_to_file(&val, &path).expect("encode f64 NaN to file");
    let decoded: f64 = oxicode::decode_from_file(&path).expect("decode f64 NaN from file");
    // NaN != NaN, so compare bit patterns
    assert_eq!(val.to_bits(), decoded.to_bits());
    std::fs::remove_file(&path).ok();
}

// Test 10: i64::MIN to file roundtrip
#[test]
fn test_adv8_10_i64_min_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_10.bin", std::process::id()));
    let val = i64::MIN;
    oxicode::encode_to_file(&val, &path).expect("encode i64::MIN to file");
    let decoded: i64 = oxicode::decode_from_file(&path).expect("decode i64::MIN from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 11: Vec<Config> (5 items) to file
#[test]
fn test_adv8_11_vec_config_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_11.bin", std::process::id()));
    let items: Vec<Config> = (0..5)
        .map(|i| Config {
            name: format!("config-{}", i),
            value: i as i64 * 100,
            enabled: i % 2 == 0,
        })
        .collect();
    oxicode::encode_to_file(&items, &path).expect("encode Vec<Config> to file");
    let decoded: Vec<Config> =
        oxicode::decode_from_file(&path).expect("decode Vec<Config> from file");
    assert_eq!(items, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 12: Option<String> Some to file
#[test]
fn test_adv8_12_option_string_some_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_12.bin", std::process::id()));
    let val: Option<String> = Some("present value".to_string());
    oxicode::encode_to_file(&val, &path).expect("encode Option::Some to file");
    let decoded: Option<String> =
        oxicode::decode_from_file(&path).expect("decode Option::Some from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 13: Option<String> None to file
#[test]
fn test_adv8_13_option_string_none_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_13.bin", std::process::id()));
    let val: Option<String> = None;
    oxicode::encode_to_file(&val, &path).expect("encode Option::None to file");
    let decoded: Option<String> =
        oxicode::decode_from_file(&path).expect("decode Option::None from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 14: File with fixed-int config
#[test]
fn test_adv8_14_file_with_fixed_int_config() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_14.bin", std::process::id()));
    let cfg_enc = oxicode::config::standard().with_fixed_int_encoding();
    let cfg_dec = oxicode::config::standard().with_fixed_int_encoding();
    let val: u32 = 12345;
    oxicode::encode_to_file_with_config(&val, &path, cfg_enc)
        .expect("encode u32 fixed-int to file");
    let decoded: u32 = oxicode::decode_from_file_with_config(&path, cfg_dec)
        .expect("decode u32 fixed-int from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 15: File with big-endian config (verify raw bytes)
#[test]
fn test_adv8_15_file_with_big_endian_config() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_15.bin", std::process::id()));
    let cfg_be = oxicode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let val: u32 = 0x01020304u32;
    oxicode::encode_to_file_with_config(&val, &path, cfg_be)
        .expect("encode u32 big-endian to file");

    // Read raw bytes and verify big-endian byte order
    let raw = std::fs::read(&path).expect("read raw bytes");
    // With fixed-int big-endian, u32 = 0x01020304 should be [0x01, 0x02, 0x03, 0x04]
    assert_eq!(raw, vec![0x01, 0x02, 0x03, 0x04]);

    let decoded: u32 = oxicode::decode_from_file_with_config(&path, cfg_be)
        .expect("decode u32 big-endian from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 16: Large Vec<u8> (5000 bytes) to file
#[test]
fn test_adv8_16_large_vec_u8_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_16.bin", std::process::id()));
    let val: Vec<u8> = (0u8..=255).cycle().take(5000).collect();
    oxicode::encode_to_file(&val, &path).expect("encode large Vec<u8> to file");
    let decoded: Vec<u8> =
        oxicode::decode_from_file(&path).expect("decode large Vec<u8> from file");
    assert_eq!(val.len(), decoded.len());
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 17: Sequential writes with encode_into_std_write
#[test]
fn test_adv8_17_sequential_encode_into_std_write() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_17.bin", std::process::id()));
    let mut file = std::fs::File::create(&path).expect("create file for sequential write");
    let cfg = oxicode::config::standard();

    let n1 = oxicode::encode_into_std_write(10u32, &mut file, cfg).expect("encode 10u32");
    let n2 = oxicode::encode_into_std_write(20u32, &mut file, cfg).expect("encode 20u32");
    let n3 = oxicode::encode_into_std_write(30u32, &mut file, cfg).expect("encode 30u32");

    assert!(n1 > 0);
    assert!(n2 > 0);
    assert!(n3 > 0);
    drop(file);

    // Read back all three values using a reader
    let raw = std::fs::read(&path).expect("read sequential file");
    let mut cursor = std::io::Cursor::new(raw);
    let v1: u32 = oxicode::decode_from_std_read(&mut cursor, cfg).expect("decode first u32");
    let v2: u32 = oxicode::decode_from_std_read(&mut cursor, cfg).expect("decode second u32");
    let v3: u32 = oxicode::decode_from_std_read(&mut cursor, cfg).expect("decode third u32");

    assert_eq!(v1, 10u32);
    assert_eq!(v2, 20u32);
    assert_eq!(v3, 30u32);

    std::fs::remove_file(&path).ok();
}

// Test 18: Overwrite file: second encode replaces first
#[test]
fn test_adv8_18_overwrite_file() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_18.bin", std::process::id()));
    let first = Config {
        name: "first".to_string(),
        value: 1,
        enabled: false,
    };
    let second = Config {
        name: "second".to_string(),
        value: 2,
        enabled: true,
    };

    oxicode::encode_to_file(&first, &path).expect("encode first Config to file");
    oxicode::encode_to_file(&second, &path).expect("encode second Config to file (overwrite)");

    let decoded: Config =
        oxicode::decode_from_file(&path).expect("decode overwritten Config from file");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 19: Non-existent file returns error
#[test]
fn test_adv8_19_nonexistent_file_returns_error() {
    let path = temp_dir().join(format!(
        "oxicode_adv8_{}_19_nonexistent_xyz.bin",
        std::process::id()
    ));
    // Ensure it doesn't exist
    std::fs::remove_file(&path).ok();
    let result = oxicode::decode_from_file::<Config>(&path);
    assert!(result.is_err(), "Expected error for non-existent file");
}

// Test 20: File size matches encode_to_vec length
#[test]
fn test_adv8_20_file_size_matches_encode_to_vec() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_20.bin", std::process::id()));
    let val = Config {
        name: "size-check".to_string(),
        value: 9999,
        enabled: false,
    };
    oxicode::encode_to_file(&val, &path).expect("encode Config to file for size check");

    let metadata = std::fs::metadata(&path).expect("get file metadata");
    let vec_bytes = oxicode::encode_to_vec(&val).expect("encode Config to vec for size check");

    assert_eq!(metadata.len() as usize, vec_bytes.len());
    std::fs::remove_file(&path).ok();
}

// Test 21: u128 to file roundtrip
#[test]
fn test_adv8_21_u128_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_21.bin", std::process::id()));
    let val: u128 = u128::MAX / 2 + 1;
    oxicode::encode_to_file(&val, &path).expect("encode u128 to file");
    let decoded: u128 = oxicode::decode_from_file(&path).expect("decode u128 from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 22: Empty string to file roundtrip
#[test]
fn test_adv8_22_empty_string_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv8_{}_22.bin", std::process::id()));
    let val = String::new();
    oxicode::encode_to_file(&val, &path).expect("encode empty String to file");
    let decoded: String = oxicode::decode_from_file(&path).expect("decode empty String from file");
    assert_eq!(val, decoded);
    assert!(decoded.is_empty());
    std::fs::remove_file(&path).ok();
}
