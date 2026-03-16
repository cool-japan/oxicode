//! Advanced tests for string slice and &str encoding in OxiCode.
//! All tests use String for roundtrip (owned type); wire format is varint length + UTF-8 bytes.

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
use std::collections::BTreeMap;

// Test 1: Empty string roundtrip
#[test]
fn test_empty_string_roundtrip() {
    let s = String::new();
    let enc = encode_to_vec(&s).expect("encode empty string");
    let (val, _bytes): (String, usize) = decode_from_slice(&enc).expect("decode empty string");
    assert_eq!(s, val);
}

// Test 2: ASCII-only string roundtrip
#[test]
fn test_ascii_string_roundtrip() {
    let s = String::from("Hello, World! 0123456789 abcdefghijklmnopqrstuvwxyz");
    let enc = encode_to_vec(&s).expect("encode ascii string");
    let (val, _bytes): (String, usize) = decode_from_slice(&enc).expect("decode ascii string");
    assert_eq!(s, val);
}

// Test 3: Multi-byte UTF-8 (Japanese chars) roundtrip
#[test]
fn test_multibyte_utf8_japanese_roundtrip() {
    let s = String::from("日本語テスト：こんにちは世界");
    let enc = encode_to_vec(&s).expect("encode japanese string");
    let (val, _bytes): (String, usize) = decode_from_slice(&enc).expect("decode japanese string");
    assert_eq!(s, val);
}

// Test 4: String with spaces roundtrip
#[test]
fn test_string_with_spaces_roundtrip() {
    let s = String::from("  leading and trailing spaces   and   multiple   internal   spaces  ");
    let enc = encode_to_vec(&s).expect("encode string with spaces");
    let (val, _bytes): (String, usize) =
        decode_from_slice(&enc).expect("decode string with spaces");
    assert_eq!(s, val);
}

// Test 5: String with newlines roundtrip
#[test]
fn test_string_with_newlines_roundtrip() {
    let s = String::from("line one\nline two\r\nline three\n");
    let enc = encode_to_vec(&s).expect("encode string with newlines");
    let (val, _bytes): (String, usize) =
        decode_from_slice(&enc).expect("decode string with newlines");
    assert_eq!(s, val);
}

// Test 6: String with special chars (tab/newline) roundtrip
#[test]
fn test_string_with_special_chars_roundtrip() {
    let s = String::from("tab\there\nnewline\r\nand\tmore\ttabs");
    let enc = encode_to_vec(&s).expect("encode string with special chars");
    let (val, _bytes): (String, usize) =
        decode_from_slice(&enc).expect("decode string with special chars");
    assert_eq!(s, val);
}

// Test 7: Very long string (1000 chars) roundtrip
#[test]
fn test_very_long_string_roundtrip() {
    let s = "abcdefghij".repeat(100); // 1000 chars
    assert_eq!(s.len(), 1000);
    let enc = encode_to_vec(&s).expect("encode long string");
    let (val, _bytes): (String, usize) = decode_from_slice(&enc).expect("decode long string");
    assert_eq!(s, val);
}

// Test 8: Bytes consumed equals encoded length
#[test]
fn test_string_consumed_bytes_equals_encoded_length() {
    let s = String::from("measure me");
    let enc = encode_to_vec(&s).expect("encode string");
    let (_val, bytes_consumed): (String, usize) = decode_from_slice(&enc).expect("decode string");
    assert_eq!(bytes_consumed, enc.len());
}

// Test 9: Wire format: first byte is varint length, then UTF-8 bytes
#[test]
fn test_string_wire_format_varint_length_prefix() {
    let s = String::from("abc");
    let enc = encode_to_vec(&s).expect("encode string abc");
    // varint for 3 is just 0x03; followed by b'a', b'b', b'c'
    assert_eq!(enc[0], 3u8, "first byte should be varint length 3");
    assert_eq!(&enc[1..], b"abc", "remaining bytes should be UTF-8 content");
}

// Test 10: String "hello" encodes to [5, b'h', b'e', b'l', b'l', b'o']
#[test]
fn test_hello_wire_format() {
    let s = String::from("hello");
    let enc = encode_to_vec(&s).expect("encode hello");
    let expected: &[u8] = &[5, b'h', b'e', b'l', b'l', b'o'];
    assert_eq!(enc.as_slice(), expected);
}

// Test 11: Vec<String> roundtrip
#[test]
fn test_vec_of_strings_roundtrip() {
    let v: Vec<String> = vec![
        String::from("first"),
        String::from("second"),
        String::from("third"),
        String::new(),
        String::from("日本語"),
    ];
    let enc = encode_to_vec(&v).expect("encode Vec<String>");
    let (val, _bytes): (Vec<String>, usize) = decode_from_slice(&enc).expect("decode Vec<String>");
    assert_eq!(v, val);
}

