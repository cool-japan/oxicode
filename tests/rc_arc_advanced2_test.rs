//! Advanced tests for Rc<T>/Arc<T>/Box<T> serialization in OxiCode.
//!
//! Box<T>, Rc<T>, and Arc<T> encode as transparent wrappers — their wire format
//! is identical to encoding the inner value directly.

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
use std::rc::Rc;
use std::sync::Arc;

// ── 1. Box<u32> roundtrip value=42 ────────────────────────────────────────────

#[test]
fn test_box_u32_roundtrip_42() {
    let original: Box<u32> = Box::new(42u32);
    let enc = encode_to_vec(&original).expect("encode Box<u32>=42");
    let (dec, _): (Box<u32>, usize) = decode_from_slice(&enc).expect("decode Box<u32>=42");
    assert_eq!(*original, *dec);
}

// ── 2. Box<u32> roundtrip value=0 ─────────────────────────────────────────────

#[test]
fn test_box_u32_roundtrip_zero() {
    let original: Box<u32> = Box::new(0u32);
    let enc = encode_to_vec(&original).expect("encode Box<u32>=0");
    let (dec, _): (Box<u32>, usize) = decode_from_slice(&enc).expect("decode Box<u32>=0");
    assert_eq!(*original, *dec);
}

// ── 3. Box<u32> encoded size same as raw u32 ──────────────────────────────────

#[test]
fn test_box_u32_same_size_as_raw_u32() {
    let value = 99u32;
    let boxed: Box<u32> = Box::new(value);
    let raw_enc = encode_to_vec(&value).expect("encode raw u32");
    let box_enc = encode_to_vec(&boxed).expect("encode Box<u32>");
    assert_eq!(
        raw_enc.len(),
        box_enc.len(),
        "Box<u32> must have identical wire size to raw u32"
    );
    assert_eq!(
        raw_enc, box_enc,
        "Box<u32> must have identical wire bytes to raw u32"
    );
}

// ── 4. Box<String> roundtrip ──────────────────────────────────────────────────

#[test]
fn test_box_string_roundtrip() {
    let original: Box<String> = Box::new(String::from("hello, oxicode!"));
    let enc = encode_to_vec(&original).expect("encode Box<String>");
    let (dec, _): (Box<String>, usize) = decode_from_slice(&enc).expect("decode Box<String>");
    assert_eq!(*original, *dec);
}

// ── 5. Box<Vec<u8>> roundtrip ─────────────────────────────────────────────────

#[test]
fn test_box_vec_u8_roundtrip() {
    let original: Box<Vec<u8>> = Box::new(vec![10u8, 20, 30, 40, 50]);
    let enc = encode_to_vec(&original).expect("encode Box<Vec<u8>>");
    let (dec, _): (Box<Vec<u8>>, usize) = decode_from_slice(&enc).expect("decode Box<Vec<u8>>");
    assert_eq!(*original, *dec);
}

// ── 6. Box<u64> consumed == encoded.len() ─────────────────────────────────────

#[test]
fn test_box_u64_consumed_equals_encoded_len() {
    let original: Box<u64> = Box::new(123_456_789u64);
    let enc = encode_to_vec(&original).expect("encode Box<u64>");
    let (_dec, consumed): (Box<u64>, usize) = decode_from_slice(&enc).expect("decode Box<u64>");
    assert_eq!(
        consumed,
        enc.len(),
        "bytes consumed must equal total encoded length"
    );
}

// ── 7. Rc<u32> roundtrip ──────────────────────────────────────────────────────

#[test]
fn test_rc_u32_roundtrip() {
    let original: Rc<u32> = Rc::new(7u32);
    let enc = encode_to_vec(&original).expect("encode Rc<u32>");
    let (dec, _): (Rc<u32>, usize) = decode_from_slice(&enc).expect("decode Rc<u32>");
    assert_eq!(*original, *dec);
}

// ── 8. Rc<u32> value preserved after roundtrip ────────────────────────────────

