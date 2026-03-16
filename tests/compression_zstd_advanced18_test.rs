//! Advanced Zstd compression tests for OxiCode — Aerospace Telemetry & Satellite Operations domain.
//!
//! Covers encode -> compress -> decompress -> decode round-trips for types that
//! model real-world aerospace telemetry: orbital elements (TLE parameters),
//! attitude determination (quaternions, Euler angles), ground station pass
//! schedules, telemetry frames, command sequences, solar array power generation,
//! thermal control readings, reaction wheel momentum, star tracker measurements,
//! GPS position/velocity, link budget calculations, space debris tracking,
//! payload data downlinks, battery depth of discharge, and eclipse predictions.

#![cfg(feature = "compression-zstd")]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

/// Orbital reference frame for coordinate transformations.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReferenceFrame {
    Eci,
    Ecef,
    Lvlh,
    Ric,
    Tnw,
}

/// Attitude control mode for the spacecraft.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AttitudeMode {
    SunPointing,
    NadirPointing,
    InertialHold,
    TargetTracking,
    SafeMode,
    Detumble,
}

/// Frequency band used for communications.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FrequencyBand {
    Uhf,
    SBand,
    XBand,
    KaBand,
    Optical,
}

/// Thermal zone identifiers on the spacecraft bus.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ThermalZone {
    SolarPanel,
    Battery,
    Payload,
    Transponder,
    ReactionWheel,
    StarTracker,
    Thruster,
    Structure,
}

/// Reaction wheel axis designation.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WheelAxis {
    X,
    Y,
    Z,
    Skew,
}

/// Command priority level.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CommandPriority {
    Immediate,
    High,
    Normal,
    Low,
    Deferred,
}

/// Eclipse state classification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EclipseState {
    FullSun,
    Penumbra,
    Umbra,
}

/// Battery charge state classification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChargeState {
    Charging,
    Discharging,
    Trickle,
    FullCharge,
    CriticalLow,
}

/// Two-Line Element set for orbital mechanics.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TwoLineElement {
    norad_id: u32,
    epoch_year: u16,
    epoch_day_fraction: u64,
    mean_motion_rev_per_day: u64,
    eccentricity: u64,
    inclination_deg: u64,
    raan_deg: u64,
    arg_perigee_deg: u64,
    mean_anomaly_deg: u64,
    bstar_drag: i64,
    revolution_number: u32,
    classification: u8,
}

/// Quaternion representation for attitude (q = w + xi + yj + zk).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Quaternion {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

/// Euler angle set in degrees (stored as fixed-point microdegrees).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EulerAngles {
    roll_udeg: i64,
    pitch_udeg: i64,
    yaw_udeg: i64,
}

/// Attitude determination record combining multiple sensor inputs.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AttitudeDetermination {
    timestamp_us: u64,
    quaternion: Quaternion,
    euler_angles: EulerAngles,
    angular_rate_x_uras: i64,
    angular_rate_y_uras: i64,
    angular_rate_z_uras: i64,
    mode: AttitudeMode,
    estimation_error_urad: u64,
}

/// Ground station visibility pass schedule entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GroundStationPass {
    station_name: String,
    latitude_udeg: i64,
    longitude_udeg: i64,
    aos_epoch_ms: u64,
    los_epoch_ms: u64,
    max_elevation_mdeg: u32,
    azimuth_aos_mdeg: u32,
    azimuth_los_mdeg: u32,
    frequency_band: FrequencyBand,
    expected_data_volume_kb: u64,
}

/// Housekeeping telemetry frame from the spacecraft bus.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HousekeepingFrame {
    frame_id: u64,
    timestamp_us: u64,
    bus_voltage_mv: u32,
    bus_current_ma: u32,
    battery_soc_permille: u16,
    cpu_temperature_mc: i32,
    memory_used_bytes: u64,
    uptime_seconds: u64,
    reboot_count: u16,
    error_flags: u32,
}

