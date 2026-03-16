//! Advanced tests for Option encoding/decoding in oxicode.
//!
//! These tests probe wire-format exactness, nested generics, struct derive,
//! BorrowDecode, sequential stress, and error paths — all distinct from the
//! coverage already present in option_result_test.rs.

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
use oxicode::{borrow_decode_from_slice, decode_from_slice, encode_to_vec, encoded_size};

// ── helpers ──────────────────────────────────────────────────────────────────

/// Encode `v` and assert the raw bytes equal `expected`.
fn assert_bytes<T: oxicode::enc::Encode + std::fmt::Debug>(v: &T, expected: &[u8]) {
    let enc = encode_to_vec(v).expect("encode");
    assert_eq!(enc.as_slice(), expected, "wire bytes mismatch for {:?}", v);
}

// ── 1. None::<u8> → [0x00] ───────────────────────────────────────────────────

#[test]
fn test_none_u8_wire_bytes() {
    assert_bytes(&Option::<u8>::None, &[0x00]);
}

// ── 2. Some(0u8) → [0x01, 0x00] ─────────────────────────────────────────────

#[test]
fn test_some_zero_u8_wire_bytes() {
    assert_bytes(&Some(0u8), &[0x01, 0x00]);
}

// ── 3. Some(42u8) → [0x01, 0x2A] ────────────────────────────────────────────

#[test]
fn test_some_42_u8_wire_bytes() {
    assert_bytes(&Some(42u8), &[0x01, 0x2A]);
}

// ── 4. Option::<u64>::None encodes to exactly 1 byte ─────────────────────────

#[test]
fn test_option_u64_none_is_1_byte() {
    let enc = encode_to_vec(&Option::<u64>::None).expect("encode");
    assert_eq!(enc.len(), 1, "None must encode to exactly 1 byte");
    assert_eq!(enc[0], 0x00);
}

// ── 5. Some(300u64) encodes to 4 bytes: tag(1) + varint u16 tag(1) + 2LE ────
//   varint for 300 (0x012C): 300 > 250, encoded as [251, 0x2C, 0x01]  → 3 bytes
//   Option tag: 1 byte  →  total 4 bytes

#[test]
fn test_option_u64_some_300_wire_bytes() {
    // 300 = 0x012C; varint encoding for 300: tag 251 + LE u16 [0x2C, 0x01]
    let expected: &[u8] = &[0x01, 251, 0x2C, 0x01];
    assert_bytes(&Some(300u64), expected);
    let enc = encode_to_vec(&Some(300u64)).expect("encode");
    assert_eq!(enc.len(), 4);
}

// ── 6. Option::<String>::None roundtrip ──────────────────────────────────────

#[test]
fn test_option_string_none_roundtrip() {
    let v: Option<String> = None;
    let enc = encode_to_vec(&v).expect("encode");
    assert_eq!(enc.len(), 1);
    let (dec, consumed): (Option<String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());
}

// ── 7. Option::<String>::Some("hello") roundtrip ─────────────────────────────

#[test]
fn test_option_string_some_hello_roundtrip() {
    let v: Option<String> = Some("hello".to_string());
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, consumed): (Option<String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());
    // Must be more than 1 byte and start with the Some tag
    assert!(enc.len() > 1);
    assert_eq!(enc[0], 0x01);
}

// ── 8. Option::<Option<u32>>::None (outer None) ──────────────────────────────

#[test]
fn test_nested_option_outer_none_wire() {
    let v: Option<Option<u32>> = None;
    let enc = encode_to_vec(&v).expect("encode");
    // Outer None → single 0x00 byte
    assert_eq!(enc, vec![0x00u8]);
}

// ── 9. None vs Some(None) produce different encodings ───────────────────────

#[test]
fn test_nested_option_none_vs_some_none_differ() {
    let outer_none: Option<Option<u32>> = None;
    let some_none: Option<Option<u32>> = Some(None);

    let enc_outer = encode_to_vec(&outer_none).expect("encode outer");
    let enc_some_none = encode_to_vec(&some_none).expect("encode some_none");

    assert_ne!(
        enc_outer, enc_some_none,
        "None and Some(None) must have different wire representations"
    );
    // Outer None = [0x00]; Some(None) = [0x01, 0x00]
    assert_eq!(enc_outer, vec![0x00u8]);
    assert_eq!(enc_some_none, vec![0x01u8, 0x00u8]);

    // Verify round-trip for Some(None)
    let (dec, _): (Option<Option<u32>>, _) = decode_from_slice(&enc_some_none).expect("decode");
    assert_eq!(dec, some_none);
}

