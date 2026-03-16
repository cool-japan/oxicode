//! Advanced string and unicode encoding tests for OxiCode (set 2).

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

/// Test 1: Empty string roundtrip (len=0)
#[test]
fn test_empty_string_roundtrip_len0() {
    let original = String::from("");
    let encoded = encode_to_vec(&original).expect("encode empty string");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode empty string");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 0, "decoded string must have length 0");
}

/// Test 2: ASCII string roundtrip "Hello, World!"
#[test]
fn test_ascii_hello_world_roundtrip() {
    let original = String::from("Hello, World!");
    let encoded = encode_to_vec(&original).expect("encode Hello, World!");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode Hello, World!");
    assert_eq!(original, decoded);
    assert_eq!(decoded, "Hello, World!");
}

/// Test 3: String with spaces roundtrip
#[test]
fn test_string_with_spaces_roundtrip() {
    let original = String::from("  hello   world  ");
    let encoded = encode_to_vec(&original).expect("encode string with spaces");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode string with spaces");
    assert_eq!(original, decoded);
    assert!(
        decoded.contains("  "),
        "decoded string must preserve multiple spaces"
    );
}

/// Test 4: Japanese string roundtrip "こんにちは世界"
#[test]
fn test_japanese_string_roundtrip() {
    let original = String::from("こんにちは世界");
    let encoded = encode_to_vec(&original).expect("encode japanese string");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode japanese string");
    assert_eq!(original, decoded);
    assert_eq!(decoded.chars().count(), 7, "japanese string has 7 chars");
}

/// Test 5: Chinese string roundtrip "你好，世界"
#[test]
fn test_chinese_string_roundtrip() {
    let original = String::from("你好，世界");
    let encoded = encode_to_vec(&original).expect("encode chinese string");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode chinese string");
    assert_eq!(original, decoded);
}

/// Test 6: Arabic string roundtrip "مرحبا بالعالم"
#[test]
fn test_arabic_string_roundtrip() {
    let original = String::from("مرحبا بالعالم");
    let encoded = encode_to_vec(&original).expect("encode arabic string");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode arabic string");
    assert_eq!(original, decoded);
}

/// Test 7: Emoji string roundtrip "🦀🔥💯"
#[test]
fn test_emoji_string_roundtrip() {
    let original = String::from("🦀🔥💯");
    let encoded = encode_to_vec(&original).expect("encode emoji string");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode emoji string");
    assert_eq!(original, decoded);
    assert_eq!(decoded.chars().count(), 3, "emoji string has 3 chars");
}

/// Test 8: String with control character \u{0001} roundtrip
#[test]
fn test_string_with_control_char_u0001_roundtrip() {
    let original = String::from("before\u{0001}after");
    let encoded = encode_to_vec(&original).expect("encode string with \\u{0001}");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode string with \\u{0001}");
    assert_eq!(original, decoded);
    assert!(
        decoded.contains('\u{0001}'),
        "decoded string must contain U+0001"
    );
}

/// Test 9: Long string 1000 'a' chars roundtrip
#[test]
fn test_long_string_1000_a_chars_roundtrip() {
    let original: String = "a".repeat(1000);
    assert_eq!(original.len(), 1000, "string must be exactly 1000 bytes");
    let encoded = encode_to_vec(&original).expect("encode 1000-char string");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode 1000-char string");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 1000, "decoded string must be 1000 bytes");
}

/// Test 10: String with newlines and tabs roundtrip
#[test]
fn test_string_with_newlines_and_tabs_roundtrip() {
    let original = String::from("line1\nline2\ttabbed\r\nwindows\r");
    let encoded = encode_to_vec(&original).expect("encode string with newlines and tabs");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode string with newlines and tabs");
    assert_eq!(original, decoded);
    assert!(
        decoded.contains('\n'),
        "decoded string must contain newline"
    );
    assert!(decoded.contains('\t'), "decoded string must contain tab");
}

/// Test 11: String with backslash roundtrip
#[test]
fn test_string_with_backslash_roundtrip() {
    let original = String::from("path\\to\\file\\name");
    let encoded = encode_to_vec(&original).expect("encode string with backslash");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode string with backslash");
    assert_eq!(original, decoded);
    assert!(
        decoded.contains('\\'),
        "decoded string must contain backslash"
    );
}

/// Test 12: String with quotes roundtrip
#[test]
fn test_string_with_quotes_roundtrip() {
    let original = String::from("she said \"hello\" and 'goodbye'");
    let encoded = encode_to_vec(&original).expect("encode string with quotes");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode string with quotes");
    assert_eq!(original, decoded);
    assert!(
        decoded.contains('"'),
        "decoded string must contain double quote"
    );
    assert!(
        decoded.contains('\''),
        "decoded string must contain single quote"
    );
}

/// Test 13: Unicode BMP boundary (U+FFFF) roundtrip
#[test]
fn test_unicode_bmp_boundary_u_ffff_roundtrip() {
    let original = String::from("\u{FFFF}");
    assert_eq!(original.len(), 3, "U+FFFF is 3 UTF-8 bytes");
    let encoded = encode_to_vec(&original).expect("encode U+FFFF string");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode U+FFFF string");
    assert_eq!(original, decoded);
    assert_eq!(
        decoded.chars().next().expect("must have char"),
        '\u{FFFF}',
        "decoded char must be U+FFFF"
    );
}

