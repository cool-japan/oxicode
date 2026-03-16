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
struct WeatherReading {
    station_id: u32,
    temperature_c: i16,
    humidity_pct: u32,
    timestamp_s: u64,
    is_valid: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WindVector {
    speed_kph: u32,
    direction_deg: u32,
    gust_kph: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PressureReading {
    station_id: u32,
    pressure_hpa: u32,
    altitude_m: i16,
    timestamp_s: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WeatherCondition {
    Clear,
    Cloudy,
    Rainy { intensity_mm: u32 },
    Stormy { wind_kph: u32, pressure_hpa: u32 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClimateRecord {
    reading: WeatherReading,
    wind: WindVector,
    pressure: PressureReading,
    condition: WeatherCondition,
}

fn arb_weather_condition() -> impl Strategy<Value = WeatherCondition> {
    prop_oneof![
        Just(WeatherCondition::Clear),
        Just(WeatherCondition::Cloudy),
        any::<u32>().prop_map(|intensity_mm| WeatherCondition::Rainy { intensity_mm }),
        (any::<u32>(), any::<u32>()).prop_map(|(wind_kph, pressure_hpa)| {
            WeatherCondition::Stormy {
                wind_kph,
                pressure_hpa,
            }
        }),
    ]
}

fn arb_weather_reading() -> impl Strategy<Value = WeatherReading> {
    (
        any::<u32>(),
        any::<i16>(),
        0u32..=100u32,
        any::<u64>(),
        any::<bool>(),
    )
        .prop_map(
            |(station_id, temperature_c, humidity_pct, timestamp_s, is_valid)| WeatherReading {
                station_id,
                temperature_c,
                humidity_pct,
                timestamp_s,
                is_valid,
            },
        )
}

fn arb_wind_vector() -> impl Strategy<Value = WindVector> {
    (any::<u32>(), 0u32..=360u32, any::<u32>()).prop_map(|(speed_kph, direction_deg, gust_kph)| {
        WindVector {
            speed_kph,
            direction_deg,
            gust_kph,
        }
    })
}

fn arb_pressure_reading() -> impl Strategy<Value = PressureReading> {
    (any::<u32>(), any::<u32>(), any::<i16>(), any::<u64>()).prop_map(
        |(station_id, pressure_hpa, altitude_m, timestamp_s)| PressureReading {
            station_id,
            pressure_hpa,
            altitude_m,
            timestamp_s,
        },
    )
}

fn arb_climate_record() -> impl Strategy<Value = ClimateRecord> {
    (
        arb_weather_reading(),
        arb_wind_vector(),
        arb_pressure_reading(),
        arb_weather_condition(),
    )
        .prop_map(|(reading, wind, pressure, condition)| ClimateRecord {
            reading,
            wind,
            pressure,
            condition,
        })
}

// Test 1: WeatherReading basic roundtrip
proptest! {
    #[test]
    fn test_weather_reading_roundtrip(reading in arb_weather_reading()) {
        let bytes = encode_to_vec(&reading).expect("encode failed");
        let (decoded, consumed): (WeatherReading, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&reading, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}

// Test 2: WindVector basic roundtrip
proptest! {
    #[test]
    fn test_wind_vector_roundtrip(wind in arb_wind_vector()) {
        let bytes = encode_to_vec(&wind).expect("encode failed");
        let (decoded, consumed): (WindVector, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&wind, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}

// Test 3: PressureReading basic roundtrip
proptest! {
    #[test]
    fn test_pressure_reading_roundtrip(pressure in arb_pressure_reading()) {
        let bytes = encode_to_vec(&pressure).expect("encode failed");
        let (decoded, consumed): (PressureReading, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&pressure, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}

// Test 4: WeatherCondition enum roundtrip (all variants)
proptest! {
    #[test]
    fn test_weather_condition_roundtrip(condition in arb_weather_condition()) {
        let bytes = encode_to_vec(&condition).expect("encode failed");
        let (decoded, consumed): (WeatherCondition, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&condition, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}

// Test 5: ClimateRecord nested struct roundtrip
proptest! {
    #[test]
    fn test_climate_record_roundtrip(record in arb_climate_record()) {
        let bytes = encode_to_vec(&record).expect("encode failed");
        let (decoded, consumed): (ClimateRecord, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&record, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}

// Test 6: Vec<WeatherReading> roundtrip
proptest! {
    #[test]
    fn test_vec_weather_reading_roundtrip(
        readings in prop::collection::vec(arb_weather_reading(), 0..=16)
    ) {
        let bytes = encode_to_vec(&readings).expect("encode failed");
        let (decoded, consumed): (Vec<WeatherReading>, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&readings, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}

// Test 7: Option<WeatherReading> roundtrip
proptest! {
    #[test]
    fn test_option_weather_reading_roundtrip(
        opt in prop::option::of(arb_weather_reading())
    ) {
        let bytes = encode_to_vec(&opt).expect("encode failed");
        let (decoded, consumed): (Option<WeatherReading>, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&opt, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}

// Test 8: Deterministic encoding — same WeatherReading produces same bytes
proptest! {
    #[test]
    fn test_weather_reading_deterministic(reading in arb_weather_reading()) {
        let bytes1 = encode_to_vec(&reading).expect("encode failed");
        let bytes2 = encode_to_vec(&reading).expect("encode failed");
        prop_assert_eq!(bytes1, bytes2);
    }
}

// Test 9: Consumed bytes equals encoded length for ClimateRecord
proptest! {
    #[test]
    fn test_climate_record_consumed_equals_len(record in arb_climate_record()) {
        let bytes = encode_to_vec(&record).expect("encode failed");
        let expected_len = bytes.len();
        let (_, consumed): (ClimateRecord, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(consumed, expected_len);
    }
}

// Test 10: i16 temperature field range preserved
proptest! {
    #[test]
    fn test_temperature_i16_range_preserved(temperature_c in any::<i16>()) {
        let reading = WeatherReading {
            station_id: 1,
            temperature_c,
            humidity_pct: 50,
            timestamp_s: 0,
            is_valid: true,
        };
        let bytes = encode_to_vec(&reading).expect("encode failed");
        let (decoded, _): (WeatherReading, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.temperature_c, temperature_c);
    }
}

// Test 11: u64 timestamp field preserved in WeatherReading
proptest! {
    #[test]
    fn test_timestamp_u64_preserved(timestamp_s in any::<u64>()) {
        let reading = WeatherReading {
            station_id: 42,
            temperature_c: 20,
            humidity_pct: 60,
            timestamp_s,
            is_valid: false,
        };
        let bytes = encode_to_vec(&reading).expect("encode failed");
        let (decoded, _): (WeatherReading, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.timestamp_s, timestamp_s);
    }
}

// Test 12: bool is_valid field preserved
proptest! {
    #[test]
    fn test_bool_field_preserved(is_valid in any::<bool>()) {
        let reading = WeatherReading {
            station_id: 7,
            temperature_c: -5,
            humidity_pct: 80,
            timestamp_s: 1_700_000_000,
            is_valid,
        };
        let bytes = encode_to_vec(&reading).expect("encode failed");
        let (decoded, _): (WeatherReading, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.is_valid, is_valid);
    }
}

// Test 13: Rainy variant intensity_mm field preserved
proptest! {
    #[test]
    fn test_rainy_intensity_preserved(intensity_mm in any::<u32>()) {
        let condition = WeatherCondition::Rainy { intensity_mm };
        let bytes = encode_to_vec(&condition).expect("encode failed");
        let (decoded, _): (WeatherCondition, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&condition, &decoded);
        if let WeatherCondition::Rainy { intensity_mm: decoded_mm } = decoded {
            prop_assert_eq!(decoded_mm, intensity_mm);
        } else {
            return Err(TestCaseError::fail("expected Rainy variant"));
        }
    }
}

// Test 14: Stormy variant both fields preserved
proptest! {
    #[test]
    fn test_stormy_fields_preserved(wind_kph in any::<u32>(), pressure_hpa in any::<u32>()) {
        let condition = WeatherCondition::Stormy { wind_kph, pressure_hpa };
        let bytes = encode_to_vec(&condition).expect("encode failed");
        let (decoded, consumed): (WeatherCondition, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&condition, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}

// Test 15: WindVector direction 0..=360 preserved
proptest! {
    #[test]
    fn test_wind_direction_range_preserved(direction_deg in 0u32..=360u32) {
        let wind = WindVector { speed_kph: 10, direction_deg, gust_kph: 15 };
        let bytes = encode_to_vec(&wind).expect("encode failed");
        let (decoded, _): (WindVector, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.direction_deg, direction_deg);
    }
}

// Test 16: PressureReading altitude_m i16 field preserved
proptest! {
    #[test]
    fn test_altitude_i16_preserved(altitude_m in any::<i16>()) {
        let pressure = PressureReading {
            station_id: 99,
            pressure_hpa: 1013,
            altitude_m,
            timestamp_s: 0,
        };
        let bytes = encode_to_vec(&pressure).expect("encode failed");
        let (decoded, _): (PressureReading, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.altitude_m, altitude_m);
    }
}

// Test 17: Two distinct WeatherReadings with different temperatures encode differently
proptest! {
    #[test]
    fn test_distinct_temperatures_encode_differently(
        t1 in any::<i16>(),
        t2 in any::<i16>()
    ) {
        prop_assume!(t1 != t2);
        let r1 = WeatherReading {
            station_id: 1, temperature_c: t1, humidity_pct: 50, timestamp_s: 0, is_valid: true,
        };
        let r2 = WeatherReading {
            station_id: 1, temperature_c: t2, humidity_pct: 50, timestamp_s: 0, is_valid: true,
        };
        let bytes1 = encode_to_vec(&r1).expect("encode failed");
        let bytes2 = encode_to_vec(&r2).expect("encode failed");
        prop_assert_ne!(bytes1, bytes2);
    }
}

// Test 18: Trailing bytes do not affect decode of WeatherReading
proptest! {
    #[test]
    fn test_weather_reading_ignores_trailing_bytes(
        reading in arb_weather_reading(),
        extra in prop::collection::vec(any::<u8>(), 1..=16)
    ) {
        let mut bytes = encode_to_vec(&reading).expect("encode failed");
        let original_len = bytes.len();
        bytes.extend_from_slice(&extra);
        let (decoded, consumed): (WeatherReading, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&reading, &decoded);
        prop_assert_eq!(consumed, original_len);
    }
}

// Test 19: Option<WeatherCondition> roundtrip
proptest! {
    #[test]
    fn test_option_weather_condition_roundtrip(
        opt in prop::option::of(arb_weather_condition())
    ) {
        let bytes = encode_to_vec(&opt).expect("encode failed");
        let (decoded, consumed): (Option<WeatherCondition>, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&opt, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}

// Test 20: Vec<ClimateRecord> roundtrip
proptest! {
    #[test]
    fn test_vec_climate_records_roundtrip(
        records in prop::collection::vec(arb_climate_record(), 0..=4)
    ) {
        let bytes = encode_to_vec(&records).expect("encode failed");
        let (decoded, consumed): (Vec<ClimateRecord>, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&records, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}

// Test 21: Double encode-decode yields identical ClimateRecord and bytes
proptest! {
    #[test]
    fn test_climate_record_double_encode_decode_identity(record in arb_climate_record()) {
        let bytes1 = encode_to_vec(&record).expect("encode failed");
        let (decoded1, _): (ClimateRecord, usize) =
            decode_from_slice(&bytes1).expect("first decode failed");
        let bytes2 = encode_to_vec(&decoded1).expect("re-encode failed");
        let (decoded2, consumed2): (ClimateRecord, usize) =
            decode_from_slice(&bytes2).expect("second decode failed");
        prop_assert_eq!(&record, &decoded2);
        prop_assert_eq!(consumed2, bytes2.len());
        prop_assert_eq!(bytes1, bytes2);
    }
}

// Test 22: Encoded bytes are non-empty for every ClimateRecord
proptest! {
    #[test]
    fn test_climate_record_encoded_nonempty(record in arb_climate_record()) {
        let bytes = encode_to_vec(&record).expect("encode failed");
        prop_assert!(!bytes.is_empty());
    }
}
