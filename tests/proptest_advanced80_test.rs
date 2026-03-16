//! Advanced property-based tests (set 80) — Observational Astronomy & Telescope Operations domain.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers celestial coordinates (RA/Dec, Alt/Az), CCD image metadata, photometric
//! measurements, spectroscopic observations, telescope pointing models, adaptive
//! optics parameters, guide star selections, observation queue scheduling, weather
//! station readings, data pipeline reduction steps, asteroid orbit determinations,
//! exoplanet transit detections, radio interferometry baselines, dome/shutter status,
//! and calibration frame libraries.

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

/// Equatorial celestial coordinate (J2000 epoch).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EquatorialCoord {
    /// Right ascension in hours (0.0 – 24.0).
    ra_hours: f64,
    /// Declination in degrees (-90.0 – +90.0).
    dec_deg: f64,
    /// Epoch expressed as Julian year (e.g. 2000.0).
    epoch_jy: f64,
}

/// Horizontal (Alt/Az) coordinate.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HorizontalCoord {
    /// Altitude in degrees (0.0 – 90.0).
    alt_deg: f64,
    /// Azimuth in degrees (0.0 – 360.0).
    az_deg: f64,
    /// Atmospheric refraction correction in arc-seconds.
    refraction_arcsec: f64,
}

/// CCD image metadata captured during an exposure.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CcdImageMeta {
    /// Image serial number.
    image_id: u64,
    /// Exposure duration in seconds.
    exposure_sec: f64,
    /// Filter name index (0=U, 1=B, 2=V, 3=R, 4=I, 5=Ha, 6=OIII, 7=SII).
    filter_index: u8,
    /// Binning factor (1, 2, 4).
    binning: u8,
    /// CCD temperature in degrees Celsius.
    ccd_temp_c: f32,
    /// Gain in electrons per ADU.
    gain_e_per_adu: f32,
    /// Read noise in electrons RMS.
    read_noise_e: f32,
    /// Image width in pixels.
    width_px: u16,
    /// Image height in pixels.
    height_px: u16,
}

/// Photometric magnitude measurement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PhotometricMeasurement {
    /// Star catalog identifier.
    star_id: u64,
    /// Apparent magnitude.
    magnitude: f64,
    /// Magnitude uncertainty (1-sigma).
    mag_error: f64,
    /// B-V colour index.
    bv_color_index: f64,
    /// Air mass at observation time.
    airmass: f64,
    /// Photometric zero point.
    zero_point: f64,
}

/// Spectroscopic observation parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpectroscopicObs {
    /// Target identifier.
    target_id: u64,
    /// Start wavelength in Angstroms.
    lambda_start_a: f64,
    /// End wavelength in Angstroms.
    lambda_end_a: f64,
    /// Spectral resolution (R = lambda / delta_lambda).
    resolution: f64,
    /// Slit width in arc-seconds.
    slit_width_arcsec: f64,
    /// Signal-to-noise ratio achieved.
    snr: f64,
    /// Number of co-added frames.
    num_coadds: u16,
}

/// Telescope pointing model term.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PointingModelTerm {
    /// Term name hash.
    term_id: u32,
    /// Coefficient value in arc-seconds.
    coeff_arcsec: f64,
    /// Coefficient uncertainty in arc-seconds.
    coeff_error_arcsec: f64,
    /// Whether term is enabled in the active model.
    enabled: bool,
}

/// Full telescope pointing model.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PointingModel {
    /// Model revision number.
    revision: u32,
    /// Number of alignment stars used.
    num_stars_used: u16,
    /// RMS residual in arc-seconds.
    rms_arcsec: f64,
    /// Individual model terms.
    terms: Vec<PointingModelTerm>,
}

/// Adaptive optics loop parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AdaptiveOpticsParams {
    /// AO system identifier.
    ao_id: u32,
    /// Loop frequency in Hz.
    loop_freq_hz: f64,
    /// Loop gain (0.0 – 1.0).
    loop_gain: f64,
    /// Strehl ratio achieved (0.0 – 1.0).
    strehl_ratio: f64,
    /// Wavefront sensor type (0=SH, 1=Pyramid, 2=Curvature).
    wfs_type: u8,
    /// Number of actuators.
    num_actuators: u16,
    /// Deformable mirror stroke in micrometers.
    dm_stroke_um: f64,
    /// Loop closed.
    loop_closed: bool,
}

