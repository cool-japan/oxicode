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
#[cfg(feature = "serde")]
use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Record {
    id: u64,
    name: String,
    active: bool,
    tags: Vec<String>,
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
struct CamelCaseStruct {
    first_name: String,
    last_name: String,
    age: u32,
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum Event {
    Login { user_id: u64 },
    Logout { user_id: u64 },
    Message { content: String },
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_u32_roundtrip() {
    let cfg = oxicode::config::standard();
    let value: u32 = 42;
    let bytes = serde_encode(&value, cfg).expect("encode u32");
    let (decoded, _): (u32, usize) = serde_decode(&bytes, cfg).expect("decode u32");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_u64_roundtrip() {
    let cfg = oxicode::config::standard();
    let value: u64 = 1_000_000_000_000;
    let bytes = serde_encode(&value, cfg).expect("encode u64");
    let (decoded, _): (u64, usize) = serde_decode(&bytes, cfg).expect("decode u64");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_string_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = "hello, oxicode!".to_string();
    let bytes = serde_encode(&value, cfg).expect("encode String");
    let (decoded, _): (String, usize) = serde_decode(&bytes, cfg).expect("decode String");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_bool_true_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = true;
    let bytes = serde_encode(&value, cfg).expect("encode bool true");
    let (decoded, _): (bool, usize) = serde_decode(&bytes, cfg).expect("decode bool true");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_bool_false_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = false;
    let bytes = serde_encode(&value, cfg).expect("encode bool false");
    let (decoded, _): (bool, usize) = serde_decode(&bytes, cfg).expect("decode bool false");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_vec_u32_roundtrip() {
    let cfg = oxicode::config::standard();
    let value: Vec<u32> = vec![1, 2, 3, 4, 5];
    let bytes = serde_encode(&value, cfg).expect("encode Vec<u32>");
    let (decoded, _): (Vec<u32>, usize) = serde_decode(&bytes, cfg).expect("decode Vec<u32>");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_option_some_roundtrip() {
    let cfg = oxicode::config::standard();
    let value: Option<u32> = Some(99);
    let bytes = serde_encode(&value, cfg).expect("encode Option<u32> Some");
    let (decoded, _): (Option<u32>, usize) =
        serde_decode(&bytes, cfg).expect("decode Option<u32> Some");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_option_none_roundtrip() {
    let cfg = oxicode::config::standard();
    let value: Option<u32> = None;
    let bytes = serde_encode(&value, cfg).expect("encode Option<u32> None");
    let (decoded, _): (Option<u32>, usize) =
        serde_decode(&bytes, cfg).expect("decode Option<u32> None");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_point_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = Point {
        x: 1.5f64,
        y: 2.5f64,
    };
    let bytes = serde_encode(&value, cfg).expect("encode Point");
    let (decoded, _): (Point, usize) = serde_decode(&bytes, cfg).expect("decode Point");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_record_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = Record {
        id: 7,
        name: "Alice".to_string(),
        active: true,
        tags: vec!["admin".to_string(), "user".to_string()],
    };
    let bytes = serde_encode(&value, cfg).expect("encode Record");
    let (decoded, _): (Record, usize) = serde_decode(&bytes, cfg).expect("decode Record");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_vec_record_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = vec![
        Record {
            id: 1,
            name: "Bob".to_string(),
            active: false,
            tags: vec!["viewer".to_string()],
        },
        Record {
            id: 2,
            name: "Carol".to_string(),
            active: true,
            tags: vec!["editor".to_string(), "admin".to_string()],
        },
    ];
    let bytes = serde_encode(&value, cfg).expect("encode Vec<Record>");
    let (decoded, _): (Vec<Record>, usize) = serde_decode(&bytes, cfg).expect("decode Vec<Record>");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_camel_case_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = CamelCaseStruct {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    };
    let bytes = serde_encode(&value, cfg).expect("encode CamelCaseStruct");
    let (decoded, _): (CamelCaseStruct, usize) =
        serde_decode(&bytes, cfg).expect("decode CamelCaseStruct");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_event_login_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = Event::Login { user_id: 101 };
    let bytes = serde_encode(&value, cfg).expect("encode Event::Login");
    let (decoded, _): (Event, usize) = serde_decode(&bytes, cfg).expect("decode Event::Login");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_event_logout_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = Event::Logout { user_id: 202 };
    let bytes = serde_encode(&value, cfg).expect("encode Event::Logout");
    let (decoded, _): (Event, usize) = serde_decode(&bytes, cfg).expect("decode Event::Logout");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_event_message_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = Event::Message {
        content: "Hello world".to_string(),
    };
    let bytes = serde_encode(&value, cfg).expect("encode Event::Message");
    let (decoded, _): (Event, usize) = serde_decode(&bytes, cfg).expect("decode Event::Message");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_tuple_roundtrip() {
    let cfg = oxicode::config::standard();
    let value: (u32, String) = (123, "tuple_value".to_string());
    let bytes = serde_encode(&value, cfg).expect("encode (u32, String)");
    let (decoded, _): ((u32, String), usize) =
        serde_decode(&bytes, cfg).expect("decode (u32, String)");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_nested_vec_roundtrip() {
    let cfg = oxicode::config::standard();
    let value: Vec<Vec<u32>> = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    let bytes = serde_encode(&value, cfg).expect("encode Vec<Vec<u32>>");
    let (decoded, _): (Vec<Vec<u32>>, usize) =
        serde_decode(&bytes, cfg).expect("decode Vec<Vec<u32>>");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_consumed_equals_len() {
    let cfg = oxicode::config::standard();
    let value: u32 = 55;
    let bytes = serde_encode(&value, cfg).expect("encode u32 for size check");
    let (_decoded, consumed): (u32, usize) =
        serde_decode(&bytes, cfg).expect("decode u32 for size check");
    assert_eq!(consumed, bytes.len());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_i32_negative_roundtrip() {
    let cfg = oxicode::config::standard();
    let value: i32 = -42;
    let bytes = serde_encode(&value, cfg).expect("encode i32 negative");
    let (decoded, _): (i32, usize) = serde_decode(&bytes, cfg).expect("decode i32 negative");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_empty_string_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = String::new();
    let bytes = serde_encode(&value, cfg).expect("encode empty String");
    let (decoded, _): (String, usize) = serde_decode(&bytes, cfg).expect("decode empty String");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_empty_vec_roundtrip() {
    let cfg = oxicode::config::standard();
    let value: Vec<u32> = Vec::new();
    let bytes = serde_encode(&value, cfg).expect("encode empty Vec<u32>");
    let (decoded, _): (Vec<u32>, usize) = serde_decode(&bytes, cfg).expect("decode empty Vec<u32>");
    assert_eq!(value, decoded);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_large_u64_roundtrip() {
    let cfg = oxicode::config::standard();
    let value: u64 = u64::MAX;
    let bytes = serde_encode(&value, cfg).expect("encode u64::MAX");
    let (decoded, _): (u64, usize) = serde_decode(&bytes, cfg).expect("decode u64::MAX");
    assert_eq!(value, decoded);
}
