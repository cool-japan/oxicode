//! Advanced tests for char encoding in OxiCode.

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
use oxicode::{decode_from_slice, encode_to_vec, encoded_size};

// Test 1: 'A' (0x41) roundtrip, byte = [0x41], 1 byte
#[test]
fn test_char_ascii_a_roundtrip_one_byte() {
    let c = 'A';
    let enc = encode_to_vec(&c).expect("encode 'A'");
    assert_eq!(enc.len(), 1, "'A' should encode as 1 byte");
    assert_eq!(enc[0], 0x41, "'A' should encode as 0x41");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode 'A'");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 1, "consumed bytes must equal encoded length");
}

// Test 2: '\0' (null char) roundtrip, 1 byte
#[test]
fn test_char_null_roundtrip_one_byte() {
    let c = '\0';
    let enc = encode_to_vec(&c).expect("encode null char");
    assert_eq!(enc.len(), 1, "null char should encode as 1 byte");
    assert_eq!(enc[0], 0x00, "null char should encode as 0x00");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode null char");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 1, "consumed bytes must equal encoded length");
}

// Test 3: ' ' (space, 0x20) roundtrip, 1 byte
#[test]
fn test_char_space_roundtrip_one_byte() {
    let c = ' ';
    let enc = encode_to_vec(&c).expect("encode space");
    assert_eq!(enc.len(), 1, "space should encode as 1 byte");
    assert_eq!(enc[0], 0x20, "space should encode as 0x20");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode space");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 1, "consumed bytes must equal encoded length");
}

// Test 4: '~' (0x7E, max ASCII before DEL) roundtrip, 1 byte
#[test]
fn test_char_tilde_roundtrip_one_byte() {
    let c = '~';
    let enc = encode_to_vec(&c).expect("encode '~'");
    assert_eq!(enc.len(), 1, "'~' should encode as 1 byte");
    assert_eq!(enc[0], 0x7E, "'~' should encode as 0x7E");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode '~'");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 1, "consumed bytes must equal encoded length");
}

// Test 5: '\u{0080}' (first 2-byte char) roundtrip, 2 bytes
#[test]
fn test_char_first_two_byte_roundtrip() {
    let c = '\u{0080}';
    let enc = encode_to_vec(&c).expect("encode U+0080");
    assert_eq!(enc.len(), 2, "U+0080 should encode as 2 bytes");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode U+0080");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 2, "consumed bytes must equal encoded length");
}

// Test 6: 'é' (U+00E9) roundtrip, 2 bytes [0xC3, 0xA9]
#[test]
fn test_char_e_acute_roundtrip_two_bytes() {
    let c = 'é';
    let enc = encode_to_vec(&c).expect("encode 'é'");
    assert_eq!(enc.len(), 2, "'é' should encode as 2 bytes");
    assert_eq!(enc[0], 0xC3, "first byte of 'é' must be 0xC3");
    assert_eq!(enc[1], 0xA9, "second byte of 'é' must be 0xA9");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode 'é'");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 2, "consumed bytes must equal encoded length");
}

// Test 7: 'ñ' (U+00F1) roundtrip, 2 bytes
#[test]
fn test_char_n_tilde_roundtrip_two_bytes() {
    let c = 'ñ';
    let enc = encode_to_vec(&c).expect("encode 'ñ'");
    assert_eq!(enc.len(), 2, "'ñ' should encode as 2 bytes");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode 'ñ'");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 2, "consumed bytes must equal encoded length");
}

// Test 8: '\u{07FF}' (last 2-byte char) roundtrip, 2 bytes
#[test]
fn test_char_last_two_byte_roundtrip() {
    let c = '\u{07FF}';
    let enc = encode_to_vec(&c).expect("encode U+07FF");
    assert_eq!(enc.len(), 2, "U+07FF should encode as 2 bytes");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode U+07FF");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 2, "consumed bytes must equal encoded length");
}

// Test 9: '\u{0800}' (first 3-byte char) roundtrip, 3 bytes
#[test]
fn test_char_first_three_byte_roundtrip() {
    let c = '\u{0800}';
    let enc = encode_to_vec(&c).expect("encode U+0800");
    assert_eq!(enc.len(), 3, "U+0800 should encode as 3 bytes");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode U+0800");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 3, "consumed bytes must equal encoded length");
}

// Test 10: '中' (U+4E2D) roundtrip, 3 bytes
#[test]
fn test_char_cjk_zhong_roundtrip_three_bytes() {
    let c = '中';
    let enc = encode_to_vec(&c).expect("encode '中'");
    assert_eq!(enc.len(), 3, "'中' should encode as 3 bytes");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode '中'");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 3, "consumed bytes must equal encoded length");
}

// Test 11: '日' (U+65E5) roundtrip, 3 bytes
#[test]
fn test_char_cjk_nichi_roundtrip_three_bytes() {
    let c = '日';
    let enc = encode_to_vec(&c).expect("encode '日'");
    assert_eq!(enc.len(), 3, "'日' should encode as 3 bytes");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode '日'");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 3, "consumed bytes must equal encoded length");
}

// Test 12: '\u{FFFF}' (last 3-byte char) roundtrip, 3 bytes
#[test]
fn test_char_last_three_byte_roundtrip() {
    let c = '\u{FFFF}';
    let enc = encode_to_vec(&c).expect("encode U+FFFF");
    assert_eq!(enc.len(), 3, "U+FFFF should encode as 3 bytes");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode U+FFFF");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 3, "consumed bytes must equal encoded length");
}

