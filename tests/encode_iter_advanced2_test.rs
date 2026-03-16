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
use oxicode::{config, encode_to_vec, Decode, Encode};

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Wrapper {
    inner: Point2D,
    tag: String,
}

// 1. Encode iterator of u32 values
#[test]
fn test_iter_adv2_encode_u32_values() {
    let items: Vec<u32> = vec![10, 20, 30, 40, 50];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode iter u32");
    let dec: Vec<u32> = oxicode::decode_iter_from_slice::<u32>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode iter u32");
    assert_eq!(items, dec);
}

// 2. Encode iterator of String values
#[test]
fn test_iter_adv2_encode_string_values() {
    let items: Vec<String> = vec![
        "hello".to_string(),
        "world".to_string(),
        "oxicode".to_string(),
    ];
    let enc = oxicode::encode_iter_to_vec(items.iter().cloned()).expect("encode iter strings");
    let dec: Vec<String> = oxicode::decode_iter_from_slice::<String>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode iter strings");
    assert_eq!(items, dec);
}

// 3. Encode iterator of structs
#[test]
fn test_iter_adv2_encode_structs() {
    let items = vec![
        Point2D { x: 1, y: 2 },
        Point2D { x: -3, y: 4 },
        Point2D { x: 100, y: -200 },
    ];
    let enc = oxicode::encode_iter_to_vec(items.iter().cloned()).expect("encode iter structs");
    let dec: Vec<Point2D> = oxicode::decode_iter_from_slice::<Point2D>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode iter structs");
    assert_eq!(items, dec);
}

// 4. Empty iterator
#[test]
fn test_iter_adv2_empty_iterator() {
    let items: Vec<i64> = vec![];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode empty iter");
    let dec: Vec<i64> = oxicode::decode_iter_from_slice::<i64>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode empty iter");
    assert!(dec.is_empty());
}

// 5. Single-element iterator
#[test]
fn test_iter_adv2_single_element() {
    let items: Vec<u8> = vec![42];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode single elem");
    let dec: Vec<u8> = oxicode::decode_iter_from_slice::<u8>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode single elem");
    assert_eq!(dec, vec![42u8]);
}

// 6. Large iterator (100 items)
#[test]
fn test_iter_adv2_large_100_items() {
    let items: Vec<u32> = (0u32..100).collect();
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode 100 items");
    let dec: Vec<u32> = oxicode::decode_iter_from_slice::<u32>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode 100 items");
    assert_eq!(items, dec);
    assert_eq!(dec.len(), 100);
}

// 7. Decode iterator reads back all items
#[test]
fn test_iter_adv2_decode_reads_all_items() {
    let items: Vec<u64> = vec![100, 200, 300, 400, 500, 600, 700];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode all items");
    let mut count = 0usize;
    for result in oxicode::decode_iter_from_slice::<u64>(&enc).expect("decode iter setup") {
        let _ = result.expect("decode item");
        count += 1;
    }
    assert_eq!(count, items.len());
}

// 8. Decode iterator item count matches encode count
#[test]
fn test_iter_adv2_decode_count_matches() {
    let items: Vec<i32> = vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode count items");
    let decoded: Vec<i32> = oxicode::decode_iter_from_slice::<i32>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode count items");
    assert_eq!(decoded.len(), items.len());
}

// 9. Sequential decode after encode
#[test]
fn test_iter_adv2_sequential_decode() {
    let items: Vec<u32> = vec![7, 14, 21, 28, 35];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode sequential");
    let mut iter = oxicode::decode_iter_from_slice::<u32>(&enc).expect("decode iter setup");
    assert_eq!(iter.next().expect("item 0").expect("decode item 0"), 7);
    assert_eq!(iter.next().expect("item 1").expect("decode item 1"), 14);
    assert_eq!(iter.next().expect("item 2").expect("decode item 2"), 21);
    assert_eq!(iter.next().expect("item 3").expect("decode item 3"), 28);
    assert_eq!(iter.next().expect("item 4").expect("decode item 4"), 35);
    assert!(iter.next().is_none());
}

// 10. Mixed value iterator (i32 with negative and positive)
#[test]
fn test_iter_adv2_mixed_signed_values() {
    let items: Vec<i32> = vec![-1000, 0, 1, -1, 999, i32::MIN / 2, i32::MAX / 2];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode mixed values");
    let dec: Vec<i32> = oxicode::decode_iter_from_slice::<i32>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode mixed values");
    assert_eq!(items, dec);
}

// 11. Vec<u8> iterator
#[test]
fn test_iter_adv2_vec_u8_elements() {
    let items: Vec<Vec<u8>> = vec![
        vec![1, 2, 3],
        vec![],
        vec![255, 0, 128],
        vec![10, 20, 30, 40, 50],
    ];
    let enc = oxicode::encode_iter_to_vec(items.iter().cloned()).expect("encode vec u8 iter");
    let dec: Vec<Vec<u8>> = oxicode::decode_iter_from_slice::<Vec<u8>>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode vec u8 iter");
    assert_eq!(items, dec);
}

// 12. Iterator with fixed_int_encoding config
#[test]
fn test_iter_adv2_fixed_int_encoding_config() {
    let items: Vec<u32> = vec![1, 2, 3, 4, 5];
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = oxicode::encode_iter_to_vec_with_config(items.iter().copied(), cfg)
        .expect("encode fixed int config");
    let dec: Vec<u32> = oxicode::decode_iter_from_slice_with_config::<u32, _>(&enc, cfg)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode fixed int config");
    assert_eq!(items, dec);
}

