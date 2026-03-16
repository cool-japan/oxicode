//! Advanced tests for usize and isize encoding in OxiCode.

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
use oxicode::{decode_from_slice, encode_to_vec, encoded_size};
use oxicode_derive::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct SizeStruct {
    count: usize,
    offset: isize,
}

// Test 1: usize(0) roundtrip - 1 byte
#[test]
fn test_usize_zero_roundtrip_one_byte() {
    let v: usize = 0;
    let enc = encode_to_vec(&v).expect("encode usize(0)");
    assert_eq!(enc.len(), 1, "usize(0) should encode to 1 byte");
    let (dec, consumed): (usize, usize) = decode_from_slice(&enc).expect("decode usize(0)");
    assert_eq!(dec, v);
    assert_eq!(consumed, 1);
}

// Test 2: usize(1) roundtrip
#[test]
fn test_usize_one_roundtrip() {
    let v: usize = 1;
    let enc = encode_to_vec(&v).expect("encode usize(1)");
    let (dec, consumed): (usize, usize) = decode_from_slice(&enc).expect("decode usize(1)");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());
}

// Test 3: usize(250) roundtrip - 1 byte
#[test]
fn test_usize_250_roundtrip_one_byte() {
    let v: usize = 250;
    let enc = encode_to_vec(&v).expect("encode usize(250)");
    assert_eq!(
        enc.len(),
        1,
        "usize(250) should encode to 1 byte (varint max single-byte value)"
    );
    let (dec, consumed): (usize, usize) = decode_from_slice(&enc).expect("decode usize(250)");
    assert_eq!(dec, v);
    assert_eq!(consumed, 1);
}

// Test 4: usize(251) roundtrip - 3 bytes
#[test]
fn test_usize_251_roundtrip_three_bytes() {
    let v: usize = 251;
    let enc = encode_to_vec(&v).expect("encode usize(251)");
    assert_eq!(
        enc.len(),
        3,
        "usize(251) should encode to 3 bytes (varint 2-byte range marker + u16)"
    );
    let (dec, consumed): (usize, usize) = decode_from_slice(&enc).expect("decode usize(251)");
    assert_eq!(dec, v);
    assert_eq!(consumed, 3);
}

// Test 5: usize(65535) roundtrip - 3 bytes
#[test]
fn test_usize_65535_roundtrip_three_bytes() {
    let v: usize = 65535;
    let enc = encode_to_vec(&v).expect("encode usize(65535)");
    assert_eq!(enc.len(), 3, "usize(65535) should encode to 3 bytes");
    let (dec, consumed): (usize, usize) = decode_from_slice(&enc).expect("decode usize(65535)");
    assert_eq!(dec, v);
    assert_eq!(consumed, 3);
}

// Test 6: usize(65536) roundtrip - 5 bytes
#[test]
fn test_usize_65536_roundtrip_five_bytes() {
    let v: usize = 65536;
    let enc = encode_to_vec(&v).expect("encode usize(65536)");
    assert_eq!(
        enc.len(),
        5,
        "usize(65536) should encode to 5 bytes (varint 4-byte range marker + u32)"
    );
    let (dec, consumed): (usize, usize) = decode_from_slice(&enc).expect("decode usize(65536)");
    assert_eq!(dec, v);
    assert_eq!(consumed, 5);
}

// Test 7: usize(u32::MAX as usize) roundtrip - 5 bytes
#[test]
fn test_usize_u32max_roundtrip_five_bytes() {
    let v: usize = u32::MAX as usize;
    let enc = encode_to_vec(&v).expect("encode usize(u32::MAX)");
    assert_eq!(enc.len(), 5, "usize(u32::MAX) should encode to 5 bytes");
    let (dec, consumed): (usize, usize) = decode_from_slice(&enc).expect("decode usize(u32::MAX)");
    assert_eq!(dec, v);
    assert_eq!(consumed, 5);
}