// Test 13: '\u{10000}' (first 4-byte char) roundtrip, 4 bytes
#[test]
fn test_char_first_four_byte_roundtrip() {
    let c = '\u{10000}';
    let enc = encode_to_vec(&c).expect("encode U+10000");
    assert_eq!(enc.len(), 4, "U+10000 should encode as 4 bytes");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode U+10000");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 4, "consumed bytes must equal encoded length");
}

// Test 14: '🦀' (U+1F980) roundtrip, 4 bytes
#[test]
fn test_char_crab_emoji_roundtrip_four_bytes() {
    let c = '🦀';
    let enc = encode_to_vec(&c).expect("encode crab emoji");
    assert_eq!(enc.len(), 4, "crab emoji should encode as 4 bytes");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode crab emoji");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 4, "consumed bytes must equal encoded length");
}

// Test 15: '😀' (U+1F600) roundtrip, 4 bytes
#[test]
fn test_char_grinning_face_emoji_roundtrip_four_bytes() {
    let c = '😀';
    let enc = encode_to_vec(&c).expect("encode grinning face emoji");
    assert_eq!(enc.len(), 4, "grinning face emoji should encode as 4 bytes");
    let (dec, consumed): (char, usize) =
        decode_from_slice(&enc).expect("decode grinning face emoji");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 4, "consumed bytes must equal encoded length");
}

// Test 16: '\u{10FFFF}' (max Unicode) roundtrip, 4 bytes
#[test]
fn test_char_max_unicode_roundtrip_four_bytes() {
    let c = '\u{10FFFF}';
    let enc = encode_to_vec(&c).expect("encode U+10FFFF");
    assert_eq!(enc.len(), 4, "U+10FFFF should encode as 4 bytes");
    let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode U+10FFFF");
    assert_eq!(dec, c, "decoded char must equal original");
    assert_eq!(consumed, 4, "consumed bytes must equal encoded length");
}

// Test 17: Vec<char> roundtrip
#[test]
fn test_vec_of_chars_roundtrip() {
    let chars: Vec<char> = vec!['H', 'e', 'l', 'l', 'o', ' ', '中', '🦀'];
    let enc = encode_to_vec(&chars).expect("encode Vec<char>");
    let (dec, _): (Vec<char>, usize) = decode_from_slice(&enc).expect("decode Vec<char>");
    assert_eq!(dec, chars, "decoded Vec<char> must equal original");
}

// Test 18: Option<char> Some and None roundtrip
#[test]
fn test_option_char_some_and_none_roundtrip() {
    let some_char: Option<char> = Some('é');
    let enc_some = encode_to_vec(&some_char).expect("encode Some(char)");
    let (dec_some, _): (Option<char>, usize) =
        decode_from_slice(&enc_some).expect("decode Some(char)");
    assert_eq!(
        dec_some, some_char,
        "decoded Some(char) must equal original"
    );

    let none_char: Option<char> = None;
    let enc_none = encode_to_vec(&none_char).expect("encode None::<char>");
    let (dec_none, _): (Option<char>, usize) =
        decode_from_slice(&enc_none).expect("decode None::<char>");
    assert_eq!(dec_none, none_char, "decoded None must equal original");
}

// Test 19: char 'A' has exact bytes [0x41]
#[test]
fn test_char_a_exact_bytes() {
    let enc = encode_to_vec(&'A').expect("encode 'A' for exact bytes");
    assert_eq!(enc, vec![0x41u8], "bytes for 'A' must be exactly [0x41]");
}

// Test 20: char 'é' has exact bytes [0xC3, 0xA9]
#[test]
fn test_char_e_acute_exact_bytes() {
    let enc = encode_to_vec(&'é').expect("encode 'é' for exact bytes");
    assert_eq!(
        enc,
        vec![0xC3u8, 0xA9u8],
        "bytes for 'é' must be exactly [0xC3, 0xA9]"
    );
}

// Test 21: encoded_size('A') == 1, encoded_size('中') == 3, encoded_size('🦀') == 4
#[test]
fn test_encoded_size_for_chars() {
    let size_a = encoded_size(&'A').expect("encoded_size for 'A'");
    assert_eq!(size_a, 1, "encoded_size('A') must be 1");

    let size_zhong = encoded_size(&'中').expect("encoded_size for '中'");
    assert_eq!(size_zhong, 3, "encoded_size('中') must be 3");

    let size_crab = encoded_size(&'🦀').expect("encoded_size for '🦀'");
    assert_eq!(size_crab, 4, "encoded_size('🦀') must be 4");
}

// Test 22: Struct with char field roundtrip
#[derive(Debug, PartialEq, oxicode_derive::Encode, oxicode_derive::Decode)]
struct CharRecord {
    label: char,
    value: u32,
    symbol: char,
}

#[test]
fn test_struct_with_char_field_roundtrip() {
    let record = CharRecord {
        label: 'X',
        value: 42,
        symbol: '🦀',
    };
    let enc = encode_to_vec(&record).expect("encode CharRecord");
    let (dec, consumed): (CharRecord, usize) = decode_from_slice(&enc).expect("decode CharRecord");
    assert_eq!(dec, record, "decoded CharRecord must equal original");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length"
    );
}