/// Guide star selection record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GuideStarSelection {
    /// Catalog star identifier.
    catalog_id: u64,
    /// Star magnitude in guide band.
    guide_mag: f64,
    /// Angular separation from science target in arc-minutes.
    sep_arcmin: f64,
    /// Position angle from science target in degrees.
    pa_deg: f64,
    /// Proper motion RA in mas/yr.
    pm_ra_mas_yr: f64,
    /// Proper motion Dec in mas/yr.
    pm_dec_mas_yr: f64,
    /// Selected as primary guide star.
    is_primary: bool,
}

/// Observation queue entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ObsQueueEntry {
    /// Observation block identifier.
    ob_id: u64,
    /// Priority (lower = higher priority).
    priority: u16,
    /// Target coordinates.
    target: EquatorialCoord,
    /// Required seeing constraint in arc-seconds.
    max_seeing_arcsec: f64,
    /// Minimum elevation constraint in degrees.
    min_elevation_deg: f64,
    /// Requested exposure time in seconds.
    requested_exp_sec: f64,
    /// Whether observation is time-critical.
    time_critical: bool,
    /// Lunar distance constraint in degrees.
    min_lunar_dist_deg: f64,
}

/// Weather station reading.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherReading {
    /// Station timestamp (Unix seconds).
    timestamp_s: u64,
    /// Atmospheric seeing in arc-seconds (FWHM).
    seeing_arcsec: f64,
    /// Cloud cover fraction (0.0 – 1.0).
    cloud_cover: f64,
    /// Relative humidity (0.0 – 1.0).
    humidity: f64,
    /// Wind speed in m/s.
    wind_speed_ms: f64,
    /// Wind direction in degrees.
    wind_dir_deg: f64,
    /// Barometric pressure in hPa.
    pressure_hpa: f64,
    /// Ambient temperature in degrees Celsius.
    ambient_temp_c: f64,
    /// Dew point in degrees Celsius.
    dew_point_c: f64,
}

/// Data reduction pipeline step.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReductionStep {
    /// Bias subtraction.
    BiasSubtract { master_bias_id: u32 },
    /// Dark current subtraction.
    DarkSubtract {
        master_dark_id: u32,
        scale_factor: f64,
    },
    /// Flat field correction.
    FlatField {
        master_flat_id: u32,
        filter_index: u8,
    },
    /// Cosmic ray rejection.
    CosmicRayReject { sigma_clip: f64, max_iterations: u8 },
    /// Sky background subtraction.
    SkySubtract { mesh_size: u16, sigma: f64 },
    /// Astrometric solution.
    AstrometricSolve {
        catalog_id: u32,
        match_radius_arcsec: f64,
    },
    /// Photometric calibration.
    PhotometricCalib {
        zero_point: f64,
        extinction_coeff: f64,
    },
}

/// Reduction pipeline configuration.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReductionPipeline {
    /// Pipeline run identifier.
    run_id: u64,
    /// Ordered reduction steps.
    steps: Vec<ReductionStep>,
}

/// Asteroid orbital elements.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AsteroidOrbit {
    /// Minor planet designation hash.
    designation_hash: u64,
    /// Semi-major axis in AU.
    semi_major_axis_au: f64,
    /// Eccentricity.
    eccentricity: f64,
    /// Inclination in degrees.
    inclination_deg: f64,
    /// Longitude of ascending node in degrees.
    lon_asc_node_deg: f64,
    /// Argument of perihelion in degrees.
    arg_perihelion_deg: f64,
    /// Mean anomaly at epoch in degrees.
    mean_anomaly_deg: f64,
    /// Epoch as MJD.
    epoch_mjd: f64,
    /// Absolute magnitude H.
    abs_mag_h: f64,
    /// Number of observations used.
    num_obs: u32,
}

/// Exoplanet transit detection event.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TransitDetection {
    /// Host star identifier.
    host_star_id: u64,
    /// Transit midpoint (BJD_TDB).
    midpoint_bjd: f64,
    /// Transit depth (fractional flux decrease).
    depth_ppm: f64,
    /// Transit duration in hours.
    duration_hours: f64,
    /// Impact parameter (0.0 – 1.0+).
    impact_param: f64,
    /// Orbital period in days.
    period_days: f64,
    /// Planet radius in Earth radii (derived).
    radius_r_earth: f64,
    /// Signal detection efficiency.
    sde: f64,
}

