//! Advanced array/slice serialization tests — second batch.
//!
//! Covers angles not exercised by fixed_array_advanced_test.rs or
//! array_encoding_advanced_test.rs:
//!   * Zero-element fixed arrays, exact byte patterns, all-0xFF payloads
//!   * Signed/negative values, float consts, wide byte-range arrays
//!   * Nested 2D arrays, tuple arrays, Option arrays
//!   * Config variants (fixed-int, big-endian)
//!   * Wire-size invariants, consumed-byte assertions
//!   * Struct containing a fixed-array field with a derived codec

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
    encode_to_vec_with_config, Decode, Encode,
};

// ============================================================================
// Test 1 — [u8; 0] empty fixed array roundtrip
// ============================================================================

#[test]
fn test_array_slice_adv2_empty_u8_0_roundtrip() {
    let original: [u8; 0] = [];
    let encoded = encode_to_vec(&original).expect("encode [u8; 0]");
    // Fixed-size arrays have no length prefix; 0-element array → 0 bytes.
    assert_eq!(encoded.len(), 0, "[u8; 0] must produce zero encoded bytes");

    let (decoded, consumed): ([u8; 0], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 0]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 0, "consuming [u8; 0] must advance by 0 bytes");
}

// ============================================================================
// Test 2 — [u8; 1] single-element roundtrip
// ============================================================================

#[test]
fn test_array_slice_adv2_single_u8_roundtrip() {
    let original: [u8; 1] = [0xC3];
    let encoded = encode_to_vec(&original).expect("encode [u8; 1]");
    assert_eq!(encoded.len(), 1, "[u8; 1] must encode as exactly 1 byte");
    assert_eq!(
        encoded[0], 0xC3,
        "wire byte must match source byte verbatim"
    );

    let (decoded, consumed): ([u8; 1], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 1]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 1);
}

// ============================================================================
// Test 3 — [u8; 16] all-zeros roundtrip
// ============================================================================

#[test]
fn test_array_slice_adv2_u8_16_all_zeros() {
    let original: [u8; 16] = [0u8; 16];
    let encoded = encode_to_vec(&original).expect("encode [u8; 16] zeros");
    assert_eq!(
        encoded.len(),
        16,
        "[u8; 16] all-zero must be exactly 16 bytes on the wire"
    );
    assert!(
        encoded.iter().all(|&b| b == 0),
        "every wire byte must be zero"
    );

    let (decoded, consumed): ([u8; 16], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 16] zeros");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 16);
}

// ============================================================================
// Test 4 — [u8; 16] all-0xFF roundtrip
// ============================================================================

#[test]
fn test_array_slice_adv2_u8_16_all_ff() {
    let original: [u8; 16] = [0xFF; 16];
    let encoded = encode_to_vec(&original).expect("encode [u8; 16] 0xFF");
    assert_eq!(
        encoded.len(),
        16,
        "[u8; 16] all-0xFF must be exactly 16 bytes — no length prefix"
    );
    assert!(
        encoded.iter().all(|&b| b == 0xFF),
        "every wire byte must be 0xFF"
    );

    let (decoded, consumed): ([u8; 16], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 16] 0xFF");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 16);
}

// ============================================================================
// Test 5 — [u32; 4] mixed values roundtrip
// ============================================================================

