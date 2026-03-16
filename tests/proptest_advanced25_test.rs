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
use proptest::prelude::*;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Axis {
    X,
    Y,
    Z,
    Custom(i32),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Transform {
    origin: Coordinate,
    scale: u32,
    axis: Axis,
}

proptest! {
    #[test]
    fn prop_u8_roundtrip(val in any::<u8>()) {
        let enc = encode_to_vec(&val).expect("encode u8");
        let (decoded, _): (u8, usize) = decode_from_slice(&enc).expect("decode u8");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_u16_roundtrip(val in any::<u16>()) {
        let enc = encode_to_vec(&val).expect("encode u16");
        let (decoded, _): (u16, usize) = decode_from_slice(&enc).expect("decode u16");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_u32_roundtrip(val in any::<u32>()) {
        let enc = encode_to_vec(&val).expect("encode u32");
        let (decoded, _): (u32, usize) = decode_from_slice(&enc).expect("decode u32");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_u64_roundtrip(val in any::<u64>()) {
        let enc = encode_to_vec(&val).expect("encode u64");
        let (decoded, _): (u64, usize) = decode_from_slice(&enc).expect("decode u64");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_i8_roundtrip(val in any::<i8>()) {
        let enc = encode_to_vec(&val).expect("encode i8");
        let (decoded, _): (i8, usize) = decode_from_slice(&enc).expect("decode i8");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_i16_roundtrip(val in any::<i16>()) {
        let enc = encode_to_vec(&val).expect("encode i16");
        let (decoded, _): (i16, usize) = decode_from_slice(&enc).expect("decode i16");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_i32_roundtrip(val in any::<i32>()) {
        let enc = encode_to_vec(&val).expect("encode i32");
        let (decoded, _): (i32, usize) = decode_from_slice(&enc).expect("decode i32");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_i64_roundtrip(val in any::<i64>()) {
        let enc = encode_to_vec(&val).expect("encode i64");
        let (decoded, _): (i64, usize) = decode_from_slice(&enc).expect("decode i64");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_bool_roundtrip(val in any::<bool>()) {
        let enc = encode_to_vec(&val).expect("encode bool");
        let (decoded, _): (bool, usize) = decode_from_slice(&enc).expect("decode bool");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_string_roundtrip(val in "[a-z]{0,20}") {
        let enc = encode_to_vec(&val).expect("encode String");
        let (decoded, _): (String, usize) = decode_from_slice(&enc).expect("decode String");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_vec_u8_roundtrip(val in proptest::collection::vec(any::<u8>(), 0..50)) {
        let enc = encode_to_vec(&val).expect("encode Vec<u8>");
        let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8>");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_vec_u32_roundtrip(val in proptest::collection::vec(any::<u32>(), 0..50)) {
        let enc = encode_to_vec(&val).expect("encode Vec<u32>");
        let (decoded, _): (Vec<u32>, usize) = decode_from_slice(&enc).expect("decode Vec<u32>");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_option_u32_roundtrip(val in proptest::option::of(any::<u32>())) {
        let enc = encode_to_vec(&val).expect("encode Option<u32>");
        let (decoded, _): (Option<u32>, usize) = decode_from_slice(&enc).expect("decode Option<u32>");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_tuple_u32_u32_roundtrip(a in any::<u32>(), b in any::<u32>()) {
        let val = (a, b);
        let enc = encode_to_vec(&val).expect("encode (u32, u32)");
        let (decoded, _): ((u32, u32), usize) = decode_from_slice(&enc).expect("decode (u32, u32)");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_coordinate_roundtrip(x in any::<i64>(), y in any::<i64>(), z in any::<i64>()) {
        let val = Coordinate { x, y, z };
        let enc = encode_to_vec(&val).expect("encode Coordinate");
        let (decoded, _): (Coordinate, usize) = decode_from_slice(&enc).expect("decode Coordinate");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_axis_roundtrip(variant in 0u32..4, custom_val in any::<i32>()) {
        let val = match variant {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => Axis::Custom(custom_val),
        };
        let enc = encode_to_vec(&val).expect("encode Axis");
        let (decoded, _): (Axis, usize) = decode_from_slice(&enc).expect("decode Axis");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_transform_roundtrip(
        x in any::<i64>(),
        y in any::<i64>(),
        z in any::<i64>(),
        scale in any::<u32>(),
        axis_variant in 0u32..4,
        custom_axis in any::<i32>(),
    ) {
        let axis = match axis_variant {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => Axis::Custom(custom_axis),
        };
        let val = Transform {
            origin: Coordinate { x, y, z },
            scale,
            axis,
        };
        let enc = encode_to_vec(&val).expect("encode Transform");
        let (decoded, _): (Transform, usize) = decode_from_slice(&enc).expect("decode Transform");
        prop_assert_eq!(val, decoded);
    }
}

proptest! {
    #[test]
    fn prop_consumed_bytes_equals_encoded_length_u64(val in any::<u64>()) {
        let enc = encode_to_vec(&val).expect("encode u64 for length check");
        let (_, consumed): (u64, usize) = decode_from_slice(&enc).expect("decode u64 for length check");
        prop_assert_eq!(enc.len(), consumed);
    }
}

proptest! {
    #[test]
    fn prop_consumed_bytes_equals_encoded_length_string(val in "[a-z]{0,20}") {
        let enc = encode_to_vec(&val).expect("encode String for length check");
        let (_, consumed): (String, usize) = decode_from_slice(&enc).expect("decode String for length check");
        prop_assert_eq!(enc.len(), consumed);
    }
}

proptest! {
    #[test]
    fn prop_double_encode_decode_roundtrip_u32(val in any::<u32>()) {
        let enc1 = encode_to_vec(&val).expect("encode u32 first time");
        let (decoded1, _): (u32, usize) = decode_from_slice(&enc1).expect("decode u32 first time");
        let enc2 = encode_to_vec(&decoded1).expect("encode u32 second time");
        prop_assert_eq!(enc1, enc2);
    }
}

proptest! {
    #[test]
    fn prop_vec_coordinate_roundtrip(
        items in proptest::collection::vec(
            (any::<i64>(), any::<i64>(), any::<i64>()).prop_map(|(x, y, z)| Coordinate { x, y, z }),
            0..5,
        )
    ) {
        let enc = encode_to_vec(&items).expect("encode Vec<Coordinate>");
        let (decoded, _): (Vec<Coordinate>, usize) = decode_from_slice(&enc).expect("decode Vec<Coordinate>");
        prop_assert_eq!(items, decoded);
    }
}

proptest! {
    #[test]
    fn prop_encode_twice_same_bytes_u32(val in any::<u32>()) {
        let enc1 = encode_to_vec(&val).expect("encode u32 first encode");
        let enc2 = encode_to_vec(&val).expect("encode u32 second encode");
        prop_assert_eq!(enc1, enc2);
    }
}
