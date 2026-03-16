//! Boundary value tests for varint encoding

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

macro_rules! test_boundary {
    ($name:ident, $ty:ty, $val:expr) => {
        #[test]
        fn $name() {
            let v: $ty = $val;
            let enc = encode_to_vec(&v).expect("encode");
            let (dec, _): ($ty, _) = decode_from_slice(&enc).expect("decode");
            assert_eq!(v, dec);
        }
    };
}

// u8 boundaries
test_boundary!(test_u8_min, u8, 0);
test_boundary!(test_u8_mid, u8, 127);
test_boundary!(test_u8_max, u8, 255);

// u16 boundaries
test_boundary!(test_u16_min, u16, 0);
test_boundary!(test_u16_varint_1byte, u16, 127);
test_boundary!(test_u16_varint_2byte, u16, 128);
test_boundary!(test_u16_max, u16, u16::MAX);

// u32 boundaries (oxicode varint: 0-250 = 1 byte, 251-65535 = 3 bytes, >65535 = 5 bytes)
test_boundary!(test_u32_min, u32, 0);
test_boundary!(test_u32_single_byte_max, u32, 250);
test_boundary!(test_u32_two_byte_threshold, u32, 251);
test_boundary!(test_u32_u16_max, u32, u16::MAX as u32);
test_boundary!(test_u32_u16_max_plus1, u32, u16::MAX as u32 + 1);
test_boundary!(test_u32_mid, u32, 0x0FFF_FFFF);
test_boundary!(test_u32_max, u32, u32::MAX);

// u64 boundaries
test_boundary!(test_u64_min, u64, 0);
test_boundary!(test_u64_max, u64, u64::MAX);
test_boundary!(test_u64_mid, u64, u64::MAX / 2);

// i8 boundaries (zigzag)
test_boundary!(test_i8_min, i8, i8::MIN);
test_boundary!(test_i8_neg1, i8, -1);
test_boundary!(test_i8_zero, i8, 0);
test_boundary!(test_i8_pos1, i8, 1);
test_boundary!(test_i8_max, i8, i8::MAX);

// i32 boundaries (zigzag)
test_boundary!(test_i32_min, i32, i32::MIN);
test_boundary!(test_i32_neg1, i32, -1);
test_boundary!(test_i32_zero, i32, 0);
test_boundary!(test_i32_max, i32, i32::MAX);

// i64 boundaries
test_boundary!(test_i64_min, i64, i64::MIN);
test_boundary!(test_i64_max, i64, i64::MAX);

// f32/f64 special values
#[test]
fn test_f32_nan_roundtrip() {
    let v = f32::NAN;
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (f32, _) = decode_from_slice(&enc).expect("decode");
    assert!(dec.is_nan());
}

#[test]
fn test_f64_infinity_roundtrip() {
    let v = f64::INFINITY;
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (f64, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, f64::INFINITY);
}

#[test]
fn test_f64_neg_infinity_roundtrip() {
    let v = f64::NEG_INFINITY;
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (f64, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, f64::NEG_INFINITY);
}

// char boundaries
#[test]
fn test_char_null() {
    let v = '\0';
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (char, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_char_max_unicode() {
    let v = '\u{10FFFF}'; // Maximum Unicode codepoint
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (char, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// Varint size tests
// oxicode uses bincode-compatible varint: values 0-250 encode as 1 byte,
// values 251-65535 encode as 3 bytes (marker + 2 bytes), etc.
#[test]
fn test_varint_1_byte_range() {
    // 0-250 encode as 1 byte (SINGLE_BYTE_MAX = 250)
    for i in 0u64..=250 {
        let enc = encode_to_vec(&i).expect("encode");
        assert_eq!(enc.len(), 1, "Value {} should encode as 1 byte", i);
    }
}

#[test]
fn test_varint_3_byte_boundary() {
    // 251 should encode as 3 bytes: marker byte + 2 bytes for u16
    let enc = encode_to_vec(&251u64).expect("encode");
    assert_eq!(
        enc.len(),
        3,
        "Value 251 should encode as 3 bytes (marker + u16)"
    );
}

#[test]
fn test_varint_u16_max_3_bytes() {
    // u16::MAX encodes as 3 bytes: marker byte + 2 bytes
    let enc = encode_to_vec(&(u16::MAX as u64)).expect("encode");
    assert_eq!(enc.len(), 3, "u16::MAX should encode as 3 bytes");
}

#[test]
fn test_varint_u32_range_5_bytes() {
    // Values > u16::MAX and <= u32::MAX encode as 5 bytes: marker byte + 4 bytes
    let val = u16::MAX as u64 + 1;
    let enc = encode_to_vec(&val).expect("encode");
    assert_eq!(
        enc.len(),
        5,
        "Value {} should encode as 5 bytes (marker + u32)",
        val
    );
}

#[test]
fn test_varint_u64_large_9_bytes() {
    // Values > u32::MAX encode as 9 bytes: marker byte + 8 bytes
    let val = u32::MAX as u64 + 1;
    let enc = encode_to_vec(&val).expect("encode");
    assert_eq!(
        enc.len(),
        9,
        "Value {} should encode as 9 bytes (marker + u64)",
        val
    );
}

// Empty collection tests
#[test]
fn test_empty_vec() {
    let empty: Vec<u32> = vec![];
    let enc = encode_to_vec(&empty).expect("encode");
    let (dec, _): (Vec<u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(empty, dec);
}

#[test]
fn test_empty_hashmap() {
    use std::collections::HashMap;
    let empty: HashMap<String, u32> = HashMap::new();
    let enc = encode_to_vec(&empty).expect("encode");
    let (dec, _): (HashMap<String, u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(empty, dec);
}

#[test]
fn test_empty_btreemap() {
    use std::collections::BTreeMap;
    let empty: BTreeMap<String, u32> = BTreeMap::new();
    let enc = encode_to_vec(&empty).expect("encode");
    let (dec, _): (BTreeMap<String, u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(empty, dec);
}

#[test]
fn test_empty_hashset() {
    use std::collections::HashSet;
    let empty: HashSet<u32> = HashSet::new();
    let enc = encode_to_vec(&empty).expect("encode");
    let (dec, _): (HashSet<u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(empty, dec);
}

#[test]
fn test_empty_vecdeque() {
    use std::collections::VecDeque;
    let empty: VecDeque<u32> = VecDeque::new();
    let enc = encode_to_vec(&empty).expect("encode");
    let (dec, _): (VecDeque<u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(empty, dec);
}

// Stress tests
#[test]
fn test_large_vec_alternating_extremes() {
    let data: Vec<u64> = (0..10_000)
        .map(|i| if i % 2 == 0 { u64::MAX } else { 0 })
        .collect();
    let enc = encode_to_vec(&data).expect("encode");
    let (dec, _): (Vec<u64>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(data, dec);
}

#[test]
fn test_deeply_nested_vec() {
    let data: Vec<Vec<Vec<u32>>> = vec![
        vec![vec![1, 2, 3], vec![4, 5]],
        vec![vec![], vec![u32::MAX]],
        vec![],
    ];
    let enc = encode_to_vec(&data).expect("encode");
    let (dec, _): (Vec<Vec<Vec<u32>>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(data, dec);
}