// ── 10. Some(Some(42u32)) encodes and decodes correctly ──────────────────────

#[test]
fn test_nested_option_some_some_42_roundtrip() {
    let v: Option<Option<u32>> = Some(Some(42));
    let enc = encode_to_vec(&v).expect("encode");
    // [0x01 (outer Some), 0x01 (inner Some), varint(42) = 0x2A]
    assert_eq!(enc, vec![0x01u8, 0x01u8, 0x2Au8]);
    let (dec, _): (Option<Option<u32>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, v);
}

// ── 11. Option::<Vec<u8>>::Some with data roundtrip ──────────────────────────

#[test]
fn test_option_vec_u8_with_data_roundtrip() {
    let payload: Vec<u8> = (0u8..=15u8).collect();
    let v: Option<Vec<u8>> = Some(payload.clone());
    let enc = encode_to_vec(&v).expect("encode");
    // Starts with Some tag
    assert_eq!(enc[0], 0x01u8);
    let (dec, consumed): (Option<Vec<u8>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, Some(payload));
    assert_eq!(consumed, enc.len());
}

// ── 12. Vec<Option<u32>> with mixed None/Some values ─────────────────────────

#[test]
fn test_vec_of_option_u32_mixed_roundtrip() {
    let v: Vec<Option<u32>> = vec![
        None,
        Some(0),
        None,
        Some(u32::MAX),
        Some(1),
        None,
        Some(250),
        Some(251),
    ];
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, consumed): (Vec<Option<u32>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());
}

// ── 13. Option inside a derive-annotated struct ───────────────────────────────

#[test]
fn test_option_in_derived_struct() {
    use oxicode_derive::{Decode, Encode};

    #[derive(Encode, Decode, Debug, PartialEq)]
    struct Record {
        id: u64,
        tag: Option<u32>,
        label: Option<String>,
    }

    let r = Record {
        id: 9_999_999,
        tag: Some(7),
        label: None,
    };
    let enc = encode_to_vec(&r).expect("encode");
    let (dec, consumed): (Record, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, r);
    assert_eq!(consumed, enc.len());

    // Also test with both fields populated
    let r2 = Record {
        id: 1,
        tag: None,
        label: Some("oxicode".to_string()),
    };
    let enc2 = encode_to_vec(&r2).expect("encode r2");
    let (dec2, _): (Record, _) = decode_from_slice(&enc2).expect("decode r2");
    assert_eq!(dec2, r2);
}

// ── 14. Option<Box<String>> roundtrip ────────────────────────────────────────

#[test]
fn test_option_box_string_roundtrip() {
    let v: Option<Box<String>> = Some(Box::new("boxed value".to_string()));
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, consumed): (Option<Box<String>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());

    let none_v: Option<Box<String>> = None;
    let enc_none = encode_to_vec(&none_v).expect("encode none");
    assert_eq!(enc_none.len(), 1);
    let (dec_none, _): (Option<Box<String>>, _) =
        decode_from_slice(&enc_none).expect("decode none");
    assert_eq!(dec_none, none_v);
}

// ── 15. Option<(u32, u64)> tuple roundtrip ───────────────────────────────────

#[test]
fn test_option_tuple_u32_u64_roundtrip() {
    let none_v: Option<(u32, u64)> = None;
    let some_v: Option<(u32, u64)> = Some((42u32, 1_000_000u64));

    for v in [none_v, some_v] {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, consumed): (Option<(u32, u64)>, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(dec, v);
        assert_eq!(consumed, enc.len());
    }
}

// ── 16. Option<f64> None and Some roundtrip ──────────────────────────────────

#[test]
fn test_option_f64_none_and_some_roundtrip() {
    let cases: &[Option<f64>] = &[
        None,
        Some(0.0),
        Some(-1.5),
        Some(f64::INFINITY),
        Some(f64::NEG_INFINITY),
        Some(f64::NAN),
        Some(f64::MAX),
        Some(f64::MIN),
    ];

    for &v in cases {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, consumed): (Option<f64>, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(consumed, enc.len());
        match (v, dec) {
            (None, None) => {}
            (Some(a), Some(b)) if a.is_nan() => assert!(b.is_nan()),
            (Some(a), Some(b)) => assert_eq!(a.to_bits(), b.to_bits()),
            _ => panic!("option variant mismatch"),
        }
    }
}

