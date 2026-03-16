//! Advanced tests for deeply nested collection encoding in OxiCode.
//!
//! Covers 2D/3D/4D vectors, nested maps, option-wrapped collections,
//! tuple-containing vectors, and config-driven encoding variants.

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
use std::collections::{BTreeMap, HashMap, HashSet};

// ---------------------------------------------------------------------------
// 1. Vec<Vec<u32>> roundtrip (2D vector)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_u32_2d_roundtrip() {
    let original: Vec<Vec<u32>> = vec![
        vec![1, 2, 3],
        vec![10, 20, 30, 40],
        vec![100, 200],
        vec![0, u32::MAX],
    ];
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Vec<u32>>");
    let (decoded, consumed): (Vec<Vec<u32>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<Vec<u32>>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 2. Vec<Vec<Vec<u8>>> roundtrip (3D vector)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_vec_u8_3d_roundtrip() {
    let original: Vec<Vec<Vec<u8>>> = vec![
        vec![vec![1, 2, 3], vec![4, 5]],
        vec![vec![0xFF, 0x00], vec![], vec![42]],
        vec![vec![7, 8, 9, 10]],
    ];
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Vec<Vec<u8>>>");
    let (decoded, consumed): (Vec<Vec<Vec<u8>>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<Vec<Vec<u8>>>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 3. Vec<Vec<u32>> empty outer roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_u32_empty_outer_roundtrip() {
    let original: Vec<Vec<u32>> = vec![];
    let bytes = encode_to_vec(&original).expect("Failed to encode empty outer Vec<Vec<u32>>");
    let (decoded, consumed): (Vec<Vec<u32>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode empty outer Vec<Vec<u32>>");
    assert_eq!(decoded, original);
    assert!(decoded.is_empty());
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 4. Vec<Vec<u32>> with empty inner vecs roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_u32_empty_inner_roundtrip() {
    let original: Vec<Vec<u32>> = vec![vec![], vec![], vec![1], vec![], vec![2, 3]];
    let bytes =
        encode_to_vec(&original).expect("Failed to encode Vec<Vec<u32>> with empty inner vecs");
    let (decoded, consumed): (Vec<Vec<u32>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<Vec<u32>> with empty inner vecs");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
    // Verify the empty vecs are preserved
    assert!(decoded[0].is_empty());
    assert!(decoded[1].is_empty());
    assert!(decoded[3].is_empty());
}

// ---------------------------------------------------------------------------
// 5. Option<Vec<Vec<u8>>> Some roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_vec_vec_u8_some_roundtrip() {
    let original: Option<Vec<Vec<u8>>> = Some(vec![
        vec![1, 2, 3],
        vec![],
        vec![0xAB, 0xCD, 0xEF],
        vec![255, 128, 0],
    ]);
    let bytes = encode_to_vec(&original).expect("Failed to encode Option<Vec<Vec<u8>>>");
    let (decoded, consumed): (Option<Vec<Vec<u8>>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Option<Vec<Vec<u8>>>");
    assert_eq!(decoded, original);
    assert!(decoded.is_some());
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 6. Vec<Option<Vec<u8>>> with Some/None mix roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_option_vec_u8_mixed_roundtrip() {
    let original: Vec<Option<Vec<u8>>> = vec![
        Some(vec![1, 2, 3]),
        None,
        Some(vec![]),
        Some(vec![0xFF]),
        None,
        Some(vec![10, 20, 30, 40, 50]),
    ];
    let bytes =
        encode_to_vec(&original).expect("Failed to encode Vec<Option<Vec<u8>>> with mixed values");
    let (decoded, consumed): (Vec<Option<Vec<u8>>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<Option<Vec<u8>>> with mixed values");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
    assert!(decoded[1].is_none());
    assert!(decoded[4].is_none());
}

// ---------------------------------------------------------------------------
// 7. HashMap<String, Vec<u32>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_string_vec_u32_roundtrip() {
    let mut original: HashMap<String, Vec<u32>> = HashMap::new();
    original.insert("primes".to_string(), vec![2, 3, 5, 7, 11, 13]);
    original.insert("fibonacci".to_string(), vec![1, 1, 2, 3, 5, 8, 13]);
    original.insert("empty".to_string(), vec![]);
    original.insert("single".to_string(), vec![42]);

    let bytes = encode_to_vec(&original).expect("Failed to encode HashMap<String, Vec<u32>>");
    let (decoded, consumed): (HashMap<String, Vec<u32>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode HashMap<String, Vec<u32>>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 8. HashMap<String, Vec<String>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_string_vec_string_roundtrip() {
    let mut original: HashMap<String, Vec<String>> = HashMap::new();
    original.insert(
        "fruits".to_string(),
        vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ],
    );
    original.insert(
        "colors".to_string(),
        vec!["red".to_string(), "green".to_string(), "blue".to_string()],
    );
    original.insert("empty_list".to_string(), vec![]);

    let bytes = encode_to_vec(&original).expect("Failed to encode HashMap<String, Vec<String>>");
    let (decoded, consumed): (HashMap<String, Vec<String>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode HashMap<String, Vec<String>>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 9. Vec<HashMap<String, u32>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_hashmap_string_u32_roundtrip() {
    let mut map1: HashMap<String, u32> = HashMap::new();
    map1.insert("a".to_string(), 1);
    map1.insert("b".to_string(), 2);

    let mut map2: HashMap<String, u32> = HashMap::new();
    map2.insert("x".to_string(), 100);
    map2.insert("y".to_string(), 200);
    map2.insert("z".to_string(), 300);

    let original: Vec<HashMap<String, u32>> = vec![map1, map2, HashMap::new()];

    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<HashMap<String, u32>>");
    let (decoded, consumed): (Vec<HashMap<String, u32>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<HashMap<String, u32>>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 10. BTreeMap<String, Vec<u8>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_string_vec_u8_roundtrip() {
    let mut original: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    original.insert("alpha".to_string(), vec![0x01, 0x02, 0x03]);
    original.insert("beta".to_string(), vec![]);
    original.insert("gamma".to_string(), vec![0xAA, 0xBB, 0xCC, 0xDD]);
    original.insert("delta".to_string(), vec![255, 128, 64, 32, 16, 8, 4, 2, 1]);

    let bytes = encode_to_vec(&original).expect("Failed to encode BTreeMap<String, Vec<u8>>");
    let (decoded, consumed): (BTreeMap<String, Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeMap<String, Vec<u8>>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 11. Vec<BTreeMap<String, u32>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_btreemap_string_u32_roundtrip() {
    let map1: BTreeMap<String, u32> = [
        ("one".to_string(), 1u32),
        ("two".to_string(), 2u32),
        ("three".to_string(), 3u32),
    ]
    .into_iter()
    .collect();

    let map2: BTreeMap<String, u32> = [("hundred".to_string(), 100u32)].into_iter().collect();

    let original: Vec<BTreeMap<String, u32>> = vec![map1, map2, BTreeMap::new()];

    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<BTreeMap<String, u32>>");
    let (decoded, consumed): (Vec<BTreeMap<String, u32>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<BTreeMap<String, u32>>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 12. HashSet<String> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_hashset_string_roundtrip() {
    let original: HashSet<String> = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
        "elderberry".to_string(),
    ]
    .into_iter()
    .collect();

    let bytes = encode_to_vec(&original).expect("Failed to encode HashSet<String>");
    let (decoded, consumed): (HashSet<String>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode HashSet<String>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 13. Vec<(u32, Vec<u8>)> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_tuple_u32_vec_u8_roundtrip() {
    let original: Vec<(u32, Vec<u8>)> = vec![
        (1u32, vec![0x01, 0x02, 0x03]),
        (42u32, vec![]),
        (100u32, vec![0xFF, 0xFE, 0xFD]),
        (999u32, vec![0, 128, 255]),
        (u32::MAX, vec![0xAB, 0xCD]),
    ];

    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<(u32, Vec<u8>)>");
    let (decoded, consumed): (Vec<(u32, Vec<u8>)>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<(u32, Vec<u8>)>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 14. Vec<Vec<Option<u32>>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_option_u32_roundtrip() {
    let original: Vec<Vec<Option<u32>>> = vec![
        vec![Some(1), None, Some(3)],
        vec![None, None],
        vec![Some(0), Some(u32::MAX)],
        vec![],
        vec![Some(42)],
    ];

    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Vec<Option<u32>>>");
    let (decoded, consumed): (Vec<Vec<Option<u32>>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<Vec<Option<u32>>>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 15. HashMap<u32, HashMap<String, Vec<u8>>> nested roundtrip
// ---------------------------------------------------------------------------
#[allow(clippy::type_complexity)]
#[test]
fn test_hashmap_nested_deep_roundtrip() {
    let mut inner1: HashMap<String, Vec<u8>> = HashMap::new();
    inner1.insert("bytes_a".to_string(), vec![1, 2, 3]);
    inner1.insert("bytes_b".to_string(), vec![]);

    let mut inner2: HashMap<String, Vec<u8>> = HashMap::new();
    inner2.insert("data".to_string(), vec![0xFF, 0x00, 0xAB]);
    inner2.insert("more".to_string(), vec![10, 20, 30, 40, 50]);

    let mut original: HashMap<u32, HashMap<String, Vec<u8>>> = HashMap::new();
    original.insert(1u32, inner1);
    original.insert(2u32, inner2);
    original.insert(3u32, HashMap::new());

    let bytes =
        encode_to_vec(&original).expect("Failed to encode HashMap<u32, HashMap<String, Vec<u8>>>");
    let (decoded, consumed): (HashMap<u32, HashMap<String, Vec<u8>>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode HashMap<u32, HashMap<String, Vec<u8>>>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 16. Vec<Vec<u32>> consumed equals encoded length
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_u32_consumed_equals_encoded_length() {
    let original: Vec<Vec<u32>> = vec![
        vec![1, 2, 3, 4, 5],
        vec![100, 200, 300],
        vec![],
        vec![999, 1000, 1001, 1002],
    ];

    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Vec<u32>> for size check");
    let (decoded, consumed): (Vec<Vec<u32>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<Vec<u32>> for size check");

    assert_eq!(decoded, original);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal the encoded slice length"
    );
    assert!(!bytes.is_empty(), "encoded bytes must not be empty");
}

// ---------------------------------------------------------------------------
// 17. Deeply nested: Vec<Vec<Vec<Vec<u8>>>> roundtrip (4 levels)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_4d_nested_roundtrip() {
    let original: Vec<Vec<Vec<Vec<u8>>>> = vec![
        vec![vec![vec![1, 2], vec![3, 4, 5]], vec![vec![6], vec![]]],
        vec![vec![vec![0xFF, 0x00], vec![0xAB, 0xCD, 0xEF]]],
        vec![vec![]],
        vec![],
    ];

    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Vec<Vec<Vec<u8>>>>");
    let (decoded, consumed): (Vec<Vec<Vec<Vec<u8>>>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<Vec<Vec<Vec<u8>>>>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 18. Vec<Vec<u8>> with fixed-int config
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_u8_fixed_int_config_roundtrip() {
    let original: Vec<Vec<u8>> = vec![
        vec![0x01, 0x02, 0x03],
        vec![],
        vec![0xAA, 0xBB, 0xCC, 0xDD],
        vec![255, 128, 0],
    ];

    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec_with_config(&original, cfg).expect("Failed to encode with fixed-int config");
    let (decoded, consumed): (Vec<Vec<u8>>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("Failed to decode with fixed-int config");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 19. Vec<Vec<u32>> with big-endian config
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_u32_big_endian_config_roundtrip() {
    let original: Vec<Vec<u32>> = vec![
        vec![0xDEAD_BEEF, 0xCAFE_BABE],
        vec![1, 2, 3, 4],
        vec![],
        vec![u32::MAX, 0, 1],
    ];

    let cfg = config::standard().with_big_endian();
    let bytes =
        encode_to_vec_with_config(&original, cfg).expect("Failed to encode with big-endian config");
    let (decoded, consumed): (Vec<Vec<u32>>, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("Failed to decode with big-endian config");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 20. Vec<Option<Vec<Option<u32>>>> doubly-optional roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_option_vec_option_u32_doubly_optional_roundtrip() {
    let original: Vec<Option<Vec<Option<u32>>>> = vec![
        Some(vec![Some(1), None, Some(3)]),
        None,
        Some(vec![]),
        Some(vec![None, None, Some(42)]),
        Some(vec![Some(u32::MAX), Some(0)]),
        None,
    ];

    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Option<Vec<Option<u32>>>>");
    let (decoded, consumed): (Vec<Option<Vec<Option<u32>>>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<Option<Vec<Option<u32>>>>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
    assert!(decoded[1].is_none());
    assert!(decoded[5].is_none());
}

// ---------------------------------------------------------------------------
// 21. BTreeMap<String, BTreeMap<String, u32>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_nested_btreemap_roundtrip() {
    let mut inner_a: BTreeMap<String, u32> = BTreeMap::new();
    inner_a.insert("x".to_string(), 10u32);
    inner_a.insert("y".to_string(), 20u32);
    inner_a.insert("z".to_string(), 30u32);

    let mut inner_b: BTreeMap<String, u32> = BTreeMap::new();
    inner_b.insert("p".to_string(), 100u32);
    inner_b.insert("q".to_string(), 200u32);

    let mut original: BTreeMap<String, BTreeMap<String, u32>> = BTreeMap::new();
    original.insert("group_a".to_string(), inner_a);
    original.insert("group_b".to_string(), inner_b);
    original.insert("empty_group".to_string(), BTreeMap::new());

    let bytes =
        encode_to_vec(&original).expect("Failed to encode BTreeMap<String, BTreeMap<String, u32>>");
    let (decoded, consumed): (BTreeMap<String, BTreeMap<String, u32>>, usize) =
        decode_from_slice(&bytes)
            .expect("Failed to decode BTreeMap<String, BTreeMap<String, u32>>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());

    // Verify sorted key ordering is preserved
    let outer_keys: Vec<&str> = decoded.keys().map(String::as_str).collect();
    assert_eq!(outer_keys, vec!["empty_group", "group_a", "group_b"]);
}

// ---------------------------------------------------------------------------
// 22. Vec<Vec<u64>> with large values roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_u64_large_values_roundtrip() {
    let original: Vec<Vec<u64>> = vec![
        vec![u64::MAX, u64::MAX - 1, u64::MAX - 2],
        vec![u64::MAX / 2, u64::MAX / 3, u64::MAX / 4],
        vec![0, 1, u64::MAX],
        vec![
            0x0102_0304_0506_0708,
            0xDEAD_BEEF_CAFE_BABE,
            0xFFFF_FFFF_FFFF_FFFF,
        ],
        vec![],
    ];

    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Vec<u64>> with large values");
    let (decoded, consumed): (Vec<Vec<u64>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<Vec<u64>> with large values");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded[0][0], u64::MAX);
    assert_eq!(decoded[2][2], u64::MAX);
}
