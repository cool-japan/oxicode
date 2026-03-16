//! Advanced tests for String and &str encoding edge cases in OxiCode.

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

/// Test 1: Empty &str "" roundtrip - bytes should be [0x00]
#[test]
fn test_empty_str_roundtrip_byte_repr() {
    let s: &str = "";
    let enc = encode_to_vec(&s).expect("encode empty &str");
    assert_eq!(
        enc,
        vec![0x00],
        "empty &str must encode as single 0x00 varint"
    );
    let (dec, bytes_read): (String, _) = decode_from_slice(&enc).expect("decode empty &str");
    assert_eq!(dec, "", "decoded value must be empty string");
    assert_eq!(bytes_read, 1, "must consume exactly 1 byte");
}

/// Test 2: Single ASCII char "a" roundtrip - bytes [0x01, 0x61]
#[test]
fn test_single_ascii_char_roundtrip() {
    let s: &str = "a";
    let enc = encode_to_vec(&s).expect("encode 'a'");
    assert_eq!(
        enc,
        vec![0x01, 0x61],
        "single 'a' must encode as [0x01, 0x61]"
    );
    let (dec, bytes_read): (String, _) = decode_from_slice(&enc).expect("decode 'a'");
    assert_eq!(dec, "a", "decoded value must be 'a'");
    assert_eq!(bytes_read, 2, "must consume exactly 2 bytes");
}

/// Test 3: Two ASCII chars "hi" roundtrip - bytes [0x02, 0x68, 0x69]
#[test]
fn test_two_ascii_chars_roundtrip() {
    let s: &str = "hi";
    let enc = encode_to_vec(&s).expect("encode 'hi'");
    assert_eq!(
        enc,
        vec![0x02, 0x68, 0x69],
        "'hi' must encode as [0x02, 0x68, 0x69]"
    );
    let (dec, bytes_read): (String, _) = decode_from_slice(&enc).expect("decode 'hi'");
    assert_eq!(dec, "hi", "decoded value must be 'hi'");
    assert_eq!(bytes_read, 3, "must consume exactly 3 bytes");
}

/// Test 4: 250-char string roundtrip - length varint = 1 byte (0xFA)
#[test]
fn test_250_char_string_varint_one_byte() {
    let s: String = "x".repeat(250);
    let enc = encode_to_vec(&s).expect("encode 250-char string");
    // varint for 250 is 0xFA (fits in 1 byte since 250 < 251 threshold)
    assert_eq!(enc[0], 0xFA, "varint for 250 must be 0xFA as single byte");
    assert_eq!(
        enc.len(),
        251,
        "total encoded length must be 251 (1 varint + 250 bytes)"
    );
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode 250-char string");
    assert_eq!(dec, s, "decoded value must match original 250-char string");
}

/// Test 5: 251-char string roundtrip - length varint = 3 bytes (0xFB prefix)
#[test]
fn test_251_char_string_varint_three_bytes() {
    let s: String = "y".repeat(251);
    let enc = encode_to_vec(&s).expect("encode 251-char string");
    // varint for 251+ uses 0xFB prefix + 2 more bytes
    assert_eq!(enc[0], 0xFB, "varint for 251 must start with 0xFB prefix");
    assert_eq!(
        enc.len(),
        254,
        "total encoded length must be 254 (3 varint bytes + 251 bytes)"
    );
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode 251-char string");
    assert_eq!(dec, s, "decoded value must match original 251-char string");
}

/// Test 6: String with all printable ASCII roundtrip
#[test]
fn test_all_printable_ascii_roundtrip() {
    let s: String = (0x20u8..=0x7Eu8).map(|b| b as char).collect();
    assert_eq!(
        s.len(),
        95,
        "printable ASCII range 0x20..=0x7E has 95 characters"
    );
    let enc = encode_to_vec(&s).expect("encode all printable ASCII");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode all printable ASCII");
    assert_eq!(
        dec, s,
        "decoded value must match all printable ASCII string"
    );
}

