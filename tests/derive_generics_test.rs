//! Tests for derive macros on generic types with various bound patterns.

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

// Generic with multiple type params
#[derive(Debug, PartialEq, Encode, Decode)]
struct Triple<A, B, C> {
    first: A,
    second: B,
    third: C,
}

#[test]
fn test_triple_u32_string_bool() {
    let t = Triple {
        first: 42u32,
        second: "hello".to_string(),
        third: true,
    };
    let enc = encode_to_vec(&t).expect("encode");
    let (dec, _): (Triple<u32, String, bool>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(t, dec);
}

// Generic with nested generics
#[derive(Debug, PartialEq, Encode, Decode)]
struct Nested<T> {
    outer: Vec<Option<T>>,
    count: usize,
}

#[test]
fn test_nested_vec_option_u64() {
    let n = Nested {
        outer: vec![Some(1u64), None, Some(3), Some(4)],
        count: 4,
    };
    let enc = encode_to_vec(&n).expect("encode");
    let (dec, _): (Nested<u64>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(n, dec);
}

// Generic enum
#[derive(Debug, PartialEq, Encode, Decode)]
enum Either<L, R> {
    Left(L),
    Right(R),
    Both { left: L, right: R },
    Neither,
}

#[test]
fn test_either_enum_all_variants() {
    let cases: Vec<Either<u32, String>> = vec![
        Either::Left(42u32),
        Either::Right("hello".to_string()),
        Either::Both {
            left: 1,
            right: "world".to_string(),
        },
        Either::Neither,
    ];
    for case in cases {
        let enc = encode_to_vec(&case).expect("encode");
        let (dec, _): (Either<u32, String>, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(case, dec);
    }
}

// Recursive-like structures (using Box to break cycles)
#[derive(Debug, PartialEq, Encode, Decode)]
struct LinkedNode {
    value: u32,
    next: Option<Box<LinkedNode>>,
}

#[test]
fn test_linked_list_roundtrip() {
    let list = LinkedNode {
        value: 1,
        next: Some(Box::new(LinkedNode {
            value: 2,
            next: Some(Box::new(LinkedNode {
                value: 3,
                next: None,
            })),
        })),
    };
    let enc = encode_to_vec(&list).expect("encode");
    let (dec, _): (LinkedNode, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(list, dec);
}

// Struct with Arc fields
#[test]
fn test_struct_with_arc_fields() {
    use std::sync::Arc;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WithArc {
        data: Arc<Vec<u32>>,
        label: Arc<String>,
    }

    let v = WithArc {
        data: Arc::new(vec![1, 2, 3]),
        label: Arc::new("arc_test".to_string()),
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (WithArc, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}
