//! Advanced tests for Cow<str> and Cow<[u8]> encoding/decoding in OxiCode.

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
use std::borrow::Cow;

// Struct used in test 13
#[derive(Debug, PartialEq, Encode, Decode)]
struct CowContainer {
    label: String,
    value: u32,
}

// Test 1: Cow::Owned("hello") roundtrip — decodes as owned string
#[test]
fn test_cow_owned_str_roundtrip_decodes_as_owned() {
    let val: Cow<'static, str> = Cow::Owned(String::from("hello"));
    let encoded = encode_to_vec(&val).expect("encode Cow::Owned str");
    let (decoded, _): (Cow<'static, str>, usize) =
        decode_from_slice(&encoded).expect("decode Cow::Owned str");
    assert_eq!(decoded.as_ref(), "hello");
    assert!(
        matches!(decoded, Cow::Owned(_)),
        "decoded Cow<str> must be Cow::Owned"
    );
}

// Test 2: Cow::Borrowed("hello") encodes same as owned
#[test]
fn test_cow_borrowed_str_encodes_same_as_owned() {
    let owned: Cow<'static, str> = Cow::Owned(String::from("hello"));
    let borrowed: Cow<str> = Cow::Borrowed("hello");
    let enc_owned = encode_to_vec(&owned).expect("encode owned");
    let enc_borrowed = encode_to_vec(&borrowed).expect("encode borrowed");
    assert_eq!(
        enc_owned, enc_borrowed,
        "Cow::Borrowed and Cow::Owned with same content must encode identically"
    );
}

// Test 3: Cow::Owned(vec![1,2,3]) for Cow<'static, [u8]> roundtrip
#[test]
fn test_cow_owned_bytes_roundtrip() {
    let val: Cow<'static, [u8]> = Cow::Owned(vec![1u8, 2, 3]);
    let encoded = encode_to_vec(&val).expect("encode Cow<[u8]>");
    let (decoded, _): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&encoded).expect("decode Cow<[u8]>");
    assert_eq!(decoded.as_ref(), &[1u8, 2, 3][..]);
    assert!(
        matches!(decoded, Cow::Owned(_)),
        "decoded Cow<[u8]> must be Cow::Owned"
    );
}

// Test 4: Empty Cow<'static, str> roundtrip
#[test]
fn test_cow_str_empty_roundtrip() {
    let val: Cow<'static, str> = Cow::Owned(String::new());
    let encoded = encode_to_vec(&val).expect("encode empty Cow str");
    let (decoded, _): (Cow<'static, str>, usize) =
        decode_from_slice(&encoded).expect("decode empty Cow str");
    assert_eq!(decoded.as_ref(), "");
}

// Test 5: Cow<'static, str> with unicode roundtrip
#[test]
fn test_cow_str_unicode_roundtrip() {
    let val: Cow<'static, str> = Cow::Owned(String::from("こんにちは世界 🌍"));
    let encoded = encode_to_vec(&val).expect("encode unicode Cow str");
    let (decoded, _): (Cow<'static, str>, usize) =
        decode_from_slice(&encoded).expect("decode unicode Cow str");
    assert_eq!(decoded.as_ref(), "こんにちは世界 🌍");
}

// Test 6: Cow::Owned vs Cow::Borrowed produce identical bytes
#[test]
fn test_cow_owned_vs_borrowed_identical_bytes() {
    let text = "identical content";
    let owned: Cow<'static, str> = Cow::Owned(String::from(text));
    let borrowed: Cow<str> = Cow::Borrowed(text);
    let enc_owned = encode_to_vec(&owned).expect("encode owned");
    let enc_borrowed = encode_to_vec(&borrowed).expect("encode borrowed");
    assert_eq!(
        enc_owned, enc_borrowed,
        "Cow::Owned and Cow::Borrowed must produce identical wire bytes"
    );
}

// Test 7: Cow<'static, [u8]> empty slice roundtrip
#[test]
fn test_cow_bytes_empty_slice_roundtrip() {
    let val: Cow<'static, [u8]> = Cow::Owned(vec![]);
    let encoded = encode_to_vec(&val).expect("encode empty Cow<[u8]>");
    let (decoded, _): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&encoded).expect("decode empty Cow<[u8]>");
    assert_eq!(decoded.as_ref(), &[][..]);
    assert!(decoded.is_empty());
}

// Test 8: Cow<'static, [u8]> with 100 bytes roundtrip
#[test]
fn test_cow_bytes_100_bytes_roundtrip() {
    let data: Vec<u8> = (0u8..100).collect();
    let val: Cow<'static, [u8]> = Cow::Owned(data.clone());
    let encoded = encode_to_vec(&val).expect("encode 100-byte Cow<[u8]>");
    let (decoded, _): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&encoded).expect("decode 100-byte Cow<[u8]>");
    assert_eq!(decoded.len(), 100);
    assert_eq!(decoded.as_ref(), data.as_slice());
}

