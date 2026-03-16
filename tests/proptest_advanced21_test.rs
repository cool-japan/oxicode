//! Advanced property-based roundtrip tests (set 21) using proptest.
//!
//! Tests verify encode → decode is a perfect roundtrip for various types,
//! including custom derived structs, enums, configs, and nested options.

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
use proptest::prelude::*;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Matrix2x2 {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
    Triangle(f32, f32, f32),
}

// 1. u8 roundtrip
proptest! {
    #[test]
    fn prop_u8_roundtrip(value: u8) {
        let encoded = encode_to_vec(&value).expect("encode u8 failed");
        let (decoded, consumed): (u8, usize) =
            decode_from_slice(&encoded).expect("decode u8 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 2. i8 roundtrip
proptest! {
    #[test]
    fn prop_i8_roundtrip(value: i8) {
        let encoded = encode_to_vec(&value).expect("encode i8 failed");
        let (decoded, consumed): (i8, usize) =
            decode_from_slice(&encoded).expect("decode i8 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 3. u16 roundtrip
proptest! {
    #[test]
    fn prop_u16_roundtrip(value: u16) {
        let encoded = encode_to_vec(&value).expect("encode u16 failed");
        let (decoded, consumed): (u16, usize) =
            decode_from_slice(&encoded).expect("decode u16 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 4. i16 roundtrip
proptest! {
    #[test]
    fn prop_i16_roundtrip(value: i16) {
        let encoded = encode_to_vec(&value).expect("encode i16 failed");
        let (decoded, consumed): (i16, usize) =
            decode_from_slice(&encoded).expect("decode i16 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 5. u32 roundtrip
proptest! {
    #[test]
    fn prop_u32_roundtrip(value: u32) {
        let encoded = encode_to_vec(&value).expect("encode u32 failed");
        let (decoded, consumed): (u32, usize) =
            decode_from_slice(&encoded).expect("decode u32 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 6. i32 roundtrip
proptest! {
    #[test]
    fn prop_i32_roundtrip(value: i32) {
        let encoded = encode_to_vec(&value).expect("encode i32 failed");
        let (decoded, consumed): (i32, usize) =
            decode_from_slice(&encoded).expect("decode i32 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 7. u64 roundtrip
proptest! {
    #[test]
    fn prop_u64_roundtrip(value: u64) {
        let encoded = encode_to_vec(&value).expect("encode u64 failed");
        let (decoded, consumed): (u64, usize) =
            decode_from_slice(&encoded).expect("decode u64 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 8. i64 roundtrip
proptest! {
    #[test]
    fn prop_i64_roundtrip(value: i64) {
        let encoded = encode_to_vec(&value).expect("encode i64 failed");
        let (decoded, consumed): (i64, usize) =
            decode_from_slice(&encoded).expect("decode i64 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 9. bool roundtrip
proptest! {
    #[test]
    fn prop_bool_roundtrip(value: bool) {
        let encoded = encode_to_vec(&value).expect("encode bool failed");
        let (decoded, consumed): (bool, usize) =
            decode_from_slice(&encoded).expect("decode bool failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 10. String roundtrip
proptest! {
    #[test]
    fn prop_string_roundtrip(value: String) {
        let encoded = encode_to_vec(&value).expect("encode String failed");
        let (decoded, consumed): (String, usize) =
            decode_from_slice(&encoded).expect("decode String failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 11. Vec<u8> roundtrip (length 0..100)
proptest! {
    #[test]
    fn prop_vec_u8_roundtrip(v in proptest::collection::vec(any::<u8>(), 0..100)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<u8> failed");
        let (decoded, consumed): (Vec<u8>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<u8> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 12. Vec<u32> roundtrip (length 0..50)
proptest! {
    #[test]
    fn prop_vec_u32_roundtrip(v in proptest::collection::vec(any::<u32>(), 0..50)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<u32> failed");
        let (decoded, consumed): (Vec<u32>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<u32> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 13. Option<u32> roundtrip
proptest! {
    #[test]
    fn prop_option_u32_roundtrip(opt in proptest::option::of(any::<u32>())) {
        let encoded = encode_to_vec(&opt).expect("encode Option<u32> failed");
        let (decoded, consumed): (Option<u32>, usize) =
            decode_from_slice(&encoded).expect("decode Option<u32> failed");
        prop_assert_eq!(decoded, opt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 14. (u32, String) tuple roundtrip
proptest! {
    #[test]
    fn prop_tuple_u32_string_roundtrip(value: (u32, String)) {
        let encoded = encode_to_vec(&value).expect("encode (u32, String) failed");
        let (decoded, consumed): ((u32, String), usize) =
            decode_from_slice(&encoded).expect("decode (u32, String) failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 15. Point3D roundtrip (finite f32 range)
proptest! {
    #[test]
    fn prop_point3d_roundtrip(
        x in -1e6f32..1e6f32,
        y in -1e6f32..1e6f32,
        z in -1e6f32..1e6f32,
    ) {
        let value = Point3D { x, y, z };
        let encoded = encode_to_vec(&value).expect("encode Point3D failed");
        let (decoded, consumed): (Point3D, usize) =
            decode_from_slice(&encoded).expect("decode Point3D failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 16. Matrix2x2 roundtrip (finite f64 range)
proptest! {
    #[test]
    fn prop_matrix2x2_roundtrip(
        a in -1e12f64..1e12f64,
        b in -1e12f64..1e12f64,
        c in -1e12f64..1e12f64,
        d in -1e12f64..1e12f64,
    ) {
        let value = Matrix2x2 { a, b, c, d };
        let encoded = encode_to_vec(&value).expect("encode Matrix2x2 failed");
        let (decoded, consumed): (Matrix2x2, usize) =
            decode_from_slice(&encoded).expect("decode Matrix2x2 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 17. Shape::Circle roundtrip
proptest! {
    #[test]
    fn prop_shape_circle_roundtrip(radius in 0.0f32..1000.0f32) {
        let value = Shape::Circle(radius);
        let encoded = encode_to_vec(&value).expect("encode Shape::Circle failed");
        let (decoded, consumed): (Shape, usize) =
            decode_from_slice(&encoded).expect("decode Shape::Circle failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 18. Shape::Rectangle roundtrip
proptest! {
    #[test]
    fn prop_shape_rectangle_roundtrip(
        w in 0.0f32..1000.0f32,
        h in 0.0f32..1000.0f32,
    ) {
        let value = Shape::Rectangle(w, h);
        let encoded = encode_to_vec(&value).expect("encode Shape::Rectangle failed");
        let (decoded, consumed): (Shape, usize) =
            decode_from_slice(&encoded).expect("decode Shape::Rectangle failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 19. consumed bytes equals encoded length for u64
proptest! {
    #[test]
    fn prop_consumed_bytes_equals_encoded_len_u64(value: u64) {
        let encoded = encode_to_vec(&value).expect("encode u64 for consumed check failed");
        let (_decoded, consumed): (u64, usize) =
            decode_from_slice(&encoded).expect("decode u64 for consumed check failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 20. u32 with fixed_int_encoding config roundtrip (always 4 bytes)
proptest! {
    #[test]
    fn prop_fixed_int_u32_roundtrip(value: u32) {
        let cfg = config::standard().with_fixed_int_encoding();
        let encoded =
            encode_to_vec_with_config(&value, cfg).expect("encode u32 fixed_int failed");
        let (decoded, consumed): (u32, usize) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode u32 fixed_int failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
        prop_assert_eq!(
            encoded.len(),
            4usize,
            "u32 with fixed_int_encoding must always be 4 bytes"
        );
    }
}

// 21. Vec<String> roundtrip (length 0..20)
proptest! {
    #[test]
    fn prop_vec_string_roundtrip(v in proptest::collection::vec(any::<String>(), 0..20)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<String> failed");
        let (decoded, consumed): (Vec<String>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<String> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 22. Option<Option<u32>> nested option roundtrip
proptest! {
    #[test]
    fn prop_nested_option_roundtrip(opt: Option<Option<u32>>) {
        let encoded = encode_to_vec(&opt).expect("encode Option<Option<u32>> failed");
        let (decoded, consumed): (Option<Option<u32>>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Option<u32>> failed");
        prop_assert_eq!(decoded, opt);
        prop_assert_eq!(consumed, encoded.len());
    }
}
