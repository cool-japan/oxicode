//! Stress tests for correctness at scale

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

#[test]
fn test_encode_decode_10k_strings() {
    let strings: Vec<String> = (0..10_000).map(|i| format!("string_{:05}", i)).collect();
    let enc = encode_to_vec(&strings).expect("encode");
    let (dec, _): (Vec<String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(strings, dec);
}

#[test]
fn test_encode_decode_nested_vecs() {
    // 100x100 nested Vec
    let data: Vec<Vec<u32>> = (0..100)
        .map(|i| (0u32..100).map(|j| i * 100 + j).collect())
        .collect();
    let enc = encode_to_vec(&data).expect("encode");
    let (dec, _): (Vec<Vec<u32>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(data, dec);
}

#[test]
fn test_encode_1mb_bytes() {
    let data: Vec<u8> = (0..1_000_000).map(|i| (i % 256) as u8).collect();
    let enc = encode_to_vec(&data).expect("encode");
    let (dec, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(data, dec);
}

#[test]
fn test_deep_option_nesting() {
    type DeepOpt = Option<Option<Option<Option<Option<u32>>>>>;
    let value: DeepOpt = Some(Some(Some(Some(Some(42)))));
    let enc = encode_to_vec(&value).expect("encode");
    let (dec, _): (DeepOpt, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(value, dec);
}

#[test]
fn test_encode_decode_btreemap_1k_entries() {
    use std::collections::BTreeMap;
    let map: BTreeMap<u32, String> = (0..1000).map(|i| (i, format!("value_{}", i))).collect();
    let enc = encode_to_vec(&map).expect("encode");
    let (dec, _): (BTreeMap<u32, String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(map, dec);
}

#[test]
fn test_consecutive_encode_decode() {
    // Encode multiple values consecutively and decode them back
    let mut buf = Vec::new();
    let values: Vec<u64> = (0..1000).collect();

    for &v in &values {
        let enc = encode_to_vec(&v).expect("encode");
        buf.extend_from_slice(&enc);
    }

    // Decode them back sequentially
    let mut offset = 0;
    for &expected in &values {
        let (decoded, consumed): (u64, _) = decode_from_slice(&buf[offset..]).expect("decode");
        assert_eq!(decoded, expected);
        offset += consumed;
    }
    assert_eq!(offset, buf.len());
}

#[test]
fn test_encode_decode_all_ascii_chars() {
    let chars: Vec<char> = (0x20u32..=0x7E)
        .map(|c| char::from_u32(c).expect("valid ASCII codepoint"))
        .collect();
    let enc = encode_to_vec(&chars).expect("encode");
    let (dec, _): (Vec<char>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(chars, dec);
}
