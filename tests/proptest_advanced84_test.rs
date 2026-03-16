//! Advanced property-based tests (set 84) -- Satellite Imagery & Remote Sensing domain.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers spectral bands, ground resolution cells, orbit parameters, SAR data,
//! NDVI vegetation indices, cloud cover classifications, georeferencing affine
//! transforms, tile coordinates, atmospheric correction parameters, change
//! detection results, DEM elevation grids, radiometric calibration, land-use
//! classifications, sun-synchronous orbit metadata, swath geometry, image
//! histograms, temporal composites, ground control points, pan-sharpening
//! parameters, thermal infrared readings, water body delineation, and
//! multi-sensor fusion records.

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

// ── Domain types ──────────────────────────────────────────────────────────────

/// A single spectral band measurement from a multispectral sensor.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpectralBand {
    /// Band identifier (e.g. 0=coastal, 1=blue, 2=green, ...).
    band_id: u8,
    /// Central wavelength in nanometres.
    wavelength_nm: u32,
    /// Reflectance value scaled to 0..10000 (0.0000 -- 1.0000).
    reflectance: u16,
    /// Signal-to-noise ratio.
    snr: f32,
    /// Gain coefficient applied during radiometric correction.
    gain: f64,
    /// Offset coefficient applied during radiometric correction.
    offset: f64,
}

/// Ground resolution cell -- the smallest addressable area on the ground.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GroundResolutionCell {
    /// Row index in the raster grid.
    row: u32,
    /// Column index in the raster grid.
    col: u32,
    /// Ground sample distance in metres.
    gsd_m: f32,
    /// Centre latitude (WGS-84).
    center_lat: f64,
    /// Centre longitude (WGS-84).
    center_lon: f64,
    /// Elevation above geoid in metres.
    elevation_m: f32,
}

/// Satellite orbit parameters (Keplerian elements).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrbitParameters {
    /// Semi-major axis in kilometres.
    semi_major_axis_km: f64,
    /// Eccentricity (0 = circular, <1 = elliptical).
    eccentricity: f64,
    /// Inclination in degrees (0..180).
    inclination_deg: f64,
    /// Right ascension of ascending node in degrees.
    raan_deg: f64,
    /// Argument of perigee in degrees.
    arg_perigee_deg: f64,
    /// Mean anomaly in degrees.
    mean_anomaly_deg: f64,
    /// Epoch as Unix timestamp (seconds).
    epoch_s: u64,
}

/// SAR (Synthetic Aperture Radar) backscatter measurement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SarBackscatter {
    /// Polarisation channel.
    polarisation: SarPolarisation,
    /// Backscatter coefficient sigma-nought in dB.
    sigma0_db: f32,
    /// Incidence angle in degrees.
    incidence_angle_deg: f32,
    /// Range pixel spacing in metres.
    range_spacing_m: f32,
    /// Azimuth pixel spacing in metres.
    azimuth_spacing_m: f32,
    /// Number of looks used.
    num_looks: u8,
}

/// SAR polarisation modes.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SarPolarisation {
    /// Horizontal transmit, horizontal receive.
    Hh,
    /// Horizontal transmit, vertical receive.
    Hv,
    /// Vertical transmit, horizontal receive.
    Vh,
    /// Vertical transmit, vertical receive.
    Vv,
}

/// NDVI (Normalized Difference Vegetation Index) measurement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NdviMeasurement {
    /// Pixel row.
    row: u32,
    /// Pixel column.
    col: u32,
    /// Red band reflectance (scaled 0..10000).
    red_reflectance: u16,
    /// NIR band reflectance (scaled 0..10000).
    nir_reflectance: u16,
    /// Computed NDVI value (-1.0 .. 1.0) stored as i16 scaled by 10000.
    ndvi_scaled: i16,
    /// Quality flag: 0=good, 1=cloud, 2=shadow, 3=water.
    quality_flag: u8,
}