/// Radio interferometry baseline.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InterferometryBaseline {
    /// Antenna pair (first).
    antenna_a: u16,
    /// Antenna pair (second).
    antenna_b: u16,
    /// Baseline length in meters.
    length_m: f64,
    /// East component in meters.
    east_m: f64,
    /// North component in meters.
    north_m: f64,
    /// Up component in meters.
    up_m: f64,
    /// Observing frequency in GHz.
    freq_ghz: f64,
    /// UV coverage sampling weight.
    uv_weight: f64,
}

/// Dome and shutter operational status.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DomeShutterStatus {
    /// Dome parked, shutter closed.
    Parked,
    /// Dome slewing to target azimuth.
    Slewing { target_az_deg: f64 },
    /// Dome tracking telescope, shutter open.
    Tracking {
        current_az_deg: f64,
        slit_width_deg: f64,
    },
    /// Shutter opening.
    ShutterOpening { percent_open: f64 },
    /// Shutter closing (emergency or normal).
    ShutterClosing { percent_open: f64, emergency: bool },
    /// Dome in maintenance mode.
    Maintenance { fault_code: u32 },
}

/// Calibration frame library entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CalibrationFrame {
    /// Frame identifier.
    frame_id: u64,
    /// Frame type (0=Bias, 1=Dark, 2=Flat, 3=Arc, 4=Standard).
    frame_type: u8,
    /// Filter index (for flats).
    filter_index: u8,
    /// Exposure time in seconds (for darks).
    exposure_sec: f64,
    /// CCD temperature at capture in Celsius.
    ccd_temp_c: f32,
    /// Number of frames combined.
    num_combined: u16,
    /// Creation timestamp (Unix seconds).
    created_s: u64,
    /// Valid until timestamp (Unix seconds).
    valid_until_s: u64,
}

/// Calibration library (collection of frames).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CalibrationLibrary {
    /// Library identifier.
    library_id: u32,
    /// Telescope identifier.
    telescope_id: u32,
    /// Frames in the library.
    frames: Vec<CalibrationFrame>,
}

// ── prop_compose! strategies ────────────────────────────────────────────────

prop_compose! {
    fn arb_equatorial_coord()(
        ra_hours in 0.0f64..24.0,
        dec_deg in -90.0f64..90.0,
        epoch_jy in 1950.0f64..2100.0,
    ) -> EquatorialCoord {
        EquatorialCoord { ra_hours, dec_deg, epoch_jy }
    }
}

prop_compose! {
    fn arb_horizontal_coord()(
        alt_deg in 0.0f64..90.0,
        az_deg in 0.0f64..360.0,
        refraction_arcsec in 0.0f64..120.0,
    ) -> HorizontalCoord {
        HorizontalCoord { alt_deg, az_deg, refraction_arcsec }
    }
}

prop_compose! {
    fn arb_ccd_image_meta()(
        image_id in any::<u64>(),
        exposure_sec in 0.001f64..3600.0,
        filter_index in 0u8..8,
        binning in prop_oneof![Just(1u8), Just(2u8), Just(4u8)],
        ccd_temp_c in -60.0f32..0.0,
        gain_e_per_adu in 0.1f32..10.0,
        read_noise_e in 1.0f32..30.0,
        width_px in 512u16..8192,
        height_px in 512u16..8192,
    ) -> CcdImageMeta {
        CcdImageMeta {
            image_id, exposure_sec, filter_index, binning,
            ccd_temp_c, gain_e_per_adu, read_noise_e, width_px, height_px,
        }
    }
}

prop_compose! {
    fn arb_photometric()(
        star_id in any::<u64>(),
        magnitude in -2.0f64..25.0,
        mag_error in 0.001f64..1.0,
        bv_color_index in -0.5f64..2.5,
        airmass in 1.0f64..4.0,
        zero_point in 20.0f64..30.0,
    ) -> PhotometricMeasurement {
        PhotometricMeasurement {
            star_id, magnitude, mag_error, bv_color_index, airmass, zero_point,
        }
    }
}

prop_compose! {
    fn arb_spectroscopic()(
        target_id in any::<u64>(),
        lambda_start_a in 3000.0f64..5000.0,
        lambda_end_a in 5000.0f64..11000.0,
        resolution in 500.0f64..150000.0,
        slit_width_arcsec in 0.1f64..5.0,
        snr in 1.0f64..1000.0,
        num_coadds in 1u16..100,
    ) -> SpectroscopicObs {
        SpectroscopicObs {
            target_id, lambda_start_a, lambda_end_a, resolution,
            slit_width_arcsec, snr, num_coadds,
        }
    }
}

