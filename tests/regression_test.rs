//! Regression tests for specific bugs that have been found and fixed.

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

// Regression: encoding i128::MIN should not overflow
#[test]
fn regression_i128_min_no_overflow() {
    let v = i128::MIN;
    let enc = encode_to_vec(&v).expect("encode i128::MIN");
    let (dec, _): (i128, _) = decode_from_slice(&enc).expect("decode i128::MIN");
    assert_eq!(v, dec);
}

// Regression: empty slice should not cause issues
#[test]
fn regression_empty_slice_decoding() {
    let result: Result<(u8, _), _> = decode_from_slice(&[]);
    assert!(result.is_err());
}

// Regression: long string (> 250 chars, affects varint length encoding)
#[test]
fn regression_string_length_251_chars() {
    let s: String = "x".repeat(251); // > 250, needs 3-byte varint length
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
}

// Regression: large Vec (> 65535 elements, affects varint length)
#[test]
fn regression_vec_length_65536_elements() {
    let v: Vec<u8> = (0..=255u8).cycle().take(65536).collect();
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// Regression: nested struct with all-zero values
#[test]
fn regression_all_zero_values() {
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct AllZeros {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        e: i8,
        f: i16,
        g: i32,
        h: i64,
        i: f32,
        j: f64,
        k: bool,
    }

    let v = AllZeros {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: 0,
        g: 0,
        h: 0,
        i: 0.0,
        j: 0.0,
        k: false,
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (AllZeros, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// Regression: decode only consumes exactly the right number of bytes
#[test]
fn regression_exact_byte_consumption() {
    // Encode several independent values then concatenate
    let v1 = 42u32;
    let v2 = "hello".to_string();
    let v3 = vec![1u8, 2, 3];

    let mut combined = Vec::new();
    combined.extend(encode_to_vec(&v1).expect("encode v1"));
    combined.extend(encode_to_vec(&v2).expect("encode v2"));
    combined.extend(encode_to_vec(&v3).expect("encode v3"));

    let (dec1, n1) = decode_from_slice::<u32>(&combined).expect("decode v1");
    let (dec2, n2) = decode_from_slice::<String>(&combined[n1..]).expect("decode v2");
    let (dec3, n3) = decode_from_slice::<Vec<u8>>(&combined[n1 + n2..]).expect("decode v3");

    assert_eq!(v1, dec1);
    assert_eq!(v2, dec2);
    assert_eq!(v3, dec3);
    assert_eq!(n1 + n2 + n3, combined.len());
}
