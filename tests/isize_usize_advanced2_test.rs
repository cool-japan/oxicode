//! Advanced tests for isize and usize serialization in OxiCode (set 2).
//! 22 top-level #[test] functions — no #[cfg(test)] wrapper.

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
// Test 1: usize value 0 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_usize_value_0_roundtrip() {
    let v: usize = 0;
    let encoded = encode_to_vec(&v).expect("encode usize 0");
    let (decoded, _consumed): (usize, usize) = decode_from_slice(&encoded).expect("decode usize 0");
    assert_eq!(decoded, v, "usize 0 roundtrip must be identity");
}

// ---------------------------------------------------------------------------
// Test 2: usize value 1 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_usize_value_1_roundtrip() {
    let v: usize = 1;
    let encoded = encode_to_vec(&v).expect("encode usize 1");
    let (decoded, _consumed): (usize, usize) = decode_from_slice(&encoded).expect("decode usize 1");
    assert_eq!(decoded, v, "usize 1 roundtrip must be identity");
}

// ---------------------------------------------------------------------------
// Test 3: usize value 127 roundtrip (1-byte varint max)
// ---------------------------------------------------------------------------

#[test]
fn test_usize_value_127_roundtrip() {
    let v: usize = 127;
    let encoded = encode_to_vec(&v).expect("encode usize 127");
    let (decoded, _consumed): (usize, usize) =
        decode_from_slice(&encoded).expect("decode usize 127");
    assert_eq!(decoded, v, "usize 127 roundtrip must be identity");
}

// ---------------------------------------------------------------------------
// Test 4: usize value 128 roundtrip (2-byte varint territory)
// ---------------------------------------------------------------------------

#[test]
fn test_usize_value_128_roundtrip() {
    let v: usize = 128;
    let encoded = encode_to_vec(&v).expect("encode usize 128");
    let (decoded, _consumed): (usize, usize) =
        decode_from_slice(&encoded).expect("decode usize 128");
    assert_eq!(decoded, v, "usize 128 roundtrip must be identity");
}

// ---------------------------------------------------------------------------
// Test 5: usize::MAX roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_usize_max_roundtrip() {
    let v: usize = usize::MAX;
    let encoded = encode_to_vec(&v).expect("encode usize::MAX");
    let (decoded, _consumed): (usize, usize) =
        decode_from_slice(&encoded).expect("decode usize::MAX");
    assert_eq!(decoded, v, "usize::MAX roundtrip must be identity");
}

// ---------------------------------------------------------------------------
// Test 6: isize value 0 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_isize_value_0_roundtrip() {
    let v: isize = 0;
    let encoded = encode_to_vec(&v).expect("encode isize 0");
    let (decoded, _consumed): (isize, usize) = decode_from_slice(&encoded).expect("decode isize 0");
    assert_eq!(decoded, v, "isize 0 roundtrip must be identity");
}

// ---------------------------------------------------------------------------
// Test 7: isize value -1 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_isize_value_neg1_roundtrip() {
    let v: isize = -1;
    let encoded = encode_to_vec(&v).expect("encode isize -1");
    let (decoded, _consumed): (isize, usize) =
        decode_from_slice(&encoded).expect("decode isize -1");
    assert_eq!(decoded, v, "isize -1 roundtrip must be identity");
}

// ---------------------------------------------------------------------------
// Test 8: isize value i8::MAX as isize roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_isize_value_i8_max_roundtrip() {
    let v: isize = i8::MAX as isize;
    let encoded = encode_to_vec(&v).expect("encode isize i8::MAX");
    let (decoded, _consumed): (isize, usize) =
        decode_from_slice(&encoded).expect("decode isize i8::MAX");
    assert_eq!(decoded, v, "isize i8::MAX roundtrip must be identity");
}

// ---------------------------------------------------------------------------
// Test 9: isize value i8::MIN as isize roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_isize_value_i8_min_roundtrip() {
    let v: isize = i8::MIN as isize;
    let encoded = encode_to_vec(&v).expect("encode isize i8::MIN");
    let (decoded, _consumed): (isize, usize) =
        decode_from_slice(&encoded).expect("decode isize i8::MIN");
    assert_eq!(decoded, v, "isize i8::MIN roundtrip must be identity");
}

// ---------------------------------------------------------------------------
// Test 10: isize::MIN roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_isize_min_roundtrip() {
    let v: isize = isize::MIN;
    let encoded = encode_to_vec(&v).expect("encode isize::MIN");
    let (decoded, _consumed): (isize, usize) =
        decode_from_slice(&encoded).expect("decode isize::MIN");
    assert_eq!(decoded, v, "isize::MIN roundtrip must be identity");
}

// ---------------------------------------------------------------------------
// Test 11: isize::MAX roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_isize_max_roundtrip() {
    let v: isize = isize::MAX;
    let encoded = encode_to_vec(&v).expect("encode isize::MAX");
    let (decoded, _consumed): (isize, usize) =
        decode_from_slice(&encoded).expect("decode isize::MAX");
    assert_eq!(decoded, v, "isize::MAX roundtrip must be identity");
}

// ---------------------------------------------------------------------------
// Test 12: usize consumed == encoded.len()
// ---------------------------------------------------------------------------

#[test]
fn test_usize_consumed_equals_encoded_len() {
    let values: &[usize] = &[0, 1, 127, 128, 250, 251, 1000, 65535, 65536, usize::MAX];
    for &v in values {
        let encoded = encode_to_vec(&v).expect("encode usize");
        let (_decoded, consumed): (usize, usize) =
            decode_from_slice(&encoded).expect("decode usize");
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed must equal encoded.len() for usize value {}",
            v
        );
    }
}

// ---------------------------------------------------------------------------
// Test 13: isize consumed == encoded.len()
// ---------------------------------------------------------------------------