// 13. Encode then decode round-trip via iter
#[test]
fn test_iter_adv2_round_trip() {
    let original: Vec<u64> = vec![u64::MAX, u64::MIN, 42, 0, 1_000_000];
    let enc =
        oxicode::encode_iter_to_vec(original.iter().copied()).expect("encode round trip iter");
    let restored: Vec<u64> = oxicode::decode_iter_from_slice::<u64>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode round trip iter");
    assert_eq!(original, restored);
}

// 14. Byte count from iter encode is non-zero for non-empty data
#[test]
fn test_iter_adv2_byte_count_non_empty() {
    let items: Vec<u32> = vec![1, 2, 3];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode byte count");
    assert!(
        !enc.is_empty(),
        "encoded bytes must be non-empty for non-empty input"
    );
    let empty_enc =
        oxicode::encode_iter_to_vec(core::iter::empty::<u32>()).expect("encode empty byte count");
    assert!(enc.len() > empty_enc.len());
}

// 15. Consecutive iterations over the same encoded data
#[test]
fn test_iter_adv2_consecutive_iterations() {
    let items: Vec<u32> = vec![11, 22, 33];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode consecutive");
    let first: Vec<u32> = oxicode::decode_iter_from_slice::<u32>(&enc)
        .expect("decode iter setup first")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode first pass");
    let second: Vec<u32> = oxicode::decode_iter_from_slice::<u32>(&enc)
        .expect("decode iter setup second")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode second pass");
    assert_eq!(first, second);
    assert_eq!(first, items);
}

// 16. Option values in iterator
#[test]
fn test_iter_adv2_option_values() {
    let items: Vec<Option<u32>> = vec![Some(1), None, Some(3), None, Some(5)];
    let enc = oxicode::encode_iter_to_vec(items.iter().cloned()).expect("encode option iter");
    let dec: Vec<Option<u32>> = oxicode::decode_iter_from_slice::<Option<u32>>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode option iter");
    assert_eq!(items, dec);
}

// 17. Enum values in iterator
#[test]
fn test_iter_adv2_enum_values() {
    let items: Vec<Color> = vec![
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Custom(255, 128, 0),
        Color::Red,
    ];
    let enc = oxicode::encode_iter_to_vec(items.iter().cloned()).expect("encode enum iter");
    let dec: Vec<Color> = oxicode::decode_iter_from_slice::<Color>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode enum iter");
    assert_eq!(items, dec);
}

// 18. Nested struct values in iterator
#[test]
fn test_iter_adv2_nested_struct_values() {
    let items: Vec<Wrapper> = vec![
        Wrapper {
            inner: Point2D { x: 1, y: 2 },
            tag: "first".to_string(),
        },
        Wrapper {
            inner: Point2D { x: -10, y: 20 },
            tag: "second".to_string(),
        },
    ];
    let enc = oxicode::encode_iter_to_vec(items.iter().cloned()).expect("encode nested structs");
    let dec: Vec<Wrapper> = oxicode::decode_iter_from_slice::<Wrapper>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode nested structs");
    assert_eq!(items, dec);
}

// 19. Iterator consume: all items decoded (no leftover)
#[test]
fn test_iter_adv2_consume_all_items() {
    let items: Vec<u8> = (0u8..20).collect();
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode consume all");
    let decoded: Vec<u8> = oxicode::decode_iter_from_slice::<u8>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode consume all");
    assert_eq!(decoded.len(), 20);
    assert_eq!(decoded, items);
}

// 20. Error on truncated iter data
#[test]
fn test_iter_adv2_error_on_truncated_data() {
    let items: Vec<u32> = vec![1, 2, 3, 4, 5];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode truncated");
    // Truncate to half the bytes — this should cause a decode error at some point
    let truncated = &enc[..enc.len() / 2];
    let results: Vec<Result<u32, _>> = oxicode::decode_iter_from_slice::<u32>(truncated)
        .expect("decode iter setup truncated")
        .collect();
    let has_error = results.iter().any(|r| r.is_err());
    // With truncated data, we expect either fewer items or at least one decode error
    assert!(
        has_error || results.len() < items.len(),
        "truncated data must produce fewer items or an error"
    );
}

// 21. Iterator with tuples
#[test]
fn test_iter_adv2_tuple_values() {
    let items: Vec<(u32, bool)> = vec![(1, true), (2, false), (3, true), (4, false)];
    let enc = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode tuple iter");
    let dec: Vec<(u32, bool)> = oxicode::decode_iter_from_slice::<(u32, bool)>(&enc)
        .expect("decode iter setup")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode tuple iter");
    assert_eq!(items, dec);
}

// 22. Iter encode matches encode_to_vec for same data
#[test]
fn test_iter_adv2_matches_encode_to_vec() {
    let items: Vec<u32> = vec![100, 200, 300, 400, 500];
    let iter_enc =
        oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode via iter_to_vec");
    let vec_enc = encode_to_vec(&items).expect("encode via encode_to_vec");
    assert_eq!(
        iter_enc, vec_enc,
        "encode_iter_to_vec must produce identical bytes to encode_to_vec for the same data"
    );
}
