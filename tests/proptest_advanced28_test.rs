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
struct GeoPoint {
    lat: i32,    // latitude * 1e6
    lon: i32,    // longitude * 1e6
    alt_cm: i32, // altitude in cm
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeoRoute {
    waypoints: Vec<GeoPoint>,
    direction: Cardinal,
    distance_m: u64,
}

proptest! {
    #[test]
    fn test_geopoint_roundtrip(lat in i32::MIN..=i32::MAX, lon in i32::MIN..=i32::MAX, alt_cm in i32::MIN..=i32::MAX) {
        let val = GeoPoint { lat, lon, alt_cm };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<GeoPoint>(&bytes).expect("decode GeoPoint failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_cardinal_roundtrip(variant in 0u8..4u8) {
        let val = match variant {
            0 => Cardinal::North,
            1 => Cardinal::South,
            2 => Cardinal::East,
            _ => Cardinal::West,
        };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<Cardinal>(&bytes).expect("decode Cardinal failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_georoute_roundtrip(
        lats in prop::collection::vec(i32::MIN..=i32::MAX, 0..8),
        lons in prop::collection::vec(i32::MIN..=i32::MAX, 0..8),
        alts in prop::collection::vec(i32::MIN..=i32::MAX, 0..8),
        dir_idx in 0u8..4u8,
        distance_m in u64::MIN..=u64::MAX,
    ) {
        let len = lats.len().min(lons.len()).min(alts.len());
        let waypoints: Vec<GeoPoint> = (0..len)
            .map(|i| GeoPoint { lat: lats[i], lon: lons[i], alt_cm: alts[i] })
            .collect();
        let direction = match dir_idx {
            0 => Cardinal::North,
            1 => Cardinal::South,
            2 => Cardinal::East,
            _ => Cardinal::West,
        };
        let val = GeoRoute { waypoints, direction, distance_m };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<GeoRoute>(&bytes).expect("decode GeoRoute failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_vec_geopoint_roundtrip(
        points in prop::collection::vec(
            (i32::MIN..=i32::MAX, i32::MIN..=i32::MAX, i32::MIN..=i32::MAX),
            0..10
        )
    ) {
        let val: Vec<GeoPoint> = points
            .into_iter()
            .map(|(lat, lon, alt_cm)| GeoPoint { lat, lon, alt_cm })
            .collect();
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<Vec<GeoPoint>>(&bytes).expect("decode Vec<GeoPoint> failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_consumed_bytes_equals_encoded_length_geopoint(lat in i32::MIN..=i32::MAX, lon in i32::MIN..=i32::MAX, alt_cm in i32::MIN..=i32::MAX) {
        let val = GeoPoint { lat, lon, alt_cm };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (_, consumed) = decode_from_slice::<GeoPoint>(&bytes).expect("decode GeoPoint failed");
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_consumed_bytes_equals_encoded_length_georoute(
        lats in prop::collection::vec(i32::MIN..=i32::MAX, 0..5),
        lons in prop::collection::vec(i32::MIN..=i32::MAX, 0..5),
        alts in prop::collection::vec(i32::MIN..=i32::MAX, 0..5),
        distance_m in u64::MIN..=u64::MAX,
    ) {
        let len = lats.len().min(lons.len()).min(alts.len());
        let waypoints: Vec<GeoPoint> = (0..len)
            .map(|i| GeoPoint { lat: lats[i], lon: lons[i], alt_cm: alts[i] })
            .collect();
        let val = GeoRoute { waypoints, direction: Cardinal::North, distance_m };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (_, consumed) = decode_from_slice::<GeoRoute>(&bytes).expect("decode GeoRoute failed");
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_encode_deterministic_geopoint(lat in i32::MIN..=i32::MAX, lon in i32::MIN..=i32::MAX, alt_cm in i32::MIN..=i32::MAX) {
        let val = GeoPoint { lat, lon, alt_cm };
        let bytes1 = encode_to_vec(&val);
        let bytes2 = encode_to_vec(&val);
        prop_assert_eq!(bytes1, bytes2);
    }

    #[test]
    fn test_encode_deterministic_georoute(
        lats in prop::collection::vec(i32::MIN..=i32::MAX, 0..4),
        lons in prop::collection::vec(i32::MIN..=i32::MAX, 0..4),
        alts in prop::collection::vec(i32::MIN..=i32::MAX, 0..4),
        distance_m in u64::MIN..=u64::MAX,
    ) {
        let len = lats.len().min(lons.len()).min(alts.len());
        let waypoints: Vec<GeoPoint> = (0..len)
            .map(|i| GeoPoint { lat: lats[i], lon: lons[i], alt_cm: alts[i] })
            .collect();
        let val = GeoRoute { waypoints, direction: Cardinal::East, distance_m };
        let bytes1 = encode_to_vec(&val);
        let bytes2 = encode_to_vec(&val);
        prop_assert_eq!(bytes1, bytes2);
    }

    #[test]
    fn test_u8_roundtrip(v in u8::MIN..=u8::MAX) {
        let bytes = encode_to_vec(&v).expect("encode failed");
        let (decoded, consumed) = decode_from_slice::<u8>(&bytes).expect("decode u8 failed");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_i32_roundtrip(v in i32::MIN..=i32::MAX) {
        let bytes = encode_to_vec(&v).expect("encode failed");
        let (decoded, consumed) = decode_from_slice::<i32>(&bytes).expect("decode i32 failed");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_u64_roundtrip(v in u64::MIN..=u64::MAX) {
        let bytes = encode_to_vec(&v).expect("encode failed");
        let (decoded, consumed) = decode_from_slice::<u64>(&bytes).expect("decode u64 failed");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_i64_roundtrip(v in i64::MIN..=i64::MAX) {
        let bytes = encode_to_vec(&v).expect("encode failed");
        let (decoded, consumed) = decode_from_slice::<i64>(&bytes).expect("decode i64 failed");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_bool_roundtrip(v in proptest::bool::ANY) {
        let bytes = encode_to_vec(&v).expect("encode failed");
        let (decoded, consumed) = decode_from_slice::<bool>(&bytes).expect("decode bool failed");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_string_roundtrip(v in ".*") {
        let bytes = encode_to_vec(&v).expect("encode failed");
        let (decoded, consumed) = decode_from_slice::<String>(&bytes).expect("decode String failed");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_f32_roundtrip(v in proptest::num::f32::ANY) {
        let bytes = encode_to_vec(&v).expect("encode failed");
        let (decoded, consumed) = decode_from_slice::<f32>(&bytes).expect("decode f32 failed");
        prop_assert_eq!(v.to_bits(), decoded.to_bits());
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_f64_roundtrip(v in proptest::num::f64::ANY) {
        let bytes = encode_to_vec(&v).expect("encode failed");
        let (decoded, consumed) = decode_from_slice::<f64>(&bytes).expect("decode f64 failed");
        prop_assert_eq!(v.to_bits(), decoded.to_bits());
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_vec_u8_roundtrip(v in prop::collection::vec(u8::MIN..=u8::MAX, 0..64)) {
        let bytes = encode_to_vec(&v).expect("encode failed");
        let (decoded, consumed) = decode_from_slice::<Vec<u8>>(&bytes).expect("decode Vec<u8> failed");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_vec_string_roundtrip(v in prop::collection::vec(".*", 0..8)) {
        let bytes = encode_to_vec(&v).expect("encode failed");
        let (decoded, consumed) = decode_from_slice::<Vec<String>>(&bytes).expect("decode Vec<String> failed");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_option_u64_roundtrip(v in proptest::option::of(u64::MIN..=u64::MAX)) {
        let bytes = encode_to_vec(&v).expect("encode failed");
        let (decoded, consumed) = decode_from_slice::<Option<u64>>(&bytes).expect("decode Option<u64> failed");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_option_geopoint_roundtrip(
        has_val in proptest::bool::ANY,
        lat in i32::MIN..=i32::MAX,
        lon in i32::MIN..=i32::MAX,
        alt_cm in i32::MIN..=i32::MAX,
    ) {
        let val: Option<GeoPoint> = if has_val {
            Some(GeoPoint { lat, lon, alt_cm })
        } else {
            None
        };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, consumed) = decode_from_slice::<Option<GeoPoint>>(&bytes).expect("decode Option<GeoPoint> failed");
        prop_assert_eq!(val, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_cardinal_encode_deterministic(variant in 0u8..4u8) {
        let val = match variant {
            0 => Cardinal::North,
            1 => Cardinal::South,
            2 => Cardinal::East,
            _ => Cardinal::West,
        };
        let bytes1 = encode_to_vec(&val);
        let bytes2 = encode_to_vec(&val);
        prop_assert_eq!(bytes1, bytes2);
    }

    #[test]
    fn test_geopoint_distinct_values_distinct_or_equal_bytes(
        lat1 in i32::MIN..=i32::MAX,
        lon1 in i32::MIN..=i32::MAX,
        alt1 in i32::MIN..=i32::MAX,
        lat2 in i32::MIN..=i32::MAX,
        lon2 in i32::MIN..=i32::MAX,
        alt2 in i32::MIN..=i32::MAX,
    ) {
        let a = GeoPoint { lat: lat1, lon: lon1, alt_cm: alt1 };
        let b = GeoPoint { lat: lat2, lon: lon2, alt_cm: alt2 };
        let bytes_a = encode_to_vec(&a);
        let bytes_b = encode_to_vec(&b);
        if a == b {
            prop_assert_eq!(&bytes_a, &bytes_b);
        } else {
            prop_assert_ne!(&bytes_a, &bytes_b);
        }
    }
}