/// Test 7: String with null byte \0 (encode as UTF-8, null is valid) roundtrip
#[test]
fn test_string_with_null_byte_roundtrip() {
    let s = "before\0after".to_string();
    assert_eq!(
        s.len(),
        12,
        "string with embedded null must have correct byte length"
    );
    let enc = encode_to_vec(&s).expect("encode string with null byte");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode string with null byte");
    assert_eq!(dec, s, "decoded value must preserve embedded null byte");
    assert_eq!(dec.as_bytes()[6], 0x00, "null byte must be at position 6");
}

/// Test 8: String "hello world" has expected byte count (2+11=13 bytes: varint(11) + UTF-8)
#[test]
fn test_hello_world_byte_count() {
    let s: &str = "hello world";
    let enc = encode_to_vec(&s).expect("encode 'hello world'");
    // varint(11) = 1 byte (0x0B), UTF-8 bytes = 11 bytes, total = 12 bytes
    assert_eq!(
        enc.len(),
        12,
        "'hello world' must encode to 12 bytes (1 varint + 11 utf8)"
    );
    assert_eq!(enc[0], 0x0B, "varint for 11 must be 0x0B");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode 'hello world'");
    assert_eq!(dec, "hello world", "decoded value must be 'hello world'");
}

/// Test 9: Unicode 2-byte: "é" (U+00E9) roundtrip - UTF-8 is [0xC3, 0xA9], so bytes = [0x02, 0xC3, 0xA9]
#[test]
fn test_unicode_2byte_e_acute_roundtrip() {
    let s: &str = "é";
    assert_eq!(s.len(), 2, "'é' must be 2 UTF-8 bytes");
    let enc = encode_to_vec(&s).expect("encode 'é'");
    assert_eq!(
        enc,
        vec![0x02, 0xC3, 0xA9],
        "'é' must encode as [0x02, 0xC3, 0xA9]"
    );
    assert_eq!(enc.len(), 3, "total encoded length of 'é' must be 3 bytes");
    let (dec, bytes_read): (String, _) = decode_from_slice(&enc).expect("decode 'é'");
    assert_eq!(dec, "é", "decoded value must be 'é'");
    assert_eq!(bytes_read, 3, "must consume exactly 3 bytes for 'é'");
}

/// Test 10: Unicode 3-byte: "中" (U+4E2D) roundtrip - UTF-8 is [0xE4, 0xB8, 0xAD], bytes = [0x03, 0xE4, 0xB8, 0xAD]
#[test]
fn test_unicode_3byte_cjk_roundtrip() {
    let s: &str = "中";
    assert_eq!(s.len(), 3, "'中' must be 3 UTF-8 bytes");
    let enc = encode_to_vec(&s).expect("encode '中'");
    assert_eq!(
        enc,
        vec![0x03, 0xE4, 0xB8, 0xAD],
        "'中' must encode as [0x03, 0xE4, 0xB8, 0xAD]"
    );
    assert_eq!(enc.len(), 4, "total encoded length of '中' must be 4 bytes");
    let (dec, bytes_read): (String, _) = decode_from_slice(&enc).expect("decode '中'");
    assert_eq!(dec, "中", "decoded value must be '中'");
    assert_eq!(bytes_read, 4, "must consume exactly 4 bytes for '中'");
}

/// Test 11: Unicode 4-byte emoji "🦀" (U+1F980) roundtrip
#[test]
fn test_unicode_4byte_emoji_crab_roundtrip() {
    let s: &str = "🦀";
    assert_eq!(s.len(), 4, "'🦀' must be 4 UTF-8 bytes");
    let utf8_bytes = s.as_bytes();
    assert_eq!(
        utf8_bytes,
        &[0xF0, 0x9F, 0xA6, 0x80],
        "'🦀' UTF-8 must be [0xF0, 0x9F, 0xA6, 0x80]"
    );
    let enc = encode_to_vec(&s).expect("encode '🦀'");
    assert_eq!(
        enc.len(),
        5,
        "total encoded length of '🦀' must be 5 bytes (1 varint + 4 utf8)"
    );
    assert_eq!(enc[0], 0x04, "varint for 4 must be 0x04");
    let (dec, bytes_read): (String, _) = decode_from_slice(&enc).expect("decode '🦀'");
    assert_eq!(dec, "🦀", "decoded value must be '🦀'");
    assert_eq!(bytes_read, 5, "must consume exactly 5 bytes for '🦀'");
}