// Test 12: Option<String> Some roundtrip
#[test]
fn test_option_string_some_roundtrip() {
    let opt: Option<String> = Some(String::from("present"));
    let enc = encode_to_vec(&opt).expect("encode Option<String> Some");
    let (val, _bytes): (Option<String>, usize) =
        decode_from_slice(&enc).expect("decode Option<String> Some");
    assert_eq!(opt, val);
}

// Test 13: Option<String> None roundtrip
#[test]
fn test_option_string_none_roundtrip() {
    let opt: Option<String> = None;
    let enc = encode_to_vec(&opt).expect("encode Option<String> None");
    let (val, _bytes): (Option<String>, usize) =
        decode_from_slice(&enc).expect("decode Option<String> None");
    assert_eq!(opt, val);
}

// Test 14: String with fixed-int config roundtrip
#[test]
fn test_string_with_fixed_int_config_roundtrip() {
    let s = String::from("fixed int config test");
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&s, cfg).expect("encode with fixed-int config");
    let (val, _bytes): (String, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode with fixed-int config");
    assert_eq!(s, val);
}

// Test 15: String with big-endian config roundtrip
#[test]
fn test_string_with_big_endian_config_roundtrip() {
    let s = String::from("big endian config test");
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&s, cfg).expect("encode with big-endian config");
    let (val, _bytes): (String, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode with big-endian config");
    assert_eq!(s, val);
}

// Test 16: Two strings encode to different bytes if content differs
#[test]
fn test_different_strings_encode_differently() {
    let s1 = String::from("alpha");
    let s2 = String::from("beta");
    let enc1 = encode_to_vec(&s1).expect("encode s1");
    let enc2 = encode_to_vec(&s2).expect("encode s2");
    assert_ne!(
        enc1, enc2,
        "different strings must produce different encodings"
    );
}

// Test 17: Struct { name: String, value: String } roundtrip
#[test]
fn test_struct_with_two_string_fields_roundtrip() {
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct NameValue {
        name: String,
        value: String,
    }

    let nv = NameValue {
        name: String::from("temperature"),
        value: String::from("42.0 celsius"),
    };
    let enc = encode_to_vec(&nv).expect("encode NameValue");
    let (val, _bytes): (NameValue, usize) = decode_from_slice(&enc).expect("decode NameValue");
    assert_eq!(nv, val);
}

// Test 18: String equality preserved after roundtrip
#[test]
fn test_string_equality_preserved_after_roundtrip() {
    let s = String::from("equality check: αβγδεζηθ");
    let enc = encode_to_vec(&s).expect("encode");
    let (val, _bytes): (String, usize) = decode_from_slice(&enc).expect("decode");
    assert!(
        s == val,
        "string equality must be preserved after roundtrip"
    );
}

// Test 19: String len preserved after roundtrip
#[test]
fn test_string_len_preserved_after_roundtrip() {
    let s = String::from("length matters: こんにちは");
    let original_len = s.len();
    let enc = encode_to_vec(&s).expect("encode");
    let (val, _bytes): (String, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(
        original_len,
        val.len(),
        "byte length must match after roundtrip"
    );
    assert_eq!(
        s.chars().count(),
        val.chars().count(),
        "char count must match"
    );
}

// Test 20: BTreeMap<String, String> roundtrip
#[test]
fn test_btreemap_string_string_roundtrip() {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    map.insert(String::from("key_one"), String::from("value_one"));
    map.insert(String::from("key_two"), String::from("value_two"));
    map.insert(String::from("empty_val"), String::new());
    map.insert(
        String::from("unicode_key_日本"),
        String::from("unicode_val_世界"),
    );

    let enc = encode_to_vec(&map).expect("encode BTreeMap<String,String>");
    let (val, _bytes): (BTreeMap<String, String>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap<String,String>");
    assert_eq!(map, val);
}

// Test 21: Vec<Option<String>> roundtrip
#[test]
fn test_vec_of_option_string_roundtrip() {
    let v: Vec<Option<String>> = vec![
        Some(String::from("first")),
        None,
        Some(String::from("third")),
        Some(String::new()),
        None,
        Some(String::from("emoji 🎉")),
    ];
    let enc = encode_to_vec(&v).expect("encode Vec<Option<String>>");
    let (val, _bytes): (Vec<Option<String>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Option<String>>");
    assert_eq!(v, val);
}

// Test 22: Long unicode string (emoji) roundtrip
#[test]
fn test_long_unicode_emoji_string_roundtrip() {
    // Each emoji is 4 bytes in UTF-8; repeat to get a long string
    let s = "🦀🌍🎉🔥💡🚀🎨🌟".repeat(50);
    assert!(s.len() > 100, "should be a long byte sequence");
    let enc = encode_to_vec(&s).expect("encode emoji string");
    let (val, _bytes): (String, usize) = decode_from_slice(&enc).expect("decode emoji string");
    assert_eq!(s, val);
}