/// Telecommand record for uplink command sequences.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CommandRecord {
    command_id: u64,
    sequence_number: u32,
    priority: CommandPriority,
    execution_time_ms: u64,
    opcode: u16,
    parameters: Vec<u8>,
    checksum: u32,
    acknowledged: bool,
}

/// Solar array power generation reading.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SolarArrayReading {
    panel_id: u8,
    timestamp_us: u64,
    voltage_mv: u32,
    current_ma: u32,
    power_mw: u32,
    temperature_mc: i32,
    sun_angle_mdeg: u32,
    degradation_permille: u16,
}

/// Thermal control system reading for a spacecraft zone.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThermalReading {
    zone: ThermalZone,
    timestamp_us: u64,
    temperature_mc: i32,
    heater_duty_cycle_permille: u16,
    radiator_position_mdeg: u32,
    heat_flux_mw_per_m2: i32,
    setpoint_mc: i32,
}

/// Reaction wheel telemetry for a single wheel.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReactionWheelState {
    axis: WheelAxis,
    timestamp_us: u64,
    speed_mrpm: i64,
    torque_unm: i64,
    momentum_unmps: i64,
    temperature_mc: i32,
    current_ma: u32,
    friction_coefficient_u: u32,
}

/// Star tracker measurement for attitude reference.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StarTrackerMeasurement {
    tracker_id: u8,
    timestamp_us: u64,
    num_stars_tracked: u16,
    quaternion: Quaternion,
    confidence_permille: u16,
    right_ascension_uas: i64,
    declination_uas: i64,
    roll_uas: i64,
    stray_light_detected: bool,
}

/// GPS-based position and velocity solution.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GpsSolution {
    timestamp_us: u64,
    frame: ReferenceFrame,
    position_x_um: i64,
    position_y_um: i64,
    position_z_um: i64,
    velocity_x_ums: i64,
    velocity_y_ums: i64,
    velocity_z_ums: i64,
    pdop_milli: u32,
    num_satellites: u8,
    fix_valid: bool,
}

/// Link budget calculation for a communication pass.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LinkBudget {
    pass_id: u64,
    frequency_band: FrequencyBand,
    eirp_mdbw: i32,
    path_loss_mdb: i32,
    atmospheric_loss_mdb: i32,
    antenna_gain_mdb: i32,
    noise_temperature_mk: u32,
    snr_mdb: i32,
    bit_rate_bps: u64,
    margin_mdb: i32,
    coding_gain_mdb: i32,
}

/// Tracked space debris object near the spacecraft.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DebrisTrack {
    catalog_id: u64,
    timestamp_us: u64,
    range_um: u64,
    range_rate_ums: i64,
    azimuth_uas: i64,
    elevation_uas: i64,
    estimated_size_mm: u32,
    miss_distance_um: u64,
    probability_of_collision_ppb: u32,
    maneuver_recommended: bool,
}

/// Payload data downlink session record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PayloadDownlink {
    session_id: u64,
    payload_name: String,
    start_time_ms: u64,
    end_time_ms: u64,
    total_bytes: u64,
    bytes_transferred: u64,
    frames_sent: u32,
    frames_lost: u32,
    frequency_band: FrequencyBand,
    bit_error_rate_ppb: u32,
}

/// Battery depth-of-discharge and health telemetry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatteryTelemetry {
    cell_id: u8,
    timestamp_us: u64,
    voltage_mv: u32,
    current_ma: i32,
    temperature_mc: i32,
    depth_of_discharge_permille: u16,
    charge_state: ChargeState,
    cycle_count: u32,
    internal_resistance_mohm: u32,
    capacity_remaining_mah: u32,
}

/// Eclipse prediction window for orbital thermal and power planning.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EclipsePrediction {
    orbit_number: u32,
    entry_epoch_ms: u64,
    exit_epoch_ms: u64,
    duration_ms: u32,
    state: EclipseState,
    max_shadow_depth_permille: u16,
    solar_beta_angle_mdeg: i32,
    expected_temp_drop_mc: i32,
    battery_dod_increase_permille: u16,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn round_trip<T>(value: &T) -> T
