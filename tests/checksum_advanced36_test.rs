//! Advanced checksum tests for OxiCode — exactly 22 top-level #[test] functions.
//! Theme: Water treatment and wastewater management systems.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced36_test

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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Test 1: Influent quality parameters (raw wastewater characterization)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct InfluentQuality {
    sample_id: String,
    bod5_mg_per_l: f64,
    cod_mg_per_l: f64,
    tss_mg_per_l: f64,
    ph: f64,
    ammonia_n_mg_per_l: f64,
    total_phosphorus_mg_per_l: f64,
    temperature_celsius: f64,
    flow_rate_m3_per_hour: f64,
    timestamp_epoch_secs: u64,
}

#[test]
fn test_influent_quality_parameters() {
    let sample = InfluentQuality {
        sample_id: "INF-2026-03-15-0800".into(),
        bod5_mg_per_l: 220.5,
        cod_mg_per_l: 480.0,
        tss_mg_per_l: 250.3,
        ph: 7.1,
        ammonia_n_mg_per_l: 35.2,
        total_phosphorus_mg_per_l: 7.8,
        temperature_celsius: 14.6,
        flow_rate_m3_per_hour: 1250.0,
        timestamp_epoch_secs: 1_773_676_800,
    };
    let bytes = encode_with_checksum(&sample).expect("encode influent quality");
    let (decoded, _): (InfluentQuality, _) =
        decode_with_checksum(&bytes).expect("decode influent quality");
    assert_eq!(decoded, sample);
}

