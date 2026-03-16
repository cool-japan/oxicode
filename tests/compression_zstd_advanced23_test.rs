//! Advanced Zstd compression tests for OxiCode — Power Grid SCADA & Energy Distribution domain.
//!
//! Covers encode → compress → decompress → decode round-trips for types that
//! model real-world power grid data: substation bus voltages, transformer tap
//! positions, circuit breaker status, load flow analysis results, fault current
//! calculations, protection relay settings, automatic generation control,
//! demand-side management, PMU synchrophasors, SCADA alarm hierarchies,
//! outage management records, vegetation management zones, power quality
//! metrics, distributed energy resource management, and transmission congestion
//! pricing.

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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BreakerState {
    Open,
    Closed,
    Tripped,
    Racking,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VoltageLevel {
    Kv765,
    Kv500,
    Kv345,
    Kv230,
    Kv138,
    Kv69,
    Kv34,
    Kv13,
    Kv4,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PhaseId {
    A,
    B,
    C,
    Neutral,
    Ground,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AlarmSeverity {
    Information,
    Warning,
    Minor,
    Major,
    Critical,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RelayFunction {
    Overcurrent,
    DistanceZone1,
    DistanceZone2,
    DistanceZone3,
    Differential,
    UnderFrequency,
    OverFrequency,
    UnderVoltage,
    OverVoltage,
    SyncCheck,
    Reclosing,
    BreakerFailure,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OutageType {
    Planned,
    ForcedEquipment,
    ForcedWeather,
    ForcedVegetation,
    ForcedAnimal,
    ForcedUnknown,
    MomentaryInterruption,
    SustainedInterruption,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DerType {
    SolarPv,
    WindTurbine,
    BatteryStorage,
    MicroTurbine,
    FuelCell,
    CombinedHeatPower,
    ElectricVehicleCharger,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CongestionZone {
    Unconstrained,
    Marginal,
    Constrained,
    Binding,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VegetationRisk {
    Low,
    Moderate,
    High,
    Imminent,
}

/// Substation bus voltage measurement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BusVoltage {
    substation_id: u32,
    bus_number: u16,
    voltage_level: VoltageLevel,
    phase: PhaseId,
    voltage_pu_millionths: u32, // per-unit × 10^6
    angle_microdeg: i32,        // degrees × 10^6
    timestamp_us: u64,
}

/// Transformer tap changer position and status.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TransformerTap {
    transformer_id: u32,
    winding: u8,
    tap_position: i16,
    tap_min: i16,
    tap_max: i16,
    voltage_ratio_pu_millionths: u32,
    auto_mode: bool,
    last_change_us: u64,
}

/// Circuit breaker with detailed status.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CircuitBreaker {
    breaker_id: u32,
    substation_id: u32,
    bay_name: String,
    state: BreakerState,
    rated_kv: u32,
    rated_ka_interrupting: u32,
    trip_count: u32,
    last_maintenance_day: u32,
    sf6_pressure_kpa: u16,
}

/// Load flow analysis result for a single bus.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LoadFlowResult {
    bus_id: u32,
    voltage_pu_millionths: u32,
    angle_microdeg: i32,
    p_mw_milliwatts: i64,
    q_mvar_milliwatts: i64,
    converged: bool,
    iteration_count: u16,
}

/// Fault current calculation output.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FaultCurrent {
    fault_id: u32,
    bus_id: u32,
    fault_type: u8, // 0=3ph, 1=SLG, 2=LL, 3=LLG
    i_sym_ka_millionths: u64,
    i_asym_ka_millionths: u64,
    x_r_ratio_thousandths: u32,
    breaker_duty_percent: u16,
    contributing_sources: Vec<u32>,
}

/// Protection relay setting group.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RelaySettings {
    relay_id: u32,
    function: RelayFunction,
    pickup_milliamps: u32,
    time_dial_thousandths: u32,
    curve_type: u8,
    instantaneous_milliamps: u32,
    ct_ratio_primary: u32,
    ct_ratio_secondary: u16,
    pt_ratio_primary: u32,
    pt_ratio_secondary: u16,
    enabled: bool,
}

/// Automatic generation control (AGC) signal.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AgcSignal {
    area_id: u32,
    ace_mw_milliwatts: i64, // area control error
    frequency_bias_mw_hz: i64,
    scheduled_interchange_mw: i64,
    actual_interchange_mw: i64,
    frequency_deviation_micro_hz: i32,
    timestamp_us: u64,
    units_on_regulation: Vec<u32>,
}

/// Demand-side management event.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DsmEvent {
    event_id: u64,
    program_name: String,
    reduction_kw_milliwatts: i64,
    participants: u32,
    start_timestamp_us: u64,
    duration_minutes: u32,
    incentive_cents_per_kwh: u16,
    region_id: u16,
}

