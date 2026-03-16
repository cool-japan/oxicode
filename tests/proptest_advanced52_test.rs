//! Proptest-based property tests for aerospace / flight simulation data serialization.

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AircraftType {
    Commercial,
    Military,
    PrivateJet,
    Helicopter,
    Cargo,
    Drone,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FlightPhase {
    Preflight,
    Takeoff,
    Climbing,
    Cruising,
    Descending,
    Landing,
    Parked,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlightData {
    flight_id: u64,
    altitude_ft: f32,
    speed_knots: f32,
    heading_deg: f32,
    vertical_speed_fpm: f32,
    phase: FlightPhase,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AircraftStatus {
    tail_number: String,
    aircraft_type: AircraftType,
    fuel_lbs: f32,
    data_points: Vec<FlightData>,
}

// ---------------------------------------------------------------------------
// Proptest strategies
// ---------------------------------------------------------------------------

fn arb_aircraft_type() -> impl Strategy<Value = AircraftType> {
    prop_oneof![
        Just(AircraftType::Commercial),
        Just(AircraftType::Military),
        Just(AircraftType::PrivateJet),
        Just(AircraftType::Helicopter),
        Just(AircraftType::Cargo),
        Just(AircraftType::Drone),
    ]
}

fn arb_flight_phase() -> impl Strategy<Value = FlightPhase> {
    prop_oneof![
        Just(FlightPhase::Preflight),
        Just(FlightPhase::Takeoff),
        Just(FlightPhase::Climbing),
        Just(FlightPhase::Cruising),
        Just(FlightPhase::Descending),
        Just(FlightPhase::Landing),
        Just(FlightPhase::Parked),
    ]
}

fn arb_flight_data() -> impl Strategy<Value = FlightData> {
    (
        any::<u64>(),
        -1000.0f32..=60000.0f32,
        0.0f32..=700.0f32,
        0.0f32..=360.0f32,
        -6000.0f32..=6000.0f32,
        arb_flight_phase(),
    )
        .prop_map(
            |(flight_id, altitude_ft, speed_knots, heading_deg, vertical_speed_fpm, phase)| {
                FlightData {
                    flight_id,
                    altitude_ft,
                    speed_knots,
                    heading_deg,
                    vertical_speed_fpm,
                    phase,
                }
            },
        )
}

fn arb_tail_number() -> impl Strategy<Value = String> {
    "[A-Z]{1,2}[0-9]{3,5}".prop_map(|s| s)
}

fn arb_aircraft_status() -> impl Strategy<Value = AircraftStatus> {
    (
        arb_tail_number(),
        arb_aircraft_type(),
        0.0f32..=50000.0f32,
        prop::collection::vec(arb_flight_data(), 0..=8),
    )
        .prop_map(
            |(tail_number, aircraft_type, fuel_lbs, data_points)| AircraftStatus {
                tail_number,
                aircraft_type,
                fuel_lbs,
                data_points,
            },
        )
}

// ---------------------------------------------------------------------------
// Tests (22 #[test] functions inside proptest! blocks)
// ---------------------------------------------------------------------------

proptest! {
    // 1. AircraftType roundtrip
    #[test]
    fn test_aircraft_type_roundtrip(val in arb_aircraft_type()) {
        let bytes = encode_to_vec(&val).expect("encode AircraftType");
        let (decoded, _): (AircraftType, usize) =
            decode_from_slice(&bytes).expect("decode AircraftType");
        prop_assert_eq!(val, decoded);
    }

    // 2. FlightPhase roundtrip
    #[test]
    fn test_flight_phase_roundtrip(val in arb_flight_phase()) {
        let bytes = encode_to_vec(&val).expect("encode FlightPhase");
        let (decoded, _): (FlightPhase, usize) =
            decode_from_slice(&bytes).expect("decode FlightPhase");
        prop_assert_eq!(val, decoded);
    }

    // 3. FlightData roundtrip
    #[test]
    fn test_flight_data_roundtrip(val in arb_flight_data()) {
        let bytes = encode_to_vec(&val).expect("encode FlightData");
        let (decoded, _): (FlightData, usize) =
            decode_from_slice(&bytes).expect("decode FlightData");
        prop_assert_eq!(val, decoded);
    }

    // 4. AircraftStatus roundtrip
    #[test]
    fn test_aircraft_status_roundtrip(val in arb_aircraft_status()) {
        let bytes = encode_to_vec(&val).expect("encode AircraftStatus");
        let (decoded, _): (AircraftStatus, usize) =
            decode_from_slice(&bytes).expect("decode AircraftStatus");
        prop_assert_eq!(val, decoded);
    }

    // 5. consumed == bytes.len() for AircraftType
    #[test]
    fn test_aircraft_type_consumed_equals_len(val in arb_aircraft_type()) {
        let bytes = encode_to_vec(&val).expect("encode AircraftType");
        let (_, consumed): (AircraftType, usize) =
            decode_from_slice(&bytes).expect("decode AircraftType");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 6. consumed == bytes.len() for FlightData
    #[test]
    fn test_flight_data_consumed_equals_len(val in arb_flight_data()) {
        let bytes = encode_to_vec(&val).expect("encode FlightData");
        let (_, consumed): (FlightData, usize) =
            decode_from_slice(&bytes).expect("decode FlightData");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 7. consumed == bytes.len() for AircraftStatus
    #[test]
    fn test_aircraft_status_consumed_equals_len(val in arb_aircraft_status()) {
        let bytes = encode_to_vec(&val).expect("encode AircraftStatus");
        let (_, consumed): (AircraftStatus, usize) =
            decode_from_slice(&bytes).expect("decode AircraftStatus");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 8. Deterministic encoding for FlightData (same input -> same bytes)
    #[test]
    fn test_flight_data_deterministic(val in arb_flight_data()) {
        let bytes1 = encode_to_vec(&val).expect("encode FlightData first");
        let bytes2 = encode_to_vec(&val).expect("encode FlightData second");
        prop_assert_eq!(bytes1, bytes2);
    }

    // 9. Deterministic encoding for AircraftStatus
    #[test]
    fn test_aircraft_status_deterministic(val in arb_aircraft_status()) {
        let bytes1 = encode_to_vec(&val).expect("encode AircraftStatus first");
        let bytes2 = encode_to_vec(&val).expect("encode AircraftStatus second");
        prop_assert_eq!(bytes1, bytes2);
    }

    // 10. Vec<FlightData> roundtrip
    #[test]
    fn test_vec_flight_data_roundtrip(
        items in prop::collection::vec(arb_flight_data(), 0..=16)
    ) {
        let bytes = encode_to_vec(&items).expect("encode Vec<FlightData>");
        let (decoded, _): (Vec<FlightData>, usize) =
            decode_from_slice(&bytes).expect("decode Vec<FlightData>");
        prop_assert_eq!(items, decoded);
    }

    // 11. Option<FlightData> roundtrip — Some variant
    #[test]
    fn test_option_flight_data_some_roundtrip(val in arb_flight_data()) {
        let opt: Option<FlightData> = Some(val);
        let bytes = encode_to_vec(&opt).expect("encode Option<FlightData> Some");
        let (decoded, _): (Option<FlightData>, usize) =
            decode_from_slice(&bytes).expect("decode Option<FlightData> Some");
        prop_assert_eq!(opt, decoded);
    }

    // 12. Option<AircraftType> roundtrip — None variant
    #[test]
    fn test_option_aircraft_type_none_roundtrip(_dummy in 0u8..=255u8) {
        let opt: Option<AircraftType> = None;
        let bytes = encode_to_vec(&opt).expect("encode Option<AircraftType> None");
        let (decoded, _): (Option<AircraftType>, usize) =
            decode_from_slice(&bytes).expect("decode Option<AircraftType> None");
        prop_assert_eq!(opt, decoded);
    }

    // 13. Re-encode idempotency for FlightData
    #[test]
    fn test_flight_data_reencode_idempotency(val in arb_flight_data()) {
        let bytes1 = encode_to_vec(&val).expect("encode FlightData original");
        let (decoded, _): (FlightData, usize) =
            decode_from_slice(&bytes1).expect("decode FlightData");
        let bytes2 = encode_to_vec(&decoded).expect("re-encode FlightData");
        prop_assert_eq!(bytes1, bytes2);
    }

    // 14. Re-encode idempotency for AircraftStatus
    #[test]
    fn test_aircraft_status_reencode_idempotency(val in arb_aircraft_status()) {
        let bytes1 = encode_to_vec(&val).expect("encode AircraftStatus original");
        let (decoded, _): (AircraftStatus, usize) =
            decode_from_slice(&bytes1).expect("decode AircraftStatus");
        let bytes2 = encode_to_vec(&decoded).expect("re-encode AircraftStatus");
        prop_assert_eq!(bytes1, bytes2);
    }

    // 15. All AircraftType enum variants are individually encodable
    #[test]
    fn test_all_aircraft_type_variants_roundtrip(_dummy in 0u8..=255u8) {
        let variants = [
            AircraftType::Commercial,
            AircraftType::Military,
            AircraftType::PrivateJet,
            AircraftType::Helicopter,
            AircraftType::Cargo,
            AircraftType::Drone,
        ];
        for variant in &variants {
            let bytes = encode_to_vec(variant).expect("encode AircraftType variant");
            let (decoded, consumed): (AircraftType, usize) =
                decode_from_slice(&bytes).expect("decode AircraftType variant");
            prop_assert_eq!(variant, &decoded);
            prop_assert_eq!(consumed, bytes.len());
        }
    }

    // 16. All FlightPhase enum variants are individually encodable
    #[test]
    fn test_all_flight_phase_variants_roundtrip(_dummy in 0u8..=255u8) {
        let variants = [
            FlightPhase::Preflight,
            FlightPhase::Takeoff,
            FlightPhase::Climbing,
            FlightPhase::Cruising,
            FlightPhase::Descending,
            FlightPhase::Landing,
            FlightPhase::Parked,
        ];
        for variant in &variants {
            let bytes = encode_to_vec(variant).expect("encode FlightPhase variant");
            let (decoded, consumed): (FlightPhase, usize) =
                decode_from_slice(&bytes).expect("decode FlightPhase variant");
            prop_assert_eq!(variant, &decoded);
            prop_assert_eq!(consumed, bytes.len());
        }
    }

    // 17. FlightData with extreme altitude values roundtrip
    #[test]
    fn test_flight_data_extreme_altitude(
        flight_id in any::<u64>(),
        altitude_ft in prop_oneof![
            Just(f32::MIN_POSITIVE),
            Just(0.0f32),
            Just(60000.0f32),
            -1000.0f32..=-0.001f32,
        ],
        phase in arb_flight_phase(),
    ) {
        let val = FlightData {
            flight_id,
            altitude_ft,
            speed_knots: 250.0,
            heading_deg: 90.0,
            vertical_speed_fpm: 0.0,
            phase,
        };
        let bytes = encode_to_vec(&val).expect("encode extreme-altitude FlightData");
        let (decoded, consumed): (FlightData, usize) =
            decode_from_slice(&bytes).expect("decode extreme-altitude FlightData");
        prop_assert_eq!(&val, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 18. AircraftStatus with empty data_points
    #[test]
    fn test_aircraft_status_empty_data_points(
        tail_number in arb_tail_number(),
        aircraft_type in arb_aircraft_type(),
        fuel_lbs in 0.0f32..=50000.0f32,
    ) {
        let val = AircraftStatus {
            tail_number,
            aircraft_type,
            fuel_lbs,
            data_points: vec![],
        };
        let bytes = encode_to_vec(&val).expect("encode AircraftStatus empty data_points");
        let (decoded, consumed): (AircraftStatus, usize) =
            decode_from_slice(&bytes).expect("decode AircraftStatus empty data_points");
        prop_assert_eq!(&val, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 19. AircraftStatus with a single data point
    #[test]
    fn test_aircraft_status_single_data_point(
        status in arb_aircraft_status(),
        extra_point in arb_flight_data(),
    ) {
        let mut val = status;
        val.data_points = vec![extra_point];
        let bytes = encode_to_vec(&val).expect("encode AircraftStatus single data point");
        let (decoded, consumed): (AircraftStatus, usize) =
            decode_from_slice(&bytes).expect("decode AircraftStatus single data point");
        prop_assert_eq!(&val, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 20. Two distinct values produce distinct encodings (FlightData — differ by flight_id)
    #[test]
    fn test_flight_data_distinct_ids_distinct_encodings(
        id_a in 0u64..=u64::MAX / 2,
        id_b in (u64::MAX / 2 + 1)..=u64::MAX,
        phase in arb_flight_phase(),
    ) {
        let make = |flight_id| FlightData {
            flight_id,
            altitude_ft: 10000.0,
            speed_knots: 300.0,
            heading_deg: 180.0,
            vertical_speed_fpm: 500.0,
            phase: phase.clone(),
        };
        let bytes_a = encode_to_vec(&make(id_a)).expect("encode FlightData a");
        let bytes_b = encode_to_vec(&make(id_b)).expect("encode FlightData b");
        prop_assert_ne!(bytes_a, bytes_b);
    }

    // 21. Option<AircraftStatus> Some/None size relationship
    #[test]
    fn test_option_aircraft_status_size(val in arb_aircraft_status()) {
        let some_opt: Option<AircraftStatus> = Some(val.clone());
        let none_opt: Option<AircraftStatus> = None;
        let some_bytes = encode_to_vec(&some_opt).expect("encode Option Some AircraftStatus");
        let none_bytes = encode_to_vec(&none_opt).expect("encode Option None AircraftStatus");
        // Some encoding must be strictly larger than None encoding
        prop_assert!(some_bytes.len() > none_bytes.len());
        let (decoded_some, c_some): (Option<AircraftStatus>, usize) =
            decode_from_slice(&some_bytes).expect("decode Option Some AircraftStatus");
        let (decoded_none, c_none): (Option<AircraftStatus>, usize) =
            decode_from_slice(&none_bytes).expect("decode Option None AircraftStatus");
        prop_assert_eq!(some_opt, decoded_some);
        prop_assert_eq!(none_opt, decoded_none);
        prop_assert_eq!(c_some, some_bytes.len());
        prop_assert_eq!(c_none, none_bytes.len());
    }

    // 22. Large Vec<AircraftStatus> roundtrip with consumed == bytes.len()
    #[test]
    fn test_large_vec_aircraft_status_roundtrip(
        items in prop::collection::vec(arb_aircraft_status(), 0..=6)
    ) {
        let bytes = encode_to_vec(&items).expect("encode Vec<AircraftStatus>");
        let (decoded, consumed): (Vec<AircraftStatus>, usize) =
            decode_from_slice(&bytes).expect("decode Vec<AircraftStatus>");
        prop_assert_eq!(&items, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}
