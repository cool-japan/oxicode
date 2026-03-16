//! Tests for #[oxicode(flatten)] field attribute

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

#[test]
fn test_flatten_basic() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Inner {
        x: u32,
        y: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Outer {
        #[oxicode(flatten)]
        inner: Inner,
        z: u32,
    }

    let value = Outer {
        inner: Inner { x: 1, y: 2 },
        z: 3,
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Outer, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(value, decoded);

    // Verify that flatten is binary-compatible with encoding Inner's fields directly
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Flat {
        x: u32,
        y: u32,
        z: u32,
    }
    let flat = Flat { x: 1, y: 2, z: 3 };
    let flat_encoded = encode_to_vec(&flat).expect("encode flat");
    assert_eq!(
        encoded, flat_encoded,
        "flatten should produce identical bytes to manually flattened struct"
    );
}

#[test]
fn test_flatten_with_skip() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Meta {
        id: u64,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Record {
        #[oxicode(flatten)]
        meta: Meta,
        #[oxicode(skip)]
        cached: String,
        value: f64,
    }

    let record = Record {
        meta: Meta { id: 42 },
        cached: "ignored".to_string(),
        value: 3.14,
    };
    let encoded = encode_to_vec(&record).expect("encode");
    let (decoded, _): (Record, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(record.meta, decoded.meta);
    assert_eq!(record.value, decoded.value);
    assert_eq!(decoded.cached, String::default()); // skip restores to Default
}
