//! Advanced numeric boundary tests — 22 top-level #[test] functions.
//!
//! Tests cover roundtrip correctness, encoded size guarantees, and byte-order
//! properties for all primitive integer types at their extreme values using
//! both the standard varint configuration and the fixed-int configuration.

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
// 1. All u8 boundary values: 0, 1, 127, 128, 254, 255 roundtrips in a loop
// ---------------------------------------------------------------------------
#[test]
fn test_u8_all_boundaries_roundtrip() {
    let values: [u8; 6] = [0, 1, 127, 128, 254, 255];
    for v in values {
        let encoded = encode_to_vec(&v).expect("encode u8 boundary");
        let (decoded, _): (u8, _) = decode_from_slice(&encoded).expect("decode u8 boundary");
        assert_eq!(v, decoded, "u8 roundtrip failed for value {v}");
    }
}

// ---------------------------------------------------------------------------
// 2. All u16 boundaries roundtrips
// ---------------------------------------------------------------------------
#[test]
fn test_u16_all_boundaries_roundtrip() {
    let values: [u16; 10] = [0, 1, 127, 128, 255, 256, 16383, 16384, 65534, 65535];
    for v in values {
        let encoded = encode_to_vec(&v).expect("encode u16 boundary");
        let (decoded, _): (u16, _) = decode_from_slice(&encoded).expect("decode u16 boundary");
        assert_eq!(v, decoded, "u16 roundtrip failed for value {v}");
    }
}

// ---------------------------------------------------------------------------
// 3. u32 boundaries roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u32_boundaries_roundtrip() {
    let values: [u32; 5] = [0, 1, u16::MAX as u32, u16::MAX as u32 + 1, u32::MAX];
    for v in values {
        let encoded = encode_to_vec(&v).expect("encode u32 boundary");
        let (decoded, _): (u32, _) = decode_from_slice(&encoded).expect("decode u32 boundary");
        assert_eq!(v, decoded, "u32 roundtrip failed for value {v}");
    }
}

// ---------------------------------------------------------------------------
// 4. u64 boundaries roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u64_boundaries_roundtrip() {
    let values: [u64; 5] = [0, 1, u32::MAX as u64, u32::MAX as u64 + 1, u64::MAX];
    for v in values {
        let encoded = encode_to_vec(&v).expect("encode u64 boundary");
        let (decoded, _): (u64, _) = decode_from_slice(&encoded).expect("decode u64 boundary");
        assert_eq!(v, decoded, "u64 roundtrip failed for value {v}");
    }
}

// ---------------------------------------------------------------------------
// 5. i8 boundaries roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i8_boundaries_roundtrip() {
    let values: [i8; 5] = [i8::MIN, -1, 0, 1, i8::MAX];
    for v in values {
        let encoded = encode_to_vec(&v).expect("encode i8 boundary");
        let (decoded, _): (i8, _) = decode_from_slice(&encoded).expect("decode i8 boundary");
        assert_eq!(v, decoded, "i8 roundtrip failed for value {v}");
    }
}

// ---------------------------------------------------------------------------
// 6. i16 boundaries roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i16_boundaries_roundtrip() {
    let values: [i16; 5] = [i16::MIN, -1, 0, 1, i16::MAX];
    for v in values {
        let encoded = encode_to_vec(&v).expect("encode i16 boundary");
        let (decoded, _): (i16, _) = decode_from_slice(&encoded).expect("decode i16 boundary");
        assert_eq!(v, decoded, "i16 roundtrip failed for value {v}");
    }
}

// ---------------------------------------------------------------------------
// 7. i32 boundaries roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i32_boundaries_roundtrip() {
    let values: [i32; 5] = [i32::MIN, -1, 0, 1, i32::MAX];
    for v in values {
        let encoded = encode_to_vec(&v).expect("encode i32 boundary");
        let (decoded, _): (i32, _) = decode_from_slice(&encoded).expect("decode i32 boundary");
        assert_eq!(v, decoded, "i32 roundtrip failed for value {v}");
    }
}

