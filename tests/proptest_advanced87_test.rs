//! Advanced property-based tests (set 87) — Nuclear Fission Power Plant Operations
//! and Safety Systems domain.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers reactor core temperature, control rod positions, neutron flux, primary
//! coolant loop, steam generator tubes, containment pressure, radiation dosimetry,
//! spent fuel pool, emergency diesel generators, ECCS valve positions, reactor
//! protection system trips, and more.

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

/// Reactor core thermocouple reading at a specific axial/radial position.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CoreTemperatureReading {
    /// Thermocouple channel identifier.
    channel_id: u32,
    /// Axial position from bottom of core (0.0 – 4.0 m).
    axial_position_m: f32,
    /// Radial position from core centre (0.0 – 2.5 m).
    radial_position_m: f32,
    /// Measured temperature in degrees Celsius.
    temperature_c: f32,
    /// Measurement timestamp (Unix epoch seconds).
    timestamp_s: u64,
    /// Whether the thermocouple passed its last calibration check.
    calibrated: bool,
}

/// Control rod assembly position and drive mechanism state.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ControlRodPosition {
    /// Rod cluster control assembly identifier (0–52 typical PWR).
    rcca_id: u16,
    /// Steps withdrawn from fully inserted (0 = fully inserted, 228 = fully withdrawn).
    steps_withdrawn: u16,
    /// Rod speed demand in steps per minute.
    speed_steps_per_min: f32,
    /// Drive mechanism coil current in amperes.
    coil_current_a: f32,
    /// Rod bottom signal active (fully inserted indication).
    rod_bottom: bool,
}

/// Neutron flux measurement from ex-core or in-core detectors.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NeutronFluxMeasurement {
    /// Detector identifier.
    detector_id: u32,
    /// Neutron flux in neutrons/cm^2/s (log scale stored as f64).
    flux_n_per_cm2_s: f64,
    /// Power range channel reading as a percentage of rated thermal power.
    power_range_pct: f32,
    /// Detector type: 0=source-range, 1=intermediate-range, 2=power-range.
    range_type: u8,
    /// Axial offset ratio (-1.0 to +1.0).
    axial_offset: f32,
}

/// Primary coolant loop parameters (PWR hot/cold leg).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrimaryCoolantLoop {
    /// Loop identifier (1–4 for a 4-loop PWR).
    loop_id: u8,
    /// Hot leg temperature in degrees Celsius.
    t_hot_c: f32,
    /// Cold leg temperature in degrees Celsius.
    t_cold_c: f32,
    /// Reactor coolant pump speed in RPM.
    rcp_speed_rpm: f32,
    /// Loop flow rate in kg/s.
    flow_rate_kg_per_s: f32,
    /// Pressuriser pressure in MPa.
    pressuriser_pressure_mpa: f32,
    /// Pressuriser level as fraction (0.0 – 1.0).
    pressuriser_level_frac: f32,
}

/// Steam generator tube inspection record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SteamGeneratorTube {
    /// Steam generator identifier.
    sg_id: u8,
    /// Tube row number.
    row: u16,
    /// Tube column number.
    column: u16,
    /// Wall thickness degradation as a percentage of nominal.
    degradation_pct: f32,
    /// Whether the tube is plugged (out of service).
    plugged: bool,
    /// Eddy-current test signal amplitude in volts.
    eddy_current_v: f32,
}

/// Containment building pressure and environmental reading.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ContainmentPressure {
    /// Sensor identifier.
    sensor_id: u32,
    /// Absolute pressure in kPa.
    pressure_kpa: f32,
    /// Temperature inside containment in degrees Celsius.
    temperature_c: f32,
    /// Relative humidity as fraction (0.0 – 1.0).
    humidity_frac: f32,
    /// Hydrogen concentration as volume percentage.
    hydrogen_vol_pct: f32,
}

/// Personal radiation dosimetry record (TLD or electronic dosimeter).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiationDosimetry {
    /// Dosimeter badge identifier.
    badge_id: u64,
    /// Cumulative dose in millisieverts for the monitoring period.
    dose_msv: f32,
    /// Dose rate in microsieverts per hour at time of reading.
    dose_rate_usv_per_h: f32,
    /// Whether the alarm threshold has been exceeded.
    alarm_active: bool,
    /// Worker zone code (0=green, 1=yellow, 2=orange, 3=red).
    zone_code: u8,
}

