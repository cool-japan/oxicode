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
#[test]
fn test_encode_iter_to_vec() {
    let data = [10u32, 20, 30, 40, 50];
    let enc = oxicode::encode_iter_to_vec(data.iter().copied()).expect("encode");
    // Must be identical to encoding a Vec
    let vec_enc = oxicode::encode_to_vec(&data.to_vec()).expect("encode vec");
    assert_eq!(enc, vec_enc);
}

#[test]
fn test_decode_iter_from_slice_basic() {
    let data = vec![1u32, 2, 3, 4, 5];
    let enc = oxicode::encode_to_vec(&data).expect("encode");
    let items: Vec<u32> = oxicode::decode_iter_from_slice::<u32>(&enc)
        .expect("iter")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode all");
    assert_eq!(items, data);
}

#[test]
fn test_decode_iter_lazy_sum() {
    let data: Vec<u64> = (1..=100).collect();
    let enc = oxicode::encode_to_vec(&data).expect("encode");
    let sum: u64 = oxicode::decode_iter_from_slice::<u64>(&enc)
        .expect("iter")
        .filter_map(|r| r.ok())
        .sum();
    assert_eq!(sum, 5050);
}

#[test]
fn test_decode_iter_partial() {
    // Decode only first 3 items from a 10-item sequence
    let data = vec![1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let enc = oxicode::encode_to_vec(&data).expect("encode");
    let first_three: Vec<u32> = oxicode::decode_iter_from_slice::<u32>(&enc)
        .expect("iter")
        .take(3)
        .collect::<Result<Vec<_>, _>>()
        .expect("decode");
    assert_eq!(first_three, vec![1, 2, 3]);
}

#[test]
fn test_encode_iter_to_vec_with_config() {
    let data = [1u32, 2, 3];
    let config = oxicode::config::standard().with_fixed_int_encoding();
    let enc =
        oxicode::encode_iter_to_vec_with_config(data.iter().copied(), config).expect("encode");
    let vec_enc = oxicode::encode_to_vec_with_config(&data.to_vec(), config).expect("encode vec");
    assert_eq!(enc, vec_enc);
}

#[test]
fn test_decode_iter_empty_sequence() {
    let data: Vec<u32> = vec![];
    let enc = oxicode::encode_to_vec(&data).expect("encode");
    let items: Vec<u32> = oxicode::decode_iter_from_slice::<u32>(&enc)
        .expect("iter")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode all");
    assert_eq!(items, data);
}

#[test]
fn test_encode_seq_to_vec_range() {
    // True streaming encode — no intermediate Vec
    let enc = oxicode::encode_seq_to_vec(0u32..5).expect("encode");
    let (dec, _): (Vec<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_encode_seq_wire_identical_to_vec() {
    // Must be byte-for-byte identical to encoding a Vec
    let data = vec![10u32, 20, 30];
    let vec_enc = oxicode::encode_to_vec(&data).expect("encode vec");
    let seq_enc = oxicode::encode_seq_to_vec(data.iter().copied()).expect("encode seq");
    assert_eq!(
        vec_enc, seq_enc,
        "ExactSizeIterator encoding must match Vec encoding"
    );
}

#[test]
fn test_encode_seq_into_slice() {
    let mut buf = [0u8; 64];
    // Use a slice iterator, which always implements ExactSizeIterator
    let items = [1u32, 2, 3];
    let n = oxicode::encode_seq_into_slice(items.iter().copied(), &mut buf).expect("encode");
    let (dec, _): (Vec<u32>, _) = oxicode::decode_from_slice(&buf[..n]).expect("decode");
    assert_eq!(dec, vec![1, 2, 3]);
}

#[test]
fn test_encode_seq_empty() {
    let enc = oxicode::encode_seq_to_vec(core::iter::empty::<u32>()).expect("encode");
    let (dec, _): (Vec<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert!(dec.is_empty());
}

#[test]
fn test_encode_iter_to_vec_basic_u32() {
    let items = vec![1u32, 2u32, 3u32, 4u32, 5u32];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode_iter");
    let dec: Vec<u32> = oxicode::decode_iter_from_slice::<u32>(&enc)
        .expect("iter")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode iter");
    assert_eq!(items, dec);
}

#[test]
fn test_encode_iter_empty_u32() {
    let items: Vec<u32> = vec![];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode_iter");
    let dec: Vec<u32> = oxicode::decode_iter_from_slice::<u32>(&enc)
        .expect("iter")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode iter");
    assert_eq!(items, dec);
}

#[test]
fn test_encode_iter_strings() {
    let items = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];
    let enc = oxicode::encode_iter_to_vec(items.iter().cloned()).expect("encode_iter");
    let dec: Vec<String> = oxicode::decode_iter_from_slice::<String>(&enc)
        .expect("iter")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode iter");
    assert_eq!(items, dec);
}

#[test]
fn test_encode_iter_large_sequence() {
    let items: Vec<u64> = (0u64..1000).collect();
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode_iter");
    let dec: Vec<u64> = oxicode::decode_iter_from_slice::<u64>(&enc)
        .expect("iter")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode iter");
    assert_eq!(items, dec);
}