/// Cloud cover classification for a scene tile.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CloudCoverClass {
    /// Clear sky (0--10% cloud).
    Clear { confidence_pct: u8 },
    /// Partly cloudy (10--50%).
    PartlyCloudy { coverage_pct: u8, altitude_km: f32 },
    /// Mostly cloudy (50--90%).
    MostlyCloudy { coverage_pct: u8, thickness_km: f32 },
    /// Overcast (90--100%).
    Overcast { optical_depth: f32 },
    /// Cloud shadow region.
    Shadow { shadow_length_m: f32 },
}

/// Affine georeferencing transform (maps pixel coords to map coords).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeoTransform {
    /// X origin (top-left corner easting).
    origin_x: f64,
    /// Y origin (top-left corner northing).
    origin_y: f64,
    /// Pixel width in map units.
    pixel_width: f64,
    /// Row rotation.
    row_rotation: f64,
    /// Column rotation.
    col_rotation: f64,
    /// Pixel height in map units (usually negative for north-up).
    pixel_height: f64,
}

/// Tile coordinate within a tiled imagery pyramid.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TileCoordinate {
    /// Zoom level (0 = whole earth).
    zoom: u8,
    /// Tile column index.
    tile_x: u32,
    /// Tile row index.
    tile_y: u32,
    /// Tile size in pixels (e.g. 256).
    tile_size_px: u16,
    /// Image format code (0=PNG, 1=JPEG, 2=TIFF, 3=WebP).
    format_code: u8,
}

/// Atmospheric correction parameters for a scene.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AtmosphericCorrection {
    /// Aerosol optical depth at 550 nm.
    aod_550: f32,
    /// Water vapour column in g/cm^2.
    water_vapour_g_cm2: f32,
    /// Ozone column in Dobson units.
    ozone_du: f32,
    /// Solar zenith angle in degrees.
    solar_zenith_deg: f32,
    /// View zenith angle in degrees.
    view_zenith_deg: f32,
    /// Relative azimuth between sun and sensor in degrees.
    relative_azimuth_deg: f32,
    /// Visibility in kilometres.
    visibility_km: f32,
}

/// Change detection result between two image acquisitions.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChangeDetectionResult {
    /// Pixel row.
    row: u32,
    /// Pixel column.
    col: u32,
    /// Pre-event value (reflectance or backscatter).
    pre_value: f32,
    /// Post-event value.
    post_value: f32,
    /// Magnitude of change.
    magnitude: f32,
    /// Change classification code.
    change_class: ChangeClass,
    /// Statistical confidence (0.0 -- 1.0) scaled to u16.
    confidence_scaled: u16,
}

/// Types of detected changes.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChangeClass {
    /// No significant change.
    NoChange,
    /// Urban expansion / new construction.
    UrbanExpansion,
    /// Deforestation.
    Deforestation,
    /// Flood / inundation.
    Flood,
    /// Agricultural conversion.
    Agriculture,
    /// Burnt area.
    BurntArea,
    /// Landslide.
    Landslide,
}

/// A row of DEM (Digital Elevation Model) samples.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DemRow {
    /// Row index in the DEM grid.
    row_index: u32,
    /// Starting longitude of this row in degrees * 1e7.
    start_lon_e7: i32,
    /// Elevation samples in centimetres above geoid.
    elevations_cm: Vec<i32>,
    /// Posting interval in arc-seconds.
    posting_arcsec: f32,
}

/// Radiometric calibration coefficients for a sensor band.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiometricCalibration {
    /// Band identifier.
    band_id: u8,
    /// Multiplicative rescaling factor (gain).
    mult_factor: f64,
    /// Additive rescaling factor (offset).
    add_factor: f64,
    /// K1 thermal constant (for thermal bands, 0 otherwise).
    k1_constant: f64,
    /// K2 thermal constant (for thermal bands, 0 otherwise).
    k2_constant: f64,
    /// Maximum quantised calibrated pixel value.
    qcal_max: u16,
}

/// Land-use / land-cover classification result.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LandUseClass {
    /// Water body.
    Water { depth_category: u8 },
    /// Forest.
    Forest {
        canopy_cover_pct: u8,
        tree_height_m: f32,
    },
    /// Grassland / shrubland.
    Grassland { biomass_kg_m2: f32 },
    /// Cropland.
    Cropland {
        crop_type_code: u16,
        growth_stage: u8,
    },
    /// Urban / built-up.
    Urban { impervious_pct: u8 },
    /// Barren / desert.
    Barren,
    /// Snow / ice.
    SnowIce { albedo: f32 },
}

