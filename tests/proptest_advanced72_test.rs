//! Advanced property-based tests (set 72) — Renewable Energy Systems domain.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers solar irradiance, wind turbine RPM, battery state-of-charge, grid
//! frequency, photovoltaic cell voltage, hydrogen fuel cell pressure, tidal
//! current speed, geothermal gradient, energy storage cycles, power factor,
//! inverter efficiency, and curtailment events.

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

/// Instantaneous solar irradiance measurement (W/m²).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SolarIrradiance {
    /// Station identifier.
    station_id: u32,
    /// Irradiance value in W/m² (0.0 – 1400.0).
    irradiance_w_per_m2: f32,
    /// Measurement timestamp (Unix seconds).
    timestamp_s: u64,
    /// Panel tilt angle in degrees.
    tilt_deg: f32,
}

/// Wind turbine operating state.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WindTurbine {
    /// Turbine identifier.
    turbine_id: u64,
    /// Rotor speed in RPM.
    rotor_rpm: f32,
    /// Nacelle yaw angle in degrees.
    yaw_deg: f32,
    /// Blade pitch angle in degrees.
    pitch_deg: f32,
    /// Output power in kW.
    power_kw: f32,
    /// Currently operational.
    active: bool,
}

/// Battery energy storage state-of-charge record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatteryStateOfCharge {
    /// Battery pack identifier.
    pack_id: u32,
    /// State of charge (0.0 – 1.0).
    soc_fraction: f32,
    /// Instantaneous charge/discharge current in amperes.
    current_a: f32,
    /// Terminal voltage in volts.
    voltage_v: f32,
    /// Cumulative charge/discharge cycles.
    cycle_count: u32,
}

/// Grid frequency deviation event.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GridFrequencyEvent {
    /// Nominal operation — measured frequency in Hz.
    Nominal { frequency_hz: f32 },
    /// Under-frequency detected below threshold.
    UnderFrequency { frequency_hz: f32, deficit_mhz: f32 },
    /// Over-frequency detected above threshold.
    OverFrequency { frequency_hz: f32, excess_mhz: f32 },
}

/// Photovoltaic cell electrical measurement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PvCellMeasurement {
    /// Cell array index.
    array_index: u16,
    /// Open-circuit voltage in volts.
    voc_v: f32,
    /// Short-circuit current in amperes.
    isc_a: f32,
    /// Maximum power point voltage in volts.
    vmpp_v: f32,
    /// Maximum power point current in amperes.
    impp_a: f32,
    /// Cell temperature in °C.
    cell_temp_c: f32,
}

/// Hydrogen fuel cell operating snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HydrogenFuelCell {
    /// Cell stack identifier.
    stack_id: u32,
    /// Hydrogen supply pressure in bar.
    h2_pressure_bar: f32,
    /// Stack output voltage in volts.
    stack_voltage_v: f32,
    /// Stack current in amperes.
    stack_current_a: f32,
    /// Membrane humidification level (0.0 – 1.0).
    humidity_fraction: f32,
}

/// Tidal current sensor reading.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TidalCurrentReading {
    /// Sensor buoy identifier.
    buoy_id: u64,
    /// Current speed in m/s.
    speed_m_per_s: f32,
    /// Current direction in degrees (0 = North).
    direction_deg: f32,
    /// Water depth of sensor in metres.
    depth_m: f32,
    /// Salinity in PSU.
    salinity_psu: f32,
}

/// Geothermal borehole gradient record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeothermalGradient {
    /// Borehole identifier.
    borehole_id: u32,
    /// Temperature at surface in °C.
    surface_temp_c: f32,
    /// Temperature at borehole bottom in °C.
    bottom_temp_c: f32,
    /// Borehole depth in metres.
    depth_m: f32,
    /// Thermal gradient in °C/km.
    gradient_c_per_km: f32,
}

/// Curtailment event record for a renewable asset.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CurtailmentEvent {
    /// Asset identifier.
    asset_id: u64,
    /// Curtailed power in MW.
    curtailed_mw: f32,
    /// Duration of curtailment in seconds.
    duration_s: u32,
    /// Reason code.
    reason_code: u8,
    /// Grid zone tag.
    zone_tag: String,
}