prop_compose! {
    fn arb_pointing_term()(
        term_id in any::<u32>(),
        coeff_arcsec in -300.0f64..300.0,
        coeff_error_arcsec in 0.01f64..10.0,
        enabled in any::<bool>(),
    ) -> PointingModelTerm {
        PointingModelTerm { term_id, coeff_arcsec, coeff_error_arcsec, enabled }
    }
}

prop_compose! {
    fn arb_pointing_model()(
        revision in any::<u32>(),
        num_stars_used in 10u16..200,
        rms_arcsec in 0.1f64..30.0,
        terms in prop::collection::vec(arb_pointing_term(), 1..12),
    ) -> PointingModel {
        PointingModel { revision, num_stars_used, rms_arcsec, terms }
    }
}

prop_compose! {
    fn arb_ao_params()(
        ao_id in any::<u32>(),
        loop_freq_hz in 100.0f64..3000.0,
        loop_gain in 0.01f64..1.0,
        strehl_ratio in 0.01f64..1.0,
        wfs_type in 0u8..3,
        num_actuators in 32u16..5000,
        dm_stroke_um in 1.0f64..20.0,
        loop_closed in any::<bool>(),
    ) -> AdaptiveOpticsParams {
        AdaptiveOpticsParams {
            ao_id, loop_freq_hz, loop_gain, strehl_ratio,
            wfs_type, num_actuators, dm_stroke_um, loop_closed,
        }
    }
}

prop_compose! {
    fn arb_guide_star()(
        catalog_id in any::<u64>(),
        guide_mag in 5.0f64..18.0,
        sep_arcmin in 0.1f64..30.0,
        pa_deg in 0.0f64..360.0,
        pm_ra_mas_yr in -500.0f64..500.0,
        pm_dec_mas_yr in -500.0f64..500.0,
        is_primary in any::<bool>(),
    ) -> GuideStarSelection {
        GuideStarSelection {
            catalog_id, guide_mag, sep_arcmin, pa_deg,
            pm_ra_mas_yr, pm_dec_mas_yr, is_primary,
        }
    }
}

prop_compose! {
    fn arb_obs_queue_entry()(
        ob_id in any::<u64>(),
        priority in 1u16..1000,
        target in arb_equatorial_coord(),
        max_seeing_arcsec in 0.3f64..3.0,
        min_elevation_deg in 15.0f64..60.0,
        requested_exp_sec in 1.0f64..7200.0,
        time_critical in any::<bool>(),
        min_lunar_dist_deg in 10.0f64..90.0,
    ) -> ObsQueueEntry {
        ObsQueueEntry {
            ob_id, priority, target, max_seeing_arcsec,
            min_elevation_deg, requested_exp_sec, time_critical, min_lunar_dist_deg,
        }
    }
}

prop_compose! {
    fn arb_weather()(
        timestamp_s in any::<u64>(),
        seeing_arcsec in 0.3f64..5.0,
        cloud_cover in 0.0f64..1.0,
        humidity in 0.0f64..1.0,
        wind_speed_ms in 0.0f64..40.0,
        wind_dir_deg in 0.0f64..360.0,
        pressure_hpa in 700.0f64..1100.0,
        ambient_temp_c in -20.0f64..40.0,
        dew_point_c in -30.0f64..30.0,
    ) -> WeatherReading {
        WeatherReading {
            timestamp_s, seeing_arcsec, cloud_cover, humidity,
            wind_speed_ms, wind_dir_deg, pressure_hpa, ambient_temp_c, dew_point_c,
        }
    }
}

prop_compose! {
    fn arb_reduction_step()(
        variant in 0u8..7,
        master_id in any::<u32>(),
        scale in 0.5f64..2.0,
        filter_idx in 0u8..8,
        sigma in 1.0f64..10.0,
        max_iter in 1u8..20,
        mesh in 32u16..512,
        radius in 0.5f64..5.0,
        zp in 20.0f64..30.0,
        ext in 0.0f64..1.0,
    ) -> ReductionStep {
        match variant {
            0 => ReductionStep::BiasSubtract { master_bias_id: master_id },
            1 => ReductionStep::DarkSubtract { master_dark_id: master_id, scale_factor: scale },
            2 => ReductionStep::FlatField { master_flat_id: master_id, filter_index: filter_idx },
            3 => ReductionStep::CosmicRayReject { sigma_clip: sigma, max_iterations: max_iter },
            4 => ReductionStep::SkySubtract { mesh_size: mesh, sigma },
            5 => ReductionStep::AstrometricSolve { catalog_id: master_id, match_radius_arcsec: radius },
            _ => ReductionStep::PhotometricCalib { zero_point: zp, extinction_coeff: ext },
        }
    }
}

