//! Advanced Cow<str> and Cow<[u8]> encoding tests for OxiCode (set 4)

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
use std::borrow::Cow;

// Test 1: Cow::Borrowed("hello") roundtrip — encode borrowed, decode owned
#[test]
fn test_cow4_str_borrowed_hello_roundtrip() {
    let val: Cow<'static, str> = Cow::Borrowed("hello");
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed str hello");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed str hello");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
    // Decoded via decode_from_slice is always Owned
    assert!(matches!(decoded, Cow::Owned(_)));
}

// Test 2: Cow::Owned(String::from("hello")) roundtrip
#[test]
fn test_cow4_str_owned_hello_roundtrip() {
    let val: Cow<'static, str> = Cow::Owned(String::from("hello"));
    let enc = encode_to_vec(&val).expect("encode Cow::Owned str hello");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Owned str hello");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 3: Cow::Borrowed("") empty str roundtrip
#[test]
fn test_cow4_str_borrowed_empty_roundtrip() {
    let val: Cow<'static, str> = Cow::Borrowed("");
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed empty str");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed empty str");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
    assert_eq!(decoded.as_ref(), "");
}

// Test 4: Cow::<str>::Owned(String::new()) roundtrip
#[test]
fn test_cow4_str_owned_string_new_roundtrip() {
    let val: Cow<'static, str> = Cow::Owned(String::new());
    let enc = encode_to_vec(&val).expect("encode Cow::Owned empty String");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Owned empty String");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
    assert_eq!(decoded.as_ref(), "");
}

// Test 5: Cow<str> same bytes as String for same content
#[test]
fn test_cow4_str_same_bytes_as_string() {
    let content = "same encoding as String";
    let cow_val: Cow<'static, str> = Cow::Borrowed(content);
    let string_val = String::from(content);
    let enc_cow = encode_to_vec(&cow_val).expect("encode Cow<str>");
    let enc_string = encode_to_vec(&string_val).expect("encode String");
    assert_eq!(
        enc_cow, enc_string,
        "Cow<str> and String must produce identical wire bytes"
    );
}

// Test 6: Cow::Borrowed(b"bytes") as Cow<[u8]> roundtrip
#[test]
fn test_cow4_bytes_borrowed_literal_roundtrip() {
    let val: Cow<'static, [u8]> = Cow::Borrowed(b"bytes");
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed b\"bytes\"");
    let (decoded, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed b\"bytes\"");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
    // Decoded via decode_from_slice is always Owned
    assert!(matches!(decoded, Cow::Owned(_)));
}

