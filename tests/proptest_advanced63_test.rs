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
struct WeatherStation {
    id: u32,
    lat: i32,
    lon: i32,
    elevation_m: i16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherReading {
    station_id: u32,
    temp_c_x10: i32,
    humidity_pct: u8,
    pressure_hpa_x10: u32,
    wind_speed_ms_x10: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ForecastCell {
    grid_x: u16,
    grid_y: u16,
    precip_mm_x10: u32,
    cloud_cover_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClimateRecord {
    year: u16,
    month: u8,
    avg_temp_x10: i32,
    total_precip_x10: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WeatherEvent {
    Clear,
    Cloudy,
    Rain,
    Snow,
    Thunderstorm,
    Fog,
}

proptest! {
    #[test]
    fn test_weather_station_roundtrip(
        id in 0u32..=u32::MAX,
        lat in (-90_000_000i32..=90_000_000i32),
        lon in (-180_000_000i32..=180_000_000i32),
        elevation_m in i16::MIN..=i16::MAX,
    ) {
        let station = WeatherStation { id, lat, lon, elevation_m };
        let encoded = encode_to_vec(&station).expect("encode WeatherStation");
        let (decoded, _): (WeatherStation, usize) = decode_from_slice(&encoded).expect("decode WeatherStation");
        prop_assert_eq!(station, decoded);
    }

    #[test]
    fn test_weather_reading_roundtrip(
        station_id in 0u32..=u32::MAX,
        temp_c_x10 in (-600i32..=600i32),
        humidity_pct in 0u8..=100u8,
        pressure_hpa_x10 in 8000u32..=12000u32,
        wind_speed_ms_x10 in 0u16..=1000u16,
    ) {
        let reading = WeatherReading {
            station_id,
            temp_c_x10,
            humidity_pct,
            pressure_hpa_x10,
            wind_speed_ms_x10,
        };
        let encoded = encode_to_vec(&reading).expect("encode WeatherReading");
        let (decoded, _): (WeatherReading, usize) = decode_from_slice(&encoded).expect("decode WeatherReading");
        prop_assert_eq!(reading, decoded);
    }

    #[test]
    fn test_forecast_cell_roundtrip(
        grid_x in 0u16..=u16::MAX,
        grid_y in 0u16..=u16::MAX,
        precip_mm_x10 in 0u32..=50000u32,
        cloud_cover_pct in 0u8..=100u8,
    ) {
        let cell = ForecastCell { grid_x, grid_y, precip_mm_x10, cloud_cover_pct };
        let encoded = encode_to_vec(&cell).expect("encode ForecastCell");
        let (decoded, _): (ForecastCell, usize) = decode_from_slice(&encoded).expect("decode ForecastCell");
        prop_assert_eq!(cell, decoded);
    }

    #[test]
    fn test_climate_record_roundtrip(
        year in 1800u16..=2200u16,
        month in 1u8..=12u8,
        avg_temp_x10 in (-800i32..=600i32),
        total_precip_x10 in 0u32..=100000u32,
    ) {
        let record = ClimateRecord { year, month, avg_temp_x10, total_precip_x10 };
        let encoded = encode_to_vec(&record).expect("encode ClimateRecord");
        let (decoded, _): (ClimateRecord, usize) = decode_from_slice(&encoded).expect("decode ClimateRecord");
        prop_assert_eq!(record, decoded);
    }

    #[test]
    fn test_weather_event_roundtrip(event_idx in 0usize..6) {
        let events = [
            WeatherEvent::Clear,
            WeatherEvent::Cloudy,
            WeatherEvent::Rain,
            WeatherEvent::Snow,
            WeatherEvent::Thunderstorm,
            WeatherEvent::Fog,
        ];
        let event = events[event_idx].clone();
        let encoded = encode_to_vec(&event).expect("encode WeatherEvent");
        let (decoded, _): (WeatherEvent, usize) = decode_from_slice(&encoded).expect("decode WeatherEvent");
        prop_assert_eq!(event, decoded);
    }

    #[test]
    fn test_weather_station_deterministic_encoding(
        id in 0u32..=u32::MAX,
        lat in (-90_000_000i32..=90_000_000i32),
        lon in (-180_000_000i32..=180_000_000i32),
        elevation_m in i16::MIN..=i16::MAX,
    ) {
        let station = WeatherStation { id, lat, lon, elevation_m };
        let encoded1 = encode_to_vec(&station).expect("first encode WeatherStation");
        let encoded2 = encode_to_vec(&station).expect("second encode WeatherStation");
        prop_assert_eq!(encoded1, encoded2);
    }

    #[test]
    fn test_weather_reading_deterministic_encoding(
        station_id in 0u32..=u32::MAX,
        temp_c_x10 in (-600i32..=600i32),
        humidity_pct in 0u8..=100u8,
        pressure_hpa_x10 in 8000u32..=12000u32,
        wind_speed_ms_x10 in 0u16..=1000u16,
    ) {
        let reading = WeatherReading {
            station_id,
            temp_c_x10,
            humidity_pct,
            pressure_hpa_x10,
            wind_speed_ms_x10,
        };
        let encoded1 = encode_to_vec(&reading).expect("first encode WeatherReading");
        let encoded2 = encode_to_vec(&reading).expect("second encode WeatherReading");
        prop_assert_eq!(encoded1, encoded2);
    }

    #[test]
    fn test_weather_station_consumed_bytes(
        id in 0u32..=u32::MAX,
        lat in (-90_000_000i32..=90_000_000i32),
        lon in (-180_000_000i32..=180_000_000i32),
        elevation_m in i16::MIN..=i16::MAX,
    ) {
        let station = WeatherStation { id, lat, lon, elevation_m };
        let encoded = encode_to_vec(&station).expect("encode WeatherStation");
        let expected_len = encoded.len();
        let (_, consumed): (WeatherStation, usize) = decode_from_slice(&encoded).expect("decode WeatherStation");
        prop_assert_eq!(consumed, expected_len);
    }

    #[test]
    fn test_weather_reading_consumed_bytes(
        station_id in 0u32..=u32::MAX,
        temp_c_x10 in (-600i32..=600i32),
        humidity_pct in 0u8..=100u8,
        pressure_hpa_x10 in 8000u32..=12000u32,
        wind_speed_ms_x10 in 0u16..=1000u16,
    ) {
        let reading = WeatherReading {
            station_id,
            temp_c_x10,
            humidity_pct,
            pressure_hpa_x10,
            wind_speed_ms_x10,
        };
        let encoded = encode_to_vec(&reading).expect("encode WeatherReading");
        let expected_len = encoded.len();
        let (_, consumed): (WeatherReading, usize) = decode_from_slice(&encoded).expect("decode WeatherReading");
        prop_assert_eq!(consumed, expected_len);
    }

    #[test]
    fn test_forecast_cell_consumed_bytes(
        grid_x in 0u16..=u16::MAX,
        grid_y in 0u16..=u16::MAX,
        precip_mm_x10 in 0u32..=50000u32,
        cloud_cover_pct in 0u8..=100u8,
    ) {
        let cell = ForecastCell { grid_x, grid_y, precip_mm_x10, cloud_cover_pct };
        let encoded = encode_to_vec(&cell).expect("encode ForecastCell");
        let expected_len = encoded.len();
        let (_, consumed): (ForecastCell, usize) = decode_from_slice(&encoded).expect("decode ForecastCell");
        prop_assert_eq!(consumed, expected_len);
    }

    #[test]
    fn test_climate_record_consumed_bytes(
        year in 1800u16..=2200u16,
        month in 1u8..=12u8,
        avg_temp_x10 in (-800i32..=600i32),
        total_precip_x10 in 0u32..=100000u32,
    ) {
        let record = ClimateRecord { year, month, avg_temp_x10, total_precip_x10 };
        let encoded = encode_to_vec(&record).expect("encode ClimateRecord");
        let expected_len = encoded.len();
        let (_, consumed): (ClimateRecord, usize) = decode_from_slice(&encoded).expect("decode ClimateRecord");
        prop_assert_eq!(consumed, expected_len);
    }

    #[test]
    fn test_vec_weather_readings_roundtrip(
        readings in prop::collection::vec(
            (
                0u32..=9999u32,
                (-600i32..=600i32),
                0u8..=100u8,
                8000u32..=12000u32,
                0u16..=1000u16,
            ),
            0..=50,
        )
    ) {
        let readings_vec: Vec<WeatherReading> = readings
            .into_iter()
            .map(|(station_id, temp_c_x10, humidity_pct, pressure_hpa_x10, wind_speed_ms_x10)| {
                WeatherReading { station_id, temp_c_x10, humidity_pct, pressure_hpa_x10, wind_speed_ms_x10 }
            })
            .collect();
        let encoded = encode_to_vec(&readings_vec).expect("encode Vec<WeatherReading>");
        let (decoded, _): (Vec<WeatherReading>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<WeatherReading>");
        prop_assert_eq!(readings_vec, decoded);
    }

    #[test]
    fn test_vec_forecast_cells_roundtrip(
        cells in prop::collection::vec(
            (0u16..=u16::MAX, 0u16..=u16::MAX, 0u32..=50000u32, 0u8..=100u8),
            0..=50,
        )
    ) {
        let cells_vec: Vec<ForecastCell> = cells
            .into_iter()
            .map(|(grid_x, grid_y, precip_mm_x10, cloud_cover_pct)| {
                ForecastCell { grid_x, grid_y, precip_mm_x10, cloud_cover_pct }
            })
            .collect();
        let encoded = encode_to_vec(&cells_vec).expect("encode Vec<ForecastCell>");
        let (decoded, _): (Vec<ForecastCell>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<ForecastCell>");
        prop_assert_eq!(cells_vec, decoded);
    }

    #[test]
    fn test_option_weather_station_some_roundtrip(
        id in 0u32..=u32::MAX,
        lat in (-90_000_000i32..=90_000_000i32),
        lon in (-180_000_000i32..=180_000_000i32),
        elevation_m in i16::MIN..=i16::MAX,
    ) {
        let station = Some(WeatherStation { id, lat, lon, elevation_m });
        let encoded = encode_to_vec(&station).expect("encode Option<WeatherStation> Some");
        let (decoded, _): (Option<WeatherStation>, usize) =
            decode_from_slice(&encoded).expect("decode Option<WeatherStation> Some");
        prop_assert_eq!(station, decoded);
    }

    #[test]
    fn test_option_weather_station_none_roundtrip(_dummy in 0u8..=1u8) {
        let station: Option<WeatherStation> = None;
        let encoded = encode_to_vec(&station).expect("encode Option<WeatherStation> None");
        let (decoded, _): (Option<WeatherStation>, usize) =
            decode_from_slice(&encoded).expect("decode Option<WeatherStation> None");
        prop_assert_eq!(station, decoded);
    }

    #[test]
    fn test_option_climate_record_roundtrip(
        include in any::<bool>(),
        year in 1800u16..=2200u16,
        month in 1u8..=12u8,
        avg_temp_x10 in (-800i32..=600i32),
        total_precip_x10 in 0u32..=100000u32,
    ) {
        let record: Option<ClimateRecord> = if include {
            Some(ClimateRecord { year, month, avg_temp_x10, total_precip_x10 })
        } else {
            None
        };
        let encoded = encode_to_vec(&record).expect("encode Option<ClimateRecord>");
        let (decoded, _): (Option<ClimateRecord>, usize) =
            decode_from_slice(&encoded).expect("decode Option<ClimateRecord>");
        prop_assert_eq!(record, decoded);
    }

    #[test]
    fn test_large_collection_weather_stations(
        stations in prop::collection::vec(
            (
                0u32..=999999u32,
                (-90_000_000i32..=90_000_000i32),
                (-180_000_000i32..=180_000_000i32),
                i16::MIN..=i16::MAX,
            ),
            100..=200,
        )
    ) {
        let stations_vec: Vec<WeatherStation> = stations
            .into_iter()
            .map(|(id, lat, lon, elevation_m)| WeatherStation { id, lat, lon, elevation_m })
            .collect();
        let encoded = encode_to_vec(&stations_vec).expect("encode large Vec<WeatherStation>");
        let (decoded, consumed): (Vec<WeatherStation>, usize) =
            decode_from_slice(&encoded).expect("decode large Vec<WeatherStation>");
        prop_assert_eq!(&stations_vec, &decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_large_collection_climate_records(
        records in prop::collection::vec(
            (1800u16..=2200u16, 1u8..=12u8, (-800i32..=600i32), 0u32..=100000u32),
            100..=200,
        )
    ) {
        let records_vec: Vec<ClimateRecord> = records
            .into_iter()
            .map(|(year, month, avg_temp_x10, total_precip_x10)| {
                ClimateRecord { year, month, avg_temp_x10, total_precip_x10 }
            })
            .collect();
        let encoded = encode_to_vec(&records_vec).expect("encode large Vec<ClimateRecord>");
        let (decoded, consumed): (Vec<ClimateRecord>, usize) =
            decode_from_slice(&encoded).expect("decode large Vec<ClimateRecord>");
        prop_assert_eq!(&records_vec, &decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_weather_event_all_variants_consumed_bytes(event_idx in 0usize..6) {
        let events = [
            WeatherEvent::Clear,
            WeatherEvent::Cloudy,
            WeatherEvent::Rain,
            WeatherEvent::Snow,
            WeatherEvent::Thunderstorm,
            WeatherEvent::Fog,
        ];
        let event = events[event_idx].clone();
        let encoded = encode_to_vec(&event).expect("encode WeatherEvent for consumed bytes");
        let expected_len = encoded.len();
        let (_, consumed): (WeatherEvent, usize) =
            decode_from_slice(&encoded).expect("decode WeatherEvent for consumed bytes");
        prop_assert_eq!(consumed, expected_len);
    }

    #[test]
    fn test_climate_record_deterministic_encoding(
        year in 1800u16..=2200u16,
        month in 1u8..=12u8,
        avg_temp_x10 in (-800i32..=600i32),
        total_precip_x10 in 0u32..=100000u32,
    ) {
        let record = ClimateRecord { year, month, avg_temp_x10, total_precip_x10 };
        let encoded1 = encode_to_vec(&record).expect("first encode ClimateRecord");
        let encoded2 = encode_to_vec(&record).expect("second encode ClimateRecord");
        prop_assert_eq!(encoded1, encoded2);
    }

    #[test]
    fn test_forecast_cell_deterministic_encoding(
        grid_x in 0u16..=u16::MAX,
        grid_y in 0u16..=u16::MAX,
        precip_mm_x10 in 0u32..=50000u32,
        cloud_cover_pct in 0u8..=100u8,
    ) {
        let cell = ForecastCell { grid_x, grid_y, precip_mm_x10, cloud_cover_pct };
        let encoded1 = encode_to_vec(&cell).expect("first encode ForecastCell");
        let encoded2 = encode_to_vec(&cell).expect("second encode ForecastCell");
        prop_assert_eq!(encoded1, encoded2);
    }

    #[test]
    fn test_vec_weather_events_roundtrip(
        event_indices in prop::collection::vec(0usize..6, 0..=30)
    ) {
        let all_events = [
            WeatherEvent::Clear,
            WeatherEvent::Cloudy,
            WeatherEvent::Rain,
            WeatherEvent::Snow,
            WeatherEvent::Thunderstorm,
            WeatherEvent::Fog,
        ];
        let events_vec: Vec<WeatherEvent> = event_indices
            .iter()
            .map(|&i| all_events[i].clone())
            .collect();
        let encoded = encode_to_vec(&events_vec).expect("encode Vec<WeatherEvent>");
        let (decoded, consumed): (Vec<WeatherEvent>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<WeatherEvent>");
        prop_assert_eq!(&events_vec, &decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}