/// Spent fuel pool temperature and level monitoring.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpentFuelPool {
    /// Pool identifier (some plants have multiple pools).
    pool_id: u8,
    /// Bulk water temperature in degrees Celsius.
    water_temp_c: f32,
    /// Water level above top of fuel assemblies in metres.
    level_above_fuel_m: f32,
    /// Cooling system outlet temperature in degrees Celsius.
    cooling_outlet_c: f32,
    /// Boron concentration in ppm.
    boron_ppm: f32,
    /// Number of fuel assemblies currently stored.
    assembly_count: u16,
}

/// Emergency diesel generator operational state.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DieselGeneratorState {
    /// Standby — ready to start on demand.
    Standby {
        /// Jacket water heater temperature in degrees Celsius.
        jacket_water_temp_c: f32,
        /// Fuel oil day tank level as fraction (0.0 – 1.0).
        fuel_level_frac: f32,
    },
    /// Running — providing emergency power.
    Running {
        /// Output power in kW.
        output_kw: f32,
        /// Frequency in Hz.
        frequency_hz: f32,
        /// Lube oil pressure in kPa.
        lube_oil_pressure_kpa: f32,
        /// Exhaust temperature in degrees Celsius.
        exhaust_temp_c: f32,
    },
    /// Failed — unable to start or tripped during operation.
    Failed {
        /// Failure code.
        failure_code: u16,
    },
}

/// Emergency Core Cooling System (ECCS) valve position record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EccsValvePosition {
    /// Valve tag identifier.
    valve_tag: String,
    /// Stem position as fraction (0.0 = fully closed, 1.0 = fully open).
    stem_position_frac: f32,
    /// Whether the valve has received an automatic actuation signal.
    auto_actuated: bool,
    /// Accumulator pressure in MPa (for accumulator injection valves).
    accumulator_pressure_mpa: f32,
    /// Motor operator current in amperes (zero if air-operated).
    motor_current_a: f32,
}

/// Reactor Protection System (RPS) trip signal record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RpsTrip {
    /// No trip condition — all parameters normal.
    Normal,
    /// High neutron flux trip.
    HighFlux { flux_pct: f32 },
    /// Over-temperature delta-T trip.
    OverTemperatureDeltaT { delta_t_c: f32, setpoint_c: f32 },
    /// Low pressuriser pressure trip.
    LowPressure {
        pressure_mpa: f32,
        setpoint_mpa: f32,
    },
    /// High pressuriser level trip.
    HighPressuriserlevel { level_frac: f32 },
    /// Loss of coolant flow trip.
    LowFlow { loop_id: u8, flow_pct: f32 },
}

/// Boron concentration measurement in the Reactor Coolant System.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BoronConcentration {
    /// Sample point identifier.
    sample_point_id: u32,
    /// Boron concentration in ppm.
    boron_ppm: f32,
    /// Sample temperature in degrees Celsius.
    sample_temp_c: f32,
    /// Timestamp of sample (Unix epoch seconds).
    timestamp_s: u64,
}

/// Reactor coolant system chemistry sample.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CoolantChemistry {
    /// Sample identifier.
    sample_id: u64,
    /// pH value (at 25 degrees C).
    ph_25c: f32,
    /// Dissolved oxygen in ppb.
    dissolved_o2_ppb: f32,
    /// Lithium-7 concentration in ppm.
    lithium7_ppm: f32,
    /// Chloride concentration in ppb.
    chloride_ppb: f32,
    /// Conductivity in microsiemens per cm.
    conductivity_us_per_cm: f32,
}

/// Main turbine generator operating snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TurbineGeneratorSnapshot {
    /// Unit identifier.
    unit_id: u8,
    /// Turbine speed in RPM.
    speed_rpm: f32,
    /// Generator output in MW (electrical).
    output_mwe: f32,
    /// Main steam pressure in MPa.
    main_steam_pressure_mpa: f32,
    /// Condenser vacuum in kPa absolute.
    condenser_vacuum_kpa: f32,
    /// Generator hydrogen cooling gas pressure in kPa.
    h2_cooling_kpa: f32,
}

