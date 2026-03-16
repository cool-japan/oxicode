//! Advanced tests for byte slice and array serialization in OxiCode.

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

#[test]
fn test_vec_u8_empty_roundtrip() {
    let original: Vec<u8> = Vec::new();
    let enc = encode_to_vec(&original).expect("encode empty Vec<u8>");
    let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode empty Vec<u8>");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_u8_single_roundtrip() {
    let original: Vec<u8> = vec![42u8];
    let enc = encode_to_vec(&original).expect("encode single Vec<u8>");
    let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode single Vec<u8>");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_u8_full_byte_range() {
    let original: Vec<u8> = (0u8..=255u8).collect();
    assert_eq!(original.len(), 256);
    let enc = encode_to_vec(&original).expect("encode full byte range Vec<u8>");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&enc).expect("decode full byte range Vec<u8>");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_u8_1kb_roundtrip() {
    let original: Vec<u8> = (0u8..=255u8).cycle().take(1024).collect();
    assert_eq!(original.len(), 1024);
    let enc = encode_to_vec(&original).expect("encode 1KB Vec<u8>");
    let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode 1KB Vec<u8>");
    assert_eq!(original, decoded);
}

#[test]
fn test_array_u8_4_roundtrip() {
    let original: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let enc = encode_to_vec(&original).expect("encode [u8; 4]");
    let (decoded, _): ([u8; 4], usize) = decode_from_slice(&enc).expect("decode [u8; 4]");
    assert_eq!(original, decoded);
}

#[test]
fn test_array_u8_16_roundtrip() {
    let original: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let enc = encode_to_vec(&original).expect("encode [u8; 16]");
    let (decoded, _): ([u8; 16], usize) = decode_from_slice(&enc).expect("decode [u8; 16]");
    assert_eq!(original, decoded);
}

#[test]
fn test_array_u8_32_roundtrip() {
    let original: [u8; 32] = {
        let mut arr = [0u8; 32];
        for (i, b) in arr.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(7);
        }
        arr
    };
    let enc = encode_to_vec(&original).expect("encode [u8; 32]");
    let (decoded, _): ([u8; 32], usize) = decode_from_slice(&enc).expect("decode [u8; 32]");
    assert_eq!(original, decoded);
}

#[test]
fn test_array_u8_64_roundtrip() {
    let original: [u8; 64] = {
        let mut arr = [0u8; 64];
        for (i, b) in arr.iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(10);
        }
        arr
    };
    let enc = encode_to_vec(&original).expect("encode [u8; 64]");
    let (decoded, _): ([u8; 64], usize) = decode_from_slice(&enc).expect("decode [u8; 64]");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_u8_vs_array_u8_4_size() {
    let bytes = [0x01u8, 0x02u8, 0x03u8, 0x04u8];
    let vec_original: Vec<u8> = bytes.to_vec();
    let arr_original: [u8; 4] = bytes;

    let vec_enc = encode_to_vec(&vec_original).expect("encode Vec<u8> of 4 bytes");
    let arr_enc = encode_to_vec(&arr_original).expect("encode [u8; 4]");

    // Vec<u8> has a length prefix, [u8; 4] does not — array encoding is smaller or equal
    assert!(
        arr_enc.len() <= vec_enc.len(),
        "array encoding ({}) should be <= vec encoding ({})",
        arr_enc.len(),
        vec_enc.len()
    );
}

#[test]
fn test_vec_u8_consumed_equals_len() {
    let original: Vec<u8> = vec![10u8, 20u8, 30u8];
    let enc = encode_to_vec(&original).expect("encode Vec<u8>");
    let (_, consumed): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8>");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes should equal encoded length"
    );
}

#[test]
fn test_array_u8_16_consumed_equals_len() {
    let original: [u8; 16] = [0u8; 16];
    let enc = encode_to_vec(&original).expect("encode [u8; 16]");
    let (_, consumed): ([u8; 16], usize) = decode_from_slice(&enc).expect("decode [u8; 16]");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes should equal encoded length for array"
    );
}