#[test]
fn test_isize_consumed_equals_encoded_len() {
    let values: &[isize] = &[0, 1, -1, 127, -128, 1000, -1000, isize::MIN, isize::MAX];
    for &v in values {
        let encoded = encode_to_vec(&v).expect("encode isize");
        let (_decoded, consumed): (isize, usize) =
            decode_from_slice(&encoded).expect("decode isize");
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed must equal encoded.len() for isize value {}",
            v
        );
    }
}

// ---------------------------------------------------------------------------
// Test 14: Vec<usize> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_usize_roundtrip() {
    let v: Vec<usize> = vec![0, 1, 127, 128, 250, 251, 65535, 65536, usize::MAX];
    let encoded = encode_to_vec(&v).expect("encode Vec<usize>");
    let (decoded, consumed): (Vec<usize>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<usize>");
    assert_eq!(decoded, v, "Vec<usize> roundtrip must be identity");
    assert_eq!(consumed, encoded.len(), "consumed must equal encoded.len()");
}

// ---------------------------------------------------------------------------
// Test 15: Vec<isize> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_isize_roundtrip() {
    let v: Vec<isize> = vec![0, 1, -1, 100, -100, isize::MIN, isize::MAX];
    let encoded = encode_to_vec(&v).expect("encode Vec<isize>");
    let (decoded, consumed): (Vec<isize>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<isize>");
    assert_eq!(decoded, v, "Vec<isize> roundtrip must be identity");
    assert_eq!(consumed, encoded.len(), "consumed must equal encoded.len()");
}

// ---------------------------------------------------------------------------
// Test 16: Option<usize> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_usize_some_roundtrip() {
    let v: Option<usize> = Some(42);
    let encoded = encode_to_vec(&v).expect("encode Option<usize> Some(42)");
    let (decoded, consumed): (Option<usize>, usize) =
        decode_from_slice(&encoded).expect("decode Option<usize>");
    assert_eq!(decoded, v, "Option<usize> Some roundtrip must be identity");
    assert_eq!(consumed, encoded.len(), "consumed must equal encoded.len()");
}

// ---------------------------------------------------------------------------
// Test 17: Option<isize> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_isize_none_roundtrip() {
    let v: Option<isize> = None;
    let encoded = encode_to_vec(&v).expect("encode Option<isize> None");
    let (decoded, consumed): (Option<isize>, usize) =
        decode_from_slice(&encoded).expect("decode Option<isize> None");
    assert_eq!(decoded, v, "Option<isize> None roundtrip must be identity");
    assert_eq!(consumed, encoded.len(), "consumed must equal encoded.len()");
}

// ---------------------------------------------------------------------------
// Test 18: Fixed-int config with usize value 42 (roundtrip only — size is platform-dependent)
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_config_usize_value_42_roundtrip() {
    let v: usize = 42;
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&v, cfg).expect("encode usize 42 fixed-int");
    let (decoded, consumed): (usize, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode usize 42 fixed-int");
    assert_eq!(
        decoded, v,
        "fixed-int config usize 42 roundtrip must be identity"
    );
    assert_eq!(consumed, encoded.len(), "consumed must equal encoded.len()");
}

// ---------------------------------------------------------------------------
// Test 19: Big-endian config with usize value 0x0102 (roundtrip only — size is platform-dependent)
// ---------------------------------------------------------------------------

#[test]
fn test_big_endian_config_usize_value_0x0102_roundtrip() {
    let v: usize = 0x0102;
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&v, cfg).expect("encode usize 0x0102 big-endian");
    let (decoded, consumed): (usize, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode usize 0x0102 big-endian");
    assert_eq!(
        decoded, v,
        "big-endian fixed-int config usize 0x0102 roundtrip must be identity"
    );
    assert_eq!(consumed, encoded.len(), "consumed must equal encoded.len()");
}

// ---------------------------------------------------------------------------
// Test 20: (usize, isize) tuple roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_usize_isize_tuple_roundtrip() {
    let v: (usize, isize) = (usize::MAX / 2, isize::MIN / 2);
    let encoded = encode_to_vec(&v).expect("encode (usize, isize) tuple");
    let (decoded, consumed): ((usize, isize), usize) =
        decode_from_slice(&encoded).expect("decode (usize, isize) tuple");
    assert_eq!(
        decoded, v,
        "(usize, isize) tuple roundtrip must be identity"
    );
    assert_eq!(consumed, encoded.len(), "consumed must equal encoded.len()");
}

// ---------------------------------------------------------------------------
// Test 21: usize small values (0..=10) all encode and decode correctly
// ---------------------------------------------------------------------------

#[test]
fn test_usize_small_values_0_to_10_roundtrip() {
    for v in 0usize..=10 {
        let encoded = encode_to_vec(&v).expect("encode usize small value");
        let (decoded, consumed): (usize, usize) =
            decode_from_slice(&encoded).expect("decode usize small value");
        assert_eq!(
            decoded, v,
            "usize small value {} roundtrip must be identity",
            v
        );
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed must equal encoded.len() for usize {}",
            v
        );
    }
}

// ---------------------------------------------------------------------------
// Test 22: isize negative range (-10..=0) all encode and decode correctly
// ---------------------------------------------------------------------------

#[test]
fn test_isize_negative_range_neg10_to_0_roundtrip() {
    for v in -10isize..=0 {
        let encoded = encode_to_vec(&v).expect("encode isize negative value");
        let (decoded, consumed): (isize, usize) =
            decode_from_slice(&encoded).expect("decode isize negative value");
        assert_eq!(
            decoded, v,
            "isize negative value {} roundtrip must be identity",
            v
        );
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed must equal encoded.len() for isize {}",
            v
        );
    }
}
