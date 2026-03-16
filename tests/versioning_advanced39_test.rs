#![cfg(feature = "versioning")]
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
use oxicode::versioning::Version;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

// ── Domain types: Renewable Energy Grid Management ──────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InverterStatus {
    Online,
    Standby,
    Fault,
    Curtailed,
    Disconnected,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TurbineState {
    Generating,
    Idling,
    CutOut,
    Maintenance,
    EmergencyStop,
    Yawing,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BatteryChemistry {
    LithiumIronPhosphate,
    NickelManganeseCoablt,
    SodiumIon,
    VanadiumRedoxFlow,
    ZincBromine,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DemandResponseType {
    LoadShedding,
    PeakShaving,
    FrequencyResponse,
    VoltageSupport,
    EmergencyCurtailment,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MeterPhase {
    SinglePhase,
    ThreePhaseWye,
    ThreePhaseDelta,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CurtailmentReason {
    TransmissionCongestion,
    NegativePricing,
    OverGeneration,
    GridStability,
    EnvironmentalConstraint,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IslandingMode {
    GridConnected,
    Islanded,
    Transitioning,
    BlackStart,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ElectrolyzerType {
    Pem,
    Alkaline,
    SolidOxide,
}

// ── Structs ─────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SolarFarmOutput {
    farm_id: u32,
    timestamp_ns: u64,
    irradiance_w_per_m2: u16,
    panel_efficiency_pct_x100: u16,
    inverter_status: InverterStatus,
    dc_voltage_mv: u32,
    dc_current_ma: u32,
    ac_power_w: u32,
    ambient_temp_c_x10: i16,
    module_temp_c_x10: i16,
    tracking_azimuth_mrad: u16,
    tracking_elevation_mrad: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WindTurbineTelemetry {
    turbine_id: u32,
    timestamp_ns: u64,
    rotor_rpm_x10: u16,
    pitch_angle_mrad: i16,
    yaw_angle_mrad: u16,
    wind_speed_mm_per_s: u32,
    wind_direction_mrad: u16,
    power_output_w: u32,
    generator_temp_c_x10: i16,
    gearbox_oil_temp_c_x10: i16,
    vibration_mg: u16,
    state: TurbineState,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatteryEnergyStorage {
    unit_id: u32,
    timestamp_ns: u64,
    chemistry: BatteryChemistry,
    soc_pct_x100: u16,
    soh_pct_x100: u16,
    charge_rate_w: u32,
    discharge_rate_w: u32,
    cycle_count: u32,
    cell_voltage_mv_min: u16,
    cell_voltage_mv_max: u16,
    pack_temp_c_x10: i16,
    capacity_wh: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GridFrequencyRegulation {
    region_id: u32,
    timestamp_ns: u64,
    frequency_mhz: u32,
    nominal_frequency_mhz: u32,
    deviation_mhz: i32,
    regulation_up_mw: u32,
    regulation_down_mw: u32,
    inertia_constant_ms: u16,
    rocof_mhz_per_s: i32,
    agc_setpoint_mw: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DemandResponseEvent {
    event_id: u64,
    timestamp_ns: u64,
    response_type: DemandResponseType,
    target_reduction_kw: u32,
    actual_reduction_kw: u32,
    duration_s: u32,
    num_participants: u16,
    incentive_cents_per_kwh: u16,
    compliance_pct_x10: u16,
    region_code: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartMeterReading {
    meter_id: u64,
    timestamp_ns: u64,
    phase: MeterPhase,
    active_power_w: i32,
    reactive_power_var: i32,
    voltage_v_x10: u16,
    current_ma: u32,
    power_factor_x1000: u16,
    cumulative_kwh_x100: u64,
    interval_minutes: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PowerPurchaseAgreement {
    ppa_id: u64,
    generator_id: u32,
    buyer_id: u32,
    strike_price_cents_per_mwh: u32,
    contract_capacity_kw: u32,
    start_date_days_since_epoch: u32,
    end_date_days_since_epoch: u32,
    escalation_rate_bps: u16,
    curtailment_cap_hours: u16,
    is_physical_delivery: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RenewableEnergyCertificate {
    rec_id: u64,
    vintage_year: u16,
    generation_mwh_x100: u32,
    fuel_type_code: u8,
    facility_id: u32,
    tracking_system_id: u16,
    issued_date_days: u32,
    retired_date_days: u32,
    is_retired: bool,
    beneficiary_id: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CurtailmentEvent {
    event_id: u64,
    timestamp_ns: u64,
    reason: CurtailmentReason,
    curtailed_mw_x10: u32,
    duration_s: u32,
    affected_generators: u16,
    lost_energy_mwh_x100: u32,
    compensation_cents: u64,
    was_economic: bool,
    region_code: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MicrogridIslandingEvent {
    microgrid_id: u32,
    timestamp_ns: u64,
    mode: IslandingMode,
    local_generation_kw: u32,
    local_load_kw: u32,
    battery_soc_pct_x100: u16,
    frequency_mhz: u32,
    voltage_v_x10: u16,
    transition_time_ms: u32,
    critical_loads_served: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HydrogenElectrolysis {
    unit_id: u32,
    timestamp_ns: u64,
    electrolyzer_type: ElectrolyzerType,
    power_input_kw: u32,
    hydrogen_rate_g_per_s_x100: u32,
    efficiency_pct_x100: u16,
    stack_voltage_mv: u32,
    stack_current_ma: u32,
    water_flow_ml_per_min: u32,
    outlet_pressure_kpa: u32,
    stack_temp_c_x10: i16,
    runtime_hours: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CarbonOffsetRecord {
    record_id: u64,
    project_id: u32,
    vintage_year: u16,
    offset_tonnes_co2e_x100: u32,
    baseline_emission_factor_x1000: u32,
    actual_emission_factor_x1000: u32,
    verification_body_id: u16,
    registry_serial_start: u64,
    registry_serial_end: u64,
    is_verified: bool,
    is_retired: bool,
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn test_solar_farm_output_roundtrip() {
    let output = SolarFarmOutput {
        farm_id: 101,
        timestamp_ns: 1_710_000_000_000_000_000,
        irradiance_w_per_m2: 950,
        panel_efficiency_pct_x100: 2150,
        inverter_status: InverterStatus::Online,
        dc_voltage_mv: 600_000,
        dc_current_ma: 85_000,
        ac_power_w: 48_000,
        ambient_temp_c_x10: 325,
        module_temp_c_x10: 580,
        tracking_azimuth_mrad: 3142,
        tracking_elevation_mrad: 900,
    };
    let bytes = encode_to_vec(&output).expect("encode SolarFarmOutput failed");
    let (decoded, consumed): (SolarFarmOutput, usize) =
        decode_from_slice(&bytes).expect("decode SolarFarmOutput failed");
    assert_eq!(output, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_solar_farm_versioned_v1_0_0() {
    let output = SolarFarmOutput {
        farm_id: 202,
        timestamp_ns: 1_720_000_000_000_000_000,
        irradiance_w_per_m2: 1050,
        panel_efficiency_pct_x100: 2230,
        inverter_status: InverterStatus::Curtailed,
        dc_voltage_mv: 580_000,
        dc_current_ma: 72_000,
        ac_power_w: 39_500,
        ambient_temp_c_x10: 410,
        module_temp_c_x10: 620,
        tracking_azimuth_mrad: 2800,
        tracking_elevation_mrad: 1100,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&output, ver).expect("versioned encode SolarFarmOutput");
    let (decoded, dec_ver, _consumed): (SolarFarmOutput, Version, usize) =
        decode_versioned_value(&bytes).expect("versioned decode SolarFarmOutput");
    assert_eq!(output, decoded);
    assert_eq!(dec_ver, ver);
}

#[test]
fn test_wind_turbine_telemetry_roundtrip() {
    let telemetry = WindTurbineTelemetry {
        turbine_id: 42,
        timestamp_ns: 1_715_000_000_000_000_000,
        rotor_rpm_x10: 145,
        pitch_angle_mrad: 52,
        yaw_angle_mrad: 4200,
        wind_speed_mm_per_s: 12_500,
        wind_direction_mrad: 2618,
        power_output_w: 3_000_000,
        generator_temp_c_x10: 850,
        gearbox_oil_temp_c_x10: 720,
        vibration_mg: 340,
        state: TurbineState::Generating,
    };
    let bytes = encode_to_vec(&telemetry).expect("encode WindTurbineTelemetry failed");
    let (decoded, consumed): (WindTurbineTelemetry, usize) =
        decode_from_slice(&bytes).expect("decode WindTurbineTelemetry failed");
    assert_eq!(telemetry, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_wind_turbine_versioned_upgrade_scenario() {
    let telemetry = WindTurbineTelemetry {
        turbine_id: 77,
        timestamp_ns: 1_718_000_000_000_000_000,
        rotor_rpm_x10: 120,
        pitch_angle_mrad: -30,
        yaw_angle_mrad: 5100,
        wind_speed_mm_per_s: 8_200,
        wind_direction_mrad: 1570,
        power_output_w: 1_800_000,
        generator_temp_c_x10: 680,
        gearbox_oil_temp_c_x10: 610,
        vibration_mg: 210,
        state: TurbineState::Yawing,
    };

    let v1 = Version::new(1, 0, 0);
    let bytes_v1 = encode_versioned_value(&telemetry, v1).expect("v1 encode turbine");
    let (dec_v1, ver_v1, _): (WindTurbineTelemetry, Version, usize) =
        decode_versioned_value(&bytes_v1).expect("v1 decode turbine");
    assert_eq!(telemetry, dec_v1);
    assert_eq!(ver_v1, v1);

    let v2 = Version::new(2, 0, 0);
    let bytes_v2 = encode_versioned_value(&telemetry, v2).expect("v2 encode turbine");
    let (dec_v2, ver_v2, _): (WindTurbineTelemetry, Version, usize) =
        decode_versioned_value(&bytes_v2).expect("v2 decode turbine");
    assert_eq!(telemetry, dec_v2);
    assert_eq!(ver_v2, v2);
    assert!(ver_v2.major > ver_v1.major);
}

#[test]
fn test_battery_energy_storage_roundtrip() {
    let bess = BatteryEnergyStorage {
        unit_id: 5,
        timestamp_ns: 1_712_000_000_000_000_000,
        chemistry: BatteryChemistry::LithiumIronPhosphate,
        soc_pct_x100: 7500,
        soh_pct_x100: 9600,
        charge_rate_w: 250_000,
        discharge_rate_w: 0,
        cycle_count: 1240,
        cell_voltage_mv_min: 3200,
        cell_voltage_mv_max: 3380,
        pack_temp_c_x10: 280,
        capacity_wh: 500_000,
    };
    let bytes = encode_to_vec(&bess).expect("encode BESS failed");
    let (decoded, consumed): (BatteryEnergyStorage, usize) =
        decode_from_slice(&bytes).expect("decode BESS failed");
    assert_eq!(bess, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_battery_versioned_multiple_chemistries() {
    let chemistries = vec![
        BatteryChemistry::LithiumIronPhosphate,
        BatteryChemistry::NickelManganeseCoablt,
        BatteryChemistry::SodiumIon,
        BatteryChemistry::VanadiumRedoxFlow,
        BatteryChemistry::ZincBromine,
    ];
    let ver = Version::new(1, 2, 0);
    for (idx, chem) in chemistries.into_iter().enumerate() {
        let bess = BatteryEnergyStorage {
            unit_id: idx as u32 + 1,
            timestamp_ns: 1_713_000_000_000_000_000,
            chemistry: chem,
            soc_pct_x100: 5000 + (idx as u16) * 1000,
            soh_pct_x100: 9800,
            charge_rate_w: 0,
            discharge_rate_w: 100_000,
            cycle_count: 500 + idx as u32 * 100,
            cell_voltage_mv_min: 3100,
            cell_voltage_mv_max: 3400,
            pack_temp_c_x10: 250,
            capacity_wh: 200_000,
        };
        let bytes = encode_versioned_value(&bess, ver).expect("encode versioned BESS");
        let (decoded, dec_ver, _): (BatteryEnergyStorage, Version, usize) =
            decode_versioned_value(&bytes).expect("decode versioned BESS");
        assert_eq!(bess, decoded);
        assert_eq!(dec_ver, ver);
    }
}

#[test]
fn test_grid_frequency_regulation_roundtrip() {
    let reg = GridFrequencyRegulation {
        region_id: 10,
        timestamp_ns: 1_714_000_000_000_000_000,
        frequency_mhz: 60_000,
        nominal_frequency_mhz: 60_000,
        deviation_mhz: -15,
        regulation_up_mw: 500,
        regulation_down_mw: 350,
        inertia_constant_ms: 4500,
        rocof_mhz_per_s: -80,
        agc_setpoint_mw: 120,
    };
    let bytes = encode_to_vec(&reg).expect("encode GridFrequencyRegulation failed");
    let (decoded, consumed): (GridFrequencyRegulation, usize) =
        decode_from_slice(&bytes).expect("decode GridFrequencyRegulation failed");
    assert_eq!(reg, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_grid_frequency_versioned_deviation_scenarios() {
    let deviations: Vec<i32> = vec![-500, -100, 0, 50, 300];
    let ver = Version::new(3, 1, 0);
    for dev in deviations {
        let reg = GridFrequencyRegulation {
            region_id: 20,
            timestamp_ns: 1_716_000_000_000_000_000,
            frequency_mhz: (60_000_i32 + dev) as u32,
            nominal_frequency_mhz: 60_000,
            deviation_mhz: dev,
            regulation_up_mw: 800,
            regulation_down_mw: 600,
            inertia_constant_ms: 3800,
            rocof_mhz_per_s: dev * 2,
            agc_setpoint_mw: -200,
        };
        let bytes = encode_versioned_value(&reg, ver).expect("versioned encode grid freq");
        let (decoded, dec_ver, _): (GridFrequencyRegulation, Version, usize) =
            decode_versioned_value(&bytes).expect("versioned decode grid freq");
        assert_eq!(reg, decoded);
        assert_eq!(dec_ver, ver);
    }
}

#[test]
fn test_demand_response_event_roundtrip() {
    let event = DemandResponseEvent {
        event_id: 900_001,
        timestamp_ns: 1_719_000_000_000_000_000,
        response_type: DemandResponseType::PeakShaving,
        target_reduction_kw: 5000,
        actual_reduction_kw: 4800,
        duration_s: 3600,
        num_participants: 1200,
        incentive_cents_per_kwh: 25,
        compliance_pct_x10: 960,
        region_code: 44,
    };
    let bytes = encode_to_vec(&event).expect("encode DemandResponseEvent failed");
    let (decoded, consumed): (DemandResponseEvent, usize) =
        decode_from_slice(&bytes).expect("decode DemandResponseEvent failed");
    assert_eq!(event, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_demand_response_versioned_all_types() {
    let types = vec![
        DemandResponseType::LoadShedding,
        DemandResponseType::PeakShaving,
        DemandResponseType::FrequencyResponse,
        DemandResponseType::VoltageSupport,
        DemandResponseType::EmergencyCurtailment,
    ];
    let ver = Version::new(1, 5, 3);
    for (i, dr_type) in types.into_iter().enumerate() {
        let event = DemandResponseEvent {
            event_id: 1000 + i as u64,
            timestamp_ns: 1_720_000_000_000_000_000,
            response_type: dr_type,
            target_reduction_kw: 3000 + i as u32 * 500,
            actual_reduction_kw: 2800 + i as u32 * 400,
            duration_s: 1800,
            num_participants: 800,
            incentive_cents_per_kwh: 30,
            compliance_pct_x10: 950,
            region_code: 12,
        };
        let bytes = encode_versioned_value(&event, ver).expect("versioned encode DR event");
        let (decoded, dec_ver, _): (DemandResponseEvent, Version, usize) =
            decode_versioned_value(&bytes).expect("versioned decode DR event");
        assert_eq!(event, decoded);
        assert_eq!(dec_ver, ver);
    }
}

#[test]
fn test_smart_meter_reading_roundtrip() {
    let reading = SmartMeterReading {
        meter_id: 123_456_789,
        timestamp_ns: 1_717_000_000_000_000_000,
        phase: MeterPhase::ThreePhaseWye,
        active_power_w: 15_200,
        reactive_power_var: 3_400,
        voltage_v_x10: 2400,
        current_ma: 63_000,
        power_factor_x1000: 975,
        cumulative_kwh_x100: 1_250_000,
        interval_minutes: 15,
    };
    let bytes = encode_to_vec(&reading).expect("encode SmartMeterReading failed");
    let (decoded, consumed): (SmartMeterReading, usize) =
        decode_from_slice(&bytes).expect("decode SmartMeterReading failed");
    assert_eq!(reading, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_smart_meter_versioned_reverse_power_flow() {
    let reading = SmartMeterReading {
        meter_id: 987_654_321,
        timestamp_ns: 1_718_500_000_000_000_000,
        phase: MeterPhase::SinglePhase,
        active_power_w: -5_200,
        reactive_power_var: -800,
        voltage_v_x10: 2420,
        current_ma: 21_700,
        power_factor_x1000: 988,
        cumulative_kwh_x100: 450_000,
        interval_minutes: 5,
    };
    let ver = Version::new(2, 3, 1);
    let bytes = encode_versioned_value(&reading, ver).expect("versioned encode meter");
    let (decoded, dec_ver, _): (SmartMeterReading, Version, usize) =
        decode_versioned_value(&bytes).expect("versioned decode meter");
    assert_eq!(reading, decoded);
    assert_eq!(dec_ver, ver);
    assert!(
        decoded.active_power_w < 0,
        "should represent export/reverse flow"
    );
}

#[test]
fn test_power_purchase_agreement_roundtrip() {
    let ppa = PowerPurchaseAgreement {
        ppa_id: 50_001,
        generator_id: 300,
        buyer_id: 7000,
        strike_price_cents_per_mwh: 3500,
        contract_capacity_kw: 100_000,
        start_date_days_since_epoch: 19_723,
        end_date_days_since_epoch: 26_298,
        escalation_rate_bps: 200,
        curtailment_cap_hours: 500,
        is_physical_delivery: true,
    };
    let bytes = encode_to_vec(&ppa).expect("encode PPA failed");
    let (decoded, consumed): (PowerPurchaseAgreement, usize) =
        decode_from_slice(&bytes).expect("decode PPA failed");
    assert_eq!(ppa, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_ppa_versioned_financial_vs_physical() {
    let ver = Version::new(1, 0, 0);
    let physical_ppa = PowerPurchaseAgreement {
        ppa_id: 60_001,
        generator_id: 400,
        buyer_id: 8000,
        strike_price_cents_per_mwh: 2800,
        contract_capacity_kw: 50_000,
        start_date_days_since_epoch: 20_000,
        end_date_days_since_epoch: 23_650,
        escalation_rate_bps: 150,
        curtailment_cap_hours: 300,
        is_physical_delivery: true,
    };
    let financial_ppa = PowerPurchaseAgreement {
        ppa_id: 60_002,
        is_physical_delivery: false,
        ..physical_ppa.clone()
    };

    let bytes_phys = encode_versioned_value(&physical_ppa, ver).expect("encode physical PPA");
    let (dec_phys, ver_phys, _): (PowerPurchaseAgreement, Version, usize) =
        decode_versioned_value(&bytes_phys).expect("decode physical PPA");
    assert_eq!(physical_ppa, dec_phys);
    assert!(dec_phys.is_physical_delivery);
    assert_eq!(ver_phys, ver);

    let bytes_fin = encode_versioned_value(&financial_ppa, ver).expect("encode financial PPA");
    let (dec_fin, _, _): (PowerPurchaseAgreement, Version, usize) =
        decode_versioned_value(&bytes_fin).expect("decode financial PPA");
    assert_eq!(financial_ppa, dec_fin);
    assert!(!dec_fin.is_physical_delivery);
}

#[test]
fn test_renewable_energy_certificate_roundtrip() {
    let rec = RenewableEnergyCertificate {
        rec_id: 1_000_000_001,
        vintage_year: 2025,
        generation_mwh_x100: 150_000,
        fuel_type_code: 1,
        facility_id: 5500,
        tracking_system_id: 3,
        issued_date_days: 20_100,
        retired_date_days: 20_200,
        is_retired: true,
        beneficiary_id: 9001,
    };
    let bytes = encode_to_vec(&rec).expect("encode REC failed");
    let (decoded, consumed): (RenewableEnergyCertificate, usize) =
        decode_from_slice(&bytes).expect("decode REC failed");
    assert_eq!(rec, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_rec_versioned_retirement_lifecycle() {
    let ver_issue = Version::new(1, 0, 0);
    let ver_retire = Version::new(1, 1, 0);

    let issued_rec = RenewableEnergyCertificate {
        rec_id: 2_000_000_001,
        vintage_year: 2026,
        generation_mwh_x100: 200_000,
        fuel_type_code: 2,
        facility_id: 6600,
        tracking_system_id: 5,
        issued_date_days: 20_500,
        retired_date_days: 0,
        is_retired: false,
        beneficiary_id: 0,
    };
    let bytes_issued = encode_versioned_value(&issued_rec, ver_issue).expect("encode issued REC");
    let (dec_issued, dv_issue, _): (RenewableEnergyCertificate, Version, usize) =
        decode_versioned_value(&bytes_issued).expect("decode issued REC");
    assert_eq!(issued_rec, dec_issued);
    assert!(!dec_issued.is_retired);
    assert_eq!(dv_issue, ver_issue);

    let retired_rec = RenewableEnergyCertificate {
        retired_date_days: 20_600,
        is_retired: true,
        beneficiary_id: 9500,
        ..issued_rec.clone()
    };
    let bytes_retired =
        encode_versioned_value(&retired_rec, ver_retire).expect("encode retired REC");
    let (dec_retired, dv_retire, _): (RenewableEnergyCertificate, Version, usize) =
        decode_versioned_value(&bytes_retired).expect("decode retired REC");
    assert_eq!(retired_rec, dec_retired);
    assert!(dec_retired.is_retired);
    assert_eq!(dv_retire, ver_retire);
    assert!(dv_retire.minor > dv_issue.minor);
}

#[test]
fn test_curtailment_versioned_all_reasons() {
    let reasons = vec![
        CurtailmentReason::TransmissionCongestion,
        CurtailmentReason::NegativePricing,
        CurtailmentReason::OverGeneration,
        CurtailmentReason::GridStability,
        CurtailmentReason::EnvironmentalConstraint,
    ];
    let ver = Version::new(2, 0, 1);
    for (i, reason) in reasons.into_iter().enumerate() {
        let event = CurtailmentEvent {
            event_id: 80_000 + i as u64,
            timestamp_ns: 1_722_000_000_000_000_000,
            reason,
            curtailed_mw_x10: 800 + i as u32 * 200,
            duration_s: 3600,
            affected_generators: 5 + i as u16,
            lost_energy_mwh_x100: 100_000,
            compensation_cents: 5_000_000,
            was_economic: i % 2 == 0,
            region_code: 33,
        };
        let bytes = encode_versioned_value(&event, ver).expect("versioned encode curtailment");
        let (decoded, dec_ver, _): (CurtailmentEvent, Version, usize) =
            decode_versioned_value(&bytes).expect("versioned decode curtailment");
        assert_eq!(event, decoded);
        assert_eq!(dec_ver, ver);
    }
}

#[test]
fn test_microgrid_versioned_mode_transitions() {
    let modes = vec![
        (IslandingMode::GridConnected, Version::new(1, 0, 0)),
        (IslandingMode::Transitioning, Version::new(1, 1, 0)),
        (IslandingMode::Islanded, Version::new(1, 2, 0)),
        (IslandingMode::BlackStart, Version::new(2, 0, 0)),
    ];
    for (mode, ver) in modes {
        let event = MicrogridIslandingEvent {
            microgrid_id: 22,
            timestamp_ns: 1_724_000_000_000_000_000,
            mode,
            local_generation_kw: 300,
            local_load_kw: 250,
            battery_soc_pct_x100: 6000,
            frequency_mhz: 59_980,
            voltage_v_x10: 2390,
            transition_time_ms: 200,
            critical_loads_served: true,
        };
        let bytes = encode_versioned_value(&event, ver).expect("versioned encode microgrid");
        let (decoded, dec_ver, _): (MicrogridIslandingEvent, Version, usize) =
            decode_versioned_value(&bytes).expect("versioned decode microgrid");
        assert_eq!(event, decoded);
        assert_eq!(dec_ver, ver);
    }
}

#[test]
fn test_hydrogen_versioned_electrolyzer_types() {
    let types = vec![
        ElectrolyzerType::Pem,
        ElectrolyzerType::Alkaline,
        ElectrolyzerType::SolidOxide,
    ];
    let ver = Version::new(4, 2, 1);
    for etype in types {
        let unit = HydrogenElectrolysis {
            unit_id: 10,
            timestamp_ns: 1_726_000_000_000_000_000,
            electrolyzer_type: etype,
            power_input_kw: 2000,
            hydrogen_rate_g_per_s_x100: 1100,
            efficiency_pct_x100: 7200,
            stack_voltage_mv: 160_000,
            stack_current_ma: 12_500_000,
            water_flow_ml_per_min: 20_000,
            outlet_pressure_kpa: 2500,
            stack_temp_c_x10: 750,
            runtime_hours: 8_000,
        };
        let bytes = encode_versioned_value(&unit, ver).expect("versioned encode electrolysis");
        let (decoded, dec_ver, _): (HydrogenElectrolysis, Version, usize) =
            decode_versioned_value(&bytes).expect("versioned decode electrolysis");
        assert_eq!(unit, decoded);
        assert_eq!(dec_ver, ver);
    }
}

#[test]
fn test_carbon_offset_record_roundtrip() {
    let record = CarbonOffsetRecord {
        record_id: 500_001,
        project_id: 1200,
        vintage_year: 2025,
        offset_tonnes_co2e_x100: 50_000,
        baseline_emission_factor_x1000: 850,
        actual_emission_factor_x1000: 120,
        verification_body_id: 7,
        registry_serial_start: 10_000_000,
        registry_serial_end: 10_000_499,
        is_verified: true,
        is_retired: false,
    };
    let bytes = encode_to_vec(&record).expect("encode CarbonOffsetRecord failed");
    let (decoded, consumed): (CarbonOffsetRecord, usize) =
        decode_from_slice(&bytes).expect("decode CarbonOffsetRecord failed");
    assert_eq!(record, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_carbon_offset_versioned_verification_flow() {
    let ver_draft = Version::new(1, 0, 0);
    let ver_verified = Version::new(1, 1, 0);

    let draft = CarbonOffsetRecord {
        record_id: 600_001,
        project_id: 1500,
        vintage_year: 2026,
        offset_tonnes_co2e_x100: 75_000,
        baseline_emission_factor_x1000: 920,
        actual_emission_factor_x1000: 80,
        verification_body_id: 0,
        registry_serial_start: 0,
        registry_serial_end: 0,
        is_verified: false,
        is_retired: false,
    };
    let bytes_draft = encode_versioned_value(&draft, ver_draft).expect("encode draft offset");
    let (dec_draft, dv_draft, _): (CarbonOffsetRecord, Version, usize) =
        decode_versioned_value(&bytes_draft).expect("decode draft offset");
    assert_eq!(draft, dec_draft);
    assert!(!dec_draft.is_verified);
    assert_eq!(dv_draft, ver_draft);

    let verified = CarbonOffsetRecord {
        verification_body_id: 12,
        registry_serial_start: 20_000_000,
        registry_serial_end: 20_000_749,
        is_verified: true,
        ..draft.clone()
    };
    let bytes_ver =
        encode_versioned_value(&verified, ver_verified).expect("encode verified offset");
    let (dec_ver, dv_ver, _): (CarbonOffsetRecord, Version, usize) =
        decode_versioned_value(&bytes_ver).expect("decode verified offset");
    assert_eq!(verified, dec_ver);
    assert!(dec_ver.is_verified);
    assert_eq!(dv_ver, ver_verified);
}

#[test]
fn test_combined_solar_battery_versioned() {
    let solar = SolarFarmOutput {
        farm_id: 999,
        timestamp_ns: 1_730_000_000_000_000_000,
        irradiance_w_per_m2: 1100,
        panel_efficiency_pct_x100: 2300,
        inverter_status: InverterStatus::Online,
        dc_voltage_mv: 620_000,
        dc_current_ma: 90_000,
        ac_power_w: 52_000,
        ambient_temp_c_x10: 350,
        module_temp_c_x10: 600,
        tracking_azimuth_mrad: 3000,
        tracking_elevation_mrad: 1050,
    };
    let battery = BatteryEnergyStorage {
        unit_id: 50,
        timestamp_ns: 1_730_000_000_000_000_000,
        chemistry: BatteryChemistry::VanadiumRedoxFlow,
        soc_pct_x100: 4200,
        soh_pct_x100: 9900,
        charge_rate_w: 52_000,
        discharge_rate_w: 0,
        cycle_count: 350,
        cell_voltage_mv_min: 1200,
        cell_voltage_mv_max: 1500,
        pack_temp_c_x10: 300,
        capacity_wh: 1_000_000,
    };

    let ver = Version::new(3, 0, 0);
    let solar_bytes = encode_versioned_value(&solar, ver).expect("versioned encode solar");
    let battery_bytes = encode_versioned_value(&battery, ver).expect("versioned encode battery");

    let (dec_solar, sv, _): (SolarFarmOutput, Version, usize) =
        decode_versioned_value(&solar_bytes).expect("versioned decode solar");
    let (dec_battery, bv, _): (BatteryEnergyStorage, Version, usize) =
        decode_versioned_value(&battery_bytes).expect("versioned decode battery");

    assert_eq!(solar, dec_solar);
    assert_eq!(battery, dec_battery);
    assert_eq!(sv, bv);
    assert_eq!(
        dec_solar.ac_power_w, dec_battery.charge_rate_w,
        "solar output should match battery charge rate in coupled system"
    );
}