/// Sun-synchronous orbit metadata for a scene.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SunSyncMetadata {
    /// Local time of descending node (hours * 100, e.g. 1030 = 10:30).
    ltdn_hhmm: u16,
    /// Repeat cycle in days.
    repeat_cycle_days: u16,
    /// Orbit altitude in kilometres.
    altitude_km: f32,
    /// Swath width in kilometres.
    swath_width_km: f32,
    /// Sensor name tag.
    sensor_name: String,
    /// Scene identifier.
    scene_id: u64,
}

/// Swath geometry description.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SwathGeometry {
    /// Near-range ground distance in kilometres.
    near_range_km: f32,
    /// Far-range ground distance in kilometres.
    far_range_km: f32,
    /// Along-track extent in kilometres.
    along_track_km: f32,
    /// Cross-track extent in kilometres.
    cross_track_km: f32,
    /// Number of detector rows.
    detector_rows: u16,
    /// Number of detector columns.
    detector_cols: u16,
}

/// Histogram bucket for an image band.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImageHistogramBucket {
    /// Band identifier.
    band_id: u8,
    /// Lower bound of the bucket (DN value).
    lower_dn: u16,
    /// Upper bound of the bucket (DN value).
    upper_dn: u16,
    /// Number of pixels in this bucket.
    count: u64,
    /// Cumulative fraction (0.0 -- 1.0) as scaled u32.
    cumulative_fraction_scaled: u32,
}

/// Temporal composite specification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TemporalComposite {
    /// Start timestamp (Unix seconds).
    start_epoch_s: u64,
    /// End timestamp (Unix seconds).
    end_epoch_s: u64,
    /// Number of input scenes used.
    scene_count: u16,
    /// Compositing method code (0=median, 1=mean, 2=max-NDVI, 3=min-cloud).
    method_code: u8,
    /// Target band count.
    band_count: u8,
    /// Output tile identifier.
    output_tile_id: u32,
}

/// Ground control point for geometric correction.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GroundControlPoint {
    /// GCP identifier.
    gcp_id: u32,
    /// Image pixel X.
    pixel_x: f64,
    /// Image pixel Y.
    pixel_y: f64,
    /// Map X (easting).
    map_x: f64,
    /// Map Y (northing).
    map_y: f64,
    /// Residual error in metres.
    residual_m: f32,
}

/// Pan-sharpening parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PanSharpenParams {
    /// Method code (0=Brovey, 1=IHS, 2=PCA, 3=Gram-Schmidt).
    method_code: u8,
    /// Pan band spatial resolution in metres.
    pan_res_m: f32,
    /// Multispectral resolution in metres.
    ms_res_m: f32,
    /// Number of bands to fuse.
    band_count: u8,
    /// Sharpening weight (0.0 -- 1.0).
    weight: f32,
    /// Output bit depth.
    output_bits: u8,
}

/// Thermal infrared reading from a satellite sensor.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThermalIrReading {
    /// Pixel row.
    row: u32,
    /// Pixel column.
    col: u32,
    /// Brightness temperature in Kelvin * 100.
    brightness_temp_k100: u32,
    /// Emissivity * 10000.
    emissivity_scaled: u16,
    /// Land surface temperature in Kelvin * 100.
    lst_k100: u32,
    /// Quality code (0=good, 1=thin cirrus, 2=cloudy).
    quality_code: u8,
}

/// Water body delineation result.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaterBodyDelineation {
    /// Feature identifier.
    feature_id: u64,
    /// Area in square metres.
    area_m2: f64,
    /// Perimeter in metres.
    perimeter_m: f64,
    /// MNDWI value scaled by 10000.
    mndwi_scaled: i16,
    /// Classification: 0=river, 1=lake, 2=reservoir, 3=wetland, 4=coastal.
    water_type: u8,
    /// Turbidity NTU * 100.
    turbidity_ntu100: u32,
}

