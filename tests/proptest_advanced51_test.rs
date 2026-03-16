//! Advanced property-based roundtrip tests (set 51) using proptest.
//!
//! Domain: Weather forecasting / meteorological data.
//! Tests verify encode → decode roundtrips for domain types and structural
//! invariants such as consumed == bytes.len() and deterministic encoding.

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
enum WeatherPhenomenon {
    Clear,
    Cloudy,
    Rain,
    Snow,
    Fog,
    Storm,
    Hail,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PressureTrend {
    Rising,
    Falling,
    Steady,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherObservation {
    station_id: u32,
    temp_celsius: f32,
    humidity: f32,
    pressure_hpa: f32,
    phenomenon: WeatherPhenomenon,
    trend: PressureTrend,
    wind_speed_ms: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ForecastPeriod {
    period_id: u64,
    observations: Vec<WeatherObservation>,
    avg_temp: f32,
    precipitation_mm: f32,
}

// ---------------------------------------------------------------------------
// Arbitrary strategies
// ---------------------------------------------------------------------------

fn arb_weather_phenomenon() -> impl Strategy<Value = WeatherPhenomenon> {
    prop_oneof![
        Just(WeatherPhenomenon::Clear),
        Just(WeatherPhenomenon::Cloudy),
        Just(WeatherPhenomenon::Rain),
        Just(WeatherPhenomenon::Snow),
        Just(WeatherPhenomenon::Fog),
        Just(WeatherPhenomenon::Storm),
        Just(WeatherPhenomenon::Hail),
    ]
}

fn arb_pressure_trend() -> impl Strategy<Value = PressureTrend> {
    prop_oneof![
        Just(PressureTrend::Rising),
        Just(PressureTrend::Falling),
        Just(PressureTrend::Steady),
    ]
}

fn arb_weather_observation() -> impl Strategy<Value = WeatherObservation> {
    (
        any::<u32>(),
        proptest::num::f32::NORMAL,
        proptest::num::f32::NORMAL,
        proptest::num::f32::NORMAL,
        arb_weather_phenomenon(),
        arb_pressure_trend(),
        proptest::num::f32::NORMAL,
    )
        .prop_map(
            |(
                station_id,
                temp_celsius,
                humidity,
                pressure_hpa,
                phenomenon,
                trend,
                wind_speed_ms,
            )| {
                WeatherObservation {
                    station_id,
                    temp_celsius,
                    humidity,
                    pressure_hpa,
                    phenomenon,
                    trend,
                    wind_speed_ms,
                }
            },
        )
}

fn arb_forecast_period() -> impl Strategy<Value = ForecastPeriod> {
    (
        any::<u64>(),
        proptest::collection::vec(arb_weather_observation(), 0..8),
        proptest::num::f32::NORMAL,
        proptest::num::f32::NORMAL,
    )
        .prop_map(
            |(period_id, observations, avg_temp, precipitation_mm)| ForecastPeriod {
                period_id,
                observations,
                avg_temp,
                precipitation_mm,
            },
        )
}

// ---------------------------------------------------------------------------
// Test 1: WeatherPhenomenon roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_phenomenon_roundtrip(phenomenon in arb_weather_phenomenon()) {
        let encoded = encode_to_vec(&phenomenon).expect("encode WeatherPhenomenon failed");
        let (decoded, consumed): (WeatherPhenomenon, usize) =
            decode_from_slice(&encoded).expect("decode WeatherPhenomenon failed");
        prop_assert_eq!(decoded, phenomenon);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 2: PressureTrend roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_pressure_trend_roundtrip(trend in arb_pressure_trend()) {
        let encoded = encode_to_vec(&trend).expect("encode PressureTrend failed");
        let (decoded, consumed): (PressureTrend, usize) =
            decode_from_slice(&encoded).expect("decode PressureTrend failed");
        prop_assert_eq!(decoded, trend);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 3: WeatherObservation roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_observation_roundtrip(obs in arb_weather_observation()) {
        let encoded = encode_to_vec(&obs).expect("encode WeatherObservation failed");
        let (decoded, consumed): (WeatherObservation, usize) =
            decode_from_slice(&encoded).expect("decode WeatherObservation failed");
        prop_assert_eq!(decoded, obs);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 4: ForecastPeriod roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_forecast_period_roundtrip(period in arb_forecast_period()) {
        let encoded = encode_to_vec(&period).expect("encode ForecastPeriod failed");
        let (decoded, consumed): (ForecastPeriod, usize) =
            decode_from_slice(&encoded).expect("decode ForecastPeriod failed");
        prop_assert_eq!(decoded, period);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 5: consumed == bytes.len() for WeatherPhenomenon
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_phenomenon_consumed_equals_len(phenomenon in arb_weather_phenomenon()) {
        let encoded = encode_to_vec(&phenomenon).expect("encode WeatherPhenomenon failed");
        let (_decoded, consumed): (WeatherPhenomenon, usize) =
            decode_from_slice(&encoded).expect("decode WeatherPhenomenon failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 6: consumed == bytes.len() for WeatherObservation
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_observation_consumed_equals_len(obs in arb_weather_observation()) {
        let encoded = encode_to_vec(&obs).expect("encode WeatherObservation failed");
        let (_decoded, consumed): (WeatherObservation, usize) =
            decode_from_slice(&encoded).expect("decode WeatherObservation failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 7: consumed == bytes.len() for ForecastPeriod
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_forecast_period_consumed_equals_len(period in arb_forecast_period()) {
        let encoded = encode_to_vec(&period).expect("encode ForecastPeriod failed");
        let (_decoded, consumed): (ForecastPeriod, usize) =
            decode_from_slice(&encoded).expect("decode ForecastPeriod failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 8: deterministic encoding for WeatherPhenomenon
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_phenomenon_deterministic(phenomenon in arb_weather_phenomenon()) {
        let encoded_a = encode_to_vec(&phenomenon).expect("first encode WeatherPhenomenon failed");
        let encoded_b = encode_to_vec(&phenomenon).expect("second encode WeatherPhenomenon failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }
}

// ---------------------------------------------------------------------------
// Test 9: deterministic encoding for WeatherObservation
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_observation_deterministic(obs in arb_weather_observation()) {
        let encoded_a = encode_to_vec(&obs).expect("first encode WeatherObservation failed");
        let encoded_b = encode_to_vec(&obs).expect("second encode WeatherObservation failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }
}

// ---------------------------------------------------------------------------
// Test 10: deterministic encoding for ForecastPeriod
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_forecast_period_deterministic(period in arb_forecast_period()) {
        let encoded_a = encode_to_vec(&period).expect("first encode ForecastPeriod failed");
        let encoded_b = encode_to_vec(&period).expect("second encode ForecastPeriod failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }
}

// ---------------------------------------------------------------------------
// Test 11: Vec<WeatherObservation> roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_vec_observation_roundtrip(
        observations in proptest::collection::vec(arb_weather_observation(), 0..10)
    ) {
        let encoded = encode_to_vec(&observations).expect("encode Vec<WeatherObservation> failed");
        let (decoded, consumed): (Vec<WeatherObservation>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<WeatherObservation> failed");
        prop_assert_eq!(decoded, observations);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 12: Option<WeatherObservation> roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_option_observation_roundtrip(
        maybe_obs in proptest::option::of(arb_weather_observation())
    ) {
        let encoded = encode_to_vec(&maybe_obs).expect("encode Option<WeatherObservation> failed");
        let (decoded, consumed): (Option<WeatherObservation>, usize) =
            decode_from_slice(&encoded).expect("decode Option<WeatherObservation> failed");
        prop_assert_eq!(decoded, maybe_obs);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 13: Option<ForecastPeriod> roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_option_forecast_period_roundtrip(
        maybe_period in proptest::option::of(arb_forecast_period())
    ) {
        let encoded = encode_to_vec(&maybe_period).expect("encode Option<ForecastPeriod> failed");
        let (decoded, consumed): (Option<ForecastPeriod>, usize) =
            decode_from_slice(&encoded).expect("decode Option<ForecastPeriod> failed");
        prop_assert_eq!(decoded, maybe_period);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 14: re-encode idempotency for WeatherObservation
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_observation_reencode_idempotency(obs in arb_weather_observation()) {
        let encoded_first = encode_to_vec(&obs).expect("first encode WeatherObservation failed");
        let (decoded, _consumed): (WeatherObservation, usize) =
            decode_from_slice(&encoded_first).expect("decode WeatherObservation failed");
        let encoded_second = encode_to_vec(&decoded).expect("re-encode WeatherObservation failed");
        prop_assert_eq!(encoded_first, encoded_second);
    }
}

// ---------------------------------------------------------------------------
// Test 15: re-encode idempotency for ForecastPeriod
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_forecast_period_reencode_idempotency(period in arb_forecast_period()) {
        let encoded_first = encode_to_vec(&period).expect("first encode ForecastPeriod failed");
        let (decoded, _consumed): (ForecastPeriod, usize) =
            decode_from_slice(&encoded_first).expect("decode ForecastPeriod failed");
        let encoded_second = encode_to_vec(&decoded).expect("re-encode ForecastPeriod failed");
        prop_assert_eq!(encoded_first, encoded_second);
    }
}

// ---------------------------------------------------------------------------
// Test 16: all WeatherPhenomenon variants encode and decode correctly (exhaustive)
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_all_phenomenon_variants(_dummy: bool) {
        let variants = [
            WeatherPhenomenon::Clear,
            WeatherPhenomenon::Cloudy,
            WeatherPhenomenon::Rain,
            WeatherPhenomenon::Snow,
            WeatherPhenomenon::Fog,
            WeatherPhenomenon::Storm,
            WeatherPhenomenon::Hail,
        ];
        for variant in &variants {
            let encoded = encode_to_vec(variant).expect("encode WeatherPhenomenon variant failed");
            let (decoded, consumed): (WeatherPhenomenon, usize) =
                decode_from_slice(&encoded).expect("decode WeatherPhenomenon variant failed");
            prop_assert_eq!(&decoded, variant);
            prop_assert_eq!(consumed, encoded.len());
        }
    }
}

// ---------------------------------------------------------------------------
// Test 17: all PressureTrend variants encode and decode correctly (exhaustive)
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_all_pressure_trend_variants(_dummy: bool) {
        let variants = [
            PressureTrend::Rising,
            PressureTrend::Falling,
            PressureTrend::Steady,
        ];
        for variant in &variants {
            let encoded = encode_to_vec(variant).expect("encode PressureTrend variant failed");
            let (decoded, consumed): (PressureTrend, usize) =
                decode_from_slice(&encoded).expect("decode PressureTrend variant failed");
            prop_assert_eq!(&decoded, variant);
            prop_assert_eq!(consumed, encoded.len());
        }
    }
}

// ---------------------------------------------------------------------------
// Test 18: WeatherObservation with station_id == 0 roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_observation_station_zero_roundtrip(
        temp_celsius in proptest::num::f32::NORMAL,
        humidity in proptest::num::f32::NORMAL,
        pressure_hpa in proptest::num::f32::NORMAL,
        phenomenon in arb_weather_phenomenon(),
        trend in arb_pressure_trend(),
        wind_speed_ms in proptest::num::f32::NORMAL,
    ) {
        let obs = WeatherObservation {
            station_id: 0,
            temp_celsius,
            humidity,
            pressure_hpa,
            phenomenon,
            trend,
            wind_speed_ms,
        };
        let encoded = encode_to_vec(&obs).expect("encode WeatherObservation (station 0) failed");
        let (decoded, consumed): (WeatherObservation, usize) =
            decode_from_slice(&encoded).expect("decode WeatherObservation (station 0) failed");
        prop_assert_eq!(decoded, obs);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 19: ForecastPeriod with empty observations roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_forecast_period_empty_observations(
        period_id: u64,
        avg_temp in proptest::num::f32::NORMAL,
        precipitation_mm in proptest::num::f32::NORMAL,
    ) {
        let period = ForecastPeriod {
            period_id,
            observations: Vec::new(),
            avg_temp,
            precipitation_mm,
        };
        let encoded = encode_to_vec(&period).expect("encode ForecastPeriod (empty obs) failed");
        let (decoded, consumed): (ForecastPeriod, usize) =
            decode_from_slice(&encoded).expect("decode ForecastPeriod (empty obs) failed");
        prop_assert_eq!(decoded, period);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 20: WeatherObservation with max station_id roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_observation_max_station_id_roundtrip(
        temp_celsius in proptest::num::f32::NORMAL,
        humidity in proptest::num::f32::NORMAL,
        pressure_hpa in proptest::num::f32::NORMAL,
        phenomenon in arb_weather_phenomenon(),
        trend in arb_pressure_trend(),
        wind_speed_ms in proptest::num::f32::NORMAL,
    ) {
        let obs = WeatherObservation {
            station_id: u32::MAX,
            temp_celsius,
            humidity,
            pressure_hpa,
            phenomenon,
            trend,
            wind_speed_ms,
        };
        let encoded = encode_to_vec(&obs).expect("encode WeatherObservation (max station) failed");
        let (decoded, consumed): (WeatherObservation, usize) =
            decode_from_slice(&encoded).expect("decode WeatherObservation (max station) failed");
        prop_assert_eq!(decoded, obs);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 21: Vec<ForecastPeriod> roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_vec_forecast_period_roundtrip(
        periods in proptest::collection::vec(arb_forecast_period(), 0..5)
    ) {
        let encoded = encode_to_vec(&periods).expect("encode Vec<ForecastPeriod> failed");
        let (decoded, consumed): (Vec<ForecastPeriod>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<ForecastPeriod> failed");
        prop_assert_eq!(decoded, periods);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 22: Pair of (WeatherObservation, ForecastPeriod) tuple roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_weather_observation_forecast_pair_roundtrip(
        obs in arb_weather_observation(),
        period in arb_forecast_period(),
    ) {
        let pair = (obs, period);
        let encoded = encode_to_vec(&pair).expect("encode (WeatherObservation, ForecastPeriod) failed");
        let (decoded, consumed): ((WeatherObservation, ForecastPeriod), usize) =
            decode_from_slice(&encoded).expect("decode (WeatherObservation, ForecastPeriod) failed");
        prop_assert_eq!(decoded, pair);
        prop_assert_eq!(consumed, encoded.len());
    }
}