where
    T: Encode + Decode + std::fmt::Debug + PartialEq,
{
    let encoded = encode_to_vec(value).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (T, _) = decode_from_slice(&decompressed).expect("decode_from_slice failed");
    decoded
}

fn compressed_ratio(data: &[u8]) -> f64 {
    let compressed = compress(data, Compression::Zstd).expect("compress for ratio failed");
    compressed.len() as f64 / data.len() as f64
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_tle_orbital_elements() {
    let tle = TwoLineElement {
        norad_id: 25544,
        epoch_year: 2026,
        epoch_day_fraction: 74_500_000_000,
        mean_motion_rev_per_day: 15_492_513_000,
        eccentricity: 6_777_000,
        inclination_deg: 51_644_200_000,
        raan_deg: 247_463_100_000,
        arg_perigee_deg: 130_536_000_000,
        mean_anomaly_deg: 234_817_000_000,
        bstar_drag: 34_817,
        revolution_number: 42987,
        classification: b'U',
    };
    let result = round_trip(&tle);
    assert_eq!(tle, result);
}

#[test]
fn test_attitude_determination_sunpointing() {
    let att = AttitudeDetermination {
        timestamp_us: 1_710_000_000_000,
        quaternion: Quaternion {
            w: 999_847_695,
            x: -1_000_000,
            y: 17_452_406,
            z: 342_020,
        },
        euler_angles: EulerAngles {
            roll_udeg: 0,
            pitch_udeg: -2_000_000,
            yaw_udeg: 1_000_000,
        },
        angular_rate_x_uras: 50,
        angular_rate_y_uras: -30,
        angular_rate_z_uras: 10,
        mode: AttitudeMode::SunPointing,
        estimation_error_urad: 175,
    };
    let result = round_trip(&att);
    assert_eq!(att, result);
}

#[test]
fn test_attitude_modes_all_variants() {
    let modes = vec![
        AttitudeMode::SunPointing,
        AttitudeMode::NadirPointing,
        AttitudeMode::InertialHold,
        AttitudeMode::TargetTracking,
        AttitudeMode::SafeMode,
        AttitudeMode::Detumble,
    ];
    let result = round_trip(&modes);
    assert_eq!(modes, result);
}

#[test]
fn test_ground_station_pass_schedule() {
    let passes = vec![
        GroundStationPass {
            station_name: "Svalbard SvalSat".to_string(),
            latitude_udeg: 78_229_772_000,
            longitude_udeg: 15_407_530_000,
            aos_epoch_ms: 1_742_000_000_000,
            los_epoch_ms: 1_742_000_720_000,
            max_elevation_mdeg: 78_300,
            azimuth_aos_mdeg: 195_000,
            azimuth_los_mdeg: 15_000,
            frequency_band: FrequencyBand::XBand,
            expected_data_volume_kb: 512_000,
        },
        GroundStationPass {
            station_name: "Troll Station Antarctica".to_string(),
            latitude_udeg: -72_011_667_000,
            longitude_udeg: 2_535_000_000,
            aos_epoch_ms: 1_742_005_400_000,
            los_epoch_ms: 1_742_006_100_000,
            max_elevation_mdeg: 45_200,
            azimuth_aos_mdeg: 320_000,
            azimuth_los_mdeg: 40_000,
            frequency_band: FrequencyBand::SBand,
            expected_data_volume_kb: 128_000,
        },
    ];
    let result = round_trip(&passes);
    assert_eq!(passes, result);
}

#[test]
fn test_housekeeping_frame_nominal() {
    let frame = HousekeepingFrame {
        frame_id: 1_000_001,
        timestamp_us: 1_710_100_000_000,
        bus_voltage_mv: 28_200,
        bus_current_ma: 3_150,
        battery_soc_permille: 870,
        cpu_temperature_mc: 42_500,
        memory_used_bytes: 134_217_728,
        uptime_seconds: 8_640_000,
        reboot_count: 3,
        error_flags: 0x0000_0000,
    };
    let result = round_trip(&frame);
    assert_eq!(frame, result);
}

#[test]
fn test_housekeeping_frames_bulk_compression_ratio() {
    let frames: Vec<HousekeepingFrame> = (0..200)
        .map(|i| HousekeepingFrame {
            frame_id: i as u64,
            timestamp_us: 1_710_000_000_000 + (i as u64) * 1_000_000,
            bus_voltage_mv: 28_000 + (i % 500) as u32,
            bus_current_ma: 3_000 + (i % 300) as u32,
            battery_soc_permille: 800 + (i % 200) as u16,
            cpu_temperature_mc: 40_000 + (i % 5000) as i32,
            memory_used_bytes: 134_000_000 + (i as u64) * 1024,
            uptime_seconds: 8_640_000 + i as u64,
            reboot_count: 2,
            error_flags: 0,
        })
        .collect();
    let encoded = encode_to_vec(&frames).expect("encode bulk hk");
    let ratio = compressed_ratio(&encoded);
    assert!(
        ratio < 1.0,
        "Zstd should compress repetitive housekeeping data, ratio = {ratio}"
    );
    let result = round_trip(&frames);
    assert_eq!(frames, result);
}

#[test]
fn test_command_sequence_with_parameters() {
    let commands = vec![
        CommandRecord {
            command_id: 5001,
            sequence_number: 1,
            priority: CommandPriority::High,
            execution_time_ms: 1_742_001_000_000,
            opcode: 0x0A01,
            parameters: vec![0x01, 0x02, 0x03, 0xFF],
            checksum: 0xDEAD_BEEF,
            acknowledged: true,
        },
        CommandRecord {
            command_id: 5002,
            sequence_number: 2,
            priority: CommandPriority::Immediate,
            execution_time_ms: 1_742_001_000_500,
            opcode: 0x0B10,
            parameters: vec![0x10, 0x20],
            checksum: 0xCAFE_BABE,
            acknowledged: false,
        },
        CommandRecord {
            command_id: 5003,
            sequence_number: 3,
            priority: CommandPriority::Deferred,
            execution_time_ms: 1_742_080_000_000,
            opcode: 0x0C00,
            parameters: vec![],
            checksum: 0x1234_5678,
            acknowledged: false,
        },
    ];
    let result = round_trip(&commands);
    assert_eq!(commands, result);
}

#[test]
fn test_solar_array_multi_panel() {
    let panels: Vec<SolarArrayReading> = (0..6)
        .map(|panel_id| SolarArrayReading {
            panel_id,
            timestamp_us: 1_710_200_000_000,
            voltage_mv: 32_400 - (panel_id as u32) * 200,
            current_ma: 2_800 + (panel_id as u32) * 50,
            power_mw: 90_720 + (panel_id as u32) * 1_000,
            temperature_mc: 65_000 + (panel_id as i32) * 3_000,
            sun_angle_mdeg: 5_000 + (panel_id as u32) * 1_000,
            degradation_permille: 15 + (panel_id as u16) * 2,
        })
        .collect();
    let result = round_trip(&panels);
    assert_eq!(panels, result);
}

#[test]
fn test_thermal_control_all_zones() {
    let readings = vec![
        ThermalReading {
            zone: ThermalZone::SolarPanel,
            timestamp_us: 1_710_300_000_000,
            temperature_mc: 85_000,
            heater_duty_cycle_permille: 0,
            radiator_position_mdeg: 0,
            heat_flux_mw_per_m2: 1361_000,
            setpoint_mc: 80_000,
        },
        ThermalReading {
            zone: ThermalZone::Battery,
            timestamp_us: 1_710_300_000_000,
            temperature_mc: 22_000,
            heater_duty_cycle_permille: 350,
            radiator_position_mdeg: 45_000,
            heat_flux_mw_per_m2: -500,
            setpoint_mc: 20_000,
        },
        ThermalReading {
            zone: ThermalZone::Payload,
            timestamp_us: 1_710_300_000_000,
            temperature_mc: -15_000,
            heater_duty_cycle_permille: 950,
            radiator_position_mdeg: 0,
            heat_flux_mw_per_m2: -12_000,
            setpoint_mc: -20_000,
        },
        ThermalReading {
            zone: ThermalZone::Transponder,
            timestamp_us: 1_710_300_000_000,
            temperature_mc: 55_000,
            heater_duty_cycle_permille: 0,
            radiator_position_mdeg: 90_000,
            heat_flux_mw_per_m2: 25_000,
            setpoint_mc: 50_000,
        },
        ThermalReading {
            zone: ThermalZone::Thruster,
            timestamp_us: 1_710_300_000_000,
            temperature_mc: 5_000,
            heater_duty_cycle_permille: 200,
            radiator_position_mdeg: 0,
            heat_flux_mw_per_m2: -8_000,
            setpoint_mc: 10_000,
        },
    ];
    let result = round_trip(&readings);
    assert_eq!(readings, result);
}

#[test]
fn test_reaction_wheel_four_axis_assembly() {
    let wheels = vec![
        ReactionWheelState {
            axis: WheelAxis::X,
            timestamp_us: 1_710_400_000_000,
            speed_mrpm: 3_500_000,
            torque_unm: 200,
            momentum_unmps: 15_000_000,
            temperature_mc: 38_000,
            current_ma: 120,
            friction_coefficient_u: 50,
        },
        ReactionWheelState {
            axis: WheelAxis::Y,
            timestamp_us: 1_710_400_000_000,
            speed_mrpm: -2_100_000,
            torque_unm: -150,
            momentum_unmps: -9_000_000,
            temperature_mc: 36_500,
            current_ma: 95,
            friction_coefficient_u: 48,
        },
        ReactionWheelState {
            axis: WheelAxis::Z,
            timestamp_us: 1_710_400_000_000,
            speed_mrpm: 800_000,
            torque_unm: 50,
            momentum_unmps: 3_500_000,
            temperature_mc: 37_200,
            current_ma: 60,
            friction_coefficient_u: 52,
        },
        ReactionWheelState {
            axis: WheelAxis::Skew,
            timestamp_us: 1_710_400_000_000,
            speed_mrpm: -500_000,
            torque_unm: -30,
            momentum_unmps: -2_100_000,
            temperature_mc: 35_800,
            current_ma: 45,
            friction_coefficient_u: 55,
        },
    ];
    let result = round_trip(&wheels);
    assert_eq!(wheels, result);
}

#[test]
fn test_star_tracker_measurement_high_confidence() {
    let measurement = StarTrackerMeasurement {
        tracker_id: 1,
        timestamp_us: 1_710_500_000_000,
        num_stars_tracked: 47,
        quaternion: Quaternion {
            w: 707_106_781,
            x: 0,
            y: 707_106_781,
            z: 0,
        },
        confidence_permille: 998,
        right_ascension_uas: 180_000_000_000,
        declination_uas: -45_000_000_000,
        roll_uas: 1_234_567,
        stray_light_detected: false,
    };
    let result = round_trip(&measurement);
    assert_eq!(measurement, result);
}

#[test]
fn test_star_tracker_stray_light_degraded() {
    let measurement = StarTrackerMeasurement {
        tracker_id: 2,
        timestamp_us: 1_710_500_500_000,
        num_stars_tracked: 3,
        quaternion: Quaternion {
            w: 500_000_000,
            x: 500_000_000,
            y: 500_000_000,
            z: 500_000_000,
        },
        confidence_permille: 120,
        right_ascension_uas: 90_000_000_000,
        declination_uas: 10_000_000_000,
        roll_uas: -500_000,
        stray_light_detected: true,
    };
    let result = round_trip(&measurement);
    assert_eq!(measurement, result);
    assert!(measurement.stray_light_detected);
    assert!(measurement.confidence_permille < 500);
}

#[test]
fn test_gps_solution_full_fix() {
    let solution = GpsSolution {
        timestamp_us: 1_710_600_000_000,
        frame: ReferenceFrame::Eci,
        position_x_um: 6_778_137_000_000_000,
        position_y_um: 1_234_567_000_000_000,
        position_z_um: -3_456_789_000_000_000,
        velocity_x_ums: -1_234_000_000,
        velocity_y_ums: 7_654_000_000,
        velocity_z_ums: 321_000_000,
        pdop_milli: 1_800,
        num_satellites: 12,
        fix_valid: true,
    };
    let result = round_trip(&solution);
    assert_eq!(solution, result);
}

#[test]
fn test_gps_solutions_time_series_compression() {
    let solutions: Vec<GpsSolution> = (0..100)
        .map(|i| GpsSolution {
            timestamp_us: 1_710_600_000_000 + (i as u64) * 1_000_000,
            frame: ReferenceFrame::Eci,
            position_x_um: 6_778_137_000_000_000 + (i as i64) * 7_654_000,
            position_y_um: 1_234_567_000_000_000 + (i as i64) * 1_234_000,
            position_z_um: -3_456_789_000_000_000 - (i as i64) * 321_000,
            velocity_x_ums: -1_234_000_000 + (i as i64) * 10,
            velocity_y_ums: 7_654_000_000 - (i as i64) * 5,
            velocity_z_ums: 321_000_000 + (i as i64) * 2,
            pdop_milli: 1_800 + (i % 10) as u32,
            num_satellites: 10 + (i % 4) as u8,
            fix_valid: true,
        })
        .collect();
    let encoded = encode_to_vec(&solutions).expect("encode gps series");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress gps series");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd must compress GPS time series: {} vs {}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("decompress gps series");
    let (decoded, _): (Vec<GpsSolution>, _) =
        decode_from_slice(&decompressed).expect("decode gps series");
    assert_eq!(solutions, decoded);
}

#[test]
fn test_link_budget_xband() {
    let budget = LinkBudget {
        pass_id: 42,
        frequency_band: FrequencyBand::XBand,
        eirp_mdbw: 13_000,
        path_loss_mdb: -182_000,
        atmospheric_loss_mdb: -500,
        antenna_gain_mdb: 45_000,
        noise_temperature_mk: 135_000,
        snr_mdb: 12_500,
        bit_rate_bps: 150_000_000,
        margin_mdb: 3_200,
        coding_gain_mdb: 7_500,
    };
    let result = round_trip(&budget);
    assert_eq!(budget, result);
}

#[test]
fn test_space_debris_tracking_conjunction() {
    let debris = vec![
        DebrisTrack {
            catalog_id: 39_084,
            timestamp_us: 1_710_700_000_000,
            range_um: 15_000_000_000_000,
            range_rate_ums: -500_000_000,
            azimuth_uas: 45_000_000_000,
            elevation_uas: 30_000_000_000,
            estimated_size_mm: 100,
            miss_distance_um: 500_000_000_000,
            probability_of_collision_ppb: 1_500,
            maneuver_recommended: false,
        },
        DebrisTrack {
            catalog_id: 41_335,
            timestamp_us: 1_710_700_500_000,
            range_um: 2_000_000_000_000,
            range_rate_ums: -1_200_000_000,
            azimuth_uas: 120_000_000_000,
            elevation_uas: 5_000_000_000,
            estimated_size_mm: 50,
            miss_distance_um: 50_000_000_000,
            probability_of_collision_ppb: 150_000,
            maneuver_recommended: true,
        },
    ];
    let result = round_trip(&debris);
    assert_eq!(debris, result);
    assert!(debris[1].maneuver_recommended);
    assert!(debris[1].probability_of_collision_ppb > debris[0].probability_of_collision_ppb);
}

#[test]
fn test_payload_downlink_session() {
    let downlink = PayloadDownlink {
        session_id: 9001,
        payload_name: "Earth Observation Imager".to_string(),
        start_time_ms: 1_742_010_000_000,
        end_time_ms: 1_742_010_600_000,
        total_bytes: 2_147_483_648,
        bytes_transferred: 2_100_000_000,
        frames_sent: 524_288,
        frames_lost: 128,
        frequency_band: FrequencyBand::KaBand,
        bit_error_rate_ppb: 50,
    };
    let result = round_trip(&downlink);
    assert_eq!(downlink, result);
}

#[test]
fn test_battery_depth_of_discharge_cycle() {
    let cells: Vec<BatteryTelemetry> = (0..8)
        .map(|cell_id| BatteryTelemetry {
            cell_id,
            timestamp_us: 1_710_800_000_000,
            voltage_mv: 3_700 + (cell_id as u32) * 10,
            current_ma: if cell_id % 2 == 0 { 500 } else { -500 },
            temperature_mc: 22_000 + (cell_id as i32) * 500,
            depth_of_discharge_permille: 250 + (cell_id as u16) * 5,
            charge_state: if cell_id % 2 == 0 {
                ChargeState::Charging
            } else {
                ChargeState::Discharging
            },
            cycle_count: 4_500 + (cell_id as u32) * 100,
            internal_resistance_mohm: 35 + (cell_id as u32) * 2,
            capacity_remaining_mah: 9_500 - (cell_id as u32) * 50,
        })
        .collect();
    let result = round_trip(&cells);
    assert_eq!(cells, result);
}

#[test]
fn test_eclipse_prediction_sequence() {
    let predictions = vec![
        EclipsePrediction {
            orbit_number: 47_001,
            entry_epoch_ms: 1_742_020_000_000,
            exit_epoch_ms: 1_742_022_160_000,
            duration_ms: 2_160_000,
            state: EclipseState::Umbra,
            max_shadow_depth_permille: 1000,
            solar_beta_angle_mdeg: 25_000,
            expected_temp_drop_mc: -35_000,
            battery_dod_increase_permille: 80,
        },
        EclipsePrediction {
            orbit_number: 47_001,
            entry_epoch_ms: 1_742_019_800_000,
            exit_epoch_ms: 1_742_020_000_000,
            duration_ms: 200_000,
            state: EclipseState::Penumbra,
            max_shadow_depth_permille: 450,
            solar_beta_angle_mdeg: 25_000,
            expected_temp_drop_mc: -5_000,
            battery_dod_increase_permille: 10,
        },
        EclipsePrediction {
            orbit_number: 47_002,
            entry_epoch_ms: 1_742_025_500_000,
            exit_epoch_ms: 1_742_027_600_000,
            duration_ms: 2_100_000,
            state: EclipseState::Umbra,
            max_shadow_depth_permille: 1000,
            solar_beta_angle_mdeg: 24_500,
            expected_temp_drop_mc: -34_000,
            battery_dod_increase_permille: 78,
        },
    ];
    let result = round_trip(&predictions);
    assert_eq!(predictions, result);
}

#[test]
fn test_full_satellite_state_snapshot() {
    let tle = TwoLineElement {
        norad_id: 55_001,
        epoch_year: 2026,
        epoch_day_fraction: 74_123_456_789,
        mean_motion_rev_per_day: 14_800_000_000,
        eccentricity: 1_500_000,
        inclination_deg: 97_400_000_000,
        raan_deg: 120_000_000_000,
        arg_perigee_deg: 90_000_000_000,
        mean_anomaly_deg: 270_000_000_000,
        bstar_drag: 12_345,
        revolution_number: 1_200,
        classification: b'U',
    };
    let gps = GpsSolution {
        timestamp_us: 1_710_900_000_000,
        frame: ReferenceFrame::Ecef,
        position_x_um: 4_200_000_000_000_000,
        position_y_um: 2_000_000_000_000_000,
        position_z_um: 5_500_000_000_000_000,
        velocity_x_ums: -3_000_000_000,
        velocity_y_ums: 6_000_000_000,
        velocity_z_ums: 1_500_000_000,
        pdop_milli: 2_100,
        num_satellites: 9,
        fix_valid: true,
    };
    let attitude = AttitudeDetermination {
        timestamp_us: 1_710_900_000_000,
        quaternion: Quaternion {
            w: 923_879_532,
            x: 0,
            y: 0,
            z: 382_683_432,
        },
        euler_angles: EulerAngles {
            roll_udeg: 0,
            pitch_udeg: 0,
            yaw_udeg: 45_000_000,
        },
        angular_rate_x_uras: 1_047,
        angular_rate_y_uras: 0,
        angular_rate_z_uras: 0,
        mode: AttitudeMode::NadirPointing,
        estimation_error_urad: 100,
    };
    let snapshot = (tle.clone(), gps.clone(), attitude.clone());
    let result = round_trip(&snapshot);
    assert_eq!(snapshot, result);
}

#[test]
fn test_mixed_telemetry_compression_efficiency() {
    let thermal_readings: Vec<ThermalReading> = (0..50)
        .map(|i| ThermalReading {
            zone: match i % 8 {
                0 => ThermalZone::SolarPanel,
                1 => ThermalZone::Battery,
                2 => ThermalZone::Payload,
                3 => ThermalZone::Transponder,
                4 => ThermalZone::ReactionWheel,
                5 => ThermalZone::StarTracker,
                6 => ThermalZone::Thruster,
                _ => ThermalZone::Structure,
            },
            timestamp_us: 1_711_000_000_000 + (i as u64) * 500_000,
            temperature_mc: 20_000 + ((i as i32) % 40) * 1_000,
            heater_duty_cycle_permille: (i as u16) * 20,
            radiator_position_mdeg: 45_000,
            heat_flux_mw_per_m2: 500 + (i as i32) * 100,
            setpoint_mc: 25_000,
        })
        .collect();

    let wheels: Vec<ReactionWheelState> = (0..50)
        .map(|i| ReactionWheelState {
            axis: match i % 4 {
                0 => WheelAxis::X,
                1 => WheelAxis::Y,
                2 => WheelAxis::Z,
                _ => WheelAxis::Skew,
            },
            timestamp_us: 1_711_000_000_000 + (i as u64) * 500_000,
            speed_mrpm: 2_000_000 + ((i as i64) % 20) * 100_000,
            torque_unm: (i as i64) * 10,
            momentum_unmps: 10_000_000 + (i as i64) * 50_000,
            temperature_mc: 35_000 + (i as i32) * 50,
            current_ma: 100 + (i as u32) % 50,
            friction_coefficient_u: 50,
        })
        .collect();

    let combined = (thermal_readings.clone(), wheels.clone());
    let encoded = encode_to_vec(&combined).expect("encode mixed telemetry");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress mixed telemetry");
    let decompressed = decompress(&compressed).expect("decompress mixed telemetry");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd should compress mixed telemetry: {} vs {}",
        compressed.len(),
        encoded.len()
    );

    let (decoded, _): ((Vec<ThermalReading>, Vec<ReactionWheelState>), _) =
        decode_from_slice(&decompressed).expect("decode mixed telemetry");
    assert_eq!(combined, decoded);
}

#[test]
fn test_battery_critical_low_state() {
    let critical_cell = BatteryTelemetry {
        cell_id: 0,
        timestamp_us: 1_711_100_000_000,
        voltage_mv: 2_900,
        current_ma: -2_000,
        temperature_mc: 45_000,
        depth_of_discharge_permille: 950,
        charge_state: ChargeState::CriticalLow,
        cycle_count: 12_000,
        internal_resistance_mohm: 85,
        capacity_remaining_mah: 500,
    };
    let result = round_trip(&critical_cell);
    assert_eq!(critical_cell, result);
    assert_eq!(result.charge_state, ChargeState::CriticalLow);
    assert!(result.depth_of_discharge_permille > 900);
}