/// PMU synchrophasor measurement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PmuMeasurement {
    pmu_id: u32,
    timestamp_us: u64,
    voltage_magnitude_pu_millionths: u32,
    voltage_angle_microdeg: i32,
    current_magnitude_milliamps: u32,
    current_angle_microdeg: i32,
    frequency_micro_hz: u64,
    rocof_mhz_per_sec: i32, // rate of change of frequency
    phase: PhaseId,
    data_quality: u8,
}

/// SCADA alarm entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScadaAlarm {
    alarm_id: u64,
    severity: AlarmSeverity,
    point_name: String,
    substation_id: u32,
    value_millionths: i64,
    limit_millionths: i64,
    acknowledged: bool,
    suppressed: bool,
    timestamp_us: u64,
    operator_note: String,
}

/// Outage management record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OutageRecord {
    outage_id: u64,
    outage_type: OutageType,
    equipment_id: u32,
    customers_affected: u32,
    start_timestamp_us: u64,
    restore_timestamp_us: Option<u64>,
    cause_code: u16,
    crew_assigned: String,
    saidi_minutes: u32,
    saifi_events: u32,
}

/// Vegetation management zone around transmission lines.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VegetationZone {
    zone_id: u32,
    line_id: u32,
    span_start: u32,
    span_end: u32,
    risk: VegetationRisk,
    tree_height_cm: u16,
    clearance_cm: u16,
    last_trim_day: u32,
    next_trim_day: u32,
    species: String,
}

/// Power quality snapshot (THD, sags, swells).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PowerQuality {
    meter_id: u32,
    timestamp_us: u64,
    thd_voltage_hundredths: u16,
    thd_current_hundredths: u16,
    sag_count: u16,
    swell_count: u16,
    interruption_count: u16,
    min_voltage_pu_millionths: u32,
    max_voltage_pu_millionths: u32,
    harmonics: Vec<u32>, // magnitude for harmonics 2..N
}

/// Distributed energy resource (DER) status.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DerStatus {
    der_id: u32,
    der_type: DerType,
    capacity_kw_milliwatts: u64,
    output_kw_milliwatts: i64,
    reactive_kvar_milliwatts: i64,
    state_of_charge_hundredths: Option<u16>,
    voltage_pu_millionths: u32,
    online: bool,
    curtailed: bool,
    timestamp_us: u64,
}

/// Transmission congestion pricing record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CongestionPrice {
    node_id: u32,
    interval_start_us: u64,
    lmp_cents_per_mwh: i64,
    energy_component_cents: i64,
    congestion_component_cents: i64,
    loss_component_cents: i64,
    zone: CongestionZone,
    binding_constraints: Vec<String>,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn make_bus_voltage(idx: u32) -> BusVoltage {
    BusVoltage {
        substation_id: 100 + idx,
        bus_number: (idx % 8) as u16 + 1,
        voltage_level: match idx % 5 {
            0 => VoltageLevel::Kv345,
            1 => VoltageLevel::Kv230,
            2 => VoltageLevel::Kv138,
            3 => VoltageLevel::Kv69,
            _ => VoltageLevel::Kv13,
        },
        phase: match idx % 3 {
            0 => PhaseId::A,
            1 => PhaseId::B,
            _ => PhaseId::C,
        },
        voltage_pu_millionths: 1_000_000 + (idx % 50) * 1_000,
        angle_microdeg: -((idx as i32) * 120_000),
        timestamp_us: 1_700_000_000_000_000 + u64::from(idx) * 16_667,
    }
}