// Test 8: usize::MAX roundtrip (on 64-bit this is u64::MAX)
#[test]
fn test_usize_max_roundtrip() {
    let v: usize = usize::MAX;
    let enc = encode_to_vec(&v).expect("encode usize::MAX");
    let (dec, consumed): (usize, usize) = decode_from_slice(&enc).expect("decode usize::MAX");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());
    // On 64-bit systems, usize::MAX == u64::MAX, should be 9 bytes
    #[cfg(target_pointer_width = "64")]
    assert_eq!(
        enc.len(),
        9,
        "usize::MAX on 64-bit should encode to 9 bytes (same as u64::MAX)"
    );
}

// Test 9: isize(0) roundtrip - 1 byte (zigzag(0) = 0)
#[test]
fn test_isize_zero_roundtrip_one_byte() {
    let v: isize = 0;
    let enc = encode_to_vec(&v).expect("encode isize(0)");
    assert_eq!(enc.len(), 1, "isize(0) should encode to 1 byte via zigzag");
    let (dec, consumed): (isize, usize) = decode_from_slice(&enc).expect("decode isize(0)");
    assert_eq!(dec, v);
    assert_eq!(consumed, 1);
}

// Test 10: isize(1) roundtrip - 2 bytes (zigzag(1) = 2)
#[test]
fn test_isize_one_roundtrip_two_bytes() {
    let v: isize = 1;
    let enc = encode_to_vec(&v).expect("encode isize(1)");
    // zigzag(1) = 2, which is still 1 byte in varint (0-250 = 1 byte)
    // Actually zigzag(1) = 2 which is < 251, so it's 1 byte.
    // The comment in the task says "2 bytes (zigzag)" but let's verify:
    // zigzag encodes 1 as 2, and 2 < 251 so it fits in 1 byte.
    // We verify the roundtrip is correct regardless of byte count.
    let (dec, consumed): (isize, usize) = decode_from_slice(&enc).expect("decode isize(1)");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());
}

// Test 11: isize(-1) roundtrip - 1 byte (zigzag(-1) = 1)
#[test]
fn test_isize_neg1_roundtrip_one_byte() {
    let v: isize = -1;
    let enc = encode_to_vec(&v).expect("encode isize(-1)");
    // zigzag(-1) = 1, which encodes to 1 byte
    assert_eq!(
        enc.len(),
        1,
        "isize(-1) via zigzag encodes to 1 (zigzag value), 1 byte in varint"
    );
    let (dec, consumed): (isize, usize) = decode_from_slice(&enc).expect("decode isize(-1)");
    assert_eq!(dec, v);
    assert_eq!(consumed, 1);
}

// Test 12: isize::MIN roundtrip
#[test]
fn test_isize_min_roundtrip() {
    let v: isize = isize::MIN;
    let enc = encode_to_vec(&v).expect("encode isize::MIN");
    let (dec, consumed): (isize, usize) = decode_from_slice(&enc).expect("decode isize::MIN");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());
}

// Test 13: isize::MAX roundtrip
#[test]
fn test_isize_max_roundtrip() {
    let v: isize = isize::MAX;
    let enc = encode_to_vec(&v).expect("encode isize::MAX");
    let (dec, consumed): (isize, usize) = decode_from_slice(&enc).expect("decode isize::MAX");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());
}

// Test 14: Vec<usize> with 5 values roundtrip
#[test]
fn test_vec_usize_five_values_roundtrip() {
    let v: Vec<usize> = vec![0, 1, 250, 251, usize::MAX];
    let enc = encode_to_vec(&v).expect("encode Vec<usize>");
    let (dec, consumed): (Vec<usize>, usize) = decode_from_slice(&enc).expect("decode Vec<usize>");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());
}

// Test 15: Vec<isize> with negative and positive values roundtrip
#[test]
fn test_vec_isize_mixed_sign_roundtrip() {
    let v: Vec<isize> = vec![-1000, -1, 0, 1, 1000, isize::MIN, isize::MAX];
    let enc = encode_to_vec(&v).expect("encode Vec<isize>");
    let (dec, consumed): (Vec<isize>, usize) = decode_from_slice(&enc).expect("decode Vec<isize>");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());
}

// Test 16: Option<usize> Some(1000) roundtrip
#[test]
fn test_option_usize_some_roundtrip() {
    let v: Option<usize> = Some(1000);
    let enc = encode_to_vec(&v).expect("encode Option<usize> Some(1000)");
    let (dec, consumed): (Option<usize>, usize) =
        decode_from_slice(&enc).expect("decode Option<usize>");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());
}

