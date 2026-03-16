//! Tests for derive macros

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
use oxicode::{config, Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Named {
    name: String,
    age: u32,
    active: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Tuple(u32, String, bool);

#[derive(Debug, PartialEq, Encode, Decode)]
struct Unit;

#[derive(Debug, PartialEq, Encode, Decode)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Generic<T> {
    value: T,
    count: usize,
}

#[test]
fn test_struct_named_fields() {
    let point = Point { x: 1.5, y: 2.5 };

    let encoded = oxicode::encode_to_vec(&point).expect("Failed to encode");
    let (decoded, _): (Point, _) = oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(point, decoded);
}

#[test]
fn test_struct_with_string() {
    let named = Named {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };

    let encoded = oxicode::encode_to_vec(&named).expect("Failed to encode");
    let (decoded, _): (Named, _) = oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(named, decoded);
}

#[test]
fn test_tuple_struct() {
    let tuple = Tuple(42, String::from("hello"), false);

    let encoded = oxicode::encode_to_vec(&tuple).expect("Failed to encode");
    let (decoded, _): (Tuple, _) = oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(tuple, decoded);
}

#[test]
fn test_unit_struct() {
    let unit = Unit;

    let encoded = oxicode::encode_to_vec(&unit).expect("Failed to encode");
    let (decoded, _): (Unit, _) = oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(unit, decoded);
}

#[test]
fn test_enum_unit_variant() {
    let msg = Message::Quit;

    let encoded = oxicode::encode_to_vec(&msg).expect("Failed to encode");
    let (decoded, _): (Message, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(msg, decoded);
}

#[test]
fn test_enum_struct_variant() {
    let msg = Message::Move { x: 10, y: 20 };

    let encoded = oxicode::encode_to_vec(&msg).expect("Failed to encode");
    let (decoded, _): (Message, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(msg, decoded);
}

#[test]
fn test_enum_tuple_variant() {
    let msg = Message::Write(String::from("test"));

    let encoded = oxicode::encode_to_vec(&msg).expect("Failed to encode");
    let (decoded, _): (Message, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(msg, decoded);
}

#[test]
fn test_enum_multiple_fields() {
    let msg = Message::ChangeColor(255, 128, 64);

    let encoded = oxicode::encode_to_vec(&msg).expect("Failed to encode");
    let (decoded, _): (Message, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(msg, decoded);
}

#[test]
fn test_generic_struct() {
    let generic = Generic {
        value: 42u64,
        count: 10,
    };

    let encoded = oxicode::encode_to_vec(&generic).expect("Failed to encode");
    let (decoded, _): (Generic<u64>, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(generic, decoded);
}

#[test]
fn test_generic_with_vec() {
    let generic = Generic {
        value: vec![1, 2, 3, 4, 5],
        count: 5,
    };

    let encoded = oxicode::encode_to_vec(&generic).expect("Failed to encode");
    let (decoded, _): (Generic<Vec<i32>>, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(generic, decoded);
}

#[test]
fn test_nested_structs() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Inner {
        value: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Outer {
        inner: Inner,
        name: String,
    }

    let outer = Outer {
        inner: Inner { value: 123 },
        name: String::from("nested"),
    };

    let encoded = oxicode::encode_to_vec(&outer).expect("Failed to encode");
    let (decoded, _): (Outer, _) = oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(outer, decoded);
}

#[test]
fn test_config_legacy() {
    let point = Point {
        x: std::f32::consts::PI,
        y: std::f32::consts::E,
    };

    let config = config::legacy();
    let encoded = oxicode::encode_to_vec_with_config(&point, config).expect("Failed to encode");
    let (decoded, _): (Point, _) =
        oxicode::decode_from_slice_with_config(&encoded, config).expect("Failed to decode");

    assert_eq!(point, decoded);
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiGeneric<A, B> {
    first: A,
    second: B,
}

#[test]
fn test_multi_generic_struct() {
    let value = MultiGeneric {
        first: 42u32,
        second: "hello".to_string(),
    };
    let encoded = oxicode::encode_to_vec(&value).expect("Failed to encode");
    let (decoded, _): (MultiGeneric<u32, String>, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");
    assert_eq!(value, decoded);
}

#[test]
fn test_nested_options() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WithOptions {
        name: Option<String>,
        count: Option<u64>,
    }
    let value = WithOptions {
        name: Some("test".to_string()),
        count: None,
    };
    let encoded = oxicode::encode_to_vec(&value).expect("Failed to encode");
    let (decoded, _): (WithOptions, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");
    assert_eq!(value, decoded);
}

#[test]
fn test_nested_collections() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WithCollections {
        items: Vec<Vec<u32>>,
    }
    let value = WithCollections {
        items: vec![vec![1, 2], vec![3, 4, 5]],
    };
    let encoded = oxicode::encode_to_vec(&value).expect("Failed to encode");
    let (decoded, _): (WithCollections, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");
    assert_eq!(value, decoded);
}