fn make_transformer_tap(idx: u32) -> TransformerTap {
    TransformerTap {
        transformer_id: 5000 + idx,
        winding: (idx % 3) as u8 + 1,
        tap_position: (idx % 33) as i16 - 16,
        tap_min: -16,
        tap_max: 16,
        voltage_ratio_pu_millionths: 1_000_000 + (idx % 33) * 625,
        auto_mode: idx % 2 == 0,
        last_change_us: 1_700_000_000_000_000 + u64::from(idx) * 60_000_000,
    }
}

fn make_circuit_breaker(idx: u32) -> CircuitBreaker {
    CircuitBreaker {
        breaker_id: 8000 + idx,
        substation_id: 100 + idx / 4,
        bay_name: format!("Bay-{:03}", idx),
        state: match idx % 5 {
            0 => BreakerState::Closed,
            1 => BreakerState::Open,
            2 => BreakerState::Closed,
            3 => BreakerState::Tripped,
            _ => BreakerState::Unknown,
        },
        rated_kv: match idx % 3 {
            0 => 345,
            1 => 230,
            _ => 138,
        },
        rated_ka_interrupting: 40 + idx % 20,
        trip_count: idx * 7,
        last_maintenance_day: 19700 + idx,
        sf6_pressure_kpa: 600 + (idx % 100) as u16,
    }
}

fn make_load_flow_result(idx: u32) -> LoadFlowResult {
    LoadFlowResult {
        bus_id: idx,
        voltage_pu_millionths: 990_000 + idx * 500,
        angle_microdeg: -(idx as i32) * 3_500,
        p_mw_milliwatts: (idx as i64) * 150_000 - 50_000_000,
        q_mvar_milliwatts: (idx as i64) * 30_000 - 10_000_000,
        converged: true,
        iteration_count: 5 + (idx % 10) as u16,
    }
}

fn make_fault_current(idx: u32) -> FaultCurrent {
    FaultCurrent {
        fault_id: 3000 + idx,
        bus_id: idx,
        fault_type: (idx % 4) as u8,
        i_sym_ka_millionths: 25_000_000 + u64::from(idx) * 500_000,
        i_asym_ka_millionths: 32_000_000 + u64::from(idx) * 650_000,
        x_r_ratio_thousandths: 8_000 + idx * 200,
        breaker_duty_percent: 70 + (idx % 30) as u16,
        contributing_sources: (0..3).map(|s| 100 + idx * 10 + s).collect(),
    }
}

fn make_relay_settings(idx: u32) -> RelaySettings {
    RelaySettings {
        relay_id: 2000 + idx,
        function: match idx % 6 {
            0 => RelayFunction::Overcurrent,
            1 => RelayFunction::DistanceZone1,
            2 => RelayFunction::DistanceZone2,
            3 => RelayFunction::Differential,
            4 => RelayFunction::UnderFrequency,
            _ => RelayFunction::BreakerFailure,
        },
        pickup_milliamps: 500 + idx * 100,
        time_dial_thousandths: 1_500 + idx * 50,
        curve_type: (idx % 5) as u8,
        instantaneous_milliamps: 5_000 + idx * 500,
        ct_ratio_primary: 600 + idx * 200,
        ct_ratio_secondary: 5,
        pt_ratio_primary: 345_000,
        pt_ratio_secondary: 120,
        enabled: idx % 7 != 0,
    }
}

