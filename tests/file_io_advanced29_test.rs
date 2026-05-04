//! Advanced file I/O tests for OxiCode — ocean monitoring / marine science / oceanography domain.

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

// ---------------------------------------------------------------------------
// Domain model
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SedimentType {
    Clay,
    Silt,
    Sand,
    Gravel,
    Rock,
    CoralReef,
}

/// Fixed oceanographic monitoring station.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OceanStation {
    station_id: u32,
    /// Latitude in micro-degrees (degrees × 10^6).
    lat_deg_x1e6: i32,
    /// Longitude in micro-degrees (degrees × 10^6).
    lon_deg_x1e6: i32,
    depth_m: f32,
    name: String,
}

/// Conductivity-Temperature-Depth reading at a single depth.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CTDReading {
    station_id: u32,
    depth_m: f32,
    temperature_c: f32,
    salinity_psu: f32,
    pressure_dbar: f32,
}

/// Horizontal and vertical current velocity vector at a point.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CurrentVector {
    lat: i32,
    lon: i32,
    depth_m: f32,
    /// Eastward velocity (m/s).
    u_ms: f32,
    /// Northward velocity (m/s).
    v_ms: f32,
    /// Vertical velocity (m/s).
    w_ms: f32,
}

/// Tidal observation at a coastal gauge.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TidalObservation {
    station_id: u32,
    /// Unix timestamp (seconds).
    timestamp: u64,
    water_level_cm: i32,
    predicted_cm: i32,
}

/// Seabed sediment sample.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeabedSample {
    sample_id: u32,
    lat: i32,
    lon: i32,
    depth_m: f32,
    sediment_type: SedimentType,
    organic_pct: f32,
}

/// Full vertical CTD profile for a station at a given time.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OceanicProfile {
    station_id: u32,
    /// Unix timestamp (seconds).
    timestamp: u64,
    readings: Vec<CTDReading>,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_station(id: u32) -> OceanStation {
    OceanStation {
        station_id: id,
        lat_deg_x1e6: 37_774_900_i32 + id as i32,
        lon_deg_x1e6: -122_419_400_i32 + id as i32,
        depth_m: 500.0 + id as f32,
        name: format!("Station-{id:04}"),
    }
}

fn make_ctd(station_id: u32, depth_m: f32) -> CTDReading {
    CTDReading {
        station_id,
        depth_m,
        temperature_c: 4.0 + depth_m * 0.001,
        salinity_psu: 35.0 + depth_m * 0.0001,
        pressure_dbar: depth_m * 1.01,
    }
}

