//! Advanced smart pointer encoding tests — level 3.
//!
//! Covers Box<T>, Rc<T>, and Arc<T> at a deeper level than previous test files.
//! All smart pointers are transparent wrappers whose wire format is identical to
//! encoding the inner value directly.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

// ── 1. Box<Vec<String>> roundtrip ─────────────────────────────────────────────

#[test]
fn test_box_vec_string_roundtrip() {
    let original: Box<Vec<String>> = Box::new(vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
    ]);
    let enc = encode_to_vec(&original).expect("encode Box<Vec<String>>");
    let (dec, _): (Box<Vec<String>>, usize) =
        decode_from_slice(&enc).expect("decode Box<Vec<String>>");
    assert_eq!(*original, *dec);
}

// ── 2. Box<Box<u32>> double-boxed roundtrip ───────────────────────────────────

#[test]
fn test_box_box_u32_roundtrip() {
    let original: Box<Box<u32>> = Box::new(Box::new(99u32));
    let enc = encode_to_vec(&original).expect("encode Box<Box<u32>>");
    let (dec, _): (Box<Box<u32>>, usize) = decode_from_slice(&enc).expect("decode Box<Box<u32>>");
    assert_eq!(**original, **dec);
}

// ── 3. Box<Option<String>> roundtrip ──────────────────────────────────────────

#[test]
fn test_box_option_string_roundtrip() {
    let original: Box<Option<String>> = Box::new(Some(String::from("boxed optional")));
    let enc = encode_to_vec(&original).expect("encode Box<Option<String>>");
    let (dec, _): (Box<Option<String>>, usize) =
        decode_from_slice(&enc).expect("decode Box<Option<String>>");
    assert_eq!(*original, *dec);
}

// ── 4. Box<(u32, String)> tuple roundtrip ─────────────────────────────────────

#[test]
fn test_box_tuple_u32_string_roundtrip() {
    let original: Box<(u32, String)> = Box::new((42u32, String::from("tuple in box")));
    let enc = encode_to_vec(&original).expect("encode Box<(u32, String)>");
    let (dec, _): (Box<(u32, String)>, usize) =
        decode_from_slice(&enc).expect("decode Box<(u32, String)>");
    assert_eq!(*original, *dec);
}

// ── 5. Vec<Box<String>> roundtrip ─────────────────────────────────────────────