/// Test 14: Unicode supplementary plane char roundtrip ("😀" U+1F600)
#[test]
fn test_unicode_supplementary_plane_u1f600_roundtrip() {
    let original = String::from("😀");
    assert_eq!(original.len(), 4, "U+1F600 is 4 UTF-8 bytes");
    let encoded = encode_to_vec(&original).expect("encode U+1F600 emoji");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode U+1F600 emoji");
    assert_eq!(original, decoded);
    assert_eq!(
        decoded.chars().count(),
        1,
        "supplementary plane char is 1 char"
    );
}

/// Test 15: Mixed script string roundtrip
#[test]
fn test_mixed_script_string_roundtrip() {
    let original = String::from("Hello, 世界! αβγ 🦀 مرحبا ñ café");
    let encoded = encode_to_vec(&original).expect("encode mixed script string");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode mixed script string");
    assert_eq!(original, decoded);
}

/// Test 16: String consumed bytes equals encoded len
#[test]
fn test_string_consumed_bytes_equals_encoded_len() {
    let original = String::from("Unicode: こんにちは 🦀");
    let encoded = encode_to_vec(&original).expect("encode for consumed check");
    let (_decoded, consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode for consumed check");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal total encoded length"
    );
}

/// Test 17: Empty string encodes to exactly 1 byte (varint 0 length)
#[test]
fn test_empty_string_encodes_to_exactly_1_byte() {
    let original = String::from("");
    let encoded = encode_to_vec(&original).expect("encode empty string for byte check");
    assert_eq!(
        encoded.len(),
        1,
        "empty string must encode to exactly 1 byte"
    );
    assert_eq!(encoded[0], 0x00, "empty string varint length must be 0x00");
}

/// Test 18: String encoding starts with varint of UTF-8 byte length
#[test]
fn test_string_encoding_starts_with_varint_utf8_byte_length() {
    let s = "abc";
    let enc = encode_to_vec(&s.to_string()).expect("encode");
    // First byte should be 3 (length of "abc")
    assert_eq!(enc[0], 3, "length prefix should be 3");
    assert_eq!(&enc[1..], b"abc");
}

/// Test 19: Vec<String> with unicode strings roundtrip
#[test]
fn test_vec_string_with_unicode_roundtrip() {
    let original: Vec<String> = vec![
        String::from("日本語"),
        String::from("한국어"),
        String::from("中文"),
        String::from("Ελληνικά"),
        String::from("🦀 Rust"),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<String> with unicode");
    let (decoded, _consumed): (Vec<String>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<String> with unicode");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 5, "decoded vec must have 5 elements");
}

/// Test 20: Option<String> Some with unicode roundtrip
#[test]
fn test_option_string_some_with_unicode_roundtrip() {
    let some_val: Option<String> = Some(String::from("こんにちは🌸"));
    let encoded_some = encode_to_vec(&some_val).expect("encode Some unicode string");
    let (decoded_some, _consumed): (Option<String>, usize) =
        decode_from_slice(&encoded_some).expect("decode Some unicode string");
    assert_eq!(some_val, decoded_some);

    let none_val: Option<String> = None;
    let encoded_none = encode_to_vec(&none_val).expect("encode None string");
    let (decoded_none, _consumed): (Option<String>, usize) =
        decode_from_slice(&encoded_none).expect("decode None string");
    assert_eq!(none_val, decoded_none);
    assert!(decoded_none.is_none(), "decoded None must be None variant");
}

/// Test 21: String with all ASCII printable chars roundtrip
#[test]
fn test_all_ascii_printable_chars_roundtrip() {
    let original: String = (0x20u8..=0x7eu8).map(|b| b as char).collect();
    assert_eq!(original.len(), 95, "printable ASCII has 95 characters");
    let encoded = encode_to_vec(&original).expect("encode all printable ASCII");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode all printable ASCII");
    assert_eq!(original, decoded);
}

/// Test 22: Fixed-length strings comparison: longer string encodes to more bytes
#[test]
fn test_longer_string_encodes_to_more_bytes() {
    let short = String::from("hi");
    let long = String::from("hello world");
    let cfg = config::standard();

    let enc_short = encode_to_vec_with_config(&short, cfg).expect("encode short string");
    let enc_long = encode_to_vec_with_config(&long, cfg).expect("encode long string");

    assert!(
        enc_long.len() > enc_short.len(),
        "longer string ({} bytes) must encode to more bytes than shorter string ({} bytes)",
        enc_long.len(),
        enc_short.len()
    );

    // Verify both roundtrip correctly
    let (dec_short, _): (String, usize) =
        decode_from_slice_with_config(&enc_short, cfg).expect("decode short string");
    let (dec_long, _): (String, usize) =
        decode_from_slice_with_config(&enc_long, cfg).expect("decode long string");
    assert_eq!(dec_short, short);
    assert_eq!(dec_long, long);
}