/// Multi-sensor fusion record combining optical and radar data.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MultiSensorFusion {
    /// Optical reflectance (scaled 0..10000).
    optical_reflectance: u16,
    /// SAR backscatter in dB * 100 (stored as i32 to handle negative dB).
    sar_sigma0_db100: i32,
    /// Fusion weight for optical channel (0..100).
    optical_weight: u8,
    /// Fusion weight for radar channel (0..100).
    radar_weight: u8,
    /// Fused classification code.
    fused_class: u16,
    /// Confidence score (0..10000).
    confidence: u16,
    /// Timestamp of optical acquisition (Unix seconds).
    optical_epoch_s: u64,
    /// Timestamp of radar acquisition (Unix seconds).
    radar_epoch_s: u64,
}

// ── Arbitrary generators ─────────────────────────────────────────────────────

prop_compose! {
    fn arb_spectral_band()(
        band_id in 0u8..13,
        wavelength_nm in 400u32..2500,
        reflectance in 0u16..10000,
        snr in 10.0f32..500.0,
        gain in 0.001f64..2.0,
        offset in -100.0f64..100.0,
    ) -> SpectralBand {
        SpectralBand { band_id, wavelength_nm, reflectance, snr, gain, offset }
    }
}

prop_compose! {
    fn arb_ground_resolution_cell()(
        row in 0u32..50000,
        col in 0u32..50000,
        gsd_m in 0.3f32..30.0,
        center_lat in -90.0f64..90.0,
        center_lon in -180.0f64..180.0,
        elevation_m in -500.0f32..9000.0,
    ) -> GroundResolutionCell {
        GroundResolutionCell { row, col, gsd_m, center_lat, center_lon, elevation_m }
    }
}

prop_compose! {
    fn arb_orbit_parameters()(
        semi_major_axis_km in 6400.0f64..42200.0,
        eccentricity in 0.0f64..0.9,
        inclination_deg in 0.0f64..180.0,
        raan_deg in 0.0f64..360.0,
        arg_perigee_deg in 0.0f64..360.0,
        mean_anomaly_deg in 0.0f64..360.0,
        epoch_s in 1_000_000_000u64..2_000_000_000,
    ) -> OrbitParameters {
        OrbitParameters {
            semi_major_axis_km, eccentricity, inclination_deg,
            raan_deg, arg_perigee_deg, mean_anomaly_deg, epoch_s,
        }
    }
}

fn arb_sar_polarisation() -> impl Strategy<Value = SarPolarisation> {
    prop_oneof![
        Just(SarPolarisation::Hh),
        Just(SarPolarisation::Hv),
        Just(SarPolarisation::Vh),
        Just(SarPolarisation::Vv),
    ]
}

prop_compose! {
    fn arb_sar_backscatter()(
        polarisation in arb_sar_polarisation(),
        sigma0_db in -30.0f32..10.0,
        incidence_angle_deg in 15.0f32..60.0,
        range_spacing_m in 1.0f32..40.0,
        azimuth_spacing_m in 1.0f32..40.0,
        num_looks in 1u8..16,
    ) -> SarBackscatter {
        SarBackscatter {
            polarisation, sigma0_db, incidence_angle_deg,
            range_spacing_m, azimuth_spacing_m, num_looks,
        }
    }
}

prop_compose! {
    fn arb_ndvi_measurement()(
        row in 0u32..100000,
        col in 0u32..100000,
        red_reflectance in 0u16..10000,
        nir_reflectance in 0u16..10000,
        ndvi_scaled in -10000i16..10000,
        quality_flag in 0u8..4,
    ) -> NdviMeasurement {
        NdviMeasurement { row, col, red_reflectance, nir_reflectance, ndvi_scaled, quality_flag }
    }
}

fn arb_cloud_cover_class() -> impl Strategy<Value = CloudCoverClass> {
    prop_oneof![
        (0u8..100).prop_map(|c| CloudCoverClass::Clear { confidence_pct: c }),
        (10u8..50, 1.0f32..15.0).prop_map(|(p, a)| CloudCoverClass::PartlyCloudy {
            coverage_pct: p,
            altitude_km: a,
        }),
        (50u8..90, 0.1f32..5.0).prop_map(|(p, t)| CloudCoverClass::MostlyCloudy {
            coverage_pct: p,
            thickness_km: t,
        }),
        (0.1f32..100.0).prop_map(|d| CloudCoverClass::Overcast { optical_depth: d }),
        (10.0f32..5000.0).prop_map(|s| CloudCoverClass::Shadow { shadow_length_m: s }),
    ]
}

