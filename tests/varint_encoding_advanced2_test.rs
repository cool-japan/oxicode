//! Advanced varint encoding behavior tests for OxiCode (22 tests).
//!
//! Tests cover size guarantees, fixed-int vs varint comparison,
//! signed integer zigzag behavior, and roundtrip correctness across
//! a wide range of values and types.

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config,
};

// ---------------------------------------------------------------------------
// Test 1: u8=0 encodes to exactly 1 byte in standard config
// ---------------------------------------------------------------------------
#[test]
fn test_u8_zero_standard_size() {
    let val: u8 = 0;
    let enc = encode_to_vec(&val).expect("encode u8(0)");
    assert_eq!(
        enc.len(),
        1,
        "u8(0) must encode to exactly 1 byte in standard config"
    );
    let (decoded, _): (u8, usize) = decode_from_slice(&enc).expect("decode u8(0)");
    assert_eq!(decoded, val, "u8(0) roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 2: u8=255 encodes to at least 1 byte (u8 always uses 1 byte)
// ---------------------------------------------------------------------------
#[test]
fn test_u8_max_standard_size() {
    let val: u8 = u8::MAX;
    let enc = encode_to_vec(&val).expect("encode u8::MAX");
    assert!(enc.len() >= 1, "u8::MAX must encode to at least 1 byte");
    let (decoded, _): (u8, usize) = decode_from_slice(&enc).expect("decode u8::MAX");
    assert_eq!(decoded, val, "u8::MAX roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 3: u16=0 encodes to exactly 1 byte in standard config (varint: 0 ≤ 250)
// ---------------------------------------------------------------------------
#[test]
fn test_u16_zero_standard_size() {
    let val: u16 = 0;
    let enc = encode_to_vec(&val).expect("encode u16(0)");
    assert_eq!(
        enc.len(),
        1,
        "u16(0) must encode to exactly 1 byte — 0 is in the 1-byte varint range"
    );
    let (decoded, _): (u16, usize) = decode_from_slice(&enc).expect("decode u16(0)");
    assert_eq!(decoded, val, "u16(0) roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 4: u16=65535 encodes to at least 2 bytes in standard config
// ---------------------------------------------------------------------------
#[test]
fn test_u16_max_standard_size() {
    let val: u16 = u16::MAX;
    let enc = encode_to_vec(&val).expect("encode u16::MAX");
    assert!(enc.len() >= 2, "u16::MAX must encode to at least 2 bytes");
    let (decoded, _): (u16, usize) = decode_from_slice(&enc).expect("decode u16::MAX");
    assert_eq!(decoded, val, "u16::MAX roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 5: u32=0 encodes to exactly 1 byte in standard config
// ---------------------------------------------------------------------------
#[test]
fn test_u32_zero_standard_size() {
    let val: u32 = 0;
    let enc = encode_to_vec(&val).expect("encode u32(0)");
    assert_eq!(
        enc.len(),
        1,
        "u32(0) must encode to exactly 1 byte in standard config"
    );
    let (decoded, _): (u32, usize) = decode_from_slice(&enc).expect("decode u32(0)");
    assert_eq!(decoded, val, "u32(0) roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 6: u32::MAX encodes to at least 4 bytes in standard config
// ---------------------------------------------------------------------------
#[test]
fn test_u32_max_standard_size() {
    let val: u32 = u32::MAX;
    let enc = encode_to_vec(&val).expect("encode u32::MAX");
    assert!(enc.len() >= 4, "u32::MAX must encode to at least 4 bytes");
    let (decoded, _): (u32, usize) = decode_from_slice(&enc).expect("decode u32::MAX");
    assert_eq!(decoded, val, "u32::MAX roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 7: u64=0 encodes to exactly 1 byte in standard config
// ---------------------------------------------------------------------------
#[test]
fn test_u64_zero_standard_size() {
    let val: u64 = 0;
    let enc = encode_to_vec(&val).expect("encode u64(0)");
    assert_eq!(
        enc.len(),
        1,
        "u64(0) must encode to exactly 1 byte in standard config"
    );
    let (decoded, _): (u64, usize) = decode_from_slice(&enc).expect("decode u64(0)");
    assert_eq!(decoded, val, "u64(0) roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 8: u64::MAX encodes to at least 8 bytes in standard config
// ---------------------------------------------------------------------------
#[test]
fn test_u64_max_standard_size() {
    let val: u64 = u64::MAX;
    let enc = encode_to_vec(&val).expect("encode u64::MAX");
    assert!(enc.len() >= 8, "u64::MAX must encode to at least 8 bytes");
    let (decoded, _): (u64, usize) = decode_from_slice(&enc).expect("decode u64::MAX");
    assert_eq!(decoded, val, "u64::MAX roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 9: Small u32 encodes to fewer or equal bytes than large u32
// ---------------------------------------------------------------------------
#[test]
fn test_u32_small_smaller_than_large() {
    let small: u32 = 1;
    let large: u32 = u32::MAX;
    let small_enc = encode_to_vec(&small).expect("encode u32(1)");
    let large_enc = encode_to_vec(&large).expect("encode u32::MAX");
    assert!(
        small_enc.len() <= large_enc.len(),
        "u32(1) ({} bytes) should encode to <= bytes than u32::MAX ({} bytes)",
        small_enc.len(),
        large_enc.len()
    );
}

// ---------------------------------------------------------------------------
// Test 10: Small u64 encodes to fewer or equal bytes than large u64
// ---------------------------------------------------------------------------
#[test]
fn test_u64_small_smaller_than_large() {
    let small: u64 = 1;
    let large: u64 = u64::MAX;
    let small_enc = encode_to_vec(&small).expect("encode u64(1)");
    let large_enc = encode_to_vec(&large).expect("encode u64::MAX");
    assert!(
        small_enc.len() <= large_enc.len(),
        "u64(1) ({} bytes) should encode to <= bytes than u64::MAX ({} bytes)",
        small_enc.len(),
        large_enc.len()
    );
}

// ---------------------------------------------------------------------------
// Test 11: u32 with fixed_int_encoding always uses exactly 4 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_u32_fixed_int_always_4_bytes() {
    let fixed_cfg = config::standard().with_fixed_int_encoding();
    let cases: &[u32] = &[0, 1, u32::MAX];
    for &val in cases {
        let enc = encode_to_vec_with_config(&val, fixed_cfg).expect("encode u32 fixed");
        assert_eq!(
            enc.len(),
            4,
            "u32({}) with fixed_int_encoding must always be exactly 4 bytes, got {}",
            val,
            enc.len()
        );
        let (decoded, _): (u32, usize) =
            decode_from_slice_with_config(&enc, fixed_cfg).expect("decode u32 fixed");
        assert_eq!(decoded, val, "u32({}) fixed_int roundtrip mismatch", val);
    }
}

// ---------------------------------------------------------------------------
// Test 12: u64 with fixed_int_encoding always uses exactly 8 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_u64_fixed_int_always_8_bytes() {
    let fixed_cfg = config::standard().with_fixed_int_encoding();
    let cases: &[u64] = &[0, 1, u64::MAX];
    for &val in cases {
        let enc = encode_to_vec_with_config(&val, fixed_cfg).expect("encode u64 fixed");
        assert_eq!(
            enc.len(),
            8,
            "u64({}) with fixed_int_encoding must always be exactly 8 bytes, got {}",
            val,
            enc.len()
        );
        let (decoded, _): (u64, usize) =
            decode_from_slice_with_config(&enc, fixed_cfg).expect("decode u64 fixed");
        assert_eq!(decoded, val, "u64({}) fixed_int roundtrip mismatch", val);
    }
}

// ---------------------------------------------------------------------------
// Test 13: u16 with fixed_int_encoding always uses exactly 2 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_u16_fixed_int_always_2_bytes() {
    let fixed_cfg = config::standard().with_fixed_int_encoding();
    let cases: &[u16] = &[0, 1, u16::MAX];
    for &val in cases {
        let enc = encode_to_vec_with_config(&val, fixed_cfg).expect("encode u16 fixed");
        assert_eq!(
            enc.len(),
            2,
            "u16({}) with fixed_int_encoding must always be exactly 2 bytes, got {}",
            val,
            enc.len()
        );
        let (decoded, _): (u16, usize) =
            decode_from_slice_with_config(&enc, fixed_cfg).expect("decode u16 fixed");
        assert_eq!(decoded, val, "u16({}) fixed_int roundtrip mismatch", val);
    }
}

// ---------------------------------------------------------------------------
// Test 14: u8 with fixed_int_encoding always uses exactly 1 byte
// ---------------------------------------------------------------------------
#[test]
fn test_u8_fixed_int_always_1_byte() {
    let fixed_cfg = config::standard().with_fixed_int_encoding();
    let cases: &[u8] = &[0, 1, u8::MAX];
    for &val in cases {
        let enc = encode_to_vec_with_config(&val, fixed_cfg).expect("encode u8 fixed");
        assert_eq!(
            enc.len(),
            1,
            "u8({}) with fixed_int_encoding must always be exactly 1 byte, got {}",
            val,
            enc.len()
        );
        let (decoded, _): (u8, usize) =
            decode_from_slice_with_config(&enc, fixed_cfg).expect("decode u8 fixed");
        assert_eq!(decoded, val, "u8({}) fixed_int roundtrip mismatch", val);
    }
}

// ---------------------------------------------------------------------------
// Test 15: i32=0 encodes to exactly 1 byte (zigzag(0) = 0, which is in 1-byte range)
// ---------------------------------------------------------------------------
#[test]
fn test_i32_zero_standard_size() {
    let val: i32 = 0;
    let enc = encode_to_vec(&val).expect("encode i32(0)");
    assert_eq!(
        enc.len(),
        1,
        "i32(0) zigzag=0 must encode to exactly 1 byte"
    );
    let (decoded, _): (i32, usize) = decode_from_slice(&enc).expect("decode i32(0)");
    assert_eq!(decoded, val, "i32(0) roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 16: i32=-1 encodes to exactly 1 byte (zigzag(-1) = 1, which is in 1-byte range)
// ---------------------------------------------------------------------------
#[test]
fn test_i32_minus_one_size() {
    let val: i32 = -1;
    let enc = encode_to_vec(&val).expect("encode i32(-1)");
    // zigzag(-1) = 1, which is <= 250, so must be exactly 1 byte
    assert_eq!(
        enc.len(),
        1,
        "i32(-1) zigzag=1 must encode to exactly 1 byte"
    );
    let (decoded, _): (i32, usize) = decode_from_slice(&enc).expect("decode i32(-1)");
    assert_eq!(decoded, val, "i32(-1) roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 17: i32::MAX encodes to at least 4 bytes (zigzag(i32::MAX) is large)
// ---------------------------------------------------------------------------
#[test]
fn test_i32_large_positive_size() {
    let val: i32 = i32::MAX;
    let enc = encode_to_vec(&val).expect("encode i32::MAX");
    assert!(enc.len() >= 4, "i32::MAX must encode to at least 4 bytes");
    let (decoded, _): (i32, usize) = decode_from_slice(&enc).expect("decode i32::MAX");
    assert_eq!(decoded, val, "i32::MAX roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 18: i64=0 encodes to exactly 1 byte (zigzag(0) = 0, 1-byte range)
// ---------------------------------------------------------------------------
#[test]
fn test_i64_zero_standard_size() {
    let val: i64 = 0;
    let enc = encode_to_vec(&val).expect("encode i64(0)");
    assert_eq!(
        enc.len(),
        1,
        "i64(0) zigzag=0 must encode to exactly 1 byte"
    );
    let (decoded, _): (i64, usize) = decode_from_slice(&enc).expect("decode i64(0)");
    assert_eq!(decoded, val, "i64(0) roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 19: Roundtrip for boundary values 0, 127, 128, 255, 256, 65535, 65536 as u32
// ---------------------------------------------------------------------------
#[test]
fn test_varint_roundtrip_boundary_values() {
    let boundary_vals: &[u32] = &[0, 127, 128, 255, 256, 65535, 65536];
    for &val in boundary_vals {
        let enc = encode_to_vec(&val).expect("encode boundary u32");
        let (decoded, consumed): (u32, usize) =
            decode_from_slice(&enc).expect("decode boundary u32");
        assert_eq!(decoded, val, "u32({}) boundary roundtrip mismatch", val);
        assert_eq!(
            consumed,
            enc.len(),
            "u32({}) consumed bytes must equal encoded length",
            val
        );
    }
    // Spot-check: 0 should be 1 byte (varint range 0..=250)
    let enc_zero = encode_to_vec(&0u32).expect("encode 0");
    assert_eq!(enc_zero.len(), 1, "u32(0) must be exactly 1 byte");
    // Spot-check: 65536 exceeds the u16 range so it needs more bytes than 65535
    let enc_65535 = encode_to_vec(&65535u32).expect("encode 65535");
    let enc_65536 = encode_to_vec(&65536u32).expect("encode 65536");
    assert!(
        enc_65535.len() <= enc_65536.len(),
        "u32(65535) should encode to <= bytes than u32(65536)"
    );
}

// ---------------------------------------------------------------------------
// Test 20: For u32::MAX, fixed_int (4 bytes) == standard varint size (also 5 bytes ≥ 4 bytes)
//          so fixed_int size <= standard size for u32::MAX
// ---------------------------------------------------------------------------
#[test]
fn test_fixed_int_vs_standard_large_value_size() {
    let val: u32 = u32::MAX;
    let std_cfg = config::standard();
    let fixed_cfg = config::standard().with_fixed_int_encoding();

    let std_enc = encode_to_vec_with_config(&val, std_cfg).expect("encode u32::MAX standard");
    let fixed_enc = encode_to_vec_with_config(&val, fixed_cfg).expect("encode u32::MAX fixed");

    assert_eq!(
        fixed_enc.len(),
        4,
        "u32::MAX with fixed_int must always be 4 bytes"
    );
    assert!(
        std_enc.len() >= 4,
        "u32::MAX with standard (varint) must be at least 4 bytes, got {}",
        std_enc.len()
    );

    // Both should roundtrip correctly
    let (std_decoded, _): (u32, usize) =
        decode_from_slice_with_config(&std_enc, std_cfg).expect("decode u32::MAX standard");
    let (fixed_decoded, _): (u32, usize) =
        decode_from_slice_with_config(&fixed_enc, fixed_cfg).expect("decode u32::MAX fixed");
    assert_eq!(std_decoded, val, "u32::MAX standard roundtrip mismatch");
    assert_eq!(fixed_decoded, val, "u32::MAX fixed roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 21: usize=0 roundtrip in standard config
// ---------------------------------------------------------------------------
#[test]
fn test_usize_zero_standard_roundtrip() {
    let val: usize = 0;
    let enc = encode_to_vec(&val).expect("encode usize(0)");
    assert_eq!(
        enc.len(),
        1,
        "usize(0) must encode to exactly 1 byte in standard config"
    );
    let (decoded, consumed): (usize, usize) = decode_from_slice(&enc).expect("decode usize(0)");
    assert_eq!(decoded, val, "usize(0) roundtrip mismatch");
    assert_eq!(
        consumed,
        enc.len(),
        "usize(0) consumed must equal encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 22: usize=1_000_000 roundtrip in standard config
// ---------------------------------------------------------------------------
#[test]
fn test_usize_large_roundtrip() {
    let val: usize = 1_000_000;
    let enc = encode_to_vec(&val).expect("encode usize(1_000_000)");
    // 1_000_000 > 65535, so it must take more than 3 bytes in varint
    assert!(
        enc.len() >= 3,
        "usize(1_000_000) must encode to at least 3 bytes"
    );
    let (decoded, consumed): (usize, usize) =
        decode_from_slice(&enc).expect("decode usize(1_000_000)");
    assert_eq!(decoded, val, "usize(1_000_000) roundtrip mismatch");
    assert_eq!(
        consumed,
        enc.len(),
        "usize(1_000_000) consumed must equal encoded length"
    );
}
