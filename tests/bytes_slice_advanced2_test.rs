//! Advanced byte slice and byte array encoding/decoding tests (set 2).

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

// ─── Test 1: [u8; 1] single byte roundtrip ───────────────────────────────────

#[test]
fn test_fixed_array_u8_1_roundtrip() {
    let original: [u8; 1] = [0xAB];
    let enc = encode_to_vec(&original).expect("encode [u8; 1]");
    let (val, _): ([u8; 1], _) = decode_from_slice(&enc).expect("decode [u8; 1]");
    assert_eq!(original, val);
}

// ─── Test 2: [u8; 4] fixed array roundtrip ───────────────────────────────────

#[test]
fn test_fixed_array_u8_4_roundtrip() {
    let original: [u8; 4] = [1, 2, 3, 4];
    let enc = encode_to_vec(&original).expect("encode [u8; 4]");
    let (val, _): ([u8; 4], _) = decode_from_slice(&enc).expect("decode [u8; 4]");
    assert_eq!(original, val);
}

// ─── Test 3: [u8; 8] fixed array roundtrip ───────────────────────────────────

#[test]
fn test_fixed_array_u8_8_roundtrip() {
    let original: [u8; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
    let enc = encode_to_vec(&original).expect("encode [u8; 8]");
    let (val, _): ([u8; 8], _) = decode_from_slice(&enc).expect("decode [u8; 8]");
    assert_eq!(original, val);
}

// ─── Test 4: [u8; 16] fixed array roundtrip ──────────────────────────────────

#[test]
fn test_fixed_array_u8_16_roundtrip() {
    let original: [u8; 16] = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let enc = encode_to_vec(&original).expect("encode [u8; 16]");
    let (val, _): ([u8; 16], _) = decode_from_slice(&enc).expect("decode [u8; 16]");
    assert_eq!(original, val);
}

// ─── Test 5: [u8; 32] fixed array roundtrip ──────────────────────────────────

#[test]
fn test_fixed_array_u8_32_roundtrip() {
    let original: [u8; 32] = {
        let mut arr = [0u8; 32];
        for (i, b) in arr.iter_mut().enumerate() {
            *b = i as u8;
        }
        arr
    };
    let enc = encode_to_vec(&original).expect("encode [u8; 32]");
    let (val, _): ([u8; 32], _) = decode_from_slice(&enc).expect("decode [u8; 32]");
    assert_eq!(original, val);
}

// ─── Test 6: [u8; 64] fixed array roundtrip ──────────────────────────────────

#[test]
fn test_fixed_array_u8_64_roundtrip() {
    let original: [u8; 64] = {
        let mut arr = [0u8; 64];
        for (i, b) in arr.iter_mut().enumerate() {
            *b = (i * 2 % 256) as u8;
        }
        arr
    };
    let enc = encode_to_vec(&original).expect("encode [u8; 64]");
    let (val, _): ([u8; 64], _) = decode_from_slice(&enc).expect("decode [u8; 64]");
    assert_eq!(original, val);
}

// ─── Test 7: Vec<u8> empty roundtrip ─────────────────────────────────────────

#[test]
fn test_vec_u8_empty_roundtrip() {
    let original: Vec<u8> = Vec::new();
    let enc = encode_to_vec(&original).expect("encode empty Vec<u8>");
    let (val, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode empty Vec<u8>");
    assert_eq!(original, val);
}

// ─── Test 8: Vec<u8> with 1 byte roundtrip ───────────────────────────────────

#[test]
fn test_vec_u8_single_byte_roundtrip() {
    let original: Vec<u8> = vec![0x42];
    let enc = encode_to_vec(&original).expect("encode Vec<u8> 1 byte");
    let (val, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode Vec<u8> 1 byte");
    assert_eq!(original, val);
}

// ─── Test 9: Vec<u8> with 255 bytes (0..255) roundtrip ───────────────────────

#[test]
fn test_vec_u8_255_bytes_roundtrip() {
    let original: Vec<u8> = (0u8..255).collect();
    let enc = encode_to_vec(&original).expect("encode Vec<u8> 0..255");
    let (val, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode Vec<u8> 0..255");
    assert_eq!(original, val);
}

// ─── Test 10: Vec<u8> with 1000 bytes roundtrip ──────────────────────────────

#[test]
fn test_vec_u8_1000_bytes_roundtrip() {
    let original: Vec<u8> = (0u32..1000).map(|i| (i % 256) as u8).collect();
    let enc = encode_to_vec(&original).expect("encode Vec<u8> 1000 bytes");
    let (val, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode Vec<u8> 1000 bytes");
    assert_eq!(original, val);
}

// ─── Test 11: Vec<u8> all zeros (256 bytes) roundtrip ────────────────────────

#[test]
fn test_vec_u8_all_zeros_roundtrip() {
    let original: Vec<u8> = vec![0u8; 256];
    let enc = encode_to_vec(&original).expect("encode Vec<u8> all zeros");
    let (val, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode Vec<u8> all zeros");
    assert_eq!(original, val);
}

// ─── Test 12: Vec<u8> all 0xFF (256 bytes) roundtrip ─────────────────────────

#[test]
fn test_vec_u8_all_ff_roundtrip() {
    let original: Vec<u8> = vec![0xFFu8; 256];
    let enc = encode_to_vec(&original).expect("encode Vec<u8> all 0xFF");
    let (val, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode Vec<u8> all 0xFF");
    assert_eq!(original, val);
}

// ─── Test 13: Fixed-int config with [u8; 4] ──────────────────────────────────

#[test]
fn test_fixed_array_u8_4_with_fixed_int_config() {
    let original: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode [u8; 4] fixed-int");
    let (val, _): ([u8; 4], _) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u8; 4] fixed-int");
    assert_eq!(original, val);
}

// ─── Test 14: Big-endian config with [u8; 4] ─────────────────────────────────

#[test]
fn test_fixed_array_u8_4_with_big_endian_config() {
    let original: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode [u8; 4] big-endian");
    let (val, _): ([u8; 4], _) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u8; 4] big-endian");
    assert_eq!(original, val);
}

// ─── Test 15: Wire size of [u8; 4] is exactly 4 bytes ────────────────────────
//
// [u8; N] fixed arrays encode as exactly N bytes with NO length prefix.

#[test]
fn test_fixed_array_u8_4_wire_size_is_exactly_4() {
    let original: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let enc = encode_to_vec(&original).expect("encode [u8; 4] wire size");
    assert_eq!(
        enc.len(),
        4,
        "expected [u8; 4] to encode as exactly 4 bytes, got {}",
        enc.len()
    );
}

// ─── Test 16: Wire size of Vec<u8> with 4 elements includes varint length prefix

#[test]
fn test_vec_u8_4_elements_wire_size_includes_length_prefix() {
    let original: Vec<u8> = vec![0x11, 0x22, 0x33, 0x44];
    let enc = encode_to_vec(&original).expect("encode Vec<u8> 4 elements wire size");
    // varint(4) = 1 byte (values < 128 encode as 1 byte in standard varint) + 4 data bytes = 5
    assert!(
        enc.len() > 4,
        "expected Vec<u8> with 4 elements to be larger than 4 bytes (needs length prefix), got {}",
        enc.len()
    );
    // The length prefix occupies at least 1 byte, total must be at least 5
    assert_eq!(
        enc.len(),
        5,
        "expected Vec<u8>(4 elements) to encode as 5 bytes (1 varint + 4 data), got {}",
        enc.len()
    );
}

// ─── Test 17: [u8; 0] zero-length fixed array roundtrip ──────────────────────

#[test]
fn test_fixed_array_u8_0_roundtrip() {
    let original: [u8; 0] = [];
    let enc = encode_to_vec(&original).expect("encode [u8; 0]");
    let (val, _): ([u8; 0], _) = decode_from_slice(&enc).expect("decode [u8; 0]");
    assert_eq!(original, val);
    assert_eq!(enc.len(), 0, "[u8; 0] must encode to 0 bytes");
}

// ─── Test 18: Struct containing [u8; 8] field roundtrip ──────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithFixedArray {
    id: u32,
    data: [u8; 8],
}

#[test]
fn test_struct_with_fixed_array_field_roundtrip() {
    let original = WithFixedArray {
        id: 42,
        data: [1, 2, 3, 4, 5, 6, 7, 8],
    };
    let enc = encode_to_vec(&original).expect("encode WithFixedArray");
    let (val, _): (WithFixedArray, _) = decode_from_slice(&enc).expect("decode WithFixedArray");
    assert_eq!(original, val);
}

// ─── Test 19: Struct containing Vec<u8> field roundtrip ──────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithVecBytes {
    tag: u8,
    payload: Vec<u8>,
}

#[test]
fn test_struct_with_vec_u8_field_roundtrip() {
    let original = WithVecBytes {
        tag: 0xFF,
        payload: vec![10, 20, 30, 40, 50],
    };
    let enc = encode_to_vec(&original).expect("encode WithVecBytes");
    let (val, _): (WithVecBytes, _) = decode_from_slice(&enc).expect("decode WithVecBytes");
    assert_eq!(original, val);
}

// ─── Test 20: Vec<[u8; 4]> roundtrip ─────────────────────────────────────────

#[test]
fn test_vec_of_fixed_arrays_roundtrip() {
    let original: Vec<[u8; 4]> = vec![
        [0x01, 0x02, 0x03, 0x04],
        [0xAA, 0xBB, 0xCC, 0xDD],
        [0x10, 0x20, 0x30, 0x40],
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<[u8; 4]>");
    let (val, _): (Vec<[u8; 4]>, _) = decode_from_slice(&enc).expect("decode Vec<[u8; 4]>");
    assert_eq!(original, val);
}

// ─── Test 21: decode_from_slice on partial bytes returns error ────────────────

#[test]
fn test_decode_from_partial_bytes_returns_error() {
    // Encode a [u8; 8] — requires exactly 8 bytes. Provide only 3 bytes.
    let partial: &[u8] = &[0x01, 0x02, 0x03];
    let result: oxicode::Result<([u8; 8], usize)> = decode_from_slice(partial);
    assert!(
        result.is_err(),
        "expected error when decoding [u8; 8] from only 3 bytes"
    );
}

// ─── Test 22: [u8; 32] all bytes preserved correctly ─────────────────────────

#[test]
fn test_fixed_array_u8_32_all_bytes_preserved() {
    let original: [u8; 32] = {
        let mut arr = [0u8; 32];
        for (i, b) in arr.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(7).wrapping_add(13);
        }
        arr
    };
    let enc = encode_to_vec(&original).expect("encode [u8; 32] byte preservation");
    let (val, _): ([u8; 32], _) =
        decode_from_slice(&enc).expect("decode [u8; 32] byte preservation");
    for (i, (&expected, &actual)) in original.iter().zip(val.iter()).enumerate() {
        assert_eq!(
            expected, actual,
            "byte at index {} differs: expected {:#04x}, got {:#04x}",
            i, expected, actual
        );
    }
}