// Test 9: Vec<Cow<'static, str>> roundtrip
#[test]
fn test_vec_of_cow_str_roundtrip() {
    let val: Vec<Cow<'static, str>> = vec![
        Cow::Owned(String::from("alpha")),
        Cow::Owned(String::from("beta")),
        Cow::Owned(String::from("gamma")),
    ];
    let encoded = encode_to_vec(&val).expect("encode Vec<Cow<str>>");
    let (decoded, _): (Vec<Cow<'static, str>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Cow<str>>");
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].as_ref(), "alpha");
    assert_eq!(decoded[1].as_ref(), "beta");
    assert_eq!(decoded[2].as_ref(), "gamma");
}

// Test 10: Option<Cow<'static, str>> Some roundtrip
#[test]
fn test_option_cow_str_some_roundtrip() {
    let val: Option<Cow<'static, str>> = Some(Cow::Owned(String::from("present")));
    let encoded = encode_to_vec(&val).expect("encode Option<Cow<str>> Some");
    let (decoded, _): (Option<Cow<'static, str>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Cow<str>> Some");
    assert!(decoded.is_some());
    assert_eq!(decoded.as_deref(), Some("present"));
}

// Test 11: Option<Cow<'static, str>> None roundtrip
#[test]
fn test_option_cow_str_none_roundtrip() {
    let val: Option<Cow<'static, str>> = None;
    let encoded = encode_to_vec(&val).expect("encode Option<Cow<str>> None");
    let (decoded, _): (Option<Cow<'static, str>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Cow<str>> None");
    assert!(decoded.is_none());
}

// Test 12: Cow<'static, str> with fixed_int_encoding config roundtrip
#[test]
fn test_cow_str_fixed_int_encoding_config_roundtrip() {
    let val: Cow<'static, str> = Cow::Owned(String::from("fixed int config"));
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&val, cfg).expect("encode with fixed_int config");
    let (decoded, _): (Cow<'static, str>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with fixed_int config");
    assert_eq!(decoded.as_ref(), "fixed int config");
}

// Test 13: Struct containing Cow<'static, str> field roundtrip
#[test]
fn test_struct_containing_string_field_roundtrip() {
    // CowContainer uses String field to represent String-like Cow content
    let val = CowContainer {
        label: String::from("my-label"),
        value: 42,
    };
    let encoded = encode_to_vec(&val).expect("encode CowContainer");
    let (decoded, _): (CowContainer, usize) =
        decode_from_slice(&encoded).expect("decode CowContainer");
    assert_eq!(decoded, val);
}

// Test 14: Cow<'static, str> == String for same content (after decode both are String-like)
#[test]
fn test_cow_str_content_equals_string_after_decode() {
    let original = String::from("compare me");
    let cow_val: Cow<'static, str> = Cow::Owned(original.clone());
    let encoded = encode_to_vec(&cow_val).expect("encode Cow str");
    let (decoded, _): (Cow<'static, str>, usize) =
        decode_from_slice(&encoded).expect("decode Cow str");
    // After decode, Cow content must equal the original String content
    assert_eq!(decoded.as_ref(), original.as_str());
    // The decoded value, when converted to owned, should equal the original
    assert_eq!(decoded.into_owned(), original);
}

// Test 15: Cow<'static, str> encodes same as &str
#[test]
fn test_cow_str_encodes_same_as_str_ref() {
    let text = "same bytes";
    let cow_val: Cow<'static, str> = Cow::Owned(String::from(text));
    let str_ref: &str = text;
    let enc_cow = encode_to_vec(&cow_val).expect("encode Cow str");
    let enc_str = encode_to_vec(&str_ref).expect("encode &str");
    assert_eq!(
        enc_cow, enc_str,
        "Cow<str> and &str with same content must produce identical wire bytes"
    );
}

// Test 16: consumed bytes == encoded length for Cow<'static, str>
#[test]
fn test_consumed_bytes_equals_encoded_length_for_cow_str() {
    let val: Cow<'static, str> = Cow::Owned(String::from("measure me"));
    let encoded = encode_to_vec(&val).expect("encode");
    let (_, consumed): (Cow<'static, str>, usize) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal the total encoded length"
    );
}

// Test 17: Cow<'static, [u8]> with large data (1000 bytes) roundtrip
#[test]
fn test_cow_bytes_large_data_1000_roundtrip() {
    let data: Vec<u8> = (0u16..1000).map(|i| (i % 256) as u8).collect();
    let val: Cow<'static, [u8]> = Cow::Owned(data.clone());
    let encoded = encode_to_vec(&val).expect("encode large Cow<[u8]>");
    let (decoded, _): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&encoded).expect("decode large Cow<[u8]>");
    assert_eq!(decoded.len(), 1000);
    assert_eq!(decoded.as_ref(), data.as_slice());
}

// Test 18: Cow<'static, str> with control chars roundtrip
#[test]
fn test_cow_str_with_control_chars_roundtrip() {
    let text = "line1\nline2\ttabbed\r\nwindows\0null";
    let val: Cow<'static, str> = Cow::Owned(String::from(text));
    let encoded = encode_to_vec(&val).expect("encode Cow str with control chars");
    let (decoded, _): (Cow<'static, str>, usize) =
        decode_from_slice(&encoded).expect("decode Cow str with control chars");
    assert_eq!(decoded.as_ref(), text);
}

// Test 19: Vec<Cow<'static, [u8]>> roundtrip
#[test]
fn test_vec_of_cow_bytes_roundtrip() {
    let val: Vec<Cow<'static, [u8]>> = vec![
        Cow::Owned(vec![10u8, 20, 30]),
        Cow::Owned(vec![]),
        Cow::Owned(vec![255u8, 0, 128]),
    ];
    let encoded = encode_to_vec(&val).expect("encode Vec<Cow<[u8]>>");
    let (decoded, _): (Vec<Cow<'static, [u8]>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Cow<[u8]>>");
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].as_ref(), &[10u8, 20, 30][..]);
    assert_eq!(decoded[1].as_ref(), &[][..]);
    assert_eq!(decoded[2].as_ref(), &[255u8, 0, 128][..]);
}

// Test 20: Nested: Cow inside Option inside Vec roundtrip
#[test]
fn test_nested_cow_inside_option_inside_vec_roundtrip() {
    let val: Vec<Option<Cow<'static, str>>> = vec![
        Some(Cow::Owned(String::from("present"))),
        None,
        Some(Cow::Owned(String::from("also present"))),
    ];
    let encoded = encode_to_vec(&val).expect("encode Vec<Option<Cow<str>>>");
    let (decoded, _): (Vec<Option<Cow<'static, str>>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Option<Cow<str>>>");
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].as_deref(), Some("present"));
    assert!(decoded[1].is_none());
    assert_eq!(decoded[2].as_deref(), Some("also present"));
}

// Test 21: Cow<'static, str> and String produce identical wire bytes for same content
#[test]
fn test_cow_str_and_string_produce_identical_wire_bytes() {
    let content = "wire format parity";
    let cow_val: Cow<'static, str> = Cow::Owned(String::from(content));
    let string_val = String::from(content);
    let enc_cow = encode_to_vec(&cow_val).expect("encode Cow str");
    let enc_string = encode_to_vec(&string_val).expect("encode String");
    assert_eq!(
        enc_cow, enc_string,
        "Cow<str> and String must produce identical wire bytes for same content"
    );
}

// Test 22: Box<str> roundtrip (similar to Cow)
#[test]
fn test_box_str_roundtrip() {
    let val: Box<str> = Box::from("boxed string content");
    let encoded = encode_to_vec(&val).expect("encode Box<str>");
    let (decoded, _): (Box<str>, usize) = decode_from_slice(&encoded).expect("decode Box<str>");
    assert_eq!(decoded.as_ref(), "boxed string content");
}