prop_compose! {
    fn arb_geo_transform()(
        origin_x in -20_000_000.0f64..20_000_000.0,
        origin_y in -20_000_000.0f64..20_000_000.0,
        pixel_width in 0.1f64..1000.0,
        row_rotation in -0.01f64..0.01,
        col_rotation in -0.01f64..0.01,
        pixel_height in -1000.0f64..-0.1,
    ) -> GeoTransform {
        GeoTransform { origin_x, origin_y, pixel_width, row_rotation, col_rotation, pixel_height }
    }
}

prop_compose! {
    fn arb_tile_coordinate()(
        zoom in 0u8..22,
        tile_x in 0u32..1_000_000,
        tile_y in 0u32..1_000_000,
        tile_size_px in prop_oneof![Just(256u16), Just(512u16)],
        format_code in 0u8..4,
    ) -> TileCoordinate {
        TileCoordinate { zoom, tile_x, tile_y, tile_size_px, format_code }
    }
}

prop_compose! {
    fn arb_atmospheric_correction()(
        aod_550 in 0.01f32..3.0,
        water_vapour_g_cm2 in 0.1f32..7.0,
        ozone_du in 200.0f32..500.0,
        solar_zenith_deg in 0.0f32..85.0,
        view_zenith_deg in 0.0f32..70.0,
        relative_azimuth_deg in 0.0f32..360.0,
        visibility_km in 1.0f32..300.0,
    ) -> AtmosphericCorrection {
        AtmosphericCorrection {
            aod_550, water_vapour_g_cm2, ozone_du,
            solar_zenith_deg, view_zenith_deg, relative_azimuth_deg, visibility_km,
        }
    }
}

fn arb_change_class() -> impl Strategy<Value = ChangeClass> {
    prop_oneof![
        Just(ChangeClass::NoChange),
        Just(ChangeClass::UrbanExpansion),
        Just(ChangeClass::Deforestation),
        Just(ChangeClass::Flood),
        Just(ChangeClass::Agriculture),
        Just(ChangeClass::BurntArea),
        Just(ChangeClass::Landslide),
    ]
}

prop_compose! {
    fn arb_change_detection_result()(
        row in 0u32..100000,
        col in 0u32..100000,
        pre_value in 0.0f32..1.0,
        post_value in 0.0f32..1.0,
        magnitude in 0.0f32..2.0,
        change_class in arb_change_class(),
        confidence_scaled in 0u16..10000,
    ) -> ChangeDetectionResult {
        ChangeDetectionResult {
            row, col, pre_value, post_value, magnitude, change_class, confidence_scaled,
        }
    }
}

prop_compose! {
    fn arb_dem_row()(
        row_index in 0u32..50000,
        start_lon_e7 in -1_800_000_000i32..1_800_000_000,
        elevation_count in 1usize..32,
        posting_arcsec in prop_oneof![Just(1.0f32), Just(3.0f32), Just(30.0f32)],
    )(
        elevations_cm in proptest::collection::vec(-50000i32..900000, elevation_count),
        row_index in Just(row_index),
        start_lon_e7 in Just(start_lon_e7),
        posting_arcsec in Just(posting_arcsec),
    ) -> DemRow {
        DemRow { row_index, start_lon_e7, elevations_cm, posting_arcsec }
    }
}

prop_compose! {
    fn arb_radiometric_calibration()(
        band_id in 0u8..13,
        mult_factor in 0.0001f64..0.1,
        add_factor in -100.0f64..100.0,
        k1_constant in 0.0f64..2000.0,
        k2_constant in 0.0f64..2000.0,
        qcal_max in 255u16..65535,
    ) -> RadiometricCalibration {
        RadiometricCalibration { band_id, mult_factor, add_factor, k1_constant, k2_constant, qcal_max }
    }
}

