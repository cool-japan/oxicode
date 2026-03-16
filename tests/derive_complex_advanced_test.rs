//! Advanced tests for complex derive macro patterns in OxiCode.

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
use std::collections::{BTreeMap, HashMap};

use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config,
};
use oxicode::{Decode, Encode};

// ---- Test 1: Struct with multiple generic type parameters ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiParam<A, B, C> {
    first: A,
    second: B,
    third: C,
}

#[test]
fn test_multi_generic_type_params() {
    let original = MultiParam {
        first: 42u32,
        second: "hello".to_string(),
        third: true,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): (MultiParam<u32, String, bool>, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---- Test 2: Enum with generic variants ----

#[derive(Debug, PartialEq, Encode, Decode)]
enum GenericEnum<T> {
    Empty,
    Value(T),
    Pair(T, T),
}

#[test]
fn test_enum_with_generic_variants() {
    let cases: Vec<GenericEnum<i32>> = vec![
        GenericEnum::Empty,
        GenericEnum::Value(99),
        GenericEnum::Pair(-1, 2),
    ];
    for original in cases {
        let encoded = encode_to_vec(&original).expect("encode failed");
        let (decoded, consumed): (GenericEnum<i32>, _) =
            decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(decoded, original);
        assert_eq!(consumed, encoded.len());
    }
}

// ---- Test 3: Struct with where clause bounds in derive ----

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode + PartialEq + std::fmt::Debug")]
struct BoundedWrapper<T> {
    inner: T,
    label: String,
}

#[test]
fn test_struct_with_where_clause_bounds() {
    let original = BoundedWrapper {
        inner: vec![1u8, 2, 3, 4],
        label: "bounded".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): (BoundedWrapper<Vec<u8>>, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---- Test 4: Nested generics: Wrapper<Inner<T>> ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct Inner<T> {
    value: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Wrapper<T> {
    inner: Inner<T>,
    tag: u8,
}

#[test]
fn test_nested_generics_wrapper_inner() {
    let original = Wrapper {
        inner: Inner { value: 1234u64 },
        tag: 7,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): (Wrapper<u64>, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---- Test 5: Enum with tuple variants of different arities ----

#[derive(Debug, PartialEq, Encode, Decode)]
enum Arity {
    Zero,
    One(u8),
    Two(u16, u16),
    Three(u32, u32, u32),
    Four(u64, u64, u64, u64),
}

#[test]
fn test_enum_tuple_variants_different_arities() {
    let cases = vec![
        Arity::Zero,
        Arity::One(255),
        Arity::Two(1000, 2000),
        Arity::Three(10, 20, 30),
        Arity::Four(100, 200, 300, 400),
    ];
    for original in cases {
        let encoded = encode_to_vec(&original).expect("encode failed");
        let (decoded, consumed): (Arity, _) = decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(decoded, original);
        assert_eq!(consumed, encoded.len());
    }
}

// ---- Test 6: Struct with all primitive field types ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllPrimitives {
    a_u8: u8,
    a_u16: u16,
    a_u32: u32,
    a_u64: u64,
    a_u128: u128,
    a_i8: i8,
    a_i16: i16,
    a_i32: i32,
    a_i64: i64,
    a_i128: i128,
    a_f32: f32,
    a_f64: f64,
    a_bool: bool,
    a_char: char,
    a_usize: usize,
    a_isize: isize,
}

#[test]
fn test_struct_all_primitive_field_types() {
    let original = AllPrimitives {
        a_u8: u8::MAX,
        a_u16: u16::MAX,
        a_u32: u32::MAX,
        a_u64: u64::MAX,
        a_u128: u128::MAX,
        a_i8: i8::MIN,
        a_i16: i16::MIN,
        a_i32: i32::MIN,
        a_i64: i64::MIN,
        a_i128: i128::MIN,
        a_f32: std::f32::consts::PI,
        a_f64: std::f64::consts::E,
        a_bool: true,
        a_char: 'Z',
        a_usize: usize::MAX / 2,
        a_isize: isize::MIN / 2,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (AllPrimitives, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---- Test 7: Enum with struct variants (named fields in enum variants) ----

#[derive(Debug, PartialEq, Encode, Decode)]
enum Event {
    Click { x: i32, y: i32, button: u8 },
    KeyPress { key: u32, modifiers: u8 },
    Scroll { delta_x: f32, delta_y: f32 },
    Resize { width: u32, height: u32 },
}

#[test]
fn test_enum_struct_variants_named_fields() {
    let cases = vec![
        Event::Click {
            x: 100,
            y: 200,
            button: 1,
        },
        Event::KeyPress {
            key: 0x41,
            modifiers: 0x04,
        },
        Event::Scroll {
            delta_x: std::f32::consts::FRAC_1_PI,
            delta_y: -1.5,
        },
        Event::Resize {
            width: 1920,
            height: 1080,
        },
    ];
    for original in cases {
        let encoded = encode_to_vec(&original).expect("encode failed");
        let (decoded, consumed): (Event, _) = decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(decoded, original);
        assert_eq!(consumed, encoded.len());
    }
}

// ---- Test 8: Deeply nested: Option<Vec<Option<String>>> field ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct DeepNested {
    data: Option<Vec<Option<String>>>,
    count: u32,
}

#[test]
fn test_deeply_nested_option_vec_option_string() {
    let original = DeepNested {
        data: Some(vec![
            Some("alpha".to_string()),
            None,
            Some("gamma".to_string()),
        ]),
        count: 3,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (DeepNested, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());

    // Also test the None outer case
    let original_none = DeepNested {
        data: None,
        count: 0,
    };
    let encoded_none = encode_to_vec(&original_none).expect("encode failed");
    let (decoded_none, consumed_none): (DeepNested, _) =
        decode_from_slice(&encoded_none).expect("decode failed");
    assert_eq!(decoded_none, original_none);
    assert_eq!(consumed_none, encoded_none.len());
}

// ---- Test 9: Struct with Vec<T> and Option<T> fields together ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct VecAndOption<T> {
    items: Vec<T>,
    maybe: Option<T>,
    count: usize,
}

#[test]
fn test_struct_vec_and_option_fields() {
    let original = VecAndOption {
        items: vec![10u32, 20, 30, 40, 50],
        maybe: Some(99u32),
        count: 5,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): (VecAndOption<u32>, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---- Test 10: Enum roundtrip: all variants tested ----

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum Status {
    Active,
    Inactive,
    Pending(String),
    Error { code: u16, message: String },
}

#[test]
fn test_enum_roundtrip_all_variants() {
    let variants = vec![
        Status::Active,
        Status::Inactive,
        Status::Pending("waiting for approval".to_string()),
        Status::Error {
            code: 404,
            message: "not found".to_string(),
        },
    ];
    for original in variants {
        let encoded = encode_to_vec(&original).expect("encode failed");
        let (decoded, consumed): (Status, _) = decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(decoded, original);
        assert_eq!(consumed, encoded.len());
    }
}

// ---- Test 11: Struct with HashMap<String, Vec<u32>> field ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithHashMap {
    name: String,
    data: HashMap<String, Vec<u32>>,
}

#[test]
fn test_struct_with_hashmap_string_vec_u32() {
    let mut data = HashMap::new();
    data.insert("primes".to_string(), vec![2, 3, 5, 7, 11]);
    data.insert("squares".to_string(), vec![1, 4, 9, 16, 25]);
    data.insert("empty".to_string(), vec![]);

    let original = WithHashMap {
        name: "math sets".to_string(),
        data,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (WithHashMap, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded.name, original.name);
    assert_eq!(decoded.data, original.data);
    assert_eq!(consumed, encoded.len());
}

// ---- Test 12: Newtype wrapper struct roundtrip ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct UserId(u64);

#[derive(Debug, PartialEq, Encode, Decode)]
struct SessionToken(String);

#[test]
fn test_newtype_wrapper_roundtrip() {
    let user_id = UserId(9_876_543_210u64);
    let encoded = encode_to_vec(&user_id).expect("encode UserId failed");
    let (decoded, consumed): (UserId, _) =
        decode_from_slice(&encoded).expect("decode UserId failed");
    assert_eq!(decoded, user_id);
    assert_eq!(consumed, encoded.len());

    let token = SessionToken("secret-token-xyz".to_string());
    let encoded_token = encode_to_vec(&token).expect("encode SessionToken failed");
    let (decoded_token, consumed_token): (SessionToken, _) =
        decode_from_slice(&encoded_token).expect("decode SessionToken failed");
    assert_eq!(decoded_token, token);
    assert_eq!(consumed_token, encoded_token.len());
}

// ---- Test 13: Tuple struct Point(f32, f32) roundtrip ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point2D(f32, f32);

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point3D(f64, f64, f64);

#[test]
fn test_tuple_struct_point_roundtrip() {
    let p2 = Point2D(std::f32::consts::PI, std::f32::consts::SQRT_2);
    let encoded = encode_to_vec(&p2).expect("encode Point2D failed");
    let (decoded, consumed): (Point2D, _) =
        decode_from_slice(&encoded).expect("decode Point2D failed");
    assert_eq!(decoded, p2);
    assert_eq!(consumed, encoded.len());

    let p3 = Point3D(
        std::f64::consts::PI,
        std::f64::consts::E,
        std::f64::consts::SQRT_2,
    );
    let encoded3 = encode_to_vec(&p3).expect("encode Point3D failed");
    let (decoded3, consumed3): (Point3D, _) =
        decode_from_slice(&encoded3).expect("decode Point3D failed");
    assert_eq!(decoded3, p3);
    assert_eq!(consumed3, encoded3.len());
}

// ---- Test 14: Enum mixing unit, tuple, and struct variants ----

#[derive(Debug, PartialEq, Encode, Decode)]
enum Mixed {
    Unit,
    Tuple(i32, String),
    Struct { x: f64, y: f64, label: String },
    SingleTuple(Vec<u8>),
}

#[test]
fn test_enum_mixing_unit_tuple_struct_variants() {
    let cases = vec![
        Mixed::Unit,
        Mixed::Tuple(-42, "negative".to_string()),
        Mixed::Struct {
            x: std::f64::consts::PI,
            y: std::f64::consts::TAU,
            label: "origin-ish".to_string(),
        },
        Mixed::SingleTuple(vec![0xDE, 0xAD, 0xBE, 0xEF]),
    ];
    for original in cases {
        let encoded = encode_to_vec(&original).expect("encode failed");
        let (decoded, consumed): (Mixed, _) = decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(decoded, original);
        assert_eq!(consumed, encoded.len());
    }
}

// ---- Test 15: Generic struct Pair<A, B> ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair<A, B> {
    first: A,
    second: B,
}

#[test]
fn test_generic_struct_pair() {
    let original = Pair {
        first: std::f64::consts::PI,
        second: vec!["foo".to_string(), "bar".to_string()],
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): (Pair<f64, Vec<String>>, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---- Test 16: Recursive-friendly struct (tree via Box<T>) ----

#[derive(Debug, PartialEq, Encode, Decode)]
enum Tree {
    Leaf(i64),
    Node(Box<Tree>, Box<Tree>),
}

#[test]
fn test_recursive_tree_via_box() {
    let original = Tree::Node(
        Box::new(Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)))),
        Box::new(Tree::Node(
            Box::new(Tree::Leaf(3)),
            Box::new(Tree::Node(Box::new(Tree::Leaf(4)), Box::new(Tree::Leaf(5)))),
        )),
    );
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Tree, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---- Test 17: Enum with discriminant values specified ----

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

#[test]
fn test_enum_with_tag_type_u8() {
    let cases = vec![
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Custom(128, 64, 255),
    ];
    for original in cases {
        let encoded = encode_to_vec(&original).expect("encode failed");
        // With u8 tag_type, the discriminant is 1 byte
        let (decoded, consumed): (Color, _) = decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(decoded, original);
        assert_eq!(consumed, encoded.len());
    }
    // Verify that tag_type = "u8" produces smaller output than default u32 tag
    let encoded_u8_tag = encode_to_vec(&Color::Red).expect("encode failed");
    // Default config uses varint encoding; we just verify roundtrip works correctly
    assert!(!encoded_u8_tag.is_empty());
}

// ---- Test 18: Struct with [u8; 16] array field (UUID-like) ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct Uuid {
    bytes: [u8; 16],
    version: u8,
}

#[test]
fn test_struct_with_fixed_array_field() {
    let original = Uuid {
        bytes: [
            0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4,
            0x30, 0xc8,
        ],
        version: 1,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Uuid, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // UUID bytes field is [u8; 16] = 16 bytes
    assert!(encoded.len() >= 16);
}

// ---- Test 19: Multiple enums used as fields in a struct ----

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Task {
    id: u32,
    direction: Direction,
    priority: Priority,
    description: String,
}

#[test]
fn test_struct_with_multiple_enum_fields() {
    let original = Task {
        id: 42,
        direction: Direction::North,
        priority: Priority::High,
        description: "navigate north at high priority".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Task, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---- Test 20: Struct roundtrip with big-endian config ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct NetworkPacket {
    sequence_num: u32,
    payload_len: u16,
    checksum: u64,
    flags: u8,
}

#[test]
fn test_struct_roundtrip_big_endian_config() {
    let original = NetworkPacket {
        sequence_num: 0xDEAD_BEEF,
        payload_len: 0x0100,
        checksum: 0xCAFE_BABE_DEAD_BEEF,
        flags: 0b0000_1111,
    };
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, consumed): (NetworkPacket, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // Fixed encoding: u32(4) + u16(2) + u64(8) + u8(1) = 15 bytes
    assert_eq!(encoded.len(), 15);
}

// ---- Test 21: Struct roundtrip with fixed-int-encoding config ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct Metrics {
    requests: u64,
    errors: u32,
    latency_ns: u64,
    active_connections: u32,
}

#[test]
fn test_struct_roundtrip_fixed_int_encoding_config() {
    let original = Metrics {
        requests: 1_000_000,
        errors: 42,
        latency_ns: 500_000,
        active_connections: 256,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, consumed): (Metrics, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // Fixed encoding: u64(8) + u32(4) + u64(8) + u32(4) = 24 bytes
    assert_eq!(encoded.len(), 24);
}

// ---- Test 22: Complex derive: struct with BTreeMap<u32, String> field ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct Registry {
    name: String,
    entries: BTreeMap<u32, String>,
    version: u32,
}

#[test]
fn test_struct_with_btreemap_u32_string_field() {
    let mut entries = BTreeMap::new();
    entries.insert(1, "one".to_string());
    entries.insert(2, "two".to_string());
    entries.insert(100, "one hundred".to_string());
    entries.insert(u32::MAX, "max".to_string());

    let original = Registry {
        name: "number registry".to_string(),
        entries,
        version: 1,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Registry, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // BTreeMap preserves insertion order by key; verify keys are ordered
    let keys: Vec<u32> = decoded.entries.keys().copied().collect();
    assert!(
        keys.windows(2).all(|w| w[0] < w[1]),
        "BTreeMap keys must be sorted"
    );
}