// Test 7: Cow::<[u8]>::Owned(vec![1,2,3]) roundtrip
#[test]
fn test_cow4_bytes_owned_vec_123_roundtrip() {
    let val: Cow<'static, [u8]> = Cow::Owned(vec![1u8, 2, 3]);
    let enc = encode_to_vec(&val).expect("encode Cow::Owned vec![1,2,3]");
    let (decoded, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Owned vec![1,2,3]");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
    assert_eq!(decoded.as_ref(), &[1u8, 2, 3]);
}

// Test 8: Cow<[u8]> same bytes as Vec<u8> for same content
#[test]
fn test_cow4_bytes_same_bytes_as_vec() {
    let data = vec![0xDEu8, 0xAD, 0xBE, 0xEF];
    let cow_val: Cow<'static, [u8]> = Cow::Owned(data.clone());
    let enc_cow = encode_to_vec(&cow_val).expect("encode Cow<[u8]>");
    let enc_vec = encode_to_vec(&data).expect("encode Vec<u8>");
    assert_eq!(
        enc_cow, enc_vec,
        "Cow<[u8]> and Vec<u8> must produce identical wire bytes"
    );
}

// Test 9: Cow<str> with unicode content roundtrip
#[test]
fn test_cow4_str_unicode_roundtrip() {
    let val: Cow<'static, str> = Cow::Owned("こんにちは世界🌏".to_string());
    let enc = encode_to_vec(&val).expect("encode Cow<str> unicode");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow<str> unicode");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 10: Cow<str> with long string (500 chars) roundtrip
#[test]
fn test_cow4_str_long_500_chars_roundtrip() {
    let long_str = "x".repeat(500);
    let val: Cow<'static, str> = Cow::Owned(long_str);
    let enc = encode_to_vec(&val).expect("encode Cow<str> 500-char string");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow<str> 500-char string");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
    assert_eq!(decoded.len(), 500);
}

// Test 11: Vec<Cow<'static, str>> roundtrip (mix of Borrowed/Owned)
#[test]
fn test_cow4_vec_str_mix_roundtrip() {
    let val: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("first"),
        Cow::Owned("second".to_string()),
        Cow::Borrowed("third"),
        Cow::Owned("fourth".to_string()),
        Cow::Borrowed("fifth"),
    ];
    let enc = encode_to_vec(&val).expect("encode Vec<Cow<'static, str>>");
    let (decoded, consumed): (Vec<Cow<'static, str>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Cow<'static, str>>");
    assert_eq!(val.len(), decoded.len());
    for (orig, dec) in val.iter().zip(decoded.iter()) {
        assert_eq!(orig.as_ref(), dec.as_ref());
    }
    assert_eq!(consumed, enc.len());
}

// Test 12: Option<Cow<'static, str>> Some roundtrip
#[test]
fn test_cow4_option_str_some_roundtrip() {
    let val: Option<Cow<'static, str>> = Some(Cow::Borrowed("option some value"));
    let enc = encode_to_vec(&val).expect("encode Option<Cow<str>> Some");
    let (decoded, consumed): (Option<Cow<'static, str>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Cow<str>> Some");
    assert!(decoded.is_some());
    assert_eq!(val.as_deref(), decoded.as_deref());
    assert_eq!(consumed, enc.len());
}

// Test 13: Option<Cow<'static, str>> None roundtrip
#[test]
fn test_cow4_option_str_none_roundtrip() {
    let val: Option<Cow<'static, str>> = None;
    let enc = encode_to_vec(&val).expect("encode Option<Cow<str>> None");
    let (decoded, consumed): (Option<Cow<'static, str>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Cow<str>> None");
    assert!(decoded.is_none());
    assert_eq!(consumed, enc.len());
}

// Test 14: Cow<str> consumed bytes equals encoded length
#[test]
fn test_cow4_str_consumed_equals_encoded_len() {
    let val: Cow<'static, str> = Cow::Borrowed("consumed bytes verification for str");
    let enc = encode_to_vec(&val).expect("encode Cow<str> for consumed-check");
    let (_, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow<str> for consumed-check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed must equal full encoded length for Cow<str>"
    );
}

// Test 15: Cow<[u8]> consumed bytes equals encoded length
#[test]
fn test_cow4_bytes_consumed_equals_encoded_len() {
    let val: Cow<'static, [u8]> = Cow::Owned(vec![0xCAu8, 0xFE, 0xBA, 0xBE]);
    let enc = encode_to_vec(&val).expect("encode Cow<[u8]> for consumed-check");
    let (_, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&enc).expect("decode Cow<[u8]> for consumed-check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed must equal full encoded length for Cow<[u8]>"
    );
}

// Test 16: Config variant — standard config for Cow<str>
#[test]
fn test_cow4_str_standard_config_roundtrip() {
    let val: Cow<'static, str> = Cow::Borrowed("standard config cow str");
    let cfg = config::standard();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode Cow<str> standard config");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Cow<str> standard config");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 17: Config variant — fixed int encoding for Cow<str>
#[test]
fn test_cow4_str_fixed_int_config_roundtrip() {
    let val: Cow<'static, str> = Cow::Owned("fixed int encoding cow str".to_string());
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode Cow<str> fixed_int config");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Cow<str> fixed_int config");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 18: Cow<str> with CJK characters
#[test]
fn test_cow4_str_cjk_roundtrip() {
    let val: Cow<'static, str> = Cow::Owned("中文한국어日本語العربيةภาษาไทย".to_string());
    let enc = encode_to_vec(&val).expect("encode Cow<str> CJK");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow<str> CJK");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 19: Cow<[u8]> with all byte values 0-255
#[test]
fn test_cow4_bytes_all_byte_values_roundtrip() {
    let data: Vec<u8> = (0u8..=255).collect();
    let val: Cow<'static, [u8]> = Cow::Owned(data);
    let enc = encode_to_vec(&val).expect("encode Cow<[u8]> all byte values");
    let (decoded, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&enc).expect("decode Cow<[u8]> all byte values");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(decoded.len(), 256);
    assert_eq!(consumed, enc.len());
}

// Test 20: Large Cow<[u8]> (1000 bytes) roundtrip
#[test]
fn test_cow4_bytes_large_1000_roundtrip() {
    let data: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    let val: Cow<'static, [u8]> = Cow::Owned(data);
    let enc = encode_to_vec(&val).expect("encode Cow<[u8]> 1000 bytes");
    let (decoded, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&enc).expect("decode Cow<[u8]> 1000 bytes");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(decoded.len(), 1000);
    assert_eq!(consumed, enc.len());
}

// Test 21: Cow<str> with newlines/tabs/special chars
#[test]
fn test_cow4_str_special_chars_roundtrip() {
    let val: Cow<'static, str> =
        Cow::Owned("line1\nline2\ttabbed\r\nwindows\x00null\x1besc\\backslash\"quote".to_string());
    let enc = encode_to_vec(&val).expect("encode Cow<str> special chars");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow<str> special chars");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 22: Vec<Cow<'static, [u8]>> roundtrip
#[test]
fn test_cow4_vec_bytes_roundtrip() {
    let val: Vec<Cow<'static, [u8]>> = vec![
        Cow::Owned(vec![0x01u8, 0x02, 0x03]),
        Cow::Borrowed(b"\xDE\xAD\xBE\xEF"),
        Cow::Owned(vec![]),
        Cow::Borrowed(b"\xFF\x00\xFF"),
        Cow::Owned(vec![42u8; 10]),
    ];
    let enc = encode_to_vec(&val).expect("encode Vec<Cow<'static, [u8]>>");
    let (decoded, consumed): (Vec<Cow<'static, [u8]>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Cow<'static, [u8]>>");
    assert_eq!(val.len(), decoded.len());
    for (orig, dec) in val.iter().zip(decoded.iter()) {
        assert_eq!(orig.as_ref(), dec.as_ref());
    }
    assert_eq!(consumed, enc.len());
}