#[test]
fn test_rc_u32_value_preserved() {
    let value = 255u32;
    let original: Rc<u32> = Rc::new(value);
    let enc = encode_to_vec(&original).expect("encode Rc<u32>");
    let (dec, _): (Rc<u32>, usize) = decode_from_slice(&enc).expect("decode Rc<u32>");
    assert_eq!(*dec, value, "decoded Rc<u32> must preserve original value");
}

// ── 9. Rc<String> roundtrip ───────────────────────────────────────────────────

#[test]
fn test_rc_string_roundtrip() {
    let original: Rc<String> = Rc::new(String::from("rc string test"));
    let enc = encode_to_vec(&original).expect("encode Rc<String>");
    let (dec, _): (Rc<String>, usize) = decode_from_slice(&enc).expect("decode Rc<String>");
    assert_eq!(*original, *dec);
}

// ── 10. Rc<Vec<u32>> roundtrip ────────────────────────────────────────────────

#[test]
fn test_rc_vec_u32_roundtrip() {
    let original: Rc<Vec<u32>> = Rc::new(vec![1u32, 2, 3, 100, 200]);
    let enc = encode_to_vec(&original).expect("encode Rc<Vec<u32>>");
    let (dec, _): (Rc<Vec<u32>>, usize) = decode_from_slice(&enc).expect("decode Rc<Vec<u32>>");
    assert_eq!(*original, *dec);
}

// ── 11. Arc<u32> roundtrip ────────────────────────────────────────────────────

#[test]
fn test_arc_u32_roundtrip() {
    let original: Arc<u32> = Arc::new(42u32);
    let enc = encode_to_vec(&original).expect("encode Arc<u32>");
    let (dec, _): (Arc<u32>, usize) = decode_from_slice(&enc).expect("decode Arc<u32>");
    assert_eq!(*original, *dec);
}

// ── 12. Arc<String> roundtrip ─────────────────────────────────────────────────

#[test]
fn test_arc_string_roundtrip() {
    let original: Arc<String> = Arc::new(String::from("arc string test"));
    let enc = encode_to_vec(&original).expect("encode Arc<String>");
    let (dec, _): (Arc<String>, usize) = decode_from_slice(&enc).expect("decode Arc<String>");
    assert_eq!(*original, *dec);
}

// ── 13. Arc<Vec<u8>> roundtrip ────────────────────────────────────────────────

#[test]
fn test_arc_vec_u8_roundtrip() {
    let original: Arc<Vec<u8>> = Arc::new(vec![0u8, 127, 255]);
    let enc = encode_to_vec(&original).expect("encode Arc<Vec<u8>>");
    let (dec, _): (Arc<Vec<u8>>, usize) = decode_from_slice(&enc).expect("decode Arc<Vec<u8>>");
    assert_eq!(*original, *dec);
}

// ── 14. Vec<Box<u32>> roundtrip ───────────────────────────────────────────────

#[test]
fn test_vec_box_u32_roundtrip() {
    let original: Vec<Box<u32>> = vec![Box::new(1u32), Box::new(2u32), Box::new(3u32)];
    let enc = encode_to_vec(&original).expect("encode Vec<Box<u32>>");
    let (dec, _): (Vec<Box<u32>>, usize) = decode_from_slice(&enc).expect("decode Vec<Box<u32>>");
    assert_eq!(original.len(), dec.len());
    for (a, b) in original.iter().zip(dec.iter()) {
        assert_eq!(**a, **b);
    }
}

// ── 15. Vec<Rc<u32>> roundtrip ────────────────────────────────────────────────
// Rc is not Send, but encode/decode on a single thread is fine.

#[test]
fn test_vec_rc_u32_roundtrip() {
    let original: Vec<Rc<u32>> = vec![Rc::new(10u32), Rc::new(20u32), Rc::new(30u32)];
    let enc = encode_to_vec(&original).expect("encode Vec<Rc<u32>>");
    let (dec, _): (Vec<Rc<u32>>, usize) = decode_from_slice(&enc).expect("decode Vec<Rc<u32>>");
    assert_eq!(original.len(), dec.len());
    for (a, b) in original.iter().zip(dec.iter()) {
        assert_eq!(**a, **b);
    }
}

// ── 16. Option<Box<u32>> Some roundtrip ───────────────────────────────────────

