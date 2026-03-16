#![cfg(feature = "serde")]
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
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestHeader {
    element_size: usize,
    shape: Vec<usize>,
    total_elements: usize,
    metadata: Option<String>,
}

#[test]
fn test_serde_owned_roundtrip() {
    let header = TestHeader {
        element_size: 8,
        shape: vec![10, 20, 30],
        total_elements: 6000,
        metadata: Some("test data".to_string()),
    };

    // Encode
    let cfg = oxicode::config::standard();
    let bytes = oxicode::serde::encode_to_vec(&header, cfg).unwrap();
    println!("Encoded {} bytes", bytes.len());
    println!("Bytes: {:?}", &bytes[..bytes.len().min(50)]);

    // Decode with owned
    let result = oxicode::serde::decode_owned_from_slice::<TestHeader, _>(&bytes, cfg);
    match result {
        Ok((decoded, len)) => {
            println!("Decoded {} bytes", len);
            assert_eq!(header, decoded);
            assert_eq!(len, bytes.len());
        }
        Err(e) => {
            println!("Decode error: {:?}", e);
            panic!("Failed to decode");
        }
    }
}

#[test]
fn test_serde_owned_simple_struct() {
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct Simple {
        a: usize,
        b: Vec<usize>,
    }

    let simple = Simple {
        a: 42,
        b: vec![1, 2, 3],
    };

    let cfg = oxicode::config::standard();
    let bytes = oxicode::serde::encode_to_vec(&simple, cfg).unwrap();
    let (decoded, _): (Simple, usize) =
        oxicode::serde::decode_owned_from_slice(&bytes, cfg).unwrap();

    assert_eq!(simple, decoded);
}

#[test]
fn test_serde_owned_vec_usize() {
    let data = vec![1usize, 2, 3, 4, 5];

    let cfg = oxicode::config::standard();
    let bytes = oxicode::serde::encode_to_vec(&data, cfg).unwrap();

    let (decoded, _): (Vec<usize>, usize) =
        oxicode::serde::decode_owned_from_slice(&bytes, cfg).unwrap();

    assert_eq!(data, decoded);
}