/// Seismic monitoring record for the plant site.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeismicReading {
    /// Accelerograph station identifier.
    station_id: u32,
    /// Peak ground acceleration in units of g.
    pga_g: f32,
    /// Cumulative absolute velocity in cm/s.
    cav_cm_per_s: f32,
    /// Operating basis earthquake (OBE) threshold exceeded.
    obe_exceeded: bool,
    /// Safe shutdown earthquake (SSE) threshold exceeded.
    sse_exceeded: bool,
}

// ── Tests 1–22 ────────────────────────────────────────────────────────────────

// ── 1. CoreTemperatureReading roundtrip ───────────────────────────────────────

#[test]
fn test_core_temperature_roundtrip() {
    proptest!(|(
        channel_id: u32,
        axial_position_m in 0.0f32..4.0f32,
        radial_position_m in 0.0f32..2.5f32,
        temperature_c in 250.0f32..350.0f32,
        timestamp_s: u64,
        calibrated: bool,
    )| {
        let val = CoreTemperatureReading {
            channel_id, axial_position_m, radial_position_m,
            temperature_c, timestamp_s, calibrated,
        };
        let enc = encode_to_vec(&val).expect("encode CoreTemperatureReading failed");
        let (dec, consumed): (CoreTemperatureReading, usize) =
            decode_from_slice(&enc).expect("decode CoreTemperatureReading failed");
        prop_assert_eq!(&val, &dec, "CoreTemperatureReading roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 2. CoreTemperatureReading deterministic encoding ──────────────────────────

#[test]
fn test_core_temperature_determinism() {
    proptest!(|(
        channel_id: u32,
        axial_position_m in 0.0f32..4.0f32,
        radial_position_m in 0.0f32..2.5f32,
        temperature_c in 250.0f32..350.0f32,
        timestamp_s: u64,
        calibrated: bool,
    )| {
        let val = CoreTemperatureReading {
            channel_id, axial_position_m, radial_position_m,
            temperature_c, timestamp_s, calibrated,
        };
        let enc1 = encode_to_vec(&val).expect("first encode CoreTemperatureReading failed");
        let enc2 = encode_to_vec(&val).expect("second encode CoreTemperatureReading failed");
        prop_assert_eq!(enc1, enc2, "encoding must be deterministic");
    });
}

// ── 3. ControlRodPosition roundtrip ───────────────────────────────────────────

#[test]
fn test_control_rod_position_roundtrip() {
    proptest!(|(
        rcca_id in 0u16..53u16,
        steps_withdrawn in 0u16..229u16,
        speed_steps_per_min in 0.0f32..72.0f32,
        coil_current_a in 0.0f32..10.0f32,
        rod_bottom: bool,
    )| {
        let val = ControlRodPosition {
            rcca_id, steps_withdrawn, speed_steps_per_min,
            coil_current_a, rod_bottom,
        };
        let enc = encode_to_vec(&val).expect("encode ControlRodPosition failed");
        let (dec, consumed): (ControlRodPosition, usize) =
            decode_from_slice(&enc).expect("decode ControlRodPosition failed");
        prop_assert_eq!(&val, &dec, "ControlRodPosition roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 4. Vec<ControlRodPosition> roundtrip ──────────────────────────────────────

#[test]
fn test_vec_control_rod_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(
            (
                0u16..53u16,
                0u16..229u16,
                0.0f32..72.0f32,
                0.0f32..10.0f32,
                any::<bool>(),
            ).prop_map(|(rcca_id, steps_withdrawn, speed_steps_per_min, coil_current_a, rod_bottom)| {
                ControlRodPosition { rcca_id, steps_withdrawn, speed_steps_per_min, coil_current_a, rod_bottom }
            }),
            0..12usize,
        ),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<ControlRodPosition> failed");
        let (dec, consumed): (Vec<ControlRodPosition>, usize) =
            decode_from_slice(&enc).expect("decode Vec<ControlRodPosition> failed");
        prop_assert_eq!(&items, &dec, "Vec<ControlRodPosition> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 5. NeutronFluxMeasurement roundtrip ───────────────────────────────────────

#[test]
fn test_neutron_flux_roundtrip() {
    proptest!(|(
        detector_id: u32,
        flux_n_per_cm2_s in 1e6f64..1e14f64,
        power_range_pct in 0.0f32..120.0f32,
        range_type in 0u8..3u8,
        axial_offset in (-1.0f32)..1.0f32,
    )| {
        let val = NeutronFluxMeasurement {
            detector_id, flux_n_per_cm2_s, power_range_pct,
            range_type, axial_offset,
        };
        let enc = encode_to_vec(&val).expect("encode NeutronFluxMeasurement failed");
        let (dec, consumed): (NeutronFluxMeasurement, usize) =
            decode_from_slice(&enc).expect("decode NeutronFluxMeasurement failed");
        prop_assert_eq!(&val, &dec, "NeutronFluxMeasurement roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 6. NeutronFluxMeasurement re-encode idempotency ───────────────────────────

#[test]
fn test_neutron_flux_reencode_idempotent() {
    proptest!(|(
        detector_id: u32,
        flux_n_per_cm2_s in 1e6f64..1e14f64,
        power_range_pct in 0.0f32..120.0f32,
        range_type in 0u8..3u8,
        axial_offset in (-1.0f32)..1.0f32,
    )| {
        let val = NeutronFluxMeasurement {
            detector_id, flux_n_per_cm2_s, power_range_pct,
            range_type, axial_offset,
        };
        let enc1 = encode_to_vec(&val).expect("first encode NeutronFluxMeasurement failed");
        let (decoded, _): (NeutronFluxMeasurement, usize) =
            decode_from_slice(&enc1).expect("decode NeutronFluxMeasurement failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode NeutronFluxMeasurement failed");
        prop_assert_eq!(enc1, enc2, "re-encoding must produce identical bytes");
    });
}

// ── 7. PrimaryCoolantLoop roundtrip ───────────────────────────────────────────

#[test]
fn test_primary_coolant_loop_roundtrip() {
    proptest!(|(
        loop_id in 1u8..5u8,
        t_hot_c in 300.0f32..330.0f32,
        t_cold_c in 280.0f32..295.0f32,
        rcp_speed_rpm in 1100.0f32..1200.0f32,
        flow_rate_kg_per_s in 4000.0f32..5500.0f32,
        pressuriser_pressure_mpa in 14.5f32..16.0f32,
        pressuriser_level_frac in 0.2f32..0.8f32,
    )| {
        let val = PrimaryCoolantLoop {
            loop_id, t_hot_c, t_cold_c, rcp_speed_rpm,
            flow_rate_kg_per_s, pressuriser_pressure_mpa, pressuriser_level_frac,
        };
        let enc = encode_to_vec(&val).expect("encode PrimaryCoolantLoop failed");
        let (dec, consumed): (PrimaryCoolantLoop, usize) =
            decode_from_slice(&enc).expect("decode PrimaryCoolantLoop failed");
        prop_assert_eq!(&val, &dec, "PrimaryCoolantLoop roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 8. SteamGeneratorTube roundtrip ───────────────────────────────────────────

#[test]
fn test_steam_generator_tube_roundtrip() {
    proptest!(|(
        sg_id in 1u8..5u8,
        row in 1u16..100u16,
        column in 1u16..150u16,
        degradation_pct in 0.0f32..100.0f32,
        plugged: bool,
        eddy_current_v in 0.0f32..10.0f32,
    )| {
        let val = SteamGeneratorTube {
            sg_id, row, column, degradation_pct, plugged, eddy_current_v,
        };
        let enc = encode_to_vec(&val).expect("encode SteamGeneratorTube failed");
        let (dec, consumed): (SteamGeneratorTube, usize) =
            decode_from_slice(&enc).expect("decode SteamGeneratorTube failed");
        prop_assert_eq!(&val, &dec, "SteamGeneratorTube roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 9. Vec<SteamGeneratorTube> roundtrip ──────────────────────────────────────

#[test]
fn test_vec_steam_generator_tube_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(
            (
                1u8..5u8,
                1u16..100u16,
                1u16..150u16,
                0.0f32..100.0f32,
                any::<bool>(),
                0.0f32..10.0f32,
            ).prop_map(|(sg_id, row, column, degradation_pct, plugged, eddy_current_v)| {
                SteamGeneratorTube { sg_id, row, column, degradation_pct, plugged, eddy_current_v }
            }),
            0..8usize,
        ),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<SteamGeneratorTube> failed");
        let (dec, consumed): (Vec<SteamGeneratorTube>, usize) =
            decode_from_slice(&enc).expect("decode Vec<SteamGeneratorTube> failed");
        prop_assert_eq!(&items, &dec, "Vec<SteamGeneratorTube> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 10. ContainmentPressure roundtrip ─────────────────────────────────────────

#[test]
fn test_containment_pressure_roundtrip() {
    proptest!(|(
        sensor_id: u32,
        pressure_kpa in 90.0f32..450.0f32,
        temperature_c in 20.0f32..150.0f32,
        humidity_frac in 0.0f32..1.0f32,
        hydrogen_vol_pct in 0.0f32..5.0f32,
    )| {
        let val = ContainmentPressure {
            sensor_id, pressure_kpa, temperature_c, humidity_frac, hydrogen_vol_pct,
        };
        let enc = encode_to_vec(&val).expect("encode ContainmentPressure failed");
        let (dec, consumed): (ContainmentPressure, usize) =
            decode_from_slice(&enc).expect("decode ContainmentPressure failed");
        prop_assert_eq!(&val, &dec, "ContainmentPressure roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 11. RadiationDosimetry roundtrip ──────────────────────────────────────────

#[test]
fn test_radiation_dosimetry_roundtrip() {
    proptest!(|(
        badge_id: u64,
        dose_msv in 0.0f32..50.0f32,
        dose_rate_usv_per_h in 0.0f32..10000.0f32,
        alarm_active: bool,
        zone_code in 0u8..4u8,
    )| {
        let val = RadiationDosimetry {
            badge_id, dose_msv, dose_rate_usv_per_h, alarm_active, zone_code,
        };
        let enc = encode_to_vec(&val).expect("encode RadiationDosimetry failed");
        let (dec, consumed): (RadiationDosimetry, usize) =
            decode_from_slice(&enc).expect("decode RadiationDosimetry failed");
        prop_assert_eq!(&val, &dec, "RadiationDosimetry roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 12. RadiationDosimetry re-encode idempotency ──────────────────────────────

#[test]
fn test_radiation_dosimetry_reencode_idempotent() {
    proptest!(|(
        badge_id: u64,
        dose_msv in 0.0f32..50.0f32,
        dose_rate_usv_per_h in 0.0f32..10000.0f32,
        alarm_active: bool,
        zone_code in 0u8..4u8,
    )| {
        let val = RadiationDosimetry {
            badge_id, dose_msv, dose_rate_usv_per_h, alarm_active, zone_code,
        };
        let enc1 = encode_to_vec(&val).expect("first encode RadiationDosimetry failed");
        let (decoded, _): (RadiationDosimetry, usize) =
            decode_from_slice(&enc1).expect("decode RadiationDosimetry failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode RadiationDosimetry failed");
        prop_assert_eq!(enc1, enc2, "re-encoding must produce identical bytes");
    });
}

// ── 13. SpentFuelPool roundtrip ───────────────────────────────────────────────

#[test]
fn test_spent_fuel_pool_roundtrip() {
    proptest!(|(
        pool_id in 1u8..4u8,
        water_temp_c in 25.0f32..65.0f32,
        level_above_fuel_m in 5.0f32..12.0f32,
        cooling_outlet_c in 20.0f32..45.0f32,
        boron_ppm in 2000.0f32..2600.0f32,
        assembly_count in 0u16..3000u16,
    )| {
        let val = SpentFuelPool {
            pool_id, water_temp_c, level_above_fuel_m,
            cooling_outlet_c, boron_ppm, assembly_count,
        };
        let enc = encode_to_vec(&val).expect("encode SpentFuelPool failed");
        let (dec, consumed): (SpentFuelPool, usize) =
            decode_from_slice(&enc).expect("decode SpentFuelPool failed");
        prop_assert_eq!(&val, &dec, "SpentFuelPool roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 14. DieselGeneratorState Standby roundtrip ────────────────────────────────

#[test]
fn test_diesel_generator_standby_roundtrip() {
    proptest!(|(
        jacket_water_temp_c in 50.0f32..80.0f32,
        fuel_level_frac in 0.5f32..1.0f32,
    )| {
        let val = DieselGeneratorState::Standby {
            jacket_water_temp_c, fuel_level_frac,
        };
        let enc = encode_to_vec(&val).expect("encode DieselGeneratorState::Standby failed");
        let (dec, consumed): (DieselGeneratorState, usize) =
            decode_from_slice(&enc).expect("decode DieselGeneratorState::Standby failed");
        prop_assert_eq!(&val, &dec, "DieselGeneratorState::Standby roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 15. DieselGeneratorState Running roundtrip ────────────────────────────────

#[test]
fn test_diesel_generator_running_roundtrip() {
    proptest!(|(
        output_kw in 500.0f32..7000.0f32,
        frequency_hz in 59.5f32..60.5f32,
        lube_oil_pressure_kpa in 300.0f32..600.0f32,
        exhaust_temp_c in 300.0f32..600.0f32,
    )| {
        let val = DieselGeneratorState::Running {
            output_kw, frequency_hz, lube_oil_pressure_kpa, exhaust_temp_c,
        };
        let enc = encode_to_vec(&val).expect("encode DieselGeneratorState::Running failed");
        let (dec, consumed): (DieselGeneratorState, usize) =
            decode_from_slice(&enc).expect("decode DieselGeneratorState::Running failed");
        prop_assert_eq!(&val, &dec, "DieselGeneratorState::Running roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 16. DieselGeneratorState Failed roundtrip ─────────────────────────────────

#[test]
fn test_diesel_generator_failed_roundtrip() {
    proptest!(|(
        failure_code: u16,
    )| {
        let val = DieselGeneratorState::Failed { failure_code };
        let enc = encode_to_vec(&val).expect("encode DieselGeneratorState::Failed failed");
        let (dec, consumed): (DieselGeneratorState, usize) =
            decode_from_slice(&enc).expect("decode DieselGeneratorState::Failed failed");
        prop_assert_eq!(&val, &dec, "DieselGeneratorState::Failed roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 17. EccsValvePosition roundtrip ───────────────────────────────────────────

#[test]
fn test_eccs_valve_position_roundtrip() {
    proptest!(|(
        valve_tag in "[A-Z]{2}-[0-9]{3}",
        stem_position_frac in 0.0f32..1.0f32,
        auto_actuated: bool,
        accumulator_pressure_mpa in 0.0f32..5.0f32,
        motor_current_a in 0.0f32..50.0f32,
    )| {
        let val = EccsValvePosition {
            valve_tag, stem_position_frac, auto_actuated,
            accumulator_pressure_mpa, motor_current_a,
        };
        let enc = encode_to_vec(&val).expect("encode EccsValvePosition failed");
        let (dec, consumed): (EccsValvePosition, usize) =
            decode_from_slice(&enc).expect("decode EccsValvePosition failed");
        prop_assert_eq!(&val, &dec, "EccsValvePosition roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 18. RpsTrip enum variant roundtrips ───────────────────────────────────────

#[test]
fn test_rps_trip_all_variants_roundtrip() {
    let trip_strategy = prop_oneof![
        Just(RpsTrip::Normal),
        (80.0f32..120.0f32).prop_map(|flux_pct| RpsTrip::HighFlux { flux_pct }),
        (20.0f32..40.0f32, 25.0f32..35.0f32).prop_map(|(delta_t_c, setpoint_c)| {
            RpsTrip::OverTemperatureDeltaT {
                delta_t_c,
                setpoint_c,
            }
        }),
        (10.0f32..15.0f32, 12.0f32..14.0f32).prop_map(|(pressure_mpa, setpoint_mpa)| {
            RpsTrip::LowPressure {
                pressure_mpa,
                setpoint_mpa,
            }
        }),
        (0.5f32..1.0f32).prop_map(|level_frac| RpsTrip::HighPressuriserlevel { level_frac }),
        (1u8..5u8, 50.0f32..100.0f32)
            .prop_map(|(loop_id, flow_pct)| { RpsTrip::LowFlow { loop_id, flow_pct } }),
    ];

    proptest!(|(val in trip_strategy)| {
        let enc = encode_to_vec(&val).expect("encode RpsTrip failed");
        let (dec, consumed): (RpsTrip, usize) =
            decode_from_slice(&enc).expect("decode RpsTrip failed");
        prop_assert_eq!(&val, &dec, "RpsTrip roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 19. BoronConcentration roundtrip ──────────────────────────────────────────

#[test]
fn test_boron_concentration_roundtrip() {
    proptest!(|(
        sample_point_id: u32,
        boron_ppm in 0.0f32..2500.0f32,
        sample_temp_c in 20.0f32..60.0f32,
        timestamp_s: u64,
    )| {
        let val = BoronConcentration {
            sample_point_id, boron_ppm, sample_temp_c, timestamp_s,
        };
        let enc = encode_to_vec(&val).expect("encode BoronConcentration failed");
        let (dec, consumed): (BoronConcentration, usize) =
            decode_from_slice(&enc).expect("decode BoronConcentration failed");
        prop_assert_eq!(&val, &dec, "BoronConcentration roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 20. CoolantChemistry roundtrip ────────────────────────────────────────────

#[test]
fn test_coolant_chemistry_roundtrip() {
    proptest!(|(
        sample_id: u64,
        ph_25c in 6.0f32..8.0f32,
        dissolved_o2_ppb in 0.0f32..100.0f32,
        lithium7_ppm in 0.0f32..5.0f32,
        chloride_ppb in 0.0f32..150.0f32,
        conductivity_us_per_cm in 0.0f32..50.0f32,
    )| {
        let val = CoolantChemistry {
            sample_id, ph_25c, dissolved_o2_ppb,
            lithium7_ppm, chloride_ppb, conductivity_us_per_cm,
        };
        let enc = encode_to_vec(&val).expect("encode CoolantChemistry failed");
        let (dec, consumed): (CoolantChemistry, usize) =
            decode_from_slice(&enc).expect("decode CoolantChemistry failed");
        prop_assert_eq!(&val, &dec, "CoolantChemistry roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 21. TurbineGeneratorSnapshot roundtrip ────────────────────────────────────

#[test]
fn test_turbine_generator_snapshot_roundtrip() {
    proptest!(|(
        unit_id in 1u8..4u8,
        speed_rpm in 1790.0f32..1810.0f32,
        output_mwe in 0.0f32..1400.0f32,
        main_steam_pressure_mpa in 5.0f32..7.5f32,
        condenser_vacuum_kpa in 3.0f32..10.0f32,
        h2_cooling_kpa in 300.0f32..450.0f32,
    )| {
        let val = TurbineGeneratorSnapshot {
            unit_id, speed_rpm, output_mwe,
            main_steam_pressure_mpa, condenser_vacuum_kpa, h2_cooling_kpa,
        };
        let enc = encode_to_vec(&val).expect("encode TurbineGeneratorSnapshot failed");
        let (dec, consumed): (TurbineGeneratorSnapshot, usize) =
            decode_from_slice(&enc).expect("decode TurbineGeneratorSnapshot failed");
        prop_assert_eq!(&val, &dec, "TurbineGeneratorSnapshot roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 22. SeismicReading roundtrip ──────────────────────────────────────────────

#[test]
fn test_seismic_reading_roundtrip() {
    proptest!(|(
        station_id: u32,
        pga_g in 0.0f32..2.0f32,
        cav_cm_per_s in 0.0f32..100.0f32,
        obe_exceeded: bool,
        sse_exceeded: bool,
    )| {
        let val = SeismicReading {
            station_id, pga_g, cav_cm_per_s, obe_exceeded, sse_exceeded,
        };
        let enc = encode_to_vec(&val).expect("encode SeismicReading failed");
        let (dec, consumed): (SeismicReading, usize) =
            decode_from_slice(&enc).expect("decode SeismicReading failed");
        prop_assert_eq!(&val, &dec, "SeismicReading roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}
