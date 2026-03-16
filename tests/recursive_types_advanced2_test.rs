//! Advanced tests for recursive and self-referential type serialization in OxiCode (set 2)

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
    encode_to_vec_with_config, Decode, Encode,
};

// ---------------------------------------------------------------------------
// Recursive / self-referential types used across all tests
// ---------------------------------------------------------------------------

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
enum BinaryTree {
    Leaf(u32),
    Node(Box<BinaryTree>, u32, Box<BinaryTree>),
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct ListNode {
    value: u32,
    next: Option<Box<ListNode>>,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
}

// ---------------------------------------------------------------------------
// BinaryTree tests
// ---------------------------------------------------------------------------

#[test]
fn test_binary_tree_leaf_roundtrip() {
    let original = BinaryTree::Leaf(42);
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BinaryTree, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_binary_tree_depth_1_roundtrip() {
    let original = BinaryTree::Node(
        Box::new(BinaryTree::Leaf(1)),
        10,
        Box::new(BinaryTree::Leaf(2)),
    );
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BinaryTree, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_binary_tree_depth_2_roundtrip() {
    let original = BinaryTree::Node(
        Box::new(BinaryTree::Node(
            Box::new(BinaryTree::Leaf(1)),
            5,
            Box::new(BinaryTree::Leaf(3)),
        )),
        10,
        Box::new(BinaryTree::Node(
            Box::new(BinaryTree::Leaf(7)),
            15,
            Box::new(BinaryTree::Leaf(20)),
        )),
    );
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BinaryTree, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_binary_tree_depth_3_roundtrip() {
    let original = BinaryTree::Node(
        Box::new(BinaryTree::Node(
            Box::new(BinaryTree::Node(
                Box::new(BinaryTree::Leaf(1)),
                3,
                Box::new(BinaryTree::Leaf(4)),
            )),
            7,
            Box::new(BinaryTree::Leaf(9)),
        )),
        15,
        Box::new(BinaryTree::Node(
            Box::new(BinaryTree::Leaf(20)),
            25,
            Box::new(BinaryTree::Node(
                Box::new(BinaryTree::Leaf(30)),
                35,
                Box::new(BinaryTree::Leaf(40)),
            )),
        )),
    );
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BinaryTree, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_binary_tree_left_skewed_roundtrip() {
    // left-skewed tree, depth 4
    let original = BinaryTree::Node(
        Box::new(BinaryTree::Node(
            Box::new(BinaryTree::Node(
                Box::new(BinaryTree::Node(
                    Box::new(BinaryTree::Leaf(1)),
                    2,
                    Box::new(BinaryTree::Leaf(3)),
                )),
                4,
                Box::new(BinaryTree::Leaf(5)),
            )),
            6,
            Box::new(BinaryTree::Leaf(7)),
        )),
        8,
        Box::new(BinaryTree::Leaf(9)),
    );
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BinaryTree, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_binary_tree_right_skewed_roundtrip() {
    // right-skewed tree, depth 4
    let original = BinaryTree::Node(
        Box::new(BinaryTree::Leaf(1)),
        2,
        Box::new(BinaryTree::Node(
            Box::new(BinaryTree::Leaf(3)),
            4,
            Box::new(BinaryTree::Node(
                Box::new(BinaryTree::Leaf(5)),
                6,
                Box::new(BinaryTree::Node(
                    Box::new(BinaryTree::Leaf(7)),
                    8,
                    Box::new(BinaryTree::Leaf(9)),
                )),
            )),
        )),
    );
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BinaryTree, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_binary_tree_consumed_equals_len() {
    let original = BinaryTree::Node(
        Box::new(BinaryTree::Leaf(100)),
        200,
        Box::new(BinaryTree::Leaf(300)),
    );
    let enc = encode_to_vec(&original).expect("encode failed");
    let (_, consumed): (BinaryTree, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// ListNode tests
// ---------------------------------------------------------------------------

#[test]
fn test_list_node_single_roundtrip() {
    let original = ListNode {
        value: 1,
        next: None,
    };
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (ListNode, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_list_node_two_elements_roundtrip() {
    let original = ListNode {
        value: 1,
        next: Some(Box::new(ListNode {
            value: 2,
            next: None,
        })),
    };
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (ListNode, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_list_node_three_elements_roundtrip() {
    let original = ListNode {
        value: 10,
        next: Some(Box::new(ListNode {
            value: 20,
            next: Some(Box::new(ListNode {
                value: 30,
                next: None,
            })),
        })),
    };
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (ListNode, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_list_node_values_preserved() {
    let original = ListNode {
        value: 100,
        next: Some(Box::new(ListNode {
            value: 200,
            next: Some(Box::new(ListNode {
                value: 300,
                next: Some(Box::new(ListNode {
                    value: 400,
                    next: None,
                })),
            })),
        })),
    };
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (ListNode, usize) = decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded.value, 100);
    let n1 = decoded.next.as_ref().expect("expected second node");
    assert_eq!(n1.value, 200);
    let n2 = n1.next.as_ref().expect("expected third node");
    assert_eq!(n2.value, 300);
    let n3 = n2.next.as_ref().expect("expected fourth node");
    assert_eq!(n3.value, 400);
    assert!(n3.next.is_none());
}

#[test]
fn test_list_node_consumed_equals_len() {
    let original = ListNode {
        value: 7,
        next: Some(Box::new(ListNode {
            value: 8,
            next: None,
        })),
    };
    let enc = encode_to_vec(&original).expect("encode failed");
    let (_, consumed): (ListNode, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// JsonValue tests
// ---------------------------------------------------------------------------

#[test]
fn test_json_value_null_roundtrip() {
    let original = JsonValue::Null;
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (JsonValue, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_json_value_bool_true_roundtrip() {
    let original = JsonValue::Bool(true);
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (JsonValue, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_json_value_bool_false_roundtrip() {
    let original = JsonValue::Bool(false);
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (JsonValue, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_json_value_number_roundtrip() {
    let original = JsonValue::Number(1.5f64);
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (JsonValue, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_json_value_str_roundtrip() {
    let original = JsonValue::Str("hello".to_string());
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (JsonValue, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_json_value_array_empty_roundtrip() {
    let original = JsonValue::Array(vec![]);
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (JsonValue, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_json_value_array_nested_roundtrip() {
    let original = JsonValue::Array(vec![
        JsonValue::Bool(true),
        JsonValue::Number(42.0f64),
        JsonValue::Str("world".to_string()),
    ]);
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (JsonValue, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_json_value_deeply_nested_array() {
    // Array containing Array containing Null
    let original = JsonValue::Array(vec![JsonValue::Array(vec![JsonValue::Null])]);
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (JsonValue, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Config-variant tests
// ---------------------------------------------------------------------------

#[test]
fn test_binary_tree_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = BinaryTree::Node(
        Box::new(BinaryTree::Leaf(11)),
        22,
        Box::new(BinaryTree::Leaf(33)),
    );
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, _): (BinaryTree, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_list_node_option_none_at_end() {
    // Verify that None correctly terminates the linked list
    let original = ListNode {
        value: 55,
        next: Some(Box::new(ListNode {
            value: 66,
            next: None,
        })),
    };
    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (ListNode, usize) = decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded.value, 55);
    let tail = decoded.next.as_ref().expect("expected tail node");
    assert_eq!(tail.value, 66);
    assert!(tail.next.is_none(), "tail.next must be None");
}
