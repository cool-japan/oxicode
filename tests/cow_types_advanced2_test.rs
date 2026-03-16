//! Advanced roundtrip tests for Cow<str> and Cow<[u8]> — second set of distinct tests.
//! Exercises encode/decode with configs, option wrapping, sequential decode, and more.

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
use std::borrow::Cow;

// ---------------------------------------------------------------------------
// 1. Cow::Borrowed("hello") encodes and decodes to "hello"
// ---------------------------------------------------------------------------
#[test]
fn test_cow_str_borrowed_hello_roundtrip() {
    let cow: Cow<'_, str> = Cow::Borrowed("hello");
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(val, "hello");
}

// ---------------------------------------------------------------------------
// 2. Cow::Owned(String::from("world")) encodes and decodes to "world"
// ---------------------------------------------------------------------------
#[test]
fn test_cow_str_owned_world_roundtrip() {
    let cow: Cow<'_, str> = Cow::Owned(String::from("world"));
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(val, "world");
}

// ---------------------------------------------------------------------------
// 3. Cow::Borrowed("") empty string roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_cow_str_borrowed_empty_roundtrip() {
    let cow: Cow<'_, str> = Cow::Borrowed("");
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(val, "");
}

// ---------------------------------------------------------------------------
// 4. Cow::Borrowed("hello") and String::from("hello") produce same bytes
// ---------------------------------------------------------------------------
#[test]
fn test_cow_str_borrowed_same_bytes_as_string() {
    let cow: Cow<'_, str> = Cow::Borrowed("hello");
    let s = String::from("hello");
    let cow_enc = encode_to_vec(&cow).expect("encode cow");
    let str_enc = encode_to_vec(&s).expect("encode string");
    assert_eq!(
        cow_enc, str_enc,
        "Cow<str> and String must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// 5. Cow<str> with Unicode: "日本語" roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_cow_str_unicode_japanese_roundtrip() {
    let cow: Cow<'_, str> = Cow::Borrowed("日本語");
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(val, "日本語");
}

// ---------------------------------------------------------------------------
// 6. Cow<[u8]> Borrowed slice roundtrip: &[0u8, 1, 2, 3]
// ---------------------------------------------------------------------------
#[test]
fn test_cow_bytes_borrowed_slice_roundtrip() {
    let data: &[u8] = &[0u8, 1, 2, 3];
    let cow: Cow<'_, [u8]> = Cow::Borrowed(data);
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (val, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(val.as_slice(), data);
}

// ---------------------------------------------------------------------------
// 7. Cow<[u8]> Owned vec roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_cow_bytes_owned_vec_roundtrip() {
    let cow: Cow<'_, [u8]> = Cow::Owned(vec![10u8, 20, 30, 40, 50]);
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (val, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(val, vec![10u8, 20, 30, 40, 50]);
}

// ---------------------------------------------------------------------------
// 8. Cow<[u8]> empty slice roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_cow_bytes_empty_slice_roundtrip() {
    let cow: Cow<'_, [u8]> = Cow::Borrowed(&[]);
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (val, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode failed");
    assert!(val.is_empty());
}

// ---------------------------------------------------------------------------
// 9. Cow<[u8]> and Vec<u8> with same data produce same bytes
// ---------------------------------------------------------------------------
#[test]
fn test_cow_bytes_same_encoding_as_vec() {
    let data = vec![0xABu8, 0xCD, 0xEF];
    let cow: Cow<'_, [u8]> = Cow::Borrowed(&data);
    let cow_enc = encode_to_vec(&cow).expect("encode cow");
    let vec_enc = encode_to_vec(&data).expect("encode vec");
    assert_eq!(
        cow_enc, vec_enc,
        "Cow<[u8]> and Vec<u8> must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// 10. Cow<str> consumed bytes == encoded length
// ---------------------------------------------------------------------------
#[test]
fn test_cow_str_consumed_bytes_equals_encoded_len() {
    let cow: Cow<'_, str> = Cow::Borrowed("oxicode");
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (_, consumed): (String, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// 11. Cow<[u8]> consumed bytes == encoded length
// ---------------------------------------------------------------------------
#[test]
fn test_cow_bytes_consumed_bytes_equals_encoded_len() {
    let cow: Cow<'_, [u8]> = Cow::Borrowed(&[1u8, 2, 3, 4, 5, 6, 7]);
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (_, consumed): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// 12. Vec<Cow<str>> — encode Vec<Cow<str>>, decode as Vec<String> for comparison
// ---------------------------------------------------------------------------
#[test]
fn test_vec_cow_str_roundtrip_as_string() {
    let cows: Vec<Cow<'_, str>> = vec![
        Cow::Borrowed("alpha"),
        Cow::Owned(String::from("beta")),
        Cow::Borrowed("gamma"),
    ];
    let enc = encode_to_vec(&cows).expect("encode failed");
    let (decoded, _): (Vec<String>, usize) = decode_from_slice(&enc).expect("decode failed");
    let expected = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];
    assert_eq!(decoded, expected);
}

// ---------------------------------------------------------------------------
// 13. Option<Cow<str>> Some roundtrip (decodes as String)
// ---------------------------------------------------------------------------
#[test]
fn test_option_cow_str_some_roundtrip() {
    let opt: Option<Cow<'_, str>> = Some(Cow::Borrowed("present"));
    let enc = encode_to_vec(&opt).expect("encode failed");
    let (decoded, _): (Option<String>, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(decoded, Some("present".to_string()));
}

// ---------------------------------------------------------------------------
// 14. Option<Cow<str>> None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_cow_str_none_roundtrip() {
    let opt: Option<Cow<'_, str>> = None;
    let enc = encode_to_vec(&opt).expect("encode failed");
    let (decoded, _): (Option<String>, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(decoded, None);
}

// ---------------------------------------------------------------------------
// 15. Encode a Cow<str> field directly (not in a struct — avoids lifetime issues)
// ---------------------------------------------------------------------------
#[test]
fn test_cow_str_as_direct_field_value() {
    // Encode two values that would be fields in a hypothetical struct
    let name: Cow<'_, str> = Cow::Borrowed("Alice");
    let id: u32 = 42;
    let name_enc = encode_to_vec(&name).expect("encode name");
    let id_enc = encode_to_vec(&id).expect("encode id");
    // Decode them back independently
    let (decoded_name, _): (String, usize) = decode_from_slice(&name_enc).expect("decode name");
    let (decoded_id, _): (u32, usize) = decode_from_slice(&id_enc).expect("decode id");
    assert_eq!(decoded_name, "Alice");
    assert_eq!(decoded_id, 42);
}

// ---------------------------------------------------------------------------
// 16. Cow::Borrowed long string (100 chars) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_cow_str_borrowed_100_chars_roundtrip() {
    let s = "x".repeat(100);
    let cow: Cow<'_, str> = Cow::Borrowed(&s);
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(val, s);
    assert_eq!(val.len(), 100);
}

// ---------------------------------------------------------------------------
// 17. Cow::Owned large byte vec (256 bytes) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_cow_bytes_owned_256_bytes_roundtrip() {
    let data: Vec<u8> = (0u8..=255).collect();
    let cow: Cow<'_, [u8]> = Cow::Owned(data.clone());
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (val, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(val, data);
    assert_eq!(val.len(), 256);
}

// ---------------------------------------------------------------------------
// 18. Cow<str> with all ASCII printable chars roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_cow_str_all_ascii_printable_roundtrip() {
    // ASCII printable: 0x20 (space) through 0x7E (~)
    let s: String = (0x20u8..=0x7Eu8).map(|b| b as char).collect();
    let cow: Cow<'_, str> = Cow::Borrowed(&s);
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(val, s);
}

// ---------------------------------------------------------------------------
// 19. Both standard and legacy configs roundtrip Cow<str> correctly, but legacy uses
//     fixed-width length prefix so the encoded bytes differ in size from standard (varint).
// ---------------------------------------------------------------------------
#[test]
fn test_cow_str_configs_both_roundtrip_correctly() {
    use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config};
    let cow: Cow<'_, str> = Cow::Borrowed("config roundtrip");
    // Standard config (varint integers for length prefix)
    let standard_enc =
        encode_to_vec_with_config(&cow, config::standard()).expect("encode standard");
    // Legacy config (fixed-width integers for length prefix)
    let legacy_enc = encode_to_vec_with_config(&cow, config::legacy()).expect("encode legacy");
    // Decode with each respective config — values must match
    let (val_std, consumed_std): (String, usize) =
        decode_from_slice_with_config(&standard_enc, config::standard()).expect("decode standard");
    let (val_leg, consumed_leg): (String, usize) =
        decode_from_slice_with_config(&legacy_enc, config::legacy()).expect("decode legacy");
    assert_eq!(val_std, "config roundtrip");
    assert_eq!(val_leg, "config roundtrip");
    assert_eq!(consumed_std, standard_enc.len());
    assert_eq!(consumed_leg, legacy_enc.len());
    // Legacy uses fixed-width (8-byte) length prefix; standard uses varint (1 byte here),
    // so legacy encoding is strictly larger.
    assert!(legacy_enc.len() > standard_enc.len());
}

// ---------------------------------------------------------------------------
// 20. Big-endian config with Cow<[u8]> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_cow_bytes_big_endian_config_roundtrip() {
    use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config};
    let data = vec![0xFEu8, 0xDC, 0xBA, 0x98];
    let cow: Cow<'_, [u8]> = Cow::Owned(data.clone());
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&cow, cfg).expect("encode failed");
    let (val, consumed): (Vec<u8>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode failed");
    assert_eq!(val, data);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// 21. Cow<str> with emoji characters: "🦀🎉"
// ---------------------------------------------------------------------------
#[test]
fn test_cow_str_emoji_roundtrip() {
    let cow: Cow<'_, str> = Cow::Borrowed("🦀🎉");
    let enc = encode_to_vec(&cow).expect("encode failed");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(val, "🦀🎉");
}

// ---------------------------------------------------------------------------
// 22. Multiple Cow<str> encoded sequentially, decode in order
// ---------------------------------------------------------------------------
#[test]
fn test_cow_str_sequential_encode_decode() {
    let first: Cow<'_, str> = Cow::Borrowed("first");
    let second: Cow<'_, str> = Cow::Owned(String::from("second"));
    let third: Cow<'_, str> = Cow::Borrowed("third");

    let mut buf = Vec::new();
    buf.extend_from_slice(&encode_to_vec(&first).expect("encode first"));
    buf.extend_from_slice(&encode_to_vec(&second).expect("encode second"));
    buf.extend_from_slice(&encode_to_vec(&third).expect("encode third"));

    let (val1, n1): (String, usize) = decode_from_slice(&buf).expect("decode first");
    let (val2, n2): (String, usize) = decode_from_slice(&buf[n1..]).expect("decode second");
    let (val3, n3): (String, usize) = decode_from_slice(&buf[n1 + n2..]).expect("decode third");

    assert_eq!(val1, "first");
    assert_eq!(val2, "second");
    assert_eq!(val3, "third");
    assert_eq!(n1 + n2 + n3, buf.len());
}
