//! Tests for zero-copy BorrowDecode

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
use oxicode::config;

#[test]
fn test_borrow_decode_str() {
    let original = "Hello, OxiCode! 🦀";

    // Encode to bytes (&str implements Encode)
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");

    // Borrow decode (zero-copy)
    let (decoded, _): (&str, _) =
        oxicode::borrow_decode_from_slice(&bytes).expect("Failed to decode");

    assert_eq!(original, decoded);

    // Verify it's actually borrowing from the input
    // Length is encoded as varint - for "Hello, OxiCode! 🦀" (21 bytes), varint uses 1 byte
    assert_eq!(decoded.as_ptr(), unsafe { bytes.as_ptr().add(1) });
}

#[test]
fn test_borrow_decode_bytes() {
    let original: &[u8] = &[1, 2, 3, 4, 5, 255, 0, 128];

    // Encode to bytes (&[u8] implements Encode)
    let encoded = oxicode::encode_to_vec(&original).expect("Failed to encode");

    // Borrow decode (zero-copy)
    let (decoded, _): (&[u8], _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(original, decoded);

    // Verify it's actually borrowing (8 bytes because length=8, varint uses 1 byte)
    assert_eq!(decoded.as_ptr(), unsafe { encoded.as_ptr().add(1) });
}

#[test]
fn test_borrow_decode_str_with_config() {
    let original = "Test with legacy config";

    let config = config::legacy();
    let bytes = oxicode::encode_to_vec_with_config(&original, config).expect("Failed to encode");

    let (decoded, _): (&str, _) =
        oxicode::borrow_decode_from_slice_with_config(&bytes, config).expect("Failed to decode");

    assert_eq!(original, decoded);
}

#[test]
fn test_borrow_decode_empty_str() {
    let original = "";

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (&str, _) =
        oxicode::borrow_decode_from_slice(&bytes).expect("Failed to decode");

    assert_eq!(original, decoded);
}

#[test]
fn test_borrow_decode_empty_bytes() {
    let original: &[u8] = &[];

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (&[u8], _) =
        oxicode::borrow_decode_from_slice(&bytes).expect("Failed to decode");

    assert_eq!(original, decoded);
}

#[test]
fn test_borrow_decode_unicode() {
    let original = "こんにちは世界 🌍🚀✨";

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (&str, _) =
        oxicode::borrow_decode_from_slice(&bytes).expect("Failed to decode");

    assert_eq!(original, decoded);
}

#[test]
fn test_borrow_decode_invalid_utf8() {
    // Manually create invalid UTF-8 using standard config (varint)
    let invalid_bytes: Vec<u8> = {
        let mut buf = Vec::new();
        // Encode length as varint: 4 bytes = 0x04 (single byte)
        buf.push(4);
        // Invalid UTF-8 sequence
        buf.extend_from_slice(&[0xFF, 0xFE, 0xFD, 0xFC]);
        buf
    };

    let result = oxicode::borrow_decode_from_slice::<&str>(&invalid_bytes);
    assert!(result.is_err(), "Expected UTF-8 validation error");
}