#[test]
fn test_vec_box_string_roundtrip() {
    let original: Vec<Box<String>> = vec![
        Box::new(String::from("one")),
        Box::new(String::from("two")),
        Box::new(String::from("three")),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Box<String>>");
    let (dec, _): (Vec<Box<String>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Box<String>>");
    assert_eq!(original.len(), dec.len());
    for (a, b) in original.iter().zip(dec.iter()) {
        assert_eq!(**a, **b);
    }
}

// ── 6. Box encoding size == raw T encoding size for Box<u64> ──────────────────

#[test]
fn test_box_u64_encoding_size_equals_raw() {
    let value = 0xDEAD_BEEF_CAFE_BABEu64;
    let boxed: Box<u64> = Box::new(value);
    let raw_enc = encode_to_vec(&value).expect("encode raw u64");
    let box_enc = encode_to_vec(&boxed).expect("encode Box<u64>");
    assert_eq!(
        raw_enc.len(),
        box_enc.len(),
        "Box<u64> wire size must equal raw u64 wire size"
    );
    assert_eq!(
        raw_enc, box_enc,
        "Box<u64> wire bytes must be identical to raw u64"
    );
}

// ── 7. Box<[u8; 4]> bytes match raw [u8; 4] bytes ─────────────────────────────

#[test]
fn test_box_fixed_array_bytes_match_raw() {
    let raw: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
    let boxed: Box<[u8; 4]> = Box::new(raw);
    let raw_enc = encode_to_vec(&raw).expect("encode raw [u8; 4]");
    let box_enc = encode_to_vec(&boxed).expect("encode Box<[u8; 4]>");
    assert_eq!(
        raw_enc, box_enc,
        "Box<[u8; 4]> must produce identical bytes to raw [u8; 4]"
    );
    // Also verify roundtrip
    let (dec, _): (Box<[u8; 4]>, usize) = decode_from_slice(&box_enc).expect("decode Box<[u8; 4]>");
    assert_eq!(*dec, raw);
}

// ── 8. Rc<HashMap<String, u32>> roundtrip ─────────────────────────────────────

#[test]
fn test_rc_hashmap_string_u32_roundtrip() {
    let mut map = HashMap::new();
    map.insert(String::from("alpha"), 1u32);
    map.insert(String::from("beta"), 2u32);
    map.insert(String::from("gamma"), 3u32);
    let original: Rc<HashMap<String, u32>> = Rc::new(map);
    let enc = encode_to_vec(&original).expect("encode Rc<HashMap<String, u32>>");
    let (dec, _): (Rc<HashMap<String, u32>>, usize) =
        decode_from_slice(&enc).expect("decode Rc<HashMap<String, u32>>");
    assert_eq!(*original, *dec);
}

// ── 9. Rc<Vec<String>> roundtrip ──────────────────────────────────────────────

#[test]
fn test_rc_vec_string_roundtrip() {
    let original: Rc<Vec<String>> = Rc::new(vec![
        String::from("rc"),
        String::from("vec"),
        String::from("string"),
    ]);
    let enc = encode_to_vec(&original).expect("encode Rc<Vec<String>>");
    let (dec, _): (Rc<Vec<String>>, usize) =
        decode_from_slice(&enc).expect("decode Rc<Vec<String>>");
    assert_eq!(*original, *dec);
}

// ── 10. Rc<(u32, bool)> roundtrip ─────────────────────────────────────────────

#[test]
fn test_rc_tuple_u32_bool_roundtrip() {
    let original: Rc<(u32, bool)> = Rc::new((777u32, true));
    let enc = encode_to_vec(&original).expect("encode Rc<(u32, bool)>");
    let (dec, _): (Rc<(u32, bool)>, usize) =
        decode_from_slice(&enc).expect("decode Rc<(u32, bool)>");
    assert_eq!(*original, *dec);
}

// ── 11. Multiple Rc clones of same value all encode identically ───────────────

#[test]
fn test_rc_clones_encode_identically() {
    let original: Rc<u32> = Rc::new(12345u32);
    let clone1 = Rc::clone(&original);
    let clone2 = Rc::clone(&original);
    let clone3 = Rc::clone(&original);

    let enc_orig = encode_to_vec(&original).expect("encode original Rc");
    let enc_c1 = encode_to_vec(&clone1).expect("encode clone1 Rc");
    let enc_c2 = encode_to_vec(&clone2).expect("encode clone2 Rc");
    let enc_c3 = encode_to_vec(&clone3).expect("encode clone3 Rc");

    assert_eq!(
        enc_orig, enc_c1,
        "original and clone1 must encode identically"
    );
    assert_eq!(
        enc_orig, enc_c2,
        "original and clone2 must encode identically"
    );
    assert_eq!(
        enc_orig, enc_c3,
        "original and clone3 must encode identically"
    );
}

// ── 12. Rc encoding identical to Box encoding for same value ──────────────────

#[test]
fn test_rc_encoding_identical_to_box() {
    let value = 888u32;
    let rc_val: Rc<u32> = Rc::new(value);
    let box_val: Box<u32> = Box::new(value);
    let rc_enc = encode_to_vec(&rc_val).expect("encode Rc<u32>");
    let box_enc = encode_to_vec(&box_val).expect("encode Box<u32>");
    assert_eq!(
        rc_enc, box_enc,
        "Rc<u32> and Box<u32> must produce identical wire bytes for the same value"
    );
}

// ── 13. Arc<Vec<u8>> roundtrip ────────────────────────────────────────────────

#[test]
fn test_arc_vec_u8_roundtrip_advanced() {
    let original: Arc<Vec<u8>> = Arc::new(vec![1u8, 2, 4, 8, 16, 32, 64, 128, 255]);
    let enc = encode_to_vec(&original).expect("encode Arc<Vec<u8>>");
    let (dec, _): (Arc<Vec<u8>>, usize) = decode_from_slice(&enc).expect("decode Arc<Vec<u8>>");
    assert_eq!(*original, *dec);
}

// ── 14. Arc<String> bytes == raw String bytes ─────────────────────────────────

#[test]
fn test_arc_string_bytes_equal_raw_string_bytes() {
    let s = String::from("arc string wire format check");
    let arc_val: Arc<String> = Arc::new(s.clone());
    let raw_enc = encode_to_vec(&s).expect("encode raw String");
    let arc_enc = encode_to_vec(&arc_val).expect("encode Arc<String>");
    assert_eq!(
        raw_enc, arc_enc,
        "Arc<String> wire bytes must be identical to raw String bytes"
    );
}

// ── 15. Arc<HashMap<String, u32>> roundtrip ───────────────────────────────────

#[test]
fn test_arc_hashmap_string_u32_roundtrip() {
    let mut map = HashMap::new();
    map.insert(String::from("x"), 10u32);
    map.insert(String::from("y"), 20u32);
    let original: Arc<HashMap<String, u32>> = Arc::new(map);
    let enc = encode_to_vec(&original).expect("encode Arc<HashMap<String, u32>>");
    let (dec, _): (Arc<HashMap<String, u32>>, usize) =
        decode_from_slice(&enc).expect("decode Arc<HashMap<String, u32>>");
    assert_eq!(*original, *dec);
}

// ── 16. Arc<(f64, f64)> roundtrip ─────────────────────────────────────────────

#[test]
fn test_arc_tuple_f64_f64_roundtrip() {
    let original: Arc<(f64, f64)> = Arc::new((3.141592653589793f64, 2.718281828459045f64));
    let enc = encode_to_vec(&original).expect("encode Arc<(f64, f64)>");
    let (dec, _): (Arc<(f64, f64)>, usize) =
        decode_from_slice(&enc).expect("decode Arc<(f64, f64)>");
    let (orig_a, orig_b) = *original;
    let (dec_a, dec_b) = *dec;
    assert_eq!(orig_a, dec_a, "first f64 must match after roundtrip");
    assert_eq!(orig_b, dec_b, "second f64 must match after roundtrip");
}

// ── 17. Arc::clone encodes same bytes as original ─────────────────────────────

#[test]
fn test_arc_clone_encodes_same_bytes_as_original() {
    let original: Arc<String> = Arc::new(String::from("cloned arc test value"));
    let cloned = Arc::clone(&original);
    let orig_enc = encode_to_vec(&original).expect("encode original Arc<String>");
    let clone_enc = encode_to_vec(&cloned).expect("encode cloned Arc<String>");
    assert_eq!(
        orig_enc, clone_enc,
        "Arc::clone must produce identical wire bytes as original"
    );
}

// ── 18. Box<T>, Rc<T>, Arc<T> all encode same bytes for T=u32(42) ─────────────

#[test]
fn test_box_rc_arc_same_bytes_for_u32_42() {
    let value = 42u32;
    let box_val: Box<u32> = Box::new(value);
    let rc_val: Rc<u32> = Rc::new(value);
    let arc_val: Arc<u32> = Arc::new(value);

    let box_enc = encode_to_vec(&box_val).expect("encode Box<u32>=42");
    let rc_enc = encode_to_vec(&rc_val).expect("encode Rc<u32>=42");
    let arc_enc = encode_to_vec(&arc_val).expect("encode Arc<u32>=42");

    assert_eq!(box_enc, rc_enc, "Box and Rc must produce identical bytes");
    assert_eq!(box_enc, arc_enc, "Box and Arc must produce identical bytes");
    assert_eq!(rc_enc, arc_enc, "Rc and Arc must produce identical bytes");
}

// ── 19. Vec<Arc<u32>> roundtrip ───────────────────────────────────────────────

#[test]
fn test_vec_arc_u32_roundtrip() {
    let original: Vec<Arc<u32>> = vec![
        Arc::new(100u32),
        Arc::new(200u32),
        Arc::new(300u32),
        Arc::new(400u32),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Arc<u32>>");
    let (dec, _): (Vec<Arc<u32>>, usize) = decode_from_slice(&enc).expect("decode Vec<Arc<u32>>");
    assert_eq!(original.len(), dec.len());
    for (a, b) in original.iter().zip(dec.iter()) {
        assert_eq!(**a, **b);
    }
}

// ── 20. Option<Box<String>> Some roundtrip ────────────────────────────────────

#[test]
fn test_option_box_string_some_roundtrip() {
    let original: Option<Box<String>> = Some(Box::new(String::from("optional boxed string")));
    let enc = encode_to_vec(&original).expect("encode Option<Box<String>> Some");
    let (dec, _): (Option<Box<String>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Box<String>> Some");
    assert!(dec.is_some(), "decoded value must be Some");
    let orig_inner = original.as_ref().expect("original Some").as_ref();
    let dec_inner = dec.as_ref().expect("decoded Some").as_ref();
    assert_eq!(orig_inner, dec_inner);
}

// ── 21. Option<Rc<String>> None roundtrip ─────────────────────────────────────

#[test]
fn test_option_rc_string_none_roundtrip() {
    let original: Option<Rc<String>> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Rc<String>> None");
    let (dec, _): (Option<Rc<String>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Rc<String>> None");
    assert!(dec.is_none(), "decoded Option<Rc<String>> must be None");
}

// ── 22. Struct containing Box<String> and Arc<u32> fields roundtrip ───────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SmartPtrStruct {
    name: Box<String>,
    id: std::sync::Arc<u32>,
}

#[test]
fn test_struct_with_box_and_arc_fields_roundtrip() {
    let original = SmartPtrStruct {
        name: Box::new(String::from("smart pointer struct")),
        id: Arc::new(9999u32),
    };
    let enc = encode_to_vec(&original).expect("encode SmartPtrStruct");
    let (dec, _): (SmartPtrStruct, usize) = decode_from_slice(&enc).expect("decode SmartPtrStruct");
    assert_eq!(*original.name, *dec.name, "name field must match");
    assert_eq!(*original.id, *dec.id, "id field must match");
}