prop_compose! {
    fn arb_reduction_pipeline()(
        run_id in any::<u64>(),
        steps in prop::collection::vec(arb_reduction_step(), 1..8),
    ) -> ReductionPipeline {
        ReductionPipeline { run_id, steps }
    }
}

prop_compose! {
    fn arb_asteroid_orbit()(
        designation_hash in any::<u64>(),
        semi_major_axis_au in 0.5f64..50.0,
        eccentricity in 0.0f64..0.99,
        inclination_deg in 0.0f64..180.0,
        lon_asc_node_deg in 0.0f64..360.0,
        arg_perihelion_deg in 0.0f64..360.0,
        mean_anomaly_deg in 0.0f64..360.0,
        epoch_mjd in 50000.0f64..70000.0,
        abs_mag_h in 5.0f64..30.0,
        num_obs in 3u32..5000,
    ) -> AsteroidOrbit {
        AsteroidOrbit {
            designation_hash, semi_major_axis_au, eccentricity, inclination_deg,
            lon_asc_node_deg, arg_perihelion_deg, mean_anomaly_deg, epoch_mjd,
            abs_mag_h, num_obs,
        }
    }
}

prop_compose! {
    fn arb_transit_detection()(
        host_star_id in any::<u64>(),
        midpoint_bjd in 2458000.0f64..2462000.0,
        depth_ppm in 10.0f64..50000.0,
        duration_hours in 0.5f64..15.0,
        impact_param in 0.0f64..1.2,
        period_days in 0.5f64..365.0,
        radius_r_earth in 0.3f64..25.0,
        sde in 5.0f64..100.0,
    ) -> TransitDetection {
        TransitDetection {
            host_star_id, midpoint_bjd, depth_ppm, duration_hours,
            impact_param, period_days, radius_r_earth, sde,
        }
    }
}

prop_compose! {
    fn arb_baseline()(
        antenna_a in 0u16..64,
        antenna_b in 0u16..64,
        length_m in 10.0f64..16000.0,
        east_m in -8000.0f64..8000.0,
        north_m in -8000.0f64..8000.0,
        up_m in -500.0f64..500.0,
        freq_ghz in 0.1f64..350.0,
        uv_weight in 0.01f64..10.0,
    ) -> InterferometryBaseline {
        InterferometryBaseline {
            antenna_a, antenna_b, length_m, east_m, north_m, up_m, freq_ghz, uv_weight,
        }
    }
}

prop_compose! {
    fn arb_dome_status()(
        variant in 0u8..6,
        az in 0.0f64..360.0,
        slit in 1.0f64..10.0,
        pct in 0.0f64..100.0,
        emerg in any::<bool>(),
        fault in any::<u32>(),
    ) -> DomeShutterStatus {
        match variant {
            0 => DomeShutterStatus::Parked,
            1 => DomeShutterStatus::Slewing { target_az_deg: az },
            2 => DomeShutterStatus::Tracking { current_az_deg: az, slit_width_deg: slit },
            3 => DomeShutterStatus::ShutterOpening { percent_open: pct },
            4 => DomeShutterStatus::ShutterClosing { percent_open: pct, emergency: emerg },
            _ => DomeShutterStatus::Maintenance { fault_code: fault },
        }
    }
}

prop_compose! {
    fn arb_calib_frame()(
        frame_id in any::<u64>(),
        frame_type in 0u8..5,
        filter_index in 0u8..8,
        exposure_sec in 0.0f64..3600.0,
        ccd_temp_c in -60.0f32..0.0,
        num_combined in 1u16..200,
        created_s in any::<u64>(),
        valid_until_s in any::<u64>(),
    ) -> CalibrationFrame {
        CalibrationFrame {
            frame_id, frame_type, filter_index, exposure_sec,
            ccd_temp_c, num_combined, created_s, valid_until_s,
        }
    }
}