// ---------------------------------------------------------------------------
// Test 2: Effluent discharge compliance
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum ComplianceStatus {
    WithinLimits,
    ApproachingLimit,
    ExceedingLimit,
    CriticalViolation,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EffluentCompliance {
    permit_id: String,
    parameter_name: String,
    measured_value: f64,
    permit_limit: f64,
    unit: String,
    status: ComplianceStatus,
    sampling_location: String,
    is_composite_sample: bool,
}

#[test]
fn test_effluent_discharge_compliance() {
    let record = EffluentCompliance {
        permit_id: "NPDES-MA-0102345".into(),
        parameter_name: "BOD5".into(),
        measured_value: 18.4,
        permit_limit: 30.0,
        unit: "mg/L".into(),
        status: ComplianceStatus::WithinLimits,
        sampling_location: "Outfall 001".into(),
        is_composite_sample: true,
    };
    let bytes = encode_with_checksum(&record).expect("encode effluent compliance");
    let (decoded, _): (EffluentCompliance, _) =
        decode_with_checksum(&bytes).expect("decode effluent compliance");
    assert_eq!(decoded, record);
}

// ---------------------------------------------------------------------------
// Test 3: Chlorine disinfection dosing
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct ChlorineDosing {
    dosing_point_id: String,
    target_residual_mg_per_l: f64,
    actual_residual_mg_per_l: f64,
    dose_rate_kg_per_day: f64,
    contact_time_minutes: f64,
    ct_value: f64,
    flow_at_dosing_m3_per_hour: f64,
    chemical_type: String,
    tank_level_percent: f64,
    auto_dosing_enabled: bool,
}

#[test]
fn test_chlorine_disinfection_dosing() {
    let dosing = ChlorineDosing {
        dosing_point_id: "CL-DOSE-003".into(),
        target_residual_mg_per_l: 0.5,
        actual_residual_mg_per_l: 0.48,
        dose_rate_kg_per_day: 42.5,
        contact_time_minutes: 30.0,
        ct_value: 450.0,
        flow_at_dosing_m3_per_hour: 850.0,
        chemical_type: "Sodium Hypochlorite 12%".into(),
        tank_level_percent: 67.3,
        auto_dosing_enabled: true,
    };
    let bytes = encode_with_checksum(&dosing).expect("encode chlorine dosing");
    let (decoded, _): (ChlorineDosing, _) =
        decode_with_checksum(&bytes).expect("decode chlorine dosing");
    assert_eq!(decoded, dosing);
}

// ---------------------------------------------------------------------------
// Test 4: UV disinfection system status
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum UvLampStatus {
    Operating,
    Standby,
    Failed,
    AgeWarning,
    Replaced,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct UvDisinfectionBank {
    bank_id: String,
    num_lamps_total: u16,
    num_lamps_active: u16,
    uv_transmittance_percent: f64,
    uv_dose_mj_per_cm2: f64,
    lamp_hours_oldest: u32,
    lamp_statuses: Vec<UvLampStatus>,
    sleeve_fouling_factor: f64,
    flow_rate_m3_per_hour: f64,
}

#[test]
fn test_uv_disinfection_bank() {
    let bank = UvDisinfectionBank {
        bank_id: "UV-BANK-A2".into(),
        num_lamps_total: 48,
        num_lamps_active: 46,
        uv_transmittance_percent: 65.2,
        uv_dose_mj_per_cm2: 40.0,
        lamp_hours_oldest: 8_200,
        lamp_statuses: vec![
            UvLampStatus::Operating,
            UvLampStatus::Operating,
            UvLampStatus::AgeWarning,
            UvLampStatus::Failed,
            UvLampStatus::Operating,
            UvLampStatus::Standby,
        ],
        sleeve_fouling_factor: 0.87,
        flow_rate_m3_per_hour: 620.0,
    };
    let bytes = encode_with_checksum(&bank).expect("encode UV bank");
    let (decoded, _): (UvDisinfectionBank, _) =
        decode_with_checksum(&bytes).expect("decode UV bank");
    assert_eq!(decoded, bank);
}

// ---------------------------------------------------------------------------
// Test 5: Membrane filtration state (MBR / UF / RO)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum MembraneOperatingMode {
    Filtration,
    Backwash,
    ChemicalClean,
    IntegrityTest,
    Standby,
    Offline,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MembraneModule {
    module_id: String,
    technology: String,
    operating_mode: MembraneOperatingMode,
    transmembrane_pressure_kpa: f64,
    permeate_flux_lmh: f64,
    specific_flux_lmh_per_bar: f64,
    total_operating_hours: u32,
    cycles_since_cip: u32,
    permeability_decline_percent: f64,
    is_integrity_ok: bool,
}

#[test]
fn test_membrane_filtration_state() {
    let module = MembraneModule {
        module_id: "MBR-TRAIN2-MOD07".into(),
        technology: "Hollow Fiber PVDF 0.04um".into(),
        operating_mode: MembraneOperatingMode::Filtration,
        transmembrane_pressure_kpa: 28.5,
        permeate_flux_lmh: 22.0,
        specific_flux_lmh_per_bar: 77.2,
        total_operating_hours: 14_500,
        cycles_since_cip: 320,
        permeability_decline_percent: 12.3,
        is_integrity_ok: true,
    };
    let bytes = encode_with_checksum(&module).expect("encode membrane module");
    let (decoded, _): (MembraneModule, _) =
        decode_with_checksum(&bytes).expect("decode membrane module");
    assert_eq!(decoded, module);
}

// ---------------------------------------------------------------------------
// Test 6: Sludge dewatering metrics
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct SludgeDewatering {
    press_id: String,
    feed_solids_percent: f64,
    cake_solids_percent: f64,
    polymer_dose_kg_per_dry_tonne: f64,
    filtrate_tss_mg_per_l: f64,
    throughput_m3_per_hour: f64,
    belt_speed_m_per_min: f64,
    belt_tension_kn: f64,
    cake_output_tonnes_per_day: f64,
    operating_hours_today: f64,
}

#[test]
fn test_sludge_dewatering_metrics() {
    let dewatering = SludgeDewatering {
        press_id: "BFP-02".into(),
        feed_solids_percent: 3.2,
        cake_solids_percent: 22.5,
        polymer_dose_kg_per_dry_tonne: 6.8,
        filtrate_tss_mg_per_l: 120.0,
        throughput_m3_per_hour: 18.0,
        belt_speed_m_per_min: 1.5,
        belt_tension_kn: 35.0,
        cake_output_tonnes_per_day: 12.8,
        operating_hours_today: 7.5,
    };
    let bytes = encode_with_checksum(&dewatering).expect("encode sludge dewatering");
    let (decoded, _): (SludgeDewatering, _) =
        decode_with_checksum(&bytes).expect("decode sludge dewatering");
    assert_eq!(decoded, dewatering);
}

// ---------------------------------------------------------------------------
// Test 7: SCADA alarm point
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum AlarmSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum AlarmState {
    Active,
    Acknowledged,
    Cleared,
    Shelved,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ScadaAlarm {
    alarm_tag: String,
    description: String,
    severity: AlarmSeverity,
    state: AlarmState,
    setpoint_value: f64,
    actual_value: f64,
    area: String,
    timestamp_epoch_secs: u64,
    auto_acknowledge: bool,
    escalation_delay_secs: u32,
}

#[test]
fn test_scada_alarm_point() {
    let alarm = ScadaAlarm {
        alarm_tag: "FIT-401-HH".into(),
        description: "Clarifier influent flow very high".into(),
        severity: AlarmSeverity::High,
        state: AlarmState::Active,
        setpoint_value: 1500.0,
        actual_value: 1623.7,
        area: "Primary Treatment".into(),
        timestamp_epoch_secs: 1_773_680_400,
        auto_acknowledge: false,
        escalation_delay_secs: 300,
    };
    let bytes = encode_with_checksum(&alarm).expect("encode SCADA alarm");
    let (decoded, _): (ScadaAlarm, _) = decode_with_checksum(&bytes).expect("decode SCADA alarm");
    assert_eq!(decoded, alarm);
}

// ---------------------------------------------------------------------------
// Test 8: Regulatory compliance sampling schedule
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum SamplingFrequency {
    Continuous,
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annual,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ComplianceSamplingPoint {
    point_id: String,
    parameter: String,
    frequency: SamplingFrequency,
    permit_limit: f64,
    unit: String,
    recent_results: Vec<f64>,
    is_exceedance_reported: bool,
    lab_method: String,
}

#[test]
fn test_regulatory_compliance_sampling() {
    let point = ComplianceSamplingPoint {
        point_id: "SP-EFF-NH3".into(),
        parameter: "Ammonia as N".into(),
        frequency: SamplingFrequency::Weekly,
        permit_limit: 5.0,
        unit: "mg/L".into(),
        recent_results: vec![2.1, 3.4, 1.8, 2.9, 4.1, 3.7, 2.5],
        is_exceedance_reported: false,
        lab_method: "EPA 350.1".into(),
    };
    let bytes = encode_with_checksum(&point).expect("encode compliance sampling");
    let (decoded, _): (ComplianceSamplingPoint, _) =
        decode_with_checksum(&bytes).expect("decode compliance sampling");
    assert_eq!(decoded, point);
}

// ---------------------------------------------------------------------------
// Test 9: Stormwater overflow event (CSO)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct CombinedSewerOverflow {
    event_id: String,
    outfall_number: u16,
    start_epoch_secs: u64,
    duration_minutes: u32,
    estimated_volume_m3: f64,
    rainfall_mm: f64,
    receiving_water: String,
    ecoli_count_per_100ml: Option<u32>,
    notification_sent: bool,
    regulatory_report_filed: bool,
}

#[test]
fn test_stormwater_overflow_event() {
    let event = CombinedSewerOverflow {
        event_id: "CSO-2026-017".into(),
        outfall_number: 4,
        start_epoch_secs: 1_773_652_200,
        duration_minutes: 185,
        estimated_volume_m3: 12_500.0,
        rainfall_mm: 42.5,
        receiving_water: "Mill Creek".into(),
        ecoli_count_per_100ml: Some(24_000),
        notification_sent: true,
        regulatory_report_filed: false,
    };
    let bytes = encode_with_checksum(&event).expect("encode CSO event");
    let (decoded, _): (CombinedSewerOverflow, _) =
        decode_with_checksum(&bytes).expect("decode CSO event");
    assert_eq!(decoded, event);
}

// ---------------------------------------------------------------------------
// Test 10: Pump station telemetry
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum PumpRunState {
    Off,
    Running,
    LeadPump,
    LagPump,
    Alternating,
    Fault,
    Maintenance,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PumpStationTelemetry {
    station_id: String,
    wet_well_level_m: f64,
    high_level_alarm_m: f64,
    low_level_alarm_m: f64,
    pump_states: Vec<PumpRunState>,
    total_flow_today_m3: f64,
    current_flow_l_per_sec: f64,
    power_consumption_kw: f64,
    backup_generator_available: bool,
    communication_status_ok: bool,
}

#[test]
fn test_pump_station_telemetry() {
    let telemetry = PumpStationTelemetry {
        station_id: "PS-NORTH-07".into(),
        wet_well_level_m: 2.45,
        high_level_alarm_m: 3.80,
        low_level_alarm_m: 0.60,
        pump_states: vec![
            PumpRunState::LeadPump,
            PumpRunState::Off,
            PumpRunState::Maintenance,
        ],
        total_flow_today_m3: 4_320.0,
        current_flow_l_per_sec: 85.3,
        power_consumption_kw: 37.2,
        backup_generator_available: true,
        communication_status_ok: true,
    };
    let bytes = encode_with_checksum(&telemetry).expect("encode pump station telemetry");
    let (decoded, _): (PumpStationTelemetry, _) =
        decode_with_checksum(&bytes).expect("decode pump station telemetry");
    assert_eq!(decoded, telemetry);
}

// ---------------------------------------------------------------------------
// Test 11: Distribution network pressure zone
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct PressureZone {
    zone_id: String,
    zone_name: String,
    hydraulic_grade_line_m: f64,
    min_pressure_kpa: f64,
    max_pressure_kpa: f64,
    avg_pressure_kpa: f64,
    num_connections: u32,
    total_demand_m3_per_day: f64,
    reservoir_level_percent: f64,
    prv_settings: Vec<f64>,
    fire_flow_capacity_l_per_sec: f64,
}

#[test]
fn test_distribution_pressure_zone() {
    let zone = PressureZone {
        zone_id: "PZ-03-HIGHLAND".into(),
        zone_name: "Highland Terrace Zone".into(),
        hydraulic_grade_line_m: 285.0,
        min_pressure_kpa: 210.0,
        max_pressure_kpa: 480.0,
        avg_pressure_kpa: 345.0,
        num_connections: 2_840,
        total_demand_m3_per_day: 5_680.0,
        reservoir_level_percent: 78.4,
        prv_settings: vec![350.0, 320.0, 280.0],
        fire_flow_capacity_l_per_sec: 120.0,
    };
    let bytes = encode_with_checksum(&zone).expect("encode pressure zone");
    let (decoded, _): (PressureZone, _) =
        decode_with_checksum(&bytes).expect("decode pressure zone");
    assert_eq!(decoded, zone);
}

// ---------------------------------------------------------------------------
// Test 12: Ozone disinfection and oxidation
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct OzoneSystem {
    generator_id: String,
    ozone_production_kg_per_hour: f64,
    applied_dose_mg_per_l: f64,
    residual_mg_per_l: f64,
    contact_time_minutes: f64,
    ct_achieved: f64,
    ct_required: f64,
    power_consumption_kwh_per_kg: f64,
    off_gas_destruct_temp_celsius: f64,
    ambient_ozone_ppm: f64,
    leak_detected: bool,
}

#[test]
fn test_ozone_disinfection_system() {
    let ozone = OzoneSystem {
        generator_id: "O3-GEN-02".into(),
        ozone_production_kg_per_hour: 8.5,
        applied_dose_mg_per_l: 3.0,
        residual_mg_per_l: 0.15,
        contact_time_minutes: 16.0,
        ct_achieved: 2.4,
        ct_required: 1.6,
        power_consumption_kwh_per_kg: 12.0,
        off_gas_destruct_temp_celsius: 320.0,
        ambient_ozone_ppm: 0.02,
        leak_detected: false,
    };
    let bytes = encode_with_checksum(&ozone).expect("encode ozone system");
    let (decoded, _): (OzoneSystem, _) = decode_with_checksum(&bytes).expect("decode ozone system");
    assert_eq!(decoded, ozone);
}

// ---------------------------------------------------------------------------
// Test 13: Activated sludge process parameters
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct ActivatedSludgeProcess {
    basin_id: String,
    mlss_mg_per_l: f64,
    mlvss_mg_per_l: f64,
    srt_days: f64,
    food_to_microorganism_ratio: f64,
    dissolved_oxygen_mg_per_l: f64,
    svi_ml_per_g: f64,
    return_sludge_rate_percent: f64,
    waste_sludge_m3_per_day: f64,
    aeration_blower_power_kw: f64,
    nitrification_active: bool,
    denitrification_active: bool,
}

#[test]
fn test_activated_sludge_process() {
    let process = ActivatedSludgeProcess {
        basin_id: "AER-BASIN-3B".into(),
        mlss_mg_per_l: 3_500.0,
        mlvss_mg_per_l: 2_800.0,
        srt_days: 12.0,
        food_to_microorganism_ratio: 0.15,
        dissolved_oxygen_mg_per_l: 2.1,
        svi_ml_per_g: 120.0,
        return_sludge_rate_percent: 75.0,
        waste_sludge_m3_per_day: 45.0,
        aeration_blower_power_kw: 185.0,
        nitrification_active: true,
        denitrification_active: true,
    };
    let bytes = encode_with_checksum(&process).expect("encode activated sludge");
    let (decoded, _): (ActivatedSludgeProcess, _) =
        decode_with_checksum(&bytes).expect("decode activated sludge");
    assert_eq!(decoded, process);
}

// ---------------------------------------------------------------------------
// Test 14: Chemical feed inventory and dosing
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum ChemicalType {
    SodiumHypochlorite,
    FerricChloride,
    Polymer,
    SodiumHydroxide,
    SulfuricAcid,
    Alum,
    Lime,
    ActivatedCarbon,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ChemicalInventory {
    chemical: ChemicalType,
    storage_tank_id: String,
    concentration_percent: f64,
    current_volume_liters: f64,
    max_capacity_liters: f64,
    daily_usage_liters: f64,
    days_remaining: f64,
    delivery_scheduled: bool,
    unit_cost_per_liter: f64,
}

#[test]
fn test_chemical_feed_inventory() {
    let inventory = ChemicalInventory {
        chemical: ChemicalType::FerricChloride,
        storage_tank_id: "CHEM-T-FeCl3-01".into(),
        concentration_percent: 40.0,
        current_volume_liters: 8_200.0,
        max_capacity_liters: 20_000.0,
        daily_usage_liters: 450.0,
        days_remaining: 18.2,
        delivery_scheduled: true,
        unit_cost_per_liter: 0.85,
    };
    let bytes = encode_with_checksum(&inventory).expect("encode chemical inventory");
    let (decoded, _): (ChemicalInventory, _) =
        decode_with_checksum(&bytes).expect("decode chemical inventory");
    assert_eq!(decoded, inventory);
}

// ---------------------------------------------------------------------------
// Test 15: Clarifier performance monitoring
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct ClarifierPerformance {
    clarifier_id: String,
    surface_overflow_rate_m3_per_m2_per_day: f64,
    weir_loading_m3_per_m_per_day: f64,
    blanket_depth_m: f64,
    effluent_tss_mg_per_l: f64,
    solids_loading_kg_per_m2_per_day: f64,
    scraper_torque_percent: f64,
    scum_baffle_condition: String,
    desludge_frequency_per_day: u8,
    is_bulking_detected: bool,
}

#[test]
fn test_clarifier_performance() {
    let clarifier = ClarifierPerformance {
        clarifier_id: "SEC-CLAR-04".into(),
        surface_overflow_rate_m3_per_m2_per_day: 24.0,
        weir_loading_m3_per_m_per_day: 185.0,
        blanket_depth_m: 0.45,
        effluent_tss_mg_per_l: 12.3,
        solids_loading_kg_per_m2_per_day: 85.0,
        scraper_torque_percent: 22.0,
        scum_baffle_condition: "Good".into(),
        desludge_frequency_per_day: 6,
        is_bulking_detected: false,
    };
    let bytes = encode_with_checksum(&clarifier).expect("encode clarifier");
    let (decoded, _): (ClarifierPerformance, _) =
        decode_with_checksum(&bytes).expect("decode clarifier");
    assert_eq!(decoded, clarifier);
}

// ---------------------------------------------------------------------------
// Test 16: Anaerobic digester gas production
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct DigestorGasProduction {
    digester_id: String,
    temperature_celsius: f64,
    volatile_solids_reduction_percent: f64,
    gas_production_m3_per_day: f64,
    methane_content_percent: f64,
    co2_content_percent: f64,
    h2s_ppm: f64,
    gas_holder_level_percent: f64,
    gas_utilization: String,
    combined_heat_power_kw: f64,
    flare_active: bool,
}

#[test]
fn test_digester_gas_production() {
    let digester = DigestorGasProduction {
        digester_id: "AD-02".into(),
        temperature_celsius: 35.5,
        volatile_solids_reduction_percent: 58.0,
        gas_production_m3_per_day: 2_800.0,
        methane_content_percent: 63.5,
        co2_content_percent: 34.2,
        h2s_ppm: 180.0,
        gas_holder_level_percent: 55.0,
        gas_utilization: "CHP Engine + Boiler".into(),
        combined_heat_power_kw: 420.0,
        flare_active: false,
    };
    let bytes = encode_with_checksum(&digester).expect("encode digester gas");
    let (decoded, _): (DigestorGasProduction, _) =
        decode_with_checksum(&bytes).expect("decode digester gas");
    assert_eq!(decoded, digester);
}

// ---------------------------------------------------------------------------
// Test 17: Water quality grab sample with lab results
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct LabAnalysisResult {
    sample_number: String,
    location: String,
    collected_epoch_secs: u64,
    analyzed_epoch_secs: u64,
    total_coliform_mpn_per_100ml: u32,
    ecoli_mpn_per_100ml: u32,
    turbidity_ntu: f64,
    free_chlorine_mg_per_l: f64,
    total_chlorine_mg_per_l: f64,
    alkalinity_mg_per_l_caco3: f64,
    hardness_mg_per_l_caco3: f64,
    meets_drinking_water_standard: bool,
}

#[test]
fn test_lab_analysis_results() {
    let result = LabAnalysisResult {
        sample_number: "LAB-2026-03-15-042".into(),
        location: "Distribution Reservoir Outlet".into(),
        collected_epoch_secs: 1_773_673_200,
        analyzed_epoch_secs: 1_773_694_800,
        total_coliform_mpn_per_100ml: 0,
        ecoli_mpn_per_100ml: 0,
        turbidity_ntu: 0.12,
        free_chlorine_mg_per_l: 0.85,
        total_chlorine_mg_per_l: 1.10,
        alkalinity_mg_per_l_caco3: 95.0,
        hardness_mg_per_l_caco3: 140.0,
        meets_drinking_water_standard: true,
    };
    let bytes = encode_with_checksum(&result).expect("encode lab analysis");
    let (decoded, _): (LabAnalysisResult, _) =
        decode_with_checksum(&bytes).expect("decode lab analysis");
    assert_eq!(decoded, result);
}

// ---------------------------------------------------------------------------
// Test 18: Aeration diffuser performance
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum DiffuserType {
    FineBubbleDisc,
    FineBubbleTube,
    CoarseBubble,
    JetAeration,
    SurfaceAerator,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AerationGrid {
    grid_id: String,
    diffuser_type: DiffuserType,
    num_diffusers: u32,
    airflow_nm3_per_hour: f64,
    standard_oxygen_transfer_efficiency_percent: f64,
    actual_oxygen_transfer_rate_kg_per_hour: f64,
    headloss_kpa: f64,
    years_in_service: f64,
    fouling_factor: f64,
    blower_discharge_pressure_kpa: f64,
}

#[test]
fn test_aeration_diffuser_performance() {
    let grid = AerationGrid {
        grid_id: "AERATION-GRID-2A".into(),
        diffuser_type: DiffuserType::FineBubbleDisc,
        num_diffusers: 1_200,
        airflow_nm3_per_hour: 3_600.0,
        standard_oxygen_transfer_efficiency_percent: 28.0,
        actual_oxygen_transfer_rate_kg_per_hour: 145.0,
        headloss_kpa: 5.2,
        years_in_service: 4.5,
        fouling_factor: 0.82,
        blower_discharge_pressure_kpa: 62.0,
    };
    let bytes = encode_with_checksum(&grid).expect("encode aeration grid");
    let (decoded, _): (AerationGrid, _) =
        decode_with_checksum(&bytes).expect("decode aeration grid");
    assert_eq!(decoded, grid);
}

// ---------------------------------------------------------------------------
// Test 19: Water main break / leak detection event
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum LeakSeverity {
    Minor,
    Moderate,
    Major,
    Catastrophic,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WaterMainBreak {
    incident_id: String,
    pipe_material: String,
    pipe_diameter_mm: u16,
    installation_year: u16,
    estimated_flow_loss_l_per_min: f64,
    severity: LeakSeverity,
    pressure_drop_kpa: f64,
    affected_connections: u32,
    isolation_valves_closed: Vec<String>,
    repair_crew_dispatched: bool,
    boil_water_advisory_issued: bool,
}

#[test]
fn test_water_main_break_event() {
    let incident = WaterMainBreak {
        incident_id: "WMB-2026-0089".into(),
        pipe_material: "Cast Iron".into(),
        pipe_diameter_mm: 300,
        installation_year: 1962,
        estimated_flow_loss_l_per_min: 850.0,
        severity: LeakSeverity::Major,
        pressure_drop_kpa: 120.0,
        affected_connections: 340,
        isolation_valves_closed: vec!["V-4201".into(), "V-4203".into(), "V-4207".into()],
        repair_crew_dispatched: true,
        boil_water_advisory_issued: false,
    };
    let bytes = encode_with_checksum(&incident).expect("encode main break");
    let (decoded, _): (WaterMainBreak, _) =
        decode_with_checksum(&bytes).expect("decode main break");
    assert_eq!(decoded, incident);
}

// ---------------------------------------------------------------------------
// Test 20: Biosolids land application record
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum BiosolidsClass {
    ClassA,
    ClassB,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BiosolidsApplication {
    permit_number: String,
    site_name: String,
    biosolids_class: BiosolidsClass,
    application_rate_dry_tonnes_per_ha: f64,
    total_nitrogen_percent: f64,
    total_phosphorus_percent: f64,
    cadmium_mg_per_kg: f64,
    lead_mg_per_kg: f64,
    mercury_mg_per_kg: f64,
    area_applied_ha: f64,
    agronomic_rate_compliant: bool,
    pathogen_reduction_verified: bool,
}

#[test]
fn test_biosolids_land_application() {
    let record = BiosolidsApplication {
        permit_number: "BIO-APP-2026-014".into(),
        site_name: "Henderson Farm Field B".into(),
        biosolids_class: BiosolidsClass::ClassB,
        application_rate_dry_tonnes_per_ha: 5.0,
        total_nitrogen_percent: 4.2,
        total_phosphorus_percent: 2.1,
        cadmium_mg_per_kg: 1.8,
        lead_mg_per_kg: 45.0,
        mercury_mg_per_kg: 0.9,
        area_applied_ha: 24.0,
        agronomic_rate_compliant: true,
        pathogen_reduction_verified: true,
    };
    let bytes = encode_with_checksum(&record).expect("encode biosolids application");
    let (decoded, _): (BiosolidsApplication, _) =
        decode_with_checksum(&bytes).expect("decode biosolids application");
    assert_eq!(decoded, record);
}

// ---------------------------------------------------------------------------
// Test 21: SCADA historian trending data point
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum DataQuality {
    Good,
    Uncertain,
    Bad,
    Substituted,
    Interpolated,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct HistorianTrendPoint {
    tag_name: String,
    engineering_unit: String,
    timestamp_epoch_millis: u64,
    value: f64,
    quality: DataQuality,
    hi_hi_alarm: f64,
    hi_alarm: f64,
    lo_alarm: f64,
    lo_lo_alarm: f64,
    deadband_percent: f64,
    compression_enabled: bool,
}

#[test]
fn test_historian_trend_data() {
    let point = HistorianTrendPoint {
        tag_name: "LIT-201.PV".into(),
        engineering_unit: "m".into(),
        timestamp_epoch_millis: 1_773_680_400_000,
        value: 3.24,
        quality: DataQuality::Good,
        hi_hi_alarm: 4.50,
        hi_alarm: 4.00,
        lo_alarm: 1.00,
        lo_lo_alarm: 0.50,
        deadband_percent: 0.5,
        compression_enabled: true,
    };
    let bytes = encode_with_checksum(&point).expect("encode historian point");
    let (decoded, _): (HistorianTrendPoint, _) =
        decode_with_checksum(&bytes).expect("decode historian point");
    assert_eq!(decoded, point);
}

// ---------------------------------------------------------------------------
// Test 22: Reverse osmosis desalination train
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct RoDesalinationTrain {
    train_id: String,
    feed_tds_mg_per_l: f64,
    permeate_tds_mg_per_l: f64,
    concentrate_tds_mg_per_l: f64,
    recovery_percent: f64,
    feed_pressure_bar: f64,
    differential_pressure_bar: f64,
    normalized_permeate_flow_m3_per_hour: f64,
    salt_rejection_percent: f64,
    specific_energy_kwh_per_m3: f64,
    membrane_age_years: f64,
    antiscalant_dose_mg_per_l: f64,
    cip_due: bool,
}

#[test]
fn test_ro_desalination_train() {
    let train = RoDesalinationTrain {
        train_id: "RO-TRAIN-04".into(),
        feed_tds_mg_per_l: 35_000.0,
        permeate_tds_mg_per_l: 180.0,
        concentrate_tds_mg_per_l: 70_000.0,
        recovery_percent: 45.0,
        feed_pressure_bar: 62.0,
        differential_pressure_bar: 2.8,
        normalized_permeate_flow_m3_per_hour: 420.0,
        salt_rejection_percent: 99.48,
        specific_energy_kwh_per_m3: 3.8,
        membrane_age_years: 3.2,
        antiscalant_dose_mg_per_l: 2.5,
        cip_due: false,
    };
    let bytes = encode_with_checksum(&train).expect("encode RO train");
    let (decoded, _): (RoDesalinationTrain, _) =
        decode_with_checksum(&bytes).expect("decode RO train");
    assert_eq!(decoded, train);
}