fn make_profile(station_id: u32, n_readings: usize) -> OceanicProfile {
    let readings = (0..n_readings)
        .map(|i| make_ctd(station_id, i as f32 * 2.0))
        .collect();
    OceanicProfile {
        station_id,
        timestamp: 1_700_000_000 + station_id as u64,
        readings,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_ocean_station_file_roundtrip() {
    let station = make_station(1);
    let path = tmp("oxicode_ocean_station_1.bin");

    encode_to_file(&station, &path).expect("encode OceanStation to file");
    let decoded: OceanStation = decode_from_file(&path).expect("decode OceanStation from file");

    assert_eq!(station, decoded);
    std::fs::remove_file(&path).expect("cleanup ocean_station_1");
}

#[test]
fn test_ctd_reading_file_roundtrip() {
    let ctd = make_ctd(42, 250.0);
    let path = tmp("oxicode_ctd_reading_1.bin");

    encode_to_file(&ctd, &path).expect("encode CTDReading to file");
    let decoded: CTDReading = decode_from_file(&path).expect("decode CTDReading from file");

    assert_eq!(ctd, decoded);
    std::fs::remove_file(&path).expect("cleanup ctd_reading_1");
}

#[test]
fn test_current_vector_file_roundtrip() {
    let cv = CurrentVector {
        lat: 35_600_000,
        lon: 139_700_000,
        depth_m: 100.0,
        u_ms: 0.35,
        v_ms: -0.12,
        w_ms: 0.002,
    };
    let path = tmp("oxicode_current_vector_1.bin");

    encode_to_file(&cv, &path).expect("encode CurrentVector to file");
    let decoded: CurrentVector = decode_from_file(&path).expect("decode CurrentVector from file");

    assert_eq!(cv, decoded);
    std::fs::remove_file(&path).expect("cleanup current_vector_1");
}

#[test]
fn test_tidal_observation_file_roundtrip() {
    let obs = TidalObservation {
        station_id: 7,
        timestamp: 1_720_000_000,
        water_level_cm: 312,
        predicted_cm: 298,
    };
    let path = tmp("oxicode_tidal_obs_1.bin");

    encode_to_file(&obs, &path).expect("encode TidalObservation to file");
    let decoded: TidalObservation =
        decode_from_file(&path).expect("decode TidalObservation from file");

    assert_eq!(obs, decoded);
    std::fs::remove_file(&path).expect("cleanup tidal_obs_1");
}

#[test]
fn test_seabed_sample_file_roundtrip() {
    let sample = SeabedSample {
        sample_id: 101,
        lat: -18_000_000,
        lon: 147_000_000,
        depth_m: 1800.0,
        sediment_type: SedimentType::CoralReef,
        organic_pct: 3.7,
    };
    let path = tmp("oxicode_seabed_sample_1.bin");

    encode_to_file(&sample, &path).expect("encode SeabedSample to file");
    let decoded: SeabedSample = decode_from_file(&path).expect("decode SeabedSample from file");

    assert_eq!(sample, decoded);
    std::fs::remove_file(&path).expect("cleanup seabed_sample_1");
}

#[test]
fn test_oceanic_profile_file_roundtrip() {
    let profile = make_profile(5, 20);
    let path = tmp("oxicode_oceanic_profile_1.bin");

    encode_to_file(&profile, &path).expect("encode OceanicProfile to file");
    let decoded: OceanicProfile = decode_from_file(&path).expect("decode OceanicProfile from file");

    assert_eq!(profile, decoded);
    std::fs::remove_file(&path).expect("cleanup oceanic_profile_1");
}

#[test]
fn test_large_profile_500_readings() {
    let profile = make_profile(99, 500);
    assert_eq!(profile.readings.len(), 500);
    let path = tmp("oxicode_large_profile_500.bin");

    encode_to_file(&profile, &path).expect("encode large profile (500 readings)");
    let decoded: OceanicProfile =
        decode_from_file(&path).expect("decode large profile (500 readings)");

    assert_eq!(decoded.readings.len(), 500);
    assert_eq!(profile, decoded);
    std::fs::remove_file(&path).expect("cleanup large_profile_500");
}

#[test]
fn test_vec_of_stations_file_roundtrip() {
    let stations: Vec<OceanStation> = (1..=15).map(make_station).collect();
    let path = tmp("oxicode_vec_stations.bin");

    encode_to_file(&stations, &path).expect("encode Vec<OceanStation>");
    let decoded: Vec<OceanStation> = decode_from_file(&path).expect("decode Vec<OceanStation>");

    assert_eq!(stations.len(), decoded.len());
    assert_eq!(stations, decoded);
    std::fs::remove_file(&path).expect("cleanup vec_stations");
}

#[test]
fn test_byte_match_file_vs_encode_to_vec_station() {
    let station = make_station(3);
    let path = tmp("oxicode_byte_match_station.bin");

    encode_to_file(&station, &path).expect("encode station for byte match");
    let file_bytes = std::fs::read(&path).expect("read station file");
    let vec_bytes = encode_to_vec(&station).expect("encode_to_vec station");

    assert_eq!(
        file_bytes, vec_bytes,
        "file bytes must match encode_to_vec bytes"
    );
    std::fs::remove_file(&path).expect("cleanup byte_match_station");
}

#[test]
fn test_byte_match_file_vs_encode_to_vec_profile() {
    let profile = make_profile(10, 30);
    let path = tmp("oxicode_byte_match_profile.bin");

    encode_to_file(&profile, &path).expect("encode profile for byte match");
    let file_bytes = std::fs::read(&path).expect("read profile file");
    let vec_bytes = encode_to_vec(&profile).expect("encode_to_vec profile");

    assert_eq!(
        file_bytes, vec_bytes,
        "file bytes must match encode_to_vec bytes for profile"
    );
    std::fs::remove_file(&path).expect("cleanup byte_match_profile");
}

#[test]
fn test_error_on_missing_file() {
    let path = tmp("oxicode_ocean_nonexistent_xyz_29.bin");
    // Ensure it doesn't exist
    let _ = std::fs::remove_file(&path);
    let result = decode_from_file::<OceanStation>(&path);
    assert!(
        result.is_err(),
        "decode_from_file on missing file must return Err"
    );
}

#[test]
fn test_overwrite_station_file() {
    let path = tmp("oxicode_overwrite_station.bin");

    let first = make_station(1);
    encode_to_file(&first, &path).expect("first write");

    let second = make_station(2);
    encode_to_file(&second, &path).expect("overwrite write");

    let decoded: OceanStation = decode_from_file(&path).expect("decode after overwrite");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    std::fs::remove_file(&path).expect("cleanup overwrite_station");
}

#[test]
fn test_all_sediment_type_variants() {
    let variants = [
        SedimentType::Clay,
        SedimentType::Silt,
        SedimentType::Sand,
        SedimentType::Gravel,
        SedimentType::Rock,
        SedimentType::CoralReef,
    ];
    for (i, variant) in variants.iter().enumerate() {
        let sample = SeabedSample {
            sample_id: i as u32,
            lat: -10_000_000 + i as i32 * 1_000_000,
            lon: 100_000_000 + i as i32 * 500_000,
            depth_m: 200.0 * i as f32 + 50.0,
            sediment_type: variant.clone(),
            organic_pct: i as f32 * 0.5,
        };
        let path = tmp(format!("oxicode_sediment_{i}.bin"));
        encode_to_file(&sample, &path).expect("encode sediment sample");
        let decoded: SeabedSample = decode_from_file(&path).expect("decode sediment sample");
        assert_eq!(sample, decoded);
        std::fs::remove_file(&path).expect("cleanup sediment variant");
    }
}

#[test]
fn test_vec_of_ctd_readings_roundtrip() {
    let readings: Vec<CTDReading> = (0..50).map(|i| make_ctd(7, i as f32 * 5.0)).collect();
    let path = tmp("oxicode_vec_ctd_readings.bin");

    encode_to_file(&readings, &path).expect("encode Vec<CTDReading>");
    let decoded: Vec<CTDReading> = decode_from_file(&path).expect("decode Vec<CTDReading>");

    assert_eq!(readings, decoded);
    std::fs::remove_file(&path).expect("cleanup vec_ctd_readings");
}

#[test]
fn test_option_station_some_roundtrip() {
    let opt: Option<OceanStation> = Some(make_station(77));
    let path = tmp("oxicode_option_station_some.bin");

    encode_to_file(&opt, &path).expect("encode Option<OceanStation> Some");
    let decoded: Option<OceanStation> =
        decode_from_file(&path).expect("decode Option<OceanStation> Some");

    assert_eq!(opt, decoded);
    std::fs::remove_file(&path).expect("cleanup option_station_some");
}

#[test]
fn test_option_station_none_roundtrip() {
    let opt: Option<OceanStation> = None;
    let path = tmp("oxicode_option_station_none.bin");

    encode_to_file(&opt, &path).expect("encode Option<OceanStation> None");
    let decoded: Option<OceanStation> =
        decode_from_file(&path).expect("decode Option<OceanStation> None");

    assert_eq!(opt, decoded);
    assert!(decoded.is_none());
    std::fs::remove_file(&path).expect("cleanup option_station_none");
}

#[test]
fn test_tidal_observation_decode_from_slice() {
    let obs = TidalObservation {
        station_id: 33,
        timestamp: 1_710_000_000,
        water_level_cm: -45,
        predicted_cm: -50,
    };
    let encoded = encode_to_vec(&obs).expect("encode TidalObservation to vec");
    let (decoded, consumed): (TidalObservation, usize) =
        decode_from_slice(&encoded).expect("decode TidalObservation from slice");

    assert_eq!(obs, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_current_vector_decode_from_slice() {
    let cv = CurrentVector {
        lat: 51_500_000,
        lon: -0_127_800,
        depth_m: 50.0,
        u_ms: 1.2,
        v_ms: -0.7,
        w_ms: 0.0,
    };
    let encoded = encode_to_vec(&cv).expect("encode CurrentVector to vec");
    let (decoded, consumed): (CurrentVector, usize) =
        decode_from_slice(&encoded).expect("decode CurrentVector from slice");

    assert_eq!(cv, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_multiple_profiles_different_depths() {
    let depths_counts = [(100usize, "shallow"), (300usize, "mid"), (600usize, "deep")];
    for (id, (count, label)) in depths_counts.iter().enumerate() {
        let profile = make_profile(id as u32, *count);
        let path = tmp(format!("oxicode_profile_{label}.bin"));

        encode_to_file(&profile, &path).expect("encode profile");
        let decoded: OceanicProfile = decode_from_file(&path).expect("decode profile");

        assert_eq!(
            decoded.readings.len(),
            *count,
            "reading count mismatch for {label}"
        );
        assert_eq!(profile, decoded);
        std::fs::remove_file(&path).expect("cleanup profile");
    }
}

#[test]
fn test_station_with_unicode_name() {
    let station = OceanStation {
        station_id: 999,
        lat_deg_x1e6: 35_658_000,
        lon_deg_x1e6: 139_691_000,
        depth_m: 40.0,
        name: "東京湾観測点 🌊".to_string(),
    };
    let path = tmp("oxicode_station_unicode.bin");

    encode_to_file(&station, &path).expect("encode station unicode");
    let decoded: OceanStation = decode_from_file(&path).expect("decode station unicode");

    assert_eq!(station, decoded);
    assert_eq!(decoded.name, "東京湾観測点 🌊");
    std::fs::remove_file(&path).expect("cleanup station_unicode");
}

#[test]
fn test_vec_of_tidal_observations_file_roundtrip() {
    let observations: Vec<TidalObservation> = (0..100)
        .map(|i| TidalObservation {
            station_id: i % 5,
            timestamp: 1_700_000_000 + i as u64 * 3600,
            water_level_cm: (i as i32 * 7 - 350),
            predicted_cm: (i as i32 * 7 - 345),
        })
        .collect();
    let path = tmp("oxicode_vec_tidal_obs.bin");

    encode_to_file(&observations, &path).expect("encode Vec<TidalObservation>");
    let decoded: Vec<TidalObservation> =
        decode_from_file(&path).expect("decode Vec<TidalObservation>");

    assert_eq!(observations.len(), decoded.len());
    assert_eq!(observations, decoded);
    std::fs::remove_file(&path).expect("cleanup vec_tidal_obs");
}

#[test]
fn test_seabed_sample_coral_reef_organic_pct() {
    let sample = SeabedSample {
        sample_id: 555,
        lat: -23_000_000,
        lon: 150_000_000,
        depth_m: 12.5,
        sediment_type: SedimentType::CoralReef,
        organic_pct: 18.3,
    };
    let encoded = encode_to_vec(&sample).expect("encode coral reef sample");
    let (decoded, _): (SeabedSample, usize) =
        decode_from_slice(&encoded).expect("decode coral reef sample");

    assert_eq!(sample, decoded);
    assert!((decoded.organic_pct - 18.3_f32).abs() < 1e-5);
}

#[test]
fn test_empty_oceanic_profile_file_roundtrip() {
    let profile = OceanicProfile {
        station_id: 0,
        timestamp: 0,
        readings: Vec::new(),
    };
    let path = tmp("oxicode_empty_profile.bin");

    encode_to_file(&profile, &path).expect("encode empty OceanicProfile");
    let decoded: OceanicProfile = decode_from_file(&path).expect("decode empty OceanicProfile");

    assert_eq!(profile, decoded);
    assert!(decoded.readings.is_empty());
    std::fs::remove_file(&path).expect("cleanup empty_profile");
}
