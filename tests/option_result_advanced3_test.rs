//! Advanced Option/Result combination tests for OxiCode (set 3).

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

// ---------------------------------------------------------------------------
// 1. Option<Option<u32>> Some(Some(42))
// ---------------------------------------------------------------------------

#[test]
fn test_option_option_u32_some_some() {
    let v: Option<Option<u32>> = Some(Some(42));
    let enc = encode_to_vec(&v).expect("encode Option<Option<u32>> Some(Some(42))");
    let (dec, _): (Option<Option<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Option<u32>> Some(Some(42))");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 2. Option<Option<u32>> Some(None)
// ---------------------------------------------------------------------------

#[test]
fn test_option_option_u32_some_none() {
    let v: Option<Option<u32>> = Some(None);
    let enc = encode_to_vec(&v).expect("encode Option<Option<u32>> Some(None)");
    let (dec, _): (Option<Option<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Option<u32>> Some(None)");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 3. Option<Option<u32>> None
// ---------------------------------------------------------------------------

#[test]
fn test_option_option_u32_none() {
    let v: Option<Option<u32>> = None;
    let enc = encode_to_vec(&v).expect("encode Option<Option<u32>> None");
    let (dec, _): (Option<Option<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Option<u32>> None");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 4. Result<u32, String> Ok(42)
// ---------------------------------------------------------------------------

#[test]
fn test_result_ok_roundtrip() {
    let v: Result<u32, String> = Ok(42);
    let enc = encode_to_vec(&v).expect("encode Result Ok(42)");
    let (dec, _): (Result<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode Result Ok(42)");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 5. Result<u32, String> Err("fail")
// ---------------------------------------------------------------------------

#[test]
fn test_result_err_roundtrip() {
    let v: Result<u32, String> = Err("fail".to_string());
    let enc = encode_to_vec(&v).expect("encode Result Err(fail)");
    let (dec, _): (Result<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode Result Err(fail)");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 6. Result<u32, String> Ok — bytes consumed equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_result_ok_consumed_len() {
    let v: Result<u32, String> = Ok(100);
    let enc = encode_to_vec(&v).expect("encode Result Ok(100)");
    let (_, consumed): (Result<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode Result Ok(100)");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length"
    );
}

// ---------------------------------------------------------------------------
// 7. Result<u32, String> Err — bytes consumed equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_result_err_consumed_len() {
    let v: Result<u32, String> = Err("error-string".to_string());
    let enc = encode_to_vec(&v).expect("encode Result Err");
    let (_, consumed): (Result<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode Result Err");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length"
    );
}

// ---------------------------------------------------------------------------
// 8. Vec<Option<u32>> with mixed Some/None
// ---------------------------------------------------------------------------

#[test]
fn test_vec_option_u32_roundtrip() {
    let v: Vec<Option<u32>> = vec![Some(1), None, Some(3), None, Some(5)];
    let enc = encode_to_vec(&v).expect("encode Vec<Option<u32>>");
    let (dec, _): (Vec<Option<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Option<u32>>");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 9. Option<Vec<u32>> Some
// ---------------------------------------------------------------------------

#[test]
fn test_option_vec_u32_some_roundtrip() {
    let v: Option<Vec<u32>> = Some(vec![10, 20, 30]);
    let enc = encode_to_vec(&v).expect("encode Option<Vec<u32>> Some");
    let (dec, _): (Option<Vec<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Vec<u32>> Some");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 10. Option<Vec<u32>> None
// ---------------------------------------------------------------------------

#[test]
fn test_option_vec_u32_none_roundtrip() {
    let v: Option<Vec<u32>> = None;
    let enc = encode_to_vec(&v).expect("encode Option<Vec<u32>> None");
    let (dec, _): (Option<Vec<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Vec<u32>> None");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 11. Result<String, u32> Ok("hello")
// ---------------------------------------------------------------------------

#[test]
fn test_result_ok_string_roundtrip() {
    let v: Result<String, u32> = Ok("hello".to_string());
    let enc = encode_to_vec(&v).expect("encode Result<String, u32> Ok");
    let (dec, _): (Result<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode Result<String, u32> Ok");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 12. Result<String, u32> Err(99)
// ---------------------------------------------------------------------------

#[test]
fn test_result_err_u32_roundtrip() {
    let v: Result<String, u32> = Err(99);
    let enc = encode_to_vec(&v).expect("encode Result<String, u32> Err(99)");
    let (dec, _): (Result<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode Result<String, u32> Err(99)");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 13. Option<String> Some
// ---------------------------------------------------------------------------

#[test]
fn test_option_string_some_roundtrip() {
    let v: Option<String> = Some("oxicode advanced".to_string());
    let enc = encode_to_vec(&v).expect("encode Option<String> Some");
    let (dec, _): (Option<String>, usize) =
        decode_from_slice(&enc).expect("decode Option<String> Some");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 14. Option<String> None
// ---------------------------------------------------------------------------

#[test]
fn test_option_string_none_roundtrip() {
    let v: Option<String> = None;
    let enc = encode_to_vec(&v).expect("encode Option<String> None");
    let (dec, _): (Option<String>, usize) =
        decode_from_slice(&enc).expect("decode Option<String> None");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 15. Result<Result<u32, String>, u64> Ok(Ok(1))
// ---------------------------------------------------------------------------

#[test]
fn test_nested_result_ok_ok() {
    let v: Result<Result<u32, String>, u64> = Ok(Ok(1));
    let enc = encode_to_vec(&v).expect("encode nested Result Ok(Ok(1))");
    let (dec, _): (Result<Result<u32, String>, u64>, usize) =
        decode_from_slice(&enc).expect("decode nested Result Ok(Ok(1))");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 16. Result<Result<u32, String>, u64> Ok(Err("x"))
// ---------------------------------------------------------------------------

#[test]
fn test_nested_result_ok_err() {
    let v: Result<Result<u32, String>, u64> = Ok(Err("x".to_string()));
    let enc = encode_to_vec(&v).expect("encode nested Result Ok(Err)");
    let (dec, _): (Result<Result<u32, String>, u64>, usize) =
        decode_from_slice(&enc).expect("decode nested Result Ok(Err)");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 17. Result<Result<u32, String>, u64> Err(999)
// ---------------------------------------------------------------------------

#[test]
fn test_nested_result_err() {
    let v: Result<Result<u32, String>, u64> = Err(999);
    let enc = encode_to_vec(&v).expect("encode nested Result Err(999)");
    let (dec, _): (Result<Result<u32, String>, u64>, usize) =
        decode_from_slice(&enc).expect("decode nested Result Err(999)");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 18. Option<(u32, u64)> Some((1, 2))
// ---------------------------------------------------------------------------

#[test]
fn test_option_tuple_roundtrip() {
    let v: Option<(u32, u64)> = Some((1, 2));
    let enc = encode_to_vec(&v).expect("encode Option<(u32, u64)> Some");
    let (dec, _): (Option<(u32, u64)>, usize) =
        decode_from_slice(&enc).expect("decode Option<(u32, u64)> Some");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 19. Result<(u32, u64), String> Ok((1, 2))
// ---------------------------------------------------------------------------

#[test]
fn test_result_tuple_ok() {
    let v: Result<(u32, u64), String> = Ok((1, 2));
    let enc = encode_to_vec(&v).expect("encode Result<(u32, u64), String> Ok");
    let (dec, _): (Result<(u32, u64), String>, usize) =
        decode_from_slice(&enc).expect("decode Result<(u32, u64), String> Ok");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 20. Option<u64>::None encodes to exactly 1 byte (just variant tag)
// ---------------------------------------------------------------------------

#[test]
fn test_option_none_size() {
    let v: Option<u64> = None;
    let enc = encode_to_vec(&v).expect("encode Option<u64> None");
    assert_eq!(enc.len(), 1, "Option::None must encode to exactly 1 byte");
}

// ---------------------------------------------------------------------------
// 21. Option<u64> Some with fixed_int_encoding config
// ---------------------------------------------------------------------------

#[test]
fn test_option_some_u64_fixed_int_config() {
    let v: Option<u64> = Some(0x0102_0304_0506_0708_u64);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&v, cfg).expect("encode Option<u64> Some fixed_int");
    let (dec, consumed): (Option<u64>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Option<u64> Some fixed_int");
    assert_eq!(v, dec);
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length"
    );
    // variant tag (1 byte) + u64 fixed-int (8 bytes) = 9 bytes
    assert_eq!(
        enc.len(),
        9,
        "Option<u64> Some with fixed_int must encode to 9 bytes"
    );
}

// ---------------------------------------------------------------------------
// 22. Result<u32, u32> Ok(0x01020304) with big-endian config
// ---------------------------------------------------------------------------

#[test]
fn test_result_ok_big_endian_config() {
    let v: Result<u32, u32> = Ok(0x0102_0304);
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&v, cfg).expect("encode Result Ok big-endian");
    let (dec, consumed): (Result<u32, u32>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Result Ok big-endian");
    assert_eq!(v, dec);
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length"
    );
    // In fixed-int mode the variant discriminant is also encoded as a fixed u32 (4 bytes),
    // so Ok(u32) encodes as: 4-byte tag + 4-byte value = 8 bytes total.
    assert_eq!(
        enc.len(),
        8,
        "Result Ok<u32> with big-endian fixed must encode to 8 bytes"
    );
    // The last 4 bytes are the value in MSB-first (big-endian) order.
    assert_eq!(
        &enc[4..],
        &[0x01, 0x02, 0x03, 0x04],
        "big-endian u32 0x01020304 must serialise as MSB-first in the value bytes"
    );
}