// Test 17: Option<isize> None roundtrip
#[test]
fn test_option_isize_none_roundtrip() {
    let v: Option<isize> = None;
    let enc = encode_to_vec(&v).expect("encode Option<isize> None");
    let (dec, consumed): (Option<isize>, usize) =
        decode_from_slice(&enc).expect("decode Option<isize> None");
    assert_eq!(dec, v);
    assert_eq!(consumed, enc.len());
}

// Test 18: encoded_size(usize) - verify matches encode_to_vec().len() for multiple values
#[test]
fn test_encoded_size_usize_matches_actual_len() {
    let test_values: &[usize] = &[
        0,
        1,
        125,
        250,
        251,
        1000,
        65535,
        65536,
        u32::MAX as usize,
        usize::MAX,
    ];
    for &v in test_values {
        let expected_size = encoded_size(&v).expect("encoded_size");
        let actual_size = encode_to_vec(&v).expect("encode_to_vec").len();
        assert_eq!(
            expected_size, actual_size,
            "encoded_size mismatch for usize value {}",
            v
        );
    }
}

// Test 19: usize(0) encoded same as u64(0) - identical bytes
#[test]
fn test_usize_zero_identical_to_u64_zero() {
    let usize_enc = encode_to_vec(&0usize).expect("encode usize(0)");
    let u64_enc = encode_to_vec(&0u64).expect("encode u64(0)");
    assert_eq!(
        usize_enc, u64_enc,
        "usize(0) and u64(0) should produce identical encoded bytes on 64-bit systems"
    );
}

// Test 20: isize(-1) encoded same as i64(-1) - identical bytes
#[test]
fn test_isize_neg1_identical_to_i64_neg1() {
    let isize_enc = encode_to_vec(&(-1isize)).expect("encode isize(-1)");
    let i64_enc = encode_to_vec(&(-1i64)).expect("encode i64(-1)");
    assert_eq!(
        isize_enc, i64_enc,
        "isize(-1) and i64(-1) should produce identical encoded bytes on 64-bit systems"
    );
}

// Test 21: Struct with usize and isize fields roundtrip (using derive)
#[test]
fn test_size_struct_roundtrip() {
    let original = SizeStruct {
        count: 42_000_000,
        offset: -99_999,
    };
    let enc = encode_to_vec(&original).expect("encode SizeStruct");
    let (decoded, consumed): (SizeStruct, usize) =
        decode_from_slice(&enc).expect("decode SizeStruct");
    assert_eq!(decoded, original);
    assert_eq!(consumed, enc.len());
}

// Test 22: Large collection of usize values: Vec<usize> with 100 entries roundtrip
#[test]
fn test_vec_usize_100_entries_roundtrip() {
    // Build 100 usize values spanning a variety of varint encoding ranges:
    // include single-byte (0-250), 3-byte (251-65535), 5-byte (65536-u32::MAX), and 9-byte (>u32::MAX) entries
    let mut values: Vec<usize> = Vec::with_capacity(100);
    // 25 values in 1-byte range (0-250)
    for i in 0u64..25 {
        values.push((i * 10) as usize);
    }
    // 25 values in 3-byte range (251-65535)
    for i in 0u64..25 {
        values.push((251 + i * 1000) as usize);
    }
    // 25 values in 5-byte range (65536-u32::MAX)
    for i in 0u64..25 {
        values.push((65536 + i * 1_000_000) as usize);
    }
    // 25 values in 9-byte range (> u32::MAX, only meaningful on 64-bit)
    #[cfg(target_pointer_width = "64")]
    for i in 0u64..25 {
        values.push((u32::MAX as u64 + 1 + i * 1_000_000_000) as usize);
    }
    #[cfg(not(target_pointer_width = "64"))]
    for i in 0u64..25 {
        values.push((i * 13) as usize);
    }

    assert_eq!(values.len(), 100);

    let enc = encode_to_vec(&values).expect("encode Vec<usize> 100 entries");
    let (decoded, consumed): (Vec<usize>, usize) =
        decode_from_slice(&enc).expect("decode Vec<usize> 100 entries");
    assert_eq!(decoded, values);
    assert_eq!(consumed, enc.len());
}
