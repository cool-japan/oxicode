//! Tests for `encode_to_vec_with_config` and `decode_from_slice_with_config` APIs.

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
use oxicode::{config, Decode, Encode};
use std::collections::HashMap;

// ── shared test types ────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SimpleStruct {
    id: u32,
    value: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NestedStruct {
    inner: SimpleStruct,
    tag: u64,
}

// ── Test 1: standard config roundtrip for u32 ────────────────────────────────

#[test]
fn test_standard_config_roundtrip_u32() {
    let cfg = config::standard();
    let original: u32 = 12345;
    let bytes = oxicode::encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, consumed): (u32, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ── Test 2: standard config roundtrip for String ─────────────────────────────

#[test]
fn test_standard_config_roundtrip_string() {
    let cfg = config::standard();
    let original = String::from("hello, oxicode!");
    let bytes = oxicode::encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, consumed): (String, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ── Test 3: legacy config roundtrip for u32 ──────────────────────────────────

#[test]
fn test_legacy_config_roundtrip_u32() {
    let cfg = config::legacy();
    let original: u32 = 99_999;
    let bytes = oxicode::encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, consumed): (u32, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ── Test 4: legacy config roundtrip for String ───────────────────────────────

#[test]
fn test_legacy_config_roundtrip_string() {
    let cfg = config::legacy();
    let original = String::from("legacy encoding test");
    let bytes = oxicode::encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, consumed): (String, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ── Test 5: with_fixed_int_encoding() u32 = 4 bytes ──────────────────────────

#[test]
fn test_fixed_int_encoding_u32_is_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = oxicode::encode_to_vec_with_config(&42u32, cfg).expect("encode failed");
    assert_eq!(
        bytes.len(),
        4,
        "u32 with fixed encoding must be exactly 4 bytes"
    );
    let (decoded, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode failed");
    assert_eq!(decoded, 42u32);
}

// ── Test 6: with_fixed_int_encoding() u64 = 8 bytes ──────────────────────────

#[test]
fn test_fixed_int_encoding_u64_is_8_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        oxicode::encode_to_vec_with_config(&0xDEAD_BEEF_CAFE_BABEu64, cfg).expect("encode failed");
    assert_eq!(
        bytes.len(),
        8,
        "u64 with fixed encoding must be exactly 8 bytes"
    );
    let (decoded, _): (u64, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode failed");
    assert_eq!(decoded, 0xDEAD_BEEF_CAFE_BABEu64);
}

// ── Test 7: with_big_endian() u32 verify byte order ──────────────────────────

#[test]
fn test_big_endian_u32_byte_order() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value: u32 = 0x01020304;
    let bytes = oxicode::encode_to_vec_with_config(&value, cfg).expect("encode failed");
    assert_eq!(bytes.len(), 4);
    // Big-endian: most significant byte first
    assert_eq!(bytes[0], 0x01);
    assert_eq!(bytes[1], 0x02);
    assert_eq!(bytes[2], 0x03);
    assert_eq!(bytes[3], 0x04);
    let (decoded, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode failed");
    assert_eq!(decoded, value);
}

// ── Test 8: with_big_endian() u16 verify byte order ──────────────────────────

#[test]
fn test_big_endian_u16_byte_order() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value: u16 = 0xABCD;
    let bytes = oxicode::encode_to_vec_with_config(&value, cfg).expect("encode failed");
    assert_eq!(bytes.len(), 2);
    assert_eq!(bytes[0], 0xAB, "high byte first in big endian");
    assert_eq!(bytes[1], 0xCD, "low byte second in big endian");
    let (decoded, _): (u16, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode failed");
    assert_eq!(decoded, value);
}

// ── Test 9: with_limit(100) success for small data ───────────────────────────

#[test]
fn test_with_limit_success_for_small_data() {
    let cfg = config::standard().with_limit::<100>();
    let value: u32 = 7;
    let bytes = oxicode::encode_to_vec_with_config(&value, cfg).expect("encode within limit");
    let (decoded, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode within limit");
    assert_eq!(decoded, value);
}

// ── Test 10: with_limit(4) failure for large data on decode ──────────────────

#[test]
fn test_with_limit_failure_for_large_data() {
    // Encode a Vec of many elements without limit first
    let large: Vec<u64> = (0u64..50).collect();
    let no_limit_cfg = config::standard().with_no_limit();
    let bytes = oxicode::encode_to_vec_with_config(&large, no_limit_cfg).expect("encode");

    // Now attempt to decode with a tiny limit — must fail
    let tight_cfg = config::standard().with_limit::<4>();
    let result: oxicode::Result<(Vec<u64>, _)> =
        oxicode::decode_from_slice_with_config(&bytes, tight_cfg);
    assert!(
        result.is_err(),
        "decode should fail when data exceeds limit"
    );
}

// ── Test 11: with_no_limit() works for large data ────────────────────────────

#[test]
fn test_with_no_limit_works_for_large_data() {
    let cfg = config::standard().with_no_limit();
    let large: Vec<u8> = (0u8..=255).cycle().take(4096).collect();
    let bytes = oxicode::encode_to_vec_with_config(&large, cfg).expect("encode large");
    let (decoded, _): (Vec<u8>, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode large");
    assert_eq!(decoded, large);
}

// ── Test 12: standard config is default (same as no config) ──────────────────

#[test]
fn test_standard_config_matches_default_encode() {
    let value: u32 = 42;
    let with_std = oxicode::encode_to_vec_with_config(&value, config::standard())
        .expect("encode with standard");
    let default_enc = oxicode::encode_to_vec(&value).expect("encode default");
    assert_eq!(
        with_std, default_enc,
        "standard config must produce identical bytes to default encode"
    );
}

// ── Test 13: encode with config A, decode with config A (matches) ─────────────

#[test]
fn test_encode_and_decode_same_config_consistency() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let original: u64 = 0x0102030405060708;
    let bytes = oxicode::encode_to_vec_with_config(&original, cfg).expect("encode");
    let (decoded, _): (u64, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(decoded, original);
}

// ── Test 14: struct roundtrip with standard config ────────────────────────────

#[test]
fn test_struct_roundtrip_standard_config() {
    let cfg = config::standard();
    let original = SimpleStruct {
        id: 7,
        value: std::f64::consts::PI,
    };
    let bytes = oxicode::encode_to_vec_with_config(&original, cfg).expect("encode");
    let (decoded, consumed): (SimpleStruct, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ── Test 15: Vec<u8> with all configs ─────────────────────────────────────────

#[test]
fn test_vec_u8_with_all_configs() {
    let data: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];

    // Encode with standard config and verify non-empty
    let bytes_std =
        oxicode::encode_to_vec_with_config(&data, config::standard()).expect("encode standard");
    assert!(
        !bytes_std.is_empty(),
        "standard config produced empty output"
    );

    // Encode with legacy config and verify non-empty
    let bytes_legacy =
        oxicode::encode_to_vec_with_config(&data, config::legacy()).expect("encode legacy");
    assert!(
        !bytes_legacy.is_empty(),
        "legacy config produced empty output"
    );

    // Encode with big-endian fixed config and verify non-empty
    let bytes_be_fixed = oxicode::encode_to_vec_with_config(
        &data,
        config::standard()
            .with_big_endian()
            .with_fixed_int_encoding(),
    )
    .expect("encode big_endian_fixed");
    assert!(
        !bytes_be_fixed.is_empty(),
        "big_endian_fixed config produced empty output"
    );

    // Verify round-trip for standard config
    let cfg_standard = config::standard();
    let (decoded_std, _): (Vec<u8>, _) =
        oxicode::decode_from_slice_with_config(&bytes_std, cfg_standard).expect("decode standard");
    assert_eq!(decoded_std, data);

    // Verify round-trip for legacy config
    let cfg_legacy = config::legacy();
    let (decoded_legacy, _): (Vec<u8>, _) =
        oxicode::decode_from_slice_with_config(&bytes_legacy, cfg_legacy).expect("decode legacy");
    assert_eq!(decoded_legacy, data);
}

// ── Test 16: nested struct with fixed int encoding ────────────────────────────

#[test]
fn test_nested_struct_with_fixed_int_encoding() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = NestedStruct {
        inner: SimpleStruct {
            id: 100,
            value: std::f64::consts::E,
        },
        tag: 0xFFFF_FFFF_FFFF_FFFFu64,
    };
    let bytes = oxicode::encode_to_vec_with_config(&original, cfg).expect("encode");
    let (decoded, consumed): (NestedStruct, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ── Test 17: HashMap with standard config ─────────────────────────────────────

#[test]
fn test_hashmap_with_standard_config() {
    let cfg = config::standard();
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("alpha".to_string(), 1);
    map.insert("beta".to_string(), 2);
    map.insert("gamma".to_string(), 3);

    let bytes = oxicode::encode_to_vec_with_config(&map, cfg).expect("encode hashmap");
    let (decoded, consumed): (HashMap<String, u32>, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode hashmap");
    assert_eq!(decoded, map);
    assert_eq!(consumed, bytes.len());
}

// ── Test 18: config is Copy – pass by value multiple times ────────────────────

#[test]
fn test_config_is_copy_pass_by_value_multiple_times() {
    let cfg = config::standard().with_fixed_int_encoding();

    let a: u32 = 111;
    let b: u32 = 222;
    let c: u32 = 333;

    // cfg is Copy, so we can use it multiple times without cloning
    let bytes_a = oxicode::encode_to_vec_with_config(&a, cfg).expect("encode a");
    let bytes_b = oxicode::encode_to_vec_with_config(&b, cfg).expect("encode b");
    let bytes_c = oxicode::encode_to_vec_with_config(&c, cfg).expect("encode c");

    // All three must be exactly 4 bytes (fixed u32)
    assert_eq!(bytes_a.len(), 4);
    assert_eq!(bytes_b.len(), 4);
    assert_eq!(bytes_c.len(), 4);

    let (da, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&bytes_a, cfg).expect("decode a");
    let (db, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&bytes_b, cfg).expect("decode b");
    let (dc, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&bytes_c, cfg).expect("decode c");

    assert_eq!(da, a);
    assert_eq!(db, b);
    assert_eq!(dc, c);
}

// ── Test 19: encode_to_vec_with_config consistency with encode_into_slice ─────

#[test]
fn test_encode_to_vec_with_config_consistent_with_encode_into_slice() {
    let cfg = config::standard();
    let value: u64 = 0x0807_0605_0403_0201;

    let vec_bytes = oxicode::encode_to_vec_with_config(&value, cfg).expect("vec encode");

    let mut buf = [0u8; 64];
    let written = oxicode::encode_into_slice(value, &mut buf, cfg).expect("slice encode");

    assert_eq!(
        vec_bytes.as_slice(),
        &buf[..written],
        "encode_to_vec_with_config and encode_into_slice must produce identical bytes"
    );
}

// ── Test 20: decode_from_slice_with_config returns consumed byte count ─────────

#[test]
fn test_decode_from_slice_with_config_returns_consumed_byte_count() {
    let cfg = config::standard().with_fixed_int_encoding();
    let value: u32 = 0xCAFEBABE;
    let bytes = oxicode::encode_to_vec_with_config(&value, cfg).expect("encode");

    // Pad with extra bytes to confirm only the correct number is reported consumed
    let mut padded = bytes.clone();
    padded.extend_from_slice(&[0xAA, 0xBB, 0xCC, 0xDD]);

    let (decoded, consumed): (u32, _) =
        oxicode::decode_from_slice_with_config(&padded, cfg).expect("decode");
    assert_eq!(decoded, value);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal the encoded size, not the padded size"
    );
}

// ── Test 21: encode_into_slice_with_config ─────────────────────────────────────

#[test]
fn test_encode_into_slice_with_config() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value: u32 = 0x11223344;

    let mut buf = [0u8; 32];
    let written = oxicode::encode_into_slice(value, &mut buf, cfg).expect("encode into slice");

    assert_eq!(written, 4, "fixed u32 must write 4 bytes");
    // Verify big-endian byte layout
    assert_eq!(&buf[..4], &[0x11, 0x22, 0x33, 0x44]);

    let (decoded, consumed): (u32, _) =
        oxicode::decode_from_slice_with_config(&buf[..written], cfg).expect("decode");
    assert_eq!(decoded, value);
    assert_eq!(consumed, written);
}

// ── Test 22: encoded_size_with_config ─────────────────────────────────────────

#[test]
fn test_encoded_size_with_config() {
    // Fixed encoding: u32 must always report 4 bytes
    let cfg_fixed = config::standard().with_fixed_int_encoding();
    let size_fixed = oxicode::encoded_size_with_config(&42u32, cfg_fixed).expect("size fixed");
    assert_eq!(size_fixed, 4, "fixed u32 size must be 4");

    // Fixed encoding: u64 must always report 8 bytes
    let size_u64 = oxicode::encoded_size_with_config(&42u64, cfg_fixed).expect("size u64");
    assert_eq!(size_u64, 8, "fixed u64 size must be 8");

    // Variable encoding: small integers should be compact
    let cfg_var = config::standard().with_variable_int_encoding();
    let size_small = oxicode::encoded_size_with_config(&1u64, cfg_var).expect("size small varint");
    assert!(
        size_small < 8,
        "varint-encoded 1u64 must be smaller than 8 bytes"
    );

    // encoded_size_with_config must match actual encoded byte length
    let actual_bytes = oxicode::encode_to_vec_with_config(&42u32, cfg_fixed).expect("encode");
    assert_eq!(
        size_fixed,
        actual_bytes.len(),
        "encoded_size_with_config must match actual encoded length"
    );
}
