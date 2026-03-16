//! Advanced property-based tests (set 47) using proptest.
//!
//! Climate science / environmental monitoring domain.
//! 22 #[test] functions, each contained within its own proptest! { } block.
//! Covers roundtrip, consumed == bytes.len(), deterministic encoding,
//! all enum variants, vec of structs, and option types.

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

// ── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ClimateZone {
    Tropical,
    Subtropical,
    Temperate,
    Boreal,
    Polar,
    Arid,
    Mediterranean,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PollutantType {
    CO2,
    CH4,
    NO2,
    SO2,
    PM25,
    PM10,
    Ozone,
    Lead,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Station {
    station_id: u32,
    lat: f64,
    lon: f64,
    altitude_m: f32,
    zone: ClimateZone,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Measurement {
    station_id: u32,
    pollutant: PollutantType,
    value: f64,
    unit: String,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClimateReport {
    period_start: u64,
    period_end: u64,
    stations: Vec<Station>,
    measurements: Vec<Measurement>,
}

// ── Helper strategies ─────────────────────────────────────────────────────────

fn climate_zone_strategy() -> impl Strategy<Value = ClimateZone> {
    prop_oneof![
        Just(ClimateZone::Tropical),
        Just(ClimateZone::Subtropical),
        Just(ClimateZone::Temperate),
        Just(ClimateZone::Boreal),
        Just(ClimateZone::Polar),
        Just(ClimateZone::Arid),
        Just(ClimateZone::Mediterranean),
    ]
}

fn pollutant_type_strategy() -> impl Strategy<Value = PollutantType> {
    prop_oneof![
        Just(PollutantType::CO2),
        Just(PollutantType::CH4),
        Just(PollutantType::NO2),
        Just(PollutantType::SO2),
        Just(PollutantType::PM25),
        Just(PollutantType::PM10),
        Just(PollutantType::Ozone),
        Just(PollutantType::Lead),
    ]
}

fn station_strategy() -> impl Strategy<Value = Station> {
    (
        any::<u32>(),
        proptest::num::f64::NORMAL,
        proptest::num::f64::NORMAL,
        proptest::num::f32::NORMAL,
        climate_zone_strategy(),
    )
        .prop_map(|(station_id, lat, lon, altitude_m, zone)| Station {
            station_id,
            lat,
            lon,
            altitude_m,
            zone,
        })
}

fn measurement_strategy() -> impl Strategy<Value = Measurement> {
    (
        any::<u32>(),
        pollutant_type_strategy(),
        proptest::num::f64::NORMAL,
        "[a-zA-Z0-9/]{1,10}",
        any::<u64>(),
    )
        .prop_map(
            |(station_id, pollutant, value, unit, timestamp)| Measurement {
                station_id,
                pollutant,
                value,
                unit,
                timestamp,
            },
        )
}

// ── Test 1: Station roundtrip ─────────────────────────────────────────────────

proptest! {
    #[test]
    fn prop_station_roundtrip(
        station_id: u32,
        lat in proptest::num::f64::NORMAL,
        lon in proptest::num::f64::NORMAL,
        altitude_m in proptest::num::f32::NORMAL,
        zone_idx in 0usize..7,
    ) {
        let zones = [
            ClimateZone::Tropical,
            ClimateZone::Subtropical,
            ClimateZone::Temperate,
            ClimateZone::Boreal,
            ClimateZone::Polar,
            ClimateZone::Arid,
            ClimateZone::Mediterranean,
        ];
        let station = Station {
            station_id,
            lat,
            lon,
            altitude_m,
            zone: zones[zone_idx].clone(),
        };
        let encoded = encode_to_vec(&station).expect("encode Station failed");
        let (decoded, consumed): (Station, usize) =
            decode_from_slice(&encoded).expect("decode Station failed");
        prop_assert_eq!(station, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 2: Measurement roundtrip ────────────────────────────────────────────

proptest! {
    #[test]
    fn prop_measurement_roundtrip(
        station_id: u32,
        pollutant_idx in 0usize..8,
        value in proptest::num::f64::NORMAL,
        unit in "[a-zA-Z0-9/]{1,10}",
        timestamp: u64,
    ) {
        let pollutants = [
            PollutantType::CO2,
            PollutantType::CH4,
            PollutantType::NO2,
            PollutantType::SO2,
            PollutantType::PM25,
            PollutantType::PM10,
            PollutantType::Ozone,
            PollutantType::Lead,
        ];
        let measurement = Measurement {
            station_id,
            pollutant: pollutants[pollutant_idx].clone(),
            value,
            unit,
            timestamp,
        };
        let encoded = encode_to_vec(&measurement).expect("encode Measurement failed");
        let (decoded, consumed): (Measurement, usize) =
            decode_from_slice(&encoded).expect("decode Measurement failed");
        prop_assert_eq!(measurement, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 3: ClimateReport roundtrip ──────────────────────────────────────────

proptest! {
    #[test]
    fn prop_climate_report_roundtrip(
        period_start: u64,
        period_end: u64,
        stations in prop::collection::vec(station_strategy(), 0..5usize),
        measurements in prop::collection::vec(measurement_strategy(), 0..10usize),
    ) {
        let report = ClimateReport {
            period_start,
            period_end,
            stations,
            measurements,
        };
        let encoded = encode_to_vec(&report).expect("encode ClimateReport failed");
        let (decoded, consumed): (ClimateReport, usize) =
            decode_from_slice(&encoded).expect("decode ClimateReport failed");
        prop_assert_eq!(report, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 4: Station consumed == bytes.len() ───────────────────────────────────

proptest! {
    #[test]
    fn prop_station_consumed_equals_encoded_len(
        station in station_strategy(),
    ) {
        let encoded = encode_to_vec(&station).expect("encode Station failed");
        let (_decoded, consumed): (Station, usize) =
            decode_from_slice(&encoded).expect("decode Station failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 5: Measurement consumed == bytes.len() ───────────────────────────────

proptest! {
    #[test]
    fn prop_measurement_consumed_equals_encoded_len(
        meas in measurement_strategy(),
    ) {
        let encoded = encode_to_vec(&meas).expect("encode Measurement failed");
        let (_decoded, consumed): (Measurement, usize) =
            decode_from_slice(&encoded).expect("decode Measurement failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 6: ClimateReport consumed == bytes.len() ────────────────────────────

proptest! {
    #[test]
    fn prop_climate_report_consumed_equals_encoded_len(
        period_start: u64,
        period_end: u64,
        stations in prop::collection::vec(station_strategy(), 0..4usize),
        measurements in prop::collection::vec(measurement_strategy(), 0..6usize),
    ) {
        let report = ClimateReport { period_start, period_end, stations, measurements };
        let encoded = encode_to_vec(&report).expect("encode ClimateReport failed");
        let (_decoded, consumed): (ClimateReport, usize) =
            decode_from_slice(&encoded).expect("decode ClimateReport failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 7: Deterministic encoding of Station ────────────────────────────────

proptest! {
    #[test]
    fn prop_station_encoding_is_deterministic(
        station in station_strategy(),
    ) {
        let enc1 = encode_to_vec(&station).expect("first encode Station failed");
        let enc2 = encode_to_vec(&station).expect("second encode Station failed");
        prop_assert_eq!(enc1, enc2, "Station encoding must be deterministic");
    }
}

// ── Test 8: Deterministic encoding of Measurement ────────────────────────────

proptest! {
    #[test]
    fn prop_measurement_encoding_is_deterministic(
        meas in measurement_strategy(),
    ) {
        let enc1 = encode_to_vec(&meas).expect("first encode Measurement failed");
        let enc2 = encode_to_vec(&meas).expect("second encode Measurement failed");
        prop_assert_eq!(enc1, enc2, "Measurement encoding must be deterministic");
    }
}

// ── Test 9: Deterministic encoding of ClimateReport ──────────────────────────

proptest! {
    #[test]
    fn prop_climate_report_encoding_is_deterministic(
        period_start: u64,
        period_end: u64,
        stations in prop::collection::vec(station_strategy(), 0..4usize),
        measurements in prop::collection::vec(measurement_strategy(), 0..5usize),
    ) {
        let report = ClimateReport { period_start, period_end, stations, measurements };
        let enc1 = encode_to_vec(&report).expect("first encode ClimateReport failed");
        let enc2 = encode_to_vec(&report).expect("second encode ClimateReport failed");
        prop_assert_eq!(enc1, enc2, "ClimateReport encoding must be deterministic");
    }
}

// ── Test 10: All ClimateZone variants roundtrip ───────────────────────────────

proptest! {
    #[test]
    fn prop_all_climate_zone_variants_roundtrip(
        zone_idx in 0usize..7,
    ) {
        let zones = [
            ClimateZone::Tropical,
            ClimateZone::Subtropical,
            ClimateZone::Temperate,
            ClimateZone::Boreal,
            ClimateZone::Polar,
            ClimateZone::Arid,
            ClimateZone::Mediterranean,
        ];
        let zone = zones[zone_idx].clone();
        let encoded = encode_to_vec(&zone).expect("encode ClimateZone failed");
        let (decoded, consumed): (ClimateZone, usize) =
            decode_from_slice(&encoded).expect("decode ClimateZone failed");
        prop_assert_eq!(zone, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 11: All PollutantType variants roundtrip ─────────────────────────────

proptest! {
    #[test]
    fn prop_all_pollutant_type_variants_roundtrip(
        pollutant_idx in 0usize..8,
    ) {
        let pollutants = [
            PollutantType::CO2,
            PollutantType::CH4,
            PollutantType::NO2,
            PollutantType::SO2,
            PollutantType::PM25,
            PollutantType::PM10,
            PollutantType::Ozone,
            PollutantType::Lead,
        ];
        let pollutant = pollutants[pollutant_idx].clone();
        let encoded = encode_to_vec(&pollutant).expect("encode PollutantType failed");
        let (decoded, consumed): (PollutantType, usize) =
            decode_from_slice(&encoded).expect("decode PollutantType failed");
        prop_assert_eq!(pollutant, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 12: Vec<Station> roundtrip ──────────────────────────────────────────

proptest! {
    #[test]
    fn prop_vec_station_roundtrip(
        stations in prop::collection::vec(station_strategy(), 0..10usize),
    ) {
        let encoded = encode_to_vec(&stations).expect("encode Vec<Station> failed");
        let (decoded, consumed): (Vec<Station>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Station> failed");
        prop_assert_eq!(stations, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 13: Vec<Measurement> roundtrip ──────────────────────────────────────

proptest! {
    #[test]
    fn prop_vec_measurement_roundtrip(
        measurements in prop::collection::vec(measurement_strategy(), 0..15usize),
    ) {
        let encoded = encode_to_vec(&measurements).expect("encode Vec<Measurement> failed");
        let (decoded, consumed): (Vec<Measurement>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Measurement> failed");
        prop_assert_eq!(measurements, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 14: Option<Station> roundtrip ───────────────────────────────────────

proptest! {
    #[test]
    fn prop_option_station_roundtrip(
        opt in prop::option::of(station_strategy()),
    ) {
        let encoded = encode_to_vec(&opt).expect("encode Option<Station> failed");
        let (decoded, consumed): (Option<Station>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Station> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 15: Option<Measurement> roundtrip ───────────────────────────────────

proptest! {
    #[test]
    fn prop_option_measurement_roundtrip(
        opt in prop::option::of(measurement_strategy()),
    ) {
        let encoded = encode_to_vec(&opt).expect("encode Option<Measurement> failed");
        let (decoded, consumed): (Option<Measurement>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Measurement> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 16: Option<ClimateZone> roundtrip ───────────────────────────────────

proptest! {
    #[test]
    fn prop_option_climate_zone_roundtrip(
        opt in prop::option::of(climate_zone_strategy()),
    ) {
        let encoded = encode_to_vec(&opt).expect("encode Option<ClimateZone> failed");
        let (decoded, consumed): (Option<ClimateZone>, usize) =
            decode_from_slice(&encoded).expect("decode Option<ClimateZone> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 17: Option<PollutantType> roundtrip ─────────────────────────────────

proptest! {
    #[test]
    fn prop_option_pollutant_type_roundtrip(
        opt in prop::option::of(pollutant_type_strategy()),
    ) {
        let encoded = encode_to_vec(&opt).expect("encode Option<PollutantType> failed");
        let (decoded, consumed): (Option<PollutantType>, usize) =
            decode_from_slice(&encoded).expect("decode Option<PollutantType> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 18: Re-encoding decoded Station gives identical bytes ────────────────

proptest! {
    #[test]
    fn prop_station_reencode_idempotent(
        station in station_strategy(),
    ) {
        let enc1 = encode_to_vec(&station).expect("first encode Station failed");
        let (decoded, _): (Station, usize) =
            decode_from_slice(&enc1).expect("decode Station failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Station failed");
        prop_assert_eq!(enc1, enc2, "re-encoding Station must produce identical bytes");
    }
}

// ── Test 19: Re-encoding decoded Measurement gives identical bytes ────────────

proptest! {
    #[test]
    fn prop_measurement_reencode_idempotent(
        meas in measurement_strategy(),
    ) {
        let enc1 = encode_to_vec(&meas).expect("first encode Measurement failed");
        let (decoded, _): (Measurement, usize) =
            decode_from_slice(&enc1).expect("decode Measurement failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Measurement failed");
        prop_assert_eq!(enc1, enc2, "re-encoding Measurement must produce identical bytes");
    }
}

// ── Test 20: Re-encoding decoded ClimateReport gives identical bytes ──────────

proptest! {
    #[test]
    fn prop_climate_report_reencode_idempotent(
        period_start: u64,
        period_end: u64,
        stations in prop::collection::vec(station_strategy(), 0..4usize),
        measurements in prop::collection::vec(measurement_strategy(), 0..5usize),
    ) {
        let report = ClimateReport { period_start, period_end, stations, measurements };
        let enc1 = encode_to_vec(&report).expect("first encode ClimateReport failed");
        let (decoded, _): (ClimateReport, usize) =
            decode_from_slice(&enc1).expect("decode ClimateReport failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode ClimateReport failed");
        prop_assert_eq!(enc1, enc2, "re-encoding ClimateReport must produce identical bytes");
    }
}

// ── Test 21: Vec<ClimateReport> roundtrip ────────────────────────────────────

proptest! {
    #[test]
    fn prop_vec_climate_report_roundtrip(
        reports in prop::collection::vec(
            (
                any::<u64>(),
                any::<u64>(),
                prop::collection::vec(station_strategy(), 0..3usize),
                prop::collection::vec(measurement_strategy(), 0..4usize),
            ).prop_map(|(period_start, period_end, stations, measurements)| ClimateReport {
                period_start,
                period_end,
                stations,
                measurements,
            }),
            0..5usize,
        ),
    ) {
        let encoded = encode_to_vec(&reports).expect("encode Vec<ClimateReport> failed");
        let (decoded, consumed): (Vec<ClimateReport>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<ClimateReport> failed");
        prop_assert_eq!(reports, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 22: Option<ClimateReport> roundtrip ─────────────────────────────────

proptest! {
    #[test]
    fn prop_option_climate_report_roundtrip(
        opt in prop::option::of(
            (
                any::<u64>(),
                any::<u64>(),
                prop::collection::vec(station_strategy(), 0..3usize),
                prop::collection::vec(measurement_strategy(), 0..4usize),
            ).prop_map(|(period_start, period_end, stations, measurements)| ClimateReport {
                period_start,
                period_end,
                stations,
                measurements,
            }),
        ),
    ) {
        let encoded = encode_to_vec(&opt).expect("encode Option<ClimateReport> failed");
        let (decoded, consumed): (Option<ClimateReport>, usize) =
            decode_from_slice(&encoded).expect("decode Option<ClimateReport> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}
