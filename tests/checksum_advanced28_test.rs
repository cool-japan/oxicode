//! Advanced checksum tests for OxiCode — space situational awareness and debris tracking.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced28_test
//!
//! Exactly 22 `#[test]` functions.

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum, HEADER_SIZE};
use oxicode::{encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types — space situational awareness
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CatalogedSpaceObject {
    norad_id: u32,
    cospar_id: String,
    name: String,
    rcs_m2: f64,
    origin_country: String,
    object_type: SpaceObjectType,
    orbit_regime: OrbitRegime,
    launch_year: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SpaceObjectType {
    Payload,
    RocketBody,
    Debris,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OrbitRegime {
    Leo,
    Meo,
    Geo,
    Heo,
    GraveyardGeo,
    SunSynchronous,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StateVector {
    epoch_mjd: f64,
    x_km: f64,
    y_km: f64,
    z_km: f64,
    vx_km_s: f64,
    vy_km_s: f64,
    vz_km_s: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CovarianceMatrix {
    cr_r: f64,
    ct_r: f64,
    ct_t: f64,
    cn_r: f64,
    cn_t: f64,
    cn_n: f64,
    crdot_r: f64,
    crdot_t: f64,
    crdot_n: f64,
    crdot_rdot: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrbitDetermination {
    object_id: u32,
    state: StateVector,
    covariance: CovarianceMatrix,
    residual_rms: f64,
    observations_used: u32,
    solver_iterations: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConjunctionAssessment {
    primary_norad_id: u32,
    secondary_norad_id: u32,
    tca_mjd: f64,
    miss_distance_m: f64,
    radial_miss_m: f64,
    in_track_miss_m: f64,
    cross_track_miss_m: f64,
    collision_probability: f64,
    screening_volume_km: f64,
    hard_body_radius_m: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ManeuverPlan {
    object_id: u32,
    maneuver_epoch_mjd: f64,
    delta_v_r_m_s: f64,
    delta_v_t_m_s: f64,
    delta_v_n_m_s: f64,
    burn_duration_s: f64,
    propellant_mass_kg: f64,
    maneuver_type: ManeuverType,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ManeuverType {
    CollisionAvoidance,
    StationKeeping,
    OrbitRaise,
    OrbitLower,
    Deorbit,
    PhaseAdjust,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BreakupEvent {
    parent_norad_id: u32,
    event_epoch_mjd: f64,
    fragment_count: u32,
    delta_v_spread_m_s: f64,
    event_type: BreakupType,
    altitude_km: f64,
    fragment_ids: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BreakupType {
    Collision,
    Explosion,
    AnomalousEvent,
    Deliberate,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LaserRangingObservation {
    station_id: String,
    target_norad_id: u32,
    epoch_mjd: f64,
    range_km: f64,
    range_rate_km_s: f64,
    normal_point_rms_mm: f64,
    wavelength_nm: f64,
    return_photon_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadarTrackingMeasurement {
    radar_id: String,
    target_norad_id: u32,
    epoch_mjd: f64,
    range_km: f64,
    azimuth_deg: f64,
    elevation_deg: f64,
    range_rate_km_s: f64,
    snr_db: f64,
    integration_time_s: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpaceWeatherIndices {
    epoch_mjd: f64,
    kp_index: f64,
    f10_7_sfu: f64,
    f10_7_81day_avg: f64,
    dst_nt: f64,
    ap_index: f64,
    solar_wind_speed_km_s: f64,
    proton_flux: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AtmosphericDragModel {
    object_id: u32,
    drag_coefficient: f64,
    area_to_mass_ratio: f64,
    ballistic_coefficient: f64,
    atmospheric_model: String,
    altitude_km: f64,
    density_kg_m3: f64,
    drag_acceleration_m_s2: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CollisionScreeningResult {
    conjunction_id: String,
    assessments: Vec<ConjunctionAssessment>,
    total_screened_pairs: u64,
    high_risk_count: u32,
    screening_threshold_km: f64,
    time_window_days: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReentryPrediction {
    object_norad_id: u32,
    object_name: String,
    predicted_epoch_mjd: f64,
    uncertainty_hours: f64,
    latitude_deg: f64,
    longitude_deg: f64,
    ground_track_width_km: f64,
    surviving_mass_fraction: f64,
    casualty_area_m2: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MegaConstellationSlot {
    plane_index: u16,
    slot_index: u16,
    norad_id: u32,
    inclination_deg: f64,
    altitude_km: f64,
    raan_deg: f64,
    mean_anomaly_deg: f64,
    status: ConstellationSlotStatus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConstellationSlotStatus {
    Operational,
    DriftingToSlot,
    SpareParked,
    Decommissioned,
    Failed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MegaConstellationCoordination {
    constellation_name: String,
    operator: String,
    total_planes: u16,
    slots_per_plane: u16,
    active_satellites: Vec<MegaConstellationSlot>,
    filing_designation: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpaceSustainabilityRating {
    object_norad_id: u32,
    operator: String,
    deorbit_plan: bool,
    passivation_complete: bool,
    trackability_score: f64,
    collision_avoidance_capable: bool,
    data_sharing_level: DataSharingLevel,
    estimated_orbital_lifetime_years: f64,
    compliance_score: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DataSharingLevel {
    Full,
    EphemerisOnly,
    ManeuverNotification,
    None,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OpticalTrackingObservation {
    station_name: String,
    latitude_deg: f64,
    longitude_deg: f64,
    altitude_m: f64,
    target_norad_id: u32,
    epoch_mjd: f64,
    right_ascension_deg: f64,
    declination_deg: f64,
    visual_magnitude: f64,
    angular_rate_deg_s: f64,
    exposure_time_s: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FragmentationDebrisCloud {
    parent_event_id: String,
    fragment_states: Vec<StateVector>,
    mean_area_to_mass: f64,
    spread_velocity_m_s: f64,
    cloud_age_days: f64,
    trackable_count: u32,
    estimated_total_fragments: u32,
    peak_spatial_density: f64,
}

// ---------------------------------------------------------------------------
// Test 1: Cataloged space object roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_cataloged_space_object_checksum() {
    let obj = CatalogedSpaceObject {
        norad_id: 25544,
        cospar_id: "1998-067A".to_string(),
        name: "ISS (ZARYA)".to_string(),
        rcs_m2: 432.6,
        origin_country: "ISS".to_string(),
        object_type: SpaceObjectType::Payload,
        orbit_regime: OrbitRegime::Leo,
        launch_year: 1998,
    };
    let encoded = encode_with_checksum(&obj).expect("encode cataloged object");
    let (decoded, consumed): (CatalogedSpaceObject, _) =
        decode_with_checksum(&encoded).expect("decode cataloged object");
    assert_eq!(decoded, obj);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: Conjunction assessment message roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_conjunction_assessment_checksum() {
    let cdm = ConjunctionAssessment {
        primary_norad_id: 25544,
        secondary_norad_id: 44237,
        tca_mjd: 60400.456789,
        miss_distance_m: 347.2,
        radial_miss_m: 52.1,
        in_track_miss_m: 312.8,
        cross_track_miss_m: 128.4,
        collision_probability: 1.2e-5,
        screening_volume_km: 5.0,
        hard_body_radius_m: 15.0,
    };
    let encoded = encode_with_checksum(&cdm).expect("encode CDM");
    let (decoded, consumed): (ConjunctionAssessment, _) =
        decode_with_checksum(&encoded).expect("decode CDM");
    assert_eq!(decoded, cdm);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: Orbit determination solution roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_orbit_determination_checksum() {
    let od = OrbitDetermination {
        object_id: 25544,
        state: StateVector {
            epoch_mjd: 60400.0,
            x_km: -4156.539,
            y_km: 2721.885,
            z_km: 4615.778,
            vx_km_s: -4.838,
            vy_km_s: -5.217,
            vz_km_s: 2.664,
        },
        covariance: CovarianceMatrix {
            cr_r: 4.4e-4,
            ct_r: 2.3e-5,
            ct_t: 5.6e-3,
            cn_r: 1.1e-5,
            cn_t: -3.2e-5,
            cn_n: 2.1e-4,
            crdot_r: 8.9e-7,
            crdot_t: -1.2e-6,
            crdot_n: 3.4e-7,
            crdot_rdot: 6.7e-9,
        },
        residual_rms: 0.42,
        observations_used: 1247,
        solver_iterations: 6,
    };
    let encoded = encode_with_checksum(&od).expect("encode OD");
    let (decoded, consumed): (OrbitDetermination, _) =
        decode_with_checksum(&encoded).expect("decode OD");
    assert_eq!(decoded, od);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 4: Maneuver planning roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_maneuver_plan_checksum() {
    let plan = ManeuverPlan {
        object_id: 25544,
        maneuver_epoch_mjd: 60401.25,
        delta_v_r_m_s: 0.0,
        delta_v_t_m_s: 0.35,
        delta_v_n_m_s: 0.0,
        burn_duration_s: 12.5,
        propellant_mass_kg: 2.8,
        maneuver_type: ManeuverType::CollisionAvoidance,
    };
    let encoded = encode_with_checksum(&plan).expect("encode maneuver plan");
    let (decoded, consumed): (ManeuverPlan, _) =
        decode_with_checksum(&encoded).expect("decode maneuver plan");
    assert_eq!(decoded, plan);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: Breakup event analysis roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_breakup_event_checksum() {
    let event = BreakupEvent {
        parent_norad_id: 19650,
        event_epoch_mjd: 59250.333,
        fragment_count: 523,
        delta_v_spread_m_s: 145.0,
        event_type: BreakupType::Explosion,
        altitude_km: 780.0,
        fragment_ids: (50000..50020).collect(),
    };
    let encoded = encode_with_checksum(&event).expect("encode breakup event");
    let (decoded, consumed): (BreakupEvent, _) =
        decode_with_checksum(&encoded).expect("decode breakup event");
    assert_eq!(decoded, event);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 6: Laser ranging observation roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_laser_ranging_checksum() {
    let obs = LaserRangingObservation {
        station_id: "YARL-7090".to_string(),
        target_norad_id: 22195,
        epoch_mjd: 60400.123456,
        range_km: 592.341,
        range_rate_km_s: -1.234,
        normal_point_rms_mm: 3.7,
        wavelength_nm: 532.0,
        return_photon_count: 847,
    };
    let encoded = encode_with_checksum(&obs).expect("encode SLR observation");
    let (decoded, consumed): (LaserRangingObservation, _) =
        decode_with_checksum(&encoded).expect("decode SLR observation");
    assert_eq!(decoded, obs);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: Radar tracking measurement roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_radar_tracking_checksum() {
    let meas = RadarTrackingMeasurement {
        radar_id: "UEWR-CAPE".to_string(),
        target_norad_id: 25544,
        epoch_mjd: 60400.789,
        range_km: 1450.2,
        azimuth_deg: 187.34,
        elevation_deg: 42.15,
        range_rate_km_s: 3.21,
        snr_db: 28.5,
        integration_time_s: 0.01,
    };
    let encoded = encode_with_checksum(&meas).expect("encode radar measurement");
    let (decoded, consumed): (RadarTrackingMeasurement, _) =
        decode_with_checksum(&encoded).expect("decode radar measurement");
    assert_eq!(decoded, meas);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: Space weather indices roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_space_weather_indices_checksum() {
    let wx = SpaceWeatherIndices {
        epoch_mjd: 60400.0,
        kp_index: 3.7,
        f10_7_sfu: 148.2,
        f10_7_81day_avg: 142.5,
        dst_nt: -32.0,
        ap_index: 12.0,
        solar_wind_speed_km_s: 420.0,
        proton_flux: 2.3e2,
    };
    let encoded = encode_with_checksum(&wx).expect("encode space weather");
    let (decoded, consumed): (SpaceWeatherIndices, _) =
        decode_with_checksum(&encoded).expect("decode space weather");
    assert_eq!(decoded, wx);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: Atmospheric drag model roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_atmospheric_drag_model_checksum() {
    let drag = AtmosphericDragModel {
        object_id: 25544,
        drag_coefficient: 2.2,
        area_to_mass_ratio: 0.0037,
        ballistic_coefficient: 122.5,
        atmospheric_model: "NRLMSISE-00".to_string(),
        altitude_km: 420.0,
        density_kg_m3: 2.8e-12,
        drag_acceleration_m_s2: 1.5e-6,
    };
    let encoded = encode_with_checksum(&drag).expect("encode drag model");
    let (decoded, consumed): (AtmosphericDragModel, _) =
        decode_with_checksum(&encoded).expect("decode drag model");
    assert_eq!(decoded, drag);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: Collision avoidance screening roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_collision_screening_checksum() {
    let screening = CollisionScreeningResult {
        conjunction_id: "CDM-2025-001234".to_string(),
        assessments: vec![
            ConjunctionAssessment {
                primary_norad_id: 25544,
                secondary_norad_id: 44237,
                tca_mjd: 60400.5,
                miss_distance_m: 347.2,
                radial_miss_m: 52.1,
                in_track_miss_m: 312.8,
                cross_track_miss_m: 128.4,
                collision_probability: 1.2e-5,
                screening_volume_km: 5.0,
                hard_body_radius_m: 15.0,
            },
            ConjunctionAssessment {
                primary_norad_id: 25544,
                secondary_norad_id: 39090,
                tca_mjd: 60401.1,
                miss_distance_m: 1200.0,
                radial_miss_m: 400.0,
                in_track_miss_m: 950.0,
                cross_track_miss_m: 580.0,
                collision_probability: 3.4e-8,
                screening_volume_km: 5.0,
                hard_body_radius_m: 10.0,
            },
        ],
        total_screened_pairs: 48_000_000,
        high_risk_count: 12,
        screening_threshold_km: 5.0,
        time_window_days: 7.0,
    };
    let encoded = encode_with_checksum(&screening).expect("encode screening result");
    let (decoded, consumed): (CollisionScreeningResult, _) =
        decode_with_checksum(&encoded).expect("decode screening result");
    assert_eq!(decoded, screening);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: Re-entry prediction roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_reentry_prediction_checksum() {
    let pred = ReentryPrediction {
        object_norad_id: 48275,
        object_name: "CZ-5B R/B".to_string(),
        predicted_epoch_mjd: 60402.75,
        uncertainty_hours: 12.5,
        latitude_deg: 31.2,
        longitude_deg: -74.8,
        ground_track_width_km: 2200.0,
        surviving_mass_fraction: 0.35,
        casualty_area_m2: 18.7,
    };
    let encoded = encode_with_checksum(&pred).expect("encode reentry prediction");
    let (decoded, consumed): (ReentryPrediction, _) =
        decode_with_checksum(&encoded).expect("decode reentry prediction");
    assert_eq!(decoded, pred);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: Mega-constellation coordination roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_mega_constellation_coordination_checksum() {
    let constellation = MegaConstellationCoordination {
        constellation_name: "Starlink Gen2".to_string(),
        operator: "SpaceX".to_string(),
        total_planes: 72,
        slots_per_plane: 22,
        active_satellites: vec![
            MegaConstellationSlot {
                plane_index: 0,
                slot_index: 0,
                norad_id: 55001,
                inclination_deg: 53.2,
                altitude_km: 550.0,
                raan_deg: 0.0,
                mean_anomaly_deg: 0.0,
                status: ConstellationSlotStatus::Operational,
            },
            MegaConstellationSlot {
                plane_index: 0,
                slot_index: 1,
                norad_id: 55002,
                inclination_deg: 53.2,
                altitude_km: 550.0,
                raan_deg: 0.0,
                mean_anomaly_deg: 16.36,
                status: ConstellationSlotStatus::DriftingToSlot,
            },
            MegaConstellationSlot {
                plane_index: 1,
                slot_index: 0,
                norad_id: 55050,
                inclination_deg: 53.2,
                altitude_km: 540.0,
                raan_deg: 5.0,
                mean_anomaly_deg: 0.0,
                status: ConstellationSlotStatus::SpareParked,
            },
        ],
        filing_designation: "SAT-MOD-20230101-00001".to_string(),
    };
    let encoded = encode_with_checksum(&constellation).expect("encode constellation");
    let (decoded, consumed): (MegaConstellationCoordination, _) =
        decode_with_checksum(&encoded).expect("decode constellation");
    assert_eq!(decoded, constellation);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: Space sustainability rating roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_space_sustainability_rating_checksum() {
    let rating = SpaceSustainabilityRating {
        object_norad_id: 48274,
        operator: "OneWeb".to_string(),
        deorbit_plan: true,
        passivation_complete: true,
        trackability_score: 0.92,
        collision_avoidance_capable: true,
        data_sharing_level: DataSharingLevel::Full,
        estimated_orbital_lifetime_years: 5.0,
        compliance_score: 0.88,
    };
    let encoded = encode_with_checksum(&rating).expect("encode sustainability rating");
    let (decoded, consumed): (SpaceSustainabilityRating, _) =
        decode_with_checksum(&encoded).expect("decode sustainability rating");
    assert_eq!(decoded, rating);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: Optical tracking observation roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_optical_tracking_checksum() {
    let obs = OpticalTrackingObservation {
        station_name: "Maui Space Surveillance".to_string(),
        latitude_deg: 20.7084,
        longitude_deg: -156.2570,
        altitude_m: 3058.0,
        target_norad_id: 36411,
        epoch_mjd: 60400.875,
        right_ascension_deg: 142.567,
        declination_deg: -12.345,
        visual_magnitude: 8.2,
        angular_rate_deg_s: 0.42,
        exposure_time_s: 0.5,
    };
    let encoded = encode_with_checksum(&obs).expect("encode optical observation");
    let (decoded, consumed): (OpticalTrackingObservation, _) =
        decode_with_checksum(&encoded).expect("decode optical observation");
    assert_eq!(decoded, obs);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Fragmentation debris cloud roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_fragmentation_debris_cloud_checksum() {
    let cloud = FragmentationDebrisCloud {
        parent_event_id: "COSMOS-1408-ASAT".to_string(),
        fragment_states: vec![
            StateVector {
                epoch_mjd: 60400.0,
                x_km: -4100.0,
                y_km: 2700.0,
                z_km: 4600.0,
                vx_km_s: -4.82,
                vy_km_s: -5.21,
                vz_km_s: 2.66,
            },
            StateVector {
                epoch_mjd: 60400.0,
                x_km: -4102.5,
                y_km: 2698.3,
                z_km: 4601.2,
                vx_km_s: -4.83,
                vy_km_s: -5.20,
                vz_km_s: 2.67,
            },
        ],
        mean_area_to_mass: 0.012,
        spread_velocity_m_s: 250.0,
        cloud_age_days: 1100.0,
        trackable_count: 1632,
        estimated_total_fragments: 7500,
        peak_spatial_density: 3.2e-9,
    };
    let encoded = encode_with_checksum(&cloud).expect("encode debris cloud");
    let (decoded, consumed): (FragmentationDebrisCloud, _) =
        decode_with_checksum(&encoded).expect("decode debris cloud");
    assert_eq!(decoded, cloud);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: Vector of cataloged objects roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_catalog_batch_checksum() {
    let objects = vec![
        CatalogedSpaceObject {
            norad_id: 25544,
            cospar_id: "1998-067A".to_string(),
            name: "ISS (ZARYA)".to_string(),
            rcs_m2: 432.6,
            origin_country: "ISS".to_string(),
            object_type: SpaceObjectType::Payload,
            orbit_regime: OrbitRegime::Leo,
            launch_year: 1998,
        },
        CatalogedSpaceObject {
            norad_id: 20580,
            cospar_id: "1990-037A".to_string(),
            name: "HST".to_string(),
            rcs_m2: 56.8,
            origin_country: "US".to_string(),
            object_type: SpaceObjectType::Payload,
            orbit_regime: OrbitRegime::Leo,
            launch_year: 1990,
        },
        CatalogedSpaceObject {
            norad_id: 36585,
            cospar_id: "2010-013A".to_string(),
            name: "SDO".to_string(),
            rcs_m2: 12.4,
            origin_country: "US".to_string(),
            object_type: SpaceObjectType::Payload,
            orbit_regime: OrbitRegime::Geo,
            launch_year: 2010,
        },
    ];
    let encoded = encode_with_checksum(&objects).expect("encode object batch");
    let (decoded, consumed): (Vec<CatalogedSpaceObject>, _) =
        decode_with_checksum(&encoded).expect("decode object batch");
    assert_eq!(decoded, objects);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: Multiple maneuver types roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_maneuver_type_variants_checksum() {
    let plans = vec![
        ManeuverPlan {
            object_id: 25544,
            maneuver_epoch_mjd: 60401.0,
            delta_v_r_m_s: 0.0,
            delta_v_t_m_s: 0.35,
            delta_v_n_m_s: 0.0,
            burn_duration_s: 12.5,
            propellant_mass_kg: 2.8,
            maneuver_type: ManeuverType::CollisionAvoidance,
        },
        ManeuverPlan {
            object_id: 25544,
            maneuver_epoch_mjd: 60410.0,
            delta_v_r_m_s: 0.0,
            delta_v_t_m_s: 1.2,
            delta_v_n_m_s: 0.0,
            burn_duration_s: 45.0,
            propellant_mass_kg: 8.5,
            maneuver_type: ManeuverType::OrbitRaise,
        },
        ManeuverPlan {
            object_id: 40000,
            maneuver_epoch_mjd: 60500.0,
            delta_v_r_m_s: -5.0,
            delta_v_t_m_s: -120.0,
            delta_v_n_m_s: 0.0,
            burn_duration_s: 3600.0,
            propellant_mass_kg: 150.0,
            maneuver_type: ManeuverType::Deorbit,
        },
    ];
    let encoded = encode_with_checksum(&plans).expect("encode maneuver plans");
    let (decoded, consumed): (Vec<ManeuverPlan>, _) =
        decode_with_checksum(&encoded).expect("decode maneuver plans");
    assert_eq!(decoded, plans);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 18: Encoded size is payload + header
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_header_size_consistency() {
    let obj = CatalogedSpaceObject {
        norad_id: 1,
        cospar_id: "2000-001A".to_string(),
        name: "TEST SAT".to_string(),
        rcs_m2: 1.0,
        origin_country: "XX".to_string(),
        object_type: SpaceObjectType::Unknown,
        orbit_regime: OrbitRegime::Leo,
        launch_year: 2000,
    };
    let plain = encode_to_vec(&obj).expect("plain encode");
    let checksummed = encode_with_checksum(&obj).expect("checksummed encode");
    assert_eq!(checksummed.len(), plain.len() + HEADER_SIZE);
    let (decoded, _): (CatalogedSpaceObject, _) =
        decode_with_checksum(&checksummed).expect("decode with checksum");
    assert_eq!(decoded, obj);
}

// ---------------------------------------------------------------------------
// Test 19: Corruption detection — flip single bit in payload
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_single_bit_flip() {
    let obs = RadarTrackingMeasurement {
        radar_id: "HAYSTAX".to_string(),
        target_norad_id: 25544,
        epoch_mjd: 60400.0,
        range_km: 800.0,
        azimuth_deg: 90.0,
        elevation_deg: 45.0,
        range_rate_km_s: -2.0,
        snr_db: 30.0,
        integration_time_s: 0.02,
    };
    let mut encoded = encode_with_checksum(&obs).expect("encode radar measurement");
    let last_idx = encoded.len() - 1;
    encoded[last_idx] ^= 0x01;
    let result: std::result::Result<(RadarTrackingMeasurement, usize), _> =
        decode_with_checksum(&encoded);
    assert!(result.is_err(), "corrupted payload must be detected");
}

// ---------------------------------------------------------------------------
// Test 20: Corruption detection — zero out CRC bytes
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_zeroed_crc() {
    let wx = SpaceWeatherIndices {
        epoch_mjd: 60400.0,
        kp_index: 5.0,
        f10_7_sfu: 200.0,
        f10_7_81day_avg: 180.0,
        dst_nt: -80.0,
        ap_index: 48.0,
        solar_wind_speed_km_s: 600.0,
        proton_flux: 1.0e4,
    };
    let mut encoded = encode_with_checksum(&wx).expect("encode space weather");
    // CRC32 is at bytes 12..16 in header
    encoded[12] = 0;
    encoded[13] = 0;
    encoded[14] = 0;
    encoded[15] = 0;
    let result: std::result::Result<(SpaceWeatherIndices, usize), _> =
        decode_with_checksum(&encoded);
    assert!(result.is_err(), "zeroed CRC must be detected");
}

// ---------------------------------------------------------------------------
// Test 21: Corruption detection — truncated buffer
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_truncated_buffer() {
    let event = BreakupEvent {
        parent_norad_id: 19650,
        event_epoch_mjd: 59250.333,
        fragment_count: 100,
        delta_v_spread_m_s: 50.0,
        event_type: BreakupType::Collision,
        altitude_km: 800.0,
        fragment_ids: vec![60000, 60001, 60002],
    };
    let encoded = encode_with_checksum(&event).expect("encode breakup event");
    let truncated = &encoded[..encoded.len() / 2];
    let result: std::result::Result<(BreakupEvent, usize), _> = decode_with_checksum(truncated);
    assert!(result.is_err(), "truncated buffer must be detected");
}

// ---------------------------------------------------------------------------
// Test 22: Corruption detection — magic byte overwrite
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_magic_overwrite() {
    let rating = SpaceSustainabilityRating {
        object_norad_id: 55555,
        operator: "Acme Orbital".to_string(),
        deorbit_plan: false,
        passivation_complete: false,
        trackability_score: 0.3,
        collision_avoidance_capable: false,
        data_sharing_level: DataSharingLevel::None,
        estimated_orbital_lifetime_years: 200.0,
        compliance_score: 0.15,
    };
    let mut encoded = encode_with_checksum(&rating).expect("encode sustainability rating");
    // Overwrite magic bytes
    encoded[0] = 0xFF;
    encoded[1] = 0xFF;
    encoded[2] = 0xFF;
    let result: std::result::Result<(SpaceSustainabilityRating, usize), _> =
        decode_with_checksum(&encoded);
    assert!(result.is_err(), "corrupted magic bytes must be detected");
}
