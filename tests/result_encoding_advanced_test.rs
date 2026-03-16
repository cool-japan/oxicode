//! Advanced tests for Result<T, E> encoding/decoding.
//!
//! These tests exercise byte-format guarantees, edge cases, nested types,
//! derived structs, large numbers, truncation errors, and complex generics.
//! They complement the basic coverage in option_result_test.rs.

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
use oxicode::{config, decode_from_slice, encode_to_vec, encoded_size};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// 1. Ok(42u32) encodes to [0x00, ...value...] (tag 0 for Ok)
// ---------------------------------------------------------------------------
#[test]
fn test_result_ok_tag_is_zero() {
    let v: Result<u32, u32> = Ok(42);
    let enc = encode_to_vec(&v).expect("encode");
    assert_eq!(enc[0], 0x00, "Ok variant must encode with tag byte 0x00");
}

// ---------------------------------------------------------------------------
// 2. Err(42u32) encodes to [0x01, ...value...] (tag 1 for Err)
// ---------------------------------------------------------------------------
#[test]
fn test_result_err_tag_is_one() {
    let v: Result<u32, u32> = Err(42);
    let enc = encode_to_vec(&v).expect("encode");
    assert_eq!(enc[0], 0x01, "Err variant must encode with tag byte 0x01");
}