#[test]
fn test_option_box_u32_some_roundtrip() {
    let original: Option<Box<u32>> = Some(Box::new(77u32));
    let enc = encode_to_vec(&original).expect("encode Option<Box<u32>> Some");
    let (dec, _): (Option<Box<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Box<u32>> Some");
    assert!(dec.is_some(), "decoded Option must be Some");
    let orig_inner: u32 = **original.as_ref().expect("original");
    let dec_inner: u32 = *dec.expect("dec");
    assert_eq!(orig_inner, dec_inner);
}

// ── 17. Option<Box<u32>> None roundtrip ───────────────────────────────────────

#[test]
fn test_option_box_u32_none_roundtrip() {
    let original: Option<Box<u32>> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Box<u32>> None");
    let (dec, _): (Option<Box<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Box<u32>> None");
    assert!(dec.is_none(), "decoded Option must be None");
}

// ── 18. Fixed-int config with Box<u32> ────────────────────────────────────────

#[test]
fn test_box_u32_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: Box<u32> = Box::new(1u32);
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode Box<u32> fixed-int");
    let (dec, _): (Box<u32>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Box<u32> fixed-int");
    // Fixed-int u32 is always exactly 4 bytes
    assert_eq!(enc.len(), 4, "fixed-int u32 must be 4 bytes");
    assert_eq!(*original, *dec);
}

// ── 19. Big-endian config with Box<u32> ───────────────────────────────────────

#[test]
fn test_box_u32_big_endian_config() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value = 0x0102_0304u32;
    let original: Box<u32> = Box::new(value);
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode Box<u32> big-endian");
    let (dec, _): (Box<u32>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Box<u32> big-endian");
    assert_eq!(*original, *dec);
    // Big-endian fixed-int: most-significant byte first
    assert_eq!(
        enc[0], 0x01,
        "big-endian most-significant byte must be 0x01"
    );
    assert_eq!(enc[1], 0x02);
    assert_eq!(enc[2], 0x03);
    assert_eq!(enc[3], 0x04);
}

// ── 20. Box<[u8; 4]> roundtrip fixed array ────────────────────────────────────

#[test]
fn test_box_fixed_array_u8_roundtrip() {
    let original: Box<[u8; 4]> = Box::new([0xDE, 0xAD, 0xBE, 0xEF]);
    let enc = encode_to_vec(&original).expect("encode Box<[u8; 4]>");
    let (dec, _): (Box<[u8; 4]>, usize) = decode_from_slice(&enc).expect("decode Box<[u8; 4]>");
    assert_eq!(*original, *dec);
}

// ── 21. Rc<u32> and Arc<u32> encode to same bytes for same value ──────────────

#[test]
fn test_rc_arc_u32_same_wire_bytes() {
    let value = 12345u32;
    let rc_val: Rc<u32> = Rc::new(value);
    let arc_val: Arc<u32> = Arc::new(value);
    let rc_enc = encode_to_vec(&rc_val).expect("encode Rc<u32>");
    let arc_enc = encode_to_vec(&arc_val).expect("encode Arc<u32>");
    assert_eq!(
        rc_enc, arc_enc,
        "Rc<u32> and Arc<u32> must produce identical wire bytes for the same value"
    );
}

// ── 22. Box<bool> roundtrip both true and false ───────────────────────────────

#[test]
fn test_box_bool_roundtrip_true_and_false() {
    let original_true: Box<bool> = Box::new(true);
    let enc_true = encode_to_vec(&original_true).expect("encode Box<bool>=true");
    let (dec_true, _): (Box<bool>, usize) =
        decode_from_slice(&enc_true).expect("decode Box<bool>=true");
    assert!(*dec_true, "decoded Box<bool> for true must be true");

    let original_false: Box<bool> = Box::new(false);
    let enc_false = encode_to_vec(&original_false).expect("encode Box<bool>=false");
    let (dec_false, _): (Box<bool>, usize) =
        decode_from_slice(&enc_false).expect("decode Box<bool>=false");
    assert!(!*dec_false, "decoded Box<bool> for false must be false");

    // The wire bytes must differ between true and false
    assert_ne!(
        enc_true, enc_false,
        "true and false must have distinct wire encodings"
    );
}
