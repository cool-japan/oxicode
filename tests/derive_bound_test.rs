//! Tests for the `#[oxicode(bound = "T: Encode + Decode")]` container attribute.
//!
//! The `bound` attribute overrides the automatically-generated where-clause on
//! `Encode`, `Decode`, and `BorrowDecode` impls, allowing callers to specify
//! tighter, looser, or entirely different trait constraints than the derive
//! macro would produce by default.
//!
//! When `bound` is set:
//!  - The `impl_generics` block uses the bare type params (no auto-bound in angles).
//!  - The `where` clause is replaced entirely by the user-supplied predicates.
//!  - An empty bound string (`bound = ""`) produces an impl with no where clause at all.

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

// ---------------------------------------------------------------------------
// Test 1 – Simple generic struct with a single explicit bound (roundtrip)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode")]
struct BoundSingle<T> {
    value: T,
    tag: u32,
}

#[test]
fn test_01_simple_generic_explicit_bound_roundtrip() {
    let original = BoundSingle {
        value: 0xDEAD_u32,
        tag: 7,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundSingle<u32>, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 2 – Generic struct with two type params and explicit bounds on each
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "K: Encode + Decode, V: Encode + Decode")]
struct BoundPair<K, V> {
    key: K,
    val: V,
}

#[test]
fn test_02_two_type_params_explicit_bounds_roundtrip() {
    let original = BoundPair {
        key: "hello".to_string(),
        val: 99u64,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundPair<String, u64>, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 3 – Nested generic: struct holds Vec<T>; bound is placed on T directly
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode")]
struct BoundVecField<T> {
    items: Vec<T>,
    label: String,
}

#[test]
fn test_03_nested_vec_explicit_bound_roundtrip() {
    let original = BoundVecField {
        items: vec![10u32, 20, 30, 40],
        label: "numbers".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundVecField<u32>, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 4 – Enum with generic variants and explicit bound
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode")]
enum BoundMaybe<T> {
    Nothing,
    Just(T),
    Pair { first: T, second: T },
}

#[test]
fn test_04_enum_generic_variants_explicit_bound_roundtrip() {
    let cases: Vec<BoundMaybe<i32>> = vec![
        BoundMaybe::Nothing,
        BoundMaybe::Just(-7),
        BoundMaybe::Pair {
            first: 100,
            second: 200,
        },
    ];
    for case in cases {
        let enc = encode_to_vec(&case).expect("encode");
        let (decoded, _): (BoundMaybe<i32>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(case, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 5 – Tighter bound: user adds Clone on top of Encode + Decode
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode + Clone")]
struct BoundCloneRequired<T> {
    data: T,
}

#[test]
fn test_05_tighter_bound_clone_roundtrip() {
    let original = BoundCloneRequired {
        data: vec![1u8, 2, 3],
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundCloneRequired<Vec<u8>>, usize) =
        decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 6 – Generic struct with Vec<T> field and explicit bound (roundtrip)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode")]
struct BoundCollection<T> {
    elements: Vec<T>,
    count: usize,
}

#[test]
fn test_06_vec_field_explicit_bound_roundtrip() {
    let original = BoundCollection {
        elements: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
        count: 3,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundCollection<String>, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 7 – Generic struct with Option<T> field and explicit bound
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode")]
struct BoundOptional<T> {
    maybe: Option<T>,
    always: T,
}

#[test]
fn test_07_option_field_explicit_bound_roundtrip() {
    let with_some = BoundOptional {
        maybe: Some(42u32),
        always: 1u32,
    };
    let without = BoundOptional::<u32> {
        maybe: None,
        always: 0,
    };
    for original in [with_some, without] {
        let enc = encode_to_vec(&original).expect("encode");
        let (decoded, _): (BoundOptional<u32>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(original, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 8 – Multi-level nested generic (Vec<Vec<T>>) with explicit bound
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode")]
struct BoundMatrix<T> {
    rows: Vec<Vec<T>>,
    width: u32,
    height: u32,
}

#[test]
fn test_08_multi_level_nested_generic_bound_roundtrip() {
    let original = BoundMatrix {
        rows: vec![vec![1u16, 2, 3], vec![4, 5, 6]],
        width: 3,
        height: 2,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundMatrix<u16>, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 9 – Generic struct WITHOUT explicit bound (default auto-generated bounds)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct NoBoundGeneric<T> {
    inner: T,
    flag: bool,
}

#[test]
fn test_09_no_bound_attribute_default_behavior_roundtrip() {
    let original = NoBoundGeneric {
        inner: "world".to_string(),
        flag: true,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (NoBoundGeneric<String>, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 10 – Explicit bound with PartialOrd to tighten the constraint
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode + PartialOrd")]
struct BoundOrdered<T> {
    min: T,
    max: T,
}

#[test]
fn test_10_bound_with_partialord_constraint_roundtrip() {
    let original = BoundOrdered {
        min: 1i64,
        max: 100i64,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundOrdered<i64>, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 11 – Unit-like struct with empty bound string (no where clause)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "")]
struct BoundEmpty {
    id: u32,
    label: String,
    active: bool,
}

#[test]
fn test_11_unit_struct_empty_bound_roundtrip() {
    let original = BoundEmpty {
        id: 999,
        label: "empty-bound".to_string(),
        active: true,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundEmpty, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 12 – Tuple struct with single generic and explicit bound
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode")]
struct BoundTuple1<T>(T);

#[test]
fn test_12_tuple_struct_single_field_explicit_bound_roundtrip() {
    let original = BoundTuple1(255u8);
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundTuple1<u8>, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 13 – Tuple struct with two generics and explicit bounds
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "A: Encode + Decode, B: Encode + Decode")]
struct BoundTuple2<A, B>(A, B);

#[test]
fn test_13_tuple_struct_two_generics_explicit_bounds_roundtrip() {
    let original = BoundTuple2(42i32, true);
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundTuple2<i32, bool>, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 14 – Enum with struct-like variants and explicit bound
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode")]
enum BoundShape<T> {
    Circle { radius: T },
    Rectangle { width: T, height: T },
    Point,
}

#[test]
fn test_14_enum_struct_variants_explicit_bound_roundtrip() {
    let cases: Vec<BoundShape<u32>> = vec![
        BoundShape::Circle { radius: 5 },
        BoundShape::Rectangle {
            width: 10,
            height: 20,
        },
        BoundShape::Point,
    ];
    for case in cases {
        let enc = encode_to_vec(&case).expect("encode");
        let (decoded, _): (BoundShape<u32>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(case, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 15 – Three type params with explicit bounds on each
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "X: Encode + Decode, Y: Encode + Decode, Z: Encode + Decode")]
struct BoundTriple<X, Y, Z> {
    x: X,
    y: Y,
    z: Z,
}

#[test]
fn test_15_three_type_params_explicit_bounds_roundtrip() {
    let original = BoundTriple {
        x: 1u8,
        y: 1000u32,
        z: "triple".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundTriple<u8, u32, String>, usize) =
        decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 16 – Generic struct holding another generic struct (nested types)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode")]
struct BoundOuter<T> {
    inner: BoundSingle<T>,
    extra: u64,
}

#[test]
fn test_16_struct_holding_generic_struct_bound_roundtrip() {
    let original = BoundOuter {
        inner: BoundSingle {
            value: 7u32,
            tag: 3,
        },
        extra: 9999u64,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundOuter<u32>, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 17 – Enum mixing unit, tuple, and struct variants with explicit bound
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode")]
enum BoundMixed<T> {
    Empty,
    Single(T),
    Named { value: T, tag: u32 },
    Two(T, T),
}

#[test]
fn test_17_enum_mixed_variant_kinds_explicit_bound_roundtrip() {
    let cases: Vec<BoundMixed<String>> = vec![
        BoundMixed::Empty,
        BoundMixed::Single("one".to_string()),
        BoundMixed::Named {
            value: "named".to_string(),
            tag: 42,
        },
        BoundMixed::Two("a".to_string(), "b".to_string()),
    ];
    for case in cases {
        let enc = encode_to_vec(&case).expect("encode");
        let (decoded, _): (BoundMixed<String>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(case, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 18 – Large data volume through a bounded generic container
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode")]
struct BoundBuffer<T> {
    data: Vec<T>,
    version: u32,
}

#[test]
fn test_18_large_data_volume_explicit_bound_roundtrip() {
    let data: Vec<u64> = (0u64..512).collect();
    let original = BoundBuffer { data, version: 1 };
    let enc = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundBuffer<u64>, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, decoded);
    assert_eq!(decoded.data.len(), 512);
    assert_eq!(decoded.data[511], 511u64);
}