#[test]
fn test_array_slice_adv2_u32_4_mixed_roundtrip() {
    let original: [u32; 4] = [0, 127, 256, u32::MAX];
    let encoded = encode_to_vec(&original).expect("encode [u32; 4] mixed");
    // Not asserting a specific length because varint encoding varies per value.
    assert!(!encoded.is_empty(), "encoded bytes must not be empty");

    let (decoded, consumed): ([u32; 4], usize) =
        decode_from_slice(&encoded).expect("decode [u32; 4] mixed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ============================================================================
// Test 6 — [i64; 3] negative values roundtrip
// ============================================================================

#[test]
fn test_array_slice_adv2_i64_3_negative_roundtrip() {
    let original: [i64; 3] = [i64::MIN, -1, -123_456_789];
    let encoded = encode_to_vec(&original).expect("encode [i64; 3] negative");
    assert!(
        !encoded.is_empty(),
        "negative i64 array must encode non-empty"
    );

    let (decoded, consumed): ([i64; 3], usize) =
        decode_from_slice(&encoded).expect("decode [i64; 3] negative");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ============================================================================
// Test 7 — [f64; 2] float array using named constants (bit-exact)
// ============================================================================

#[test]
fn test_array_slice_adv2_f64_2_named_consts_roundtrip() {
    let original: [f64; 2] = [std::f64::consts::PI, std::f64::consts::LN_2];
    let encoded = encode_to_vec(&original).expect("encode [f64; 2] consts");
    // Two f64 values at 8 bytes each → 16 bytes total, no length prefix.
    assert_eq!(
        encoded.len(),
        16,
        "[f64; 2] must encode as exactly 16 bytes"
    );

    let (decoded, consumed): ([f64; 2], usize) =
        decode_from_slice(&encoded).expect("decode [f64; 2] consts");
    assert_eq!(consumed, 16);
    assert_eq!(
        decoded[0].to_bits(),
        original[0].to_bits(),
        "PI must be bit-exact after roundtrip"
    );
    assert_eq!(
        decoded[1].to_bits(),
        original[1].to_bits(),
        "LN_2 must be bit-exact after roundtrip"
    );
}

// ============================================================================
// Test 8 — [bool; 8] bool array roundtrip
// ============================================================================

#[test]
fn test_array_slice_adv2_bool_8_roundtrip() {
    let original: [bool; 8] = [false, true, false, false, true, true, false, true];
    let encoded = encode_to_vec(&original).expect("encode [bool; 8]");
    // Each bool encodes as 1 byte; [bool; 8] → 8 bytes, no length prefix.
    assert_eq!(
        encoded.len(),
        8,
        "[bool; 8] must be exactly 8 bytes on the wire"
    );

    let (decoded, consumed): ([bool; 8], usize) =
        decode_from_slice(&encoded).expect("decode [bool; 8]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 8);
}

// ============================================================================
// Test 9 — [u8; 256] full byte-range array
// ============================================================================

#[test]
fn test_array_slice_adv2_u8_256_full_range() {
    let mut original = [0u8; 256];
    for (i, slot) in original.iter_mut().enumerate() {
        *slot = i as u8;
    }
    let encoded = encode_to_vec(&original).expect("encode [u8; 256]");
    assert_eq!(
        encoded.len(),
        256,
        "[u8; 256] must encode as exactly 256 bytes"
    );
    // Wire bytes must be the identity mapping 0..=255.
    for (i, &byte) in encoded.iter().enumerate() {
        assert_eq!(byte, i as u8, "wire byte at position {i} must equal {i}");
    }

    let (decoded, consumed): ([u8; 256], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 256]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 256);
}

// ============================================================================
// Test 10 — [u16; 4] roundtrip
// ============================================================================

#[test]
fn test_array_slice_adv2_u16_4_roundtrip() {
    let original: [u16; 4] = [0, 255, 256, u16::MAX];
    let encoded = encode_to_vec(&original).expect("encode [u16; 4]");
    assert!(
        !encoded.is_empty(),
        "[u16; 4] encoded bytes must not be empty"
    );

    let (decoded, consumed): ([u16; 4], usize) =
        decode_from_slice(&encoded).expect("decode [u16; 4]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ============================================================================
// Test 11 — [[u8; 4]; 3] 2D fixed array roundtrip
// ============================================================================

#[test]
fn test_array_slice_adv2_nested_u8_4_3_roundtrip() {
    let original: [[u8; 4]; 3] = [
        [0x11, 0x22, 0x33, 0x44],
        [0x55, 0x66, 0x77, 0x88],
        [0x99, 0xAA, 0xBB, 0xCC],
    ];
    let encoded = encode_to_vec(&original).expect("encode [[u8; 4]; 3]");
    // Inner arrays have no length prefix; outer array has no length prefix.
    // Total: 3 * 4 = 12 bytes.
    assert_eq!(
        encoded.len(),
        12,
        "[[u8; 4]; 3] must encode as exactly 12 bytes — no prefix at any level"
    );

    let (decoded, consumed): ([[u8; 4]; 3], usize) =
        decode_from_slice(&encoded).expect("decode [[u8; 4]; 3]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 12);
}

// ============================================================================
// Test 12 — [(u32, u32); 3] tuple-pair array (replaces String array)
// ============================================================================

#[test]
fn test_array_slice_adv2_tuple_pair_u32_3_roundtrip() {
    let original: [(u32, u32); 3] = [(0, 0), (1, u32::MAX), (100_000, 200_000)];
    let encoded = encode_to_vec(&original).expect("encode [(u32, u32); 3]");
    assert!(
        !encoded.is_empty(),
        "tuple-pair array must encode non-empty"
    );

    let (decoded, consumed): ([(u32, u32); 3], usize) =
        decode_from_slice(&encoded).expect("decode [(u32, u32); 3]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ============================================================================
// Test 13 — [(u8, u16); 2] mixed-width tuple array roundtrip
// ============================================================================

#[test]
fn test_array_slice_adv2_tuple_u8_u16_2_roundtrip() {
    let original: [(u8, u16); 2] = [(0, 0), (255, u16::MAX)];
    let encoded = encode_to_vec(&original).expect("encode [(u8, u16); 2]");
    assert!(
        !encoded.is_empty(),
        "[(u8, u16); 2] encoded bytes must not be empty"
    );

    let (decoded, consumed): ([(u8, u16); 2], usize) =
        decode_from_slice(&encoded).expect("decode [(u8, u16); 2]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ============================================================================
// Test 14 — [Option<u32>; 3] option array roundtrip
// ============================================================================

#[test]
fn test_array_slice_adv2_option_u32_3_roundtrip() {
    let original: [Option<u32>; 3] = [Some(42), None, Some(u32::MAX)];
    let encoded = encode_to_vec(&original).expect("encode [Option<u32>; 3]");
    assert!(
        !encoded.is_empty(),
        "[Option<u32>; 3] must encode non-empty"
    );

    let (decoded, consumed): ([Option<u32>; 3], usize) =
        decode_from_slice(&encoded).expect("decode [Option<u32>; 3]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ============================================================================
// Test 15 — fixed-int config with [u32; 4]: 16 bytes exactly
// ============================================================================

#[test]
fn test_array_slice_adv2_u32_4_fixed_int_config_16_bytes() {
    let original: [u32; 4] = [1, 2, 3, 4];
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode [u32; 4] fixed-int");
    // With fixed-int encoding each u32 is 4 bytes; [u32; 4] = 16 bytes, no prefix.
    assert_eq!(
        encoded.len(),
        16,
        "[u32; 4] with fixed-int encoding must be exactly 16 bytes"
    );

    let (decoded, consumed): ([u32; 4], usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode [u32; 4] fixed-int");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 16);
}

// ============================================================================
// Test 16 — big-endian config with [u32; 4]: byte-order verification
// ============================================================================

#[test]
fn test_array_slice_adv2_u32_4_big_endian_byte_order() {
    let original: [u32; 4] = [0x0102_0304, 0xDEAD_BEEF, 0x0000_0000, 0xFFFF_FFFF];
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode [u32; 4] big-endian fixed");
    assert_eq!(
        encoded.len(),
        16,
        "[u32; 4] big-endian fixed must be exactly 16 bytes"
    );
    // First element 0x01020304 in big-endian:
    assert_eq!(
        &encoded[0..4],
        &[0x01, 0x02, 0x03, 0x04],
        "first element must appear in big-endian byte order"
    );
    // Second element 0xDEADBEEF in big-endian:
    assert_eq!(
        &encoded[4..8],
        &[0xDE, 0xAD, 0xBE, 0xEF],
        "second element must appear in big-endian byte order"
    );

    let (decoded, consumed): ([u32; 4], usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode [u32; 4] big-endian fixed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 16);
}

// ============================================================================
// Test 17 — wire size of [u8; N]: encoded length equals N (no varint prefix)
// ============================================================================

#[test]
fn test_array_slice_adv2_wire_size_u8_n_equals_n() {
    // Demonstrate the invariant for several N values.
    let arr4: [u8; 4] = [1, 2, 3, 4];
    let arr8: [u8; 8] = [0xAA; 8];
    let arr64: [u8; 64] = [0x5A; 64];

    let enc4 = encode_to_vec(&arr4).expect("encode [u8; 4]");
    let enc8 = encode_to_vec(&arr8).expect("encode [u8; 8]");
    let enc64 = encode_to_vec(&arr64).expect("encode [u8; 64]");

    assert_eq!(enc4.len(), 4, "[u8; 4] wire size must equal 4");
    assert_eq!(enc8.len(), 8, "[u8; 8] wire size must equal 8");
    assert_eq!(enc64.len(), 64, "[u8; 64] wire size must equal 64");

    // Cross-check: a Vec<u8> of the same length carries a varint prefix so is larger.
    let vec64 = encode_to_vec(&arr64.to_vec()).expect("encode Vec<u8> 64");
    assert!(
        vec64.len() > 64,
        "Vec<u8> must be larger than [u8; 64] due to length prefix"
    );
}

// ============================================================================
// Test 18 — [u8; 32] roundtrip: consumed == encoded.len()
// ============================================================================

#[test]
fn test_array_slice_adv2_u8_32_consumed_equals_encoded_len() {
    let original: [u8; 32] = [
        0xFA, 0xCE, 0xB0, 0x0C, 0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD,
        0xEF, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0,
        0xF0, 0xFF,
    ];
    let encoded = encode_to_vec(&original).expect("encode [u8; 32]");
    assert_eq!(encoded.len(), 32, "[u8; 32] must encode as 32 bytes");

    let (decoded, consumed): ([u8; 32], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 32]");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal the full encoded length"
    );
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 19 — [i32; 4] with negative boundary values (i32::MIN, i32::MAX)
// ============================================================================

#[test]
fn test_array_slice_adv2_i32_4_boundary_values_roundtrip() {
    let original: [i32; 4] = [i32::MIN, -1, 0, i32::MAX];
    let encoded = encode_to_vec(&original).expect("encode [i32; 4] boundaries");
    assert!(
        !encoded.is_empty(),
        "boundary i32 array must encode non-empty"
    );

    let (decoded, consumed): ([i32; 4], usize) =
        decode_from_slice(&encoded).expect("decode [i32; 4] boundaries");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // Boundary values must survive the full encode→decode cycle.
    assert_eq!(decoded[0], i32::MIN);
    assert_eq!(decoded[3], i32::MAX);
}

// ============================================================================
// Test 20 — [[u32; 2]; 2] nested 2D array roundtrip
// ============================================================================

#[test]
fn test_array_slice_adv2_nested_u32_2x2_roundtrip() {
    let original: [[u32; 2]; 2] = [[0, u32::MAX], [1000, 2000]];
    let encoded = encode_to_vec(&original).expect("encode [[u32; 2]; 2]");
    assert!(!encoded.is_empty(), "[[u32; 2]; 2] must encode non-empty");

    let (decoded, consumed): ([[u32; 2]; 2], usize) =
        decode_from_slice(&encoded).expect("decode [[u32; 2]; 2]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ============================================================================
// Test 21 — [u8; 8] verify exact wire-byte pattern
// ============================================================================

#[test]
fn test_array_slice_adv2_u8_8_exact_wire_bytes() {
    let original: [u8; 8] = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
    let encoded = encode_to_vec(&original).expect("encode [u8; 8] pattern");
    // u8 arrays encode verbatim — no transformation, no prefix.
    assert_eq!(
        encoded.as_slice(),
        &[0x00u8, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77],
        "[u8; 8] wire bytes must exactly match the source array"
    );

    let (decoded, consumed): ([u8; 8], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 8] pattern");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 8);
}

// ============================================================================
// Test 22 — Struct containing a fixed-array field (derived Encode/Decode)
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct HashDigest {
    algorithm_id: u8,
    digest: [u8; 32],
    truncated: bool,
}

#[test]
fn test_array_slice_adv2_struct_with_fixed_array_field() {
    let original = HashDigest {
        algorithm_id: 3,
        digest: [
            0xBA, 0xAD, 0xF0, 0x0D, 0xBA, 0xAD, 0xF0, 0x0D, 0xBA, 0xAD, 0xF0, 0x0D, 0xBA, 0xAD,
            0xF0, 0x0D, 0xBA, 0xAD, 0xF0, 0x0D, 0xBA, 0xAD, 0xF0, 0x0D, 0xBA, 0xAD, 0xF0, 0x0D,
            0xBA, 0xAD, 0xF0, 0x0D,
        ],
        truncated: false,
    };
    let encoded = encode_to_vec(&original).expect("encode HashDigest");
    // Layout: algorithm_id (1 byte) + digest ([u8; 32] = 32 bytes) + truncated (1 byte) = 34.
    assert_eq!(
        encoded.len(),
        34,
        "HashDigest must encode as exactly 34 bytes"
    );
    // Verify the digest starts at byte offset 1 (after algorithm_id).
    assert_eq!(
        &encoded[1..33],
        &original.digest,
        "digest field must appear verbatim at offset 1 in the encoding"
    );

    let (decoded, consumed): (HashDigest, usize) =
        decode_from_slice(&encoded).expect("decode HashDigest");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 34);
}