// ── 17. Option<bool>: all three meaningful variants ──────────────────────────

#[test]
fn test_option_bool_all_variants() {
    let cases: &[Option<bool>] = &[None, Some(false), Some(true)];
    for &v in cases {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, _): (Option<bool>, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(dec, v);
    }
    // Wire check: None=[0], Some(false)=[1,0], Some(true)=[1,1]
    assert_bytes(&Option::<bool>::None, &[0x00]);
    assert_bytes(&Some(false), &[0x01, 0x00]);
    assert_bytes(&Some(true), &[0x01, 0x01]);
}

// ── 18. Sequential encode/decode of 100 Option<u32> values ───────────────────

#[test]
fn test_sequential_encode_decode_100_option_u32() {
    let source: Vec<Option<u32>> = (0u32..100)
        .map(|i| if i % 3 == 0 { None } else { Some(i * 997) })
        .collect();

    for v in &source {
        let enc = encode_to_vec(v).expect("encode");
        let (dec, consumed): (Option<u32>, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(&dec, v);
        assert_eq!(consumed, enc.len());
    }
}

// ── 19. Option<Result<u32, u32>> – all four variant combinations ──────────────

#[test]
fn test_option_result_all_variants() {
    let cases: Vec<Option<Result<u32, u32>>> = vec![
        None,
        Some(Ok(0)),
        Some(Ok(u32::MAX)),
        Some(Err(0)),
        Some(Err(u32::MAX)),
    ];

    for v in &cases {
        let enc = encode_to_vec(v).expect("encode");
        let (dec, consumed): (Option<Result<u32, u32>>, _) =
            decode_from_slice(&enc).expect("decode");
        assert_eq!(&dec, v);
        assert_eq!(consumed, enc.len());
    }
}

// ── 20. encoded_size agrees with actual Vec length for None and Some ──────────

#[test]
fn test_encoded_size_none_vs_some() {
    let none_v: Option<u64> = None;
    let some_v: Option<u64> = Some(1);

    let size_none = encoded_size(&none_v).expect("size none");
    let size_some = encoded_size(&some_v).expect("size some");

    let enc_none = encode_to_vec(&none_v).expect("enc none");
    let enc_some = encode_to_vec(&some_v).expect("enc some");

    assert_eq!(
        size_none,
        enc_none.len(),
        "encoded_size must match vec len for None"
    );
    assert_eq!(
        size_some,
        enc_some.len(),
        "encoded_size must match vec len for Some"
    );

    // None is always strictly smaller than Some(any value)
    assert!(
        size_none < size_some,
        "None ({size_none} bytes) must be smaller than Some ({size_some} bytes)"
    );
    assert_eq!(size_none, 1);
}

// ── 21. BorrowDecode for Option<&str> (zero-copy) ────────────────────────────

#[test]
fn test_borrow_decode_option_str_some() {
    let owned: Option<String> = Some("zero-copy test".to_string());
    let enc = encode_to_vec(&owned).expect("encode");

    let (dec, consumed): (Option<&str>, _) = borrow_decode_from_slice(&enc).expect("borrow decode");
    assert_eq!(dec, Some("zero-copy test"));
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_borrow_decode_option_str_none() {
    let owned: Option<String> = None;
    let enc = encode_to_vec(&owned).expect("encode");

    let (dec, consumed): (Option<&str>, _) =
        borrow_decode_from_slice(&enc).expect("borrow decode none");
    assert_eq!(dec, None);
    assert_eq!(consumed, 1);
}

// ── 22. Decode error on truncated data ───────────────────────────────────────

#[test]
fn test_option_decode_error_on_truncated_data() {
    // A full Some(42u32) encoding starts with [0x01, 0x2A].
    // Feed only the Some tag [0x01] without the payload — must fail.
    let truncated: &[u8] = &[0x01];
    let result: oxicode::Result<(Option<u32>, usize)> = decode_from_slice(truncated);
    assert!(
        result.is_err(),
        "decoding truncated Some payload must return an error"
    );

    // Similarly, a completely empty slice must fail (not even the tag byte).
    let empty: &[u8] = &[];
    let result2: oxicode::Result<(Option<u32>, usize)> = decode_from_slice(empty);
    assert!(
        result2.is_err(),
        "decoding from empty slice must return an error"
    );
}
