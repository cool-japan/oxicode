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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

// -----------------------------------------------------------------------
// 1. Vec<Vec<u8>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_vec_vec_u8_roundtrip() {
    let original: Vec<Vec<u8>> = vec![
        vec![0x00, 0x01, 0x02],
        vec![],
        vec![0xFF, 0xFE, 0xFD],
        (0u8..=9).collect(),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Vec<u8>> failed");
    let (decoded, consumed): (Vec<Vec<u8>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Vec<u8>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 2. Vec<Vec<String>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_vec_vec_string_roundtrip() {
    let original: Vec<Vec<String>> = vec![
        vec!["alpha".to_string(), "beta".to_string()],
        vec![],
        vec!["gamma".to_string()],
        vec![
            "delta".to_string(),
            "epsilon".to_string(),
            "zeta".to_string(),
        ],
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Vec<String>> failed");
    let (decoded, consumed): (Vec<Vec<String>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Vec<String>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 3. Vec<Option<u32>> roundtrip (mixed Some/None)
// -----------------------------------------------------------------------
#[test]
fn test_vec_option_u32_mixed_roundtrip() {
    let original: Vec<Option<u32>> =
        vec![Some(1), None, Some(42), None, Some(u32::MAX), None, Some(0)];
    let encoded = encode_to_vec(&original).expect("encode Vec<Option<u32>> failed");
    let (decoded, consumed): (Vec<Option<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Option<u32>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded[0], Some(1));
    assert_eq!(decoded[1], None);
    assert_eq!(decoded[2], Some(42));
}

// -----------------------------------------------------------------------
// 4. Vec<(u32, String)> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_vec_tuple_u32_string_roundtrip() {
    let original: Vec<(u32, String)> = vec![
        (1, "one".to_string()),
        (2, "two".to_string()),
        (100, "hundred".to_string()),
        (0, "zero".to_string()),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<(u32, String)> failed");
    let (decoded, consumed): (Vec<(u32, String)>, _) =
        decode_from_slice(&encoded).expect("decode Vec<(u32, String)> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 5. Option<Vec<u32>> Some roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_option_vec_u32_some_roundtrip() {
    let original: Option<Vec<u32>> = Some(vec![10, 20, 30, 40, 50]);
    let encoded = encode_to_vec(&original).expect("encode Option<Vec<u32>> Some failed");
    let (decoded, consumed): (Option<Vec<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Option<Vec<u32>> Some failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.is_some());
    assert_eq!(decoded.expect("expected Some"), vec![10, 20, 30, 40, 50]);
}

// -----------------------------------------------------------------------
// 6. Option<Vec<u32>> None roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_option_vec_u32_none_roundtrip() {
    let original: Option<Vec<u32>> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Vec<u32>> None failed");
    let (decoded, consumed): (Option<Vec<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Option<Vec<u32>> None failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.is_none());
}

// -----------------------------------------------------------------------
// 7. Vec<Vec<Vec<u8>>> three levels deep
// -----------------------------------------------------------------------
#[test]
fn test_vec_vec_vec_u8_three_levels_roundtrip() {
    let original: Vec<Vec<Vec<u8>>> = vec![
        vec![vec![0x01, 0x02], vec![0x03]],
        vec![vec![], vec![0xFF]],
        vec![],
        vec![(0u8..=4u8).collect::<Vec<u8>>()],
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Vec<Vec<u8>>> failed");
    let (decoded, consumed): (Vec<Vec<Vec<u8>>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Vec<Vec<u8>>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded[2].is_empty());
}

// -----------------------------------------------------------------------
// 8. BTreeMap<String, Vec<u32>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_btreemap_string_vec_u32_roundtrip() {
    let mut original: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    original.insert("fibonacci".to_string(), vec![1, 1, 2, 3, 5, 8, 13]);
    original.insert("primes".to_string(), vec![2, 3, 5, 7, 11]);
    original.insert("empty".to_string(), vec![]);
    let encoded = encode_to_vec(&original).expect("encode BTreeMap<String, Vec<u32>> failed");
    let (decoded, consumed): (BTreeMap<String, Vec<u32>>, _) =
        decode_from_slice(&encoded).expect("decode BTreeMap<String, Vec<u32>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.get("empty"), Some(&vec![]));
}

// -----------------------------------------------------------------------
// 9. BTreeMap<u32, BTreeMap<u32, String>> nested map roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_btreemap_nested_maps_roundtrip() {
    let mut original: BTreeMap<u32, BTreeMap<u32, String>> = BTreeMap::new();

    let mut inner1: BTreeMap<u32, String> = BTreeMap::new();
    inner1.insert(1, "one".to_string());
    inner1.insert(2, "two".to_string());

    let mut inner2: BTreeMap<u32, String> = BTreeMap::new();
    inner2.insert(100, "hundred".to_string());
    inner2.insert(200, "two hundred".to_string());
    inner2.insert(300, "three hundred".to_string());

    original.insert(10, inner1);
    original.insert(20, inner2);
    original.insert(30, BTreeMap::new());

    let encoded =
        encode_to_vec(&original).expect("encode BTreeMap<u32, BTreeMap<u32, String>> failed");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): (BTreeMap<u32, BTreeMap<u32, String>>, _) =
        decode_from_slice(&encoded).expect("decode BTreeMap<u32, BTreeMap<u32, String>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.get(&30).expect("key 30 missing").is_empty());
}

// -----------------------------------------------------------------------
// 10. Vec<BTreeMap<String, u32>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_vec_btreemap_string_u32_roundtrip() {
    let map1: BTreeMap<String, u32> = [("alpha".to_string(), 1u32), ("beta".to_string(), 2u32)]
        .into_iter()
        .collect();
    let map2: BTreeMap<String, u32> = [("gamma".to_string(), 3u32)].into_iter().collect();
    let map3: BTreeMap<String, u32> = BTreeMap::new();

    let original: Vec<BTreeMap<String, u32>> = vec![map1, map2, map3];
    let encoded = encode_to_vec(&original).expect("encode Vec<BTreeMap<String, u32>> failed");
    let (decoded, consumed): (Vec<BTreeMap<String, u32>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<BTreeMap<String, u32>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded[2].is_empty());
}

// -----------------------------------------------------------------------
// 11. HashMap<String, Vec<u8>> roundtrip — compare with assert_eq!(decoded, original)
// -----------------------------------------------------------------------
#[test]
fn test_hashmap_string_vec_u8_roundtrip() {
    let mut original: HashMap<String, Vec<u8>> = HashMap::new();
    original.insert("header".to_string(), vec![0xDE, 0xAD, 0xBE, 0xEF]);
    original.insert("empty".to_string(), vec![]);
    original.insert("data".to_string(), (0u8..=15).collect());
    original.insert("trailer".to_string(), vec![0xFF, 0x00]);
    let encoded = encode_to_vec(&original).expect("encode HashMap<String, Vec<u8>> failed");
    let (decoded, consumed): (HashMap<String, Vec<u8>>, _) =
        decode_from_slice(&encoded).expect("decode HashMap<String, Vec<u8>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 12. Option<HashMap<String, u32>> Some roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_option_hashmap_string_u32_some_roundtrip() {
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("key1".to_string(), 100);
    map.insert("key2".to_string(), 200);
    map.insert("key3".to_string(), 300);
    let original: Option<HashMap<String, u32>> = Some(map);
    let encoded = encode_to_vec(&original).expect("encode Option<HashMap<String, u32>> failed");
    let (decoded, consumed): (Option<HashMap<String, u32>>, _) =
        decode_from_slice(&encoded).expect("decode Option<HashMap<String, u32>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    let inner = decoded.expect("expected Some(HashMap)");
    assert_eq!(inner.get("key1"), Some(&100u32));
    assert_eq!(inner.get("key2"), Some(&200u32));
    assert_eq!(inner.get("key3"), Some(&300u32));
}

// -----------------------------------------------------------------------
// 13. BTreeSet<Vec<u8>> roundtrip (sorted sets of byte vectors)
// -----------------------------------------------------------------------
#[test]
fn test_btreeset_vec_u8_roundtrip() {
    let original: BTreeSet<Vec<u8>> = vec![
        vec![0x03, 0x04],
        vec![0x01, 0x02],
        vec![],
        vec![0xFF],
        vec![0x00, 0x00],
    ]
    .into_iter()
    .collect();
    let encoded = encode_to_vec(&original).expect("encode BTreeSet<Vec<u8>> failed");
    let (decoded, consumed): (BTreeSet<Vec<u8>>, _) =
        decode_from_slice(&encoded).expect("decode BTreeSet<Vec<u8>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // BTreeSet<Vec<u8>> iterates in lexicographic order
    let items: Vec<&Vec<u8>> = decoded.iter().collect();
    assert!(
        items.windows(2).all(|w| w[0] <= w[1]),
        "BTreeSet must be sorted"
    );
}

// -----------------------------------------------------------------------
// 14. Vec<BTreeSet<u32>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_vec_btreeset_u32_roundtrip() {
    let set1: BTreeSet<u32> = vec![30u32, 10, 20].into_iter().collect();
    let set2: BTreeSet<u32> = vec![999u32, 1].into_iter().collect();
    let set3: BTreeSet<u32> = BTreeSet::new();

    let original: Vec<BTreeSet<u32>> = vec![set1, set2, set3];
    let encoded = encode_to_vec(&original).expect("encode Vec<BTreeSet<u32>> failed");
    let (decoded, consumed): (Vec<BTreeSet<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<BTreeSet<u32>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    let first: Vec<u32> = decoded[0].iter().copied().collect();
    assert_eq!(first, vec![10, 20, 30]);
    assert!(decoded[2].is_empty());
}

// -----------------------------------------------------------------------
// 15. (Vec<u32>, BTreeMap<String, u64>) tuple roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_tuple_vec_u32_btreemap_string_u64_roundtrip() {
    let vec_part: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut map_part: BTreeMap<String, u64> = BTreeMap::new();
    map_part.insert("million".to_string(), 1_000_000u64);
    map_part.insert("billion".to_string(), 1_000_000_000u64);
    map_part.insert("max".to_string(), u64::MAX);

    let original: (Vec<u32>, BTreeMap<String, u64>) = (vec_part, map_part);
    let encoded =
        encode_to_vec(&original).expect("encode (Vec<u32>, BTreeMap<String, u64>) failed");
    let (decoded, consumed): ((Vec<u32>, BTreeMap<String, u64>), _) =
        decode_from_slice(&encoded).expect("decode (Vec<u32>, BTreeMap<String, u64>) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.0, vec![1, 2, 3, 4, 5]);
    assert_eq!(decoded.1.get("million"), Some(&1_000_000u64));
}

// -----------------------------------------------------------------------
// 16. Vec<Option<String>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_vec_option_string_roundtrip() {
    let original: Vec<Option<String>> = vec![
        Some("hello".to_string()),
        None,
        Some("world".to_string()),
        None,
        Some(String::new()),
        Some("last".to_string()),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Option<String>> failed");
    let (decoded, consumed): (Vec<Option<String>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Option<String>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded[1], None);
    assert_eq!(decoded[4], Some(String::new()));
}

// -----------------------------------------------------------------------
// 17. Consumed == encoded.len() for nested vec
// -----------------------------------------------------------------------
#[test]
fn test_consumed_equals_encoded_len_for_nested_vec() {
    let original: Vec<Vec<u32>> = vec![vec![1, 2, 3], vec![], vec![100, 200, 300, 400], vec![42]];
    let encoded = encode_to_vec(&original).expect("encode nested vec for consume check failed");
    let (decoded, consumed): (Vec<Vec<u32>>, _) =
        decode_from_slice(&encoded).expect("decode nested vec for consume check failed");
    assert_eq!(decoded, original);
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal encoded length for nested vec"
    );
}

// -----------------------------------------------------------------------
// 18. Fixed-int config with Vec<Vec<u32>>
// -----------------------------------------------------------------------
#[test]
fn test_fixed_int_config_vec_vec_u32_roundtrip() {
    let original: Vec<Vec<u32>> = vec![
        vec![0, 1, 2, u32::MAX],
        vec![u32::MAX / 2, u32::MAX / 3],
        vec![],
    ];
    let cfg = config::legacy();
    let encoded = encode_to_vec_with_config(&original, cfg)
        .expect("encode Vec<Vec<u32>> with legacy config failed");
    let (decoded, consumed): (Vec<Vec<u32>>, _) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode Vec<Vec<u32>> with legacy config failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 19. Big-endian config with BTreeMap<String, u32>
// -----------------------------------------------------------------------
#[test]
fn test_big_endian_config_btreemap_string_u32_roundtrip() {
    let mut original: BTreeMap<String, u32> = BTreeMap::new();
    original.insert("low".to_string(), 1u32);
    original.insert("mid".to_string(), 65535u32);
    original.insert("high".to_string(), u32::MAX);
    original.insert("zero".to_string(), 0u32);
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&original, cfg)
        .expect("encode BTreeMap with big-endian config failed");
    let (decoded, consumed): (BTreeMap<String, u32>, _) =
        decode_from_slice_with_config(&encoded, cfg)
            .expect("decode BTreeMap with big-endian config failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 20. BTreeMap<String, Option<u64>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_btreemap_string_option_u64_roundtrip() {
    let mut original: BTreeMap<String, Option<u64>> = BTreeMap::new();
    original.insert("present_max".to_string(), Some(u64::MAX));
    original.insert("absent".to_string(), None);
    original.insert("present_zero".to_string(), Some(0u64));
    original.insert("also_absent".to_string(), None);
    original.insert("present_mid".to_string(), Some(1_000_000u64));
    let encoded = encode_to_vec(&original).expect("encode BTreeMap<String, Option<u64>> failed");
    let (decoded, consumed): (BTreeMap<String, Option<u64>>, _) =
        decode_from_slice(&encoded).expect("decode BTreeMap<String, Option<u64>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.get("absent"), Some(&None));
    assert_eq!(decoded.get("present_max"), Some(&Some(u64::MAX)));
}

// -----------------------------------------------------------------------
// 21. Vec<(String, Vec<u8>)> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_vec_tuple_string_vec_u8_roundtrip() {
    let original: Vec<(String, Vec<u8>)> = vec![
        ("header".to_string(), vec![0xDE, 0xAD, 0xBE, 0xEF]),
        ("empty".to_string(), vec![]),
        ("data".to_string(), (0u8..=7).collect()),
        ("trailer".to_string(), vec![0xFF]),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<(String, Vec<u8>)> failed");
    let (decoded, consumed): (Vec<(String, Vec<u8>)>, _) =
        decode_from_slice(&encoded).expect("decode Vec<(String, Vec<u8>)> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded[1].1, Vec::<u8>::new());
}

// -----------------------------------------------------------------------
// 22. HashMap<u32, HashSet<u32>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_hashmap_u32_hashset_u32_roundtrip() {
    let mut original: HashMap<u32, HashSet<u32>> = HashMap::new();
    let set_a: HashSet<u32> = vec![1u32, 2, 3].into_iter().collect();
    let set_b: HashSet<u32> = vec![10u32, 20, 30, 40].into_iter().collect();
    let set_c: HashSet<u32> = HashSet::new();
    original.insert(1, set_a);
    original.insert(2, set_b);
    original.insert(3, set_c);
    let encoded = encode_to_vec(&original).expect("encode HashMap<u32, HashSet<u32>> failed");
    let (decoded, _consumed): (HashMap<u32, HashSet<u32>>, _) =
        decode_from_slice(&encoded).expect("decode HashMap<u32, HashSet<u32>> failed");
    assert_eq!(decoded, original);
}

// Suppress unused import warning for Encode/Decode brought in for derive usage
const _: () = {
    fn _assert_encode_decode<T: Encode + Decode>() {}
};
