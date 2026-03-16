//! Tests for #[oxicode(crate = "oxicode")] container attribute.
//! This allows the derive macros to work when oxicode is re-exported
//! under a different name.

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
// The default behavior (no crate attr) should work fine
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// With explicit crate path matching the actual crate name
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(crate = "oxicode")]
struct ExplicitCratePath {
    value: u32,
    name: String,
}

#[test]
fn test_explicit_crate_path_roundtrip() {
    let v = ExplicitCratePath {
        value: 42,
        name: "test".to_string(),
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (ExplicitCratePath, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_explicit_crate_path_binary_compat() {
    // Same struct without crate attr should produce identical bytes
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct NoCratePath {
        value: u32,
        name: String,
    }

    let data1 = ExplicitCratePath {
        value: 100,
        name: "hello".to_string(),
    };
    let data2 = NoCratePath {
        value: 100,
        name: "hello".to_string(),
    };

    let enc1 = encode_to_vec(&data1).expect("encode with crate attr");
    let enc2 = encode_to_vec(&data2).expect("encode without crate attr");

    assert_eq!(enc1, enc2, "crate attribute should not affect wire format");
}

// With enum
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(crate = "oxicode")]
enum Status {
    Active,
    Inactive,
    Pending(String),
}

#[test]
fn test_crate_path_with_enum() {
    for status in [
        Status::Active,
        Status::Inactive,
        Status::Pending("waiting".to_string()),
    ] {
        let enc = encode_to_vec(&status).expect("encode");
        let (dec, _): (Status, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(status, dec);
    }
}

#[test]
fn test_crate_path_with_generics() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(crate = "oxicode")]
    struct GenericWithCrate<T> {
        inner: T,
    }

    let v = GenericWithCrate {
        inner: vec![1u32, 2, 3],
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (GenericWithCrate<Vec<u32>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}
