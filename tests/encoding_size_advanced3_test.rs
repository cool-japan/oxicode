//! Advanced encoding size tests (set 3): verifies encoded_size matches encode_to_vec length
//! for a wide variety of types, including structs, enums, Options, tuples, and configs.

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
    config, decode_from_slice, encode_to_vec, encode_to_vec_with_config, encoded_size,
    encoded_size_with_config, Decode, Encode,
};

// ---------- Helper types ----------

#[derive(Encode, Decode, PartialEq, Debug)]
struct SimplePoint {
    x: i32,
    y: i32,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Nested {
    name: String,
    values: Vec<u16>,
    flag: bool,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Wrapper<T: Encode + Decode> {
    inner: T,
    tag: u8,
}

// ---------- 22 top-level tests ----------

/// Test 1: encoded_size(0u8) == encode_to_vec length and == 1
#[test]
fn test_encoded_size_u8_zero_is_one() {
    let val = 0u8;
    let size = encoded_size(&val).expect("encoded_size u8 zero");
    let actual = encode_to_vec(&val).expect("encode u8 zero").len();
    assert_eq!(size, actual);
    assert_eq!(size, 1);
}

/// Test 2: encoded_size(0u32) == encode_to_vec length and == 1
#[test]
fn test_encoded_size_u32_zero_is_one() {
    let val = 0u32;
    let size = encoded_size(&val).expect("encoded_size u32 zero");
    let actual = encode_to_vec(&val).expect("encode u32 zero").len();
    assert_eq!(size, actual);
    assert_eq!(size, 1);
}

/// Test 3: encoded_size(u32::MAX) == encode_to_vec length and == 5
#[test]
fn test_encoded_size_u32_max_is_five() {
    let val = u32::MAX;
    let size = encoded_size(&val).expect("encoded_size u32::MAX");
    let actual = encode_to_vec(&val).expect("encode u32::MAX").len();
    assert_eq!(size, actual);
    assert_eq!(size, 5);
}

/// Test 4: encoded_size(true) == encode_to_vec length and == 1
#[test]
fn test_encoded_size_bool_true_is_one() {
    let val = true;
    let size = encoded_size(&val).expect("encoded_size bool true");
    let actual = encode_to_vec(&val).expect("encode bool true").len();
    assert_eq!(size, actual);
    assert_eq!(size, 1);
}

/// Test 5: encoded_size("") == 1 (length prefix byte only, empty content)
#[test]
fn test_encoded_size_empty_str_is_one() {
    let val = "".to_string();
    let size = encoded_size(&val).expect("encoded_size empty string");
    let actual = encode_to_vec(&val).expect("encode empty string").len();
    assert_eq!(size, actual);
    assert_eq!(size, 1);
}

/// Test 6: encoded_size("hello") == 6 (1 length byte + 5 content bytes)
#[test]
fn test_encoded_size_hello_is_six() {
    let val = "hello".to_string();
    let size = encoded_size(&val).expect("encoded_size hello");
    let actual = encode_to_vec(&val).expect("encode hello").len();
    assert_eq!(size, actual);
    assert_eq!(size, 6);
}

/// Test 7: encoded_size(Vec::<u8>::new()) == 1 (length prefix only)
#[test]
fn test_encoded_size_empty_vec_u8_is_one() {
    let val: Vec<u8> = Vec::new();
    let size = encoded_size(&val).expect("encoded_size empty vec u8");
    let actual = encode_to_vec(&val).expect("encode empty vec u8").len();
    assert_eq!(size, actual);
    assert_eq!(size, 1);
}

/// Test 8: encoded_size matches encode_to_vec length for u64::MAX
#[test]
fn test_encoded_size_u64_max_matches_vec() {
    let val = u64::MAX;
    let size = encoded_size(&val).expect("encoded_size u64::MAX");
    let actual = encode_to_vec(&val).expect("encode u64::MAX").len();
    assert_eq!(size, actual);
    // u64::MAX in varint requires 9 bytes (marker 0xFF + 8 bytes)
    assert_eq!(size, 9);
}

/// Test 9: encoded_size matches encode_to_vec length for a String of 100 chars
#[test]
fn test_encoded_size_string_100_chars_matches_vec() {
    let val: String = "a".repeat(100);
    let size = encoded_size(&val).expect("encoded_size 100-char string");
    let actual = encode_to_vec(&val).expect("encode 100-char string").len();
    assert_eq!(size, actual);
    // 100 bytes content + 1 byte length prefix (100 <= 250)
    assert_eq!(size, 101);
}

/// Test 10: encoded_size matches encode_to_vec length for Vec<u32> of 50 elements
#[test]
fn test_encoded_size_vec_u32_50_matches_vec() {
    let val: Vec<u32> = (0u32..50).collect();
    let size = encoded_size(&val).expect("encoded_size vec u32 50");
    let actual = encode_to_vec(&val).expect("encode vec u32 50").len();
    assert_eq!(size, actual);
}

/// Test 11: encoded_size_with_config fixed-int: u32 is always 4 bytes
#[test]
fn test_encoded_size_with_config_fixed_u32_is_four() {
    let val = 42u32;
    let cfg = config::standard().with_fixed_int_encoding();
    let size = encoded_size_with_config(&val, cfg).expect("encoded_size fixed u32");
    let actual = encode_to_vec_with_config(&val, cfg)
        .expect("encode fixed u32")
        .len();
    assert_eq!(size, actual);
    assert_eq!(size, 4);
}

/// Test 12: encoded_size_with_config fixed-int: u64 is always 8 bytes
#[test]
fn test_encoded_size_with_config_fixed_u64_is_eight() {
    let val = 0u64;
    let cfg = config::standard().with_fixed_int_encoding();
    let size = encoded_size_with_config(&val, cfg).expect("encoded_size fixed u64");
    let actual = encode_to_vec_with_config(&val, cfg)
        .expect("encode fixed u64")
        .len();
    assert_eq!(size, actual);
    assert_eq!(size, 8);
}

/// Test 13: encoded_size for a simple struct matches encode_to_vec length
#[test]
fn test_encoded_size_simple_struct_matches_vec() {
    let val = SimplePoint { x: -42, y: 100 };
    let size = encoded_size(&val).expect("encoded_size SimplePoint");
    let actual = encode_to_vec(&val).expect("encode SimplePoint").len();
    assert_eq!(size, actual);
}

/// Test 14: encoded_size for enum unit variant matches encode_to_vec length
#[test]
fn test_encoded_size_enum_unit_variant_matches_vec() {
    let val = Color::Green;
    let size = encoded_size(&val).expect("encoded_size Color::Green");
    let actual = encode_to_vec(&val).expect("encode Color::Green").len();
    assert_eq!(size, actual);
}

/// Test 15: encoded_size for Option Some matches encode_to_vec length
#[test]
fn test_encoded_size_option_some_matches_vec() {
    let val: Option<u64> = Some(12345);
    let size = encoded_size(&val).expect("encoded_size Option Some");
    let actual = encode_to_vec(&val).expect("encode Option Some").len();
    assert_eq!(size, actual);
}

/// Test 16: encoded_size for Option None matches encode_to_vec length
#[test]
fn test_encoded_size_option_none_matches_vec() {
    let val: Option<u64> = None;
    let size = encoded_size(&val).expect("encoded_size Option None");
    let actual = encode_to_vec(&val).expect("encode Option None").len();
    assert_eq!(size, actual);
    assert_eq!(size, 1);
}

/// Test 17: encoded_size for enum tuple variant (Color::Custom) matches encode_to_vec length
#[test]
fn test_encoded_size_enum_tuple_variant_matches_vec() {
    let val = Color::Custom(255, 128, 0);
    let size = encoded_size(&val).expect("encoded_size Color::Custom");
    let actual = encode_to_vec(&val).expect("encode Color::Custom").len();
    assert_eq!(size, actual);
}

/// Test 18: encoded_size for a nested struct matches encode_to_vec length
#[test]
fn test_encoded_size_nested_struct_matches_vec() {
    let val = Nested {
        name: "oxicode".to_string(),
        values: vec![1u16, 2, 3, 4, 5],
        flag: true,
    };
    let size = encoded_size(&val).expect("encoded_size Nested");
    let actual = encode_to_vec(&val).expect("encode Nested").len();
    assert_eq!(size, actual);
}

/// Test 19: encoded_size for a generic wrapper struct matches encode_to_vec length
#[test]
fn test_encoded_size_generic_wrapper_matches_vec() {
    let val = Wrapper {
        inner: 42u32,
        tag: 7u8,
    };
    let size = encoded_size(&val).expect("encoded_size Wrapper");
    let actual = encode_to_vec(&val).expect("encode Wrapper").len();
    assert_eq!(size, actual);
}

/// Test 20: encoded_size for a tuple matches encode_to_vec length
#[test]
fn test_encoded_size_tuple_matches_vec() {
    let val = (1u8, 2u32, 3u64, true);
    let size = encoded_size(&val).expect("encoded_size tuple");
    let actual = encode_to_vec(&val).expect("encode tuple").len();
    assert_eq!(size, actual);
}

/// Test 21: encoded_size_with_config fixed-int for a struct is consistent with encode_to_vec_with_config
#[test]
fn test_encoded_size_with_config_fixed_struct_matches_vec() {
    let val = SimplePoint { x: 0, y: i32::MAX };
    let cfg = config::standard().with_fixed_int_encoding();
    let size = encoded_size_with_config(&val, cfg).expect("encoded_size fixed SimplePoint");
    let actual = encode_to_vec_with_config(&val, cfg)
        .expect("encode fixed SimplePoint")
        .len();
    assert_eq!(size, actual);
    // Two fixed i32 fields: 4 + 4 = 8 bytes
    assert_eq!(size, 8);
}

/// Test 22: encoded_size for Vec<String> matches encode_to_vec length and round-trips correctly
#[test]
fn test_encoded_size_vec_of_strings_matches_vec_and_roundtrips() {
    let val: Vec<String> = vec![
        "alpha".to_string(),
        "beta".to_string(),
        "gamma".to_string(),
        "delta".to_string(),
    ];
    let size = encoded_size(&val).expect("encoded_size Vec<String>");
    let bytes = encode_to_vec(&val).expect("encode Vec<String>");
    assert_eq!(size, bytes.len());
    // Verify round-trip correctness
    let (decoded, _): (Vec<String>, usize) = decode_from_slice(&bytes).expect("decode Vec<String>");
    assert_eq!(decoded, val);
}
