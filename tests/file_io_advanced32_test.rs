//! Advanced file I/O tests for OxiCode — space exploration / planetary science / rover telemetry domain.

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

// ─── Domain types ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RoverMode {
    Standby,
    Traversal,
    Science,
    Charging,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InstrumentType {
    Camera,
    Spectrometer,
    DrillCore,
    Weather,
    Radar,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TerrainType {
    Flat,
    Rocky,
    Crater,
    Sandy,
    IcyTerrain,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MissionPhase {
    Launch,
    Transit,
    Orbit,
    Landing,
    Surface,
    Return,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RoverPosition {
    rover_id: u32,
    x_m: f32,
    y_m: f32,
    heading_deg: f32,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScienceSample {
    sample_id: u32,
    rover_id: u32,
    lat_x1e6: i32,
    lon_x1e6: i32,
    depth_cm: u8,
    composition: Vec<u8>,
    collected_at: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SystemHealth {
    rover_id: u32,
    battery_pct: u8,
    temperature_c: i8,
    memory_free_kb: u32,
    mode: RoverMode,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InstrumentReading {
    instrument_id: u32,
    instrument_type: InstrumentType,
    timestamp: u64,
    data: Vec<u8>,
    quality_score: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TerrainMap {
    rover_id: u32,
    resolution_m: f32,
    width_cells: u16,
    height_cells: u16,
    cells: Vec<TerrainType>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MissionEvent {
    event_id: u64,
    mission_phase: MissionPhase,
    timestamp: u64,
    description: String,
    is_anomaly: bool,
}

// ─── Test 1: RoverPosition file roundtrip ───────────────────────────────────

#[test]
fn test_rover_position_file_roundtrip() {
    let pos = RoverPosition {
        rover_id: 1,
        x_m: 123.456,
        y_m: -78.9,
        heading_deg: 270.0,
        timestamp: 1_700_000_000,
    };
    let path = tmp("oxicode_rover_position.bin");
    encode_to_file(&pos, &path).expect("encode RoverPosition to file");
    let decoded: RoverPosition = decode_from_file(&path).expect("decode RoverPosition from file");
    assert_eq!(pos, decoded);
    std::fs::remove_file(&path).expect("cleanup RoverPosition file");
}

// ─── Test 2: ScienceSample file roundtrip ───────────────────────────────────

#[test]
fn test_science_sample_file_roundtrip() {
    let sample = ScienceSample {
        sample_id: 42,
        rover_id: 2,
        lat_x1e6: 4_593_000,
        lon_x1e6: -137_210_000,
        depth_cm: 15,
        composition: vec![12, 34, 56, 78, 90, 11],
        collected_at: 1_700_001_000,
    };
    let path = tmp("oxicode_science_sample.bin");
    encode_to_file(&sample, &path).expect("encode ScienceSample to file");
    let decoded: ScienceSample = decode_from_file(&path).expect("decode ScienceSample from file");
    assert_eq!(sample, decoded);
    std::fs::remove_file(&path).expect("cleanup ScienceSample file");
}

// ─── Test 3: SystemHealth file roundtrip ────────────────────────────────────

#[test]
fn test_system_health_file_roundtrip() {
    let health = SystemHealth {
        rover_id: 3,
        battery_pct: 87,
        temperature_c: -42,
        memory_free_kb: 204_800,
        mode: RoverMode::Science,
    };
    let path = tmp("oxicode_system_health.bin");
    encode_to_file(&health, &path).expect("encode SystemHealth to file");
    let decoded: SystemHealth = decode_from_file(&path).expect("decode SystemHealth from file");
    assert_eq!(health, decoded);
    std::fs::remove_file(&path).expect("cleanup SystemHealth file");
}

// ─── Test 4: InstrumentReading file roundtrip ───────────────────────────────

#[test]
fn test_instrument_reading_file_roundtrip() {
    let reading = InstrumentReading {
        instrument_id: 7,
        instrument_type: InstrumentType::Spectrometer,
        timestamp: 1_700_002_500,
        data: (0u8..128).collect(),
        quality_score: 99,
    };
    let path = tmp("oxicode_instrument_reading.bin");
    encode_to_file(&reading, &path).expect("encode InstrumentReading to file");
    let decoded: InstrumentReading =
        decode_from_file(&path).expect("decode InstrumentReading from file");
    assert_eq!(reading, decoded);
    std::fs::remove_file(&path).expect("cleanup InstrumentReading file");
}

// ─── Test 5: TerrainMap file roundtrip (small) ──────────────────────────────

#[test]
fn test_terrain_map_small_file_roundtrip() {
    let cells = vec![
        TerrainType::Flat,
        TerrainType::Rocky,
        TerrainType::Crater,
        TerrainType::Sandy,
        TerrainType::IcyTerrain,
        TerrainType::Rocky,
    ];
    let map = TerrainMap {
        rover_id: 1,
        resolution_m: 0.5,
        width_cells: 3,
        height_cells: 2,
        cells,
    };
    let path = tmp("oxicode_terrain_map_small.bin");
    encode_to_file(&map, &path).expect("encode small TerrainMap to file");
    let decoded: TerrainMap = decode_from_file(&path).expect("decode small TerrainMap from file");
    assert_eq!(map, decoded);
    std::fs::remove_file(&path).expect("cleanup small TerrainMap file");
}

// ─── Test 6: MissionEvent file roundtrip ────────────────────────────────────

#[test]
fn test_mission_event_file_roundtrip() {
    let event = MissionEvent {
        event_id: 1001,
        mission_phase: MissionPhase::Landing,
        timestamp: 1_700_003_000,
        description: "Touchdown confirmed — Jezero Crater".to_string(),
        is_anomaly: false,
    };
    let path = tmp("oxicode_mission_event.bin");
    encode_to_file(&event, &path).expect("encode MissionEvent to file");
    let decoded: MissionEvent = decode_from_file(&path).expect("decode MissionEvent from file");
    assert_eq!(event, decoded);
    std::fs::remove_file(&path).expect("cleanup MissionEvent file");
}

// ─── Test 7: Large TerrainMap (100×100 cells) ───────────────────────────────

#[test]
fn test_terrain_map_large_100x100_file_roundtrip() {
    let terrain_cycle = [
        TerrainType::Flat,
        TerrainType::Rocky,
        TerrainType::Crater,
        TerrainType::Sandy,
        TerrainType::IcyTerrain,
    ];
    let cells: Vec<TerrainType> = (0..10_000)
        .map(|i| terrain_cycle[i % terrain_cycle.len()].clone())
        .collect();
    let map = TerrainMap {
        rover_id: 5,
        resolution_m: 1.0,
        width_cells: 100,
        height_cells: 100,
        cells,
    };
    let path = tmp("oxicode_terrain_map_large.bin");
    encode_to_file(&map, &path).expect("encode large TerrainMap to file");
    let decoded: TerrainMap = decode_from_file(&path).expect("decode large TerrainMap from file");
    assert_eq!(map.rover_id, decoded.rover_id);
    assert_eq!(map.width_cells, decoded.width_cells);
    assert_eq!(map.height_cells, decoded.height_cells);
    assert_eq!(map.cells.len(), decoded.cells.len());
    assert_eq!(map, decoded);
    std::fs::remove_file(&path).expect("cleanup large TerrainMap file");
}

// ─── Test 8: Large sample collection (500 samples) ──────────────────────────

#[test]
fn test_large_science_sample_collection_file_roundtrip() {
    let samples: Vec<ScienceSample> = (0..500)
        .map(|i| ScienceSample {
            sample_id: i as u32,
            rover_id: 1,
            lat_x1e6: 4_000_000 + i * 100,
            lon_x1e6: -137_000_000 + i * 50,
            depth_cm: (i % 200) as u8,
            composition: vec![(i % 256) as u8; 32],
            collected_at: 1_700_000_000 + i as u64 * 60,
        })
        .collect();
    let path = tmp("oxicode_large_sample_collection.bin");
    encode_to_file(&samples, &path).expect("encode large sample collection to file");
    let decoded: Vec<ScienceSample> =
        decode_from_file(&path).expect("decode large sample collection from file");
    assert_eq!(samples.len(), decoded.len());
    assert_eq!(samples, decoded);
    std::fs::remove_file(&path).expect("cleanup large sample collection file");
}

// ─── Test 9: File bytes match encode_to_vec (RoverPosition) ─────────────────

#[test]
fn test_rover_position_file_bytes_match_encode_to_vec() {
    let pos = RoverPosition {
        rover_id: 99,
        x_m: 0.0,
        y_m: 0.0,
        heading_deg: 180.0,
        timestamp: 9_999_999,
    };
    let path = tmp("oxicode_rover_pos_bytes_match.bin");
    encode_to_file(&pos, &path).expect("encode RoverPosition for bytes match");
    let file_bytes = std::fs::read(&path).expect("read RoverPosition file bytes");
    let vec_bytes = encode_to_vec(&pos).expect("encode_to_vec RoverPosition");
    assert_eq!(file_bytes, vec_bytes);
    std::fs::remove_file(&path).expect("cleanup RoverPosition bytes match file");
}

// ─── Test 10: File bytes match encode_to_vec (SystemHealth) ─────────────────

#[test]
fn test_system_health_file_bytes_match_encode_to_vec() {
    let health = SystemHealth {
        rover_id: 7,
        battery_pct: 55,
        temperature_c: 10,
        memory_free_kb: 65_536,
        mode: RoverMode::Traversal,
    };
    let path = tmp("oxicode_system_health_bytes_match.bin");
    encode_to_file(&health, &path).expect("encode SystemHealth for bytes match");
    let file_bytes = std::fs::read(&path).expect("read SystemHealth file bytes");
    let vec_bytes = encode_to_vec(&health).expect("encode_to_vec SystemHealth");
    assert_eq!(file_bytes, vec_bytes);
    std::fs::remove_file(&path).expect("cleanup SystemHealth bytes match file");
}

// ─── Test 11: File bytes match encode_to_vec (MissionEvent) ─────────────────

#[test]
fn test_mission_event_file_bytes_match_encode_to_vec() {
    let event = MissionEvent {
        event_id: 77,
        mission_phase: MissionPhase::Surface,
        timestamp: 1_800_000_000,
        description: "Sample arm deployed successfully".to_string(),
        is_anomaly: false,
    };
    let path = tmp("oxicode_mission_event_bytes_match.bin");
    encode_to_file(&event, &path).expect("encode MissionEvent for bytes match");
    let file_bytes = std::fs::read(&path).expect("read MissionEvent file bytes");
    let vec_bytes = encode_to_vec(&event).expect("encode_to_vec MissionEvent");
    assert_eq!(file_bytes, vec_bytes);
    std::fs::remove_file(&path).expect("cleanup MissionEvent bytes match file");
}

// ─── Test 12: Overwrite existing file with new RoverPosition ────────────────

#[test]
fn test_overwrite_rover_position_file() {
    let path = tmp("oxicode_rover_overwrite.bin");

    let first = RoverPosition {
        rover_id: 10,
        x_m: 1.0,
        y_m: 2.0,
        heading_deg: 90.0,
        timestamp: 100,
    };
    encode_to_file(&first, &path).expect("encode first RoverPosition");

    let second = RoverPosition {
        rover_id: 20,
        x_m: 999.0,
        y_m: -999.0,
        heading_deg: 45.0,
        timestamp: 200,
    };
    encode_to_file(&second, &path).expect("encode second RoverPosition (overwrite)");

    let decoded: RoverPosition = decode_from_file(&path).expect("decode overwritten RoverPosition");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    std::fs::remove_file(&path).expect("cleanup overwrite file");
}

// ─── Test 13: Error on missing file ─────────────────────────────────────────

#[test]
fn test_decode_from_missing_file_returns_error() {
    let path = tmp("oxicode_does_not_exist_rover_telemetry_xyz.bin");
    // Ensure it really doesn't exist
    let _ = std::fs::remove_file(&path);
    let result = decode_from_file::<RoverPosition>(&path);
    assert!(result.is_err(), "expected error decoding from missing file");
}

// ─── Test 14: All RoverMode variants roundtrip ──────────────────────────────

#[test]
fn test_all_rover_mode_variants_file_roundtrip() {
    let modes = vec![
        RoverMode::Standby,
        RoverMode::Traversal,
        RoverMode::Science,
        RoverMode::Charging,
        RoverMode::Emergency,
    ];
    let path = tmp("oxicode_rover_modes.bin");
    encode_to_file(&modes, &path).expect("encode RoverMode variants");
    let decoded: Vec<RoverMode> = decode_from_file(&path).expect("decode RoverMode variants");
    assert_eq!(modes, decoded);
    std::fs::remove_file(&path).expect("cleanup RoverMode variants file");
}

// ─── Test 15: All InstrumentType variants roundtrip ─────────────────────────

#[test]
fn test_all_instrument_type_variants_file_roundtrip() {
    let types = vec![
        InstrumentType::Camera,
        InstrumentType::Spectrometer,
        InstrumentType::DrillCore,
        InstrumentType::Weather,
        InstrumentType::Radar,
    ];
    let path = tmp("oxicode_instrument_types.bin");
    encode_to_file(&types, &path).expect("encode InstrumentType variants");
    let decoded: Vec<InstrumentType> =
        decode_from_file(&path).expect("decode InstrumentType variants");
    assert_eq!(types, decoded);
    std::fs::remove_file(&path).expect("cleanup InstrumentType variants file");
}

// ─── Test 16: All MissionPhase variants roundtrip ───────────────────────────

#[test]
fn test_all_mission_phase_variants_file_roundtrip() {
    let phases = vec![
        MissionPhase::Launch,
        MissionPhase::Transit,
        MissionPhase::Orbit,
        MissionPhase::Landing,
        MissionPhase::Surface,
        MissionPhase::Return,
    ];
    let path = tmp("oxicode_mission_phases.bin");
    encode_to_file(&phases, &path).expect("encode MissionPhase variants");
    let decoded: Vec<MissionPhase> = decode_from_file(&path).expect("decode MissionPhase variants");
    assert_eq!(phases, decoded);
    std::fs::remove_file(&path).expect("cleanup MissionPhase variants file");
}

// ─── Test 17: All TerrainType variants roundtrip ────────────────────────────

#[test]
fn test_all_terrain_type_variants_file_roundtrip() {
    let terrains = vec![
        TerrainType::Flat,
        TerrainType::Rocky,
        TerrainType::Crater,
        TerrainType::Sandy,
        TerrainType::IcyTerrain,
    ];
    let path = tmp("oxicode_terrain_types.bin");
    encode_to_file(&terrains, &path).expect("encode TerrainType variants");
    let decoded: Vec<TerrainType> = decode_from_file(&path).expect("decode TerrainType variants");
    assert_eq!(terrains, decoded);
    std::fs::remove_file(&path).expect("cleanup TerrainType variants file");
}

// ─── Test 18: Option<RoverPosition> — Some roundtrip ────────────────────────

#[test]
fn test_option_rover_position_some_file_roundtrip() {
    let maybe_pos: Option<RoverPosition> = Some(RoverPosition {
        rover_id: 55,
        x_m: 3.14,
        y_m: 2.72,
        heading_deg: 315.0,
        timestamp: 2_000_000_000,
    });
    let path = tmp("oxicode_option_rover_position_some.bin");
    encode_to_file(&maybe_pos, &path).expect("encode Some(RoverPosition)");
    let decoded: Option<RoverPosition> =
        decode_from_file(&path).expect("decode Some(RoverPosition)");
    assert_eq!(maybe_pos, decoded);
    std::fs::remove_file(&path).expect("cleanup Some(RoverPosition) file");
}

// ─── Test 19: Option<RoverPosition> — None roundtrip ────────────────────────

#[test]
fn test_option_rover_position_none_file_roundtrip() {
    let maybe_pos: Option<RoverPosition> = None;
    let path = tmp("oxicode_option_rover_position_none.bin");
    encode_to_file(&maybe_pos, &path).expect("encode None RoverPosition");
    let decoded: Option<RoverPosition> =
        decode_from_file(&path).expect("decode None RoverPosition");
    assert_eq!(maybe_pos, decoded);
    std::fs::remove_file(&path).expect("cleanup None(RoverPosition) file");
}

// ─── Test 20: Anomaly mission event roundtrip ───────────────────────────────

#[test]
fn test_anomaly_mission_event_file_roundtrip() {
    let event = MissionEvent {
        event_id: 9999,
        mission_phase: MissionPhase::Surface,
        timestamp: 1_900_000_000,
        description: "CRITICAL: Wheel motor stall detected on front-right actuator".to_string(),
        is_anomaly: true,
    };
    let path = tmp("oxicode_anomaly_event.bin");
    encode_to_file(&event, &path).expect("encode anomaly MissionEvent");
    let decoded: MissionEvent =
        decode_from_file(&path).expect("decode anomaly MissionEvent from file");
    assert_eq!(event, decoded);
    assert!(decoded.is_anomaly);
    std::fs::remove_file(&path).expect("cleanup anomaly MissionEvent file");
}

// ─── Test 21: decode_from_slice consistency with file ───────────────────────

#[test]
fn test_instrument_reading_decode_from_slice_matches_file() {
    let reading = InstrumentReading {
        instrument_id: 12,
        instrument_type: InstrumentType::Radar,
        timestamp: 1_750_000_000,
        data: vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE],
        quality_score: 77,
    };
    let path = tmp("oxicode_instrument_slice_vs_file.bin");
    encode_to_file(&reading, &path).expect("encode InstrumentReading for slice comparison");

    // Decode from file
    let from_file: InstrumentReading =
        decode_from_file(&path).expect("decode InstrumentReading from file");

    // Decode from vec bytes via decode_from_slice
    let bytes = encode_to_vec(&reading).expect("encode_to_vec InstrumentReading");
    let (from_slice, consumed): (InstrumentReading, usize) =
        decode_from_slice(&bytes).expect("decode_from_slice InstrumentReading");

    assert_eq!(from_file, from_slice);
    assert_eq!(consumed, bytes.len());
    std::fs::remove_file(&path).expect("cleanup slice vs file test file");
}

// ─── Test 22: Mixed rover telemetry batch roundtrip ─────────────────────────

#[test]
fn test_mixed_rover_telemetry_batch_file_roundtrip() {
    // Represent a telemetry frame as a tuple of positions, health snapshots, and readings
    let positions: Vec<RoverPosition> = (0..10)
        .map(|i| RoverPosition {
            rover_id: i,
            x_m: i as f32 * 5.5,
            y_m: i as f32 * -3.3,
            heading_deg: (i as f32 * 36.0) % 360.0,
            timestamp: 1_700_000_000 + i as u64 * 10,
        })
        .collect();

    let health_snapshots: Vec<SystemHealth> = (0..10)
        .map(|i| SystemHealth {
            rover_id: i,
            battery_pct: 100 - (i * 3) as u8,
            temperature_c: -20 + i as i8,
            memory_free_kb: 131_072 - i as u32 * 1024,
            mode: if i % 2 == 0 {
                RoverMode::Traversal
            } else {
                RoverMode::Science
            },
        })
        .collect();

    let readings: Vec<InstrumentReading> = (0..10)
        .map(|i| InstrumentReading {
            instrument_id: i,
            instrument_type: match i % 5 {
                0 => InstrumentType::Camera,
                1 => InstrumentType::Spectrometer,
                2 => InstrumentType::DrillCore,
                3 => InstrumentType::Weather,
                _ => InstrumentType::Radar,
            },
            timestamp: 1_700_000_000 + i as u64 * 5,
            data: vec![i as u8; 16],
            quality_score: 80 + i as u8,
        })
        .collect();

    let path_pos = tmp("oxicode_batch_positions.bin");
    let path_health = tmp("oxicode_batch_health.bin");
    let path_readings = tmp("oxicode_batch_readings.bin");

    encode_to_file(&positions, &path_pos).expect("encode batch positions");
    encode_to_file(&health_snapshots, &path_health).expect("encode batch health");
    encode_to_file(&readings, &path_readings).expect("encode batch readings");

    let dec_pos: Vec<RoverPosition> = decode_from_file(&path_pos).expect("decode batch positions");
    let dec_health: Vec<SystemHealth> =
        decode_from_file(&path_health).expect("decode batch health");
    let dec_readings: Vec<InstrumentReading> =
        decode_from_file(&path_readings).expect("decode batch readings");

    assert_eq!(positions, dec_pos);
    assert_eq!(health_snapshots, dec_health);
    assert_eq!(readings, dec_readings);

    std::fs::remove_file(&path_pos).expect("cleanup batch positions file");
    std::fs::remove_file(&path_health).expect("cleanup batch health file");
    std::fs::remove_file(&path_readings).expect("cleanup batch readings file");
}