#[test]
fn test_vec_u8_fixed_int_config() {
    let original: Vec<u8> = vec![1u8, 2u8, 3u8, 4u8, 5u8];
    let cfg = config::legacy();
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode Vec<u8> with legacy config");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Vec<u8> with legacy config");
    assert_eq!(original, decoded);
}

#[test]
fn test_array_u8_4_big_endian_config() {
    let original: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let cfg = config::standard().with_big_endian();
    let enc =
        encode_to_vec_with_config(&original, cfg).expect("encode [u8;4] with big_endian config");
    let (decoded, _): ([u8; 4], usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u8;4] with big_endian config");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_i8_roundtrip() {
    let original: Vec<i8> = vec![-128i8, -64i8, -1i8, 0i8, 1i8, 64i8, 127i8];
    let enc = encode_to_vec(&original).expect("encode Vec<i8>");
    let (decoded, _): (Vec<i8>, usize) = decode_from_slice(&enc).expect("decode Vec<i8>");
    assert_eq!(original, decoded);
}

#[test]
fn test_array_i8_8_roundtrip() {
    let original: [i8; 8] = [-100i8, -50i8, -10i8, -1i8, 0i8, 1i8, 50i8, 100i8];
    let enc = encode_to_vec(&original).expect("encode [i8; 8]");
    let (decoded, _): ([i8; 8], usize) = decode_from_slice(&enc).expect("decode [i8; 8]");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_u8_all_zeros_roundtrip() {
    let original: Vec<u8> = vec![0u8; 100];
    assert_eq!(original.len(), 100);
    let enc = encode_to_vec(&original).expect("encode Vec<u8> all zeros");
    let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8> all zeros");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_u8_all_max_roundtrip() {
    let original: Vec<u8> = vec![0xFFu8; 50];
    assert_eq!(original.len(), 50);
    let enc = encode_to_vec(&original).expect("encode Vec<u8> all 0xFF");
    let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8> all 0xFF");
    assert_eq!(original, decoded);
}

#[test]
fn test_nested_vec_u8_roundtrip() {
    let original: Vec<Vec<u8>> = vec![
        vec![1u8, 2u8, 3u8],
        vec![10u8, 20u8],
        vec![100u8, 110u8, 120u8, 130u8],
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Vec<u8>>");
    let (decoded, _): (Vec<Vec<u8>>, usize) = decode_from_slice(&enc).expect("decode Vec<Vec<u8>>");
    assert_eq!(original, decoded);
}

#[test]
fn test_option_vec_u8_some_roundtrip() {
    let original: Option<Vec<u8>> = Some(vec![5u8, 10u8, 15u8, 20u8]);
    let enc = encode_to_vec(&original).expect("encode Option<Vec<u8>> Some");
    let (decoded, _): (Option<Vec<u8>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Vec<u8>> Some");
    assert_eq!(original, decoded);
}

#[test]
fn test_option_vec_u8_none_roundtrip() {
    let original: Option<Vec<u8>> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Vec<u8>> None");
    let (decoded, _): (Option<Vec<u8>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Vec<u8>> None");
    assert_eq!(original, decoded);
}

#[test]
fn test_tuple_vec_u8_roundtrip() {
    let original: (Vec<u8>, Vec<u8>) = (vec![1u8, 2u8, 3u8], vec![4u8, 5u8, 6u8, 7u8]);
    let enc = encode_to_vec(&original).expect("encode (Vec<u8>, Vec<u8>)");
    let (decoded, _): ((Vec<u8>, Vec<u8>), usize) =
        decode_from_slice(&enc).expect("decode (Vec<u8>, Vec<u8>)");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_u8_reencode_stability() {
    let original: Vec<u8> = vec![9u8, 18u8, 27u8, 36u8, 45u8];
    let enc1 = encode_to_vec(&original).expect("first encode Vec<u8>");
    let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&enc1).expect("decode Vec<u8>");
    let enc2 = encode_to_vec(&decoded).expect("re-encode decoded Vec<u8>");
    assert_eq!(
        enc1, enc2,
        "re-encoding a decoded value must produce identical bytes"
    );
}
