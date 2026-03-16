#![cfg(feature = "serde")]
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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum SpectralBand {
    Coastal,
    Blue,
    Green,
    Red,
    RedEdge,
    NearInfrared,
    ShortWaveInfrared1,
    ShortWaveInfrared2,
    Panchromatic,
    Thermal,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum LandCoverClass {
    Water,
    UrbanBuiltUp,
    Agriculture,
    Forest,
    Grassland,
    Barren,
    Snow,
    Wetland,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum OrbitType {
    LowEarthOrbit,
    MediumEarthOrbit,
    GeostationaryOrbit,
    SunSynchronous,
    PolarOrbit,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AtmosphericCorrectionMethod {
    None,
    DarkObjectSubtraction,
    Flaash,
    Quac,
    SixS {
        aerosol_optical_depth: u32,
        water_vapor_cm: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GroundControlPoint {
    gcp_id: u32,
    latitude_microdeg: i64,
    longitude_microdeg: i64,
    elevation_mm: i32,
    pixel_x: f32,
    pixel_y: f32,
    residual_m: Option<f32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SpectralBandInfo {
    band: SpectralBand,
    center_wavelength_nm: u16,
    bandwidth_nm: u16,
    radiometric_resolution_bits: u8,
    radiance_min: f32,
    radiance_max: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ImageTile {
    tile_id: u64,
    column: u32,
    row: u32,
    width_px: u32,
    height_px: u32,
    overlap_px: u16,
    pixel_data: Vec<u16>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VegetationIndices {
    ndvi: i32,
    evi: i32,
    savi: i32,
    ndwi: i32,
    nbr: i32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct OrbitParameters {
    satellite_name: String,
    orbit_type: OrbitType,
    altitude_km: u32,
    inclination_mdeg: u32,
    period_seconds: u32,
    revisit_time_hours: Option<u16>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SceneMetadata {
    scene_id: String,
    acquisition_timestamp_us: u64,
    cloud_cover_percent: u8,
    sun_elevation_mdeg: i32,
    sun_azimuth_mdeg: u32,
    ground_sample_distance_cm: u32,
    orbit: OrbitParameters,
    bands: Vec<SpectralBandInfo>,
    atmospheric_correction: AtmosphericCorrectionMethod,
    gcps: Vec<GroundControlPoint>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LandCoverSegment {
    segment_id: u32,
    class: LandCoverClass,
    area_m2: u64,
    confidence_pct: u8,
    centroid_lat_microdeg: i64,
    centroid_lon_microdeg: i64,
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn default_orbit() -> OrbitParameters {
    OrbitParameters {
        satellite_name: "Sentinel-2A".to_string(),
        orbit_type: OrbitType::SunSynchronous,
        altitude_km: 786,
        inclination_mdeg: 98_570,
        period_seconds: 5_955,
        revisit_time_hours: Some(120),
    }
}

fn make_band_info(band: SpectralBand, center_nm: u16, width_nm: u16, bits: u8) -> SpectralBandInfo {
    SpectralBandInfo {
        band,
        center_wavelength_nm: center_nm,
        bandwidth_nm: width_nm,
        radiometric_resolution_bits: bits,
        radiance_min: 0.0,
        radiance_max: 1000.0,
    }
}

fn make_gcp(id: u32, lat: i64, lon: i64, elev: i32) -> GroundControlPoint {
    GroundControlPoint {
        gcp_id: id,
        latitude_microdeg: lat,
        longitude_microdeg: lon,
        elevation_mm: elev,
        pixel_x: (id as f32) * 100.0,
        pixel_y: (id as f32) * 100.0,
        residual_m: None,
    }
}

fn make_minimal_scene(scene_id: &str, timestamp: u64, cloud_cover: u8) -> SceneMetadata {
    SceneMetadata {
        scene_id: scene_id.to_string(),
        acquisition_timestamp_us: timestamp,
        cloud_cover_percent: cloud_cover,
        sun_elevation_mdeg: 45_000,
        sun_azimuth_mdeg: 160_000,
        ground_sample_distance_cm: 1000,
        orbit: default_orbit(),
        bands: vec![],
        atmospheric_correction: AtmosphericCorrectionMethod::None,
        gcps: vec![],
    }
}

// ── 1. SpectralBand: all variants roundtrip ───────────────────────────────────
#[test]
fn test_spectral_band_all_variants() {
    let cfg = config::standard();
    let bands = [
        SpectralBand::Coastal,
        SpectralBand::Blue,
        SpectralBand::Green,
        SpectralBand::Red,
        SpectralBand::RedEdge,
        SpectralBand::NearInfrared,
        SpectralBand::ShortWaveInfrared1,
        SpectralBand::ShortWaveInfrared2,
        SpectralBand::Panchromatic,
        SpectralBand::Thermal,
    ];
    for band in &bands {
        let bytes = encode_to_vec(band, cfg).expect("encode SpectralBand variant");
        let (decoded, _): (SpectralBand, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode SpectralBand variant");
        assert_eq!(band, &decoded);
    }
}

// ── 2. LandCoverClass: all variants roundtrip ─────────────────────────────────
#[test]
fn test_land_cover_class_all_variants() {
    let cfg = config::standard();
    let classes = [
        LandCoverClass::Water,
        LandCoverClass::UrbanBuiltUp,
        LandCoverClass::Agriculture,
        LandCoverClass::Forest,
        LandCoverClass::Grassland,
        LandCoverClass::Barren,
        LandCoverClass::Snow,
        LandCoverClass::Wetland,
    ];
    for class in &classes {
        let bytes = encode_to_vec(class, cfg).expect("encode LandCoverClass variant");
        let (decoded, _): (LandCoverClass, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode LandCoverClass variant");
        assert_eq!(class, &decoded);
    }
}

// ── 3. OrbitType: all variants roundtrip ─────────────────────────────────────
#[test]
fn test_orbit_type_all_variants() {
    let cfg = config::standard();
    let orbit_types = [
        OrbitType::LowEarthOrbit,
        OrbitType::MediumEarthOrbit,
        OrbitType::GeostationaryOrbit,
        OrbitType::SunSynchronous,
        OrbitType::PolarOrbit,
    ];
    for ot in &orbit_types {
        let bytes = encode_to_vec(ot, cfg).expect("encode OrbitType variant");
        let (decoded, _): (OrbitType, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode OrbitType variant");
        assert_eq!(ot, &decoded);
    }
}

// ── 4. AtmosphericCorrectionMethod: all variants including SixS ──────────────
#[test]
fn test_atmospheric_correction_all_variants() {
    let cfg = config::standard();
    let methods = vec![
        AtmosphericCorrectionMethod::None,
        AtmosphericCorrectionMethod::DarkObjectSubtraction,
        AtmosphericCorrectionMethod::Flaash,
        AtmosphericCorrectionMethod::Quac,
        AtmosphericCorrectionMethod::SixS {
            aerosol_optical_depth: 150,
            water_vapor_cm: 22,
        },
    ];
    for method in &methods {
        let bytes = encode_to_vec(method, cfg).expect("encode AtmosphericCorrectionMethod variant");
        let (decoded, _): (AtmosphericCorrectionMethod, usize) =
            decode_owned_from_slice(&bytes, cfg)
                .expect("decode AtmosphericCorrectionMethod variant");
        assert_eq!(method, &decoded);
    }
}

// ── 5. GroundControlPoint with residual_m = Some ─────────────────────────────
#[test]
fn test_ground_control_point_with_residual() {
    let cfg = config::standard();
    let gcp = GroundControlPoint {
        gcp_id: 42,
        latitude_microdeg: 37_774_930,
        longitude_microdeg: -122_419_420,
        elevation_mm: 16_500,
        pixel_x: 4200.5,
        pixel_y: 3100.25,
        residual_m: Some(0.35),
    };
    let bytes = encode_to_vec(&gcp, cfg).expect("encode GroundControlPoint with residual");
    let (decoded, _): (GroundControlPoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode GroundControlPoint with residual");
    assert_eq!(gcp, decoded);
    assert!(decoded.residual_m.is_some());
}

// ── 6. GroundControlPoint with residual_m = None ─────────────────────────────
#[test]
fn test_ground_control_point_no_residual() {
    let cfg = config::standard();
    let gcp = make_gcp(7, 51_507_400, -122_000, 30_000);
    let bytes = encode_to_vec(&gcp, cfg).expect("encode GroundControlPoint no residual");
    let (decoded, _): (GroundControlPoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode GroundControlPoint no residual");
    assert_eq!(gcp, decoded);
    assert!(decoded.residual_m.is_none());
}

// ── 7. SpectralBandInfo roundtrip ─────────────────────────────────────────────
#[test]
fn test_spectral_band_info_roundtrip() {
    let cfg = config::standard();
    let info = make_band_info(SpectralBand::NearInfrared, 842, 115, 12);
    let bytes = encode_to_vec(&info, cfg).expect("encode SpectralBandInfo");
    let (decoded, _): (SpectralBandInfo, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SpectralBandInfo");
    assert_eq!(info, decoded);
    assert_eq!(decoded.center_wavelength_nm, 842);
}

// ── 8. ImageTile with pixel data vec ─────────────────────────────────────────
#[test]
fn test_image_tile_roundtrip() {
    let cfg = config::standard();
    let pixel_data: Vec<u16> = (0u16..256).collect();
    let tile = ImageTile {
        tile_id: 100_001,
        column: 3,
        row: 7,
        width_px: 256,
        height_px: 1,
        overlap_px: 16,
        pixel_data,
    };
    let bytes = encode_to_vec(&tile, cfg).expect("encode ImageTile");
    let (decoded, _): (ImageTile, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ImageTile");
    assert_eq!(tile, decoded);
    assert_eq!(decoded.pixel_data.len(), 256);
}

// ── 9. VegetationIndices roundtrip ───────────────────────────────────────────
#[test]
fn test_vegetation_indices_roundtrip() {
    let cfg = config::standard();
    let vi = VegetationIndices {
        ndvi: 6800,
        evi: 5200,
        savi: 4900,
        ndwi: -2100,
        nbr: 3100,
    };
    let bytes = encode_to_vec(&vi, cfg).expect("encode VegetationIndices");
    let (decoded, _): (VegetationIndices, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VegetationIndices");
    assert_eq!(vi, decoded);
    assert_eq!(decoded.ndvi, 6800);
}

// ── 10. OrbitParameters with revisit_time_hours = Some ───────────────────────
#[test]
fn test_orbit_parameters_with_revisit() {
    let cfg = config::standard();
    let orbit = default_orbit();
    let bytes = encode_to_vec(&orbit, cfg).expect("encode OrbitParameters");
    let (decoded, _): (OrbitParameters, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode OrbitParameters");
    assert_eq!(orbit, decoded);
    assert!(decoded.revisit_time_hours.is_some());
    assert_eq!(decoded.revisit_time_hours, Some(120));
}

// ── 11. OrbitParameters with revisit_time_hours = None ───────────────────────
#[test]
fn test_orbit_parameters_no_revisit() {
    let cfg = config::standard();
    let orbit = OrbitParameters {
        satellite_name: "GEO-IMAGER".to_string(),
        orbit_type: OrbitType::GeostationaryOrbit,
        altitude_km: 35_786,
        inclination_mdeg: 0,
        period_seconds: 86_400,
        revisit_time_hours: None,
    };
    let bytes = encode_to_vec(&orbit, cfg).expect("encode OrbitParameters no revisit");
    let (decoded, _): (OrbitParameters, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode OrbitParameters no revisit");
    assert_eq!(orbit, decoded);
    assert!(decoded.revisit_time_hours.is_none());
}

// ── 12. SceneMetadata minimal roundtrip (standard config) ────────────────────
#[test]
fn test_scene_metadata_basic_roundtrip() {
    let cfg = config::standard();
    let scene = make_minimal_scene("S2A_MSIL2A_20260301T100000", 1_740_830_400_000_000, 5);
    let bytes = encode_to_vec(&scene, cfg).expect("encode SceneMetadata basic");
    let (decoded, _): (SceneMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SceneMetadata basic");
    assert_eq!(scene, decoded);
    assert_eq!(decoded.cloud_cover_percent, 5);
}

// ── 13. SceneMetadata with full band stack ────────────────────────────────────
#[test]
fn test_scene_metadata_full_band_stack() {
    let cfg = config::standard();
    let mut scene = make_minimal_scene("S2B_MSIL2A_20260315T103000", 1_741_955_400_000_000, 2);
    scene.bands = vec![
        make_band_info(SpectralBand::Coastal, 443, 27, 12),
        make_band_info(SpectralBand::Blue, 490, 98, 12),
        make_band_info(SpectralBand::Green, 560, 45, 12),
        make_band_info(SpectralBand::Red, 665, 38, 12),
        make_band_info(SpectralBand::RedEdge, 705, 19, 12),
        make_band_info(SpectralBand::NearInfrared, 842, 115, 12),
        make_band_info(SpectralBand::ShortWaveInfrared1, 1610, 143, 12),
        make_band_info(SpectralBand::ShortWaveInfrared2, 2190, 242, 12),
        make_band_info(SpectralBand::Panchromatic, 500, 300, 12),
        make_band_info(SpectralBand::Thermal, 10900, 600, 8),
    ];
    let bytes = encode_to_vec(&scene, cfg).expect("encode SceneMetadata full bands");
    let (decoded, _): (SceneMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SceneMetadata full bands");
    assert_eq!(scene, decoded);
    assert_eq!(decoded.bands.len(), 10);
}

// ── 14. SceneMetadata big-endian config ──────────────────────────────────────
#[test]
fn test_scene_metadata_big_endian() {
    let cfg = config::standard().with_big_endian();
    let mut scene = make_minimal_scene("BE_SCENE_001", 1_740_000_000_000_000, 10);
    scene.atmospheric_correction = AtmosphericCorrectionMethod::SixS {
        aerosol_optical_depth: 200,
        water_vapor_cm: 18,
    };
    scene.gcps = vec![
        make_gcp(1, 48_856_613, 2_352_222, 35_000),
        make_gcp(2, 48_860_000, 2_355_000, 36_000),
    ];
    let bytes = encode_to_vec(&scene, cfg).expect("encode SceneMetadata big_endian");
    let (decoded, _): (SceneMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SceneMetadata big_endian");
    assert_eq!(scene, decoded);
}

// ── 15. SceneMetadata fixed-int config ───────────────────────────────────────
#[test]
fn test_scene_metadata_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let mut scene = make_minimal_scene("FI_SCENE_002", 1_741_000_000_000_000, 0);
    scene.atmospheric_correction = AtmosphericCorrectionMethod::Flaash;
    scene.bands = vec![
        make_band_info(SpectralBand::Red, 665, 38, 12),
        make_band_info(SpectralBand::Green, 560, 45, 12),
        make_band_info(SpectralBand::Blue, 490, 98, 12),
    ];
    let bytes = encode_to_vec(&scene, cfg).expect("encode SceneMetadata fixed_int");
    let (decoded, _): (SceneMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SceneMetadata fixed_int");
    assert_eq!(scene, decoded);
    assert_eq!(decoded.bands.len(), 3);
}

// ── 16. Vec<GroundControlPoint> roundtrip ────────────────────────────────────
#[test]
fn test_vec_ground_control_points_roundtrip() {
    let cfg = config::standard();
    let gcps: Vec<GroundControlPoint> = (0u32..8)
        .map(|i| GroundControlPoint {
            gcp_id: i,
            latitude_microdeg: (51_000_000 + i as i64 * 10_000),
            longitude_microdeg: (13_000_000 + i as i64 * 10_000),
            elevation_mm: 100_000 + (i as i32 * 500),
            pixel_x: (i as f32) * 512.0,
            pixel_y: (i as f32) * 512.0,
            residual_m: if i % 2 == 0 {
                Some(0.1 * i as f32)
            } else {
                None
            },
        })
        .collect();
    let bytes = encode_to_vec(&gcps, cfg).expect("encode Vec<GroundControlPoint>");
    let (decoded, _): (Vec<GroundControlPoint>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<GroundControlPoint>");
    assert_eq!(gcps, decoded);
    assert_eq!(decoded.len(), 8);
}

// ── 17. Vec<LandCoverSegment> roundtrip ──────────────────────────────────────
#[test]
fn test_vec_land_cover_segments_roundtrip() {
    let cfg = config::standard();
    let classes = [
        LandCoverClass::Forest,
        LandCoverClass::Agriculture,
        LandCoverClass::Water,
        LandCoverClass::UrbanBuiltUp,
        LandCoverClass::Grassland,
    ];
    let segments: Vec<LandCoverSegment> = classes
        .iter()
        .enumerate()
        .map(|(i, class)| LandCoverSegment {
            segment_id: i as u32,
            class: class.clone(),
            area_m2: 1_000_000 * (i as u64 + 1),
            confidence_pct: 85 + i as u8,
            centroid_lat_microdeg: 40_000_000 + i as i64 * 500_000,
            centroid_lon_microdeg: -75_000_000 + i as i64 * 500_000,
        })
        .collect();
    let bytes = encode_to_vec(&segments, cfg).expect("encode Vec<LandCoverSegment>");
    let (decoded, _): (Vec<LandCoverSegment>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<LandCoverSegment>");
    assert_eq!(segments, decoded);
    assert_eq!(decoded.len(), 5);
}

// ── 18. Large ImageTile: 512x512 pixels (262144 u16 values) ──────────────────
#[test]
fn test_large_image_tile_512x512() {
    let cfg = config::standard();
    let pixel_data: Vec<u16> = (0u32..262144).map(|i| (i % 4096) as u16).collect();
    let tile = ImageTile {
        tile_id: 999_999,
        column: 15,
        row: 23,
        width_px: 512,
        height_px: 512,
        overlap_px: 32,
        pixel_data,
    };
    let bytes = encode_to_vec(&tile, cfg).expect("encode large 512x512 ImageTile");
    let (decoded, _): (ImageTile, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode large 512x512 ImageTile");
    assert_eq!(tile, decoded);
    assert_eq!(decoded.pixel_data.len(), 262144);
    assert_eq!(decoded.width_px, 512);
    assert_eq!(decoded.height_px, 512);
}

// ── 19. Consumed bytes equals encoded length ──────────────────────────────────
#[test]
fn test_consumed_bytes_scene_metadata() {
    let cfg = config::standard();
    let mut scene = make_minimal_scene("CONSUMED_BYTES_CHECK", 1_740_500_000_000_000, 15);
    scene.bands = vec![
        make_band_info(SpectralBand::Red, 665, 38, 12),
        make_band_info(SpectralBand::NearInfrared, 842, 115, 12),
    ];
    scene.gcps = vec![
        make_gcp(1, 35_689_500, 139_691_700, 38_000),
        make_gcp(2, 35_692_000, 139_695_000, 40_000),
        make_gcp(3, 35_686_000, 139_688_000, 36_000),
    ];
    let bytes = encode_to_vec(&scene, cfg).expect("encode for consumed-bytes check");
    let (decoded, consumed): (SceneMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode for consumed-bytes check");
    assert_eq!(scene, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ── 20. SceneMetadata: DarkObjectSubtraction and Quac variants ───────────────
#[test]
fn test_scene_metadata_dark_object_and_quac() {
    let cfg = config::standard();
    let mut scene_dos = make_minimal_scene("DOS_SCENE_001", 1_740_600_000_000_000, 3);
    scene_dos.atmospheric_correction = AtmosphericCorrectionMethod::DarkObjectSubtraction;
    let mut scene_quac = make_minimal_scene("QUAC_SCENE_001", 1_740_700_000_000_000, 7);
    scene_quac.atmospheric_correction = AtmosphericCorrectionMethod::Quac;

    for scene in &[scene_dos, scene_quac] {
        let bytes = encode_to_vec(scene, cfg).expect("encode scene with correction method");
        let (decoded, _): (SceneMetadata, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode scene with correction method");
        assert_eq!(scene, &decoded);
    }
}

// ── 21. Nested: SceneMetadata with full GCPs having residuals ─────────────────
#[test]
fn test_scene_metadata_gcps_with_residuals() {
    let cfg = config::standard();
    let mut scene = make_minimal_scene("GCP_RESIDUAL_SCENE", 1_741_200_000_000_000, 8);
    scene.gcps = vec![
        GroundControlPoint {
            gcp_id: 1,
            latitude_microdeg: 52_520_000,
            longitude_microdeg: 13_405_000,
            elevation_mm: 34_000,
            pixel_x: 256.0,
            pixel_y: 256.0,
            residual_m: Some(0.12),
        },
        GroundControlPoint {
            gcp_id: 2,
            latitude_microdeg: 52_525_000,
            longitude_microdeg: 13_410_000,
            elevation_mm: 35_500,
            pixel_x: 768.0,
            pixel_y: 256.0,
            residual_m: Some(0.08),
        },
        GroundControlPoint {
            gcp_id: 3,
            latitude_microdeg: 52_515_000,
            longitude_microdeg: 13_400_000,
            elevation_mm: 33_000,
            pixel_x: 256.0,
            pixel_y: 768.0,
            residual_m: None,
        },
    ];
    let bytes = encode_to_vec(&scene, cfg).expect("encode scene with GCP residuals");
    let (decoded, _): (SceneMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode scene with GCP residuals");
    assert_eq!(scene, decoded);
    assert!(decoded.gcps[0].residual_m.is_some());
    assert!(decoded.gcps[1].residual_m.is_some());
    assert!(decoded.gcps[2].residual_m.is_none());
}

// ── 22. Combined config (big-endian + fixed-int) for full SceneMetadata ───────
#[test]
fn test_scene_metadata_combined_config() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let mut scene = make_minimal_scene("COMBINED_CFG_SCENE", 1_742_000_000_000_000, 12);
    scene.bands = vec![
        make_band_info(SpectralBand::Blue, 490, 98, 12),
        make_band_info(SpectralBand::Green, 560, 45, 12),
        make_band_info(SpectralBand::Red, 665, 38, 12),
    ];
    scene.atmospheric_correction = AtmosphericCorrectionMethod::SixS {
        aerosol_optical_depth: 180,
        water_vapor_cm: 25,
    };
    scene.gcps = vec![
        make_gcp(1, -33_868_800, 151_209_300, 5_000),
        make_gcp(2, -33_872_000, 151_215_000, 7_500),
    ];
    let bytes = encode_to_vec(&scene, cfg).expect("encode SceneMetadata combined config");
    let (decoded, consumed): (SceneMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SceneMetadata combined config");
    assert_eq!(scene, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must match with combined config"
    );
    assert_eq!(decoded.bands.len(), 3);
    assert_eq!(decoded.gcps.len(), 2);
}