fn make_agc_signal(idx: u32) -> AgcSignal {
    AgcSignal {
        area_id: idx,
        ace_mw_milliwatts: (idx as i64) * 100_000 - 5_000_000,
        frequency_bias_mw_hz: -840_000_000 + (idx as i64) * 10_000_000,
        scheduled_interchange_mw: 500_000_000 + (idx as i64) * 50_000_000,
        actual_interchange_mw: 498_000_000 + (idx as i64) * 50_500_000,
        frequency_deviation_micro_hz: -(idx as i32) * 1_000 + 500,
        timestamp_us: 1_700_000_000_000_000 + u64::from(idx) * 4_000_000,
        units_on_regulation: (0..4).map(|u| 9000 + idx * 10 + u).collect(),
    }
}

fn make_dsm_event(idx: u64) -> DsmEvent {
    DsmEvent {
        event_id: 40_000 + idx,
        program_name: format!("DR-Program-{}", idx % 5),
        reduction_kw_milliwatts: (idx as i64 + 1) * 2_500_000,
        participants: (idx as u32 + 1) * 120,
        start_timestamp_us: 1_700_000_000_000_000 + idx * 3_600_000_000,
        duration_minutes: 60 + (idx as u32 % 4) * 30,
        incentive_cents_per_kwh: 15 + (idx % 10) as u16,
        region_id: (idx % 8) as u16,
    }
}

fn make_pmu_measurement(idx: u32) -> PmuMeasurement {
    PmuMeasurement {
        pmu_id: 600 + idx / 3,
        timestamp_us: 1_700_000_000_000_000 + u64::from(idx) * 16_667,
        voltage_magnitude_pu_millionths: 1_002_000 + idx * 100,
        voltage_angle_microdeg: -(idx as i32) * 2_000,
        current_magnitude_milliamps: 300_000 + idx * 1_000,
        current_angle_microdeg: -(idx as i32) * 2_500 - 30_000_000,
        frequency_micro_hz: 60_000_000 + (idx % 50) as u64 * 100,
        rocof_mhz_per_sec: (idx as i32 % 21) - 10,
        phase: match idx % 3 {
            0 => PhaseId::A,
            1 => PhaseId::B,
            _ => PhaseId::C,
        },
        data_quality: 0,
    }
}

fn make_scada_alarm(idx: u64) -> ScadaAlarm {
    ScadaAlarm {
        alarm_id: 90_000 + idx,
        severity: match idx % 6 {
            0 => AlarmSeverity::Information,
            1 => AlarmSeverity::Warning,
            2 => AlarmSeverity::Minor,
            3 => AlarmSeverity::Major,
            4 => AlarmSeverity::Critical,
            _ => AlarmSeverity::Emergency,
        },
        point_name: format!("SUB{:03}.BUS{}.V", 100 + idx / 3, idx % 3),
        substation_id: (100 + idx / 3) as u32,
        value_millionths: 1_060_000 + (idx as i64) * 1_000,
        limit_millionths: 1_050_000,
        acknowledged: idx % 4 == 0,
        suppressed: false,
        timestamp_us: 1_700_000_000_000_000 + idx * 1_000_000,
        operator_note: if idx % 3 == 0 {
            "Investigating".to_string()
        } else {
            String::new()
        },
    }
}

fn make_outage_record(idx: u64) -> OutageRecord {
    OutageRecord {
        outage_id: 70_000 + idx,
        outage_type: match idx % 4 {
            0 => OutageType::Planned,
            1 => OutageType::ForcedWeather,
            2 => OutageType::ForcedVegetation,
            _ => OutageType::ForcedEquipment,
        },
        equipment_id: (5000 + idx * 3) as u32,
        customers_affected: (idx as u32 + 1) * 42,
        start_timestamp_us: 1_700_000_000_000_000 + idx * 7_200_000_000,
        restore_timestamp_us: if idx % 3 == 0 {
            None
        } else {
            Some(1_700_000_000_000_000 + idx * 7_200_000_000 + 3_600_000_000)
        },
        cause_code: (idx % 20) as u16 + 100,
        crew_assigned: format!("Crew-{:02}", idx % 12),
        saidi_minutes: (idx as u32 + 1) * 15,
        saifi_events: 1,
    }
}

