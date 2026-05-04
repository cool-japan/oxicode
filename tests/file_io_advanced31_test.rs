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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

fn tmp(name: impl AsRef<str>) -> std::path::PathBuf {
    temp_dir().join(format!("{}_{}", name.as_ref(), std::process::id()))
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CropType {
    Wheat,
    Corn,
    Soybean,
    Rice,
    Barley,
    Sunflower,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SoilType {
    Sandy,
    Loamy,
    Clay,
    Silty,
    Peaty,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IrrigationMethod {
    Drip,
    Sprinkler,
    Flood,
    Subsurface,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PestCategory {
    Insect,
    Fungus,
    Weed,
    Virus,
    Nematode,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FieldSensor {
    sensor_id: u32,
    field_id: u32,
    lat_x1e6: i32,
    lon_x1e6: i32,
    depth_cm: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SoilReading {
    sensor_id: u32,
    timestamp: u64,
    moisture_pct: u8,
    temperature_c_x10: i16,
    ph_x100: u16,
    nitrogen_ppm: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CropField {
    field_id: u32,
    area_m2: u32,
    crop_type: CropType,
    soil_type: SoilType,
    planted_date: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IrrigationEvent {
    field_id: u32,
    start_time: u64,
    duration_min: u32,
    volume_liters: u32,
    method: IrrigationMethod,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PestAlert {
    field_id: u32,
    timestamp: u64,
    category: PestCategory,
    severity: u8,
    treatment: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HarvestRecord {
    field_id: u32,
    harvest_date: u64,
    yield_kg: u32,
    moisture_content_pct: u8,
    quality_score: u8,
}

#[test]
fn test_field_sensor_roundtrip_file() {
    let sensor = FieldSensor {
        sensor_id: 1001,
        field_id: 42,
        lat_x1e6: 48_853_400,
        lon_x1e6: 2_348_800,
        depth_cm: 30,
    };
    let path = tmp("field_sensor_31.bin");
    encode_to_file(&sensor, &path).expect("encode_to_file failed for FieldSensor");
    let decoded: FieldSensor =
        decode_from_file(&path).expect("decode_from_file failed for FieldSensor");
    assert_eq!(sensor, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for field_sensor_31.bin");
}

#[test]
fn test_soil_reading_roundtrip_file() {
    let reading = SoilReading {
        sensor_id: 2002,
        timestamp: 1_700_000_000,
        moisture_pct: 65,
        temperature_c_x10: 225,
        ph_x100: 680,
        nitrogen_ppm: 120,
    };
    let path = tmp("soil_reading_31.bin");
    encode_to_file(&reading, &path).expect("encode_to_file failed for SoilReading");
    let decoded: SoilReading =
        decode_from_file(&path).expect("decode_from_file failed for SoilReading");
    assert_eq!(reading, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for soil_reading_31.bin");
}

#[test]
fn test_crop_field_wheat_loamy_roundtrip_file() {
    let field = CropField {
        field_id: 101,
        area_m2: 50_000,
        crop_type: CropType::Wheat,
        soil_type: SoilType::Loamy,
        planted_date: 1_680_000_000,
    };
    let path = tmp("crop_field_wheat_31.bin");
    encode_to_file(&field, &path).expect("encode_to_file failed for CropField Wheat");
    let decoded: CropField =
        decode_from_file(&path).expect("decode_from_file failed for CropField Wheat");
    assert_eq!(field, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for crop_field_wheat_31.bin");
}

#[test]
fn test_crop_field_rice_clay_roundtrip_file() {
    let field = CropField {
        field_id: 202,
        area_m2: 120_000,
        crop_type: CropType::Rice,
        soil_type: SoilType::Clay,
        planted_date: 1_685_000_000,
    };
    let path = tmp("crop_field_rice_31.bin");
    encode_to_file(&field, &path).expect("encode_to_file failed for CropField Rice");
    let decoded: CropField =
        decode_from_file(&path).expect("decode_from_file failed for CropField Rice");
    assert_eq!(field, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for crop_field_rice_31.bin");
}

#[test]
fn test_irrigation_event_drip_roundtrip_file() {
    let event = IrrigationEvent {
        field_id: 301,
        start_time: 1_700_100_000,
        duration_min: 90,
        volume_liters: 15_000,
        method: IrrigationMethod::Drip,
    };
    let path = tmp("irrigation_drip_31.bin");
    encode_to_file(&event, &path).expect("encode_to_file failed for IrrigationEvent Drip");
    let decoded: IrrigationEvent =
        decode_from_file(&path).expect("decode_from_file failed for IrrigationEvent Drip");
    assert_eq!(event, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for irrigation_drip_31.bin");
}

#[test]
fn test_irrigation_event_flood_roundtrip_file() {
    let event = IrrigationEvent {
        field_id: 302,
        start_time: 1_700_200_000,
        duration_min: 240,
        volume_liters: 80_000,
        method: IrrigationMethod::Flood,
    };
    let path = tmp("irrigation_flood_31.bin");
    encode_to_file(&event, &path).expect("encode_to_file failed for IrrigationEvent Flood");
    let decoded: IrrigationEvent =
        decode_from_file(&path).expect("decode_from_file failed for IrrigationEvent Flood");
    assert_eq!(event, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for irrigation_flood_31.bin");
}

#[test]
fn test_pest_alert_with_treatment_roundtrip_file() {
    let alert = PestAlert {
        field_id: 401,
        timestamp: 1_700_300_000,
        category: PestCategory::Fungus,
        severity: 7,
        treatment: Some("Apply fungicide at 2L/ha".to_string()),
    };
    let path = tmp("pest_alert_treatment_31.bin");
    encode_to_file(&alert, &path).expect("encode_to_file failed for PestAlert with treatment");
    let decoded: PestAlert =
        decode_from_file(&path).expect("decode_from_file failed for PestAlert with treatment");
    assert_eq!(alert, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for pest_alert_treatment_31.bin");
}

#[test]
fn test_pest_alert_no_treatment_roundtrip_file() {
    let alert = PestAlert {
        field_id: 402,
        timestamp: 1_700_350_000,
        category: PestCategory::Insect,
        severity: 3,
        treatment: None,
    };
    let path = tmp("pest_alert_no_treatment_31.bin");
    encode_to_file(&alert, &path).expect("encode_to_file failed for PestAlert no treatment");
    let decoded: PestAlert =
        decode_from_file(&path).expect("decode_from_file failed for PestAlert no treatment");
    assert_eq!(alert, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for pest_alert_no_treatment_31.bin");
}

#[test]
fn test_harvest_record_roundtrip_file() {
    let record = HarvestRecord {
        field_id: 501,
        harvest_date: 1_725_000_000,
        yield_kg: 42_000,
        moisture_content_pct: 14,
        quality_score: 92,
    };
    let path = tmp("harvest_record_31.bin");
    encode_to_file(&record, &path).expect("encode_to_file failed for HarvestRecord");
    let decoded: HarvestRecord =
        decode_from_file(&path).expect("decode_from_file failed for HarvestRecord");
    assert_eq!(record, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for harvest_record_31.bin");
}

#[test]
fn test_large_soil_reading_set_roundtrip_file() {
    let readings: Vec<SoilReading> = (0u32..512)
        .map(|i| SoilReading {
            sensor_id: 3000 + i,
            timestamp: 1_700_000_000 + (i as u64) * 300,
            moisture_pct: (30 + (i % 60)) as u8,
            temperature_c_x10: (180 + (i % 80) as i16),
            ph_x100: (620 + (i % 160)) as u16,
            nitrogen_ppm: (80 + (i % 200)) as u16,
        })
        .collect();
    let path = tmp("large_soil_readings_31.bin");
    encode_to_file(&readings, &path).expect("encode_to_file failed for large soil readings");
    let decoded: Vec<SoilReading> =
        decode_from_file(&path).expect("decode_from_file failed for large soil readings");
    assert_eq!(readings.len(), decoded.len());
    assert_eq!(readings, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for large_soil_readings_31.bin");
}

#[test]
fn test_vec_of_crop_fields_roundtrip_file() {
    let fields = vec![
        CropField {
            field_id: 601,
            area_m2: 30_000,
            crop_type: CropType::Corn,
            soil_type: SoilType::Sandy,
            planted_date: 1_682_000_000,
        },
        CropField {
            field_id: 602,
            area_m2: 75_000,
            crop_type: CropType::Soybean,
            soil_type: SoilType::Silty,
            planted_date: 1_683_000_000,
        },
        CropField {
            field_id: 603,
            area_m2: 20_000,
            crop_type: CropType::Barley,
            soil_type: SoilType::Peaty,
            planted_date: 1_684_000_000,
        },
        CropField {
            field_id: 604,
            area_m2: 95_000,
            crop_type: CropType::Sunflower,
            soil_type: SoilType::Loamy,
            planted_date: 1_685_000_000,
        },
    ];
    let path = tmp("vec_crop_fields_31.bin");
    encode_to_file(&fields, &path).expect("encode_to_file failed for vec of CropFields");
    let decoded: Vec<CropField> =
        decode_from_file(&path).expect("decode_from_file failed for vec of CropFields");
    assert_eq!(fields, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for vec_crop_fields_31.bin");
}

#[test]
fn test_option_treatment_none_vs_some_roundtrip() {
    let alert_none = PestAlert {
        field_id: 701,
        timestamp: 1_700_400_000,
        category: PestCategory::Weed,
        severity: 5,
        treatment: None,
    };
    let alert_some = PestAlert {
        field_id: 702,
        timestamp: 1_700_450_000,
        category: PestCategory::Nematode,
        severity: 9,
        treatment: Some("Soil fumigation required immediately".to_string()),
    };
    let path_none = tmp("pest_none_option_31.bin");
    let path_some = tmp("pest_some_option_31.bin");

    encode_to_file(&alert_none, &path_none)
        .expect("encode_to_file failed for PestAlert None option");
    encode_to_file(&alert_some, &path_some)
        .expect("encode_to_file failed for PestAlert Some option");

    let decoded_none: PestAlert =
        decode_from_file(&path_none).expect("decode_from_file failed for PestAlert None option");
    let decoded_some: PestAlert =
        decode_from_file(&path_some).expect("decode_from_file failed for PestAlert Some option");

    assert_eq!(alert_none, decoded_none);
    assert_eq!(alert_some, decoded_some);
    assert!(decoded_none.treatment.is_none());
    assert!(decoded_some.treatment.is_some());

    std::fs::remove_file(&path_none).expect("cleanup failed for pest_none_option_31.bin");
    std::fs::remove_file(&path_some).expect("cleanup failed for pest_some_option_31.bin");
}

#[test]
fn test_overwrite_existing_file() {
    let path = tmp("overwrite_field_31.bin");

    let field_v1 = CropField {
        field_id: 801,
        area_m2: 10_000,
        crop_type: CropType::Wheat,
        soil_type: SoilType::Sandy,
        planted_date: 1_680_000_000,
    };
    encode_to_file(&field_v1, &path).expect("encode_to_file failed on first write");

    let field_v2 = CropField {
        field_id: 802,
        area_m2: 25_000,
        crop_type: CropType::Sunflower,
        soil_type: SoilType::Peaty,
        planted_date: 1_690_000_000,
    };
    encode_to_file(&field_v2, &path).expect("encode_to_file failed on overwrite");

    let decoded: CropField =
        decode_from_file(&path).expect("decode_from_file failed after overwrite");
    assert_eq!(field_v2, decoded);
    assert_ne!(field_v1, decoded);

    std::fs::remove_file(&path).expect("cleanup failed for overwrite_field_31.bin");
}

#[test]
fn test_error_on_missing_file() {
    let path = tmp("nonexistent_agriculture_31.bin");
    let result: Result<CropField, _> = decode_from_file(&path);
    assert!(
        result.is_err(),
        "Expected error when decoding from missing file"
    );
}

#[test]
fn test_bytes_match_between_file_and_encode_to_vec() {
    let sensor = FieldSensor {
        sensor_id: 9001,
        field_id: 55,
        lat_x1e6: -33_868_820,
        lon_x1e6: 151_209_290,
        depth_cm: 20,
    };
    let path = tmp("bytes_match_sensor_31.bin");
    encode_to_file(&sensor, &path).expect("encode_to_file failed for bytes match test");

    let file_bytes = std::fs::read(&path).expect("fs::read failed for bytes match test");
    let vec_bytes = encode_to_vec(&sensor).expect("encode_to_vec failed for bytes match test");

    assert_eq!(
        file_bytes, vec_bytes,
        "File bytes must match encode_to_vec bytes"
    );
    std::fs::remove_file(&path).expect("cleanup failed for bytes_match_sensor_31.bin");
}

#[test]
fn test_all_crop_types_roundtrip_vec() {
    let crop_types = vec![
        CropType::Wheat,
        CropType::Corn,
        CropType::Soybean,
        CropType::Rice,
        CropType::Barley,
        CropType::Sunflower,
    ];
    let bytes = encode_to_vec(&crop_types).expect("encode_to_vec failed for all CropTypes");
    let (decoded, consumed) = decode_from_slice::<Vec<CropType>>(&bytes)
        .expect("decode_from_slice failed for all CropTypes");
    assert_eq!(crop_types, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_all_soil_types_roundtrip_vec() {
    let soil_types = vec![
        SoilType::Sandy,
        SoilType::Loamy,
        SoilType::Clay,
        SoilType::Silty,
        SoilType::Peaty,
    ];
    let bytes = encode_to_vec(&soil_types).expect("encode_to_vec failed for all SoilTypes");
    let (decoded, consumed) = decode_from_slice::<Vec<SoilType>>(&bytes)
        .expect("decode_from_slice failed for all SoilTypes");
    assert_eq!(soil_types, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_all_irrigation_methods_roundtrip_vec() {
    let methods = vec![
        IrrigationMethod::Drip,
        IrrigationMethod::Sprinkler,
        IrrigationMethod::Flood,
        IrrigationMethod::Subsurface,
    ];
    let bytes = encode_to_vec(&methods).expect("encode_to_vec failed for all IrrigationMethods");
    let (decoded, consumed) = decode_from_slice::<Vec<IrrigationMethod>>(&bytes)
        .expect("decode_from_slice failed for all IrrigationMethods");
    assert_eq!(methods, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_all_pest_categories_roundtrip_vec() {
    let categories = vec![
        PestCategory::Insect,
        PestCategory::Fungus,
        PestCategory::Weed,
        PestCategory::Virus,
        PestCategory::Nematode,
    ];
    let bytes = encode_to_vec(&categories).expect("encode_to_vec failed for all PestCategories");
    let (decoded, consumed) = decode_from_slice::<Vec<PestCategory>>(&bytes)
        .expect("decode_from_slice failed for all PestCategories");
    assert_eq!(categories, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_irrigation_subsurface_sprinkler_roundtrip_file() {
    let events = vec![
        IrrigationEvent {
            field_id: 1001,
            start_time: 1_700_500_000,
            duration_min: 45,
            volume_liters: 6_000,
            method: IrrigationMethod::Subsurface,
        },
        IrrigationEvent {
            field_id: 1002,
            start_time: 1_700_600_000,
            duration_min: 60,
            volume_liters: 9_000,
            method: IrrigationMethod::Sprinkler,
        },
    ];
    let path = tmp("irrigation_sub_sprinkler_31.bin");
    encode_to_file(&events, &path)
        .expect("encode_to_file failed for Subsurface/Sprinkler IrrigationEvents");
    let decoded: Vec<IrrigationEvent> = decode_from_file(&path)
        .expect("decode_from_file failed for Subsurface/Sprinkler IrrigationEvents");
    assert_eq!(events, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for irrigation_sub_sprinkler_31.bin");
}

#[test]
fn test_harvest_record_extreme_values_roundtrip_file() {
    let record = HarvestRecord {
        field_id: u32::MAX,
        harvest_date: u64::MAX,
        yield_kg: u32::MAX,
        moisture_content_pct: u8::MAX,
        quality_score: u8::MAX,
    };
    let path = tmp("harvest_extreme_31.bin");
    encode_to_file(&record, &path).expect("encode_to_file failed for HarvestRecord extreme values");
    let decoded: HarvestRecord =
        decode_from_file(&path).expect("decode_from_file failed for HarvestRecord extreme values");
    assert_eq!(record, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for harvest_extreme_31.bin");
}

#[test]
fn test_field_sensor_negative_coordinates_roundtrip_file() {
    let sensor = FieldSensor {
        sensor_id: 5555,
        field_id: 777,
        lat_x1e6: -34_603_480,
        lon_x1e6: -58_381_940,
        depth_cm: 60,
    };
    let path = tmp("sensor_negative_coords_31.bin");
    encode_to_file(&sensor, &path)
        .expect("encode_to_file failed for FieldSensor negative coordinates");
    let decoded: FieldSensor = decode_from_file(&path)
        .expect("decode_from_file failed for FieldSensor negative coordinates");
    assert_eq!(sensor, decoded);
    std::fs::remove_file(&path).expect("cleanup failed for sensor_negative_coords_31.bin");
}

#[test]
fn test_pest_alert_virus_high_severity_roundtrip_file() {
    let alert = PestAlert {
        field_id: 9901,
        timestamp: 1_720_000_000,
        category: PestCategory::Virus,
        severity: 10,
        treatment: Some("Quarantine and destroy affected crop immediately".to_string()),
    };
    let path = tmp("pest_virus_severe_31.bin");
    encode_to_file(&alert, &path).expect("encode_to_file failed for virus PestAlert");
    let decoded: PestAlert =
        decode_from_file(&path).expect("decode_from_file failed for virus PestAlert");
    assert_eq!(alert, decoded);
    assert_eq!(decoded.severity, 10);
    assert_eq!(decoded.category, PestCategory::Virus);
    std::fs::remove_file(&path).expect("cleanup failed for pest_virus_severe_31.bin");
}
