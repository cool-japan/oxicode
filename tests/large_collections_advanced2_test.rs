//! Advanced tests for large collection encoding in OxiCode.
//!
//! Covers 22 scenarios exercising large Vec encoding, fixed-int configs,
//! big-endian configs, varint boundaries, partial-decode failures, and more.

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
// 1. Vec<u8> with 1000 elements roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_1000_roundtrip() {
    let original: Vec<u8> = (0u16..1000).map(|i| (i % 256) as u8).collect();
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<u8> with 1000 elements");
    let (decoded, consumed): (Vec<u8>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<u8> with 1000 elements");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 2. Vec<u32> with 1000 elements roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u32_1000_roundtrip() {
    let original: Vec<u32> = (0u32..1000).map(|i| i * 3 + 7).collect();
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<u32> with 1000 elements");
    let (decoded, consumed): (Vec<u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<u32> with 1000 elements");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 3. Vec<u64> with 500 elements roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u64_500_roundtrip() {
    let original: Vec<u64> = (0u64..500).map(|i| i * i + 1).collect();
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<u64> with 500 elements");
    let (decoded, consumed): (Vec<u64>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<u64> with 500 elements");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 4. Vec<String> with 100 elements roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_string_100_roundtrip() {
    let original: Vec<String> = (0u32..100).map(|i| format!("item_{:04}", i)).collect();
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<String> with 100 elements");
    let (decoded, consumed): (Vec<String>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<String> with 100 elements");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 5. Vec<u8> with 10000 elements roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_10000_roundtrip() {
    let original: Vec<u8> = (0u32..10000).map(|i| (i % 256) as u8).collect();
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<u8> with 10000 elements");
    let (decoded, consumed): (Vec<u8>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<u8> with 10000 elements");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 6. Vec<u8> consumed equals encoded length for 1000 elements
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_consumed_equals_encoded_len() {
    let original: Vec<u8> = (0u16..1000).map(|i| (i % 256) as u8).collect();
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<u8> for consumed check");
    let (_decoded, consumed): (Vec<u8>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<u8> for consumed check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed byte count must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// 7. Vec<u32> with fixed-int config (1000 elements)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u32_fixed_int_config_1000() {
    let original: Vec<u32> = (0u32..1000).collect();
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec_with_config(&original, cfg).expect("Failed to encode Vec<u32> fixed-int");
    let (decoded, consumed): (Vec<u32>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("Failed to decode Vec<u32> fixed-int");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 8. Vec<u64> with fixed-int config (500 elements) — verify 8 bytes per element
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u64_fixed_int_8_bytes_per_element() {
    let original: Vec<u64> = (0u64..500).collect();
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec_with_config(&original, cfg).expect("Failed to encode Vec<u64> fixed-int");
    let (decoded, consumed): (Vec<u64>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("Failed to decode Vec<u64> fixed-int");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
    // Each u64 is exactly 8 bytes with fixed-int encoding; plus varint length prefix.
    // The data portion must be at least 500 * 8 bytes.
    assert!(
        bytes.len() >= 500 * 8,
        "fixed-int Vec<u64> must be at least 500*8 bytes, got {}",
        bytes.len()
    );
}

// ---------------------------------------------------------------------------
// 9. Vec<bool> with 1000 elements roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_bool_1000_roundtrip() {
    let original: Vec<bool> = (0u32..1000).map(|i| i % 2 == 0).collect();
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<bool> with 1000 elements");
    let (decoded, consumed): (Vec<bool>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<bool> with 1000 elements");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 10. Vec<i32> with negative values (1000 elements)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_i32_negative_1000_roundtrip() {
    let original: Vec<i32> = (-500i32..500).collect();
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<i32> with negative values");
    let (decoded, consumed): (Vec<i32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<i32> with negative values");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 11. Vec<u8> all-zeros (10000 elements) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_all_zeros_10000_roundtrip() {
    let original: Vec<u8> = vec![0u8; 10000];
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<u8> all-zeros");
    let (decoded, consumed): (Vec<u8>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<u8> all-zeros");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
    assert!(decoded.iter().all(|&b| b == 0), "all bytes must be zero");
}

// ---------------------------------------------------------------------------
// 12. Vec<u8> all-255 (10000 elements) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_all_255_10000_roundtrip() {
    let original: Vec<u8> = vec![255u8; 10000];
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<u8> all-255");
    let (decoded, consumed): (Vec<u8>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<u8> all-255");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
    assert!(decoded.iter().all(|&b| b == 255), "all bytes must be 255");
}

// ---------------------------------------------------------------------------
// 13. Nested Vec<Vec<u8>> with 100 inner vecs
// ---------------------------------------------------------------------------
#[test]
fn test_nested_vec_vec_u8_100_roundtrip() {
    let original: Vec<Vec<u8>> = (0u8..100).map(|i| (0u8..i).collect::<Vec<u8>>()).collect();
    let bytes =
        encode_to_vec(&original).expect("Failed to encode nested Vec<Vec<u8>> with 100 inner vecs");
    let (decoded, consumed): (Vec<Vec<u8>>, usize) = decode_from_slice(&bytes)
        .expect("Failed to decode nested Vec<Vec<u8>> with 100 inner vecs");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 14. Vec<u32> deterministic values roundtrip (use 0..1000u32)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u32_deterministic_0_to_1000_roundtrip() {
    let original: Vec<u32> = (0u32..1000).collect();
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<u32> deterministic 0..1000");
    let (decoded, consumed): (Vec<u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<u32> deterministic 0..1000");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
    for (idx, (&orig, &dec)) in original.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(orig, dec, "element at index {} must match", idx);
    }
}

// ---------------------------------------------------------------------------
// 15. Large String (10000 chars) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_large_string_10000_chars_roundtrip() {
    let original: String = "x".repeat(10000);
    let bytes = encode_to_vec(&original).expect("Failed to encode large String with 10000 chars");
    let (decoded, consumed): (String, usize) =
        decode_from_slice(&bytes).expect("Failed to decode large String with 10000 chars");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 10000);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 16. Vec<u64> with u64::MAX values roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u64_max_values_roundtrip() {
    let original: Vec<u64> = vec![u64::MAX; 100];
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<u64> with u64::MAX values");
    let (decoded, consumed): (Vec<u64>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<u64> with u64::MAX values");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
    assert!(
        decoded.iter().all(|&v| v == u64::MAX),
        "all values must be u64::MAX"
    );
}

// ---------------------------------------------------------------------------
// 17. Vec<u32> encoding grows monotonically with size
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u32_encoding_grows_monotonically() {
    let sizes = [10usize, 100, 500, 1000];
    let mut prev_len = 0usize;
    for &n in &sizes {
        let v: Vec<u32> = (0u32..n as u32).collect();
        let bytes = encode_to_vec(&v).expect("Failed to encode Vec<u32> for size growth test");
        assert!(
            bytes.len() > prev_len,
            "encoded length must grow: size {} produced {} bytes, prev was {}",
            n,
            bytes.len(),
            prev_len
        );
        prev_len = bytes.len();
    }
}

// ---------------------------------------------------------------------------
// 18. Vec<u8> with varint boundary sizes (255 and 256 elements)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_varint_boundary_255_and_256() {
    // 255-element vec
    let original_255: Vec<u8> = (0u8..255).collect();
    let bytes_255 =
        encode_to_vec(&original_255).expect("Failed to encode Vec<u8> with 255 elements");
    let (decoded_255, consumed_255): (Vec<u8>, usize) =
        decode_from_slice(&bytes_255).expect("Failed to decode Vec<u8> with 255 elements");
    assert_eq!(original_255, decoded_255);
    assert_eq!(consumed_255, bytes_255.len());

    // 256-element vec
    let original_256: Vec<u8> = (0u16..256).map(|i| i as u8).collect();
    let bytes_256 =
        encode_to_vec(&original_256).expect("Failed to encode Vec<u8> with 256 elements");
    let (decoded_256, consumed_256): (Vec<u8>, usize) =
        decode_from_slice(&bytes_256).expect("Failed to decode Vec<u8> with 256 elements");
    assert_eq!(original_256, decoded_256);
    assert_eq!(consumed_256, bytes_256.len());

    assert_eq!(decoded_255.len(), 255);
    assert_eq!(decoded_256.len(), 256);
}

// ---------------------------------------------------------------------------
// 19. Large Vec<u8> with big-endian config
// ---------------------------------------------------------------------------
#[test]
fn test_large_vec_u8_big_endian_config() {
    let original: Vec<u8> = (0u16..1000).map(|i| (i % 256) as u8).collect();
    let cfg = config::standard().with_big_endian();
    let bytes =
        encode_to_vec_with_config(&original, cfg).expect("Failed to encode Vec<u8> big-endian");
    let (decoded, consumed): (Vec<u8>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("Failed to decode Vec<u8> big-endian");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 20. Vec<i64> with i64::MIN values roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_i64_min_values_roundtrip() {
    let original: Vec<i64> = vec![i64::MIN; 100];
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<i64> with i64::MIN values");
    let (decoded, consumed): (Vec<i64>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<i64> with i64::MIN values");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
    assert!(
        decoded.iter().all(|&v| v == i64::MIN),
        "all values must be i64::MIN"
    );
}

// ---------------------------------------------------------------------------
// 21. Vec<f64> with 100 exact values roundtrip (whole numbers like 1.0, 2.0, …)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_f64_exact_values_roundtrip() {
    let original: Vec<f64> = (1u32..=100).map(f64::from).collect();
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<f64> with exact values");
    let (decoded, consumed): (Vec<f64>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<f64> with exact values");
    assert_eq!(original.len(), decoded.len());
    assert_eq!(consumed, bytes.len());
    for (idx, (&orig, &dec)) in original.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(
            orig.to_bits(),
            dec.to_bits(),
            "f64 bit pattern must match at index {}",
            idx
        );
    }
}

// ---------------------------------------------------------------------------
// 22. Vec<u8> encode then decode partial (first part only) — must fail
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_decode_truncated_fails() {
    // Encode a vec of 200 elements, then try to decode from only the first 100 bytes.
    // This should fail because the data is truncated.
    let original: Vec<u8> = (0u8..200).collect();
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<u8> with 200 elements");

    // Only use the first 100 bytes, which cannot represent the full 200-element vec.
    let truncated = &bytes[..100];
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice(truncated);
    assert!(
        result.is_err(),
        "decoding truncated bytes must return an error, not silently succeed"
    );
}