// ---------------------------------------------------------------------------
// 8. i64 boundaries roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i64_boundaries_roundtrip() {
    let values: [i64; 5] = [i64::MIN, -1, 0, 1, i64::MAX];
    for v in values {
        let encoded = encode_to_vec(&v).expect("encode i64 boundary");
        let (decoded, _): (i64, _) = decode_from_slice(&encoded).expect("decode i64 boundary");
        assert_eq!(v, decoded, "i64 roundtrip failed for value {v}");
    }
}

// ---------------------------------------------------------------------------
// 9. u128 boundaries roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u128_boundaries_roundtrip() {
    let values: [u128; 3] = [0, u64::MAX as u128, u128::MAX];
    for v in values {
        let encoded = encode_to_vec(&v).expect("encode u128 boundary");
        let (decoded, _): (u128, _) = decode_from_slice(&encoded).expect("decode u128 boundary");
        assert_eq!(v, decoded, "u128 roundtrip failed for value {v}");
    }
}

// ---------------------------------------------------------------------------
// 10. i128 boundaries roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i128_boundaries_roundtrip() {
    let values: [i128; 5] = [i128::MIN, -1, 0, 1, i128::MAX];
    for v in values {
        let encoded = encode_to_vec(&v).expect("encode i128 boundary");
        let (decoded, _): (i128, _) = decode_from_slice(&encoded).expect("decode i128 boundary");
        assert_eq!(v, decoded, "i128 roundtrip failed for value {v}");
    }
}

// ---------------------------------------------------------------------------
// 11. Encoding size increases for larger values (varint property)
//     Verifies that the varint scheme produces strictly smaller encodings for
//     small values compared to large values within the same type.
// ---------------------------------------------------------------------------
#[test]
fn test_varint_encoding_size_increases_with_value() {
    // u64: 0 → 1 byte, u16::MAX → 3 bytes, u32::MAX → 5 bytes, u64::MAX → 9 bytes
    let enc_0 = encode_to_vec(&0u64).expect("encode 0u64");
    let enc_u16_max = encode_to_vec(&(u16::MAX as u64)).expect("encode u16::MAX as u64");
    let enc_u32_max = encode_to_vec(&(u32::MAX as u64)).expect("encode u32::MAX as u64");
    let enc_u64_max = encode_to_vec(&u64::MAX).expect("encode u64::MAX");

    assert!(
        enc_0.len() < enc_u16_max.len(),
        "0u64 ({}) must be smaller than u16::MAX ({})",
        enc_0.len(),
        enc_u16_max.len()
    );
    assert!(
        enc_u16_max.len() < enc_u32_max.len(),
        "u16::MAX ({}) must be smaller than u32::MAX ({})",
        enc_u16_max.len(),
        enc_u32_max.len()
    );
    assert!(
        enc_u32_max.len() < enc_u64_max.len(),
        "u32::MAX ({}) must be smaller than u64::MAX ({})",
        enc_u32_max.len(),
        enc_u64_max.len()
    );
}

// ---------------------------------------------------------------------------
// 12. u8::MAX encodes in 1 byte (varint), u16::MAX encodes in multiple bytes
// ---------------------------------------------------------------------------
#[test]
fn test_u8_max_1_byte_u16_max_multi_byte() {
    // u8::MAX = 255, which is above SINGLE_BYTE_MAX (250), so as u64 it takes 3 bytes
    // but as u8 the type itself only occupies 1 byte (fixed representation)
    let enc_u8_max = encode_to_vec(&u8::MAX).expect("encode u8::MAX");
    assert_eq!(
        enc_u8_max.len(),
        1,
        "u8::MAX must encode as exactly 1 byte, got {}",
        enc_u8_max.len()
    );

    // u16::MAX as u16 with varint encoding: 3 bytes (marker + 2 data bytes)
    let enc_u16_max = encode_to_vec(&u16::MAX).expect("encode u16::MAX");
    assert!(
        enc_u16_max.len() > 1,
        "u16::MAX must encode in more than 1 byte, got {}",
        enc_u16_max.len()
    );
}

