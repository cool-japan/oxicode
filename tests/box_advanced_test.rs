//! Advanced roundtrip tests for `Box<T>` encoding/decoding in OxiCode.
//!
//! Tests cover primitive types, compound types, nested boxes, config variants,
//! and byte-size invariants.

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
use oxicode::{config, decode_from_slice, encode_to_vec};

// ---------------------------------------------------------------------------
// 1. Box<u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_box_u32_roundtrip() {
    let original: Box<u32> = Box::new(1_234_567);
    let enc = encode_to_vec(&original).expect("encode Box<u32>");
    let (dec, _): (Box<u32>, _) = decode_from_slice(&enc).expect("decode Box<u32>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 2. Box<String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_box_string_roundtrip() {
    let original: Box<String> = Box::new(String::from("oxicode box test"));
    let enc = encode_to_vec(&original).expect("encode Box<String>");
    let (dec, _): (Box<String>, _) = decode_from_slice(&enc).expect("decode Box<String>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 3. Box<Vec<u8>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_box_vec_u8_roundtrip() {
    let original: Box<Vec<u8>> = Box::new(vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF]);
    let enc = encode_to_vec(&original).expect("encode Box<Vec<u8>>");
    let (dec, _): (Box<Vec<u8>>, _) = decode_from_slice(&enc).expect("decode Box<Vec<u8>>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 4. Box<Option<u64>> Some and None
// ---------------------------------------------------------------------------

#[test]
fn test_box_option_u64_some_roundtrip() {
    let original: Box<Option<u64>> = Box::new(Some(u64::MAX / 2));
    let enc = encode_to_vec(&original).expect("encode Box<Option<u64>> Some");
    let (dec, _): (Box<Option<u64>>, _) =
        decode_from_slice(&enc).expect("decode Box<Option<u64>> Some");
    assert_eq!(original, dec);
}

#[test]
fn test_box_option_u64_none_roundtrip() {
    let original: Box<Option<u64>> = Box::new(None);
    let enc = encode_to_vec(&original).expect("encode Box<Option<u64>> None");
    let (dec, _): (Box<Option<u64>>, _) =
        decode_from_slice(&enc).expect("decode Box<Option<u64>> None");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 5. Box<Box<u32>> nested boxes
// ---------------------------------------------------------------------------

#[test]
fn test_box_box_u32_roundtrip() {
    let original: Box<Box<u32>> = Box::new(Box::new(99_999));
    let enc = encode_to_vec(&original).expect("encode Box<Box<u32>>");
    let (dec, _): (Box<Box<u32>>, _) = decode_from_slice(&enc).expect("decode Box<Box<u32>>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 6. Box<[u8; 8]> array in box
// ---------------------------------------------------------------------------

#[test]
fn test_box_array_u8_roundtrip() {
    let original: Box<[u8; 8]> = Box::new([1, 2, 3, 4, 5, 6, 7, 8]);
    let enc = encode_to_vec(&original).expect("encode Box<[u8; 8]>");
    let (dec, _): (Box<[u8; 8]>, _) = decode_from_slice(&enc).expect("decode Box<[u8; 8]>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 7. Box<(u32, u64, bool)> tuple in box
// ---------------------------------------------------------------------------

#[test]
fn test_box_tuple_roundtrip() {
    let original: Box<(u32, u64, bool)> = Box::new((42, 999_999_999_999, true));
    let enc = encode_to_vec(&original).expect("encode Box<(u32, u64, bool)>");
    let (dec, _): (Box<(u32, u64, bool)>, _) =
        decode_from_slice(&enc).expect("decode Box<(u32, u64, bool)>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 8. Box<i128> large int
// ---------------------------------------------------------------------------

#[test]
fn test_box_i128_roundtrip() {
    let original: Box<i128> = Box::new(i128::MIN + 1);
    let enc = encode_to_vec(&original).expect("encode Box<i128>");
    let (dec, _): (Box<i128>, _) = decode_from_slice(&enc).expect("decode Box<i128>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 9. Box<f64> float — use std::f64::consts::PI
// ---------------------------------------------------------------------------

#[test]
fn test_box_f64_roundtrip() {
    let original: Box<f64> = Box::new(std::f64::consts::PI);
    let enc = encode_to_vec(&original).expect("encode Box<f64>");
    let (dec, _): (Box<f64>, _) = decode_from_slice(&enc).expect("decode Box<f64>");
    // Compare bit patterns to handle -0.0/NaN edge cases correctly.
    assert_eq!(
        original.to_bits(),
        dec.to_bits(),
        "f64 bit pattern mismatch"
    );
}

// ---------------------------------------------------------------------------
// 10. Box<bool>
// ---------------------------------------------------------------------------

#[test]
fn test_box_bool_roundtrip() {
    for value in [true, false] {
        let original: Box<bool> = Box::new(value);
        let enc = encode_to_vec(&original).expect("encode Box<bool>");
        let (dec, _): (Box<bool>, _) = decode_from_slice(&enc).expect("decode Box<bool>");
        assert_eq!(original, dec, "bool={value}");
    }
}

// ---------------------------------------------------------------------------
// 11. Vec<Box<u32>> — vec of boxes
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_box_u32_roundtrip() {
    let original: Vec<Box<u32>> = vec![Box::new(1), Box::new(2), Box::new(3), Box::new(u32::MAX)];
    let enc = encode_to_vec(&original).expect("encode Vec<Box<u32>>");
    let (dec, _): (Vec<Box<u32>>, _) = decode_from_slice(&enc).expect("decode Vec<Box<u32>>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 12. Box<Vec<Box<String>>> deeply nested
// ---------------------------------------------------------------------------

#[test]
fn test_box_vec_of_box_string_roundtrip() {
    let original: Box<Vec<Box<String>>> = Box::new(vec![
        Box::new(String::from("alpha")),
        Box::new(String::from("beta")),
        Box::new(String::from("gamma")),
    ]);
    let enc = encode_to_vec(&original).expect("encode Box<Vec<Box<String>>>");
    let (dec, _): (Box<Vec<Box<String>>>, _) =
        decode_from_slice(&enc).expect("decode Box<Vec<Box<String>>>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 13. Fixed-int encoding with Box<u32>
// ---------------------------------------------------------------------------

#[test]
fn test_box_u32_fixed_int_encoding() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let original: Box<u32> = Box::new(12345);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&original, cfg).expect("fixed-int encode Box<u32>");
    // Fixed-int u32 must be exactly 4 bytes.
    assert_eq!(enc.len(), 4, "fixed-int Box<u32> must be 4 bytes");
    let (dec, _): (Box<u32>, _) =
        decode_from_slice_with_config(&enc, cfg).expect("fixed-int decode Box<u32>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 14. Big-endian config with Box<u32>
// ---------------------------------------------------------------------------

#[test]
fn test_box_u32_big_endian_roundtrip() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let original: Box<u32> = Box::new(0x0102_0304);
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&original, cfg).expect("big-endian encode Box<u32>");
    assert_eq!(
        enc,
        [0x01, 0x02, 0x03, 0x04],
        "big-endian Box<u32> byte order mismatch"
    );
    let (dec, _): (Box<u32>, _) =
        decode_from_slice_with_config(&enc, cfg).expect("big-endian decode Box<u32>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 15. Box<()> — box over unit type
// ---------------------------------------------------------------------------

#[test]
fn test_box_unit_roundtrip() {
    let original: Box<()> = Box::new(());
    let enc = encode_to_vec(&original).expect("encode Box<()>");
    let (dec, _): (Box<()>, _) = decode_from_slice(&enc).expect("decode Box<()>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 16. Box<u8> single byte
// ---------------------------------------------------------------------------

#[test]
fn test_box_u8_roundtrip() {
    for value in [0u8, 1, 127, 128, 255] {
        let original: Box<u8> = Box::new(value);
        let enc = encode_to_vec(&original).expect("encode Box<u8>");
        let (dec, _): (Box<u8>, _) = decode_from_slice(&enc).expect("decode Box<u8>");
        assert_eq!(original, dec, "u8={value}");
    }
}

// ---------------------------------------------------------------------------
// 17. Box<char>
// ---------------------------------------------------------------------------

#[test]
fn test_box_char_roundtrip() {
    for ch in ['A', 'z', '0', '€', '🦀'] {
        let original: Box<char> = Box::new(ch);
        let enc = encode_to_vec(&original).expect("encode Box<char>");
        let (dec, _): (Box<char>, _) = decode_from_slice(&enc).expect("decode Box<char>");
        assert_eq!(original, dec, "char='{ch}'");
    }
}

// ---------------------------------------------------------------------------
// 18. Box<Result<u32, String>>
// ---------------------------------------------------------------------------

#[test]
fn test_box_result_roundtrip() {
    let ok_val: Box<Result<u32, String>> = Box::new(Ok(42));
    let enc_ok = encode_to_vec(&ok_val).expect("encode Box<Result> Ok");
    let (dec_ok, _): (Box<Result<u32, String>>, _) =
        decode_from_slice(&enc_ok).expect("decode Box<Result> Ok");
    assert_eq!(ok_val, dec_ok);

    let err_val: Box<Result<u32, String>> = Box::new(Err(String::from("something went wrong")));
    let enc_err = encode_to_vec(&err_val).expect("encode Box<Result> Err");
    let (dec_err, _): (Box<Result<u32, String>>, _) =
        decode_from_slice(&enc_err).expect("decode Box<Result> Err");
    assert_eq!(err_val, dec_err);
}

// ---------------------------------------------------------------------------
// 19. Box<Option<Box<u32>>> complex nesting
// ---------------------------------------------------------------------------

#[test]
fn test_box_option_box_u32_roundtrip() {
    let some_val: Box<Option<Box<u32>>> = Box::new(Some(Box::new(7777)));
    let enc = encode_to_vec(&some_val).expect("encode Box<Option<Box<u32>>> Some");
    let (dec, _): (Box<Option<Box<u32>>>, _) =
        decode_from_slice(&enc).expect("decode Box<Option<Box<u32>>> Some");
    assert_eq!(some_val, dec);

    let none_val: Box<Option<Box<u32>>> = Box::new(None);
    let enc_none = encode_to_vec(&none_val).expect("encode Box<Option<Box<u32>>> None");
    let (dec_none, _): (Box<Option<Box<u32>>>, _) =
        decode_from_slice(&enc_none).expect("decode Box<Option<Box<u32>>> None");
    assert_eq!(none_val, dec_none);
}

// ---------------------------------------------------------------------------
// 20. Large data in Box — 1000-element Vec
// ---------------------------------------------------------------------------

#[test]
fn test_box_large_vec_roundtrip() {
    let data: Vec<u32> = (0u32..1000).collect();
    let original: Box<Vec<u32>> = Box::new(data);
    let enc = encode_to_vec(&original).expect("encode Box<Vec<u32>> large");
    let (dec, _): (Box<Vec<u32>>, _) = decode_from_slice(&enc).expect("decode Box<Vec<u32>> large");
    assert_eq!(original.len(), dec.len());
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 21. Multiple boxes in tuple (Box<u32>, Box<String>)
// ---------------------------------------------------------------------------

#[test]
fn test_tuple_of_boxes_roundtrip() {
    let original: (Box<u32>, Box<String>) = (Box::new(314), Box::new(String::from("pi approx")));
    let enc = encode_to_vec(&original).expect("encode (Box<u32>, Box<String>)");
    let (dec, _): ((Box<u32>, Box<String>), _) =
        decode_from_slice(&enc).expect("decode (Box<u32>, Box<String>)");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 22. Byte size verification: Box<u32> encoding == plain u32 encoding
// ---------------------------------------------------------------------------

#[test]
fn test_box_u32_encoding_size_equals_u32() {
    let value: u32 = 42;
    let boxed: Box<u32> = Box::new(value);

    let enc_raw = encode_to_vec(&value).expect("encode u32");
    let enc_boxed = encode_to_vec(&boxed).expect("encode Box<u32>");

    assert_eq!(
        enc_raw, enc_boxed,
        "Box<u32> must encode identically to u32 (transparent encoding)"
    );
    assert_eq!(
        enc_raw.len(),
        enc_boxed.len(),
        "Box<u32> and u32 must have the same encoded byte size"
    );
}
