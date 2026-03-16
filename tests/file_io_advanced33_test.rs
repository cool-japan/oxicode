//! Advanced file I/O tests for OxiCode — water treatment / municipal utilities / environmental monitoring domain.

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

// ── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WaterSource {
    Surface,
    Groundwater,
    Desalination,
    Recycled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TreatmentStage {
    Coagulation,
    Sedimentation,
    Filtration,
    Disinfection,
    Softening,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ContaminantType {
    Bacteria,
    Virus,
    Chemical,
    HeavyMetal,
    Nitrate,
    Microplastic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaterQualityReading {
    sensor_id: u32,
    timestamp: u64,
    ph_x100: u16,
    turbidity_ntu_x100: u32,
    chlorine_mg_l_x1000: u32,
    tds_ppm: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TreatmentUnit {
    unit_id: u32,
    stage: TreatmentStage,
    source: WaterSource,
    capacity_ml_per_day: u64,
    efficiency_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ContaminantDetection {
    detection_id: u64,
    sensor_id: u32,
    contaminant_type: ContaminantType,
    concentration_ppb_x100: u32,
    timestamp: u64,
    alert_level: AlertSeverity,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PumpStation {
    station_id: u32,
    name: String,
    flow_rate_lps_x100: u32,
    pressure_kpa_x10: u32,
    running: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaintenanceLog {
    log_id: u64,
    unit_id: u32,
    timestamp: u64,
    description: String,
    next_due: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkPipe {
    pipe_id: u32,
    diameter_mm: u16,
    length_m: u32,
    material: String,
    installation_year: u16,
}

// ── Helper constructors ───────────────────────────────────────────────────────

fn sample_quality_reading(sensor_id: u32) -> WaterQualityReading {
    WaterQualityReading {
        sensor_id,
        timestamp: 1_700_000_000 + sensor_id as u64,
        ph_x100: 740,
        turbidity_ntu_x100: 85,
        chlorine_mg_l_x1000: 500,
        tds_ppm: 320,
    }
}

fn sample_treatment_unit(unit_id: u32) -> TreatmentUnit {
    TreatmentUnit {
        unit_id,
        stage: TreatmentStage::Filtration,
        source: WaterSource::Surface,
        capacity_ml_per_day: 50_000_000,
        efficiency_pct: 97,
    }
}

fn sample_contaminant_detection(detection_id: u64) -> ContaminantDetection {
    ContaminantDetection {
        detection_id,
        sensor_id: 42,
        contaminant_type: ContaminantType::Bacteria,
        concentration_ppb_x100: 150,
        timestamp: 1_700_100_000,
        alert_level: AlertSeverity::Warning,
    }
}

fn sample_pump_station(station_id: u32) -> PumpStation {
    PumpStation {
        station_id,
        name: format!("Pump Station {station_id}"),
        flow_rate_lps_x100: 2500,
        pressure_kpa_x10: 4800,
        running: true,
    }
}

fn sample_maintenance_log(log_id: u64, next_due: Option<u64>) -> MaintenanceLog {
    MaintenanceLog {
        log_id,
        unit_id: 7,
        timestamp: 1_699_900_000,
        description: "Replaced filter media; calibrated pH probe.".to_string(),
        next_due,
    }
}

fn sample_network_pipe(pipe_id: u32) -> NetworkPipe {
    NetworkPipe {
        pipe_id,
        diameter_mm: 300,
        length_m: 1_200,
        material: "Ductile Iron".to_string(),
        installation_year: 1998,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn test_water_quality_reading_file_roundtrip() {
    let reading = sample_quality_reading(1);
    let path = temp_dir().join("oxicode_wt33_quality_reading.bin");

    encode_to_file(&reading, &path).expect("encode WaterQualityReading to file");
    let decoded: WaterQualityReading =
        decode_from_file(&path).expect("decode WaterQualityReading from file");

    assert_eq!(reading, decoded);
    std::fs::remove_file(&path).expect("cleanup WaterQualityReading file");
}

#[test]
fn test_treatment_unit_file_roundtrip() {
    let unit = sample_treatment_unit(10);
    let path = temp_dir().join("oxicode_wt33_treatment_unit.bin");

    encode_to_file(&unit, &path).expect("encode TreatmentUnit to file");
    let decoded: TreatmentUnit = decode_from_file(&path).expect("decode TreatmentUnit from file");

    assert_eq!(unit, decoded);
    std::fs::remove_file(&path).expect("cleanup TreatmentUnit file");
}

#[test]
fn test_contaminant_detection_file_roundtrip() {
    let detection = sample_contaminant_detection(9_001);
    let path = temp_dir().join("oxicode_wt33_contaminant_detection.bin");

    encode_to_file(&detection, &path).expect("encode ContaminantDetection to file");
    let decoded: ContaminantDetection =
        decode_from_file(&path).expect("decode ContaminantDetection from file");

    assert_eq!(detection, decoded);
    std::fs::remove_file(&path).expect("cleanup ContaminantDetection file");
}

#[test]
fn test_pump_station_file_roundtrip() {
    let station = sample_pump_station(3);
    let path = temp_dir().join("oxicode_wt33_pump_station.bin");

    encode_to_file(&station, &path).expect("encode PumpStation to file");
    let decoded: PumpStation = decode_from_file(&path).expect("decode PumpStation from file");

    assert_eq!(station, decoded);
    std::fs::remove_file(&path).expect("cleanup PumpStation file");
}

#[test]
fn test_maintenance_log_with_next_due_some() {
    let log = sample_maintenance_log(101, Some(1_702_000_000));
    let path = temp_dir().join("oxicode_wt33_maintenance_log_some.bin");

    encode_to_file(&log, &path).expect("encode MaintenanceLog (Some) to file");
    let decoded: MaintenanceLog =
        decode_from_file(&path).expect("decode MaintenanceLog (Some) from file");

    assert_eq!(log, decoded);
    assert_eq!(decoded.next_due, Some(1_702_000_000));
    std::fs::remove_file(&path).expect("cleanup MaintenanceLog Some file");
}

#[test]
fn test_maintenance_log_with_next_due_none() {
    let log = sample_maintenance_log(202, None);
    let path = temp_dir().join("oxicode_wt33_maintenance_log_none.bin");

    encode_to_file(&log, &path).expect("encode MaintenanceLog (None) to file");
    let decoded: MaintenanceLog =
        decode_from_file(&path).expect("decode MaintenanceLog (None) from file");

    assert_eq!(log, decoded);
    assert!(decoded.next_due.is_none());
    std::fs::remove_file(&path).expect("cleanup MaintenanceLog None file");
}

#[test]
fn test_network_pipe_file_roundtrip() {
    let pipe = sample_network_pipe(555);
    let path = temp_dir().join("oxicode_wt33_network_pipe.bin");

    encode_to_file(&pipe, &path).expect("encode NetworkPipe to file");
    let decoded: NetworkPipe = decode_from_file(&path).expect("decode NetworkPipe from file");

    assert_eq!(pipe, decoded);
    std::fs::remove_file(&path).expect("cleanup NetworkPipe file");
}

#[test]
fn test_large_quality_reading_set_500() {
    let readings: Vec<WaterQualityReading> = (0u32..500)
        .map(|i| WaterQualityReading {
            sensor_id: i,
            timestamp: 1_700_000_000 + i as u64 * 60,
            ph_x100: 700 + (i % 100) as u16,
            turbidity_ntu_x100: 50 + i * 2,
            chlorine_mg_l_x1000: 400 + (i % 200),
            tds_ppm: 200 + i,
        })
        .collect();

    let path = temp_dir().join("oxicode_wt33_large_readings_500.bin");

    encode_to_file(&readings, &path).expect("encode 500 WaterQualityReadings to file");
    let decoded: Vec<WaterQualityReading> =
        decode_from_file(&path).expect("decode 500 WaterQualityReadings from file");

    assert_eq!(readings.len(), decoded.len());
    assert_eq!(readings, decoded);
    std::fs::remove_file(&path).expect("cleanup large readings file");
}

#[test]
fn test_vec_treatment_units_roundtrip() {
    let units: Vec<TreatmentUnit> = vec![
        TreatmentUnit {
            unit_id: 1,
            stage: TreatmentStage::Coagulation,
            source: WaterSource::Surface,
            capacity_ml_per_day: 10_000_000,
            efficiency_pct: 88,
        },
        TreatmentUnit {
            unit_id: 2,
            stage: TreatmentStage::Sedimentation,
            source: WaterSource::Groundwater,
            capacity_ml_per_day: 20_000_000,
            efficiency_pct: 93,
        },
        TreatmentUnit {
            unit_id: 3,
            stage: TreatmentStage::Disinfection,
            source: WaterSource::Desalination,
            capacity_ml_per_day: 30_000_000,
            efficiency_pct: 99,
        },
        TreatmentUnit {
            unit_id: 4,
            stage: TreatmentStage::Softening,
            source: WaterSource::Recycled,
            capacity_ml_per_day: 5_000_000,
            efficiency_pct: 82,
        },
    ];

    let path = temp_dir().join("oxicode_wt33_vec_treatment_units.bin");

    encode_to_file(&units, &path).expect("encode Vec<TreatmentUnit> to file");
    let decoded: Vec<TreatmentUnit> =
        decode_from_file(&path).expect("decode Vec<TreatmentUnit> from file");

    assert_eq!(units, decoded);
    std::fs::remove_file(&path).expect("cleanup Vec<TreatmentUnit> file");
}

#[test]
fn test_all_contaminant_types_vec_roundtrip() {
    let detections: Vec<ContaminantDetection> = vec![
        ContaminantDetection {
            detection_id: 1,
            sensor_id: 10,
            contaminant_type: ContaminantType::Bacteria,
            concentration_ppb_x100: 100,
            timestamp: 1_700_200_000,
            alert_level: AlertSeverity::Info,
        },
        ContaminantDetection {
            detection_id: 2,
            sensor_id: 11,
            contaminant_type: ContaminantType::Virus,
            concentration_ppb_x100: 200,
            timestamp: 1_700_200_060,
            alert_level: AlertSeverity::Warning,
        },
        ContaminantDetection {
            detection_id: 3,
            sensor_id: 12,
            contaminant_type: ContaminantType::Chemical,
            concentration_ppb_x100: 500,
            timestamp: 1_700_200_120,
            alert_level: AlertSeverity::Critical,
        },
        ContaminantDetection {
            detection_id: 4,
            sensor_id: 13,
            contaminant_type: ContaminantType::HeavyMetal,
            concentration_ppb_x100: 900,
            timestamp: 1_700_200_180,
            alert_level: AlertSeverity::Emergency,
        },
        ContaminantDetection {
            detection_id: 5,
            sensor_id: 14,
            contaminant_type: ContaminantType::Nitrate,
            concentration_ppb_x100: 350,
            timestamp: 1_700_200_240,
            alert_level: AlertSeverity::Warning,
        },
        ContaminantDetection {
            detection_id: 6,
            sensor_id: 15,
            contaminant_type: ContaminantType::Microplastic,
            concentration_ppb_x100: 50,
            timestamp: 1_700_200_300,
            alert_level: AlertSeverity::Info,
        },
    ];

    let path = temp_dir().join("oxicode_wt33_all_contaminant_types.bin");

    encode_to_file(&detections, &path).expect("encode all ContaminantType variants");
    let decoded: Vec<ContaminantDetection> =
        decode_from_file(&path).expect("decode all ContaminantType variants");

    assert_eq!(detections, decoded);
    std::fs::remove_file(&path).expect("cleanup all ContaminantType file");
}

#[test]
fn test_overwrite_pump_station_file() {
    let path = temp_dir().join("oxicode_wt33_pump_overwrite.bin");

    let first = sample_pump_station(1);
    encode_to_file(&first, &path).expect("encode first PumpStation");

    let second = PumpStation {
        station_id: 99,
        name: "Overwritten Station".to_string(),
        flow_rate_lps_x100: 9_999,
        pressure_kpa_x10: 1_234,
        running: false,
    };
    encode_to_file(&second, &path).expect("encode second PumpStation (overwrite)");

    let decoded: PumpStation = decode_from_file(&path).expect("decode PumpStation after overwrite");

    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    std::fs::remove_file(&path).expect("cleanup overwrite PumpStation file");
}

#[test]
fn test_error_on_missing_file() {
    let path = temp_dir().join("oxicode_wt33_definitely_does_not_exist_zzz99.bin");
    let result = decode_from_file::<WaterQualityReading>(&path);
    assert!(
        result.is_err(),
        "Expected error when decoding from a non-existent file"
    );
}

#[test]
fn test_bytes_match_encode_to_vec_for_quality_reading() {
    let reading = sample_quality_reading(77);
    let path = temp_dir().join("oxicode_wt33_bytes_match_reading.bin");

    encode_to_file(&reading, &path).expect("encode WaterQualityReading for bytes-match test");
    let file_bytes = std::fs::read(&path).expect("read WaterQualityReading file bytes");
    let vec_bytes = encode_to_vec(&reading).expect("encode_to_vec WaterQualityReading");

    assert_eq!(file_bytes, vec_bytes);
    std::fs::remove_file(&path).expect("cleanup bytes-match reading file");
}

#[test]
fn test_bytes_match_encode_to_vec_for_treatment_unit() {
    let unit = sample_treatment_unit(5);
    let path = temp_dir().join("oxicode_wt33_bytes_match_unit.bin");

    encode_to_file(&unit, &path).expect("encode TreatmentUnit for bytes-match test");
    let file_bytes = std::fs::read(&path).expect("read TreatmentUnit file bytes");
    let vec_bytes = encode_to_vec(&unit).expect("encode_to_vec TreatmentUnit");

    assert_eq!(file_bytes, vec_bytes);
    std::fs::remove_file(&path).expect("cleanup bytes-match unit file");
}

#[test]
fn test_decode_from_slice_quality_reading() {
    let reading = sample_quality_reading(33);
    let bytes = encode_to_vec(&reading).expect("encode WaterQualityReading to vec");
    let (decoded, consumed): (WaterQualityReading, usize) =
        decode_from_slice(&bytes).expect("decode WaterQualityReading from slice");

    assert_eq!(reading, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_decode_from_slice_contaminant_detection() {
    let detection = sample_contaminant_detection(12_345);
    let bytes = encode_to_vec(&detection).expect("encode ContaminantDetection to vec");
    let (decoded, consumed): (ContaminantDetection, usize) =
        decode_from_slice(&bytes).expect("decode ContaminantDetection from slice");

    assert_eq!(detection, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_all_alert_severity_variants_file() {
    let severities: Vec<AlertSeverity> = vec![
        AlertSeverity::Info,
        AlertSeverity::Warning,
        AlertSeverity::Critical,
        AlertSeverity::Emergency,
    ];
    let path = temp_dir().join("oxicode_wt33_alert_severities.bin");

    encode_to_file(&severities, &path).expect("encode AlertSeverity variants");
    let decoded: Vec<AlertSeverity> =
        decode_from_file(&path).expect("decode AlertSeverity variants");

    assert_eq!(severities, decoded);
    std::fs::remove_file(&path).expect("cleanup AlertSeverity file");
}

#[test]
fn test_all_water_source_variants_file() {
    let sources: Vec<WaterSource> = vec![
        WaterSource::Surface,
        WaterSource::Groundwater,
        WaterSource::Desalination,
        WaterSource::Recycled,
    ];
    let path = temp_dir().join("oxicode_wt33_water_sources.bin");

    encode_to_file(&sources, &path).expect("encode WaterSource variants");
    let decoded: Vec<WaterSource> = decode_from_file(&path).expect("decode WaterSource variants");

    assert_eq!(sources, decoded);
    std::fs::remove_file(&path).expect("cleanup WaterSource file");
}

#[test]
fn test_all_treatment_stage_variants_file() {
    let stages: Vec<TreatmentStage> = vec![
        TreatmentStage::Coagulation,
        TreatmentStage::Sedimentation,
        TreatmentStage::Filtration,
        TreatmentStage::Disinfection,
        TreatmentStage::Softening,
    ];
    let path = temp_dir().join("oxicode_wt33_treatment_stages.bin");

    encode_to_file(&stages, &path).expect("encode TreatmentStage variants");
    let decoded: Vec<TreatmentStage> =
        decode_from_file(&path).expect("decode TreatmentStage variants");

    assert_eq!(stages, decoded);
    std::fs::remove_file(&path).expect("cleanup TreatmentStage file");
}

#[test]
fn test_network_pipe_boundary_values() {
    let pipe = NetworkPipe {
        pipe_id: u32::MAX,
        diameter_mm: u16::MAX,
        length_m: u32::MAX,
        material: String::new(),
        installation_year: u16::MIN,
    };
    let path = temp_dir().join("oxicode_wt33_network_pipe_boundary.bin");

    encode_to_file(&pipe, &path).expect("encode NetworkPipe boundary values");
    let decoded: NetworkPipe = decode_from_file(&path).expect("decode NetworkPipe boundary values");

    assert_eq!(pipe, decoded);
    std::fs::remove_file(&path).expect("cleanup NetworkPipe boundary file");
}

#[test]
fn test_maintenance_log_long_description() {
    let long_desc = "Routine inspection. ".repeat(200);
    let log = MaintenanceLog {
        log_id: 99_999,
        unit_id: 31,
        timestamp: 1_705_000_000,
        description: long_desc.clone(),
        next_due: Some(1_710_000_000),
    };
    let path = temp_dir().join("oxicode_wt33_maintenance_log_long_desc.bin");

    encode_to_file(&log, &path).expect("encode MaintenanceLog with long description");
    let decoded: MaintenanceLog =
        decode_from_file(&path).expect("decode MaintenanceLog with long description");

    assert_eq!(log, decoded);
    assert_eq!(decoded.description.len(), long_desc.len());
    std::fs::remove_file(&path).expect("cleanup long description maintenance log file");
}

#[test]
fn test_mixed_types_sequential_file_writes() {
    let reading_path = temp_dir().join("oxicode_wt33_mixed_reading.bin");
    let pipe_path = temp_dir().join("oxicode_wt33_mixed_pipe.bin");
    let station_path = temp_dir().join("oxicode_wt33_mixed_station.bin");

    let reading = sample_quality_reading(200);
    let pipe = sample_network_pipe(300);
    let station = sample_pump_station(400);

    encode_to_file(&reading, &reading_path).expect("encode mixed reading");
    encode_to_file(&pipe, &pipe_path).expect("encode mixed pipe");
    encode_to_file(&station, &station_path).expect("encode mixed station");

    let r_reading: WaterQualityReading =
        decode_from_file(&reading_path).expect("decode mixed reading");
    let r_pipe: NetworkPipe = decode_from_file(&pipe_path).expect("decode mixed pipe");
    let r_station: PumpStation = decode_from_file(&station_path).expect("decode mixed station");

    assert_eq!(reading, r_reading);
    assert_eq!(pipe, r_pipe);
    assert_eq!(station, r_station);

    std::fs::remove_file(&reading_path).expect("cleanup mixed reading file");
    std::fs::remove_file(&pipe_path).expect("cleanup mixed pipe file");
    std::fs::remove_file(&station_path).expect("cleanup mixed station file");
}
