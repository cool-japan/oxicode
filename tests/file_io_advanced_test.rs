//! Advanced integration tests for file I/O encoding in OxiCode.
//!
//! 22 top-level `#[test]` functions — no module wrapper, no unwrap().

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

fn tmp(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("{}_{}", name, std::process::id()))
}

// ── shared helper types ──────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SimpleStruct {
    id: u32,
    label: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NestedStruct {
    header: SimpleStruct,
    tags: Vec<String>,
    active: bool,
    score: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

// ── test 1: write u32 to file, read back ────────────────────────────────────

#[test]
fn test_adv_file_roundtrip_u32() {
    let path = tmp("oxicode_adv2_u32");
    let value: u32 = 0xDEAD_BEEF;

    oxicode::encode_to_file(&value, &path).expect("encode u32 to file");
    let decoded: u32 = oxicode::decode_from_file(&path).expect("decode u32 from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 2: write String to file, read back ─────────────────────────────────

#[test]
fn test_adv_file_roundtrip_string() {
    let path = tmp("oxicode_adv2_string");
    let value = String::from("Hello, OxiCode file I/O — advanced!");

    oxicode::encode_to_file(&value, &path).expect("encode String to file");
    let decoded: String = oxicode::decode_from_file(&path).expect("decode String from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 3: write Vec<u8> to file, read back ────────────────────────────────

#[test]
fn test_adv_file_roundtrip_vec_u8() {
    let path = tmp("oxicode_adv2_vec_u8");
    let value: Vec<u8> = (0u8..=127).collect();

    oxicode::encode_to_file(&value, &path).expect("encode Vec<u8> to file");
    let decoded: Vec<u8> = oxicode::decode_from_file(&path).expect("decode Vec<u8> from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 4: write struct to file, read back ─────────────────────────────────

#[test]
fn test_adv_file_roundtrip_struct() {
    let path = tmp("oxicode_adv2_struct");
    let value = SimpleStruct {
        id: 42,
        label: String::from("struct_test"),
    };

    oxicode::encode_to_file(&value, &path).expect("encode struct to file");
    let decoded: SimpleStruct = oxicode::decode_from_file(&path).expect("decode struct from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 5: write enum to file, read back ───────────────────────────────────

#[test]
fn test_adv_file_roundtrip_enum() {
    let path = tmp("oxicode_adv2_enum");
    let value = Color::Custom(200, 100, 50);

    oxicode::encode_to_file(&value, &path).expect("encode enum to file");
    let decoded: Color = oxicode::decode_from_file(&path).expect("decode enum from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 6: write then read with fixed int config ───────────────────────────

#[test]
fn test_adv_file_roundtrip_fixed_int_config() {
    use oxicode::config;

    let path = tmp("oxicode_adv2_fixedint");
    let cfg = config::standard().with_fixed_int_encoding();
    let value: u64 = 123_456_789_u64;

    oxicode::encode_to_file_with_config(&value, &path, cfg)
        .expect("encode u64 with fixed int config to file");
    let decoded: u64 = oxicode::decode_from_file_with_config(&path, cfg)
        .expect("decode u64 with fixed int config from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 7: write then read with big endian config ──────────────────────────

#[test]
fn test_adv_file_roundtrip_big_endian_config() {
    use oxicode::config;

    let path = tmp("oxicode_adv2_bigendian");
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value: u32 = 0x0102_0304;

    oxicode::encode_to_file_with_config(&value, &path, cfg)
        .expect("encode u32 with big endian config to file");
    let decoded: u32 = oxicode::decode_from_file_with_config(&path, cfg)
        .expect("decode u32 with big endian config from file");

    assert_eq!(value, decoded);

    // Verify raw bytes are big-endian order
    let raw = std::fs::read(&path).expect("read raw big endian file");
    assert_eq!(raw, [0x01, 0x02, 0x03, 0x04]);

    std::fs::remove_file(&path).ok();
}

// ── test 8: write large data (10000 bytes Vec<u8>) to file ──────────────────

#[test]
fn test_adv_file_roundtrip_large_vec() {
    let path = tmp("oxicode_adv2_large");
    let value: Vec<u8> = (0u8..=255).cycle().take(10_000).collect();

    oxicode::encode_to_file(&value, &path).expect("encode large Vec<u8> to file");
    let decoded: Vec<u8> =
        oxicode::decode_from_file(&path).expect("decode large Vec<u8> from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 9: write multiple values sequentially to same file (using Writer) ──

#[test]
fn test_adv_file_sequential_write_and_first_read() {
    use std::io::BufReader;

    let path = tmp("oxicode_adv2_seq");
    std::fs::remove_file(&path).ok();

    let cfg = oxicode::config::standard();

    // Write two u32 values sequentially using encode_into_std_write
    {
        let mut file = std::fs::File::create(&path).expect("create seq file");
        oxicode::encode_into_std_write(42u32, &mut file, cfg)
            .expect("write first sequential value");
        oxicode::encode_into_std_write(99u32, &mut file, cfg)
            .expect("write second sequential value");
    }

    // Read both values back sequentially
    let file = std::fs::File::open(&path).expect("open seq file");
    let mut reader = BufReader::new(file);

    let first: u32 =
        oxicode::decode_from_std_read(&mut reader, cfg).expect("read first sequential value");
    let second: u32 =
        oxicode::decode_from_std_read(&mut reader, cfg).expect("read second sequential value");

    assert_eq!(first, 42u32);
    assert_eq!(second, 99u32);

    std::fs::remove_file(&path).ok();
}

// ── test 10: write empty Vec<u8> to file, read back ─────────────────────────

#[test]
fn test_adv_file_roundtrip_empty_vec() {
    let path = tmp("oxicode_adv2_empty_vec");
    let value: Vec<u8> = Vec::new();

    oxicode::encode_to_file(&value, &path).expect("encode empty Vec<u8> to file");
    let decoded: Vec<u8> =
        oxicode::decode_from_file(&path).expect("decode empty Vec<u8> from file");

    assert_eq!(value, decoded);
    assert!(decoded.is_empty());
    std::fs::remove_file(&path).ok();
}

// ── test 11: write Option<String> Some to file, read back ───────────────────

#[test]
fn test_adv_file_roundtrip_option_some() {
    let path = tmp("oxicode_adv2_opt_some");
    let value: Option<String> = Some(String::from("optional_value"));

    oxicode::encode_to_file(&value, &path).expect("encode Option Some to file");
    let decoded: Option<String> =
        oxicode::decode_from_file(&path).expect("decode Option Some from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 12: write Option<String> None to file, read back ───────────────────

#[test]
fn test_adv_file_roundtrip_option_none() {
    let path = tmp("oxicode_adv2_opt_none");
    let value: Option<String> = None;

    oxicode::encode_to_file(&value, &path).expect("encode Option None to file");
    let decoded: Option<String> =
        oxicode::decode_from_file(&path).expect("decode Option None from file");

    assert_eq!(value, decoded);
    assert!(decoded.is_none());
    std::fs::remove_file(&path).ok();
}

// ── test 13: write bool values to file, read back ───────────────────────────

#[test]
fn test_adv_file_roundtrip_bool_true_and_false() {
    let path_true = tmp("oxicode_adv2_bool_true");
    let path_false = tmp("oxicode_adv2_bool_false");

    oxicode::encode_to_file(&true, &path_true).expect("encode bool true to file");
    oxicode::encode_to_file(&false, &path_false).expect("encode bool false to file");

    let decoded_true: bool =
        oxicode::decode_from_file(&path_true).expect("decode bool true from file");
    let decoded_false: bool =
        oxicode::decode_from_file(&path_false).expect("decode bool false from file");

    assert!(decoded_true);
    assert!(!decoded_false);

    std::fs::remove_file(&path_true).ok();
    std::fs::remove_file(&path_false).ok();
}

// ── test 14: write u64::MAX to file, read back ──────────────────────────────

#[test]
fn test_adv_file_roundtrip_u64_max() {
    let path = tmp("oxicode_adv2_u64max");
    let value: u64 = u64::MAX;

    oxicode::encode_to_file(&value, &path).expect("encode u64::MAX to file");
    let decoded: u64 = oxicode::decode_from_file(&path).expect("decode u64::MAX from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 15: write i64::MIN to file, read back ──────────────────────────────

#[test]
fn test_adv_file_roundtrip_i64_min() {
    let path = tmp("oxicode_adv2_i64min");
    let value: i64 = i64::MIN;

    oxicode::encode_to_file(&value, &path).expect("encode i64::MIN to file");
    let decoded: i64 = oxicode::decode_from_file(&path).expect("decode i64::MIN from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 16: write Vec<String> to file, read back ───────────────────────────

#[test]
fn test_adv_file_roundtrip_vec_string() {
    let path = tmp("oxicode_adv2_vec_string");
    let value: Vec<String> = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
        String::from("delta"),
    ];

    oxicode::encode_to_file(&value, &path).expect("encode Vec<String> to file");
    let decoded: Vec<String> =
        oxicode::decode_from_file(&path).expect("decode Vec<String> from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 17: write then overwrite file – second encode replaces first ────────

#[test]
fn test_adv_file_overwrite_replaces_content() {
    let path = tmp("oxicode_adv2_overwrite");

    let first: u32 = 111;
    let second: u32 = 999;

    oxicode::encode_to_file(&first, &path).expect("first encode to file");
    oxicode::encode_to_file(&second, &path).expect("second (overwrite) encode to file");

    let decoded: u32 = oxicode::decode_from_file(&path).expect("decode after overwrite");
    assert_eq!(second, decoded, "overwrite should produce second value");

    std::fs::remove_file(&path).ok();
}

// ── test 18: read from non-existent file returns error ───────────────────────

#[test]
fn test_adv_file_decode_nonexistent_returns_error() {
    let path = tmp("oxicode_adv2_nonexistent_xyzzy_99");
    // Ensure it really does not exist
    let _ = std::fs::remove_file(&path);

    let result = oxicode::decode_from_file::<u32>(&path);
    assert!(
        result.is_err(),
        "decoding non-existent file must return Err"
    );
}

// ── test 19: write f64 (std::f64::consts::PI) to file, read back ────────────

#[test]
fn test_adv_file_roundtrip_f64_pi() {
    let path = tmp("oxicode_adv2_f64_pi");
    let value: f64 = std::f64::consts::PI;

    oxicode::encode_to_file(&value, &path).expect("encode f64 PI to file");
    let decoded: f64 = oxicode::decode_from_file(&path).expect("decode f64 PI from file");

    // Bit-equality guarantees exact IEEE 754 round-trip
    assert_eq!(
        value.to_bits(),
        decoded.to_bits(),
        "f64 PI round-trip must be bit-identical"
    );
    std::fs::remove_file(&path).ok();
}

// ── test 20: write u128 to file, read back ──────────────────────────────────

#[test]
fn test_adv_file_roundtrip_u128() {
    let path = tmp("oxicode_adv2_u128");
    let value: u128 = u128::MAX / 3;

    oxicode::encode_to_file(&value, &path).expect("encode u128 to file");
    let decoded: u128 = oxicode::decode_from_file(&path).expect("decode u128 from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 21: write struct with multiple nested fields to file ────────────────

#[test]
fn test_adv_file_roundtrip_nested_struct() {
    let path = tmp("oxicode_adv2_nested");
    let value = NestedStruct {
        header: SimpleStruct {
            id: 7,
            label: String::from("nested_header"),
        },
        tags: vec![
            String::from("tag1"),
            String::from("tag2"),
            String::from("tag3"),
        ],
        active: true,
        score: std::f64::consts::E,
    };

    oxicode::encode_to_file(&value, &path).expect("encode NestedStruct to file");
    let decoded: NestedStruct =
        oxicode::decode_from_file(&path).expect("decode NestedStruct from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 22: file size matches encode_to_vec length ─────────────────────────

#[test]
fn test_adv_file_size_matches_encode_to_vec_length() {
    let path = tmp("oxicode_adv2_size_check");
    let value = NestedStruct {
        header: SimpleStruct {
            id: 100,
            label: String::from("size_check"),
        },
        tags: vec![String::from("a"), String::from("bb"), String::from("ccc")],
        active: false,
        score: std::f64::consts::PI,
    };

    oxicode::encode_to_file(&value, &path).expect("encode to file for size check");

    let file_bytes = std::fs::read(&path).expect("read file bytes for size check");
    let vec_bytes = oxicode::encode_to_vec(&value).expect("encode_to_vec for size check");

    assert_eq!(
        file_bytes.len(),
        vec_bytes.len(),
        "file byte length must equal encode_to_vec length"
    );
    assert_eq!(
        file_bytes, vec_bytes,
        "file bytes must equal encode_to_vec bytes"
    );

    std::fs::remove_file(&path).ok();
}
