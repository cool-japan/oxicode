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

// --- Domain types: Space Mission Operations & Spacecraft Telemetry ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AttitudeQuaternion {
    spacecraft_id: String,
    timestamp_epoch_ms: u64,
    q_w: f64,
    q_x: f64,
    q_y: f64,
    q_z: f64,
    rate_x_deg_s: f64,
    rate_y_deg_s: f64,
    rate_z_deg_s: f64,
    mode: AttitudeMode,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AttitudeMode {
    SunPointing,
    EarthPointing,
    InertialHold,
    Slewing,
    SafeMode,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct KeplerianElements {
    object_name: String,
    epoch_mjd: f64,
    semi_major_axis_km: f64,
    eccentricity: f64,
    inclination_deg: f64,
    raan_deg: f64,
    arg_periapsis_deg: f64,
    true_anomaly_deg: f64,
    period_seconds: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GroundStationPass {
    station_id: String,
    station_name: String,
    latitude_deg: f64,
    longitude_deg: f64,
    aos_epoch_ms: u64,
    los_epoch_ms: u64,
    max_elevation_deg: f64,
    azimuth_aos_deg: f64,
    azimuth_los_deg: f64,
    pass_priority: u8,
    frequency_mhz: f64,
    scheduled: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SolarPanelPowerBudget {
    panel_id: String,
    panel_count: u8,
    area_m2: f64,
    efficiency_pct: f64,
    solar_flux_w_m2: f64,
    generated_watts: f64,
    consumed_watts: f64,
    battery_capacity_wh: f64,
    battery_soc_pct: f64,
    eclipse_duration_min: f64,
    sunlit_duration_min: f64,
    margin_watts: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ThermalControlReading {
    subsystem_name: String,
    sensor_id: u32,
    temperature_c: f64,
    heater_on: bool,
    heater_power_w: f64,
    radiator_area_m2: f64,
    min_operational_c: f64,
    max_operational_c: f64,
    zone: ThermalZone,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ThermalZone {
    SunFacing,
    AntiSun,
    Ram,
    Wake,
    Nadir,
    Zenith,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReactionWheelState {
    wheel_id: u8,
    axis_label: String,
    speed_rpm: f64,
    max_speed_rpm: f64,
    torque_nm: f64,
    temperature_c: f64,
    current_draw_a: f64,
    status: WheelStatus,
    total_revolutions: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum WheelStatus {
    Nominal,
    Degraded,
    SpeedLimited,
    OverTemp,
    Offline,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct StarTrackerMeasurement {
    tracker_id: u8,
    timestamp_epoch_ms: u64,
    right_ascension_deg: f64,
    declination_deg: f64,
    roll_deg: f64,
    num_stars_tracked: u16,
    confidence: f64,
    exposure_ms: f64,
    stray_light_detected: bool,
    quaternion: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PropulsionBudget {
    mission_name: String,
    engine_type: String,
    specific_impulse_s: f64,
    total_delta_v_m_s: f64,
    expended_delta_v_m_s: f64,
    remaining_delta_v_m_s: f64,
    propellant_mass_kg: f64,
    propellant_remaining_kg: f64,
    burn_count: u32,
    maneuvers: Vec<ManeuverRecord>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ManeuverRecord {
    maneuver_id: u32,
    epoch_ms: u64,
    delta_v_m_s: f64,
    duration_s: f64,
    direction: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CommandUplinkSequence {
    sequence_id: u64,
    spacecraft_id: String,
    operator: String,
    commands: Vec<UplinkCommand>,
    priority: u8,
    execution_window_start_ms: u64,
    execution_window_end_ms: u64,
    requires_confirmation: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct UplinkCommand {
    cmd_id: u32,
    opcode: u16,
    subsystem: String,
    parameters: Vec<i64>,
    description: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DsnScheduleEntry {
    antenna_name: String,
    antenna_diameter_m: f64,
    spacecraft_target: String,
    start_epoch_ms: u64,
    end_epoch_ms: u64,
    band: DsnBand,
    uplink_rate_bps: u64,
    downlink_rate_bps: u64,
    round_trip_light_time_s: f64,
    allocated: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DsnBand {
    SBand,
    XBand,
    KaBand,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DebrisAvoidanceManeuver {
    conjunction_id: String,
    debris_catalog_id: String,
    time_of_closest_approach_ms: u64,
    miss_distance_m: f64,
    collision_probability: f64,
    maneuver_required: bool,
    maneuver_delta_v_m_s: f64,
    maneuver_epoch_ms: u64,
    post_maneuver_miss_distance_m: f64,
    fuel_cost_kg: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LaunchVehicleStageSeparation {
    vehicle_name: String,
    stage_number: u8,
    separation_altitude_km: f64,
    separation_velocity_m_s: f64,
    separation_epoch_ms: u64,
    stage_mass_kg: f64,
    payload_to_next_stage_kg: f64,
    nominal: bool,
    telemetry_health: StageTelemetryHealth,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum StageTelemetryHealth {
    AllGreen,
    MinorAnomaly { code: u16, description: String },
    MajorAnomaly { code: u16, description: String },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct OnboardDataStorage {
    recorder_id: String,
    capacity_gb: f64,
    used_gb: f64,
    write_rate_mbps: f64,
    read_rate_mbps: f64,
    files_stored: u32,
    oldest_file_epoch_ms: u64,
    newest_file_epoch_ms: u64,
    error_count: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MissionTimeline {
    mission_id: String,
    phase: MissionPhase,
    phase_start_epoch_ms: u64,
    phase_end_epoch_ms: Option<u64>,
    elapsed_days: u32,
    objectives_total: u16,
    objectives_completed: u16,
    critical_events: Vec<CriticalEvent>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum MissionPhase {
    PreLaunch,
    Launch,
    EarlyOrbitCheckout,
    Commissioning,
    PrimaryMission,
    ExtendedMission,
    Decommissioning,
    EndOfLife,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CriticalEvent {
    event_id: u32,
    name: String,
    epoch_ms: u64,
    success: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TelemetryDownlinkFrame {
    frame_id: u64,
    spacecraft_id: String,
    apid: u16,
    sequence_count: u16,
    data_length: u32,
    timestamp_epoch_ms: u64,
    crc_valid: bool,
    reed_solomon_corrected_bits: u8,
    signal_to_noise_db: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct OrbitDeterminationSolution {
    solution_id: String,
    epoch_ms: u64,
    position_x_km: f64,
    position_y_km: f64,
    position_z_km: f64,
    velocity_x_km_s: f64,
    velocity_y_km_s: f64,
    velocity_z_km_s: f64,
    position_uncertainty_m: f64,
    velocity_uncertainty_mm_s: f64,
    drag_coefficient: f64,
    observations_used: u32,
    residual_rms: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PayloadInstrument {
    instrument_name: String,
    instrument_type: InstrumentType,
    power_draw_w: f64,
    data_rate_kbps: f64,
    operational: bool,
    temperature_c: f64,
    calibration_epoch_ms: u64,
    observation_count: u64,
    fov_deg: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum InstrumentType {
    OpticalImager,
    SyntheticApertureRadar,
    Spectrometer,
    Magnetometer,
    ParticleDetector,
    RadioOccultation,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GravityAssistPlan {
    flyby_body: String,
    flyby_epoch_ms: u64,
    periapsis_altitude_km: f64,
    incoming_v_infinity_km_s: f64,
    outgoing_v_infinity_km_s: f64,
    turn_angle_deg: f64,
    delta_v_gained_m_s: f64,
    b_plane_target_km: f64,
    b_plane_achieved_km: f64,
    navigational_corrections: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EclipseWindow {
    eclipse_type: EclipseType,
    entry_epoch_ms: u64,
    exit_epoch_ms: u64,
    duration_s: f64,
    depth_pct: f64,
    battery_discharge_wh: f64,
    min_temperature_c: f64,
    heater_activation_required: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum EclipseType {
    Umbra,
    Penumbra,
}

// === Tests ===

#[test]
fn test_attitude_quaternion_roundtrip() {
    let val = AttitudeQuaternion {
        spacecraft_id: "ARTEMIS-II".to_string(),
        timestamp_epoch_ms: 1_735_000_000_000,
        q_w: 0.707_106_781_186_547_6,
        q_x: 0.0,
        q_y: 0.707_106_781_186_547_6,
        q_z: 0.0,
        rate_x_deg_s: 0.001,
        rate_y_deg_s: -0.002,
        rate_z_deg_s: 0.0005,
        mode: AttitudeMode::EarthPointing,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode attitude quaternion");
    let (decoded, _): (AttitudeQuaternion, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode attitude quaternion");
    assert_eq!(val, decoded);
}

#[test]
fn test_attitude_modes_all_variants() {
    let modes = vec![
        AttitudeMode::SunPointing,
        AttitudeMode::EarthPointing,
        AttitudeMode::InertialHold,
        AttitudeMode::Slewing,
        AttitudeMode::SafeMode,
    ];
    for mode in &modes {
        let bytes = encode_to_vec(mode, config::standard()).expect("encode attitude mode");
        let (decoded, _): (AttitudeMode, usize) =
            decode_owned_from_slice(&bytes, config::standard()).expect("decode attitude mode");
        assert_eq!(*mode, decoded);
    }
}

#[test]
fn test_keplerian_elements_geo() {
    let val = KeplerianElements {
        object_name: "GOES-18".to_string(),
        epoch_mjd: 60_310.5,
        semi_major_axis_km: 42_164.0,
        eccentricity: 0.0001,
        inclination_deg: 0.05,
        raan_deg: 75.3,
        arg_periapsis_deg: 270.0,
        true_anomaly_deg: 120.5,
        period_seconds: 86_164.1,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode keplerian elements");
    let (decoded, _): (KeplerianElements, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode keplerian elements");
    assert_eq!(val, decoded);
}

#[test]
fn test_ground_station_pass_schedule() {
    let val = GroundStationPass {
        station_id: "GGTN-01".to_string(),
        station_name: "Goldstone DSN Complex".to_string(),
        latitude_deg: 35.4267,
        longitude_deg: -116.89,
        aos_epoch_ms: 1_735_000_000_000,
        los_epoch_ms: 1_735_000_720_000,
        max_elevation_deg: 67.3,
        azimuth_aos_deg: 145.2,
        azimuth_los_deg: 305.8,
        pass_priority: 1,
        frequency_mhz: 8450.0,
        scheduled: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode ground station pass");
    let (decoded, _): (GroundStationPass, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode ground station pass");
    assert_eq!(val, decoded);
}

#[test]
fn test_solar_panel_power_budget() {
    let val = SolarPanelPowerBudget {
        panel_id: "SAW-PORT".to_string(),
        panel_count: 2,
        area_m2: 12.5,
        efficiency_pct: 29.5,
        solar_flux_w_m2: 1361.0,
        generated_watts: 5020.0,
        consumed_watts: 3800.0,
        battery_capacity_wh: 4800.0,
        battery_soc_pct: 87.5,
        eclipse_duration_min: 35.7,
        sunlit_duration_min: 56.3,
        margin_watts: 1220.0,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode power budget");
    let (decoded, _): (SolarPanelPowerBudget, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode power budget");
    assert_eq!(val, decoded);
}

#[test]
fn test_thermal_control_readings() {
    let readings = vec![
        ThermalControlReading {
            subsystem_name: "Battery Pack A".to_string(),
            sensor_id: 101,
            temperature_c: 22.5,
            heater_on: false,
            heater_power_w: 0.0,
            radiator_area_m2: 0.8,
            min_operational_c: 5.0,
            max_operational_c: 45.0,
            zone: ThermalZone::AntiSun,
        },
        ThermalControlReading {
            subsystem_name: "Star Tracker".to_string(),
            sensor_id: 205,
            temperature_c: -12.3,
            heater_on: true,
            heater_power_w: 15.0,
            radiator_area_m2: 0.1,
            min_operational_c: -20.0,
            max_operational_c: 50.0,
            zone: ThermalZone::Ram,
        },
    ];
    let bytes = encode_to_vec(&readings, config::standard()).expect("encode thermal readings");
    let (decoded, _): (Vec<ThermalControlReading>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode thermal readings");
    assert_eq!(readings, decoded);
}

#[test]
fn test_thermal_zones_all_variants() {
    let zones = vec![
        ThermalZone::SunFacing,
        ThermalZone::AntiSun,
        ThermalZone::Ram,
        ThermalZone::Wake,
        ThermalZone::Nadir,
        ThermalZone::Zenith,
    ];
    let bytes = encode_to_vec(&zones, config::standard()).expect("encode thermal zones");
    let (decoded, _): (Vec<ThermalZone>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode thermal zones");
    assert_eq!(zones, decoded);
}

#[test]
fn test_reaction_wheel_assembly() {
    let wheels = vec![
        ReactionWheelState {
            wheel_id: 1,
            axis_label: "X+".to_string(),
            speed_rpm: 2500.0,
            max_speed_rpm: 6000.0,
            torque_nm: 0.2,
            temperature_c: 38.5,
            current_draw_a: 0.45,
            status: WheelStatus::Nominal,
            total_revolutions: 8_500_000_000,
        },
        ReactionWheelState {
            wheel_id: 2,
            axis_label: "Y+".to_string(),
            speed_rpm: -1800.0,
            max_speed_rpm: 6000.0,
            torque_nm: -0.15,
            temperature_c: 41.2,
            current_draw_a: 0.38,
            status: WheelStatus::Nominal,
            total_revolutions: 7_200_000_000,
        },
        ReactionWheelState {
            wheel_id: 3,
            axis_label: "Z+".to_string(),
            speed_rpm: 50.0,
            max_speed_rpm: 6000.0,
            torque_nm: 0.01,
            temperature_c: 55.8,
            current_draw_a: 0.52,
            status: WheelStatus::OverTemp,
            total_revolutions: 9_000_000_000,
        },
        ReactionWheelState {
            wheel_id: 4,
            axis_label: "SKEW".to_string(),
            speed_rpm: 0.0,
            max_speed_rpm: 6000.0,
            torque_nm: 0.0,
            temperature_c: 25.0,
            current_draw_a: 0.0,
            status: WheelStatus::Offline,
            total_revolutions: 3_100_000_000,
        },
    ];
    let bytes = encode_to_vec(&wheels, config::standard()).expect("encode reaction wheels");
    let (decoded, _): (Vec<ReactionWheelState>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode reaction wheels");
    assert_eq!(wheels, decoded);
}

#[test]
fn test_star_tracker_measurement() {
    let val = StarTrackerMeasurement {
        tracker_id: 1,
        timestamp_epoch_ms: 1_735_001_234_567,
        right_ascension_deg: 186.65,
        declination_deg: 12.72,
        roll_deg: 45.003,
        num_stars_tracked: 47,
        confidence: 0.9985,
        exposure_ms: 0.5,
        stray_light_detected: false,
        quaternion: vec![0.5, 0.5, 0.5, 0.5],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode star tracker");
    let (decoded, _): (StarTrackerMeasurement, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode star tracker");
    assert_eq!(val, decoded);
}

#[test]
fn test_propulsion_budget_with_maneuvers() {
    let val = PropulsionBudget {
        mission_name: "Lunar Gateway Resupply".to_string(),
        engine_type: "Bipropellant MMH/NTO".to_string(),
        specific_impulse_s: 312.0,
        total_delta_v_m_s: 1850.0,
        expended_delta_v_m_s: 420.0,
        remaining_delta_v_m_s: 1430.0,
        propellant_mass_kg: 2200.0,
        propellant_remaining_kg: 1700.0,
        burn_count: 3,
        maneuvers: vec![
            ManeuverRecord {
                maneuver_id: 1,
                epoch_ms: 1_734_500_000_000,
                delta_v_m_s: 200.0,
                duration_s: 180.5,
                direction: vec![0.707, 0.707, 0.0],
            },
            ManeuverRecord {
                maneuver_id: 2,
                epoch_ms: 1_734_800_000_000,
                delta_v_m_s: 150.0,
                duration_s: 135.2,
                direction: vec![0.0, 0.0, 1.0],
            },
            ManeuverRecord {
                maneuver_id: 3,
                epoch_ms: 1_735_000_000_000,
                delta_v_m_s: 70.0,
                duration_s: 62.8,
                direction: vec![-0.577, -0.577, -0.577],
            },
        ],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode propulsion budget");
    let (decoded, _): (PropulsionBudget, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode propulsion budget");
    assert_eq!(val, decoded);
}

#[test]
fn test_command_uplink_sequence() {
    let val = CommandUplinkSequence {
        sequence_id: 90001,
        spacecraft_id: "EUROPA-CLIPPER".to_string(),
        operator: "JPL-OPS-TEAM-A".to_string(),
        commands: vec![
            UplinkCommand {
                cmd_id: 1,
                opcode: 0x4A01,
                subsystem: "AOCS".to_string(),
                parameters: vec![1, 0, 90, 0],
                description: "Slew to science target orientation".to_string(),
            },
            UplinkCommand {
                cmd_id: 2,
                opcode: 0x5B10,
                subsystem: "PAYLOAD".to_string(),
                parameters: vec![3, 120],
                description: "Begin SUDA dust collection sequence".to_string(),
            },
            UplinkCommand {
                cmd_id: 3,
                opcode: 0x2F00,
                subsystem: "COMMS".to_string(),
                parameters: vec![8450, 1],
                description: "Switch to X-band high gain downlink".to_string(),
            },
        ],
        priority: 2,
        execution_window_start_ms: 1_735_100_000_000,
        execution_window_end_ms: 1_735_100_600_000,
        requires_confirmation: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode command sequence");
    let (decoded, _): (CommandUplinkSequence, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode command sequence");
    assert_eq!(val, decoded);
}

#[test]
fn test_dsn_schedule_entry() {
    let val = DsnScheduleEntry {
        antenna_name: "DSS-14".to_string(),
        antenna_diameter_m: 70.0,
        spacecraft_target: "VOYAGER-1".to_string(),
        start_epoch_ms: 1_735_200_000_000,
        end_epoch_ms: 1_735_228_800_000,
        band: DsnBand::XBand,
        uplink_rate_bps: 16,
        downlink_rate_bps: 160,
        round_trip_light_time_s: 46_200.0,
        allocated: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode DSN schedule");
    let (decoded, _): (DsnScheduleEntry, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode DSN schedule");
    assert_eq!(val, decoded);
}

#[test]
fn test_debris_avoidance_maneuver() {
    let val = DebrisAvoidanceManeuver {
        conjunction_id: "CDM-2025-00431".to_string(),
        debris_catalog_id: "NORAD-25544-DEB-0012".to_string(),
        time_of_closest_approach_ms: 1_735_050_000_000,
        miss_distance_m: 85.0,
        collision_probability: 1.2e-4,
        maneuver_required: true,
        maneuver_delta_v_m_s: 0.35,
        maneuver_epoch_ms: 1_735_046_000_000,
        post_maneuver_miss_distance_m: 2500.0,
        fuel_cost_kg: 0.12,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode debris avoidance");
    let (decoded, _): (DebrisAvoidanceManeuver, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode debris avoidance");
    assert_eq!(val, decoded);
}

#[test]
fn test_launch_vehicle_nominal_separation() {
    let val = LaunchVehicleStageSeparation {
        vehicle_name: "Falcon Heavy".to_string(),
        stage_number: 1,
        separation_altitude_km: 80.0,
        separation_velocity_m_s: 2200.0,
        separation_epoch_ms: 1_735_000_162_000,
        stage_mass_kg: 22_200.0,
        payload_to_next_stage_kg: 63_800.0,
        nominal: true,
        telemetry_health: StageTelemetryHealth::AllGreen,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode stage separation");
    let (decoded, _): (LaunchVehicleStageSeparation, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode stage separation");
    assert_eq!(val, decoded);
}

#[test]
fn test_launch_vehicle_anomaly_separation() {
    let val = LaunchVehicleStageSeparation {
        vehicle_name: "SLS Block 2".to_string(),
        stage_number: 2,
        separation_altitude_km: 185.0,
        separation_velocity_m_s: 7200.0,
        separation_epoch_ms: 1_735_000_500_000,
        stage_mass_kg: 3_490.0,
        payload_to_next_stage_kg: 45_000.0,
        nominal: false,
        telemetry_health: StageTelemetryHealth::MinorAnomaly {
            code: 0x0A12,
            description: "Pyrotechnic bolt 3 fired 50ms late".to_string(),
        },
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode anomaly stage separation");
    let (decoded, _): (LaunchVehicleStageSeparation, usize) =
        decode_owned_from_slice(&bytes, config::standard())
            .expect("decode anomaly stage separation");
    assert_eq!(val, decoded);
}

#[test]
fn test_onboard_data_storage() {
    let val = OnboardDataStorage {
        recorder_id: "SSR-A".to_string(),
        capacity_gb: 128.0,
        used_gb: 97.3,
        write_rate_mbps: 600.0,
        read_rate_mbps: 300.0,
        files_stored: 14_302,
        oldest_file_epoch_ms: 1_734_000_000_000,
        newest_file_epoch_ms: 1_735_100_000_000,
        error_count: 2,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode data storage");
    let (decoded, _): (OnboardDataStorage, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode data storage");
    assert_eq!(val, decoded);
}

#[test]
fn test_mission_timeline_with_events() {
    let val = MissionTimeline {
        mission_id: "DRAGONFLY".to_string(),
        phase: MissionPhase::PrimaryMission,
        phase_start_epoch_ms: 1_730_000_000_000,
        phase_end_epoch_ms: Some(1_798_000_000_000),
        elapsed_days: 58,
        objectives_total: 12,
        objectives_completed: 4,
        critical_events: vec![
            CriticalEvent {
                event_id: 1,
                name: "Titan atmospheric entry".to_string(),
                epoch_ms: 1_730_000_100_000,
                success: true,
            },
            CriticalEvent {
                event_id: 2,
                name: "First powered flight".to_string(),
                epoch_ms: 1_730_500_000_000,
                success: true,
            },
            CriticalEvent {
                event_id: 3,
                name: "Selk crater sample acquisition".to_string(),
                epoch_ms: 1_732_000_000_000,
                success: true,
            },
        ],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode mission timeline");
    let (decoded, _): (MissionTimeline, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode mission timeline");
    assert_eq!(val, decoded);
}

#[test]
fn test_telemetry_downlink_frame() {
    let val = TelemetryDownlinkFrame {
        frame_id: 1_000_000_042,
        spacecraft_id: "JWST".to_string(),
        apid: 1024,
        sequence_count: 16383,
        data_length: 65536,
        timestamp_epoch_ms: 1_735_100_500_000,
        crc_valid: true,
        reed_solomon_corrected_bits: 3,
        signal_to_noise_db: 12.4,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode telemetry frame");
    let (decoded, _): (TelemetryDownlinkFrame, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode telemetry frame");
    assert_eq!(val, decoded);
}

#[test]
fn test_orbit_determination_solution() {
    let val = OrbitDeterminationSolution {
        solution_id: "OD-2025-12-22-001".to_string(),
        epoch_ms: 1_735_000_000_000,
        position_x_km: -6578.137,
        position_y_km: 1234.567,
        position_z_km: 3456.789,
        velocity_x_km_s: -1.234,
        velocity_y_km_s: -7.123,
        velocity_z_km_s: 0.456,
        position_uncertainty_m: 15.3,
        velocity_uncertainty_mm_s: 0.8,
        drag_coefficient: 2.2,
        observations_used: 847,
        residual_rms: 0.023,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode OD solution");
    let (decoded, _): (OrbitDeterminationSolution, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode OD solution");
    assert_eq!(val, decoded);
}

#[test]
fn test_payload_instrument_roundtrip() {
    let val = PayloadInstrument {
        instrument_name: "NIRCam".to_string(),
        instrument_type: InstrumentType::OpticalImager,
        power_draw_w: 72.0,
        data_rate_kbps: 57_200.0,
        operational: true,
        temperature_c: -233.15,
        calibration_epoch_ms: 1_734_000_000_000,
        observation_count: 58_421,
        fov_deg: 2.2,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode payload instrument");
    let (decoded, _): (PayloadInstrument, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode payload instrument");
    assert_eq!(val, decoded);
}

#[test]
fn test_gravity_assist_plan() {
    let val = GravityAssistPlan {
        flyby_body: "Jupiter".to_string(),
        flyby_epoch_ms: 1_800_000_000_000,
        periapsis_altitude_km: 200_000.0,
        incoming_v_infinity_km_s: 8.5,
        outgoing_v_infinity_km_s: 8.5,
        turn_angle_deg: 12.3,
        delta_v_gained_m_s: 3200.0,
        b_plane_target_km: 500_000.0,
        b_plane_achieved_km: 500_012.5,
        navigational_corrections: vec![0.01, -0.005, 0.002, -0.001],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode gravity assist");
    let (decoded, _): (GravityAssistPlan, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode gravity assist");
    assert_eq!(val, decoded);
}

#[test]
fn test_eclipse_window_roundtrip() {
    let windows = vec![
        EclipseWindow {
            eclipse_type: EclipseType::Umbra,
            entry_epoch_ms: 1_735_002_000_000,
            exit_epoch_ms: 1_735_004_140_000,
            duration_s: 2140.0,
            depth_pct: 100.0,
            battery_discharge_wh: 310.5,
            min_temperature_c: -45.0,
            heater_activation_required: true,
        },
        EclipseWindow {
            eclipse_type: EclipseType::Penumbra,
            entry_epoch_ms: 1_735_001_800_000,
            exit_epoch_ms: 1_735_002_000_000,
            duration_s: 200.0,
            depth_pct: 35.0,
            battery_discharge_wh: 18.2,
            min_temperature_c: -10.0,
            heater_activation_required: false,
        },
    ];
    let bytes = encode_to_vec(&windows, config::standard()).expect("encode eclipse windows");
    let (decoded, _): (Vec<EclipseWindow>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode eclipse windows");
    assert_eq!(windows, decoded);
}