// ---------------------------------------------------------------------------
// 13. Fixed-int u8 is always 1 byte
// ---------------------------------------------------------------------------
#[test]
fn test_fixed_int_u8_always_1_byte() {
    let cfg = config::standard().with_fixed_int_encoding();
    for v in [0u8, 1, 127, 128, 254, 255] {
        let encoded = encode_to_vec_with_config(&v, cfg).expect("encode u8 fixed-int");
        assert_eq!(
            encoded.len(),
            1,
            "fixed-int u8 value {v} must occupy exactly 1 byte, got {}",
            encoded.len()
        );
        let (decoded, _): (u8, _) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode u8 fixed-int");
        assert_eq!(v, decoded, "fixed-int u8 roundtrip failed for {v}");
    }
}

// ---------------------------------------------------------------------------
// 14. Fixed-int u16 is always 2 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_fixed_int_u16_always_2_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for v in [0u16, 1, 255, 256, 16384, 65534, 65535] {
        let encoded = encode_to_vec_with_config(&v, cfg).expect("encode u16 fixed-int");
        assert_eq!(
            encoded.len(),
            2,
            "fixed-int u16 value {v} must occupy exactly 2 bytes, got {}",
            encoded.len()
        );
        let (decoded, _): (u16, _) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode u16 fixed-int");
        assert_eq!(v, decoded, "fixed-int u16 roundtrip failed for {v}");
    }
}

// ---------------------------------------------------------------------------
// 15. Fixed-int u32 is always 4 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_fixed_int_u32_always_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for v in [0u32, 1, u16::MAX as u32, u16::MAX as u32 + 1, u32::MAX] {
        let encoded = encode_to_vec_with_config(&v, cfg).expect("encode u32 fixed-int");
        assert_eq!(
            encoded.len(),
            4,
            "fixed-int u32 value {v} must occupy exactly 4 bytes, got {}",
            encoded.len()
        );
        let (decoded, _): (u32, _) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode u32 fixed-int");
        assert_eq!(v, decoded, "fixed-int u32 roundtrip failed for {v}");
    }
}

// ---------------------------------------------------------------------------
// 16. Fixed-int u64 is always 8 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_fixed_int_u64_always_8_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for v in [0u64, 1, u32::MAX as u64, u32::MAX as u64 + 1, u64::MAX] {
        let encoded = encode_to_vec_with_config(&v, cfg).expect("encode u64 fixed-int");
        assert_eq!(
            encoded.len(),
            8,
            "fixed-int u64 value {v} must occupy exactly 8 bytes, got {}",
            encoded.len()
        );
        let (decoded, _): (u64, _) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode u64 fixed-int");
        assert_eq!(v, decoded, "fixed-int u64 roundtrip failed for {v}");
    }
}

// ---------------------------------------------------------------------------
// 17. Big-endian u32 boundary value byte order check
//     With big-endian fixed encoding, the most significant byte comes first.
// ---------------------------------------------------------------------------
#[test]
fn test_big_endian_u32_byte_order() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    // Value 0x01020304: big-endian bytes should be [0x01, 0x02, 0x03, 0x04]
    let v: u32 = 0x01020304;
    let encoded = encode_to_vec_with_config(&v, cfg).expect("encode big-endian u32");
    assert_eq!(encoded.len(), 4, "fixed big-endian u32 must be 4 bytes");
    assert_eq!(
        encoded[0], 0x01,
        "big-endian: byte 0 must be 0x01, got 0x{:02X}",
        encoded[0]
    );
    assert_eq!(
        encoded[1], 0x02,
        "big-endian: byte 1 must be 0x02, got 0x{:02X}",
        encoded[1]
    );
    assert_eq!(
        encoded[2], 0x03,
        "big-endian: byte 2 must be 0x03, got 0x{:02X}",
        encoded[2]
    );
    assert_eq!(
        encoded[3], 0x04,
        "big-endian: byte 3 must be 0x04, got 0x{:02X}",
        encoded[3]
    );

    // Roundtrip
    let (decoded, _): (u32, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode big-endian u32");
    assert_eq!(v, decoded, "big-endian u32 roundtrip failed");
}

