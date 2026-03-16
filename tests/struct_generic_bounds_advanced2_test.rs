//! Advanced tests for structs and enums with generic type bounds in OxiCode.
//! Covers Container<T>, KeyValue<K,V>, TaggedValue<T>, and Pair<A,B> with
//! various concrete type instantiations, config variations, and nested cases.

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

// ---------------------------------------------------------------------------
// Type definitions
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Container<T> {
    item: T,
    count: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TaggedValue<T> {
    Present(T),
    Absent,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair<A, B> {
    left: A,
    right: B,
}

// ---------------------------------------------------------------------------
// 1. Container<u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_container_u32_roundtrip() {
    let original = Container {
        item: 42u32,
        count: 7,
    };
    let encoded = encode_to_vec(&original).expect("encode Container<u32>");
    let (decoded, _): (Container<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Container<u32>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 2. Container<String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_container_string_roundtrip() {
    let original = Container {
        item: "hello oxicode".to_string(),
        count: 3,
    };
    let encoded = encode_to_vec(&original).expect("encode Container<String>");
    let (decoded, _): (Container<String>, usize) =
        decode_from_slice(&encoded).expect("decode Container<String>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 3. Container<Vec<u8>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_container_vec_u8_roundtrip() {
    let original = Container {
        item: vec![0u8, 1, 2, 3, 255],
        count: 5,
    };
    let encoded = encode_to_vec(&original).expect("encode Container<Vec<u8>>");
    let (decoded, _): (Container<Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode Container<Vec<u8>>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 4. Container<bool> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_container_bool_roundtrip() {
    let original = Container {
        item: true,
        count: 1,
    };
    let encoded = encode_to_vec(&original).expect("encode Container<bool>");
    let (decoded, _): (Container<bool>, usize) =
        decode_from_slice(&encoded).expect("decode Container<bool>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 5. KeyValue<String, u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_keyvalue_string_u32_roundtrip() {
    let original = KeyValue {
        key: "age".to_string(),
        value: 30u32,
    };
    let encoded = encode_to_vec(&original).expect("encode KeyValue<String, u32>");
    let (decoded, _): (KeyValue<String, u32>, usize) =
        decode_from_slice(&encoded).expect("decode KeyValue<String, u32>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 6. KeyValue<u32, String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_keyvalue_u32_string_roundtrip() {
    let original = KeyValue {
        key: 99u32,
        value: "ninety-nine".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode KeyValue<u32, String>");
    let (decoded, _): (KeyValue<u32, String>, usize) =
        decode_from_slice(&encoded).expect("decode KeyValue<u32, String>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 7. KeyValue<u64, Vec<u8>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_keyvalue_u64_vec_u8_roundtrip() {
    let original = KeyValue {
        key: u64::MAX / 2,
        value: vec![10u8, 20, 30, 40],
    };
    let encoded = encode_to_vec(&original).expect("encode KeyValue<u64, Vec<u8>>");
    let (decoded, _): (KeyValue<u64, Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode KeyValue<u64, Vec<u8>>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 8. TaggedValue<u32> Present roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_value_u32_present_roundtrip() {
    let original = TaggedValue::Present(123u32);
    let encoded = encode_to_vec(&original).expect("encode TaggedValue::Present(u32)");
    let (decoded, _): (TaggedValue<u32>, usize) =
        decode_from_slice(&encoded).expect("decode TaggedValue::Present(u32)");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 9. TaggedValue<u32> Absent roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_value_u32_absent_roundtrip() {
    let original: TaggedValue<u32> = TaggedValue::Absent;
    let encoded = encode_to_vec(&original).expect("encode TaggedValue::Absent");
    let (decoded, _): (TaggedValue<u32>, usize) =
        decode_from_slice(&encoded).expect("decode TaggedValue::Absent");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 10. TaggedValue<String> Present roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_value_string_present_roundtrip() {
    let original = TaggedValue::Present("greetings".to_string());
    let encoded = encode_to_vec(&original).expect("encode TaggedValue::Present(String)");
    let (decoded, _): (TaggedValue<String>, usize) =
        decode_from_slice(&encoded).expect("decode TaggedValue::Present(String)");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 11. TaggedValue<Vec<u8>> Present with empty vec
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_value_vec_u8_present_empty_roundtrip() {
    let original = TaggedValue::Present(Vec::<u8>::new());
    let encoded = encode_to_vec(&original).expect("encode TaggedValue::Present(empty Vec<u8>)");
    let (decoded, _): (TaggedValue<Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode TaggedValue::Present(empty Vec<u8>)");
    assert_eq!(original, decoded);
    if let TaggedValue::Present(ref v) = decoded {
        assert!(v.is_empty());
    }
}

// ---------------------------------------------------------------------------
// 12. Vec<Container<u32>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_container_u32_roundtrip() {
    let original: Vec<Container<u32>> = vec![
        Container {
            item: 1u32,
            count: 10,
        },
        Container {
            item: 2u32,
            count: 20,
        },
        Container {
            item: 3u32,
            count: 30,
        },
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Container<u32>>");
    let (decoded, _): (Vec<Container<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Container<u32>>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 13. Option<Container<u32>> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_container_u32_some_roundtrip() {
    let original: Option<Container<u32>> = Some(Container {
        item: 77u32,
        count: 4,
    });
    let encoded = encode_to_vec(&original).expect("encode Option<Container<u32>> Some");
    let (decoded, _): (Option<Container<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Container<u32>> Some");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 14. Option<Container<u32>> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_container_u32_none_roundtrip() {
    let original: Option<Container<u32>> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Container<u32>> None");
    let (decoded, _): (Option<Container<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Container<u32>> None");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 15. Container<u32> consumed equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_container_u32_consumed_equals_encoded_len() {
    let original = Container {
        item: 500u32,
        count: 2,
    };
    let encoded = encode_to_vec(&original).expect("encode for consumed check");
    let (_, consumed): (Container<u32>, usize) =
        decode_from_slice(&encoded).expect("decode for consumed check");
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 16. KeyValue<u32, u32> with fixed-int (legacy) config
// ---------------------------------------------------------------------------

#[test]
fn test_keyvalue_u32_u32_legacy_config_roundtrip() {
    let original = KeyValue {
        key: 1u32,
        value: 2u32,
    };
    let cfg = config::legacy();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode KeyValue<u32,u32> legacy");
    // legacy encodes each u32 as 4 bytes fixed; key(4) + value(4) = 8 bytes
    assert_eq!(
        encoded.len(),
        8,
        "legacy fixed-int u32 pair must be 8 bytes"
    );
    let (decoded, _): (KeyValue<u32, u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode KeyValue<u32,u32> legacy");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 17. Container<u64> with big-endian config
// ---------------------------------------------------------------------------

#[test]
fn test_container_u64_big_endian_config_roundtrip() {
    let original = Container {
        item: 0xDEAD_BEEF_CAFE_BABEu64,
        count: 1,
    };
    let cfg = config::standard().with_big_endian();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Container<u64> big-endian");
    let (decoded, _): (Container<u64>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Container<u64> big-endian");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 18. Pair<u32, String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_pair_u32_string_roundtrip() {
    let original = Pair {
        left: 42u32,
        right: "pair test".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode Pair<u32, String>");
    let (decoded, _): (Pair<u32, String>, usize) =
        decode_from_slice(&encoded).expect("decode Pair<u32, String>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 19. Pair<Vec<u8>, bool> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_pair_vec_u8_bool_roundtrip() {
    let original = Pair {
        left: vec![9u8, 8, 7, 6, 5],
        right: false,
    };
    let encoded = encode_to_vec(&original).expect("encode Pair<Vec<u8>, bool>");
    let (decoded, _): (Pair<Vec<u8>, bool>, usize) =
        decode_from_slice(&encoded).expect("decode Pair<Vec<u8>, bool>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 20. Vec<TaggedValue<u32>> mixed Present/Absent roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_tagged_value_mixed_roundtrip() {
    let original: Vec<TaggedValue<u32>> = vec![
        TaggedValue::Present(1),
        TaggedValue::Absent,
        TaggedValue::Present(3),
        TaggedValue::Absent,
        TaggedValue::Present(99),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<TaggedValue<u32>> mixed");
    let (decoded, _): (Vec<TaggedValue<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<TaggedValue<u32>> mixed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 21. Container<Option<u32>> with inner Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_container_option_u32_some_roundtrip() {
    let original = Container {
        item: Some(256u32),
        count: 9,
    };
    let encoded = encode_to_vec(&original).expect("encode Container<Option<u32>> Some");
    let (decoded, _): (Container<Option<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Container<Option<u32>> Some");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 22. Container<Option<u32>> with inner None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_container_option_u32_none_roundtrip() {
    let original = Container {
        item: None::<u32>,
        count: 0,
    };
    let encoded = encode_to_vec(&original).expect("encode Container<Option<u32>> None");
    let (decoded, _): (Container<Option<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Container<Option<u32>> None");
    assert_eq!(original, decoded);
}
