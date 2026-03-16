//! Advanced varint encoding format tests — exact byte-level verification (22 tests).
//!
//! OxiCode varint encoding for unsigned integers:
//! - Values 0-250:            1 byte  (the value itself)
//! - Values 251-65535:        marker 0xFB + 2 bytes LE (u16)
//! - Values 65536-4294967295: marker 0xFC + 4 bytes LE (u32)
//! - Values 4294967296+:      marker 0xFD + 8 bytes LE (u64)

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
use oxicode::{decode_from_slice, encode_to_vec};

// ---------------------------------------------------------------------------
// Test 1: u32(0) encodes as [0x00] (1 byte)
// ---------------------------------------------------------------------------
#[test]
fn test_varint_u32_zero_encodes_as_single_zero_byte() {
    let enc = encode_to_vec(&0u32).expect("encode u32(0)");
    assert_eq!(enc, vec![0x00], "u32(0) must encode as [0x00]");
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32(0)");
    assert_eq!(val, 0u32, "u32(0) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 2: u32(1) encodes as [0x01]
// ---------------------------------------------------------------------------
#[test]
fn test_varint_u32_one_encodes_as_0x01() {
    let enc = encode_to_vec(&1u32).expect("encode u32(1)");
    assert_eq!(enc, vec![0x01], "u32(1) must encode as [0x01]");
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32(1)");
    assert_eq!(val, 1u32, "u32(1) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 3: u32(127) encodes as [0x7F]
// ---------------------------------------------------------------------------
#[test]
fn test_varint_u32_127_encodes_as_0x7f() {
    let enc = encode_to_vec(&127u32).expect("encode u32(127)");
    assert_eq!(enc, vec![0x7F], "u32(127) must encode as [0x7F]");
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32(127)");
    assert_eq!(val, 127u32, "u32(127) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 4: u32(250) encodes as [0xFA] (max 1-byte value)
// ---------------------------------------------------------------------------
#[test]
fn test_varint_u32_250_encodes_as_0xfa_max_single_byte() {
    let enc = encode_to_vec(&250u32).expect("encode u32(250)");
    assert_eq!(
        enc,
        vec![0xFA],
        "u32(250) must encode as [0xFA] — max 1-byte varint"
    );
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32(250)");
    assert_eq!(val, 250u32, "u32(250) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 5: u32(251) encodes as [0xFB, 0xFB, 0x00] (marker + u16 LE)
// ---------------------------------------------------------------------------
#[test]
fn test_varint_u32_251_encodes_with_u16_marker() {
    let enc = encode_to_vec(&251u32).expect("encode u32(251)");
    // 251 in LE u16 is [0xFB, 0x00]; prefixed by marker 0xFB
    assert_eq!(enc, vec![0xFB, 0xFB, 0x00], "u32(251) varint bytes");
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32(251)");
    assert_eq!(val, 251u32, "u32(251) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 6: u32(256) encodes as [0xFB, 0x00, 0x01]
// ---------------------------------------------------------------------------
#[test]
fn test_varint_u32_256_encodes_as_fb_00_01() {
    let enc = encode_to_vec(&256u32).expect("encode u32(256)");
    // 256 in LE u16 is [0x00, 0x01]; prefixed by marker 0xFB
    assert_eq!(enc, vec![0xFB, 0x00, 0x01], "u32(256) varint bytes");
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32(256)");
    assert_eq!(val, 256u32, "u32(256) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 7: u32(65535) encodes as [0xFB, 0xFF, 0xFF]
// ---------------------------------------------------------------------------
#[test]
fn test_varint_u32_65535_encodes_as_fb_ff_ff() {
    let enc = encode_to_vec(&65535u32).expect("encode u32(65535)");
    // 65535 = u16::MAX in LE is [0xFF, 0xFF]; prefixed by marker 0xFB
    assert_eq!(enc, vec![0xFB, 0xFF, 0xFF], "u32(65535) varint bytes");
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32(65535)");
    assert_eq!(val, 65535u32, "u32(65535) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 8: u32(65536) encodes as [0xFC, 0x00, 0x00, 0x01, 0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_varint_u32_65536_encodes_with_u32_marker() {
    let enc = encode_to_vec(&65536u32).expect("encode u32(65536)");
    // 65536 = 0x00010000 in LE u32 is [0x00, 0x00, 0x01, 0x00]; prefixed by marker 0xFC
    assert_eq!(
        enc,
        vec![0xFC, 0x00, 0x00, 0x01, 0x00],
        "u32(65536) varint bytes"
    );
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32(65536)");
    assert_eq!(val, 65536u32, "u32(65536) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 9: u32(u32::MAX) encodes as [0xFC, 0xFF, 0xFF, 0xFF, 0xFF]
// ---------------------------------------------------------------------------
#[test]
fn test_varint_u32_max_encodes_as_fc_ff_ff_ff_ff() {
    let enc = encode_to_vec(&u32::MAX).expect("encode u32::MAX");
    // u32::MAX = 0xFFFFFFFF in LE is [0xFF, 0xFF, 0xFF, 0xFF]; prefixed by marker 0xFC
    assert_eq!(
        enc,
        vec![0xFC, 0xFF, 0xFF, 0xFF, 0xFF],
        "u32::MAX varint bytes"
    );
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32::MAX");
    assert_eq!(val, u32::MAX, "u32::MAX roundtrip");
}

// ---------------------------------------------------------------------------
// Test 10: u64(u32::MAX + 1) encodes with 0xFD marker (9 bytes total)
// ---------------------------------------------------------------------------
#[test]
fn test_varint_u64_u32max_plus_one_encodes_with_u64_marker() {
    let v: u64 = u32::MAX as u64 + 1;
    let enc = encode_to_vec(&v).expect("encode u64(u32::MAX + 1)");
    // marker 0xFD + 8 bytes LE
    assert_eq!(enc.len(), 9, "u64(u32::MAX+1) must encode as 9 bytes");
    assert_eq!(enc[0], 0xFD, "first byte must be 0xFD marker");
    // value 0x0000_0001_0000_0000 in LE is [0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]
    assert_eq!(
        enc[1..],
        [0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
        "u64(u32::MAX+1) LE bytes"
    );
    let (val, _): (u64, _) = decode_from_slice(&enc).expect("decode u64(u32::MAX+1)");
    assert_eq!(val, v, "u64(u32::MAX+1) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 11: u64(u64::MAX) encodes as [0xFD, 0xFF x8]
// ---------------------------------------------------------------------------
#[test]
fn test_varint_u64_max_encodes_as_fd_followed_by_eight_ff_bytes() {
    let enc = encode_to_vec(&u64::MAX).expect("encode u64::MAX");
    assert_eq!(
        enc,
        vec![0xFD, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
        "u64::MAX must encode as [0xFD, 0xFF x8]"
    );
    let (val, _): (u64, _) = decode_from_slice(&enc).expect("decode u64::MAX");
    assert_eq!(val, u64::MAX, "u64::MAX roundtrip");
}

// ---------------------------------------------------------------------------
// Test 12: All values 0-250 encode as exactly 1 byte
// ---------------------------------------------------------------------------
#[test]
fn test_varint_all_values_0_to_250_encode_as_exactly_one_byte() {
    for v in 0u32..=250 {
        let enc = encode_to_vec(&v).expect("encode u32 in 0-250 range");
        assert_eq!(
            enc.len(),
            1,
            "u32({}) must encode as 1 byte, got {} bytes",
            v,
            enc.len()
        );
        assert_eq!(
            enc[0], v as u8,
            "u32({}) single byte must equal the value itself",
            v
        );
    }
}

// ---------------------------------------------------------------------------
// Test 13: Values 251-65535 encode as exactly 3 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_varint_values_251_to_65535_encode_as_exactly_three_bytes() {
    // Sample boundaries and interior values rather than exhaustive scan for speed
    let samples: &[u32] = &[251, 252, 500, 1000, 32767, 32768, 65534, 65535];
    for &v in samples {
        let enc = encode_to_vec(&v).expect("encode u32 in 251-65535 range");
        assert_eq!(
            enc.len(),
            3,
            "u32({}) must encode as 3 bytes (0xFB + 2 LE), got {} bytes",
            v,
            enc.len()
        );
        assert_eq!(enc[0], 0xFB, "u32({}) first byte must be 0xFB marker", v);
    }
}

// ---------------------------------------------------------------------------
// Test 14: Values 65536-4294967295 encode as exactly 5 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_varint_values_65536_to_u32max_encode_as_exactly_five_bytes() {
    let samples: &[u32] = &[65536, 65537, 100_000, 1_000_000, u32::MAX - 1, u32::MAX];
    for &v in samples {
        let enc = encode_to_vec(&v).expect("encode u32 in 65536-u32::MAX range");
        assert_eq!(
            enc.len(),
            5,
            "u32({}) must encode as 5 bytes (0xFC + 4 LE), got {} bytes",
            v,
            enc.len()
        );
        assert_eq!(enc[0], 0xFC, "u32({}) first byte must be 0xFC marker", v);
    }
}

// ---------------------------------------------------------------------------
// Test 15: Values > u32::MAX encode as exactly 9 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_varint_values_above_u32max_encode_as_exactly_nine_bytes() {
    let samples: &[u64] = &[
        u32::MAX as u64 + 1,
        u32::MAX as u64 + 2,
        1_000_000_000_000u64,
        u64::MAX - 1,
        u64::MAX,
    ];
    for &v in samples {
        let enc = encode_to_vec(&v).expect("encode u64 above u32::MAX");
        assert_eq!(
            enc.len(),
            9,
            "u64({}) must encode as 9 bytes (0xFD + 8 LE), got {} bytes",
            v,
            enc.len()
        );
        assert_eq!(enc[0], 0xFD, "u64({}) first byte must be 0xFD marker", v);
    }
}

// ---------------------------------------------------------------------------
// Test 16: i32(0) zigzag encodes as [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_varint_i32_zero_zigzag_encodes_as_0x00() {
    let enc = encode_to_vec(&0i32).expect("encode i32(0)");
    // zigzag(0) = 0 => single byte 0x00
    assert_eq!(enc, vec![0x00], "i32(0) zigzag must encode as [0x00]");
    let (val, _): (i32, _) = decode_from_slice(&enc).expect("decode i32(0)");
    assert_eq!(val, 0i32, "i32(0) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 17: i32(-1) zigzag encodes as [0x01]
// ---------------------------------------------------------------------------
#[test]
fn test_varint_i32_neg1_zigzag_encodes_as_0x01() {
    let enc = encode_to_vec(&(-1i32)).expect("encode i32(-1)");
    // zigzag(-1) = 1 => single byte 0x01
    assert_eq!(enc, vec![0x01], "i32(-1) zigzag must encode as [0x01]");
    let (val, _): (i32, _) = decode_from_slice(&enc).expect("decode i32(-1)");
    assert_eq!(val, -1i32, "i32(-1) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 18: i32(1) zigzag encodes as [0x02]
// ---------------------------------------------------------------------------
#[test]
fn test_varint_i32_pos1_zigzag_encodes_as_0x02() {
    let enc = encode_to_vec(&1i32).expect("encode i32(1)");
    // zigzag(1) = 2 => single byte 0x02
    assert_eq!(enc, vec![0x02], "i32(1) zigzag must encode as [0x02]");
    let (val, _): (i32, _) = decode_from_slice(&enc).expect("decode i32(1)");
    assert_eq!(val, 1i32, "i32(1) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 19: i32(-2) zigzag encodes as [0x03]
// ---------------------------------------------------------------------------
#[test]
fn test_varint_i32_neg2_zigzag_encodes_as_0x03() {
    let enc = encode_to_vec(&(-2i32)).expect("encode i32(-2)");
    // zigzag(-2) = 3 => single byte 0x03
    assert_eq!(enc, vec![0x03], "i32(-2) zigzag must encode as [0x03]");
    let (val, _): (i32, _) = decode_from_slice(&enc).expect("decode i32(-2)");
    assert_eq!(val, -2i32, "i32(-2) roundtrip");
}

// ---------------------------------------------------------------------------
// Test 20: i32(i32::MIN) zigzag encodes as 5 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_varint_i32_min_zigzag_encodes_as_five_bytes() {
    let enc = encode_to_vec(&i32::MIN).expect("encode i32::MIN");
    // zigzag(i32::MIN) = (i32::MIN as u32).wrapping_shl(1) ^ ((i32::MIN >> 31) as u32)
    // = 0x8000_0000 << 1 ^ 0xFFFF_FFFF = 0x0000_0000 ^ 0xFFFF_FFFF = 0xFFFF_FFFF = u32::MAX
    // u32::MAX = 4294967295 > 65535, so 5 bytes: [0xFC, 0xFF, 0xFF, 0xFF, 0xFF]
    assert_eq!(
        enc.len(),
        5,
        "i32::MIN zigzag=u32::MAX must encode as 5 bytes"
    );
    assert_eq!(
        enc[0], 0xFC,
        "i32::MIN first byte must be 0xFC (u32 varint marker)"
    );
    let (val, _): (i32, _) = decode_from_slice(&enc).expect("decode i32::MIN");
    assert_eq!(val, i32::MIN, "i32::MIN roundtrip");
}

// ---------------------------------------------------------------------------
// Test 21: Decode then re-encode preserves byte-for-byte identity
// ---------------------------------------------------------------------------
#[test]
fn test_varint_decode_reencode_byte_identity_various_values() {
    let u32_values: &[u32] = &[0, 1, 127, 250, 251, 256, 65535, 65536, u32::MAX];
    for &v in u32_values {
        let enc1 = encode_to_vec(&v).expect("first encode");
        let (decoded, _): (u32, _) = decode_from_slice(&enc1).expect("decode");
        let enc2 = encode_to_vec(&decoded).expect("second encode");
        assert_eq!(
            enc1, enc2,
            "re-encoding u32({}) must produce identical bytes",
            v
        );
    }

    let u64_values: &[u64] = &[
        0,
        250,
        251,
        65535,
        65536,
        u32::MAX as u64,
        u32::MAX as u64 + 1,
        u64::MAX,
    ];
    for &v in u64_values {
        let enc1 = encode_to_vec(&v).expect("first encode u64");
        let (decoded, _): (u64, _) = decode_from_slice(&enc1).expect("decode u64");
        let enc2 = encode_to_vec(&decoded).expect("second encode u64");
        assert_eq!(
            enc1, enc2,
            "re-encoding u64({}) must produce identical bytes",
            v
        );
    }
}

// ---------------------------------------------------------------------------
// Test 22: Big-endian config with u32(0x01020304) fixed-int: [0x01, 0x02, 0x03, 0x04]
// ---------------------------------------------------------------------------
#[test]
fn test_varint_big_endian_fixed_int_u32_exact_bytes() {
    use oxicode::{config, encode_to_vec_with_config};

    let v: u32 = 0x01020304;
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&v, cfg).expect("encode u32(0x01020304) big-endian fixed");
    assert_eq!(
        enc,
        vec![0x01, 0x02, 0x03, 0x04],
        "u32(0x01020304) with big-endian fixed-int must encode as [0x01, 0x02, 0x03, 0x04]"
    );
    let (val, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&enc, cfg).expect("decode big-endian fixed u32");
    assert_eq!(val, v, "u32(0x01020304) big-endian fixed roundtrip");
}