/// Test 12: Mixed ASCII+Unicode string roundtrip
#[test]
fn test_mixed_ascii_unicode_roundtrip() {
    let s = "Hello, 世界! αβγ 🦀 ñ ü café".to_string();
    let enc = encode_to_vec(&s).expect("encode mixed ASCII+Unicode string");
    let (dec, bytes_read): (String, _) =
        decode_from_slice(&enc).expect("decode mixed ASCII+Unicode string");
    assert_eq!(dec, s, "decoded value must match original mixed string");
    assert_eq!(
        bytes_read,
        enc.len(),
        "bytes_read must equal total encoded length"
    );
}

/// Test 13: Very long string (10000 chars) roundtrip
#[test]
fn test_very_long_string_10000_chars() {
    let s: String = "abcdefghij".repeat(1000);
    assert_eq!(s.len(), 10000, "string must be exactly 10000 bytes");
    let enc = encode_to_vec(&s).expect("encode 10000-char string");
    // 10000 requires multi-byte varint encoding
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode 10000-char string");
    assert_eq!(
        dec, s,
        "decoded value must match original 10000-char string"
    );
    assert_eq!(
        dec.len(),
        10000,
        "decoded string must have exactly 10000 chars"
    );
}

/// Test 14: String::from("owned string") roundtrip
#[test]
fn test_owned_string_from_roundtrip() {
    let s = String::from("owned string");
    let enc = encode_to_vec(&s).expect("encode owned String");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode owned String");
    assert_eq!(dec, s, "decoded value must match original owned String");
    assert_eq!(
        dec, "owned string",
        "decoded value must equal 'owned string'"
    );
}

/// Test 15: &str "borrowed" roundtrip with decoded String equality
#[test]
fn test_borrowed_str_decode_as_string_equality() {
    let s: &str = "borrowed";
    let enc = encode_to_vec(&s).expect("encode borrowed &str");
    let enc_owned = encode_to_vec(&s.to_string()).expect("encode owned String from &str");
    assert_eq!(
        enc, enc_owned,
        "&str and String::from(&str) must produce identical encoding"
    );
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode borrowed &str as String");
    assert_eq!(dec.as_str(), s, "decoded String must equal original &str");
}

/// Test 16: Vec<String> roundtrip
#[test]
fn test_vec_of_strings_roundtrip() {
    let v: Vec<String> = vec![
        "first".to_string(),
        "second".to_string(),
        "".to_string(),
        "中文".to_string(),
        "🦀🦀🦀".to_string(),
    ];
    let enc = encode_to_vec(&v).expect("encode Vec<String>");
    let (dec, _): (Vec<String>, _) = decode_from_slice(&enc).expect("decode Vec<String>");
    assert_eq!(dec, v, "decoded Vec<String> must match original");
    assert_eq!(dec.len(), 5, "decoded vec must have 5 elements");
}

/// Test 17: Option<String> Some and None roundtrip
#[test]
fn test_option_string_some_and_none_roundtrip() {
    let some_val: Option<String> = Some("optional content".to_string());
    let none_val: Option<String> = None;

    let enc_some = encode_to_vec(&some_val).expect("encode Option<String> Some");
    let (dec_some, _): (Option<String>, _) =
        decode_from_slice(&enc_some).expect("decode Option<String> Some");
    assert_eq!(
        dec_some, some_val,
        "decoded Some(String) must match original"
    );

    let enc_none = encode_to_vec(&none_val).expect("encode Option<String> None");
    let (dec_none, _): (Option<String>, _) =
        decode_from_slice(&enc_none).expect("decode Option<String> None");
    assert_eq!(dec_none, none_val, "decoded None must remain None");
    assert!(dec_none.is_none(), "decoded None must be None variant");

    // None must encode more compactly than Some
    assert!(
        enc_none.len() < enc_some.len(),
        "None encoding must be shorter than Some encoding"
    );
}