fn make_vegetation_zone(idx: u32) -> VegetationZone {
    VegetationZone {
        zone_id: 20_000 + idx,
        line_id: 1000 + idx / 5,
        span_start: idx * 300,
        span_end: idx * 300 + 300,
        risk: match idx % 4 {
            0 => VegetationRisk::Low,
            1 => VegetationRisk::Moderate,
            2 => VegetationRisk::High,
            _ => VegetationRisk::Imminent,
        },
        tree_height_cm: (800 + (idx % 20) * 50) as u16,
        clearance_cm: (500 - (idx % 10) * 30) as u16,
        last_trim_day: 19500 + idx,
        next_trim_day: 19500 + idx + 365,
        species: format!("Species-{}", idx % 8),
    }
}

fn make_power_quality(idx: u32) -> PowerQuality {
    PowerQuality {
        meter_id: 15_000 + idx,
        timestamp_us: 1_700_000_000_000_000 + u64::from(idx) * 900_000_000,
        thd_voltage_hundredths: 250 + (idx % 100) as u16,
        thd_current_hundredths: 800 + (idx % 200) as u16,
        sag_count: (idx % 5) as u16,
        swell_count: (idx % 3) as u16,
        interruption_count: (idx % 2) as u16,
        min_voltage_pu_millionths: 920_000 + idx * 100,
        max_voltage_pu_millionths: 1_050_000 + idx * 100,
        harmonics: (2u32..14).map(|h| 10_000 / (h * h)).collect(),
    }
}

fn make_der_status(idx: u32) -> DerStatus {
    DerStatus {
        der_id: 30_000 + idx,
        der_type: match idx % 7 {
            0 => DerType::SolarPv,
            1 => DerType::WindTurbine,
            2 => DerType::BatteryStorage,
            3 => DerType::MicroTurbine,
            4 => DerType::FuelCell,
            5 => DerType::CombinedHeatPower,
            _ => DerType::ElectricVehicleCharger,
        },
        capacity_kw_milliwatts: (idx as u64 + 1) * 500_000,
        output_kw_milliwatts: (idx as i64 + 1) * 350_000,
        reactive_kvar_milliwatts: (idx as i64) * 20_000 - 100_000,
        state_of_charge_hundredths: if idx % 7 == 2 {
            Some(7500 + (idx % 25) as u16 * 100)
        } else {
            None
        },
        voltage_pu_millionths: 1_000_000 + idx * 500,
        online: idx % 6 != 0,
        curtailed: idx % 11 == 0,
        timestamp_us: 1_700_000_000_000_000 + u64::from(idx) * 300_000_000,
    }
}

