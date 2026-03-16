//! Tests for Weather / Meteorology — advanced enum roundtrip coverage.
//!
//! Domain types model a simplified meteorological observation and reporting system:
//! weather conditions, precipitation types, wind directions, measurements, stations,
//! and full weather reports with optional 24-hour forecasts.

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WeatherCondition {
    Sunny,
    PartlyCloudy,
    Overcast,
    Rainy,
    Stormy,
    Snowy,
    Foggy,
    Windy,
    Hail,
    Tornado,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PrecipitationType {
    Rain,
    Snow,
    Sleet,
    Hail,
    Drizzle,
    None,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WindDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherMeasurement {
    temperature_c: f32,
    humidity_pct: f32,
    pressure_hpa: f32,
    wind_speed_ms: f32,
    wind_dir: WindDirection,
    precip_type: PrecipitationType,
    precip_mm: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherStation {
    station_id: u32,
    name: String,
    lat: f64,
    lon: f64,
    altitude_m: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherReport {
    station: WeatherStation,
    timestamp: u64,
    condition: WeatherCondition,
    measurement: WeatherMeasurement,
    forecast_24h: Option<WeatherCondition>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_station(
    station_id: u32,
    name: &str,
    lat: f64,
    lon: f64,
    altitude_m: i32,
) -> WeatherStation {
    WeatherStation {
        station_id,
        name: name.to_string(),
        lat,
        lon,
        altitude_m,
    }
}

fn make_measurement(
    temperature_c: f32,
    humidity_pct: f32,
    pressure_hpa: f32,
    wind_speed_ms: f32,
    wind_dir: WindDirection,
    precip_type: PrecipitationType,
    precip_mm: f32,
) -> WeatherMeasurement {
    WeatherMeasurement {
        temperature_c,
        humidity_pct,
        pressure_hpa,
        wind_speed_ms,
        wind_dir,
        precip_type,
        precip_mm,
    }
}

fn make_report(
    station: WeatherStation,
    timestamp: u64,
    condition: WeatherCondition,
    measurement: WeatherMeasurement,
    forecast_24h: Option<WeatherCondition>,
) -> WeatherReport {
    WeatherReport {
        station,
        timestamp,
        condition,
        measurement,
        forecast_24h,
    }
}

// ---------------------------------------------------------------------------
// Test 1: WeatherCondition — all 10 variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_condition_all_variants_roundtrip() {
    let conditions = vec![
        WeatherCondition::Sunny,
        WeatherCondition::PartlyCloudy,
        WeatherCondition::Overcast,
        WeatherCondition::Rainy,
        WeatherCondition::Stormy,
        WeatherCondition::Snowy,
        WeatherCondition::Foggy,
        WeatherCondition::Windy,
        WeatherCondition::Hail,
        WeatherCondition::Tornado,
    ];

    for condition in &conditions {
        let bytes = encode_to_vec(condition).expect("encode WeatherCondition");
        let (decoded, consumed): (WeatherCondition, usize) =
            decode_from_slice(&bytes).expect("decode WeatherCondition");
        assert_eq!(condition, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for WeatherCondition"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 2: WeatherCondition — discriminant uniqueness across all 10 variants
// ---------------------------------------------------------------------------

#[test]
fn test_weather_condition_discriminant_uniqueness() {
    let conditions = vec![
        WeatherCondition::Sunny,
        WeatherCondition::PartlyCloudy,
        WeatherCondition::Overcast,
        WeatherCondition::Rainy,
        WeatherCondition::Stormy,
        WeatherCondition::Snowy,
        WeatherCondition::Foggy,
        WeatherCondition::Windy,
        WeatherCondition::Hail,
        WeatherCondition::Tornado,
    ];

    let encodings: Vec<Vec<u8>> = conditions
        .iter()
        .map(|c| encode_to_vec(c).expect("encode WeatherCondition for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "WeatherCondition variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 3: PrecipitationType — all 6 variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_precipitation_type_all_variants_roundtrip() {
    let precip_types = vec![
        PrecipitationType::Rain,
        PrecipitationType::Snow,
        PrecipitationType::Sleet,
        PrecipitationType::Hail,
        PrecipitationType::Drizzle,
        PrecipitationType::None,
    ];

    for pt in &precip_types {
        let bytes = encode_to_vec(pt).expect("encode PrecipitationType");
        let (decoded, consumed): (PrecipitationType, usize) =
            decode_from_slice(&bytes).expect("decode PrecipitationType");
        assert_eq!(pt, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for PrecipitationType"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 4: PrecipitationType — discriminant uniqueness
// ---------------------------------------------------------------------------

#[test]
fn test_precipitation_type_discriminant_uniqueness() {
    let precip_types = vec![
        PrecipitationType::Rain,
        PrecipitationType::Snow,
        PrecipitationType::Sleet,
        PrecipitationType::Hail,
        PrecipitationType::Drizzle,
        PrecipitationType::None,
    ];

    let encodings: Vec<Vec<u8>> = precip_types
        .iter()
        .map(|pt| encode_to_vec(pt).expect("encode PrecipitationType for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "PrecipitationType variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 5: WindDirection — all 8 cardinal/intercardinal variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_wind_direction_all_variants_roundtrip() {
    let dirs = vec![
        WindDirection::N,
        WindDirection::NE,
        WindDirection::E,
        WindDirection::SE,
        WindDirection::S,
        WindDirection::SW,
        WindDirection::W,
        WindDirection::NW,
    ];

    for dir in &dirs {
        let bytes = encode_to_vec(dir).expect("encode WindDirection");
        let (decoded, consumed): (WindDirection, usize) =
            decode_from_slice(&bytes).expect("decode WindDirection");
        assert_eq!(dir, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for WindDirection"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 6: WindDirection — discriminant uniqueness
// ---------------------------------------------------------------------------

#[test]
fn test_wind_direction_discriminant_uniqueness() {
    let dirs = vec![
        WindDirection::N,
        WindDirection::NE,
        WindDirection::E,
        WindDirection::SE,
        WindDirection::S,
        WindDirection::SW,
        WindDirection::W,
        WindDirection::NW,
    ];

    let encodings: Vec<Vec<u8>> = dirs
        .iter()
        .map(|d| encode_to_vec(d).expect("encode WindDirection for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "WindDirection variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 7: WeatherMeasurement — clear summer day roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_measurement_clear_summer_day_roundtrip() {
    let m = make_measurement(
        28.5,
        45.0,
        1013.25,
        3.2,
        WindDirection::SW,
        PrecipitationType::None,
        0.0,
    );
    let bytes = encode_to_vec(&m).expect("encode WeatherMeasurement clear summer");
    let (decoded, consumed): (WeatherMeasurement, usize) =
        decode_from_slice(&bytes).expect("decode WeatherMeasurement clear summer");
    assert_eq!(m, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal full WeatherMeasurement encoding"
    );
}

// ---------------------------------------------------------------------------
// Test 8: WeatherMeasurement — blizzard conditions roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_measurement_blizzard_roundtrip() {
    let m = make_measurement(
        -18.3,
        92.0,
        985.0,
        22.5,
        WindDirection::NW,
        PrecipitationType::Snow,
        45.2,
    );
    let bytes = encode_to_vec(&m).expect("encode WeatherMeasurement blizzard");
    let (decoded, consumed): (WeatherMeasurement, usize) =
        decode_from_slice(&bytes).expect("decode WeatherMeasurement blizzard");
    assert_eq!(m, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal blizzard measurement encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 9: WeatherStation — high-altitude Alpine station roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_station_alpine_roundtrip() {
    let station = make_station(4001, "Zugspitze Observatory", 47.421_3, 10.985_4, 2962);
    let bytes = encode_to_vec(&station).expect("encode WeatherStation alpine");
    let (decoded, consumed): (WeatherStation, usize) =
        decode_from_slice(&bytes).expect("decode WeatherStation alpine");
    assert_eq!(station, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal alpine station encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 10: WeatherStation — below sea level desert station roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_station_below_sea_level_roundtrip() {
    let station = make_station(
        7777,
        "Dead Sea Meteorological Post",
        31.559_7,
        35.473_0,
        -430,
    );
    let bytes = encode_to_vec(&station).expect("encode WeatherStation below sea level");
    let (decoded, consumed): (WeatherStation, usize) =
        decode_from_slice(&bytes).expect("decode WeatherStation below sea level");
    assert_eq!(station, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal below-sea-level station encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 11: WeatherReport — sunny day with forecast roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_report_sunny_with_forecast_roundtrip() {
    let station = make_station(1001, "Helsinki Airport", 60.317_2, 24.963_3, 54);
    let measurement = make_measurement(
        22.0,
        55.0,
        1018.0,
        4.5,
        WindDirection::SE,
        PrecipitationType::None,
        0.0,
    );
    let report = make_report(
        station,
        1_700_000_000,
        WeatherCondition::Sunny,
        measurement,
        Some(WeatherCondition::PartlyCloudy),
    );
    let bytes = encode_to_vec(&report).expect("encode WeatherReport sunny");
    let (decoded, consumed): (WeatherReport, usize) =
        decode_from_slice(&bytes).expect("decode WeatherReport sunny");
    assert_eq!(report, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal sunny report encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 12: WeatherReport — tornado warning with no 24h forecast roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_report_tornado_no_forecast_roundtrip() {
    let station = make_station(
        5050,
        "Tornado Alley Station KS-04",
        37.689_2,
        -97.336_0,
        402,
    );
    let measurement = make_measurement(
        32.1,
        78.0,
        965.0,
        45.0,
        WindDirection::SW,
        PrecipitationType::Hail,
        12.5,
    );
    let report = make_report(
        station,
        1_710_000_000,
        WeatherCondition::Tornado,
        measurement,
        Option::None,
    );
    let bytes = encode_to_vec(&report).expect("encode WeatherReport tornado");
    let (decoded, consumed): (WeatherReport, usize) =
        decode_from_slice(&bytes).expect("decode WeatherReport tornado");
    assert_eq!(report, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal tornado report encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 13: WeatherReport — foggy coast with forecast stormy roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_report_foggy_forecast_stormy_roundtrip() {
    let station = make_station(2002, "San Francisco Bay Station", 37.774_9, -122.419_4, 16);
    let measurement = make_measurement(
        14.5,
        95.0,
        1007.0,
        2.0,
        WindDirection::W,
        PrecipitationType::Drizzle,
        0.8,
    );
    let report = make_report(
        station,
        1_720_000_000,
        WeatherCondition::Foggy,
        measurement,
        Some(WeatherCondition::Stormy),
    );
    let bytes = encode_to_vec(&report).expect("encode WeatherReport foggy");
    let (decoded, consumed): (WeatherReport, usize) =
        decode_from_slice(&bytes).expect("decode WeatherReport foggy");
    assert_eq!(report, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal foggy report encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 14: WeatherReport — extreme cold with sleet roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_report_extreme_cold_sleet_roundtrip() {
    let station = make_station(3003, "Yakutsk Synoptic Station", 62.027_8, 129.732_5, 100);
    let measurement = make_measurement(
        -52.6,
        83.0,
        1040.0,
        1.5,
        WindDirection::N,
        PrecipitationType::Sleet,
        2.1,
    );
    let report = make_report(
        station,
        1_730_000_000,
        WeatherCondition::Snowy,
        measurement,
        Some(WeatherCondition::Overcast),
    );
    let bytes = encode_to_vec(&report).expect("encode WeatherReport extreme cold");
    let (decoded, consumed): (WeatherReport, usize) =
        decode_from_slice(&bytes).expect("decode WeatherReport extreme cold");
    assert_eq!(report, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal extreme cold report encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Vec<WeatherReport> — batch of mixed reports roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_weather_reports_roundtrip() {
    let reports: Vec<WeatherReport> = vec![
        make_report(
            make_station(1, "Station Alpha", 51.5074, -0.1278, 11),
            1_700_000_001,
            WeatherCondition::Sunny,
            make_measurement(
                18.0,
                60.0,
                1012.0,
                3.0,
                WindDirection::NE,
                PrecipitationType::None,
                0.0,
            ),
            Some(WeatherCondition::PartlyCloudy),
        ),
        make_report(
            make_station(2, "Station Beta", 48.8566, 2.3522, 35),
            1_700_000_002,
            WeatherCondition::Rainy,
            make_measurement(
                11.0,
                88.0,
                999.0,
                8.5,
                WindDirection::NW,
                PrecipitationType::Rain,
                5.4,
            ),
            Some(WeatherCondition::Stormy),
        ),
        make_report(
            make_station(3, "Station Gamma", -33.8688, 151.2093, 39),
            1_700_000_003,
            WeatherCondition::Windy,
            make_measurement(
                25.5,
                42.0,
                1020.0,
                17.2,
                WindDirection::S,
                PrecipitationType::None,
                0.0,
            ),
            Option::None,
        ),
        make_report(
            make_station(4, "Station Delta", 35.6762, 139.6503, 40),
            1_700_000_004,
            WeatherCondition::Hail,
            make_measurement(
                8.0,
                76.0,
                990.0,
                25.0,
                WindDirection::E,
                PrecipitationType::Hail,
                18.0,
            ),
            Some(WeatherCondition::Overcast),
        ),
    ];

    let bytes = encode_to_vec(&reports).expect("encode Vec<WeatherReport>");
    let (decoded, consumed): (Vec<WeatherReport>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<WeatherReport>");
    assert_eq!(reports, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal Vec<WeatherReport> encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Big-endian config — WeatherReport roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_report_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let station = make_station(9001, "ECMWF Reading UK", 51.453_4, -0.969_4, 76);
    let measurement = make_measurement(
        15.3,
        72.0,
        1005.5,
        11.2,
        WindDirection::SW,
        PrecipitationType::Rain,
        3.7,
    );
    let report = make_report(
        station,
        1_740_000_000,
        WeatherCondition::Rainy,
        measurement,
        Some(WeatherCondition::Overcast),
    );
    let bytes = encode_to_vec_with_config(&report, cfg).expect("encode big-endian WeatherReport");
    let (decoded, consumed): (WeatherReport, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big-endian WeatherReport");
    assert_eq!(report, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal big-endian WeatherReport encoding"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Fixed-int config — WeatherReport roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_report_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let station = make_station(u32::MAX, "Boundary Station MAX", 90.0, 180.0, i32::MAX);
    let measurement = make_measurement(
        -273.15,
        100.0,
        870.0,
        0.0,
        WindDirection::N,
        PrecipitationType::Snow,
        999.9,
    );
    let report = make_report(
        station,
        u64::MAX,
        WeatherCondition::Snowy,
        measurement,
        Some(WeatherCondition::Tornado),
    );
    let bytes = encode_to_vec_with_config(&report, cfg).expect("encode fixed-int WeatherReport");
    let (decoded, consumed): (WeatherReport, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed-int WeatherReport");
    assert_eq!(report, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal fixed-int WeatherReport encoding"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Big-endian + fixed-int config — WeatherReport roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_report_big_endian_fixed_int_combined_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let station = make_station(1234, "Combo Config Station", -23.550_5, -46.633_3, 760);
    let measurement = make_measurement(
        30.0,
        65.0,
        1008.0,
        6.0,
        WindDirection::NE,
        PrecipitationType::Drizzle,
        0.5,
    );
    let report = make_report(
        station,
        1_750_000_000,
        WeatherCondition::PartlyCloudy,
        measurement,
        Some(WeatherCondition::Sunny),
    );
    let bytes =
        encode_to_vec_with_config(&report, cfg).expect("encode big-endian+fixed WeatherReport");
    let (decoded, consumed): (WeatherReport, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big-endian+fixed WeatherReport");
    assert_eq!(report, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal big-endian+fixed WeatherReport encoding"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Consumed bytes accuracy — sequential decoding from concatenated buffer
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_bytes_accuracy_sequential_weather_reports() {
    let station1 = make_station(10, "Station One", 52.0, 4.0, 5);
    let station2 = make_station(20, "Station Two", 45.0, 9.0, 200);
    let station3 = make_station(30, "Station Three", 64.0, 26.0, 78);

    let report1 = make_report(
        station1,
        2_000_000_001,
        WeatherCondition::Sunny,
        make_measurement(
            20.0,
            50.0,
            1015.0,
            2.0,
            WindDirection::E,
            PrecipitationType::None,
            0.0,
        ),
        Some(WeatherCondition::Windy),
    );
    let report2 = make_report(
        station2,
        2_000_000_002,
        WeatherCondition::Stormy,
        make_measurement(
            5.0,
            95.0,
            970.0,
            30.0,
            WindDirection::NW,
            PrecipitationType::Rain,
            22.0,
        ),
        Option::None,
    );
    let report3 = make_report(
        station3,
        2_000_000_003,
        WeatherCondition::Overcast,
        make_measurement(
            8.0,
            80.0,
            1000.0,
            5.5,
            WindDirection::S,
            PrecipitationType::Drizzle,
            1.2,
        ),
        Some(WeatherCondition::Rainy),
    );

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend(encode_to_vec(&report1).expect("encode report1"));
    buffer.extend(encode_to_vec(&report2).expect("encode report2"));
    buffer.extend(encode_to_vec(&report3).expect("encode report3"));

    let (decoded1, consumed1): (WeatherReport, usize) =
        decode_from_slice(&buffer).expect("decode report1");
    assert_eq!(report1, decoded1);

    let (decoded2, consumed2): (WeatherReport, usize) =
        decode_from_slice(&buffer[consumed1..]).expect("decode report2");
    assert_eq!(report2, decoded2);

    let (decoded3, consumed3): (WeatherReport, usize) =
        decode_from_slice(&buffer[consumed1 + consumed2..]).expect("decode report3");
    assert_eq!(report3, decoded3);

    assert_eq!(
        consumed1 + consumed2 + consumed3,
        buffer.len(),
        "sum of consumed bytes must equal total buffer length"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Option<WeatherCondition> — Some and None variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_weather_condition_some_none_roundtrip() {
    let some_sunny: Option<WeatherCondition> = Some(WeatherCondition::Sunny);
    let some_tornado: Option<WeatherCondition> = Some(WeatherCondition::Tornado);
    let none_forecast: Option<WeatherCondition> = Option::None;

    for opt in &[some_sunny, some_tornado, none_forecast] {
        let bytes = encode_to_vec(opt).expect("encode Option<WeatherCondition>");
        let (decoded, consumed): (Option<WeatherCondition>, usize) =
            decode_from_slice(&bytes).expect("decode Option<WeatherCondition>");
        assert_eq!(opt, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal Option<WeatherCondition> encoding length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 21: WeatherReport — hail storm with all Southern Hemisphere coords
// ---------------------------------------------------------------------------

#[test]
fn test_weather_report_hailstorm_southern_hemisphere_roundtrip() {
    let station = make_station(8080, "Buenos Aires Observatory", -34.603_7, -58.381_6, 25);
    let measurement = make_measurement(
        19.5,
        84.0,
        992.0,
        38.0,
        WindDirection::SE,
        PrecipitationType::Hail,
        28.3,
    );
    let report = make_report(
        station,
        1_760_000_000,
        WeatherCondition::Hail,
        measurement,
        Some(WeatherCondition::Stormy),
    );
    let bytes = encode_to_vec(&report).expect("encode WeatherReport hailstorm southern");
    let (decoded, consumed): (WeatherReport, usize) =
        decode_from_slice(&bytes).expect("decode WeatherReport hailstorm southern");
    assert_eq!(report, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal hailstorm southern hemisphere encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 22: WeatherMeasurement — boundary values across all float fields roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_weather_measurement_boundary_float_values_roundtrip() {
    // Use finite extreme values to avoid NaN equality issues
    let measurements = vec![
        make_measurement(
            f32::MIN,
            0.0,
            f32::MIN_POSITIVE,
            0.0,
            WindDirection::NW,
            PrecipitationType::Snow,
            0.0,
        ),
        make_measurement(
            f32::MAX,
            100.0,
            1100.0,
            f32::MAX,
            WindDirection::SE,
            PrecipitationType::Rain,
            f32::MAX,
        ),
        make_measurement(
            0.0,
            0.0,
            0.0,
            0.0,
            WindDirection::N,
            PrecipitationType::None,
            0.0,
        ),
        make_measurement(
            -40.0,
            50.0,
            1013.25,
            10.8,
            WindDirection::SW,
            PrecipitationType::Sleet,
            7.7,
        ),
    ];

    for m in &measurements {
        let bytes = encode_to_vec(m).expect("encode boundary WeatherMeasurement");
        let (decoded, consumed): (WeatherMeasurement, usize) =
            decode_from_slice(&bytes).expect("decode boundary WeatherMeasurement");
        assert_eq!(m, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal boundary WeatherMeasurement encoding length"
        );
    }
}
