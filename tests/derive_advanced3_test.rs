//! Advanced derive macro tests - set 3

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

// Unit struct
#[derive(Debug, PartialEq, Encode, Decode)]
struct UnitType;

// Newtype struct
#[derive(Debug, PartialEq, Encode, Decode)]
struct Newtype(u64);

// Tuple struct
#[derive(Debug, PartialEq, Encode, Decode)]
struct Point3D(f32, f32, f32);

// Struct with all field types
#[derive(Debug, PartialEq, Encode, Decode)]
struct AllFields {
    a: u8,
    b: i16,
    c: u32,
    d: i64,
    e: f32,
    f: f64,
    g: bool,
    h: String,
    i: Vec<u8>,
    j: Option<u32>,
}

// Enum with all variant styles
#[derive(Debug, PartialEq, Encode, Decode)]
enum AllVariants {
    Unit,
    Newtype(u32),
    Tuple(u8, u16, u32),
    Struct { x: i32, y: i32, z: i32 },
}

// Generic struct
#[derive(Debug, PartialEq, Encode, Decode)]
struct Wrapper<T> {
    inner: T,
    tag: String,
}

// ---- UnitType tests ----

#[test]
fn test_unit_type_roundtrip() {
    let val = UnitType;
    let bytes = encode_to_vec(&val).expect("encode UnitType failed");
    let (decoded, consumed): (UnitType, usize) =
        decode_from_slice(&bytes).expect("decode UnitType failed");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_unit_type_encoded_length() {
    let val = UnitType;
    let bytes = encode_to_vec(&val).expect("encode UnitType failed");
    // A unit struct has no fields — encoded bytes should be minimal (0 or very small)
    assert!(
        bytes.len() <= 8,
        "unit struct should encode to <=8 bytes, got {}",
        bytes.len()
    );
}

// ---- Newtype tests ----

#[test]
fn test_newtype_zero() {
    let val = Newtype(0u64);
    let bytes = encode_to_vec(&val).expect("encode Newtype(0) failed");
    let (decoded, _): (Newtype, usize) =
        decode_from_slice(&bytes).expect("decode Newtype(0) failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_newtype_max_value() {
    let val = Newtype(u64::MAX);
    let bytes = encode_to_vec(&val).expect("encode Newtype(MAX) failed");
    let (decoded, _): (Newtype, usize) =
        decode_from_slice(&bytes).expect("decode Newtype(MAX) failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_newtype_arbitrary_value() {
    let val = Newtype(0xDEAD_BEEF_CAFE_1234u64);
    let bytes = encode_to_vec(&val).expect("encode Newtype failed");
    let (decoded, consumed): (Newtype, usize) =
        decode_from_slice(&bytes).expect("decode Newtype failed");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---- Point3D tests ----

#[test]
fn test_point3d_origin() {
    let val = Point3D(0.0f32, 0.0f32, 0.0f32);
    let bytes = encode_to_vec(&val).expect("encode Point3D origin failed");
    let (decoded, _): (Point3D, usize) =
        decode_from_slice(&bytes).expect("decode Point3D origin failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_point3d_arbitrary() {
    let val = Point3D(1.5f32, -3.14f32, 42.0f32);
    let bytes = encode_to_vec(&val).expect("encode Point3D failed");
    let (decoded, consumed): (Point3D, usize) =
        decode_from_slice(&bytes).expect("decode Point3D failed");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_point3d_with_legacy_config() {
    let val = Point3D(
        std::f32::consts::PI,
        std::f32::consts::E,
        std::f32::consts::SQRT_2,
    );
    let cfg = config::legacy();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode Point3D legacy failed");
    let (decoded, _): (Point3D, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Point3D legacy failed");
    assert_eq!(val, decoded);
}

// ---- AllFields tests ----

#[test]
fn test_all_fields_roundtrip() {
    let val = AllFields {
        a: 255u8,
        b: -1000i16,
        c: 1_000_000u32,
        d: -9_000_000_000i64,
        e: 3.14f32,
        f: 2.718_281_828f64,
        g: true,
        h: String::from("oxicode"),
        i: vec![0u8, 1, 2, 3, 127, 255],
        j: Some(42u32),
    };
    let bytes = encode_to_vec(&val).expect("encode AllFields failed");
    let (decoded, consumed): (AllFields, usize) =
        decode_from_slice(&bytes).expect("decode AllFields failed");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_all_fields_option_none() {
    let val = AllFields {
        a: 0u8,
        b: 0i16,
        c: 0u32,
        d: 0i64,
        e: 0.0f32,
        f: 0.0f64,
        g: false,
        h: String::new(),
        i: Vec::new(),
        j: None,
    };
    let bytes = encode_to_vec(&val).expect("encode AllFields(None) failed");
    let (decoded, _): (AllFields, usize) =
        decode_from_slice(&bytes).expect("decode AllFields(None) failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_all_fields_large_vec() {
    let large_vec: Vec<u8> = (0u8..=255u8).collect();
    let val = AllFields {
        a: 1u8,
        b: 2i16,
        c: 3u32,
        d: 4i64,
        e: 5.0f32,
        f: 6.0f64,
        g: true,
        h: String::from("large"),
        i: large_vec.clone(),
        j: Some(999u32),
    };
    let bytes = encode_to_vec(&val).expect("encode AllFields large vec failed");
    let (decoded, consumed): (AllFields, usize) =
        decode_from_slice(&bytes).expect("decode AllFields large vec failed");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---- AllVariants tests ----

#[test]
fn test_all_variants_unit() {
    let val = AllVariants::Unit;
    let bytes = encode_to_vec(&val).expect("encode AllVariants::Unit failed");
    let (decoded, consumed): (AllVariants, usize) =
        decode_from_slice(&bytes).expect("decode AllVariants::Unit failed");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_all_variants_newtype() {
    let val = AllVariants::Newtype(0xFFFF_FFFFu32);
    let bytes = encode_to_vec(&val).expect("encode AllVariants::Newtype failed");
    let (decoded, _): (AllVariants, usize) =
        decode_from_slice(&bytes).expect("decode AllVariants::Newtype failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_all_variants_tuple() {
    let val = AllVariants::Tuple(0u8, 1000u16, 1_000_000u32);
    let bytes = encode_to_vec(&val).expect("encode AllVariants::Tuple failed");
    let (decoded, consumed): (AllVariants, usize) =
        decode_from_slice(&bytes).expect("decode AllVariants::Tuple failed");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_all_variants_struct() {
    let val = AllVariants::Struct {
        x: -100i32,
        y: 0i32,
        z: 100i32,
    };
    let bytes = encode_to_vec(&val).expect("encode AllVariants::Struct failed");
    let (decoded, _): (AllVariants, usize) =
        decode_from_slice(&bytes).expect("decode AllVariants::Struct failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_all_variants_all_roundtrip_sequence() {
    let variants = [
        AllVariants::Unit,
        AllVariants::Newtype(42u32),
        AllVariants::Tuple(10u8, 200u16, 30_000u32),
        AllVariants::Struct {
            x: 1i32,
            y: 2i32,
            z: 3i32,
        },
    ];
    for val in &variants {
        let bytes = encode_to_vec(val).expect("encode AllVariants sequence failed");
        let (decoded, consumed): (AllVariants, usize) =
            decode_from_slice(&bytes).expect("decode AllVariants sequence failed");
        assert_eq!(val, &decoded);
        assert_eq!(consumed, bytes.len());
    }
}

// ---- Wrapper<T> tests ----

#[test]
fn test_wrapper_u32() {
    let val = Wrapper::<u32> {
        inner: 42u32,
        tag: String::from("number"),
    };
    let bytes = encode_to_vec(&val).expect("encode Wrapper<u32> failed");
    let (decoded, consumed): (Wrapper<u32>, usize) =
        decode_from_slice(&bytes).expect("decode Wrapper<u32> failed");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_wrapper_string() {
    let val = Wrapper::<String> {
        inner: String::from("hello, world!"),
        tag: String::from("greeting"),
    };
    let bytes = encode_to_vec(&val).expect("encode Wrapper<String> failed");
    let (decoded, _): (Wrapper<String>, usize) =
        decode_from_slice(&bytes).expect("decode Wrapper<String> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapper_vec_f64() {
    let val = Wrapper::<Vec<f64>> {
        inner: vec![1.0f64, 2.0f64, 3.0f64, std::f64::consts::PI],
        tag: String::from("floats"),
    };
    let bytes = encode_to_vec(&val).expect("encode Wrapper<Vec<f64>> failed");
    let (decoded, consumed): (Wrapper<Vec<f64>>, usize) =
        decode_from_slice(&bytes).expect("decode Wrapper<Vec<f64>> failed");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_wrapper_nested_struct() {
    let val = Wrapper::<Point3D> {
        inner: Point3D(1.0f32, 2.0f32, 3.0f32),
        tag: String::from("point"),
    };
    let bytes = encode_to_vec(&val).expect("encode Wrapper<Point3D> failed");
    let (decoded, _): (Wrapper<Point3D>, usize) =
        decode_from_slice(&bytes).expect("decode Wrapper<Point3D> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapper_empty_tag() {
    let val = Wrapper::<u64> {
        inner: 0u64,
        tag: String::new(),
    };
    let bytes = encode_to_vec(&val).expect("encode Wrapper empty tag failed");
    let (decoded, consumed): (Wrapper<u64>, usize) =
        decode_from_slice(&bytes).expect("decode Wrapper empty tag failed");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_wrapper_with_standard_config() {
    let val = Wrapper::<Newtype> {
        inner: Newtype(12345678901234u64),
        tag: String::from("newtype-wrapper"),
    };
    let cfg = config::standard();
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode Wrapper standard config failed");
    let (decoded, consumed): (Wrapper<Newtype>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Wrapper standard config failed");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}