fn arb_land_use_class() -> impl Strategy<Value = LandUseClass> {
    prop_oneof![
        (0u8..4).prop_map(|d| LandUseClass::Water { depth_category: d }),
        (0u8..100, 1.0f32..60.0).prop_map(|(c, h)| LandUseClass::Forest {
            canopy_cover_pct: c,
            tree_height_m: h,
        }),
        (0.1f32..50.0).prop_map(|b| LandUseClass::Grassland { biomass_kg_m2: b }),
        (0u16..500, 0u8..10).prop_map(|(c, g)| LandUseClass::Cropland {
            crop_type_code: c,
            growth_stage: g,
        }),
        (0u8..100).prop_map(|i| LandUseClass::Urban { impervious_pct: i }),
        Just(LandUseClass::Barren),
        (0.1f32..1.0).prop_map(|a| LandUseClass::SnowIce { albedo: a }),
    ]
}

prop_compose! {
    fn arb_sun_sync_metadata()(
        ltdn_hhmm in 600u16..1400,
        repeat_cycle_days in 1u16..46,
        altitude_km in 400.0f32..900.0,
        swath_width_km in 10.0f32..3000.0,
        sensor_name in "[A-Z]{3,8}",
        scene_id in any::<u64>(),
    ) -> SunSyncMetadata {
        SunSyncMetadata {
            ltdn_hhmm, repeat_cycle_days, altitude_km,
            swath_width_km, sensor_name, scene_id,
        }
    }
}

prop_compose! {
    fn arb_swath_geometry()(
        near_range_km in 100.0f32..500.0,
        far_range_km in 500.0f32..1500.0,
        along_track_km in 10.0f32..3000.0,
        cross_track_km in 10.0f32..600.0,
        detector_rows in 1u16..10000,
        detector_cols in 1u16..20000,
    ) -> SwathGeometry {
        SwathGeometry {
            near_range_km, far_range_km, along_track_km,
            cross_track_km, detector_rows, detector_cols,
        }
    }
}

prop_compose! {
    fn arb_image_histogram_bucket()(
        band_id in 0u8..13,
        lower_dn in 0u16..65000,
        span in 1u16..256,
        count in any::<u64>(),
        cumulative_fraction_scaled in 0u32..1_000_000,
    ) -> ImageHistogramBucket {
        let upper_dn = lower_dn.saturating_add(span);
        ImageHistogramBucket { band_id, lower_dn, upper_dn, count, cumulative_fraction_scaled }
    }
}

prop_compose! {
    fn arb_temporal_composite()(
        start_epoch_s in 1_500_000_000u64..1_700_000_000,
        duration_s in 86400u64..2_592_000,
        scene_count in 2u16..365,
        method_code in 0u8..4,
        band_count in 1u8..13,
        output_tile_id in any::<u32>(),
    ) -> TemporalComposite {
        TemporalComposite {
            start_epoch_s,
            end_epoch_s: start_epoch_s + duration_s,
            scene_count, method_code, band_count, output_tile_id,
        }
    }
}

prop_compose! {
    fn arb_ground_control_point()(
        gcp_id in any::<u32>(),
        pixel_x in 0.0f64..100000.0,
        pixel_y in 0.0f64..100000.0,
        map_x in -20_000_000.0f64..20_000_000.0,
        map_y in -20_000_000.0f64..20_000_000.0,
        residual_m in 0.0f32..50.0,
    ) -> GroundControlPoint {
        GroundControlPoint { gcp_id, pixel_x, pixel_y, map_x, map_y, residual_m }
    }
}

prop_compose! {
    fn arb_pan_sharpen_params()(
        method_code in 0u8..4,
        pan_res_m in 0.3f32..2.0,
        ms_res_m in 2.0f32..30.0,
        band_count in 3u8..8,
        weight in 0.0f32..1.0,
        output_bits in prop_oneof![Just(8u8), Just(16u8)],
    ) -> PanSharpenParams {
        PanSharpenParams { method_code, pan_res_m, ms_res_m, band_count, weight, output_bits }
    }
}

