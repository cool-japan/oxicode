//! Tests that verify the exact encoding sizes for various types.
//! These are important for protocol/format documentation.

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
use oxicode::encode_to_vec;

// Fixed-size types (always same byte count)
#[test]
fn test_f32_always_4_bytes() {
    let values = [0.0f32, 1.0, -1.0, f32::MAX, f32::MIN, f32::INFINITY];
    for v in values {
        let enc = encode_to_vec(&v).expect("encode");
        assert_eq!(enc.len(), 4, "f32 {} should be 4 bytes", v);
    }
}

#[test]
fn test_f64_always_8_bytes() {
    let values = [0.0f64, 1.0, -1.0, f64::MAX, f64::MIN, f64::INFINITY];
    for v in values {
        let enc = encode_to_vec(&v).expect("encode");
        assert_eq!(enc.len(), 8, "f64 {} should be 8 bytes", v);
    }
}

#[test]
fn test_bool_always_1_byte() {
    assert_eq!(encode_to_vec(&true).expect("encode").len(), 1);
    assert_eq!(encode_to_vec(&false).expect("encode").len(), 1);
}

// Varint sizes
#[test]
fn test_u64_varint_1_byte_range() {
    // oxicode varint: 0-250 all encode as 1 byte (SINGLE_BYTE_MAX = 250)
    for v in 0u64..=250 {
        let enc = encode_to_vec(&v).expect("encode");
        assert_eq!(enc.len(), 1, "u64 {} should encode to 1 byte", v);
    }
}

#[test]
fn test_u64_varint_3_byte_range() {
    // oxicode varint: 0-250 = 1 byte, 251-65535 = 3 bytes (marker + u16)
    for v in [251u64, 255, 1000, 16383, 65535] {
        let enc = encode_to_vec(&v).expect("encode");
        assert_eq!(enc.len(), 3, "u64 {} should encode to 3 bytes", v);
    }
}

#[test]
fn test_i64_varint_zigzag_sizes() {
    // With zigzag encoding: 0 -> 0, -1 -> 1, 1 -> 2, -2 -> 3, etc.
    // 0 should be 1 byte
    assert_eq!(encode_to_vec(&0i64).expect("encode").len(), 1);
    // Small negatives should also be small
    let neg1_enc = encode_to_vec(&(-1i64)).expect("encode");
    assert_eq!(neg1_enc.len(), 1, "i64 -1 should be 1 byte with zigzag");
}

// String sizes
#[test]
fn test_empty_string_size() {
    let enc = encode_to_vec(&"".to_string()).expect("encode");
    assert_eq!(enc.len(), 1, "empty string should be 1 byte (length = 0)");
}

#[test]
fn test_ascii_string_size() {
    let s = "hello"; // 5 ASCII bytes
    let enc = encode_to_vec(&s.to_string()).expect("encode");
    assert_eq!(
        enc.len(),
        1 + 5,
        "5-char ASCII string: 1 byte length + 5 bytes content"
    );
}

// Option sizes
#[test]
fn test_option_none_size() {
    let enc = encode_to_vec(&Option::<u32>::None).expect("encode");
    assert_eq!(enc.len(), 1, "None should be 1 byte");
}

#[test]
fn test_unit_type_zero_bytes() {
    let enc = encode_to_vec(&()).expect("encode");
    assert_eq!(enc.len(), 0, "() should encode to 0 bytes");
}

// Vec overhead
#[test]
fn test_vec_length_prefix_overhead() {
    let empty: Vec<u8> = vec![];
    let one: Vec<u8> = vec![42];

    let enc_empty = encode_to_vec(&empty).expect("encode");
    let enc_one = encode_to_vec(&one).expect("encode");

    // empty vec: just length byte (0)
    assert_eq!(enc_empty.len(), 1);
    // vec with one element: 1 (length) + 1 (value) = 2
    assert_eq!(enc_one.len(), 2);
}
