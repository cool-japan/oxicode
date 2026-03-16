//! Advanced string encoding tests for OxiCode — 22 unique test functions.

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
// Helper structs
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiString {
    first: String,
    second: String,
    third: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Inner {
    label: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Outer {
    tag: String,
    inner: Inner,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. Empty string roundtrip
#[test]
fn test_string_empty_roundtrip() {
    let val = String::new();
    let enc = encode_to_vec(&val).expect("encode empty string");
    let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode empty string");
    assert_eq!(val, dec);
}

/// 2. ASCII string roundtrip
#[test]
fn test_string_ascii_roundtrip() {
    let val = "The quick brown fox jumps over the lazy dog".to_string();
    let enc = encode_to_vec(&val).expect("encode ASCII string");
    let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode ASCII string");
    assert_eq!(val, dec);
}

/// 3. Unicode string with Japanese characters roundtrip
#[test]
fn test_string_japanese_roundtrip() {
    let val = "日本語テスト：漢字とひらがなとカタカナ".to_string();
    let enc = encode_to_vec(&val).expect("encode Japanese string");
    let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode Japanese string");
    assert_eq!(val, dec);
}

/// 4. Unicode string with emoji roundtrip
#[test]
fn test_string_emoji_roundtrip() {
    let val = "Hello 🦀🎉🌍✨🔥💯".to_string();
    let enc = encode_to_vec(&val).expect("encode emoji string");
    let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode emoji string");
    assert_eq!(val, dec);
}

/// 5. Very long string (10000 chars) roundtrip
#[test]
fn test_string_very_long_roundtrip() {
    let val: String = "あ".repeat(10_000);
    let enc = encode_to_vec(&val).expect("encode long string");
    let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode long string");
    assert_eq!(val, dec);
}

/// 6. String with null bytes roundtrip
#[test]
fn test_string_null_bytes_roundtrip() {
    let val = "before\0middle\0\0after".to_string();
    let enc = encode_to_vec(&val).expect("encode null-byte string");
    let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode null-byte string");
    assert_eq!(val, dec);
}

/// 7. String with all ASCII printable chars roundtrip
#[test]
fn test_string_all_ascii_printable_roundtrip() {
    let val: String = (0x20u8..=0x7e).map(|b| b as char).collect();
    let enc = encode_to_vec(&val).expect("encode all-printable-ASCII string");
    let (dec, _): (String, usize) =
        decode_from_slice(&enc).expect("decode all-printable-ASCII string");
    assert_eq!(val, dec);
}

/// 8. String with control chars (tabs and newlines) roundtrip
#[test]
fn test_string_control_chars_roundtrip() {
    let val = "line1\tcolumn\nline2\r\nline3\t\t".to_string();
    let enc = encode_to_vec(&val).expect("encode control-char string");
    let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode control-char string");
    assert_eq!(val, dec);
}

/// 9. Vec<String> with mixed lengths roundtrip
#[test]
fn test_vec_of_strings_roundtrip() {
    let val: Vec<String> = vec![
        String::new(),
        "short".to_string(),
        "a bit longer string here".to_string(),
        "日本語".to_string(),
        "🦀".repeat(100),
    ];
    let enc = encode_to_vec(&val).expect("encode Vec<String>");
    let (dec, _): (Vec<String>, usize) = decode_from_slice(&enc).expect("decode Vec<String>");
    assert_eq!(val, dec);
}

/// 10. Option<String> Some roundtrip
#[test]
fn test_option_string_some_roundtrip() {
    let val: Option<String> = Some("present value".to_string());
    let enc = encode_to_vec(&val).expect("encode Option<String> Some");
    let (dec, _): (Option<String>, usize) =
        decode_from_slice(&enc).expect("decode Option<String> Some");
    assert_eq!(val, dec);
}

/// 11. Option<String> None roundtrip
#[test]
fn test_option_string_none_roundtrip() {
    let val: Option<String> = None;
    let enc = encode_to_vec(&val).expect("encode Option<String> None");
    let (dec, _): (Option<String>, usize) =
        decode_from_slice(&enc).expect("decode Option<String> None");
    assert_eq!(val, dec);
}

/// 12. String encoding layout: first bytes are length prefix (varint)
#[test]
fn test_string_encoding_layout_varint_prefix() {
    // "hello" is 5 ASCII bytes; varint(5) == 0x05 (single byte)
    let val = "hello".to_string();
    let enc = encode_to_vec(&val).expect("encode hello");
    assert_eq!(
        enc[0], 5,
        "first byte of encoded 'hello' must be the varint length 5"
    );
    // remaining bytes must be the UTF-8 representation
    assert_eq!(&enc[1..], b"hello");
}

/// 13. Empty string encodes to exactly 1 byte (varint 0)
#[test]
fn test_string_empty_encodes_to_one_byte() {
    let val = String::new();
    let enc = encode_to_vec(&val).expect("encode empty string");
    assert_eq!(
        enc.len(),
        1,
        "empty string must encode to exactly 1 byte (varint 0)"
    );
    assert_eq!(enc[0], 0, "the single byte must be 0x00");
}

/// 14. String with big_endian config roundtrip (string data is unaffected by endianness)
#[test]
fn test_string_big_endian_config_roundtrip() {
    let val = "big-endian string".to_string();
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode with big_endian config");
    let (dec, _): (String, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode with big_endian config");
    assert_eq!(val, dec);
}

/// 15. String with fixed_int_encoding config roundtrip
#[test]
fn test_string_fixed_int_encoding_config_roundtrip() {
    let val = "fixed int encoding test".to_string();
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode with fixed_int_encoding config");
    let (dec, _): (String, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode with fixed_int_encoding config");
    assert_eq!(val, dec);
}

/// 16. Box<String> roundtrip
#[test]
fn test_box_string_roundtrip() {
    let val: Box<String> = Box::new("boxed string value".to_string());
    let enc = encode_to_vec(&val).expect("encode Box<String>");
    let (dec, _): (Box<String>, usize) = decode_from_slice(&enc).expect("decode Box<String>");
    assert_eq!(val, dec);
}

/// 17. Struct with multiple String fields roundtrip
#[test]
fn test_struct_multi_string_fields_roundtrip() {
    let val = MultiString {
        first: "alpha".to_string(),
        second: "beta ベータ".to_string(),
        third: "gamma 🎯".to_string(),
    };
    let enc = encode_to_vec(&val).expect("encode MultiString");
    let (dec, _): (MultiString, usize) = decode_from_slice(&enc).expect("decode MultiString");
    assert_eq!(val, dec);
}

/// 18. Nested struct with String in inner struct roundtrip
#[test]
fn test_nested_struct_string_roundtrip() {
    let val = Outer {
        tag: "outer-tag".to_string(),
        inner: Inner {
            label: "inner-label 内側".to_string(),
        },
    };
    let enc = encode_to_vec(&val).expect("encode Outer");
    let (dec, _): (Outer, usize) = decode_from_slice(&enc).expect("decode Outer");
    assert_eq!(val, dec);
}

/// 19. Vec<Option<String>> mixed roundtrip
#[test]
fn test_vec_option_string_mixed_roundtrip() {
    let val: Vec<Option<String>> = vec![
        Some("first".to_string()),
        None,
        Some(String::new()),
        None,
        Some("last 最後".to_string()),
    ];
    let enc = encode_to_vec(&val).expect("encode Vec<Option<String>>");
    let (dec, _): (Vec<Option<String>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Option<String>>");
    assert_eq!(val, dec);
}

/// 20. String containing valid but unusual UTF-8 sequences roundtrip
#[test]
fn test_string_unusual_utf8_roundtrip() {
    // Multi-byte sequences: U+00E9 (é), U+20AC (€), U+1F600 (😀), U+0000 (NUL)
    let val = "\u{00E9}\u{20AC}\u{1F600}\u{0000}\u{FEFF}end".to_string();
    let enc = encode_to_vec(&val).expect("encode unusual UTF-8 string");
    let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode unusual UTF-8 string");
    assert_eq!(val, dec);
}

/// 21. Bytes consumed matches encoded string byte count + length prefix size
#[test]
fn test_string_bytes_consumed_matches_encoding() {
    let val = "measurement".to_string(); // 11 ASCII bytes; varint(11) fits in 1 byte
    let enc = encode_to_vec(&val).expect("encode measurement string");
    let (_, consumed): (String, usize) =
        decode_from_slice(&enc).expect("decode measurement string");
    // The total encoded length must equal the number of bytes consumed during decoding.
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal full encoded length"
    );
    // varint(11) == 1 byte prefix + 11 payload bytes == 12 bytes total
    assert_eq!(
        enc.len(),
        12,
        "encoded length must be 1 (varint) + 11 (UTF-8)"
    );
}

/// 22. Two different strings produce different encodings
#[test]
fn test_different_strings_produce_different_encodings() {
    let a = "string_alpha".to_string();
    let b = "string_ALPHA".to_string();
    let enc_a = encode_to_vec(&a).expect("encode string_alpha");
    let enc_b = encode_to_vec(&b).expect("encode string_ALPHA");
    assert_ne!(
        enc_a, enc_b,
        "distinct strings must produce distinct byte sequences"
    );
}