prop_compose! {
    fn arb_thermal_ir_reading()(
        row in 0u32..50000,
        col in 0u32..50000,
        brightness_temp_k100 in 20000u32..35000,
        emissivity_scaled in 8000u16..10000,
        lst_k100 in 22000u32..34000,
        quality_code in 0u8..3,
    ) -> ThermalIrReading {
        ThermalIrReading {
            row, col, brightness_temp_k100, emissivity_scaled, lst_k100, quality_code,
        }
    }
}

prop_compose! {
    fn arb_water_body_delineation()(
        feature_id in any::<u64>(),
        area_m2 in 100.0f64..1_000_000_000.0,
        perimeter_m in 40.0f64..1_000_000.0,
        mndwi_scaled in -10000i16..10000,
        water_type in 0u8..5,
        turbidity_ntu100 in 0u32..100000,
    ) -> WaterBodyDelineation {
        WaterBodyDelineation {
            feature_id, area_m2, perimeter_m, mndwi_scaled, water_type, turbidity_ntu100,
        }
    }
}

prop_compose! {
    fn arb_multi_sensor_fusion()(
        optical_reflectance in 0u16..10000,
        sar_sigma0_db100 in -3000i32..1000,
        optical_weight in 0u8..101,
        radar_weight in 0u8..101,
        fused_class in 0u16..256,
        confidence in 0u16..10001,
        optical_epoch_s in 1_500_000_000u64..2_000_000_000,
        radar_epoch_s in 1_500_000_000u64..2_000_000_000,
    ) -> MultiSensorFusion {
        MultiSensorFusion {
            optical_reflectance, sar_sigma0_db100, optical_weight, radar_weight,
            fused_class, confidence, optical_epoch_s, radar_epoch_s,
        }
    }
}

// ── Tests (22 total) ─────────────────────────────────────────────────────────