prop_compose! {
    fn arb_calib_library()(
        library_id in any::<u32>(),
        telescope_id in any::<u32>(),
        frames in prop::collection::vec(arb_calib_frame(), 0..10),
    ) -> CalibrationLibrary {
        CalibrationLibrary { library_id, telescope_id, frames }
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[test]
fn test_equatorial_coord_roundtrip() {
    proptest!(|(coord in arb_equatorial_coord())| {
        let encoded = encode_to_vec(&coord).expect("encode equatorial coord");
        let (decoded, _): (EquatorialCoord, _) = decode_from_slice(&encoded)
            .expect("decode equatorial coord");
        prop_assert_eq!(coord, decoded);
    });
}

#[test]
fn test_horizontal_coord_roundtrip() {
    proptest!(|(coord in arb_horizontal_coord())| {
        let encoded = encode_to_vec(&coord).expect("encode horizontal coord");
        let (decoded, _): (HorizontalCoord, _) = decode_from_slice(&encoded)
            .expect("decode horizontal coord");
        prop_assert_eq!(coord, decoded);
    });
}

#[test]
fn test_ccd_image_meta_roundtrip() {
    proptest!(|(meta in arb_ccd_image_meta())| {
        let encoded = encode_to_vec(&meta).expect("encode CCD image meta");
        let (decoded, _): (CcdImageMeta, _) = decode_from_slice(&encoded)
            .expect("decode CCD image meta");
        prop_assert_eq!(meta, decoded);
    });
}

#[test]
fn test_photometric_measurement_roundtrip() {
    proptest!(|(pm in arb_photometric())| {
        let encoded = encode_to_vec(&pm).expect("encode photometric measurement");
        let (decoded, _): (PhotometricMeasurement, _) = decode_from_slice(&encoded)
            .expect("decode photometric measurement");
        prop_assert_eq!(pm, decoded);
    });
}

#[test]
fn test_spectroscopic_obs_roundtrip() {
    proptest!(|(obs in arb_spectroscopic())| {
        let encoded = encode_to_vec(&obs).expect("encode spectroscopic obs");
        let (decoded, _): (SpectroscopicObs, _) = decode_from_slice(&encoded)
            .expect("decode spectroscopic obs");
        prop_assert_eq!(obs, decoded);
    });
}

#[test]
fn test_pointing_model_term_roundtrip() {
    proptest!(|(term in arb_pointing_term())| {
        let encoded = encode_to_vec(&term).expect("encode pointing model term");
        let (decoded, _): (PointingModelTerm, _) = decode_from_slice(&encoded)
            .expect("decode pointing model term");
        prop_assert_eq!(term, decoded);
    });
}

#[test]
fn test_pointing_model_roundtrip() {
    proptest!(|(model in arb_pointing_model())| {
        let encoded = encode_to_vec(&model).expect("encode pointing model");
        let (decoded, _): (PointingModel, _) = decode_from_slice(&encoded)
            .expect("decode pointing model");
        prop_assert_eq!(model, decoded);
    });
}

#[test]
fn test_adaptive_optics_params_roundtrip() {
    proptest!(|(ao in arb_ao_params())| {
        let encoded = encode_to_vec(&ao).expect("encode AO params");
        let (decoded, _): (AdaptiveOpticsParams, _) = decode_from_slice(&encoded)
            .expect("decode AO params");
        prop_assert_eq!(ao, decoded);
    });
}

#[test]
fn test_guide_star_selection_roundtrip() {
    proptest!(|(gs in arb_guide_star())| {
        let encoded = encode_to_vec(&gs).expect("encode guide star");
        let (decoded, _): (GuideStarSelection, _) = decode_from_slice(&encoded)
            .expect("decode guide star");
        prop_assert_eq!(gs, decoded);
    });
}

#[test]
fn test_obs_queue_entry_roundtrip() {
    proptest!(|(entry in arb_obs_queue_entry())| {
        let encoded = encode_to_vec(&entry).expect("encode obs queue entry");
        let (decoded, _): (ObsQueueEntry, _) = decode_from_slice(&encoded)
            .expect("decode obs queue entry");
        prop_assert_eq!(entry, decoded);
    });
}

#[test]
fn test_weather_reading_roundtrip() {
    proptest!(|(wr in arb_weather())| {
        let encoded = encode_to_vec(&wr).expect("encode weather reading");
        let (decoded, _): (WeatherReading, _) = decode_from_slice(&encoded)
            .expect("decode weather reading");
        prop_assert_eq!(wr, decoded);
    });
}

#[test]
fn test_reduction_step_roundtrip() {
    proptest!(|(step in arb_reduction_step())| {
        let encoded = encode_to_vec(&step).expect("encode reduction step");
        let (decoded, _): (ReductionStep, _) = decode_from_slice(&encoded)
            .expect("decode reduction step");
        prop_assert_eq!(step, decoded);
    });
}

#[test]
fn test_reduction_pipeline_roundtrip() {
    proptest!(|(pipeline in arb_reduction_pipeline())| {
        let encoded = encode_to_vec(&pipeline).expect("encode reduction pipeline");
        let (decoded, _): (ReductionPipeline, _) = decode_from_slice(&encoded)
            .expect("decode reduction pipeline");
        prop_assert_eq!(pipeline, decoded);
    });
}

#[test]
fn test_asteroid_orbit_roundtrip() {
    proptest!(|(orbit in arb_asteroid_orbit())| {
        let encoded = encode_to_vec(&orbit).expect("encode asteroid orbit");
        let (decoded, _): (AsteroidOrbit, _) = decode_from_slice(&encoded)
            .expect("decode asteroid orbit");
        prop_assert_eq!(orbit, decoded);
    });
}

#[test]
fn test_transit_detection_roundtrip() {
    proptest!(|(td in arb_transit_detection())| {
        let encoded = encode_to_vec(&td).expect("encode transit detection");
        let (decoded, _): (TransitDetection, _) = decode_from_slice(&encoded)
            .expect("decode transit detection");
        prop_assert_eq!(td, decoded);
    });
}

#[test]
fn test_interferometry_baseline_roundtrip() {
    proptest!(|(bl in arb_baseline())| {
        let encoded = encode_to_vec(&bl).expect("encode interferometry baseline");
        let (decoded, _): (InterferometryBaseline, _) = decode_from_slice(&encoded)
            .expect("decode interferometry baseline");
        prop_assert_eq!(bl, decoded);
    });
}

#[test]
fn test_dome_shutter_status_roundtrip() {
    proptest!(|(ds in arb_dome_status())| {
        let encoded = encode_to_vec(&ds).expect("encode dome/shutter status");
        let (decoded, _): (DomeShutterStatus, _) = decode_from_slice(&encoded)
            .expect("decode dome/shutter status");
        prop_assert_eq!(ds, decoded);
    });
}

#[test]
fn test_calibration_frame_roundtrip() {
    proptest!(|(cf in arb_calib_frame())| {
        let encoded = encode_to_vec(&cf).expect("encode calibration frame");
        let (decoded, _): (CalibrationFrame, _) = decode_from_slice(&encoded)
            .expect("decode calibration frame");
        prop_assert_eq!(cf, decoded);
    });
}

#[test]
fn test_calibration_library_roundtrip() {
    proptest!(|(lib in arb_calib_library())| {
        let encoded = encode_to_vec(&lib).expect("encode calibration library");
        let (decoded, _): (CalibrationLibrary, _) = decode_from_slice(&encoded)
            .expect("decode calibration library");
        prop_assert_eq!(lib, decoded);
    });
}

#[test]
fn test_guide_star_batch_roundtrip() {
    proptest!(|(stars in prop::collection::vec(arb_guide_star(), 1..8))| {
        let encoded = encode_to_vec(&stars).expect("encode guide star batch");
        let (decoded, _): (Vec<GuideStarSelection>, _) = decode_from_slice(&encoded)
            .expect("decode guide star batch");
        prop_assert_eq!(stars, decoded);
    });
}

#[test]
fn test_observation_schedule_roundtrip() {
    proptest!(|(queue in prop::collection::vec(arb_obs_queue_entry(), 1..6))| {
        let encoded = encode_to_vec(&queue).expect("encode observation schedule");
        let (decoded, _): (Vec<ObsQueueEntry>, _) = decode_from_slice(&encoded)
            .expect("decode observation schedule");
        prop_assert_eq!(queue, decoded);
    });
}

#[test]
fn test_multi_baseline_array_roundtrip() {
    proptest!(|(baselines in prop::collection::vec(arb_baseline(), 1..10))| {
        let encoded = encode_to_vec(&baselines).expect("encode baseline array");
        let (decoded, _): (Vec<InterferometryBaseline>, _) = decode_from_slice(&encoded)
            .expect("decode baseline array");
        prop_assert_eq!(baselines, decoded);
    });
}