// ---------------------------------------------------------------------------
// 18. i32::MIN zigzag encoding — consumed == encoded.len()
//     i32::MIN zigzag maps to u32::MAX (0xFFFF_FFFF), which needs 5 varint bytes.
// ---------------------------------------------------------------------------
#[test]
fn test_i32_min_zigzag_consumed_equals_encoded_len() {
    let v = i32::MIN;
    let encoded = encode_to_vec(&v).expect("encode i32::MIN");

    // Zigzag(i32::MIN) = u32::MAX = 0xFFFF_FFFF, which is above u16::MAX,
    // so the varint encoding requires 5 bytes (marker + 4 data bytes).
    assert_eq!(
        encoded.len(),
        5,
        "i32::MIN must encode as 5 bytes via zigzag varint, got {}",
        encoded.len()
    );

    let (decoded, consumed): (i32, _) = decode_from_slice(&encoded).expect("decode i32::MIN");
    assert_eq!(v, decoded, "i32::MIN roundtrip failed");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes ({consumed}) must equal encoded length ({})",
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// 19. Vec of all u8 boundary values roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_boundaries_roundtrip() {
    let values: Vec<u8> = vec![0, 1, 127, 128, 254, 255];
    let encoded = encode_to_vec(&values).expect("encode Vec<u8> boundaries");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_from_slice(&encoded).expect("decode Vec<u8> boundaries");
    assert_eq!(values, decoded, "Vec<u8> boundaries roundtrip failed");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed ({consumed}) must equal encoded length ({})",
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// 20. Option<u64::MAX> Some roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_u64_max_some_roundtrip() {
    let v: Option<u64> = Some(u64::MAX);
    let encoded = encode_to_vec(&v).expect("encode Option<u64::MAX> Some");
    let (decoded, consumed): (Option<u64>, _) =
        decode_from_slice(&encoded).expect("decode Option<u64::MAX> Some");
    assert_eq!(v, decoded, "Option<u64::MAX> Some roundtrip failed");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed ({consumed}) must equal encoded length ({})",
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// 21. Tuple of (u8::MAX, u16::MAX, u32::MAX, u64::MAX) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tuple_unsigned_max_roundtrip() {
    let v: (u8, u16, u32, u64) = (u8::MAX, u16::MAX, u32::MAX, u64::MAX);
    let encoded = encode_to_vec(&v).expect("encode tuple (u8::MAX, u16::MAX, u32::MAX, u64::MAX)");
    let (decoded, consumed): ((u8, u16, u32, u64), _) =
        decode_from_slice(&encoded).expect("decode tuple (u8::MAX, u16::MAX, u32::MAX, u64::MAX)");
    assert_eq!(
        v, decoded,
        "tuple (u8::MAX, u16::MAX, u32::MAX, u64::MAX) roundtrip failed"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed ({consumed}) must equal encoded length ({})",
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// 22. Tuple of (i8::MIN, i16::MIN, i32::MIN, i64::MIN) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tuple_signed_min_roundtrip() {
    let v: (i8, i16, i32, i64) = (i8::MIN, i16::MIN, i32::MIN, i64::MIN);
    let encoded = encode_to_vec(&v).expect("encode tuple (i8::MIN, i16::MIN, i32::MIN, i64::MIN)");
    let (decoded, consumed): ((i8, i16, i32, i64), _) =
        decode_from_slice(&encoded).expect("decode tuple (i8::MIN, i16::MIN, i32::MIN, i64::MIN)");
    assert_eq!(
        v, decoded,
        "tuple (i8::MIN, i16::MIN, i32::MIN, i64::MIN) roundtrip failed"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed ({consumed}) must equal encoded length ({})",
        encoded.len()
    );
}