#[test]
fn test_spectral_band_roundtrip() {
    proptest!(|(val in arb_spectral_band())| {
        let bytes = encode_to_vec(&val).expect("encode spectral band");
        let (decoded, _) = decode_from_slice::<SpectralBand>(&bytes).expect("decode spectral band");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_ground_resolution_cell_roundtrip() {
    proptest!(|(val in arb_ground_resolution_cell())| {
        let bytes = encode_to_vec(&val).expect("encode ground resolution cell");
        let (decoded, _) = decode_from_slice::<GroundResolutionCell>(&bytes).expect("decode ground resolution cell");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_orbit_parameters_roundtrip() {
    proptest!(|(val in arb_orbit_parameters())| {
        let bytes = encode_to_vec(&val).expect("encode orbit parameters");
        let (decoded, _) = decode_from_slice::<OrbitParameters>(&bytes).expect("decode orbit parameters");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_sar_backscatter_roundtrip() {
    proptest!(|(val in arb_sar_backscatter())| {
        let bytes = encode_to_vec(&val).expect("encode SAR backscatter");
        let (decoded, _) = decode_from_slice::<SarBackscatter>(&bytes).expect("decode SAR backscatter");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_ndvi_measurement_roundtrip() {
    proptest!(|(val in arb_ndvi_measurement())| {
        let bytes = encode_to_vec(&val).expect("encode NDVI measurement");
        let (decoded, _) = decode_from_slice::<NdviMeasurement>(&bytes).expect("decode NDVI measurement");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_cloud_cover_class_roundtrip() {
    proptest!(|(val in arb_cloud_cover_class())| {
        let bytes = encode_to_vec(&val).expect("encode cloud cover class");
        let (decoded, _) = decode_from_slice::<CloudCoverClass>(&bytes).expect("decode cloud cover class");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_geo_transform_roundtrip() {
    proptest!(|(val in arb_geo_transform())| {
        let bytes = encode_to_vec(&val).expect("encode geo transform");
        let (decoded, _) = decode_from_slice::<GeoTransform>(&bytes).expect("decode geo transform");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_tile_coordinate_roundtrip() {
    proptest!(|(val in arb_tile_coordinate())| {
        let bytes = encode_to_vec(&val).expect("encode tile coordinate");
        let (decoded, _) = decode_from_slice::<TileCoordinate>(&bytes).expect("decode tile coordinate");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_atmospheric_correction_roundtrip() {
    proptest!(|(val in arb_atmospheric_correction())| {
        let bytes = encode_to_vec(&val).expect("encode atmospheric correction");
        let (decoded, _) = decode_from_slice::<AtmosphericCorrection>(&bytes).expect("decode atmospheric correction");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_change_detection_result_roundtrip() {
    proptest!(|(val in arb_change_detection_result())| {
        let bytes = encode_to_vec(&val).expect("encode change detection result");
        let (decoded, _) = decode_from_slice::<ChangeDetectionResult>(&bytes).expect("decode change detection result");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_dem_row_roundtrip() {
    proptest!(|(val in arb_dem_row())| {
        let bytes = encode_to_vec(&val).expect("encode DEM row");
        let (decoded, _) = decode_from_slice::<DemRow>(&bytes).expect("decode DEM row");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_radiometric_calibration_roundtrip() {
    proptest!(|(val in arb_radiometric_calibration())| {
        let bytes = encode_to_vec(&val).expect("encode radiometric calibration");
        let (decoded, _) = decode_from_slice::<RadiometricCalibration>(&bytes).expect("decode radiometric calibration");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_land_use_class_roundtrip() {
    proptest!(|(val in arb_land_use_class())| {
        let bytes = encode_to_vec(&val).expect("encode land-use class");
        let (decoded, _) = decode_from_slice::<LandUseClass>(&bytes).expect("decode land-use class");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_sun_sync_metadata_roundtrip() {
    proptest!(|(val in arb_sun_sync_metadata())| {
        let bytes = encode_to_vec(&val).expect("encode sun-sync metadata");
        let (decoded, _) = decode_from_slice::<SunSyncMetadata>(&bytes).expect("decode sun-sync metadata");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_swath_geometry_roundtrip() {
    proptest!(|(val in arb_swath_geometry())| {
        let bytes = encode_to_vec(&val).expect("encode swath geometry");
        let (decoded, _) = decode_from_slice::<SwathGeometry>(&bytes).expect("decode swath geometry");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_image_histogram_bucket_roundtrip() {
    proptest!(|(val in arb_image_histogram_bucket())| {
        let bytes = encode_to_vec(&val).expect("encode image histogram bucket");
        let (decoded, _) = decode_from_slice::<ImageHistogramBucket>(&bytes).expect("decode image histogram bucket");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_temporal_composite_roundtrip() {
    proptest!(|(val in arb_temporal_composite())| {
        let bytes = encode_to_vec(&val).expect("encode temporal composite");
        let (decoded, _) = decode_from_slice::<TemporalComposite>(&bytes).expect("decode temporal composite");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_ground_control_point_roundtrip() {
    proptest!(|(val in arb_ground_control_point())| {
        let bytes = encode_to_vec(&val).expect("encode ground control point");
        let (decoded, _) = decode_from_slice::<GroundControlPoint>(&bytes).expect("decode ground control point");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_pan_sharpen_params_roundtrip() {
    proptest!(|(val in arb_pan_sharpen_params())| {
        let bytes = encode_to_vec(&val).expect("encode pan-sharpen params");
        let (decoded, _) = decode_from_slice::<PanSharpenParams>(&bytes).expect("decode pan-sharpen params");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_thermal_ir_reading_roundtrip() {
    proptest!(|(val in arb_thermal_ir_reading())| {
        let bytes = encode_to_vec(&val).expect("encode thermal IR reading");
        let (decoded, _) = decode_from_slice::<ThermalIrReading>(&bytes).expect("decode thermal IR reading");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_water_body_delineation_roundtrip() {
    proptest!(|(val in arb_water_body_delineation())| {
        let bytes = encode_to_vec(&val).expect("encode water body delineation");
        let (decoded, _) = decode_from_slice::<WaterBodyDelineation>(&bytes).expect("decode water body delineation");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_multi_sensor_fusion_roundtrip() {
    proptest!(|(val in arb_multi_sensor_fusion())| {
        let bytes = encode_to_vec(&val).expect("encode multi-sensor fusion");
        let (decoded, _) = decode_from_slice::<MultiSensorFusion>(&bytes).expect("decode multi-sensor fusion");
        prop_assert_eq!(val, decoded);
    });
}