/// Test 18: Tuple (String, u32) roundtrip
#[test]
fn test_tuple_string_u32_roundtrip() {
    let t: (String, u32) = ("hello".to_string(), 42u32);
    let enc = encode_to_vec(&t).expect("encode (String, u32)");
    let (dec, _): ((String, u32), _) = decode_from_slice(&enc).expect("decode (String, u32)");
    assert_eq!(
        dec.0, "hello",
        "decoded tuple first element must be 'hello'"
    );
    assert_eq!(dec.1, 42u32, "decoded tuple second element must be 42");
    assert_eq!(dec, t, "decoded tuple must match original");
}

/// Test 19: Newline, tab, CR characters in string roundtrip
#[test]
fn test_control_chars_newline_tab_cr_roundtrip() {
    let s = "line1\nline2\ttabbed\r\nwindows".to_string();
    let enc = encode_to_vec(&s).expect("encode string with control chars");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode string with control chars");
    assert_eq!(
        dec, s,
        "decoded value must preserve newline, tab, and CR characters"
    );
    assert!(dec.contains('\n'), "decoded string must contain newline");
    assert!(dec.contains('\t'), "decoded string must contain tab");
    assert!(
        dec.contains('\r'),
        "decoded string must contain carriage return"
    );
}

/// Test 20: String with all bytes 0x01-0x7E (valid ASCII subset) roundtrip
#[test]
fn test_ascii_0x01_to_0x7e_roundtrip() {
    let s: String = (0x01u8..=0x7Eu8).map(|b| b as char).collect();
    assert_eq!(s.len(), 126, "ASCII range 0x01..=0x7E has 126 bytes");
    // Verify it's valid UTF-8 (single-byte ASCII range is always valid)
    assert!(
        std::str::from_utf8(s.as_bytes()).is_ok(),
        "bytes 0x01..=0x7E must form valid UTF-8"
    );
    let enc = encode_to_vec(&s).expect("encode ASCII 0x01-0x7E string");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode ASCII 0x01-0x7E string");
    assert_eq!(dec, s, "decoded value must match ASCII 0x01-0x7E string");
    assert_eq!(dec.len(), 126, "decoded string must have 126 bytes");
}

/// Test 21: Multiple strings encoded sequentially decode correctly
#[test]
fn test_multiple_strings_sequential_decode() {
    let s1 = "first".to_string();
    let s2 = "second".to_string();
    let s3 = "third".to_string();

    let mut buf = Vec::new();
    buf.extend_from_slice(&encode_to_vec(&s1).expect("encode s1"));
    buf.extend_from_slice(&encode_to_vec(&s2).expect("encode s2"));
    buf.extend_from_slice(&encode_to_vec(&s3).expect("encode s3"));

    let (dec1, n1): (String, _) = decode_from_slice(&buf).expect("decode s1 from sequential buf");
    assert_eq!(dec1, "first", "first decoded string must be 'first'");

    let (dec2, n2): (String, _) =
        decode_from_slice(&buf[n1..]).expect("decode s2 from sequential buf");
    assert_eq!(dec2, "second", "second decoded string must be 'second'");

    let (dec3, n3): (String, _) =
        decode_from_slice(&buf[n1 + n2..]).expect("decode s3 from sequential buf");
    assert_eq!(dec3, "third", "third decoded string must be 'third'");

    assert_eq!(
        n1 + n2 + n3,
        buf.len(),
        "sum of all byte offsets must equal total buffer length"
    );
}

/// Test 22: Empty String::new() roundtrip
#[test]
fn test_empty_string_new_roundtrip() {
    let s = String::new();
    assert_eq!(s.len(), 0, "String::new() must have length 0");
    let enc = encode_to_vec(&s).expect("encode String::new()");
    assert_eq!(
        enc,
        vec![0x00],
        "empty String::new() must encode as single 0x00 byte"
    );
    assert_eq!(enc.len(), 1, "encoded empty String must be exactly 1 byte");
    let (dec, bytes_read): (String, _) =
        decode_from_slice(&enc).expect("decode String::new() from [0x00]");
    assert_eq!(dec, String::new(), "decoded value must equal String::new()");
    assert_eq!(dec.len(), 0, "decoded string must have length 0");
    assert_eq!(
        bytes_read, 1,
        "must consume exactly 1 byte for empty String"
    );
}
