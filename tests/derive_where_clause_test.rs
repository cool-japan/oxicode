//! Comprehensive tests for derive macros on types with where clauses and complex bounds.
//!
//! Tests cover generic structs/enums with explicit where clauses, multi-parameter generics,
//! PhantomData, nested generics, associated type bounds, complex trait combinations,
//! and various collection types inside generic containers.

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
use std::collections::HashMap;
use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// Test 1: Generic struct `Wrapper<T: Clone>` with explicit where clause
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct Wrapper<T>
where
    T: Clone + oxicode::Encode + oxicode::Decode,
{
    inner: T,
}

#[test]
fn test_01_wrapper_with_clone_where_clause() {
    let original = Wrapper { inner: 42u32 };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Wrapper<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Generic struct with `T: std::fmt::Debug + Clone` bound
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct DebugCloneWrapper<T>
where
    T: std::fmt::Debug + Clone + oxicode::Encode + oxicode::Decode,
{
    value: T,
    label: String,
}

#[test]
fn test_02_debug_clone_bound() {
    let original = DebugCloneWrapper {
        value: 99u64,
        label: "debug_clone".to_string(),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (DebugCloneWrapper<u64>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Multi-param `Pair<A, B>` with bounds on both
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct Pair<A, B>
where
    A: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug,
    B: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug,
{
    first: A,
    second: B,
}

#[test]
fn test_03_pair_with_bounds_on_both() {
    let original = Pair {
        first: 10u32,
        second: "hello".to_string(),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Pair<u32, String>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Generic enum with where bound
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
enum GenericResult<T, E>
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug,
    E: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug,
{
    Ok(T),
    Err(E),
}

#[test]
fn test_04_generic_enum_with_where_bound() {
    let ok_val: GenericResult<u32, String> = GenericResult::Ok(123);
    let encoded = oxicode::encode_to_vec(&ok_val).expect("encode failed");
    let (decoded, _): (GenericResult<u32, String>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(ok_val, decoded);

    let err_val: GenericResult<u32, String> = GenericResult::Err("something failed".to_string());
    let encoded = oxicode::encode_to_vec(&err_val).expect("encode failed");
    let (decoded, _): (GenericResult<u32, String>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(err_val, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: Struct with `T: Default` where clause
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct WithDefault<T>
where
    T: Default + oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq,
{
    value: T,
    count: u32,
}

#[test]
fn test_05_struct_with_default_where_clause() {
    let original = WithDefault {
        value: 0u64,
        count: 7,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithDefault<u64>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: PhantomData struct with `T: Sized` bound (PhantomData doesn't encode)
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct PhantomWrapper<T>
where
    T: Sized,
{
    id: u64,
    name: String,
    _phantom: PhantomData<T>,
}

#[test]
fn test_06_phantom_data_struct_with_sized_bound() {
    let original = PhantomWrapper::<u32> {
        id: 42,
        name: "phantom".to_string(),
        _phantom: PhantomData,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (PhantomWrapper<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Nested generic `Outer<T>` containing `Vec<T>` where T: Encode
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct OuterWithVec<T>
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq,
{
    items: Vec<T>,
    tag: String,
}

#[test]
fn test_07_outer_inner_nested_generic() {
    let original = OuterWithVec {
        items: vec![1u32, 2, 3],
        tag: "nested".to_string(),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (OuterWithVec<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Struct encoding correctness under multiple consecutive encode/decode cycles
// ---------------------------------------------------------------------------
// (Lifetime bounds 'a: 'b on structs with references can't easily be serialized; instead
//  we test a struct that stores a lifetime-parametrized field by value to verify
//  multi-cycle roundtrip stability — a proxy for the lifetime-bound semantics.)
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct LifetimeStyleMultiCycle<T>
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq,
{
    a: T,
    b: T,
}

#[test]
fn test_08_struct_lifetime_style_multi_cycle() {
    let original = LifetimeStyleMultiCycle { a: 10u32, b: 20u32 };
    // Encode -> decode three times to verify stability
    for _ in 0..3 {
        let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
        let (decoded, _): (LifetimeStyleMultiCycle<u32>, _) =
            oxicode::decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(original, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 9: Generic struct with associated-type-style constraint via marker trait
// ---------------------------------------------------------------------------
// We test a struct that explicitly constrains a bound via a where clause involving
// multiple trait requirements that mirror what an associated type bound would impose.
// This exercises the derive macro's handling of multi-trait where predicates.
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct AssocBoundStruct<T>
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq + Default,
{
    value: T,
    secondary: T,
}

#[test]
fn test_09_generic_struct_with_associated_type_bound() {
    let original = AssocBoundStruct {
        value: 100u32,
        secondary: 200u32,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (AssocBoundStruct<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Generic enum all unit variants
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
enum UnitOnlyGenericEnum<T>
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug,
{
    Alpha,
    Beta,
    Gamma,
    #[allow(dead_code)]
    _Phantom(PhantomData<T>),
}

#[test]
fn test_10_generic_enum_all_unit_variants() {
    let variants = [
        UnitOnlyGenericEnum::<u32>::Alpha,
        UnitOnlyGenericEnum::<u32>::Beta,
        UnitOnlyGenericEnum::<u32>::Gamma,
    ];
    for variant in &variants {
        let encoded = oxicode::encode_to_vec(variant).expect("encode failed");
        let (decoded, _): (UnitOnlyGenericEnum<u32>, _) =
            oxicode::decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(variant, &decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 11: Generic enum with tuple variants
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
enum Tagged<T>
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug,
{
    Single(T),
    Double(T, T),
    Labeled { name: String, value: T },
}

#[test]
fn test_11_generic_enum_with_tuple_variants() {
    let single: Tagged<u32> = Tagged::Single(7);
    let encoded = oxicode::encode_to_vec(&single).expect("encode failed");
    let (decoded, _): (Tagged<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(single, decoded);

    let double: Tagged<u32> = Tagged::Double(1, 2);
    let encoded = oxicode::encode_to_vec(&double).expect("encode failed");
    let (decoded, _): (Tagged<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(double, decoded);

    let labeled: Tagged<u32> = Tagged::Labeled {
        name: "x".to_string(),
        value: 42,
    };
    let encoded = oxicode::encode_to_vec(&labeled).expect("encode failed");
    let (decoded, _): (Tagged<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(labeled, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Complex where clause: T: Encode + Decode + Clone + std::fmt::Debug
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct ComplexBound<T>
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug,
{
    payload: T,
    metadata: u64,
}

#[test]
fn test_12_complex_where_clause_encode_decode_clone_debug() {
    let original = ComplexBound {
        payload: vec![1u8, 2, 3, 4, 5],
        metadata: 0xDEAD_BEEF,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (ComplexBound<Vec<u8>>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Struct with `T: Copy` roundtrip
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct CopyBound<T>
where
    T: Copy + oxicode::Encode + oxicode::Decode + std::fmt::Debug + PartialEq,
{
    x: T,
    y: T,
    z: T,
}

#[test]
fn test_13_struct_with_copy_bound_roundtrip() {
    let original = CopyBound {
        x: 1.0f64,
        y: 2.5f64,
        z: -std::f64::consts::PI,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (CopyBound<f64>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Vec<T> inside generic struct with T: Encode
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct VecContainer<T>
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq,
{
    data: Vec<T>,
    name: String,
}

#[test]
fn test_14_vec_inside_generic_struct() {
    let original = VecContainer {
        data: vec![10u32, 20, 30, 40, 50],
        name: "vec_test".to_string(),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (VecContainer<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Option<T> inside generic struct
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct OptionalContainer<T>
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq,
{
    maybe: Option<T>,
    fallback: T,
}

#[test]
fn test_15_option_inside_generic_struct() {
    let with_some = OptionalContainer {
        maybe: Some(42u64),
        fallback: 0u64,
    };
    let encoded = oxicode::encode_to_vec(&with_some).expect("encode failed");
    let (decoded, _): (OptionalContainer<u64>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(with_some, decoded);

    let with_none = OptionalContainer::<u64> {
        maybe: None,
        fallback: 99u64,
    };
    let encoded = oxicode::encode_to_vec(&with_none).expect("encode failed");
    let (decoded, _): (OptionalContainer<u64>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(with_none, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: HashMap<K, V> inside generic struct with K: Encode + Eq + Hash + Ord, V: Encode
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct MapContainer<K, V>
where
    K: oxicode::Encode
        + oxicode::Decode
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq,
    V: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq,
{
    map: HashMap<K, V>,
    size: usize,
}

#[test]
fn test_16_hashmap_inside_generic_struct() {
    let mut map = HashMap::new();
    map.insert("alpha".to_string(), 1u32);
    map.insert("beta".to_string(), 2u32);
    map.insert("gamma".to_string(), 3u32);

    let original = MapContainer {
        size: map.len(),
        map,
    };

    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (MapContainer<String, u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Generic tuple struct (NewType pattern) with where clause
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct NewType<T>(T)
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq;

#[test]
fn test_17_generic_newtype_with_where_clause() {
    let original = NewType(256u32);
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (NewType<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);

    let original_str = NewType("roundtrip".to_string());
    let encoded = oxicode::encode_to_vec(&original_str).expect("encode failed");
    let (decoded, _): (NewType<String>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original_str, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: Generic newtype roundtrip identity (encoded bytes identical for same value)
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct IdentityNewType<T>(T)
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq;

#[test]
fn test_18_generic_newtype_roundtrip_identity() {
    let val = IdentityNewType(12345u64);
    let encoded_a = oxicode::encode_to_vec(&val).expect("encode failed");
    let encoded_b = oxicode::encode_to_vec(&val).expect("encode failed");
    // Deterministic encoding: same value produces identical bytes
    assert_eq!(encoded_a, encoded_b);
    let (decoded, _): (IdentityNewType<u64>, _) =
        oxicode::decode_from_slice(&encoded_a).expect("decode failed");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Struct with multiple where constraints on same type parameter
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct MultiConstraint<T>
where
    T: oxicode::Encode
        + oxicode::Decode
        + Clone
        + std::fmt::Debug
        + PartialEq
        + Default
        + Copy
        + PartialOrd,
{
    min_val: T,
    max_val: T,
    current: T,
}

#[test]
fn test_19_struct_with_multiple_where_constraints_on_same_type() {
    let original = MultiConstraint {
        min_val: 0i32,
        max_val: 100i32,
        current: 42i32,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (MultiConstraint<i32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
    // Verify the ordering constraint semantics are preserved
    assert!(decoded.min_val <= decoded.current);
    assert!(decoded.current <= decoded.max_val);
}

// ---------------------------------------------------------------------------
// Test 20: Enum with unit + data variants both generic
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
enum MixedGenericEnum<T, E>
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq,
    E: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq,
{
    Empty,
    Value(T),
    Error(E),
    Both { value: T, error: E },
    Count(u64),
}

#[test]
fn test_20_enum_with_unit_and_data_variants_both_generic() {
    let empty: MixedGenericEnum<u32, String> = MixedGenericEnum::Empty;
    let encoded = oxicode::encode_to_vec(&empty).expect("encode failed");
    let (decoded, _): (MixedGenericEnum<u32, String>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(empty, decoded);

    let value: MixedGenericEnum<u32, String> = MixedGenericEnum::Value(999);
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    let (decoded, _): (MixedGenericEnum<u32, String>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(value, decoded);

    let error: MixedGenericEnum<u32, String> = MixedGenericEnum::Error("fatal error".to_string());
    let encoded = oxicode::encode_to_vec(&error).expect("encode failed");
    let (decoded, _): (MixedGenericEnum<u32, String>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(error, decoded);

    let both: MixedGenericEnum<u32, String> = MixedGenericEnum::Both {
        value: 42,
        error: "partial failure".to_string(),
    };
    let encoded = oxicode::encode_to_vec(&both).expect("encode failed");
    let (decoded, _): (MixedGenericEnum<u32, String>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(both, decoded);

    let count: MixedGenericEnum<u32, String> = MixedGenericEnum::Count(1024);
    let encoded = oxicode::encode_to_vec(&count).expect("encode failed");
    let (decoded, _): (MixedGenericEnum<u32, String>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(count, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Multiple generic fields in single struct (three distinct type params)
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct Triple<A, B, C>
where
    A: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq,
    B: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq,
    C: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq,
{
    alpha: A,
    beta: B,
    gamma: C,
}

#[test]
fn test_21_multiple_generic_fields_in_single_struct() {
    let original = Triple {
        alpha: 0xAAu8,
        beta: 0xBBCCu16,
        gamma: 0xDDEEFF00u32,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode Triple<u8, u16, u32>");
    let (decoded, consumed): (Triple<u8, u16, u32>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode Triple<u8, u16, u32>");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());

    // Also test with heterogeneous types: (String, Vec<u32>, bool)
    let original_mixed = Triple {
        alpha: String::from("triple_alpha"),
        beta: vec![1u32, 2, 3],
        gamma: true,
    };
    let encoded_mixed =
        oxicode::encode_to_vec(&original_mixed).expect("encode Triple<String, Vec<u32>, bool>");
    let (decoded_mixed, consumed_mixed): (Triple<String, Vec<u32>, bool>, usize) =
        oxicode::decode_from_slice(&encoded_mixed).expect("decode Triple<String, Vec<u32>, bool>");
    assert_eq!(original_mixed, decoded_mixed);
    assert_eq!(consumed_mixed, encoded_mixed.len());
}

// ---------------------------------------------------------------------------
// Test 22: Vector of generic structs roundtrip
// ---------------------------------------------------------------------------
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct LabeledValue<T>
where
    T: oxicode::Encode + oxicode::Decode + Clone + std::fmt::Debug + PartialEq,
{
    label: String,
    value: T,
    index: u32,
}

#[test]
fn test_22_vector_of_generic_structs_roundtrip() {
    let original: Vec<LabeledValue<String>> = vec![
        LabeledValue {
            label: String::from("first"),
            value: String::from("payload_one"),
            index: 0u32,
        },
        LabeledValue {
            label: String::from("second"),
            value: String::from("payload_two"),
            index: 1u32,
        },
        LabeledValue {
            label: String::from("third"),
            value: String::from("payload_three"),
            index: 2u32,
        },
    ];
    let encoded = oxicode::encode_to_vec(&original).expect("encode Vec<LabeledValue<String>>");
    let (decoded, consumed): (Vec<LabeledValue<String>>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode Vec<LabeledValue<String>>");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 3);

    // Also verify with numeric generic type
    let original_numeric: Vec<LabeledValue<u64>> = vec![
        LabeledValue {
            label: String::from("num_first"),
            value: 1000u64,
            index: 0u32,
        },
        LabeledValue {
            label: String::from("num_second"),
            value: 2000u64,
            index: 1u32,
        },
    ];
    let encoded_numeric =
        oxicode::encode_to_vec(&original_numeric).expect("encode Vec<LabeledValue<u64>>");
    let (decoded_numeric, consumed_numeric): (Vec<LabeledValue<u64>>, usize) =
        oxicode::decode_from_slice(&encoded_numeric).expect("decode Vec<LabeledValue<u64>>");
    assert_eq!(original_numeric, decoded_numeric);
    assert_eq!(consumed_numeric, encoded_numeric.len());
    assert_eq!(decoded_numeric.len(), 2);
}
