//! Advanced Cow<str> and Cow<[u8]> encoding tests for OxiCode (set 3)

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

// Test 1: Cow::<str>::Borrowed("hello") roundtrip
#[test]
fn test_cow_str_borrowed_hello_roundtrip() {
    let val: Cow<'_, str> = Cow::Borrowed("hello");
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed str");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed str");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 2: Cow::<str>::Owned("hello".to_string()) roundtrip
#[test]
fn test_cow_str_owned_hello_roundtrip() {
    let val: Cow<'_, str> = Cow::Owned("hello".to_string());
    let enc = encode_to_vec(&val).expect("encode Cow::Owned str");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Owned str");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 3: Cow::<str>::Borrowed("") empty roundtrip
#[test]
fn test_cow_str_borrowed_empty_roundtrip() {
    let val: Cow<'_, str> = Cow::Borrowed("");
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed empty str");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed empty str");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 4: Cow::<str>::Borrowed("hello") and Cow::<str>::Owned("hello") produce same bytes
#[test]
fn test_cow_str_borrowed_owned_same_bytes() {
    let borrowed: Cow<'_, str> = Cow::Borrowed("hello");
    let owned: Cow<'_, str> = Cow::Owned("hello".to_string());
    let enc_borrowed = encode_to_vec(&borrowed).expect("encode Cow::Borrowed");
    let enc_owned = encode_to_vec(&owned).expect("encode Cow::Owned");
    assert_eq!(
        enc_borrowed, enc_owned,
        "Borrowed and Owned must produce same wire bytes"
    );
}

// Test 5: Cow::<[u8]>::Borrowed(&[1, 2, 3]) roundtrip
#[test]
fn test_cow_bytes_borrowed_roundtrip() {
    let val: Cow<'_, [u8]> = Cow::Borrowed(&[1u8, 2, 3]);
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed [u8]");
    let (decoded, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed [u8]");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 6: Cow::<[u8]>::Owned(vec![1, 2, 3]) roundtrip
#[test]
fn test_cow_bytes_owned_roundtrip() {
    let val: Cow<'_, [u8]> = Cow::Owned(vec![1u8, 2, 3]);
    let enc = encode_to_vec(&val).expect("encode Cow::Owned [u8]");
    let (decoded, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Owned [u8]");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 7: Cow::<[u8]>::Borrowed(&[]) empty roundtrip
#[test]
fn test_cow_bytes_borrowed_empty_roundtrip() {
    let val: Cow<'_, [u8]> = Cow::Borrowed(&[]);
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed empty [u8]");
    let (decoded, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed empty [u8]");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 8: Cow::<[u8]>::Borrowed and Cow::<[u8]>::Owned produce same bytes for same data
#[test]
fn test_cow_bytes_borrowed_owned_same_bytes() {
    let data = [10u8, 20, 30];
    let borrowed: Cow<'_, [u8]> = Cow::Borrowed(&data);
    let owned: Cow<'_, [u8]> = Cow::Owned(data.to_vec());
    let enc_borrowed = encode_to_vec(&borrowed).expect("encode Cow::Borrowed [u8]");
    let enc_owned = encode_to_vec(&owned).expect("encode Cow::Owned [u8]");
    assert_eq!(
        enc_borrowed, enc_owned,
        "Borrowed and Owned [u8] must produce same wire bytes"
    );
}

// Test 9: Cow::<str>::Borrowed with unicode "日本語" roundtrip
#[test]
fn test_cow_str_borrowed_unicode_roundtrip() {
    let val: Cow<'_, str> = Cow::Borrowed("日本語");
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed unicode str");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed unicode str");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 10: Cow::<str>::Borrowed with emoji "🦀" roundtrip
#[test]
fn test_cow_str_borrowed_emoji_roundtrip() {
    let val: Cow<'_, str> = Cow::Borrowed("🦀");
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed emoji str");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed emoji str");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 11: Cow::<str> consumed bytes equals encoded len
#[test]
fn test_cow_str_consumed_equals_encoded_len() {
    let val: Cow<'_, str> = Cow::Borrowed("consumed bytes check");
    let enc = encode_to_vec(&val).expect("encode Cow str for consumed check");
    let (_, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow str for consumed check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal full encoded length"
    );
}

// Test 12: Cow::<[u8]> consumed bytes equals encoded len
#[test]
fn test_cow_bytes_consumed_equals_encoded_len() {
    let val: Cow<'_, [u8]> = Cow::Borrowed(&[0xABu8, 0xCD, 0xEF]);
    let enc = encode_to_vec(&val).expect("encode Cow [u8] for consumed check");
    let (_, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&enc).expect("decode Cow [u8] for consumed check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal full encoded length"
    );
}

// Test 13: Cow::<str>::Borrowed with ASCII printable chars roundtrip
#[test]
fn test_cow_str_borrowed_ascii_printable_roundtrip() {
    // Printable ASCII: 0x20 (' ') through 0x7E ('~')
    let ascii: String = (0x20u8..=0x7Eu8).map(|b| b as char).collect();
    let val: Cow<'_, str> = Cow::Borrowed(&ascii);
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed ASCII printable str");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed ASCII printable str");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 14: Cow::<[u8]>::Borrowed with 100 bytes roundtrip
#[test]
fn test_cow_bytes_borrowed_100_bytes_roundtrip() {
    let data: Vec<u8> = (0u8..100).collect();
    let val: Cow<'_, [u8]> = Cow::Borrowed(&data);
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed 100 bytes");
    let (decoded, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed 100 bytes");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 15: Vec<Cow<str>> roundtrip with 5 borrowed strings
#[test]
fn test_vec_cow_str_borrowed_roundtrip() {
    let strings = ["alpha", "beta", "gamma", "delta", "epsilon"];
    let val: Vec<Cow<'_, str>> = strings.iter().map(|s| Cow::Borrowed(*s)).collect();
    let enc = encode_to_vec(&val).expect("encode Vec<Cow<str>>");
    let (decoded, consumed): (Vec<Cow<'static, str>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Cow<str>>");
    assert_eq!(val.len(), decoded.len());
    for (orig, dec) in val.iter().zip(decoded.iter()) {
        assert_eq!(orig.as_ref(), dec.as_ref());
    }
    assert_eq!(consumed, enc.len());
}

// Test 16: Option<Cow<str>> Some roundtrip
#[test]
fn test_option_cow_str_some_roundtrip() {
    let val: Option<Cow<'_, str>> = Some(Cow::Borrowed("some value"));
    let enc = encode_to_vec(&val).expect("encode Option<Cow<str>> Some");
    let (decoded, consumed): (Option<Cow<'static, str>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Cow<str>> Some");
    assert!(decoded.is_some());
    assert_eq!(val.as_deref(), decoded.as_deref());
    assert_eq!(consumed, enc.len());
}

// Test 17: Option<Cow<str>> None roundtrip
#[test]
fn test_option_cow_str_none_roundtrip() {
    let val: Option<Cow<'_, str>> = None;
    let enc = encode_to_vec(&val).expect("encode Option<Cow<str>> None");
    let (decoded, consumed): (Option<Cow<'static, str>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Cow<str>> None");
    assert!(decoded.is_none());
    assert_eq!(consumed, enc.len());
}

// Test 18: Cow::<str> fixed int config roundtrip
#[test]
fn test_cow_str_fixed_int_config_roundtrip() {
    let val: Cow<'_, str> = Cow::Borrowed("fixed int config test");
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode Cow<str> fixed_int");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Cow<str> fixed_int");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 19: Cow::<str>::Borrowed with whitespace only roundtrip
#[test]
fn test_cow_str_borrowed_whitespace_roundtrip() {
    let val: Cow<'_, str> = Cow::Borrowed("   \t\n\r  ");
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed whitespace str");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed whitespace str");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 20: Cow::<[u8]>::Borrowed with all byte values 0-255 roundtrip
#[test]
fn test_cow_bytes_borrowed_all_values_roundtrip() {
    let data: Vec<u8> = (0u8..=255).collect();
    let val: Cow<'_, [u8]> = Cow::Borrowed(&data);
    let enc = encode_to_vec(&val).expect("encode Cow::Borrowed all byte values");
    let (decoded, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&enc).expect("decode Cow::Borrowed all byte values");
    assert_eq!(val.as_ref(), decoded.as_ref());
    assert_eq!(consumed, enc.len());
}

// Test 21: Cow::<str> and String produce same wire bytes for same content
#[test]
fn test_cow_str_and_string_same_wire_bytes() {
    let content = "wire bytes parity check";
    let cow_val: Cow<'_, str> = Cow::Borrowed(content);
    let string_val = content.to_string();
    let enc_cow = encode_to_vec(&cow_val).expect("encode Cow<str>");
    let enc_string = encode_to_vec(&string_val).expect("encode String");
    assert_eq!(
        enc_cow, enc_string,
        "Cow<str> and String must produce identical wire bytes"
    );
}

// Test 22: Vec<Cow<[u8]>> roundtrip with 3 elements
#[test]
fn test_vec_cow_bytes_roundtrip() {
    let a = vec![0xAAu8, 0xBB];
    let b = vec![0xCCu8, 0xDD, 0xEE];
    let c = vec![0xFFu8];
    let val: Vec<Cow<'_, [u8]>> = vec![
        Cow::Borrowed(a.as_slice()),
        Cow::Borrowed(b.as_slice()),
        Cow::Borrowed(c.as_slice()),
    ];
    let enc = encode_to_vec(&val).expect("encode Vec<Cow<[u8]>>");
    let (decoded, consumed): (Vec<Cow<'static, [u8]>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Cow<[u8]>>");
    assert_eq!(val.len(), decoded.len());
    for (orig, dec) in val.iter().zip(decoded.iter()) {
        assert_eq!(orig.as_ref(), dec.as_ref());
    }
    assert_eq!(consumed, enc.len());
}