/// Power quality measurement at a grid connection point.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PowerQuality {
    /// Meter point identifier.
    meter_id: u32,
    /// Displacement power factor (-1.0 – 1.0).
    power_factor: f32,
    /// Total harmonic distortion (0.0 – 1.0).
    thd_fraction: f32,
    /// Reactive power in kVAr.
    reactive_kvar: f32,
    /// Active power in kW.
    active_kw: f32,
}

/// Inverter efficiency record at a specific operating point.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InverterEfficiency {
    /// Inverter serial number.
    serial: String,
    /// DC input power in kW.
    dc_input_kw: f32,
    /// AC output power in kW.
    ac_output_kw: f32,
    /// Conversion efficiency (0.0 – 1.0).
    efficiency_fraction: f32,
    /// Ambient temperature in °C.
    ambient_temp_c: f32,
}

// ── Tests 1–22 ────────────────────────────────────────────────────────────────

// ── 1. SolarIrradiance roundtrip ──────────────────────────────────────────────

#[test]
fn test_solar_irradiance_roundtrip() {
    proptest!(|(
        station_id: u32,
        irradiance_w_per_m2 in 0.0f32..1400.0f32,
        timestamp_s: u64,
        tilt_deg in 0.0f32..90.0f32,
    )| {
        let val = SolarIrradiance { station_id, irradiance_w_per_m2, timestamp_s, tilt_deg };
        let enc = encode_to_vec(&val).expect("encode SolarIrradiance failed");
        let (dec, consumed): (SolarIrradiance, usize) =
            decode_from_slice(&enc).expect("decode SolarIrradiance failed");
        prop_assert_eq!(&val, &dec, "SolarIrradiance roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 2. SolarIrradiance re-encode determinism ──────────────────────────────────

#[test]
fn test_solar_irradiance_determinism() {
    proptest!(|(
        station_id: u32,
        irradiance_w_per_m2 in 0.0f32..1400.0f32,
        timestamp_s: u64,
        tilt_deg in 0.0f32..90.0f32,
    )| {
        let val = SolarIrradiance { station_id, irradiance_w_per_m2, timestamp_s, tilt_deg };
        let enc1 = encode_to_vec(&val).expect("first encode SolarIrradiance failed");
        let enc2 = encode_to_vec(&val).expect("second encode SolarIrradiance failed");
        prop_assert_eq!(enc1, enc2, "encoding must be deterministic");
    });
}

// ── 3. WindTurbine roundtrip ──────────────────────────────────────────────────

#[test]
fn test_wind_turbine_roundtrip() {
    proptest!(|(
        turbine_id: u64,
        rotor_rpm in 0.0f32..25.0f32,
        yaw_deg in 0.0f32..360.0f32,
        pitch_deg in (-5.0f32)..90.0f32,
        power_kw in 0.0f32..5000.0f32,
        active: bool,
    )| {
        let val = WindTurbine { turbine_id, rotor_rpm, yaw_deg, pitch_deg, power_kw, active };
        let enc = encode_to_vec(&val).expect("encode WindTurbine failed");
        let (dec, consumed): (WindTurbine, usize) =
            decode_from_slice(&enc).expect("decode WindTurbine failed");
        prop_assert_eq!(&val, &dec, "WindTurbine roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 4. Vec<WindTurbine> roundtrip ─────────────────────────────────────────────

#[test]
fn test_vec_wind_turbine_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(
            (
                any::<u64>(),
                0.0f32..25.0f32,
                0.0f32..360.0f32,
                (-5.0f32)..90.0f32,
                0.0f32..5000.0f32,
                any::<bool>(),
            ).prop_map(|(turbine_id, rotor_rpm, yaw_deg, pitch_deg, power_kw, active)| {
                WindTurbine { turbine_id, rotor_rpm, yaw_deg, pitch_deg, power_kw, active }
            }),
            0..8usize,
        ),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<WindTurbine> failed");
        let (dec, consumed): (Vec<WindTurbine>, usize) =
            decode_from_slice(&enc).expect("decode Vec<WindTurbine> failed");
        prop_assert_eq!(&items, &dec, "Vec<WindTurbine> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 5. BatteryStateOfCharge roundtrip ─────────────────────────────────────────

#[test]
fn test_battery_soc_roundtrip() {
    proptest!(|(
        pack_id: u32,
        soc_fraction in 0.0f32..1.0f32,
        current_a in (-500.0f32)..500.0f32,
        voltage_v in 0.0f32..800.0f32,
        cycle_count in 0u32..10_000u32,
    )| {
        let val = BatteryStateOfCharge { pack_id, soc_fraction, current_a, voltage_v, cycle_count };
        let enc = encode_to_vec(&val).expect("encode BatteryStateOfCharge failed");
        let (dec, consumed): (BatteryStateOfCharge, usize) =
            decode_from_slice(&enc).expect("decode BatteryStateOfCharge failed");
        prop_assert_eq!(&val, &dec, "BatteryStateOfCharge roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 6. BatteryStateOfCharge re-encode idempotency ─────────────────────────────

#[test]
fn test_battery_soc_reencode_idempotent() {
    proptest!(|(
        pack_id: u32,
        soc_fraction in 0.0f32..1.0f32,
        current_a in (-500.0f32)..500.0f32,
        voltage_v in 0.0f32..800.0f32,
        cycle_count in 0u32..10_000u32,
    )| {
        let val = BatteryStateOfCharge { pack_id, soc_fraction, current_a, voltage_v, cycle_count };
        let enc1 = encode_to_vec(&val).expect("first encode BatteryStateOfCharge failed");
        let (decoded, _): (BatteryStateOfCharge, usize) =
            decode_from_slice(&enc1).expect("decode BatteryStateOfCharge failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode BatteryStateOfCharge failed");
        prop_assert_eq!(enc1, enc2, "re-encoding must produce identical bytes");
    });
}

// ── 7. GridFrequencyEvent::Nominal roundtrip ──────────────────────────────────

#[test]
fn test_grid_frequency_nominal_roundtrip() {
    proptest!(|(frequency_hz in 49.0f32..51.0f32)| {
        let val = GridFrequencyEvent::Nominal { frequency_hz };
        let enc = encode_to_vec(&val).expect("encode GridFrequencyEvent::Nominal failed");
        let (dec, consumed): (GridFrequencyEvent, usize) =
            decode_from_slice(&enc).expect("decode GridFrequencyEvent::Nominal failed");
        prop_assert_eq!(&val, &dec, "GridFrequencyEvent::Nominal roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 8. GridFrequencyEvent::UnderFrequency roundtrip ───────────────────────────

#[test]
fn test_grid_frequency_under_roundtrip() {
    proptest!(|(
        frequency_hz in 47.0f32..49.9f32,
        deficit_mhz in 0.1f32..3000.0f32,
    )| {
        let val = GridFrequencyEvent::UnderFrequency { frequency_hz, deficit_mhz };
        let enc = encode_to_vec(&val).expect("encode GridFrequencyEvent::UnderFrequency failed");
        let (dec, consumed): (GridFrequencyEvent, usize) =
            decode_from_slice(&enc).expect("decode GridFrequencyEvent::UnderFrequency failed");
        prop_assert_eq!(&val, &dec, "GridFrequencyEvent::UnderFrequency roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 9. GridFrequencyEvent::OverFrequency roundtrip ────────────────────────────

#[test]
fn test_grid_frequency_over_roundtrip() {
    proptest!(|(
        frequency_hz in 50.1f32..53.0f32,
        excess_mhz in 0.1f32..3000.0f32,
    )| {
        let val = GridFrequencyEvent::OverFrequency { frequency_hz, excess_mhz };
        let enc = encode_to_vec(&val).expect("encode GridFrequencyEvent::OverFrequency failed");
        let (dec, consumed): (GridFrequencyEvent, usize) =
            decode_from_slice(&enc).expect("decode GridFrequencyEvent::OverFrequency failed");
        prop_assert_eq!(&val, &dec, "GridFrequencyEvent::OverFrequency roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 10. PvCellMeasurement roundtrip ───────────────────────────────────────────

#[test]
fn test_pv_cell_measurement_roundtrip() {
    proptest!(|(
        array_index: u16,
        voc_v in 0.0f32..100.0f32,
        isc_a in 0.0f32..20.0f32,
        vmpp_v in 0.0f32..80.0f32,
        impp_a in 0.0f32..18.0f32,
        cell_temp_c in (-20.0f32)..85.0f32,
    )| {
        let val = PvCellMeasurement { array_index, voc_v, isc_a, vmpp_v, impp_a, cell_temp_c };
        let enc = encode_to_vec(&val).expect("encode PvCellMeasurement failed");
        let (dec, consumed): (PvCellMeasurement, usize) =
            decode_from_slice(&enc).expect("decode PvCellMeasurement failed");
        prop_assert_eq!(&val, &dec, "PvCellMeasurement roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 11. Option<PvCellMeasurement> roundtrip ───────────────────────────────────

#[test]
fn test_option_pv_cell_roundtrip() {
    proptest!(|(
        array_index: u16,
        voc_v in 0.0f32..100.0f32,
        isc_a in 0.0f32..20.0f32,
        present: bool,
    )| {
        let inner = PvCellMeasurement {
            array_index,
            voc_v,
            isc_a,
            vmpp_v: voc_v * 0.8,
            impp_a: isc_a * 0.9,
            cell_temp_c: 25.0,
        };
        let val: Option<PvCellMeasurement> = if present { Some(inner) } else { None };
        let enc = encode_to_vec(&val).expect("encode Option<PvCellMeasurement> failed");
        let (dec, consumed): (Option<PvCellMeasurement>, usize) =
            decode_from_slice(&enc).expect("decode Option<PvCellMeasurement> failed");
        prop_assert_eq!(&val, &dec, "Option<PvCellMeasurement> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 12. HydrogenFuelCell roundtrip ────────────────────────────────────────────

#[test]
fn test_hydrogen_fuel_cell_roundtrip() {
    proptest!(|(
        stack_id: u32,
        h2_pressure_bar in 1.0f32..350.0f32,
        stack_voltage_v in 0.0f32..200.0f32,
        stack_current_a in 0.0f32..1000.0f32,
        humidity_fraction in 0.0f32..1.0f32,
    )| {
        let val = HydrogenFuelCell {
            stack_id, h2_pressure_bar, stack_voltage_v, stack_current_a, humidity_fraction,
        };
        let enc = encode_to_vec(&val).expect("encode HydrogenFuelCell failed");
        let (dec, consumed): (HydrogenFuelCell, usize) =
            decode_from_slice(&enc).expect("decode HydrogenFuelCell failed");
        prop_assert_eq!(&val, &dec, "HydrogenFuelCell roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 13. Vec<HydrogenFuelCell> roundtrip ───────────────────────────────────────

#[test]
fn test_vec_hydrogen_fuel_cell_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(
            (
                any::<u32>(),
                1.0f32..350.0f32,
                0.0f32..200.0f32,
                0.0f32..1000.0f32,
                0.0f32..1.0f32,
            ).prop_map(|(stack_id, h2_pressure_bar, stack_voltage_v, stack_current_a, humidity_fraction)| {
                HydrogenFuelCell {
                    stack_id, h2_pressure_bar, stack_voltage_v, stack_current_a, humidity_fraction,
                }
            }),
            0..6usize,
        ),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<HydrogenFuelCell> failed");
        let (dec, consumed): (Vec<HydrogenFuelCell>, usize) =
            decode_from_slice(&enc).expect("decode Vec<HydrogenFuelCell> failed");
        prop_assert_eq!(&items, &dec, "Vec<HydrogenFuelCell> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 14. TidalCurrentReading roundtrip ─────────────────────────────────────────

#[test]
fn test_tidal_current_reading_roundtrip() {
    proptest!(|(
        buoy_id: u64,
        speed_m_per_s in 0.0f32..5.0f32,
        direction_deg in 0.0f32..360.0f32,
        depth_m in 0.0f32..200.0f32,
        salinity_psu in 30.0f32..40.0f32,
    )| {
        let val = TidalCurrentReading {
            buoy_id, speed_m_per_s, direction_deg, depth_m, salinity_psu,
        };
        let enc = encode_to_vec(&val).expect("encode TidalCurrentReading failed");
        let (dec, consumed): (TidalCurrentReading, usize) =
            decode_from_slice(&enc).expect("decode TidalCurrentReading failed");
        prop_assert_eq!(&val, &dec, "TidalCurrentReading roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 15. GeothermalGradient roundtrip ──────────────────────────────────────────

#[test]
fn test_geothermal_gradient_roundtrip() {
    proptest!(|(
        borehole_id: u32,
        surface_temp_c in 5.0f32..30.0f32,
        bottom_temp_c in 30.0f32..400.0f32,
        depth_m in 100.0f32..10_000.0f32,
        gradient_c_per_km in 20.0f32..80.0f32,
    )| {
        let val = GeothermalGradient {
            borehole_id, surface_temp_c, bottom_temp_c, depth_m, gradient_c_per_km,
        };
        let enc = encode_to_vec(&val).expect("encode GeothermalGradient failed");
        let (dec, consumed): (GeothermalGradient, usize) =
            decode_from_slice(&enc).expect("decode GeothermalGradient failed");
        prop_assert_eq!(&val, &dec, "GeothermalGradient roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 16. CurtailmentEvent roundtrip ────────────────────────────────────────────

#[test]
fn test_curtailment_event_roundtrip() {
    proptest!(|(
        asset_id: u64,
        curtailed_mw in 0.0f32..500.0f32,
        duration_s in 0u32..86_400u32,
        reason_code: u8,
        zone_tag in "[A-Z]{2,6}[0-9]{1,3}",
    )| {
        let val = CurtailmentEvent { asset_id, curtailed_mw, duration_s, reason_code, zone_tag };
        let enc = encode_to_vec(&val).expect("encode CurtailmentEvent failed");
        let (dec, consumed): (CurtailmentEvent, usize) =
            decode_from_slice(&enc).expect("decode CurtailmentEvent failed");
        prop_assert_eq!(&val, &dec, "CurtailmentEvent roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 17. Vec<CurtailmentEvent> roundtrip ───────────────────────────────────────

#[test]
fn test_vec_curtailment_event_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(
            (
                any::<u64>(),
                0.0f32..500.0f32,
                0u32..86_400u32,
                any::<u8>(),
                "[A-Z]{2,6}[0-9]{1,3}",
            ).prop_map(|(asset_id, curtailed_mw, duration_s, reason_code, zone_tag)| {
                CurtailmentEvent { asset_id, curtailed_mw, duration_s, reason_code, zone_tag }
            }),
            0..10usize,
        ),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<CurtailmentEvent> failed");
        let (dec, consumed): (Vec<CurtailmentEvent>, usize) =
            decode_from_slice(&enc).expect("decode Vec<CurtailmentEvent> failed");
        prop_assert_eq!(&items, &dec, "Vec<CurtailmentEvent> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 18. PowerQuality roundtrip ────────────────────────────────────────────────

#[test]
fn test_power_quality_roundtrip() {
    proptest!(|(
        meter_id: u32,
        power_factor in (-1.0f32)..1.0f32,
        thd_fraction in 0.0f32..1.0f32,
        reactive_kvar in (-10_000.0f32)..10_000.0f32,
        active_kw in 0.0f32..50_000.0f32,
    )| {
        let val = PowerQuality { meter_id, power_factor, thd_fraction, reactive_kvar, active_kw };
        let enc = encode_to_vec(&val).expect("encode PowerQuality failed");
        let (dec, consumed): (PowerQuality, usize) =
            decode_from_slice(&enc).expect("decode PowerQuality failed");
        prop_assert_eq!(&val, &dec, "PowerQuality roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 19. Option<PowerQuality> roundtrip ────────────────────────────────────────

#[test]
fn test_option_power_quality_roundtrip() {
    proptest!(|(
        meter_id: u32,
        power_factor in (-1.0f32)..1.0f32,
        present: bool,
    )| {
        let inner = PowerQuality {
            meter_id,
            power_factor,
            thd_fraction: 0.05,
            reactive_kvar: 100.0,
            active_kw: 1000.0,
        };
        let val: Option<PowerQuality> = if present { Some(inner) } else { None };
        let enc = encode_to_vec(&val).expect("encode Option<PowerQuality> failed");
        let (dec, consumed): (Option<PowerQuality>, usize) =
            decode_from_slice(&enc).expect("decode Option<PowerQuality> failed");
        prop_assert_eq!(&val, &dec, "Option<PowerQuality> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 20. InverterEfficiency roundtrip ──────────────────────────────────────────

#[test]
fn test_inverter_efficiency_roundtrip() {
    proptest!(|(
        serial in "[A-Z0-9]{6,12}",
        dc_input_kw in 0.0f32..1000.0f32,
        ac_output_kw in 0.0f32..985.0f32,
        efficiency_fraction in 0.85f32..0.99f32,
        ambient_temp_c in (-10.0f32)..60.0f32,
    )| {
        let val = InverterEfficiency {
            serial, dc_input_kw, ac_output_kw, efficiency_fraction, ambient_temp_c,
        };
        let enc = encode_to_vec(&val).expect("encode InverterEfficiency failed");
        let (dec, consumed): (InverterEfficiency, usize) =
            decode_from_slice(&enc).expect("decode InverterEfficiency failed");
        prop_assert_eq!(&val, &dec, "InverterEfficiency roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 21. InverterEfficiency re-encode determinism ──────────────────────────────

#[test]
fn test_inverter_efficiency_determinism() {
    proptest!(|(
        serial in "[A-Z0-9]{6,12}",
        dc_input_kw in 0.0f32..1000.0f32,
        ac_output_kw in 0.0f32..985.0f32,
        efficiency_fraction in 0.85f32..0.99f32,
        ambient_temp_c in (-10.0f32)..60.0f32,
    )| {
        let val = InverterEfficiency {
            serial, dc_input_kw, ac_output_kw, efficiency_fraction, ambient_temp_c,
        };
        let enc1 = encode_to_vec(&val).expect("first encode InverterEfficiency failed");
        let enc2 = encode_to_vec(&val).expect("second encode InverterEfficiency failed");
        prop_assert_eq!(enc1, enc2, "encoding must be deterministic");
    });
}

// ── 22. Mixed renewable snapshot — consumed bytes == bytes.len() ───────────────

#[test]
fn test_mixed_renewable_snapshot_consumed_bytes() {
    proptest!(|(
        station_id: u32,
        irradiance_w_per_m2 in 0.0f32..1400.0f32,
        timestamp_s: u64,
        pack_id: u32,
        soc_fraction in 0.0f32..1.0f32,
        cycle_count in 0u32..10_000u32,
        frequency_hz in 49.0f32..51.0f32,
        curtailed_mw in 0.0f32..100.0f32,
    )| {
        // Encode a heterogeneous tuple of renewable energy readings.
        let solar = SolarIrradiance {
            station_id,
            irradiance_w_per_m2,
            timestamp_s,
            tilt_deg: 30.0,
        };
        let battery = BatteryStateOfCharge {
            pack_id,
            soc_fraction,
            current_a: 50.0,
            voltage_v: 400.0,
            cycle_count,
        };
        let freq_event = GridFrequencyEvent::Nominal { frequency_hz };
        let curtail = CurtailmentEvent {
            asset_id: station_id as u64,
            curtailed_mw,
            duration_s: 300,
            reason_code: 1,
            zone_tag: String::from("EU01"),
        };

        let enc_solar = encode_to_vec(&solar).expect("encode solar snapshot failed");
        let enc_battery = encode_to_vec(&battery).expect("encode battery snapshot failed");
        let enc_freq = encode_to_vec(&freq_event).expect("encode grid freq snapshot failed");
        let enc_curtail = encode_to_vec(&curtail).expect("encode curtailment snapshot failed");

        let (_, c_solar): (SolarIrradiance, usize) =
            decode_from_slice(&enc_solar).expect("decode solar snapshot failed");
        let (_, c_battery): (BatteryStateOfCharge, usize) =
            decode_from_slice(&enc_battery).expect("decode battery snapshot failed");
        let (_, c_freq): (GridFrequencyEvent, usize) =
            decode_from_slice(&enc_freq).expect("decode grid freq snapshot failed");
        let (_, c_curtail): (CurtailmentEvent, usize) =
            decode_from_slice(&enc_curtail).expect("decode curtailment snapshot failed");

        prop_assert_eq!(c_solar, enc_solar.len(), "solar consumed bytes mismatch");
        prop_assert_eq!(c_battery, enc_battery.len(), "battery consumed bytes mismatch");
        prop_assert_eq!(c_freq, enc_freq.len(), "grid freq consumed bytes mismatch");
        prop_assert_eq!(c_curtail, enc_curtail.len(), "curtailment consumed bytes mismatch");
    });
}