// ---------------------------------------------------------------------------
// 3. Result<u32, u32> Ok(0) round-trip
// ---------------------------------------------------------------------------
#[test]
fn test_result_u32_ok_zero_roundtrip() {
    let v: Result<u32, u32> = Ok(0);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<u32, u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 4. Result<u32, u32> Err(0) round-trip
// ---------------------------------------------------------------------------
#[test]
fn test_result_u32_err_zero_roundtrip() {
    let v: Result<u32, u32> = Err(0);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<u32, u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 5. Result<String, String> Ok("hello")
// ---------------------------------------------------------------------------
#[test]
fn test_result_string_ok_roundtrip() {
    let v: Result<String, String> = Ok("hello".to_string());
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<String, String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 6. Result<String, String> Err("error message")
// ---------------------------------------------------------------------------
#[test]
fn test_result_string_err_roundtrip() {
    let v: Result<String, String> = Err("error message".to_string());
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<String, String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 7. Result<Vec<u8>, u32> Ok with data
// ---------------------------------------------------------------------------
#[test]
fn test_result_vec_ok_roundtrip() {
    let v: Result<Vec<u8>, u32> = Ok(vec![0xde, 0xad, 0xbe, 0xef, 0x00, 0xff]);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<Vec<u8>, u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 8. Result<u32, Vec<u8>> Err with data
// ---------------------------------------------------------------------------
#[test]
fn test_result_err_vec_roundtrip() {
    let v: Result<u32, Vec<u8>> = Err(vec![1, 2, 3, 255, 0]);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<u32, Vec<u8>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 9. Result<(), ()> Ok(())
// ---------------------------------------------------------------------------
#[test]
fn test_result_unit_ok_roundtrip() {
    let v: Result<(), ()> = Ok(());
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<(), ()>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 10. Result<(), ()> Err(())
// ---------------------------------------------------------------------------
#[test]
fn test_result_unit_err_roundtrip() {
    let v: Result<(), ()> = Err(());
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<(), ()>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 11. Vec<Result<u32, String>> mixed
// ---------------------------------------------------------------------------
#[test]
fn test_vec_of_mixed_results_roundtrip() {
    let v: Vec<Result<u32, String>> = vec![
        Ok(0),
        Err("first error".to_string()),
        Ok(u32::MAX),
        Err(String::new()),
        Ok(12345),
    ];
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Vec<Result<u32, String>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 12. Result<Option<u32>, String> nested (Ok(None), Ok(Some(_)), Err(_))
// ---------------------------------------------------------------------------
#[test]
fn test_result_nested_option_all_variants() {
    let cases: Vec<Result<Option<u32>, String>> = vec![
        Ok(None),
        Ok(Some(0)),
        Ok(Some(u32::MAX)),
        Err("nested err".to_string()),
    ];
    for v in &cases {
        let enc = encode_to_vec(v).expect("encode");
        let (dec, _): (Result<Option<u32>, String>, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(*v, dec, "round-trip failed for {:?}", v);
    }
}

// ---------------------------------------------------------------------------
// 13. Result in struct with derive
// ---------------------------------------------------------------------------
#[test]
fn test_result_in_derived_struct() {
    #[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
    struct Report {
        id: u32,
        outcome: Result<String, u32>,
        code: i16,
    }

    let ok_report = Report {
        id: 1,
        outcome: Ok("success".to_string()),
        code: 200,
    };
    let err_report = Report {
        id: 2,
        outcome: Err(404),
        code: -1,
    };

    for r in [&ok_report, &err_report] {
        let enc = encode_to_vec(r).expect("encode");
        let (dec, _): (Report, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(*r, dec);
    }
}

// ---------------------------------------------------------------------------
// 14. Result<Result<u32, u32>, u32> double nested
// ---------------------------------------------------------------------------
#[test]
fn test_result_double_nested_roundtrip() {
    let cases: Vec<Result<Result<u32, u32>, u32>> =
        vec![Ok(Ok(0)), Ok(Ok(999)), Ok(Err(1)), Err(2)];
    for v in &cases {
        let enc = encode_to_vec(v).expect("encode");
        let (dec, _): (Result<Result<u32, u32>, u32>, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(*v, dec, "round-trip failed for {:?}", v);
    }
}

// ---------------------------------------------------------------------------
// 15. Result<u128, u128> large numbers
// ---------------------------------------------------------------------------
#[test]
fn test_result_u128_large_numbers_roundtrip() {
    let large = u128::MAX;
    let ok_val: Result<u128, u128> = Ok(large);
    let err_val: Result<u128, u128> = Err(large - 1);

    let enc_ok = encode_to_vec(&ok_val).expect("encode ok");
    let (dec_ok, _): (Result<u128, u128>, _) = decode_from_slice(&enc_ok).expect("decode ok");
    assert_eq!(ok_val, dec_ok);

    let enc_err = encode_to_vec(&err_val).expect("encode err");
    let (dec_err, _): (Result<u128, u128>, _) = decode_from_slice(&enc_err).expect("decode err");
    assert_eq!(err_val, dec_err);
}

// ---------------------------------------------------------------------------
// 16. Sequential 100 Results encode/decode
// ---------------------------------------------------------------------------
#[test]
fn test_sequential_100_results_roundtrip() {
    let results: Vec<Result<u64, String>> = (0u64..100)
        .map(|i| {
            if i % 2 == 0 {
                Ok(i * i)
            } else {
                Err(format!("err-{}", i))
            }
        })
        .collect();

    let enc = encode_to_vec(&results).expect("encode");
    let (dec, consumed): (Vec<Result<u64, String>>, _) = decode_from_slice(&enc).expect("decode");

    assert_eq!(results, dec);
    assert_eq!(consumed, enc.len(), "all bytes should be consumed");
}

// ---------------------------------------------------------------------------
// 17. encoded_size for Ok vs Err
// ---------------------------------------------------------------------------
#[test]
fn test_encoded_size_ok_vs_err() {
    // Both carry the same-sized payload (u32), so sizes should be equal
    let ok_val: Result<u32, u32> = Ok(1);
    let err_val: Result<u32, u32> = Err(1);

    let size_ok = encoded_size(&ok_val).expect("size ok");
    let size_err = encoded_size(&err_val).expect("size err");
    assert_eq!(
        size_ok, size_err,
        "Ok and Err with same-size payload must have equal encoded size"
    );

    // The size must equal the actual encoded length
    let enc_ok = encode_to_vec(&ok_val).expect("encode");
    assert_eq!(size_ok, enc_ok.len());
}

// ---------------------------------------------------------------------------
// 18. byte format: first byte is 0 for Ok, 1 for Err (with fixed-int config)
// ---------------------------------------------------------------------------
#[test]
fn test_result_byte_format_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();

    let ok_val: Result<u32, u32> = Ok(0xABCD_1234);
    let err_val: Result<u32, u32> = Err(0xABCD_1234);

    let enc_ok = oxicode::encode_to_vec_with_config(&ok_val, cfg).expect("encode ok");
    let enc_err = oxicode::encode_to_vec_with_config(&err_val, cfg).expect("encode err");

    assert_eq!(
        enc_ok[0], 0x00,
        "Ok tag must be 0x00 under fixed-int config"
    );
    assert_eq!(
        enc_err[0], 0x01,
        "Err tag must be 0x01 under fixed-int config"
    );

    // Payload bytes (everything after tag) must be identical
    assert_eq!(
        &enc_ok[1..],
        &enc_err[1..],
        "payload bytes of Ok and Err with same value must be equal"
    );
}

// ---------------------------------------------------------------------------
// 19. Result<f64, String> Ok(NaN) — compare via to_bits
// ---------------------------------------------------------------------------
#[test]
fn test_result_ok_nan_roundtrip() {
    let nan = f64::NAN;
    let v: Result<f64, String> = Ok(nan);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<f64, String>, _) = decode_from_slice(&enc).expect("decode");

    match (v, dec) {
        (Ok(orig), Ok(recovered)) => {
            assert_eq!(
                orig.to_bits(),
                recovered.to_bits(),
                "NaN bit pattern must survive round-trip"
            );
        }
        _ => panic!("expected Ok(NaN) on both sides"),
    }
}

// ---------------------------------------------------------------------------
// 20. Decode truncated Ok data gives error
// ---------------------------------------------------------------------------
#[test]
fn test_decode_truncated_ok_data_gives_error() {
    let v: Result<u64, String> = Ok(u64::MAX);
    let enc = encode_to_vec(&v).expect("encode");

    // Drop the last byte so the u64 payload is incomplete
    let truncated = &enc[..enc.len() - 1];
    let result: oxicode::Result<(Result<u64, String>, usize)> = decode_from_slice(truncated);
    assert!(
        result.is_err(),
        "decoding truncated Ok payload must return an error"
    );
}

// ---------------------------------------------------------------------------
// 21. Decode truncated Err data gives error
// ---------------------------------------------------------------------------
#[test]
fn test_decode_truncated_err_data_gives_error() {
    let v: Result<u32, String> = Err("this is a long enough error string".to_string());
    let enc = encode_to_vec(&v).expect("encode");

    // Keep only the tag byte; discard the payload entirely
    let truncated = &enc[..1];
    let result: oxicode::Result<(Result<u32, String>, usize)> = decode_from_slice(truncated);
    assert!(
        result.is_err(),
        "decoding truncated Err payload must return an error"
    );
}

// ---------------------------------------------------------------------------
// 22. Result<HashMap<String, u32>, Vec<u8>>
// ---------------------------------------------------------------------------
#[test]
fn test_result_hashmap_ok_roundtrip() {
    let mut map = HashMap::new();
    map.insert("alpha".to_string(), 1u32);
    map.insert("beta".to_string(), 2u32);
    map.insert("gamma".to_string(), 300u32);

    let v: Result<HashMap<String, u32>, Vec<u8>> = Ok(map.clone());
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, consumed): (Result<HashMap<String, u32>, Vec<u8>>, _) =
        decode_from_slice(&enc).expect("decode");

    assert_eq!(consumed, enc.len());
    match dec {
        Ok(dec_map) => assert_eq!(map, dec_map),
        Err(_) => panic!("expected Ok variant"),
    }
}

#[test]
fn test_result_hashmap_err_roundtrip() {
    let payload = vec![0xCA, 0xFE, 0xBA, 0xBE];
    let v: Result<HashMap<String, u32>, Vec<u8>> = Err(payload.clone());
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, consumed): (Result<HashMap<String, u32>, Vec<u8>>, _) =
        decode_from_slice(&enc).expect("decode");

    assert_eq!(consumed, enc.len());
    match dec {
        Err(dec_payload) => assert_eq!(payload, dec_payload),
        Ok(_) => panic!("expected Err variant"),
    }
}