fn make_congestion_price(idx: u32) -> CongestionPrice {
    CongestionPrice {
        node_id: 50_000 + idx,
        interval_start_us: 1_700_000_000_000_000 + u64::from(idx) * 300_000_000,
        lmp_cents_per_mwh: 3_500 + (idx as i64) * 150,
        energy_component_cents: 2_800 + (idx as i64) * 50,
        congestion_component_cents: 500 + (idx as i64) * 80,
        loss_component_cents: 200 + (idx as i64) * 20,
        zone: match idx % 5 {
            0 => CongestionZone::Unconstrained,
            1 => CongestionZone::Marginal,
            2 => CongestionZone::Constrained,
            3 => CongestionZone::Binding,
            _ => CongestionZone::Emergency,
        },
        binding_constraints: (0..(idx % 3))
            .map(|c| format!("Constraint-{}-{}", idx, c))
            .collect(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. Round-trip for a single bus voltage measurement.
#[test]
fn test_zstd_bus_voltage_roundtrip() {
    let bv = make_bus_voltage(0);
    let encoded = encode_to_vec(&bv).expect("encode BusVoltage failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (BusVoltage, usize) =
        decode_from_slice(&decompressed).expect("decode BusVoltage failed");
    assert_eq!(bv, decoded);
}

/// 2. Round-trip for a vector of transformer tap positions.
#[test]
fn test_zstd_transformer_taps_roundtrip() {
    let taps: Vec<TransformerTap> = (0..20).map(make_transformer_tap).collect();
    let encoded = encode_to_vec(&taps).expect("encode Vec<TransformerTap> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<TransformerTap>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<TransformerTap> failed");
    assert_eq!(taps, decoded);
}

/// 3. Round-trip for circuit breakers with size comparison.
#[test]
fn test_zstd_circuit_breakers_compression_ratio() {
    let breakers: Vec<CircuitBreaker> = (0..50).map(make_circuit_breaker).collect();
    let encoded = encode_to_vec(&breakers).expect("encode Vec<CircuitBreaker> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed {} should be smaller than encoded {}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<CircuitBreaker>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<CircuitBreaker> failed");
    assert_eq!(breakers, decoded);
}

/// 4. Round-trip for load flow analysis results.
#[test]
fn test_zstd_load_flow_results_roundtrip() {
    let results: Vec<LoadFlowResult> = (0..100).map(make_load_flow_result).collect();
    let encoded = encode_to_vec(&results).expect("encode Vec<LoadFlowResult> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<LoadFlowResult>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<LoadFlowResult> failed");
    assert_eq!(results, decoded);
}

/// 5. Round-trip for fault current calculations.
#[test]
fn test_zstd_fault_currents_roundtrip() {
    let faults: Vec<FaultCurrent> = (0..30).map(make_fault_current).collect();
    let encoded = encode_to_vec(&faults).expect("encode Vec<FaultCurrent> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<FaultCurrent>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<FaultCurrent> failed");
    assert_eq!(faults, decoded);
}

/// 6. Round-trip for protection relay settings.
#[test]
fn test_zstd_relay_settings_roundtrip() {
    let relays: Vec<RelaySettings> = (0..24).map(make_relay_settings).collect();
    let encoded = encode_to_vec(&relays).expect("encode Vec<RelaySettings> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<RelaySettings>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<RelaySettings> failed");
    assert_eq!(relays, decoded);
}

/// 7. Round-trip for AGC signals with size comparison.
#[test]
fn test_zstd_agc_signals_compression_ratio() {
    let signals: Vec<AgcSignal> = (0..40).map(make_agc_signal).collect();
    let encoded = encode_to_vec(&signals).expect("encode Vec<AgcSignal> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed {} should be smaller than encoded {}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<AgcSignal>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<AgcSignal> failed");
    assert_eq!(signals, decoded);
}

/// 8. Round-trip for demand-side management events.
#[test]
fn test_zstd_dsm_events_roundtrip() {
    let events: Vec<DsmEvent> = (0..15).map(make_dsm_event).collect();
    let encoded = encode_to_vec(&events).expect("encode Vec<DsmEvent> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<DsmEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<DsmEvent> failed");
    assert_eq!(events, decoded);
}

/// 9. Round-trip for PMU synchrophasor measurements.
#[test]
fn test_zstd_pmu_measurements_roundtrip() {
    let measurements: Vec<PmuMeasurement> = (0..60).map(make_pmu_measurement).collect();
    let encoded = encode_to_vec(&measurements).expect("encode Vec<PmuMeasurement> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<PmuMeasurement>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<PmuMeasurement> failed");
    assert_eq!(measurements, decoded);
}

/// 10. Round-trip for SCADA alarm hierarchy entries.
#[test]
fn test_zstd_scada_alarms_roundtrip() {
    let alarms: Vec<ScadaAlarm> = (0..25).map(make_scada_alarm).collect();
    let encoded = encode_to_vec(&alarms).expect("encode Vec<ScadaAlarm> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<ScadaAlarm>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<ScadaAlarm> failed");
    assert_eq!(alarms, decoded);
}

/// 11. Round-trip for outage management records.
#[test]
fn test_zstd_outage_records_roundtrip() {
    let records: Vec<OutageRecord> = (0..18).map(make_outage_record).collect();
    let encoded = encode_to_vec(&records).expect("encode Vec<OutageRecord> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<OutageRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<OutageRecord> failed");
    assert_eq!(records, decoded);
}

/// 12. Round-trip for vegetation management zones with size comparison.
#[test]
fn test_zstd_vegetation_zones_compression_ratio() {
    let zones: Vec<VegetationZone> = (0..80).map(make_vegetation_zone).collect();
    let encoded = encode_to_vec(&zones).expect("encode Vec<VegetationZone> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed {} should be smaller than encoded {}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<VegetationZone>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<VegetationZone> failed");
    assert_eq!(zones, decoded);
}

/// 13. Round-trip for power quality snapshots.
#[test]
fn test_zstd_power_quality_roundtrip() {
    let snapshots: Vec<PowerQuality> = (0..30).map(make_power_quality).collect();
    let encoded = encode_to_vec(&snapshots).expect("encode Vec<PowerQuality> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<PowerQuality>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<PowerQuality> failed");
    assert_eq!(snapshots, decoded);
}

/// 14. Round-trip for distributed energy resource statuses.
#[test]
fn test_zstd_der_status_roundtrip() {
    let statuses: Vec<DerStatus> = (0..35).map(make_der_status).collect();
    let encoded = encode_to_vec(&statuses).expect("encode Vec<DerStatus> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<DerStatus>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<DerStatus> failed");
    assert_eq!(statuses, decoded);
}

/// 15. Round-trip for transmission congestion pricing records.
#[test]
fn test_zstd_congestion_pricing_roundtrip() {
    let prices: Vec<CongestionPrice> = (0..48).map(make_congestion_price).collect();
    let encoded = encode_to_vec(&prices).expect("encode Vec<CongestionPrice> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<CongestionPrice>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<CongestionPrice> failed");
    assert_eq!(prices, decoded);
}

/// 16. Large-scale PMU synchrophasor batch with compression ratio check.
#[test]
fn test_zstd_pmu_large_batch_compression() {
    let measurements: Vec<PmuMeasurement> = (0..1000).map(make_pmu_measurement).collect();
    let encoded = encode_to_vec(&measurements).expect("encode large PMU batch failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed {} should be smaller than encoded {}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<PmuMeasurement>, usize) =
        decode_from_slice(&decompressed).expect("decode large PMU batch failed");
    assert_eq!(measurements, decoded);
}

/// 17. Mixed grid snapshot containing multiple domain types.
#[test]
fn test_zstd_mixed_grid_snapshot_roundtrip() {
    let snapshot = (
        (0..5).map(make_bus_voltage).collect::<Vec<_>>(),
        (0..3).map(make_transformer_tap).collect::<Vec<_>>(),
        (0..4).map(make_circuit_breaker).collect::<Vec<_>>(),
        (0..5).map(make_load_flow_result).collect::<Vec<_>>(),
    );
    let encoded = encode_to_vec(&snapshot).expect("encode grid snapshot failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (
        (
            Vec<BusVoltage>,
            Vec<TransformerTap>,
            Vec<CircuitBreaker>,
            Vec<LoadFlowResult>,
        ),
        usize,
    ) = decode_from_slice(&decompressed).expect("decode grid snapshot failed");
    assert_eq!(snapshot, decoded);
}

/// 18. Outage records with Optional restore timestamps round-trip.
#[test]
fn test_zstd_outage_optional_restore_roundtrip() {
    let mut records: Vec<OutageRecord> = (0..10).map(make_outage_record).collect();
    // Ensure mixture of Some and None restore timestamps
    records[0].restore_timestamp_us = None;
    records[1].restore_timestamp_us = Some(1_700_000_010_000_000);
    records[2].restore_timestamp_us = None;
    let encoded = encode_to_vec(&records).expect("encode outage optional failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<OutageRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode outage optional failed");
    assert_eq!(records, decoded);
}

/// 19. SCADA alarm hierarchy with all severity levels.
#[test]
fn test_zstd_scada_alarm_all_severities() {
    let alarms: Vec<ScadaAlarm> = (0..6).map(make_scada_alarm).collect();
    // Verify we have all 6 severity variants
    let severities: Vec<&AlarmSeverity> = alarms.iter().map(|a| &a.severity).collect();
    assert_eq!(severities.len(), 6, "should have all severity levels");
    let encoded = encode_to_vec(&alarms).expect("encode severity alarms failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<ScadaAlarm>, usize) =
        decode_from_slice(&decompressed).expect("decode severity alarms failed");
    assert_eq!(alarms, decoded);
}

/// 20. DER fleet with mixed types and battery SOC round-trip.
#[test]
fn test_zstd_der_fleet_mixed_types() {
    let fleet: Vec<DerStatus> = (0..28).map(make_der_status).collect();
    // Verify some have SOC, others do not
    let with_soc = fleet
        .iter()
        .filter(|d| d.state_of_charge_hundredths.is_some())
        .count();
    let without_soc = fleet
        .iter()
        .filter(|d| d.state_of_charge_hundredths.is_none())
        .count();
    assert!(with_soc > 0, "should have DERs with SOC");
    assert!(without_soc > 0, "should have DERs without SOC");
    let encoded = encode_to_vec(&fleet).expect("encode DER fleet failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<DerStatus>, usize) =
        decode_from_slice(&decompressed).expect("decode DER fleet failed");
    assert_eq!(fleet, decoded);
}

/// 21. Congestion prices with variable binding constraint lists.
#[test]
fn test_zstd_congestion_variable_constraints() {
    let prices: Vec<CongestionPrice> = (0..12).map(make_congestion_price).collect();
    // Verify we get variable-length constraint vectors (0, 1, 2)
    let lengths: Vec<usize> = prices.iter().map(|p| p.binding_constraints.len()).collect();
    assert!(
        lengths.contains(&0) && lengths.contains(&1) && lengths.contains(&2),
        "should have variable constraint list lengths, got {:?}",
        lengths
    );
    let encoded = encode_to_vec(&prices).expect("encode congestion constraints failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<CongestionPrice>, usize) =
        decode_from_slice(&decompressed).expect("decode congestion constraints failed");
    assert_eq!(prices, decoded);
}

/// 22. Full energy distribution composite with compression size check.
#[test]
fn test_zstd_full_energy_distribution_composite() {
    let composite = (
        (0..20).map(make_bus_voltage).collect::<Vec<_>>(),
        (0..10).map(make_relay_settings).collect::<Vec<_>>(),
        (0..8u64).map(make_dsm_event).collect::<Vec<_>>(),
        (0..12).map(make_power_quality).collect::<Vec<_>>(),
        (0..15).map(make_congestion_price).collect::<Vec<_>>(),
    );
    let encoded = encode_to_vec(&composite).expect("encode composite failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed {} should be smaller than encoded {}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (
        (
            Vec<BusVoltage>,
            Vec<RelaySettings>,
            Vec<DsmEvent>,
            Vec<PowerQuality>,
            Vec<CongestionPrice>,
        ),
        usize,
    ) = decode_from_slice(&decompressed).expect("decode composite failed");
    assert_eq!(composite, decoded);
}
